use hirmos_causal_core::survival::coxph::{
    compute_right_censored_residuals, evaluate_right_censored, evaluate_time_varying,
    fit_right_censored, fit_time_varying, predict_expectation, predict_percentile,
    predict_survival, test_proportional_hazards, BatchMode, CoxCovariates, CoxDataError,
    CoxFitError, CoxFitOptions, CoxPenalty, CoxProfile, Event, PredictionError, ResidualError,
    RightCensoredData, StandardErrorMethod, TimeTransform, TimeVaryingData,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Fixture {
    right_censored: Vec<RightCase>,
    time_varying: Vec<TimeCase>,
}

#[derive(Deserialize)]
struct RightCase {
    name: String,
    input: RightInput,
    normalization: Normalization,
    fit: ExpectedFit,
    baseline: RightBaseline,
    probes: Option<Probes>,
    residuals: Option<ExpectedResiduals>,
    proportional_hazards: Option<HashMap<String, ExpectedHazardsTest>>,
    predictions: ExpectedPredictions,
}

#[derive(Deserialize)]
struct RightInput {
    duration: Vec<f64>,
    event: Vec<bool>,
    covariates: Vec<Vec<f64>>,
    weights: Option<Vec<f64>>,
    entry: Option<Vec<f64>>,
    strata: Option<Vec<String>>,
    penalizer: FixturePenalty,
    l1_ratio: f64,
    batch_mode: Option<bool>,
    #[serde(default)]
    robust: bool,
    cluster: Option<Vec<usize>>,
    initial_point: Option<Vec<f64>>,
    alpha: f64,
}

#[derive(Deserialize)]
struct TimeCase {
    name: String,
    input: TimeInput,
    normalization: Normalization,
    fit: ExpectedFit,
    baseline: TimeBaseline,
    probe: ExpectedEvaluation,
}

#[derive(Deserialize)]
struct TimeInput {
    subject: Vec<usize>,
    start: Vec<f64>,
    stop: Vec<f64>,
    event: Vec<bool>,
    covariates: Vec<Vec<f64>>,
    weights: Option<Vec<f64>>,
    strata: Option<Vec<String>>,
    penalizer: FixturePenalty,
    l1_ratio: f64,
    initial_point: Option<Vec<f64>>,
    alpha: f64,
}

#[derive(Deserialize)]
struct Normalization {
    mean: Vec<f64>,
    std: Vec<f64>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum FixturePenalty {
    Uniform(f64),
    ByCoefficient(Vec<f64>),
}

impl FixturePenalty {
    fn to_model(&self) -> CoxPenalty {
        match self {
            Self::Uniform(value) => CoxPenalty::Uniform(*value),
            Self::ByCoefficient(values) => CoxPenalty::ByCoefficient(values.clone()),
        }
    }
}

#[derive(Deserialize)]
struct ExpectedFit {
    coefficients: Vec<f64>,
    hazard_ratios: Vec<f64>,
    variance: Vec<Vec<f64>>,
    standard_errors: Vec<f64>,
    z: Vec<f64>,
    p: Vec<f64>,
    coefficient_lower: Vec<f64>,
    coefficient_upper: Vec<f64>,
    log_likelihood: f64,
    partial_aic: f64,
    log_likelihood_ratio: f64,
    log_likelihood_ratio_p: f64,
    log_partial_hazards: Vec<f64>,
    #[serde(default)]
    concordance: Option<f64>,
}

#[derive(Deserialize)]
struct RightBaseline {
    times: Vec<f64>,
    hazard: Vec<Vec<f64>>,
    cumulative_hazard: Vec<Vec<f64>>,
    survival: Vec<Vec<f64>>,
    strata: Option<Vec<ExpectedStratumBaseline>>,
}

#[derive(Deserialize)]
struct ExpectedStratumBaseline {
    label: String,
    times: Vec<f64>,
    hazard: Vec<f64>,
    cumulative_hazard: Vec<f64>,
    survival: Vec<f64>,
}

#[derive(Deserialize)]
struct TimeBaseline {
    times: Vec<f64>,
    cumulative_hazard: Vec<Vec<f64>>,
    survival: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
struct ExpectedResiduals {
    martingale: Vec<Vec<f64>>,
    deviance: Vec<Vec<f64>>,
    schoenfeld: Vec<Vec<f64>>,
    scaled_schoenfeld: Vec<Vec<f64>>,
    score: Vec<Vec<f64>>,
    delta_beta: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
struct ExpectedHazardsTest {
    statistic: Vec<f64>,
    p: Vec<f64>,
}

#[derive(Deserialize)]
struct ExpectedPredictions {
    covariates: Vec<Vec<f64>>,
    strata: Option<Vec<String>>,
    times: Vec<f64>,
    conditional_after: Vec<f64>,
    log_partial_hazard: Vec<f64>,
    partial_hazard: Vec<f64>,
    cumulative_hazard: Vec<Vec<f64>>,
    survival: Vec<Vec<f64>>,
    conditional_cumulative_hazard: Vec<Vec<f64>>,
    conditional_survival: Vec<Vec<f64>>,
    median: Vec<f64>,
    expectation: Vec<f64>,
    conditional_median: Vec<f64>,
    conditional_expectation: Vec<f64>,
}

#[derive(Deserialize)]
struct Probes {
    beta: Vec<f64>,
    single: ExpectedEvaluation,
    batch: ExpectedEvaluation,
}

#[derive(Deserialize)]
struct ExpectedEvaluation {
    #[serde(default)]
    beta: Vec<f64>,
    hessian: Vec<Vec<f64>>,
    gradient: Vec<f64>,
    log_likelihood: f64,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../oracle/fixtures/lifelines_coxph.json"))
        .expect("lifelines Cox fixture")
}

fn flatten(rows: &[Vec<f64>]) -> Vec<f64> {
    rows.iter().flatten().copied().collect()
}

fn last_column(rows: &[Vec<f64>]) -> Vec<f64> {
    rows.iter()
        .map(|row| *row.last().expect("residual fixture row"))
        .collect()
}

fn shared_baseline(
    fit: &hirmos_causal_core::survival::coxph::CoxFit,
) -> &[hirmos_causal_core::survival::coxph::BaselineEstimate] {
    fit.baseline.shared().expect("unstratified fixture")
}

fn events(values: &[bool]) -> Vec<Event> {
    values
        .iter()
        .map(|value| {
            if *value {
                Event::Observed
            } else {
                Event::Censored
            }
        })
        .collect()
}

fn assert_close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    let difference = (actual - expected).abs();
    let allowed = tolerance * (1.0 + expected.abs());
    assert!(
        difference <= allowed,
        "{label}: actual={actual:.17e}, expected={expected:.17e}, difference={difference:.3e}, tolerance={allowed:.3e}"
    );
}

fn assert_slice(label: &str, actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(actual.len(), expected.len(), "{label} length");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(&format!("{label}[{index}]"), *actual, *expected, tolerance);
    }
}

fn right_data(input: &RightInput) -> RightCensoredData {
    let rows = input.duration.len();
    let columns = input.covariates[0].len();
    let covariates = CoxCovariates::new(rows, columns, flatten(&input.covariates)).unwrap();
    let weights = input.weights.clone().unwrap_or_else(|| vec![1.0; rows]);
    let strata = input.strata.as_ref().map(|labels| {
        let mut unique = labels.clone();
        unique.sort();
        unique.dedup();
        labels
            .iter()
            .map(|label| unique.binary_search(label).expect("known stratum label"))
            .collect::<Vec<_>>()
    });
    if strata.is_some() || input.cluster.is_some() {
        RightCensoredData::with_grouping(
            input.duration.clone(),
            events(&input.event),
            weights,
            input.entry.clone(),
            strata,
            input.cluster.clone(),
            covariates,
        )
        .unwrap()
    } else {
        RightCensoredData::with_options(
            input.duration.clone(),
            events(&input.event),
            weights,
            input.entry.clone(),
            covariates,
        )
        .unwrap()
    }
}

fn options(input: &RightInput) -> CoxFitOptions {
    CoxFitOptions {
        penalizer: input.penalizer.to_model(),
        l1_ratio: input.l1_ratio,
        batch_mode: match input.batch_mode {
            Some(true) => BatchMode::Batch,
            Some(false) => BatchMode::Single,
            None => BatchMode::Automatic,
        },
        standard_errors: if input.cluster.is_some() {
            StandardErrorMethod::Clustered
        } else if input.robust {
            StandardErrorMethod::Sandwich
        } else {
            StandardErrorMethod::ModelBased
        },
        initial_point: input.initial_point.clone(),
        alpha: input.alpha,
        ..CoxFitOptions::default()
    }
}

fn time_data(input: &TimeInput) -> TimeVaryingData {
    let rows = input.start.len();
    let columns = input.covariates[0].len();
    let strata = input.strata.as_ref().map(|labels| {
        let mut unique = labels.clone();
        unique.sort();
        unique.dedup();
        labels
            .iter()
            .map(|label| unique.binary_search(label).expect("known stratum label"))
            .collect::<Vec<_>>()
    });
    TimeVaryingData::with_strata(
        input.subject.clone(),
        input.start.clone(),
        input.stop.clone(),
        events(&input.event),
        input.weights.clone().unwrap_or_else(|| vec![1.0; rows]),
        strata,
        CoxCovariates::new(rows, columns, flatten(&input.covariates)).unwrap(),
    )
    .unwrap()
}

fn time_options(input: &TimeInput) -> CoxFitOptions {
    CoxFitOptions {
        precision: 1e-8,
        maximum_steps: 50,
        penalizer: input.penalizer.to_model(),
        l1_ratio: input.l1_ratio,
        initial_point: input.initial_point.clone(),
        alpha: input.alpha,
        ..CoxFitOptions::default()
    }
}

fn compare_fit(
    name: &str,
    actual: &hirmos_causal_core::survival::coxph::CoxFit,
    expected: &ExpectedFit,
) {
    assert_slice(
        &format!("{name} coefficients"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.coefficient)
            .collect::<Vec<_>>(),
        &expected.coefficients,
        3e-8,
    );
    assert_slice(
        &format!("{name} hazard ratios"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.hazard_ratio)
            .collect::<Vec<_>>(),
        &expected.hazard_ratios,
        3e-8,
    );
    assert_slice(
        &format!("{name} covariance"),
        &actual.covariance,
        &flatten(&expected.variance),
        8e-8,
    );
    assert_slice(
        &format!("{name} standard errors"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.standard_error)
            .collect::<Vec<_>>(),
        &expected.standard_errors,
        3e-8,
    );
    assert_slice(
        &format!("{name} z"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.z)
            .collect::<Vec<_>>(),
        &expected.z,
        5e-8,
    );
    assert_slice(
        &format!("{name} p"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.p_value)
            .collect::<Vec<_>>(),
        &expected.p,
        5e-8,
    );
    assert_slice(
        &format!("{name} lower"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.lower)
            .collect::<Vec<_>>(),
        &expected.coefficient_lower,
        3e-8,
    );
    assert_slice(
        &format!("{name} upper"),
        &actual
            .coefficients
            .iter()
            .map(|value| value.upper)
            .collect::<Vec<_>>(),
        &expected.coefficient_upper,
        3e-8,
    );
    assert_close(
        &format!("{name} log likelihood"),
        actual.log_likelihood,
        expected.log_likelihood,
        2e-9,
    );
    assert_close(
        &format!("{name} partial AIC"),
        actual.partial_aic,
        expected.partial_aic,
        2e-9,
    );
    assert_close(
        &format!("{name} LR"),
        actual.likelihood_ratio,
        expected.log_likelihood_ratio,
        2e-8,
    );
    assert_close(
        &format!("{name} LR p"),
        actual.likelihood_ratio_p_value,
        expected.log_likelihood_ratio_p,
        5e-8,
    );
    assert_slice(
        &format!("{name} log partial hazards"),
        &actual.log_partial_hazards,
        &expected.log_partial_hazards,
        3e-8,
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn right_censored_coxph_matches_lifelines_0303() {
    for case in fixture().right_censored {
        let data = right_data(&case.input);
        let actual = fit_right_censored(&data, &options(&case.input))
            .unwrap_or_else(|problem| panic!("{} failed: {problem:?}", case.name));
        assert_slice(
            &format!("{} means", case.name),
            &actual.model.covariate_means,
            &case.normalization.mean,
            2e-15,
        );
        assert_slice(
            &format!("{} std", case.name),
            &actual.model.covariate_standard_deviations,
            &case.normalization.std,
            2e-15,
        );
        compare_fit(&case.name, &actual.model, &case.fit);
        assert_close(
            &format!("{} concordance", case.name),
            actual
                .concordance_index
                .expect("fixture has comparable observation pairs")
                .value(),
            case.fit
                .concordance
                .expect("right-censored fixture concordance"),
            2e-15,
        );
        if let Some(expected_strata) = &case.baseline.strata {
            let mut labels = case.input.strata.clone().expect("stratum labels");
            labels.sort();
            labels.dedup();
            let actual_strata = actual
                .model
                .baseline
                .stratified()
                .expect("stratified fixture");
            assert_eq!(actual_strata.len(), expected_strata.len());
            for (actual_curve, expected_curve) in actual_strata.iter().zip(expected_strata) {
                assert_eq!(labels[actual_curve.stratum], expected_curve.label);
                assert_slice(
                    &format!("{} stratum {} times", case.name, expected_curve.label),
                    &actual_curve
                        .estimates
                        .iter()
                        .map(|value| value.time)
                        .collect::<Vec<_>>(),
                    &expected_curve.times,
                    0.0,
                );
                assert_slice(
                    &format!("{} stratum {} hazard", case.name, expected_curve.label),
                    &actual_curve
                        .estimates
                        .iter()
                        .map(|value| value.hazard)
                        .collect::<Vec<_>>(),
                    &expected_curve.hazard,
                    3e-8,
                );
                assert_slice(
                    &format!("{} stratum {} cumulative", case.name, expected_curve.label),
                    &actual_curve
                        .estimates
                        .iter()
                        .map(|value| value.cumulative_hazard)
                        .collect::<Vec<_>>(),
                    &expected_curve.cumulative_hazard,
                    3e-8,
                );
                assert_slice(
                    &format!("{} stratum {} survival", case.name, expected_curve.label),
                    &actual_curve
                        .estimates
                        .iter()
                        .map(|value| value.survival)
                        .collect::<Vec<_>>(),
                    &expected_curve.survival,
                    3e-8,
                );
            }
        } else {
            assert_slice(
                &format!("{} baseline times", case.name),
                &shared_baseline(&actual.model)
                    .iter()
                    .map(|value| value.time)
                    .collect::<Vec<_>>(),
                &case.baseline.times,
                0.0,
            );
            assert_slice(
                &format!("{} baseline hazard", case.name),
                &shared_baseline(&actual.model)
                    .iter()
                    .map(|value| value.hazard)
                    .collect::<Vec<_>>(),
                &flatten(&case.baseline.hazard),
                3e-8,
            );
            assert_slice(
                &format!("{} baseline cumulative", case.name),
                &shared_baseline(&actual.model)
                    .iter()
                    .map(|value| value.cumulative_hazard)
                    .collect::<Vec<_>>(),
                &flatten(&case.baseline.cumulative_hazard),
                3e-8,
            );
            assert_slice(
                &format!("{} baseline survival", case.name),
                &shared_baseline(&actual.model)
                    .iter()
                    .map(|value| value.survival)
                    .collect::<Vec<_>>(),
                &flatten(&case.baseline.survival),
                3e-8,
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn right_censored_residuals_match_lifelines_0303() {
    for case in fixture().right_censored {
        let Some(expected) = case.residuals else {
            continue;
        };
        let data = right_data(&case.input);
        let fitted = fit_right_censored(&data, &options(&case.input))
            .unwrap_or_else(|problem| panic!("{} failed: {problem:?}", case.name));
        let actual = compute_right_censored_residuals(&data, &fitted.model)
            .unwrap_or_else(|problem| panic!("{} residuals failed: {problem:?}", case.name));
        assert_slice(
            &format!("{} martingale", case.name),
            &actual.martingale,
            &last_column(&expected.martingale),
            6e-8,
        );
        assert_slice(
            &format!("{} deviance", case.name),
            &actual.deviance,
            &last_column(&expected.deviance),
            6e-8,
        );
        assert_slice(
            &format!("{} schoenfeld", case.name),
            &actual.schoenfeld,
            &flatten(&expected.schoenfeld),
            6e-8,
        );
        assert_slice(
            &format!("{} scaled schoenfeld", case.name),
            &actual.scaled_schoenfeld,
            &flatten(&expected.scaled_schoenfeld),
            8e-8,
        );
        assert_slice(
            &format!("{} score", case.name),
            &actual.score,
            &flatten(&expected.score),
            8e-8,
        );
        assert_slice(
            &format!("{} delta beta", case.name),
            &actual.delta_beta,
            &flatten(&expected.delta_beta),
            8e-8,
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn proportional_hazards_tests_match_lifelines_0303() {
    for case in fixture().right_censored {
        let Some(expected) = case.proportional_hazards else {
            continue;
        };
        let data = right_data(&case.input);
        let fitted = fit_right_censored(&data, &options(&case.input))
            .unwrap_or_else(|problem| panic!("{} failed: {problem:?}", case.name));
        for (name, transform) in [
            ("rank", TimeTransform::EventRank),
            ("km", TimeTransform::KaplanMeier),
            ("identity", TimeTransform::Identity),
            ("log", TimeTransform::LogTime),
        ] {
            let actual = test_proportional_hazards(&data, &fitted.model, transform)
                .unwrap_or_else(|problem| panic!("{} {name} failed: {problem:?}", case.name));
            let expected = &expected[name];
            assert_slice(
                &format!("{} {name} PH statistic", case.name),
                &actual.iter().map(|test| test.statistic).collect::<Vec<_>>(),
                &expected.statistic,
                2e-7,
            );
            assert_slice(
                &format!("{} {name} PH p", case.name),
                &actual.iter().map(|test| test.p_value).collect::<Vec<_>>(),
                &expected.p,
                2e-7,
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn right_censored_predictions_match_lifelines_0303() {
    for case in fixture().right_censored {
        let data = right_data(&case.input);
        let fitted = fit_right_censored(&data, &options(&case.input))
            .unwrap_or_else(|problem| panic!("{} failed: {problem:?}", case.name));
        let profiles = match &case.predictions.strata {
            Some(labels) => {
                let mut known = case.input.strata.clone().expect("stratified input");
                known.sort();
                known.dedup();
                labels
                    .iter()
                    .zip(&case.predictions.covariates)
                    .map(|(label, covariates)| CoxProfile::Stratified {
                        stratum: known.binary_search(label).expect("known stratum"),
                        covariates: covariates.clone(),
                    })
                    .collect::<Vec<_>>()
            }
            None => case
                .predictions
                .covariates
                .iter()
                .cloned()
                .map(CoxProfile::Shared)
                .collect(),
        };
        let actual = predict_survival(&fitted.model, &profiles, &case.predictions.times, None)
            .unwrap_or_else(|problem| panic!("{} prediction failed: {problem:?}", case.name));
        assert_slice(
            &format!("{} prediction log partial hazard", case.name),
            &actual
                .iter()
                .map(|value| value.log_partial_hazard)
                .collect::<Vec<_>>(),
            &case.predictions.log_partial_hazard,
            3e-8,
        );
        assert_slice(
            &format!("{} prediction partial hazard", case.name),
            &actual
                .iter()
                .map(|value| value.partial_hazard)
                .collect::<Vec<_>>(),
            &case.predictions.partial_hazard,
            3e-8,
        );
        let cumulative = (0..case.predictions.times.len())
            .flat_map(|time| {
                actual
                    .iter()
                    .map(move |profile| profile.values[time].cumulative_hazard)
            })
            .collect::<Vec<_>>();
        let survival = (0..case.predictions.times.len())
            .flat_map(|time| {
                actual
                    .iter()
                    .map(move |profile| profile.values[time].survival)
            })
            .collect::<Vec<_>>();
        assert_slice(
            &format!("{} prediction cumulative hazard", case.name),
            &cumulative,
            &flatten(&case.predictions.cumulative_hazard),
            5e-8,
        );
        assert_slice(
            &format!("{} prediction survival", case.name),
            &survival,
            &flatten(&case.predictions.survival),
            5e-8,
        );

        let conditional = predict_survival(
            &fitted.model,
            &profiles,
            &case.predictions.times,
            Some(&case.predictions.conditional_after),
        )
        .unwrap_or_else(|problem| {
            panic!("{} conditional prediction failed: {problem:?}", case.name)
        });
        let conditional_cumulative = (0..case.predictions.times.len())
            .flat_map(|time| {
                conditional
                    .iter()
                    .map(move |profile| profile.values[time].cumulative_hazard)
            })
            .collect::<Vec<_>>();
        let conditional_survival = (0..case.predictions.times.len())
            .flat_map(|time| {
                conditional
                    .iter()
                    .map(move |profile| profile.values[time].survival)
            })
            .collect::<Vec<_>>();
        assert_slice(
            &format!("{} conditional cumulative hazard", case.name),
            &conditional_cumulative,
            &flatten(&case.predictions.conditional_cumulative_hazard),
            5e-8,
        );
        assert_slice(
            &format!("{} conditional survival", case.name),
            &conditional_survival,
            &flatten(&case.predictions.conditional_survival),
            5e-8,
        );
        assert_slice(
            &format!("{} median", case.name),
            &predict_percentile(&fitted.model, &profiles, 0.5, None).unwrap(),
            &case.predictions.median,
            0.0,
        );
        assert_slice(
            &format!("{} expectation", case.name),
            &predict_expectation(&fitted.model, &profiles, None).unwrap(),
            &case.predictions.expectation,
            5e-8,
        );
        assert_slice(
            &format!("{} conditional median", case.name),
            &predict_percentile(
                &fitted.model,
                &profiles,
                0.5,
                Some(&case.predictions.conditional_after),
            )
            .unwrap(),
            &case.predictions.conditional_median,
            0.0,
        );
        assert_slice(
            &format!("{} conditional expectation", case.name),
            &predict_expectation(
                &fitted.model,
                &profiles,
                Some(&case.predictions.conditional_after),
            )
            .unwrap(),
            &case.predictions.conditional_expectation,
            5e-8,
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn efron_gradients_match_both_lifelines_execution_lanes() {
    for case in fixture().right_censored {
        let Some(probes) = case.probes else { continue };
        let data = right_data(&case.input);
        for (mode, expected) in [
            (BatchMode::Single, probes.single),
            (BatchMode::Batch, probes.batch),
        ] {
            let actual = evaluate_right_censored(&data, &probes.beta, mode).unwrap();
            assert_slice(
                &format!("{} {mode:?} hessian", case.name),
                &actual.hessian,
                &flatten(&expected.hessian),
                3e-13,
            );
            assert_slice(
                &format!("{} {mode:?} gradient", case.name),
                &actual.gradient,
                &expected.gradient,
                3e-13,
            );
            assert_close(
                &format!("{} {mode:?} ll", case.name),
                actual.log_likelihood,
                expected.log_likelihood,
                3e-13,
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn time_varying_coxph_matches_lifelines_0303() {
    for case in fixture().time_varying {
        let data = time_data(&case.input);
        let actual = fit_time_varying(&data, &time_options(&case.input))
            .unwrap_or_else(|problem| panic!("{} failed: {problem:?}", case.name));
        assert_slice(
            &format!("{} means", case.name),
            &actual.covariate_means,
            &case.normalization.mean,
            2e-15,
        );
        assert_slice(
            &format!("{} std", case.name),
            &actual.covariate_standard_deviations,
            &case.normalization.std,
            2e-15,
        );
        compare_fit(&case.name, &actual, &case.fit);
        assert_slice(
            &format!("{} baseline times", case.name),
            &shared_baseline(&actual)
                .iter()
                .map(|value| value.time)
                .collect::<Vec<_>>(),
            &case.baseline.times,
            0.0,
        );
        assert_slice(
            &format!("{} baseline cumulative", case.name),
            &shared_baseline(&actual)
                .iter()
                .map(|value| value.cumulative_hazard)
                .collect::<Vec<_>>(),
            &flatten(&case.baseline.cumulative_hazard),
            3e-8,
        );
        assert_slice(
            &format!("{} baseline survival", case.name),
            &shared_baseline(&actual)
                .iter()
                .map(|value| value.survival)
                .collect::<Vec<_>>(),
            &flatten(&case.baseline.survival),
            3e-8,
        );
        let probe = evaluate_time_varying(&data, &case.probe.beta).unwrap();
        assert_slice(
            &format!("{} probe Hessian", case.name),
            &probe.hessian,
            &flatten(&case.probe.hessian),
            3e-13,
        );
        assert_slice(
            &format!("{} probe gradient", case.name),
            &probe.gradient,
            &case.probe.gradient,
            3e-13,
        );
        assert_close(
            &format!("{} probe log likelihood", case.name),
            probe.log_likelihood,
            case.probe.log_likelihood,
            3e-13,
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn shared_models_agree_with_survival_364() {
    let fixture = fixture();
    let mut fitted = HashMap::new();
    for (source_name, case_name) in [
        ("right_censored", "three_covariates_single"),
        ("stratified", "stratified"),
    ] {
        let case = fixture
            .right_censored
            .iter()
            .find(|case| case.name == case_name)
            .expect("right-censored cross-check case");
        let result = fit_right_censored(&right_data(&case.input), &options(&case.input))
            .unwrap_or_else(|problem| panic!("{case_name} failed: {problem:?}"));
        fitted.insert(source_name, result.model);
    }
    let time_case = fixture
        .time_varying
        .iter()
        .find(|case| case.name == "two_covariates")
        .expect("start-stop cross-check case");
    fitted.insert(
        "start_stop",
        fit_time_varying(
            &time_data(&time_case.input),
            &time_options(&time_case.input),
        )
        .expect("start-stop cross-check fit"),
    );

    for (line_number, line) in include_str!("../oracle/fixtures/survival_coxph_crosscheck.csv")
        .lines()
        .skip(1)
        .enumerate()
    {
        let fields = line.split(',').collect::<Vec<_>>();
        assert_eq!(fields.len(), 5, "cross-check row {}", line_number + 2);
        let model = &fitted[fields[0]];
        let coefficient_index = match fields[1] {
            "usage" | "assistant" => 0,
            "experience" | "pressure" => 1,
            "complexity" => 2,
            term => panic!("unknown cross-check term {term}"),
        };
        let expected_coefficient = fields[2].parse::<f64>().expect("R coefficient");
        let expected_standard_error = fields[3].parse::<f64>().expect("R standard error");
        let expected_log_likelihood = fields[4].parse::<f64>().expect("R log likelihood");
        assert_close(
            &format!("{} {} R coefficient", fields[0], fields[1]),
            model.coefficients[coefficient_index].coefficient,
            expected_coefficient,
            1e-5,
        );
        assert_close(
            &format!("{} {} R standard error", fields[0], fields[1]),
            model.coefficients[coefficient_index].standard_error,
            expected_standard_error,
            1e-6,
        );
        assert_close(
            &format!("{} R log likelihood", fields[0]),
            model.log_likelihood,
            expected_log_likelihood,
            1e-10,
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_cox_states_are_rejected() {
    assert_eq!(
        CoxCovariates::new(0, 1, vec![]),
        Err(CoxDataError::EmptyRows)
    );
    assert_eq!(
        CoxCovariates::new(2, 1, vec![1.0, 1.0]),
        Err(CoxDataError::ConstantCovariate { column: 0 })
    );

    let covariates = || CoxCovariates::new(2, 1, vec![0.0, 1.0]).unwrap();
    assert_eq!(
        RightCensoredData::new(
            vec![1.0],
            vec![Event::Observed, Event::Censored],
            covariates(),
        ),
        Err(CoxDataError::RowCount {
            expected: 2,
            actual: 1,
        })
    );
    assert_eq!(
        RightCensoredData::new(
            vec![1.0, f64::NAN],
            vec![Event::Observed, Event::Censored],
            covariates(),
        ),
        Err(CoxDataError::InvalidDuration { row: 1 })
    );
    assert_eq!(
        RightCensoredData::with_options(
            vec![1.0, 2.0],
            vec![Event::Observed, Event::Censored],
            vec![1.0, 0.0],
            None,
            covariates(),
        ),
        Err(CoxDataError::InvalidWeight { row: 1 })
    );
    assert_eq!(
        RightCensoredData::with_options(
            vec![1.0, 2.0],
            vec![Event::Observed, Event::Censored],
            vec![1.0, 1.0],
            Some(vec![1.0, 0.0]),
            covariates(),
        ),
        Err(CoxDataError::InvalidEntry { row: 0 })
    );
    assert_eq!(
        RightCensoredData::new(
            vec![1.0, 2.0],
            vec![Event::Censored, Event::Censored],
            covariates(),
        ),
        Err(CoxDataError::NoEvents)
    );

    assert_eq!(
        TimeVaryingData::new(
            vec![1, 2],
            vec![0.0, 2.0],
            vec![1.0, 1.0],
            vec![Event::Observed, Event::Censored],
            covariates(),
        ),
        Err(CoxDataError::InvalidInterval { row: 1 })
    );
    assert_eq!(
        TimeVaryingData::new(
            vec![1, 2],
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![Event::Observed, Event::Censored],
            covariates(),
        ),
        Err(CoxDataError::ImmediateEvent { row: 0 })
    );

    let fixture = fixture();
    let right_case = fixture
        .right_censored
        .iter()
        .find(|case| case.name == "three_covariates_single")
        .unwrap();
    let right_censored_data = right_data(&right_case.input);
    let mut invalid_options = options(&right_case.input);
    invalid_options.penalizer = CoxPenalty::ByCoefficient(vec![0.1]);
    assert_eq!(
        fit_right_censored(&right_censored_data, &invalid_options),
        Err(CoxFitError::InvalidConfiguration)
    );
    let mut clustered_options = options(&right_case.input);
    clustered_options.standard_errors = StandardErrorMethod::Clustered;
    assert_eq!(
        fit_right_censored(&right_censored_data, &clustered_options),
        Err(CoxFitError::InvalidConfiguration)
    );

    let fitted = fit_right_censored(&right_censored_data, &options(&right_case.input)).unwrap();
    let profiles = [CoxProfile::Shared(right_case.input.covariates[0].clone())];
    assert_eq!(
        predict_percentile(&fitted.model, &profiles, 0.5, Some(&[])),
        Err(PredictionError::InvalidConditionalTime)
    );
    assert_eq!(
        predict_expectation(&fitted.model, &[], None),
        Err(PredictionError::EmptyProfiles)
    );

    let delayed_case = fixture
        .right_censored
        .iter()
        .find(|case| case.name == "delayed_entry")
        .unwrap();
    let delayed_data = right_data(&delayed_case.input);
    let delayed_fit = fit_right_censored(&delayed_data, &options(&delayed_case.input)).unwrap();
    assert_eq!(
        compute_right_censored_residuals(&delayed_data, &delayed_fit.model),
        Err(ResidualError::DelayedEntryUnsupported)
    );

    let time_case = fixture
        .time_varying
        .iter()
        .find(|case| case.name == "two_covariates")
        .unwrap();
    let mut unsupported_options = time_options(&time_case.input);
    unsupported_options.standard_errors = StandardErrorMethod::Sandwich;
    assert_eq!(
        fit_time_varying(&time_data(&time_case.input), &unsupported_options),
        Err(CoxFitError::InvalidConfiguration)
    );
}
