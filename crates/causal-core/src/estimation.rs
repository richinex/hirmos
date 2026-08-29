//! The 805 estimation layer ported 1:1 from statsmodels: OLS with Newey-West (HAC) errors and
//! normal-based inference, WLS, and the Durbin-Watson statistic.

use nalgebra::{DMatrix, DVector};

fn norm_sf(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}

pub struct HacOls {
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// 95% normal-based intervals as (lower, upper).
    pub conf_int: Vec<(f64, f64)>,
    pub resid: Vec<f64>,
    pub rsquared: f64,
}

fn lstsq(x: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    let qr = x.clone().qr();
    qr.r()
        .solve_upper_triangular(&(qr.q().transpose() * y))
        .expect("design is rank deficient")
}

/// OLS with cov_type="HAC": Bartlett weights, no small-sample correction, z-based inference.
pub fn ols_hac(y: &[f64], x: &DMatrix<f64>, maxlags: usize) -> HacOls {
    let n = x.nrows();
    let k = x.ncols();
    let yv = DVector::from_column_slice(y);
    let beta = lstsq(x, &yv);
    let fitted = x * &beta;
    let resid: Vec<f64> = (0..n).map(|i| y[i] - fitted[i]).collect();

    // xu rows: exog scaled by the residual.
    let mut xu = DMatrix::<f64>::zeros(n, k);
    for i in 0..n {
        for j in 0..k {
            xu[(i, j)] = x[(i, j)] * resid[i];
        }
    }
    let mut s = xu.transpose() * &xu;
    for lag in 1..=maxlags {
        let w = 1.0 - lag as f64 / (maxlags as f64 + 1.0);
        let top = xu.rows(lag, n - lag).transpose() * xu.rows(0, n - lag);
        s += (&top + top.transpose()) * w;
    }
    let xtx_inv = (x.transpose() * x).try_inverse().expect("X'X is singular");
    let cov = &xtx_inv * s * &xtx_inv;

    let bse: Vec<f64> = (0..k).map(|j| cov[(j, j)].sqrt()).collect();
    let pvalues: Vec<f64> = (0..k)
        .map(|j| 2.0 * norm_sf((beta[j] / bse[j]).abs()))
        .collect();
    let z975 = spec_math::cephes64::ndtri(0.975);
    let conf_int: Vec<(f64, f64)> = (0..k)
        .map(|j| (beta[j] - z975 * bse[j], beta[j] + z975 * bse[j]))
        .collect();

    let ymean = y.iter().sum::<f64>() / n as f64;
    let tss: f64 = y.iter().map(|v| (v - ymean) * (v - ymean)).sum();
    let ssr: f64 = resid.iter().map(|r| r * r).sum();
    HacOls {
        params: beta.iter().copied().collect(),
        bse,
        pvalues,
        conf_int,
        resid,
        rsquared: 1.0 - ssr / tss,
    }
}

pub struct WlsFit {
    pub params: Vec<f64>,
    pub pvalues: Vec<f64>,
}

/// statsmodels WLS with t-based inference at the classic covariance.
pub fn wls(y: &[f64], x: &DMatrix<f64>, weights: &[f64]) -> WlsFit {
    let n = x.nrows();
    let k = x.ncols();
    let mut xw = x.clone();
    let mut yw = DVector::from_column_slice(y);
    for i in 0..n {
        let sw = weights[i].sqrt();
        yw[i] *= sw;
        for j in 0..k {
            xw[(i, j)] *= sw;
        }
    }
    let beta = lstsq(&xw, &yw);
    let fitted = &xw * &beta;
    let ssr: f64 = (0..n).map(|i| (yw[i] - fitted[i]).powi(2)).sum();
    let df = (n - k) as f64;
    let sigma2 = ssr / df;
    let xtx_inv = (xw.transpose() * &xw)
        .try_inverse()
        .expect("X'X is singular");
    let pvalues: Vec<f64> = (0..k)
        .map(|j| {
            let t = beta[j] / (sigma2 * xtx_inv[(j, j)]).sqrt();
            crate::parcorr::analytic_pvalue_t(t.abs(), df)
        })
        .collect();
    WlsFit {
        params: beta.iter().copied().collect(),
        pvalues,
    }
}

/// sum of squared residual differences over the residual sum of squares.
pub fn durbin_watson(resid: &[f64]) -> f64 {
    let num: f64 = resid
        .windows(2)
        .map(|w| (w[1] - w[0]) * (w[1] - w[0]))
        .sum();
    let den: f64 = resid.iter().map(|r| r * r).sum();
    num / den
}
