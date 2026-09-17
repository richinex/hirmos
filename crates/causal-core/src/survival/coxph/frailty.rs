//! Shared gamma frailty for right-censored Cox models, as survival's `coxph` fits it.
//!
//! The port follows three source files of survival 3.6-4. `coxfit5.c` holds the sparse
//! penalised Newton iteration: the frailty coefficients occupy the diagonal block of the
//! information matrix, so only the dense covariate rows are stored (`cholesky3`, `chsolve3`
//! and `chinv3` factor that ragged shape). `coxpenal.fit.R` runs the outer loop that hands
//! the fitted penalised likelihood to the frailty control function and restarts the Newton
//! iteration from the fit whose theta was closest to the next guess. `frailty.gamma.R`
//! supplies the gamma penalty and, through `frailty.controlgam.R`, the "em" theta search
//! with its loglik correction (`frailty.gammacon.R`) and bracketing step (`frailty.brent.R`).
//! The degrees of freedom and the two variance matrices come from `coxpenal.df.R`, and the
//! per-term tests from `summary.coxph.penal.R`.
//!
//! Only the sparse gamma term with covariates is ported: one frailty term, no dense penalty,
//! right-censored rows without delayed entry, optional strata and case weights.

use super::concordance::prediction_concordance_index;
use super::data::{Event, RightCensoredData};
use spec_math::cephes64::chdtrc;

const COXSAFE_LARGE: f64 = 22.0;
const COXSAFE_SMALL: f64 = -200.0;
/// `coxph.wtest` default `toler.chol`.
const WTEST_TOLERANCE: f64 = 1e-9;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TieMethod {
    Breslow,
    Efron,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaFrailtyOptions {
    pub ties: TieMethod,
    /// `coxph.control` `eps`: relative change in the penalised loglik that ends the Newton loop.
    pub eps: f64,
    /// `coxph.control` `toler.chol`.
    pub toler_chol: f64,
    /// `coxph.control` `iter.max`, the Newton steps allowed per outer iteration.
    pub iter_max: usize,
    /// `coxph.control` `outer.max`, the theta updates allowed.
    pub outer_max: usize,
    /// `frailty.gamma` `eps`: relative change in the corrected loglik that ends the theta search.
    pub frailty_eps: f64,
    /// `coxph` `nocenter`: a covariate whose values all lie in this set is not centred.
    pub nocenter: Vec<f64>,
    /// Column groups that form one model term each, in formula order. Every column that is
    /// in no group is its own term. The frailty term always comes last.
    pub terms: Vec<Vec<usize>>,
    /// `coxph.control` `timefix`: bin times that differ by less than `sqrt(.Machine$double.eps)`,
    /// absolutely or relative to the mean time, into the first value of each run (`aeqSurv`).
    pub timefix: bool,
}

impl Default for GammaFrailtyOptions {
    fn default() -> Self {
        Self {
            ties: TieMethod::Efron,
            eps: 1e-9,
            toler_chol: f64::EPSILON.powf(0.75),
            iter_max: 20,
            outer_max: 10,
            frailty_eps: 1e-5,
            nocenter: vec![-1.0, 0.0, 1.0],
            terms: Vec::new(),
            timefix: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrailtyTermTest {
    pub statistic: f64,
    pub df: f64,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PenaltyState {
    /// `coxlist1$first`: minus the penalty gradient handed to the score.
    pub first: Vec<f64>,
    /// `coxlist1$second`: the penalty curvature added to the frailty diagonal.
    pub second: Vec<f64>,
    /// `coxlist1$penalty`: the value added to the partial likelihood.
    pub penalty: f64,
    pub flag: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaFrailtyFit {
    pub coefficients: Vec<f64>,
    pub standard_errors: Vec<f64>,
    pub standard_errors2: Vec<f64>,
    pub coefficient_chisq: Vec<f64>,
    pub coefficient_p_values: Vec<f64>,
    /// `fit$var`, row-major nvar × nvar.
    pub var: Vec<f64>,
    /// `fit$var2`, row-major nvar × nvar.
    pub var2: Vec<f64>,
    pub frail: Vec<f64>,
    pub fvar: Vec<f64>,
    /// Degrees of freedom per term, the frailty term last.
    pub df: Vec<f64>,
    pub trace_h: Vec<f64>,
    /// Partial likelihood at the initial values and at the fit.
    pub loglik: [f64; 2],
    pub penalty: [f64; 2],
    pub outer_iterations: usize,
    pub inner_iterations: usize,
    /// Outer iterations whose Newton loop hit `iter_max`.
    pub inner_failures: Vec<usize>,
    pub means: Vec<f64>,
    pub linear_predictors: Vec<f64>,
    /// One row per outer iteration: theta, penalised loglik, corrected loglik.
    pub history: Vec<[f64; 3]>,
    /// The theta the control function proposed last; equals the final row's theta when converged.
    pub theta: f64,
    pub corrected_loglik: f64,
    pub frailty_test: FrailtyTermTest,
    pub likelihood_ratio: f64,
    pub likelihood_ratio_df: f64,
    pub likelihood_ratio_p_value: f64,
    pub penalty_state: PenaltyState,
    /// Harrell's C on the linear predictor including the frailty, ties in the predictor at one half.
    pub concordance: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GammaFrailtyError {
    DelayedEntryUnsupported,
    GroupLength { expected: usize, actual: usize },
    SingleGroup,
    TermColumn { column: usize },
    NoCoefficients,
    TiedMaximum,
}

/// The saved state of `coxfit5_a`: sorted rows, tie marks, centred covariates.
struct State {
    n: usize,
    nvar: usize,
    nf: usize,
    covar: Vec<Vec<f64>>,
    means: Vec<f64>,
    status: Vec<u8>,
    weights: Vec<f64>,
    ttime: Vec<f64>,
    sort: Vec<usize>,
    strata: Vec<usize>,
    frail: Vec<usize>,
    mark: Vec<f64>,
    wtave: Vec<f64>,
    method: f64,
    score: Vec<f64>,
}

struct Iteration {
    loglik: f64,
    iter: usize,
    imat: Vec<Vec<f64>>,
    jmat: Vec<Vec<f64>>,
    fdiag: Vec<f64>,
    penalty: PenaltyState,
}

fn coxsafe(x: f64) -> f64 {
    if x < COXSAFE_SMALL {
        COXSAFE_SMALL
    } else if x > COXSAFE_LARGE {
        COXSAFE_LARGE
    } else {
        x
    }
}

/// `aeqSurv`: times within `sqrt(.Machine$double.eps)` of each other, absolutely or relative to
/// the mean time, become the first time of their run.
pub(super) fn aeq_surv(times: &[f64]) -> Vec<f64> {
    let tolerance = f64::EPSILON.sqrt();
    let mut y = times.iter().cloned().filter(|t| t.is_finite()).collect::<Vec<_>>();
    y.sort_by(f64::total_cmp);
    y.dedup();
    if y.len() < 2 {
        return times.to_vec();
    }
    let scale = r_mean(&y.iter().map(|v| v.abs()).collect::<Vec<_>>());
    let mut cuts = vec![y[0]];
    let mut any_tied = false;
    for i in 1..y.len() {
        let dy = y[i] - y[i - 1];
        let tied = dy <= tolerance || dy / scale <= tolerance;
        if tied {
            any_tied = true;
        } else {
            cuts.push(y[i]);
        }
    }
    if !any_tied {
        return times.to_vec();
    }
    times
        .iter()
        .map(|t| {
            if !t.is_finite() {
                return *t;
            }
            let index = cuts.partition_point(|c| c <= t);
            cuts[index.saturating_sub(1)]
        })
        .collect()
}

/// R's `mean` for doubles: the sum divided by n, then corrected by the mean residual.
fn r_mean(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let mut s = values.iter().sum::<f64>() / n;
    let t = values.iter().map(|v| v - s).sum::<f64>();
    s += t / n;
    s
}

fn prepare(
    data: &RightCensoredData,
    groups: &[usize],
    options: &GammaFrailtyOptions,
) -> Result<State, GammaFrailtyError> {
    if data.entries().is_some() {
        return Err(GammaFrailtyError::DelayedEntryUnsupported);
    }
    let n = data.rows();
    let nvar = data.columns();
    if groups.len() != n {
        return Err(GammaFrailtyError::GroupLength {
            expected: n,
            actual: groups.len(),
        });
    }
    // frailty.gamma: as.numeric(factor(x)), then coxpenal.fit: match(frailx, sort(unique(frailx)))
    let mut levels = groups.to_vec();
    levels.sort_unstable();
    levels.dedup();
    if levels.len() < 2 {
        return Err(GammaFrailtyError::SingleGroup);
    }
    let frail = groups
        .iter()
        .map(|g| levels.partition_point(|l| l < g) + 1)
        .collect::<Vec<_>>();
    let nf = levels.len();

    let fixed = if options.timefix {
        aeq_surv(data.durations())
    } else {
        data.durations().to_vec()
    };
    let durations = fixed.as_slice();
    let status = data
        .events()
        .iter()
        .map(|e| if *e == Event::Observed { 1u8 } else { 0u8 })
        .collect::<Vec<_>>();
    let weights = data.weights().to_vec();
    // coxpenal.fit: sorted <- order(strata, -y[,1], y[,2]); newstrat <- cumsum(table(strata))
    let mut sort = (0..n).collect::<Vec<_>>();
    let strata_of = data.strata();
    sort.sort_by(|&a, &b| {
        let sa = strata_of.map_or(0, |s| s[a]);
        let sb = strata_of.map_or(0, |s| s[b]);
        sa.cmp(&sb)
            .then(durations[b].total_cmp(&durations[a]))
            .then(status[a].cmp(&status[b]))
    });
    let strata = match strata_of {
        None => vec![n],
        Some(s) => {
            let mut keys = s.to_vec();
            keys.sort_unstable();
            keys.dedup();
            let mut cumulative = 0;
            keys.iter()
                .map(|k| {
                    cumulative += s.iter().filter(|v| *v == k).count();
                    cumulative
                })
                .collect()
        }
    };

    let values = data.covariates().values();
    let mut covar = (0..nvar)
        .map(|i| (0..n).map(|p| values[p * nvar + i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // mark and wtave
    let mut mark = vec![0.0; n];
    let mut wtave = vec![0.0; n];
    let mut i = 0;
    let mut istrat = 0;
    while i < n {
        let p = sort[i];
        if status[p] == 1 {
            let mut temp = 0.0;
            let mut ndead = 0.0;
            let mut j = i;
            while j < n {
                let k = sort[j];
                if durations[k] != durations[p] || j == strata[istrat] {
                    break;
                }
                ndead += f64::from(status[p]);
                temp += weights[k];
                j += 1;
            }
            let k = sort[j - 1];
            mark[k] = ndead;
            wtave[k] = temp / ndead;
            i = j;
        } else {
            i += 1;
        }
        if istrat < strata.len() && i == strata[istrat] {
            istrat += 1;
        }
    }

    // centre the covariates unless every value sits in nocenter
    let mut means = vec![0.0; nvar];
    for i in 0..nvar {
        let docenter = !covar[i]
            .iter()
            .all(|v| options.nocenter.iter().any(|c| c == v));
        if docenter {
            let temp = covar[i].iter().sum::<f64>() / n as f64;
            means[i] = temp;
            for value in &mut covar[i] {
                *value -= temp;
            }
        }
    }

    Ok(State {
        n,
        nvar,
        nf,
        covar,
        means,
        status,
        weights,
        ttime: durations.to_vec(),
        sort,
        strata,
        frail,
        mark,
        wtave,
        method: match options.ties {
            TieMethod::Breslow => 0.0,
            TieMethod::Efron => 1.0,
        },
        score: vec![0.0; n],
    })
}

/// `coxfit5_a`: the loglik at the initial coefficients with no frailty.
fn initial_loglik(state: &State, beta: &[f64]) -> f64 {
    let (n, nvar) = (state.n, state.nvar);
    let mut loglik = 0.0;
    let mut a = vec![0.0; nvar];
    let mut a2 = vec![0.0; nvar];
    let mut u = vec![0.0; nvar];
    let mut denom = 0.0;
    let mut efron_wt = 0.0;
    let mut istrat = 0;
    for ii in 0..n {
        if ii == state.strata[istrat] {
            denom = 0.0;
            a.iter_mut().for_each(|v| *v = 0.0);
            istrat += 1;
        }
        let p = state.sort[ii];
        let mut zbeta = 0.0;
        for i in 0..nvar {
            zbeta += beta[i] * state.covar[i][p];
        }
        let zbeta = coxsafe(zbeta);
        let risk = zbeta.exp() * state.weights[p];
        denom += risk;
        for i in 0..nvar {
            a[i] += risk * state.covar[i][p];
        }
        if state.status[p] == 1 {
            efron_wt += risk;
            loglik += state.weights[p] * zbeta;
            for i in 0..nvar {
                u[i] += state.weights[p] * state.covar[i][p];
                a2[i] += risk * state.covar[i][p];
            }
        }
        if state.mark[p] > 0.0 {
            let ndead = state.mark[p];
            let mut k = 0.0;
            while k < ndead {
                let temp = k * state.method / ndead;
                let d2 = denom - temp * efron_wt;
                loglik -= state.wtave[p] * d2.ln();
                for i in 0..nvar {
                    let temp2 = (a[i] - temp * a2[i]) / d2;
                    u[i] -= state.wtave[p] * temp2;
                }
                k += 1.0;
            }
            efron_wt = 0.0;
            a2.iter_mut().for_each(|v| *v = 0.0);
        }
    }
    loglik
}

/// The gamma penalty of `frailty.gamma`, applied the way `f.expr1` and `cox_callback` do:
/// the frailty coefficients are recentred in place and the score, curvature and value returned.
fn gamma_penalty(fbeta: &mut [f64], theta: f64, previous: &PenaltyState) -> PenaltyState {
    if theta == 0.0 {
        return PenaltyState {
            first: previous.first.clone(),
            second: previous.second.clone(),
            penalty: 0.0,
            flag: true,
        };
    }
    let exps = fbeta.iter().map(|c| c.exp()).collect::<Vec<_>>();
    let recenter = r_mean(&exps).ln();
    for c in fbeta.iter_mut() {
        *c -= recenter;
    }
    let nu = 1.0 / theta;
    let first = fbeta.iter().map(|c| (c.exp() - 1.0) * nu).collect::<Vec<_>>();
    let second = fbeta.iter().map(|c| c.exp() * nu).collect::<Vec<_>>();
    let penalty = -fbeta.iter().sum::<f64>() * nu;
    PenaltyState {
        first: first.iter().map(|f| -f).collect(),
        second,
        penalty: -penalty,
        flag: false,
    }
}

/// `cholesky3`: factor the ragged information matrix, `m` diagonal rows then `n - m` dense ones.
fn cholesky3(matrix: &mut [Vec<f64>], n: usize, m: usize, diag: &mut [f64], toler: f64) -> i32 {
    let n2 = n - m;
    let mut nonneg = 1;
    let mut eps = 0.0_f64;
    for i in 0..m {
        if diag[i] < eps {
            eps = diag[i];
        }
    }
    for i in 0..n2 {
        if matrix[i][i + m] < eps {
            eps = matrix[i][i + m];
        }
    }
    if eps == 0.0 {
        eps = toler;
    } else {
        eps *= toler;
    }
    let mut rank = 0;
    for i in 0..m {
        let pivot = diag[i];
        if !pivot.is_finite() || pivot < eps {
            for j in 0..n2 {
                matrix[j][i] = 0.0;
            }
            if pivot < -8.0 * eps {
                nonneg = -1;
            }
        } else {
            rank += 1;
            for j in 0..n2 {
                let temp = matrix[j][i] / pivot;
                matrix[j][i] = temp;
                matrix[j][j + m] -= temp * temp * pivot;
                for k in (j + 1)..n2 {
                    matrix[k][j + m] -= temp * matrix[k][i];
                }
            }
        }
    }
    for i in 0..n2 {
        let pivot = matrix[i][i + m];
        if !pivot.is_finite() || pivot < eps {
            for j in i..n2 {
                matrix[j][i + m] = 0.0;
            }
            if pivot < -8.0 * eps {
                nonneg = -1;
            }
        } else {
            rank += 1;
            for j in (i + 1)..n2 {
                let temp = matrix[j][i + m] / pivot;
                matrix[j][i + m] = temp;
                matrix[j][j + m] -= temp * temp * pivot;
                for k in (j + 1)..n2 {
                    matrix[k][j + m] -= temp * matrix[k][i + m];
                }
            }
        }
    }
    rank * nonneg
}

fn chsolve3(matrix: &[Vec<f64>], n: usize, m: usize, diag: &[f64], y: &mut [f64]) {
    let n2 = n - m;
    for i in 0..n2 {
        let mut temp = y[i + m];
        for j in 0..m {
            temp -= y[j] * matrix[i][j];
        }
        for j in 0..i {
            temp -= y[j + m] * matrix[i][j + m];
        }
        y[i + m] = temp;
    }
    for i in (0..n2).rev() {
        if matrix[i][i + m] == 0.0 {
            y[i + m] = 0.0;
        } else {
            let mut temp = y[i + m] / matrix[i][i + m];
            for j in (i + 1)..n2 {
                temp -= y[j + m] * matrix[j][i + m];
            }
            y[i + m] = temp;
        }
    }
    for i in (0..m).rev() {
        if diag[i] == 0.0 {
            y[i] = 0.0;
        } else {
            let mut temp = y[i] / diag[i];
            for j in 0..n2 {
                temp -= y[j + m] * matrix[j][i];
            }
            y[i] = temp;
        }
    }
}

fn chinv3(matrix: &mut [Vec<f64>], n: usize, m: usize, fdiag: &mut [f64]) {
    let n2 = n - m;
    for i in 0..m {
        if fdiag[i] > 0.0 {
            fdiag[i] = 1.0 / fdiag[i];
            for j in 0..n2 {
                matrix[j][i] = -matrix[j][i];
            }
        }
    }
    for i in 0..n2 {
        let ii = i + m;
        if matrix[i][ii] > 0.0 {
            matrix[i][ii] = 1.0 / matrix[i][ii];
            for j in (i + 1)..n2 {
                matrix[j][ii] = -matrix[j][ii];
                for k in 0..ii {
                    let add = matrix[j][ii] * matrix[i][k];
                    matrix[j][k] += add;
                }
            }
        }
    }
}

/// `coxfit5_b`: Newton iteration at a fixed theta from the given starting values.
#[allow(clippy::too_many_arguments)]
fn iterate(
    state: &mut State,
    maxiter: usize,
    beta: &mut [f64],
    fbeta: &mut [f64],
    theta: f64,
    previous_penalty: &PenaltyState,
    eps: f64,
    toler_chol: f64,
) -> Iteration {
    let (n, nvar, nf) = (state.n, state.nvar, state.nf);
    let nvar2 = nvar + nf;
    let method = state.method;
    let mut imat = vec![vec![0.0; nvar2]; nvar];
    let mut jmat = vec![vec![0.0; nvar2]; nvar];
    let mut cmat = vec![vec![0.0; nvar2]; nvar];
    let mut cmat2 = vec![vec![0.0; nvar2]; nvar];
    let mut a = vec![0.0; nvar2];
    let mut a2 = vec![0.0; nvar2];
    let mut u = vec![0.0; nvar2];
    let mut tmean = vec![0.0; nvar2];
    let mut fdiag = vec![0.0; nvar2];
    let mut oldbeta = vec![0.0; nvar2];
    oldbeta[..nf].copy_from_slice(fbeta);
    oldbeta[nf..].copy_from_slice(beta);
    let mut penalty = previous_penalty.clone();
    let mut loglik = 0.0;
    let mut newlk = 0.0;
    let mut halving = false;
    let mut iter = 0;
    let mut converged = false;

    let finish = |jmat: &mut Vec<Vec<f64>>, imat: &mut Vec<Vec<f64>>, fdiag: &mut Vec<f64>| {
        for i in 0..nvar {
            for j in 0..nvar2 {
                imat[i][j] = jmat[i][j];
            }
        }
        chinv3(jmat, nvar2, nf, fdiag);
        for i in nf..nvar2 {
            fdiag[i] = jmat[i - nf][i];
            jmat[i - nf][i] = 1.0;
            imat[i - nf][i] = 1.0;
            for j in (i + 1)..nvar2 {
                jmat[i - nf][j] = 0.0;
                imat[i - nf][j] = 0.0;
            }
        }
    };

    while iter <= maxiter {
        newlk = 0.0;
        fdiag[..nf].iter_mut().for_each(|v| *v = 0.0);
        u.iter_mut().for_each(|v| *v = 0.0);
        jmat.iter_mut().for_each(|row| row.iter_mut().for_each(|v| *v = 0.0));

        let mut istrat = 0;
        let mut denom = 0.0;
        let mut efron_wt = 0.0;
        for ip in 0..n {
            let p = state.sort[ip];
            if ip == 0 || ip == state.strata[istrat] {
                efron_wt = 0.0;
                denom = 0.0;
                a.iter_mut().for_each(|v| *v = 0.0);
                a2.iter_mut().for_each(|v| *v = 0.0);
                cmat.iter_mut().for_each(|row| row.iter_mut().for_each(|v| *v = 0.0));
                cmat2.iter_mut().for_each(|row| row.iter_mut().for_each(|v| *v = 0.0));
            }
            if ip == state.strata[istrat] {
                istrat += 1;
            }
            let fgrp = state.frail[p] - 1;
            let mut zbeta = fbeta[fgrp];
            for i in 0..nvar {
                zbeta += beta[i] * state.covar[i][p];
            }
            let zbeta = coxsafe(zbeta);
            state.score[p] = zbeta.exp();
            let risk = state.score[p] * state.weights[p];
            denom += risk;
            a[fgrp] += risk;
            for i in 0..nvar {
                let ci = state.covar[i][p];
                a[i + nf] += risk * ci;
                cmat[i][fgrp] += risk * ci;
                for j in 0..=i {
                    cmat[i][j + nf] += risk * ci * state.covar[j][p];
                }
            }
            if state.status[p] == 1 {
                let w = state.weights[p];
                efron_wt += risk;
                newlk += w * zbeta;
                u[fgrp] += w;
                a2[fgrp] += risk;
                for i in 0..nvar {
                    let ci = state.covar[i][p];
                    u[i + nf] += w * ci;
                    a2[i + nf] += risk * ci;
                    cmat2[i][fgrp] += risk * ci;
                    for j in 0..=i {
                        cmat2[i][j + nf] += risk * ci * state.covar[j][p];
                    }
                }
            }
            if state.mark[p] > 0.0 {
                let ndead = state.mark[p];
                let wt = state.wtave[p];
                let mut k = 0.0;
                while k < ndead {
                    let temp = k * method / ndead;
                    let d2 = denom - temp * efron_wt;
                    newlk -= wt * d2.ln();
                    for i in 0..nvar2 {
                        let temp2 = (a[i] - temp * a2[i]) / d2;
                        tmean[i] = temp2;
                        u[i] -= wt * temp2;
                        if i < nf {
                            fdiag[i] += temp2 * (1.0 - temp2);
                        } else {
                            let ii = i - nf;
                            for j in 0..=i {
                                jmat[ii][j] +=
                                    wt * ((cmat[ii][j] - temp * cmat2[ii][j]) / d2 - temp2 * tmean[j]);
                            }
                        }
                    }
                    k += 1.0;
                }
                efron_wt = 0.0;
                a2.iter_mut().for_each(|v| *v = 0.0);
                cmat2.iter_mut().for_each(|row| row.iter_mut().for_each(|v| *v = 0.0));
            }
        }

        // the sparse penalty; the callback recentres fbeta in place
        penalty = gamma_penalty(fbeta, theta, &penalty);
        if penalty.flag {
            for i in 0..nf {
                u[i] = 0.0;
                fdiag[i] = 1.0;
                for row in jmat.iter_mut() {
                    row[i] = 0.0;
                }
            }
        } else {
            for i in 0..nf {
                u[i] += penalty.first[i];
                fdiag[i] += penalty.second[i];
            }
            newlk += penalty.penalty;
        }

        let _flag = cholesky3(&mut jmat, nvar2, nf, &mut fdiag, toler_chol);
        if newlk.abs() < eps || ((1.0 - loglik / newlk).abs() <= eps && !halving) {
            loglik = newlk;
            converged = true;
            break;
        }
        if iter == maxiter {
            break;
        }
        if iter > 0 && newlk < loglik {
            halving = true;
            for i in 0..nvar {
                beta[i] = (oldbeta[i + nf] + beta[i]) / 2.0;
            }
            for i in 0..nf {
                fbeta[i] = (oldbeta[i] + fbeta[i]) / 2.0;
            }
        } else {
            halving = false;
            loglik = newlk;
            chsolve3(&jmat, nvar2, nf, &fdiag, &mut u);
            for i in 0..nvar {
                oldbeta[i + nf] = beta[i];
                beta[i] += u[i + nf];
            }
            for i in 0..nf {
                oldbeta[i] = fbeta[i];
                fbeta[i] += u[i];
            }
        }
        iter += 1;
    }
    if !converged {
        loglik = newlk;
    }
    finish(&mut jmat, &mut imat, &mut fdiag);
    Iteration {
        loglik,
        iter,
        imat,
        jmat,
        fdiag,
        penalty,
    }
}

/// `frailty.gammacon`: the correction that turns the penalised loglik into the marginal one.
fn gamma_correction(deaths: &[f64], nu: f64) -> f64 {
    let maxd = deaths.iter().cloned().fold(0.0_f64, f64::max);
    let term1 = if nu > 1e7 * maxd {
        deaths.iter().map(|d| d * d).sum::<f64>() / nu
    } else {
        deaths
            .iter()
            .map(|d| d + nu * (nu / (nu + d)).ln())
            .sum::<f64>()
    };
    let levels = maxd.round() as usize;
    let mut tbl = vec![0usize; levels + 1];
    for d in deaths {
        if *d > 0.0 {
            tbl[d.round() as usize] += 1;
        }
    }
    let mut ctbl = vec![0usize; levels + 1];
    let mut running = 0;
    for level in (1..=levels).rev() {
        running += tbl[level];
        ctbl[level] = running;
    }
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    for level in 1..=levels {
        for _ in 0..ctbl[level] {
            numerator.push(nu + (level as f64 - 1.0));
        }
        for _ in 0..(tbl[level] * level) {
            denominator.push(nu + level as f64);
        }
    }
    let term2 = numerator
        .iter()
        .zip(&denominator)
        .map(|(a, b)| (a / b).ln())
        .sum::<f64>();
    term1 + term2
}

/// `frailty.brent`: the next theta from the bracketing history, on the given scale.
fn brent(x: &[f64], y: &[f64]) -> Result<f64, GammaFrailtyError> {
    let n = x.len();
    if n < 3 {
        return Ok(r_mean(x));
    }
    let mut ord = (0..n).collect::<Vec<_>>();
    ord.sort_by(|&a, &b| x[a].total_cmp(&x[b]));
    let xx = ord.iter().map(|&i| x[i]).collect::<Vec<_>>();
    let yy = ord.iter().map(|&i| y[i]).collect::<Vec<_>>();
    let ymax = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let best = (0..n).filter(|&i| yy[i] == ymax).collect::<Vec<_>>();
    if best.len() > 1 {
        return Err(GammaFrailtyError::TiedMaximum);
    }
    let best = best[0];
    let lower = 0.0;
    if best == 0 {
        let mut new = xx[0] - 3.0 * (xx[1] - xx[0]);
        if new < lower {
            let above = xx
                .iter()
                .cloned()
                .filter(|v| *v > lower)
                .fold(f64::INFINITY, f64::min);
            new = lower + (above - lower) / 10.0;
        }
        return Ok(new);
    }
    if best == n - 1 {
        return Ok(xx[n - 1] + 3.0 * (xx[n - 1] - xx[n - 2]));
    }
    let bx = [xx[best - 1], xx[best], xx[best + 1]];
    let by = [yy[best - 1], yy[best], yy[best + 1]];
    let temp1 = (bx[1] - bx[0]).powi(2) * (by[1] - by[2]) - (bx[1] - bx[2]).powi(2) * (by[1] - by[0]);
    let temp2 = (bx[1] - bx[0]) * (by[1] - by[2]) - (bx[1] - bx[2]) * (by[1] - by[0]);
    let new = bx[1] - 0.5 * temp1 / temp2;
    let bouncing = n > 4 && (new - x[n - 1]) > 0.5 * (x[n - 2] - x[n - 3]).abs();
    if new < bx[0] || new > bx[2] || bouncing {
        if (bx[1] - bx[0]) > (bx[2] - bx[1]) {
            Ok(bx[1] - 0.38 * (bx[1] - bx[0]))
        } else {
            Ok(bx[1] + 0.32 * (bx[2] - bx[1]))
        }
    } else {
        Ok(new)
    }
}

struct ControlState {
    theta: f64,
    done: bool,
    history: Vec<[f64; 3]>,
    corrected: f64,
}

/// `frailty.controlgam` for the "em" method with no initial value.
fn control_step(
    iter: usize,
    old: &ControlState,
    deaths: &[f64],
    loglik: f64,
    frailty_eps: f64,
) -> Result<ControlState, GammaFrailtyError> {
    let theta = old.theta;
    let correct = if theta == 0.0 {
        0.0
    } else {
        gamma_correction(deaths, 1.0 / theta)
    };
    let mut history = old.history.clone();
    history.push([theta, loglik, loglik + correct]);
    if iter == 1 {
        return Ok(ControlState {
            theta: 1.0,
            done: false,
            history,
            corrected: loglik + correct,
        });
    }
    if iter == 2 {
        let theta = if history[1][2] < history[0][2] + 1.0 {
            r_mean(&[history[0][0], history[1][0]])
        } else {
            2.0 * history[1][0]
        };
        return Ok(ControlState {
            theta,
            done: false,
            history,
            corrected: loglik + correct,
        });
    }
    let done = (1.0 - history[iter - 1][2] / history[iter - 2][2]).abs() < frailty_eps;
    let x = history.iter().map(|row| row[0]).collect::<Vec<_>>();
    let y = history.iter().map(|row| row[2]).collect::<Vec<_>>();
    let xmax = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ymax = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let newtheta = if y[iter - 1] == ymax && x[iter - 1] == xmax {
        2.0 * xmax
    } else {
        let sqrt_x = x.iter().map(|v| v.sqrt()).collect::<Vec<_>>();
        brent(&sqrt_x, &y)?.powi(2)
    };
    Ok(ControlState {
        theta: newtheta,
        done,
        history,
        corrected: loglik + correct,
    })
}

/// `cholesky2` on a row-major square matrix, as `coxph.wtest` uses it.
pub(super) fn cholesky2(matrix: &mut [f64], n: usize, toler: f64) {
    let mut eps = 0.0_f64;
    for i in 0..n {
        if matrix[i * n + i] > eps {
            eps = matrix[i * n + i];
        }
        for j in (i + 1)..n {
            matrix[j * n + i] = matrix[i * n + j];
        }
    }
    if eps == 0.0 {
        eps = toler;
    } else {
        eps *= toler;
    }
    for i in 0..n {
        let pivot = matrix[i * n + i];
        if !pivot.is_finite() || pivot < eps {
            matrix[i * n + i] = 0.0;
        } else {
            for j in (i + 1)..n {
                let temp = matrix[j * n + i] / pivot;
                matrix[j * n + i] = temp;
                matrix[j * n + j] -= temp * temp * pivot;
                for k in (j + 1)..n {
                    matrix[k * n + j] -= temp * matrix[k * n + i];
                }
            }
        }
    }
}

pub(super) fn chsolve2(matrix: &[f64], n: usize, y: &mut [f64]) {
    for i in 0..n {
        let mut temp = y[i];
        for j in 0..i {
            temp -= y[j] * matrix[i * n + j];
        }
        y[i] = temp;
    }
    for i in (0..n).rev() {
        if matrix[i * n + i] == 0.0 {
            y[i] = 0.0;
        } else {
            let mut temp = y[i] / matrix[i * n + i];
            for j in (i + 1)..n {
                temp -= y[j] * matrix[j * n + i];
            }
            y[i] = temp;
        }
    }
}

/// `sum(diag(coxph.wtest(var, b)$solve))` for square `var` and `b` of order `k`.
fn wtest_trace(var: &[f64], b: &[f64], k: usize) -> f64 {
    if k == 1 {
        return b[0] / var[0];
    }
    let mut factor = var.to_vec();
    cholesky2(&mut factor, k, WTEST_TOLERANCE);
    let mut trace = 0.0;
    for column in 0..k {
        let mut solve = (0..k).map(|row| b[row * k + column]).collect::<Vec<_>>();
        chsolve2(&factor, k, &mut solve);
        trace += solve[column];
    }
    trace
}

struct DegreesOfFreedom {
    var: Vec<f64>,
    var2: Vec<f64>,
    fvar: Vec<f64>,
    df: Vec<f64>,
    trace_h: Vec<f64>,
}

/// `coxpenal.df` for one sparse term beside dense covariates.
fn penalised_df(
    jmat: &[Vec<f64>],
    fdiag: &[f64],
    nvar: usize,
    nf: usize,
    pen1: &[f64],
    terms: &[Vec<usize>],
) -> DegreesOfFreedom {
    let d1 = &fdiag[..nf];
    let d2 = &fdiag[nf..];
    // hinv[f, i] = jmat[i][f]; hinv[nf + k, i] = jmat[i][nf + k]
    let a_diag = (0..nf)
        .map(|f| d1[f] + (0..nvar).map(|i| jmat[i][f] * jmat[i][f] * d2[i]).sum::<f64>())
        .collect::<Vec<_>>();
    let mut b = vec![0.0; nf * nvar];
    for f in 0..nf {
        for k in 0..nvar {
            b[f * nvar + k] = (0..nvar)
                .map(|i| jmat[i][f] * d2[i] * jmat[i][nf + k])
                .sum::<f64>();
        }
    }
    let mut c = vec![0.0; nvar * nvar];
    for k in 0..nvar {
        for l in 0..nvar {
            c[k * nvar + l] = (0..nvar)
                .map(|i| jmat[i][nf + k] * d2[i] * jmat[i][nf + l])
                .sum::<f64>();
        }
    }
    let mut var2 = c.clone();
    for k in 0..nvar {
        for l in 0..nvar {
            var2[k * nvar + l] -= (0..nf)
                .map(|f| b[f * nvar + k] * pen1[f] * b[f * nvar + l])
                .sum::<f64>();
        }
    }
    let mut df = Vec::with_capacity(terms.len() + 1);
    let mut trace_h = Vec::with_capacity(terms.len() + 1);
    for term in terms {
        let k = term.len();
        let sub = |m: &[f64]| {
            let mut out = vec![0.0; k * k];
            for (r, &i) in term.iter().enumerate() {
                for (s, &j) in term.iter().enumerate() {
                    out[r * k + s] = m[i * nvar + j];
                }
            }
            out
        };
        df.push(wtest_trace(&sub(&c), &sub(&var2), k));
        trace_h.push(term.iter().map(|&i| c[i * nvar + i]).sum::<f64>());
    }
    let sparse_df = nf as f64 - a_diag.iter().zip(pen1).map(|(a, p)| a * p).sum::<f64>();
    df.push(sparse_df);
    trace_h.push(a_diag.iter().sum::<f64>());
    DegreesOfFreedom {
        var: c,
        var2,
        fvar: a_diag,
        df,
        trace_h,
    }
}

/// Fit a Cox model with a shared gamma frailty on `groups`, one group id per row.
pub fn fit_gamma_frailty(
    data: &RightCensoredData,
    groups: &[usize],
    options: &GammaFrailtyOptions,
) -> Result<GammaFrailtyFit, GammaFrailtyError> {
    let mut state = prepare(data, groups, options)?;
    let (n, nvar, nf) = (state.n, state.nvar, state.nf);
    if nvar == 0 {
        return Err(GammaFrailtyError::NoCoefficients);
    }
    let mut terms = options.terms.clone();
    let mut covered = vec![false; nvar];
    for term in &terms {
        for &column in term {
            if column >= nvar {
                return Err(GammaFrailtyError::TermColumn { column });
            }
            covered[column] = true;
        }
    }
    for column in 0..nvar {
        if !covered[column] {
            terms.push(vec![column]);
        }
    }
    terms.sort_by_key(|term| term[0]);

    let deaths = {
        let mut d = vec![0.0; nf];
        for p in 0..n {
            d[state.frail[p] - 1] += f64::from(state.status[p]);
        }
        d
    };

    let mut init = vec![0.0; nvar];
    let mut finit = vec![0.0; nf];
    let loglik0 = initial_loglik(&state, &init);

    let mut control = ControlState {
        theta: 0.0,
        done: false,
        history: Vec::new(),
        corrected: 0.0,
    };
    let mut penalty_state = PenaltyState {
        first: vec![0.0; nf],
        second: vec![0.0; nf],
        penalty: 0.0,
        flag: false,
    };
    let mut iter2 = 0;
    let mut iterfail = Vec::new();
    let mut thetasave = vec![control.theta];
    let mut coefsave: Vec<Vec<f64>> = Vec::new();
    let mut fsave: Vec<Vec<f64>> = Vec::new();
    let mut penalty0 = 0.0;
    let mut penalty_last = 0.0;
    let mut outer = 0;
    let mut result: Option<(Iteration, Vec<f64>, Vec<f64>)> = None;

    for iter in 1..=options.outer_max {
        outer = iter;
        let mut beta = init.clone();
        let mut fbeta = finit.clone();
        let theta = control.theta;
        let fit = iterate(
            &mut state,
            options.iter_max,
            &mut beta,
            &mut fbeta,
            theta,
            &penalty_state,
            options.eps,
            options.toler_chol,
        );
        iter2 += fit.iter;
        if fit.iter >= options.iter_max {
            iterfail.push(iter);
        }
        penalty_state = fit.penalty.clone();
        let penalty = -fit.penalty.penalty;
        if iter == 1 {
            penalty0 = penalty;
        }
        penalty_last = penalty;
        let next = control_step(iter, &control, &deaths, fit.loglik, options.frailty_eps)?;
        control = next;
        result = Some((fit, beta.clone(), fbeta.clone()));
        if control.done {
            break;
        }
        if iter == 1 {
            init = beta.clone();
            finit = fbeta.clone();
            coefsave.push(beta);
            fsave.push(fbeta);
            thetasave.push(control.theta);
        } else {
            coefsave.push(beta);
            fsave.push(fbeta);
            let temp = control.theta;
            let howclose = thetasave
                .iter()
                .take(iter)
                .map(|t| (t - temp) * (t - temp))
                .collect::<Vec<_>>();
            let best = howclose.iter().cloned().fold(f64::INFINITY, f64::min);
            let which = howclose.iter().position(|h| *h == best).unwrap_or(0);
            init = coefsave[which].clone();
            finit = fsave[which].clone();
            thetasave.push(temp);
        }
    }
    let (fit, beta, fbeta) = result.expect("outer_max is at least one");

    let mut fdiag = fit.fdiag.clone();
    if fit.penalty.flag {
        for value in fdiag.iter_mut().take(nf) {
            *value = 0.0;
        }
    }
    let dof = penalised_df(&fit.jmat, &fdiag, nvar, nf, &fit.penalty.second, &terms);
    let loglik1 = fit.loglik + penalty_last;

    let mut linear_predictors = vec![0.0; n];
    for p in 0..n {
        let mut lp = fbeta[state.frail[p] - 1];
        for i in 0..nvar {
            lp += state.covar[i][p] * beta[i];
        }
        linear_predictors[p] = lp;
    }

    let standard_errors = (0..nvar).map(|k| dof.var[k * nvar + k].sqrt()).collect::<Vec<_>>();
    let standard_errors2 = (0..nvar).map(|k| dof.var2[k * nvar + k].sqrt()).collect::<Vec<_>>();
    let coefficient_chisq = (0..nvar)
        .map(|k| beta[k] * beta[k] / dof.var[k * nvar + k])
        .collect::<Vec<_>>();
    let coefficient_p_values = coefficient_chisq
        .iter()
        .map(|stat| chdtrc(1.0, *stat))
        .collect::<Vec<_>>();
    let frailty_df = *dof.df.last().expect("the frailty term is last");
    let frailty_statistic = fbeta
        .iter()
        .zip(&dof.fvar)
        .map(|(f, v)| f * f / v)
        .sum::<f64>();
    let frailty_test = FrailtyTermTest {
        statistic: frailty_statistic,
        df: frailty_df,
        p_value: chdtrc(frailty_df.max(0.5), frailty_statistic),
    };
    let total_df = dof.df.iter().sum::<f64>();
    let likelihood_ratio = -2.0 * (loglik0 - loglik1);
    let coefficients = (0..nvar)
        .map(|k| if fdiag[nf + k] == 0.0 { f64::NAN } else { beta[k] })
        .collect::<Vec<_>>();
    // survival's concordance(fit) reverses the linear predictor: a larger value means an earlier event.
    let predictions = linear_predictors.iter().map(|lp| -lp).collect::<Vec<_>>();
    let concordance = prediction_concordance_index(&state.ttime, data.events(), &predictions, data.strata())
        .map(|index| index.value());

    Ok(GammaFrailtyFit {
        coefficients,
        standard_errors,
        standard_errors2,
        coefficient_chisq,
        coefficient_p_values,
        var: dof.var,
        var2: dof.var2,
        frail: fbeta,
        fvar: dof.fvar,
        df: dof.df,
        trace_h: dof.trace_h,
        loglik: [loglik0, loglik1],
        penalty: [penalty0, penalty_last],
        outer_iterations: outer,
        inner_iterations: iter2,
        inner_failures: iterfail,
        means: state.means.clone(),
        linear_predictors,
        history: control.history.clone(),
        theta: control.theta,
        corrected_loglik: control.corrected,
        frailty_test,
        likelihood_ratio,
        likelihood_ratio_df: total_df,
        likelihood_ratio_p_value: chdtrc(total_df, likelihood_ratio),
        penalty_state: fit.penalty,
        concordance,
    })
}
