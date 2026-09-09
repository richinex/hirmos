use std::num::NonZeroUsize;

use hirmos_causal_core::survival::flexsurv::fit::{
    fit, FlexSurvFitPlan, GradientRecipe, OptimizationEvidence, SurvivalDataset, SurvivalRecord,
    UncertaintyEvidence,
};
use hirmos_causal_core::survival::flexsurv::likelihood::BackgroundMortality;
use hirmos_causal_core::survival::flexsurv::model::{
    CovariateMatrix, FlexSurvFamily, RegressionModel,
};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::predict::ExpectedMortalityAtTime;
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn interval_problem() -> (RegressionModel, SurvivalDataset) {
    let fixture = fixture();
    let source = fixture["censoring_data"].as_array().unwrap();
    let mut records = Vec::with_capacity(source.len());
    let mut covariates = Vec::with_capacity(source.len());

    for row in source {
        let lower = row["lower"].as_f64();
        let upper = row["upper"].as_f64();
        let observation = match row["kind"].as_i64().unwrap() {
            0 => SurvivalObservation::right_censored(lower.unwrap()).unwrap(),
            1 => SurvivalObservation::exact(lower.unwrap()).unwrap(),
            2 => SurvivalObservation::left_censored(upper.unwrap()).unwrap(),
            3 => SurvivalObservation::interval_censored(lower.unwrap(), upper.unwrap()).unwrap(),
            _ => unreachable!(),
        };
        records.push(SurvivalRecord::weighted(observation, number(&row["weight"])).unwrap());
        covariates.push(number(&row["x"]));
    }

    let model = RegressionModel::new(FlexSurvFamily::Weibull, source.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(source.len(), 1, covariates).unwrap())
        .unwrap();
    (model, SurvivalDataset::new(records).unwrap())
}

fn counting_problem() -> (RegressionModel, SurvivalDataset) {
    let fixture = fixture();
    let rows = fixture["counting_data"].as_array().unwrap();
    let mut records = Vec::with_capacity(rows.len());
    let mut covariates = Vec::with_capacity(rows.len());
    for row in rows {
        let start = number(&row["start"]);
        let stop = number(&row["stop"]);
        let observation = if row["event"].as_i64().unwrap() == 1 {
            SurvivalObservation::delayed_event(start, stop).unwrap()
        } else {
            SurvivalObservation::delayed_right_censored(start, stop).unwrap()
        };
        records.push(SurvivalRecord::new(observation));
        covariates.push(number(&row["x"]));
    }
    let model = RegressionModel::new(FlexSurvFamily::Weibull, rows.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(rows.len(), 1, covariates).unwrap())
        .unwrap();
    (model, SurvivalDataset::new(records).unwrap())
}

fn ovarian_problem(
    family: FlexSurvFamily,
    covariate_name: &str,
) -> (RegressionModel, SurvivalDataset) {
    let fixture = fixture();
    let source = &fixture["ovarian"];
    let times = source["futime"].as_array().unwrap();
    let events = source["fustat"].as_array().unwrap();
    let covariates = source[covariate_name]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let records = times
        .iter()
        .zip(events)
        .map(|(time, event)| {
            let observation = if event.as_i64().unwrap() == 1 {
                SurvivalObservation::exact(number(time)).unwrap()
            } else {
                SurvivalObservation::right_censored(number(time)).unwrap()
            };
            SurvivalRecord::new(observation)
        })
        .collect::<Vec<_>>();
    let model = RegressionModel::new(family, times.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(times.len(), 1, covariates).unwrap())
        .unwrap();
    (model, SurvivalDataset::new(records).unwrap())
}

fn ovarian_intercept_problem(family: FlexSurvFamily) -> (RegressionModel, SurvivalDataset) {
    let fixture = fixture();
    let source = &fixture["ovarian"];
    let times = source["futime"].as_array().unwrap();
    let events = source["fustat"].as_array().unwrap();
    let records = times
        .iter()
        .zip(events)
        .map(|(time, event)| {
            let observation = if event.as_i64().unwrap() == 1 {
                SurvivalObservation::exact(number(time)).unwrap()
            } else {
                SurvivalObservation::right_censored(number(time)).unwrap()
            };
            SurvivalRecord::new(observation)
        })
        .collect::<Vec<_>>();
    (
        RegressionModel::new(family, times.len()).unwrap(),
        SurvivalDataset::new(records).unwrap(),
    )
}

fn assert_explicit_fit(
    fixture_name: &str,
    family: FlexSurvFamily,
    covariate_name: &str,
    natural_baseline: &[f64],
    coefficients: &[f64],
    expected_gradient: GradientRecipe,
) {
    let fixture = fixture();
    let expected = &fixture["fits"][fixture_name];
    let expected_parameters = expected["transformed_estimates"].as_array().unwrap();
    let expected_counts = expected["counts"].as_array().unwrap();
    let (model, data) = ovarian_problem(family, covariate_name);
    let parameter_count = model.parameter_count();
    let plan = FlexSurvFitPlan::optimize(
        model,
        data,
        natural_baseline,
        coefficients,
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(parameter_count).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap_or_else(|error| panic!("{fixture_name}: {error:?}"));

    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected_parameters[index]);
        assert!(
            (value - target).abs() < 3e-8,
            "{fixture_name} parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!(
        (actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-9,
        "{fixture_name} log likelihood: actual={:.17e}, expected={:.17e}",
        actual.log_likelihood,
        number(&expected["log_likelihood"])
    );
    match actual.optimization {
        OptimizationEvidence::Optimized {
            gradient,
            function_count,
            gradient_count,
            ..
        } => {
            assert_eq!(gradient, expected_gradient);
            assert_eq!(
                function_count,
                expected_counts[0].as_u64().unwrap() as usize
            );
            assert_eq!(
                gradient_count,
                expected_counts[1].as_u64().unwrap() as usize
            );
        }
        OptimizationEvidence::AllParametersFixed => panic!("expected an optimized fit"),
    }
    assert_uncertainty(fixture_name, &actual.uncertainty, expected, 2e-5);
}

fn assert_explicit_intercept_fit(
    fixture_name: &str,
    family: FlexSurvFamily,
    natural_baseline: &[f64],
) {
    let fixture = fixture();
    let expected = &fixture["fits"][fixture_name];
    let expected_parameters = expected["transformed_estimates"].as_array().unwrap();
    let expected_counts = expected["counts"].as_array().unwrap();
    let (model, data) = ovarian_intercept_problem(family);
    let parameter_count = model.parameter_count();
    let plan = FlexSurvFitPlan::optimize(
        model,
        data,
        natural_baseline,
        &[],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(parameter_count).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap_or_else(|error| panic!("{fixture_name}: {error:?}"));

    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected_parameters[index]);
        assert!(
            (value - target).abs() < 1e-6,
            "{fixture_name} parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!(
        (actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-8,
        "{fixture_name} log likelihood: actual={:.17e}, expected={:.17e}",
        actual.log_likelihood,
        number(&expected["log_likelihood"])
    );
    match actual.optimization {
        OptimizationEvidence::Optimized {
            gradient,
            function_count,
            gradient_count,
            ..
        } => {
            assert_eq!(gradient, GradientRecipe::BaseRCentralDifference);
            assert_eq!(
                function_count,
                expected_counts[0].as_u64().unwrap() as usize
            );
            assert_eq!(
                gradient_count,
                expected_counts[1].as_u64().unwrap() as usize
            );
        }
        OptimizationEvidence::AllParametersFixed => panic!("expected an optimized fit"),
    }
    assert_uncertainty(fixture_name, &actual.uncertainty, expected, 2e-4);
}

fn assert_automatic_fit(
    fixture_name: &str,
    family: FlexSurvFamily,
    covariate_name: Option<&str>,
    parameter_tolerance: f64,
    likelihood_tolerance: f64,
) {
    let fixture = fixture();
    let expected = &fixture["fits"][fixture_name];
    let expected_parameters = expected["transformed_estimates"].as_array().unwrap();
    let expected_counts = expected["counts"].as_array().unwrap();
    let (model, data) = match covariate_name {
        Some(name) => ovarian_problem(family, name),
        None => ovarian_intercept_problem(family),
    };
    let parameter_count = model.parameter_count();
    let plan = FlexSurvFitPlan::optimize_automatic(
        model,
        data,
        ROptimControl::bfgs_defaults(NonZeroUsize::new(parameter_count).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap_or_else(|error| panic!("{fixture_name}: {error:?}"));

    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected_parameters[index]);
        assert!(
            (value - target).abs() <= parameter_tolerance * target.abs().max(1.0),
            "{fixture_name} parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!(
        (actual.log_likelihood - number(&expected["log_likelihood"])).abs() <= likelihood_tolerance,
        "{fixture_name} log likelihood: actual={:.17e}, expected={:.17e}",
        actual.log_likelihood,
        number(&expected["log_likelihood"])
    );
    match actual.optimization {
        OptimizationEvidence::Optimized {
            function_count,
            gradient_count,
            ..
        } => {
            let source_function_count = expected_counts[0].as_u64().unwrap() as usize;
            let source_gradient_count = expected_counts[1].as_u64().unwrap() as usize;
            assert_eq!(gradient_count, source_gradient_count);
            if fixture_name == "exponential_age" {
                // The source start is already the optimum to floating-point
                // precision. Native and WASM arithmetic can make the line
                // search take a few no-op objective probes before returning
                // that same fit, so only the numerical result is invariant.
                assert!((2..=5).contains(&function_count));
                assert_eq!(source_function_count, 2);
            } else {
                assert_eq!(function_count, source_function_count);
            }
        }
        OptimizationEvidence::AllParametersFixed => panic!("expected an optimized fit"),
    }
}

fn assert_uncertainty(
    fixture_name: &str,
    actual: &UncertaintyEvidence,
    expected: &Value,
    tolerance: f64,
) {
    let UncertaintyEvidence::Hessian {
        covariance,
        covariance_order,
        natural,
        ..
    } = actual
    else {
        panic!("{fixture_name}: expected Hessian uncertainty")
    };
    assert_eq!(
        *covariance_order,
        expected["covariance_order"].as_u64().unwrap() as usize
    );
    for (index, value) in covariance.iter().enumerate() {
        let target = expected["covariance"][index].as_f64().unwrap();
        assert!(
            (value - target).abs() <= tolerance * target.abs().max(1.0),
            "{fixture_name} covariance {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    for (index, interval) in natural.iter().enumerate() {
        let Some(interval) = interval else { continue };
        let target = expected["standard_errors"][index].as_f64().unwrap();
        assert!(
            (interval.standard_error - target).abs() <= tolerance * target.abs().max(1.0),
            "{fixture_name} SE {index}: actual={:.17e}, expected={target:.17e}",
            interval.standard_error
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn mixed_censoring_numeric_fit_matches_flexsurv() {
    let fixture = fixture();
    let expected = &fixture["fits"]["interval_weibull"];
    let (model, data) = interval_problem();
    let plan = FlexSurvFitPlan::optimize(
        model,
        data,
        &[1.4, 3.5],
        &[0.1],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(3).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap();
    let expected_parameters = expected["transformed_estimates"].as_array().unwrap();

    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected_parameters[index]);
        assert!(
            (value - target).abs() < 2e-8,
            "parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!((actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-10);
    assert!((actual.aic - number(&expected["aic"])).abs() < 2e-10);
    assert!((actual.bic - number(&expected["bic"])).abs() < 2e-10);
    match actual.optimization {
        OptimizationEvidence::Optimized {
            function_count,
            gradient_count,
            ..
        } => {
            assert_eq!(function_count, 53);
            assert_eq!(gradient_count, 21);
        }
        OptimizationEvidence::AllParametersFixed => panic!("expected an optimized fit"),
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn all_fixed_is_a_distinct_fit_state() {
    let (model, data) = interval_problem();
    let plan = FlexSurvFitPlan::all_fixed(model, data, &[1.4, 3.5], &[0.1]).unwrap();
    let actual = fit(plan).unwrap();
    assert_eq!(actual.estimated_parameter_count, 0);
    assert_eq!(
        actual.optimization,
        OptimizationEvidence::AllParametersFixed
    );
    assert_eq!(actual.aic, -2.0 * actual.log_likelihood);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn analytic_gradient_families_follow_flexsurv_search() {
    assert_explicit_fit(
        "exponential_age_explicit",
        FlexSurvFamily::Exponential,
        "age",
        &[0.002],
        &[0.02],
        GradientRecipe::FlexSurvAnalytic,
    );
    assert_explicit_fit(
        "weibull_treatment_explicit",
        FlexSurvFamily::Weibull,
        "rx_indicator",
        &[1.0, 500.0],
        &[0.1],
        GradientRecipe::FlexSurvAnalytic,
    );
    assert_explicit_fit(
        "loglogistic_age_explicit",
        FlexSurvFamily::LogLogistic,
        "age",
        &[1.5, 500.0],
        &[-0.02],
        GradientRecipe::FlexSurvAnalytic,
    );
    assert_explicit_fit(
        "gompertz_age_explicit",
        FlexSurvFamily::Gompertz,
        "age",
        &[0.001, 0.002],
        &[0.01],
        GradientRecipe::FlexSurvAnalytic,
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn numerical_gradient_families_follow_flexsurv_search() {
    assert_explicit_fit(
        "lognormal_age_explicit",
        FlexSurvFamily::LogNormal,
        "age",
        &[6.5, 1.0],
        &[-0.01],
        GradientRecipe::BaseRCentralDifference,
    );
    assert_explicit_intercept_fit(
        "gamma_intercept_explicit",
        FlexSurvFamily::Gamma,
        &[1.5, 0.002],
    );
    assert_explicit_intercept_fit(
        "generalized_gamma_intercept_explicit",
        FlexSurvFamily::GeneralizedGamma,
        &[6.5, 1.0, 0.0],
    );
    assert_explicit_intercept_fit(
        "generalized_f_intercept_explicit",
        FlexSurvFamily::GeneralizedF,
        &[6.5, 1.0, 0.1, 1.0],
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn original_generalized_families_support_source_fixed_parameters() {
    let fixture = fixture();
    let cases = [
        (
            "generalized_gamma_original_intercept",
            FlexSurvFamily::GeneralizedGammaOriginal,
            vec![1.0, 500.0, 1.0],
            vec![0, 2],
        ),
        (
            "generalized_f_original_intercept",
            FlexSurvFamily::GeneralizedFOriginal,
            vec![6.2, 0.7, 1.3, 0.9],
            vec![1, 2, 3],
        ),
    ];
    for (name, family, baseline, fixed) in cases {
        let expected = &fixture["fits"][name];
        let (model, data) = ovarian_intercept_problem(family);
        let plan = FlexSurvFitPlan::optimize(
            model,
            data,
            &baseline,
            &[],
            &fixed,
            ROptimControl::bfgs_defaults(NonZeroUsize::new(1).unwrap()),
        )
        .unwrap();
        let actual = fit(plan).unwrap();
        for (index, value) in actual.transformed_parameters.iter().enumerate() {
            let target = number(&expected["transformed_estimates"][index]);
            assert!(
                (value - target).abs() < 3e-8,
                "{name} parameter {index}: actual={value:.17e}, expected={target:.17e}"
            );
        }
        assert!((actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-9);
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn automatic_initial_values_follow_flexsurv_search() {
    assert_automatic_fit(
        "exponential_age",
        FlexSurvFamily::Exponential,
        Some("age"),
        3e-8,
        2e-9,
    );
    assert_automatic_fit(
        "weibull_treatment",
        FlexSurvFamily::Weibull,
        Some("rx_indicator"),
        3e-8,
        2e-9,
    );
    assert_automatic_fit(
        "lognormal_age",
        FlexSurvFamily::LogNormal,
        Some("age"),
        3e-8,
        2e-9,
    );
    assert_automatic_fit(
        "loglogistic_age",
        FlexSurvFamily::LogLogistic,
        Some("age"),
        3e-8,
        2e-9,
    );
    assert_automatic_fit("gamma_intercept", FlexSurvFamily::Gamma, None, 2e-7, 2e-8);
    assert_automatic_fit(
        "generalized_gamma_intercept",
        FlexSurvFamily::GeneralizedGamma,
        None,
        2e-7,
        2e-8,
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn automatic_initial_values_support_mixed_censoring() {
    let fixture = fixture();
    let expected = &fixture["fits"]["interval_weibull_automatic"];
    let (model, data) = interval_problem();
    let plan = FlexSurvFitPlan::optimize_automatic(
        model,
        data,
        ROptimControl::bfgs_defaults(NonZeroUsize::new(3).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap();
    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected["transformed_estimates"][index]);
        assert!(
            (value - target).abs() < 4e-8,
            "parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!((actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-9);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn automatic_initial_values_support_counting_process_rows() {
    let fixture = fixture();
    let expected = &fixture["fits"]["counting_weibull_automatic"];
    let (model, data) = counting_problem();
    let plan = FlexSurvFitPlan::optimize_automatic(
        model,
        data,
        ROptimControl::bfgs_defaults(NonZeroUsize::new(3).unwrap()),
    )
    .unwrap();
    let actual = fit(plan).unwrap();
    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = number(&expected["transformed_estimates"][index]);
        assert!(
            (value - target).abs() < 4e-8,
            "parameter {index}: actual={value:.17e}, expected={target:.17e}"
        );
    }
    assert!((actual.log_likelihood - number(&expected["log_likelihood"])).abs() < 2e-9);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn relative_survival_fit_matches_flexsurv() {
    let fixture = fixture();
    let source = fixture["relative_survival"]["data"].as_array().unwrap();
    let expected = &fixture["relative_survival"]["fit"];
    let mut records = Vec::with_capacity(source.len());
    let mut covariates = Vec::with_capacity(source.len());
    for row in source {
        let time = number(&row["time"]);
        let status = row["status"].as_i64().unwrap();
        let observation = if status == 1 {
            SurvivalObservation::exact(time).unwrap()
        } else {
            SurvivalObservation::right_censored(time).unwrap()
        };
        let background = if status == 1 {
            BackgroundMortality::event_hazard(number(&row["bhazard"])).unwrap()
        } else {
            BackgroundMortality::conditional_death_probability(0.0).unwrap()
        };
        records.push(
            SurvivalRecord::new(observation)
                .with_background_mortality(background)
                .unwrap(),
        );
        covariates.push(number(&row["x"]));
    }
    let model = RegressionModel::new(FlexSurvFamily::Exponential, source.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(source.len(), 1, covariates).unwrap())
        .unwrap();
    let plan = FlexSurvFitPlan::optimize(
        model,
        SurvivalDataset::new(records).unwrap(),
        &[0.3],
        &[0.0],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(2).unwrap()),
    )
    .unwrap();
    let fit = fit(plan).unwrap();
    for (index, actual) in fit.transformed_parameters.iter().enumerate() {
        let target = number(&expected["transformed_estimates"][index]);
        assert!(
            (actual - target).abs() < 4e-8,
            "relative parameter {index}: actual={actual:.17e}, expected={target:.17e}"
        );
    }
    assert!((fit.log_likelihood - number(&expected["log_likelihood"])).abs() < 3e-9);
    for (actual, target) in fit
        .individual_log_likelihood
        .iter()
        .zip(expected["individual_log_likelihood"].as_array().unwrap())
    {
        assert!((actual - number(target)).abs() < 3e-9);
    }
    if let UncertaintyEvidence::Hessian { covariance, .. } = &fit.uncertainty {
        for (actual, target) in covariance
            .iter()
            .zip(expected["covariance"].as_array().unwrap())
        {
            assert!((actual - number(target)).abs() < 5e-7);
        }
    } else {
        panic!("relative survival fit should have Hessian uncertainty")
    }

    let prediction = &fixture["relative_survival"]["prediction"];
    let covariates = fit
        .prediction_covariates(vec![number(&prediction["x"])])
        .unwrap();
    let expected = ExpectedMortalityAtTime::new(
        number(&prediction["expected_survival"]),
        number(&prediction["expected_hazard"]),
    )
    .unwrap();
    let actual = fit
        .predict_all_cause(&covariates, number(&prediction["time"]), expected)
        .unwrap();
    assert!((actual.relative_survival - number(&prediction["relative_survival"])).abs() < 3e-8);
    assert!((actual.excess_hazard - number(&prediction["excess_hazard"])).abs() < 3e-8);
    assert!((actual.survival - number(&prediction["all_cause_survival"])).abs() < 3e-8);
    assert!((actual.hazard - number(&prediction["all_cause_hazard"])).abs() < 3e-8);
}
