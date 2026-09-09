use hirmos_causal_core::survival::flexsurv::distribution::FlexSurvDistribution;
use hirmos_causal_core::survival::flexsurv::likelihood::{log_likelihood, LikelihoodRow};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/flexsurv.json")).unwrap()
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap()
}

fn close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 2e-10 * expected.abs().max(1.0),
        "{label}: actual {actual:.17e}, expected {expected:.17e}"
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn exponential_individual_likelihood_matches_flexsurv() {
    let fixture = fixture();
    let fit = &fixture["fits"]["exponential_age"];
    let transformed = fit["transformed_estimates"].as_array().unwrap();
    let intercept = number(&transformed[0]);
    let age_coefficient = number(&transformed[1]);
    let ovarian = &fixture["ovarian"];
    let times = ovarian["futime"].as_array().unwrap();
    let events = ovarian["fustat"].as_array().unwrap();
    let ages = ovarian["age"].as_array().unwrap();
    let expected = fit["individual_log_likelihood"].as_array().unwrap();

    let mut rows = Vec::new();
    for index in 0..times.len() {
        let time = number(&times[index]);
        let observation = if events[index].as_i64().unwrap() == 1 {
            SurvivalObservation::exact(time).unwrap()
        } else {
            SurvivalObservation::right_censored(time).unwrap()
        };
        let rate = (intercept + age_coefficient * number(&ages[index])).exp();
        let row = LikelihoodRow::new(
            observation,
            FlexSurvDistribution::exponential(rate).unwrap(),
            1.0,
        )
        .unwrap();
        close(
            row.contribution().unwrap(),
            number(&expected[index]),
            &format!("row {index}"),
        );
        rows.push(row);
    }

    close(
        log_likelihood(&rows).unwrap(),
        number(&fit["log_likelihood"]),
        "exponential log likelihood",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn all_censoring_forms_match_flexsurv() {
    let fixture = fixture();
    let fit = &fixture["fits"]["interval_weibull"];
    let transformed = fit["transformed_estimates"].as_array().unwrap();
    let shape = number(&transformed[0]).exp();
    let scale_intercept = number(&transformed[1]);
    let coefficient = number(&transformed[2]);

    let rows = fixture["censoring_data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| {
            let kind = source["kind"].as_i64().unwrap();
            let upper = source["upper"].as_f64();
            let lower = source["lower"].as_f64();
            let observation = match kind {
                0 => SurvivalObservation::right_censored(lower.unwrap()).unwrap(),
                1 => SurvivalObservation::exact(lower.unwrap()).unwrap(),
                2 => SurvivalObservation::left_censored(upper.unwrap()).unwrap(),
                3 => {
                    SurvivalObservation::interval_censored(lower.unwrap(), upper.unwrap()).unwrap()
                }
                _ => unreachable!(),
            };
            let scale = (scale_intercept + coefficient * number(&source["x"])).exp();
            LikelihoodRow::new(
                observation,
                FlexSurvDistribution::weibull(shape, scale).unwrap(),
                number(&source["weight"]),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    close(
        log_likelihood(&rows).unwrap(),
        number(&fit["log_likelihood"]),
        "mixed-censoring Weibull log likelihood",
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn counting_process_likelihood_matches_flexsurv() {
    let fixture = fixture();
    let fit = &fixture["fits"]["counting_weibull"];
    let transformed = fit["transformed_estimates"].as_array().unwrap();
    let shape = number(&transformed[0]).exp();
    let scale_intercept = number(&transformed[1]);
    let coefficient = number(&transformed[2]);

    let rows = fixture["counting_data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| {
            let start = number(&source["start"]);
            let stop = number(&source["stop"]);
            let observation = if source["event"].as_i64().unwrap() == 1 {
                SurvivalObservation::delayed_event(start, stop).unwrap()
            } else {
                SurvivalObservation::delayed_right_censored(start, stop).unwrap()
            };
            let scale = (scale_intercept + coefficient * number(&source["x"])).exp();
            LikelihoodRow::new(
                observation,
                FlexSurvDistribution::weibull(shape, scale).unwrap(),
                1.0,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    close(
        log_likelihood(&rows).unwrap(),
        number(&fit["log_likelihood"]),
        "counting-process Weibull log likelihood",
    );
}
