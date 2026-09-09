//! `flexsurvreg` likelihood fitting with Base R's BFGS numerical recipe.

use std::collections::BTreeSet;

use super::covariance::{hessian_to_covariance, CovarianceError};
use super::initial::{automatic_initial_values, InitialValueError};
use super::likelihood::{BackgroundMortality, LikelihoodError, LikelihoodRow};
use super::model::{ModelError, RegressionModel};
use super::observation::{NonNegativeTime, SurvivalObservation};
use super::r_optim::{
    r_optim_bfgs, r_optim_bfgs_numeric, r_optim_hessian, r_optim_hessian_numeric, ROptimControl,
    ROptimError,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalRecord {
    observation: SurvivalObservation,
    weight: f64,
    right_truncation: Option<NonNegativeTime>,
    background: Option<BackgroundMortality>,
}

impl SurvivalRecord {
    pub fn new(observation: SurvivalObservation) -> Self {
        Self {
            observation,
            weight: 1.0,
            right_truncation: None,
            background: None,
        }
    }

    pub fn weighted(observation: SurvivalObservation, weight: f64) -> Result<Self, FitError> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(FitError::InvalidWeight);
        }
        Ok(Self {
            observation,
            weight,
            right_truncation: None,
            background: None,
        })
    }

    pub fn with_right_truncation(mut self, time: f64) -> Result<Self, FitError> {
        let time = NonNegativeTime::new(time)
            .map_err(LikelihoodError::Observation)
            .map_err(FitError::Likelihood)?;
        if time.get() < self.observation.bounds().lower {
            return Err(FitError::Likelihood(
                LikelihoodError::RightTruncationBeforeObservation,
            ));
        }
        if self.background.is_some() {
            return Err(FitError::Likelihood(
                LikelihoodError::BackgroundMortalityWithRightTruncation,
            ));
        }
        self.right_truncation = Some(time);
        Ok(self)
    }

    pub fn with_background_mortality(
        mut self,
        background: BackgroundMortality,
    ) -> Result<Self, FitError> {
        if self.right_truncation.is_some() {
            return Err(FitError::Likelihood(
                LikelihoodError::BackgroundMortalityWithRightTruncation,
            ));
        }
        // The row validates that the typed background input matches this
        // observation's exact-event or censoring form.
        LikelihoodRow::new(
            self.observation,
            super::distribution::FlexSurvDistribution::exponential(1.0)
                .expect("the constant exponential distribution is valid"),
            self.weight,
        )?
        .with_background_mortality(background)?;
        self.background = Some(background);
        Ok(self)
    }

    fn likelihood_row(
        self,
        distribution: super::distribution::FlexSurvDistribution,
    ) -> Result<LikelihoodRow, LikelihoodError> {
        let row = LikelihoodRow::new(self.observation, distribution, self.weight)?;
        let row = match self.right_truncation {
            Some(time) => row.with_right_truncation(time.get())?,
            None => row,
        };
        match self.background {
            Some(background) => row.with_background_mortality(background),
            None => Ok(row),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurvivalDataset(Vec<SurvivalRecord>);

impl SurvivalDataset {
    pub fn new(records: Vec<SurvivalRecord>) -> Result<Self, FitError> {
        if records.is_empty() {
            return Err(FitError::EmptyDataset);
        }
        let counting = records[0].observation.is_counting();
        if records
            .iter()
            .any(|record| record.observation.is_counting() != counting)
        {
            return Err(FitError::MixedObservationForms);
        }
        Ok(Self(records))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub(crate) fn records(&self) -> &[SurvivalRecord] {
        &self.0
    }
}

impl SurvivalRecord {
    pub(crate) fn observation(self) -> SurvivalObservation {
        self.observation
    }

    pub(crate) fn weight(self) -> f64 {
        self.weight
    }

    pub(crate) fn right_truncation(self) -> Option<f64> {
        self.right_truncation.map(NonNegativeTime::get)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum FitPlanKind {
    AllFixed,
    Optimize {
        fixed: BTreeSet<usize>,
        control: ROptimControl,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexSurvFitPlan {
    model: RegressionModel,
    data: SurvivalDataset,
    transformed_initial: Vec<f64>,
    kind: FitPlanKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationEvidence {
    AllParametersFixed,
    Optimized {
        convergence: OptimizerConvergence,
        gradient: GradientRecipe,
        function_count: usize,
        gradient_count: usize,
        iterations: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GradientRecipe {
    FlexSurvAnalytic,
    BaseRCentralDifference,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptimizerConvergence {
    Converged,
    IterationLimit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexSurvFit {
    pub(crate) model: RegressionModel,
    pub transformed_parameters: Vec<f64>,
    pub natural_baseline: Vec<f64>,
    pub individual_log_likelihood: Vec<f64>,
    pub log_likelihood: f64,
    pub estimated_parameter_count: usize,
    pub aic: f64,
    pub bic: f64,
    pub optimization: OptimizationEvidence,
    pub uncertainty: UncertaintyEvidence,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParameterInterval {
    pub standard_error: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UncertaintyEvidence {
    AllParametersFixed,
    Hessian {
        confidence_level: f64,
        covariance: Vec<f64>,
        covariance_order: usize,
        transformed: Vec<Option<ParameterInterval>>,
        natural: Vec<Option<ParameterInterval>>,
        smallest_unrepaired_eigenvalue: f64,
        positive_definite_repair: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum FitError {
    EmptyDataset,
    MixedObservationForms,
    InvalidWeight,
    RowCount {
        model: usize,
        data: usize,
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
    Model(ModelError),
    Likelihood(LikelihoodError),
    Optimizer(ROptimError),
    Covariance(CovarianceError),
    InitialValues(InitialValueError),
    NonFiniteLikelihood,
}

impl From<ModelError> for FitError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<LikelihoodError> for FitError {
    fn from(value: LikelihoodError) -> Self {
        Self::Likelihood(value)
    }
}

impl From<ROptimError> for FitError {
    fn from(value: ROptimError) -> Self {
        Self::Optimizer(value)
    }
}

impl From<CovarianceError> for FitError {
    fn from(value: CovarianceError) -> Self {
        Self::Covariance(value)
    }
}

impl From<InitialValueError> for FitError {
    fn from(value: InitialValueError) -> Self {
        Self::InitialValues(value)
    }
}

impl FlexSurvFitPlan {
    pub fn all_fixed(
        model: RegressionModel,
        data: SurvivalDataset,
        natural_baseline: &[f64],
        coefficients: &[f64],
    ) -> Result<Self, FitError> {
        Self::new(
            model,
            data,
            natural_baseline,
            coefficients,
            FitPlanKind::AllFixed,
        )
    }

    pub fn optimize(
        model: RegressionModel,
        data: SurvivalDataset,
        natural_baseline: &[f64],
        coefficients: &[f64],
        fixed_parameters: &[usize],
        control: ROptimControl,
    ) -> Result<Self, FitError> {
        let parameter_count = model.parameter_count();
        let mut fixed = BTreeSet::new();
        for index in fixed_parameters.iter().copied() {
            if index >= parameter_count {
                return Err(FitError::FixedParameterOutsideModel {
                    index,
                    parameter_count,
                });
            }
            fixed.insert(index);
        }
        let free_count = parameter_count - fixed.len();
        if free_count == 0 {
            return Err(FitError::NoFreeParameters);
        }
        if control.parameter_count() != free_count {
            return Err(FitError::OptimizerControlLength {
                expected: free_count,
                actual: control.parameter_count(),
            });
        }
        Self::new(
            model,
            data,
            natural_baseline,
            coefficients,
            FitPlanKind::Optimize { fixed, control },
        )
    }

    pub fn optimize_automatic(
        model: RegressionModel,
        data: SurvivalDataset,
        control: ROptimControl,
    ) -> Result<Self, FitError> {
        let (natural_baseline, coefficients) = automatic_initial_values(&model, &data)?;
        Self::optimize(model, data, &natural_baseline, &coefficients, &[], control)
    }

    fn new(
        model: RegressionModel,
        data: SurvivalDataset,
        natural_baseline: &[f64],
        coefficients: &[f64],
        kind: FitPlanKind,
    ) -> Result<Self, FitError> {
        if model.rows() != data.len() {
            return Err(FitError::RowCount {
                model: model.rows(),
                data: data.len(),
            });
        }
        let transformed_initial =
            model.transformed_initial_values(natural_baseline, coefficients)?;
        Ok(Self {
            model,
            data,
            transformed_initial,
            kind,
        })
    }
}

fn likelihood_at(
    model: &RegressionModel,
    data: &SurvivalDataset,
    transformed: &[f64],
) -> Result<(f64, Vec<f64>), FitError> {
    let mut individual = Vec::with_capacity(data.len());
    for (row, record) in data.0.iter().copied().enumerate() {
        let distribution = model
            .distribution_at(row, transformed)
            .map_err(ModelError::from)?;
        individual.push(record.likelihood_row(distribution)?.contribution()?);
    }
    let total = individual.iter().sum::<f64>();
    if total.is_finite() {
        Ok((total, individual))
    } else {
        Err(FitError::NonFiniteLikelihood)
    }
}

fn analytic_gradient_at(
    model: &RegressionModel,
    data: &SurvivalDataset,
    transformed: &[f64],
) -> Vec<f64> {
    let mut gradient = vec![0.0; model.parameter_count()];
    for (row, record) in data.0.iter().copied().enumerate() {
        let bounds = record.observation.bounds();
        let parameters = model.natural_parameters_at(row, transformed);
        let observed = model
            .family()
            .score(bounds.exact, bounds.lower, &parameters);
        let entry = model.family().score(false, bounds.entry, &parameters);
        let baseline = observed
            .iter()
            .zip(entry)
            .map(|(observed, entry)| observed - entry)
            .collect::<Vec<_>>();
        let mut expanded = model.expand_score(row, &baseline);
        if let Some(background) = record
            .background
            .and_then(BackgroundMortality::event_hazard_value)
        {
            let distribution = model
                .distribution_at(row, transformed)
                .expect("validated fit parameters form a distribution");
            let hazard = distribution
                .evaluate(bounds.lower)
                .expect("validated event time can be evaluated")
                .hazard;
            let survival_score = model.family().score(false, bounds.lower, &parameters);
            let hazard_score = observed
                .iter()
                .zip(survival_score)
                .map(|(density, survival)| density - survival)
                .collect::<Vec<_>>();
            let hazard_score = model.expand_score(row, &hazard_score);
            let offset = 1.0 / (1.0 + background / hazard);
            for (value, score) in expanded.iter_mut().zip(hazard_score) {
                *value -= offset * background * score / hazard;
            }
        }
        for (total, value) in gradient.iter_mut().zip(expanded) {
            *total -= value * record.weight;
        }
    }
    gradient
}

fn supports_analytic_gradient(model: &RegressionModel, data: &SurvivalDataset) -> bool {
    model.family().has_analytic_gradient()
        && data.0.iter().all(|record| {
            let bounds = record.observation.bounds();
            bounds.exact || bounds.upper.is_infinite()
        })
}

fn supports_analytic_hessian(model: &RegressionModel, data: &SurvivalDataset) -> bool {
    model.family().has_analytic_hessian()
        && data.0.iter().all(|record| {
            let bounds = record.observation.bounds();
            bounds.exact || bounds.upper.is_infinite()
        })
}

fn analytic_hessian_at(
    model: &RegressionModel,
    data: &SurvivalDataset,
    transformed: &[f64],
) -> Vec<f64> {
    let count = model.parameter_count();
    let mut hessian = vec![0.0; count * count];
    for (row, record) in data.0.iter().copied().enumerate() {
        let bounds = record.observation.bounds();
        let parameters = model.natural_parameters_at(row, transformed);
        let observed = model
            .family()
            .second_score(bounds.exact, bounds.lower, &parameters);
        let entry = model
            .family()
            .second_score(false, bounds.entry, &parameters);
        let baseline = observed
            .iter()
            .zip(entry)
            .map(|(observed, entry)| observed - entry)
            .collect::<Vec<_>>();
        let mut expanded = model.expand_second_score(row, &baseline);
        if let Some(background) = record
            .background
            .and_then(BackgroundMortality::event_hazard_value)
        {
            let distribution = model
                .distribution_at(row, transformed)
                .expect("validated fit parameters form a distribution");
            let hazard = distribution
                .evaluate(bounds.lower)
                .expect("validated event time can be evaluated")
                .hazard;
            let density_score =
                model.expand_score(row, &model.family().score(true, bounds.lower, &parameters));
            let survival_score =
                model.expand_score(row, &model.family().score(false, bounds.lower, &parameters));
            let density_second = model.expand_second_score(
                row,
                &model.family().second_score(true, bounds.lower, &parameters),
            );
            let survival_second = model.expand_second_score(
                row,
                &model
                    .family()
                    .second_score(false, bounds.lower, &parameters),
            );
            let offset = 1.0 / (1.0 + background / hazard);
            let dhazinv = survival_score
                .iter()
                .zip(&density_score)
                .map(|(survival, density)| (survival - density) / hazard)
                .collect::<Vec<_>>();
            for i in 0..count {
                for j in 0..count {
                    let index = i * count + j;
                    let correction = -offset
                        * background
                        * (offset * dhazinv[i] * dhazinv[j] * background
                            + (density_second[index] - survival_second[index]) / hazard
                            + (density_score[i] - survival_score[i]) * dhazinv[j]);
                    expanded[index] += correction;
                }
            }
        }
        for (total, value) in hessian.iter_mut().zip(expanded) {
            *total -= value * record.weight;
        }
    }
    hessian
}

pub(crate) fn hessian_at(
    model: &RegressionModel,
    data: &SurvivalDataset,
    transformed: &[f64],
    control: &ROptimControl,
) -> Result<Vec<f64>, FitError> {
    let hessian = if supports_analytic_hessian(model, data) {
        analytic_hessian_at(model, data, transformed)
    } else if supports_analytic_gradient(model, data) {
        r_optim_hessian(transformed, control, |candidate| {
            analytic_gradient_at(model, data, candidate)
        })?
    } else {
        r_optim_hessian_numeric(transformed, control, |candidate| {
            likelihood_at(model, data, candidate)
                .map(|(value, _)| -value)
                .unwrap_or(f64::INFINITY)
        })?
    };
    Ok(hessian)
}

fn insert_free(initial: &[f64], free: &[usize], candidate: &[f64]) -> Vec<f64> {
    let mut full = initial.to_vec();
    for (index, value) in free.iter().zip(candidate.iter()) {
        full[*index] = *value;
    }
    full
}

pub fn fit(plan: FlexSurvFitPlan) -> Result<FlexSurvFit, FitError> {
    let (transformed_parameters, estimated_parameter_count, optimization) = match &plan.kind {
        FitPlanKind::AllFixed => (
            plan.transformed_initial.clone(),
            0,
            OptimizationEvidence::AllParametersFixed,
        ),
        FitPlanKind::Optimize { fixed, control } => {
            let free = (0..plan.model.parameter_count())
                .filter(|index| !fixed.contains(index))
                .collect::<Vec<_>>();
            let initial = free
                .iter()
                .map(|index| plan.transformed_initial[*index])
                .collect::<Vec<_>>();
            let analytic = supports_analytic_gradient(&plan.model, &plan.data);
            let result = if analytic {
                r_optim_bfgs(
                    &initial,
                    control,
                    |candidate| {
                        let full = insert_free(&plan.transformed_initial, &free, candidate);
                        match likelihood_at(&plan.model, &plan.data, &full) {
                            Ok((value, _)) => -value,
                            Err(_) => f64::INFINITY,
                        }
                    },
                    |candidate| {
                        let full = insert_free(&plan.transformed_initial, &free, candidate);
                        let gradient = analytic_gradient_at(&plan.model, &plan.data, &full);
                        free.iter().map(|index| gradient[*index]).collect()
                    },
                )?
            } else {
                r_optim_bfgs_numeric(&initial, control, |candidate| {
                    let full = insert_free(&plan.transformed_initial, &free, candidate);
                    match likelihood_at(&plan.model, &plan.data, &full) {
                        Ok((value, _)) => -value,
                        Err(_) => f64::INFINITY,
                    }
                })?
            };
            let full = insert_free(&plan.transformed_initial, &free, &result.parameters);
            let convergence = if result.convergence == 0 {
                OptimizerConvergence::Converged
            } else {
                OptimizerConvergence::IterationLimit
            };
            (
                full,
                free.len(),
                OptimizationEvidence::Optimized {
                    convergence,
                    gradient: if analytic {
                        GradientRecipe::FlexSurvAnalytic
                    } else {
                        GradientRecipe::BaseRCentralDifference
                    },
                    function_count: result.function_count,
                    gradient_count: result.gradient_count,
                    iterations: result.iterations,
                },
            )
        }
    };

    let uncertainty = match &plan.kind {
        FitPlanKind::AllFixed => UncertaintyEvidence::AllParametersFixed,
        FitPlanKind::Optimize { fixed, control } => {
            let free = (0..plan.model.parameter_count())
                .filter(|index| !fixed.contains(index))
                .collect::<Vec<_>>();
            let free_parameters = free
                .iter()
                .map(|index| transformed_parameters[*index])
                .collect::<Vec<_>>();
            let hessian = if supports_analytic_hessian(&plan.model, &plan.data) {
                let full = analytic_hessian_at(&plan.model, &plan.data, &transformed_parameters);
                free.iter()
                    .flat_map(|row| {
                        free.iter()
                            .map(|column| full[*row * plan.model.parameter_count() + *column])
                    })
                    .collect::<Vec<_>>()
            } else if supports_analytic_gradient(&plan.model, &plan.data) {
                r_optim_hessian(&free_parameters, control, |candidate| {
                    let full = insert_free(&transformed_parameters, &free, candidate);
                    let gradient = analytic_gradient_at(&plan.model, &plan.data, &full);
                    free.iter().map(|index| gradient[*index]).collect()
                })?
            } else {
                r_optim_hessian_numeric(&free_parameters, control, |candidate| {
                    let full = insert_free(&transformed_parameters, &free, candidate);
                    match likelihood_at(&plan.model, &plan.data, &full) {
                        Ok((value, _)) => -value,
                        Err(_) => f64::INFINITY,
                    }
                })?
            };
            let covariance = hessian_to_covariance(&hessian, free.len())?;
            let confidence_level = 0.95;
            let quantile = spec_math::cephes64::ndtri(0.5 + confidence_level / 2.0);
            let mut transformed_intervals = vec![None; plan.model.parameter_count()];
            let mut natural_intervals = vec![None; plan.model.parameter_count()];
            for (free_index, parameter_index) in free.iter().copied().enumerate() {
                let standard_error = covariance.values[free_index * free.len() + free_index].sqrt();
                let estimate = transformed_parameters[parameter_index];
                let lower = estimate - quantile * standard_error;
                let upper = estimate + quantile * standard_error;
                transformed_intervals[parameter_index] = Some(ParameterInterval {
                    standard_error,
                    lower,
                    upper,
                });
                let natural_lower = plan.model.natural_parameter_value(parameter_index, lower);
                let natural_upper = plan.model.natural_parameter_value(parameter_index, upper);
                let natural_standard_error =
                    plan.model
                        .natural_standard_error(parameter_index, estimate, standard_error);
                natural_intervals[parameter_index] = Some(ParameterInterval {
                    standard_error: natural_standard_error,
                    lower: natural_lower,
                    upper: natural_upper,
                });
            }
            UncertaintyEvidence::Hessian {
                confidence_level,
                covariance: covariance.values,
                covariance_order: covariance.order,
                transformed: transformed_intervals,
                natural: natural_intervals,
                smallest_unrepaired_eigenvalue: covariance.smallest_unrepaired_eigenvalue,
                positive_definite_repair: covariance.repaired,
            }
        }
    };

    let (log_likelihood, individual_log_likelihood) =
        likelihood_at(&plan.model, &plan.data, &transformed_parameters)?;
    let sample_size = plan.data.0.iter().map(|row| row.weight).sum::<f64>();
    let parameter_count = estimated_parameter_count as f64;
    Ok(FlexSurvFit {
        natural_baseline: plan.model.natural_baseline(&transformed_parameters)?,
        model: plan.model,
        transformed_parameters,
        individual_log_likelihood,
        log_likelihood,
        estimated_parameter_count,
        aic: -2.0 * log_likelihood + 2.0 * parameter_count,
        bic: -2.0 * log_likelihood + parameter_count * sample_size.ln(),
        optimization,
        uncertainty,
    })
}
