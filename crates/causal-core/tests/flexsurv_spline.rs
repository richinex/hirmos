use std::num::NonZeroUsize;

use hirmos_causal_core::survival::flexsurv::fit::{
    OptimizationEvidence, SurvivalDataset, SurvivalRecord, UncertaintyEvidence,
};
use hirmos_causal_core::survival::flexsurv::model::CovariateMatrix;
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use hirmos_causal_core::survival::flexsurv::spline::{
    fit_spline, RoystonParmarSpline, SplineFitPlan, SplineRegressionModel, SplineScale,
    SplineTimeScale,
};
use serde_json::Value;

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn royston_parmar_distributions_match_flexsurv() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    for case in fixture["spline_distributions"].as_array().unwrap() {
        let scale = match case["scale"].as_str().unwrap() {
            "hazard" => SplineScale::Hazard,
            "odds" => SplineScale::Odds,
            "normal" => SplineScale::Normal,
            value => panic!("unexpected spline scale {value}"),
        };
        let coefficients = case["gamma"]
            .as_array()
            .unwrap()
            .iter()
            .map(number)
            .collect();
        let knots = case["knots"]
            .as_array()
            .unwrap()
            .iter()
            .map(number)
            .collect();
        let spline =
            RoystonParmarSpline::new(coefficients, knots, scale, SplineTimeScale::Log).unwrap();
        for (index, time) in case["times"].as_array().unwrap().iter().enumerate() {
            let time = number(time);
            let actual = spline.evaluate(time).unwrap();
            let label = format!("{} at {time}", case["scale"].as_str().unwrap());
            close(
                actual.density,
                number(&case["density"][index]),
                3e-10,
                &label,
            );
            close(actual.cdf, number(&case["cdf"][index]), 3e-10, &label);
            close(
                actual.survival,
                number(&case["survival"][index]),
                3e-10,
                &label,
            );
            close(actual.hazard, number(&case["hazard"][index]), 3e-9, &label);
            close(
                actual.cumulative_hazard,
                number(&case["cumulative_hazard"][index]),
                3e-9,
                &label,
            );
        }
        for (probability, expected) in case["probabilities"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["quantiles"].as_array().unwrap())
        {
            close(
                spline.quantile(number(probability)).unwrap(),
                number(expected),
                2e-8,
                "spline quantile",
            );
        }
        close(
            spline.restricted_mean(0.2, 5.0).unwrap(),
            number(&case["restricted_mean"]),
            2e-7,
            "spline restricted mean",
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_spline_states_stop_at_construction() {
    assert!(RoystonParmarSpline::new(
        vec![0.0, 1.0],
        vec![0.0],
        SplineScale::Hazard,
        SplineTimeScale::Log,
    )
    .is_err());
    assert!(RoystonParmarSpline::new(
        vec![0.0, 1.0, 0.0],
        vec![0.0, 1.0, 1.0],
        SplineScale::Hazard,
        SplineTimeScale::Log,
    )
    .is_err());
    assert!(RoystonParmarSpline::new(
        vec![0.0, 1.0],
        vec![0.0, 1.0, 2.0],
        SplineScale::Hazard,
        SplineTimeScale::Log,
    )
    .is_err());
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn spline_regression_matches_flexsurvspline() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    let source = &fixture["ovarian"];
    let expected = &fixture["fits"]["spline_hazard_age"];
    let times = source["futime"].as_array().unwrap();
    let events = source["fustat"].as_array().unwrap();
    let ages = source["age"]
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
    let knots = expected["knots"]
        .as_array()
        .unwrap()
        .iter()
        .map(number)
        .collect::<Vec<_>>();
    let model = SplineRegressionModel::new(
        times.len(),
        knots,
        SplineScale::Hazard,
        SplineTimeScale::Log,
    )
    .unwrap()
    .with_location_covariates(CovariateMatrix::new(times.len(), 1, ages).unwrap())
    .unwrap();
    let data = SurvivalDataset::new(records).unwrap();
    let plan = SplineFitPlan::optimize(
        model,
        data,
        &[-7.0, 1.0, 0.0],
        &[0.05],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(4).unwrap()),
    )
    .unwrap();
    let actual = fit_spline(plan).unwrap();

    for (index, value) in actual.parameters.iter().enumerate() {
        close(
            *value,
            number(&expected["transformed_estimates"][index]),
            2e-7,
            &format!("spline fit parameter {index}"),
        );
    }
    close(
        actual.log_likelihood,
        number(&expected["log_likelihood"]),
        2e-9,
        "spline fit log likelihood",
    );
    close(actual.aic, number(&expected["aic"]), 2e-9, "spline AIC");
    close(actual.bic, number(&expected["bic"]), 2e-9, "spline BIC");
    for (index, value) in actual.individual_log_likelihood.iter().enumerate() {
        close(
            *value,
            number(&expected["individual_log_likelihood"][index]),
            2e-9,
            &format!("spline row log likelihood {index}"),
        );
    }
    let OptimizationEvidence::Optimized {
        function_count,
        gradient_count,
        ..
    } = actual.optimization
    else {
        panic!("expected optimized spline fit")
    };
    assert_eq!(
        function_count,
        expected["counts"][0].as_u64().unwrap() as usize
    );
    assert_eq!(
        gradient_count,
        expected["counts"][1].as_u64().unwrap() as usize
    );
    let UncertaintyEvidence::Hessian {
        covariance,
        covariance_order,
        transformed,
        ..
    } = actual.uncertainty
    else {
        panic!("expected Hessian uncertainty")
    };
    assert_eq!(covariance_order, 4);
    for (index, value) in covariance.iter().enumerate() {
        close(
            *value,
            number(&expected["covariance"][index]),
            3e-5,
            &format!("spline covariance {index}"),
        );
    }
    for (index, interval) in transformed.iter().enumerate() {
        close(
            interval.as_ref().unwrap().standard_error,
            number(&expected["standard_errors"][index]),
            3e-5,
            &format!("spline standard error {index}"),
        );
    }
}
