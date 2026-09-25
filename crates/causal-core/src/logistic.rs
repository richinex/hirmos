//! sklearn's binary LogisticRegression at defaults: fused half-binomial loss and gradient with
//! the Cython stability branches, l2 on coefficients only, scipy's L-BFGS-B at lbfgs settings.

use crate::lbfgsb::lbfgsb;

pub struct Logistic {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub termination: crate::lbfgsb::LbfgsbTermination,
}

/// closs_grad_half_binomial: loss and gradient per sample with log1pexp's branch cutoffs.
fn closs_grad(y: f64, raw: f64) -> (f64, f64) {
    if raw <= -37.0 {
        let e = raw.exp();
        (e - y * raw, e - y)
    } else if raw <= -2.0 {
        let e = raw.exp();
        (libm::log1p(e) - y * raw, ((1.0 - y) * e - y) / (1.0 + e))
    } else if raw <= 18.0 {
        let e = (-raw).exp();
        (
            libm::log1p(e) + (1.0 - y) * raw,
            ((1.0 - y) - y * e) / (1.0 + e),
        )
    } else {
        let e = (-raw).exp();
        (e + (1.0 - y) * raw, ((1.0 - y) - y * e) / (1.0 + e))
    }
}

/// `LogisticRegression`'s own default.
pub const SKLEARN_DEFAULT_MAX_ITER: usize = 100;

pub fn fit_logistic(x: &[Vec<f64>], y: &[f64], max_iter: usize) -> Logistic {
    fit_with_penalty(x, y, true, max_iter)
}

/// sklearn's LogisticRegression(penalty=None), retaining optimizer status. `max_iter` is the
/// caller's because sklearn's own default of 100 is not what every caller asks for.
pub fn fit_unpenalized(x: &[Vec<f64>], y: &[f64], max_iter: usize) -> Logistic {
    fit_with_penalty(x, y, false, max_iter)
}

fn fit_with_penalty(x: &[Vec<f64>], y: &[f64], penalized: bool, max_iter: usize) -> Logistic {
    let n = x.len();
    let p = x[0].len();
    let l2 = if penalized { 1.0 / n as f64 } else { 0.0 };
    let n_dof = p + 1;
    let res = lbfgsb(
        &vec![0.0; n_dof],
        &vec![f64::NEG_INFINITY; n_dof],
        &vec![f64::INFINITY; n_dof],
        &vec![0i32; n_dof],
        10,
        64.0,
        1e-4,
        50,
        max_iter,
        |w| {
            let sw_sum = n as f64;
            let mut loss_sum = 0.0;
            let mut gp = vec![0.0; n];
            for i in 0..n {
                let mut raw = w[p];
                for j in 0..p {
                    raw += x[i][j] * w[j];
                }
                let (li, gi) = closs_grad(y[i], raw);
                loss_sum += li;
                gp[i] = gi / sw_sum;
            }
            let w2: f64 = w[..p].iter().map(|v| v * v).sum();
            let loss = loss_sum / sw_sum + 0.5 * l2 * w2;
            let mut grad = vec![0.0; n_dof];
            for j in 0..p {
                let mut s = 0.0;
                for (i, g) in gp.iter().enumerate() {
                    s += x[i][j] * g;
                }
                grad[j] = s + l2 * w[j];
            }
            grad[p] = gp.iter().sum();
            (loss, grad)
        },
    );
    Logistic {
        coef: res.x[..p].to_vec(),
        intercept: res.x[p],
        termination: res.termination,
    }
}

impl Logistic {
    pub fn predict_probability(&self, x: &[Vec<f64>]) -> Vec<f64> {
        x.iter()
            .map(|row| {
                let raw =
                    self.intercept + row.iter().zip(&self.coef).map(|(v, c)| v * c).sum::<f64>();
                if raw >= 0.0 {
                    1.0 / (1.0 + (-raw).exp())
                } else {
                    let e = raw.exp();
                    e / (1.0 + e)
                }
            })
            .collect()
    }

    /// predict: class 1 where the decision function is positive.
    pub fn predict(&self, x: &[Vec<f64>]) -> Vec<f64> {
        x.iter()
            .map(|row| {
                let raw =
                    self.intercept + row.iter().zip(&self.coef).map(|(v, c)| v * c).sum::<f64>();
                f64::from(raw > 0.0)
            })
            .collect()
    }
}
