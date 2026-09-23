//! Numerical summaries from bsts 0.9.11 R/summary.bsts.R, R/diagnostics.R,
//! R/utils.R and BOOM 0.9.16 R/suggest_burn_log_likelihood.R.
//! Copyright Google LLC; LGPL-2.1-or-later sources preserved in vendor.
use crate::Error;
use nalgebra::DMatrix;

#[derive(Debug, PartialEq)]
pub struct Burn {
    pub iterations: usize,
    /// R warns when the suggestion would discard every iteration.
    pub adjusted: bool,
}

pub fn suggest_burn(
    niter: usize,
    fraction: f64,
    log_likelihood: Option<&[f64]>,
) -> Result<Burn, Error> {
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return Err(Error::InvalidProbability);
    }
    if niter as u128 > (1u128 << 53) {
        return Err(Error::Shape);
    }
    let mut burn = match log_likelihood {
        None => (fraction * niter as f64).floor() as usize,
        Some(logs) => {
            if logs.len() != niter {
                return Err(Error::Shape);
            }
            if logs.is_empty() {
                return Err(Error::Empty);
            }
            if logs.iter().any(|v| !v.is_finite()) {
                return Err(Error::NonFinite);
            }
            // R's round uses ties-to-even, not Rust's round-away-from-zero.
            let count = (fraction * logs.len() as f64).round_ties_even() as usize;
            if count == 0 {
                return Err(Error::Empty);
            }
            let threshold =
                hirmos_causal_core::negbin_nuts::quantile(&logs[logs.len() - count..], 0.9);
            logs.iter()
                .position(|v| *v >= threshold)
                .ok_or(Error::NonFinite)?
        }
    };
    let adjusted = burn >= niter;
    if adjusted {
        burn = niter.saturating_sub(1).max(usize::from(niter == 0));
    }
    Ok(Burn {
        iterations: burn.max(1),
        adjusted,
    })
}

#[derive(Debug, PartialEq)]
pub enum Measure {
    Finite(f64),
    Missing,
    NegativeInfinity,
    PositiveInfinity,
}
impl Measure {
    fn from_value(v: f64) -> Self {
        if v.is_nan() {
            Self::Missing
        } else if v == f64::NEG_INFINITY {
            Self::NegativeInfinity
        } else if v == f64::INFINITY {
            Self::PositiveInfinity
        } else {
            Self::Finite(v)
        }
    }
}
#[derive(Debug)]
pub struct GaussianSummary {
    pub residual_sd: f64,
    pub prediction_sd: Measure,
    pub r_squared: f64,
    pub relative_gof: Measure,
}
fn sample_variance(x: &[f64]) -> f64 {
    if x.len() < 2 {
        return f64::NAN;
    }
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    x.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (x.len() - 1) as f64
}
/// No-regression branch of summary.bsts. Missing y is omitted from the
/// response variance, but not from the differenced-series GOF denominator.
pub fn gaussian_summary(
    y: &[Option<f64>],
    sigma: &[f64],
    errors: &DMatrix<f64>,
    burn: usize,
) -> Result<GaussianSummary, Error> {
    if errors.ncols() != y.len() || errors.nrows() != sigma.len() || burn >= sigma.len() {
        return Err(Error::Shape);
    }
    if y.iter()
        .flatten()
        .chain(sigma.iter())
        .chain(errors.iter())
        .any(|v| !v.is_finite())
    {
        return Err(Error::NonFinite);
    }
    if sigma.iter().any(|s| *s < 0.) {
        return Err(Error::InvalidScale);
    }
    let observed: Vec<_> = y.iter().flatten().copied().collect();
    let variance = sample_variance(&observed);
    if !variance.is_finite() || variance <= 0. {
        return Err(Error::InvalidScale);
    }
    let residual_sd = sigma[burn..].iter().sum::<f64>() / (sigma.len() - burn) as f64;
    let means: Vec<_> = (0..errors.ncols())
        .map(|t| {
            (burn..errors.nrows()).map(|i| errors[(i, t)]).sum::<f64>()
                / (errors.nrows() - burn) as f64
        })
        .collect();
    let differences: Option<Vec<_>> = y.windows(2).map(|w| Some(w[1]? - w[0]?)).collect();
    let denominator = differences
        .map(|d| sample_variance(&d) * (d.len() as f64 - 1.))
        .unwrap_or(f64::NAN);
    let r_squared = 1. - residual_sd.powi(2) / variance;
    if !residual_sd.is_finite() || !r_squared.is_finite() {
        return Err(Error::NonFinite);
    }
    Ok(GaussianSummary {
        residual_sd,
        prediction_sd: Measure::from_value(sample_variance(&means).sqrt()),
        r_squared,
        relative_gof: Measure::from_value(
            1. - means.iter().map(|v| v * v).sum::<f64>() / denominator,
        ),
    })
}

/// R residuals are fitted-minus-observed, not observed-minus-fitted.
/// The matrix is the sum of component contributions, after timestamp mapping.
pub fn residuals(
    y: &[Option<f64>],
    fitted: &DMatrix<f64>,
    burn: usize,
) -> Result<Vec<Vec<Option<f64>>>, Error> {
    if fitted.ncols() != y.len() || burn >= fitted.nrows() {
        return Err(Error::Shape);
    }
    if y.iter()
        .flatten()
        .chain(fitted.iter())
        .any(|v| !v.is_finite())
    {
        return Err(Error::NonFinite);
    }
    let mut rows = Vec::with_capacity(fitted.nrows() - burn);
    for i in burn..fitted.nrows() {
        let mut row = Vec::with_capacity(y.len());
        for (t, value) in y.iter().enumerate() {
            let residual = value.map(|v| fitted[(i, t)] - v);
            if residual.is_some_and(|v| !v.is_finite()) {
                return Err(Error::NonFinite);
            }
            row.push(residual);
        }
        rows.push(row);
    }
    Ok(rows)
}
