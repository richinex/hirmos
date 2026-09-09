use std::num::NonZeroUsize;

use hirmos_causal_core::survival::flexsurv::fit::{
    fit, FlexSurvFit, FlexSurvFitPlan, SurvivalDataset, SurvivalRecord,
};
use hirmos_causal_core::survival::flexsurv::model::{
    CovariateMatrix, FlexSurvFamily, RegressionModel,
};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use serde_json::Value;

fn fit_rows(fixture: &Value, name: &str) -> FlexSurvFit {
    let rows = fixture["ph_start_stop"][name].as_array().unwrap();
    let mut records = Vec::with_capacity(rows.len());
    let mut covariates = Vec::with_capacity(rows.len());
    for row in rows {
        let start = row["start"].as_f64().unwrap();
        let stop = row["stop"].as_f64().unwrap();
        let event = row["event"].as_i64().unwrap() == 1;
        let observation = if event {
            SurvivalObservation::delayed_event(start, stop).unwrap()
        } else {
            SurvivalObservation::delayed_right_censored(start, stop).unwrap()
        };
        records.push(SurvivalRecord::new(observation));
        covariates.push(row["x"].as_f64().unwrap());
    }
    let model = RegressionModel::new(FlexSurvFamily::WeibullPh, rows.len())
        .unwrap()
        .with_location_covariates(CovariateMatrix::new(rows.len(), 1, covariates).unwrap())
        .unwrap();
    let plan = FlexSurvFitPlan::optimize(
        model,
        SurvivalDataset::new(records).unwrap(),
        &[1.2, 0.04],
        &[0.2],
        &[],
        ROptimControl::bfgs_defaults(NonZeroUsize::new(3).unwrap()),
    )
    .unwrap();
    fit(plan).unwrap()
}

fn check_fit(fixture: &Value, fixture_name: &str, actual: &FlexSurvFit) {
    let expected = &fixture["fits"][fixture_name];
    for (index, value) in actual.transformed_parameters.iter().enumerate() {
        let target = expected["transformed_estimates"][index].as_f64().unwrap();
        assert!((value - target).abs() < 2e-8, "parameter {index}");
    }
    assert!((actual.log_likelihood - expected["log_likelihood"].as_f64().unwrap()).abs() < 2e-9);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn unchanged_covariate_has_identical_split_and_unsplit_likelihood() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    let unsplit = fit_rows(&fixture, "constant_unsplit");
    let split = fit_rows(&fixture, "constant_split");
    check_fit(&fixture, "weibull_ph_unsplit", &unsplit);
    check_fit(&fixture, "weibull_ph_constant_split", &split);
    for (left, right) in unsplit
        .transformed_parameters
        .iter()
        .zip(&split.transformed_parameters)
    {
        assert!((left - right).abs() < 3e-15);
    }
    let likelihood_difference = (unsplit.log_likelihood - split.log_likelihood).abs();
    assert!(
        likelihood_difference < 2e-12,
        "split likelihood differs by {likelihood_difference:e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn changing_covariate_uses_left_truncated_ph_spells() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap();
    let actual = fit_rows(&fixture, "changing_split");
    check_fit(&fixture, "weibull_ph_changing_split", &actual);

    let flexsurv_coefficient = actual.transformed_parameters[2];
    let cox_coefficient = fixture["ph_start_stop"]["cox_changing_coefficient"]
        .as_f64()
        .unwrap();
    let generating_coefficient = fixture["ph_start_stop"]["generating_coefficient"]
        .as_f64()
        .unwrap();
    assert!((flexsurv_coefficient - generating_coefficient).abs() < 0.04);
    assert!((cox_coefficient - generating_coefficient).abs() < 0.04);
    assert!((flexsurv_coefficient - cox_coefficient).abs() < 0.02);
}
