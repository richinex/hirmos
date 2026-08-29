//! `scipy.optimize.fmin_bfgs`, the optimiser statsmodels reaches for when a discrete model
//! is fitted with `method="bfgs"`. The line search is `line_search_wolfe1`, which drives the
//! same MINPACK `dcsrch` the L-BFGS-B port uses.
//!
//! The `line_search_wolfe2` fallback is not ported. `BfgsResult::line_search_failed` reports
//! whether scipy would have reached for it.

use crate::lbfgsb::{Dcsrch, SrchTask};

#[derive(Clone, Debug)]
pub enum BfgsEvaluation {
    Function { x: Vec<f64>, value: f64 },
    Gradient { x: Vec<f64>, value: Vec<f64> },
}

pub struct BfgsResult {
    pub x: Vec<f64>,
    pub fun: f64,
    pub jac: Vec<f64>,
    pub nfev: usize,
    pub njev: usize,
    pub nit: usize,
    /// scipy's `warnflag`: 0 success, 1 maxiter, 2 line search or loss of precision.
    pub warnflag: u8,
    pub line_search_failed: bool,
    /// Ordered objective and gradient calls, including their evaluation points.
    pub evaluations: Vec<BfgsEvaluation>,
}

struct Counted<'a, F, G> {
    f: &'a F,
    g: &'a G,
    nfev: usize,
    njev: usize,
    evaluations: Vec<BfgsEvaluation>,
}

impl<F: Fn(&[f64]) -> f64, G: Fn(&[f64]) -> Vec<f64>> Counted<'_, F, G> {
    fn fun(&mut self, x: &[f64]) -> f64 {
        self.nfev += 1;
        let value = (self.f)(x);
        self.evaluations.push(BfgsEvaluation::Function {
            x: x.to_vec(),
            value,
        });
        value
    }

    fn grad(&mut self, x: &[f64]) -> Vec<f64> {
        self.njev += 1;
        let value = (self.g)(x);
        self.evaluations.push(BfgsEvaluation::Gradient {
            x: x.to_vec(),
            value: value.clone(),
        });
        value
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn norm2(a: &[f64]) -> f64 {
    dot(a, a).sqrt()
}

fn norm_inf(a: &[f64]) -> f64 {
    a.iter().fold(0.0f64, |m, v| m.max(v.abs()))
}

struct LineSearch {
    stp: Option<f64>,
    phi: f64,
    phi0: f64,
    grad: Vec<f64>,
}

/// `scalar_search_wolfe1` wrapped around `line_search_wolfe1`: the returned step is `None`
/// when dcsrch ends in a warning or an error, which is scipy's signal to try wolfe2.
#[allow(clippy::too_many_arguments)]
fn line_search_wolfe1<F: Fn(&[f64]) -> f64, G: Fn(&[f64]) -> Vec<f64>>(
    ev: &mut Counted<F, G>,
    xk: &[f64],
    pk: &[f64],
    gfk: &[f64],
    phi0: f64,
    old_phi0: Option<f64>,
    c1: f64,
    c2: f64,
    amin: f64,
    amax: f64,
    xtol: f64,
) -> LineSearch {
    let derphi0 = dot(gfk, pk);
    let mut alpha = 1.0;
    if let Some(old) = old_phi0 {
        if derphi0 != 0.0 {
            let candidate = 1.01 * 2.0 * (phi0 - old) / derphi0;
            alpha = candidate.min(1.0);
            if alpha < 0.0 {
                alpha = 1.0;
            }
        }
    }

    let at = |ev: &mut Counted<F, G>, s: f64| -> (f64, Vec<f64>, f64) {
        let point: Vec<f64> = xk.iter().zip(pk).map(|(x, p)| x + s * p).collect();
        let phi = ev.fun(&point);
        let grad = ev.grad(&point);
        let derphi = dot(&grad, pk);
        (phi, grad, derphi)
    };

    let mut search = match Dcsrch::start_with(phi0, derphi0, alpha, amax, c1, c2, xtol) {
        Ok(s) => s,
        Err(_) => {
            return LineSearch {
                stp: None,
                phi: phi0,
                phi0,
                grad: gfk.to_vec(),
            };
        }
    };
    let (mut phi, mut grad, mut derphi) = at(ev, alpha);
    for _ in 0..100 {
        match search.step(phi, derphi, &mut alpha, amin, amax) {
            SrchTask::Fg => {
                let next = at(ev, alpha);
                phi = next.0;
                grad = next.1;
                derphi = next.2;
            }
            SrchTask::Convergence => {
                return LineSearch {
                    stp: Some(alpha),
                    phi,
                    phi0,
                    grad,
                };
            }
            _ => {
                return LineSearch {
                    stp: None,
                    phi,
                    phi0,
                    grad,
                }
            }
        }
    }
    LineSearch {
        stp: None,
        phi,
        phi0,
        grad,
    }
}

/// `scipy.optimize.minimize(method="BFGS")` at the settings statsmodels passes: `gtol=1e-5`,
/// the infinity norm, `c1=1e-4` and `c2=0.9`.
pub fn fmin_bfgs<F, G>(f: F, g: G, x0: &[f64], gtol: f64, maxiter: usize) -> BfgsResult
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut ev = Counted {
        f: &f,
        g: &g,
        nfev: 0,
        njev: 0,
        evaluations: Vec::new(),
    };
    let xrtol = 0.0;
    let (c1, c2, amin, amax, xtol) = (1e-4, 0.9, 1e-100, 1e100, 1e-14);

    let mut old_fval = ev.fun(x0);
    let mut gfk = ev.grad(x0);
    let mut xk = x0.to_vec();
    // scipy's initial step guess, chosen so the first trial step is about one.
    let mut old_old_fval = Some(old_fval + norm2(&gfk) / 2.0);

    let mut hk = vec![0.0; n * n];
    for i in 0..n {
        hk[i * n + i] = 1.0;
    }

    let mut k = 0;
    let mut warnflag = 0u8;
    let mut line_search_failed = false;
    let mut gnorm = norm_inf(&gfk);

    while gnorm > gtol && k < maxiter {
        let pk: Vec<f64> = (0..n)
            .map(|i| -(0..n).map(|j| hk[i * n + j] * gfk[j]).sum::<f64>())
            .collect();
        let ls = line_search_wolfe1(
            &mut ev,
            &xk,
            &pk,
            &gfk,
            old_fval,
            old_old_fval,
            c1,
            c2,
            amin,
            amax,
            xtol,
        );
        let alpha = match ls.stp {
            Some(a) => a,
            None => {
                line_search_failed = true;
                warnflag = 2;
                break;
            }
        };
        old_fval = ls.phi;
        old_old_fval = Some(ls.phi0);

        let sk: Vec<f64> = pk.iter().map(|p| alpha * p).collect();
        for i in 0..n {
            xk[i] += sk[i];
        }
        let gfkp1 = ls.grad;
        let yk: Vec<f64> = gfkp1.iter().zip(&gfk).map(|(a, b)| a - b).collect();
        gfk = gfkp1;
        k += 1;

        gnorm = norm_inf(&gfk);
        if gnorm <= gtol {
            break;
        }
        if alpha * norm2(&pk) <= xrtol * (xrtol + norm2(&xk)) {
            break;
        }
        if !old_fval.is_finite() {
            warnflag = 2;
            break;
        }

        let rhok_inv = dot(&yk, &sk);
        let rhok = if rhok_inv == 0.0 {
            1000.0
        } else {
            1.0 / rhok_inv
        };

        // Hk <- A1 Hk A2' + rho s s', with A1 = I - rho s y' and A2 = I - rho y s'.
        let mut a1 = vec![0.0; n * n];
        let mut a2 = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                let kron = if i == j { 1.0 } else { 0.0 };
                a1[i * n + j] = kron - sk[i] * yk[j] * rhok;
                a2[i * n + j] = kron - yk[i] * sk[j] * rhok;
            }
        }
        let mut tmp = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                tmp[i * n + j] = (0..n).map(|m| hk[i * n + m] * a2[m * n + j]).sum();
            }
        }
        for i in 0..n {
            for j in 0..n {
                hk[i * n + j] = (0..n).map(|m| a1[i * n + m] * tmp[m * n + j]).sum::<f64>()
                    + rhok * sk[i] * sk[j];
            }
        }
    }

    if warnflag == 0 && k >= maxiter {
        warnflag = 1;
    }
    let Counted {
        nfev,
        njev,
        evaluations,
        ..
    } = ev;
    BfgsResult {
        x: xk,
        fun: old_fval,
        jac: gfk,
        nfev,
        njev,
        nit: k,
        warnflag,
        line_search_failed,
        evaluations,
    }
}
