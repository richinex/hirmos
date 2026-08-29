// Parity for HAC OLS, WLS, and Durbin-Watson against statsmodels fixtures.
use hirmos_causal_core::{durbin_watson, ols_hac, wls};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn estimation_matches_statsmodels() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/estimation.json")).unwrap();
    let fx = &root["hac"];
    let y: Vec<f64> = serde_json::from_value(fx["y"].clone()).unwrap();
    let xr: Vec<Vec<f64>> = serde_json::from_value(fx["x"].clone()).unwrap();
    let x = DMatrix::from_fn(xr.len(), xr[0].len(), |i, j| xr[i][j]);

    for case in fx["cases"].as_array().unwrap() {
        let maxlags = case["maxlags"].as_u64().unwrap() as usize;
        let r = ols_hac(&y, &x, maxlags);
        let params: Vec<f64> = serde_json::from_value(case["params"].clone()).unwrap();
        let bse: Vec<f64> = serde_json::from_value(case["bse"].clone()).unwrap();
        let pv: Vec<f64> = serde_json::from_value(case["pvalues"].clone()).unwrap();
        let ci: Vec<Vec<f64>> = serde_json::from_value(case["conf_int"].clone()).unwrap();
        for j in 0..params.len() {
            close(&format!("hac{maxlags} params[{j}]"), r.params[j], params[j]);
            close(&format!("hac{maxlags} bse[{j}]"), r.bse[j], bse[j]);
            close(&format!("hac{maxlags} p[{j}]"), r.pvalues[j], pv[j]);
            close(
                &format!("hac{maxlags} ci_lo[{j}]"),
                r.conf_int[j].0,
                ci[j][0],
            );
            close(
                &format!("hac{maxlags} ci_hi[{j}]"),
                r.conf_int[j].1,
                ci[j][1],
            );
        }
        close(
            &format!("hac{maxlags} dw"),
            durbin_watson(&r.resid),
            case["dw"].as_f64().unwrap(),
        );
        close(
            &format!("hac{maxlags} r2"),
            r.rsquared,
            case["rsquared"].as_f64().unwrap(),
        );
    }
    println!("hac + dw: all cases match");

    let fw = &root["wls"];
    let weights: Vec<f64> = serde_json::from_value(fw["weights"].clone()).unwrap();
    let r = wls(&y, &x, &weights);
    let params: Vec<f64> = serde_json::from_value(fw["params"].clone()).unwrap();
    let pv: Vec<f64> = serde_json::from_value(fw["pvalues"].clone()).unwrap();
    for j in 0..params.len() {
        close(&format!("wls params[{j}]"), r.params[j], params[j]);
        close(&format!("wls p[{j}]"), r.pvalues[j], pv[j]);
    }
    println!("wls: params and pvalues match");
}
