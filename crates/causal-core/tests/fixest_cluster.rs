use hirmos_causal_core::estimation::{
    ols_within_convention, weighted_within_convention, WithinConvention, WithinErrors,
};
use nalgebra::DMatrix;
use serde_json::Value;
fn close(a: f64, b: f64, case: &str) {
    assert!(
        (a - b).abs() < 1e-9 * b.abs().max(1.0),
        "{case}: {a} != {b}"
    );
}
#[test]
fn clustered_fixed_effects_match_fixest_0142() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/fixest_cluster.json")).unwrap();
    for (name, f) in fixture["cases"].as_object().unwrap() {
        let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
        let x: Vec<Vec<f64>> = serde_json::from_value(f["x"].clone()).unwrap();
        let x = DMatrix::from_fn(y.len(), 2, |i, j| x[i][j]);
        let u: Vec<u64> = serde_json::from_value(f["unit"].clone()).unwrap();
        let t: Vec<u64> = serde_json::from_value(f["time"].clone()).unwrap();
        let c: Vec<u64> = serde_json::from_value(f["cluster"].clone()).unwrap();
        let w: Vec<f64> = serde_json::from_value(f["weights"].clone()).unwrap();
        let (units, times) = match f["fe"].as_str().unwrap() {
            "unit" => (u.as_slice(), None),
            "time" => (t.as_slice(), None),
            _ => (u.as_slice(), Some(t.as_slice())),
        };
        let fit = if f["weighted"].as_bool().unwrap() {
            weighted_within_convention(
                &y,
                &x,
                units,
                times,
                &w,
                WithinErrors::ClusterBy(&c),
                WithinConvention::Fixest,
            )
        } else {
            ols_within_convention(
                &y,
                &x,
                units,
                times,
                WithinErrors::ClusterBy(&c),
                WithinConvention::Fixest,
            )
        }
        .unwrap();
        let parameters = fit.cluster_parameters.unwrap();
        assert_eq!(parameters, f["k"].as_u64().unwrap() as usize, "{name}");
        assert_eq!(
            fit.inference_df,
            f["df"].as_u64().unwrap() as usize,
            "{name}"
        );
        let g = fit.inference_df as f64 + 1.0;
        let correction = g / (g - 1.0) * (y.len() - 1) as f64 / (y.len() - parameters) as f64;
        for i in 0..2 {
            close(fit.params[i], f["coefficients"][i].as_f64().unwrap(), name);
            close(fit.bse[i], f["se"][i].as_f64().unwrap(), name);
            close(fit.pvalues[i], f["p"][i].as_f64().unwrap(), name);
            close(
                fit.conf_int[i].0,
                f["interval"][i][0].as_f64().unwrap(),
                name,
            );
            close(
                fit.conf_int[i].1,
                f["interval"][i][1].as_f64().unwrap(),
                name,
            );
            for j in 0..2 {
                close(
                    fit.covariance[(i, j)],
                    f["covariance"][i][j].as_f64().unwrap(),
                    name,
                );
                close(
                    fit.covariance[(i, j)] / correction,
                    f["raw"][i][j].as_f64().unwrap(),
                    name,
                );
            }
        }
    }
}
