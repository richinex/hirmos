//! The value of a target policy estimated from a bandit log that records the logging policy's propensities.

use std::fmt;

use crate::causal_effects::numpy_percentile;
use crate::nprandom::Mt19937;
use crate::numpy_reduce::numpy_mean;

#[derive(Clone, Debug, PartialEq)]
pub enum OffPolicyEvaluationError {
    EmptyLog,
    LengthMismatch { rewards: usize, propensities: usize, targets: usize },
    NonFiniteReward { round: usize },
    InvalidPropensity { round: usize, value: f64 },
    InvalidTargetProbability { round: usize, value: f64 },
    InvalidMaxWeight(f64),
    InvalidAlpha(f64),
    NoBootstrapRounds,
}

impl fmt::Display for OffPolicyEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLog => write!(f, "the log has no rounds"),
            Self::LengthMismatch { rewards, propensities, targets } => write!(
                f,
                "rewards ({rewards}), propensities ({propensities}) and target probabilities ({targets}) differ in length"
            ),
            Self::NonFiniteReward { round } => write!(f, "round {round} has a non-finite reward"),
            Self::InvalidPropensity { round, value } => {
                write!(f, "round {round} has propensity {value}, outside (0, 1]")
            }
            Self::InvalidTargetProbability { round, value } => {
                write!(f, "round {round} has target probability {value}, outside [0, 1]")
            }
            Self::InvalidMaxWeight(value) => write!(f, "the maximum weight {value} must be positive"),
            Self::InvalidAlpha(value) => write!(f, "alpha {value} must lie in (0, 1)"),
            Self::NoBootstrapRounds => write!(f, "the bootstrap needs at least one round"),
        }
    }
}

impl std::error::Error for OffPolicyEvaluationError {}

/// One logged round: the reward, the logging policy's probability of the logged action, and the
/// target policy's probability of that same action in that round.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LoggedRound {
    pub reward: f64,
    pub propensity: f64,
    pub target: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Estimator {
    /// Importance weights capped at `max_weight`; `f64::INFINITY` leaves them uncapped.
    InverseProbability { max_weight: f64 },
    SelfNormalized,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BootstrapInterval {
    pub mean: f64,
    pub lower: f64,
    pub upper: f64,
}

fn weights(rounds: &[LoggedRound]) -> Result<Vec<f64>, OffPolicyEvaluationError> {
    if rounds.is_empty() {
        return Err(OffPolicyEvaluationError::EmptyLog);
    }
    rounds
        .iter()
        .enumerate()
        .map(|(round, row)| {
            if !row.reward.is_finite() {
                return Err(OffPolicyEvaluationError::NonFiniteReward { round });
            }
            if !(row.propensity > 0.0 && row.propensity <= 1.0) {
                return Err(OffPolicyEvaluationError::InvalidPropensity { round, value: row.propensity });
            }
            if !(0.0..=1.0).contains(&row.target) {
                return Err(OffPolicyEvaluationError::InvalidTargetProbability { round, value: row.target });
            }
            Ok(row.target / row.propensity)
        })
        .collect()
}

/// Each round's weighted reward, whose mean is the estimated policy value.
pub fn round_rewards(rounds: &[LoggedRound], estimator: Estimator) -> Result<Vec<f64>, OffPolicyEvaluationError> {
    let mut weights = weights(rounds)?;
    match estimator {
        Estimator::InverseProbability { max_weight } => {
            if !(max_weight > 0.0) {
                return Err(OffPolicyEvaluationError::InvalidMaxWeight(max_weight));
            }
            for (weight, row) in weights.iter_mut().zip(rounds) {
                *weight = weight.min(max_weight) * row.reward;
            }
        }
        Estimator::SelfNormalized => {
            let mean = numpy_mean(&weights);
            for (weight, row) in weights.iter_mut().zip(rounds) {
                *weight = row.reward * *weight / mean;
            }
        }
    }
    Ok(weights)
}

/// Pair the three per-round inputs into logged rounds.
pub fn logged_rounds(
    rewards: &[f64],
    propensities: &[f64],
    targets: &[f64],
) -> Result<Vec<LoggedRound>, OffPolicyEvaluationError> {
    if rewards.len() != propensities.len() || rewards.len() != targets.len() {
        return Err(OffPolicyEvaluationError::LengthMismatch {
            rewards: rewards.len(),
            propensities: propensities.len(),
            targets: targets.len(),
        });
    }
    Ok(rewards
        .iter()
        .zip(propensities)
        .zip(targets)
        .map(|((&reward, &propensity), &target)| LoggedRound { reward, propensity, target })
        .collect())
}

pub fn policy_value(rounds: &[LoggedRound], estimator: Estimator) -> Result<f64, OffPolicyEvaluationError> {
    Ok(numpy_mean(&round_rewards(rounds, estimator)?))
}

/// Percentile bootstrap of the mean of `samples`, drawing indices from `RandomState(seed)`.
pub fn bootstrap_interval(
    samples: &[f64],
    alpha: f64,
    rounds: usize,
    seed: u32,
) -> Result<BootstrapInterval, OffPolicyEvaluationError> {
    if samples.is_empty() {
        return Err(OffPolicyEvaluationError::EmptyLog);
    }
    if !(alpha > 0.0 && alpha < 1.0) {
        return Err(OffPolicyEvaluationError::InvalidAlpha(alpha));
    }
    if rounds == 0 {
        return Err(OffPolicyEvaluationError::NoBootstrapRounds);
    }
    let mut rng = Mt19937::seeded(seed);
    let n = samples.len();
    let mut drawn = vec![0.0; n];
    let mut means = Vec::with_capacity(rounds);
    for _ in 0..rounds {
        for value in &mut drawn {
            *value = samples[rng.randint(n as u64) as usize];
        }
        means.push(numpy_mean(&drawn));
    }
    let lower = 100.0 * (alpha / 2.0);
    let upper = 100.0 * (1.0 - alpha / 2.0);
    Ok(BootstrapInterval {
        mean: numpy_mean(&means),
        lower: numpy_percentile(&means, lower / 100.0),
        upper: numpy_percentile(&means, upper / 100.0),
    })
}

/// The percentile bootstrap of [`bootstrap_interval`] for several sample vectors of one length at once: every
/// vector is resampled with the same drawn indices, so each interval equals a separate call with the same seed.
/// `on_round` receives the number of rounds finished.
pub fn bootstrap_intervals(
    samples: &[&[f64]],
    alpha: f64,
    rounds: usize,
    seed: u32,
    mut on_round: impl FnMut(usize),
) -> Result<Vec<BootstrapInterval>, OffPolicyEvaluationError> {
    let n = samples.first().map_or(0, |first| first.len());
    if n == 0 || samples.iter().any(|sample| sample.len() != n) {
        return Err(OffPolicyEvaluationError::EmptyLog);
    }
    if !(alpha > 0.0 && alpha < 1.0) {
        return Err(OffPolicyEvaluationError::InvalidAlpha(alpha));
    }
    if rounds == 0 {
        return Err(OffPolicyEvaluationError::NoBootstrapRounds);
    }
    let mut rng = Mt19937::seeded(seed);
    let mut indices = vec![0usize; n];
    let mut drawn = vec![0.0; n];
    let mut means = vec![Vec::with_capacity(rounds); samples.len()];
    for round in 0..rounds {
        for index in &mut indices {
            *index = rng.randint(n as u64) as usize;
        }
        for (sample, found) in samples.iter().zip(&mut means) {
            for (value, &index) in drawn.iter_mut().zip(&indices) {
                *value = sample[index];
            }
            found.push(numpy_mean(&drawn));
        }
        on_round(round + 1);
    }
    let lower = 100.0 * (alpha / 2.0);
    let upper = 100.0 * (1.0 - alpha / 2.0);
    Ok(means
        .iter()
        .map(|found| BootstrapInterval {
            mean: numpy_mean(found),
            lower: numpy_percentile(found, lower / 100.0),
            upper: numpy_percentile(found, upper / 100.0),
        })
        .collect())
}

/// How well the logged rows support a target rule's weighted estimate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeightDiagnostics {
    /// Share of rows whose importance weight is positive.
    pub support: f64,
    pub max_weight: f64,
    /// Kish's effective sample size, (Σw)² ÷ Σw², the convention of cobalt's `ESS`.
    pub effective_sample_size: f64,
}

/// Diagnose the importance weights `target ÷ propensity` of every round.
pub fn weight_diagnostics(rounds: &[LoggedRound]) -> Result<WeightDiagnostics, OffPolicyEvaluationError> {
    let weights = weights(rounds)?;
    let total: f64 = weights.iter().sum();
    let squares: f64 = weights.iter().map(|w| w * w).sum();
    Ok(WeightDiagnostics {
        support: weights.iter().filter(|&&w| w > 0.0).count() as f64 / weights.len() as f64,
        max_weight: weights.iter().copied().fold(0.0, f64::max),
        effective_sample_size: if squares > 0.0 { total * total / squares } else { 0.0 },
    })
}

/// Each round's weighted reward under the target rule minus its logged reward: the samples whose mean is the
/// rule's estimated gain over the logging policy on the same rows.
pub fn paired_differences(rounds: &[LoggedRound], estimator: Estimator) -> Result<Vec<f64>, OffPolicyEvaluationError> {
    let mut samples = round_rewards(rounds, estimator)?;
    for (sample, row) in samples.iter_mut().zip(rounds) {
        *sample -= row.reward;
    }
    Ok(samples)
}
