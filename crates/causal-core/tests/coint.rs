// Parity against statsmodels cointegration fixtures: Johansen across det orders and lag depths,
// Engle-Granger pairs, on synthetic I(1) systems and macro levels.
use hirmos_causal_core::{coint, coint_johansen};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn johansen_matches_statsmodels() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/coint.json")).unwrap();
    for fx in root["johansen"].as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
        let det = fx["det_order"].as_i64().unwrap() as i32;
        let k = fx["k_ar_diff"].as_u64().unwrap() as usize;
        let res = coint_johansen(&data, det, k);
        let eig: Vec<f64> = serde_json::from_value(fx["eig"].clone()).unwrap();
        let lr1: Vec<f64> = serde_json::from_value(fx["lr1"].clone()).unwrap();
        let lr2: Vec<f64> = serde_json::from_value(fx["lr2"].clone()).unwrap();
        let cvt: Vec<[f64; 3]> = serde_json::from_value(fx["cvt"].clone()).unwrap();
        let cvm: Vec<[f64; 3]> = serde_json::from_value(fx["cvm"].clone()).unwrap();
        for i in 0..eig.len() {
            close(&format!("{name} eig[{i}]"), res.eig[i], eig[i]);
            close(&format!("{name} lr1[{i}]"), res.lr1[i], lr1[i]);
            close(&format!("{name} lr2[{i}]"), res.lr2[i], lr2[i]);
            assert_eq!(res.cvt[i], cvt[i], "{name} cvt[{i}]");
            assert_eq!(res.cvm[i], cvm[i], "{name} cvm[{i}]");
        }
        println!("{name}: eig, trace, max-eig and tables match");
    }
}

#[test]
fn engle_granger_matches_statsmodels() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/coint.json")).unwrap();
    for fx in root["engle_granger"].as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let y0: Vec<f64> = serde_json::from_value(fx["y0"].clone()).unwrap();
        let y1: Vec<f64> = serde_json::from_value(fx["y1"].clone()).unwrap();
        let res = coint(&y0, &y1);
        close(
            &format!("{name} stat"),
            res.stat,
            fx["stat"].as_f64().unwrap(),
        );
        close(
            &format!("{name} pval"),
            res.pvalue,
            fx["pval"].as_f64().unwrap(),
        );
        let crit: Vec<f64> = serde_json::from_value(fx["crit"].clone()).unwrap();
        for k in 0..3 {
            close(&format!("{name} crit[{k}]"), res.crit[k], crit[k]);
        }
        println!("{name}: stat, p and crit match");
    }
}
