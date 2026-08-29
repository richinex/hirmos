// Parity for VECM order/rank selection and ML fit, plus the 805 Chow computation.
use hirmos_causal_core::{chow_break, select_coint_rank, vecm_fit, vecm_select_order};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

fn check_mat(label: &str, got: &nalgebra::DMatrix<f64>, want: &[Vec<f64>]) {
    for i in 0..want.len() {
        for j in 0..want[0].len() {
            close(&format!("{label}[{i}][{j}]"), got[(i, j)], want[i][j]);
        }
    }
}

#[test]
fn vecm_matches_statsmodels() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/vecm.json")).unwrap();
    let fx = &root["vecm"];
    let endog: Vec<Vec<f64>> = serde_json::from_value(fx["endog"].clone()).unwrap();
    let maxlags = fx["maxlags"].as_u64().unwrap() as usize;
    for case in fx["cases"].as_array().unwrap() {
        let det = case["deterministic"].as_str().unwrap();
        let det_order = case["det_order"].as_i64().unwrap() as i32;
        let aic_k = vecm_select_order(&endog, maxlags, det);
        assert_eq!(
            aic_k,
            case["aic_k"].as_u64().unwrap() as usize,
            "{det} aic_k"
        );
        let k_try = aic_k.max(1);
        let rank = select_coint_rank(&endog, det_order, k_try, 1);
        assert_eq!(rank, case["rank"].as_u64().unwrap() as usize, "{det} rank");
        let res = vecm_fit(&endog, k_try, 1, det);
        let alpha: Vec<Vec<f64>> = serde_json::from_value(case["alpha"].clone()).unwrap();
        let beta: Vec<Vec<f64>> = serde_json::from_value(case["beta"].clone()).unwrap();
        let dcc: Vec<Vec<f64>> = serde_json::from_value(case["det_coef_coint"].clone()).unwrap();
        let gamma: Vec<Vec<f64>> = serde_json::from_value(case["gamma"].clone()).unwrap();
        let pa: Vec<Vec<f64>> = serde_json::from_value(case["pvalues_alpha"].clone()).unwrap();
        check_mat(&format!("{det} alpha"), &res.alpha, &alpha);
        check_mat(&format!("{det} beta"), &res.beta, &beta);
        check_mat(&format!("{det} det_coef_coint"), &res.det_coef_coint, &dcc);
        check_mat(&format!("{det} gamma"), &res.gamma, &gamma);
        check_mat(&format!("{det} pvalues_alpha"), &res.pvalues_alpha, &pa);
        println!("{det}: order, rank, alpha, beta, gamma and alpha p-values match");
    }

    let fx = &root["chow"];
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    let x: Vec<f64> = serde_json::from_value(fx["x"].clone()).unwrap();
    let k = fx["k"].as_u64().unwrap() as usize;
    let (f, p) = chow_break(&y, &x, k);
    close("chow f", f, fx["f"].as_f64().unwrap());
    close("chow p", p, fx["p"].as_f64().unwrap());
    println!("chow: F and p match");
}
