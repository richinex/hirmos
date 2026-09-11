//! Breslow baseline estimates used by lifelines after an Efron coefficient fit.

use std::collections::HashMap;

use super::data::{Event, RightCensoredData, TimeVaryingData};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BaselineEstimate {
    pub time: f64,
    pub hazard: f64,
    pub cumulative_hazard: f64,
    pub survival: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StratumBaseline {
    pub stratum: usize,
    pub estimates: Vec<BaselineEstimate>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BaselineCurves {
    Shared(Vec<BaselineEstimate>),
    Stratified(Vec<StratumBaseline>),
}

impl BaselineCurves {
    pub fn shared(&self) -> Option<&[BaselineEstimate]> {
        match self {
            Self::Shared(estimates) => Some(estimates),
            Self::Stratified(_) => None,
        }
    }

    pub fn stratified(&self) -> Option<&[StratumBaseline]> {
        match self {
            Self::Shared(_) => None,
            Self::Stratified(estimates) => Some(estimates),
        }
    }
}

fn centered_partial_hazards(
    covariates: &[f64],
    rows: usize,
    columns: usize,
    means: &[f64],
    coefficients: &[f64],
) -> Vec<f64> {
    (0..rows)
        .map(|row| {
            let start = row * columns;
            (0..columns)
                .map(|column| (covariates[start + column] - means[column]) * coefficients[column])
                .sum::<f64>()
                .exp()
        })
        .collect()
}

fn right_censored_curve(
    data: &RightCensoredData,
    partial_hazards: &[f64],
    times: &[f64],
    selected: impl Iterator<Item = usize>,
) -> Vec<BaselineEstimate> {
    let mut deaths = vec![0.0; times.len()];
    let mut risk_at_exit = vec![0.0; times.len()];
    let time_indices = times
        .iter()
        .enumerate()
        .map(|(index, time)| (time.to_bits(), index))
        .collect::<HashMap<_, _>>();
    for row in selected {
        let time_index = time_indices[&data.durations()[row].to_bits()];
        risk_at_exit[time_index] += data.weights()[row] * partial_hazards[row];
        if data.events()[row] == Event::Observed {
            deaths[time_index] += data.weights()[row];
        }
    }

    let mut risk = 0.0;
    for value in risk_at_exit.iter_mut().rev() {
        risk += *value;
        *value = risk;
    }

    let mut cumulative_hazard = 0.0;
    times
        .iter()
        .copied()
        .enumerate()
        .map(|(index, time)| {
            let hazard = if risk_at_exit[index] == 0.0 {
                0.0
            } else {
                deaths[index] / risk_at_exit[index]
            };
            cumulative_hazard += hazard;
            BaselineEstimate {
                time,
                hazard,
                cumulative_hazard,
                survival: (-cumulative_hazard).exp(),
            }
        })
        .collect()
}

pub(crate) fn right_censored_baseline(
    data: &RightCensoredData,
    means: &[f64],
    coefficients: &[f64],
) -> BaselineCurves {
    let partial = centered_partial_hazards(
        data.covariates().values(),
        data.rows(),
        data.columns(),
        means,
        coefficients,
    );
    let mut times = data.durations().to_vec();
    times.sort_by(f64::total_cmp);
    times.dedup_by(|left, right| left.total_cmp(right).is_eq());
    let Some(strata) = data.strata() else {
        return BaselineCurves::Shared(right_censored_curve(
            data,
            &partial,
            &times,
            0..data.rows(),
        ));
    };
    let mut groups = strata.to_vec();
    groups.sort_unstable();
    groups.dedup();
    BaselineCurves::Stratified(
        groups
            .into_iter()
            .map(|stratum| StratumBaseline {
                stratum,
                estimates: right_censored_curve(
                    data,
                    &partial,
                    &times,
                    (0..data.rows()).filter(|row| strata[*row] == stratum),
                ),
            })
            .collect(),
    )
}

pub(crate) fn time_varying_baseline(
    data: &TimeVaryingData,
    means: &[f64],
    coefficients: &[f64],
) -> BaselineCurves {
    let partial = centered_partial_hazards(
        data.covariates().values(),
        data.rows(),
        data.columns(),
        means,
        coefficients,
    );
    let mut times = (0..data.rows())
        .filter(|row| data.events()[*row] == Event::Observed)
        .map(|row| data.stops()[row])
        .collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    times.dedup_by(|left, right| left.total_cmp(right).is_eq());
    let mut cumulative_hazard = 0.0;
    BaselineCurves::Shared(
        times
            .into_iter()
            .map(|time| {
                let mut death_weight = 0.0;
                let mut risk = 0.0;
                for row in 0..data.rows() {
                    if data.starts()[row] < time && time <= data.stops()[row] {
                        risk += partial[row];
                        if data.events()[row] == Event::Observed && data.stops()[row] == time {
                            death_weight += data.weights()[row];
                        }
                    }
                }
                let hazard = death_weight / risk;
                cumulative_hazard += hazard;
                BaselineEstimate {
                    time,
                    hazard,
                    cumulative_hazard,
                    survival: (-cumulative_hazard).exp(),
                }
            })
            .collect(),
    )
}
