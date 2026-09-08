//! ADF and KPSS, ported 1:1 from statsmodels/tsa/stattools/_stattools.py (adfuller with autolag
//! AIC, kpss with Hobijn auto nlags).

use crate::mackinnon::{mackinnoncrit, mackinnonp, Regression};
use crate::ols::Ols;
use nalgebra::{DMatrix, DVector};

pub struct AdfResult {
    pub stat: f64,
    pub pvalue: f64,
    pub usedlag: usize,
    pub nobs: usize,
    /// 1%, 5%, 10%.
    pub crit: [f64; 3],
}

fn ntrend(regression: Regression) -> usize {
    match regression {
        Regression::N => 0,
        Regression::C => 1,
        Regression::Ct => 2,
    }
}

/// Rows of the ADF design for a given lag: [trend block][level][diff lags], trend prepended when
/// `prepend`, appended otherwise, exactly as add_trend is called in the two adfuller phases.
fn adf_design(
    x: &[f64],
    lag: usize,
    regression: Regression,
    prepend: bool,
) -> (DMatrix<f64>, DVector<f64>) {
    let n = x.len();
    let rows = n - 1 - lag;
    let columns = ntrend(regression) + 1 + lag;
    let mut design = DMatrix::<f64>::zeros(rows, columns);
    let mut y = DVector::<f64>::zeros(rows);
    for i in 0..rows {
        let base = lag + i;
        y[i] = x[base + 1] - x[base];
        let mut column = 0;
        let mut push = |design: &mut DMatrix<f64>, value: f64| {
            design[(i, column)] = value;
            column += 1;
        };
        if prepend && regression != Regression::N {
            push(&mut design, 1.0);
            if regression == Regression::Ct {
                push(&mut design, (i + 1) as f64);
            }
        }
        push(&mut design, x[base]);
        for j in 1..=lag {
            push(&mut design, x[base + 1 - j] - x[base - j]);
        }
        if !prepend && regression != Regression::N {
            push(&mut design, 1.0);
            if regression == Regression::Ct {
                push(&mut design, (i + 1) as f64);
            }
        }
    }
    (design, y)
}

pub fn adfuller(x: &[f64], regression: Regression) -> AdfResult {
    adfuller_maxlag(x, regression, None)
}

pub fn adfuller_maxlag(x: &[f64], regression: Regression, maxlag: Option<usize>) -> AdfResult {
    let n = x.len();
    let trend = ntrend(regression);
    let schwert = (12.0 * (n as f64 / 100.0).powf(0.25)).ceil() as usize;
    let maxlag = maxlag.unwrap_or_else(|| schwert.min(n / 2 - trend - 1));

    // Lag search: statsmodels fits one default-pseudoinverse OLS model for every prefix.
    let startlag = trend + 1;
    let (full, y) = adf_design(x, maxlag, regression, true);
    let mut best: Option<(f64, usize)> = None;
    for lag_columns in startlag..=startlag + maxlag {
        let design = full.columns(0, lag_columns).into_owned();
        let aic = Ols::fit(&design, &y).aic();
        if best.is_none_or(|(best_aic, _)| aic < best_aic) {
            best = Some((aic, lag_columns));
        }
    }
    let usedlag = best.expect("at least one lag fits").1 - startlag;

    // Final regression on the sample trimmed to the chosen lag, trend appended.
    let (design, y) = adf_design(x, usedlag, regression, false);
    let nobs = design.nrows();
    let stat = Ols::fit(&design, &y).tvalues()[0];
    AdfResult {
        stat,
        pvalue: mackinnonp(stat, regression),
        usedlag,
        nobs,
        crit: mackinnoncrit(regression, nobs),
    }
}

pub struct KpssResult {
    pub stat: f64,
    pub pvalue: f64,
    pub nlags: usize,
    /// 10%, 5%, 2.5%, 1%.
    pub crit: [f64; 4],
}

const KPSS_CRIT_C: [f64; 4] = [0.347, 0.463, 0.574, 0.739];
const KPSS_CRIT_CT: [f64; 4] = [0.119, 0.146, 0.176, 0.216];
const KPSS_PVALS: [f64; 4] = [0.10, 0.05, 0.025, 0.01];

fn kpss_autolag(resids: &[f64], nobs: usize) -> usize {
    let covlags = (nobs as f64).powf(2.0 / 9.0) as usize;
    let mut s0: f64 = resids.iter().map(|r| r * r).sum::<f64>() / nobs as f64;
    let mut s1 = 0.0;
    for i in 1..=covlags {
        let prod: f64 =
            (i..nobs).map(|t| resids[t] * resids[t - i]).sum::<f64>() / (nobs as f64 / 2.0);
        s0 += prod;
        s1 += i as f64 * prod;
    }
    let s_hat = s1 / s0;
    let gamma_hat = 1.1447 * (s_hat * s_hat).powf(1.0 / 3.0);
    (gamma_hat * (nobs as f64).powf(1.0 / 3.0)) as usize
}

fn sigma_est(resids: &[f64], nobs: usize, lags: usize) -> f64 {
    let mut s_hat: f64 = resids.iter().map(|r| r * r).sum();
    for i in 1..=lags {
        let prod: f64 = (i..nobs).map(|t| resids[t] * resids[t - i]).sum();
        s_hat += 2.0 * prod * (1.0 - i as f64 / (lags as f64 + 1.0));
    }
    s_hat / nobs as f64
}

/// np.interp with the same end clamping the oracle relies on.
fn interp(x: f64, xp: &[f64; 4], fp: &[f64; 4]) -> f64 {
    if x <= xp[0] {
        return fp[0];
    }
    if x >= xp[3] {
        return fp[3];
    }
    for i in 0..3 {
        if x <= xp[i + 1] {
            let t = (x - xp[i]) / (xp[i + 1] - xp[i]);
            return fp[i] + t * (fp[i + 1] - fp[i]);
        }
    }
    fp[3]
}

pub fn kpss(x: &[f64], regression: Regression) -> KpssResult {
    let nobs = x.len();
    let (resids, crit) = match regression {
        Regression::N => panic!("kpss supports only c and ct regressions"),
        Regression::C => {
            let mean = x.iter().sum::<f64>() / nobs as f64;
            (
                x.iter().map(|v| v - mean).collect::<Vec<f64>>(),
                KPSS_CRIT_C,
            )
        }
        Regression::Ct => {
            let mut design = DMatrix::<f64>::zeros(nobs, 2);
            for i in 0..nobs {
                design[(i, 0)] = 1.0;
                design[(i, 1)] = (i + 1) as f64;
            }
            let y = DVector::from_column_slice(x);
            let fit = Ols::fit(&design, &y);
            (fit.resid.iter().copied().collect(), KPSS_CRIT_CT)
        }
    };
    let nlags = kpss_autolag(&resids, nobs).min(nobs - 1);
    let mut cumsum = 0.0;
    let mut eta = 0.0;
    for r in &resids {
        cumsum += r;
        eta += cumsum * cumsum;
    }
    eta /= (nobs * nobs) as f64;
    let stat = eta / sigma_est(&resids, nobs, nlags);
    KpssResult {
        stat,
        pvalue: interp(stat, &crit, &KPSS_PVALS),
        nlags,
        crit,
    }
}
