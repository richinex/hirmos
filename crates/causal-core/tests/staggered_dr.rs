use nalgebra::DMatrix;
use serde_json::Value;
use hirmos_causal_core::staggered_did::dr::{fit, score, FitStatus, Sample};

fn values(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "Rust={a:.16} R={b:.16}");
}

#[test]
fn conditional_scores_match_drdid() {
    let cases: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/staggered-did/dr.json"))
            .unwrap(),
    )
    .unwrap();
    for (_, c) in cases.as_object().unwrap() {
        let n = c["x"].as_array().unwrap().len();
        let p = c["x"][0].as_array().unwrap().len();
        let x = DMatrix::from_fn(n, p, |i, j| c["x"][i][j].as_f64().unwrap());
        let sample = Sample::new(
            x,
            values(&c["change"]),
            values(&c["treated"]).iter().map(|v| *v == 1.0).collect(),
            values(&c["weights"]),
        )
        .unwrap();
        let result = score(
            &sample,
            &values(&c["propensity"]),
            &values(&c["outcome"]),
            c["trim"].as_f64().unwrap(),
        )
        .unwrap();
        close(result.att, c["att"].as_f64().unwrap());
        close(result.se, c["se"].as_f64().unwrap());
        for (a, b) in result.influence.iter().zip(values(&c["influence"])) {
            close(*a, b);
        }
        let fitted = fit(&sample, c["trim"].as_f64().unwrap()).unwrap();
        assert!(matches!(fitted.status, FitStatus::Converged { .. }));
        close(fitted.score.att, c["att"].as_f64().unwrap());
        close(fitted.score.se, c["se"].as_f64().unwrap());
        for (a, b) in fitted.score.influence.iter().zip(values(&c["influence"])) {
            close(*a, b);
        }
        for (a, b) in fitted.propensity.iter().zip(values(&c["propensity"])) {
            close(*a, b);
        }
        for (a, b) in fitted.outcome.iter().zip(values(&c["outcome"])) {
            close(*a, b);
        }
        for (a, b) in fitted
            .coefficients
            .iter()
            .zip(values(&c["propensity_coefficients"]))
        {
            close(*a, b);
        }
    }
}
