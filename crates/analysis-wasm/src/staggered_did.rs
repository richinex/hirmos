//! Browser boundary for balanced-panel cohort-time ATT estimation.
use hirmos_causal_core::staggered_did::{self as did, inference as ci};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Controls { NeverTreated, NotYetTreated }
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Baseline { Varying, Universal }
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Inference {
    Analytical,
    BootstrapPointwise { iterations: usize, seed: u32 },
    BootstrapSimultaneous { iterations: usize, seed: u32 },
}
impl Inference {
    fn method(self) -> ci::Method {
        match self {
            Self::Analytical => ci::Method::Analytical,
            Self::BootstrapPointwise { iterations, seed } => ci::Method::BootstrapPointwise { iterations, seed },
            Self::BootstrapSimultaneous { iterations, seed } => ci::Method::BootstrapSimultaneous { iterations, seed },
        }
    }
    fn pointwise(self) -> Self {
        match self {
            Self::BootstrapSimultaneous { iterations, seed } => Self::BootstrapPointwise { iterations, seed },
            other => other,
        }
    }
}
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Specification {
    controls: Controls,
    baseline: Baseline,
    anticipation: u32,
    first_event: Option<i64>,
    last_event: Option<i64>,
    balance: Option<u32>,
    confidence: f64,
    inference: Inference,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    rows: usize,
    columns: usize,
    units: Vec<String>,
    times: Vec<i64>,
    /// One first-treatment period per row. Null means never treated.
    adoption: Vec<Option<i64>>,
    weights: Option<Vec<f64>>,
    clusters: Option<Vec<String>>,
    specification: Specification,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Interval {
    Reference,
    Unavailable { estimate: f64 },
    Estimated { estimate: f64, standard_error: f64, lower: f64, upper: f64 },
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Coverage { Pointwise, Simultaneous { critical: f64, large_critical: bool } }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Family<K = i64> {
    keys: Vec<K>,
    intervals: Vec<Interval>,
    coverage: Coverage,
    /// Analytical influence-function covariance, not bootstrap covariance.
    analytical_covariance: Vec<Vec<f64>>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Support { event: i64, cohorts: Vec<i64>, treated_units: usize, reference_cohorts: Vec<i64> }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Change {
    BeyondObservedWindow { unit: String, adoption: i64 },
    LatestCohortAsComparison { unit: String, adoption: i64 },
    NoUntreatedComparison { period: i64 },
    NoPreTreatment { unit: String },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SmallCohort { adoption: Option<i64>, count: usize, required: usize }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum FitStatus { Converged { iterations: usize }, IterationLimit }
#[derive(Serialize)]
struct Fit { cohort: i64, period: i64, status: FitStatus }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    version: u8,
    observations: usize,
    retained_observations: usize,
    units: Vec<String>,
    times: Vec<i64>,
    covariates: usize,
    weighted: bool,
    cluster_count: usize,
    specification: Specification,
    changes: Vec<Change>,
    small_cohorts: Vec<SmallCohort>,
    fits: Vec<Fit>,
    events: Family,
    cohorts: Family,
    calendar: Family,
    cells: Family<[i64; 2]>,
    support: Vec<Support>,
    /// Headline intervals are pointwise; simultaneous coverage belongs to each displayed family.
    overall: Overall,
}
#[derive(Serialize)]
struct Overall { dynamic: Interval, group: Interval, calendar: Interval, simple: Interval }

fn interval(value: ci::Interval) -> Interval {
    match value {
        ci::Interval::Reference => Interval::Reference,
        ci::Interval::Unavailable { att } => Interval::Unavailable { estimate: att },
        ci::Interval::Estimated { att, se, lower, upper } => Interval::Estimated { estimate: att, standard_error: se, lower, upper },
    }
}
fn infer(points: &[ci::Point<'_>], clusters: &[u64], spec: &Specification, method: Inference) -> Result<ci::Inference, String> {
    let confidence = ci::Confidence::new(spec.confidence).map_err(|_| "Confidence must be strictly between zero and one.")?;
    ci::infer(points, clusters, confidence, method.method()).map_err(|error| match error {
        ci::Error::AnalyticalClustering => "Additional clustering requires multiplier-bootstrap inference.",
        ci::Error::UnavailableSimultaneousBand => "A simultaneous band could not be estimated. No pointwise interval was substituted.",
        ci::Error::BandNarrowerThanPointwise => "The estimated simultaneous critical value is below the pointwise critical value. No interval was substituted.",
        ci::Error::Bootstrap(_) => "Multiplier-bootstrap inference failed. Check replications and influence-function values.",
        ci::Error::Confidence => "Confidence must be strictly between zero and one.",
        ci::Error::Shape | ci::Error::NonFinite => "Inference requires finite estimates and unit-aligned influence functions.",
    }.to_owned())
}
fn family<K>(entries: Vec<(K, ci::Point<'_>)>, clusters: &[u64], spec: &Specification) -> Result<Family<K>, String> {
    let (keys, points): (Vec<_>, Vec<_>) = entries.into_iter().unzip();
    let result = infer(&points, clusters, spec, spec.inference)?;
    let n = clusters.len() as f64;
    let analytical_covariance = points.iter().map(|a| points.iter().map(|b| match (a,b) {
        (ci::Point::Estimated(a),ci::Point::Estimated(b)) => a.influence.iter().zip(&b.influence).map(|(a,b)| a*b).sum::<f64>() / (n*n),
        _ => 0.0,
    }).collect()).collect();
    let coverage = match result.coverage {
        ci::Coverage::Pointwise => Coverage::Pointwise,
        ci::Coverage::Simultaneous { critical, large_critical } => Coverage::Simultaneous { critical, large_critical },
    };
    Ok(Family { keys, intervals: result.intervals.into_iter().map(interval).collect(), coverage, analytical_covariance })
}

pub(crate) fn run(values: &[f64], request: Request) -> Result<Evidence, String> {
    let Request { rows, columns, units, times, adoption, weights, clusters, specification: spec } = request;
    crate::matrix::validate_dense_matrix("Staggered DiD", values, rows, columns)?;
    if columns == 0 || units.len()!=rows || times.len()!=rows || adoption.len()!=rows
        || weights.as_ref().is_some_and(|v| v.len()!=rows) || clusters.as_ref().is_some_and(|v| v.len()!=rows)
        || units.iter().any(|v| v.is_empty()) || clusters.as_ref().is_some_and(|v| v.iter().any(|v| v.is_empty())) {
        return Err("Staggered DiD needs aligned outcomes, covariates, unit keys, periods and adoption periods.".into());
    }
    let names: Vec<_> = units.iter().cloned().collect::<BTreeSet<_>>().into_iter().collect();
    let ids: BTreeMap<_,_> = names.iter().enumerate().map(|(i,name)| (name.as_str(),did::Unit(i as u64))).collect();
    let observations: Vec<_> = (0..rows).map(|i| did::Row { unit:ids[units[i].as_str()], period:did::Period(times[i]), adoption:adoption[i].map_or(did::Adoption::Never,|g| did::Adoption::At(did::Period(g))), outcome:values[i] }).collect();
    let controls = match spec.controls { Controls::NeverTreated => did::Controls::NeverTreated, Controls::NotYetTreated => did::Controls::NotYetTreated };
    let baseline = match spec.baseline { Baseline::Varying => did::Baseline::Varying, Baseline::Universal => did::Baseline::Universal };
    let anticipation = did::Anticipation(spec.anticipation);
    let window = did::EventWindow::new(spec.first_event,spec.last_event,spec.balance).map_err(problem)?;
    // Validate all supplied weights and clusters before preprocessing removes any rows.
    let mut weight_by_unit = BTreeMap::new();
    let mut cluster_by_unit = BTreeMap::new();
    for i in 0..rows {
        let unit = observations[i].unit;
        if let Some(w) = &weights {
            if !w[i].is_finite() || w[i]<0.0 || weight_by_unit.insert(unit,w[i]).is_some_and(|old| old != w[i]) {
                return Err("Sampling weights must be finite, nonnegative and constant within each unit.".into());
            }
        }
        if let Some(c) = &clusters {
            if cluster_by_unit.insert(unit,c[i].clone()).is_some_and(|old| old != c[i]) {
                return Err("Each unit must belong to one cluster throughout the panel.".into());
            }
        }
    }
    let prepared = did::preparation::prepare_with_covariates(&observations,controls,anticipation,columns-1).map_err(problem)?;
    let retained: BTreeSet<_> = prepared.panel.units().collect();
    let removed: BTreeSet<_> = prepared.changes.iter().filter_map(|change| match change { did::preparation::Change::NoUntreatedComparison { period } => Some(period.0), _=>None }).collect();
    let keep: Vec<_> = (0..rows).filter(|&i| retained.contains(&observations[i].unit) && !removed.contains(&times[i])).collect();
    let panel = match &weights {
        Some(_) => prepared.panel.with_weights(weight_by_unit.into_iter().filter(|(unit,_)| retained.contains(unit)).collect()).map_err(problem)?,
        None => prepared.panel,
    };
    let result = if columns == 1 {
        did::event_study(&panel,controls,baseline,anticipation,window)
    } else {
        let adjusted = did::Adjusted::new(&panel, keep.iter().map(|&i| (observations[i].unit, observations[i].period, (1..columns).map(|j| values[j*rows+i]).collect())).collect()).map_err(problem)?;
        adjusted.event_study(controls,baseline,anticipation,window)
    }.map_err(problem)?;
    let labels: BTreeMap<_,_> = cluster_by_unit.values().cloned().collect::<BTreeSet<_>>().into_iter().enumerate().map(|(i,label)| (label,i as u64)).collect();
    let cluster_ids: Vec<_> = panel.units().map(|unit| match &clusters { None=>unit.0, Some(_)=>labels[&cluster_by_unit[&unit]] }).collect();
    let points = |entries: &BTreeMap<did::Period,did::Summary>| -> Result<Family,String> { family(entries.iter().map(|(key,s)| (key.0,ci::Point::Estimated(s))).collect(),&cluster_ids,&spec) };
    let events = family(result.events.iter().map(|(key,s)| {
        let support = &result.event_support[key];
        let point = if support.cohorts == support.reference_cohorts { ci::Point::Reference } else { ci::Point::Estimated(s) };
        (*key,point)
    }).collect(),&cluster_ids,&spec)?;
    let cohorts = points(&result.by_group)?;
    let calendar = points(&result.by_calendar)?;
    let mut cell_summaries = Vec::new();
    for ((cohort,period),cell) in &result.cells {
        let s = match cell { did::Cell::Reference => None, did::Cell::Estimated(e) => Some(did::Summary { att:e.att,se:e.se,influence:e.influence.clone() }) };
        cell_summaries.push(([cohort.0,period.0],s));
    }
    // did::MP uses one simultaneous family across every cohort-time cell.
    // Faceting that family in the UI must not recalculate its critical value.
    let cells = family(cell_summaries.iter().map(|(key,s)| (*key,s.as_ref().map_or(ci::Point::Reference,ci::Point::Estimated))).collect(),&cluster_ids,&spec)?;
    let headline = |s: &did::Summary| -> Result<Interval,String> {
        let result = infer(&[ci::Point::Estimated(s)],&cluster_ids,&spec,spec.inference.pointwise())?;
        result.intervals.into_iter().next().map(interval).ok_or_else(|| "The overall ATT interval is missing.".into())
    };
    let overall = Overall { dynamic:headline(&result.overall)?, group:headline(&result.group_overall)?, calendar:headline(&result.calendar_overall)?, simple:headline(&result.simple)? };
    let changes = prepared.changes.into_iter().map(|change| match change {
        did::preparation::Change::BeyondObservedWindow { unit,adoption } => Change::BeyondObservedWindow { unit:names[unit.0 as usize].clone(),adoption:adoption.0 },
        did::preparation::Change::LatestCohortAsComparison { unit,adoption } => Change::LatestCohortAsComparison { unit:names[unit.0 as usize].clone(),adoption:adoption.0 },
        did::preparation::Change::NoUntreatedComparison { period } => Change::NoUntreatedComparison { period:period.0 },
        did::preparation::Change::NoPreTreatment { unit } => Change::NoPreTreatment { unit:names[unit.0 as usize].clone() },
    }).collect();
    let small_cohorts = prepared.small_cohorts.into_iter().map(|c| SmallCohort { adoption:match c.adoption { did::Adoption::Never=>None,did::Adoption::At(g)=>Some(g.0) },count:c.count,required:c.required }).collect();
    let fits = result.fits.iter().map(|(g,t,status)| Fit { cohort:g.0,period:t.0,status:match status { did::dr::FitStatus::Converged { iterations } => FitStatus::Converged { iterations:*iterations }, did::dr::FitStatus::IterationLimit => FitStatus::IterationLimit } }).collect();
    let support = result.event_support.iter().map(|(event,s)| Support { event:*event,cohorts:s.cohorts.iter().map(|g| g.0).collect(),treated_units:s.treated_units,reference_cohorts:s.reference_cohorts.iter().map(|g| g.0).collect() }).collect();
    Ok(Evidence { version:1, observations:rows,retained_observations:keep.len(),units:panel.units().map(|unit| names[unit.0 as usize].clone()).collect(),times:keep.iter().map(|&i| times[i]).collect::<BTreeSet<_>>().into_iter().collect(),covariates:columns-1,weighted:weights.is_some(),cluster_count:cluster_ids.iter().collect::<BTreeSet<_>>().len(),specification:spec,changes,small_cohorts,fits,events,cohorts,calendar,cells,support,overall })
}

fn problem(error: did::Error) -> String {
    use did::Error::*;
    match error {
        TooFewPeriods => "Staggered DiD needs at least two retained periods.".into(),
        NonFinite { .. } => "Outcomes must be finite for every unit and period.".into(),
        Duplicate { .. } => "The panel contains a repeated unit-period key.".into(),
        CohortChanged { .. } => "First treatment must remain the same for every row of a unit.".into(),
        Unbalanced { .. } => "This staggered DiD specification requires a balanced, complete panel.".into(),
        NoPreTreatment { .. } | NoBaseline => "A treatment cohort has no available untreated baseline.".into(),
        UnknownPeriod(_) | MissingCohort(_) => "A requested cohort or period is outside the retained panel.".into(),
        MissingControls | TrimmedControls => "A cohort-period comparison has no supported comparison units.".into(),
        NoPostTreatment => "No supported post-treatment event remains in the selected window.".into(),
        CovariateShape => "Covariates must be finite and aligned to every retained unit-period key.".into(),
        Regression(did::dr::Error::Overlap) => "A propensity fit fails the treatment-overlap check.".into(),
        Regression(did::dr::Error::Singular) => "An adjustment model is singular or poorly conditioned. Review redundant covariates.".into(),
        Regression(_) => "A cohort-period outcome or propensity model could not be fitted.".into(),
        NumericalOverflow => "The calculation produced a non-finite value. Check outcome and covariate scales.".into(),
        InvalidWeights => "Sampling weights must cover each unit exactly once and have positive total weight.".into(),
        InvalidWindow => "The first event time must not exceed the last event time.".into(),
        SmallControlGroup { count,required } => format!("The comparison group has {count} units; this specification requires at least {required}."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn fixture() -> (Vec<f64>, Request) {
        let input: Value = serde_json::from_str(include_str!("../../causal-core/fixtures/staggered-did/mpdta-input.json")).unwrap();
        let rows = input.as_array().unwrap();
        let values = rows.iter().map(|r| r["lemp"].as_f64().unwrap()).chain(rows.iter().map(|r| r["lpop"].as_f64().unwrap())).collect();
        let request = serde_json::from_value(json!({
            "rows":rows.len(), "columns":2,
            "units":rows.iter().map(|r| r["countyreal"].as_u64().unwrap().to_string()).collect::<Vec<_>>(),
            "times":rows.iter().map(|r| r["year"].as_i64().unwrap()).collect::<Vec<_>>(),
            "adoption":rows.iter().map(|r| match r["first.treat"].as_i64().unwrap() { 0=>None,g=>Some(g) }).collect::<Vec<_>>(),
            "weights":null,"clusters":null,
            "specification":{"controls":"never-treated","baseline":"varying","anticipation":0,"firstEvent":null,"lastEvent":null,"balance":null,"confidence":0.95,"inference":{"kind":"analytical"}}
        })).unwrap();
        (values,request)
    }

    #[test]
    fn staggered_boundary_matches_adjusted_reference_and_keeps_interval_meaning() {
        let (values,request) = fixture();
        let evidence = run(&values,request).unwrap();
        let reference: Value = serde_json::from_str(include_str!("../../causal-core/fixtures/staggered-did/inference.json")).unwrap();
        for (i,interval) in evidence.events.intervals.iter().enumerate() {
            let Interval::Estimated { estimate,.. } = interval else { panic!("varying baseline is estimated") };
            let lo=reference["cases"][0]["pointwise_lower"][i].as_f64().unwrap();
            let hi=reference["cases"][0]["pointwise_upper"][i].as_f64().unwrap();
            assert!((estimate-(lo+hi)/2.0).abs()<1e-9);
        }
        assert_eq!(evidence.retained_observations,evidence.observations);
        assert_eq!(evidence.units.len()*evidence.times.len(),evidence.observations);
        assert!(evidence.changes.is_empty());
        assert!(!evidence.fits.is_empty());
        assert!(matches!(evidence.events.coverage,Coverage::Pointwise));
    }

    #[test]
    fn staggered_boundary_refuses_misalignment_and_changing_weights() {
        let (values,mut request)=fixture();
        request.adoption.pop();
        assert!(run(&values,request).err().unwrap().contains("aligned"));
        let (values,mut request)=fixture();
        let first=request.units[0].clone();
        let other=request.units.iter().enumerate().find(|(i,unit)| *i>0 && **unit==first).unwrap().0;
        let mut weights=vec![1.0;request.rows];
        weights[other]=2.0;
        request.weights=Some(weights);
        assert!(run(&values,request).err().unwrap().contains("constant within"));
    }

    #[test]
    fn staggered_boundary_serializes_normalized_reference_as_reference() {
        let (values,mut request)=fixture();
        request.specification.baseline=Baseline::Universal;
        let evidence=run(&values,request).unwrap();
        assert!(evidence.cells.intervals.iter().any(|i| matches!(i,Interval::Reference)));
        let json=serde_json::to_value(evidence).unwrap();
        assert_eq!(json["version"],1);
        assert!(json["cells"]["intervals"].as_array().unwrap().iter().any(|i| i["kind"]=="reference" && i.get("estimate").is_none()));
    }
}
