//! Zivot-Andrews structural-break unit-root test, ported 1:1 from statsmodels: ADF-chosen base
//! lags, normalised auxiliary regressions over every candidate break, interpolated p-values.

use crate::ols::Ols;
use crate::stationarity::adfuller_maxlag;
use crate::Regression;
use nalgebra::{DMatrix, DVector};

const ZA_C: [(f64, f64); 48] = [
    (0.001, -6.78442),
    (0.1, -5.83192),
    (0.2, -5.68139),
    (0.3, -5.58461),
    (0.4, -5.51308),
    (0.5, -5.45043),
    (0.6, -5.39924),
    (0.7, -5.36023),
    (0.8, -5.33219),
    (0.9, -5.30294),
    (1.0, -5.27644),
    (2.5, -5.0334),
    (5.0, -4.81067),
    (7.5, -4.67636),
    (10.0, -4.56618),
    (12.5, -4.4813),
    (15.0, -4.40507),
    (17.5, -4.33947),
    (20.0, -4.28155),
    (22.5, -4.22683),
    (25.0, -4.1783),
    (27.5, -4.13101),
    (30.0, -4.08586),
    (32.5, -4.04455),
    (35.0, -4.0038),
    (37.5, -3.96144),
    (40.0, -3.92078),
    (42.5, -3.88178),
    (45.0, -3.84503),
    (47.5, -3.80549),
    (50.0, -3.77031),
    (52.5, -3.73209),
    (55.0, -3.696),
    (57.5, -3.65985),
    (60.0, -3.62126),
    (65.0, -3.5458),
    (70.0, -3.46848),
    (75.0, -3.38533),
    (80.0, -3.29112),
    (85.0, -3.17832),
    (90.0, -3.04165),
    (92.5, -2.95146),
    (95.0, -2.83179),
    (96.0, -2.76465),
    (97.0, -2.68624),
    (98.0, -2.57884),
    (99.0, -2.40044),
    (99.9, -1.88932),
];
const ZA_T: [(f64, f64); 48] = [
    (0.001, -83.9094),
    (0.1, -13.8837),
    (0.2, -9.13205),
    (0.3, -6.32564),
    (0.4, -5.60803),
    (0.5, -5.38794),
    (0.6, -5.26585),
    (0.7, -5.18734),
    (0.8, -5.12756),
    (0.9, -5.07984),
    (1.0, -5.03421),
    (2.5, -4.65634),
    (5.0, -4.4058),
    (7.5, -4.25214),
    (10.0, -4.13678),
    (12.5, -4.03765),
    (15.0, -3.95185),
    (17.5, -3.87945),
    (20.0, -3.81295),
    (22.5, -3.75273),
    (25.0, -3.69836),
    (27.5, -3.64785),
    (30.0, -3.59819),
    (32.5, -3.55146),
    (35.0, -3.50522),
    (37.5, -3.45987),
    (40.0, -3.41672),
    (42.5, -3.37465),
    (45.0, -3.33394),
    (47.5, -3.29393),
    (50.0, -3.25316),
    (52.5, -3.21244),
    (55.0, -3.17124),
    (57.5, -3.13211),
    (60.0, -3.09204),
    (65.0, -3.01135),
    (70.0, -2.92897),
    (75.0, -2.83614),
    (80.0, -2.73893),
    (85.0, -2.6284),
    (90.0, -2.49611),
    (92.5, -2.41337),
    (95.0, -2.3082),
    (96.0, -2.25797),
    (97.0, -2.19648),
    (98.0, -2.1132),
    (99.0, -1.99138),
    (99.9, -1.67466),
];
const ZA_CT: [(f64, f64); 48] = [
    (0.001, -38.178),
    (0.1, -6.43107),
    (0.2, -6.07279),
    (0.3, -5.95496),
    (0.4, -5.86254),
    (0.5, -5.77081),
    (0.6, -5.72541),
    (0.7, -5.68406),
    (0.8, -5.65163),
    (0.9, -5.60419),
    (1.0, -5.57556),
    (2.5, -5.29704),
    (5.0, -5.07332),
    (7.5, -4.93003),
    (10.0, -4.82668),
    (12.5, -4.73711),
    (15.0, -4.6602),
    (17.5, -4.5897),
    (20.0, -4.52855),
    (22.5, -4.471),
    (25.0, -4.42011),
    (27.5, -4.37387),
    (30.0, -4.32705),
    (32.5, -4.28126),
    (35.0, -4.23793),
    (37.5, -4.19822),
    (40.0, -4.158),
    (42.5, -4.11946),
    (45.0, -4.08064),
    (47.5, -4.04286),
    (50.0, -4.00489),
    (52.5, -3.96837),
    (55.0, -3.932),
    (57.5, -3.89496),
    (60.0, -3.85577),
    (65.0, -3.77795),
    (70.0, -3.69794),
    (75.0, -3.61852),
    (80.0, -3.52485),
    (85.0, -3.41665),
    (90.0, -3.28527),
    (92.5, -3.19724),
    (95.0, -3.08769),
    (96.0, -3.03088),
    (97.0, -2.96091),
    (98.0, -2.85581),
    (99.0, -2.71015),
    (99.9, -2.28767),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ZaModel {
    C,
    T,
    Ct,
}

pub struct ZaResult {
    pub stat: f64,
    pub pvalue: f64,
    /// 1%, 5%, 10%.
    pub crit: [f64; 3],
    pub baselags: usize,
    pub bpidx: usize,
}

fn interp(x: f64, xp: &[f64], fp: &[f64]) -> f64 {
    if x <= xp[0] {
        return fp[0];
    }
    if x >= xp[xp.len() - 1] {
        return fp[fp.len() - 1];
    }
    let j = xp.partition_point(|v| *v <= x) - 1;
    let slope = (fp[j + 1] - fp[j]) / (xp[j + 1] - xp[j]);
    fp[j] + slope * (x - xp[j])
}

/// zivot_andrews(x, trim=0.15, maxlag, regression, autolag="AIC").
pub fn zivot_andrews(x: &[f64], maxlag: Option<usize>, model: ZaModel) -> ZaResult {
    try_zivot_andrews(x, maxlag, model).expect("Zivot–Andrews auxiliary regression")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZaError {
    RankDeficient,
    DecompositionFailed,
}

/// Checked entry point for callers that must report a refused auxiliary fit.
pub fn try_zivot_andrews(x: &[f64], maxlag: Option<usize>, model: ZaModel) -> Result<ZaResult, ZaError> {
    let trim = 0.15;
    let nobs = x.len();
    let adf_reg = Regression::Ct;
    let baselags = adfuller_maxlag(x, adf_reg, maxlag).usedlag;

    let trimcnt = (nobs as f64 * trim) as usize;
    let start_period = trimcnt;
    let end_period = nobs - trimcnt;
    let basecols = if model == ZaModel::Ct { 5 } else { 4 };

    let c_const = 1.0 / (nobs as f64).sqrt();
    let t_const: Vec<f64> = (1..=nobs + 1)
        .map(|v| v as f64 * 3f64.sqrt() / (nobs as f64).powf(1.5))
        .collect();

    // First-difference and normalise endog and the level series.
    let mut endog: Vec<f64> = x.windows(2).map(|w| w[1] - w[0]).collect();
    let enorm = endog.iter().map(|v| v * v).sum::<f64>().sqrt();
    for v in &mut endog {
        *v /= enorm;
    }
    let snorm = x.iter().map(|v| v * v).sum::<f64>().sqrt();
    let series: Vec<f64> = x.iter().map(|v| v / snorm).collect();

    let rows = endog.len() - baselags;
    let cols = basecols + baselags;
    let mut exog = DMatrix::<f64>::zeros(rows, cols);
    for t in 0..rows {
        exog[(t, 0)] = c_const;
        exog[(t, basecols - 1)] = series[baselags + t];
        for lag in 1..=baselags {
            exog[(t, basecols - 1 + lag)] = endog[baselags + t - lag];
        }
    }
    let y = DVector::from_iterator(rows, endog[baselags..].iter().copied());

    let mut best_stat = f64::INFINITY;
    let mut best_bp = 0usize;
    for bp in start_period + 1..=end_period {
        let cutoff = bp - (baselags + 1);
        match model {
            ZaModel::C | ZaModel::Ct => {
                for t in 0..rows {
                    exog[(t, 1)] = if t < cutoff { 0.0 } else { c_const };
                    exog[(t, 2)] = t_const[baselags + 1 + t];
                }
                if model == ZaModel::Ct {
                    for t in 0..rows {
                        exog[(t, 3)] = if t < cutoff { 0.0 } else { t_const[t - cutoff] };
                    }
                }
            }
            ZaModel::T => {
                for t in 0..rows {
                    exog[(t, 1)] = t_const[baselags + 1 + t];
                    exog[(t, 2)] = if t + 1 < cutoff {
                        0.0
                    } else {
                        t_const[t + 1 - cutoff]
                    };
                }
            }
        }
        let stat = if bp == start_period + 1 {
            let fit = Ols::try_fit(&exog, &y).map_err(|_| ZaError::DecompositionFailed)?;
            if fit.rank < cols {
                return Err(ZaError::RankDeficient);
            }
            fit.tvalues()[basecols - 1]
        } else {
            quick_ols(&exog, &y)?[basecols - 1]
        };
        if stat < best_stat {
            best_stat = stat;
            best_bp = bp;
        }
    }

    let table: &[(f64, f64)] = match model {
        ZaModel::C => &ZA_C,
        ZaModel::T => &ZA_T,
        ZaModel::Ct => &ZA_CT,
    };
    let pcnts: Vec<f64> = table.iter().map(|r| r.0).collect();
    let stats: Vec<f64> = table.iter().map(|r| r.1).collect();
    let pvalue = interp(best_stat, &stats, &pcnts) / 100.0;
    let crit = [
        interp(1.0, &pcnts, &stats),
        interp(5.0, &pcnts, &stats),
        interp(10.0, &pcnts, &stats),
    ];

    Ok(ZaResult {
        stat: best_stat,
        pvalue,
        crit,
        baselags,
        bpidx: best_bp - 1,
    })
}

/// Statsmodels ZivotAndrewsUnitRoot._quick_ols; the first candidate checks rank.
fn quick_ols(x: &DMatrix<f64>, y: &DVector<f64>) -> Result<DVector<f64>, ZaError> {
    let inverse = crate::linalg::inverse(&(x.transpose() * x))
        .map_err(|_| ZaError::DecompositionFailed)?;
    let coefficients = &inverse * (x.transpose() * y);
    let residuals = y - x * &coefficients;
    let variance = residuals.dot(&residuals) / (x.nrows() - x.ncols()) as f64;
    Ok(DVector::from_iterator(x.ncols(), (0..x.ncols()).map(|i| {
        coefficients[i] / (variance * inverse[(i, i)]).sqrt()
    })))
}
