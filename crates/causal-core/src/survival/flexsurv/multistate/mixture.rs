//! Path summaries from `fmixmsm` mixture multi-state models.

use super::super::distribution::DistributionError;
use super::super::mixture::{
    FlexSurvMixFit, MixtureError, MixturePredictionProfile, MixturePredictionRequest,
    MixtureSimulationConfiguration,
};
use super::super::uncertainty::type_seven_quantile;
use crate::survival::r_rng::RRng;

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePath {
    pub states: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePathSummary {
    pub path: MixturePath,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePathQuantile {
    pub path: MixturePath,
    pub probability: f64,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureFinalSummary {
    pub state: String,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureFinalQuantile {
    pub state: String,
    pub probability: f64,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePathInterval {
    pub path: MixturePath,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureFinalInterval {
    pub state: String,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixturePathQuantileInterval {
    pub path: MixturePath,
    pub probability: f64,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureFinalQuantileInterval {
    pub state: String,
    pub probability: f64,
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureMultiStateModel {
    models: Vec<(String, FlexSurvMixFit)>,
    pathways: Vec<MixturePath>,
    has_cycle: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MixtureMultiStateProfile {
    profiles: Vec<MixturePredictionProfile>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MixtureMultiStateError {
    NoModels,
    EmptyStateName { model: usize },
    DuplicateStateName { model: usize },
    FitShape { model: usize },
    Cycle,
    InvalidSimulationCount,
    InvalidQuantile { index: usize },
    Distribution(DistributionError),
    Mixture(MixtureError),
    PredictionModelCount { expected: usize, actual: usize },
}

impl From<DistributionError> for MixtureMultiStateError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl From<MixtureError> for MixtureMultiStateError {
    fn from(value: MixtureError) -> Self {
        Self::Mixture(value)
    }
}

impl MixtureMultiStateModel {
    pub fn new(models: Vec<(String, FlexSurvMixFit)>) -> Result<Self, MixtureMultiStateError> {
        if models.is_empty() {
            return Err(MixtureMultiStateError::NoModels);
        }
        for (index, (name, fit)) in models.iter().enumerate() {
            if name.trim().is_empty() {
                return Err(MixtureMultiStateError::EmptyStateName { model: index });
            }
            if models[..index].iter().any(|(other, _)| other == name) {
                return Err(MixtureMultiStateError::DuplicateStateName { model: index });
            }
            if fit.event_names.len() != fit.probabilities.len()
                || fit.event_names.len() != fit.component_distributions.len()
            {
                return Err(MixtureMultiStateError::FitShape { model: index });
            }
        }

        let mut model = Self {
            models,
            pathways: Vec::new(),
            has_cycle: false,
        };
        let start = model.models[0].0.clone();
        model.visit(&start, Vec::new());
        Ok(model)
    }

    fn fit(&self, state: &str) -> Option<&FlexSurvMixFit> {
        self.models
            .iter()
            .find(|(name, _)| name == state)
            .map(|(_, fit)| fit)
    }

    fn visit(&mut self, state: &str, prefix: Vec<String>) {
        if self.has_cycle {
            return;
        }
        let mut current = prefix;
        current.push(state.to_owned());
        let events = self
            .fit(state)
            .map(|fit| fit.event_names.clone())
            .unwrap_or_default();
        for destination in events {
            let mut path = current.clone();
            path.push(destination.clone());
            if path[..path.len() - 1].contains(&destination) {
                self.has_cycle = true;
                self.pathways.clear();
                return;
            }
            if self.fit(&destination).is_some() {
                self.visit(&destination, current.clone());
            } else {
                self.pathways.push(MixturePath { states: path });
            }
        }
    }

    pub fn pathways(&self) -> &[MixturePath] {
        &self.pathways
    }

    pub fn prediction_profile(
        &self,
        probability_covariates: Vec<Vec<f64>>,
        component_covariates: Vec<Vec<Vec<f64>>>,
    ) -> Result<MixtureMultiStateProfile, MixtureMultiStateError> {
        if probability_covariates.len() != self.models.len() {
            return Err(MixtureMultiStateError::PredictionModelCount {
                expected: self.models.len(),
                actual: probability_covariates.len(),
            });
        }
        if component_covariates.len() != self.models.len() {
            return Err(MixtureMultiStateError::PredictionModelCount {
                expected: self.models.len(),
                actual: component_covariates.len(),
            });
        }
        let profiles = self
            .models
            .iter()
            .enumerate()
            .map(|(index, (_, fit))| {
                fit.prediction_profile(
                    probability_covariates[index].clone(),
                    component_covariates[index].clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MixtureMultiStateProfile { profiles })
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }

    fn ensure_acyclic(&self) -> Result<(), MixtureMultiStateError> {
        if self.has_cycle {
            Err(MixtureMultiStateError::Cycle)
        } else {
            Ok(())
        }
    }

    fn edge(&self, from: &str, to: &str) -> (&FlexSurvMixFit, usize) {
        let fit = self.fit(from).expect("path source has a model");
        let event = fit
            .event_names
            .iter()
            .position(|name| name == to)
            .expect("path destination is an event");
        (fit, event)
    }

    pub fn pathway_probabilities(&self) -> Result<Vec<MixturePathSummary>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        Ok(self
            .pathways
            .iter()
            .cloned()
            .map(|path| {
                let value = path
                    .states
                    .windows(2)
                    .map(|edge| {
                        let (fit, event) = self.edge(&edge[0], &edge[1]);
                        fit.probabilities[event]
                    })
                    .product();
                MixturePathSummary { path, value }
            })
            .collect())
    }

    pub fn pathway_probabilities_for(
        &self,
        profile: &MixtureMultiStateProfile,
    ) -> Result<Vec<MixturePathSummary>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        let mut results = Vec::with_capacity(self.pathways.len());
        for path in &self.pathways {
            let mut value = 1.0;
            for edge in path.states.windows(2) {
                let model = self
                    .models
                    .iter()
                    .position(|(name, _)| name == &edge[0])
                    .expect("a pathway source has a fitted model");
                let predictions = self.models[model]
                    .1
                    .predict_events(&profile.profiles[model], MixturePredictionRequest::Mean)?;
                value *= predictions
                    .iter()
                    .find(|prediction| prediction.event == edge[1])
                    .expect("a pathway destination is a mixture event")
                    .event_probability;
            }
            results.push(MixturePathSummary {
                path: path.clone(),
                value,
            });
        }
        Ok(results)
    }

    pub fn final_state_probabilities(
        &self,
    ) -> Result<Vec<MixtureFinalSummary>, MixtureMultiStateError> {
        let mut result = Vec::<MixtureFinalSummary>::new();
        for path in self.pathway_probabilities()? {
            let state = path.path.states.last().unwrap();
            match result.iter_mut().find(|value| &value.state == state) {
                Some(value) => value.value += path.value,
                None => result.push(MixtureFinalSummary {
                    state: state.clone(),
                    value: path.value,
                }),
            }
        }
        Ok(result)
    }

    fn resampled(&self, rng: &mut RRng) -> Result<Self, MixtureMultiStateError> {
        let models = self
            .models
            .iter()
            .map(|(name, fit)| Ok((name.clone(), fit.resampled_zero_profile(rng)?)))
            .collect::<Result<Vec<_>, MixtureMultiStateError>>()?;
        Self::new(models)
    }

    pub fn pathway_probability_intervals(
        &self,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixturePathInterval>, MixtureMultiStateError> {
        let point = self.pathway_probabilities()?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        let mut rng = RRng::new(configuration.seed());
        for _ in 1..configuration.replicates() {
            for (index, value) in self
                .resampled(&mut rng)?
                .pathway_probabilities()?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(index, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixturePathInterval {
                    path: point[index].path.clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    pub fn final_state_probability_intervals(
        &self,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixtureFinalInterval>, MixtureMultiStateError> {
        let point = self.final_state_probabilities()?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        let mut rng = RRng::new(configuration.seed());
        for _ in 1..configuration.replicates() {
            for (index, value) in self
                .resampled(&mut rng)?
                .final_state_probabilities()?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(index, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixtureFinalInterval {
                    state: point[index].state.clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    pub fn mean_time_to_final(&self) -> Result<Vec<MixturePathSummary>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        self.pathways
            .iter()
            .cloned()
            .map(|path| {
                let mut value = 0.0;
                for edge in path.states.windows(2) {
                    let (fit, event) = self.edge(&edge[0], &edge[1]);
                    value += fit.component_distributions[event].mean()?;
                }
                Ok(MixturePathSummary { path, value })
            })
            .collect()
    }

    pub fn mean_time_to_final_for(
        &self,
        profile: &MixtureMultiStateProfile,
    ) -> Result<Vec<MixturePathSummary>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        let mut results = Vec::with_capacity(self.pathways.len());
        for path in &self.pathways {
            let mut value = 0.0;
            for edge in path.states.windows(2) {
                let model = self
                    .models
                    .iter()
                    .position(|(name, _)| name == &edge[0])
                    .expect("a pathway source has a fitted model");
                let predictions = self.models[model]
                    .1
                    .predict_events(&profile.profiles[model], MixturePredictionRequest::Mean)?;
                value += predictions
                    .iter()
                    .find(|prediction| prediction.event == edge[1])
                    .expect("a pathway destination is a mixture event")
                    .value;
            }
            results.push(MixturePathSummary {
                path: path.clone(),
                value,
            });
        }
        Ok(results)
    }

    pub fn mean_time_to_final_by_state(
        &self,
    ) -> Result<Vec<MixtureFinalSummary>, MixtureMultiStateError> {
        let means = self.mean_time_to_final()?;
        let path_probabilities = self.pathway_probabilities()?;
        let mut totals = Vec::<(String, f64, f64)>::new();
        for (mean, probability) in means.into_iter().zip(path_probabilities) {
            let state = mean.path.states.last().unwrap().clone();
            match totals.iter_mut().find(|value| value.0 == state) {
                Some(value) => {
                    value.1 += mean.value * probability.value;
                    value.2 += probability.value;
                }
                None => totals.push((state, mean.value * probability.value, probability.value)),
            }
        }
        Ok(totals
            .into_iter()
            .map(|(state, weighted, probability)| MixtureFinalSummary {
                state,
                value: weighted / probability,
            })
            .collect())
    }

    pub fn mean_time_to_final_intervals(
        &self,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixturePathInterval>, MixtureMultiStateError> {
        let point = self.mean_time_to_final()?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        let mut rng = RRng::new(configuration.seed());
        for _ in 1..configuration.replicates() {
            for (index, value) in self
                .resampled(&mut rng)?
                .mean_time_to_final()?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(index, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixturePathInterval {
                    path: point[index].path.clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    pub fn mean_time_to_final_by_state_intervals(
        &self,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixtureFinalInterval>, MixtureMultiStateError> {
        let point = self.mean_time_to_final_by_state()?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        let mut rng = RRng::new(configuration.seed());
        for _ in 1..configuration.replicates() {
            for (index, value) in self
                .resampled(&mut rng)?
                .mean_time_to_final_by_state()?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(values
            .into_iter()
            .enumerate()
            .map(|(index, mut values)| {
                let estimate = values[0];
                values.sort_by(f64::total_cmp);
                MixtureFinalInterval {
                    state: point[index].state.clone(),
                    estimate,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    fn sampled_path_totals(
        &self,
        simulations: usize,
        seed: u32,
    ) -> Result<Vec<Vec<f64>>, MixtureMultiStateError> {
        self.sampled_path_totals_with_rng(simulations, &mut RRng::new(seed))
    }

    fn sampled_path_totals_with_rng(
        &self,
        simulations: usize,
        rng: &mut RRng,
    ) -> Result<Vec<Vec<f64>>, MixtureMultiStateError> {
        let sampled = self
            .models
            .iter()
            .map(|(_, fit)| {
                fit.component_distributions
                    .iter()
                    .map(|distribution| {
                        (0..simulations)
                            .map(|_| distribution.sample_r(rng))
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self
            .pathways
            .iter()
            .map(|path| {
                let mut totals = vec![0.0; simulations];
                for edge in path.states.windows(2) {
                    let model = self
                        .models
                        .iter()
                        .position(|(name, _)| name == &edge[0])
                        .unwrap();
                    let event = self.models[model]
                        .1
                        .event_names
                        .iter()
                        .position(|name| name == &edge[1])
                        .unwrap();
                    for (total, value) in totals.iter_mut().zip(&sampled[model][event]) {
                        *total += value;
                    }
                }
                totals
            })
            .collect())
    }

    fn sampled_path_totals_for(
        &self,
        profile: &MixtureMultiStateProfile,
        simulations: usize,
        rng: &mut RRng,
    ) -> Result<Vec<Vec<f64>>, MixtureMultiStateError> {
        let sampled = self
            .models
            .iter()
            .enumerate()
            .map(|(model, (_, fit))| {
                fit.sample_event_times_with_rng(&profile.profiles[model], simulations, rng)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(self
            .pathways
            .iter()
            .map(|path| {
                let mut totals = vec![0.0; simulations];
                for edge in path.states.windows(2) {
                    let model = self
                        .models
                        .iter()
                        .position(|(name, _)| name == &edge[0])
                        .expect("a pathway source has a fitted model");
                    let event = self.models[model]
                        .1
                        .event_names
                        .iter()
                        .position(|name| name == &edge[1])
                        .expect("a pathway destination is a mixture event");
                    for (total, value) in totals.iter_mut().zip(&sampled[model][event]) {
                        *total += value;
                    }
                }
                totals
            })
            .collect())
    }

    pub fn time_to_final_quantiles_for(
        &self,
        profile: &MixtureMultiStateProfile,
        simulations: usize,
        probabilities: Vec<f64>,
        seed: u32,
    ) -> Result<Vec<MixturePathQuantile>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        if simulations == 0 {
            return Err(MixtureMultiStateError::InvalidSimulationCount);
        }
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || *probability < 0.0 || *probability > 1.0 {
                return Err(MixtureMultiStateError::InvalidQuantile { index });
            }
        }
        let sampled = self.sampled_path_totals_for(profile, simulations, &mut RRng::new(seed))?;
        let mut result = Vec::new();
        for (path, mut totals) in self.pathways.iter().zip(sampled) {
            totals.sort_by(f64::total_cmp);
            for probability in &probabilities {
                result.push(MixturePathQuantile {
                    path: path.clone(),
                    probability: *probability,
                    value: type_seven_quantile(&totals, *probability),
                });
            }
        }
        Ok(result)
    }

    pub fn time_to_final_quantiles(
        &self,
        simulations: usize,
        probabilities: Vec<f64>,
        seed: u32,
    ) -> Result<Vec<MixturePathQuantile>, MixtureMultiStateError> {
        self.ensure_acyclic()?;
        if simulations == 0 {
            return Err(MixtureMultiStateError::InvalidSimulationCount);
        }
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || *probability < 0.0 || *probability > 1.0 {
                return Err(MixtureMultiStateError::InvalidQuantile { index });
            }
        }
        let sampled = self.sampled_path_totals(simulations, seed)?;

        let mut result = Vec::new();
        for (path, mut totals) in self.pathways.iter().zip(sampled) {
            totals.sort_by(f64::total_cmp);
            for probability in &probabilities {
                result.push(MixturePathQuantile {
                    path: path.clone(),
                    probability: *probability,
                    value: type_seven_quantile(&totals, *probability),
                });
            }
        }
        Ok(result)
    }

    pub fn time_to_final_quantiles_by_state(
        &self,
        simulations: usize,
        probabilities: Vec<f64>,
        seed: u32,
    ) -> Result<Vec<MixtureFinalQuantile>, MixtureMultiStateError> {
        if simulations == 0 {
            return Err(MixtureMultiStateError::InvalidSimulationCount);
        }
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || *probability < 0.0 || *probability > 1.0 {
                return Err(MixtureMultiStateError::InvalidQuantile { index });
            }
        }
        let path_probabilities = self.pathway_probabilities()?;
        let path_totals = self.sampled_path_totals(simulations, seed)?;
        let mut grouped = Vec::<(String, Vec<f64>)>::new();
        for ((path, path_probability), totals) in self
            .pathways
            .iter()
            .zip(path_probabilities)
            .zip(path_totals)
        {
            let count = (simulations as f64 * path_probability.value).round_ties_even() as usize;
            let state = path.states.last().unwrap().clone();
            match grouped.iter_mut().find(|value| value.0 == state) {
                Some(value) => value.1.extend_from_slice(&totals[..count]),
                None => grouped.push((state, totals[..count].to_vec())),
            }
        }
        let mut result = Vec::new();
        for (state, mut totals) in grouped {
            totals.sort_by(f64::total_cmp);
            for probability in &probabilities {
                result.push(MixtureFinalQuantile {
                    state: state.clone(),
                    probability: *probability,
                    value: type_seven_quantile(&totals, *probability),
                });
            }
        }
        Ok(result)
    }

    pub fn time_to_final_quantile_intervals(
        &self,
        simulations: usize,
        probabilities: Vec<f64>,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixturePathQuantileInterval>, MixtureMultiStateError> {
        if simulations == 0 {
            return Err(MixtureMultiStateError::InvalidSimulationCount);
        }
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || !(0.0..=1.0).contains(probability) {
                return Err(MixtureMultiStateError::InvalidQuantile { index });
            }
        }
        let mut rng = RRng::new(configuration.seed());
        let mut point = self.path_quantiles_with_rng(simulations, &probabilities, &mut rng)?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        for _ in 1..configuration.replicates() {
            let sampled = self.resampled(&mut rng)?;
            for (index, value) in sampled
                .path_quantiles_with_rng(simulations, &probabilities, &mut rng)?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(point
            .drain(..)
            .zip(values)
            .map(|(point, mut values)| {
                values.sort_by(f64::total_cmp);
                MixturePathQuantileInterval {
                    path: point.path,
                    probability: point.probability,
                    estimate: point.value,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }

    fn path_quantiles_with_rng(
        &self,
        simulations: usize,
        probabilities: &[f64],
        rng: &mut RRng,
    ) -> Result<Vec<MixturePathQuantile>, MixtureMultiStateError> {
        let sampled = self.sampled_path_totals_with_rng(simulations, rng)?;
        let mut result = Vec::new();
        for (path, mut totals) in self.pathways.iter().zip(sampled) {
            totals.sort_by(f64::total_cmp);
            for probability in probabilities {
                result.push(MixturePathQuantile {
                    path: path.clone(),
                    probability: *probability,
                    value: type_seven_quantile(&totals, *probability),
                });
            }
        }
        Ok(result)
    }

    fn final_quantiles_with_rng(
        &self,
        simulations: usize,
        probabilities: &[f64],
        rng: &mut RRng,
    ) -> Result<Vec<MixtureFinalQuantile>, MixtureMultiStateError> {
        let path_probabilities = self.pathway_probabilities()?;
        let path_totals = self.sampled_path_totals_with_rng(simulations, rng)?;
        let mut grouped = Vec::<(String, Vec<f64>)>::new();
        for ((path, path_probability), totals) in self
            .pathways
            .iter()
            .zip(path_probabilities)
            .zip(path_totals)
        {
            let count = (simulations as f64 * path_probability.value).round_ties_even() as usize;
            let state = path
                .states
                .last()
                .expect("a pathway has a final state")
                .clone();
            match grouped.iter_mut().find(|value| value.0 == state) {
                Some(value) => value.1.extend_from_slice(&totals[..count]),
                None => grouped.push((state, totals[..count].to_vec())),
            }
        }
        let mut result = Vec::new();
        for (state, mut totals) in grouped {
            totals.sort_by(f64::total_cmp);
            for probability in probabilities {
                result.push(MixtureFinalQuantile {
                    state: state.clone(),
                    probability: *probability,
                    value: type_seven_quantile(&totals, *probability),
                });
            }
        }
        Ok(result)
    }

    pub fn time_to_final_quantile_intervals_by_state(
        &self,
        simulations: usize,
        probabilities: Vec<f64>,
        configuration: MixtureSimulationConfiguration,
    ) -> Result<Vec<MixtureFinalQuantileInterval>, MixtureMultiStateError> {
        if simulations == 0 {
            return Err(MixtureMultiStateError::InvalidSimulationCount);
        }
        for (index, probability) in probabilities.iter().enumerate() {
            if !probability.is_finite() || !(0.0..=1.0).contains(probability) {
                return Err(MixtureMultiStateError::InvalidQuantile { index });
            }
        }
        let mut rng = RRng::new(configuration.seed());
        let mut point = self.final_quantiles_with_rng(simulations, &probabilities, &mut rng)?;
        let mut values = point
            .iter()
            .map(|value| vec![value.value])
            .collect::<Vec<_>>();
        for _ in 1..configuration.replicates() {
            let sampled = self.resampled(&mut rng)?;
            for (index, value) in sampled
                .final_quantiles_with_rng(simulations, &probabilities, &mut rng)?
                .into_iter()
                .enumerate()
            {
                values[index].push(value.value);
            }
        }
        Ok(point
            .drain(..)
            .zip(values)
            .map(|(point, mut values)| {
                values.sort_by(f64::total_cmp);
                MixtureFinalQuantileInterval {
                    state: point.state,
                    probability: point.probability,
                    estimate: point.value,
                    lower: type_seven_quantile(&values, 0.025),
                    upper: type_seven_quantile(&values, 0.975),
                }
            })
            .collect())
    }
}
