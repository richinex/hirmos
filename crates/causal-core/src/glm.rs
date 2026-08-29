//! The two count estimators of 805 step 3f: `sm.GLM(family=Poisson()).fit()` by iteratively
//! reweighted least squares, and `NegativeBinomialP(p=2).fit()` by BFGS.

use nalgebra::{DMatrix, DVector};

const FLOAT_EPS: f64 = f64::EPSILON;

fn norm_cdf(x: f64) -> f64 {
    spec_math::cephes64::ndtr(x)
}

fn digamma(x: f64) -> f64 {
    spec_math::cephes64::psi(x)
}

/// `scipy.special.polygamma(1, x)`, which is the Hurwitz zeta at two.
fn trigamma(x: f64) -> f64 {
    spec_math::cephes64::zeta(2.0, x)
}

fn lgamma(x: f64) -> f64 {
    spec_math::cephes64::lgam(x)
}

/// Weighted least squares through the normal equations, returning the parameters and
/// `(X' W X)^-1`.
fn wls(x: &DMatrix<f64>, y: &DVector<f64>, w: &[f64]) -> (DVector<f64>, DMatrix<f64>) {
    let k = x.ncols();
    let mut xtwx = DMatrix::<f64>::zeros(k, k);
    let mut xtwy = DVector::<f64>::zeros(k);
    for r in 0..x.nrows() {
        for i in 0..k {
            xtwy[i] += w[r] * x[(r, i)] * y[r];
            for j in 0..k {
                xtwx[(i, j)] += w[r] * x[(r, i)] * x[(r, j)];
            }
        }
    }
    let inv = xtwx
        .clone()
        .try_inverse()
        .expect("weighted design is singular");
    (&inv * xtwy, inv)
}

pub struct PoissonGlm {
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub tvalues: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// `fittedvalues`, which for a GLM is the mean rather than the linear predictor.
    pub fitted: Vec<f64>,
    pub deviance: f64,
    pub df_resid: f64,
    pub scale: f64,
    pub converged: bool,
    pub iterations: usize,
}

/// Poisson deviance, with `endog / mu` clipped away from zero as statsmodels' `_clean` does.
fn poisson_deviance(y: &[f64], mu: &[f64]) -> f64 {
    y.iter()
        .zip(mu)
        .map(|(&yi, &m)| {
            let ratio = (yi / m).max(FLOAT_EPS);
            2.0 * (yi * ratio.ln() - (yi - m))
        })
        .sum()
}

/// `sm.GLM(y, x, family=sm.families.Poisson()).fit()`.
pub fn poisson_glm(y: &[f64], x: &DMatrix<f64>) -> PoissonGlm {
    let n = y.len();
    let mean = y.iter().sum::<f64>() / n as f64;
    // starting_mu is (y + ybar) / 2.
    let mut mu: Vec<f64> = y.iter().map(|v| (v + mean) / 2.0).collect();
    let mut lin_pred: Vec<f64> = mu.iter().map(|m| m.ln()).collect();
    let mut dev = poisson_deviance(y, &mu);

    let (atol, maxiter) = (1e-8, 100);
    let mut converged = false;
    let mut iterations = 0;
    let mut weights = vec![0.0; n];
    let mut wlsendog = DVector::<f64>::zeros(n);
    let mut params = DVector::<f64>::zeros(x.ncols());

    for it in 0..maxiter {
        iterations = it + 1;
        // For the log link the IRLS weight is 1 / (g'(mu)^2 Var(mu)) = mu.
        for i in 0..n {
            weights[i] = mu[i];
            wlsendog[i] = lin_pred[i] + (y[i] - mu[i]) / mu[i];
        }
        params = wls(x, &wlsendog, &weights).0;
        for i in 0..n {
            lin_pred[i] = (0..x.ncols()).map(|j| x[(i, j)] * params[j]).sum();
            mu[i] = lin_pred[i].exp();
        }
        let new_dev = poisson_deviance(y, &mu);
        converged = (new_dev - dev).abs() <= atol;
        dev = new_dev;
        if converged {
            break;
        }
    }

    // statsmodels refits the final weighted problem to get the covariance.
    let (_, xtwx_inv) = wls(x, &wlsendog, &weights);
    let scale = 1.0; // Poisson carries no dispersion parameter.
    let k = x.ncols();
    let bse: Vec<f64> = (0..k).map(|i| (scale * xtwx_inv[(i, i)]).sqrt()).collect();
    let tvalues: Vec<f64> = (0..k).map(|i| params[i] / bse[i]).collect();
    let pvalues = tvalues
        .iter()
        .map(|t| 2.0 * (1.0 - norm_cdf(t.abs())))
        .collect();

    PoissonGlm {
        params: params.iter().copied().collect(),
        bse,
        tvalues,
        pvalues,
        fitted: mu,
        deviance: dev,
        df_resid: (n - k) as f64,
        scale,
        converged,
        iterations,
    }
}

/// `Poisson(y, x).fit()`, the discrete model's Newton fit. Used for the starting values of
/// the negative binomial, where statsmodels runs it as a preliminary step.
pub fn poisson_mle(y: &[f64], x: &DMatrix<f64>) -> Vec<f64> {
    let n = y.len();
    let k = x.ncols();
    let mean = y.iter().sum::<f64>() / n as f64;
    // statsmodels seeds the constant with the null model and the rest with 0.001.
    let mut params = DVector::from_fn(k, |i, _| if i == 0 { mean.ln() } else { 0.001 });

    for _ in 0..100 {
        let mut score = DVector::<f64>::zeros(k);
        let mut hess = DMatrix::<f64>::zeros(k, k);
        for r in 0..n {
            let eta: f64 = (0..k).map(|j| x[(r, j)] * params[j]).sum();
            let mu = eta.exp();
            for i in 0..k {
                score[i] += x[(r, i)] * (y[r] - mu);
                for j in 0..k {
                    hess[(i, j)] -= mu * x[(r, i)] * x[(r, j)];
                }
            }
        }
        let step = hess
            .clone()
            .try_inverse()
            .expect("Poisson Hessian is singular")
            * score;
        let moved = step.amax();
        params -= step;
        if moved <= 1e-12 {
            break;
        }
    }
    params.iter().copied().collect()
}

pub struct NegativeBinomialP {
    /// The regression coefficients followed by alpha.
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub tvalues: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// `fittedvalues`, which for a count model is the linear predictor, not the mean.
    pub fitted: Vec<f64>,
    pub llf: f64,
    pub start_params: Vec<f64>,
    pub converged: bool,
    pub nfev: usize,
    pub njev: usize,
    pub nit: usize,
    pub line_search_failed: bool,
    /// Ordered objective and gradient calls made by scipy-compatible BFGS.
    pub evaluations: Vec<crate::bfgs::BfgsEvaluation>,
}

struct NbP<'a> {
    y: &'a [f64],
    x: &'a DMatrix<f64>,
    /// The `2 - parameterization` exponent the score and Hessian use.
    p: f64,
}

impl NbP<'_> {
    fn mu(&self, beta: &[f64]) -> Vec<f64> {
        (0..self.y.len())
            .map(|r| {
                let eta: f64 = (0..self.x.ncols()).map(|j| self.x[(r, j)] * beta[j]).sum();
                eta.exp()
            })
            .collect()
    }

    fn loglike(&self, params: &[f64]) -> f64 {
        let k = params.len() - 1;
        let alpha = params[k];
        let mu = self.mu(&params[..k]);
        let mut total = 0.0;
        for (r, &m) in mu.iter().enumerate() {
            let y = self.y[r];
            let a1 = m.powf(self.p) / alpha;
            let a2 = m + a1;
            total += lgamma(y + a1) - lgamma(y + 1.0) - lgamma(a1) + a1 * a1.ln() + y * m.ln()
                - (y + a1) * a2.ln();
        }
        total
    }

    fn score(&self, params: &[f64]) -> Vec<f64> {
        let k = params.len() - 1;
        let alpha = params[k];
        let mu = self.mu(&params[..k]);
        let mut out = vec![0.0; k + 1];
        for (r, &m) in mu.iter().enumerate() {
            let y = self.y[r];
            let a1 = m.powf(self.p) / alpha;
            let a2 = m + a1;
            let a3 = y + a1;
            let a4 = self.p * a1 / m;
            let dgpart = digamma(a3) - digamma(a1);
            let dgterm = dgpart + (a1 / a2).ln() + 1.0 - a3 / a2;
            let dparams = a4 * dgterm - a3 / a2 + y / m;
            for j in 0..k {
                out[j] += self.x[(r, j)] * m * dparams;
            }
            out[k] += -a1 / alpha * dgterm;
        }
        out
    }

    fn hessian(&self, params: &[f64]) -> DMatrix<f64> {
        let k = params.len() - 1;
        let alpha = params[k];
        let mu = self.mu(&params[..k]);
        let p = self.p;
        let mut hess = DMatrix::<f64>::zeros(k + 1, k + 1);
        for (r, &m) in mu.iter().enumerate() {
            let y = self.y[r];
            let a1 = m.powf(p) / alpha;
            let a2 = m + a1;
            let a3 = y + a1;
            let a4 = p * a1 / m;
            let prob = a1 / a2;
            let lprob = prob.ln();
            let dgpart = digamma(a3) - digamma(a1);
            let pgpart = trigamma(a3) - trigamma(a1);

            let coeff = m
                * m
                * ((1.0 + a4).powi(2) * a3 / (a2 * a2)
                    - a3 / a2 * (p - 1.0) * a4 / m
                    - y / (m * m)
                    - 2.0 * a4 * (1.0 + a4) / a2
                    + p * a4 / m * (lprob + dgpart + 2.0)
                    - a4 / m * (lprob + dgpart + 1.0)
                    + a4 * a4 * pgpart
                    + (-(1.0 + a4) * a3 / a2 + y / m + a4 * (lprob + dgpart + 1.0)) / m);
            for i in 0..k {
                for j in 0..k {
                    hess[(i, j)] += self.x[(r, i)] * self.x[(r, j)] * coeff;
                }
            }
            let cross = m
                * a1
                * ((1.0 + a4) * (1.0 - a3 / a2) / a2 - p * (lprob + dgpart + 2.0) / m
                    + p / m * (a3 + p * a1) / a2
                    - a4 * pgpart)
                / alpha;
            for i in 0..k {
                hess[(k, i)] += self.x[(r, i)] * cross;
            }
            hess[(k, k)] += a1
                * (2.0 * lprob + 2.0 * dgpart + 3.0 - 2.0 * a3 / a2 + a1 * pgpart - 2.0 * prob
                    + prob * a3 / a2)
                / (alpha * alpha);
        }
        // statsmodels mirrors the lower triangle into the upper one.
        for i in 0..k + 1 {
            for j in i + 1..k + 1 {
                hess[(i, j)] = hess[(j, i)];
            }
        }
        hess
    }
}

/// `NegativeBinomialP(y, x, p=parameterization).fit(disp=0)`.
pub fn negative_binomial_p(
    y: &[f64],
    x: &DMatrix<f64>,
    parameterization: f64,
) -> NegativeBinomialP {
    let n = y.len();
    let k = x.ncols();
    let model = NbP {
        y,
        x,
        p: 2.0 - parameterization,
    };

    // Preliminary Poisson fit, then a moment estimate of the dispersion.
    let beta0 = poisson_mle(y, x);
    let mu0 = model.mu(&beta0);
    let df_resid = (n - k) as f64;
    // NegativeBinomialP overrides the base dispersion estimate: squared residuals, and the
    // exponent is one below the parameterization.
    let q = parameterization - 1.0;
    let a: f64 = (0..n)
        .map(|r| {
            let resid = y[r] - mu0[r];
            (resid * resid / mu0[r] - 1.0) * mu0[r].powf(-q)
        })
        .sum::<f64>()
        / df_resid;
    let mut start_params = beta0.clone();
    start_params.push(a.max(0.05));

    // statsmodels minimises the average negative log-likelihood.
    let nobs = n as f64;
    let res = crate::bfgs::fmin_bfgs(
        |p| -model.loglike(p) / nobs,
        |p| model.score(p).iter().map(|v| -v / nobs).collect(),
        &start_params,
        1e-5,
        35,
    );

    let params = res.x.clone();
    let cov = (-model.hessian(&params))
        .try_inverse()
        .expect("negative binomial Hessian is singular");
    let bse: Vec<f64> = (0..k + 1).map(|i| cov[(i, i)].sqrt()).collect();
    let tvalues: Vec<f64> = (0..k + 1).map(|i| params[i] / bse[i]).collect();
    let pvalues = tvalues
        .iter()
        .map(|t| 2.0 * (1.0 - norm_cdf(t.abs())))
        .collect();
    let fitted = (0..n)
        .map(|r| (0..k).map(|j| x[(r, j)] * params[j]).sum())
        .collect();

    NegativeBinomialP {
        llf: model.loglike(&params),
        params,
        bse,
        tvalues,
        pvalues,
        fitted,
        start_params,
        converged: res.warnflag == 0,
        nfev: res.nfev,
        njev: res.njev,
        nit: res.nit,
        line_search_failed: res.line_search_failed,
        evaluations: res.evaluations,
    }
}
