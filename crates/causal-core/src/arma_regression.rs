//! Regression with ARMA errors: statsmodels `SARIMAX(y, exog, order=(p, 0, q), trend='n')` at
//! its defaults, ported line by line. The error-model half of the dynamic harmonic regression of
//! Hyndman and Athanasopoulos (FPP3 §10.5): the regressors carry the mean, an ARMA(p, q)
//! process carries the serial dependence, and the whole is fitted by maximum likelihood.
//!
//!   y_t = x_t'beta + u_t,  phi(L) u_t = theta(L) e_t,  e_t ~ N(0, sigma2)
//!
//! Harvey's state space with k = max(p, q + 1) states: Z = [1 0 ... 0], T the companion matrix
//! with the AR coefficients in its first column, R = [1 theta_1 ... theta_{k-1}]', Q = sigma2,
//! the regression in the observation intercept. The state is stationary, so the filter starts
//! at the unconditional covariance (the discrete Lyapunov solution) and every observation
//! enters the likelihood. Parameters are ordered as statsmodels orders them:
//! `[beta (k_exog), ar (p), ma (q), sigma2]`.

use crate::linalg;
use nalgebra::{Complex, DMatrix, DVector};

const LN_2PI: f64 = 1.8378770664093453;
/// `MLEModel.fit` at `method='lbfgs'`: forward differences at this step on the average log
/// likelihood, `maxiter=50`, and scipy's `fmin_l_bfgs_b` defaults for the rest.
const OPTIMISER_EPSILON: f64 = 1e-5;
pub const DEFAULT_MAX_ITER: usize = 50;
/// `numpy.linalg.pinv` and `pinv_extended` cut singular values below this fraction of the largest.
const PINV_RCOND: f64 = 1e-15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArmaOrder {
    pub p: usize,
    pub q: usize,
}

impl ArmaOrder {
    /// The state dimension, `max(k_ar, k_ma + 1)`.
    pub fn states(self) -> usize {
        self.p.max(self.q + 1)
    }
}

fn norm_sf(x: f64) -> f64 {
    1.0 - spec_math::cephes64::ndtr(x)
}

/// `numpy.linalg.pinv(a).dot(b)`.
fn pinv_solve(a: &DMatrix<f64>, b: &DVector<f64>) -> DVector<f64> {
    let inverse = linalg::pseudo_inverse(a, PINV_RCOND).expect("pseudo-inverse of a finite matrix");
    &inverse.matrix * b
}

/// `statsmodels.tsa.tsatools.lagmat(x, maxlag)` with `trim='forward'`: row t holds
/// x[t-1], ..., x[t-maxlag], zero where the lag reaches before the sample.
fn lagmat_forward(x: &[f64], maxlag: usize) -> DMatrix<f64> {
    DMatrix::from_fn(x.len(), maxlag, |t, lag| if t > lag { x[t - lag - 1] } else { 0.0 })
}

/// The same with `trim='both'`: only the rows where every lag is inside the sample.
fn lagmat_both(x: &[f64], maxlag: usize) -> DMatrix<f64> {
    DMatrix::from_fn(x.len() - maxlag, maxlag, |r, lag| x[r + maxlag - lag - 1])
}

/// `companion_matrix(polynomial)` for a scalar polynomial `1 + c_1 L + ... + c_n L^n`: the
/// first column holds `-c_i`, the superdiagonal ones.
fn companion(polynomial: &[f64]) -> DMatrix<f64> {
    let n = polynomial.len() - 1;
    let mut m = DMatrix::<f64>::zeros(n, n);
    for i in 0..n {
        m[(i, 0)] = -polynomial[i + 1] / polynomial[0];
        if i + 1 < n {
            m[(i, i + 1)] = 1.0;
        }
    }
    m
}

/// `is_invertible(polynomial)`: every eigenvalue of the companion matrix inside the unit circle.
fn is_invertible(polynomial: &[f64]) -> bool {
    if polynomial.len() < 2 {
        return true;
    }
    let eigen = linalg::eigen_general_right(&companion(polynomial)).expect("eigenvalues of a finite companion matrix");
    let threshold = 1.0 - 1e-10;
    eigen
        .real_values
        .iter()
        .zip(eigen.imaginary_values.iter())
        .all(|(re, im)| (re * re + im * im).sqrt() < threshold)
}

/// `constrain_stationary_univariate`: partial autocorrelations in (-1, 1) from the real line,
/// then the Durbin-Levinson recursion to the AR coefficients (Monahan 1984).
pub fn constrain_stationary_univariate(unconstrained: &[f64]) -> Vec<f64> {
    let n = unconstrained.len();
    let mut y = vec![vec![0.0; n]; n];
    let r: Vec<f64> = unconstrained.iter().map(|u| u / (1.0 + u * u).sqrt()).collect();
    for k in 0..n {
        for i in 0..k {
            y[k][i] = y[k - 1][i] + r[k] * y[k - 1][k - i - 1];
        }
        y[k][k] = r[k];
    }
    if n == 0 {
        return vec![];
    }
    y[n - 1].iter().map(|v| -v).collect()
}

/// `unconstrain_stationary_univariate`: the inverse recursion.
pub fn unconstrain_stationary_univariate(constrained: &[f64]) -> Vec<f64> {
    let n = constrained.len();
    if n == 0 {
        return vec![];
    }
    let mut y = vec![vec![0.0; n]; n];
    for i in 0..n {
        y[n - 1][i] = -constrained[i];
    }
    for k in (1..n).rev() {
        for i in 0..k {
            y[k - 1][i] = (y[k][i] - y[k][k] * y[k][k - i - 1]) / (1.0 - y[k][k] * y[k][k]);
        }
    }
    (0..n).map(|i| y[i][i] / (1.0 - y[i][i] * y[i][i]).sqrt()).collect()
}

/// `SARIMAX.transform_params` for this model: the regression passes through, the AR
/// coefficients are constrained stationary, the MA coefficients invertible (the negated
/// constraint), and the variance is squared.
pub fn transform_params(unconstrained: &[f64], k_exog: usize, order: ArmaOrder) -> Vec<f64> {
    let mut constrained = unconstrained.to_vec();
    let ar = k_exog..k_exog + order.p;
    let ma = ar.end..ar.end + order.q;
    constrained[ar.clone()].copy_from_slice(&constrain_stationary_univariate(&unconstrained[ar.clone()]));
    let ma_constrained: Vec<f64> = constrain_stationary_univariate(&unconstrained[ma.clone()]).iter().map(|v| -v).collect();
    constrained[ma.clone()].copy_from_slice(&ma_constrained);
    let last = unconstrained.len() - 1;
    constrained[last] = unconstrained[last] * unconstrained[last];
    constrained
}

pub fn untransform_params(constrained: &[f64], k_exog: usize, order: ArmaOrder) -> Vec<f64> {
    let mut unconstrained = constrained.to_vec();
    let ar = k_exog..k_exog + order.p;
    let ma = ar.end..ar.end + order.q;
    unconstrained[ar.clone()].copy_from_slice(&unconstrain_stationary_univariate(&constrained[ar.clone()]));
    let negated: Vec<f64> = constrained[ma.clone()].iter().map(|v| -v).collect();
    unconstrained[ma.clone()].copy_from_slice(&unconstrain_stationary_univariate(&negated));
    let last = constrained.len() - 1;
    unconstrained[last] = constrained[last].sqrt();
    unconstrained
}

/// `SARIMAX.start_params`: the regression by pseudo-inverse, then conditional sum of squares
/// on its residuals for the ARMA coefficients (an AR(2q) prewhitening supplies the innovations
/// the MA terms regress on), the residual variance last. Non-stationary or non-invertible
/// starts are replaced by zeros, as the library does with a warning.
pub fn start_params(y: &[f64], x: &DMatrix<f64>, order: ArmaOrder) -> Vec<f64> {
    let yv = DVector::from_column_slice(y);
    let beta = pinv_solve(x, &yv);
    let endog: Vec<f64> = (&yv - x * &beta).iter().copied().collect();
    let ArmaOrder { p, q } = order;
    let k = 2 * q;
    let r = (k + q).max(p);
    // `_conditional_sum_squares` with no trend.
    let mut prewhitened: Option<Vec<f64>> = None;
    if q > 0 {
        let target = DVector::from_column_slice(&endog[k..]);
        let lags = lagmat_both(&endog, k);
        let params_ar = pinv_solve(&lags, &target);
        prewhitened = Some((&target - &lags * params_ar).iter().copied().collect());
    }
    let target = DVector::from_column_slice(&endog[r..]);
    let rows = target.len();
    let mut design = DMatrix::<f64>::zeros(rows, p + q);
    if p > 0 {
        let lags = lagmat_forward(&endog, p);
        for t in 0..rows {
            for lag in 0..p {
                design[(t, lag)] = lags[(r + t, lag)];
            }
        }
    }
    if let Some(innovations) = &prewhitened {
        let lags = lagmat_forward(innovations, q);
        for t in 0..rows {
            for lag in 0..q {
                design[(t, p + lag)] = lags[(r - k + t, lag)];
            }
        }
    }
    let (params, residuals): (Vec<f64>, Vec<f64>) = if p + q > 0 {
        let params = pinv_solve(&design, &target);
        let residuals = (&target - &design * &params).iter().copied().collect();
        (params.iter().copied().collect(), residuals)
    } else {
        (vec![], target.iter().copied().collect())
    };
    let mut params_ar = params[..p].to_vec();
    let mut params_ma = params[p..p + q].to_vec();
    let variance = if residuals.len() > 1.max(q) {
        residuals[q..].iter().map(|v| v * v).sum::<f64>() / (residuals.len() - q) as f64
    } else {
        let mean = endog.iter().sum::<f64>() / endog.len() as f64;
        endog.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / endog.len() as f64
    };
    let mut ar_polynomial = vec![1.0];
    ar_polynomial.extend(params_ar.iter().map(|v| -v));
    if p > 0 && !is_invertible(&ar_polynomial) {
        params_ar = vec![0.0; p];
    }
    let mut ma_polynomial = vec![1.0];
    ma_polynomial.extend(params_ma.iter().copied());
    if q > 0 && !is_invertible(&ma_polynomial) {
        params_ma = vec![0.0; q];
    }
    let mut start: Vec<f64> = beta.iter().copied().collect();
    start.extend(params_ar);
    start.extend(params_ma);
    start.push(variance.max(1e-10));
    start
}

/// The state-space matrices at a parameter vector: T and R Q R'.
struct StateSpace {
    transition: DMatrix<f64>,
    state_cov: DMatrix<f64>,
}

fn state_space(params: &[f64], k_exog: usize, order: ArmaOrder) -> StateSpace {
    let m = order.states();
    let ar = &params[k_exog..k_exog + order.p];
    let ma = &params[k_exog + order.p..k_exog + order.p + order.q];
    let sigma2 = params[params.len() - 1];
    let mut transition = DMatrix::<f64>::zeros(m, m);
    for (i, phi) in ar.iter().enumerate() {
        transition[(i, 0)] = *phi;
    }
    for i in 0..m - 1 {
        transition[(i, i + 1)] = 1.0;
    }
    let mut selection = DVector::<f64>::zeros(m);
    selection[0] = 1.0;
    for (i, theta) in ma.iter().enumerate() {
        selection[i + 1] = *theta;
    }
    let state_cov = &selection * selection.transpose() * sigma2;
    StateSpace { transition, state_cov }
}

/// The stationary initial covariance: P = T P T' + R Q R', solved through its vectorised
/// form `(I - T (x) T) vec(P) = vec(R Q R')`. statsmodels reaches the same fixed point by a
/// bilinear transform and a Sylvester solve; the equation has one solution when T is stable.
pub fn stationary_covariance(transition: &DMatrix<f64>, state_cov: &DMatrix<f64>) -> DMatrix<f64> {
    let m = transition.nrows();
    let n = m * m;
    let kron = transition.kronecker(transition);
    let system = DMatrix::<f64>::identity(n, n) - kron;
    let right = DMatrix::from_column_slice(n, 1, state_cov.as_slice());
    let solution = linalg::solve(&system, &right).expect("stationary covariance of a stable transition");
    DMatrix::from_column_slice(m, m, solution.as_slice())
}

pub struct FilterOutput {
    pub llf: f64,
    pub loglikeobs: Vec<f64>,
    /// One-step-ahead forecast errors, statsmodels' `resid`.
    pub forecasts_error: Vec<f64>,
    pub forecasts_error_cov: Vec<f64>,
}

/// statsmodels' `tolerance` for the filter's steady state: once the predicted covariance stops
/// changing, F and the gain are frozen from that period, as the Cython filter freezes them.
const CONVERGENCE_TOL: f64 = 1e-19;

/// The conventional Kalman filter for the regression with ARMA errors, the predicted state
/// covariance kept symmetric as statsmodels' `STABILITY_FORCE_SYMMETRY` does and frozen at
/// the steady state, as the transpile's VARMAX port does.
pub fn kalman_filter(params: &[f64], y: &[f64], x: &DMatrix<f64>, order: ArmaOrder) -> FilterOutput {
    let k_exog = x.ncols();
    let beta = DVector::from_column_slice(&params[..k_exog]);
    let intercept = x * beta;
    let ss = state_space(params, k_exog, order);
    let m = order.states();
    let mut a = DVector::<f64>::zeros(m);
    let mut p = stationary_covariance(&ss.transition, &ss.state_cov);
    let n = y.len();
    let mut lls = Vec::with_capacity(n);
    let mut errors = Vec::with_capacity(n);
    let mut error_cov = Vec::with_capacity(n);
    let mut converged = false;
    let mut frozen: Option<(f64, DVector<f64>)> = None;
    for t in 0..n {
        let (f, gain) = match (&frozen, converged) {
            (Some((f, gain)), true) => (*f, gain.clone()),
            // M = P Z' is the first column of P; F = Z P Z' its first entry.
            _ => (p[(0, 0)], p.column(0).clone_owned()),
        };
        let v = y[t] - intercept[t] - a[0];
        lls.push(-0.5 * (LN_2PI + f.ln() + v * v / f));
        errors.push(v);
        error_cov.push(f);
        let a_filtered = &a + &gain * (v / f);
        a = &ss.transition * a_filtered;
        if !converged {
            let p_filtered = &p - &gain * gain.transpose() / f;
            let predicted = &ss.transition * p_filtered * ss.transition.transpose() + &ss.state_cov;
            let p_next = (&predicted + predicted.transpose()) * 0.5;
            // The double-precision filter measures the change as ddot(diff, diff) and, at
            // convergence, keeps the current period's F and gain for every later period.
            let change: f64 = (&p_next - &p).iter().map(|value| value * value).sum();
            if t > 0 && change < CONVERGENCE_TOL {
                converged = true;
                frozen = Some((f, gain));
            }
            p = p_next;
        }
    }
    FilterOutput { llf: lls.iter().sum(), loglikeobs: lls, forecasts_error: errors, forecasts_error_cov: error_cov }
}

pub fn loglike(params: &[f64], y: &[f64], x: &DMatrix<f64>, order: ArmaOrder) -> f64 {
    kalman_filter(params, y, x, order).llf
}

pub struct ArmaRegressionFit {
    pub order: ArmaOrder,
    pub k_exog: usize,
    /// `[beta, ar, ma, sigma2]`.
    pub params: Vec<f64>,
    /// Outer-product-of-gradients standard errors, `cov_type='opg'`.
    pub bse: Vec<f64>,
    pub pvalues: Vec<f64>,
    /// 95% normal-based intervals as (lower, upper).
    pub conf_int: Vec<(f64, f64)>,
    pub llf: f64,
    pub llf_obs: Vec<f64>,
    pub resid: Vec<f64>,
    pub forecasts_error_cov: Vec<f64>,
    /// `resid / sqrt(F)`, the residuals the serial-correlation tests read.
    pub standardized_resid: Vec<f64>,
    pub aic: f64,
    pub bic: f64,
    pub iterations: usize,
    pub converged: bool,
}

/// The stationary covariance for complex-step parameters: the same vectorised equation with
/// complex coefficients, solved as one real system twice the size through the same LU path,
/// so the derivative passes through the initialisation as statsmodels' `complex_step=True` does.
fn stationary_covariance_complex(transition: &DMatrix<Complex<f64>>, state_cov: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
    let m = transition.nrows();
    let n = m * m;
    let kron = transition.kronecker(transition);
    let system = DMatrix::<Complex<f64>>::identity(n, n) - kron;
    // [A_re -A_im; A_im A_re] [p_re; p_im] = [b_re; b_im].
    let real_system = DMatrix::from_fn(2 * n, 2 * n, |r, c| {
        let value = system[(r % n, c % n)];
        match (r < n, c < n) {
            (true, true) | (false, false) => value.re,
            (true, false) => -value.im,
            (false, true) => value.im,
        }
    });
    let right = DMatrix::from_fn(2 * n, 1, |r, _| if r < n { state_cov.as_slice()[r].re } else { state_cov.as_slice()[r - n].im });
    let solution = linalg::solve(&real_system, &right).expect("stationary covariance of a stable transition");
    DMatrix::from_fn(m, m, |i, j| Complex::new(solution[(j * m + i, 0)], solution[(n + j * m + i, 0)]))
}

/// The filter's per-observation log likelihood at complex parameters, for the complex-step
/// derivative: the arithmetic of `kalman_filter` over `Complex<f64>`, with statsmodels'
/// non-conjugating transposes so the imaginary part carries the derivative.
fn loglikeobs_complex(params: &[Complex<f64>], y: &[f64], x: &DMatrix<f64>, order: ArmaOrder) -> Vec<Complex<f64>> {
    let k_exog = x.ncols();
    let m = order.states();
    let intercept: Vec<Complex<f64>> = (0..y.len())
        .map(|t| (0..k_exog).map(|c| params[c] * x[(t, c)]).sum())
        .collect();
    let ar = &params[k_exog..k_exog + order.p];
    let ma = &params[k_exog + order.p..k_exog + order.p + order.q];
    let sigma2 = params[params.len() - 1];
    let mut transition = DMatrix::<Complex<f64>>::zeros(m, m);
    for (i, phi) in ar.iter().enumerate() {
        transition[(i, 0)] = *phi;
    }
    for i in 0..m - 1 {
        transition[(i, i + 1)] = Complex::new(1.0, 0.0);
    }
    let mut selection = DVector::<Complex<f64>>::zeros(m);
    selection[0] = Complex::new(1.0, 0.0);
    for (i, theta) in ma.iter().enumerate() {
        selection[i + 1] = *theta;
    }
    let state_cov = &selection * selection.transpose() * sigma2;
    let mut a = DVector::<Complex<f64>>::zeros(m);
    let mut p = stationary_covariance_complex(&transition, &state_cov);
    let mut lls = Vec::with_capacity(y.len());
    let mut converged = false;
    let mut frozen: Option<(Complex<f64>, DVector<Complex<f64>>)> = None;
    for t in 0..y.len() {
        let (f, gain) = match (&frozen, converged) {
            (Some((f, gain)), true) => (*f, gain.clone()),
            _ => (p[(0, 0)], p.column(0).clone_owned()),
        };
        let v = Complex::new(y[t], 0.0) - intercept[t] - a[0];
        lls.push((f.ln() + v * v / f + LN_2PI) * -0.5);
        let a_filtered = &a + &gain * (v / f);
        a = &transition * a_filtered;
        if !converged {
            let p_filtered = &p - &gain * gain.transpose() / f;
            let predicted = &transition * p_filtered * transition.transpose() + &state_cov;
            let p_next = (&predicted + predicted.transpose()) * Complex::new(0.5, 0.0);
            // statsmodels checks convergence on the real filter; under a complex step the
            // squared change is taken on the real parts, which is what the imaginary
            // perturbation leaves unchanged.
            let change: f64 = (&p_next - &p).iter().map(|value| value.re * value.re).sum();
            if t > 0 && change < CONVERGENCE_TOL {
                converged = true;
                frozen = Some((f, gain));
            }
            p = p_next;
        }
    }
    lls
}

/// `score_obs(params)` by complex step, `approx_fprime_cs(params, loglikeobs)` at the step
/// `sqrt(EPS) max(|x|, 0.1)`: exact to rounding, since the step introduces no subtraction.
pub fn score_obs(params: &[f64], y: &[f64], x: &DMatrix<f64>, order: ArmaOrder) -> DMatrix<f64> {
    let n = y.len();
    let k = params.len();
    let mut scores = DMatrix::<f64>::zeros(n, k);
    for j in 0..k {
        let h = f64::EPSILON.sqrt() * params[j].abs().max(0.1);
        let mut stepped: Vec<Complex<f64>> = params.iter().map(|v| Complex::new(*v, 0.0)).collect();
        stepped[j] = Complex::new(params[j], h);
        let lls = loglikeobs_complex(&stepped, y, x, order);
        for t in 0..n {
            scores[(t, j)] = lls[t].im / h;
        }
    }
    scores
}

/// Maximum likelihood by L-BFGS-B on the unconstrained parameters, then the OPG covariance,
/// the filter output at the optimum and the information criteria.
pub fn fit(y: &[f64], x: &DMatrix<f64>, order: ArmaOrder, max_iter: usize) -> ArmaRegressionFit {
    assert!(order.p + order.q > 0, "an ARMA(0, 0) error is ordinary least squares");
    let k_exog = x.ncols();
    let n = y.len();
    let start = start_params(y, x, order);
    let u0 = untransform_params(&start, k_exog, order);
    let k = u0.len();
    let objective = |u: &[f64]| -loglike(&transform_params(u, k_exog, order), y, x, order) / n as f64;
    let res = crate::lbfgsb::lbfgsb(
        &u0,
        &vec![f64::NEG_INFINITY; k],
        &vec![f64::INFINITY; k],
        &vec![0i32; k],
        10,
        1e7,
        1e-5,
        20,
        max_iter,
        |u| {
            let f0 = objective(u);
            let mut grad = vec![0.0; k];
            for i in 0..k {
                let mut uh = u.to_vec();
                uh[i] += OPTIMISER_EPSILON;
                let dx = uh[i] - u[i];
                grad[i] = (objective(&uh) - f0) / dx;
            }
            (f0, grad)
        },
    );
    let params = transform_params(&res.x, k_exog, order);
    let filtered = kalman_filter(&params, y, x, order);
    // cov_params_opg: the pseudo-inverse of the summed outer products of the per-observation
    // scores (`nobs * opg_information_matrix`).
    let scores = score_obs(&params, y, x, order);
    let information = scores.transpose() * &scores;
    let cov = linalg::pseudo_inverse(&information, PINV_RCOND).expect("pseudo-inverse of the information matrix").matrix;
    let bse: Vec<f64> = (0..k).map(|j| cov[(j, j)].sqrt()).collect();
    let pvalues: Vec<f64> = (0..k).map(|j| 2.0 * norm_sf((params[j] / bse[j]).abs())).collect();
    let z975 = spec_math::cephes64::ndtri(0.975);
    let conf_int: Vec<(f64, f64)> = (0..k).map(|j| (params[j] - z975 * bse[j], params[j] + z975 * bse[j])).collect();
    let standardized_resid: Vec<f64> = filtered
        .forecasts_error
        .iter()
        .zip(&filtered.forecasts_error_cov)
        .map(|(v, f)| v / f.sqrt())
        .collect();
    let df_model = k as f64;
    ArmaRegressionFit {
        order,
        k_exog,
        aic: -2.0 * filtered.llf + 2.0 * df_model,
        bic: -2.0 * filtered.llf + df_model * (n as f64).ln(),
        params,
        bse,
        pvalues,
        conf_int,
        llf: filtered.llf,
        llf_obs: filtered.loglikeobs,
        resid: filtered.forecasts_error,
        forecasts_error_cov: filtered.forecasts_error_cov,
        standardized_resid,
        iterations: res.iterations,
        converged: res.termination.converged(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stationarity_transform_round_trips() {
        let u = [0.3, -1.2, 0.7];
        let c = constrain_stationary_univariate(&u);
        let back = unconstrain_stationary_univariate(&c);
        for (a, b) in u.iter().zip(&back) {
            assert!((a - b).abs() < 1e-12);
        }
        let mut polynomial = vec![1.0];
        polynomial.extend(c.iter().map(|v| -v));
        assert!(is_invertible(&polynomial));
    }

    #[test]
    fn companion_holds_the_negated_coefficients() {
        let m = companion(&[1.0, -0.5, 0.25]);
        assert_eq!(m[(0, 0)], 0.5);
        assert_eq!(m[(1, 0)], -0.25);
        assert_eq!(m[(0, 1)], 1.0);
    }
}
