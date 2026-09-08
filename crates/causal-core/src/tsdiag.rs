//! The time series diagnostics batch: autocorrelation and partial autocorrelation, the
//! Ljung-Box whiteness test, an AR(1) prewhitening fit, Granger causality by the sum of
//! squares F test, and a VAR with a constant for one step ahead forecasting.

use crate::{least_squares, linalg, ols::Ols};
use nalgebra::{DMatrix, DVector};

/// Upper tail of the chi-square distribution, as `scipy.stats.chi2.sf`.
fn chi2_sf(x: f64, df: f64) -> f64 {
    spec_math::cephes64::igamc(df / 2.0, x / 2.0)
}

/// Upper tail of the F distribution, as `scipy.stats.f.sf`.
fn f_sf(x: f64, dfn: f64, dfd: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    spec_math::cephes64::incbet(dfd / 2.0, dfn / 2.0, dfd / (dfd + dfn * x))
}

/// `statsmodels.tsa.stattools.acf` at its defaults: demeaned, divided by the sample size
/// rather than the overlap, and normalised by the lag zero term.
pub fn acf(x: &[f64], nlags: usize) -> Vec<f64> {
    let n = x.len();
    let mean = x.iter().sum::<f64>() / n as f64;
    let xo: Vec<f64> = x.iter().map(|v| v - mean).collect();
    let mut acov = vec![0.0; nlags + 1];
    acov[0] = xo.iter().map(|v| v * v).sum::<f64>() / n as f64;
    for k in 1..=nlags {
        acov[k] = xo[k..]
            .iter()
            .zip(&xo[..n - k])
            .map(|(a, b)| a * b)
            .sum::<f64>()
            / n as f64;
    }
    acov.iter().map(|v| v / acov[0]).collect()
}

fn pacf_yw(x: &[f64], nlags: usize, adjusted: bool) -> Vec<f64> {
    let n = x.len();
    let mean = x.iter().sum::<f64>() / n as f64;
    let xo: Vec<f64> = x.iter().map(|v| v - mean).collect();
    let mut r = vec![0.0; nlags + 1];
    r[0] = xo.iter().map(|v| v * v).sum::<f64>() / n as f64;
    for k in 1..=nlags {
        // statsmodels' adjusted form divides by the overlap; its MLE form keeps n.
        r[k] = xo[..n - k]
            .iter()
            .zip(&xo[k..])
            .map(|(a, b)| a * b)
            .sum::<f64>()
            / if adjusted { (n - k) as f64 } else { n as f64 };
    }
    let mut out = vec![1.0];
    for k in 1..=nlags {
        let toeplitz = DMatrix::from_fn(k, k, |i, j| r[i.abs_diff(j)]);
        let rhs = DVector::from_fn(k, |i, _| r[i + 1]);
        let right = DMatrix::from_column_slice(k, 1, rhs.as_slice());
        let rho = linalg::solve(&toeplitz, &right).unwrap_or_else(|_| {
            &linalg::pseudo_inverse(&toeplitz, 1e-15)
                .expect("Yule-Walker pseudoinverse")
                .matrix
                * right
        });
        out.push(rho[k - 1]);
    }
    out
}

/// `pacf(method="ywadjusted")`: for each order, solve the Yule-Walker system built from
/// autocovariances divided by the overlap, and keep the last coefficient.
pub fn pacf_yw_adjusted(x: &[f64], nlags: usize) -> Vec<f64> {
    pacf_yw(x, nlags, true)
}

/// `plot_pacf`'s default `method="ywm"`: Yule-Walker with maximum-likelihood
/// autocovariances, whose denominator remains the full sample size at every lag.
pub fn pacf_yw_mle(x: &[f64], nlags: usize) -> Vec<f64> {
    pacf_yw(x, nlags, false)
}

/// The positive, zero-centred 95% limits drawn by statsmodels' default `plot_acf` and
/// `plot_pacf`. ACF uses Bartlett's lag-dependent variance; PACF uses 1/sqrt(n).
pub fn correlation_plot_limits(acf_values: &[f64], observations: usize) -> (Vec<f64>, Vec<f64>) {
    const Z_975: f64 = 1.959_963_984_540_054;
    let mut acf_limits = vec![0.0; acf_values.len()];
    let mut preceding_squares = 0.0;
    for lag in 1..acf_values.len() {
        if lag > 1 {
            preceding_squares += acf_values[lag - 1] * acf_values[lag - 1];
        }
        acf_limits[lag] = Z_975
            * ((1.0
                + if lag > 1 {
                    2.0 * preceding_squares
                } else {
                    0.0
                })
                / observations as f64)
                .sqrt();
    }
    let pacf_limit = Z_975 / (observations as f64).sqrt();
    let mut pacf_limits = vec![pacf_limit; acf_values.len()];
    pacf_limits[0] = 0.0;
    (acf_limits, pacf_limits)
}

/// `acorr_ljungbox`: the cumulative Ljung-Box statistic and its chi-square p value.
pub fn ljung_box(x: &[f64], max_lag: usize) -> (Vec<f64>, Vec<f64>) {
    let n = x.len() as f64;
    let sacf = acf(x, max_lag);
    let mut running = 0.0;
    let mut stats = Vec::with_capacity(max_lag);
    let mut pvalues = Vec::with_capacity(max_lag);
    for k in 1..=max_lag {
        running += sacf[k] * sacf[k] / (n - k as f64);
        let q = n * (n + 2.0) * running;
        stats.push(q);
        pvalues.push(chi2_sf(q, k as f64));
    }
    (stats, pvalues)
}

/// Ordinary least squares returning the coefficients, residuals and residual sum of squares.
fn ols(design: &DMatrix<f64>, y: &DVector<f64>) -> (DVector<f64>, DVector<f64>, f64) {
    let fit = Ols::fit(design, y);
    (fit.params, fit.resid, fit.ssr)
}

pub struct AutoRegFit {
    /// Intercept then the lag one coefficient, in statsmodels' order.
    pub params: Vec<f64>,
    pub pvalues: Vec<f64>,
    pub resid: Vec<f64>,
}

/// `AutoReg(x, lags=1).fit()` with its default constant trend. statsmodels forms the
/// information matrix with the maximum likelihood residual variance, dividing by the number
/// of observations rather than the residual degrees of freedom, and reports normal rather
/// than Student t p values.
pub fn autoreg1(x: &[f64]) -> AutoRegFit {
    let n = x.len();
    let nobs = n - 1;
    let design = DMatrix::from_fn(nobs, 2, |r, c| if c == 0 { 1.0 } else { x[r] });
    let y = DVector::from_fn(nobs, |r, _| x[r + 1]);
    let fit = Ols::fit(&design, &y);
    let sigma2 = fit.ssr / nobs as f64;
    let xtx_inv = fit.xtx_inverse();
    let pvalues = (0..2)
        .map(|i| {
            let se = (sigma2 * xtx_inv[(i, i)]).sqrt();
            // AutoReg reports normal p values: its results carry use_t = False.
            libm::erfc((fit.params[i] / se).abs() / std::f64::consts::SQRT_2)
        })
        .collect();
    AutoRegFit {
        params: fit.params.iter().copied().collect(),
        pvalues,
        resid: fit.resid.iter().copied().collect(),
    }
}

/// `grangercausalitytests` restricted to the statistic the notebooks read: the sum of
/// squares F test at each lag, comparing the target's own past against its past plus the
/// candidate cause's past.
pub fn granger_ssr_ftest(target: &[f64], cause: &[f64], max_lag: usize) -> Vec<(f64, f64)> {
    let n = target.len();
    let mut out = Vec::with_capacity(max_lag);
    for lag in 1..=max_lag {
        let rows = n - lag;
        let y = DVector::from_fn(rows, |r, _| target[r + lag]);
        // Restricted: a constant and the target's own lags.
        let restricted = DMatrix::from_fn(rows, lag + 1, |r, c| {
            if c == lag {
                1.0
            } else {
                target[r + lag - 1 - c]
            }
        });
        // Unrestricted: the same plus the candidate cause's lags.
        let joint = DMatrix::from_fn(rows, 2 * lag + 1, |r, c| {
            if c == 2 * lag {
                1.0
            } else if c < lag {
                target[r + lag - 1 - c]
            } else {
                cause[r + lag - 1 - (c - lag)]
            }
        });
        let (_, _, ssr_restricted) = ols(&restricted, &y);
        let (_, _, ssr_joint) = ols(&joint, &y);
        let df_resid = (rows - (2 * lag + 1)) as f64;
        let f = (ssr_restricted - ssr_joint) / ssr_joint / lag as f64 * df_resid;
        out.push((f, f_sf(f, lag as f64, df_resid)));
    }
    out
}

pub struct VarFit {
    /// Stacked coefficients in statsmodels' layout: the constant row then each lag block.
    pub params: Vec<Vec<f64>>,
    pub aic: f64,
    pub bic: f64,
    /// Residual covariance divided by the number of observations.
    pub sigma_u_mle: DMatrix<f64>,
    pub lags: usize,
}

impl VarFit {
    /// One step ahead forecast from the last `lags` observations, most recent last.
    pub fn forecast(&self, history: &[Vec<f64>]) -> Vec<f64> {
        let k = self.params[0].len();
        let mut out = self.params[0].clone();
        for lag in 1..=self.lags {
            let past = &history[history.len() - lag];
            for (j, value) in out.iter_mut().enumerate().take(k) {
                for (m, p) in past.iter().enumerate() {
                    *value += self.params[1 + (lag - 1) * k + m][j] * p;
                }
            }
        }
        out
    }
}

/// `VAR(endog).fit(maxlags=p, ic=None, trend="c")`: least squares on a constant and the
/// stacked lags, with the information criteria statsmodels reports.
pub fn var_fit_trend_c(endog: &[Vec<f64>], lags: usize) -> VarFit {
    let t = endog.len();
    let k = endog[0].len();
    let nobs = t - lags;
    // statsmodels orders the design as the constant followed by lag one, lag two, and so on.
    let design = DMatrix::from_fn(nobs, 1 + k * lags, |r, c| {
        if c == 0 {
            1.0
        } else {
            let idx = c - 1;
            let lag = idx / k + 1;
            endog[lags + r - lag][idx % k]
        }
    });
    let y = DMatrix::from_fn(nobs, k, |r, j| endog[lags + r][j]);
    let beta = least_squares::solve(&design, &y, 1e-15)
        .expect("DGELSD VAR fit")
        .coefficients;
    let resid = &y - &design * &beta;
    let sigma_u_mle = resid.transpose() * &resid / nobs as f64;

    let logdet = linalg::logdet_positive_definite_lower(&sigma_u_mle)
        .expect("residual covariance is definite");
    let free_params = (lags * k * k + k) as f64;
    let aic = logdet + (2.0 / nobs as f64) * free_params;
    let bic = logdet + ((nobs as f64).ln() / nobs as f64) * free_params;

    let params = (0..1 + k * lags)
        .map(|row| (0..k).map(|j| beta[(row, j)]).collect())
        .collect();
    VarFit {
        params,
        aic,
        bic,
        sigma_u_mle,
        lags,
    }
}
