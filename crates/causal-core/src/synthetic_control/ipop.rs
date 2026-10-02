//! kernlab 0.9-33 `R/ipop.R`, full square-H branch, specialised to Synth's
//! sum(w)=1 and 0<=w<=1 constraints. No active-set or other solver fallback.
use crate::lapack_lu::{dgetrf_fused, dgetrs_fused, reciprocal_condition_one_fused, Transpose};
use nalgebra::{DMatrix, DVector};

#[derive(Debug, PartialEq)]
pub enum IpopError {
    InvalidInput,
    SingularSystem,
    ComputationallySingularSystem { reciprocal_condition: f64 },
    NonFiniteIteration,
}
#[derive(Debug, PartialEq)]
pub enum IpopTermination {
    SignificantFigures,
    IterationLimit,
}
pub struct IpopFit {
    pub weights: Vec<f64>,
    pub dual: f64,
    pub iterations: usize,
    pub significant_figures: f64,
    pub termination: IpopTermination,
}

/// Diagnostic snapshots of the actual solver, not an alternative execution path.
pub struct IpopLinearSolve {
    pub matrix: Vec<f64>,
    pub rhs: Vec<f64>,
    pub answer: Vec<f64>,
}

fn solve(
    h: &DMatrix<f64>,
    diagonal: &[f64],
    e: f64,
    rhs: &[f64],
    trace: &mut Option<&mut Vec<IpopLinearSolve>>,
) -> Result<Vec<f64>, IpopError> {
    let n = h.nrows();
    let k = n + 1;
    let mut a = vec![0.; k * k];
    for c in 0..n {
        for r in 0..n {
            a[r + c * k] = -h[(r, c)];
        }
        a[c + c * k] -= diagonal[c];
        a[n + c * k] = 1.;
        a[c + n * k] = 1.;
    }
    a[n + n * k] = e;
    let snapshot = trace.as_ref().map(|_| a.clone());
    let norm = (0..k)
        .map(|c| (0..k).map(|r| a[r + c * k].abs()).sum::<f64>())
        .fold(0., f64::max);
    let mut pivots = vec![0; k];
    let info = dgetrf_fused(k, k, &mut a, k, &mut pivots).map_err(|_| IpopError::InvalidInput)?;
    if info != 0 {
        return Err(IpopError::SingularSystem);
    }
    let reciprocal_condition = reciprocal_condition_one_fused(k, &a, k, norm)
        .map_err(|_| IpopError::NonFiniteIteration)?;
    if reciprocal_condition < f64::EPSILON {
        return Err(IpopError::ComputationallySingularSystem {
            reciprocal_condition,
        });
    }
    let mut answer = rhs.to_vec();
    dgetrs_fused(Transpose::None, k, 1, &a, k, &pivots, &mut answer, k)
        .map_err(|_| IpopError::InvalidInput)?;
    if answer.iter().any(|x| !x.is_finite()) {
        return Err(IpopError::NonFiniteIteration);
    }
    if let (Some(records), Some(matrix)) = (trace.as_mut(), snapshot) {
        records.push(IpopLinearSolve {
            matrix,
            rhs: rhs.to_vec(),
            answer: answer.clone(),
        });
    }
    Ok(answer)
}

struct Direction {
    x: Vec<f64>,
    y: f64,
    g: Vec<f64>,
    t: Vec<f64>,
    z: Vec<f64>,
    s: Vec<f64>,
    w: f64,
    v: f64,
    p: f64,
    q: f64,
}
struct State {
    x: Vec<f64>,
    y: f64,
    g: Vec<f64>,
    t: Vec<f64>,
    z: Vec<f64>,
    s: Vec<f64>,
    w: f64,
    v: f64,
    p: f64,
    q: f64,
}
impl State {
    fn direction(
        &self,
        h: &DMatrix<f64>,
        sigma: &[f64],
        rho: f64,
        nu: &[f64],
        tau: &[f64],
        alpha: f64,
        beta: f64,
        gz: &[f64],
        gw: f64,
        gs: &[f64],
        gq: f64,
        d: &[f64],
        e: f64,
        trace: &mut Option<&mut Vec<IpopLinearSolve>>,
    ) -> Result<Direction, IpopError> {
        let n = self.x.len();
        let hb = beta - self.v * gw / self.w;
        let ha = alpha - self.p * gq / self.q;
        let hn: Vec<f64> = (0..n)
            .map(|i| nu[i] + self.g[i] * gz[i] / self.z[i])
            .collect();
        let ht: Vec<f64> = (0..n)
            .map(|i| tau[i] - self.t[i] * gs[i] / self.s[i])
            .collect();
        let mut rhs: Vec<f64> = (0..n)
            .map(|i| sigma[i] - self.z[i] * hn[i] / self.g[i] - self.s[i] * ht[i] / self.t[i])
            .collect();
        rhs.push(rho - e * (hb - self.q * ha / self.p));
        let answer = solve(h, d, e, &rhs, trace)?;
        let dx = answer[..n].to_vec();
        let dy = answer[n];
        let dw = -e * (hb - self.q * ha / self.p + dy);
        let ds: Vec<f64> = (0..n)
            .map(|i| self.s[i] * (dx[i] - ht[i]) / self.t[i])
            .collect();
        let dz: Vec<f64> = (0..n)
            .map(|i| self.z[i] * (hn[i] - dx[i]) / self.g[i])
            .collect();
        let dq = self.q * (dw - ha) / self.p;
        Ok(Direction {
            x: dx,
            y: dy,
            w: dw,
            s: ds.clone(),
            z: dz.clone(),
            q: dq,
            v: self.v * (gw - dw) / self.w,
            p: self.p * (gq - dq) / self.q,
            g: (0..n)
                .map(|i| self.g[i] * (gz[i] - dz[i]) / self.z[i])
                .collect(),
            t: (0..n)
                .map(|i| self.t[i] * (gs[i] - ds[i]) / self.s[i])
                .collect(),
        })
    }
    fn step(&self, d: &Direction, margin: f64) -> f64 {
        let mut minimum: f64 = -1.;
        for i in 0..self.x.len() {
            for value in [
                d.g[i] / self.g[i],
                d.t[i] / self.t[i],
                d.z[i] / self.z[i],
                d.s[i] / self.s[i],
            ] {
                minimum = minimum.min(value);
            }
        }
        for value in [d.w / self.w, d.p / self.p, d.v / self.v, d.q / self.q] {
            minimum = minimum.min(value);
        }
        -(1. - margin) / minimum
    }
    fn update(&mut self, d: &Direction, step: f64) {
        for i in 0..self.x.len() {
            self.x[i] += d.x[i] * step;
            self.g[i] += d.g[i] * step;
            self.t[i] += d.t[i] * step;
            self.z[i] += d.z[i] * step;
            self.s[i] += d.s[i] * step;
        }
        self.y += d.y * step;
        self.w += d.w * step;
        self.v += d.v * step;
        self.p += d.p * step;
        self.q += d.q * step;
    }
}

pub fn simplex_ipop(
    h: &DMatrix<f64>,
    c: &DVector<f64>,
    sigf: f64,
    maxiter: usize,
    margin: f64,
    bound: f64,
) -> Result<IpopFit, IpopError> {
    simplex_ipop_impl(h, c, sigf, maxiter, margin, bound, None)
}

pub fn simplex_ipop_trace(
    h: &DMatrix<f64>,
    c: &DVector<f64>,
    sigf: f64,
    maxiter: usize,
    margin: f64,
    bound: f64,
    trace: &mut Vec<IpopLinearSolve>,
) -> Result<IpopFit, IpopError> {
    simplex_ipop_impl(h, c, sigf, maxiter, margin, bound, Some(trace))
}

fn simplex_ipop_impl(
    h: &DMatrix<f64>,
    c: &DVector<f64>,
    sigf: f64,
    maxiter: usize,
    margin: f64,
    bound: f64,
    mut trace: Option<&mut Vec<IpopLinearSolve>>,
) -> Result<IpopFit, IpopError> {
    let n = c.len();
    if n < 2
        || h.nrows() != n
        || h.ncols() != n
        || maxiter == 0
        || !sigf.is_finite()
        || sigf <= 0.
        || !(0. < margin && margin < 1.)
        || !bound.is_finite()
        || bound <= 0.
        || h.iter().chain(c.iter()).any(|x| !x.is_finite())
    {
        return Err(IpopError::InvalidInput);
    }
    let mut rhs = c.as_slice().to_vec();
    rhs.push(1.);
    let start = solve(h, &vec![1.; n], 1., &rhs, &mut trace)?;
    let x = start[..n].to_vec();
    let y = start[n];
    let mut state = State {
        g: x.iter().map(|x| x.abs().max(bound)).collect(),
        z: x.iter().map(|x| x.abs().max(bound)).collect(),
        t: x.iter().map(|x| (1. - x).abs().max(bound)).collect(),
        s: x.iter().map(|x| x.abs().max(bound)).collect(),
        x,
        y,
        v: y.abs().max(bound),
        w: y.abs().max(bound),
        p: y.abs().max(bound),
        q: y.abs().max(bound),
    };
    let mut mu = (0..n).map(|i| state.z[i] * state.g[i]).sum::<f64>()
        + state.v * state.w
        + (0..n).map(|i| state.s[i] * state.t[i]).sum::<f64>()
        + state.p * state.q;
    mu /= 2. * (n + 1) as f64;
    let mut sigfig = 0.;
    for iteration in 1..=maxiter {
        // The pinned ARM OpenBLAS GEMV path accumulates with fused updates.
        // Preserve column order and the single rounding of each update.
        let hx: Vec<f64> = (0..n)
            .map(|row| (0..n).fold(0., |sum, col| h[(row, col)].mul_add(state.x[col], sum)))
            .collect();
        let rho = 1. - state.x.iter().sum::<f64>() + state.w;
        let nu: Vec<f64> = (0..n).map(|i| -state.x[i] + state.g[i]).collect();
        let tau: Vec<f64> = (0..n).map(|i| 1. - state.x[i] - state.t[i]).collect();
        let alpha = -state.w - state.p;
        let beta = state.y + state.q - state.v;
        let sigma: Vec<f64> = (0..n)
            .map(|i| c[i] - state.y - state.z[i] + state.s[i] + hx[i])
            .collect();
        let xhx = (0..n).fold(0., |sum, i| state.x[i].mul_add(hx[i], sum));
        let primal = (0..n).fold(0., |sum, i| c[i].mul_add(state.x[i], sum)) + 0.5 * xhx;
        let dual = state.y - 0.5 * xhx - state.s.iter().sum::<f64>();
        sigfig = (-((primal - dual).abs() / (primal.abs() + 1.)).log10()).max(0.);
        if sigfig >= sigf {
            return Ok(IpopFit {
                weights: state.x,
                dual: state.y,
                iterations: iteration,
                significant_figures: sigfig,
                termination: IpopTermination::SignificantFigures,
            });
        }
        let d: Vec<f64> = (0..n)
            .map(|i| state.z[i] / state.g[i] + state.s[i] / state.t[i])
            .collect();
        let e = 1. / (state.v / state.w + state.q / state.p);
        let gz: Vec<f64> = state.z.iter().map(|x| -x).collect();
        let gs: Vec<f64> = state.s.iter().map(|x| -x).collect();
        let first = state.direction(
            h, &sigma, rho, &nu, &tau, alpha, beta, &gz, -state.w, &gs, -state.q, &d, e, &mut trace,
        )?;
        let step = state.step(&first, margin);
        let newmu = mu * ((step - 1.) / (step + 10.)).powi(2);
        let gz: Vec<f64> = (0..n)
            .map(|i| mu / state.g[i] - state.z[i] - first.z[i] * first.g[i] / state.g[i])
            .collect();
        let gs: Vec<f64> = (0..n)
            .map(|i| mu / state.t[i] - state.s[i] - first.s[i] * first.t[i] / state.t[i])
            .collect();
        let gw = mu / state.v - state.w - first.w * first.v / state.v;
        let gq = mu / state.p - state.q - first.q * first.p / state.p;
        let second = state.direction(
            h, &sigma, rho, &nu, &tau, alpha, beta, &gz, gw, &gs, gq, &d, e, &mut trace,
        )?;
        let step = state.step(&second, margin);
        state.update(&second, step);
        mu = newmu;
        if state.x.iter().any(|x| !x.is_finite()) {
            return Err(IpopError::NonFiniteIteration);
        }
    }
    Ok(IpopFit {
        weights: state.x,
        dual: state.y,
        iterations: maxiter,
        significant_figures: sigfig,
        termination: IpopTermination::IterationLimit,
    })
}
