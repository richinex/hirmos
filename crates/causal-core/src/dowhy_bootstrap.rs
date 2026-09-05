//! DoWhy v0.11's generic bootstrap inference, shared by every estimator that does not implement
//! its own confidence intervals: `sklearn.utils.resample` on NumPy's global legacy `RandomState`,
//! the basic (reverse-percentile) interval, and `np.std` of the resampled estimates as the standard
//! error.

use crate::nprandom::Mt19937;
use crate::numpy_reduce::{numpy_mean, numpy_sum};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DowhyBootstrap {
    pub simulations: usize,
    pub sample_size_fraction: f64,
    pub confidence_level: f64,
    /// DoWhy v0.11 consumes NumPy's global RNG. An explicit seed makes that otherwise hidden
    /// state reproducible at the Rust boundary.
    pub seed: u32,
}

impl Default for DowhyBootstrap {
    fn default() -> Self {
        Self {
            simulations: 399,
            sample_size_fraction: 1.0,
            confidence_level: 0.95,
            seed: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootstrapError {
    InvalidSimulations,
    InvalidSampleFraction,
    InvalidConfidenceLevel,
}

impl Display for BootstrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSimulations => {
                write!(f, "bootstrap simulations must be greater than zero")
            }
            Self::InvalidSampleFraction => write!(
                f,
                "bootstrap sample-size fraction must be finite and greater than zero"
            ),
            Self::InvalidConfidenceLevel => {
                write!(f, "confidence level must be strictly between zero and one")
            }
        }
    }
}

impl Error for BootstrapError {}

impl DowhyBootstrap {
    pub fn validate(&self) -> Result<(), BootstrapError> {
        if self.simulations == 0 {
            return Err(BootstrapError::InvalidSimulations);
        }
        if !self.sample_size_fraction.is_finite() || self.sample_size_fraction <= 0.0 {
            return Err(BootstrapError::InvalidSampleFraction);
        }
        if !self.confidence_level.is_finite()
            || self.confidence_level <= 0.0
            || self.confidence_level >= 1.0
        {
            return Err(BootstrapError::InvalidConfidenceLevel);
        }
        Ok(())
    }

    /// `int(sample_size_fraction * len(data))`.
    pub fn sample_size(&self, rows: usize) -> usize {
        (self.sample_size_fraction * rows as f64) as usize
    }
}

/// One `sklearn.utils.resample(data, n_samples=sample_size)` draw: `randint(0, rows)` per row
/// from the legacy stream, sampling with replacement.
pub fn resample_rows(rng: &mut Mt19937, rows: usize, sample_size: usize) -> Vec<usize> {
    (0..sample_size)
        .map(|_| rng.randint(rows as u64) as usize)
        .collect()
}

/// DoWhy's basic bootstrap interval. It sorts each resampled estimate's deviation from the
/// full-sample estimate, then reverses the selected tails around that estimate; this is not a
/// percentile interval.
pub fn basic_interval(estimate: f64, bootstrap_estimates: &[f64], confidence_level: f64) -> [f64; 2] {
    let mut variations: Vec<f64> = bootstrap_estimates
        .iter()
        .map(|resampled| resampled - estimate)
        .collect();
    variations.sort_by(f64::total_cmp);
    let upper_index = ((1.0 - confidence_level) * variations.len() as f64) as usize;
    let lower_index = (confidence_level * variations.len() as f64) as usize;
    [
        estimate - variations[lower_index],
        estimate - variations[upper_index],
    ]
}

/// `np.std(bootstrap_estimates)`: the population standard deviation, `ddof=0`.
pub fn standard_error(bootstrap_estimates: &[f64]) -> f64 {
    let mean = numpy_mean(bootstrap_estimates);
    let squares: Vec<f64> = bootstrap_estimates
        .iter()
        .map(|estimate| {
            let centred = estimate - mean;
            centred * centred
        })
        .collect();
    (numpy_sum(&squares) / bootstrap_estimates.len() as f64).sqrt()
}
