//! Generic flexsurv likelihood contributions for censoring and truncation.

use super::distribution::{DistributionError, FlexSurvDistribution};
use super::observation::{NonNegativeTime, ObservationError, SurvivalObservation};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LikelihoodRow {
    observation: SurvivalObservation,
    distribution: FlexSurvDistribution,
    weight: f64,
    right_truncation: Option<NonNegativeTime>,
    background: Option<BackgroundMortality>,
}

/// The `bhazard` value has different source semantics for exact events and
/// censored intervals, so those inputs are represented as different variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BackgroundMortality(BackgroundMortalityKind);

#[derive(Clone, Copy, Debug, PartialEq)]
enum BackgroundMortalityKind {
    EventHazard(f64),
    ConditionalDeathProbability(f64),
}

impl BackgroundMortality {
    pub fn event_hazard(value: f64) -> Result<Self, LikelihoodError> {
        if !value.is_finite() || value < 0.0 {
            Err(LikelihoodError::InvalidBackgroundMortality)
        } else {
            Ok(Self(BackgroundMortalityKind::EventHazard(value)))
        }
    }

    pub fn conditional_death_probability(value: f64) -> Result<Self, LikelihoodError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            Err(LikelihoodError::InvalidBackgroundMortality)
        } else {
            Ok(Self(BackgroundMortalityKind::ConditionalDeathProbability(
                value,
            )))
        }
    }

    pub(crate) fn event_hazard_value(self) -> Option<f64> {
        match self.0 {
            BackgroundMortalityKind::EventHazard(value) => Some(value),
            BackgroundMortalityKind::ConditionalDeathProbability(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LikelihoodError {
    Observation(ObservationError),
    Distribution(DistributionError),
    InvalidWeight,
    RightTruncationBeforeObservation,
    BackgroundMortalityKind,
    InvalidBackgroundMortality,
    BackgroundMortalityWithRightTruncation,
    ImpossibleObservation,
    NonFiniteContribution,
}

impl From<ObservationError> for LikelihoodError {
    fn from(value: ObservationError) -> Self {
        Self::Observation(value)
    }
}

impl From<DistributionError> for LikelihoodError {
    fn from(value: DistributionError) -> Self {
        Self::Distribution(value)
    }
}

impl LikelihoodRow {
    pub fn new(
        observation: SurvivalObservation,
        distribution: FlexSurvDistribution,
        weight: f64,
    ) -> Result<Self, LikelihoodError> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(LikelihoodError::InvalidWeight);
        }
        Ok(Self {
            observation,
            distribution,
            weight,
            right_truncation: None,
            background: None,
        })
    }

    pub fn with_right_truncation(mut self, time: f64) -> Result<Self, LikelihoodError> {
        let time = NonNegativeTime::new(time)?;
        if time.get() < self.observation.bounds().lower {
            return Err(LikelihoodError::RightTruncationBeforeObservation);
        }
        if self.background.is_some() {
            return Err(LikelihoodError::BackgroundMortalityWithRightTruncation);
        }
        self.right_truncation = Some(time);
        Ok(self)
    }

    pub fn with_background_mortality(
        mut self,
        background: BackgroundMortality,
    ) -> Result<Self, LikelihoodError> {
        if self.right_truncation.is_some() {
            return Err(LikelihoodError::BackgroundMortalityWithRightTruncation);
        }
        let exact = self.observation.bounds().exact;
        match background.0 {
            BackgroundMortalityKind::EventHazard(_) => {
                if !exact {
                    return Err(LikelihoodError::BackgroundMortalityKind);
                }
            }
            BackgroundMortalityKind::ConditionalDeathProbability(_) => {
                if exact {
                    return Err(LikelihoodError::BackgroundMortalityKind);
                }
            }
        }
        self.background = Some(background);
        Ok(self)
    }

    pub fn contribution(self) -> Result<f64, LikelihoodError> {
        let bounds = self.observation.bounds();
        let log_observation = if bounds.exact {
            let log_density = self.distribution.log_density(bounds.lower)?;
            match self.background {
                Some(BackgroundMortality(BackgroundMortalityKind::EventHazard(background))) => {
                    let hazard = self.distribution.evaluate(bounds.lower)?.hazard;
                    log_density + (1.0 + background / hazard).ln()
                }
                Some(BackgroundMortality(
                    BackgroundMortalityKind::ConditionalDeathProbability(_),
                )) => return Err(LikelihoodError::BackgroundMortalityKind),
                None => log_density,
            }
        } else {
            let upper_probability = if bounds.upper.is_infinite() {
                1.0
            } else {
                self.distribution.cdf(bounds.upper)?
            };
            let lower_probability = self.distribution.cdf(bounds.lower)?;
            let probability = match self.background {
                Some(BackgroundMortality(
                    BackgroundMortalityKind::ConditionalDeathProbability(background),
                )) => {
                    let conditional_survival = if bounds.upper.is_infinite() {
                        0.0
                    } else {
                        1.0 - background
                    };
                    (upper_probability - 1.0) * conditional_survival + 1.0 - lower_probability
                }
                Some(BackgroundMortality(BackgroundMortalityKind::EventHazard(_))) => {
                    return Err(LikelihoodError::BackgroundMortalityKind)
                }
                None => upper_probability - lower_probability,
            };
            if probability <= 0.0 {
                return Err(LikelihoodError::ImpossibleObservation);
            }
            probability.ln()
        };

        let upper_probability = match self.right_truncation {
            Some(time) => self.distribution.cdf(time.get())?,
            None => 1.0,
        };
        let entry_probability = self.distribution.cdf(bounds.entry)?;
        let observed_probability = upper_probability - entry_probability;
        if observed_probability <= 0.0 {
            return Err(LikelihoodError::ImpossibleObservation);
        }

        let contribution = (log_observation - observed_probability.ln()) * self.weight;
        if contribution.is_finite() {
            Ok(contribution)
        } else {
            Err(LikelihoodError::NonFiniteContribution)
        }
    }
}

pub fn log_likelihood(rows: &[LikelihoodRow]) -> Result<f64, LikelihoodError> {
    rows.iter()
        .try_fold(0.0, |total, row| Ok(total + row.contribution()?))
}
