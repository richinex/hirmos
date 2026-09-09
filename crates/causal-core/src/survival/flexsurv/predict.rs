//! Point predictions from a fitted `flexsurvreg` model.

use super::distribution::{DistributionError, DistributionValue};
use super::fit::FlexSurvFit;
use super::model::{ModelError, PredictionCovariates};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PredictionRequest {
    AtTime(f64),
    Quantile(f64),
    Mean,
    RestrictedMean { start: f64, end: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Prediction {
    AtTime(DistributionValue),
    Quantile(f64),
    Mean(f64),
    RestrictedMean(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PredictionError {
    InvalidTime,
    InvalidExpectedSurvival,
    InvalidExpectedHazard,
    Model(ModelError),
    Distribution(DistributionError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpectedMortalityAtTime {
    survival: f64,
    hazard: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AllCausePrediction {
    pub relative_survival: f64,
    pub excess_hazard: f64,
    pub survival: f64,
    pub hazard: f64,
    pub cumulative_hazard: f64,
}

impl ExpectedMortalityAtTime {
    pub fn new(survival: f64, hazard: f64) -> Result<Self, PredictionError> {
        if !survival.is_finite() || !(0.0..=1.0).contains(&survival) {
            return Err(PredictionError::InvalidExpectedSurvival);
        }
        if !hazard.is_finite() || hazard < 0.0 {
            return Err(PredictionError::InvalidExpectedHazard);
        }
        Ok(Self { survival, hazard })
    }
}

impl From<ModelError> for PredictionError {
    fn from(value: ModelError) -> Self {
        Self::Model(value)
    }
}

impl From<DistributionError> for PredictionError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl FlexSurvFit {
    pub fn prediction_covariates(
        &self,
        values: Vec<f64>,
    ) -> Result<PredictionCovariates, PredictionError> {
        Ok(self.model.prediction_covariates(values)?)
    }

    pub fn predict(
        &self,
        covariates: &PredictionCovariates,
        request: PredictionRequest,
    ) -> Result<Prediction, PredictionError> {
        let distribution = self
            .model
            .distribution_for_prediction(covariates, &self.transformed_parameters)?;
        let prediction = match request {
            PredictionRequest::AtTime(time) => {
                if !time.is_finite() || time < 0.0 {
                    return Err(PredictionError::InvalidTime);
                }
                Prediction::AtTime(distribution.evaluate(time)?)
            }
            PredictionRequest::Quantile(probability) => {
                Prediction::Quantile(distribution.quantile(probability)?)
            }
            PredictionRequest::Mean => Prediction::Mean(distribution.mean()?),
            PredictionRequest::RestrictedMean { start, end } => {
                Prediction::RestrictedMean(distribution.restricted_mean(start, end)?)
            }
        };
        Ok(prediction)
    }

    /// Combines flexsurv's excess-hazard prediction with externally supplied
    /// population mortality at the same profile and time.
    pub fn predict_all_cause(
        &self,
        covariates: &PredictionCovariates,
        time: f64,
        expected: ExpectedMortalityAtTime,
    ) -> Result<AllCausePrediction, PredictionError> {
        if !time.is_finite() || time < 0.0 {
            return Err(PredictionError::InvalidTime);
        }
        let distribution = self
            .model
            .distribution_for_prediction(covariates, &self.transformed_parameters)?;
        let relative_survival = distribution.survival(time)?;
        let excess_hazard = distribution.evaluate(time)?.hazard;
        let survival = relative_survival * expected.survival;
        let hazard = excess_hazard + expected.hazard;
        Ok(AllCausePrediction {
            relative_survival,
            excess_hazard,
            survival,
            hazard,
            cumulative_hazard: -survival.ln(),
        })
    }
}
