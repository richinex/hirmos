//! Weighted Kaplan--Meier and Nelson--Aalen estimators translated from lifelines 0.30.3.

use spec_math::cephes64::ndtri;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EventStatus {
    Censored,
    Observed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightedObservation {
    duration: f64,
    entry: f64,
    event: EventStatus,
    weight: f64,
}

impl WeightedObservation {
    pub fn new(duration: f64, event: EventStatus, weight: f64) -> Result<Self, NonparametricError> {
        Self::with_entry(duration, 0.0, event, weight)
    }

    pub fn with_entry(
        duration: f64,
        entry: f64,
        event: EventStatus,
        weight: f64,
    ) -> Result<Self, NonparametricError> {
        if !duration.is_finite() || duration < 0.0 {
            return Err(NonparametricError::InvalidDuration);
        }
        if !entry.is_finite() || entry > duration {
            return Err(NonparametricError::InvalidEntry);
        }
        if !weight.is_finite() || weight < 0.0 {
            return Err(NonparametricError::InvalidWeight);
        }
        Ok(Self {
            duration,
            entry,
            event,
            weight,
        })
    }

    pub const fn event(self) -> EventStatus {
        self.event
    }

    pub const fn duration(self) -> f64 {
        self.duration
    }

    pub const fn entry(self) -> f64 {
        self.entry
    }

    pub const fn weight(self) -> f64 {
        self.weight
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NelsonAalenTies {
    Discrete,
    Smoothed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalEstimate {
    pub time: f64,
    pub survival: f64,
    pub cumulative_density: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CumulativeHazardEstimate {
    pub time: f64,
    pub cumulative_hazard: f64,
    pub variance: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct EventRow {
    time: f64,
    removed: f64,
    observed: f64,
    entrance: f64,
    at_risk: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonparametricError {
    EmptyDataset,
    EmptyTimeline,
    InvalidDuration,
    InvalidEntry,
    InvalidWeight,
    InvalidTimeline,
    InvalidAlpha,
    NoPositiveWeight,
    SmoothingRequiresIntegerWeights,
}

fn same_number(left: f64, right: f64) -> bool {
    left.total_cmp(&right) == Ordering::Equal
}

fn event_table(observations: &[WeightedObservation]) -> Result<Vec<EventRow>, NonparametricError> {
    if observations.is_empty() {
        return Err(NonparametricError::EmptyDataset);
    }
    if observations.iter().map(|row| row.weight).sum::<f64>() <= 0.0 {
        return Err(NonparametricError::NoPositiveWeight);
    }

    let mut times = observations
        .iter()
        .flat_map(|row| [row.duration, row.entry])
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup_by(|left, right| same_number(*left, *right));

    let mut entered = 0.0;
    let mut removed_before = 0.0;
    let mut rows = Vec::with_capacity(times.len());
    for time in times {
        let entrance = observations
            .iter()
            .filter(|row| same_number(row.entry, time))
            .map(|row| row.weight)
            .sum::<f64>();
        let removed = observations
            .iter()
            .filter(|row| same_number(row.duration, time))
            .map(|row| row.weight)
            .sum::<f64>();
        let observed = observations
            .iter()
            .filter(|row| same_number(row.duration, time) && row.event == EventStatus::Observed)
            .map(|row| row.weight)
            .sum::<f64>();
        entered += entrance;
        rows.push(EventRow {
            time,
            removed,
            observed,
            entrance,
            at_risk: entered - removed_before,
        });
        removed_before += removed;
    }
    Ok(rows)
}

fn validate_request(
    observations: &[WeightedObservation],
    timeline: &[f64],
    alpha: f64,
) -> Result<Vec<EventRow>, NonparametricError> {
    if timeline.is_empty() {
        return Err(NonparametricError::EmptyTimeline);
    }
    if timeline.iter().any(|time| !time.is_finite()) {
        return Err(NonparametricError::InvalidTimeline);
    }
    if !alpha.is_finite() || !(0.0..1.0).contains(&alpha) {
        return Err(NonparametricError::InvalidAlpha);
    }
    event_table(observations)
}

fn at_timeline<T: Copy>(timeline: &[f64], estimates: &[(f64, T)], initial: T) -> Vec<(f64, T)> {
    let mut sorted = timeline.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted
        .into_iter()
        .map(|time| {
            let value = estimates
                .iter()
                .take_while(|(event_time, _)| *event_time <= time)
                .last()
                .map_or(initial, |(_, value)| *value);
            (time, value)
        })
        .collect()
}

fn integer(value: f64) -> Option<usize> {
    if value >= 0.0 && value.fract() == 0.0 && value <= usize::MAX as f64 {
        Some(value as usize)
    } else {
        None
    }
}

fn harmonic_difference(population: usize, deaths: usize, squared: bool) -> f64 {
    ((population - deaths + 1)..=population)
        .map(|value| {
            let denominator = value as f64;
            if squared {
                1.0 / (denominator * denominator)
            } else {
                1.0 / denominator
            }
        })
        .sum()
}

pub fn nelson_aalen(
    observations: &[WeightedObservation],
    timeline: &[f64],
    alpha: f64,
    ties: NelsonAalenTies,
) -> Result<Vec<CumulativeHazardEstimate>, NonparametricError> {
    let rows = validate_request(observations, timeline, alpha)?;
    let first_time = rows[0].time;
    let mut cumulative_hazard = 0.0;
    let mut cumulative_variance = 0.0;
    let mut values = Vec::with_capacity(rows.len());
    for row in rows {
        let population = row.at_risk
            - if same_number(row.time, first_time) {
                0.0
            } else {
                row.entrance
            };
        let (increment, variance) = match ties {
            NelsonAalenTies::Discrete => {
                if population <= 0.0 {
                    (0.0, 0.0)
                } else {
                    let ratio = row.observed / population;
                    (ratio, (1.0 - ratio) * ratio / population)
                }
            }
            NelsonAalenTies::Smoothed => {
                let population = integer(population)
                    .ok_or(NonparametricError::SmoothingRequiresIntegerWeights)?;
                let deaths = integer(row.observed)
                    .filter(|deaths| *deaths <= population)
                    .ok_or(NonparametricError::SmoothingRequiresIntegerWeights)?;
                (
                    harmonic_difference(population, deaths, false),
                    harmonic_difference(population, deaths, true),
                )
            }
        };
        cumulative_hazard += increment;
        cumulative_variance += variance;
        values.push((row.time, (cumulative_hazard, cumulative_variance)));
    }

    let z = ndtri(1.0 - alpha / 2.0);
    Ok(at_timeline(timeline, &values, (0.0, 0.0))
        .into_iter()
        .map(|(time, (hazard, variance))| {
            let scale = if hazard == 0.0 { 1.0 } else { hazard };
            let shift = z * variance.sqrt() / scale;
            CumulativeHazardEstimate {
                time,
                cumulative_hazard: hazard,
                variance,
                lower: hazard * (-shift).exp(),
                upper: hazard * shift.exp(),
            }
        })
        .collect())
}

pub fn kaplan_meier(
    observations: &[WeightedObservation],
    timeline: &[f64],
    alpha: f64,
) -> Result<Vec<SurvivalEstimate>, NonparametricError> {
    let rows = validate_request(observations, timeline, alpha)?;
    let first_time = rows[0].time;
    let mut log_survival = 0.0;
    let mut cumulative_variance = 0.0;
    let mut values = Vec::with_capacity(rows.len());
    for row in rows {
        let population = row.at_risk
            - if same_number(row.time, first_time) {
                0.0
            } else {
                row.entrance
            };
        if population > 0.0 {
            let survivors = population - row.observed;
            log_survival += if survivors > 0.0 {
                survivors.ln() - population.ln()
            } else {
                f64::NEG_INFINITY
            };
            if survivors > 0.0 {
                cumulative_variance += row.observed / (population * survivors);
            }
        }
        values.push((row.time, (log_survival.exp(), cumulative_variance)));
    }

    let z = ndtri(1.0 - alpha / 2.0);
    Ok(at_timeline(timeline, &values, (1.0, 0.0))
        .into_iter()
        .map(|(time, (survival, variance))| {
            let (lower, upper) = exponential_greenwood(survival, variance, z);
            SurvivalEstimate {
                time,
                survival,
                cumulative_density: 1.0 - survival,
                lower,
                upper,
            }
        })
        .collect())
}

fn exponential_greenwood(survival: f64, variance: f64, z: f64) -> (f64, f64) {
    if survival == 1.0 {
        return (1.0, 1.0);
    }
    if survival == 0.0 {
        return (0.0, 0.0);
    }
    let log_survival = survival.ln();
    let log_negative_log = (-log_survival).ln();
    let ratio = z * variance.sqrt() / log_survival;
    (
        (-((log_negative_log - ratio).exp())).exp(),
        (-((log_negative_log + ratio).exp())).exp(),
    )
}
