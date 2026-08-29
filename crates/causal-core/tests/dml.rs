// Parity for DoubleML PLR/IRM with random-forest nuisances at the causal worker's settings.
// Tolerance note: tree splits with exactly tied proxy improvements resolve by float summation
// order, which in sklearn depends on compiler autovectorization and differs between sklearn's
// own builds (macOS wheel vs Pyodide wasm). Estimates therefore match any given build to ~1e-4
// rather than bitwise; folds, RNG streams, and all non-tied splits are exact.
use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::{dml_irm, dml_plr};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-3 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn dml_matches_doubleml() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/dml.json")).unwrap();
    let w1: Vec<f64> = serde_json::from_value(root["w1"].clone()).unwrap();
    let w2: Vec<f64> = serde_json::from_value(root["w2"].clone()).unwrap();
    let x: Vec<Vec<f64>> = w1.iter().zip(&w2).map(|(&a, &b)| vec![a, b]).collect();

    for case in root["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let d: Vec<f64> = serde_json::from_value(case["d"].clone()).unwrap();
        let y: Vec<f64> = serde_json::from_value(case["y"].clone()).unwrap();
        let light = case["light"].as_bool().unwrap();
        let (n_estimators, folds) = if light { (80, 2) } else { (200, 5) };
        let treat_binary = d.iter().all(|&v| v == 0.0 || v == 1.0);
        let mut fold_stream = Mt19937::seeded(7);
        let res = match case["kind"].as_str().unwrap() {
            "irm" => dml_irm(
                &x,
                &y,
                &d,
                case["score"].as_str().unwrap() == "ATTE",
                folds,
                n_estimators,
                5,
                0.01,
                7,
                &mut fold_stream,
            ),
            _ => dml_plr(
                &x,
                &y,
                &d,
                treat_binary,
                folds,
                n_estimators,
                5,
                7,
                &mut fold_stream,
            ),
        };
        close(
            &format!("{name} coef"),
            res.coef,
            case["coef"].as_f64().unwrap(),
        );
        close(&format!("{name} se"), res.se, case["se"].as_f64().unwrap());
        close(
            &format!("{name} ci_low"),
            res.ci_low,
            case["ci_low"].as_f64().unwrap(),
        );
        close(
            &format!("{name} ci_high"),
            res.ci_high,
            case["ci_high"].as_f64().unwrap(),
        );
        println!("{name}: coef, se and confidence interval match");
    }
}
