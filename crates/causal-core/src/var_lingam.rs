//! Port of: lingam lingam/var_lingam.py and lingam/direct_lingam.py (MIT); the VAR fit follows
//! statsmodels statsmodels/tsa/vector_ar/var_model.py (BSD-3-Clause).
//!
//! VARLiNGAM ported 1:1 from lingam: statsmodels VAR with a BIC lag sweep, DirectLiNGAM (pwling)
//! on the residuals, then adaptive-lasso pruning over contemporaneous and lagged blocks.

use crate::lars::predict_adaptive_lasso;
use nalgebra::DMatrix;

pub struct VarLingamResult {
    pub k_ar: usize,
    pub ar_coefs: Vec<DMatrix<f64>>,
    pub residuals: DMatrix<f64>,
    pub causal_order: Vec<usize>,
    pub adjacency_matrices: Vec<DMatrix<f64>>,
}

fn logdet_symm(m: &DMatrix<f64>) -> f64 {
    let chol = m
        .clone()
        .cholesky()
        .expect("covariance not positive definite");
    2.0 * chol.l().diagonal().iter().map(|v| v.ln()).sum::<f64>()
}

/// One statsmodels VAR fit with trend "n": returns (coefs per lag, residuals, bic).
fn var_fit(x: &DMatrix<f64>, lags: usize) -> (Vec<DMatrix<f64>>, DMatrix<f64>, f64) {
    let t = x.nrows();
    let k = x.ncols();
    let nobs = t - lags;
    let mut z = DMatrix::<f64>::zeros(nobs, k * lags);
    for row in 0..nobs {
        let tt = lags + row;
        for lag in 1..=lags {
            for j in 0..k {
                z[(row, (lag - 1) * k + j)] = x[(tt - lag, j)];
            }
        }
    }
    let y = x.rows(lags, nobs).into_owned();
    let qr = z.clone().qr();
    let params = qr
        .r()
        .solve_upper_triangular(&(qr.q().transpose() * &y))
        .expect("VAR design is rank deficient");
    let resid = &y - &z * &params;

    let df_resid = (nobs - k * lags) as f64;
    let sse = resid.transpose() * &resid;
    let sigma_u_mle = sse / df_resid * (df_resid / nobs as f64);
    let free_params = (lags * k * k) as f64;
    let bic = logdet_symm(&sigma_u_mle) + (nobs as f64).ln() / nobs as f64 * free_params;

    let coefs = (1..=lags)
        .map(|tau| DMatrix::from_fn(k, k, |i, j| params[((tau - 1) * k + j, i)]))
        .collect();
    (coefs, resid, bic)
}

/// The oracle's criterion sweep: fit each lag count up to max_lags, keep the first BIC minimum.
fn estimate_var_coefs(
    x: &DMatrix<f64>,
    max_lags: usize,
) -> (Vec<DMatrix<f64>>, usize, DMatrix<f64>) {
    let mut best: Option<(Vec<DMatrix<f64>>, usize, DMatrix<f64>)> = None;
    let mut min_value = f64::INFINITY;
    for lag in 1..=max_lags {
        let (coefs, resid, bic) = var_fit(x, lag);
        if bic < min_value {
            min_value = bic;
            best = Some((coefs, lag, resid));
        }
    }
    let (coefs, lag, resid) = best.expect("at least one lag");
    (coefs, lag, resid)
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn std0(v: &[f64]) -> f64 {
    let m = mean(v);
    (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / v.len() as f64).sqrt()
}

/// xi minus its projection on xj: cov(bias)/var slope, as the oracle's _residual.
fn pair_residual(xi: &[f64], xj: &[f64]) -> Vec<f64> {
    let mi = mean(xi);
    let mj = mean(xj);
    let n = xi.len() as f64;
    let cov = xi
        .iter()
        .zip(xj)
        .map(|(a, b)| (a - mi) * (b - mj))
        .sum::<f64>()
        / n;
    let var = xj.iter().map(|b| (b - mj) * (b - mj)).sum::<f64>() / n;
    let slope = cov / var;
    xi.iter().zip(xj).map(|(a, b)| a - slope * b).collect()
}

/// The maximum-entropy approximation of differential entropy.
fn entropy(u: &[f64]) -> f64 {
    const K1: f64 = 79.047;
    const K2: f64 = 7.4129;
    const GAMMA: f64 = 0.37457;
    let n = u.len() as f64;
    let logcosh = u.iter().map(|v| v.cosh().ln()).sum::<f64>() / n;
    let gauss = u.iter().map(|v| v * (-(v * v) / 2.0).exp()).sum::<f64>() / n;
    (1.0 + (2.0 * std::f64::consts::PI).ln()) / 2.0
        - K1 * (logcosh - GAMMA).powi(2)
        - K2 * gauss.powi(2)
}

fn diff_mutual_info(xi_std: &[f64], xj_std: &[f64], ri_j: &[f64], rj_i: &[f64]) -> f64 {
    let si = std0(ri_j);
    let sj = std0(rj_i);
    let ri: Vec<f64> = ri_j.iter().map(|v| v / si).collect();
    let rj: Vec<f64> = rj_i.iter().map(|v| v / sj).collect();
    (entropy(xj_std) + entropy(&ri)) - (entropy(xi_std) + entropy(&rj))
}

fn search_causal_order(x: &DMatrix<f64>, u: &[usize]) -> usize {
    if u.len() == 1 {
        return u[0];
    }
    let standardized = |col: usize| -> Vec<f64> {
        let v: Vec<f64> = x.column(col).iter().copied().collect();
        let m = mean(&v);
        let s = std0(&v);
        v.iter().map(|a| (a - m) / s).collect()
    };
    let mut best = u[0];
    let mut best_score = f64::NEG_INFINITY;
    for &i in u {
        let mut m_total = 0.0;
        for &j in u {
            if i != j {
                let xi_std = standardized(i);
                let xj_std = standardized(j);
                let ri_j = pair_residual(&xi_std, &xj_std);
                let rj_i = pair_residual(&xj_std, &xi_std);
                let d = diff_mutual_info(&xi_std, &xj_std, &ri_j, &rj_i);
                m_total += d.min(0.0).powi(2);
            }
        }
        let score = -m_total;
        if score > best_score {
            best_score = score;
            best = i;
        }
    }
    best
}

/// DirectLiNGAM with the pwling measure and adaptive-lasso adjacency.
pub fn direct_lingam(x: &DMatrix<f64>) -> (Vec<usize>, DMatrix<f64>) {
    let n = x.ncols();
    let mut u: Vec<usize> = (0..n).collect();
    let mut order: Vec<usize> = Vec::new();
    let mut work = x.clone();
    for _ in 0..n {
        let m = search_causal_order(&work, &u);
        for &i in u.clone().iter() {
            if i != m {
                let xi: Vec<f64> = work.column(i).iter().copied().collect();
                let xm: Vec<f64> = work.column(m).iter().copied().collect();
                let resid = pair_residual(&xi, &xm);
                for (row, v) in resid.into_iter().enumerate() {
                    work[(row, i)] = v;
                }
            }
        }
        order.push(m);
        u.retain(|&v| v != m);
    }

    let mut b = DMatrix::<f64>::zeros(n, n);
    for i in 1..n {
        let target = order[i];
        let predictors = &order[..i];
        let coef = predict_adaptive_lasso(x, predictors, target);
        for (k, &p) in predictors.iter().enumerate() {
            b[(target, p)] = coef[k];
        }
    }
    (order, b)
}

/// The oracle's _pruning: adaptive lasso of each variable on its contemporaneous ancestors and
/// every lagged block, rows walked from the end of the sample.
fn pruning(x: &DMatrix<f64>, b_taus: &mut [DMatrix<f64>], order: &[usize], lags: usize) {
    let t = x.nrows();
    let n = x.ncols();
    let rows = t - lags;
    for i in 0..n {
        let pos = order.iter().position(|&v| v == i).unwrap();
        let ancestors = &order[..pos];
        let width = pos + n * lags;
        let mut con = DMatrix::<f64>::zeros(rows, width + 1);
        for j in 0..rows {
            for (k, &a) in ancestors.iter().enumerate() {
                con[(j, k)] = x[(t - 1 - j, a)];
            }
            for lag in 1..=lags {
                for f in 0..n {
                    con[(j, pos + (lag - 1) * n + f)] = x[(t - 1 - j - lag, f)];
                }
            }
            con[(j, width)] = x[(t - 1 - j, i)];
        }
        let predictors: Vec<usize> = (0..width).collect();
        let coef = predict_adaptive_lasso(&con, &predictors, width);
        for (k, &a) in ancestors.iter().enumerate() {
            b_taus[0][(i, a)] = coef[k];
        }
        for lag in 1..=lags {
            for f in 0..n {
                b_taus[lag][(i, f)] = coef[pos + (lag - 1) * n + f];
            }
        }
    }
}

/// lingam's VARLiNGAM(criterion="bic").fit.
pub fn run_var_lingam(data: &[Vec<f64>], lags: usize, prune: bool) -> VarLingamResult {
    let t = data.len();
    let k = data[0].len();
    let x = DMatrix::from_fn(t, k, |i, j| data[i][j]);

    let (m_taus, k_ar, residuals) = estimate_var_coefs(&x, lags);
    let (causal_order, b0) = direct_lingam(&residuals);

    let eye = DMatrix::<f64>::identity(k, k);
    let mut b_taus: Vec<DMatrix<f64>> = vec![b0.clone()];
    for m in &m_taus {
        b_taus.push((&eye - &b0) * m);
    }
    if prune {
        pruning(&x, &mut b_taus, &causal_order, k_ar);
    }

    VarLingamResult {
        k_ar,
        ar_coefs: m_taus,
        residuals,
        causal_order,
        adjacency_matrices: b_taus,
    }
}

/// The oracle's _calc_residuals: recursive residuals under fixed AR coefficients.
fn calc_residuals(x: &DMatrix<f64>, m_taus: &[DMatrix<f64>]) -> DMatrix<f64> {
    let t = x.nrows();
    let k = x.ncols();
    let lags = m_taus.len();
    let mut out = DMatrix::<f64>::zeros(t - lags, k);
    for row in lags..t {
        for c in 0..k {
            let mut est = 0.0;
            for (tau, m) in m_taus.iter().enumerate() {
                for j in 0..k {
                    est += m[(c, j)] * x[(row - tau - 1, j)];
                }
            }
            out[(row - lags, c)] = x[(row, c)] - est;
        }
    }
    out
}

pub struct VarBootstrapResult {
    /// One horizontally concatenated (K, K*(1+lags)) matrix per bootstrap sample.
    pub adjacency_matrices: Vec<DMatrix<f64>>,
}

impl VarBootstrapResult {
    /// BootstrapResult.get_probabilities: per-lag block of exceedance frequencies.
    pub fn get_probabilities(&self, min_causal_effect: f64) -> Vec<DMatrix<f64>> {
        let k = self.adjacency_matrices[0].nrows();
        let blocks = self.adjacency_matrices[0].ncols() / k;
        let mut bp = DMatrix::<f64>::zeros(k, k * blocks);
        for am in &self.adjacency_matrices {
            for i in 0..k {
                for j in 0..k * blocks {
                    if am[(i, j)].abs() > min_causal_effect {
                        bp[(i, j)] += 1.0;
                    }
                }
            }
        }
        bp /= self.adjacency_matrices.len() as f64;
        (0..blocks)
            .map(|b| bp.columns(b * k, k).into_owned())
            .collect()
    }
}

/// VARLiNGAM.bootstrap after a fit, with numpy's global RandomState seeded: resample the
/// recursive residuals, rebuild the series under the fitted AR coefficients, refit each sample.
pub fn var_lingam_bootstrap(
    data: &[Vec<f64>],
    lags: usize,
    n_sampling: usize,
    seed: u32,
) -> VarBootstrapResult {
    let mut mt = crate::nprandom::Mt19937::seeded(seed);
    var_lingam_bootstrap_with(data, lags, n_sampling, &mut mt)
}

/// Bootstrap on a caller-owned stream against the model's own fitted VAR coefficients, the
/// behaviour of lingam's bootstrap on a pre-fitted model.
pub fn var_lingam_bootstrap_with(
    data: &[Vec<f64>],
    lags: usize,
    n_sampling: usize,
    mt: &mut crate::nprandom::Mt19937,
) -> VarBootstrapResult {
    let fitted = run_var_lingam(data, lags, true);
    var_lingam_bootstrap_stateful(data, lags, n_sampling, mt, Some(&fitted.ar_coefs)).0
}

/// lingam's VARLiNGAM.bootstrap verbatim, including its statefulness: with no carried
/// coefficients (a fresh model) the VAR is re-estimated for every resample, and the last
/// resample's coefficients are returned because lingam's next bootstrap call silently
/// reuses them for the following dataset.
pub fn var_lingam_bootstrap_stateful(
    data: &[Vec<f64>],
    lags: usize,
    n_sampling: usize,
    mt: &mut crate::nprandom::Mt19937,
    carried_coefs: Option<&[DMatrix<f64>]>,
) -> (VarBootstrapResult, Vec<DMatrix<f64>>) {
    let t = data.len();
    let k = data[0].len();
    let x = DMatrix::from_fn(t, k, |i, j| data[i][j]);

    let m_taus: Vec<DMatrix<f64>> = match carried_coefs {
        Some(coefs) => coefs.to_vec(),
        None => estimate_var_coefs(&x, lags).0,
    };
    let k_ar = m_taus.len();
    let residuals = calc_residuals(&x, &m_taus);
    let m = residuals.nrows();
    let eye = DMatrix::<f64>::identity(k, k);
    let mut adjacency_matrices = Vec::with_capacity(n_sampling);
    let mut last_coefs = m_taus.clone();
    for _ in 0..n_sampling {
        let idx: Vec<usize> = (0..t).map(|_| mt.randint(m as u64) as usize).collect();
        let mut resampled = DMatrix::<f64>::zeros(t, k);
        for j in 0..t {
            if j < k_ar {
                for c in 0..k {
                    resampled[(j, c)] = residuals[(idx[j], c)];
                }
            } else {
                for c in 0..k {
                    let mut ar = 0.0;
                    for (tau, mm) in m_taus.iter().enumerate() {
                        for jj in 0..k {
                            ar += mm[(c, jj)] * resampled[(j - tau - 1, jj)];
                        }
                    }
                    resampled[(j, c)] = ar + residuals[(idx[j], c)];
                }
            }
        }
        let sample_m_taus: Vec<DMatrix<f64>> = match carried_coefs {
            Some(coefs) => coefs.to_vec(),
            None => estimate_var_coefs(&resampled, lags).0,
        };
        let resid_i = calc_residuals(&resampled, &sample_m_taus);
        let (order, b0) = direct_lingam(&resid_i);
        let mut b_taus: Vec<DMatrix<f64>> = vec![b0.clone()];
        for mm in &sample_m_taus {
            b_taus.push((&eye - &b0) * mm);
        }
        pruning(&resampled, &mut b_taus, &order, sample_m_taus.len());
        let mut am = DMatrix::<f64>::zeros(k, k * b_taus.len());
        for (b, mat) in b_taus.iter().enumerate() {
            am.columns_mut(b * k, k).copy_from(mat);
        }
        adjacency_matrices.push(am);
        last_coefs = sample_m_taus;
    }
    (VarBootstrapResult { adjacency_matrices }, last_coefs)
}

/// lingam.utils.get_common_edge_probabilities in its 'across' mode: per-dataset edge
/// frequency over the bootstrap samples, multiplied elementwise across datasets.
pub fn common_edge_probabilities(results: &[VarBootstrapResult]) -> DMatrix<f64> {
    let shape = results[0].adjacency_matrices[0].shape();
    let mut product = DMatrix::<f64>::from_element(shape.0, shape.1, 1.0);
    for result in results {
        let count = result.adjacency_matrices.len() as f64;
        let mut freq = DMatrix::<f64>::zeros(shape.0, shape.1);
        for am in &result.adjacency_matrices {
            for r in 0..shape.0 {
                for c in 0..shape.1 {
                    if am[(r, c)] != 0.0 {
                        freq[(r, c)] += 1.0;
                    }
                }
            }
        }
        product.zip_apply(&freq, |p, f| *p *= f / count);
    }
    product
}
