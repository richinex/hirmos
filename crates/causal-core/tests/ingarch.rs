use hirmos_causal_core::ingarch::{
    detect_negative_binomial_intervention, fit_negative_binomial_ingarch, intervention_regressors,
    IngarchLink, IngarchSpecification, InterventionSchedule,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    observations: Vec<f64>,
    regressors: Vec<Vec<f64>>,
    model: Model,
    fit: OracleFit,
    scenarios: Scenarios,
}

#[derive(Deserialize)]
struct Model {
    past_obs: usize,
    past_mean: usize,
    external: Vec<bool>,
}

#[derive(Deserialize)]
struct OracleFit {
    start: Vec<f64>,
    parameters: Vec<f64>,
    score: Vec<f64>,
    fitted_means: Vec<f64>,
    linear_predictors: Vec<f64>,
    log_likelihood: f64,
    size: f64,
    dispersion: f64,
    outer_iterations: Option<serde_json::Value>,
    function_evaluations: Option<serde_json::Value>,
    gradient_evaluations: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct Scenario {
    regressors: Vec<Vec<f64>>,
    mean: Vec<f64>,
}

#[derive(Deserialize)]
struct Scenarios {
    baseline: Scenario,
    point: Scenario,
    persistent: Scenario,
    decaying: Scenario,
}

#[derive(Deserialize)]
struct IdentityFixture {
    observations: Vec<f64>,
    regressors: Vec<Vec<f64>>,
    fit: OracleFit,
    scenarios: IdentityScenarios,
}

#[derive(Deserialize)]
struct IdentityScenarios {
    control: Scenario,
    treated: Scenario,
}

#[derive(Deserialize)]
struct DetectionFixture {
    observations: Vec<f64>,
    model: DetectionModel,
    fit: DetectionFit,
    detection: DetectionOracle,
}

#[derive(Deserialize)]
struct DetectionModel {
    past_obs: usize,
    past_mean: Vec<usize>,
}

#[derive(Deserialize)]
struct DetectionFit {
    parameters: Vec<f64>,
    fitted_means: Vec<f64>,
    log_likelihood: f64,
    size: f64,
}

#[derive(Deserialize)]
struct DetectionOracle {
    candidates: Vec<usize>,
    score_statistics: Vec<f64>,
    tau_max: usize,
    test_statistic: f64,
}

fn maximum_deviation(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max)
}

#[test]
fn negative_binomial_ingarch_matches_tscount() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("../oracle/fixtures/ingarch.json")).unwrap();
    let specification = IngarchSpecification {
        link: IngarchLink::Log,
        past_observation_lags: vec![fixture.model.past_obs],
        past_mean_lags: vec![fixture.model.past_mean],
        external_regressors: fixture.model.external,
    };
    let fit =
        fit_negative_binomial_ingarch(&fixture.observations, &fixture.regressors, specification)
            .unwrap();

    eprintln!(
        "start={:.3e} params={:.3e} score={:.3e} fitted={:.3e} nu={:.3e} ll={:.3e} size={:.3e}",
        maximum_deviation(&fit.start_parameters, &fixture.fit.start),
        maximum_deviation(&fit.parameters, &fixture.fit.parameters),
        maximum_deviation(&fit.score, &fixture.fit.score),
        maximum_deviation(&fit.fitted_means, &fixture.fit.fitted_means),
        maximum_deviation(&fit.linear_predictors, &fixture.fit.linear_predictors),
        (fit.log_likelihood - fixture.fit.log_likelihood).abs(),
        (fit.size - fixture.fit.size).abs(),
    );
    eprintln!(
        "iterations={} evals={}/{}",
        fit.iterations, fit.function_evaluations, fit.gradient_evaluations
    );

    assert!(maximum_deviation(&fit.start_parameters, &fixture.fit.start) < 1e-14);
    assert!(maximum_deviation(&fit.parameters, &fixture.fit.parameters) < 1e-9);
    assert!(maximum_deviation(&fit.score, &fixture.fit.score) < 1e-7);
    assert!(maximum_deviation(&fit.fitted_means, &fixture.fit.fitted_means) < 1e-8);
    assert!(maximum_deviation(&fit.linear_predictors, &fixture.fit.linear_predictors) < 1e-8);
    assert!((fit.log_likelihood - fixture.fit.log_likelihood).abs() < 1e-8);
    assert!((fit.size - fixture.fit.size).abs() < 1e-8);
    assert!((fit.dispersion - fixture.fit.dispersion).abs() < 1e-9);

    for scenario in [
        &fixture.scenarios.baseline,
        &fixture.scenarios.point,
        &fixture.scenarios.persistent,
        &fixture.scenarios.decaying,
    ] {
        let prediction = fit
            .forecast_mean(
                &fixture.observations,
                &fixture.regressors,
                &scenario.regressors,
            )
            .unwrap();
        assert!(maximum_deviation(&prediction, &scenario.mean) < 1e-8);
    }

    let point = intervention_regressors(
        &fixture.scenarios.baseline.regressors,
        0,
        5.0,
        InterventionSchedule::Point,
    )
    .unwrap();
    let persistent = intervention_regressors(
        &fixture.scenarios.baseline.regressors,
        0,
        5.0,
        InterventionSchedule::Persistent,
    )
    .unwrap();
    let decaying = intervention_regressors(
        &fixture.scenarios.baseline.regressors,
        0,
        5.0,
        InterventionSchedule::Decaying { delta: 0.6 },
    )
    .unwrap();
    assert_eq!(point, fixture.scenarios.point.regressors);
    assert_eq!(persistent, fixture.scenarios.persistent.regressors);
    assert!(
        maximum_deviation(
            &decaying.into_iter().flatten().collect::<Vec<_>>(),
            &fixture
                .scenarios
                .decaying
                .regressors
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>()
        ) < 1e-15
    );
}

#[test]
fn identity_link_negative_binomial_ingarch_matches_tscount_defaults() {
    let fixture: IdentityFixture =
        serde_json::from_str(include_str!("../oracle/fixtures/ingarch_identity.json")).unwrap();
    let specification = IngarchSpecification {
        link: IngarchLink::Identity,
        past_observation_lags: vec![1],
        past_mean_lags: vec![1],
        external_regressors: vec![false],
    };
    let fit =
        fit_negative_binomial_ingarch(&fixture.observations, &fixture.regressors, specification)
            .unwrap();
    let control = fit
        .forecast_mean(
            &fixture.observations,
            &fixture.regressors,
            &fixture.scenarios.control.regressors,
        )
        .unwrap();
    let treated = fit
        .forecast_mean(
            &fixture.observations,
            &fixture.regressors,
            &fixture.scenarios.treated.regressors,
        )
        .unwrap();

    eprintln!(
        "identity start={:.3e} params={:.3e} score={:.3e} fitted={:.3e} nu={:.3e} ll={:.3e} size={:.3e} control={:.3e} treated={:.3e}",
        maximum_deviation(&fit.start_parameters, &fixture.fit.start),
        maximum_deviation(&fit.parameters, &fixture.fit.parameters),
        maximum_deviation(&fit.score, &fixture.fit.score),
        maximum_deviation(&fit.fitted_means, &fixture.fit.fitted_means),
        maximum_deviation(&fit.linear_predictors, &fixture.fit.linear_predictors),
        (fit.log_likelihood - fixture.fit.log_likelihood).abs(),
        (fit.size - fixture.fit.size).abs(),
        maximum_deviation(&control, &fixture.scenarios.control.mean),
        maximum_deviation(&treated, &fixture.scenarios.treated.mean),
    );
    assert!(maximum_deviation(&fit.start_parameters, &fixture.fit.start) < 1e-12);
    assert!(maximum_deviation(&fit.parameters, &fixture.fit.parameters) < 1e-8);
    assert!(maximum_deviation(&fit.score, &fixture.fit.score) < 1e-7);
    assert!(maximum_deviation(&fit.fitted_means, &fixture.fit.fitted_means) < 1e-7);
    assert!(maximum_deviation(&fit.linear_predictors, &fixture.fit.linear_predictors) < 1e-7);
    assert!((fit.log_likelihood - fixture.fit.log_likelihood).abs() < 1e-7);
    assert!((fit.size - fixture.fit.size).abs() < 1e-7);
    assert!((fit.dispersion - fixture.fit.dispersion).abs() < 1e-8);
    assert!(maximum_deviation(&control, &fixture.scenarios.control.mean) < 1e-7);
    assert!(maximum_deviation(&treated, &fixture.scenarios.treated.mean) < 1e-7);
    assert_eq!(
        fit.iterations as u64,
        fixture.fit.outer_iterations.unwrap().as_u64().unwrap()
    );
    assert_eq!(
        fit.function_evaluations as u64,
        fixture.fit.function_evaluations.unwrap().as_u64().unwrap()
    );
    assert_eq!(
        fit.gradient_evaluations as u64,
        fixture.fit.gradient_evaluations.unwrap().as_u64().unwrap()
    );
}

#[test]
fn multi_lag_campy_fit_and_unknown_time_detection_match_tscount() {
    let fixture: DetectionFixture =
        serde_json::from_str(include_str!("../oracle/fixtures/ingarch_detection.json")).unwrap();
    let regressors = vec![Vec::new(); fixture.observations.len()];
    let specification = IngarchSpecification {
        link: IngarchLink::Identity,
        past_observation_lags: vec![fixture.model.past_obs],
        past_mean_lags: fixture.model.past_mean,
        external_regressors: vec![],
    };
    let candidates: Vec<usize> = fixture
        .detection
        .candidates
        .iter()
        .map(|tau| tau - 1)
        .collect();
    let detection = detect_negative_binomial_intervention(
        &fixture.observations,
        &regressors,
        specification,
        &candidates,
        1.0,
    )
    .unwrap();
    let score_statistics: Vec<f64> = detection
        .candidates
        .iter()
        .map(|candidate| candidate.score_statistic)
        .collect();

    eprintln!(
        "campy params={:.3e} fitted={:.3e} ll={:.3e} size={:.3e} scores={:.3e} max={:.3e}",
        maximum_deviation(&detection.null_fit.parameters, &fixture.fit.parameters),
        maximum_deviation(&detection.null_fit.fitted_means, &fixture.fit.fitted_means),
        (detection.null_fit.log_likelihood - fixture.fit.log_likelihood).abs(),
        (detection.null_fit.size - fixture.fit.size).abs(),
        maximum_deviation(&score_statistics, &fixture.detection.score_statistics),
        (detection
            .candidates
            .iter()
            .map(|candidate| candidate.score_statistic)
            .fold(f64::NEG_INFINITY, f64::max)
            - fixture.detection.test_statistic)
            .abs(),
    );
    assert!(maximum_deviation(&detection.null_fit.parameters, &fixture.fit.parameters) < 1e-8);
    assert!(maximum_deviation(&detection.null_fit.fitted_means, &fixture.fit.fitted_means) < 2e-8);
    assert!((detection.null_fit.log_likelihood - fixture.fit.log_likelihood).abs() < 1e-8);
    assert!((detection.null_fit.size - fixture.fit.size).abs() < 1e-8);
    assert!(maximum_deviation(&score_statistics, &fixture.detection.score_statistics) < 1e-7);
    assert_eq!(
        detection.strongest_reference_point + 1,
        fixture.detection.tau_max
    );
    assert!(
        (score_statistics[candidates
            .iter()
            .position(|point| *point == detection.strongest_reference_point)
            .unwrap()]
            - fixture.detection.test_statistic)
            .abs()
            < 1e-7
    );
}
