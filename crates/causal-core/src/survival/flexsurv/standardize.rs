//! Direct standardisation and contrasts from flexsurv's `standsurv`.

use std::num::NonZeroUsize;

use crate::r_zeroin::{r_zeroin2, ZeroinError};

use super::distribution::DistributionError;
use super::fit::{FlexSurvFit, UncertaintyEvidence};
use super::model::{ModelError, PredictionCovariates};
use super::uncertainty::{
    sample_standard_deviation, type_seven_quantile, PredictionSimulationConfiguration,
    UncertaintyError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizationPopulation {
    profiles: Vec<PredictionCovariates>,
    weights: Vec<f64>,
    total_weight: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Measure {
    Survival,
    Hazard,
    RestrictedMean,
    Quantile,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizationRequest {
    measure: Measure,
    points: Vec<f64>,
    quantile_interval: Option<(f64, f64)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContrastKind {
    Difference,
    Ratio,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntervalTransform {
    None,
    Log,
    LogLog,
    Logit,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StandardizationUncertainty {
    None,
    Delta {
        confidence_level: f64,
        transform: IntervalTransform,
        contrast_transform: IntervalTransform,
    },
    /// Reproduces `standsurv(..., boot = TRUE)`: each scenario receives the
    /// next independent block from one continuous R-compatible RNG stream.
    Simulation(PredictionSimulationConfiguration),
    /// Uses the same parameter draw for every scenario. This is useful for
    /// paired counterfactual contrasts, but differs from flexsurv 2.3.2.
    PairedSimulation(PredictionSimulationConfiguration),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardizationContrast {
    reference: usize,
    kind: ContrastKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizationPlan {
    populations: Vec<StandardizationPopulation>,
    request: StandardizationRequest,
    contrast: Option<StandardizationContrast>,
    uncertainty: StandardizationUncertainty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StandardizedEstimate {
    pub estimate: f64,
    pub standard_error: Option<f64>,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizedScenarioResult {
    pub scenario: usize,
    pub estimates: Vec<StandardizedEstimate>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizedContrastResult {
    pub scenario: usize,
    pub reference: usize,
    pub kind: ContrastKind,
    pub estimates: Vec<StandardizedEstimate>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StandardizationResult {
    pub points: Vec<f64>,
    pub scenarios: Vec<StandardizedScenarioResult>,
    pub contrasts: Vec<StandardizedContrastResult>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StandardizationError {
    EmptyPopulation,
    EmptyRequest,
    EmptyScenarios,
    WeightCount { expected: usize, actual: usize },
    InvalidWeight { index: usize },
    ZeroTotalWeight,
    InvalidTime { index: usize },
    InvalidProbability { index: usize },
    InvalidQuantileInterval,
    ReferenceOutsideScenarios { reference: usize, scenarios: usize },
    ContrastNeedsTwoScenarios,
    InvalidConfidenceLevel,
    CovarianceUnavailable,
    CovarianceOrder { expected: usize, actual: usize },
    NonFiniteEstimate,
    Model(ModelError),
    Distribution(DistributionError),
    Uncertainty(UncertaintyError),
    RootNotBracketed,
    NonFiniteRootValue,
}

impl From<ModelError> for StandardizationError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for StandardizationError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl From<UncertaintyError> for StandardizationError {
    fn from(value: UncertaintyError) -> Self {
        Self::Uncertainty(value)
    }
}

fn root_error(value: ZeroinError) -> StandardizationError {
    match value {
        ZeroinError::RootNotBracketed => StandardizationError::RootNotBracketed,
        ZeroinError::NonFiniteValue => StandardizationError::NonFiniteRootValue,
    }
}

impl FlexSurvFit {
    pub fn standardization_population(
        &self,
        rows: Vec<Vec<f64>>,
        weights: Option<Vec<f64>>,
    ) -> Result<StandardizationPopulation, StandardizationError> {
        if rows.is_empty() {
            return Err(StandardizationError::EmptyPopulation);
        }
        let profiles = rows
            .into_iter()
            .map(|row| self.model.prediction_covariates(row))
            .collect::<Result<Vec<_>, _>>()?;
        let weights = weights.unwrap_or_else(|| vec![1.0; profiles.len()]);
        if weights.len() != profiles.len() {
            return Err(StandardizationError::WeightCount {
                expected: profiles.len(),
                actual: weights.len(),
            });
        }
        if let Some(index) = weights
            .iter()
            .position(|weight| !weight.is_finite() || *weight < 0.0)
        {
            return Err(StandardizationError::InvalidWeight { index });
        }
        let total_weight = weights.iter().sum::<f64>();
        if total_weight <= 0.0 {
            return Err(StandardizationError::ZeroTotalWeight);
        }
        Ok(StandardizationPopulation {
            profiles,
            weights,
            total_weight,
        })
    }
}

impl StandardizationRequest {
    pub fn survival(times: Vec<f64>) -> Result<Self, StandardizationError> {
        Self::times(Measure::Survival, times)
    }

    pub fn hazard(times: Vec<f64>) -> Result<Self, StandardizationError> {
        Self::times(Measure::Hazard, times)
    }

    pub fn restricted_mean(times: Vec<f64>) -> Result<Self, StandardizationError> {
        Self::times(Measure::RestrictedMean, times)
    }

    fn times(measure: Measure, times: Vec<f64>) -> Result<Self, StandardizationError> {
        if times.is_empty() {
            return Err(StandardizationError::EmptyRequest);
        }
        if let Some(index) = times
            .iter()
            .position(|time| !time.is_finite() || *time < 0.0)
        {
            return Err(StandardizationError::InvalidTime { index });
        }
        Ok(Self {
            measure,
            points: times,
            quantile_interval: None,
        })
    }

    pub fn quantiles(
        probabilities: Vec<f64>,
        interval: (f64, f64),
    ) -> Result<Self, StandardizationError> {
        if probabilities.is_empty() {
            return Err(StandardizationError::EmptyRequest);
        }
        if let Some(index) = probabilities
            .iter()
            .position(|probability| !probability.is_finite() || !(0.0..=1.0).contains(probability))
        {
            return Err(StandardizationError::InvalidProbability { index });
        }
        if !interval.0.is_finite()
            || !interval.1.is_finite()
            || interval.0 < 0.0
            || interval.0 >= interval.1
        {
            return Err(StandardizationError::InvalidQuantileInterval);
        }
        Ok(Self {
            measure: Measure::Quantile,
            points: probabilities,
            quantile_interval: Some(interval),
        })
    }
}

impl StandardizationContrast {
    pub fn new(reference: usize, kind: ContrastKind) -> Self {
        Self { reference, kind }
    }
}

impl StandardizationPlan {
    pub fn new(
        populations: Vec<StandardizationPopulation>,
        request: StandardizationRequest,
        contrast: Option<StandardizationContrast>,
        uncertainty: StandardizationUncertainty,
    ) -> Result<Self, StandardizationError> {
        let scenarios =
            NonZeroUsize::new(populations.len()).ok_or(StandardizationError::EmptyScenarios)?;
        if let Some(contrast) = contrast {
            if scenarios.get() < 2 {
                return Err(StandardizationError::ContrastNeedsTwoScenarios);
            }
            if contrast.reference >= scenarios.get() {
                return Err(StandardizationError::ReferenceOutsideScenarios {
                    reference: contrast.reference,
                    scenarios: scenarios.get(),
                });
            }
        }
        if let StandardizationUncertainty::Delta {
            confidence_level, ..
        } = uncertainty
        {
            if !confidence_level.is_finite() || !(0.0..1.0).contains(&confidence_level) {
                return Err(StandardizationError::InvalidConfidenceLevel);
            }
        }
        Ok(Self {
            populations,
            request,
            contrast,
            uncertainty,
        })
    }
}

fn population_value(
    fit: &FlexSurvFit,
    population: &StandardizationPopulation,
    request: &StandardizationRequest,
    point: f64,
    parameters: &[f64],
) -> Result<f64, StandardizationError> {
    let value = match request.measure {
        Measure::Survival => {
            let mut total = 0.0;
            for (profile, weight) in population.profiles.iter().zip(&population.weights) {
                let distribution = fit.model.distribution_for_prediction(profile, parameters)?;
                total += distribution.survival(point)? * weight;
            }
            total / population.total_weight
        }
        Measure::Hazard => {
            let mut numerator = 0.0;
            let mut denominator = 0.0;
            for (profile, weight) in population.profiles.iter().zip(&population.weights) {
                let distribution = fit.model.distribution_for_prediction(profile, parameters)?;
                let survival = distribution.survival(point)?;
                numerator += distribution.evaluate(point)?.hazard * survival * weight;
                denominator += survival * weight;
            }
            numerator / denominator
        }
        Measure::RestrictedMean => {
            let mut total = 0.0;
            for (profile, weight) in population.profiles.iter().zip(&population.weights) {
                let distribution = fit.model.distribution_for_prediction(profile, parameters)?;
                total += distribution.restricted_mean(0.0, point)? * weight;
            }
            total / population.total_weight
        }
        Measure::Quantile => {
            let interval = request.quantile_interval.unwrap();
            r_zeroin2(
                interval.0,
                interval.1,
                f64::EPSILON.powf(0.25),
                1000,
                |time| {
                    let mut total = 0.0;
                    for (profile, weight) in population.profiles.iter().zip(&population.weights) {
                        let distribution = fit
                            .model
                            .distribution_for_prediction(profile, parameters)
                            .unwrap();
                        total += distribution.survival(time).unwrap() * weight;
                    }
                    total / population.total_weight - (1.0 - point)
                },
            )
            .map_err(root_error)?
        }
    };
    if value.is_finite() {
        Ok(value)
    } else {
        Err(StandardizationError::NonFiniteEstimate)
    }
}

fn contrast_value(kind: ContrastKind, target: f64, reference: f64) -> f64 {
    match kind {
        ContrastKind::Difference => target - reference,
        ContrastKind::Ratio => target / reference,
    }
}

fn transform(transform: IntervalTransform, value: f64) -> f64 {
    match transform {
        IntervalTransform::None => value,
        IntervalTransform::Log => value.ln(),
        IntervalTransform::LogLog => (-(1.0 - value).ln()).ln(),
        IntervalTransform::Logit => (value / (1.0 - value)).ln(),
    }
}

fn inverse_transform(transform: IntervalTransform, value: f64) -> f64 {
    match transform {
        IntervalTransform::None => value,
        IntervalTransform::Log => value.exp(),
        IntervalTransform::LogLog => 1.0 - (-value.exp()).exp(),
        IntervalTransform::Logit => 1.0 / (1.0 + (-value).exp()),
    }
}

fn covariance<'a>(
    fit: &'a FlexSurvFit,
) -> Result<(&'a [f64], usize, Vec<usize>), StandardizationError> {
    let UncertaintyEvidence::Hessian {
        covariance,
        covariance_order,
        transformed,
        ..
    } = &fit.uncertainty
    else {
        return Err(StandardizationError::CovarianceUnavailable);
    };
    let free = transformed
        .iter()
        .enumerate()
        .filter_map(|(index, interval)| interval.as_ref().map(|_| index))
        .collect::<Vec<_>>();
    if free.len() != *covariance_order {
        return Err(StandardizationError::CovarianceOrder {
            expected: free.len(),
            actual: *covariance_order,
        });
    }
    Ok((covariance, *covariance_order, free))
}

fn delta_estimate<F>(
    fit: &FlexSurvFit,
    estimate: f64,
    confidence_level: f64,
    interval_transform: IntervalTransform,
    function: F,
) -> Result<StandardizedEstimate, StandardizationError>
where
    F: Fn(&[f64]) -> Result<f64, StandardizationError>,
{
    let (covariance, order, free) = covariance(fit)?;
    let epsilon = 1e-4;
    let baseline_transformed = transform(interval_transform, estimate);
    let mut gradient = Vec::with_capacity(order);
    let mut plain_gradient = Vec::with_capacity(order);
    for index in free {
        let mut changed = fit.transformed_parameters.clone();
        changed[index] += epsilon;
        let changed_value = function(&changed)?;
        plain_gradient.push((changed_value - estimate) / epsilon);
        gradient
            .push((transform(interval_transform, changed_value) - baseline_transformed) / epsilon);
    }
    let quadratic = |values: &[f64]| {
        let mut variance = 0.0;
        for row in 0..order {
            for column in 0..order {
                variance += values[row] * covariance[row * order + column] * values[column];
            }
        }
        variance.max(0.0)
    };
    let standard_error = quadratic(&plain_gradient).sqrt();
    let transformed_standard_error = quadratic(&gradient).sqrt();
    let quantile = spec_math::cephes64::ndtri(0.5 + confidence_level / 2.0);
    Ok(StandardizedEstimate {
        estimate,
        standard_error: Some(standard_error),
        lower: Some(inverse_transform(
            interval_transform,
            baseline_transformed - quantile * transformed_standard_error,
        )),
        upper: Some(inverse_transform(
            interval_transform,
            baseline_transformed + quantile * transformed_standard_error,
        )),
    })
}

fn simulation_estimate(
    estimate: f64,
    mut draws: Vec<f64>,
    configuration: PredictionSimulationConfiguration,
) -> StandardizedEstimate {
    let standard_error = sample_standard_deviation(&draws);
    draws.sort_by(f64::total_cmp);
    let tail = (1.0 - configuration.confidence_level()) / 2.0;
    StandardizedEstimate {
        estimate,
        standard_error: Some(standard_error),
        lower: Some(type_seven_quantile(&draws, tail)),
        upper: Some(type_seven_quantile(&draws, 1.0 - tail)),
    }
}

pub fn standardize(
    fit: &FlexSurvFit,
    plan: StandardizationPlan,
) -> Result<StandardizationResult, StandardizationError> {
    let point_values = plan
        .populations
        .iter()
        .map(|population| {
            plan.request
                .points
                .iter()
                .map(|point| {
                    population_value(
                        fit,
                        population,
                        &plan.request,
                        *point,
                        &fit.transformed_parameters,
                    )
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let parameter_draws = match plan.uncertainty {
        StandardizationUncertainty::Simulation(configuration) => {
            let total = configuration
                .draws()
                .checked_mul(plan.populations.len())
                .ok_or(StandardizationError::NonFiniteEstimate)?;
            let all = PredictionSimulationConfiguration::new(
                total,
                configuration.confidence_level(),
                configuration.seed(),
            )?;
            Some((
                configuration,
                fit.simulate_transformed_parameters(all)?.transformed,
                false,
            ))
        }
        StandardizationUncertainty::PairedSimulation(configuration) => Some((
            configuration,
            fit.simulate_transformed_parameters(configuration)?
                .transformed,
            true,
        )),
        _ => None,
    };
    let mut scenarios = Vec::with_capacity(plan.populations.len());
    for (scenario, population) in plan.populations.iter().enumerate() {
        let mut estimates = Vec::with_capacity(plan.request.points.len());
        for (point_index, point) in plan.request.points.iter().copied().enumerate() {
            let estimate = point_values[scenario][point_index];
            let result = match plan.uncertainty {
                StandardizationUncertainty::None => StandardizedEstimate {
                    estimate,
                    standard_error: None,
                    lower: None,
                    upper: None,
                },
                StandardizationUncertainty::Delta {
                    confidence_level,
                    transform,
                    ..
                } => delta_estimate(fit, estimate, confidence_level, transform, |parameters| {
                    population_value(fit, population, &plan.request, point, parameters)
                })?,
                StandardizationUncertainty::Simulation(_)
                | StandardizationUncertainty::PairedSimulation(_) => {
                    let (configuration, draws, paired) = parameter_draws.as_ref().unwrap();
                    let start = if *paired {
                        0
                    } else {
                        scenario * configuration.draws()
                    };
                    let draws = &draws[start..start + configuration.draws()];
                    let values = draws
                        .iter()
                        .map(|parameters| {
                            population_value(fit, population, &plan.request, point, parameters)
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    simulation_estimate(estimate, values, *configuration)
                }
            };
            estimates.push(result);
        }
        scenarios.push(StandardizedScenarioResult {
            scenario,
            estimates,
        });
    }

    let mut contrasts = Vec::new();
    if let Some(contrast) = plan.contrast {
        for scenario in 0..plan.populations.len() {
            if scenario == contrast.reference {
                continue;
            }
            let mut estimates = Vec::with_capacity(plan.request.points.len());
            for (point_index, point) in plan.request.points.iter().copied().enumerate() {
                let estimate = contrast_value(
                    contrast.kind,
                    point_values[scenario][point_index],
                    point_values[contrast.reference][point_index],
                );
                let result = match plan.uncertainty {
                    StandardizationUncertainty::None => StandardizedEstimate {
                        estimate,
                        standard_error: None,
                        lower: None,
                        upper: None,
                    },
                    StandardizationUncertainty::Delta {
                        confidence_level,
                        contrast_transform,
                        ..
                    } => delta_estimate(
                        fit,
                        estimate,
                        confidence_level,
                        contrast_transform,
                        |parameters| {
                            let target = population_value(
                                fit,
                                &plan.populations[scenario],
                                &plan.request,
                                point,
                                parameters,
                            )?;
                            let reference = population_value(
                                fit,
                                &plan.populations[contrast.reference],
                                &plan.request,
                                point,
                                parameters,
                            )?;
                            Ok(contrast_value(contrast.kind, target, reference))
                        },
                    )?,
                    StandardizationUncertainty::Simulation(_)
                    | StandardizationUncertainty::PairedSimulation(_) => {
                        let (configuration, draws, paired) = parameter_draws.as_ref().unwrap();
                        let target_start = if *paired {
                            0
                        } else {
                            scenario * configuration.draws()
                        };
                        let reference_start = if *paired {
                            0
                        } else {
                            contrast.reference * configuration.draws()
                        };
                        let values = (0..configuration.draws())
                            .map(|draw| {
                                let target_parameters = &draws[target_start + draw];
                                let reference_parameters = &draws[reference_start + draw];
                                let target = population_value(
                                    fit,
                                    &plan.populations[scenario],
                                    &plan.request,
                                    point,
                                    target_parameters,
                                )?;
                                let reference = population_value(
                                    fit,
                                    &plan.populations[contrast.reference],
                                    &plan.request,
                                    point,
                                    reference_parameters,
                                )?;
                                Ok(contrast_value(contrast.kind, target, reference))
                            })
                            .collect::<Result<Vec<_>, StandardizationError>>()?;
                        simulation_estimate(estimate, values, *configuration)
                    }
                };
                estimates.push(result);
            }
            contrasts.push(StandardizedContrastResult {
                scenario,
                reference: contrast.reference,
                kind: contrast.kind,
                estimates,
            });
        }
    }

    Ok(StandardizationResult {
        points: plan.request.points,
        scenarios,
        contrasts,
    })
}
