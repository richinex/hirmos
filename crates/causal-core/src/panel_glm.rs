//! statsmodels 0.14.6 GLM/discrete likelihoods and score sandwich covariance.
use crate::{glm, least_squares, linalg};
use nalgebra::{DMatrix, DVector};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    Shape,
    NonFinite,
    Count,
    Trials,
    Exposure,
    Rank,
    DegreesOfFreedom,
    Clusters,
    TimeOrder,
    Separation,
    Numerical,
    Contrast,
    Confidence,
    Iterations,
}
pub enum Family<'a> {
    NegativeBinomial {
        counts: &'a [f64],
        exposure: &'a [f64],
    },
    Binomial {
        successes: &'a [f64],
        trials: &'a [f64],
    },
}
pub enum Uncertainty<'a> {
    ModelBased,
    Cluster {
        ids: &'a [u64],
        correction: bool,
    },
    Hac {
        times: &'a [i64],
        lags: usize,
        correction: bool,
    },
}
pub struct Fit {
    pub coefficients: Vec<f64>,
    pub covariance: DMatrix<f64>,
    pub scores: DMatrix<f64>,
    pub bread: DMatrix<f64>,
    pub fitted: Vec<f64>,
    pub converged: bool,
    pub iterations: usize,
    pub line_search_failed: bool,
}
fn count(v: f64) -> bool {
    v.is_finite() && v >= 0.0 && v.fract() == 0.0
}

pub fn fit(
    x: &DMatrix<f64>,
    family: Family<'_>,
    max_iterations: usize,
    tolerance: f64,
) -> Result<Fit, Error> {
    let (n, k) = x.shape();
    if n == 0 || k == 0 {
        return Err(Error::Shape);
    }
    if x.iter().any(|v| !v.is_finite()) {
        return Err(Error::NonFinite);
    }
    if max_iterations == 0 || !tolerance.is_finite() || tolerance <= 0.0 {
        return Err(Error::Iterations);
    }
    if n <= k {
        return Err(Error::DegreesOfFreedom);
    }
    let rank = least_squares::solve(x, &DMatrix::zeros(n, 1), 1e-12)
        .map_err(|_| Error::Numerical)?
        .rank;
    if rank != k {
        return Err(Error::Rank);
    }
    match family {
        Family::NegativeBinomial { counts, exposure } => {
            if counts.len() != n || exposure.len() != n {
                return Err(Error::Shape);
            }
            if counts.iter().any(|v| !count(*v)) {
                return Err(Error::Count);
            }
            if exposure.iter().any(|v| !v.is_finite() || *v <= 0.0) {
                return Err(Error::Exposure);
            }
            if counts.iter().all(|v| *v == 0.0) {
                return Err(Error::Separation);
            }
            if n <= k + 1 {
                return Err(Error::DegreesOfFreedom);
            }
            let offset: Vec<f64> = exposure.iter().map(|v| v.ln()).collect();
            let model =
                glm::negative_binomial_p_bounded(counts, x, &offset, tolerance, max_iterations)
                    .map_err(|_| Error::Numerical)?;
            if model
                .params
                .iter()
                .chain(model.covariance.iter())
                .any(|v| !v.is_finite())
                || model.params[k] <= 0.0
            {
                return Err(Error::Numerical);
            }
            Ok(Fit {
                coefficients: model.params,
                covariance: model.covariance.clone(),
                bread: model.covariance,
                scores: model.scores,
                fitted: model.fitted.iter().map(|v| v.exp()).collect(),
                converged: model.converged,
                iterations: model.nit,
                line_search_failed: model.line_search_failed,
            })
        }
        Family::Binomial { successes, trials } => {
            binomial(x, successes, trials, max_iterations, tolerance)
        }
    }
}

fn binomial(
    x: &DMatrix<f64>,
    successes: &[f64],
    trials: &[f64],
    max_iterations: usize,
    tolerance: f64,
) -> Result<Fit, Error> {
    let (n, k) = x.shape();
    if successes.len() != n || trials.len() != n {
        return Err(Error::Shape);
    }
    if successes.iter().any(|v| !count(*v)) {
        return Err(Error::Count);
    }
    if trials.iter().any(|v| !count(*v) || *v == 0.0)
        || successes.iter().zip(trials).any(|(s, t)| s > t)
    {
        return Err(Error::Trials);
    }
    let y: Vec<f64> = successes.iter().zip(trials).map(|(s, t)| s / t).collect();
    let mut mu: Vec<f64> = y.iter().map(|v| (v + 0.5) / 2.0).collect();
    let mut eta: Vec<f64> = mu.iter().map(|v| (v / (1.0 - v)).ln()).collect();
    let deviance = |mu: &[f64]| -> f64 {
        y.iter()
            .zip(mu)
            .zip(trials)
            .map(|((&y, &m), &t)| {
                2.0 * t
                    * (y * (y / (m + 1e-20)).max(f64::EPSILON).ln()
                        + (1.0 - y) * ((1.0 - y) / (1.0 - m + 1e-20)).max(f64::EPSILON).ln())
            })
            .sum()
    };
    let mut dev = deviance(&mu);
    let mut weights = vec![0.0; n];
    let mut z = DVector::zeros(n);
    let mut beta = DVector::zeros(k);
    let mut converged = false;
    let mut iterations = 0;
    for iteration in 0..max_iterations {
        for i in 0..n {
            let p = mu[i].clamp(f64::EPSILON, 1.0 - f64::EPSILON);
            let derivative = 1.0 / (p * (1.0 - p));
            weights[i] = trials[i] / (derivative * derivative * p * (1.0 - p));
            z[i] = eta[i] + derivative * (y[i] - mu[i]);
        }
        beta = glm::wls_lstsq(x, &z, &weights);
        for i in 0..n {
            eta[i] = (0..k).map(|j| x[(i, j)] * beta[j]).sum();
            mu[i] = 1.0 / (1.0 + (-eta[i]).exp());
        }
        if beta.iter().chain(mu.iter()).any(|v| !v.is_finite()) {
            return Err(Error::Numerical);
        }
        if mu
            .iter()
            .zip(&y)
            .all(|(m, y)| (m - y).abs() <= 1e-8 + 1e-5 * y.abs())
        {
            return Err(Error::Separation);
        }
        let next = deviance(&mu);
        iterations = iteration + 1;
        converged = (next - dev).abs() <= tolerance;
        dev = next;
        if converged {
            break;
        }
    }
    let (_, covariance) = glm::wls_pinv(x, &z, &weights);
    let scores = DMatrix::from_fn(n, k, |i, j| x[(i, j)] * (successes[i] - trials[i] * mu[i]));
    let information = DMatrix::from_fn(k, k, |j, l| {
        (0..n)
            .map(|i| trials[i] * mu[i] * (1.0 - mu[i]) * x[(i, j)] * x[(i, l)])
            .sum()
    });
    let bread = linalg::inverse(&information).map_err(|_| Error::Numerical)?;
    Ok(Fit {
        coefficients: beta.iter().copied().collect(),
        covariance,
        scores,
        bread,
        fitted: mu,
        converged,
        iterations,
        line_search_failed: false,
    })
}

pub fn covariance(fit: &Fit, uncertainty: Uncertainty<'_>) -> Result<DMatrix<f64>, Error> {
    let (n, k) = fit.scores.shape();
    let (meat, correction) = match uncertainty {
        Uncertainty::ModelBased => return Ok(fit.covariance.clone()),
        Uncertainty::Cluster { ids, correction } => {
            if ids.len() != n {
                return Err(Error::Shape);
            }
            let mut sums: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
            for (i, id) in ids.iter().enumerate() {
                let row = sums.entry(*id).or_insert_with(|| vec![0.0; k]);
                for j in 0..k {
                    row[j] += fit.scores[(i, j)];
                }
            }
            let g = sums.len();
            if g < 2 {
                return Err(Error::Clusters);
            }
            if correction && n <= k {
                return Err(Error::DegreesOfFreedom);
            }
            let meat = DMatrix::from_fn(k, k, |j, l| sums.values().map(|s| s[j] * s[l]).sum());
            (
                meat,
                if correction {
                    g as f64 / (g - 1) as f64 * (n - 1) as f64 / (n - k) as f64
                } else {
                    1.0
                },
            )
        }
        Uncertainty::Hac {
            times,
            lags,
            correction,
        } => {
            if times.len() != n {
                return Err(Error::Shape);
            }
            if lags >= n || times.windows(2).any(|p| p[1].checked_sub(p[0]) != Some(1)) {
                return Err(Error::TimeOrder);
            }
            if correction && n <= k {
                return Err(Error::DegreesOfFreedom);
            }
            let mut meat = fit.scores.transpose() * &fit.scores;
            for lag in 1..=lags {
                let weight = 1.0 - lag as f64 / (lags + 1) as f64;
                for j in 0..k {
                    for l in 0..k {
                        meat[(j, l)] += weight
                            * (lag..n)
                                .map(|i| {
                                    fit.scores[(i, j)] * fit.scores[(i - lag, l)]
                                        + fit.scores[(i, l)] * fit.scores[(i - lag, j)]
                                })
                                .sum::<f64>();
                    }
                }
            }
            (
                meat,
                if correction {
                    n as f64 / (n - k) as f64
                } else {
                    1.0
                },
            )
        }
    };
    let covariance = (&fit.bread * meat * fit.bread.transpose()) * correction;
    if covariance.iter().any(|v| !v.is_finite()) {
        return Err(Error::Numerical);
    }
    Ok(covariance)
}

pub struct Contrast {
    pub estimate: f64,
    pub standard_error: f64,
    pub lower: f64,
    pub upper: f64,
    pub ratio: f64,
    pub ratio_lower: f64,
    pub ratio_upper: f64,
}
/// Normal-Wald contrast, matching statsmodels t_test(..., use_t=False).
pub fn contrast(
    coefficients: &[f64],
    covariance: &DMatrix<f64>,
    weights: &[f64],
    confidence: f64,
) -> Result<Contrast, Error> {
    let k = coefficients.len();
    if covariance.shape() != (k, k) || weights.len() != k {
        return Err(Error::Shape);
    }
    if !confidence.is_finite() || confidence <= 0.0 || confidence >= 1.0 {
        return Err(Error::Confidence);
    }
    if coefficients
        .iter()
        .chain(weights)
        .chain(covariance.iter())
        .any(|v| !v.is_finite())
        || weights.iter().all(|w| *w == 0.0)
    {
        return Err(Error::Contrast);
    }
    let estimate = coefficients.iter().zip(weights).map(|(b, w)| b * w).sum();
    let c = DVector::from_column_slice(weights);
    let variance = (c.transpose() * covariance * c)[(0, 0)];
    if variance <= 0.0 {
        return Err(Error::Contrast);
    }
    let standard_error = variance.sqrt();
    let critical = spec_math::cephes64::ndtri((1.0 + confidence) / 2.0);
    let lower = estimate - critical * standard_error;
    let upper = estimate + critical * standard_error;
    Ok(Contrast {
        estimate,
        standard_error,
        lower,
        upper,
        ratio: estimate.exp(),
        ratio_lower: lower.exp(),
        ratio_upper: upper.exp(),
    })
}
