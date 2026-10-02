//! Browser contracts over the parity-tested kernels. No alternate estimator.
use hirmos_causal_core::{
    augmented_synth as ridge, estimation::inference, sun_abraham as sa, sun_abraham_fit as sun,
};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SunRequest {
    rows: usize,
    columns: usize,
    outcome: usize,
    units: Vec<String>,
    times: Vec<i64>,
    adoption: Vec<Option<i64>>,
    reference_periods: Vec<i64>,
    reference_cohorts: Vec<i64>,
    confidence: f64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Interval {
    estimate: f64,
    standard_error: f64,
    p_value: f64,
    lower: f64,
    upper: f64,
}
#[derive(Serialize)]
pub(crate) struct Cell {
    cohort: i64,
    event: i64,
    support: usize,
    interval: Interval,
}
#[derive(Serialize)]
pub(crate) struct Summary {
    key: i64,
    support: usize,
    interval: Interval,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SunEvidence {
    kind: &'static str,
    times: Vec<i64>,
    request: SunRequest,
    observations: usize,
    clusters: usize,
    retained_rows: Vec<usize>,
    removed_rows: Vec<usize>,
    omitted: Vec<(i64, i64)>,
    cells: Vec<Cell>,
    events: Vec<Summary>,
    cohorts: Vec<Summary>,
    overall: Interval,
    covariance: Vec<Vec<f64>>,
}
fn matrix(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    units: &[String],
    times: &[i64],
) -> Result<DMatrix<f64>, String> {
    if rows == 0
        || columns == 0
        || rows.checked_mul(columns) != Some(values.len())
        || outcome >= columns
        || units.len() != rows
        || times.len() != rows
        || units.iter().any(|u| u.trim().is_empty())
        || values.iter().any(|v| !v.is_finite())
        || times.iter().any(|t| t.unsigned_abs() > 9007199254740991)
    {
        return Err("The finite outcome matrix and panel keys must describe the same rows.".into());
    }
    if units.iter().zip(times).collect::<BTreeSet<_>>().len() != rows {
        return Err("Each unit and period must identify one observation.".into());
    }
    Ok(DMatrix::from_column_slice(rows, columns, values))
}
fn interval(beta: f64, variance: f64, df: f64, confidence: f64) -> Result<Interval, String> {
    let c = inference::contrast(
        &[beta],
        &DMatrix::from_element(1, 1, variance),
        &[1.],
        0.,
        confidence,
        inference::Reference::Student {
            degrees_of_freedom: df,
        },
    )
    .map_err(|_| "The estimate's clustered uncertainty could not be calculated.".to_string())?;
    Ok(Interval {
        estimate: c.estimate,
        standard_error: c.standard_error,
        p_value: c.p_value,
        lower: c.interval[0],
        upper: c.interval[1],
    })
}
pub(crate) fn sun_run(values: &[f64], r: SunRequest) -> Result<SunEvidence, String> {
    let m = matrix(values, r.rows, r.columns, r.outcome, &r.units, &r.times)?;
    if r.adoption.len() != r.rows
        || r.reference_periods.is_empty()
        || !(0. < r.confidence && r.confidence < 1.)
        || r.adoption
            .iter()
            .flatten()
            .chain(&r.reference_periods)
            .chain(&r.reference_cohorts)
            .any(|t| t.unsigned_abs() > 9007199254740991)
        || r.reference_periods.iter().collect::<BTreeSet<_>>().len() != r.reference_periods.len()
        || r.reference_cohorts.iter().collect::<BTreeSet<_>>().len() != r.reference_cohorts.len()
    {
        return Err("Provide adoption for each row, a reference period and a confidence level between zero and one.".into());
    }
    let mut labels: Vec<_> = r
        .units
        .iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    // Numeric unit labels retain numeric order in the shared fitter's canonical
    // accumulation. Sorting "1", "10", "2" lexically needlessly changes rounding
    // from the numeric-panel oracle. Distinct labels remain distinct, including
    // "01" and "1"; textual or mixed identifiers use the existing lexical order.
    if labels.iter().all(|label| label.parse::<f64>().is_ok_and(|v| v.is_finite())) {
        labels.sort_by(|a, b| a.parse::<f64>().unwrap().total_cmp(&b.parse::<f64>().unwrap()).then(a.cmp(b)));
    }
    let unit_codes: std::collections::BTreeMap<_, _> = labels.iter().enumerate().map(|(i, label)| (*label, i as u64)).collect();
    let units: Vec<_> = r.units.iter().map(|u| unit_codes[u]).collect();
    let cohorts: Vec<_> = r.adoption.iter().map(|g| g.unwrap_or(i64::MIN)).collect();
    // A specified cohort must have an observed adoption period; otherwise the
    // package treats it as never treated. Refuse that silent reinterpretation.
    let periods: BTreeSet<_> = r.times.iter().copied().collect();
    if r.adoption.iter().flatten().any(|g| !periods.contains(g)) {
        return Err("Each recorded adoption period must occur in the panel's time axis.".into());
    }
    let y: Vec<_> = m.column(r.outcome).iter().copied().collect();
    let fit=sun::fit(&units,&r.times,&cohorts,&y,&r.reference_periods,&r.reference_cohorts).map_err(|e|match e {
        sun::Error::DuplicatePanelKey=>"Each unit and period must identify one observation.",
        sun::Error::InconsistentCohort=>"A unit must have the same recorded adoption period on every row.",
        sun::Error::DegreesOfFreedom=>"The retained cohort-by-event-time design needs positive residual degrees of freedom.",
        sun::Error::Singular=>"The fixed effects absorb every supported interaction.",
        sun::Error::Shape=>"The outcome and panel keys must have the same row count.",
        sun::Error::Design(_)=>"The selected references leave no supported cohort-by-event-time design.",
        sun::Error::Regression(_)=>"The retained panel needs estimable interactions and at least two independent units.",
    }.to_string())?;
    let clusters = fit
        .retained_rows
        .iter()
        .map(|i| &r.units[*i])
        .collect::<BTreeSet<_>>()
        .len();
    if clusters < 2 {
        return Err("Clustered uncertainty requires at least two retained units.".into());
    }
    let df = (clusters - 1) as f64;
    let aggregate = |target| -> Result<Interval, String> {
        let c = sa::aggregate(
            &fit.cells,
            &fit.coefficients,
            &fit.covariance,
            &fit.support,
            target,
        )
        .map_err(|_| "This aggregation has no supported treatment comparisons.".to_string())?;
        interval(c.estimate, c.variance, df, r.confidence)
    };
    let cells = fit
        .cells
        .iter()
        .enumerate()
        .map(|(j, c)| {
            Ok(Cell {
                cohort: c.cohort,
                event: c.event,
                support: fit.support[j] as usize,
                interval: interval(
                    fit.coefficients[j],
                    fit.covariance[(j, j)],
                    df,
                    r.confidence,
                )?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let events = fit
        .cells
        .iter()
        .map(|c| c.event)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|key| {
            Ok(Summary {
                key,
                support: fit
                    .cells
                    .iter()
                    .zip(&fit.support)
                    .filter(|(c, _)| c.event == key)
                    .map(|(_, s)| *s as usize)
                    .sum(),
                interval: aggregate(sa::Target::Event(key))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let cohorts = fit
        .cells
        .iter()
        .filter(|c| c.event >= 0)
        .map(|c| c.cohort)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|key| {
            Ok(Summary {
                key,
                support: fit
                    .cells
                    .iter()
                    .zip(&fit.support)
                    .filter(|(c, _)| c.cohort == key && c.event >= 0)
                    .map(|(_, s)| *s as usize)
                    .sum(),
                interval: aggregate(sa::Target::Cohort(key))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let overall = aggregate(sa::Target::OverallAtt)?;
    let covariance = (0..fit.cells.len())
        .map(|i| {
            (0..fit.cells.len())
                .map(|j| fit.covariance[(i, j)])
                .collect()
        })
        .collect();
    Ok(SunEvidence {
        kind: "sunAbraham",
        times: r.times.iter().copied().collect::<BTreeSet<_>>().into_iter().collect(),
        request: r,
        observations: fit.retained_rows.len(),
        clusters,
        retained_rows: fit.retained_rows,
        removed_rows: fit.removed_rows,
        omitted: fit
            .dropped_cells
            .iter()
            .map(|c| (c.cohort, c.event))
            .collect(),
        cells,
        events,
        cohorts,
        overall,
        covariance,
    })
}
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Selection {
    MinimumError,
    OneStandardError,
}
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum Regularization {
    Fixed {
        lambda: f64,
    },
    CrossValidation {
        maximum: Option<f64>,
        minimum_ratio: f64,
        steps: usize,
        holdout_length: usize,
        selection: Selection,
    },
}
#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Uncertainty {
    None {},
    JackknifePlus { confidence: f64 },
    Conservative { confidence: f64 },
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RidgeRequest {
    rows: usize,
    columns: usize,
    outcome: usize,
    units: Vec<String>,
    times: Vec<i64>,
    treated: String,
    donors: Vec<String>,
    pre_periods: usize,
    regularization: Regularization,
    uncertainty: Uncertainty,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CvEvidence {
    candidates: Vec<f64>,
    errors: Vec<f64>,
    standard_errors: Vec<f64>,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum Bounds {
    None {},
    Jackknife {
        lower: Vec<f64>,
        upper: Vec<f64>,
        average_lower: f64,
        average_upper: f64,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RidgeEvidence {
    request: RidgeRequest,
    periods: Vec<i64>,
    observed: Vec<f64>,
    synthetic: Vec<f64>,
    gaps: Vec<f64>,
    donor_weights: Vec<f64>,
    baseline_weights: Vec<f64>,
    lambda: f64,
    tuning: Option<CvEvidence>,
    average: f64,
    bounds: Bounds,
}
pub(crate) fn ridge_run(values: &[f64], r: RidgeRequest) -> Result<RidgeEvidence, String> {
    let m = matrix(values, r.rows, r.columns, r.outcome, &r.units, &r.times)?;
    if r.treated.is_empty()
        || r.donors.len() < 2
        || r.donors.iter().any(|u| u.is_empty() || u == &r.treated)
        || r.donors.iter().collect::<BTreeSet<_>>().len() != r.donors.len()
    {
        return Err("Choose one treated unit and at least two distinct donor units.".into());
    }
    let periods: Vec<_> = r
        .units
        .iter()
        .zip(&r.times)
        .filter(|(u, _)| *u == &r.treated)
        .map(|(_, t)| *t)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if r.pre_periods < 2 || r.pre_periods >= periods.len() {
        return Err(
            "Retain at least two pre-treatment periods and one post-treatment period.".into(),
        );
    }
    if periods
        .windows(2)
        .any(|p| p[1].checked_sub(p[0]) != Some(1))
    {
        return Err("The selected time axis must contain consecutive periods; missing periods cannot be compressed.".into());
    }
    let lookup: BTreeMap<_, _> = r
        .units
        .iter()
        .zip(&r.times)
        .enumerate()
        .map(|(i, (u, t))| ((u.as_str(), *t), i))
        .collect();
    let path = |unit: &str| {
        periods
            .iter()
            .map(|t| {
                lookup
                    .get(&(unit, *t))
                    .map(|i| m[(*i, r.outcome)])
                    .ok_or_else(|| format!("Unit {unit} has no observation at period {t}."))
            })
            .collect::<Result<Vec<_>, _>>()
    };
    let target = path(&r.treated)?;
    let donor_rows = r
        .donors
        .iter()
        .map(|u| path(u))
        .collect::<Result<Vec<_>, _>>()?;
    let donors = DMatrix::from_fn(r.donors.len(), periods.len(), |i, j| donor_rows[i][j]);
    let spec = match r.regularization {
        Regularization::Fixed { lambda } => ridge::Regularization::Fixed(lambda),
        Regularization::CrossValidation {
            maximum,
            minimum_ratio,
            steps,
            holdout_length,
            selection,
        } => ridge::Regularization::CrossValidation(ridge::Tuning {
            maximum,
            minimum_ratio,
            steps,
            holdout_length,
            selection: match selection {
                Selection::MinimumError => ridge::Selection::MinimumError,
                Selection::OneStandardError => ridge::Selection::OneStandardError,
            },
        }),
    };
    let describe = |e: ridge::Error| {
        match e {
            ridge::Error::InvalidLambda => {
                "Choose positive finite ridge regularization and a valid candidate grid."
            }
            ridge::Error::InvalidBoundary => {
                "The pre-treatment interval must support the selected held-out blocks."
            }
            ridge::Error::Optimizer => "The donor-weight optimization did not reach solved status.",
            ridge::Error::Singular => "The regularized donor system could not be solved.",
            ridge::Error::InvalidConfidence => "Choose a confidence level between zero and one.",
            ridge::Error::Empty
            | ridge::Error::Shape
            | ridge::Error::NonFinite
            | ridge::Error::InvalidWeights => {
                "The selected donor and treated series must be finite and aligned."
            }
        }
        .to_string()
    };
    let fit = ridge::fit(&donors, &target, r.pre_periods, spec).map_err(describe)?;
    let selected = match r.uncertainty {
        Uncertainty::None {} => None,
        Uncertainty::JackknifePlus { confidence } => Some((confidence, ridge::JackknifeRule::Plus)),
        Uncertainty::Conservative { confidence } => {
            Some((confidence, ridge::JackknifeRule::Conservative))
        }
    };
    let bounds = match selected {
        None => Bounds::None {},
        Some((confidence, rule)) => {
            if !(0. < confidence && confidence < 1.) {
                return Err("Choose a confidence level between zero and one.".into());
            }
            let mut b =
                ridge::jackknife_plus(&donors, &target, r.pre_periods, spec, 1. - confidence, rule)
                    .map_err(describe)?;
            let average_lower = b.lower.pop().ok_or("The average interval is missing.")?;
            let average_upper = b.upper.pop().ok_or("The average interval is missing.")?;
            Bounds::Jackknife {
                lower: b.lower,
                upper: b.upper,
                average_lower,
                average_upper,
            }
        }
    };
    let average =
        fit.effects[r.pre_periods..].iter().sum::<f64>() / (periods.len() - r.pre_periods) as f64;
    Ok(RidgeEvidence {
        request: r,
        periods,
        observed: target,
        synthetic: fit.prediction,
        gaps: fit.effects,
        donor_weights: fit.weights,
        baseline_weights: fit.baseline,
        lambda: fit.lambda,
        tuning: fit.tuning.map(|c| CvEvidence {
            candidates: c.candidates,
            errors: c.errors,
            standard_errors: c.standard_errors,
        }),
        average,
        bounds,
    })
}
