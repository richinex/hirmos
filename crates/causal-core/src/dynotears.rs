//! DYNOTEARS ported from causalnex's from_numpy_dynamic: augmented Lagrangian over the acyclicity
//! constraint trace(expm(W*W)) - d, split-positive parameterisation with L1 penalties, box-bounded
//! L-BFGS-B inner solves.

use crate::expm::expm;
use crate::lbfgsb::lbfgsb;
use nalgebra::DMatrix;

pub struct DynotearsResult {
    /// Contemporaneous weights, w[i][j] is i -> j.
    pub w: DMatrix<f64>,
    /// Lagged weights stacked by lag: rows lag-1 block first, a[(k-1)*d + i][j] is i(t-k) -> j.
    pub a: DMatrix<f64>,
}

struct Problem<'a> {
    x: &'a DMatrix<f64>,
    xlags: &'a DMatrix<f64>,
    d: usize,
    p: usize,
    lambda_w: f64,
    lambda_a: f64,
    rho: f64,
    alpha: f64,
}

impl Problem<'_> {
    fn reshape(&self, wa: &[f64]) -> (DMatrix<f64>, DMatrix<f64>) {
        let d = self.d;
        let w = DMatrix::from_fn(d, d, |i, j| wa[i * d + j] - wa[(d + i) * d + j]);
        let base = 2 * d * d;
        let a = DMatrix::from_fn(d * self.p, d, |i, j| {
            let lag = i / d;
            let row = i % d;
            let plus = base + lag * 2 * d * d + row * d + j;
            let minus = base + lag * 2 * d * d + d * d + row * d + j;
            wa[plus] - wa[minus]
        });
        (w, a)
    }

    fn h(&self, w: &DMatrix<f64>) -> f64 {
        let sq = w.component_mul(w);
        expm(&sq).trace() - self.d as f64
    }

    fn func_grad(&self, wa: &[f64]) -> (f64, Vec<f64>) {
        let (w, a) = self.reshape(wa);
        let d = self.d;
        let n = self.x.nrows() as f64;
        let eye = DMatrix::<f64>::identity(d, d);
        let resid = self.x * (&eye - &w) - self.xlags * &a;
        let loss = 0.5 / n * resid.iter().map(|v| v * v).sum::<f64>();
        let e_mat = expm(&w.component_mul(&w));
        let h_value = e_mat.trace() - d as f64;
        let l1: f64 = wa[..2 * d * d].iter().sum::<f64>() * self.lambda_w
            + wa[2 * d * d..].iter().sum::<f64>() * self.lambda_a;
        let f = loss + 0.5 * self.rho * h_value * h_value + self.alpha * h_value + l1;

        let loss_grad_w = -(self.x.transpose() * &resid) / n;
        let h_factor = self.rho * h_value + self.alpha;
        let obj_grad_w = &loss_grad_w + (e_mat.transpose().component_mul(&w)) * (2.0 * h_factor);
        let obj_grad_a = -(self.xlags.transpose() * &resid) / n;

        let mut grad = vec![0.0; wa.len()];
        for i in 0..d {
            for j in 0..d {
                grad[i * d + j] = obj_grad_w[(i, j)] + self.lambda_w;
                grad[(d + i) * d + j] = -obj_grad_w[(i, j)] + self.lambda_w;
            }
        }
        let base = 2 * d * d;
        for lag in 0..self.p {
            for row in 0..d {
                for j in 0..d {
                    let g = obj_grad_a[(lag * d + row, j)];
                    grad[base + lag * 2 * d * d + row * d + j] = g + self.lambda_a;
                    grad[base + lag * 2 * d * d + d * d + row * d + j] = -g + self.lambda_a;
                }
            }
        }
        (f, grad)
    }
}

/// from_numpy_dynamic at causalnex defaults: lambda penalties, max_iter 100, h_tol 1e-8.
pub fn dynotears(
    x: &DMatrix<f64>,
    xlags: &DMatrix<f64>,
    lambda_w: f64,
    lambda_a: f64,
) -> DynotearsResult {
    dynotears_with_progress(x, xlags, lambda_w, lambda_a, |_, _, _| {})
}

/// Fit DYNOTEARS while reporting completed augmented-Lagrangian outer iterations.
pub fn dynotears_with_progress<F>(
    x: &DMatrix<f64>,
    xlags: &DMatrix<f64>,
    lambda_w: f64,
    lambda_a: f64,
    mut progress: F,
) -> DynotearsResult
where
    F: FnMut(&'static str, usize, usize),
{
    let d = x.ncols();
    let p = xlags.ncols() / d;
    let max_iter = 100;
    let h_tol = 1e-8;

    let size = 2 * (p + 1) * d * d;
    let lower = vec![0.0; size];
    let mut upper = vec![f64::INFINITY; size];
    let mut nbd = vec![1i32; size];
    for i in 0..d {
        // Self loops banned in both split blocks of W.
        upper[i * d + i] = 0.0;
        upper[(d + i) * d + i] = 0.0;
        nbd[i * d + i] = 2;
        nbd[(d + i) * d + i] = 2;
    }

    #[allow(unused_mut)]
    let mut problem = Problem {
        x,
        xlags,
        d,
        p,
        lambda_w,
        lambda_a,
        rho: 1.0,
        alpha: 0.0,
    };
    let mut wa_est = vec![0.0; size];
    let mut h_value = f64::INFINITY;
    let mut completed_iterations = 0;

    progress("augmented Lagrangian", 0, max_iter);
    for iteration in 0..max_iter {
        let mut wa_new = wa_est.clone();
        let mut h_new = f64::INFINITY;
        while problem.rho < 1e20 && (h_new > 0.25 * h_value || h_new == f64::INFINITY) {
            let sol = lbfgsb(
                &wa_est,
                &lower,
                &upper,
                &nbd,
                10,
                1e7,
                1e-5,
                20,
                15000,
                |v| problem.func_grad(v),
            );
            wa_new = sol.x;
            let (w, _) = problem.reshape(&wa_new);
            h_new = problem.h(&w);
            if h_new > 0.25 * h_value {
                problem.rho *= 10.0;
            }
        }
        wa_est = wa_new;
        h_value = h_new;
        problem.alpha += problem.rho * h_value;
        completed_iterations = iteration + 1;
        progress("augmented Lagrangian", iteration + 1, max_iter);
        if h_value <= h_tol {
            break;
        }
    }

    let (w, a) = problem.reshape(&wa_est);
    progress(
        "complete",
        completed_iterations,
        completed_iterations.max(1),
    );
    DynotearsResult { w, a }
}
