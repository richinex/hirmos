//! VECM ported 1:1 from statsmodels: AIC lag selection over the VAR wrapper, Johansen rank
//! selection, and ML estimation with deterministic terms inside or outside the cointegration
//! relation, including the alpha inference the 805 workflow reads.

use crate::coint::coint_johansen;
use crate::{least_squares, linalg};
use nalgebra::{DMatrix, DVector};

pub mod forecast;

fn norm_sf(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}

fn min_norm_lstsq(a: &DMatrix<f64>, b: &DMatrix<f64>) -> DMatrix<f64> {
    least_squares::solve(a, b, 1e-15)
        .expect("DGELSD least-squares solve failed")
        .coefficients
}

/// vecm.select_order at the 805 call: returns the AIC-selected k_ar_diff.
pub fn vecm_select_order(endog: &[Vec<f64>], maxlags: usize, deterministic: &str) -> usize {
    let t_tot = endog.len();
    let k = endog[0].len();
    let mut exog_cols: Vec<Vec<f64>> = Vec::new();
    if deterministic.contains("co") || deterministic.contains("ci") {
        exog_cols.push(vec![1.0; t_tot]);
    }
    if deterministic.contains("lo") || deterministic.contains("li") {
        exog_cols.push((0..t_tot).map(|i| 1.0 + i as f64).collect());
    }

    let mut aics = Vec::new();
    for p in 1..=maxlags + 1 {
        let offset = maxlags + 1 - p;
        let nobs = t_tot - p - offset;
        let n_exog = exog_cols.len();
        let cols = 1 + n_exog + k * p;
        let mut z = DMatrix::<f64>::zeros(nobs, cols);
        let mut y = DMatrix::<f64>::zeros(nobs, k);
        for row in 0..nobs {
            let tt = offset + p + row;
            z[(row, 0)] = 1.0;
            for (e, col) in exog_cols.iter().enumerate() {
                z[(row, 1 + e)] = col[tt];
            }
            for lag in 1..=p {
                for j in 0..k {
                    z[(row, 1 + n_exog + (lag - 1) * k + j)] = endog[tt - lag][j];
                }
            }
            for j in 0..k {
                y[(row, j)] = endog[tt][j];
            }
        }
        let params = min_norm_lstsq(&z, &y);
        let resid = &y - &z * &params;
        let df_resid = (nobs - (k * p + 1 + n_exog)) as f64;
        let sse = resid.transpose() * &resid;
        let sigma_u_mle = sse / df_resid * (df_resid / nobs as f64);
        let ld = linalg::logdet_positive_definite_lower(&sigma_u_mle)
            .expect("sigma_u not positive definite");
        let free_params = (p * k * k + k * (1 + n_exog)) as f64;
        aics.push(ld + 2.0 / nobs as f64 * free_params);
    }
    let mut best = 0usize;
    for (i, v) in aics.iter().enumerate() {
        if *v < aics[best] {
            best = i;
        }
    }
    best
}

/// select_coint_rank with the trace test: first hypothesis whose statistic undercuts the
/// critical value at the significance column (0=90%, 1=95%, 2=99%).
pub fn select_coint_rank(
    endog: &[Vec<f64>],
    det_order: i32,
    k_ar_diff: usize,
    signif_col: usize,
) -> usize {
    let res = coint_johansen(endog, det_order, k_ar_diff);
    let neqs = res.lr1.len();
    for r in 0..neqs {
        if res.lr1[r] < res.cvt[r][signif_col] {
            return r;
        }
    }
    neqs
}

pub struct VecmResult {
    forecast_history: Vec<Vec<f64>>,
    sample_length: usize,
    deterministic: String,
    /// beta' y[t-1], including restricted deterministic terms; rows are relationships.
    /// Column zero belongs to input row k_ar_diff, not the following outcome row.
    pub cointegrating_residuals: DMatrix<f64>,
    pub alpha: DMatrix<f64>,
    /// The endogenous rows of the cointegration matrix, identity-normalised on top.
    pub beta: DMatrix<f64>,
    pub det_coef_coint: DMatrix<f64>,
    pub gamma: DMatrix<f64>,
    pub sigma_u: DMatrix<f64>,
    pub pvalues_alpha: DMatrix<f64>,
}

fn mat_sqrt(m: &DMatrix<f64>) -> DMatrix<f64> {
    let decomposition = linalg::svd_some(m).expect("DGESDD matrix square root failed");
    let scaled_right = DMatrix::from_fn(
        decomposition.right_transposed.nrows(),
        decomposition.right_transposed.ncols(),
        |row, column| {
            decomposition.singular_values[row].sqrt()
                * decomposition.right_transposed[(row, column)]
        },
    );
    decomposition.left * scaled_right
}

/// VECM(endog, k_ar_diff, coint_rank, deterministic).fit() by maximum likelihood.
pub fn vecm_fit(
    endog: &[Vec<f64>],
    k_ar_diff: usize,
    coint_rank: usize,
    deterministic: &str,
) -> VecmResult {
    let t_tot = endog.len();
    let k = endog[0].len();
    let p = k_ar_diff + 1;
    let t = t_tot - p;
    let r = coint_rank;

    // y is (K, T_tot); the estimation sample drops the presample of length p.
    let y = DMatrix::from_fn(k, t_tot, |i, j| endog[j][i]);
    let delta_y = DMatrix::from_fn(k, t_tot - 1, |i, j| y[(i, j + 1)] - y[(i, j)]);
    let delta_y_1_t = delta_y.columns(p - 1, t).into_owned();

    let mut y_lag1_rows: Vec<Vec<f64>> = (0..k)
        .map(|i| (0..t).map(|j| y[(i, p - 1 + j)]).collect())
        .collect();
    if deterministic.contains("ci") {
        y_lag1_rows.push(vec![1.0; t]);
    }
    if deterministic.contains("li") {
        y_lag1_rows.push((0..t).map(|j| (p + j) as f64).collect());
    }
    let y_lag1 = DMatrix::from_fn(y_lag1_rows.len(), t, |i, j| y_lag1_rows[i][j]);

    let mut delta_x_rows: Vec<Vec<f64>> = Vec::new();
    for lag in 1..=k_ar_diff {
        for i in 0..k {
            delta_x_rows.push((0..t).map(|j| delta_y[(i, j + p - 1 - lag)]).collect());
        }
    }
    if deterministic.contains("co") {
        delta_x_rows.push(vec![1.0; t]);
    }
    if deterministic.contains("lo") {
        delta_x_rows.push((0..t).map(|j| (p + j + 1) as f64).collect());
    }
    let delta_x = DMatrix::from_fn(delta_x_rows.len(), t, |i, j| delta_x_rows[i][j]);

    // Residualize on delta_x.
    let dx_dx = &delta_x * delta_x.transpose();
    let dx_dx_inv = linalg::inverse(&dx_dx).expect("delta_x'delta_x singular");
    let residualize = |m: &DMatrix<f64>| -> DMatrix<f64> {
        m - &(m * delta_x.transpose()) * &dx_dx_inv * &delta_x
    };
    let r0 = residualize(&delta_y_1_t);
    let r1 = residualize(&y_lag1);

    let s00 = &r0 * r0.transpose() / t as f64;
    let s01 = &r0 * r1.transpose() / t as f64;
    let s11 = &r1 * r1.transpose() / t as f64;
    let s11_ = linalg::inverse(&mat_sqrt(&s11)).expect("sqrt(s11) singular");
    let s01_s11_ = &s01 * &s11_;
    let s00_inv = linalg::inverse(&s00).expect("s00 singular");
    let m = s01_s11_.transpose() * &s00_inv * &s01_s11_;
    let decomposition =
        linalg::eigen_general_right(&m).expect("VECM general eigendecomposition failed");
    assert!(
        decomposition
            .imaginary_values
            .iter()
            .all(|value| *value == 0.0),
        "VECM eigendecomposition produced complex roots"
    );
    let mut order = crate::numpy_argsort::argsort(decomposition.real_values.as_slice());
    order.reverse();
    let dim = y_lag1.nrows();
    let mut v = DMatrix::<f64>::zeros(dim, r);
    for (c, &idx) in order.iter().take(r).enumerate() {
        for row in 0..dim {
            v[(row, c)] = decomposition.right_vectors[(row, idx)];
        }
    }

    let beta_raw = &s11_ * v;
    let top = linalg::inverse(&beta_raw.rows(0, r).into_owned()).expect("beta top block singular");
    let beta_full = beta_raw * top;
    let bsb = linalg::inverse(&(beta_full.transpose() * &s11 * &beta_full))
        .expect("beta'S11 beta singular");
    let alpha = &s01 * &beta_full * bsb;
    let gamma = (&delta_y_1_t - &alpha * beta_full.transpose() * &y_lag1)
        * delta_x.transpose()
        * &dx_dx_inv;
    let temp = &delta_y_1_t - &alpha * beta_full.transpose() * &y_lag1 - &gamma * &delta_x;
    let sigma_u = &temp * temp.transpose() / t as f64;

    // cov_params_default p.296: block-diag of beta_full and the identity over gamma columns.
    let num_det = deterministic.contains("co") as usize + deterministic.contains("lo") as usize;
    let ident = k * k_ar_diff + num_det;
    let bid_rows = dim + ident;
    let bid_cols = r + ident;
    let mut b_id = DMatrix::<f64>::zeros(bid_rows, bid_cols);
    b_id.view_mut((0, 0), (dim, r)).copy_from(&beta_full);
    for i in 0..ident {
        b_id[(dim + i, r + i)] = 1.0;
    }
    let b_y = beta_full.transpose() * &y_lag1;
    let omega11 = &b_y * b_y.transpose();
    let omega12 = &b_y * delta_x.transpose();
    let omega22 = &delta_x * delta_x.transpose();
    let osize = r + delta_x.nrows();
    let mut omega = DMatrix::<f64>::zeros(osize, osize);
    omega.view_mut((0, 0), (r, r)).copy_from(&omega11);
    omega
        .view_mut((0, r), (r, delta_x.nrows()))
        .copy_from(&omega12);
    omega
        .view_mut((r, 0), (delta_x.nrows(), r))
        .copy_from(&omega12.transpose());
    omega
        .view_mut((r, r), (delta_x.nrows(), delta_x.nrows()))
        .copy_from(&omega22);
    let mat1 = &b_id * linalg::inverse(&omega).expect("omega singular") * b_id.transpose();

    let mut pvalues_alpha = DMatrix::<f64>::zeros(k, r);
    for j in 0..r {
        for i in 0..k {
            let se = (mat1[(j, j)] * sigma_u[(i, i)]).sqrt();
            let tv = alpha[(i, j)] / se;
            pvalues_alpha[(i, j)] = 2.0 * norm_sf(tv.abs());
        }
    }

    VecmResult {
        forecast_history: endog[t_tot-p..].to_vec(),
        sample_length: t_tot,
        deterministic: deterministic.to_owned(),
        cointegrating_residuals: b_y,
        alpha,
        beta: beta_full.rows(0, k).into_owned(),
        det_coef_coint: beta_full.rows(k, dim - k).into_owned(),
        gamma,
        sigma_u,
        pvalues_alpha,
    }
}

/// Chow break test at index k for y ~ 1 + x, as 805 computes it inline: F and its p-value.
pub fn chow_break(y: &[f64], x: &[f64], k: usize) -> (f64, f64) {
    let n = y.len();
    let fit = |ys: &[f64], xs: &[f64]| -> f64 {
        let m = ys.len();
        let design = DMatrix::from_fn(m, 2, |i, j| if j == 0 { 1.0 } else { xs[i] });
        let yv = DVector::from_column_slice(ys);
        let targets = DMatrix::from_column_slice(m, 1, yv.as_slice());
        let tolerance = f64::EPSILON * m.max(2) as f64;
        let beta = least_squares::solve(&design, &targets, tolerance)
            .expect("DGELSD least-squares solve failed")
            .coefficients;
        let fitted = design * beta;
        (0..m).map(|i| (ys[i] - fitted[i]).powi(2)).sum()
    };
    let rss_pooled = fit(y, x);
    let rss_split = fit(&y[..k], &x[..k]) + fit(&y[k..], &x[k..]);
    let dfn = 2.0;
    let dfd = (n - 4) as f64;
    let f = ((rss_pooled - rss_split) / dfn) / (rss_split / dfd);
    // F survival via the incomplete beta.
    let p = spec_math::cephes64::incbet(dfd / 2.0, dfn / 2.0, dfd / (dfd + dfn * f));
    (f, p)
}
