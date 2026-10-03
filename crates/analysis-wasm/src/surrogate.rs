//! Named two-sample boundary over the shared surrogate kernels.
use hirmos_causal_core::surrogate_index as core;
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Experimental {
    surrogates: Vec<Vec<f64>>,
    treatment: Vec<f64>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Observational {
    surrogates: Vec<Vec<f64>>,
    outcome: Vec<f64>,
}
#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Adjustment {
    None,
    Baseline {
        experimental: Vec<Vec<f64>>,
        observational: Vec<Vec<f64>>,
    },
}
#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Estimator {
    Index,
    Score,
    InfluenceFunction,
}
#[derive(Deserialize, Clone)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Uncertainty {
    None,
    Bootstrap { repetitions: usize, seed: u32 },
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    experimental: Experimental,
    observational: Observational,
    adjustment: Adjustment,
    estimator: Estimator,
    uncertainty: Uncertainty,
}
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum UncertaintyEvidence {
    None,
    BootstrapStandardError {
        standard_error: f64,
        repetitions: usize,
        seed: u32,
    },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    estimator: Estimator,
    experimental_rows: usize,
    observational_rows: usize,
    surrogate_columns: usize,
    baseline_columns: usize,
    estimate: f64,
    uncertainty: UncertaintyEvidence,
}

fn matrix(rows: &[Vec<f64>], expected_rows: usize) -> Result<DMatrix<f64>, String> {
    let columns = rows.first().map(Vec::len).unwrap_or(0);
    if columns == 0
        || rows.len() != expected_rows
        || rows
            .iter()
            .any(|r| r.len() != columns || r.iter().any(|v| !v.is_finite()))
    {
        return Err(
            "Each sample must contain a finite, rectangular matrix with one row per observation."
                .into(),
        );
    }
    Ok(DMatrix::from_fn(rows.len(), columns, |i, j| rows[i][j]))
}
fn design(s: &DMatrix<f64>, baseline: Option<&DMatrix<f64>>) -> DMatrix<f64> {
    let b = baseline.map(|x| x.ncols()).unwrap_or(0);
    DMatrix::from_fn(s.nrows(), 1 + s.ncols() + b, |i, j| {
        if j == 0 {
            1.
        } else if j <= s.ncols() {
            s[(i, j - 1)]
        } else {
            baseline.unwrap()[(i, j - 1 - s.ncols())]
        }
    })
}
fn describe(error: core::Error) -> String {
    match error {
        core::Error::Shape => {
            "The selected columns must describe the same rows within each sample."
        }
        core::Error::NonFinite => {
            "The calculation contains missing, non-finite or overflowing values."
        }
        core::Error::InvalidTreatment => "Experimental treatment must contain only 0 and 1.",
        core::Error::MissingArm => {
            "The experimental sample must contain treated and control observations."
        }
        core::Error::InvalidPropensity => {
            "The fitted probabilities must remain strictly between zero and one."
        }
        core::Error::RankDeficient => "The selected design contains linearly dependent columns.",
        core::Error::Fit => "A regression could not be fitted to the selected sample.",
        core::Error::PropensityDidNotConverge => {
            "A probability model did not converge. Check separation and the selected predictors."
        }
        core::Error::InvalidResample => {
            "Bootstrap samples must preserve the experimental treatment-group sizes."
        }
        core::Error::InvalidBound => {
            "The supplied values do not satisfy the selected bias-bound restriction."
        }
    }
    .into()
}
struct PreparedSamples {
    e: DMatrix<f64>,
    o: DMatrix<f64>,
    baseline: Option<(DMatrix<f64>, DMatrix<f64>)>,
    surrogate_columns: usize,
    baseline_columns: usize,
}
fn prepare(
    experimental: &Experimental,
    observational: &Observational,
    adjustment: &Adjustment,
) -> Result<PreparedSamples, String> {
    let ne = experimental.treatment.len();
    let no = observational.outcome.len();
    let se = matrix(&experimental.surrogates, ne)?;
    let so = matrix(&observational.surrogates, no)?;
    if se.ncols() != so.ncols() {
        return Err(
            "Both samples must supply the same surrogate columns in the same order.".into(),
        );
    }
    let baseline = match adjustment {
        Adjustment::None => None,
        Adjustment::Baseline {
            experimental,
            observational,
        } => {
            let be = matrix(experimental, ne)?;
            let bo = matrix(observational, no)?;
            if be.ncols() != bo.ncols() {
                return Err(
                    "Both samples must supply the same baseline columns in the same order.".into(),
                );
            }
            Some((be, bo))
        }
    };
    let e = design(&se, baseline.as_ref().map(|(be, _)| be));
    let o = design(&so, baseline.as_ref().map(|(_, bo)| bo));
    let baseline_columns = baseline.as_ref().map(|(be, _)| be.ncols()).unwrap_or(0);
    let baseline = baseline.map(|(be, bo)| (design(&be, None), design(&bo, None)));
    Ok(PreparedSamples {
        e,
        o,
        baseline,
        surrogate_columns: se.ncols(),
        baseline_columns,
    })
}

pub(crate) fn run(request: Request) -> Result<Evidence, String> {
    let ne = request.experimental.treatment.len();
    let no = request.observational.outcome.len();
    let PreparedSamples {
        e,
        o,
        baseline,
        surrogate_columns,
        baseline_columns,
    } = prepare(
        &request.experimental,
        &request.observational,
        &request.adjustment,
    )?;
    let pair = baseline.as_ref().map(|(be, bo)| (be, bo));
    let propensity = || match pair {
        Some((be, _)) => core::Propensity::BaselineLogit(be),
        None => core::Propensity::TreatmentPrevalence,
    };
    let w = &request.experimental.treatment;
    let y = DVector::from_vec(request.observational.outcome);
    let estimate = match request.estimator {
        Estimator::Index => core::fit_index(&o, &y, &e, w, propensity()).map(|r| r.index.effect),
        Estimator::Score => core::fit_score(&o, &y, &e, w, pair).map(|r| r.effect),
        Estimator::InfluenceFunction => core::fit_influence(&o, &y, &e, w, pair).map(|r| r.effect),
    }
    .map_err(describe)?;
    let uncertainty = match request.uncertainty {
        Uncertainty::None => UncertaintyEvidence::None,
        Uncertainty::Bootstrap { repetitions, seed } => {
            let plans = core::resampling_plan(w, no, repetitions, seed as u64).map_err(describe)?;
            let fitted = match request.estimator {
                Estimator::Index => core::bootstrap_index(&o, &y, &e, w, propensity(), &plans),
                Estimator::Score => core::bootstrap_score(&o, &y, &e, w, pair, &plans),
                Estimator::InfluenceFunction => {
                    core::bootstrap_influence(&o, &y, &e, w, pair, &plans)
                }
            }
            .map_err(describe)?;
            UncertaintyEvidence::BootstrapStandardError {
                standard_error: fitted.standard_error,
                repetitions,
                seed,
            }
        }
    };
    Ok(Evidence {
        estimator: request.estimator,
        experimental_rows: ne,
        observational_rows: no,
        surrogate_columns,
        baseline_columns,
        estimate,
        uncertainty,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request() -> serde_json::Value {
        serde_json::json!({"experimental":{"surrogates":[[0.],[1.],[2.],[3.]],"treatment":[0.,1.,0.,1.]},
            "observational":{"surrogates":[[0.],[1.],[2.],[3.],[4.]],"outcome":[1.,3.,5.,7.,9.]},
            "adjustment":{"kind":"none"},"estimator":"index","uncertainty":{"kind":"none"}})
    }
    #[test]
    fn sample_roles_and_effect() {
        let result = run(serde_json::from_value(request()).unwrap()).unwrap();
        assert!((result.estimate - 2.).abs() < 1e-9);
        let json = serde_json::to_value(result).unwrap();
        assert_eq!(json["experimentalRows"], 4);
        assert_eq!(json["observationalRows"], 5);
        assert_eq!(json["uncertainty"]["kind"], "none");
        let mut r = request();
        r["experimental"]["outcome"] = serde_json::json!([1, 2, 3, 4]);
        assert!(serde_json::from_value::<Request>(r).is_err());
        let mut r = request();
        r["observational"]["treatment"] = serde_json::json!([0, 1, 0, 1, 0]);
        assert!(serde_json::from_value::<Request>(r).is_err());
    }
    #[test]
    fn malformed_sample_and_adjustment_refused() {
        let mut r = request();
        r["experimental"]["surrogates"][0] = serde_json::json!([1, 2]);
        assert!(run(serde_json::from_value(r).unwrap()).is_err());
        let mut r = request();
        r["adjustment"] = serde_json::json!({"kind":"baseline","experimental":[[1],[2],[3],[4]]});
        assert!(serde_json::from_value::<Request>(r).is_err());
        let mut r = request();
        r["uncertainty"] = serde_json::json!({"kind":"bootstrap","repetitions":1,"seed":4});
        assert!(run(serde_json::from_value(r).unwrap()).is_err());
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DiagnosticRequest {
    experimental: Experimental,
    observational: Observational,
    adjustment: Adjustment,
    analysis: DiagnosticAnalysis,
}

#[derive(Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
enum DiagnosticAnalysis {
    Validation { experimental_outcome: Vec<f64> },
    BiasBounds { restriction: Restriction },
}

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Restriction {
    BinaryWithoutSurrogacy,
    BinaryWithoutComparability,
    BoundedDirectEffect { maximum: f64 },
    BoundedSampleDifference { maximum: f64 },
}
impl Restriction {
    fn numerical(self) -> core::BiasRestriction {
        match self {
            Self::BinaryWithoutSurrogacy => core::BiasRestriction::BinaryWithoutSurrogacy,
            Self::BinaryWithoutComparability => core::BiasRestriction::BinaryWithoutComparability,
            Self::BoundedDirectEffect { maximum } => {
                core::BiasRestriction::BoundedDirectEffect { maximum }
            }
            Self::BoundedSampleDifference { maximum } => {
                core::BiasRestriction::BoundedSampleDifference { maximum }
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ValidationCoefficient {
    estimate: f64,
    standard_error: f64,
    t_statistic: f64,
    residual_degrees_of_freedom: usize,
}
impl From<core::ValidationCoefficient> for ValidationCoefficient {
    fn from(value: core::ValidationCoefficient) -> Self {
        Self {
            estimate: value.estimate,
            standard_error: value.standard_error,
            t_statistic: value.t_statistic,
            residual_degrees_of_freedom: value.residual_degrees_of_freedom,
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum ValidationDiagnostic {
    Estimated { #[serde(flatten)] coefficient: ValidationCoefficient },
    PerfectFit { estimate: f64, residual_degrees_of_freedom: usize },
}
impl From<core::ValidationDiagnostic> for ValidationDiagnostic {
    fn from(value: core::ValidationDiagnostic) -> Self {
        match value {
            core::ValidationDiagnostic::Estimated(coefficient) => Self::Estimated { coefficient: coefficient.into() },
            core::ValidationDiagnostic::PerfectFit { estimate, residual_degrees_of_freedom } => Self::PerfectFit { estimate, residual_degrees_of_freedom },
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
enum DiagnosticResult {
    Validation {
        surrogacy: ValidationDiagnostic,
        comparability: ValidationDiagnostic,
    },
    BiasBounds {
        restriction: Restriction,
        lower: f64,
        upper: f64,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiagnosticEvidence {
    experimental_rows: usize,
    observational_rows: usize,
    surrogate_columns: usize,
    baseline_columns: usize,
    analysis: DiagnosticResult,
}

pub(crate) fn diagnose(request: DiagnosticRequest) -> Result<DiagnosticEvidence, String> {
    let inputs = prepare(
        &request.experimental,
        &request.observational,
        &request.adjustment,
    )?;
    let w = &request.experimental.treatment;
    let y = DVector::from_vec(request.observational.outcome);
    let analysis = match request.analysis {
        DiagnosticAnalysis::Validation {
            experimental_outcome,
        } => {
            let fitted = core::validation_regressions(
                &inputs.o,
                &y,
                &inputs.e,
                &DVector::from_vec(experimental_outcome),
                w,
            )
            .map_err(describe)?;
            DiagnosticResult::Validation {
                surrogacy: fitted.surrogacy.into(),
                comparability: fitted.comparability.into(),
            }
        }
        DiagnosticAnalysis::BiasBounds { restriction } => {
            let propensity = match &inputs.baseline {
                Some((be, _)) => core::Propensity::BaselineLogit(be),
                None => core::Propensity::TreatmentPrevalence,
            };
            let fitted = core::fit_bias_bounds(
                &inputs.o,
                &y,
                &inputs.e,
                w,
                propensity,
                restriction.numerical(),
            )
            .map_err(describe)?;
            DiagnosticResult::BiasBounds {
                restriction,
                lower: fitted.bounds.lower,
                upper: fitted.bounds.upper,
            }
        }
    };
    Ok(DiagnosticEvidence {
        experimental_rows: w.len(),
        observational_rows: y.len(),
        surrogate_columns: inputs.surrogate_columns,
        baseline_columns: inputs.baseline_columns,
        analysis,
    })
}

/// These are different targets: observed period outcomes versus a fixed
/// long-term outcome estimated from progressively longer surrogate windows.
#[derive(Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum PathRequest {
    ObservedOutcomes {
        outcomes: Vec<Vec<f64>>,
        treatment: Vec<f64>,
        labels: Vec<String>,
        uncertainty: Uncertainty,
    },
    SurrogateWindows {
        model: Request,
        windows: Vec<SurrogateWindow>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct SurrogateWindow {
    label: String,
    surrogate_columns: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ObservedPeriod {
    label: String,
    control_mean: f64,
    treated_mean: f64,
    contrast: f64,
    cumulative_mean_contrast: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WindowEvidence {
    label: String,
    evidence: Evidence,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum PathEvidence {
    ObservedOutcomes {
        experimental_rows: usize,
        treated_rows: usize,
        control_rows: usize,
        periods: Vec<ObservedPeriod>,
        participant_benchmark: ValidationCoefficient,
        cumulative_uncertainty: CumulativeUncertainty,
    },
    SurrogateWindows {
        windows: Vec<WindowEvidence>,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CumulativeUncertainty {
    None,
    BootstrapStandardErrors {
        standard_errors: Vec<f64>,
        repetitions: usize,
        seed: u32,
    },
}

fn valid_labels<'a>(labels: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::HashSet::new();
    labels
        .into_iter()
        .all(|label| !label.trim().is_empty() && label == label.trim() && seen.insert(label))
}

pub(crate) fn path(request: PathRequest) -> Result<PathEvidence, String> {
    match request {
        PathRequest::ObservedOutcomes {
            outcomes,
            treatment,
            labels,
            uncertainty,
        } => {
            let outcomes = matrix(&outcomes, treatment.len())?;
            if labels.len() != outcomes.ncols() || !valid_labels(labels.iter().map(String::as_str))
            {
                return Err("Supply one distinct, non-empty period label for each outcome column, in chronological order.".into());
            }
            let fitted =
                core::experimental_outcome_path(&outcomes, &treatment).map_err(describe)?;
            let cumulative_uncertainty = match uncertainty {
                Uncertainty::None => CumulativeUncertainty::None,
                Uncertainty::Bootstrap { repetitions, seed } => {
                    let plans =
                        core::experimental_resampling_plan(&treatment, repetitions, seed as u64)
                            .map_err(describe)?;
                    let draws =
                        core::bootstrap_naive(&outcomes, &treatment, &plans).map_err(describe)?;
                    CumulativeUncertainty::BootstrapStandardErrors {
                        standard_errors: draws.into_iter().map(|d| d.standard_error).collect(),
                        repetitions,
                        seed,
                    }
                }
            };
            let treated_rows = treatment.iter().filter(|&&w| w == 1.).count();
            Ok(PathEvidence::ObservedOutcomes {
                experimental_rows: treatment.len(),
                treated_rows,
                control_rows: treatment.len() - treated_rows,
                periods: labels
                    .into_iter()
                    .zip(fitted.periods)
                    .map(|(label, period)| ObservedPeriod {
                        label,
                        control_mean: period.control_mean,
                        treated_mean: period.treated_mean,
                        contrast: period.contrast,
                        cumulative_mean_contrast: period.cumulative_contrast,
                    })
                    .collect(),
                participant_benchmark: fitted.participant_benchmark.into(),
                cumulative_uncertainty,
            })
        }
        PathRequest::SurrogateWindows { model, windows } => {
            // Validate the entire matrix before slicing; no ragged or missing
            // later measurements may disappear in an earlier window.
            let inputs = prepare(&model.experimental, &model.observational, &model.adjustment)?;
            if windows.is_empty()
                || !valid_labels(windows.iter().map(|w| w.label.as_str()))
                || windows[0].surrogate_columns == 0
                || windows
                    .windows(2)
                    .any(|pair| pair[0].surrogate_columns >= pair[1].surrogate_columns)
                || windows.last().unwrap().surrogate_columns != inputs.surrogate_columns
            {
                return Err("Surrogate windows must have distinct labels and strictly increasing column counts, ending with all selected surrogates.".into());
            }
            let mut results = Vec::with_capacity(windows.len());
            for window in windows {
                let prefix = |rows: &[Vec<f64>]| {
                    rows.iter()
                        .map(|row| row[..window.surrogate_columns].to_vec())
                        .collect()
                };
                // Same response, participants, baseline and bootstrap seed at
                // every window. run() regenerates identical resampling plans.
                let evidence = run(Request {
                    experimental: Experimental {
                        surrogates: prefix(&model.experimental.surrogates),
                        treatment: model.experimental.treatment.clone(),
                    },
                    observational: Observational {
                        surrogates: prefix(&model.observational.surrogates),
                        outcome: model.observational.outcome.clone(),
                    },
                    adjustment: model.adjustment.clone(),
                    estimator: model.estimator,
                    uncertainty: model.uncertainty.clone(),
                })
                .map_err(|error| format!("Surrogate window {}: {error}", window.label))?;
                results.push(WindowEvidence {
                    label: window.label,
                    evidence,
                });
            }
            Ok(PathEvidence::SurrogateWindows { windows: results })
        }
    }
}
