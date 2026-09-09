//! Clock-reset path simulation used by flexsurv's `sim.fmsm` family.

use std::collections::BTreeSet;

use super::graph::StateId;
use super::model::{ClockReset, MultiStateModel, MultiStateModelError};
use crate::survival::flexsurv::distribution::DistributionError;
use crate::survival::flexsurv::uncertainty::{
    type_seven_quantile, PredictionSimulationConfiguration,
};
use crate::survival::r_rng::RRng;

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedPath {
    pub states: Vec<StateId>,
    pub times: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemiMarkovSemantics {
    FlexSurv232,
    Corrected,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SemiMarkovSimulation {
    pub horizons: Vec<f64>,
    pub paths: Vec<SimulatedPath>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedTransition {
    pub path: usize,
    pub from: StateId,
    pub to: StateId,
    pub time: f64,
    pub delay: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedProbabilityResult {
    pub times: Vec<f64>,
    pub probabilities: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedProbabilityIntervals {
    pub estimate: SimulatedProbabilityResult,
    pub lower: Vec<Vec<f64>>,
    pub upper: Vec<Vec<f64>>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupedLengthOfStay {
    pub groups: Vec<usize>,
    pub expected_time: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimulatedLengthOfStayIntervals {
    pub estimate: Vec<f64>,
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalStateSummary {
    pub state: StateId,
    /// Probability conditional on reaching any absorbing state by the horizon,
    /// matching `simfinal_fmsm`.
    pub conditional_probability: f64,
    pub mean_time: f64,
    pub quantiles: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalOutcomeResult {
    pub horizon: f64,
    pub absorbed_fraction: f64,
    pub probabilities: Vec<f64>,
    pub summaries: Vec<FinalStateSummary>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalStateIntervals {
    pub state: StateId,
    pub conditional_probability_lower: f64,
    pub conditional_probability_upper: f64,
    pub mean_time_lower: f64,
    pub mean_time_upper: f64,
    pub quantile_lower: Vec<f64>,
    pub quantile_upper: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalOutcomeIntervals {
    pub estimate: FinalOutcomeResult,
    pub intervals: Vec<FinalStateIntervals>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SemiMarkovError {
    NoPaths,
    InvalidHorizon,
    InvalidTime { index: usize },
    StartLength { expected: usize, actual: usize },
    HorizonLength { expected: usize, actual: usize },
    GroupLength { expected: usize, actual: usize },
    InvalidQuantile { index: usize },
    NoAbsorbingOutcome,
    StateOutsideGraph { state: usize },
    NonFiniteWaitingTime,
    Model(MultiStateModelError),
    Distribution(DistributionError),
}

impl From<MultiStateModelError> for SemiMarkovError {
    fn from(value: MultiStateModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for SemiMarkovError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl MultiStateModel<ClockReset> {
    fn simulate_with_rng(
        &self,
        horizons: &[f64],
        starts: &[StateId],
        semantics: SemiMarkovSemantics,
        rng: &mut RRng,
    ) -> Result<SemiMarkovSimulation, SemiMarkovError> {
        let paths = starts.len();
        let mut current_states = starts.to_vec();
        let mut current_times = vec![0.0; paths];
        let mut histories = starts
            .iter()
            .copied()
            .map(|start| SimulatedPath {
                states: vec![start],
                times: vec![0.0],
            })
            .collect::<Vec<_>>();
        let mut active = (0..paths).collect::<Vec<_>>();

        while !active.is_empty() {
            let states_before_step = current_states.clone();
            let times_before_step = current_times.clone();
            let mut active_states = Vec::new();
            for path in &active {
                let state = states_before_step[*path];
                if !active_states.contains(&state) {
                    active_states.push(state);
                }
            }
            let mut finished = BTreeSet::new();
            for state in active_states {
                if self.graph().is_absorbing(state) {
                    continue;
                }
                let members = active
                    .iter()
                    .copied()
                    .filter(|path| states_before_step[*path] == state)
                    .collect::<Vec<_>>();
                let outgoing = self.graph().outgoing(state);
                let mut waits = vec![vec![0.0; outgoing.len()]; members.len()];
                // `sim.fmsm` calls one vectorised r-function per transition.
                // Keeping this transition-major order is required for seeded
                // parity with R's global random stream.
                for (column, transition) in outgoing.iter().enumerate() {
                    for (row, path) in members.iter().copied().enumerate() {
                        let distribution =
                            self.transitions_distribution_at(*transition, times_before_step[path])?;
                        waits[row][column] = distribution.sample_r(rng)?;
                    }
                }
                let mut censored_in_state = BTreeSet::new();
                for (row, path) in members.iter().copied().enumerate() {
                    let (column, waiting) = waits[row]
                        .iter()
                        .copied()
                        .enumerate()
                        .min_by(|left, right| left.1.total_cmp(&right.1))
                        .expect("transient state has outgoing transition");
                    if waiting.is_nan() || waiting < 0.0 {
                        return Err(SemiMarkovError::NonFiniteWaitingTime);
                    }
                    let active_index = active
                        .iter()
                        .position(|candidate| *candidate == path)
                        .expect("a member belongs to the active set");
                    let horizon = match semantics {
                        SemiMarkovSemantics::FlexSurv232 => horizons[active_index],
                        SemiMarkovSemantics::Corrected => horizons[path],
                    };
                    if !waiting.is_finite() || current_times[path] + waiting > horizon {
                        current_times[path] = horizon;
                        censored_in_state.insert(path);
                    } else {
                        current_times[path] += waiting;
                        current_states[path] = self.graph().transition(outgoing[column]).to;
                    }
                }
                match semantics {
                    SemiMarkovSemantics::FlexSurv232 => finished = censored_in_state,
                    SemiMarkovSemantics::Corrected => finished.extend(censored_in_state),
                }
            }
            for path in 0..paths {
                histories[path].states.push(current_states[path]);
                histories[path].times.push(current_times[path]);
                if self.graph().is_absorbing(current_states[path]) {
                    finished.insert(path);
                }
            }
            active.retain(|path| !finished.contains(path));
        }

        Ok(SemiMarkovSimulation {
            horizons: horizons.to_vec(),
            paths: histories,
        })
    }

    pub fn simulate_paths(
        &self,
        horizon: f64,
        start: usize,
        paths: usize,
        seed: u32,
    ) -> Result<SemiMarkovSimulation, SemiMarkovError> {
        if paths == 0 {
            return Err(SemiMarkovError::NoPaths);
        }
        if !horizon.is_finite() || horizon < 0.0 {
            return Err(SemiMarkovError::InvalidHorizon);
        }
        let start = self
            .graph()
            .state(start)
            .ok_or(SemiMarkovError::StateOutsideGraph { state: start })?;
        self.simulate_with_rng(
            &vec![horizon; paths],
            &vec![start; paths],
            SemiMarkovSemantics::FlexSurv232,
            &mut RRng::new(seed),
        )
    }

    /// The vector form documented by `sim.fmsm`: one horizon and starting
    /// state per simulated individual.
    pub fn simulate_paths_for(
        &self,
        horizons: Vec<f64>,
        starts: Vec<usize>,
        seed: u32,
    ) -> Result<SemiMarkovSimulation, SemiMarkovError> {
        if starts.is_empty() {
            return Err(SemiMarkovError::NoPaths);
        }
        if horizons.len() != starts.len() {
            return Err(SemiMarkovError::HorizonLength {
                expected: starts.len(),
                actual: horizons.len(),
            });
        }
        for (index, horizon) in horizons.iter().enumerate() {
            if !horizon.is_finite() || *horizon < 0.0 {
                return Err(SemiMarkovError::InvalidTime { index });
            }
        }
        let starts = starts
            .into_iter()
            .map(|state| {
                self.graph()
                    .state(state)
                    .ok_or(SemiMarkovError::StateOutsideGraph { state })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.simulate_with_rng(
            &horizons,
            &starts,
            SemiMarkovSemantics::FlexSurv232,
            &mut RRng::new(seed),
        )
    }

    /// Corrects the two path-bookkeeping defects in flexsurv 2.3.2 while
    /// retaining its model and random-sampling semantics.
    pub fn simulate_paths_for_corrected(
        &self,
        horizons: Vec<f64>,
        starts: Vec<usize>,
        seed: u32,
    ) -> Result<SemiMarkovSimulation, SemiMarkovError> {
        if starts.is_empty() {
            return Err(SemiMarkovError::NoPaths);
        }
        if horizons.len() != starts.len() {
            return Err(SemiMarkovError::HorizonLength {
                expected: starts.len(),
                actual: horizons.len(),
            });
        }
        for (index, horizon) in horizons.iter().enumerate() {
            if !horizon.is_finite() || *horizon < 0.0 {
                return Err(SemiMarkovError::InvalidTime { index });
            }
        }
        let starts = starts
            .into_iter()
            .map(|state| {
                self.graph()
                    .state(state)
                    .ok_or(SemiMarkovError::StateOutsideGraph { state })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.simulate_with_rng(
            &horizons,
            &starts,
            SemiMarkovSemantics::Corrected,
            &mut RRng::new(seed),
        )
    }

    /// `simfs_bytrans`: omit padded states and censoring rows, retaining one
    /// record for every transition that actually occurred.
    pub fn transitions_by_path(simulation: &SemiMarkovSimulation) -> Vec<SimulatedTransition> {
        let mut rows = Vec::new();
        for (path_index, path) in simulation.paths.iter().enumerate() {
            for index in 1..path.states.len() {
                if path.states[index] == path.states[index - 1] {
                    continue;
                }
                rows.push(SimulatedTransition {
                    path: path_index,
                    from: path.states[index - 1],
                    to: path.states[index],
                    time: path.times[index],
                    delay: path.times[index] - path.times[index - 1],
                });
            }
        }
        rows.sort_by_key(|row| row.path);
        rows
    }

    pub fn simulated_transition_probabilities(
        &self,
        times: Vec<f64>,
        paths_per_start: usize,
        seed: u32,
    ) -> Result<SimulatedProbabilityResult, SemiMarkovError> {
        self.simulated_transition_probabilities_with_rng(
            times,
            paths_per_start,
            &mut RRng::new(seed),
        )
    }

    fn simulated_transition_probabilities_with_rng(
        &self,
        times: Vec<f64>,
        paths_per_start: usize,
        rng: &mut RRng,
    ) -> Result<SimulatedProbabilityResult, SemiMarkovError> {
        if paths_per_start == 0 {
            return Err(SemiMarkovError::NoPaths);
        }
        if times.is_empty() {
            return Err(SemiMarkovError::InvalidTime { index: 0 });
        }
        for (index, time) in times.iter().enumerate() {
            if !time.is_finite() || *time < 0.0 {
                return Err(SemiMarkovError::InvalidTime { index });
            }
        }
        let horizon = times.iter().copied().fold(0.0, f64::max);
        let n = self.graph().state_count();
        let mut counts = vec![vec![0_usize; n * n]; times.len()];
        // flexsurv advances one global R stream while simulating each starting
        // state. Derive the same uninterrupted stream by simulating here.
        for start in self.graph().states() {
            let simulation = self.simulate_with_rng(
                &vec![horizon; paths_per_start],
                &vec![start; paths_per_start],
                SemiMarkovSemantics::FlexSurv232,
                rng,
            )?;
            for path in simulation.paths {
                for (time_index, time) in times.iter().enumerate() {
                    let position = path.times.partition_point(|change| change <= time);
                    let occupied = path.states[position.saturating_sub(1)];
                    counts[time_index][start.index() * n + occupied.index()] += 1;
                }
            }
        }
        let probabilities = counts
            .into_iter()
            .map(|matrix| {
                matrix
                    .into_iter()
                    .map(|count| count as f64 / paths_per_start as f64)
                    .collect()
            })
            .collect();
        Ok(SimulatedProbabilityResult {
            times,
            probabilities,
        })
    }

    fn simulated_length_of_stay_with_rng(
        &self,
        horizon: f64,
        start: StateId,
        paths: usize,
        rng: &mut RRng,
    ) -> Result<Vec<f64>, SemiMarkovError> {
        if paths == 0 {
            return Err(SemiMarkovError::NoPaths);
        }
        let simulation = self.simulate_with_rng(
            &vec![horizon; paths],
            &vec![start; paths],
            SemiMarkovSemantics::FlexSurv232,
            rng,
        )?;
        let mut totals = vec![0.0; self.graph().state_count()];
        for path in simulation.paths {
            for interval in 0..path.states.len() {
                let begin = path.times[interval];
                let end = path.times.get(interval + 1).copied().unwrap_or(horizon);
                totals[path.states[interval].index()] += end - begin;
            }
        }
        for total in &mut totals {
            *total /= paths as f64;
        }
        Ok(totals)
    }

    pub fn simulated_length_of_stay(
        &self,
        horizon: f64,
        start: usize,
        paths: usize,
        seed: u32,
    ) -> Result<Vec<f64>, SemiMarkovError> {
        if !horizon.is_finite() || horizon < 0.0 {
            return Err(SemiMarkovError::InvalidHorizon);
        }
        let start = self
            .graph()
            .state(start)
            .ok_or(SemiMarkovError::StateOutsideGraph { state: start })?;
        self.simulated_length_of_stay_with_rng(horizon, start, paths, &mut RRng::new(seed))
    }

    pub fn grouped_length_of_stay(
        &self,
        horizon: f64,
        start: usize,
        paths: usize,
        seed: u32,
        state_groups: Vec<usize>,
    ) -> Result<GroupedLengthOfStay, SemiMarkovError> {
        let expected = self.graph().state_count();
        if state_groups.len() != expected {
            return Err(SemiMarkovError::GroupLength {
                expected,
                actual: state_groups.len(),
            });
        }
        let by_state = self.simulated_length_of_stay(horizon, start, paths, seed)?;
        let mut groups = state_groups.clone();
        groups.sort_unstable();
        groups.dedup();
        let expected_time = groups
            .iter()
            .map(|group| {
                by_state
                    .iter()
                    .zip(&state_groups)
                    .filter(|(_, assigned)| *assigned == group)
                    .map(|(time, _)| *time)
                    .sum()
            })
            .collect();
        Ok(GroupedLengthOfStay {
            groups,
            expected_time,
        })
    }

    pub fn simulated_transition_probability_intervals(
        &self,
        times: Vec<f64>,
        paths_per_start: usize,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<SimulatedProbabilityIntervals, SemiMarkovError> {
        let mut rng = RRng::new(configuration.seed());
        let estimate = self.simulated_transition_probabilities_with_rng(
            times.clone(),
            paths_per_start,
            &mut rng,
        )?;
        let models = self.simulated_transition_models_with_rng(configuration, &mut rng)?;
        let cells = self.graph().state_count() * self.graph().state_count();
        let mut draws = vec![vec![Vec::with_capacity(configuration.draws()); cells]; times.len()];
        for transitions in models {
            let model = MultiStateModel::clock_reset(self.graph().clone(), transitions)?;
            let result = model.simulated_transition_probabilities_with_rng(
                times.clone(),
                paths_per_start,
                &mut rng,
            )?;
            for (time_index, matrix) in result.probabilities.into_iter().enumerate() {
                for (cell, value) in matrix.into_iter().enumerate() {
                    draws[time_index][cell].push(value);
                }
            }
        }
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        let mut lower = vec![vec![0.0; cells]; times.len()];
        let mut upper = vec![vec![0.0; cells]; times.len()];
        for time_index in 0..times.len() {
            for cell in 0..cells {
                draws[time_index][cell].sort_by(f64::total_cmp);
                lower[time_index][cell] = type_seven_quantile(&draws[time_index][cell], tail);
                upper[time_index][cell] = type_seven_quantile(&draws[time_index][cell], 1.0 - tail);
            }
        }
        Ok(SimulatedProbabilityIntervals {
            estimate,
            lower,
            upper,
            confidence_level: configuration.confidence_level(),
        })
    }

    pub fn simulated_length_of_stay_intervals(
        &self,
        horizon: f64,
        start: usize,
        paths: usize,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<SimulatedLengthOfStayIntervals, SemiMarkovError> {
        if !horizon.is_finite() || horizon < 0.0 {
            return Err(SemiMarkovError::InvalidHorizon);
        }
        let start = self
            .graph()
            .state(start)
            .ok_or(SemiMarkovError::StateOutsideGraph { state: start })?;
        let mut rng = RRng::new(configuration.seed());
        let estimate = self.simulated_length_of_stay_with_rng(horizon, start, paths, &mut rng)?;
        let models = self.simulated_transition_models_with_rng(configuration, &mut rng)?;
        let mut draws = vec![Vec::with_capacity(configuration.draws()); self.graph().state_count()];
        for transitions in models {
            let model = MultiStateModel::clock_reset(self.graph().clone(), transitions)?;
            let result =
                model.simulated_length_of_stay_with_rng(horizon, start, paths, &mut rng)?;
            for (state, value) in result.into_iter().enumerate() {
                draws[state].push(value);
            }
        }
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        let mut lower = Vec::with_capacity(draws.len());
        let mut upper = Vec::with_capacity(draws.len());
        for values in &mut draws {
            values.sort_by(f64::total_cmp);
            lower.push(type_seven_quantile(values, tail));
            upper.push(type_seven_quantile(values, 1.0 - tail));
        }
        Ok(SimulatedLengthOfStayIntervals {
            estimate,
            lower,
            upper,
            confidence_level: configuration.confidence_level(),
        })
    }

    pub fn final_outcomes(
        &self,
        horizon: f64,
        paths: usize,
        seed: u32,
        probabilities: Vec<f64>,
    ) -> Result<FinalOutcomeResult, SemiMarkovError> {
        self.final_outcomes_with_rng(horizon, paths, probabilities, &mut RRng::new(seed))
    }

    fn final_outcomes_with_rng(
        &self,
        horizon: f64,
        paths: usize,
        probabilities: Vec<f64>,
        rng: &mut RRng,
    ) -> Result<FinalOutcomeResult, SemiMarkovError> {
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || !(0.0..=1.0).contains(probability) {
                return Err(SemiMarkovError::InvalidQuantile { index });
            }
        }
        if paths == 0 {
            return Err(SemiMarkovError::NoPaths);
        }
        if !horizon.is_finite() || horizon < 0.0 {
            return Err(SemiMarkovError::InvalidHorizon);
        }
        let start = self
            .graph()
            .state(0)
            .expect("a graph has at least one state");
        let simulation = self.simulate_with_rng(
            &vec![horizon; paths],
            &vec![start; paths],
            SemiMarkovSemantics::FlexSurv232,
            rng,
        )?;
        let absorbing = self
            .graph()
            .states()
            .filter(|state| self.graph().is_absorbing(*state))
            .collect::<Vec<_>>();
        if absorbing.is_empty() {
            return Err(SemiMarkovError::NoAbsorbingOutcome);
        }
        let mut by_state = vec![Vec::new(); self.graph().state_count()];
        for path in simulation.paths {
            let final_state = *path.states.last().expect("a path contains its start state");
            if self.graph().is_absorbing(final_state) {
                by_state[final_state.index()]
                    .push(*path.times.last().expect("a path contains its start time"));
            }
        }
        let absorbed = by_state.iter().map(Vec::len).sum::<usize>();
        if absorbed == 0 {
            return Err(SemiMarkovError::NoAbsorbingOutcome);
        }
        let summaries = absorbing
            .into_iter()
            .map(|state| {
                let values = &mut by_state[state.index()];
                values.sort_by(f64::total_cmp);
                let mean_time = if values.is_empty() {
                    f64::NAN
                } else {
                    values.iter().sum::<f64>() / values.len() as f64
                };
                let quantiles = if values.is_empty() {
                    vec![f64::NAN; probabilities.len()]
                } else {
                    probabilities
                        .iter()
                        .map(|probability| type_seven_quantile(values, *probability))
                        .collect()
                };
                FinalStateSummary {
                    state,
                    conditional_probability: values.len() as f64 / absorbed as f64,
                    mean_time,
                    quantiles,
                }
            })
            .collect();
        Ok(FinalOutcomeResult {
            horizon,
            absorbed_fraction: absorbed as f64 / paths as f64,
            probabilities,
            summaries,
        })
    }

    pub fn final_outcome_intervals(
        &self,
        horizon: f64,
        paths: usize,
        probabilities: Vec<f64>,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<FinalOutcomeIntervals, SemiMarkovError> {
        let mut rng = RRng::new(configuration.seed());
        let estimate =
            self.final_outcomes_with_rng(horizon, paths, probabilities.clone(), &mut rng)?;
        let models = self.simulated_transition_models_with_rng(configuration, &mut rng)?;
        let mut probability_draws = vec![Vec::new(); estimate.summaries.len()];
        let mut mean_draws = probability_draws.clone();
        let mut quantile_draws =
            vec![vec![Vec::new(); probabilities.len()]; estimate.summaries.len()];
        for transitions in models {
            let model = MultiStateModel::clock_reset(self.graph().clone(), transitions)?;
            let candidate =
                model.final_outcomes_with_rng(horizon, paths, probabilities.clone(), &mut rng)?;
            for (state_index, summary) in candidate.summaries.into_iter().enumerate() {
                probability_draws[state_index].push(summary.conditional_probability);
                mean_draws[state_index].push(summary.mean_time);
                for (quantile, value) in summary.quantiles.into_iter().enumerate() {
                    quantile_draws[state_index][quantile].push(value);
                }
            }
        }
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        let bounds = |values: &mut Vec<f64>| {
            values.sort_by(f64::total_cmp);
            (
                type_seven_quantile(values, tail),
                type_seven_quantile(values, 1.0 - tail),
            )
        };
        let mut intervals = Vec::with_capacity(estimate.summaries.len());
        for (state_index, summary) in estimate.summaries.iter().enumerate() {
            let (conditional_probability_lower, conditional_probability_upper) =
                bounds(&mut probability_draws[state_index]);
            let (mean_time_lower, mean_time_upper) = bounds(&mut mean_draws[state_index]);
            let (quantile_lower, quantile_upper): (Vec<_>, Vec<_>) =
                quantile_draws[state_index].iter_mut().map(bounds).unzip();
            intervals.push(FinalStateIntervals {
                state: summary.state,
                conditional_probability_lower,
                conditional_probability_upper,
                mean_time_lower,
                mean_time_upper,
                quantile_lower,
                quantile_upper,
            });
        }
        Ok(FinalOutcomeIntervals {
            estimate,
            intervals,
            confidence_level: configuration.confidence_level(),
        })
    }
}
