// Parity for the 902 preprocessing helpers: scipy shapiro, complete-linkage redundancy, VIF.
use hirmos_causal_core::{cluster_redundant, correlation_matrix, shapiro, vif_redundant};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn preprocess_matches_oracles() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/preprocess.json")).unwrap();

    for case in root["shapiro"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let x: Vec<f64> = serde_json::from_value(case["x"].clone()).unwrap();
        let (w, p) = shapiro(&x);
        close(&format!("shapiro {name} w"), w, case["w"].as_f64().unwrap());
        close(&format!("shapiro {name} p"), p, case["p"].as_f64().unwrap());
    }
    println!("shapiro: all cases match");

    let fx = &root["cluster"];
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let rows = data.len();
    let cols = data[0].len();
    let m = DMatrix::from_fn(rows, cols, |i, j| data[i][j]);
    let corr = correlation_matrix(&m);
    let want_corr: Vec<Vec<f64>> = serde_json::from_value(fx["correlation"].clone()).unwrap();
    for row in 0..cols {
        for column in 0..cols {
            close(
                &format!("correlation {row},{column}"),
                corr[(row, column)],
                want_corr[row][column],
            );
        }
    }
    let (keep, drop, clusters) = cluster_redundant(&corr, fx["threshold"].as_f64().unwrap());
    let want_keep: Vec<usize> = serde_json::from_value(fx["keep"].clone()).unwrap();
    let want_drop: Vec<usize> = serde_json::from_value(fx["drop"].clone()).unwrap();
    let mut want_clusters: Vec<Vec<usize>> =
        serde_json::from_value(fx["clusters"].clone()).unwrap();
    want_clusters.sort_by_key(|c| c[0]);
    assert_eq!(keep, want_keep, "cluster keep");
    assert_eq!(drop, want_drop, "cluster drop");
    assert_eq!(clusters, want_clusters, "cluster lists");
    println!("cluster: partition matches");

    let fx = &root["vif"];
    let (keep, drop, hist) = vif_redundant(&m, fx["threshold"].as_f64().unwrap());
    let want_keep: Vec<usize> = serde_json::from_value(fx["keep"].clone()).unwrap();
    let want_drop: Vec<usize> = serde_json::from_value(fx["drop"].clone()).unwrap();
    assert_eq!(keep, want_keep, "vif keep");
    assert_eq!(drop, want_drop, "vif drop");
    let want_hist: Vec<(usize, f64)> = serde_json::from_value(fx["history"].clone()).unwrap();
    assert_eq!(hist.len(), want_hist.len(), "vif history length");
    for (k, (got, want)) in hist.iter().zip(&want_hist).enumerate() {
        assert_eq!(got.0, want.0, "vif history idx {k}");
        close(&format!("vif history value {k}"), got.1, want.1);
    }
    println!("vif: elimination path matches");
}

#[test]
fn vif_reports_exact_collinearity_without_panicking() {
    let matrix = DMatrix::from_row_slice(
        4,
        3,
        &[1.0, 1.0, 3.0, 2.0, 2.0, 1.0, 3.0, 3.0, 4.0, 4.0, 4.0, 2.0],
    );
    let (keep, drop, history) = vif_redundant(&matrix, 10.0);
    assert_eq!(keep.len() + drop.len(), 3);
    assert!(history.first().is_some_and(|entry| entry.1.is_infinite()));
}
