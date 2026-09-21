//! Interrupted time series regression as Lopez Bernal, Cummins and Gasparrini set it out
//! (International Journal of Epidemiology 2017, with the 2020 corrigendum): a segmented
//! regression of one series on time, an intervention indicator, an optional change of slope
//! centred on the last pre-intervention row, and the harmonic seasonal terms of
//! `tsModel::harmonic`. A count is fitted as the paper's quasi-Poisson model with an offset;
//! a continuous series by OLS with Newey-West errors.

use crate::estimation::{ols_hac, HacOls};
use crate::glm::{poisson_glm_with, Dispersion, PoissonGlm};
use crate::tsdiag::ljung_box;
use nalgebra::DMatrix;

/// `tsModel::harmonic(x, nfreq, period)`: for each frequency `k` of `1..=nfreq` the columns
/// `sin(x k 2π / period)`, then the cosines in the same order.
pub fn harmonic(x: &[f64], nfreq: usize, period: f64) -> DMatrix<f64> {
    assert!(nfreq > 0, "nfreq > 0");
    let mut out = DMatrix::<f64>::zeros(x.len(), 2 * nfreq);
    for (row, &value) in x.iter().enumerate() {
        for k in 1..=nfreq {
            let angle = value * k as f64 * 2.0 * std::f64::consts::PI / period;
            out[(row, k - 1)] = angle.sin();
            out[(row, nfreq + k - 1)] = angle.cos();
        }
    }
    out
}

/// How the intervention is assumed to act, the impact model the paper asks for in advance.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImpactModel {
    /// A step: the level changes at the intervention and stays changed.
    Level,
    /// A step and a change of slope, the paper's additional material (model 4).
    LevelAndSlope,
    /// A change of slope only.
    Slope,
    /// A step that ends at `until` (exclusive row index), a temporary level change.
    TemporaryLevel { until: usize },
}

/// Harmonic seasonal terms: `pairs` sine and cosine pairs at `period`, over `phase`, the
/// position of each row in its cycle (the month number in the paper).
#[derive(Clone, Debug)]
pub struct Harmonics {
    pub pairs: usize,
    pub period: f64,
    pub phase: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct InterruptedSeriesDesign {
    /// The first row after the event, 0-based. Rows before it are the pre-period.
    pub intervention_row: usize,
    /// Rows after the intervention row before the change is assumed to start.
    pub lag: usize,
    pub impact: ImpactModel,
    pub harmonics: Option<Harmonics>,
}

/// The design matrix and the name of each column. Time is `1..=n`, as `time` in the paper's
/// data; the slope change is `(time - T0) * step` with `T0` the time of the last row before
/// the change, so it is 1 on the first changed row.
pub struct Design {
    pub x: DMatrix<f64>,
    pub names: Vec<String>,
    /// The same matrix with the intervention columns set to zero: the counterfactual.
    pub counterfactual: DMatrix<f64>,
    /// The matrix with every row's harmonic terms at one fixed phase, the paper's prediction "for
    /// the same month" that draws a deseasonalised trend; none without seasonal terms.
    pub deseasonalised: Option<DMatrix<f64>>,
    pub deseasonalised_counterfactual: Option<DMatrix<f64>>,
}

pub fn design(n: usize, spec: &InterruptedSeriesDesign) -> Design {
    let start = spec.intervention_row + spec.lag;
    let step = |row: usize| -> f64 {
        let on = match spec.impact {
            ImpactModel::TemporaryLevel { until } => row >= start && row < until,
            _ => row >= start,
        };
        if on { 1.0 } else { 0.0 }
    };
    let has_step = !matches!(spec.impact, ImpactModel::Slope);
    let has_slope = matches!(spec.impact, ImpactModel::LevelAndSlope | ImpactModel::Slope);
    let mut columns: Vec<(String, Vec<f64>, bool)> = Vec::new();
    columns.push(("const".into(), vec![1.0; n], false));
    columns.push(("time".into(), (1..=n).map(|t| t as f64).collect(), false));
    if has_step {
        columns.push(("step".into(), (0..n).map(step).collect(), true));
    }
    if has_slope {
        // T0 is the time of the last pre-change row, `start` in the 1-based time scale.
        let t0 = start as f64;
        columns.push((
            "slope_change".into(),
            (0..n).map(|row| ((row + 1) as f64 - t0) * step(row)).collect(),
            true,
        ));
    }
    if let Some(seasonal) = &spec.harmonics {
        assert_eq!(seasonal.phase.len(), n, "one phase per row");
        let terms = harmonic(&seasonal.phase, seasonal.pairs, seasonal.period);
        for k in 1..=seasonal.pairs {
            columns.push((format!("sin{k}"), terms.column(k - 1).iter().copied().collect(), false));
        }
        for k in 1..=seasonal.pairs {
            columns.push((format!("cos{k}"), terms.column(seasonal.pairs + k - 1).iter().copied().collect(), false));
        }
    }
    let x = DMatrix::from_fn(n, columns.len(), |r, c| columns[c].1[r]);
    let counterfactual = DMatrix::from_fn(n, columns.len(), |r, c| if columns[c].2 { 0.0 } else { columns[c].1[r] });
    // The paper predicts every row at June, the middle of its 12-month period: here the phase
    // half a period in, whatever the period.
    let deseasonalised = spec.harmonics.as_ref().map(|seasonal| {
        let fixed = harmonic(&[seasonal.period / 2.0], seasonal.pairs, seasonal.period);
        let first = columns.len() - 2 * seasonal.pairs;
        let fix = |m: &DMatrix<f64>| DMatrix::from_fn(n, columns.len(), |r, c| if c >= first { fixed[(0, c - first)] } else { m[(r, c)] });
        (fix(&x), fix(&counterfactual))
    });
    let (deseasonalised, deseasonalised_counterfactual) = match deseasonalised {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };
    Design { x, names: columns.into_iter().map(|c| c.0).collect(), counterfactual, deseasonalised, deseasonalised_counterfactual }
}

/// `acf` and `pacf` of the residuals with their 95% limits, the paper's model check.
pub struct ResidualCorrelation {
    pub acf: Vec<f64>,
    pub acf_limits: Vec<f64>,
    pub pacf: Vec<f64>,
    pub pacf_limits: Vec<f64>,
}

fn residual_correlation(resid: &[f64], nlags: usize) -> ResidualCorrelation {
    let nlags = nlags.min(resid.len().saturating_sub(2)).max(1);
    let acf = crate::tsdiag::acf(resid, nlags);
    let pacf = crate::tsdiag::pacf_yw_mle(resid, nlags);
    let (acf_limits, pacf_limits) = crate::tsdiag::correlation_plot_limits(&acf, resid.len());
    ResidualCorrelation { acf, acf_limits, pacf, pacf_limits }
}

pub struct ContinuousFit {
    pub names: Vec<String>,
    pub fit: HacOls,
    pub fitted: Vec<f64>,
    pub counterfactual: Vec<f64>,
    pub deseasonalised: Option<Vec<f64>>,
    pub deseasonalised_counterfactual: Option<Vec<f64>>,
    /// Ljung-Box statistics and p-values of the residuals at lags `1..=max_lag`.
    pub ljung_box: (Vec<f64>, Vec<f64>),
    pub correlation: ResidualCorrelation,
}

/// The paper's design fitted to a continuous series by OLS with Newey-West errors at
/// `maxlags`, the usual choice where the outcome is not a count.
pub fn fit_continuous(y: &[f64], spec: &InterruptedSeriesDesign, maxlags: usize, ljung_box_lags: usize) -> ContinuousFit {
    let d = design(y.len(), spec);
    let fit = ols_hac(y, &d.x, maxlags);
    let predict = |m: &DMatrix<f64>| -> Vec<f64> {
        (0..m.nrows()).map(|r| (0..m.ncols()).map(|c| m[(r, c)] * fit.params[c]).sum()).collect()
    };
    let fitted = predict(&d.x);
    let counterfactual = predict(&d.counterfactual);
    let deseasonalised = d.deseasonalised.as_ref().map(&predict);
    let deseasonalised_counterfactual = d.deseasonalised_counterfactual.as_ref().map(&predict);
    let ljung_box = ljung_box(&fit.resid, ljung_box_lags);
    let correlation = residual_correlation(&fit.resid, ljung_box_lags);
    ContinuousFit { names: d.names, fit, fitted, counterfactual, deseasonalised, deseasonalised_counterfactual, ljung_box, correlation }
}

pub struct CountFit {
    pub names: Vec<String>,
    pub fit: PoissonGlm,
    /// The mean under the fitted model and under the counterfactual, on the count scale.
    pub fitted: Vec<f64>,
    pub counterfactual: Vec<f64>,
    /// The same predictions at the mean exposure, the paper's `predict(..., stdpop = mean(stdpop))`:
    /// the count standardised to one exposure, so rows are comparable.
    pub standardised: Vec<f64>,
    pub standardised_counterfactual: Vec<f64>,
    pub deseasonalised: Option<Vec<f64>>,
    pub deseasonalised_counterfactual: Option<Vec<f64>>,
    /// `exp(coef)` with normal 95% limits, `ci.lin(model, Exp = TRUE)` in the paper's code.
    pub rate_ratios: Vec<(f64, f64, f64)>,
    pub ljung_box: (Vec<f64>, Vec<f64>),
    pub correlation: ResidualCorrelation,
}

/// The paper's quasi-Poisson model: a count on the design with `log(exposure)` as the
/// offset, the dispersion estimated by Pearson's chi-square (models 2 to 4 of the code).
pub fn fit_count(y: &[f64], exposure: Option<&[f64]>, spec: &InterruptedSeriesDesign, ljung_box_lags: usize) -> CountFit {
    let d = design(y.len(), spec);
    let offset: Option<Vec<f64>> = exposure.map(|e| e.iter().map(|v| v.ln()).collect());
    let fit = poisson_glm_with(y, &d.x, offset.as_deref(), Dispersion::Pearson);
    let predict = |m: &DMatrix<f64>| -> Vec<f64> {
        (0..m.nrows())
            .map(|r| {
                let eta: f64 = (0..m.ncols()).map(|c| m[(r, c)] * fit.params[c]).sum();
                (eta + offset.as_ref().map_or(0.0, |o| o[r])).exp()
            })
            .collect()
    };
    let z975 = spec_math::cephes64::ndtri(0.975);
    let rate_ratios = fit
        .params
        .iter()
        .zip(&fit.bse)
        .map(|(&b, &se)| (b.exp(), (b - z975 * se).exp(), (b + z975 * se).exp()))
        .collect();
    let fitted = predict(&d.x);
    let counterfactual = predict(&d.counterfactual);
    let mean_offset = offset.as_ref().map_or(0.0, |o| (o.iter().map(|v| v.exp()).sum::<f64>() / o.len() as f64).ln());
    let standardise = |m: &DMatrix<f64>| -> Vec<f64> {
        (0..m.nrows())
            .map(|r| ((0..m.ncols()).map(|c| m[(r, c)] * fit.params[c]).sum::<f64>() + mean_offset).exp())
            .collect()
    };
    let standardised = standardise(&d.x);
    let standardised_counterfactual = standardise(&d.counterfactual);
    let deseasonalised = d.deseasonalised.as_ref().map(&standardise);
    let deseasonalised_counterfactual = d.deseasonalised_counterfactual.as_ref().map(&standardise);
    let ljung_box = ljung_box(&fit.resid_deviance, ljung_box_lags);
    let correlation = residual_correlation(&fit.resid_deviance, ljung_box_lags);
    CountFit { names: d.names, fit, fitted, counterfactual, standardised, standardised_counterfactual, deseasonalised, deseasonalised_counterfactual, rate_ratios, ljung_box, correlation }
}
