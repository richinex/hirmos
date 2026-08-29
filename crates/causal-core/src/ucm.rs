//! statsmodels' UnobservedComponents('lltrend', use_exact_diffuse=True) ported for the
//! Bayesian structural time series lane: the Hodrick-Prescott start parameters, the local
//! linear trend state space, and the exact diffuse Kalman filter of Durbin and Koopman.
//!
//! The model is
//!   y_t   = mu_t + eps_t,           eps_t  ~ N(0, sigma2_irregular)
//!   mu_t  = mu_{t-1} + beta_{t-1} + zeta_t,  zeta_t ~ N(0, sigma2_level)
//!   beta_t = beta_{t-1} + xi_t,     xi_t   ~ N(0, sigma2_trend)
//! so Z = [1 0], T = [[1 1], [0 1]], R = I, Q = diag(level, trend), H = irregular.

use nalgebra::DMatrix;

/// Parameter order, matching statsmodels' `param_names`.
pub const IRREGULAR: usize = 0;
pub const LEVEL: usize = 1;
pub const TREND: usize = 2;

/// statsmodels.tsa.filters.hp_filter.hpfilter: returns (cycle, trend) where
/// trend solves (I + lamb K'K) t = x for the second-difference matrix K.
pub fn hpfilter(x: &[f64], lamb: f64) -> (Vec<f64>, Vec<f64>) {
    let n = x.len();
    if n < 3 {
        return (vec![0.0; n], x.to_vec());
    }
    // A = I + lamb K'K is symmetric pentadiagonal; store its five bands.
    let mut a = DMatrix::<f64>::zeros(n, n);
    for r in 0..n {
        a[(r, r)] = 1.0;
    }
    // K has rows [1, -2, 1] starting at column i, for i in 0..n-2.
    for i in 0..n - 2 {
        let cols = [i, i + 1, i + 2];
        let vals = [1.0, -2.0, 1.0];
        for (p, &cp) in cols.iter().enumerate() {
            for (q, &cq) in cols.iter().enumerate() {
                a[(cp, cq)] += lamb * vals[p] * vals[q];
            }
        }
    }
    let rhs = DMatrix::from_column_slice(n, 1, x);
    let trend_col = a
        .cholesky()
        .expect("hp filter system is positive definite")
        .solve(&rhs);
    let trend: Vec<f64> = (0..n).map(|i| trend_col[(i, 0)]).collect();
    let cycle: Vec<f64> = x.iter().zip(&trend).map(|(v, t)| v - t).collect();
    (cycle, trend)
}

fn variance(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let mean = x.iter().sum::<f64>() / n;
    x.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n
}

/// UnobservedComponents.start_params for the local linear trend: nested HP filters supply
/// the trend and level variances, the first filter's cycle supplies the irregular variance.
pub fn start_params(y: &[f64]) -> [f64; 3] {
    let (resid, trend1) = hpfilter(y, 1600.0);
    let (cycle2, trend2) = hpfilter(&trend1, 1600.0);
    let mut params = [0.0; 3];
    params[TREND] = variance(&trend2);
    params[LEVEL] = variance(&cycle2);
    params[IRREGULAR] = variance(&resid);
    params
}

pub struct FilterOutput {
    pub llf: f64,
    /// Per observation log likelihood contributions.
    pub loglikeobs: Vec<f64>,
    pub forecasts_error: Vec<f64>,
    pub forecasts_error_cov: Vec<f64>,
    /// Filtered state a_{t|t}, shape 2 x nobs.
    pub filtered_state: Vec<[f64; 2]>,
    /// Number of periods the diffuse recursion ran for.
    pub nobs_diffuse: usize,
}

const LN_2PI: f64 = 1.8378770664093453;

/// The exact diffuse Kalman filter for the local linear trend. The first periods use the
/// Koopman diffuse recursions on (P_infinity, P_star); once the diffuse part is exhausted
/// the filter reverts to the standard recursions.
pub fn kalman_filter(params: &[f64], y: &[f64], diffuse_tol: f64) -> FilterOutput {
    // statsmodels hardcodes tolerance_diffuse = 1e-10.
    let n = y.len();
    let (h, q_level, q_trend) = (params[IRREGULAR], params[LEVEL], params[TREND]);
    // T = [[1,1],[0,1]], Z = [1,0], R = I, Q = diag(level, trend).
    let t_mat = [[1.0, 1.0], [0.0, 1.0]];
    let q = [[q_level, 0.0], [0.0, q_trend]];

    let mut a = [0.0f64; 2];
    let mut p_star = [[0.0f64; 2]; 2];
    let mut p_inf = [[1.0, 0.0], [0.0, 1.0]];

    let mut lls = Vec::with_capacity(n);
    let mut errors = Vec::with_capacity(n);
    let mut error_cov = Vec::with_capacity(n);
    let mut filtered = Vec::with_capacity(n);
    let mut nobs_diffuse = 0usize;

    for &obs in y.iter().take(n) {
        let v = obs - a[0];
        // Z P Z' picks the (0,0) entry.
        // statsmodels ends the diffuse phase when the predicted diffuse covariance is
        // numerically zero, measured by its squared Frobenius norm, not by F_infinity.
        let p_inf_norm2: f64 = p_inf.iter().flatten().map(|v| v * v).sum();
        let diffuse = p_inf_norm2 > diffuse_tol;
        let f_inf = p_inf[0][0].max(0.0);
        let f_star = (p_star[0][0] + h).max(0.0);
        // M = P Z' is the first column of P.
        let m_inf = [p_inf[0][0], p_inf[1][0]];
        let m_star = [p_star[0][0], p_star[1][0]];

        let (a_upd, p_star_upd, p_inf_upd, ll) = if diffuse && f_inf > 1e-10 {
            nobs_diffuse += 1;
            // Koopman's F_infinity > 0 branch.
            let f1 = 1.0 / f_inf;
            let f2 = -f_star * f1 * f1;
            let mut a_new = a;
            let mut ps = [[0.0f64; 2]; 2];
            let mut pi = [[0.0f64; 2]; 2];
            for i in 0..2 {
                a_new[i] += m_inf[i] * f1 * v;
                for j in 0..2 {
                    // P*_{t|t} = P* - M_inf f1 M*' - M* f1 M_inf' - M_inf f2 M_inf'
                    ps[i][j] = p_star[i][j]
                        - m_inf[i] * f1 * m_star[j]
                        - m_star[i] * f1 * m_inf[j]
                        - m_inf[i] * f2 * m_inf[j];
                    // P_inf_{t|t} = P_inf - M_inf f1 M_inf'
                    pi[i][j] = p_inf[i][j] - m_inf[i] * f1 * m_inf[j];
                }
            }
            // The diffuse periods contribute -0.5 (log 2pi + log F_inf).
            (a_new, ps, pi, -0.5 * (LN_2PI + f_inf.ln()))
        } else {
            let finv = 1.0 / f_star;
            let mut a_new = a;
            let mut ps = [[0.0f64; 2]; 2];
            for i in 0..2 {
                a_new[i] += m_star[i] * finv * v;
                for j in 0..2 {
                    ps[i][j] = p_star[i][j] - m_star[i] * finv * m_star[j];
                }
            }
            let ll = -0.5 * (LN_2PI + f_star.ln() + v * v * finv);
            (a_new, ps, p_inf, ll)
        };

        lls.push(ll);
        errors.push(v);
        error_cov.push(if diffuse { f_inf } else { f_star });
        filtered.push([a_upd[0], a_upd[1]]);

        // Predict: a = T a_{t|t}, P = T P_{t|t} T' + RQR'.
        a = [
            t_mat[0][0] * a_upd[0] + t_mat[0][1] * a_upd[1],
            t_mat[1][1] * a_upd[1],
        ];
        let mut next_star = [[0.0f64; 2]; 2];
        let mut next_inf = [[0.0f64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                let mut s_star = 0.0;
                let mut s_inf = 0.0;
                for k in 0..2 {
                    for l in 0..2 {
                        s_star += t_mat[i][k] * p_star_upd[k][l] * t_mat[j][l];
                        s_inf += t_mat[i][k] * p_inf_upd[k][l] * t_mat[j][l];
                    }
                }
                next_star[i][j] = s_star + q[i][j];
                next_inf[i][j] = s_inf;
            }
        }
        p_star = next_star;
        p_inf = next_inf;
    }

    FilterOutput {
        llf: lls.iter().sum(),
        loglikeobs: lls,
        forecasts_error: errors,
        forecasts_error_cov: error_cov,
        filtered_state: filtered,
        nobs_diffuse,
    }
}

pub fn loglike(params: &[f64], y: &[f64]) -> f64 {
    kalman_filter(params, y, 1e-10).llf
}

/// statsmodels transforms the three variances by squaring, so the optimizer works on their
/// signed square roots.
pub fn transform_params(u: &[f64]) -> [f64; 3] {
    [u[0] * u[0], u[1] * u[1], u[2] * u[2]]
}

pub fn untransform_params(c: &[f64]) -> [f64; 3] {
    [c[0].sqrt(), c[1].sqrt(), c[2].sqrt()]
}

/// The optimizer objective: -loglike/nobs on the unconstrained parameters.
pub fn objective(u: &[f64], y: &[f64]) -> f64 {
    -loglike(&transform_params(u), y) / y.len() as f64
}

pub struct UcmFit {
    pub params: [f64; 3],
    pub llf: f64,
}

/// UnobservedComponents.fit(disp=False): L-BFGS-B at scipy's defaults with statsmodels'
/// forward-difference gradient on -loglike/nobs.
pub fn fit_ucm(y: &[f64], max_iter: usize) -> UcmFit {
    let start = start_params(y);
    let u0 = untransform_params(&start);
    let n = 3;
    let eps = 1e-5;
    let res = crate::lbfgsb::lbfgsb(
        &u0,
        &vec![f64::NEG_INFINITY; n],
        &vec![f64::INFINITY; n],
        &vec![0i32; n],
        10,
        1e7,
        1e-5,
        20,
        max_iter,
        |u| {
            let f0 = objective(u, y);
            let mut grad = vec![0.0; n];
            for i in 0..n {
                let mut uh = u.to_vec();
                uh[i] += eps;
                let dx = uh[i] - u[i];
                grad[i] = (objective(&uh, y) - f0) / dx;
            }
            (f0, grad)
        },
    );
    let params = transform_params(&res.x);
    UcmFit {
        llf: loglike(&params, y),
        params,
    }
}

/// Per period quantities the smoother needs, kept from the forward pass.
struct FilterTrace {
    a: Vec<[f64; 2]>,
    p_star: Vec<[[f64; 2]; 2]>,
    p_inf: Vec<[[f64; 2]; 2]>,
    v: Vec<f64>,
    f_inf: Vec<f64>,
    f_star: Vec<f64>,
    d: usize,
}

fn filter_trace(params: &[f64], y: &[f64]) -> FilterTrace {
    let n = y.len();
    let (h, q_level, q_trend) = (params[IRREGULAR], params[LEVEL], params[TREND]);
    let t_mat = [[1.0, 1.0], [0.0, 1.0]];
    let q = [[q_level, 0.0], [0.0, q_trend]];
    let mut a = [0.0f64; 2];
    let mut p_star = [[0.0f64; 2]; 2];
    let mut p_inf = [[1.0, 0.0], [0.0, 1.0]];
    let mut tr = FilterTrace {
        a: Vec::with_capacity(n),
        p_star: Vec::with_capacity(n),
        p_inf: Vec::with_capacity(n),
        v: Vec::with_capacity(n),
        f_inf: Vec::with_capacity(n),
        f_star: Vec::with_capacity(n),
        d: 0,
    };
    for &obs in y {
        let p_inf_norm2: f64 = p_inf.iter().flatten().map(|v| v * v).sum();
        let diffuse = p_inf_norm2 > 1e-10;
        let f_inf = p_inf[0][0].max(0.0);
        let f_star = (p_star[0][0] + h).max(0.0);
        let v = obs - a[0];
        tr.a.push(a);
        tr.p_star.push(p_star);
        tr.p_inf.push(p_inf);
        tr.v.push(v);
        tr.f_inf.push(if diffuse { f_inf } else { 0.0 });
        tr.f_star.push(f_star);
        let m_inf = [p_inf[0][0], p_inf[1][0]];
        let m_star = [p_star[0][0], p_star[1][0]];
        let (a_upd, ps, pi) = if diffuse && f_inf > 1e-10 {
            tr.d += 1;
            let f1 = 1.0 / f_inf;
            let f2 = -f_star * f1 * f1;
            let mut an = a;
            let mut ps = [[0.0f64; 2]; 2];
            let mut pi = [[0.0f64; 2]; 2];
            for i in 0..2 {
                an[i] += m_inf[i] * f1 * v;
                for j in 0..2 {
                    ps[i][j] = p_star[i][j]
                        - m_inf[i] * f1 * m_star[j]
                        - m_star[i] * f1 * m_inf[j]
                        - m_inf[i] * f2 * m_inf[j];
                    pi[i][j] = p_inf[i][j] - m_inf[i] * f1 * m_inf[j];
                }
            }
            (an, ps, pi)
        } else {
            let finv = 1.0 / f_star;
            let mut an = a;
            let mut ps = [[0.0f64; 2]; 2];
            for i in 0..2 {
                an[i] += m_star[i] * finv * v;
                for j in 0..2 {
                    ps[i][j] = p_star[i][j] - m_star[i] * finv * m_star[j];
                }
            }
            (an, ps, p_inf)
        };
        a = [a_upd[0] + a_upd[1], a_upd[1]];
        let mut ns = [[0.0f64; 2]; 2];
        let mut ni = [[0.0f64; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                let (mut s, mut si) = (0.0, 0.0);
                for k in 0..2 {
                    for l in 0..2 {
                        s += t_mat[i][k] * ps[k][l] * t_mat[j][l];
                        si += t_mat[i][k] * pi[k][l] * t_mat[j][l];
                    }
                }
                ns[i][j] = s + q[i][j];
                ni[i][j] = si;
            }
        }
        p_star = ns;
        p_inf = ni;
    }
    tr
}

/// The Kalman smoother with Durbin and Koopman's exact diffuse recursions for the first
/// d periods. Returns the smoothed state, 2 x nobs.
pub fn smoothed_state(params: &[f64], y: &[f64]) -> Vec<[f64; 2]> {
    let n = y.len();
    let tr = filter_trace(params, y);
    let t_mat = [[1.0, 1.0], [0.0, 1.0]];
    // r carries the standard recursion; r1 the extra diffuse component.
    let mut r = [0.0f64; 2];
    let mut r1 = [0.0f64; 2];
    let mut out = vec![[0.0f64; 2]; n];

    for t in (0..n).rev() {
        let (a, p_star, p_inf) = (tr.a[t], tr.p_star[t], tr.p_inf[t]);
        if t < tr.d && tr.f_inf[t] > 1e-10 {
            // Diffuse smoothing: L0 = I - K0 Z, L1 = -K1 Z with Z = [1 0].
            let f_inf = tr.f_inf[t];
            let f1 = 1.0 / f_inf;
            let m_inf = [p_inf[0][0], p_inf[1][0]];
            let m_star = [p_star[0][0], p_star[1][0]];
            let f12 = -tr.f_star[t] * f1;
            // The smoother gains carry the transition: K = T P Z' / F, L = T - K Z.
            let tm_inf = [m_inf[0] + m_inf[1], m_inf[1]];
            let tm_star = [m_star[0] + m_star[1], m_star[1]];
            let k0 = [tm_inf[0] * f1, tm_inf[1] * f1];
            let k1 = [tm_star[0] * f1 + k0[0] * f12, tm_star[1] * f1 + k0[1] * f12];
            let l0 = [
                [t_mat[0][0] - k0[0], t_mat[0][1]],
                [t_mat[1][0] - k0[1], t_mat[1][1]],
            ];
            let l1 = [[-k1[0], 0.0], [-k1[1], 0.0]];
            let (r0_prev, r1_prev) = {
                let mut r0n = [0.0f64; 2];
                let mut r1n = [0.0f64; 2];
                for i in 0..2 {
                    for j in 0..2 {
                        r0n[i] += l0[j][i] * r[j];
                        r1n[i] += l0[j][i] * r1[j] + l1[j][i] * r[j];
                    }
                }
                r1n[0] += f1 * tr.v[t];
                (r0n, r1n)
            };
            for i in 0..2 {
                let mut s = a[i];
                for j in 0..2 {
                    s += p_star[i][j] * r0_prev[j] + p_inf[i][j] * r1_prev[j];
                }
                out[t][i] = s;
            }
            r = r0_prev;
            r1 = r1_prev;
        } else {
            let f = tr.f_star[t];
            let finv = 1.0 / f;
            // K = T P* Z' / F, L = T - K Z.
            let m_star = [p_star[0][0], p_star[1][0]];
            let tm = [m_star[0] + m_star[1], m_star[1]];
            let k = [tm[0] * finv, tm[1] * finv];
            let l = [
                [t_mat[0][0] - k[0], t_mat[0][1]],
                [t_mat[1][0] - k[1], t_mat[1][1]],
            ];
            let mut rn = [0.0f64; 2];
            for i in 0..2 {
                for j in 0..2 {
                    rn[i] += l[j][i] * r[j];
                }
            }
            rn[0] += finv * tr.v[t];
            for i in 0..2 {
                let mut s = a[i];
                for j in 0..2 {
                    s += p_star[i][j] * rn[j];
                }
                out[t][i] = s;
            }
            r = rn;
            r1 = [0.0; 2];
        }
    }
    out
}

/// The Durbin and Koopman (2002) simulation smoother, drawing states from their posterior.
///
/// statsmodels draws all measurement variates first, then all state variates, then the
/// initial state variates, from `numpy.random.default_rng(seed)`. With `use_exact_diffuse`
/// the initial state covariance is zero, so the initial variates are consumed but the
/// simulated initial state stays at zero.
pub fn simulation_smoother(
    params: &[f64],
    y: &[f64],
    rng: &mut crate::nprandom::NpRng,
) -> Vec<[f64; 2]> {
    let n = y.len();
    let z_eps: Vec<f64> = (0..n).map(|_| rng.standard_normal()).collect();
    let z_eta: Vec<f64> = (0..2 * n).map(|_| rng.standard_normal()).collect();
    let _z_init: Vec<f64> = (0..2).map(|_| rng.standard_normal()).collect();

    let sd_h = params[IRREGULAR].max(0.0).sqrt();
    let sd_level = params[LEVEL].max(0.0).sqrt();
    let sd_trend = params[TREND].max(0.0).sqrt();

    // Forward pass: generate alpha+ and y+ from the model with a zero initial state.
    let mut generated = vec![[0.0f64; 2]; n];
    let mut y_plus = vec![0.0f64; n];
    let mut alpha = [0.0f64; 2];
    for t in 0..n {
        generated[t] = alpha;
        y_plus[t] = alpha[0] + sd_h * z_eps[t];
        let eta = [sd_level * z_eta[2 * t], sd_trend * z_eta[2 * t + 1]];
        alpha = [alpha[0] + alpha[1] + eta[0], alpha[1] + eta[1]];
    }

    // Smooth the difference and add the generated path back.
    let y_star: Vec<f64> = y.iter().zip(&y_plus).map(|(a, b)| a - b).collect();
    let smoothed = smoothed_state(params, &y_star);
    (0..n)
        .map(|t| {
            [
                generated[t][0] + smoothed[t][0],
                generated[t][1] + smoothed[t][1],
            ]
        })
        .collect()
}

/// scipy.stats.invgamma(shape, scale=scale).rvs(): the generic inversion path, a uniform
/// from numpy's legacy RandomState through the inverse upper incomplete gamma.
pub fn invgamma_rvs(shape: f64, scale: f64, mt: &mut crate::nprandom::Mt19937) -> f64 {
    let u = mt.next_f64();
    scale / spec_math::cephes64::igamci(shape, u)
}

pub struct GibbsResult {
    /// Variance draws, niter + 1 rows including the starting values.
    pub params: Vec<[f64; 3]>,
    /// The states drawn on the first iteration, for checking the chain's start.
    pub first_states: Vec<[f64; 2]>,
}

/// The Fulton Gibbs sampler for the local linear trend: draw the states from their
/// conditional posterior with the simulation smoother, then each variance from its inverse
/// gamma conditional. The two RNG streams are separate, matching statsmodels and scipy.
pub fn gibbs(
    y: &[f64],
    niter: usize,
    prior_shape: f64,
    prior_scale: f64,
    state_rng: &mut crate::nprandom::NpRng,
    ig_rng: &mut crate::nprandom::Mt19937,
) -> GibbsResult {
    let n = y.len();
    let mut params = Vec::with_capacity(niter + 1);
    params.push(start_params(y));
    let mut first_states = Vec::new();

    for i in 1..=niter {
        let current = params[i - 1];
        let states = simulation_smoother(&current, y, state_rng);
        if i == 1 {
            first_states = states.clone();
        }
        let draw = |resid: &[f64], rng: &mut crate::nprandom::Mt19937| {
            let shape = resid.len() as f64 / 2.0 + prior_shape;
            let scale = resid.iter().map(|v| v * v).sum::<f64>() / 2.0 + prior_scale;
            invgamma_rvs(shape, scale, rng)
        };
        // Observation residual: y - Z alpha, with Z = [1 0].
        let obs: Vec<f64> = (0..n).map(|t| y[t] - states[t][0]).collect();
        // State residuals against the transition rows [1 1] and [0 1].
        let lvl: Vec<f64> = (1..n)
            .map(|t| states[t][0] - (states[t - 1][0] + states[t - 1][1]))
            .collect();
        let trd: Vec<f64> = (1..n).map(|t| states[t][1] - states[t - 1][1]).collect();
        let mut next = [0.0f64; 3];
        next[IRREGULAR] = draw(&obs, ig_rng);
        next[LEVEL] = draw(&lvl, ig_rng);
        next[TREND] = draw(&trd, ig_rng);
        params.push(next);
    }
    GibbsResult {
        params,
        first_states,
    }
}
