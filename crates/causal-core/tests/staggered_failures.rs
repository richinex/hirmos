use nalgebra::DMatrix;
use serde_json::Value;
use hirmos_causal_core::staggered_did::dr::{fit, panel_guard, Sample};

#[test]
fn nuisance_failures_and_rescaling_match_reference() {
    let cases: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/staggered-did/failures.json"
        ))
        .unwrap(),
    )
    .unwrap();
    for (name, c) in cases.as_object().unwrap() {
        let n = c["x"].as_array().unwrap().len();
        let p = c["x"][0].as_array().unwrap().len();
        let sample = Sample::new(
            DMatrix::from_fn(n, p, |i, j| c["x"][i][j].as_f64().unwrap()),
            c["change"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap())
                .collect(),
            c["treated"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap() == 1.0)
                .collect(),
            vec![1.0; n],
        )
        .unwrap();
        let result = fit(&sample, 0.995);
        assert_eq!(
            panel_guard(&sample).is_err(),
            c["guard"].as_bool().unwrap(),
            "{name}: panel guard"
        );
        if c["error"].is_string() {
            assert!(
                result.is_err(),
                "{name}: R rejected but Rust returned an estimate"
            );
        } else {
            let result =
                result.unwrap_or_else(|e| panic!("{name}: R succeeded, Rust failed {e:?}"));
            assert!(
                (result.score.att - c["att"].as_f64().unwrap()).abs() < 1e-8,
                "{name}"
            );
            assert!(
                (result.score.se - c["se"].as_f64().unwrap()).abs() < 1e-8,
                "{name}"
            );
        }
    }
}
