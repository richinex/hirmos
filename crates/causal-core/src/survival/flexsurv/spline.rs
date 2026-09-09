//! Royston-Parmar natural-cubic-spline survival distributions from flexsurv.

use std::collections::BTreeSet;
use std::f64::consts::{PI, SQRT_2};
use std::num::NonZeroUsize;

use super::covariance::{hessian_to_covariance, CovarianceError};
use super::distribution::adaptive_simpson;
use super::fit::{
    GradientRecipe, OptimizationEvidence, OptimizerConvergence, ParameterInterval, SurvivalDataset,
    UncertaintyEvidence,
};
use super::model::CovariateMatrix;
use super::r_optim::{
    r_optim_bfgs, r_optim_bfgs_numeric, r_optim_hessian_numeric, ROptimControl, ROptimError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplineScale {
    Hazard,
    Odds,
    Normal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SplineTimeScale {
    Log,
    Identity,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SplineError {
    TooFewKnots,
    KnotsNotStrictlyIncreasing,
    CoefficientCount { expected: usize, actual: usize },
    NonFiniteCoefficient { index: usize },
    NonFiniteKnot { index: usize },
    NegativeTime,
    ProbabilityOutsideUnitInterval,
    QuantileDoesNotExist,
    InvalidRestrictedMeanWindow,
    NumericalIntegrationFailed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplineValue {
    pub density: f64,
    pub cdf: f64,
    pub survival: f64,
    pub hazard: f64,
    pub cumulative_hazard: f64,
}

/// A Royston-Parmar distribution valid by construction.
#[derive(Clone, Debug, PartialEq)]
pub struct RoystonParmarSpline {
    coefficients: Vec<f64>,
    knots: Vec<f64>,
    scale: SplineScale,
    time_scale: SplineTimeScale,
}

fn positive_cube(value: f64) -> f64 {
    if value <= 0.0 {
        0.0
    } else {
        value.powi(3)
    }
}

fn positive_cube_derivative(value: f64) -> f64 {
    if value <= 0.0 {
        0.0
    } else {
        3.0 * value * value
    }
}

fn softplus(value: f64) -> f64 {
    if value > 0.0 {
        value + (-value).exp().ln_1p()
    } else {
        value.exp().ln_1p()
    }
}

fn logistic(value: f64) -> f64 {
    if value >= 0.0 {
        1.0 / (1.0 + (-value).exp())
    } else {
        let exponential = value.exp();
        exponential / (1.0 + exponential)
    }
}

fn normal_density(value: f64) -> f64 {
    (-0.5 * value * value).exp() / (2.0 * PI).sqrt()
}

fn normal_survival(value: f64) -> f64 {
    0.5 * libm::erfc(value / SQRT_2)
}

impl RoystonParmarSpline {
    pub fn new(
        coefficients: Vec<f64>,
        knots: Vec<f64>,
        scale: SplineScale,
        time_scale: SplineTimeScale,
    ) -> Result<Self, SplineError> {
        if knots.len() < 2 {
            return Err(SplineError::TooFewKnots);
        }
        if let Some(index) = knots.iter().position(|value| !value.is_finite()) {
            return Err(SplineError::NonFiniteKnot { index });
        }
        if knots.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(SplineError::KnotsNotStrictlyIncreasing);
        }
        if coefficients.len() != knots.len() {
            return Err(SplineError::CoefficientCount {
                expected: knots.len(),
                actual: coefficients.len(),
            });
        }
        if let Some(index) = coefficients.iter().position(|value| !value.is_finite()) {
            return Err(SplineError::NonFiniteCoefficient { index });
        }
        Ok(Self {
            coefficients,
            knots,
            scale,
            time_scale,
        })
    }

    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    fn transformed_time(&self, time: f64) -> f64 {
        match self.time_scale {
            SplineTimeScale::Log => time.ln(),
            SplineTimeScale::Identity => time,
        }
    }

    fn transformed_time_derivative(&self, time: f64) -> f64 {
        match self.time_scale {
            SplineTimeScale::Log => 1.0 / time,
            SplineTimeScale::Identity => 1.0,
        }
    }

    fn predictor_and_derivative(&self, transformed_time: f64) -> (f64, f64) {
        let (basis, derivative_basis) = self.basis_and_derivative(transformed_time);
        let predictor = basis
            .iter()
            .zip(&self.coefficients)
            .map(|(basis, coefficient)| basis * coefficient)
            .sum();
        let derivative = derivative_basis
            .iter()
            .zip(&self.coefficients)
            .map(|(basis, coefficient)| basis * coefficient)
            .sum();
        (predictor, derivative)
    }

    fn basis_and_derivative(&self, transformed_time: f64) -> (Vec<f64>, Vec<f64>) {
        let mut basis = vec![1.0, transformed_time];
        let mut derivative = vec![0.0, 1.0];
        let first = self.knots[0];
        let last = self.knots[self.knots.len() - 1];
        for index in 0..self.knots.len() - 2 {
            let knot = self.knots[index + 1];
            let lambda = (last - knot) / (last - first);
            let basis_value = positive_cube(transformed_time - knot)
                - lambda * positive_cube(transformed_time - first)
                - (1.0 - lambda) * positive_cube(transformed_time - last);
            let basis_derivative = positive_cube_derivative(transformed_time - knot)
                - lambda * positive_cube_derivative(transformed_time - first)
                - (1.0 - lambda) * positive_cube_derivative(transformed_time - last);
            basis.push(basis_value);
            derivative.push(basis_derivative);
        }
        (basis, derivative)
    }

    pub fn survival(&self, time: f64) -> Result<f64, SplineError> {
        if time < 0.0 {
            return Ok(1.0);
        }
        if time == 0.0 {
            return Ok(1.0);
        }
        if time.is_infinite() {
            return Ok(0.0);
        }
        let (predictor, _) = self.predictor_and_derivative(self.transformed_time(time));
        Ok(match self.scale {
            SplineScale::Hazard => (-predictor.exp()).exp(),
            SplineScale::Odds => 1.0 - logistic(predictor),
            SplineScale::Normal => normal_survival(predictor),
        })
    }

    pub fn cdf(&self, time: f64) -> Result<f64, SplineError> {
        Ok(1.0 - self.survival(time)?)
    }

    pub fn evaluate(&self, time: f64) -> Result<SplineValue, SplineError> {
        if time < 0.0 {
            return Err(SplineError::NegativeTime);
        }
        if time == 0.0 {
            return Ok(SplineValue {
                density: 0.0,
                cdf: 0.0,
                survival: 1.0,
                hazard: 0.0,
                cumulative_hazard: 0.0,
            });
        }
        let (predictor, spline_derivative) =
            self.predictor_and_derivative(self.transformed_time(time));
        let time_derivative = self.transformed_time_derivative(time);
        let slope = time_derivative * spline_derivative;
        let survival = match self.scale {
            SplineScale::Hazard => (-predictor.exp()).exp(),
            SplineScale::Odds => 1.0 - logistic(predictor),
            SplineScale::Normal => normal_survival(predictor),
        };
        let density_link = match self.scale {
            SplineScale::Hazard => (predictor - predictor.exp()).exp(),
            SplineScale::Odds => (predictor - 2.0 * softplus(predictor)).exp(),
            SplineScale::Normal => normal_density(predictor),
        };
        let density = (slope * density_link).max(0.0);
        let hazard_link = match self.scale {
            SplineScale::Hazard => predictor.exp(),
            SplineScale::Odds => logistic(predictor),
            SplineScale::Normal => normal_density(-predictor) / normal_survival(predictor),
        };
        let hazard = (slope * hazard_link).max(0.0);
        let cumulative_hazard = match self.scale {
            SplineScale::Hazard => predictor.exp(),
            SplineScale::Odds => softplus(predictor),
            SplineScale::Normal => -normal_survival(predictor).ln(),
        };
        Ok(SplineValue {
            density,
            cdf: 1.0 - survival,
            survival,
            hazard,
            cumulative_hazard,
        })
    }

    pub fn quantile(&self, probability: f64) -> Result<f64, SplineError> {
        if !(0.0..=1.0).contains(&probability) || !probability.is_finite() {
            return Err(SplineError::ProbabilityOutsideUnitInterval);
        }
        if probability == 0.0 {
            return Ok(0.0);
        }
        if probability == 1.0 {
            return Ok(f64::INFINITY);
        }

        let mut lower = 0.0;
        let mut upper = 1.0;
        for _ in 0..1024 {
            if self.cdf(upper)? >= probability {
                break;
            }
            upper *= 2.0;
            if !upper.is_finite() {
                return Err(SplineError::QuantileDoesNotExist);
            }
        }
        if self.cdf(upper)? < probability {
            return Err(SplineError::QuantileDoesNotExist);
        }
        for _ in 0..256 {
            let middle = lower + (upper - lower) / 2.0;
            if middle == lower || middle == upper {
                break;
            }
            if self.cdf(middle)? < probability {
                lower = middle;
            } else {
                upper = middle;
            }
        }
        Ok(lower + (upper - lower) / 2.0)
    }

    pub fn restricted_mean(&self, start: f64, end: f64) -> Result<f64, SplineError> {
        if !start.is_finite() || start < 0.0 || end.is_nan() || end < start {
            return Err(SplineError::InvalidRestrictedMeanWindow);
        }
        if start == end {
            return Ok(0.0);
        }
        let start_survival = self.survival(start)?;
        if start_survival == 0.0 {
            return Ok(0.0);
        }
        let integrand = |time: f64| self.survival(time).unwrap_or(f64::NAN) / start_survival;
        let value = if end.is_finite() {
            adaptive_simpson(&integrand, start, end, 1e-9, 22)
        } else {
            let transformed = |u: f64| {
                if u == 1.0 {
                    0.0
                } else {
                    let one_minus = 1.0 - u;
                    integrand(start + u / one_minus) / (one_minus * one_minus)
                }
            };
            adaptive_simpson(&transformed, 0.0, 1.0, 1e-9, 24)
        };
        if value.is_finite() {
            Ok(value)
        } else {
            Err(SplineError::NumericalIntegrationFailed)
        }
    }

    fn log_density_score(&self, time: f64) -> Vec<f64> {
        let transformed = self.transformed_time(time);
        let (basis, derivative_basis) = self.basis_and_derivative(transformed);
        let predictor = basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let derivative = derivative_basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let link = match self.scale {
            SplineScale::Hazard => 1.0 - predictor.exp(),
            SplineScale::Odds => 1.0 - 2.0 * logistic(predictor),
            SplineScale::Normal => f64::NAN,
        };
        basis
            .iter()
            .zip(derivative_basis)
            .map(|(basis, derivative_basis)| derivative_basis / derivative + basis * link)
            .collect()
    }

    fn log_survival_score(&self, time: f64) -> Vec<f64> {
        if time == 0.0 {
            return vec![0.0; self.coefficients.len()];
        }
        let (basis, _) = self.basis_and_derivative(self.transformed_time(time));
        let predictor = basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let link = match self.scale {
            SplineScale::Hazard => -predictor.exp(),
            SplineScale::Odds => -logistic(predictor),
            SplineScale::Normal => f64::NAN,
        };
        basis.into_iter().map(|basis| basis * link).collect()
    }

    fn log_density_hessian(&self, time: f64) -> Vec<f64> {
        let transformed = self.transformed_time(time);
        let (basis, derivative_basis) = self.basis_and_derivative(transformed);
        let predictor = basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let derivative = derivative_basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let link = match self.scale {
            SplineScale::Hazard => predictor.exp(),
            SplineScale::Odds => {
                let probability = logistic(predictor);
                2.0 * probability * (1.0 - probability)
            }
            SplineScale::Normal => f64::NAN,
        };
        let order = basis.len();
        let mut hessian = vec![0.0; order * order];
        for row in 0..order {
            for column in 0..order {
                hessian[row * order + column] = -derivative_basis[row] * derivative_basis[column]
                    / derivative.powi(2)
                    - basis[row] * basis[column] * link;
            }
        }
        hessian
    }

    fn log_survival_hessian(&self, time: f64) -> Vec<f64> {
        let order = self.coefficients.len();
        if time == 0.0 {
            return vec![0.0; order * order];
        }
        let (basis, _) = self.basis_and_derivative(self.transformed_time(time));
        let predictor = basis
            .iter()
            .zip(&self.coefficients)
            .map(|(value, coefficient)| value * coefficient)
            .sum::<f64>();
        let link = match self.scale {
            SplineScale::Hazard => predictor.exp(),
            SplineScale::Odds => {
                let probability = logistic(predictor);
                probability * (1.0 - probability)
            }
            SplineScale::Normal => f64::NAN,
        };
        let mut hessian = vec![0.0; order * order];
        for row in 0..order {
            for column in 0..order {
                hessian[row * order + column] = -basis[row] * basis[column] * link;
            }
        }
        hessian
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplineRegressionModel {
    rows: NonZeroUsize,
    knots: Vec<f64>,
    scale: SplineScale,
    time_scale: SplineTimeScale,
    location_covariates: Option<CovariateMatrix>,
}

impl SplineRegressionModel {
    pub fn new(
        rows: usize,
        knots: Vec<f64>,
        scale: SplineScale,
        time_scale: SplineTimeScale,
    ) -> Result<Self, SplineFitError> {
        let rows = NonZeroUsize::new(rows).ok_or(SplineFitError::EmptyRows)?;
        RoystonParmarSpline::new(vec![0.0; knots.len()], knots.clone(), scale, time_scale)?;
        Ok(Self {
            rows,
            knots,
            scale,
            time_scale,
            location_covariates: None,
        })
    }

    pub fn with_location_covariates(
        mut self,
        covariates: CovariateMatrix,
    ) -> Result<Self, SplineFitError> {
        if covariates.rows() != self.rows.get() {
            return Err(SplineFitError::RowCount {
                model: self.rows.get(),
                data: covariates.rows(),
            });
        }
        self.location_covariates = Some(covariates);
        Ok(self)
    }

    pub fn parameter_count(&self) -> usize {
        self.knots.len()
            + self
                .location_covariates
                .as_ref()
                .map(CovariateMatrix::columns)
                .unwrap_or(0)
    }

    fn distribution_at(
        &self,
        row: usize,
        parameters: &[f64],
    ) -> Result<RoystonParmarSpline, SplineError> {
        let mut coefficients = parameters[..self.knots.len()].to_vec();
        if let Some(covariates) = &self.location_covariates {
            coefficients[0] += covariates.linear_predictor(row, &parameters[self.knots.len()..]);
        }
        RoystonParmarSpline::new(
            coefficients,
            self.knots.clone(),
            self.scale,
            self.time_scale,
        )
    }

    fn expand_score(&self, row: usize, baseline: &[f64]) -> Vec<f64> {
        let mut score = baseline.to_vec();
        if let Some(covariates) = &self.location_covariates {
            for column in 0..covariates.columns() {
                score.push(covariates.value(row, column) * baseline[0]);
            }
        }
        score
    }

    fn expand_hessian(&self, row: usize, baseline: &[f64]) -> Vec<f64> {
        let baseline_order = self.knots.len();
        let order = self.parameter_count();
        let mut hessian = vec![0.0; order * order];
        for target_row in 0..order {
            let (source_row, row_multiplier) = if target_row < baseline_order {
                (target_row, 1.0)
            } else {
                (
                    0,
                    self.location_covariates
                        .as_ref()
                        .unwrap()
                        .value(row, target_row - baseline_order),
                )
            };
            for target_column in 0..order {
                let (source_column, column_multiplier) = if target_column < baseline_order {
                    (target_column, 1.0)
                } else {
                    (
                        0,
                        self.location_covariates
                            .as_ref()
                            .unwrap()
                            .value(row, target_column - baseline_order),
                    )
                };
                hessian[target_row * order + target_column] = baseline
                    [source_row * baseline_order + source_column]
                    * row_multiplier
                    * column_multiplier;
            }
        }
        hessian
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplineFitPlan {
    model: SplineRegressionModel,
    data: SurvivalDataset,
    initial: Vec<f64>,
    fixed: BTreeSet<usize>,
    control: ROptimControl,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SplineFit {
    pub parameters: Vec<f64>,
    pub spline_coefficients: Vec<f64>,
    pub regression_coefficients: Vec<f64>,
    pub individual_log_likelihood: Vec<f64>,
    pub log_likelihood: f64,
    pub estimated_parameter_count: usize,
    pub aic: f64,
    pub bic: f64,
    pub optimization: OptimizationEvidence,
    pub uncertainty: UncertaintyEvidence,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SplineFitError {
    EmptyRows,
    RowCount {
        model: usize,
        data: usize,
    },
    InitialCount {
        expected: usize,
        actual: usize,
    },
    NonFiniteInitial {
        index: usize,
    },
    FixedParameterOutsideModel {
        index: usize,
        parameter_count: usize,
    },
    NoFreeParameters,
    OptimizerControlLength {
        expected: usize,
        actual: usize,
    },
    ImpossibleObservation {
        row: usize,
    },
    NonFiniteLikelihood,
    Spline(SplineError),
    Optimizer(ROptimError),
    Covariance(CovarianceError),
}

impl From<SplineError> for SplineFitError {
    fn from(value: SplineError) -> Self {
        Self::Spline(value)
    }
}

impl From<ROptimError> for SplineFitError {
    fn from(value: ROptimError) -> Self {
        Self::Optimizer(value)
    }
}

impl From<CovarianceError> for SplineFitError {
    fn from(value: CovarianceError) -> Self {
        Self::Covariance(value)
    }
}

impl SplineFitPlan {
    pub fn optimize(
        model: SplineRegressionModel,
        data: SurvivalDataset,
        spline_coefficients: &[f64],
        regression_coefficients: &[f64],
        fixed_parameters: &[usize],
        control: ROptimControl,
    ) -> Result<Self, SplineFitError> {
        if model.rows.get() != data.len() {
            return Err(SplineFitError::RowCount {
                model: model.rows.get(),
                data: data.len(),
            });
        }
        let mut initial = spline_coefficients.to_vec();
        initial.extend_from_slice(regression_coefficients);
        if initial.len() != model.parameter_count() {
            return Err(SplineFitError::InitialCount {
                expected: model.parameter_count(),
                actual: initial.len(),
            });
        }
        if let Some(index) = initial.iter().position(|value| !value.is_finite()) {
            return Err(SplineFitError::NonFiniteInitial { index });
        }
        let mut fixed = BTreeSet::new();
        for index in fixed_parameters.iter().copied() {
            if index >= initial.len() {
                return Err(SplineFitError::FixedParameterOutsideModel {
                    index,
                    parameter_count: initial.len(),
                });
            }
            fixed.insert(index);
        }
        let free = initial.len() - fixed.len();
        if free == 0 {
            return Err(SplineFitError::NoFreeParameters);
        }
        if control.parameter_count() != free {
            return Err(SplineFitError::OptimizerControlLength {
                expected: free,
                actual: control.parameter_count(),
            });
        }
        Ok(Self {
            model,
            data,
            initial,
            fixed,
            control,
        })
    }
}

fn insert_free(initial: &[f64], free: &[usize], candidate: &[f64]) -> Vec<f64> {
    let mut full = initial.to_vec();
    for (index, value) in free.iter().zip(candidate) {
        full[*index] = *value;
    }
    full
}

fn likelihood_at(
    model: &SplineRegressionModel,
    data: &SurvivalDataset,
    parameters: &[f64],
) -> Result<(f64, Vec<f64>), SplineFitError> {
    let mut individual = Vec::with_capacity(data.len());
    for (row, record) in data.records().iter().copied().enumerate() {
        let distribution = model.distribution_at(row, parameters)?;
        let bounds = record.observation().bounds();
        let log_observation = if bounds.exact {
            distribution.evaluate(bounds.lower)?.density.ln()
        } else {
            let upper = if bounds.upper.is_infinite() {
                1.0
            } else {
                distribution.cdf(bounds.upper)?
            };
            let probability = upper - distribution.cdf(bounds.lower)?;
            if probability <= 0.0 {
                return Err(SplineFitError::ImpossibleObservation { row });
            }
            probability.ln()
        };
        let upper = match record.right_truncation() {
            Some(time) => distribution.cdf(time)?,
            None => 1.0,
        };
        let observed_probability = upper - distribution.cdf(bounds.entry)?;
        if observed_probability <= 0.0 {
            return Err(SplineFitError::ImpossibleObservation { row });
        }
        individual.push((log_observation - observed_probability.ln()) * record.weight());
    }
    let total = individual.iter().sum::<f64>();
    if total.is_finite() {
        Ok((total, individual))
    } else {
        Err(SplineFitError::NonFiniteLikelihood)
    }
}

fn supports_analytic_gradient(model: &SplineRegressionModel, data: &SurvivalDataset) -> bool {
    model.scale != SplineScale::Normal
        && data.records().iter().all(|record| {
            let bounds = record.observation().bounds();
            (bounds.exact || bounds.upper.is_infinite()) && record.right_truncation().is_none()
        })
}

fn gradient_at(
    model: &SplineRegressionModel,
    data: &SurvivalDataset,
    parameters: &[f64],
) -> Vec<f64> {
    let mut gradient = vec![0.0; parameters.len()];
    for (row, record) in data.records().iter().copied().enumerate() {
        let distribution = model.distribution_at(row, parameters).unwrap();
        let bounds = record.observation().bounds();
        let observed = if bounds.exact {
            distribution.log_density_score(bounds.lower)
        } else {
            distribution.log_survival_score(bounds.lower)
        };
        let entry = distribution.log_survival_score(bounds.entry);
        let baseline = observed
            .iter()
            .zip(entry)
            .map(|(observed, entry)| observed - entry)
            .collect::<Vec<_>>();
        for (total, value) in gradient.iter_mut().zip(model.expand_score(row, &baseline)) {
            *total -= value * record.weight();
        }
    }
    gradient
}

fn hessian_at(
    model: &SplineRegressionModel,
    data: &SurvivalDataset,
    parameters: &[f64],
) -> Vec<f64> {
    let order = parameters.len();
    let mut hessian = vec![0.0; order * order];
    for (row, record) in data.records().iter().copied().enumerate() {
        let distribution = model.distribution_at(row, parameters).unwrap();
        let bounds = record.observation().bounds();
        let observed = if bounds.exact {
            distribution.log_density_hessian(bounds.lower)
        } else {
            distribution.log_survival_hessian(bounds.lower)
        };
        let entry = distribution.log_survival_hessian(bounds.entry);
        let baseline = observed
            .iter()
            .zip(entry)
            .map(|(observed, entry)| observed - entry)
            .collect::<Vec<_>>();
        for (total, value) in hessian.iter_mut().zip(model.expand_hessian(row, &baseline)) {
            *total -= value * record.weight();
        }
    }
    hessian
}

pub fn fit_spline(plan: SplineFitPlan) -> Result<SplineFit, SplineFitError> {
    let free = (0..plan.initial.len())
        .filter(|index| !plan.fixed.contains(index))
        .collect::<Vec<_>>();
    let initial = free
        .iter()
        .map(|index| plan.initial[*index])
        .collect::<Vec<_>>();
    let analytic = supports_analytic_gradient(&plan.model, &plan.data);
    let optimized = if analytic {
        r_optim_bfgs(
            &initial,
            &plan.control,
            |candidate| {
                let full = insert_free(&plan.initial, &free, candidate);
                likelihood_at(&plan.model, &plan.data, &full)
                    .map(|(value, _)| -value)
                    .unwrap_or(f64::INFINITY)
            },
            |candidate| {
                let full = insert_free(&plan.initial, &free, candidate);
                let gradient = gradient_at(&plan.model, &plan.data, &full);
                free.iter().map(|index| gradient[*index]).collect()
            },
        )?
    } else {
        r_optim_bfgs_numeric(&initial, &plan.control, |candidate| {
            let full = insert_free(&plan.initial, &free, candidate);
            likelihood_at(&plan.model, &plan.data, &full)
                .map(|(value, _)| -value)
                .unwrap_or(f64::INFINITY)
        })?
    };
    let parameters = insert_free(&plan.initial, &free, &optimized.parameters);
    let free_parameters = free
        .iter()
        .map(|index| parameters[*index])
        .collect::<Vec<_>>();
    let hessian = if analytic {
        let full = hessian_at(&plan.model, &plan.data, &parameters);
        let order = parameters.len();
        let mut selected = Vec::with_capacity(free.len() * free.len());
        for row in &free {
            for column in &free {
                selected.push(full[*row * order + *column]);
            }
        }
        selected
    } else {
        r_optim_hessian_numeric(&free_parameters, &plan.control, |candidate| {
            let full = insert_free(&parameters, &free, candidate);
            likelihood_at(&plan.model, &plan.data, &full)
                .map(|(value, _)| -value)
                .unwrap_or(f64::INFINITY)
        })?
    };
    let covariance = hessian_to_covariance(&hessian, free.len())?;
    let confidence_level = 0.95;
    let quantile = spec_math::cephes64::ndtri(0.5 + confidence_level / 2.0);
    let mut intervals = vec![None; parameters.len()];
    for (free_index, parameter_index) in free.iter().copied().enumerate() {
        let standard_error = covariance.values[free_index * free.len() + free_index].sqrt();
        let estimate = parameters[parameter_index];
        intervals[parameter_index] = Some(ParameterInterval {
            standard_error,
            lower: estimate - quantile * standard_error,
            upper: estimate + quantile * standard_error,
        });
    }
    let uncertainty = UncertaintyEvidence::Hessian {
        confidence_level,
        covariance: covariance.values,
        covariance_order: covariance.order,
        transformed: intervals.clone(),
        natural: intervals,
        smallest_unrepaired_eigenvalue: covariance.smallest_unrepaired_eigenvalue,
        positive_definite_repair: covariance.repaired,
    };
    let (log_likelihood, individual_log_likelihood) =
        likelihood_at(&plan.model, &plan.data, &parameters)?;
    let sample_size = plan
        .data
        .records()
        .iter()
        .map(|record| record.weight())
        .sum::<f64>();
    let estimated_parameter_count = free.len();
    let knot_count = plan.model.knots.len();
    Ok(SplineFit {
        spline_coefficients: parameters[..knot_count].to_vec(),
        regression_coefficients: parameters[knot_count..].to_vec(),
        parameters,
        individual_log_likelihood,
        log_likelihood,
        estimated_parameter_count,
        aic: -2.0 * log_likelihood + 2.0 * estimated_parameter_count as f64,
        bic: -2.0 * log_likelihood + estimated_parameter_count as f64 * sample_size.ln(),
        optimization: OptimizationEvidence::Optimized {
            convergence: if optimized.convergence == 0 {
                OptimizerConvergence::Converged
            } else {
                OptimizerConvergence::IterationLimit
            },
            gradient: if analytic {
                GradientRecipe::FlexSurvAnalytic
            } else {
                GradientRecipe::BaseRCentralDifference
            },
            function_count: optimized.function_count,
            gradient_count: optimized.gradient_count,
            iterations: optimized.iterations,
        },
        uncertainty,
    })
}
