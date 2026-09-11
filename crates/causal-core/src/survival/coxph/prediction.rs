//! Predict cumulative hazards and survival curves from a fitted Cox model.

use super::baseline::BaselineEstimate;
use super::fit::CoxFit;
use super::math::dot;

#[derive(Clone, Debug, PartialEq)]
pub enum CoxProfile {
    Shared(Vec<f64>),
    Stratified {
        stratum: usize,
        covariates: Vec<f64>,
    },
}

impl CoxProfile {
    fn covariates(&self) -> &[f64] {
        match self {
            Self::Shared(values) => values,
            Self::Stratified { covariates, .. } => covariates,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurvivalPrediction {
    pub time: f64,
    pub cumulative_hazard: f64,
    pub survival: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProfilePrediction {
    pub log_partial_hazard: f64,
    pub partial_hazard: f64,
    pub values: Vec<SurvivalPrediction>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredictionError {
    EmptyProfiles,
    EmptyTimes,
    InvalidTime,
    InvalidConditionalTime,
    ProfileDimension,
    BaselineMismatch,
    UnknownStratum,
    InvalidPercentile,
}

fn validate_prediction_inputs(
    profiles: &[CoxProfile],
    conditional_after: Option<&[f64]>,
) -> Result<(), PredictionError> {
    if profiles.is_empty() {
        return Err(PredictionError::EmptyProfiles);
    }
    if let Some(values) = conditional_after {
        if values.len() != profiles.len()
            || values.iter().any(|time| !time.is_finite() || *time < 0.0)
        {
            return Err(PredictionError::InvalidConditionalTime);
        }
    }
    Ok(())
}

fn baseline<'a>(
    fit: &'a CoxFit,
    profile: &CoxProfile,
) -> Result<&'a [BaselineEstimate], PredictionError> {
    match profile {
        CoxProfile::Shared(_) => fit
            .baseline
            .shared()
            .ok_or(PredictionError::BaselineMismatch),
        CoxProfile::Stratified { stratum, .. } => fit
            .baseline
            .stratified()
            .ok_or(PredictionError::BaselineMismatch)?
            .iter()
            .find(|curve| curve.stratum == *stratum)
            .map(|curve| curve.estimates.as_slice())
            .ok_or(PredictionError::UnknownStratum),
    }
}

fn interpolate(estimates: &[BaselineEstimate], time: f64) -> f64 {
    if time <= estimates[0].time {
        return estimates[0].cumulative_hazard;
    }
    if time >= estimates[estimates.len() - 1].time {
        return estimates[estimates.len() - 1].cumulative_hazard;
    }
    let upper = estimates
        .iter()
        .position(|estimate| estimate.time >= time)
        .expect("time lies within the baseline range");
    let lower = upper - 1;
    let fraction = (time - estimates[lower].time) / (estimates[upper].time - estimates[lower].time);
    estimates[lower].cumulative_hazard
        + fraction * (estimates[upper].cumulative_hazard - estimates[lower].cumulative_hazard)
}

pub fn predict_survival(
    fit: &CoxFit,
    profiles: &[CoxProfile],
    times: &[f64],
    conditional_after: Option<&[f64]>,
) -> Result<Vec<ProfilePrediction>, PredictionError> {
    validate_prediction_inputs(profiles, conditional_after)?;
    if times.is_empty() {
        return Err(PredictionError::EmptyTimes);
    }
    if times.iter().any(|time| !time.is_finite()) {
        return Err(PredictionError::InvalidTime);
    }
    let columns = fit.coefficients.len();
    let coefficients = fit
        .coefficients
        .iter()
        .map(|estimate| estimate.coefficient)
        .collect::<Vec<_>>();
    profiles
        .iter()
        .enumerate()
        .map(|(profile_index, profile)| {
            if profile.covariates().len() != columns {
                return Err(PredictionError::ProfileDimension);
            }
            let centered = profile
                .covariates()
                .iter()
                .enumerate()
                .map(|(column, value)| value - fit.covariate_means[column])
                .collect::<Vec<_>>();
            let log_partial_hazard = dot(&centered, &coefficients);
            let partial_hazard = log_partial_hazard.exp();
            let baseline = baseline(fit, profile)?;
            let lived = conditional_after.map_or(0.0, |values| values[profile_index]);
            let lived_hazard = conditional_after.map_or(0.0, |_| interpolate(baseline, lived));
            let values = times
                .iter()
                .map(|time| {
                    let cumulative_hazard = ((interpolate(baseline, time + lived) - lived_hazard)
                        .max(0.0))
                        * partial_hazard;
                    SurvivalPrediction {
                        time: *time,
                        cumulative_hazard,
                        survival: (-cumulative_hazard).exp(),
                    }
                })
                .collect();
            Ok(ProfilePrediction {
                log_partial_hazard,
                partial_hazard,
                values,
            })
        })
        .collect()
}

pub fn predict_percentile(
    fit: &CoxFit,
    profiles: &[CoxProfile],
    percentile: f64,
    conditional_after: Option<&[f64]>,
) -> Result<Vec<f64>, PredictionError> {
    validate_prediction_inputs(profiles, conditional_after)?;
    if !percentile.is_finite() || !(0.0..=1.0).contains(&percentile) {
        return Err(PredictionError::InvalidPercentile);
    }
    profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| {
            let times = baseline(fit, profile)?
                .iter()
                .map(|estimate| estimate.time)
                .collect::<Vec<_>>();
            let conditional = conditional_after.map(|values| &values[index..=index]);
            let prediction =
                predict_survival(fit, std::slice::from_ref(profile), &times, conditional)?;
            Ok(prediction[0]
                .values
                .iter()
                .find(|value| value.survival <= percentile)
                .map_or(f64::INFINITY, |value| value.time))
        })
        .collect()
}

pub fn predict_expectation(
    fit: &CoxFit,
    profiles: &[CoxProfile],
    conditional_after: Option<&[f64]>,
) -> Result<Vec<f64>, PredictionError> {
    validate_prediction_inputs(profiles, conditional_after)?;
    profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| {
            let times = baseline(fit, profile)?
                .iter()
                .map(|estimate| estimate.time)
                .collect::<Vec<_>>();
            let conditional = conditional_after.map(|values| &values[index..=index]);
            let prediction =
                predict_survival(fit, std::slice::from_ref(profile), &times, conditional)?;
            Ok(prediction[0]
                .values
                .windows(2)
                .map(|pair| {
                    (pair[1].time - pair[0].time) * (pair[0].survival + pair[1].survival) / 2.0
                })
                .sum())
        })
        .collect()
}
