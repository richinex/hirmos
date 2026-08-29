//! Causal impact: a local level model with static regression on control series, fitted on
//! the pre-intervention window and forecast forward to give the counterfactual.
//!
//! `UnobservedComponents(endog=y, level='llevel', exog=controls)`.
//!
//!   y_t  = mu_t + beta'x_t + eps_t     eps_t  ~ N(0, sigma2_irregular)
//!   mu_t = mu_{t-1} + zeta_t           zeta_t ~ N(0, sigma2_level)
//!
//! so Z = 1, T = 1, R = 1, regressors in the observation intercept. Approximate diffuse
//! initialisation at variance 1e6, first observation dropped from the likelihood.

use crate::ucm::hpfilter;
use nalgebra::{DMatrix, DVector};

const APPROXIMATE_DIFFUSE_VARIANCE: f64 = 1e6;
const LOGLIKELIHOOD_BURN: usize = 1;
const LN_2PI: f64 = 1.8378770664093453;

fn variance(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mean = x.iter().sum::<f64>() / n;
    x.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n
}

/// Least squares by pseudo-inverse, matching numpy's `pinv(exog).dot(resid)`.
fn lstsq(exog: &[Vec<f64>], target: &[f64]) -> Vec<f64> {
    let n = exog.len();
    let k = exog[0].len();
    let a = DMatrix::from_fn(n, k, |r, c| exog[r][c]);
    let b = DVector::from_column_slice(target);
    let svd = a.svd(true, true);
    let eps = 1e-15 * svd.singular_values.max() * n.max(k) as f64;
    svd.solve(&b, eps)
        .expect("least squares solve")
        .iter()
        .copied()
        .collect()
}

/// statsmodels' start parameters for the local level with regressors: one Hodrick-Prescott
/// filter supplies the level variance, the regression is fitted to its cycle, and what is
/// left over supplies the irregular variance.
pub fn start_params(y: &[f64], exog: &[Vec<f64>]) -> Vec<f64> {
    let (cycle, trend) = hpfilter(y, 1600.0);
    let level_var = variance(&trend);
    let beta = lstsq(exog, &cycle);
    let resid: Vec<f64> = cycle
        .iter()
        .enumerate()
        .map(|(t, c)| c - exog[t].iter().zip(&beta).map(|(x, b)| x * b).sum::<f64>())
        .collect();
    let mut params = vec![variance(&resid), level_var];
    params.extend(beta);
    params
}

pub struct FilterOutput {
    pub llf: f64,
    pub loglikeobs: Vec<f64>,
    pub forecasts_error: Vec<f64>,
    /// Predicted state and its variance one step past the sample.
    pub final_state: f64,
    pub final_state_cov: f64,
}

fn regression_mean(exog_row: &[f64], beta: &[f64]) -> f64 {
    exog_row.iter().zip(beta).map(|(x, b)| x * b).sum()
}

/// The Kalman filter for the local level with regressors.
pub fn kalman_filter(params: &[f64], y: &[f64], exog: &[Vec<f64>]) -> FilterOutput {
    let (h, q) = (params[0], params[1]);
    let beta = &params[2..];
    let mut a = 0.0f64;
    let mut p = APPROXIMATE_DIFFUSE_VARIANCE;
    let mut lls: Vec<f64> = Vec::with_capacity(y.len());
    let mut errors = Vec::with_capacity(y.len());
    for (t, &obs) in y.iter().enumerate() {
        let v = obs - regression_mean(&exog[t], beta) - a;
        let f = p + h;
        lls.push(-0.5 * (LN_2PI + f.ln() + v * v / f));
        errors.push(v);
        // Update, then predict through T = 1.
        a += p / f * v;
        p -= p * p / f;
        p += q;
    }
    // statsmodels discards the first observations from the likelihood under an approximate
    // diffuse prior, and reports them as zero rather than as their raw contribution.
    for ll in lls.iter_mut().take(LOGLIKELIHOOD_BURN.min(y.len())) {
        *ll = 0.0;
    }
    let llf = lls.iter().sum();
    FilterOutput {
        llf,
        loglikeobs: lls,
        forecasts_error: errors,
        final_state: a,
        final_state_cov: p,
    }
}

pub fn loglike(params: &[f64], y: &[f64], exog: &[Vec<f64>]) -> f64 {
    kalman_filter(params, y, exog).llf
}

/// Only the two variances are constrained positive by squaring; the regression
/// coefficients pass through untouched.
pub fn transform_params(u: &[f64]) -> Vec<f64> {
    let mut c = u.to_vec();
    c[0] = u[0] * u[0];
    c[1] = u[1] * u[1];
    c
}

pub fn untransform_params(c: &[f64]) -> Vec<f64> {
    let mut u = c.to_vec();
    u[0] = c[0].sqrt();
    u[1] = c[1].sqrt();
    u
}

pub struct ImpactFit {
    pub params: Vec<f64>,
    pub llf: f64,
}

/// Maximum likelihood over the pre-intervention window.
pub fn fit(y: &[f64], exog: &[Vec<f64>], max_iter: usize) -> ImpactFit {
    let start = start_params(y, exog);
    let u0 = untransform_params(&start);
    let n = u0.len();
    let eps = 1e-5;
    let objective = |u: &[f64]| -loglike(&transform_params(u), y, exog) / y.len() as f64;
    let res = crate::lbfgsb::lbfgsb(
        &u0,
        &vec![f64::NEG_INFINITY; n],
        &vec![f64::INFINITY; n],
        &vec![0i32; n],
        10,
        1e7,
        1e-5,
        20,
        max_iter,
        |u| {
            let f0 = objective(u);
            let mut grad = vec![0.0; n];
            for i in 0..n {
                let mut uh = u.to_vec();
                uh[i] += eps;
                let dx = uh[i] - u[i];
                grad[i] = (objective(&uh) - f0) / dx;
            }
            (f0, grad)
        },
    );
    let params = transform_params(&res.x);
    ImpactFit {
        llf: loglike(&params, y, exog),
        params,
    }
}

pub struct Impact {
    /// The counterfactual path over the post-intervention window.
    pub counterfactual: Vec<f64>,
    /// Its forecast standard errors.
    pub counterfactual_se: Vec<f64>,
    pub pointwise: Vec<f64>,
    pub cumulative: f64,
    pub average: f64,
    pub params: Vec<f64>,
    pub llf: f64,
}

/// Fit the pre-intervention window, forecast the counterfactual across the post window, and
/// difference it against what actually happened.
pub fn causal_impact(y: &[f64], exog: &[Vec<f64>], n_pre: usize, max_iter: usize) -> Impact {
    let fitted = fit(&y[..n_pre], &exog[..n_pre], max_iter);
    let filt = kalman_filter(&fitted.params, &y[..n_pre], &exog[..n_pre]);
    let (h, q) = (fitted.params[0], fitted.params[1]);
    let beta = &fitted.params[2..];

    // The level is a random walk, so the forecast stays flat while its variance grows by Q.
    let mut p = filt.final_state_cov;
    let mut counterfactual = Vec::new();
    let mut se = Vec::new();
    for row in exog.iter().skip(n_pre) {
        counterfactual.push(filt.final_state + regression_mean(row, beta));
        se.push((p + h).sqrt());
        p += q;
    }
    let pointwise: Vec<f64> = y[n_pre..]
        .iter()
        .zip(&counterfactual)
        .map(|(a, c)| a - c)
        .collect();
    let cumulative: f64 = pointwise.iter().sum();
    let average = cumulative / pointwise.len() as f64;
    Impact {
        counterfactual,
        counterfactual_se: se,
        pointwise,
        cumulative,
        average,
        params: fitted.params,
        llf: fitted.llf,
    }
}
