use hirmos_causal_core::estimation::{ols_two_way, weighted_within, WithinErrors, WithinProblem};
use nalgebra::DMatrix;
use serde_json::Value;
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-8 * b.abs().max(1.0), "{a} != {b}");
}
fn matrix(v: &Value) -> DMatrix<f64> {
    let r: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(r.len(), r[0].len(), |i, j| r[i][j])
}

#[test]
fn weighted_effects_match_panelols() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/within.json")).unwrap();
    for name in [
        "weighted_unit",
        "weighted_time",
        "weighted_both",
        "weighted_unbalanced",
        "weighted_both_unbalanced",
        "weighted_school",
    ] {
        let f = &root[name];
        let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
        let w: Vec<f64> = serde_json::from_value(f["weights"].clone()).unwrap();
        let units: Vec<u64> = serde_json::from_value(f["groups"].clone()).unwrap();
        let times: Vec<u64> = serde_json::from_value(f["times"].clone()).unwrap();
        let clusters: Vec<u64> = serde_json::from_value(f["clusters"].clone()).unwrap();
        let x = matrix(&f["x"]);
        for (kind, errors) in [
            ("classical", WithinErrors::Classical),
            ("hc1", WithinErrors::Hc1),
            ("cluster", WithinErrors::ClusterBy(&clusters)),
        ] {
            let (u, t) = if name == "weighted_time" {
                (&times, None)
            } else {
                (
                    &units,
                    if name.contains("both") {
                        Some(times.as_slice())
                    } else {
                        None
                    },
                )
            };
            let result = weighted_within(&y, &x, u, t, &w, errors).unwrap();
            assert_eq!(
                result.kept,
                serde_json::from_value::<Vec<usize>>(f["kept"].clone()).unwrap(),
                "{name} {kind}"
            );
            assert_eq!(result.df_resid as u64, f["df_resid"].as_u64().unwrap());
            close(result.ssr, f["resid_ss"].as_f64().unwrap());
            for i in 0..y.len() {
                close(result.resid[i], f["residuals"][i].as_f64().unwrap());
            }
            // PanelOLS rsquared_within always means entity-within, including time-only models.
            if name != "weighted_time" {
                close(
                    result.rsquared_within,
                    f["rsquared_within"].as_f64().unwrap(),
                );
            }
            for i in 0..result.params.len() {
                for (actual, key) in [
                    (result.params[i], "params"),
                    (result.bse[i], "bse"),
                    (result.pvalues[i], "pvalues"),
                ] {
                    close(actual, f[kind][key][i].as_f64().unwrap());
                }
                close(
                    result.conf_int[i].0,
                    f[kind]["conf_int"][i][0].as_f64().unwrap(),
                );
                close(
                    result.conf_int[i].1,
                    f[kind]["conf_int"][i][1].as_f64().unwrap(),
                );
                for j in 0..result.params.len() {
                    close(
                        result.covariance[(i, j)],
                        f[kind]["covariance"][i][j].as_f64().unwrap(),
                    );
                }
            }
            let scaled: Vec<f64> = w.iter().map(|w| w * 37.0).collect();
            let rescaled = weighted_within(&y, &x, u, t, &scaled, errors).unwrap();
            for i in 0..result.params.len() {
                close(rescaled.params[i], result.params[i]);
                close(rescaled.bse[i], result.bse[i]);
            }
        }
    }
}

#[test]
fn equal_weights_preserve_existing_fit_and_invalid_weights_are_refused() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/within.json")).unwrap();
    let f = &root["two_way_balanced"];
    let y: Vec<f64> = serde_json::from_value(f["y"].clone()).unwrap();
    let u: Vec<u64> = serde_json::from_value(f["groups"].clone()).unwrap();
    let t: Vec<u64> = serde_json::from_value(f["times"].clone()).unwrap();
    let x = matrix(&f["x"]);
    for errors in [
        WithinErrors::Classical,
        WithinErrors::Hc1,
        WithinErrors::Cluster,
    ] {
        let a = ols_two_way(&y, &x, &u, &t, errors).unwrap();
        let b = weighted_within(&y, &x, &u, Some(&t), &vec![5.0; y.len()], errors).unwrap();
        assert_eq!(a.kept, b.kept);
        for i in 0..a.params.len() {
            close(a.params[i], b.params[i]);
            for j in 0..a.params.len() {
                close(a.covariance[(i, j)], b.covariance[(i, j)]);
            }
        }
    }
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let mut w = vec![1.0; y.len()];
        w[0] = invalid;
        assert_eq!(
            weighted_within(&y, &x, &u, None, &w, WithinErrors::Classical).err(),
            Some(WithinProblem::InvalidWeights)
        );
    }
    assert_eq!(
        weighted_within(&y, &x, &u, None, &[1.0], WithinErrors::Classical).err(),
        Some(WithinProblem::InvalidWeights)
    );
}
