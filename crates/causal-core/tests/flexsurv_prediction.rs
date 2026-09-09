use std::num::NonZeroUsize;

use hirmos_causal_core::survival::flexsurv::fit::{fit, FlexSurvFitPlan};
use hirmos_causal_core::survival::flexsurv::model::FlexSurvFamily;
use hirmos_causal_core::survival::flexsurv::predict::{Prediction, PredictionRequest};
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use hirmos_causal_core::survival::flexsurv::uncertainty::{
    PredictionSimulationConfiguration, PredictionSummaryRequest,
};
use serde_json::Value;

mod fit_support {
    use hirmos_causal_core::survival::flexsurv::fit::{SurvivalDataset, SurvivalRecord};
    use hirmos_causal_core::survival::flexsurv::model::{
        CovariateMatrix, FlexSurvFamily, RegressionModel,
    };
    use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
    use serde_json::Value;

    pub fn ovarian_problem(
        fixture: &Value,
        family: FlexSurvFamily,
        covariate_name: &str,
    ) -> (RegressionModel, SurvivalDataset) {
        let source = &fixture["ovarian"];
        let times = source["futime"].as_array().unwrap();
        let events = source["fustat"].as_array().unwrap();
        let covariates = source[covariate_name]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        let records = times
            .iter()
            .zip(events)
            .map(|(time, event)| {
                let time = time.as_f64().unwrap();
                let observation = if event.as_i64().unwrap() == 1 {
                    SurvivalObservation::exact(time).unwrap()
                } else {
                    SurvivalObservation::right_censored(time).unwrap()
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
}

fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exponential_predictions_match_flexsurv_summary() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    let expected_fit = &fixture["fits"]["exponential_age"];
    let transformed = expected_fit["transformed_estimates"].as_array().unwrap();
    let rate = transformed[0].as_f64().unwrap().exp();
    let coefficient = transformed[1].as_f64().unwrap();
    let (model, data) = fit_support::ovarian_problem(&fixture, FlexSurvFamily::Exponential, "age");
    let fitted =
        fit(FlexSurvFitPlan::all_fixed(model, data, &[rate], &[coefficient]).unwrap()).unwrap();
    let predictions = &expected_fit["predictions"];
    let times = predictions["times"].as_array().unwrap();

    for (profile_index, age) in [45.0, 65.0].iter().copied().enumerate() {
        let covariates = fitted.prediction_covariates(vec![age]).unwrap();
        for (time_index, time) in times.iter().enumerate() {
            let flat_index = profile_index * times.len() + time_index;
            let time = time.as_f64().unwrap();
            let Prediction::AtTime(value) = fitted
                .predict(&covariates, PredictionRequest::AtTime(time))
                .unwrap()
            else {
                unreachable!()
            };
            close(
                value.survival,
                predictions["survival"][flat_index].as_f64().unwrap(),
                2e-13,
                "survival",
            );
            close(
                value.hazard,
                predictions["hazard"][flat_index].as_f64().unwrap(),
                2e-13,
                "hazard",
            );
            close(
                value.cumulative_hazard,
                predictions["cumulative_hazard"][flat_index]
                    .as_f64()
                    .unwrap(),
                2e-13,
                "cumulative hazard",
            );
            let Prediction::RestrictedMean(rmst) = fitted
                .predict(
                    &covariates,
                    PredictionRequest::RestrictedMean {
                        start: 0.0,
                        end: time,
                    },
                )
                .unwrap()
            else {
                unreachable!()
            };
            close(
                rmst,
                predictions["rmst"][flat_index].as_f64().unwrap(),
                2e-12,
                "restricted mean",
            );
        }
        let Prediction::Quantile(median) = fitted
            .predict(&covariates, PredictionRequest::Quantile(0.5))
            .unwrap()
        else {
            unreachable!()
        };
        close(
            median,
            predictions["median"][profile_index].as_f64().unwrap(),
            2e-12,
            "median",
        );
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn simulation_intervals_match_flexsurv_normboot() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    let expected = &fixture["fits"]["exponential_age_prediction_simulation"];
    let (model, data) = fit_support::ovarian_problem(&fixture, FlexSurvFamily::Exponential, "age");
    let fitted = fit(FlexSurvFitPlan::optimize(
        model,
        data,
        &[0.002],
        &[0.02],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(2).unwrap()),
    )
    .unwrap())
    .unwrap();
    let configuration = PredictionSimulationConfiguration::new(
        expected["draws"].as_u64().unwrap() as usize,
        expected["confidence_level"].as_f64().unwrap(),
        expected["seed"].as_u64().unwrap() as u32,
    )
    .unwrap();

    let parameter_draws = fitted
        .simulate_transformed_parameters(configuration)
        .unwrap();
    let source_draws = expected["transformed_parameter_draws"].as_array().unwrap();
    let mut largest_parameter_deviation = 0.0_f64;
    for (actual, source) in parameter_draws.transformed.iter().zip(source_draws) {
        for (actual, source) in actual.iter().zip(source.as_array().unwrap()) {
            largest_parameter_deviation =
                largest_parameter_deviation.max((actual - source.as_f64().unwrap()).abs());
        }
    }
    assert!(
        largest_parameter_deviation <= 2e-5,
        "normboot transformed-parameter max deviation {largest_parameter_deviation:.3e}"
    );

    let covariates = fitted.prediction_covariates(vec![45.0]).unwrap();
    let requests = [
        (
            "survival",
            PredictionSummaryRequest::Survival {
                time: 500.0,
                start: 0.0,
            },
        ),
        (
            "hazard",
            PredictionSummaryRequest::Hazard {
                time: 500.0,
                start: 0.0,
            },
        ),
        (
            "cumulative_hazard",
            PredictionSummaryRequest::CumulativeHazard {
                time: 500.0,
                start: 0.0,
            },
        ),
        (
            "restricted_mean",
            PredictionSummaryRequest::RestrictedMean {
                start: 0.0,
                end: 500.0,
            },
        ),
        (
            "median",
            PredictionSummaryRequest::Quantile {
                probability: 0.5,
                start: 0.0,
            },
        ),
    ];
    for (name, request) in requests {
        let actual = fitted
            .predict_with_simulation(&covariates, request, configuration)
            .unwrap();
        let source = &expected[name];
        close(
            actual.estimate,
            source["estimate"].as_f64().unwrap(),
            3e-8,
            name,
        );
        close(actual.lower, source["lower"].as_f64().unwrap(), 3e-5, name);
        close(actual.upper, source["upper"].as_f64().unwrap(), 3e-5, name);
        close(
            actual.standard_error,
            source["standard_error"].as_f64().unwrap(),
            3e-5,
            name,
        );
    }
}
