// Parity for dowhy's add_unobserved_common_cause direct simulation: automatic kappa ranges
// (logistic flips and outcome correlations) and the seeded 10x10 simulation grid.
use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::{infer_kappa_t, infer_kappa_y, unobserved_common_cause_grid};
use serde_json::Value;

#[test]
fn unobserved_grid_matches_dowhy() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/unobserved.json")).unwrap();
    let t: Vec<f64> = serde_json::from_value(root["data"]["t"].clone()).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["data"]["y"].clone()).unwrap();
    let w1: Vec<f64> = serde_json::from_value(root["data"]["w1"].clone()).unwrap();
    let w2: Vec<f64> = serde_json::from_value(root["data"]["w2"].clone()).unwrap();
    let occ: Vec<Vec<f64>> = w1.iter().zip(&w2).map(|(&a, &b)| vec![a, b]).collect();

    let kt = infer_kappa_t(&occ, &t);
    let want_kt: Vec<f64> = serde_json::from_value(root["kappa_t"].clone()).unwrap();
    assert_eq!(kt.len(), want_kt.len(), "kappa_t length");
    for (g, w) in kt.iter().zip(&want_kt) {
        assert!((g - w).abs() <= 1e-10, "kappa_t: got {g}, oracle {w}");
    }

    let ky = infer_kappa_y(&occ, &y);
    let want_ky: Vec<f64> = serde_json::from_value(root["kappa_y"].clone()).unwrap();
    assert_eq!(ky.len(), want_ky.len(), "kappa_y length");
    for (g, w) in ky.iter().zip(&want_ky) {
        assert!((g - w).abs() <= 1e-10, "kappa_y: got {g}, oracle {w}");
    }

    let mut mt = Mt19937::seeded(root["seed"].as_u64().unwrap() as u32);
    let grid = unobserved_common_cause_grid(&t, &y, &occ, &kt, &ky, &mut mt);
    let want: Vec<Vec<f64>> = serde_json::from_value(root["matrix"].clone()).unwrap();
    let mut maxdev = 0.0f64;
    for (row, wrow) in grid.iter().zip(&want) {
        for (g, w) in row.iter().zip(wrow) {
            maxdev = maxdev.max((g - w).abs());
        }
    }
    println!("unobserved grid max deviation {maxdev:.3e}");
    assert!(maxdev <= 1e-8, "grid deviation {maxdev}");
}
