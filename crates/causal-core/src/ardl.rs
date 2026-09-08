//! `statsmodels.tsa.ardl`: `ardl_select_order`, `UECM`, the cointegrating vector and the
//! PSS bounds test. Single exogenous variable, `causal=False`, trend `c` or `ct`.

use crate::pss_tables;
use nalgebra::{DMatrix, DVector};

/// `scipy.stats.norm.cdf`.
fn norm_cdf(x: f64) -> f64 {
    spec_math::cephes64::ndtr(x)
}

/// `scipy.stats.t(df).ppf`, through the inverse incomplete beta as cephes stdtri does.
fn t_ppf(p: f64, df: f64) -> f64 {
    let (p, sign) = if p < 0.5 { (p, -1.0) } else { (1.0 - p, 1.0) };
    let z = spec_math::cephes64::incbi(0.5 * df, 0.5, 2.0 * p);
    sign * (df * (1.0 - z) / z).sqrt()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    None,
    C,
    Ct,
}

impl Trend {
    fn n_det(self) -> usize {
        match self {
            Trend::None => 0,
            Trend::C => 1,
            Trend::Ct => 2,
        }
    }

    /// `DeterministicProcess.in_sample()` over positions 1..=n.
    fn in_sample(self, n: usize) -> DMatrix<f64> {
        DMatrix::from_fn(n, self.n_det(), |r, c| {
            if c == 0 && self != Trend::None {
                1.0
            } else {
                (r + 1) as f64
            }
        })
    }
}

/// Ordinary least squares with the nonrobust covariance and `use_t=True`.
pub struct Ols {
    pub params: DVector<f64>,
    pub cov_params: DMatrix<f64>,
    pub resid: DVector<f64>,
    pub nobs: usize,
    pub df_model: usize,
}

impl Ols {
    pub fn df_resid(&self) -> f64 {
        (self.nobs - self.df_model) as f64
    }
}

fn ols(x: &DMatrix<f64>, y: &DVector<f64>) -> Ols {
    let fit = crate::ols::Ols::fit(x, y);
    let nobs = fit.nobs;
    let df_model = fit.rank;
    let scale = fit.ssr / (nobs - df_model) as f64;
    let cov_params = fit.xtx_inverse() * scale;
    Ols {
        params: fit.params,
        cov_params,
        resid: fit.resid,
        nobs,
        df_model,
    }
}

fn numpy_lstsq(x: &DMatrix<f64>, y: &DVector<f64>) -> DVector<f64> {
    let target = DMatrix::from_column_slice(y.len(), 1, y.as_slice());
    let tolerance = f64::EPSILON * x.nrows().max(x.ncols()) as f64;
    crate::least_squares::solve(x, &target, tolerance)
        .expect("finite ARDL candidate least-squares inputs")
        .coefficients
        .column(0)
        .into_owned()
}

/// Column `lag` of `lagmat(col, max_lag, original="in")`: NaN rows are simply not reachable
/// because everything is trimmed by `hold_back` afterwards.
fn lagged(col: &[f64], lag: usize, row: usize) -> f64 {
    col[row - lag]
}

pub struct ArdlSpec {
    pub ar_lag: usize,
    pub dl_lag: usize,
}

/// `ardl_select_order(y, maxlag, x, maxorder, ic="aic", trend=..)` for one exog column,
/// `causal=False`. Returns the selected `(p, q)` and the whole information criterion grid.
pub struct OrderSelection {
    pub ar_lag: usize,
    /// The exog order as lingam's ARDL reports it: `None` when only lag zero was chosen.
    pub dl_lag: Option<usize>,
    /// `(ar_lag, dl_lag_or_none, aic, bic, hqic)` for every candidate, in search order.
    pub grid: Vec<(usize, Option<usize>, f64, f64, f64)>,
}

pub fn ardl_select_order(
    y: &[f64],
    maxlag: usize,
    x: &[f64],
    maxorder: usize,
    ic: &str,
    trend: Trend,
) -> OrderSelection {
    let n = y.len();
    let hold_back = maxlag.max(maxorder);
    let rows = n - hold_back;

    let det = trend.in_sample(n);
    let always = DMatrix::from_fn(rows, det.ncols(), |r, c| det[(hold_back + r, c)]);
    // Endog block holds lags 1..=maxlag, the exog block lags 0..=maxorder.
    let endog_block = DMatrix::from_fn(rows, maxlag, |r, c| lagged(y, c + 1, hold_back + r));
    let exog_block = DMatrix::from_fn(rows, maxorder + 1, |r, c| lagged(x, c, hold_back + r));
    let mut yv = DVector::from_fn(rows, |r, _| y[hold_back + r]);

    // Frisch-Waugh: the deterministics are partialled out once instead of per candidate.
    let mut blocks = [endog_block, exog_block];
    let always_df = always.ncols();
    if always_df > 0 {
        let pinv = crate::linalg::pseudo_inverse(&always, 1e-15)
            .expect("ARDL deterministic projection")
            .matrix;
        for block in blocks.iter_mut() {
            *block -= &always * (&pinv * &*block);
        }
        yv -= &always * (&pinv * &yv);
    }
    let [endog_block, exog_block] = blocks;

    let compute_ics = |x: &DMatrix<f64>| -> (f64, f64, f64) {
        let resid = if x.ncols() > 0 {
            &yv - x * numpy_lstsq(x, &yv)
        } else {
            yv.clone()
        };
        let nobs = resid.len() as f64;
        let sigma2 = resid.iter().map(|v| v * v).sum::<f64>() / nobs;
        let llf = -nobs * ((2.0 * std::f64::consts::PI * sigma2).ln() + 1.0) / 2.0;
        let df = (always_df + x.ncols() + 1) as f64;
        (
            -2.0 * llf + 2.0 * df,
            -2.0 * llf + nobs.ln() * df,
            -2.0 * llf + 2.0 * nobs.ln().ln() * df,
        )
    };

    let index = match ic {
        "aic" => 0,
        "bic" => 1,
        "hqic" => 2,
        other => panic!("unknown information criterion {other}"),
    };
    let mut grid = Vec::new();
    let mut best = f64::INFINITY;
    let mut best_key = (0usize, None);
    // itertools.product over the endog count then the exog count, both counting from zero.
    for p in 0..=maxlag {
        for q in 0..=maxorder + 1 {
            let cand = DMatrix::from_fn(rows, p + q, |r, c| {
                if c < p {
                    endog_block[(r, c)]
                } else {
                    exog_block[(r, c - p)]
                }
            });
            let (aic, bic, hqic) = compute_ics(&cand);
            let dl = if q == 0 { None } else { Some(q - 1) };
            grid.push((p, dl, aic, bic, hqic));
            let val = [aic, bic, hqic][index];
            if val < best {
                best = val;
                best_key = (p, dl);
            }
        }
    }
    OrderSelection {
        ar_lag: best_key.0,
        dl_lag: best_key.1,
        grid,
    }
}

pub struct Uecm {
    pub n_det: usize,
    pub n_levels: usize,
    pub ardl_order: Vec<usize>,
    pub fit: Ols,
    /// `results.resid`, which subtracts the fitted differences from the level endog. The
    /// scale and the standard errors use `fit.resid` instead.
    pub reported_resid: Vec<f64>,
    /// `results.nobs`, which statsmodels takes from the endog lag alone and so exceeds the
    /// number of design rows whenever the exog order is the binding one.
    pub reported_nobs: usize,
}

impl Uecm {
    /// `results.df_resid`, built from `reported_nobs` rather than the design.
    pub fn reported_df_resid(&self) -> f64 {
        (self.reported_nobs - self.fit.df_model) as f64
    }
}

/// `UECM(y, p, x, q, trend).fit()` for one exogenous column.
pub fn uecm(y: &[f64], p: usize, x: &[f64], q: usize, trend: Trend) -> Uecm {
    assert!(
        q >= 1,
        "all included exog variables must have a lag length >= 1"
    );
    let n = y.len();
    let hold_back = p.max(q).max(1);
    let rows = n - hold_back;
    let n_det = trend.n_det();
    let dlag = p.saturating_sub(1);
    // The exog differences keep lags 0..q-1, which is `order[:-1]`.
    let n_dexog = q;
    let n_cols = n_det + 2 + dlag + n_dexog;

    let det = trend.in_sample(n);
    let dy = |i: usize| y[i] - y[i - 1];
    let dx = |i: usize| x[i] - x[i - 1];

    let design = DMatrix::from_fn(rows, n_cols, |r, c| {
        let i = hold_back + r;
        if c < n_det {
            det[(i, c)]
        } else if c == n_det {
            y[i - 1]
        } else if c == n_det + 1 {
            x[i - 1]
        } else if c < n_det + 2 + dlag {
            dy(i - (c - n_det - 1))
        } else {
            dx(i - (c - n_det - 2 - dlag))
        }
    });
    let target = DVector::from_fn(rows, |r, _| dy(hold_back + r));
    let fit = ols(&design, &target);
    let fitted = &design * &fit.params;
    let reported_resid = (0..rows).map(|r| y[hold_back + r] - fitted[r]).collect();
    Uecm {
        n_det,
        n_levels: 2,
        ardl_order: vec![p, q],
        fit,
        reported_resid,
        reported_nobs: n - p,
    }
}

pub struct CointegratingVector {
    pub params: Vec<f64>,
    pub bse: Vec<f64>,
    pub tvalues: Vec<f64>,
    pub pvalues: Vec<f64>,
    pub conf_int: Vec<(f64, f64)>,
}

impl Uecm {
    /// `ci_cov_params()`: the delta method Jacobian of dividing through by the level
    /// coefficient on the endogenous variable.
    pub fn ci_cov_params(&self) -> DMatrix<f64> {
        let m = self.n_det + self.n_levels;
        let cov = self.fit.cov_params.view((0, 0), (m, m)).into_owned();
        let base = self.fit.params[self.n_det];
        let mut d = DMatrix::<f64>::zeros(m, m);
        for i in 0..m {
            if i == self.n_det {
                continue;
            }
            d[(i, i)] = 1.0 / base;
            d[(i, self.n_det)] = -self.fit.params[i] / (base * base);
        }
        &d * cov * d.transpose()
    }

    /// `ci_params`, `ci_bse`, `ci_tvalues`, `ci_pvalues` and `ci_conf_int(alpha)`.
    pub fn cointegrating_vector(&self, alpha: f64) -> CointegratingVector {
        let m = self.n_det + self.n_levels;
        let base = self.fit.params[self.n_det];
        let params: Vec<f64> = (0..m).map(|i| self.fit.params[i] / base).collect();
        let cov = self.ci_cov_params();
        let bse: Vec<f64> = (0..m).map(|i| cov[(i, i)].sqrt()).collect();
        let tvalues: Vec<f64> = (0..m)
            .map(|i| {
                if i == self.n_det {
                    f64::NAN
                } else {
                    params[i] / bse[i]
                }
            })
            .collect();
        let pvalues: Vec<f64> = tvalues
            .iter()
            .map(|t| 2.0 * (1.0 - norm_cdf(t.abs())))
            .collect();
        // use_t is True for UECM, so the interval uses the Student t quantile.
        let qt = t_ppf(1.0 - alpha / 2.0, self.reported_df_resid());
        let conf_int = (0..m)
            .map(|i| (params[i] - qt * bse[i], params[i] + qt * bse[i]))
            .collect();
        CointegratingVector {
            params,
            bse,
            tvalues,
            pvalues,
            conf_int,
        }
    }
}

pub struct BoundsTest {
    pub stat: f64,
    /// Rows are `CRIT_PERCENTILES`, columns are the I(0) then the I(1) bound.
    pub crit_vals: Vec<(f64, f64)>,
    pub p_lower: f64,
    pub p_upper: f64,
}

/// `_pss_pvalue`: a polynomial in `log(stat)` read through the normal cdf.
pub fn pss_pvalue(stat: f64, k: usize, case: usize, i1: bool) -> f64 {
    let threshold = pss_tables::stat_star(k, case, i1).expect("PSS key is tabulated");
    let coef = if stat > threshold {
        pss_tables::small_p(k, case, i1)
    } else {
        pss_tables::large_p(k, case, i1)
    }
    .expect("PSS key is tabulated");
    let log_stat = stat.ln();
    let value: f64 = coef
        .iter()
        .enumerate()
        .map(|(i, c)| c * log_stat.powi(i as i32))
        .sum();
    1.0 - norm_cdf(value)
}

/// `bounds_test(case)` with the asymptotic critical values.
pub fn bounds_test(model: &Uecm, case: usize) -> BoundsTest {
    let nvar = model.ardl_order.len();
    let rest: Vec<usize> = match case {
        1 => (0..nvar).collect(),
        2 => (0..nvar + 1).collect(),
        3 => (1..nvar + 1).collect(),
        4 => (1..nvar + 2).collect(),
        5 => (2..nvar + 2).collect(),
        other => panic!("case must be 1 to 5, got {other}"),
    };
    let r = rest.len();
    let vcv = DMatrix::from_fn(r, r, |i, j| model.fit.cov_params[(rest[i], rest[j])]);
    let coef = DVector::from_fn(r, |i, _| model.fit.params[rest[i]]);
    let quad = (coef.transpose()
        * crate::linalg::inverse(&vcv).expect("restriction covariance")
        * &coef)[0];
    let stat = quad / r as f64;

    let k = nvar;
    let lower = pss_tables::crit_vals(k, case, false).expect("PSS key is tabulated");
    let upper = pss_tables::crit_vals(k, case, true).expect("PSS key is tabulated");
    BoundsTest {
        stat,
        crit_vals: lower.iter().zip(upper).map(|(&l, &u)| (l, u)).collect(),
        p_lower: pss_pvalue(stat, k, case, false),
        p_upper: pss_pvalue(stat, k, case, true),
    }
}

/// The whole 805 step: select the order, force both lags to at least one, fit, and report
/// the long-run effect of the exogenous variable with the sign convention the script uses.
pub struct LongRun {
    pub ar_lag: usize,
    pub dl_lag: usize,
    pub beta: f64,
    pub p_value: f64,
    pub ci_lo: f64,
    pub ci_hi: f64,
    pub bounds: BoundsTest,
}

pub fn ardl_long_run(y: &[f64], x: &[f64], maxlag: usize, case: usize) -> LongRun {
    let sel = ardl_select_order(y, maxlag, x, maxlag, "aic", Trend::Ct);
    let p = sel.ar_lag.max(1);
    let q = sel.dl_lag.unwrap_or(1).max(1);
    let model = uecm(y, p, x, q, Trend::Ct);
    let ci = model.cointegrating_vector(0.05);
    // ci_params is normalised on the outcome, so the effect is the negated coefficient and
    // the interval negates and swaps.
    let idx = model.n_det + 1;
    LongRun {
        ar_lag: p,
        dl_lag: q,
        beta: -ci.params[idx],
        p_value: ci.pvalues[idx],
        ci_lo: -ci.conf_int[idx].1,
        ci_hi: -ci.conf_int[idx].0,
        bounds: bounds_test(&model, case),
    }
}
