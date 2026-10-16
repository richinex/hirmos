//! TSHRC 0.1-6 two-stage survival-curve comparison.

use std::num::NonZeroUsize;

use super::data::{ComparisonData, Event, Group, SurvivalSample};
use crate::survival::r_rng::RRng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoStageConfiguration {
    pub(crate) resamples: NonZeroUsize,
    pub(crate) significance_level: f64,
    pub(crate) epsilon: f64,
    pub(crate) seed: u32,
}

impl TwoStageConfiguration {
    pub fn new(
        resamples: NonZeroUsize,
        significance_level: f64,
        epsilon: f64,
        seed: u32,
    ) -> Result<Self, TwoStageError> {
        if !significance_level.is_finite() || !(0.0..1.0).contains(&significance_level) {
            return Err(TwoStageError::InvalidSignificanceLevel);
        }
        if !epsilon.is_finite() || !(0.0..1.0).contains(&epsilon) {
            return Err(TwoStageError::InvalidEpsilon);
        }
        Ok(Self {
            resamples,
            significance_level,
            epsilon,
            seed,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TwoStageResult {
    pub log_rank_p_value: f64,
    pub crossing_hazards_p_value: f64,
    pub two_stage_p_value: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TwoStageError {
    InvalidSignificanceLevel,
    InvalidEpsilon,
    TooFewEventTimes,
    UndefinedStatistic,
}

#[derive(Clone)]
struct LifeTable {
    sorted: Vec<SurvivalSample>,
    group_one_size: usize,
    group_zero_size: usize,
    event_times: Vec<f64>,
    risk_one: Vec<usize>,
    risk_zero: Vec<usize>,
    events_one: Vec<usize>,
    events_zero: Vec<usize>,
    censored_one: Vec<usize>,
    censored_zero: Vec<usize>,
}

fn life_table(samples: &[SurvivalSample]) -> LifeTable {
    let mut sorted = samples.to_vec();
    sorted.sort_by(|left, right| {
        left.time()
            .total_cmp(&right.time())
            .then_with(|| right.event().cmp(&left.event()))
            .then_with(|| left.group().cmp(&right.group()))
    });
    let group_one_size = sorted
        .iter()
        .filter(|sample| sample.group() == Group::One)
        .count();
    let group_zero_size = sorted.len() - group_one_size;
    let mut event_times = sorted
        .iter()
        .filter(|sample| sample.event() == Event::Observed)
        .map(|sample| sample.time())
        .collect::<Vec<_>>();
    event_times.dedup();
    let mut risk_one = Vec::with_capacity(event_times.len());
    let mut risk_zero = Vec::with_capacity(event_times.len());
    let mut events_one = Vec::with_capacity(event_times.len());
    let mut events_zero = Vec::with_capacity(event_times.len());
    let mut censored_one = Vec::with_capacity(event_times.len());
    let mut censored_zero = Vec::with_capacity(event_times.len());
    let mut previous = f64::NEG_INFINITY;
    for &time in &event_times {
        risk_one.push(
            sorted
                .iter()
                .filter(|sample| sample.group() == Group::One && sample.time() >= time)
                .count(),
        );
        risk_zero.push(
            sorted
                .iter()
                .filter(|sample| sample.group() == Group::Zero && sample.time() >= time)
                .count(),
        );
        events_one.push(
            sorted
                .iter()
                .filter(|sample| {
                    sample.group() == Group::One
                        && sample.event() == Event::Observed
                        && sample.time() == time
                })
                .count(),
        );
        events_zero.push(
            sorted
                .iter()
                .filter(|sample| {
                    sample.group() == Group::Zero
                        && sample.event() == Event::Observed
                        && sample.time() == time
                })
                .count(),
        );
        censored_one.push(
            sorted
                .iter()
                .filter(|sample| {
                    sample.group() == Group::One
                        && sample.event() == Event::Censored
                        && sample.time() > previous
                        && sample.time() <= time
                })
                .count(),
        );
        censored_zero.push(
            sorted
                .iter()
                .filter(|sample| {
                    sample.group() == Group::Zero
                        && sample.event() == Event::Censored
                        && sample.time() > previous
                        && sample.time() <= time
                })
                .count(),
        );
        previous = time;
    }
    LifeTable {
        sorted,
        group_one_size,
        group_zero_size,
        event_times,
        risk_one,
        risk_zero,
        events_one,
        events_zero,
        censored_one,
        censored_zero,
    }
}

fn tshrc_normal_cdf(value: f64) -> f64 {
    let x = value.abs();
    let mut polynomial = (((((0.000_005_383 * x + 0.000_048_890_6) * x + 0.000_038_003_6) * x
        + 0.003_277_626_3)
        * x
        + 0.021_141_006_1)
        * x
        + 0.049_867_347)
        * x
        + 1.0;
    for _ in 0..4 {
        polynomial *= polynomial;
    }
    let cdf = 1.0 - 0.5 / polynomial;
    if value < 0.0 {
        1.0 - cdf
    } else {
        cdf
    }
}

fn statistic(table: &LifeTable, weights: &[f64]) -> Result<(f64, f64), TwoStageError> {
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    for (index, &weight) in weights.iter().enumerate() {
        let n_one = table.risk_one[index] as f64;
        let n_zero = table.risk_zero[index] as f64;
        let risk = n_one + n_zero;
        let d_one = table.events_one[index] as f64;
        let events = d_one + table.events_zero[index] as f64;
        numerator += weight * (d_one - n_one * events / risk);
        let variance = if risk == 1.0 {
            n_one / risk * (1.0 - n_one / risk) * events
        } else {
            n_one / risk * (1.0 - n_one / risk) * (risk - events) / (risk - 1.0) * events
        };
        denominator += weight * weight * variance;
    }
    if denominator <= 0.0 {
        return Err(TwoStageError::UndefinedStatistic);
    }
    let value = numerator / denominator.sqrt();
    Ok((value, 2.0 * (1.0 - tshrc_normal_cdf(value.abs()))))
}

fn mantel_weights(table: &LifeTable, epsilon: f64) -> Result<Vec<f64>, TwoStageError> {
    let count = table.event_times.len();
    let lower = 3_usize.max((count as f64 * epsilon).floor() as usize);
    let upper = count.saturating_sub(lower);
    if lower > upper || count < 2 {
        return Err(TwoStageError::TooFewEventTimes);
    }
    let mut survival = vec![0.0; count];
    let mut censor_survival = vec![0.0; count];
    for index in 0..count {
        let events = table.events_one[index] + table.events_zero[index];
        let risk = table.risk_one[index] + table.risk_zero[index];
        let censored = table.censored_one[index] + table.censored_zero[index];
        survival[index] = (if index == 0 { 1.0 } else { survival[index - 1] })
            * (1.0 - events as f64 / risk as f64);
        let censor_risk = if index == 0 {
            table.group_one_size + table.group_zero_size
        } else {
            table.risk_one[index - 1] + table.risk_zero[index - 1]
        };
        censor_survival[index] = (if index == 0 {
            1.0
        } else {
            censor_survival[index - 1]
        }) * (1.0 - censored as f64 / censor_risk as f64);
    }

    let weights_at = |change: usize| {
        let numerator = (0..change)
            .map(|index| {
                censor_survival[index]
                    * (survival[index] - if index == 0 { 1.0 } else { survival[index - 1] })
            })
            .sum::<f64>();
        let denominator = (change..count)
            .map(|index| censor_survival[index] * (survival[index] - survival[index - 1]))
            .sum::<f64>();
        (0..count)
            .map(|index| {
                if index + 1 < change {
                    -1.0
                } else {
                    numerator / denominator
                }
            })
            .collect::<Vec<_>>()
    };
    let mut best = 0.0;
    let mut change = lower;
    for fortran_change in lower..=upper {
        let weights = weights_at(fortran_change);
        let (value, _) = statistic(table, &weights)?;
        if value.abs() > best {
            best = value.abs();
            change = fortran_change;
        }
    }
    Ok(weights_at(change))
}

pub fn two_stage(
    data: &ComparisonData,
    configuration: TwoStageConfiguration,
) -> Result<TwoStageResult, TwoStageError> {
    let mut rng = RRng::new(configuration.seed);
    two_stage_with_rng(data, configuration, &mut rng)
}

pub(crate) fn two_stage_with_rng(
    data: &ComparisonData,
    configuration: TwoStageConfiguration,
    rng: &mut RRng,
) -> Result<TwoStageResult, TwoStageError> {
    let original = life_table(data.samples());
    let (_, log_rank_p_value) = statistic(&original, &vec![1.0; original.event_times.len()])?;
    let mut negative = 0;
    let mut positive = 0;
    for _ in 0..configuration.resamples.get() {
        // Preserve TSHRC's source behavior: ARRANGEDATA sorts the original arrays,
        // then RESAMPLE1 samples the first N1 and last N2 positions in-place.
        let mut sample = Vec::with_capacity(original.sorted.len());
        for _ in 0..original.group_one_size {
            let index = (rng.uniform() * original.group_one_size as f64) as usize;
            sample.push(original.sorted[index]);
        }
        for _ in 0..original.group_zero_size {
            let index = original.group_one_size
                + (rng.uniform() * original.group_zero_size as f64) as usize;
            sample.push(original.sorted[index]);
        }
        let table = life_table(&sample);
        let weights = mantel_weights(&table, configuration.epsilon)?;
        let (value, _) = statistic(&table, &weights)?;
        if value < 0.0 {
            negative += 1;
        } else {
            positive += 1;
        }
    }
    let crossing_hazards_p_value =
        2.0 * negative.min(positive) as f64 / configuration.resamples.get() as f64;
    let first_stage = 1.0 - (1.0 - configuration.significance_level).sqrt();
    let two_stage_p_value = if log_rank_p_value <= first_stage {
        log_rank_p_value
    } else {
        first_stage + crossing_hazards_p_value * (1.0 - first_stage)
    };
    Ok(TwoStageResult {
        log_rank_p_value,
        crossing_hazards_p_value,
        two_stage_p_value,
    })
}
