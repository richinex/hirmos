//! The synthetic control weight problem of 134: minimise `||Xw - y||^2` subject to
//! `sum(w) == 1` and `w >= 0`.
//!
//! 134 solves this with cvxpy and SCS, a first order solver whose default tolerance is 1e-4.
//! This is a primal active set method, which returns the exact optimum instead.

use nalgebra::{DMatrix, DVector};

pub struct SyntheticControl {
    pub weights: Vec<f64>,
    /// `problem.solve()`, which returns the objective value `||Xw - y||^2`.
    pub loss: f64,
    pub iterations: usize,
}

/// Least squares on the KKT system, through the pseudo-inverse so a rank deficient
/// `X'X` block still gives a descent direction.
fn kkt_solve(g: &DMatrix<f64>, rhs: &DVector<f64>) -> DVector<f64> {
    let m = g.nrows();
    let mut kkt = DMatrix::<f64>::zeros(m + 1, m + 1);
    kkt.view_mut((0, 0), (m, m)).copy_from(g);
    for i in 0..m {
        kkt[(i, m)] = 1.0;
        kkt[(m, i)] = 1.0;
    }
    let mut b = DVector::<f64>::zeros(m + 1);
    b.rows_mut(0, m).copy_from(rhs);
    let svd = kkt.svd(true, true);
    let eps = 1e-13 * svd.singular_values.max();
    svd.solve(&b, eps).expect("KKT solve")
}

pub fn fit_synthetic_control(x: &DMatrix<f64>, y: &DVector<f64>) -> SyntheticControl {
    let n = x.ncols();
    let g = x.transpose() * x * 2.0;
    let c = x.transpose() * y * -2.0;

    // The uniform weights are interior, so the working set starts empty.
    let mut w = DVector::from_element(n, 1.0 / n as f64);
    let mut active = vec![false; n];
    let tol = 1e-12;
    let mut iterations = 0;

    for _ in 0..200 * n.max(1) {
        iterations += 1;
        let free: Vec<usize> = (0..n).filter(|&i| !active[i]).collect();
        let grad = &g * &w + &c;

        let gff = DMatrix::from_fn(free.len(), free.len(), |i, j| g[(free[i], free[j])]);
        let rhs = DVector::from_fn(free.len(), |i, _| -grad[free[i]]);
        let sol = kkt_solve(&gff, &rhs);
        let lambda = sol[free.len()];

        let mut p = DVector::<f64>::zeros(n);
        for (i, &idx) in free.iter().enumerate() {
            p[idx] = sol[i];
        }

        if p.amax() <= tol {
            // Multipliers on the bounds. A negative one means that bound is holding the
            // objective up, so it leaves the working set.
            let mut worst = 0.0;
            let mut drop = None;
            for i in 0..n {
                if active[i] {
                    let mu = grad[i] + lambda;
                    if mu < worst {
                        worst = mu;
                        drop = Some(i);
                    }
                }
            }
            match drop {
                Some(i) => active[i] = false,
                None => break,
            }
            continue;
        }

        // Step to the first bound the direction runs into, capped at the full step.
        let mut alpha = 1.0;
        let mut blocking = None;
        for &i in &free {
            if p[i] < -tol {
                let limit = -w[i] / p[i];
                if limit < alpha {
                    alpha = limit;
                    blocking = Some(i);
                }
            }
        }
        w += alpha * p;
        for i in 0..n {
            if w[i] < 0.0 {
                w[i] = 0.0;
            }
        }
        if let Some(i) = blocking {
            active[i] = true;
            w[i] = 0.0;
        }
    }

    let resid = x * &w - y;
    SyntheticControl {
        weights: w.iter().copied().collect(),
        loss: resid.iter().map(|v| v * v).sum(),
        iterations,
    }
}

/// The treatment effect 134 reports: the gap between the treated series and its synthetic
/// counterpart, post-intervention.
pub struct SyntheticEffect {
    pub pre_gap: Vec<f64>,
    pub post_gap: Vec<f64>,
    pub att: f64,
}

pub fn synthetic_effect(
    y_pre_co: &DMatrix<f64>,
    y_pre_tr: &DVector<f64>,
    y_post_co: &DMatrix<f64>,
    y_post_tr: &DVector<f64>,
) -> (SyntheticControl, SyntheticEffect) {
    let sc = fit_synthetic_control(y_pre_co, y_pre_tr);
    let w = DVector::from_row_slice(&sc.weights);
    let pre = y_pre_tr - y_pre_co * &w;
    let post = y_post_tr - y_post_co * &w;
    let att = post.iter().sum::<f64>() / post.len() as f64;
    let effect = SyntheticEffect {
        pre_gap: pre.iter().copied().collect(),
        post_gap: post.iter().copied().collect(),
        att,
    };
    (sc, effect)
}
