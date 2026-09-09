//! Simulation-based prediction uncertainty from `summary.flexsurvreg`.

use std::num::NonZeroUsize;

use crate::survival::r_rng::RRng;

use super::covariance::{eigen_covariance_root, CovarianceError};
use super::distribution::{DistributionError, FlexSurvDistribution};
use super::fit::{FlexSurvFit, UncertaintyEvidence};
use super::model::{ModelError, PredictionCovariates};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PredictionSimulationConfiguration {
    draws: NonZeroUsize,
    confidence_level: f64,
    seed: u32,
}

impl PredictionSimulationConfiguration {
    pub fn new(draws: usize, confidence_level: f64, seed: u32) -> Result<Self, UncertaintyError> {
        let draws = NonZeroUsize::new(draws).ok_or(UncertaintyError::NoSimulationDraws)?;
        if !confidence_level.is_finite() || !(0.0..1.0).contains(&confidence_level) {
            return Err(UncertaintyError::InvalidConfidenceLevel);
        }
        if draws.get() < 2 {
            return Err(UncertaintyError::StandardErrorNeedsTwoDraws);
        }
        Ok(Self {
            draws,
            confidence_level,
            seed,
        })
    }

    pub fn draws(self) -> usize {
        self.draws.get()
    }

    pub fn confidence_level(self) -> f64 {
        self.confidence_level
    }

    pub fn seed(self) -> u32 {
        self.seed
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PredictionSummaryRequest {
    Survival { time: f64, start: f64 },
    Hazard { time: f64, start: f64 },
    CumulativeHazard { time: f64, start: f64 },
    Quantile { probability: f64, start: f64 },
    Mean,
    RestrictedMean { start: f64, end: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredictionSimulationResult {
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
    pub standard_error: f64,
    pub simulated: Vec<f64>,
    pub confidence_level: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterSimulationResult {
    /// Rows are simulation replicates; columns retain the fitted model's full
    /// transformed-parameter order, including fixed parameters.
    pub transformed: Vec<Vec<f64>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UncertaintyError {
    NoSimulationDraws,
    StandardErrorNeedsTwoDraws,
    InvalidConfidenceLevel,
    CovarianceUnavailable,
    CovarianceOrder { expected: usize, actual: usize },
    InvalidTime,
    Model(ModelError),
    Distribution(DistributionError),
    Covariance(CovarianceError),
    NonFiniteSimulation { draw: usize },
}

impl From<ModelError> for UncertaintyError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for UncertaintyError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl From<CovarianceError> for UncertaintyError {
    fn from(value: CovarianceError) -> Self {
        Self::Covariance(value)
    }
}

fn prediction_value(
    distribution: FlexSurvDistribution,
    request: PredictionSummaryRequest,
) -> Result<f64, UncertaintyError> {
    let value = match request {
        PredictionSummaryRequest::Survival { time, start } => {
            validate_time_pair(time, start)?;
            if time < start {
                1.0
            } else {
                distribution.survival(time)? / distribution.survival(start)?
            }
        }
        PredictionSummaryRequest::Hazard { time, start } => {
            validate_time_pair(time, start)?;
            if time < start {
                0.0
            } else {
                distribution.evaluate(time)?.hazard * distribution.survival(start)?
            }
        }
        PredictionSummaryRequest::CumulativeHazard { time, start } => {
            validate_time_pair(time, start)?;
            if time < start {
                0.0
            } else {
                let at_time = distribution.evaluate(time)?.cumulative_hazard;
                let at_start = distribution.evaluate(start)?.cumulative_hazard;
                at_time - at_start
            }
        }
        PredictionSummaryRequest::Quantile { probability, start } => {
            if !start.is_finite() || start < 0.0 {
                return Err(UncertaintyError::InvalidTime);
            }
            let start_probability = distribution.cdf(start)?;
            distribution.quantile(start_probability + (1.0 - start_probability) * probability)?
        }
        PredictionSummaryRequest::Mean => distribution.mean()?,
        PredictionSummaryRequest::RestrictedMean { start, end } => {
            distribution.restricted_mean(start, end)?
        }
    };
    Ok(value)
}

fn validate_time_pair(time: f64, start: f64) -> Result<(), UncertaintyError> {
    if time.is_nan() || time < 0.0 || !start.is_finite() || start < 0.0 {
        Err(UncertaintyError::InvalidTime)
    } else {
        Ok(())
    }
}

pub(crate) fn type_seven_quantile(sorted: &[f64], probability: f64) -> f64 {
    let position = (sorted.len() - 1) as f64 * probability;
    let lower = position.floor() as usize;
    let fraction = position - lower as f64;
    if fraction == 0.0 || lower + 1 == sorted.len() {
        sorted[lower]
    } else {
        sorted[lower] + fraction * (sorted[lower + 1] - sorted[lower])
    }
}

pub(crate) fn sample_standard_deviation(values: &[f64]) -> f64 {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let sum_squares = values
        .iter()
        .map(|value| {
            let residual = value - mean;
            residual * residual
        })
        .sum::<f64>();
    (sum_squares / (values.len() - 1) as f64).sqrt()
}

impl FlexSurvFit {
    pub(crate) fn simulate_transformed_parameters_with_rng(
        &self,
        configuration: PredictionSimulationConfiguration,
        rng: &mut RRng,
    ) -> Result<ParameterSimulationResult, UncertaintyError> {
        let (covariance, order, transformed_intervals) = match &self.uncertainty {
            UncertaintyEvidence::AllParametersFixed => {
                return Err(UncertaintyError::CovarianceUnavailable)
            }
            UncertaintyEvidence::Hessian {
                covariance,
                covariance_order,
                transformed,
                ..
            } => (covariance, *covariance_order, transformed),
        };
        let free = transformed_intervals
            .iter()
            .enumerate()
            .filter_map(|(index, interval)| interval.as_ref().map(|_| index))
            .collect::<Vec<_>>();
        if free.len() != order {
            return Err(UncertaintyError::CovarianceOrder {
                expected: free.len(),
                actual: order,
            });
        }

        let root = eigen_covariance_root(covariance, order)?;
        let free_means = free
            .iter()
            .map(|index| self.transformed_parameters[*index])
            .collect::<Vec<_>>();
        let mut parameter_draws = Vec::with_capacity(configuration.draws());

        for _ in 0..configuration.draws() {
            let normals = (0..order).map(|_| rng.normal()).collect::<Vec<_>>();
            let mut transformed = self.transformed_parameters.clone();
            for column in 0..order {
                let shift = (0..order)
                    .map(|row| normals[row] * root[row * order + column])
                    .sum::<f64>();
                transformed[free[column]] = free_means[column] + shift;
            }
            parameter_draws.push(transformed);
        }
        Ok(ParameterSimulationResult {
            transformed: parameter_draws,
        })
    }

    pub fn simulate_transformed_parameters(
        &self,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<ParameterSimulationResult, UncertaintyError> {
        self.simulate_transformed_parameters_with_rng(
            configuration,
            &mut RRng::new(configuration.seed()),
        )
    }

    pub fn predict_with_simulation(
        &self,
        covariates: &PredictionCovariates,
        request: PredictionSummaryRequest,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<PredictionSimulationResult, UncertaintyError> {
        let parameter_draws = self.simulate_transformed_parameters(configuration)?;
        let mut simulated = Vec::with_capacity(configuration.draws());

        for (draw, transformed) in parameter_draws.transformed.iter().enumerate() {
            let distribution = self
                .model
                .distribution_for_prediction(covariates, transformed)?;
            let value = prediction_value(distribution, request)?;
            if !value.is_finite() {
                return Err(UncertaintyError::NonFiniteSimulation { draw });
            }
            simulated.push(value);
        }

        let distribution = self
            .model
            .distribution_for_prediction(covariates, &self.transformed_parameters)?;
        let estimate = prediction_value(distribution, request)?;
        let mut ordered = simulated.clone();
        ordered.sort_by(f64::total_cmp);
        let tail = (1.0 - configuration.confidence_level()) / 2.0;
        Ok(PredictionSimulationResult {
            estimate,
            lower: type_seven_quantile(&ordered, tail),
            upper: type_seven_quantile(&ordered, 1.0 - tail),
            standard_error: sample_standard_deviation(&simulated),
            simulated,
            confidence_level: configuration.confidence_level(),
        })
    }
}
