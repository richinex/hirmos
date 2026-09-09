// Parity for EconML's T-learner with random-forest outcome models. The forests inherit the tie note
// of tests/dml.rs: sklearn resolves tied splits by a build-specific float summation order, so a
// row's prediction can differ where a tree took the other branch of a tie.
use hirmos_causal_core::{fit_tlearner, TLearnerError};
use serde_json::Value;

fn rows(root: &Value) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let w1: Vec<f64> = serde_json::from_value(root["w1"].clone()).unwrap();
    let w2: Vec<f64> = serde_json::from_value(root["w2"].clone()).unwrap();
    let x: Vec<Vec<f64>> = w1.iter().zip(&w2).map(|(&a, &b)| vec![a, b]).collect();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let d: Vec<f64> = serde_json::from_value(root["d"].clone()).unwrap();
    (x, y, d)
}

#[test]
fn effects_match_econml() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/tlearner.json")).unwrap();
    let (x, y, d) = rows(&root);
    let fitted = fit_tlearner(&x, &y, &d, 200, 5, 7).unwrap();
    assert_eq!(
        fitted.control_rows,
        root["control_rows"].as_u64().unwrap() as usize
    );
    assert_eq!(
        fitted.treated_rows,
        root["treated_rows"].as_u64().unwrap() as usize
    );
    let want: Vec<f64> = serde_json::from_value(root["effects"].clone()).unwrap();
    let got = fitted.effect(&x);
    let mut max_deviation = 0.0_f64;
    let mut off = 0usize;
    for (i, (&g, &w)) in got.iter().zip(&want).enumerate() {
        let deviation = (g - w).abs();
        max_deviation = max_deviation.max(deviation);
        if deviation > 1e-6 {
            off += 1;
        }
        assert!(deviation <= 5e-2, "row {i}: got {g}, oracle {w}");
    }
    let ate = got.iter().sum::<f64>() / got.len() as f64;
    let want_ate = root["ate"].as_f64().unwrap();
    assert!(
        (ate - want_ate).abs() <= 1e-3 * want_ate.abs().max(1.0),
        "ate got {ate}, oracle {want_ate}"
    );
    let query: Vec<Vec<f64>> = serde_json::from_value(root["query"].clone()).unwrap();
    let want_query: Vec<f64> = serde_json::from_value(root["query_effects"].clone()).unwrap();
    for (i, (&g, &w)) in fitted.effect(&query).iter().zip(&want_query).enumerate() {
        assert!((g - w).abs() <= 5e-2, "query {i}: got {g}, oracle {w}");
    }
    println!("max row deviation {max_deviation:.3e}, rows beyond 1e-6: {off} of {}, ate deviation {:.3e}", got.len(), (ate - want_ate).abs());
}

#[test]
fn refuses_non_binary_and_empty_arms() {
    let x = vec![vec![0.0], vec![1.0], vec![2.0]];
    assert_eq!(
        fit_tlearner(&x, &[1.0, 2.0, 3.0], &[0.0, 0.5, 1.0], 5, 1, 7).err(),
        Some(TLearnerError::TreatmentNotBinary { row: 1 })
    );
    assert_eq!(
        fit_tlearner(&x, &[1.0, 2.0, 3.0], &[1.0, 1.0, 1.0], 5, 1, 7).err(),
        Some(TLearnerError::ArmEmpty { treated: false })
    );
    assert_eq!(
        fit_tlearner(&x, &[1.0, 2.0], &[0.0, 1.0, 1.0], 5, 1, 7).err(),
        Some(TLearnerError::LengthMismatch)
    );
}
