use std::num::NonZeroUsize;

use hirmos_causal_core::survival::flexsurv::fit::{
    fit, FlexSurvFit, FlexSurvFitPlan, SurvivalDataset, SurvivalRecord,
};
use hirmos_causal_core::survival::flexsurv::model::{
    CovariateMatrix, FlexSurvFamily, RegressionModel,
};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use hirmos_causal_core::survival::flexsurv::standardize::{
    standardize, ContrastKind, IntervalTransform, StandardizationContrast, StandardizationPlan,
    StandardizationRequest, StandardizationUncertainty,
};
use hirmos_causal_core::survival::flexsurv::uncertainty::PredictionSimulationConfiguration;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

fn problem() -> (FlexSurvFit, Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<f64>) {
    let fixture = fixture();
    let ovarian = &fixture["ovarian"];
    let standard = &fixture["standardized_predictions"];
    let times = ovarian["futime"].as_array().unwrap();
    let events = ovarian["fustat"].as_array().unwrap();
    let ages = ovarian["age"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let treatments = ovarian["rx_indicator"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let weights = standard["weights"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let records = times
        .iter()
        .zip(events)
        .zip(&weights)
        .map(|((time, event), weight)| {
            let observation = if event.as_i64().unwrap() == 1 {
                SurvivalObservation::exact(number(time)).unwrap()
            } else {
                SurvivalObservation::right_censored(number(time)).unwrap()
            };
            SurvivalRecord::weighted(observation, *weight).unwrap()
        })
        .collect::<Vec<_>>();
    let covariates = treatments
        .iter()
        .zip(&ages)
        .flat_map(|(treatment, age)| [*treatment, *age])
        .collect::<Vec<_>>();
    let model = RegressionModel::new(FlexSurvFamily::WeibullPh, times.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(times.len(), 2, covariates).unwrap())
        .unwrap();
    let plan = FlexSurvFitPlan::optimize(
        model,
        SurvivalDataset::new(records).unwrap(),
        &[1.2, 0.001],
        &[0.0, 0.0],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(4).unwrap()),
    )
    .unwrap();
    let fit = fit(plan).unwrap();
    let untreated = ages.iter().map(|age| vec![0.0, *age]).collect();
    let treated = ages.iter().map(|age| vec![1.0, *age]).collect();
    (fit, untreated, treated, weights)
}

fn point_results(
    request: StandardizationRequest,
) -> hirmos_causal_core::survival::flexsurv::standardize::StandardizationResult {
    let (fit, untreated, treated, weights) = problem();
    let untreated = fit
        .standardization_population(untreated, Some(weights.clone()))
        .unwrap();
    let treated = fit
        .standardization_population(treated, Some(weights))
        .unwrap();
    let plan = StandardizationPlan::new(
        vec![untreated, treated],
        request,
        Some(StandardizationContrast::new(0, ContrastKind::Difference)),
        StandardizationUncertainty::None,
    )
    .unwrap();
    standardize(&fit, plan).unwrap()
}

fn assert_points(actual: &[f64], expected: &Value, key: &str, tolerance: f64) {
    for (index, value) in actual.iter().enumerate() {
        close(
            *value,
            number(&expected[key][index]),
            tolerance,
            &format!("{key} {index}"),
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn standardized_point_estimates_match_standsurv() {
    let fixture = fixture();
    let expected = &fixture["standardized_predictions"];
    let times = expected["times"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    for (name, request, tolerance) in [
        (
            "survival",
            StandardizationRequest::survival(times.clone()).unwrap(),
            3e-9,
        ),
        (
            "hazard",
            StandardizationRequest::hazard(times.clone()).unwrap(),
            3e-9,
        ),
        (
            "rmst",
            StandardizationRequest::restricted_mean(times.clone()).unwrap(),
            2e-8,
        ),
    ] {
        let result = point_results(request);
        let oracle = &expected[name];
        let first = result.scenarios[0]
            .estimates
            .iter()
            .map(|estimate| estimate.estimate)
            .collect::<Vec<_>>();
        let second = result.scenarios[1]
            .estimates
            .iter()
            .map(|estimate| estimate.estimate)
            .collect::<Vec<_>>();
        assert_points(&first, oracle, "at1", tolerance);
        assert_points(&second, oracle, "at2", tolerance);
    }

    let result =
        point_results(StandardizationRequest::quantiles(vec![0.25, 0.5], (1e-8, 5000.0)).unwrap());
    let first = result.scenarios[0]
        .estimates
        .iter()
        .map(|estimate| estimate.estimate)
        .collect::<Vec<_>>();
    let second = result.scenarios[1]
        .estimates
        .iter()
        .map(|estimate| estimate.estimate)
        .collect::<Vec<_>>();
    assert_points(&first, &expected["quantile"], "at1", 2e-7);
    assert_points(&second, &expected["quantile"], "at2", 2e-7);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn standardized_contrasts_match_standsurv() {
    let fixture = fixture();
    let expected = &fixture["standardized_predictions"];
    let times = expected["times"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let (fit, untreated, treated, weights) = problem();
    let populations = vec![
        fit.standardization_population(untreated, Some(weights.clone()))
            .unwrap(),
        fit.standardization_population(treated, Some(weights))
            .unwrap(),
    ];
    for (kind, oracle_name) in [
        (ContrastKind::Difference, "survival_difference"),
        (ContrastKind::Ratio, "survival_ratio"),
    ] {
        let plan = StandardizationPlan::new(
            populations.clone(),
            StandardizationRequest::survival(times.clone()).unwrap(),
            Some(StandardizationContrast::new(0, kind)),
            StandardizationUncertainty::None,
        )
        .unwrap();
        let result = standardize(&fit, plan).unwrap();
        let values = result.contrasts[0]
            .estimates
            .iter()
            .map(|estimate| estimate.estimate)
            .collect::<Vec<_>>();
        assert_points(&values, &expected[oracle_name], "contrast2_1", 2e-9);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn delta_and_simulation_uncertainty_match_standsurv() {
    let fixture = fixture();
    let expected = &fixture["standardized_predictions"];
    let times = expected["times"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let (fit, untreated, treated, weights) = problem();
    let populations = vec![
        fit.standardization_population(untreated, Some(weights.clone()))
            .unwrap(),
        fit.standardization_population(treated, Some(weights))
            .unwrap(),
    ];
    let delta = StandardizationPlan::new(
        populations.clone(),
        StandardizationRequest::survival(times.clone()).unwrap(),
        Some(StandardizationContrast::new(0, ContrastKind::Difference)),
        StandardizationUncertainty::Delta {
            confidence_level: number(&expected["confidence_level"]),
            transform: IntervalTransform::Log,
            contrast_transform: IntervalTransform::None,
        },
    )
    .unwrap();
    let actual = standardize(&fit, delta).unwrap();
    let oracle = &expected["survival_delta"];
    for scenario in 0..2 {
        for (point, estimate) in actual.scenarios[scenario].estimates.iter().enumerate() {
            let prefix = format!("at{}", scenario + 1);
            close(
                estimate.standard_error.unwrap(),
                number(&oracle[format!("{prefix}_se")][point]),
                3e-5,
                "delta standard error",
            );
            close(
                estimate.lower.unwrap(),
                number(&oracle[format!("{prefix}_lci")][point]),
                3e-5,
                "delta lower",
            );
            close(
                estimate.upper.unwrap(),
                number(&oracle[format!("{prefix}_uci")][point]),
                3e-5,
                "delta upper",
            );
        }
    }
    for (point, estimate) in actual.contrasts[0].estimates.iter().enumerate() {
        close(
            estimate.standard_error.unwrap(),
            number(&oracle["contrast2_1_se"][point]),
            3e-5,
            "delta contrast standard error",
        );
        close(
            estimate.lower.unwrap(),
            number(&oracle["contrast2_1_lci"][point]),
            3e-5,
            "delta contrast lower",
        );
        close(
            estimate.upper.unwrap(),
            number(&oracle["contrast2_1_uci"][point]),
            3e-5,
            "delta contrast upper",
        );
    }

    let configuration = PredictionSimulationConfiguration::new(
        expected["boot_draws"].as_u64().unwrap() as usize,
        number(&expected["confidence_level"]),
        expected["boot_seed"].as_u64().unwrap() as u32,
    )
    .unwrap();
    let simulation = StandardizationPlan::new(
        populations,
        StandardizationRequest::survival(times).unwrap(),
        Some(StandardizationContrast::new(0, ContrastKind::Difference)),
        StandardizationUncertainty::Simulation(configuration),
    )
    .unwrap();
    let actual = standardize(&fit, simulation).unwrap();
    let oracle = &expected["survival_boot"];
    for scenario in 0..2 {
        for (point, estimate) in actual.scenarios[scenario].estimates.iter().enumerate() {
            let prefix = format!("at{}", scenario + 1);
            close(
                estimate.standard_error.unwrap(),
                number(&oracle[format!("{prefix}_se")][point]),
                3e-5,
                "simulation standard error",
            );
            close(
                estimate.lower.unwrap(),
                number(&oracle[format!("{prefix}_lci")][point]),
                3e-5,
                "simulation lower",
            );
            close(
                estimate.upper.unwrap(),
                number(&oracle[format!("{prefix}_uci")][point]),
                3e-5,
                "simulation upper",
            );
        }
    }
    for (point, estimate) in actual.contrasts[0].estimates.iter().enumerate() {
        close(
            estimate.standard_error.unwrap(),
            number(&oracle["contrast2_1_se"][point]),
            3e-5,
            "simulation contrast standard error",
        );
        close(
            estimate.lower.unwrap(),
            number(&oracle["contrast2_1_lci"][point]),
            3e-5,
            "simulation contrast lower",
        );
        close(
            estimate.upper.unwrap(),
            number(&oracle["contrast2_1_uci"][point]),
            3e-5,
            "simulation contrast upper",
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_standardization_states_are_rejected() {
    let (fit, untreated, _, weights) = problem();
    assert!(fit.standardization_population(Vec::new(), None).is_err());
    assert!(fit
        .standardization_population(untreated.clone(), Some(vec![1.0]))
        .is_err());
    assert!(fit
        .standardization_population(untreated, Some(vec![0.0; weights.len()]))
        .is_err());
    assert!(StandardizationRequest::survival(Vec::new()).is_err());
    assert!(StandardizationRequest::quantiles(vec![1.2], (0.0, 1.0)).is_err());
}
