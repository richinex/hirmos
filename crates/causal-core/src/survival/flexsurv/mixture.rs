//! `flexsurvmix` competing-event mixture likelihood and direct BFGS fit.

use super::covariance::{eigen_covariance_root, hessian_to_covariance, CovarianceError};
use super::distribution::{DistributionError, FlexSurvDistribution};
use super::fit::{
    fit as fit_flexsurv, hessian_at as flexsurv_hessian_at, FitError, FlexSurvFitPlan,
    SurvivalDataset, SurvivalRecord,
};
use super::likelihood::{LikelihoodError, LikelihoodRow};
use super::model::{
    CovariateMatrix, DistributionParameter, FlexSurvFamily, ModelError, RegressionModel,
};
use super::observation::SurvivalObservation;
use super::r_optim::{r_optim_bfgs_numeric, ROptimControl, ROptimError};
use super::richardson::{richardson_hessian, RichardsonError};
use super::uncertainty::type_seven_quantile;
use crate::survival::r_rng::RRng;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventKnowledge {
    Known(usize),
    Unknown,
    Possible(Vec<usize>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureObservation {
    pub survival: SurvivalObservation,
    pub event: EventKnowledge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureComponent {
    name: String,
    family: FlexSurvFamily,
    natural_initial: Vec<f64>,
    designs: Vec<(DistributionParameter, CovariateMatrix, Vec<f64>)>,
}

impl MixtureComponent {
    pub fn new(name: impl Into<String>, family: FlexSurvFamily, natural_initial: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            family,
            natural_initial,
            designs: Vec::new(),
        }
    }

    pub fn with_location_covariates(
        mut self,
        matrix: CovariateMatrix,
        initial: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        let parameter = self.family.location_parameter();
        self = self.with_parameter_covariates(parameter, matrix, initial)?;
        Ok(self)
    }

    pub fn with_parameter_covariates(
        mut self,
        parameter: DistributionParameter,
        matrix: CovariateMatrix,
        initial: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        if initial.len() != matrix.columns() {
            return Err(MixtureError::ComponentCoefficientLength {
                expected: matrix.columns(),
                actual: initial.len(),
            });
        }
        if self
            .designs
            .iter()
            .any(|(existing, _, _)| *existing == parameter)
        {
            return Err(MixtureError::DuplicateComponentParameterDesign(parameter));
        }
        self.designs.push((parameter, matrix, initial));
        Ok(self)
    }

    fn model(&self, rows: usize) -> Result<RegressionModel, MixtureError> {
        let mut model = RegressionModel::new(self.family, rows)?;
        for (parameter, matrix, _) in &self.designs {
            model = model.with_parameter_covariates(*parameter, matrix.clone())?;
        }
        Ok(model)
    }

    fn coefficient_initial(&self) -> Vec<f64> {
        let location = self.family.location_parameter();
        std::iter::once(location)
            .chain(
                self.family
                    .parameters()
                    .iter()
                    .copied()
                    .filter(|parameter| *parameter != location),
            )
            .filter_map(|parameter| {
                self.designs
                    .iter()
                    .find(|(candidate, _, _)| *candidate == parameter)
                    .map(|(_, _, values)| values.clone())
            })
            .flatten()
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexSurvMixPlan {
    observations: Vec<MixtureObservation>,
    components: Vec<MixtureComponent>,
    initial_probabilities: Vec<f64>,
    probability_design: Option<CovariateMatrix>,
    initial_probability_coefficients: Vec<f64>,
    kind: MixtureFitKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MixtureFitKind {
    Fixed,
    Direct,
    Em(EmControl),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmControl {
    relative_tolerance: f64,
    maximum_iterations: usize,
}

impl EmControl {
    pub fn new(relative_tolerance: f64, maximum_iterations: usize) -> Result<Self, MixtureError> {
        if !relative_tolerance.is_finite() || relative_tolerance <= 0.0 {
            return Err(MixtureError::InvalidEmRelativeTolerance);
        }
        if maximum_iterations == 0 {
            return Err(MixtureError::InvalidEmIterationLimit);
        }
        Ok(Self {
            relative_tolerance,
            maximum_iterations,
        })
    }

    pub fn flexsurv_defaults() -> Self {
        Self {
            relative_tolerance: f64::EPSILON.sqrt(),
            maximum_iterations: 10_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexSurvMixFit {
    pub event_names: Vec<String>,
    pub probabilities: Vec<f64>,
    pub probability_coefficients: Vec<f64>,
    pub natural_component_parameters: Vec<Vec<f64>>,
    pub component_coefficients: Vec<Vec<f64>>,
    pub component_distributions: Vec<FlexSurvDistribution>,
    component_models: Vec<RegressionModel>,
    probability_covariate_count: usize,
    pub transformed_parameters: Vec<f64>,
    pub log_likelihood: f64,
    pub function_count: usize,
    pub gradient_count: usize,
    pub iterations: usize,
    pub convergence: usize,
    pub uncertainty: MixtureUncertainty,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MixtureUncertainty {
    AllParametersFixed,
    Hessian {
        covariance: Vec<f64>,
        covariance_order: usize,
        smallest_unrepaired_eigenvalue: f64,
        positive_definite_repair: bool,
    },
}

/// One prediction profile in the covariate order used to fit each mixture
/// submodel.  The value is opaque so a profile for a different model cannot be
/// passed accidentally.
#[derive(Clone, Debug, PartialEq)]
pub struct MixturePredictionProfile {
    probability: Vec<f64>,
    components: Vec<super::model::PredictionCovariates>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MixturePredictionRequest {
    Density(f64),
    CumulativeProbability(f64),
    Quantile(f64),
    Mean,
    RestrictedMean { start: f64, end: f64 },
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureEventPrediction {
    pub event: String,
    pub event_probability: f64,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureStateProbability {
    pub state: String,
    pub value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MixtureSimulationConfiguration {
    replicates: usize,
    seed: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePredictionInterval {
    pub event: String,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

impl MixtureSimulationConfiguration {
    pub fn new(replicates: usize, seed: u32) -> Result<Self, MixtureError> {
        // `cisumm_flexsurvmix` only enters its resampling loop for B > 2.
        if replicates < 3 {
            return Err(MixtureError::InvalidSimulationReplicates);
        }
        Ok(Self { replicates, seed })
    }

    pub fn replicates(self) -> usize {
        self.replicates
    }

    pub fn seed(self) -> u32 {
        self.seed
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MixtureError {
    NoObservations,
    NoComponents,
    EmptyComponentName { component: usize },
    DuplicateComponentName { component: usize },
    InvalidInitialProbability { component: usize },
    ProbabilitiesDoNotSumToOne,
    KnownEventOutsideComponents { row: usize, event: usize },
    EmptyPossibleEvents { row: usize },
    PossibleEventOutsideComponents { row: usize, event: usize },
    DuplicatePossibleEvent { row: usize, event: usize },
    ComponentCoefficientLength { expected: usize, actual: usize },
    DuplicateComponentParameterDesign(DistributionParameter),
    ProbabilityCoefficientLength { expected: usize, actual: usize },
    PredictionProbabilityCovariateLength { expected: usize, actual: usize },
    PredictionComponentCount { expected: usize, actual: usize },
    InvalidPredictionTime,
    InvalidPredictionProbability,
    InvalidSimulationReplicates,
    CovarianceUnavailable,
    FlexSurv232RejectsPartialEvents,
    FittedComponentShape,
    Model(ModelError),
    Distribution(DistributionError),
    Likelihood(LikelihoodError),
    Optimizer(ROptimError),
    Fit(FitError),
    Richardson(RichardsonError),
    Covariance(CovarianceError),
    NonFiniteLikelihood,
    InvalidEmRelativeTolerance,
    InvalidEmIterationLimit,
    EmIterationLimit { maximum_iterations: usize },
}

impl FlexSurvMixFit {
    pub fn from_fitted_components(
        event_names: Vec<String>,
        probabilities: Vec<f64>,
        natural_component_parameters: Vec<Vec<f64>>,
        component_distributions: Vec<FlexSurvDistribution>,
    ) -> Result<Self, MixtureError> {
        let components = event_names.len();
        if components == 0
            || probabilities.len() != components
            || natural_component_parameters.len() != components
            || component_distributions.len() != components
            || event_names.iter().any(|name| name.trim().is_empty())
            || event_names
                .iter()
                .enumerate()
                .any(|(index, name)| event_names[..index].contains(name))
            || probabilities
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0 || *value > 1.0)
            || (probabilities.iter().sum::<f64>() - 1.0).abs() > 1e-12
        {
            return Err(MixtureError::FittedComponentShape);
        }
        Ok(Self {
            event_names,
            probabilities,
            probability_coefficients: Vec::new(),
            component_coefficients: vec![Vec::new(); components],
            natural_component_parameters,
            component_distributions,
            component_models: Vec::new(),
            probability_covariate_count: 0,
            transformed_parameters: Vec::new(),
            log_likelihood: 0.0,
            function_count: 0,
            gradient_count: 0,
            iterations: 0,
            convergence: 0,
            uncertainty: MixtureUncertainty::AllParametersFixed,
        })
    }

    pub fn prediction_profile(
        &self,
        probability: Vec<f64>,
        components: Vec<Vec<f64>>,
    ) -> Result<MixturePredictionProfile, MixtureError> {
        if probability.len() != self.probability_covariate_count {
            return Err(MixtureError::PredictionProbabilityCovariateLength {
                expected: self.probability_covariate_count,
                actual: probability.len(),
            });
        }
        if let Some(index) = probability.iter().position(|value| !value.is_finite()) {
            return Err(MixtureError::Model(
                ModelError::NonFinitePredictionCovariate { index },
            ));
        }
        if components.len() != self.component_models.len() {
            return Err(MixtureError::PredictionComponentCount {
                expected: self.component_models.len(),
                actual: components.len(),
            });
        }
        let components = self
            .component_models
            .iter()
            .zip(components)
            .map(|(model, values)| model.prediction_covariates(values))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MixturePredictionProfile {
            probability,
            components,
        })
    }

    fn probabilities_for(
        &self,
        profile: &MixturePredictionProfile,
        transformed: &[f64],
    ) -> Vec<f64> {
        let components = self.event_names.len();
        let mut logits = vec![0.0; components];
        for component in 1..components {
            logits[component] = transformed[component - 1];
            let offset = components - 1 + (component - 1) * self.probability_covariate_count;
            logits[component] += profile
                .probability
                .iter()
                .zip(&transformed[offset..offset + self.probability_covariate_count])
                .map(|(value, coefficient)| value * coefficient)
                .sum::<f64>();
        }
        let largest = logits.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let mut probabilities = logits
            .into_iter()
            .map(|value| (value - largest).exp())
            .collect::<Vec<_>>();
        let total = probabilities.iter().sum::<f64>();
        probabilities.iter_mut().for_each(|value| *value /= total);
        probabilities
    }

    fn distributions_for(
        &self,
        profile: &MixturePredictionProfile,
        transformed: &[f64],
    ) -> Result<Vec<FlexSurvDistribution>, MixtureError> {
        let mut offset = self.event_names.len() - 1
            + (self.event_names.len() - 1) * self.probability_covariate_count;
        self.component_models
            .iter()
            .zip(&profile.components)
            .map(|(model, covariates)| {
                let end = offset + model.parameter_count();
                let distribution =
                    model.distribution_for_prediction(covariates, &transformed[offset..end])?;
                offset = end;
                Ok(distribution)
            })
            .collect()
    }

    fn predict_events_with_parameters(
        &self,
        profile: &MixturePredictionProfile,
        request: MixturePredictionRequest,
        transformed: &[f64],
    ) -> Result<Vec<MixtureEventPrediction>, MixtureError> {
        let probabilities = self.probabilities_for(profile, transformed);
        let distributions = self.distributions_for(profile, transformed)?;
        self.event_names
            .iter()
            .cloned()
            .zip(probabilities)
            .zip(distributions)
            .map(|((event, event_probability), distribution)| {
                let value = match request {
                    MixturePredictionRequest::Density(time) => {
                        if !time.is_finite() || time < 0.0 {
                            return Err(MixtureError::InvalidPredictionTime);
                        }
                        distribution.log_density(time)?.exp()
                    }
                    MixturePredictionRequest::CumulativeProbability(time) => {
                        if !time.is_finite() || time < 0.0 {
                            return Err(MixtureError::InvalidPredictionTime);
                        }
                        distribution.cdf(time)?
                    }
                    MixturePredictionRequest::Quantile(probability) => {
                        if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
                            return Err(MixtureError::InvalidPredictionProbability);
                        }
                        distribution.quantile(probability)?
                    }
                    MixturePredictionRequest::Mean => distribution.mean()?,
                    MixturePredictionRequest::RestrictedMean { start, end } => {
                        distribution.restricted_mean(start, end)?
                    }
                };
                Ok(MixtureEventPrediction {
                    event,
                    event_probability,
                    value,
                })
            })
            .collect()
    }

    pub fn predict_events(
        &self,
        profile: &MixturePredictionProfile,
        request: MixturePredictionRequest,
    ) -> Result<Vec<MixtureEventPrediction>, MixtureError> {
        self.predict_events_with_parameters(profile, request, &self.transformed_parameters)
    }

    fn parameter_draws_with_rng(
        &self,
        configuration: MixtureSimulationConfiguration,
        rng: &mut RRng,
    ) -> Result<Vec<Vec<f64>>, MixtureError> {
        let (covariance, order) = match &self.uncertainty {
            MixtureUncertainty::AllParametersFixed => {
                return Err(MixtureError::CovarianceUnavailable)
            }
            MixtureUncertainty::Hessian {
                covariance,
                covariance_order,
                ..
            } => (covariance, *covariance_order),
        };
        if order != self.transformed_parameters.len() {
            return Err(MixtureError::Covariance(CovarianceError::HessianLength {
                expected: self.transformed_parameters.len() * self.transformed_parameters.len(),
                actual: covariance.len(),
            }));
        }
        let root = eigen_covariance_root(covariance, order)?;
        let mut draws = Vec::with_capacity(configuration.replicates);
        draws.push(self.transformed_parameters.clone());
        for _ in 1..configuration.replicates {
            let normals = (0..order).map(|_| rng.normal()).collect::<Vec<_>>();
            let mut draw = self.transformed_parameters.clone();
            for column in 0..order {
                draw[column] += (0..order)
                    .map(|row| normals[row] * root[row * order + column])
                    .sum::<f64>();
            }
            draws.push(draw);
        }
        Ok(draws)
    }

    pub(crate) fn resampled_zero_profile(&self, rng: &mut RRng) -> Result<Self, MixtureError> {
        let (covariance, order) = match &self.uncertainty {
            MixtureUncertainty::AllParametersFixed => {
                return Err(MixtureError::CovarianceUnavailable)
            }
            MixtureUncertainty::Hessian {
                covariance,
                covariance_order,
                ..
            } => (covariance, *covariance_order),
        };
        let root = eigen_covariance_root(covariance, order)?;
        let normals = (0..order).map(|_| rng.normal()).collect::<Vec<_>>();
        let mut transformed = self.transformed_parameters.clone();
        for column in 0..order {
            transformed[column] += (0..order)
                .map(|row| normals[row] * root[row * order + column])
                .sum::<f64>();
        }
        let profile = MixturePredictionProfile {
            probability: vec![0.0; self.probability_covariate_count],
            components: self
                .component_models
                .iter()
                .map(|model| model.prediction_covariates(vec![0.0; model.coefficient_count()]))
                .collect::<Result<Vec<_>, _>>()?,
        };
        let mut sampled = self.clone();
        sampled.probabilities = self.probabilities_for(&profile, &transformed);
        sampled.component_distributions = self.distributions_for(&profile, &transformed)?;
        sampled.transformed_parameters = transformed;
        Ok(sampled)
    }

    fn parameter_draws(
        &self,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<Vec<f64>>, MixtureError> {
        self.parameter_draws_with_rng(configuration, &mut RRng::new(configuration.seed))
    }

    pub fn predict_events_with_uncertainty(
        &self,
        profile: &MixturePredictionProfile,
        request: MixturePredictionRequest,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixturePredictionInterval>, MixtureError> {
        let draws = self.parameter_draws(configuration)?;
        let mut values = vec![Vec::with_capacity(draws.len()); self.event_names.len()];
        for transformed in draws {
            for (event, prediction) in self
                .predict_events_with_parameters(profile, request, &transformed)?
                .into_iter()
                .enumerate()
            {
                values[event].push(prediction.value);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(event, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixturePredictionInterval {
                    event: self.event_names[event].clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    /// Mirrors `mean_flexsurvmix`/`rmst_flexsurvmix`: each new-data row is
    /// summarized in turn, consuming the shared R RNG stream before the next
    /// profile is evaluated.
    pub fn predict_event_profiles_with_uncertainty(
        &self,
        profiles: &[MixturePredictionProfile],
        request: MixturePredictionRequest,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<Vec<MixturePredictionInterval>>, MixtureError> {
        let mut rng = RRng::new(configuration.seed);
        let mut results = Vec::with_capacity(profiles.len());
        for profile in profiles {
            let draws = self.parameter_draws_with_rng(configuration, &mut rng)?;
            let mut values = vec![Vec::with_capacity(draws.len()); self.event_names.len()];
            for transformed in draws {
                for (event, prediction) in self
                    .predict_events_with_parameters(profile, request, &transformed)?
                    .into_iter()
                    .enumerate()
                {
                    values[event].push(prediction.value);
                }
            }
            results.push(
                values
                    .into_iter()
                    .enumerate()
                    .map(|(event, mut values)| {
                        let estimate = values[0];
                        values.sort_by(f64::total_cmp);
                        MixturePredictionInterval {
                            event: self.event_names[event].clone(),
                            estimate,
                            lower: type_seven_quantile(&values, 0.025),
                            upper: type_seven_quantile(&values, 0.975),
                        }
                    })
                    .collect(),
            );
        }
        Ok(results)
    }

    pub fn predict_event_probabilities_with_uncertainty(
        &self,
        profile: &MixturePredictionProfile,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixturePredictionInterval>, MixtureError> {
        let draws = self.parameter_draws(configuration)?;
        let mut values = vec![Vec::with_capacity(draws.len()); self.event_names.len()];
        for transformed in draws {
            for (event, probability) in self
                .probabilities_for(profile, &transformed)
                .into_iter()
                .enumerate()
            {
                values[event].push(probability);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(event, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixturePredictionInterval {
                    event: self.event_names[event].clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    pub fn state_probabilities(
        &self,
        profile: &MixturePredictionProfile,
        start_state: impl Into<String>,
        time: f64,
    ) -> Result<Vec<MixtureStateProbability>, MixtureError> {
        let events = self.predict_events(
            profile,
            MixturePredictionRequest::CumulativeProbability(time),
        )?;
        let mut total = 0.0;
        let mut states = events
            .into_iter()
            .map(|event| {
                let value = event.event_probability * event.value;
                total += value;
                MixtureStateProbability {
                    state: event.event,
                    value,
                }
            })
            .collect::<Vec<_>>();
        states.insert(
            0,
            MixtureStateProbability {
                state: start_state.into(),
                value: 1.0 - total,
            },
        );
        Ok(states)
    }

    pub(crate) fn sample_event_times_with_rng(
        &self,
        profile: &MixturePredictionProfile,
        simulations: usize,
        rng: &mut RRng,
    ) -> Result<Vec<Vec<f64>>, MixtureError> {
        self.distributions_for(profile, &self.transformed_parameters)?
            .into_iter()
            .map(|distribution| {
                (0..simulations)
                    .map(|_| distribution.sample_r(rng))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(MixtureError::from)
            })
            .collect()
    }
}

impl From<ModelError> for MixtureError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for MixtureError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl From<LikelihoodError> for MixtureError {
    fn from(value: LikelihoodError) -> Self {
        Self::Likelihood(value)
    }
}

impl From<ROptimError> for MixtureError {
    fn from(value: ROptimError) -> Self {
        Self::Optimizer(value)
    }
}

impl From<FitError> for MixtureError {
    fn from(value: FitError) -> Self {
        Self::Fit(value)
    }
}

impl From<RichardsonError> for MixtureError {
    fn from(value: RichardsonError) -> Self {
        Self::Richardson(value)
    }
}

impl From<CovarianceError> for MixtureError {
    fn from(value: CovarianceError) -> Self {
        Self::Covariance(value)
    }
}

impl FlexSurvMixPlan {
    pub fn fixed(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        Self::new(
            observations,
            components,
            initial_probabilities,
            MixtureFitKind::Fixed,
            false,
        )
    }

    pub fn direct(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        Self::new(
            observations,
            components,
            initial_probabilities,
            MixtureFitKind::Direct,
            false,
        )
    }

    pub fn em(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
        control: EmControl,
    ) -> Result<Self, MixtureError> {
        Self::new(
            observations,
            components,
            initial_probabilities,
            MixtureFitKind::Em(control),
            false,
        )
    }

    /// Uses flexsurv's documented partial-event likelihood while correcting
    /// the inverted named-list guard in flexsurv 2.3.2.
    pub fn direct_with_corrected_partial_events(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        Self::new(
            observations,
            components,
            initial_probabilities,
            MixtureFitKind::Direct,
            true,
        )
    }

    pub fn fixed_with_corrected_partial_events(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        Self::new(
            observations,
            components,
            initial_probabilities,
            MixtureFitKind::Fixed,
            true,
        )
    }

    fn new(
        observations: Vec<MixtureObservation>,
        components: Vec<MixtureComponent>,
        initial_probabilities: Vec<f64>,
        kind: MixtureFitKind,
        corrected_partial_events: bool,
    ) -> Result<Self, MixtureError> {
        if observations.is_empty() {
            return Err(MixtureError::NoObservations);
        }
        if components.is_empty() {
            return Err(MixtureError::NoComponents);
        }
        let mut names = Vec::<&str>::new();
        for (component, value) in components.iter().enumerate() {
            if value.name.trim().is_empty() {
                return Err(MixtureError::EmptyComponentName { component });
            }
            if names.contains(&value.name.as_str()) {
                return Err(MixtureError::DuplicateComponentName { component });
            }
            names.push(&value.name);
            let model = value.model(observations.len())?;
            model
                .transformed_initial_values(&value.natural_initial, &value.coefficient_initial())?;
        }
        if initial_probabilities.len() != components.len() {
            return Err(MixtureError::ProbabilitiesDoNotSumToOne);
        }
        for (component, value) in initial_probabilities.iter().enumerate() {
            if !value.is_finite() || *value <= 0.0 || *value > 1.0 {
                return Err(MixtureError::InvalidInitialProbability { component });
            }
        }
        if (initial_probabilities.iter().sum::<f64>() - 1.0).abs() > 1e-12 {
            return Err(MixtureError::ProbabilitiesDoNotSumToOne);
        }
        for (row, observation) in observations.iter().enumerate() {
            match &observation.event {
                EventKnowledge::Known(event) if *event >= components.len() => {
                    return Err(MixtureError::KnownEventOutsideComponents { row, event: *event });
                }
                EventKnowledge::Possible(events) => {
                    if events.is_empty() {
                        return Err(MixtureError::EmptyPossibleEvents { row });
                    }
                    for (index, event) in events.iter().enumerate() {
                        if *event >= components.len() {
                            return Err(MixtureError::PossibleEventOutsideComponents {
                                row,
                                event: *event,
                            });
                        }
                        if events[..index].contains(event) {
                            return Err(MixtureError::DuplicatePossibleEvent {
                                row,
                                event: *event,
                            });
                        }
                    }
                }
                EventKnowledge::Known(_) | EventKnowledge::Unknown => {}
            }
        }
        if !corrected_partial_events
            && observations
                .iter()
                .any(|observation| matches!(observation.event, EventKnowledge::Possible(_)))
        {
            return Err(MixtureError::FlexSurv232RejectsPartialEvents);
        }
        Ok(Self {
            observations,
            components,
            initial_probabilities,
            probability_design: None,
            initial_probability_coefficients: Vec::new(),
            kind,
        })
    }

    pub fn with_probability_covariates(
        mut self,
        matrix: CovariateMatrix,
        initial: Vec<f64>,
    ) -> Result<Self, MixtureError> {
        let expected = (self.components.len() - 1) * matrix.columns();
        if initial.len() != expected {
            return Err(MixtureError::ProbabilityCoefficientLength {
                expected,
                actual: initial.len(),
            });
        }
        if matrix.rows() != self.observations.len() {
            return Err(MixtureError::Model(ModelError::RowCount {
                expected: self.observations.len(),
                actual: matrix.rows(),
            }));
        }
        self.probability_design = Some(matrix);
        self.initial_probability_coefficients = initial;
        Ok(self)
    }
}

fn baseline_probabilities(parameters: &[f64], components: usize) -> Vec<f64> {
    let mut logits = Vec::with_capacity(components);
    logits.push(0.0);
    logits.extend_from_slice(&parameters[..components - 1]);
    let largest = logits.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut values = logits
        .iter()
        .map(|value| (value - largest).exp())
        .collect::<Vec<_>>();
    let total = values.iter().sum::<f64>();
    for value in &mut values {
        *value /= total;
    }
    values
}

fn probabilities_at(
    parameters: &[f64],
    components: usize,
    design: Option<&CovariateMatrix>,
    row: usize,
) -> Vec<f64> {
    let columns = design.map(CovariateMatrix::columns).unwrap_or(0);
    let mut logits = vec![0.0; components];
    for component in 1..components {
        logits[component] = parameters[component - 1];
        if let Some(design) = design {
            let offset = components - 1 + (component - 1) * columns;
            for column in 0..columns {
                logits[component] += design.value(row, column) * parameters[offset + column];
            }
        }
    }
    let largest = logits.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut values = logits
        .iter()
        .map(|value| (value - largest).exp())
        .collect::<Vec<_>>();
    let total = values.iter().sum::<f64>();
    for value in &mut values {
        *value /= total;
    }
    values
}

fn probability_parameter_count(plan: &FlexSurvMixPlan) -> usize {
    let logits = plan.components.len() - 1;
    let columns = plan
        .probability_design
        .as_ref()
        .map(CovariateMatrix::columns)
        .unwrap_or(0);
    logits * (1 + columns)
}

fn log_sum_exp(values: &[f64]) -> f64 {
    let largest = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    largest
        + values
            .iter()
            .map(|value| (value - largest).exp())
            .sum::<f64>()
            .ln()
}

fn negative_log_likelihood(
    plan: &FlexSurvMixPlan,
    models: &[RegressionModel],
    parameters: &[f64],
) -> Result<f64, MixtureError> {
    let probability_parameters = probability_parameter_count(plan);
    let mut component_offsets = Vec::with_capacity(models.len());
    let mut offset = probability_parameters;
    for model in models {
        component_offsets.push(offset);
        offset += model.parameter_count();
    }
    let mut total = 0.0;
    for (row, observation) in plan.observations.iter().enumerate() {
        let mixing = probabilities_at(
            parameters,
            plan.components.len(),
            plan.probability_design.as_ref(),
            row,
        );
        let allowed = match &observation.event {
            EventKnowledge::Known(event) => vec![*event],
            EventKnowledge::Unknown => (0..models.len()).collect(),
            EventKnowledge::Possible(events) => events.clone(),
        };
        let values = allowed
            .iter()
            .map(|event| {
                let start = component_offsets[*event];
                let end = start + models[*event].parameter_count();
                let distribution = models[*event].distribution_at(row, &parameters[start..end])?;
                Ok(mixing[*event].ln()
                    + LikelihoodRow::new(observation.survival, distribution, 1.0)?
                        .contribution()?)
            })
            .collect::<Result<Vec<_>, MixtureError>>()?;
        total += if matches!(observation.event, EventKnowledge::Known(_)) {
            values[0]
        } else {
            log_sum_exp(&values)
        };
    }
    if total.is_finite() {
        Ok(-total)
    } else {
        Err(MixtureError::NonFiniteLikelihood)
    }
}

fn em_fit(
    plan: &FlexSurvMixPlan,
    models: &[RegressionModel],
    initial: &[f64],
    control: EmControl,
) -> Result<super::r_optim::ROptimResult, MixtureError> {
    let component_count = plan.components.len();
    let rows = plan.observations.len();
    let probability_parameter_count = probability_parameter_count(plan);
    let initial_probability_parameters = initial[..probability_parameter_count].to_vec();
    let mut probability_parameters = initial_probability_parameters.clone();
    let mut offset = probability_parameter_count;
    let mut component_parameters = models
        .iter()
        .map(|model| {
            let end = offset + model.parameter_count();
            let values = initial[offset..end].to_vec();
            offset = end;
            values
        })
        .collect::<Vec<_>>();
    let has_unobserved_membership = plan
        .observations
        .iter()
        .any(|row| !matches!(row.event, EventKnowledge::Known(_)));
    let mut previous_log_likelihood = 0.0;

    for iteration in 0..control.maximum_iterations {
        let mut membership = vec![0.0; rows * component_count];
        for (row, observation) in plan.observations.iter().enumerate() {
            let mixing = probabilities_at(
                &probability_parameters,
                component_count,
                plan.probability_design.as_ref(),
                row,
            );
            let allowed = match &observation.event {
                EventKnowledge::Known(event) => vec![*event],
                EventKnowledge::Unknown => (0..component_count).collect(),
                EventKnowledge::Possible(events) => events.clone(),
            };
            let mut total = 0.0;
            for event in allowed {
                // flexsurvmix 2.3.2 passes only `theta[[k]]` to flexsurvreg in
                // the E step. Component covariate coefficients are therefore
                // reinitialised to zero while membership weights are formed.
                let baseline_count = models[event].family().parameters().len();
                let mut e_step_parameters = component_parameters[event].clone();
                e_step_parameters[baseline_count..].fill(0.0);
                let distribution = models[event].distribution_at(row, &e_step_parameters)?;
                let likelihood = LikelihoodRow::new(observation.survival, distribution, 1.0)?
                    .contribution()?
                    .exp();
                let value = mixing[event] * likelihood;
                membership[row * component_count + event] = value;
                total += value;
            }
            for event in 0..component_count {
                membership[row * component_count + event] /= total;
            }
        }

        if probability_parameter_count > 0 {
            let probability_control = ROptimControl::bfgs_defaults(
                std::num::NonZeroUsize::new(probability_parameter_count)
                    .expect("a positive parameter count was checked"),
            );
            let probability_result = r_optim_bfgs_numeric(
                &initial_probability_parameters,
                &probability_control,
                |candidate| {
                    -(0..rows)
                        .map(|row| {
                            let mixing = probabilities_at(
                                candidate,
                                component_count,
                                plan.probability_design.as_ref(),
                                row,
                            );
                            mixing
                                .iter()
                                .enumerate()
                                .map(|(event, probability)| {
                                    membership[row * component_count + event] * probability.ln()
                                })
                                .sum::<f64>()
                        })
                        .sum::<f64>()
                },
            )?;
            probability_parameters = probability_result.parameters;
        }

        let mut new_parameters = Vec::with_capacity(component_count);
        let mut conditional_log_likelihood = 0.0;
        for component in 0..component_count {
            let selected_rows = plan
                .observations
                .iter()
                .enumerate()
                .filter_map(|(row, _)| {
                    let weight = membership[row * component_count + component];
                    (weight > 0.0).then_some(row)
                })
                .collect::<Vec<_>>();
            let data = SurvivalDataset::new(
                selected_rows
                    .iter()
                    .map(|row| {
                        SurvivalRecord::weighted(
                            plan.observations[*row].survival,
                            membership[*row * component_count + component],
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )?;
            let model = models[component].select_rows(&selected_rows)?;
            let fit_control = ROptimControl::new(
                f64::NEG_INFINITY,
                f64::EPSILON.sqrt(),
                100,
                vec![1e-6; model.parameter_count()],
                vec![1.0; model.parameter_count()],
                10000.0,
            )?;
            let natural = models[component].natural_baseline(&component_parameters[component])?;
            let baseline_count = models[component].family().parameters().len();
            let coefficients = component_parameters[component][baseline_count..].to_vec();
            let fit = fit_flexsurv(FlexSurvFitPlan::optimize(
                model,
                data,
                &natural,
                &coefficients,
                &[],
                fit_control,
            )?)?;
            conditional_log_likelihood += fit.log_likelihood;
            new_parameters.push(fit.transformed_parameters);
        }
        component_parameters = new_parameters;

        let converged = !has_unobserved_membership
            || (iteration > 0
                && (conditional_log_likelihood / previous_log_likelihood - 1.0).abs()
                    <= control.relative_tolerance);
        previous_log_likelihood = conditional_log_likelihood;
        if converged {
            let mut parameters = probability_parameters;
            parameters.extend(component_parameters.into_iter().flatten());
            return Ok(super::r_optim::ROptimResult {
                value: negative_log_likelihood(plan, models, &parameters)?,
                parameters,
                function_count: 0,
                gradient_count: 0,
                iterations: iteration + 1,
                convergence: 0,
            });
        }
    }
    Err(MixtureError::EmIterationLimit {
        maximum_iterations: control.maximum_iterations,
    })
}

fn complete_event_hessian(
    plan: &FlexSurvMixPlan,
    models: &[RegressionModel],
    parameters: &[f64],
) -> Result<Vec<f64>, MixtureError> {
    let probability_count = probability_parameter_count(plan);
    let order = parameters.len();
    let mut hessian = vec![0.0; order * order];

    if probability_count > 0 {
        let control = ROptimControl::bfgs_defaults(
            std::num::NonZeroUsize::new(probability_count)
                .expect("a positive parameter count was checked"),
        );
        let probability_hessian = super::r_optim::r_optim_hessian_numeric(
            &parameters[..probability_count],
            &control,
            |candidate| {
                plan.observations
                    .iter()
                    .enumerate()
                    .map(|(row, observation)| {
                        let EventKnowledge::Known(event) = observation.event else {
                            unreachable!("complete-event Hessian requires known memberships")
                        };
                        -probabilities_at(
                            candidate,
                            plan.components.len(),
                            plan.probability_design.as_ref(),
                            row,
                        )[event]
                            .ln()
                    })
                    .sum()
            },
        )?;
        for row in 0..probability_count {
            for column in 0..probability_count {
                hessian[row * order + column] =
                    probability_hessian[row * probability_count + column];
            }
        }
    }

    let mut offset = probability_count;
    for (component, full_model) in models.iter().enumerate() {
        let selected = plan
            .observations
            .iter()
            .enumerate()
            .filter_map(|(row, observation)| {
                matches!(observation.event, EventKnowledge::Known(event) if event == component)
                    .then_some(row)
            })
            .collect::<Vec<_>>();
        let model = full_model.select_rows(&selected)?;
        let data = SurvivalDataset::new(
            selected
                .iter()
                .map(|row| SurvivalRecord::new(plan.observations[*row].survival))
                .collect(),
        )?;
        let count = model.parameter_count();
        let control = ROptimControl::new(
            f64::NEG_INFINITY,
            f64::EPSILON.sqrt(),
            100,
            vec![1e-6; count],
            vec![1.0; count],
            10000.0,
        )?;
        let block =
            flexsurv_hessian_at(&model, &data, &parameters[offset..offset + count], &control)?;
        for row in 0..count {
            for column in 0..count {
                hessian[(offset + row) * order + offset + column] = block[row * count + column];
            }
        }
        offset += count;
    }
    Ok(hessian)
}

pub fn fit_mixture(plan: FlexSurvMixPlan) -> Result<FlexSurvMixFit, MixtureError> {
    let rows = plan.observations.len();
    let models = plan
        .components
        .iter()
        .map(|component| component.model(rows))
        .collect::<Result<Vec<_>, _>>()?;
    let mut initial = plan.initial_probabilities[1..]
        .iter()
        .map(|probability| (probability / plan.initial_probabilities[0]).ln())
        .collect::<Vec<_>>();
    initial.extend_from_slice(&plan.initial_probability_coefficients);
    for (model, component) in models.iter().zip(&plan.components) {
        initial.extend(model.transformed_initial_values(
            &component.natural_initial,
            &component.coefficient_initial(),
        )?);
    }

    let objective = |parameters: &[f64]| -> f64 {
        negative_log_likelihood(&plan, &models, parameters).unwrap_or(f64::INFINITY)
    };

    let result = match plan.kind {
        MixtureFitKind::Fixed => super::r_optim::ROptimResult {
            parameters: initial.clone(),
            value: objective(&initial),
            function_count: 0,
            gradient_count: 0,
            iterations: 0,
            convergence: 0,
        },
        MixtureFitKind::Direct => {
            let control = ROptimControl::new(
                f64::NEG_INFINITY,
                f64::EPSILON.sqrt(),
                100,
                vec![1e-6; initial.len()],
                vec![1.0; initial.len()],
                10000.0,
            )?;
            r_optim_bfgs_numeric(&initial, &control, objective)?
        }
        MixtureFitKind::Em(control) => em_fit(&plan, &models, &initial, control)?,
    };
    if !result.value.is_finite() {
        return Err(MixtureError::NonFiniteLikelihood);
    }
    let uncertainty = if !matches!(plan.kind, MixtureFitKind::Fixed) {
        let hessian = if matches!(plan.kind, MixtureFitKind::Em(_))
            && plan
                .observations
                .iter()
                .all(|row| matches!(row.event, EventKnowledge::Known(_)))
        {
            complete_event_hessian(&plan, &models, &result.parameters)?
        } else {
            richardson_hessian(&result.parameters, 6, objective)?
        };
        let covariance = hessian_to_covariance(&hessian, result.parameters.len())?;
        MixtureUncertainty::Hessian {
            covariance: covariance.values,
            covariance_order: covariance.order,
            smallest_unrepaired_eigenvalue: covariance.smallest_unrepaired_eigenvalue,
            positive_definite_repair: covariance.repaired,
        }
    } else {
        MixtureUncertainty::AllParametersFixed
    };
    let mixing = baseline_probabilities(&result.parameters, plan.components.len());
    let probability_parameters = probability_parameter_count(&plan);
    let probability_coefficients =
        result.parameters[plan.components.len() - 1..probability_parameters].to_vec();
    let mut offset = probability_parameters;
    let mut natural = Vec::with_capacity(models.len());
    let mut component_coefficients = Vec::with_capacity(models.len());
    let mut component_distributions = Vec::with_capacity(models.len());
    for model in &models {
        let end = offset + model.parameter_count();
        let values = model.natural_baseline(&result.parameters[offset..end])?;
        let baseline_count = model.family().parameters().len();
        component_coefficients.push(result.parameters[offset + baseline_count..end].to_vec());
        let profile = model.prediction_covariates(vec![0.0; model.coefficient_count()])?;
        component_distributions
            .push(model.distribution_for_prediction(&profile, &result.parameters[offset..end])?);
        natural.push(values);
        offset = end;
    }
    Ok(FlexSurvMixFit {
        event_names: plan
            .components
            .iter()
            .map(|value| value.name.clone())
            .collect(),
        probabilities: mixing,
        probability_coefficients,
        natural_component_parameters: natural,
        component_coefficients,
        component_distributions,
        component_models: models,
        probability_covariate_count: plan
            .probability_design
            .as_ref()
            .map(CovariateMatrix::columns)
            .unwrap_or(0),
        transformed_parameters: result.parameters,
        log_likelihood: -result.value,
        function_count: result.function_count,
        gradient_count: result.gradient_count,
        iterations: result.iterations,
        convergence: result.convergence,
        uncertainty,
    })
}
