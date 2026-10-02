//! Predictor-based synthetic control. Preparation and fitting share the pinned kernels.
use hirmos_causal_core::synthetic_control::{dataprep as prep, synth};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Summary { Mean, Median, Minimum, Maximum, Sum, Variance, StandardDeviation }
impl Summary {
    fn kernel(self) -> prep::Summary {
        match self {
            Self::Mean => prep::Summary::Mean, Self::Median => prep::Summary::Median,
            Self::Minimum => prep::Summary::Minimum, Self::Maximum => prep::Summary::Maximum,
            Self::Sum => prep::Summary::Sum, Self::Variance => prep::Summary::Variance,
            Self::StandardDeviation => prep::Summary::StandardDeviation,
        }
    }
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpecialPredictor { pub column: usize, pub periods: Vec<f64>, pub summary: Summary }
#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum WeightSelection { Automatic, Supplied { weights: Vec<f64> } }
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Request {
    pub rows: usize, pub column_names: Vec<String>,
    pub units: Vec<f64>, pub times: Vec<f64>,
    pub treated: f64, pub donors: Vec<f64>, pub outcome: usize,
    pub predictors: Vec<usize>, pub summary: Summary, pub special: Vec<SpecialPredictor>,
    pub predictor_periods: Vec<f64>, pub fit_periods: Vec<f64>, pub plot_periods: Vec<f64>,
    pub selection: WeightSelection,
}
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Method { NelderMead, Bfgs }
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Start { Equal, Regression }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub enum Attempt {
    Completed { start: Start, method: Method, evaluations: usize, convergence_code: usize, outcome_mspe: f64 },
    Failed { start: Start, method: Method, evaluations: usize, detail: String },
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub enum Selection {
    SinglePredictor, SuppliedWeights,
    Optimized { start: Start, method: Method, evaluations: usize, convergence_code: usize },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance { pub predictor: String, pub treated: f64, pub synthetic: f64, pub donor_mean: f64, pub weight: f64 }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
    pub request: Request,
    pub treated_unit: f64, pub donors: Vec<f64>, pub donor_weights: Vec<f64>,
    pub fit_periods: Vec<f64>, pub plot_periods: Vec<f64>,
    pub observed: Vec<f64>, pub synthetic: Vec<Option<f64>>, pub gaps: Vec<Option<f64>>,
    pub balance: Vec<Balance>, pub predictor_loss: f64, pub outcome_mspe: f64,
    pub donor_iterations: usize, pub selection: Selection, pub attempts: Vec<Attempt>,
    pub preparation_notes: Vec<String>,
}
fn method(value: synth::PredictorOptimizer) -> Method {
    match value { synth::PredictorOptimizer::NelderMead => Method::NelderMead, synth::PredictorOptimizer::Bfgs => Method::Bfgs }
}
fn start(value: synth::PredictorStart) -> Start {
    match value { synth::PredictorStart::Equal => Start::Equal, synth::PredictorStart::Regression => Start::Regression }
}
fn fit_problem(error: synth::SynthError) -> String {
    match error {
        synth::SynthError::EmptyPredictors => "Choose at least one predictor.".into(),
        synth::SynthError::InsufficientDonors => "Choose at least two distinct donor units.".into(),
        synth::SynthError::EmptyPrePeriod => "Choose at least one fitting period.".into(),
        synth::SynthError::DimensionMismatch => "The prepared matrices have incompatible dimensions.".into(),
        synth::SynthError::NonFiniteInput => "The fitting matrices must contain finite values.".into(),
        synth::SynthError::ConstantDonorPredictor { predictor } => format!("Predictor {} has no variation across donor units. Remove it from this specification.", predictor + 1),
        synth::SynthError::InvalidPredictorWeights => "Supply one non-negative weight per summarized predictor, with a positive total.".into(),
        synth::SynthError::DonorSolver(_) => "The donor-weight calculation could not solve the numerical system.".into(),
        synth::SynthError::DonorIterationLimit => "The donor-weight calculation reached its iteration limit.".into(),
        synth::SynthError::NelderMead(_) | synth::SynthError::Bfgs(_) => "The predictor-weight optimization could not complete this attempt.".into(),
        synth::SynthError::NoUsableOptimizer => "No predictor-weight optimization attempt produced a usable fit.".into(),
    }
}
fn preparation_problem(error: prep::PreparationError) -> String {
    use prep::PreparationError as E;
    match error {
        E::InvalidKeys => "Unit and period keys must be finite numbers.".into(),
        E::InvalidRows | E::InvalidColumns => "The panel values and their column names do not match.".into(),
        E::EmptyPredictors => "Choose an ordinary or period-specific predictor.".into(),
        E::InsufficientDonors => "Choose at least two donor units.".into(),
        E::DuplicateDonor => "Each donor unit must be selected only once.".into(),
        E::TreatedAmongDonors => "The treated unit cannot also be a donor.".into(),
        E::MissingUnit { unit } => format!("Unit {unit} is not in this panel."),
        E::Unbalanced { unit, period, rows } => format!("Unit {unit} has {rows} rows at period {period}. Exactly one row is required for each selected unit and period."),
        E::InvalidWindow { .. } => "Each period selection must be non-empty, finite and free of duplicates.".into(),
        E::MissingPeriod { period, .. } => format!("Period {period} is not in this panel."),
        E::InvalidColumn { column } => format!("Column {} is not in the supplied matrix.", column + 1),
        E::DuplicatePredictor => "Select each ordinary predictor only once.".into(),
        E::AllMissingPredictor { unit, predictor } => format!("Predictor {} is missing for every selected period in unit {unit}.", predictor + 1),
        E::UndefinedSummary { unit, predictor } => format!("The summary of predictor {} is undefined for unit {unit}.", predictor + 1),
        E::NonFiniteValue { unit, period, column } => format!("Column {} has an infinite value for unit {unit} at period {period}.", column + 1),
        E::MissingOutcome { unit, period, .. } => format!("The outcome is missing for unit {unit} at period {period}. Fitting outcomes and the treated plot path must be observed."),
    }
}
pub fn run(values: &[f64], request: Request) -> Result<Evidence, String> {
    let specification = request.clone();
    let columns = request.column_names.len();
    if request.rows == 0 || columns == 0 || request.rows.checked_mul(columns) != Some(values.len())
        || request.units.len() != request.rows || request.times.len() != request.rows {
        return Err("The panel keys, values and column names must describe the same rows.".into());
    }
    let data = prep::PanelData {
        columns: request.column_names,
        rows: (0..request.rows).map(|row| prep::PanelRow {
            unit: request.units[row], period: request.times[row],
            values: (0..columns).map(|column| {
                let value = values[column * request.rows + row];
                if value.is_nan() { None } else { Some(value) }
            }).collect(),
        }).collect(),
    };
    let prepared = prep::prepare(&data, &prep::Preparation {
        treated: request.treated, donors: request.donors, outcome: request.outcome,
        predictors: request.predictors, summary: request.summary.kernel(),
        special: request.special.into_iter().map(|p| prep::SpecialPredictor {
            column: p.column, periods: p.periods, summary: p.summary.kernel(),
        }).collect(),
        predictor_periods: request.predictor_periods, fit_periods: request.fit_periods, plot_periods: request.plot_periods,
    }).map_err(preparation_problem)?;
    let matrices = prepared.matrices().map_err(fit_problem)?;
    let fit = match request.selection {
        WeightSelection::Automatic => matrices.fit_default(),
        WeightSelection::Supplied { weights } => matrices.fit_custom(&weights),
    }.map_err(fit_problem)?;
    let summary = prepared.summarize_fit(&fit.fit).map_err(fit_problem)?;
    let selection = match fit.selection {
        synth::PredictorSelection::SinglePredictor => Selection::SinglePredictor,
        synth::PredictorSelection::SuppliedWeights => Selection::SuppliedWeights,
        synth::PredictorSelection::Optimized(selected) => Selection::Optimized {
            start: start(selected.start), method: method(selected.candidate.method),
            evaluations: selected.candidate.evaluations, convergence_code: selected.candidate.convergence_code,
        },
    };
    let attempts = fit.attempts.into_iter().map(|a| match a.attempt {
        synth::OptimizerAttempt::Completed(c) => Attempt::Completed {
            start: start(a.start), method: method(c.method), evaluations: c.evaluations,
            convergence_code: c.convergence_code, outcome_mspe: c.outcome_mspe,
        },
        synth::OptimizerAttempt::Failed { method: m, error, objective_evaluations } => Attempt::Failed {
            start: start(a.start), method: method(m), evaluations: objective_evaluations, detail: fit_problem(error),
        },
    }).collect();
    let balance = prepared.predictor_names.iter().enumerate().map(|(i, name)| Balance {
        predictor: name.clone(), treated: summary.treated_predictors[i], synthetic: summary.synthetic_predictors[i],
        donor_mean: summary.donor_mean_predictors[i], weight: fit.fit.predictor_weights[i],
    }).collect();
    let preparation_notes = prepared.evidence.iter().map(|note| match note {
        prep::PreparationEvidence::SortedDonors { .. } => "Donor units were sorted into the recorded matrix order.".into(),
        prep::PreparationEvidence::SortedPeriods { .. } => "Selected periods were sorted into chronological order.".into(),
        prep::PreparationEvidence::ControlPredictorsUseMean { .. } => "Ordinary donor predictors use the mean, even when the treated-unit summary differs.".into(),
        prep::PreparationEvidence::OmittedPredictorValues { unit, predictor, periods } => format!("Predictor {} omitted {} missing values for unit {unit}.", predictor + 1, periods.len()),
    }).collect();
    Ok(Evidence {
        request: specification,
        treated_unit: prepared.treated, donors: prepared.donors, donor_weights: fit.fit.donor_weights,
        fit_periods: prepared.fit_periods, plot_periods: prepared.plot_periods, observed: prepared.y1_plot.as_slice().to_vec(),
        synthetic: summary.synthetic_path, gaps: summary.gaps, balance, predictor_loss: fit.fit.predictor_loss,
        outcome_mspe: fit.fit.outcome_mspe, donor_iterations: fit.fit.donor_iterations, selection, attempts, preparation_notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    fn texas() -> (Value, Vec<f64>, Request) {
        let oracle: Value = serde_json::from_str(include_str!("../test-data/predictor-synth-texas.json")).unwrap();
        let raw: Vec<u8> = serde_json::from_value(oracle["exact"]["rows"].clone()).unwrap();
        let decoded: Vec<f64> = raw.chunks_exact(8).map(|bytes| f64::from_le_bytes(bytes.try_into().unwrap())).collect();
        let records = oracle["data"]["rows"].as_array().unwrap();
        let rows = records.len();
        let mut values = vec![0.; rows * 7];
        for row in 0..rows { for column in 0..7 { values[column * rows + row] = decoded[row * 7 + column]; } }
        let r = &oracle["request"];
        let request: Request = serde_json::from_value(json!({
            "rows": rows, "columnNames": oracle["data"]["columns"],
            "units": records.iter().map(|row| row["unit"].as_f64().unwrap()).collect::<Vec<_>>(),
            "times": records.iter().map(|row| row["period"].as_f64().unwrap()).collect::<Vec<_>>(),
            "treated": r["treated"], "donors": r["donors"], "outcome": r["outcome"],
            "predictors": r["predictors"], "summary": "mean",
            "special": r["special"].as_array().unwrap().iter().map(|s| json!({"column":s["column"],"periods":s["periods"],"summary":"mean"})).collect::<Vec<_>>(),
            "predictorPeriods": r["predictor_periods"], "fitPeriods": r["fit_periods"], "plotPeriods": r["plot_periods"],
            "selection": {"kind":"automatic"},
        })).unwrap();
        (oracle, values, request)
    }
    fn close(actual: &[f64], expected: &Value) {
        let expected: Vec<f64> = serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(actual.len(), expected.len());
        for (i, (a, e)) in actual.iter().zip(expected).enumerate() {
            assert!((a - e).abs() <= 1e-10 * e.abs().max(1.), "position {i}: {a} != {e}");
        }
    }
    #[test]
    fn texas_facade_retains_oracle_values_and_termination_status() {
        let (oracle, values, request) = texas();
        let evidence = run(&values, request).unwrap();
        close(&evidence.donor_weights, &oracle["fit"]["w"]);
        close(&evidence.balance.iter().map(|r| r.weight).collect::<Vec<_>>(), &oracle["fit"]["v"]);
        close(&evidence.synthetic.iter().map(|v| v.unwrap()).collect::<Vec<_>>(), &oracle["synthetic"]);
        close(&evidence.gaps.iter().map(|v| v.unwrap()).collect::<Vec<_>>(), &oracle["gaps"]);
        close(&[evidence.outcome_mspe], &json!([oracle["fit"]["outcome_mspe"]]));
        assert!(matches!(evidence.selection, Selection::Optimized { start: Start::Regression, method: Method::NelderMead, evaluations: 501, convergence_code: 1 }));
        assert_eq!(evidence.attempts.len(), 4);
        assert_eq!(evidence.attempts.iter().filter(|a| matches!(a, Attempt::Failed { .. })).count(), 2);
        let serialized = serde_json::to_value(evidence).unwrap();
        assert_eq!(serialized["selection"]["convergenceCode"], 1);
        assert_eq!(serialized["request"]["selection"]["kind"], "automatic");
    }
    #[test]
    fn malformed_key_axis_is_refused_before_fitting() {
        let (_, values, mut request) = texas();
        request.units.pop();
        assert!(run(&values, request).err().unwrap().contains("same rows"));
    }
    #[test]
    fn invalid_supplied_weights_are_not_replaced_by_automatic_fitting() {
        let (_, values, mut request) = texas();
        request.selection = WeightSelection::Supplied { weights: vec![0.; 7] };
        assert!(run(&values, request).err().unwrap().contains("positive total"));
    }
}
