//! Browser boundary for the existing GRF port. No numerical implementations live here.
use hirmos_causal_core::grf::{causal, forest, inference, regression, tree, tuning};
use serde::{Deserialize, Serialize};
use crate::causal_forest_analysis as extra;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Population { All, Treated, Control, Overlap }
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ContinuousPopulation { All, Overlap }
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum Target {
    BinaryAverage { population: Population },
    BinaryConditional {},
    ContinuousAverage { population: ContinuousPopulation },
    ContinuousConditional {},
}
impl Target {
    fn binary(self) -> bool { matches!(self, Self::BinaryAverage { .. } | Self::BinaryConditional {}) }
    fn conditional(self) -> bool { matches!(self, Self::BinaryConditional {} | Self::ContinuousConditional {}) }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ConfigurationKind { CausalForest }
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum Honesty { Disabled {}, Enabled { fraction: f64, prune: bool } }
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum VariablesPerSplit { Automatic {}, Specified { count: u32 } }
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub(crate) enum Tuning { Disabled {}, All { trees: u32, repetitions: usize, draws: usize } }
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase", deny_unknown_fields)]
pub(crate) struct ImportanceRefit {
    pub nuisance_trees: u32, pub initial_trees: u32,
    pub outcome_seed: u32, pub treatment_seed: u32, pub initial_seed: u32,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Configuration {
    pub kind: ConfigurationKind,
    pub trees: u32,
    pub seed: u32,
    pub sample_fraction: f64,
    pub variables_per_split: VariablesPerSplit,
    pub minimum_node_size: usize,
    pub honesty: Honesty,
    pub alpha: f64,
    pub imbalance_penalty: f64,
    pub stabilize_splits: bool,
    pub group_size: usize,
    pub tuning: Tuning,
    pub confidence_level: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<extra::Specification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refit: Option<ImportanceRefit>,
}
impl Configuration {
    fn options(&self, columns: usize) -> Result<forest::Options, String> {
        if self.group_size < 2 || self.trees < self.group_size as u32 {
            return Err("Confidence intervals require groups of at least 2 trees and at least one complete group.".into());
        }
        if !self.confidence_level.is_finite() || self.confidence_level <= 0.0 || self.confidence_level >= 1.0 {
            return Err("The confidence level must be between 0 and 1.".into());
        }
        let mtry = match self.variables_per_split {
            VariablesPerSplit::Automatic {} => ((columns as f64).sqrt() + 20.0).ceil().min(columns as f64) as u32,
            VariablesPerSplit::Specified { count } if count > 0 && count as usize <= columns => count,
            VariablesPerSplit::Specified { .. } => return Err("Variables per split must be between 1 and the number of encoded covariates.".into()),
        };
        Ok(forest::Options {
            trees: self.trees, group_size: self.group_size, sample_fraction: self.sample_fraction,
            seed: self.seed, seed_mode: forest::SeedMode::Indexed, batches: 1,
            tree: tree::Options { mtry, min_node_size: self.minimum_node_size,
                honesty: match self.honesty {
                    Honesty::Disabled {} => tree::Honesty::Disabled,
                    Honesty::Enabled { fraction, prune } => tree::Honesty::Enabled { fraction, prune },
                }, alpha: self.alpha, imbalance_penalty: self.imbalance_penalty },
        })
    }
}
#[derive(Serialize)]
pub(crate) struct Interval { lower: f64, upper: f64 }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub(crate) enum Uncertainty {
    Unavailable { reason: String },
    Estimated { standard_error: f64, interval: Interval },
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub(crate) enum Prediction {
    Unavailable { reason: String },
    Estimated { estimate: f64, uncertainty: Uncertainty },
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub(crate) enum Summary {
    NotRequested,
    Unavailable { reason: String },
    Estimated { estimate: f64, standard_error: f64, interval: Interval },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Coefficient { estimate: f64, standard_error: f64, statistic: f64, p_value: f64 }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
pub(crate) enum Calibration {
    Unavailable { reason: String },
    Estimated { mean: Coefficient, differential: Coefficient, degrees_of_freedom: usize },
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    target: Target, confidence_level: f64, observations: usize,
    predictions: Vec<Prediction>, summary: Summary, calibration: Calibration,
    variable_importance: Vec<f64>,
    /// Actual causal forest options, including the winning tuning parameters.
    fitted: Configuration,
    #[serde(skip_serializing_if = "Option::is_none")]
    analysis: Option<extra::Evidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refit: Option<RefitEvidence>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all="camelCase")]
struct RefitEvidence {
    initial_importance: Vec<f64>, selected: Vec<usize>,
    outcome_predictions: Vec<f64>, treatment_predictions: Vec<f64>,
}

#[derive(PartialEq)]
struct RefitKey {
    x: Vec<Vec<f64>>, y: Vec<f64>, w: Vec<f64>, weights: Option<Vec<f64>>,
    membership: (Vec<usize>,usize), specification: ImportanceRefit,
}
thread_local! {
    // One exact-input memo per worker. Replaying tuning decisions must not refit
    // unchanged preliminary forests. Workers are terminated by their run owner.
    static REFIT_CACHE: std::cell::RefCell<Option<(RefitKey,RefitEvidence)>> = const { std::cell::RefCell::new(None) };
}

/// The ACIC script's selection workflow. Nuisance fits keep every adjustment
/// covariate; importance selection affects only the second causal forest.
fn prepare_refit(x: &[Vec<f64>], y: &[f64], w: &[f64], weights: Option<&[f64]>,
    clusters: &hirmos_causal_core::grf::sampling::Clusters, spec: &ImportanceRefit) -> Result<RefitEvidence, String> {
    let key = RefitKey {x:x.to_vec(),y:y.to_vec(),w:w.to_vec(),weights:weights.map(<[f64]>::to_vec),membership:clusters.membership(),specification:spec.clone()};
    if let Some(value) = REFIT_CACHE.with(|cache| cache.borrow().as_ref().filter(|(cached,_)| cached==&key).map(|(_,value)|value.clone())) { return Ok(value); }
    let value=compute_refit(x,y,w,weights,clusters,spec)?;
    REFIT_CACHE.with(|cache| *cache.borrow_mut()=Some((key,value.clone())));
    Ok(value)
}
fn compute_refit(x: &[Vec<f64>], y: &[f64], w: &[f64], weights: Option<&[f64]>,
    clusters: &hirmos_causal_core::grf::sampling::Clusters, spec: &ImportanceRefit) -> Result<RefitEvidence, String> {
    if spec.nuisance_trees == 0 || spec.initial_trees < 2 { return Err("The preliminary forests need positive tree budgets, with at least two causal trees.".into()); }
    let defaults = |trees, seed, group_size| forest::Options { trees, seed, group_size, sample_fraction: 0.5,
        seed_mode: forest::SeedMode::Indexed, batches: 1,
        tree: tree::Options { mtry: ((x.len() as f64).sqrt()+20.).ceil().min(x.len() as f64) as u32,
            min_node_size: 5, honesty: tree::Honesty::Enabled { fraction: 0.5, prune: true }, alpha: 0.05, imbalance_penalty: 0. } };
    let nuisance = |target: &[f64], seed| -> Result<Vec<f64>, String> {
        let mut columns = x.to_vec(); columns.push(target.to_vec());
        let weight = weights.map(|v| { columns.push(v.to_vec()); columns.len()-1 });
        // These are standalone regression_forest fits, whose default ci.group.size
        // is 2, not the group size of the causal wrapper's internal nuisance fits.
        regression::train(&columns, x.len(), weight, clusters, &defaults(spec.nuisance_trees, seed, 2))
            .and_then(|f| f.predict(x, true)).map_err(str::to_owned)?.into_iter()
            .map(|v| v.ok_or_else(|| "A preliminary nuisance prediction is unavailable.".into())).collect()
    };
    let outcome_predictions = nuisance(y, spec.outcome_seed)?;
    let treatment_predictions = nuisance(w, spec.treatment_seed)?;
    let initial = causal::fit(x, y, w, weights, clusters, &defaults(spec.initial_trees, spec.initial_seed, 2),
        Some(&outcome_predictions), Some(&treatment_predictions)).map_err(str::to_owned)?;
    let initial_importance = initial.forest.variable_importance(x.len(), 4, 2.).map_err(str::to_owned)?;
    let mean = initial_importance.iter().sum::<f64>() / x.len() as f64;
    let selected: Vec<_> = initial_importance.iter().enumerate().filter_map(|(i,v)| (*v > mean).then_some(i)).collect();
    if selected.is_empty() { return Err("No covariate has importance above the mean. No selected-variable refit was performed.".into()); }
    Ok(RefitEvidence { initial_importance, selected, outcome_predictions, treatment_predictions })
}

fn interval(estimate: f64, se: f64, critical: f64) -> Interval {
    Interval { lower: estimate - critical * se, upper: estimate + critical * se }
}
fn summarize(result: Result<inference::Average, &'static str>, critical: f64) -> Summary {
    match result {
        Ok(value) if value.estimate.is_finite() && value.standard_error.is_finite() && value.standard_error >= 0.0 =>
            Summary::Estimated { estimate: value.estimate, standard_error: value.standard_error, interval: interval(value.estimate, value.standard_error, critical) },
        Ok(_) => Summary::Unavailable { reason: "The aggregate estimate or its standard error is not finite.".into() },
        Err(reason) => Summary::Unavailable { reason: reason.into() },
    }
}
fn calibration(result: Result<inference::LinearSummary, &'static str>) -> Calibration {
    match result {
        Ok(value) if value.estimates.len() == 2 && value.standard_errors.len() == 2 && value.statistics.len() == 2 && value.p_values.len() == 2
            && [&value.estimates, &value.standard_errors, &value.statistics, &value.p_values].iter().all(|v| v.iter().all(|x| x.is_finite())) => {
            let coefficient = |i| Coefficient { estimate: value.estimates[i], standard_error: value.standard_errors[i], statistic: value.statistics[i], p_value: value.p_values[i] };
            Calibration::Estimated { mean: coefficient(0), differential: coefficient(1), degrees_of_freedom: value.df }
        },
        Ok(_) => Calibration::Unavailable { reason: "The calibration coefficients could not be estimated with finite uncertainty.".into() },
        Err(reason) => Calibration::Unavailable { reason: reason.into() },
    }
}

#[cfg(test)]
pub(crate) fn fit(values: &[f64], rows: usize, columns: usize, treatment: usize, outcome: usize,
    adjustment: &[usize], target: Target, config: Configuration) -> Result<Evidence, String> {
    fit_named(values, rows, columns, treatment, outcome, adjustment, target, config, &[])
}

pub(crate) fn fit_named(values: &[f64], rows: usize, columns: usize, treatment: usize, outcome: usize,
    adjustment: &[usize], target: Target, config: Configuration, names: &[String]) -> Result<Evidence, String> {
    fit_scheduled(values, rows, columns, treatment, outcome, adjustment, target, config, names, None)
}

pub(crate) fn fit_scheduled(values: &[f64], rows: usize, columns: usize, treatment: usize, outcome: usize,
    adjustment: &[usize], target: Target, config: Configuration, names: &[String],
    scheduler: Option<&mut causal::Tuner<'_>>) -> Result<Evidence, String> {
    super::matrix::validate_dense_matrix("Causal forest", values, rows, columns)?;
    let mut roles = vec![treatment, outcome];
    roles.extend_from_slice(adjustment);
    let count = roles.len();
    roles.sort_unstable(); roles.dedup();
    if rows < 2 || adjustment.is_empty() || roles.len() != count || roles.iter().any(|&c| c >= columns) {
        return Err("Choose distinct treatment, outcome and covariate columns, with at least 2 observations.".into());
    }
    let column = |c: usize| &values[c * rows..(c + 1) * rows];
    let w = column(treatment);
    let y = column(outcome);
    if target.binary() && (w.iter().any(|v| *v != 0.0 && *v != 1.0) || !w.contains(&0.0) || !w.contains(&1.0)) {
        return Err("This target requires treatment values 0 and 1, with observations in both groups.".into());
    }
    if w.iter().all(|v| *v == w[0]) { return Err("The treatment must vary across observations.".into()); }
    let x: Vec<Vec<f64>> = adjustment.iter().map(|&c| column(c).to_vec()).collect();
    let specification = config.analysis.clone().unwrap_or_default();
    if config.analysis.is_some() && names.len() != columns { return Err("The forest column identities do not match the prepared matrix.".into()); }
    let sample = extra::sample(&specification, names, values, rows)?;
    let labels = &sample.labels;
    let clusters = &sample.clusters;
    if specification.average_method == extra::AverageMethod::Tmle
        && (!matches!(target, Target::BinaryAverage { population: Population::All | Population::Treated | Population::Control })
            || sample.clustered || sample.weights.is_some()) {
        return Err("TMLE requires a binary ATE, ATT or ATC with independent, unweighted observations.".into());
    }
    if matches!(specification.projection, extra::Projection::Linear { overlap: true, .. }) && !target.binary() {
        return Err("Overlap-weighted projection requires a binary treatment.".into());
    }
    let refit = config.refit.as_ref().map(|spec| prepare_refit(&x, y, w, sample.weights.as_deref(), clusters, spec)).transpose()?;
    let original_columns = x.len();
    let x = match &refit { Some(refit) => refit.selected.iter().map(|&i| x[i].clone()).collect(), None => x };
    let y_hat = refit.as_ref().map(|r| r.outcome_predictions.as_slice());
    let w_hat = refit.as_ref().map(|r| r.treatment_predictions.as_slice());
    let options = config.options(x.len())?;
    let prune = match config.honesty { Honesty::Disabled {} => true, Honesty::Enabled { prune, .. } => prune };
    let model = match config.tuning {
        Tuning::Disabled {} => causal::fit_with_nuisance_pruning(&x, y, w, sample.weights.as_deref(), clusters, &options, y_hat, w_hat, config.stabilize_splits, prune),
        Tuning::All { trees, repetitions, draws } => {
            let parameters = [tuning::Parameter::SampleFraction, tuning::Parameter::Mtry, tuning::Parameter::MinNodeSize,
                tuning::Parameter::HonestyFraction, tuning::Parameter::HonestyPrune, tuning::Parameter::Alpha, tuning::Parameter::ImbalancePenalty]
                .into_iter().collect();
            let tuning = tuning::Config { parameters, mini_trees: trees, repetitions, draws };
            match scheduler {
                Some(scheduler) => causal::fit_tuned_with(&x, y, w, sample.weights.as_deref(), clusters, &options, y_hat, w_hat, config.stabilize_splits, prune, &tuning, scheduler),
                None => causal::fit_tuned(&x, y, w, sample.weights.as_deref(), clusters, &options, y_hat, w_hat, config.stabilize_splits, prune, &tuning),
            }
        },
    }.map_err(str::to_owned)?;
    // Upstream can retain defaults after a failed tuning attempt. Do not silently
    // present that as a successful tuned run in the product.
    if let Some(stages) = &model.tuning {
        for (name, stage) in [("outcome", stages.outcome.as_ref()), ("treatment", stages.treatment.as_ref()), ("causal", Some(&stages.causal))] {
            if let Some(stage) = stage {
                if let tuning::Status::Failed(ref reason) = stage.status {
                    let detail = match reason {
                        tuning::Failure::TooFewUsableForests => "too few candidate forests produced usable errors",
                        tuning::Failure::NearlyConstantErrors => "the candidate errors varied too little to fit the tuning model",
                        tuning::Failure::ResponseSurface(_) => "the tuning model could not be fitted",
                    };
                    return Err(format!("The {name} forest tuning could not finish: {detail}. No tuned result was saved."));
                }
            }
        }
    }
    let chosen = model.tuning.as_ref().map(|t| &t.causal.options).unwrap_or(&options);
    let fitted = Configuration {
        trees: chosen.trees, sample_fraction: chosen.sample_fraction,
        variables_per_split: VariablesPerSplit::Specified { count: chosen.tree.mtry }, minimum_node_size: chosen.tree.min_node_size,
        honesty: match chosen.tree.honesty { tree::Honesty::Disabled => Honesty::Disabled {}, tree::Honesty::Enabled { fraction, prune } => Honesty::Enabled { fraction, prune } },
        alpha: chosen.tree.alpha, imbalance_penalty: chosen.tree.imbalance_penalty, tuning: Tuning::Disabled {}, ..config.clone()
    };
    let critical = spec_math::cephes64::ndtri((1.0 + config.confidence_level) / 2.0);
    if !critical.is_finite() { return Err("The confidence level is too close to 1 to calculate finite intervals.".into()); }
    let raw = model.forest.predict(&x, true, true).map_err(str::to_owned)?;
    let predictions = raw.iter().map(|p| match p.effect {
        None => Prediction::Unavailable { reason: "No finite out-of-bag prediction is available.".into() },
        Some(estimate) => Prediction::Estimated { estimate, uncertainty: match p.variance {
            forest::Variance::Estimate(v) if v >= 0.0 && v.is_finite() => Uncertainty::Estimated { standard_error: v.sqrt(), interval: interval(estimate, v.sqrt(), critical) },
            _ => Uncertainty::Unavailable { reason: "The contributing tree groups do not provide a finite variance estimate.".into() },
        } },
    }).collect();
    let tau: Option<Vec<f64>> = raw.iter().map(|p| p.effect).collect();
    let mut analysis = config.analysis.as_ref().map(|spec| extra::unavailable(spec, "Every observation needs an out-of-bag prediction for this analysis."));
    let (summary, calibration) = match tau {
        None => (
            if target.conditional() { Summary::NotRequested } else { Summary::Unavailable { reason: "Some observations have no out-of-bag prediction. No observations were silently removed.".into() } },
            Calibration::Unavailable { reason: "Calibration requires an out-of-bag prediction for every observation.".into() },
        ),
        Some(tau) => {
            let data = inference::Observations { y, w, y_hat: &model.outcome_hat, w_hat: &model.treatment_hat, tau: &tau };
            let weights = &sample.normalized;
            let debiasing = if !target.binary() && (matches!(target, Target::ContinuousAverage { population: ContinuousPopulation::All })
                || !matches!(specification.projection, extra::Projection::None {}) || !matches!(specification.ranking, extra::Ranking::None {})) {
                Some(causal::continuous_debiasing(&x, w, &model.treatment_hat, sample.weights.as_deref(), clusters, config.seed, 1, forest::SeedMode::Indexed, 500).map_err(str::to_owned)?)
            } else { None };
            if let Some(spec) = &config.analysis { analysis = Some(extra::evaluate(spec, &data, &sample, names, values, debiasing.as_deref(), config.confidence_level)?); }
            let summary = match target {
                Target::BinaryConditional {} | Target::ContinuousConditional {} => Summary::NotRequested,
                Target::BinaryAverage { population: Population::Overlap } | Target::ContinuousAverage { population: ContinuousPopulation::Overlap } => summarize(inference::overlap(&data, weights, labels, sample.clustered), critical),
                Target::BinaryAverage { population } => {
                    let target = match population { Population::All => inference::Target::All, Population::Treated => inference::Target::Treated, Population::Control => inference::Target::Control, Population::Overlap => unreachable!() };
                    summarize(match specification.average_method {
                        extra::AverageMethod::Aipw => inference::aipw(&data, weights, labels, target, None),
                        extra::AverageMethod::Tmle => inference::tmle(&data, target),
                    }, critical)
                },
                Target::ContinuousAverage { population: ContinuousPopulation::All } => {
                    summarize(inference::aipw(&data, weights, labels, inference::Target::All, debiasing.as_deref()), critical)
                },
            };
            (summary, calibration(inference::calibration(&data, &weights, &labels, inference::Covariance::Hc3)))
        },
    };
    let final_importance = model.forest.variable_importance(x.len(), 4, 2.0).map_err(str::to_owned)?;
    let variable_importance = match &refit {
        Some(refit) => { let mut full = vec![0.; original_columns]; for (&i, value) in refit.selected.iter().zip(final_importance) { full[i] = value; } full },
        None => final_importance,
    };
    Ok(Evidence { target, confidence_level: config.confidence_level, observations: rows, predictions, summary, calibration, variable_importance, fitted, analysis, refit })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn number(value: &Value) -> f64 { value.as_f64().unwrap_or_else(|| value.as_str().unwrap().parse().unwrap()) }
    fn configuration(seed: u32) -> Configuration {
        serde_json::from_value(json!({
            "kind":"causal-forest", "trees":100, "seed":seed, "sampleFraction":0.5,
            "variablesPerSplit":{"kind":"specified","count":4}, "minimumNodeSize":5,
            "honesty":{"kind":"enabled","fraction":0.5,"prune":true}, "alpha":0.05,
            "imbalancePenalty":0, "stabilizeSplits":true, "groupSize":2,
            "tuning":{"kind":"disabled"}, "confidenceLevel":0.95,
        })).unwrap()
    }

    #[test]
    fn binary_and_continuous_facade_match_pinned_r() {
        let fixture: Value = serde_json::from_str(include_str!("../../causal-core/oracle/grf/fixtures/nuisance.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap().iter().take(2) {
            let rows = case["Y"].as_array().unwrap().len();
            let columns = case["X"].as_array().unwrap();
            let values: Vec<f64> = [&case["W"], &case["Y"]].into_iter().chain(columns.iter())
                .flat_map(|column| column.as_array().unwrap().iter().map(number)).collect();
            let binary = case["W"].as_array().unwrap().iter().all(|v| number(v) == 0.0 || number(v) == 1.0);
            let target = if binary { Target::BinaryAverage { population: Population::All } }
                else { Target::ContinuousAverage { population: ContinuousPopulation::All } };
            let result = fit(&values, rows, 6, 0, 1, &[2,3,4,5], target, configuration(case["seed"].as_u64().unwrap() as u32)).unwrap();
            let result = serde_json::to_value(result).unwrap();
            assert_eq!(result["summary"]["kind"], "estimated");
            for (field, expected) in [("estimate", 0), ("standardError", 1)] {
                let difference = (number(&result["summary"][field]) - number(&case["ate"][expected])).abs();
                assert!(difference < 2e-8, "case {} {field}: {difference}", case["case"]);
            }
            for (actual, expected) in result["predictions"].as_array().unwrap().iter().zip(case["prediction"].as_array().unwrap()) {
                assert_eq!(actual["kind"], "estimated");
                assert!((number(&actual["estimate"]) - number(&expected["predictions"])).abs() < 2e-8);
                assert!((number(&actual["uncertainty"]["standardError"]).powi(2) - number(&expected["variance.estimates"])).abs() < 2e-8);
            }
        }
    }

    #[test]
    fn boundary_rejects_wrong_target_and_roles_without_running_a_forest() {
        assert!(serde_json::from_value::<Target>(json!({"kind":"continuous-average","population":"treated"})).is_err());
        assert!(serde_json::from_value::<Honesty>(json!({"kind":"disabled","fraction":0.5})).is_err());
        let data = [0.,0.5,1.,1.5, 1.,2.,3.,4., 0.,1.,2.,3.];
        assert!(fit(&data,4,3,0,1,&[2],Target::BinaryConditional {},configuration(1)).is_err());
        assert!(fit(&data,4,3,0,1,&[1],Target::ContinuousConditional {},configuration(1)).is_err());
    }

    #[test]
    fn weighted_dense_facade_matches_r() {
        let fixture: Value = serde_json::from_str(include_str!("../../causal-core/oracle/grf/fixtures/nuisance.json")).unwrap();
        // The other clustered nuisance fixtures contain missing covariates.
        // This product boundary currently accepts dense prepared matrices only.
        for index in [3, 7] {
            let case = &fixture["cases"][index];
            let rows = case["Y"].as_array().unwrap().len();
            let mut values: Vec<f64> = [&case["W"], &case["Y"]].into_iter().chain(case["X"].as_array().unwrap().iter())
                .flat_map(|c| c.as_array().unwrap().iter().map(number)).collect();
            values.extend(case["labels"].as_array().unwrap().iter().map(number));
            values.extend(if case["weights"].is_null() { vec![1.; rows] } else { case["weights"].as_array().unwrap().iter().map(number).collect() });
            let names: Vec<String> = ["w","y","x1","x2","x3","x4","cluster","weight"].iter().map(|s| s.to_string()).collect();
            let weighting = if case["weights"].is_null() { json!({"kind":"uniform"}) } else { json!({"kind":"column","column":"weight"}) };
            let sampling = if case["equalize"] == true { json!({"kind":"equal-clusters","column":"cluster"}) }
                else if case["per_cluster"].as_u64().unwrap() > 1 { json!({"kind":"clustered","column":"cluster","equalize":false,"weighting":weighting}) }
                else { json!({"kind":"independent","weighting":weighting}) };
            let mut config = configuration(case["seed"].as_u64().unwrap() as u32);
            config.stabilize_splits = case["stabilize"].as_bool().unwrap();
            config.analysis = Some(serde_json::from_value(json!({"sampling":sampling,"averageMethod":"aipw","projection":{"kind":"none"},"ranking":{"kind":"none"}})).unwrap());
            let binary = case["W"].as_array().unwrap().iter().all(|v| number(v) == 0.0 || number(v) == 1.0);
            let target = if binary { Target::BinaryAverage { population: Population::All } } else { Target::ContinuousAverage { population: ContinuousPopulation::All } };
            let result = serde_json::to_value(fit_named(&values, rows, 8, 0, 1, &[2,3,4,5], target, config, &names).unwrap()).unwrap();
            for (field, expected) in [("estimate", 0), ("standardError", 1)] {
                assert!((number(&result["summary"][field]) - number(&case["ate"][expected])).abs() < 2e-8, "case {} {field}", case["case"]);
            }
        }
    }
}
