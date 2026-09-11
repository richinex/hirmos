//! Calculate training-set concordance for a fitted Cox model.

use super::data::Event;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ConcordanceIndex(f64);

impl ConcordanceIndex {
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Counts previously observed predictions by their rank.
struct PredictionRanks {
    values: Vec<f64>,
    counts: Vec<u64>,
    len: u64,
}

impl PredictionRanks {
    fn new(mut values: Vec<f64>) -> Self {
        values.sort_unstable_by(f64::total_cmp);
        values.dedup_by(|left, right| left.total_cmp(right).is_eq());
        Self {
            counts: vec![0; values.len() + 1],
            values,
            len: 0,
        }
    }

    fn position(&self, value: f64) -> (usize, bool) {
        let index = self
            .values
            .partition_point(|candidate| candidate.total_cmp(&value).is_lt());
        let present = self
            .values
            .get(index)
            .is_some_and(|candidate| candidate.total_cmp(&value).is_eq());
        (index, present)
    }

    fn prefix_count(&self, end: usize) -> u64 {
        let mut index = end;
        let mut count = 0;
        while index > 0 {
            count += self.counts[index];
            index &= index - 1;
        }
        count
    }

    fn rank(&self, index: usize, present: bool) -> (u64, u64) {
        let less = self.prefix_count(index);
        let equal = present
            .then(|| self.prefix_count(index + 1) - less)
            .unwrap_or(0);
        (less, equal)
    }

    fn insert(&mut self, position: usize) {
        let mut index = position + 1;
        while index < self.counts.len() {
            self.counts[index] += 1;
            index += index & index.wrapping_neg();
        }
        self.len += 1;
    }
}

fn sort_rows_by_duration(rows: &mut [usize], durations: &[f64]) {
    rows.sort_unstable_by(|left, right| durations[*left].total_cmp(&durations[*right]));
}

fn tied_duration_end(rows: &[usize], first: usize, durations: &[f64]) -> usize {
    let duration = durations[rows[first]];
    let mut end = first + 1;
    while end < rows.len() && durations[rows[end]] == duration {
        end += 1;
    }
    end
}

fn score_duration_group(
    rows: &[usize],
    first: usize,
    end: usize,
    prediction_positions: &[(usize, bool)],
    ranks: &PredictionRanks,
) -> (u64, u64, u64) {
    let pairs = ranks.len * (end - first) as u64;
    let mut correct = 0;
    let mut tied = 0;
    for row in &rows[first..end] {
        let (position, present) = prediction_positions[*row];
        let (row_correct, row_tied) = ranks.rank(position, present);
        correct += row_correct;
        tied += row_tied;
    }
    (pairs, correct, tied)
}

fn prediction_counts(
    durations: &[f64],
    events: &[Event],
    predicted_event_times: &[f64],
    selected: &[usize],
) -> (u64, u64, u64) {
    let observed_predictions = selected
        .iter()
        .copied()
        .filter(|row| events[*row] == Event::Observed)
        .map(|row| predicted_event_times[row])
        .collect::<Vec<_>>();
    if observed_predictions.is_empty() {
        return (0, 0, 0);
    }

    let mut observed = selected
        .iter()
        .copied()
        .filter(|row| events[*row] == Event::Observed)
        .collect::<Vec<_>>();
    let mut censored = selected
        .iter()
        .copied()
        .filter(|row| events[*row] == Event::Censored)
        .collect::<Vec<_>>();
    sort_rows_by_duration(&mut observed, durations);
    sort_rows_by_duration(&mut censored, durations);

    let mut ranks = PredictionRanks::new(observed_predictions);
    let prediction_positions = predicted_event_times
        .iter()
        .map(|value| ranks.position(*value))
        .collect::<Vec<_>>();
    let mut observed_index = 0;
    let mut censored_index = 0;
    let mut pairs = 0;
    let mut correct = 0;
    let mut tied = 0;

    while observed_index < observed.len() || censored_index < censored.len() {
        let next_is_censored = censored_index < censored.len()
            && (observed_index == observed.len()
                || durations[censored[censored_index]] < durations[observed[observed_index]]);
        if next_is_censored {
            let end = tied_duration_end(&censored, censored_index, durations);
            let (new_pairs, new_correct, new_tied) = score_duration_group(
                &censored,
                censored_index,
                end,
                &prediction_positions,
                &ranks,
            );
            pairs += new_pairs;
            correct += new_correct;
            tied += new_tied;
            censored_index = end;
            continue;
        }

        let end = tied_duration_end(&observed, observed_index, durations);
        let (new_pairs, new_correct, new_tied) = score_duration_group(
            &observed,
            observed_index,
            end,
            &prediction_positions,
            &ranks,
        );
        pairs += new_pairs;
        correct += new_correct;
        tied += new_tied;
        for row in &observed[observed_index..end] {
            let (position, present) = prediction_positions[*row];
            debug_assert!(present);
            ranks.insert(position);
        }
        observed_index = end;
    }

    (pairs, correct, tied)
}

pub(crate) fn concordance_index(
    durations: &[f64],
    events: &[Event],
    log_partial_hazards: &[f64],
    strata: Option<&[usize]>,
) -> Option<ConcordanceIndex> {
    let predictions: Vec<f64> = log_partial_hazards
        .iter()
        .map(|value| -value.exp())
        .collect();
    prediction_concordance_index(durations, events, &predictions, strata)
}

/// Shared lifelines concordance counter: larger predictions mean later events.
pub(crate) fn prediction_concordance_index(
    durations: &[f64],
    events: &[Event],
    predictions: &[f64],
    strata: Option<&[usize]>,
) -> Option<ConcordanceIndex> {
    let (pairs, correct, tied) = if let Some(strata) = strata {
        let mut groups = strata.to_vec();
        groups.sort_unstable();
        groups.dedup();
        groups.into_iter().fold(
            (0_u64, 0_u64, 0_u64),
            |(all_pairs, all_correct, all_tied), stratum| {
                let selected = (0..strata.len())
                    .filter(|row| strata[*row] == stratum)
                    .collect::<Vec<_>>();
                let (pairs, correct, tied) =
                    prediction_counts(durations, events, predictions, &selected);
                (all_pairs + pairs, all_correct + correct, all_tied + tied)
            },
        )
    } else {
        prediction_counts(
            durations,
            events,
            predictions,
            &(0..durations.len()).collect::<Vec<_>>(),
        )
    };
    (pairs > 0).then(|| ConcordanceIndex((correct as f64 + tied as f64 / 2.0) / pairs as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn concordance_counts(
        durations: &[f64],
        events: &[Event],
        log_partial_hazards: &[f64],
        selected: &[usize],
    ) -> (u64, u64, u64) {
        prediction_counts(
            durations,
            events,
            &log_partial_hazards
                .iter()
                .map(|v| -v.exp())
                .collect::<Vec<_>>(),
            selected,
        )
    }

    fn naive_counts(
        durations: &[f64],
        events: &[Event],
        log_partial_hazards: &[f64],
    ) -> (u64, u64, u64) {
        let predictions = log_partial_hazards
            .iter()
            .map(|value| -value.exp())
            .collect::<Vec<_>>();
        let mut pairs = 0;
        let mut correct = 0;
        let mut tied = 0;
        for left in 0..durations.len() {
            for right in left + 1..durations.len() {
                let left_observed = events[left] == Event::Observed;
                let right_observed = events[right] == Event::Observed;
                let comparable = if durations[left] == durations[right] {
                    left_observed != right_observed
                } else {
                    (left_observed && right_observed)
                        || (left_observed && durations[left] < durations[right])
                        || (right_observed && durations[right] < durations[left])
                };
                if !comparable {
                    continue;
                }
                pairs += 1;
                if predictions[left] == predictions[right] {
                    tied += 1;
                    continue;
                }
                let is_correct = if predictions[left] < predictions[right] {
                    durations[left] < durations[right]
                        || (durations[left] == durations[right] && left_observed && !right_observed)
                } else {
                    durations[left] > durations[right]
                        || (durations[left] == durations[right] && !left_observed && right_observed)
                };
                correct += u64::from(is_correct);
            }
        }
        (pairs, correct, tied)
    }

    #[test]
    fn ranked_counts_match_pairwise_counts_with_ties_and_censoring() {
        let duration_values = [1.0, 2.0, 3.0];
        let hazard_values = [-0.8, 0.0, 0.7];
        for encoded in 0..3_usize.pow(6) {
            let mut state = encoded;
            let mut durations = Vec::with_capacity(6);
            let mut hazards = Vec::with_capacity(6);
            let mut events = Vec::with_capacity(6);
            for row in 0..6 {
                durations.push(duration_values[state % duration_values.len()]);
                state /= duration_values.len();
                hazards.push(hazard_values[(encoded + row) % hazard_values.len()]);
                events.push(if (encoded >> row) & 1 == 1 {
                    Event::Observed
                } else {
                    Event::Censored
                });
            }
            let expected = naive_counts(&durations, &events, &hazards);
            let actual = concordance_counts(
                &durations,
                &events,
                &hazards,
                &(0..durations.len()).collect::<Vec<_>>(),
            );
            assert_eq!(actual, expected, "encoded case {encoded}");
        }
    }
}
