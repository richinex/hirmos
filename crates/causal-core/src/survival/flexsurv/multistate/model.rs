use std::marker::PhantomData;

use super::graph::{TransitionGraph, TransitionId};
use crate::survival::flexsurv::distribution::{DistributionError, FlexSurvDistribution};
use crate::survival::flexsurv::fit::FlexSurvFit;
use crate::survival::flexsurv::model::{ModelError, PredictionCovariates};
use crate::survival::flexsurv::uncertainty::{PredictionSimulationConfiguration, UncertaintyError};
use crate::survival::r_rng::RRng;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockForward;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockReset;

#[derive(Clone, Debug, PartialEq)]
enum TransitionDistribution {
    Fixed(FlexSurvDistribution),
    Fitted {
        fit: FlexSurvFit,
        profile: PredictionCovariates,
        covariates: Vec<f64>,
        predictable_time_covariates: Vec<usize>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionModel(TransitionDistribution);

#[derive(Clone, Debug, PartialEq)]
pub struct MultiStateModel<C> {
    graph: TransitionGraph,
    transitions: Vec<TransitionModel>,
    clock: PhantomData<C>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiStateModelError {
    TransitionCount { expected: usize, actual: usize },
    PredictableCovariateOutsideProfile { index: usize },
    DuplicatePredictableCovariate,
    InformationCriterionNeedsFittedTransitions,
    InvalidInformationCriterionPenalty,
    Model(ModelError),
    Distribution(DistributionError),
    Uncertainty(UncertaintyError),
}

impl From<ModelError> for MultiStateModelError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for MultiStateModelError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl From<UncertaintyError> for MultiStateModelError {
    fn from(value: UncertaintyError) -> Self {
        Self::Uncertainty(value)
    }
}

impl TransitionModel {
    pub fn fixed(distribution: FlexSurvDistribution) -> Self {
        Self(TransitionDistribution::Fixed(distribution))
    }

    pub fn from_fit(fit: FlexSurvFit, covariates: Vec<f64>) -> Result<Self, MultiStateModelError> {
        Self::from_fit_with_predictable_time_covariates(fit, covariates, Vec::new())
    }

    pub fn from_fit_with_predictable_time_covariates(
        fit: FlexSurvFit,
        covariates: Vec<f64>,
        predictable_time_covariates: Vec<usize>,
    ) -> Result<Self, MultiStateModelError> {
        if let Some(index) = predictable_time_covariates
            .iter()
            .copied()
            .find(|index| *index >= covariates.len())
        {
            return Err(MultiStateModelError::PredictableCovariateOutsideProfile { index });
        }
        let mut unique = predictable_time_covariates.clone();
        unique.sort_unstable();
        unique.dedup();
        if unique.len() != predictable_time_covariates.len() {
            return Err(MultiStateModelError::DuplicatePredictableCovariate);
        }
        let profile = fit.model.prediction_covariates(covariates.clone())?;
        Ok(Self(TransitionDistribution::Fitted {
            fit,
            profile,
            covariates,
            predictable_time_covariates,
        }))
    }

    pub(crate) fn distribution(&self) -> Result<FlexSurvDistribution, MultiStateModelError> {
        match &self.0 {
            TransitionDistribution::Fixed(distribution) => Ok(*distribution),
            TransitionDistribution::Fitted { fit, profile, .. } => Ok(fit
                .model
                .distribution_for_prediction(profile, &fit.transformed_parameters)?),
        }
    }

    pub(crate) fn distribution_at_process_time(
        &self,
        process_time: f64,
    ) -> Result<FlexSurvDistribution, MultiStateModelError> {
        match &self.0 {
            TransitionDistribution::Fixed(distribution) => Ok(*distribution),
            TransitionDistribution::Fitted {
                fit,
                profile,
                covariates,
                predictable_time_covariates,
            } => {
                if predictable_time_covariates.is_empty() {
                    return Ok(fit
                        .model
                        .distribution_for_prediction(profile, &fit.transformed_parameters)?);
                }
                let mut values = covariates.clone();
                for index in predictable_time_covariates {
                    values[*index] += process_time;
                }
                let profile = fit.model.prediction_covariates(values)?;
                Ok(fit
                    .model
                    .distribution_for_prediction(&profile, &fit.transformed_parameters)?)
            }
        }
    }

    fn with_transformed_parameters(&self, transformed: Vec<f64>) -> Self {
        match &self.0 {
            TransitionDistribution::Fixed(distribution) => Self::fixed(*distribution),
            TransitionDistribution::Fitted {
                fit,
                profile,
                covariates,
                predictable_time_covariates,
            } => {
                let mut fit = fit.clone();
                fit.transformed_parameters = transformed;
                Self(TransitionDistribution::Fitted {
                    fit,
                    profile: profile.clone(),
                    covariates: covariates.clone(),
                    predictable_time_covariates: predictable_time_covariates.clone(),
                })
            }
        }
    }

    fn information_criterion(&self, penalty: f64) -> Result<f64, MultiStateModelError> {
        match &self.0 {
            TransitionDistribution::Fixed(_) => {
                Err(MultiStateModelError::InformationCriterionNeedsFittedTransitions)
            }
            TransitionDistribution::Fitted { fit, .. } => {
                Ok(-2.0 * fit.log_likelihood + penalty * fit.estimated_parameter_count as f64)
            }
        }
    }
}

impl<C> MultiStateModel<C> {
    fn build(
        graph: TransitionGraph,
        transitions: Vec<TransitionModel>,
    ) -> Result<Self, MultiStateModelError> {
        if graph.transition_count() != transitions.len() {
            return Err(MultiStateModelError::TransitionCount {
                expected: graph.transition_count(),
                actual: transitions.len(),
            });
        }
        Ok(Self {
            graph,
            transitions,
            clock: PhantomData,
        })
    }

    pub fn graph(&self) -> &TransitionGraph {
        &self.graph
    }

    /// Natural-scale transition distributions, the typed equivalent of
    /// `pars.fmsm` after profiles have been materialized.
    pub fn transition_distributions(
        &self,
    ) -> Result<Vec<FlexSurvDistribution>, MultiStateModelError> {
        self.transitions
            .iter()
            .map(TransitionModel::distribution)
            .collect()
    }

    /// `AIC.fmsm`, generalized to the source's public `k` penalty argument.
    pub fn information_criterion(&self, penalty: f64) -> Result<f64, MultiStateModelError> {
        if !penalty.is_finite() || penalty < 0.0 {
            return Err(MultiStateModelError::InvalidInformationCriterionPenalty);
        }
        self.transitions
            .iter()
            .map(|transition| transition.information_criterion(penalty))
            .sum()
    }

    pub(crate) fn transition_distribution(
        &self,
        transition: TransitionId,
    ) -> Result<FlexSurvDistribution, MultiStateModelError> {
        self.transitions[transition.index()].distribution()
    }

    pub(crate) fn transitions_distribution_at(
        &self,
        transition: TransitionId,
        process_time: f64,
    ) -> Result<FlexSurvDistribution, MultiStateModelError> {
        self.transitions[transition.index()].distribution_at_process_time(process_time)
    }

    pub(crate) fn simulated_transition_models(
        &self,
        configuration: PredictionSimulationConfiguration,
    ) -> Result<Vec<Vec<TransitionModel>>, MultiStateModelError> {
        let mut rng = RRng::new(configuration.seed());
        self.simulated_transition_models_with_rng(configuration, &mut rng)
    }

    pub(crate) fn simulated_transition_models_with_rng(
        &self,
        configuration: PredictionSimulationConfiguration,
        rng: &mut RRng,
    ) -> Result<Vec<Vec<TransitionModel>>, MultiStateModelError> {
        let mut by_transition = Vec::with_capacity(self.transitions.len());
        for transition in &self.transitions {
            match &transition.0 {
                TransitionDistribution::Fixed(_) => {
                    by_transition.push(vec![None; configuration.draws()]);
                }
                TransitionDistribution::Fitted { fit, .. } => {
                    let draws = fit
                        .simulate_transformed_parameters_with_rng(configuration, rng)?
                        .transformed
                        .into_iter()
                        .map(Some)
                        .collect();
                    by_transition.push(draws);
                }
            }
        }
        Ok((0..configuration.draws())
            .map(|draw| {
                by_transition
                    .iter()
                    .zip(&self.transitions)
                    .map(|(values, transition)| match &values[draw] {
                        Some(parameters) => {
                            transition.with_transformed_parameters(parameters.clone())
                        }
                        None => transition.clone(),
                    })
                    .collect()
            })
            .collect())
    }
}

impl MultiStateModel<ClockForward> {
    pub fn clock_forward(
        graph: TransitionGraph,
        transitions: Vec<TransitionModel>,
    ) -> Result<Self, MultiStateModelError> {
        Self::build(graph, transitions)
    }
}

impl MultiStateModel<ClockReset> {
    pub fn clock_reset(
        graph: TransitionGraph,
        transitions: Vec<TransitionModel>,
    ) -> Result<Self, MultiStateModelError> {
        Self::build(graph, transitions)
    }
}
