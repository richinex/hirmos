use hirmos_causal_core::grf::inference::{self, Covariance, LinearSummary, Observations, Target};
use serde_json::Value;
fn number(v: &Value) -> f64 {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap()
}
fn floats(v: &Value) -> Vec<f64> {
    v.as_array().unwrap().iter().map(number).collect()
}
fn close(a: f64, b: f64, label: &str) {
    assert!(
        (a - b).abs() < 2e-8 * b.abs().max(1.0),
        "{label}: {a} != {b}"
    );
}
fn summary(a: LinearSummary, b: &Value, label: &str) {
    for (i, row) in b.as_array().unwrap().iter().enumerate() {
        for (a, b) in [
            a.estimates[i],
            a.standard_errors[i],
            a.statistics[i],
            a.p_values[i],
        ]
        .into_iter()
        .zip(floats(row))
        {
            close(a, b, label);
        }
    }
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn average_effects_and_heterogeneity_match_r() {
    let f: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/inference.json")).unwrap();
    for c in f["cases"].as_array().unwrap() {
        let y = floats(&c["y"]);
        let w = floats(&c["w"]);
        let yh = floats(&c["yhat"]);
        let wh = floats(&c["what"]);
        let tau = floats(&c["tau"]);
        let data = Observations {
            y: &y,
            w: &w,
            y_hat: &yh,
            w_hat: &wh,
            tau: &tau,
        };
        let weights = floats(&c["weights"]);
        let labels: Vec<_> = c["labels"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        let debiasing = if c["debiasing"].is_null() {
            None
        } else {
            Some(floats(&c["debiasing"]))
        };
        let scores = inference::scores(&data, debiasing.as_deref()).unwrap();
        for (a, b) in scores.iter().zip(floats(&c["scores"])) {
            close(*a, b, "scores");
        }
        for e in c["averages"].as_array().unwrap() {
            let target = e["target"].as_str().unwrap();
            let a = match target {
                "all" => {
                    inference::aipw(&data, &weights, &labels, Target::All, debiasing.as_deref())
                }
                "treated" => inference::aipw(&data, &weights, &labels, Target::Treated, None),
                "control" => inference::aipw(&data, &weights, &labels, Target::Control, None),
                "overlap" => {
                    inference::overlap(&data, &weights, &labels, c["clustered"].as_bool().unwrap())
                }
                _ => panic!(),
            }
            .unwrap();
            close(a.estimate, number(&e["result"][0]), target);
            close(a.standard_error, number(&e["result"][1]), target);
        }
        let x: Vec<_> = c["covariates"]
            .as_array()
            .unwrap()
            .iter()
            .map(floats)
            .collect();
        if let Some(cases) = c["tmle"].as_array() {
            for e in cases {
                let target = match e["target"].as_str().unwrap() {
                    "all" => Target::All,
                    "treated" => Target::Treated,
                    "control" => Target::Control,
                    _ => panic!(),
                };
                let a = inference::tmle(&data, target).unwrap();
                close(a.estimate, number(&e["result"][0]), "TMLE estimate");
                close(a.standard_error, number(&e["result"][1]), "TMLE SE");
            }
        }
        summary(
            inference::overlap_projection(
                &data,
                &x,
                &weights,
                &labels,
                Covariance::Hc3,
                debiasing.as_deref(),
            )
            .unwrap(),
            &c["overlap_projection"],
            "overlap projection",
        );
        for (i, cov) in [
            Covariance::Hc0,
            Covariance::Hc1,
            Covariance::Hc2,
            Covariance::Hc3,
        ]
        .into_iter()
        .enumerate()
        {
            let label = format!("case {} covariance {i}", c["case"]);
            summary(
                inference::calibration(&data, &weights, &labels, cov).unwrap(),
                &c["calibration"][i],
                &label,
            );
            summary(
                inference::projection(&scores, &x, &weights, &labels, cov).unwrap(),
                &c["projection"][i],
                &label,
            );
        }
    }
}
