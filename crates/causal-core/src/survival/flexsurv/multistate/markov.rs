//! Clock-forward prediction from flexsurv's Kolmogorov equations.

use nalgebra::DMatrix;

use super::graph::StateId;
use super::model::{ClockForward, MultiStateModel, MultiStateModelError};
use crate::survival::flexsurv::distribution::DistributionError;
use crate::survival::flexsurv::uncertainty::{
    type_seven_quantile, PredictionSimulationConfiguration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkovIntegrator {
    /// Exact for a time-homogeneous intensity matrix; used as the independent
    /// check recommended in the flexsurv examples for exponential fits.
    ConstantMatrixExponential,
    /// Classical RK4, matching `pmatrix.fs(..., method = "rk4")` and
    /// `totlos.fs(..., method = "rk4")` through deSolve 1.42.
    DeSolveRk4,
    /// deSolve 1.42's `rk45dp7` adaptive Prince-Dormand 5(4) recipe.
    DeSolveRk45Dp7,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MarkovControl {
    pub integrator: MarkovIntegrator,
    pub singular_hazard: f64,
}

impl MarkovControl {
    pub fn matrix_exponential() -> Self {
        Self {
            integrator: MarkovIntegrator::ConstantMatrixExponential,
            singular_hazard: 1e10,
        }
    }

    pub fn de_solve_rk4(singular_hazard: f64) -> Result<Self, MarkovError> {
        if !singular_hazard.is_finite() || singular_hazard <= 0.0 {
            return Err(MarkovError::InvalidSingularHazard);
        }
        Ok(Self {
            integrator: MarkovIntegrator::DeSolveRk4,
            singular_hazard,
        })
    }

    pub fn de_solve_rk45_dp7(singular_hazard: f64) -> Result<Self, MarkovError> {
        if !singular_hazard.is_finite() || singular_hazard <= 0.0 {
            return Err(MarkovError::InvalidSingularHazard);
        }
        Ok(Self {
            integrator: MarkovIntegrator::DeSolveRk45Dp7,
            singular_hazard,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionProbabilityResult {
    pub times: Vec<f64>,
    /// One row-major state-by-state matrix per requested time.
    pub probabilities: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CumulativeHazardCovarianceResult {
    pub times: Vec<f64>,
    /// Transition-major cumulative hazards, matching `msfit.flexsurvreg$Haz`.
    pub cumulative_hazards: Vec<Vec<f64>>,
    /// One row-major transition-by-transition covariance matrix per time.
    pub covariance: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LengthOfStayResult {
    pub times: Vec<f64>,
    /// One row-major matrix per time: starting state by occupied state.
    pub length_of_stay: Vec<Vec<f64>>,
    pub probabilities: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalProbabilityResult {
    pub times: Vec<f64>,
    pub destination_states: Vec<StateId>,
    pub probabilities: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionProbabilityIntervals {
    pub estimate: TransitionProbabilityResult,
    pub lower: Vec<Vec<f64>>,
    pub upper: Vec<Vec<f64>>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LengthOfStayIntervals {
    pub estimate: LengthOfStayResult,
    pub length_lower: Vec<Vec<f64>>,
    pub length_upper: Vec<Vec<f64>>,
    pub probability_lower: Vec<Vec<f64>>,
    pub probability_upper: Vec<Vec<f64>>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalStateProbabilityResult {
    pub start: StateId,
    pub destination_states: Vec<StateId>,
    pub probabilities: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalStateProbabilityIntervals {
    pub estimate: FinalStateProbabilityResult,
    pub lower: Vec<f64>,
    pub upper: Vec<f64>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompetingRisksModel {
    model: MultiStateModel<ClockForward>,
    start: StateId,
    destinations: Vec<StateId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MarkovError {
    EmptyTimes,
    InvalidTime { index: usize },
    TimesNotIncreasing { index: usize },
    InvalidSingularHazard,
    MatrixExponentialRequiresConstantHazards,
    EmptyConditioningStates,
    DuplicateConditioningState { state: usize },
    ZeroConditioningProbability { time: usize, start: usize },
    IntegrationStepLimit,
    NoDestinationStates { state: usize },
    DestinationIsNotAbsorbing { state: usize },
    OtherTransientState { state: usize },
    Model(MultiStateModelError),
    Distribution(DistributionError),
}

impl CompetingRisksModel {
    pub fn new(model: MultiStateModel<ClockForward>, start: usize) -> Result<Self, MarkovError> {
        let start = model
            .graph()
            .state(start)
            .ok_or(MarkovError::NoDestinationStates { state: start })?;
        let destinations = model
            .graph()
            .outgoing(start)
            .iter()
            .map(|transition| model.graph().transition(*transition).to)
            .collect::<Vec<_>>();
        if destinations.is_empty() {
            return Err(MarkovError::NoDestinationStates {
                state: start.index(),
            });
        }
        for destination in &destinations {
            if !model.graph().is_absorbing(*destination) {
                return Err(MarkovError::DestinationIsNotAbsorbing {
                    state: destination.index(),
                });
            }
        }
        for state in model.graph().states() {
            if state != start && !model.graph().is_absorbing(state) {
                return Err(MarkovError::OtherTransientState {
                    state: state.index(),
                });
            }
        }
        Ok(Self {
            model,
            start,
            destinations,
        })
    }

    pub fn final_state_probabilities(
        &self,
        horizon: f64,
        control: MarkovControl,
    ) -> Result<FinalStateProbabilityResult, MarkovError> {
        let conditional = self.model.conditional_transition_probabilities(
            vec![horizon],
            self.destinations.clone(),
            control,
        )?;
        let width = self.destinations.len();
        let offset = self.start.index() * width;
        Ok(FinalStateProbabilityResult {
            start: self.start,
            destination_states: self.destinations.clone(),
            probabilities: conditional.probabilities[0][offset..offset + width].to_vec(),
        })
    }

    pub fn final_state_probability_intervals(
        &self,
        horizon: f64,
        control: MarkovControl,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<FinalStateProbabilityIntervals, MarkovError> {
        let estimate = self.final_state_probabilities(horizon, control)?;
        let mut draws = vec![Vec::with_capacity(configuration.draws()); self.destinations.len()];
        for transitions in self.model.simulated_transition_models(configuration)? {
            let model = MultiStateModel::clock_forward(self.model.graph().clone(), transitions)?;
            let candidate = CompetingRisksModel::new(model, self.start.index())?
                .final_state_probabilities(horizon, control)?;
            for (state, value) in candidate.probabilities.into_iter().enumerate() {
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
        Ok(FinalStateProbabilityIntervals {
            estimate,
            lower,
            upper,
            confidence_level: configuration.confidence_level(),
        })
    }
}

impl From<MultiStateModelError> for MarkovError {
    fn from(value: MultiStateModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for MarkovError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

fn validate_times(times: &[f64]) -> Result<(), MarkovError> {
    if times.is_empty() {
        return Err(MarkovError::EmptyTimes);
    }
    for (index, time) in times.iter().enumerate() {
        if !time.is_finite() || *time < 0.0 {
            return Err(MarkovError::InvalidTime { index });
        }
        if index > 0 && *time < times[index - 1] {
            return Err(MarkovError::TimesNotIncreasing { index });
        }
    }
    Ok(())
}

fn identity(order: usize) -> Vec<f64> {
    let mut matrix = vec![0.0; order * order];
    for index in 0..order {
        matrix[index * order + index] = 1.0;
    }
    matrix
}

fn intensity(
    model: &MultiStateModel<ClockForward>,
    time: f64,
    singular_hazard: f64,
) -> Result<Vec<f64>, MarkovError> {
    let n = model.graph().state_count();
    let mut q = vec![0.0; n * n];
    for transition in model.graph().transitions() {
        let mut hazard = model
            .transition_distribution(transition.id)?
            .evaluate(time)?
            .hazard;
        if hazard == f64::INFINITY {
            hazard = singular_hazard;
        }
        q[transition.from.index() * n + transition.to.index()] = hazard;
    }
    for row in 0..n {
        let outgoing = (0..n)
            .filter(|column| *column != row)
            .map(|column| q[row * n + column])
            .sum::<f64>();
        q[row * n + row] = -outgoing;
    }
    Ok(q)
}

fn multiply(left: &[f64], right: &[f64], order: usize) -> Vec<f64> {
    let mut result = vec![0.0; order * order];
    for row in 0..order {
        for column in 0..order {
            let mut value = 0.0;
            for inner in 0..order {
                value += left[row * order + inner] * right[inner * order + column];
            }
            result[row * order + column] = value;
        }
    }
    result
}

fn derivative(
    model: &MultiStateModel<ClockForward>,
    time: f64,
    state: &[f64],
    singular_hazard: f64,
    with_lengths: bool,
) -> Result<Vec<f64>, MarkovError> {
    let n = model.graph().state_count();
    let size = n * n;
    let q = intensity(model, time, singular_hazard)?;
    if with_lengths {
        let probabilities = &state[size..];
        let probability_derivative = multiply(probabilities, &q, n);
        let mut result = Vec::with_capacity(size * 2);
        result.extend_from_slice(probabilities);
        result.extend(probability_derivative);
        Ok(result)
    } else {
        Ok(multiply(state, &q, n))
    }
}

fn add_scaled(state: &[f64], derivative: &[f64], scale: f64) -> Vec<f64> {
    state
        .iter()
        .zip(derivative)
        .map(|(value, change)| value + scale * change)
        .collect()
}

fn rk4_step(
    model: &MultiStateModel<ClockForward>,
    start: f64,
    end: f64,
    state: &[f64],
    singular_hazard: f64,
    with_lengths: bool,
) -> Result<Vec<f64>, MarkovError> {
    let step = end - start;
    let k1 = derivative(model, start, state, singular_hazard, with_lengths)?;
    let k2 = derivative(
        model,
        start + step / 2.0,
        &add_scaled(state, &k1, step / 2.0),
        singular_hazard,
        with_lengths,
    )?;
    let k3 = derivative(
        model,
        start + step / 2.0,
        &add_scaled(state, &k2, step / 2.0),
        singular_hazard,
        with_lengths,
    )?;
    let k4 = derivative(
        model,
        end,
        &add_scaled(state, &k3, step),
        singular_hazard,
        with_lengths,
    )?;
    Ok((0..state.len())
        .map(|index| {
            state[index] + step / 6.0 * (k1[index] + 2.0 * k2[index] + 2.0 * k3[index] + k4[index])
        })
        .collect())
}

fn rk4_series(
    model: &MultiStateModel<ClockForward>,
    times: &[f64],
    singular_hazard: f64,
    with_lengths: bool,
) -> Result<Vec<Vec<f64>>, MarkovError> {
    let n = model.graph().state_count();
    let size = n * n;
    let mut state = if with_lengths {
        let mut state = vec![0.0; size];
        state.extend(identity(n));
        state
    } else {
        identity(n)
    };
    let mut previous = 0.0;
    let mut result = Vec::with_capacity(times.len());
    for time in times {
        state = rk4_step(
            model,
            previous,
            *time,
            &state,
            singular_hazard,
            with_lengths,
        )?;
        result.push(state.clone());
        previous = *time;
    }
    Ok(result)
}

fn rk45_step(
    model: &MultiStateModel<ClockForward>,
    time: f64,
    step: f64,
    state: &[f64],
    singular_hazard: f64,
    with_lengths: bool,
) -> Result<(Vec<f64>, Vec<f64>), MarkovError> {
    const C: [f64; 7] = [0.0, 1.0 / 5.0, 3.0 / 10.0, 4.0 / 5.0, 8.0 / 9.0, 1.0, 1.0];
    const A: [[f64; 6]; 7] = [
        [0.0; 6],
        [1.0 / 5.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [3.0 / 40.0, 9.0 / 40.0, 0.0, 0.0, 0.0, 0.0],
        [44.0 / 45.0, -56.0 / 15.0, 32.0 / 9.0, 0.0, 0.0, 0.0],
        [
            19372.0 / 6561.0,
            -25360.0 / 2187.0,
            64448.0 / 6561.0,
            -212.0 / 729.0,
            0.0,
            0.0,
        ],
        [
            9017.0 / 3168.0,
            -355.0 / 33.0,
            46732.0 / 5247.0,
            49.0 / 176.0,
            -5103.0 / 18656.0,
            0.0,
        ],
        [
            35.0 / 384.0,
            0.0,
            500.0 / 1113.0,
            125.0 / 192.0,
            -2187.0 / 6784.0,
            11.0 / 84.0,
        ],
    ];
    const B_LOW: [f64; 7] = [
        5179.0 / 57600.0,
        0.0,
        7571.0 / 16695.0,
        393.0 / 640.0,
        -92097.0 / 339200.0,
        187.0 / 2100.0,
        1.0 / 40.0,
    ];
    const B_HIGH: [f64; 7] = [
        35.0 / 384.0,
        0.0,
        500.0 / 1113.0,
        125.0 / 192.0,
        -2187.0 / 6784.0,
        11.0 / 84.0,
        0.0,
    ];
    let mut stages = Vec::<Vec<f64>>::with_capacity(7);
    for stage in 0..7 {
        let mut intermediate = state.to_vec();
        for previous in 0..stage {
            for index in 0..state.len() {
                intermediate[index] += step * A[stage][previous] * stages[previous][index];
            }
        }
        stages.push(derivative(
            model,
            time + step * C[stage],
            &intermediate,
            singular_hazard,
            with_lengths,
        )?);
    }
    let combine = |weights: &[f64; 7]| {
        (0..state.len())
            .map(|index| {
                state[index]
                    + step
                        * (0..7)
                            .map(|stage| weights[stage] * stages[stage][index])
                            .sum::<f64>()
            })
            .collect::<Vec<_>>()
    };
    Ok((combine(&B_LOW), combine(&B_HIGH)))
}

fn rk45_series(
    model: &MultiStateModel<ClockForward>,
    times: &[f64],
    singular_hazard: f64,
    with_lengths: bool,
) -> Result<Vec<Vec<f64>>, MarkovError> {
    let n = model.graph().state_count();
    let size = n * n;
    let mut state = if with_lengths {
        let mut state = vec![0.0; size];
        state.extend(identity(n));
        state
    } else {
        identity(n)
    };
    let mut previous = 0.0;
    let mut result = Vec::with_capacity(times.len());
    for target in times {
        let mut time = previous;
        let mut step = target - previous;
        let mut previously_accepted = false;
        let mut attempts = 0usize;
        while time < *target - 100.0 * f64::EPSILON * step {
            attempts += 1;
            if attempts > 5000 {
                return Err(MarkovError::IntegrationStepLimit);
            }
            let (low, high) = rk45_step(model, time, step, &state, singular_hazard, with_lengths)?;
            let error = state
                .iter()
                .zip(&low)
                .zip(&high)
                .map(|((before, low), high)| {
                    let scale = 1e-6 + before.abs().max(high.abs()) * 1e-6;
                    ((high - low) / scale).powi(2)
                })
                .sum::<f64>()
                / state.len() as f64;
            let error = error.sqrt();
            let next_step;
            if error == 0.0 {
                next_step = (step * 10.0).min(*target - time);
                previously_accepted = true;
            } else if error < 1.0 {
                next_step = if previously_accepted {
                    (step * (0.9 * error.powf(-0.25)).min(10.0)).min(*target - time)
                } else {
                    step
                };
                previously_accepted = true;
            } else {
                next_step = step * (0.9 * error.powf(-0.25)).max(0.2);
                previously_accepted = false;
            }
            if previously_accepted {
                state = high;
                time += step;
            }
            step = next_step.min(*target - time);
        }
        result.push(state.clone());
        previous = *target;
    }
    Ok(result)
}

impl MultiStateModel<ClockForward> {
    pub fn transition_probabilities(
        &self,
        times: Vec<f64>,
        control: MarkovControl,
    ) -> Result<TransitionProbabilityResult, MarkovError> {
        validate_times(&times)?;
        let n = self.graph().state_count();
        let probabilities = match control.integrator {
            MarkovIntegrator::DeSolveRk4 => {
                rk4_series(self, &times, control.singular_hazard, false)?
            }
            MarkovIntegrator::DeSolveRk45Dp7 => {
                rk45_series(self, &times, control.singular_hazard, false)?
            }
            MarkovIntegrator::ConstantMatrixExponential => {
                let q0 = intensity(self, 0.0, control.singular_hazard)?;
                let q1 = intensity(self, 1.0, control.singular_hazard)?;
                if q0
                    .iter()
                    .zip(&q1)
                    .any(|(left, right)| (left - right).abs() > 1e-12)
                {
                    return Err(MarkovError::MatrixExponentialRequiresConstantHazards);
                }
                let q = DMatrix::from_row_slice(n, n, &q0);
                times
                    .iter()
                    .map(|time| {
                        let value = crate::expm::expm(&(q.clone() * *time));
                        value.as_slice().iter().copied().collect::<Vec<_>>()
                    })
                    .map(|column_major| {
                        let mut row_major = vec![0.0; n * n];
                        for row in 0..n {
                            for column in 0..n {
                                row_major[row * n + column] = column_major[column * n + row];
                            }
                        }
                        row_major
                    })
                    .collect()
            }
        };
        Ok(TransitionProbabilityResult {
            times,
            probabilities,
        })
    }

    pub fn total_length_of_stay(
        &self,
        times: Vec<f64>,
        control: MarkovControl,
    ) -> Result<LengthOfStayResult, MarkovError> {
        validate_times(&times)?;
        let n = self.graph().state_count();
        let size = n * n;
        let states = match control.integrator {
            MarkovIntegrator::DeSolveRk4 => {
                rk4_series(self, &times, control.singular_hazard, true)?
            }
            MarkovIntegrator::DeSolveRk45Dp7 => {
                rk45_series(self, &times, control.singular_hazard, true)?
            }
            MarkovIntegrator::ConstantMatrixExponential => {
                // This augmented-system exponential computes both integrals
                // and probabilities without inverting the singular generator.
                let q0 = intensity(self, 0.0, control.singular_hazard)?;
                let q1 = intensity(self, 1.0, control.singular_hazard)?;
                if q0
                    .iter()
                    .zip(&q1)
                    .any(|(left, right)| (left - right).abs() > 1e-12)
                {
                    return Err(MarkovError::MatrixExponentialRequiresConstantHazards);
                }
                let mut block = DMatrix::<f64>::zeros(2 * n, 2 * n);
                for row in 0..n {
                    block[(row + n, row)] = 1.0;
                    for column in 0..n {
                        block[(row + n, column + n)] = q0[row * n + column];
                    }
                }
                times
                    .iter()
                    .map(|time| {
                        let exponential = crate::expm::expm(&(block.clone() * *time));
                        let mut state = vec![0.0; 2 * size];
                        for row in 0..n {
                            for column in 0..n {
                                state[row * n + column] = exponential[(row + n, column)];
                                state[size + row * n + column] = exponential[(row + n, column + n)];
                            }
                        }
                        state
                    })
                    .collect()
            }
        };
        let length_of_stay = states.iter().map(|state| state[..size].to_vec()).collect();
        let probabilities = states.iter().map(|state| state[size..].to_vec()).collect();
        Ok(LengthOfStayResult {
            times,
            length_of_stay,
            probabilities,
        })
    }

    pub fn cumulative_transition_hazards(
        &self,
        times: &[f64],
    ) -> Result<Vec<Vec<f64>>, MarkovError> {
        validate_times(times)?;
        self.graph()
            .transitions()
            .iter()
            .map(|transition| {
                let distribution = self.transition_distribution(transition.id)?;
                times
                    .iter()
                    .map(|time| Ok(distribution.evaluate(*time)?.cumulative_hazard))
                    .collect()
            })
            .collect()
    }

    pub fn cumulative_transition_hazard_covariance(
        &self,
        times: Vec<f64>,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<CumulativeHazardCovarianceResult, MarkovError> {
        let cumulative_hazards = self.cumulative_transition_hazards(&times)?;
        let transition_count = self.graph().transition_count();
        let mut draws =
            vec![vec![Vec::with_capacity(configuration.draws()); transition_count]; times.len()];
        for transitions in self.simulated_transition_models(configuration)? {
            let model = MultiStateModel::clock_forward(self.graph().clone(), transitions)?;
            let candidate = model.cumulative_transition_hazards(&times)?;
            for transition in 0..transition_count {
                for time in 0..times.len() {
                    draws[time][transition].push(candidate[transition][time]);
                }
            }
        }
        let covariance = draws
            .iter()
            .map(|at_time| {
                let mut matrix = vec![0.0; transition_count * transition_count];
                for left in 0..transition_count {
                    let left_mean =
                        at_time[left].iter().sum::<f64>() / configuration.draws() as f64;
                    for right in left..transition_count {
                        let right_mean =
                            at_time[right].iter().sum::<f64>() / configuration.draws() as f64;
                        let value = at_time[left]
                            .iter()
                            .zip(&at_time[right])
                            .map(|(left, right)| (left - left_mean) * (right - right_mean))
                            .sum::<f64>()
                            / (configuration.draws() - 1) as f64;
                        matrix[left * transition_count + right] = value;
                        matrix[right * transition_count + left] = value;
                    }
                }
                matrix
            })
            .collect();
        Ok(CumulativeHazardCovarianceResult {
            times,
            cumulative_hazards,
            covariance,
        })
    }

    pub fn conditional_transition_probabilities(
        &self,
        times: Vec<f64>,
        destination_states: Vec<StateId>,
        control: MarkovControl,
    ) -> Result<ConditionalProbabilityResult, MarkovError> {
        if destination_states.is_empty() {
            return Err(MarkovError::EmptyConditioningStates);
        }
        for (index, state) in destination_states.iter().enumerate() {
            if destination_states[..index].contains(state) {
                return Err(MarkovError::DuplicateConditioningState {
                    state: state.index(),
                });
            }
        }
        let result = self.transition_probabilities(times, control)?;
        let n = self.graph().state_count();
        let mut probabilities = Vec::with_capacity(result.probabilities.len());
        for (time_index, matrix) in result.probabilities.iter().enumerate() {
            let mut conditioned = Vec::with_capacity(n * destination_states.len());
            for start in 0..n {
                let total = destination_states
                    .iter()
                    .map(|state| matrix[start * n + state.index()])
                    .sum::<f64>();
                if total <= 0.0 || !total.is_finite() {
                    return Err(MarkovError::ZeroConditioningProbability {
                        time: time_index,
                        start,
                    });
                }
                conditioned.extend(
                    destination_states
                        .iter()
                        .map(|state| matrix[start * n + state.index()] / total),
                );
            }
            probabilities.push(conditioned);
        }
        Ok(ConditionalProbabilityResult {
            times: result.times,
            destination_states,
            probabilities,
        })
    }

    pub fn transition_probability_intervals(
        &self,
        times: Vec<f64>,
        control: MarkovControl,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<TransitionProbabilityIntervals, MarkovError> {
        let estimate = self.transition_probabilities(times.clone(), control)?;
        let simulations = self.simulated_transition_models(configuration)?;
        let cells = self.graph().state_count().pow(2);
        let mut draws = vec![vec![Vec::with_capacity(configuration.draws()); cells]; times.len()];
        for transitions in simulations {
            let model = MultiStateModel::clock_forward(self.graph().clone(), transitions)?;
            let result = model.transition_probabilities(times.clone(), control)?;
            for (time, matrix) in result.probabilities.iter().enumerate() {
                for (cell, value) in matrix.iter().enumerate() {
                    draws[time][cell].push(*value);
                }
            }
        }
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        let mut lower = vec![vec![0.0; cells]; times.len()];
        let mut upper = lower.clone();
        for time in 0..times.len() {
            for cell in 0..cells {
                draws[time][cell].sort_by(f64::total_cmp);
                lower[time][cell] = type_seven_quantile(&draws[time][cell], tail);
                upper[time][cell] = type_seven_quantile(&draws[time][cell], 1.0 - tail);
            }
        }
        Ok(TransitionProbabilityIntervals {
            estimate,
            lower,
            upper,
            confidence_level: configuration.confidence_level(),
        })
    }

    pub fn length_of_stay_intervals(
        &self,
        times: Vec<f64>,
        control: MarkovControl,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<LengthOfStayIntervals, MarkovError> {
        let estimate = self.total_length_of_stay(times.clone(), control)?;
        let simulations = self.simulated_transition_models(configuration)?;
        let cells = self.graph().state_count().pow(2);
        let mut length_draws =
            vec![vec![Vec::with_capacity(configuration.draws()); cells]; times.len()];
        let mut probability_draws = length_draws.clone();
        for transitions in simulations {
            let model = MultiStateModel::clock_forward(self.graph().clone(), transitions)?;
            let result = model.total_length_of_stay(times.clone(), control)?;
            for time in 0..times.len() {
                for cell in 0..cells {
                    length_draws[time][cell].push(result.length_of_stay[time][cell]);
                    probability_draws[time][cell].push(result.probabilities[time][cell]);
                }
            }
        }
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        let summarize = |mut draws: Vec<Vec<Vec<f64>>>| {
            let mut lower = vec![vec![0.0; cells]; times.len()];
            let mut upper = lower.clone();
            for time in 0..times.len() {
                for cell in 0..cells {
                    draws[time][cell].sort_by(f64::total_cmp);
                    lower[time][cell] = type_seven_quantile(&draws[time][cell], tail);
                    upper[time][cell] = type_seven_quantile(&draws[time][cell], 1.0 - tail);
                }
            }
            (lower, upper)
        };
        let (length_lower, length_upper) = summarize(length_draws);
        let (probability_lower, probability_upper) = summarize(probability_draws);
        Ok(LengthOfStayIntervals {
            estimate,
            length_lower,
            length_upper,
            probability_lower,
            probability_upper,
            confidence_level: configuration.confidence_level(),
        })
    }
}
