//! Port of: scikit-learn sklearn/linear_model/_least_angle.py (BSD-3-Clause) and lingam
//! lingam/utils/__init__.py predict_adaptive_lasso (MIT).
//!
//! sklearn's LARS-lasso path with BIC selection and lingam's adaptive lasso on top, ported 1:1
//! from _lars_path_solver (Gram branch), LassoLarsIC.fit and predict_adaptive_lasso.

use nalgebra::{DMatrix, DVector};

const EQUALITY_TOLERANCE: f64 = f32::EPSILON as f64;
const TINY32: f64 = f32::MIN_POSITIVE as f64;

fn min_pos(values: &[f64]) -> f64 {
    let mut min_val = f64::MAX;
    for &v in values {
        if 0.0 < v && v < min_val {
            min_val = v;
        }
    }
    min_val
}

/// np.around to 15 decimals: scale, rint with ties to even, unscale.
fn around15(v: f64) -> f64 {
    (v * 1e15).round_ties_even() / 1e15
}

/// BLAS drotg with the reference sign convention, returning (c, s, r).
fn drotg(a: f64, b: f64) -> (f64, f64, f64) {
    let roe = if b.abs() > a.abs() { b } else { a };
    let scale = a.abs() + b.abs();
    if scale == 0.0 {
        return (1.0, 0.0, 0.0);
    }
    let r = scale * ((a / scale).powi(2) + (b / scale).powi(2)).sqrt();
    let r = if roe < 0.0 { -r } else { r };
    (a / r, b / r, r)
}

/// sklearn's arrayfuncs.cholesky_delete on the top-left n x n block of the factor.
fn cholesky_delete(l: &mut DMatrix<f64>, n: usize, go_out: usize) {
    for i in go_out..n - 1 {
        for k in 0..i + 2 {
            l[(i, k)] = l[(i + 1, k)];
        }
    }
    for i in go_out..n - 1 {
        let (mut c, mut s, r) = drotg(l[(i, i)], l[(i, i + 1)]);
        l[(i, i)] = r;
        if l[(i, i)] < 0.0 {
            l[(i, i)] = l[(i, i)].abs();
            c = -c;
            s = -s;
        }
        l[(i, i + 1)] = 0.0;
        for row in i + 1..n - 1 {
            let x = l[(row, i)];
            let y = l[(row, i + 1)];
            l[(row, i)] = c * x + s * y;
            l[(row, i + 1)] = c * y - s * x;
        }
    }
}

/// Solve L L^T x = b with the top-left n x n block of the lower factor (LAPACK potrs).
fn solve_cholesky(l: &DMatrix<f64>, n: usize, b: &[f64]) -> Vec<f64> {
    let mut z = b.to_vec();
    for i in 0..n {
        for k in 0..i {
            z[i] -= l[(i, k)] * z[k];
        }
        z[i] /= l[(i, i)];
    }
    for i in (0..n).rev() {
        for k in i + 1..n {
            z[i] -= l[(k, i)] * z[k];
        }
        z[i] /= l[(i, i)];
    }
    z
}

/// The lasso path of _lars_path_solver with a precomputed Gram matrix: returns per-step alphas
/// and coefficient vectors (original feature order).
fn lars_path_lasso_gram(
    x: &DMatrix<f64>,
    y: &DVector<f64>,
    max_iter: usize,
) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n_samples = y.len();
    let n_features = x.ncols();
    let mut cov: Vec<f64> = (0..n_features).map(|j| x.column(j).dot(y)).collect();
    let mut gram = x.transpose() * x;
    let gram_copy = gram.clone();
    let cov_copy = cov.clone();

    let max_features = max_iter.min(n_features);
    let mut coefs: Vec<Vec<f64>> = vec![vec![0.0; n_features]];
    let mut alphas: Vec<f64> = vec![0.0];

    let mut n_iter = 0usize;
    let mut n_active = 0usize;
    let mut active: Vec<usize> = Vec::new();
    let mut indices: Vec<usize> = (0..n_features).collect();
    let mut sign_active: Vec<f64> = vec![0.0; max_features];
    let mut drop = false;
    let mut idx: Vec<usize> = Vec::new();
    let mut l = DMatrix::<f64>::zeros(max_features, max_features);
    let eps = f64::EPSILON;

    loop {
        let (c_idx, c_) = if !cov.is_empty() {
            let mut best = 0usize;
            for (k, v) in cov.iter().enumerate() {
                if v.abs() > cov[best].abs() {
                    best = k;
                }
            }
            (best, cov[best])
        } else {
            (0, 0.0)
        };
        let c = c_.abs();

        let alpha = c / n_samples as f64;
        alphas[n_iter] = alpha;
        let prev_alpha = if n_iter > 0 { alphas[n_iter - 1] } else { 0.0 };
        if alpha <= EQUALITY_TOLERANCE {
            break;
        }

        if n_iter >= max_iter || n_active >= n_features {
            break;
        }

        if !drop {
            sign_active[n_active] = c_.signum();
            let m = n_active;
            let n = c_idx + n_active;

            cov.swap(c_idx, 0);
            let cov_head = cov[0];
            cov.remove(0);
            indices.swap(n, m);

            gram.swap_rows(m, n);
            gram.swap_columns(m, n);
            let diag_c = gram[(n_active, n_active)];
            for k in 0..n_active {
                l[(n_active, k)] = gram[(n_active, k)];
            }

            if n_active > 0 {
                for i in 0..n_active {
                    for k in 0..i {
                        let sub = l[(i, k)] * l[(n_active, k)];
                        l[(n_active, i)] -= sub;
                    }
                    l[(n_active, i)] /= l[(i, i)];
                }
            }
            let v: f64 = (0..n_active)
                .map(|k| l[(n_active, k)] * l[(n_active, k)])
                .sum();
            let diag = (diag_c - v).abs().sqrt().max(eps);
            l[(n_active, n_active)] = diag;

            if diag < 1e-7 {
                // The oracle zeroes the offender and swaps it back in Cov only.
                cov.insert(0, cov_head);
                cov[0] = 0.0;
                cov.swap(c_idx, 0);
                continue;
            }

            active.push(indices[n_active]);
            n_active += 1;
        }

        if n_iter > 0 && prev_alpha < alpha {
            break;
        }

        let mut least_squares = solve_cholesky(&l, n_active, &sign_active[..n_active]);
        let aa;
        if least_squares.len() == 1 && least_squares[0] == 0.0 {
            least_squares[0] = 1.0;
            aa = 1.0;
        } else {
            let dot: f64 = least_squares
                .iter()
                .zip(&sign_active[..n_active])
                .map(|(a, b)| a * b)
                .sum();
            let mut aa_val = 1.0 / dot.sqrt();
            if !aa_val.is_finite() {
                let mut i = 0;
                let mut l_ = l.view((0, 0), (n_active, n_active)).into_owned();
                while !aa_val.is_finite() {
                    for d in 0..n_active {
                        l_[(d, d)] += (2f64).powi(i) * eps;
                    }
                    least_squares = solve_cholesky(&l_, n_active, &sign_active[..n_active]);
                    let tmp: f64 = least_squares
                        .iter()
                        .zip(&sign_active[..n_active])
                        .map(|(a, b)| a * b)
                        .sum::<f64>()
                        .max(eps);
                    aa_val = 1.0 / tmp.sqrt();
                    i += 1;
                }
            }
            aa = aa_val;
            for v in &mut least_squares {
                *v *= aa;
            }
        }

        // Correlation of each inactive feature with the equiangular direction.
        let n_inactive = cov.len();
        let mut corr_eq_dir = vec![0.0; n_inactive];
        for (k, item) in corr_eq_dir.iter_mut().enumerate() {
            let mut acc = 0.0;
            for (row, ls) in least_squares.iter().enumerate() {
                acc += gram[(row, n_active + k)] * ls;
            }
            *item = around15(acc);
        }

        let g1_vals: Vec<f64> = (0..n_inactive)
            .map(|k| (c - cov[k]) / (aa - corr_eq_dir[k] + TINY32))
            .collect();
        let g2_vals: Vec<f64> = (0..n_inactive)
            .map(|k| (c + cov[k]) / (aa + corr_eq_dir[k] + TINY32))
            .collect();
        let mut gamma = min_pos(&g1_vals).min(min_pos(&g2_vals)).min(c / aa);

        drop = false;
        let prev = coefs[n_iter].clone();
        let z: Vec<f64> = active
            .iter()
            .enumerate()
            .map(|(k, &a)| -prev[a] / (least_squares[k] + TINY32))
            .collect();
        let z_pos = min_pos(&z);
        if z_pos < gamma {
            idx = (0..z.len()).filter(|&k| z[k] == z_pos).rev().collect();
            for &ii in &idx {
                sign_active[ii] = -sign_active[ii];
            }
            gamma = z_pos;
            drop = true;
        }

        n_iter += 1;
        let mut coef = prev.clone();
        for (k, &a) in active.iter().enumerate() {
            coef[a] = prev[a] + gamma * least_squares[k];
        }
        coefs.push(coef.clone());
        alphas.push(0.0);

        for (k, item) in cov.iter_mut().enumerate() {
            *item -= gamma * corr_eq_dir[k];
        }

        if drop {
            for &ii in &idx {
                cholesky_delete(&mut l, n_active, ii);
            }
            n_active -= 1;
            let drop_idx: Vec<usize> = idx.iter().map(|&ii| active.remove(ii)).collect();
            for &ii in &idx {
                for i in ii..n_active {
                    indices.swap(i, i + 1);
                    gram.swap_rows(i, i + 1);
                    gram.swap_columns(i, i + 1);
                }
            }
            for &d in drop_idx.iter().rev() {
                let mut temp = cov_copy[d];
                for j in 0..n_features {
                    temp -= gram_copy[(d, j)] * coef[j];
                }
                cov.insert(0, temp);
            }
            // np.delete over the fixed-size sign array, then append a zero.
            let mut kept: Vec<f64> = (0..max_features)
                .filter(|k| !idx.contains(k))
                .map(|k| sign_active[k])
                .collect();
            kept.push(0.0);
            sign_active = kept;
        }
    }

    alphas.truncate(n_iter + 1);
    coefs.truncate(n_iter + 1);
    (alphas, coefs)
}

fn center_columns(x: &DMatrix<f64>) -> DMatrix<f64> {
    let mut out = x.clone();
    for mut col in out.column_iter_mut() {
        let mean = col.iter().sum::<f64>() / col.len() as f64;
        for v in col.iter_mut() {
            *v -= mean;
        }
    }
    out
}

/// Least squares by QR: the coefficients sklearn's LinearRegression finds on full-rank data.
fn lstsq(x: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    let qr = x.clone().qr();
    qr.r()
        .solve_upper_triangular(&(qr.q().transpose() * y))
        .expect("rank deficient regression")
}

/// LinearRegression with intercept: center both sides, then plain least squares.
fn lin_reg(x: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    let xc = center_columns(x);
    let ymean = y.iter().sum::<f64>() / y.len() as f64;
    let yc = DVector::from_iterator(y.len(), y.iter().map(|v| v - ymean));
    lstsq(&xc, &yc)
}

/// LassoLarsIC(criterion="bic").fit(...).coef_ : the path point minimising sklearn's BIC.
pub fn lasso_lars_ic_bic(x: &DMatrix<f64>, y: &DVector<f64>) -> Vec<f64> {
    let n = y.len();
    let xc = center_columns(x);
    let ymean = y.iter().sum::<f64>() / n as f64;
    let yc = DVector::from_iterator(n, y.iter().map(|v| v - ymean));

    let (_alphas, coef_path) = lars_path_lasso_gram(&xc, &yc, 500);

    let p = x.ncols();
    let ols = lstsq(&xc, &yc);
    let fitted = &xc * &ols;
    let rss_ols: f64 = (0..n).map(|i| (yc[i] - fitted[i]).powi(2)).sum();
    let noise_variance = rss_ols / (n - p - 1) as f64;

    let criterion_factor = (n as f64).ln();
    let mut best = 0usize;
    let mut best_crit = f64::INFINITY;
    for (k, coef) in coef_path.iter().enumerate() {
        let mut rss = 0.0;
        for i in 0..n {
            let mut pred = 0.0;
            for (j, &b) in coef.iter().enumerate() {
                pred += xc[(i, j)] * b;
            }
            rss += (yc[i] - pred).powi(2);
        }
        let dof = coef.iter().filter(|v| v.abs() > f64::EPSILON).count();
        let crit = n as f64 * (2.0 * std::f64::consts::PI * noise_variance).ln()
            + rss / noise_variance
            + criterion_factor * dof as f64;
        if crit < best_crit {
            best_crit = crit;
            best = k;
        }
    }
    coef_path[best].clone()
}

/// lingam's predict_adaptive_lasso: adaptive weights from OLS on standardized data, LARS-BIC
/// pruning, then OLS refit of the survivors on the original scale.
pub fn predict_adaptive_lasso(
    data: &DMatrix<f64>,
    predictors: &[usize],
    target: usize,
) -> Vec<f64> {
    let n = data.nrows();
    let mut std_data = data.clone();
    for mut col in std_data.column_iter_mut() {
        let mean = col.iter().sum::<f64>() / n as f64;
        for v in col.iter_mut() {
            *v -= mean;
        }
        let std = (col.iter().map(|v| v * v).sum::<f64>() / n as f64).sqrt();
        if std != 0.0 {
            for v in col.iter_mut() {
                *v /= std;
            }
        }
    }

    let xs = std_data.select_columns(predictors.iter());
    let ys = DVector::from_iterator(n, std_data.column(target).iter().copied());
    let weight: Vec<f64> = lin_reg(&xs, &ys).iter().map(|v| v.abs()).collect();

    let mut xw = xs.clone();
    for (j, mut col) in xw.column_iter_mut().enumerate() {
        for v in col.iter_mut() {
            *v *= weight[j];
        }
    }
    let lasso_coef = lasso_lars_ic_bic(&xw, &ys);

    let pruned: Vec<usize> = (0..predictors.len())
        .filter(|&j| (lasso_coef[j] * weight[j]).abs() > 0.0)
        .collect();
    let mut coef = vec![0.0; predictors.len()];
    if !pruned.is_empty() {
        let cols: Vec<usize> = pruned.iter().map(|&j| predictors[j]).collect();
        let xo = data.select_columns(cols.iter());
        let yo = DVector::from_iterator(n, data.column(target).iter().copied());
        let refit = lin_reg(&xo, &yo);
        for (k, &j) in pruned.iter().enumerate() {
            coef[j] = refit[k];
        }
    }
    coef
}
