//! Parametric versus Aalen-Johansen state occupancy, following `ajfit_fmsm`.

use super::graph::StateId;
use super::markov::{MarkovControl, MarkovError};
use super::model::{ClockForward, MultiStateModel};
use crate::survival::flexsurv::observation::SurvivalObservation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransitionHistoryRow {
    pub observation: SurvivalObservation,
}

impl TransitionHistoryRow {
    pub fn new(observation: SurvivalObservation) -> Self {
        Self { observation }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AalenJohansenData {
    by_transition: Vec<Vec<TransitionHistoryRow>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OccupancyModel {
    AalenJohansen,
    Parametric,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OccupancyEstimate {
    pub time: f64,
    pub model: OccupancyModel,
    pub state: StateId,
    pub probability: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AjFitResult {
    pub estimates: Vec<OccupancyEstimate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StratumLabel(String);

#[derive(Clone, Debug, PartialEq)]
pub struct AalenJohansenStratum {
    label: StratumLabel,
    model: MultiStateModel<ClockForward>,
    data: AalenJohansenData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StratifiedAjFitResult {
    pub estimates: Vec<(String, OccupancyEstimate)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AjFitError {
    TransitionCount { expected: usize, actual: usize },
    EmptyTransition { transition: usize },
    UnsupportedCensoring { transition: usize, row: usize },
    InvalidMaximumTime,
    EmptyStratumLabel,
    NoStrata,
    DuplicateStratumLabel { stratum: usize },
    IncompatibleStratumGraph { stratum: usize },
    Markov(MarkovError),
}

impl StratumLabel {
    pub fn new(value: impl Into<String>) -> Result<Self, AjFitError> {
        let value = value.into();
        if value.trim().is_empty() {
            Err(AjFitError::EmptyStratumLabel)
        } else {
            Ok(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AalenJohansenStratum {
    pub fn new(
        label: StratumLabel,
        model: MultiStateModel<ClockForward>,
        data: AalenJohansenData,
    ) -> Self {
        Self { label, model, data }
    }
}

pub fn compare_factor_strata(
    strata: Vec<AalenJohansenStratum>,
    maximum_parametric_time: f64,
    control: MarkovControl,
) -> Result<StratifiedAjFitResult, AjFitError> {
    if strata.is_empty() {
        return Err(AjFitError::NoStrata);
    }
    let reference_states = strata[0]
        .model
        .graph()
        .states()
        .map(|state| strata[0].model.graph().state_name(state).to_owned())
        .collect::<Vec<_>>();
    let reference_transitions = strata[0]
        .model
        .graph()
        .transitions()
        .iter()
        .map(|transition| (transition.from.index(), transition.to.index()))
        .collect::<Vec<_>>();
    let mut labels = Vec::<&str>::new();
    for (index, stratum) in strata.iter().enumerate() {
        if labels.contains(&stratum.label.as_str()) {
            return Err(AjFitError::DuplicateStratumLabel { stratum: index });
        }
        labels.push(stratum.label.as_str());
        let states = stratum
            .model
            .graph()
            .states()
            .map(|state| stratum.model.graph().state_name(state).to_owned())
            .collect::<Vec<_>>();
        let transitions = stratum
            .model
            .graph()
            .transitions()
            .iter()
            .map(|transition| (transition.from.index(), transition.to.index()))
            .collect::<Vec<_>>();
        if states != reference_states || transitions != reference_transitions {
            return Err(AjFitError::IncompatibleStratumGraph { stratum: index });
        }
    }

    let mut estimates = Vec::new();
    for stratum in strata {
        let result = stratum.model.compare_with_aalen_johansen(
            stratum.data,
            maximum_parametric_time,
            control,
        )?;
        estimates.extend(
            result
                .estimates
                .into_iter()
                .map(|estimate| (stratum.label.0.clone(), estimate)),
        );
    }
    Ok(StratifiedAjFitResult { estimates })
}

impl From<MarkovError> for AjFitError {
    fn from(value: MarkovError) -> Self {
        Self::Markov(value)
    }
}

impl AalenJohansenData {
    pub fn new(
        model: &MultiStateModel<ClockForward>,
        by_transition: Vec<Vec<TransitionHistoryRow>>,
    ) -> Result<Self, AjFitError> {
        let expected = model.graph().transition_count();
        if by_transition.len() != expected {
            return Err(AjFitError::TransitionCount {
                expected,
                actual: by_transition.len(),
            });
        }
        for (transition, rows) in by_transition.iter().enumerate() {
            if rows.is_empty() {
                return Err(AjFitError::EmptyTransition { transition });
            }
            for (row, value) in rows.iter().enumerate() {
                let bounds = value.observation.bounds();
                if bounds.entry != 0.0 || (!bounds.exact && bounds.upper.is_finite()) {
                    return Err(AjFitError::UnsupportedCensoring { transition, row });
                }
            }
        }
        Ok(Self { by_transition })
    }
}

fn identity(order: usize) -> Vec<f64> {
    let mut value = vec![0.0; order * order];
    for index in 0..order {
        value[index * order + index] = 1.0;
    }
    value
}

fn multiply(left: &[f64], right: &[f64], order: usize) -> Vec<f64> {
    let mut value = vec![0.0; order * order];
    for row in 0..order {
        for column in 0..order {
            value[row * order + column] = (0..order)
                .map(|inner| left[row * order + inner] * right[inner * order + column])
                .sum();
        }
    }
    value
}

impl MultiStateModel<ClockForward> {
    /// The numerical content of `ajfit_fmsm` for an unstratified model.
    ///
    /// Each vector in `data` is the event/censoring history used to fit the
    /// correspondingly numbered transition model. This mirrors the source's
    /// stacked transition-specific Cox model without retaining an R model
    /// frame inside the fitted Rust object.
    pub fn compare_with_aalen_johansen(
        &self,
        data: AalenJohansenData,
        maximum_parametric_time: f64,
        control: MarkovControl,
    ) -> Result<AjFitResult, AjFitError> {
        if !maximum_parametric_time.is_finite() || maximum_parametric_time < 0.0 {
            return Err(AjFitError::InvalidMaximumTime);
        }

        let mut event_times = data
            .by_transition
            .iter()
            .flatten()
            .filter_map(|row| {
                let bounds = row.observation.bounds();
                bounds.exact.then_some(bounds.lower)
            })
            .collect::<Vec<_>>();
        event_times.sort_by(f64::total_cmp);
        // `coxph` applies survival's `aeqSurv` time fix before building risk
        // sets. Values that differ only by floating-point construction noise
        // therefore form one event time.
        event_times.dedup_by(|left, right| {
            (*left - *right).abs() <= f64::EPSILON.sqrt() * left.abs().max(right.abs()).max(1.0)
        });

        let states = self.graph().state_count();
        let mut product = identity(states);
        let mut estimates = Vec::with_capacity((event_times.len() + 100) * states);
        for state in self.graph().states() {
            estimates.push(OccupancyEstimate {
                time: 0.0,
                model: OccupancyModel::AalenJohansen,
                state,
                probability: if state.index() == 0 { 1.0 } else { 0.0 },
            });
        }

        for time in event_times {
            let mut increment = identity(states);
            for transition in self.graph().transitions() {
                let rows = &data.by_transition[transition.id.index()];
                let risk = rows
                    .iter()
                    .filter(|row| row.observation.bounds().lower >= time)
                    .count();
                let events = rows
                    .iter()
                    .filter(|row| {
                        let bounds = row.observation.bounds();
                        bounds.exact
                            && (bounds.lower - time).abs()
                                <= f64::EPSILON.sqrt() * bounds.lower.abs().max(time.abs()).max(1.0)
                    })
                    .count();
                if events == 0 || risk == 0 {
                    continue;
                }
                // `ajfit_fmsm` calls `coxph` without overriding its Efron tie
                // method. With no covariates and unit weights its baseline
                // hazard increment is this sum, not the Breslow `d / risk`.
                let hazard = (0..events)
                    .map(|tied| 1.0 / (risk - tied) as f64)
                    .sum::<f64>();
                increment[transition.from.index() * states + transition.from.index()] -= hazard;
                increment[transition.from.index() * states + transition.to.index()] += hazard;
            }
            product = multiply(&product, &increment, states);
            for state in self.graph().states() {
                estimates.push(OccupancyEstimate {
                    time,
                    model: OccupancyModel::AalenJohansen,
                    state,
                    probability: product[state.index()],
                });
            }
        }

        let denominator = 99.0;
        let times = (0..100)
            .map(|index| maximum_parametric_time * index as f64 / denominator)
            .collect::<Vec<_>>();
        let parametric = self.transition_probabilities(times.clone(), control)?;
        for (time, probabilities) in times.into_iter().zip(parametric.probabilities) {
            for state in self.graph().states() {
                estimates.push(OccupancyEstimate {
                    time,
                    model: OccupancyModel::Parametric,
                    state,
                    probability: probabilities[state.index()],
                });
            }
        }
        Ok(AjFitResult { estimates })
    }
}
