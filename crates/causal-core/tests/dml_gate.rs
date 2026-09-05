// Parity for DoubleML's group average treatment effects on the DML port's own fixtures. The fits
// inherit tests/dml.rs's tolerance note: sklearn's tied tree splits resolve by build-specific
// float summation order, so the nuisances agree with any given build to about 1e-4, not bitwise.
use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::{dml_irm, dml_plr, group_effects, DmlResult, GroupEffectError};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) -> f64 {
    let deviation = (got - want).abs();
    assert!(
        deviation <= 1e-3 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
    deviation
}

fn fit(root: &Value, case: &Value) -> DmlResult {
    let w1: Vec<f64> = serde_json::from_value(root["w1"].clone()).unwrap();
    let w2: Vec<f64> = serde_json::from_value(root["w2"].clone()).unwrap();
    let x: Vec<Vec<f64>> = w1.iter().zip(&w2).map(|(&a, &b)| vec![a, b]).collect();
    let d: Vec<f64> = serde_json::from_value(case["d"].clone()).unwrap();
    let y: Vec<f64> = serde_json::from_value(case["y"].clone()).unwrap();
    let treat_binary = d.iter().all(|&v| v == 0.0 || v == 1.0);
    let mut fold_stream = Mt19937::seeded(7);
    match case["kind"].as_str().unwrap() {
        "irm" => dml_irm(&x, &y, &d, case["score"].as_str().unwrap() == "ATTE", 5, 200, 5, 0.01, 7, &mut fold_stream),
        _ => dml_plr(&x, &y, &d, treat_binary, 5, 200, 5, 7, &mut fold_stream),
    }
}

#[test]
fn group_effects_match_doubleml_gate() {
    let dml: Value = serde_json::from_str(include_str!("../oracle/fixtures/dml.json")).unwrap();
    let gate: Value = serde_json::from_str(include_str!("../oracle/fixtures/dml_gate.json")).unwrap();
    let case_by_name = |name: &str| {
        dml["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["name"] == name)
            .unwrap()
    };
    let mut max_deviation = 0.0_f64;
    for record in gate["cases"].as_array().unwrap() {
        let name = record["name"].as_str().unwrap();
        let fitted = fit(&dml, case_by_name(name));
        close(&format!("{name} coef"), fitted.coef, record["coef"].as_f64().unwrap());
        for (grouping, expected) in record["groupings"].as_object().unwrap() {
            let labels: Vec<usize> = serde_json::from_value(gate["groupings"][grouping].clone()).unwrap();
            let count = labels.iter().max().unwrap() + 1;
            let got = group_effects(&fitted, &labels, count, 0.95).unwrap();
            let effects: Vec<f64> = serde_json::from_value(expected["effects"].clone()).unwrap();
            let standard_errors: Vec<f64> = serde_json::from_value(expected["standard_errors"].clone()).unwrap();
            let ci_low: Vec<f64> = serde_json::from_value(expected["ci_low"].clone()).unwrap();
            let ci_high: Vec<f64> = serde_json::from_value(expected["ci_high"].clone()).unwrap();
            let observations: Vec<usize> = serde_json::from_value(expected["observations"].clone()).unwrap();
            assert_eq!(got.groups.len(), effects.len());
            for (g, group) in got.groups.iter().enumerate() {
                let label = format!("{name} {grouping} group {g}");
                assert_eq!(group.observations, observations[g], "{label} observations");
                max_deviation = max_deviation.max(close(&format!("{label} effect"), group.effect, effects[g]));
                close(&format!("{label} se"), group.standard_error, standard_errors[g]);
                close(&format!("{label} ci low"), group.confidence_interval[0], ci_low[g]);
                close(&format!("{label} ci high"), group.confidence_interval[1], ci_high[g]);
            }
        }
        println!("{name}: group effects, standard errors and intervals match");
    }
    println!("group effects: max effect deviation {max_deviation:.3e}");

    assert_eq!(gate["atte_refuses"], true);
    let atte = fit(&dml, case_by_name("irm_atte"));
    let labels: Vec<usize> = serde_json::from_value(gate["groupings"]["w2_sign"].clone()).unwrap();
    assert_eq!(group_effects(&atte, &labels, 2, 0.95).unwrap_err(), GroupEffectError::ScoreNotSupported);
}

#[test]
fn group_labels_are_checked_before_any_arithmetic() {
    let dml: Value = serde_json::from_str(include_str!("../oracle/fixtures/dml.json")).unwrap();
    let case = dml["cases"].as_array().unwrap().iter().find(|case| case["name"] == "plr_binary").unwrap();
    let fitted = fit(&dml, case);
    let n = fitted.coef.is_finite().then_some(400).unwrap();
    assert_eq!(group_effects(&fitted, &vec![0; n - 1], 1, 0.95).unwrap_err(), GroupEffectError::LengthMismatch { expected: n, actual: n - 1 });
    assert_eq!(group_effects(&fitted, &vec![0; n], 2, 0.95).unwrap_err(), GroupEffectError::EmptyGroup { group: 1 });
    assert_eq!(group_effects(&fitted, &vec![1; n], 1, 0.95).unwrap_err(), GroupEffectError::GroupOutOfRange { row: 0, group: 1 });
    assert_eq!(group_effects(&fitted, &vec![0; n], 1, 1.0).unwrap_err(), GroupEffectError::InvalidConfidenceLevel);
}
