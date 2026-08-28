// Parity against numpy RNG streams and causationentropy golden fixtures.
use hirmos_causal_core::nprandom::NpRng;
use serde_json::Value;

#[test]
fn nprng_matches_numpy() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/ocse.json")).unwrap();
    let fx = &root["rng"];
    let raw: Vec<u64> = serde_json::from_value(fx["raw"].clone()).unwrap();
    let mut rng = NpRng::seeded(42);
    for (i, &want) in raw.iter().enumerate() {
        assert_eq!(rng.next_u64(), want, "raw[{i}]");
    }
    let mut rng = NpRng::seeded(42);
    let want_perms: Vec<Vec<usize>> = serde_json::from_value(fx["perms"].clone()).unwrap();
    for (k, want) in
        [(5usize, 0), (10, 1), (3, 2), (100, 3), (7, 4)].map(|(k, i)| (k, &want_perms[i]))
    {
        assert_eq!(&rng.permutation(k), want, "permutation({k})");
    }
    let want_arr: Vec<usize> = serde_json::from_value(fx["array_perm"].clone()).unwrap();
    assert_eq!(
        rng.permutation_of(&[4, 9, 2, 7]),
        want_arr,
        "array permutation"
    );
    println!("nprng: raw stream and permutations match");
}

use hirmos_causal_core::ocse::{cmi, discover_network_reference_compatible, CmiMethod};
use nalgebra::DMatrix;

fn to_matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j])
}

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

#[test]
fn cmi_estimators_match() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/ocse.json")).unwrap();
    let fx = &root["cmi_cases"];
    let x = to_matrix(&fx["x"]);
    let y = to_matrix(&fx["y"]);
    let z = to_matrix(&fx["z"]);
    for case in fx["cases"].as_array().unwrap() {
        let method = match case["method"].as_str().unwrap() {
            "gaussian" => CmiMethod::Gaussian,
            _ => CmiMethod::Knn,
        };
        let k = case["k"].as_u64().unwrap() as usize;
        let label = format!("{}(k={k})", case["method"].as_str().unwrap());
        close(
            &format!("{label} mi"),
            cmi(&x, &y, None, method, k),
            case["mi"].as_f64().unwrap(),
        );
        close(
            &format!("{label} cmi"),
            cmi(&x, &y, Some(&z), method, k),
            case["cmi"].as_f64().unwrap(),
        );
    }
    println!("cmi estimators: all cases match");
}

#[test]
fn discover_network_matches() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/ocse.json")).unwrap();
    for name in ["ocse_gaussian", "ocse_knn"] {
        let fx = &root[name];
        let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
        let max_lag = fx["max_lag"].as_u64().unwrap() as usize;
        let n_shuffles = fx["n_shuffles"].as_u64().unwrap() as usize;
        let method = match fx["information"].as_str().unwrap() {
            "gaussian" => CmiMethod::Gaussian,
            _ => CmiMethod::Knn,
        };
        let mut edges = discover_network_reference_compatible(
            &data, max_lag, 0.05, 0.05, n_shuffles, method, 5,
        );
        // networkx lists edges grouped by source node; discovery order is by target.
        edges.sort_by_key(|e| e.src);
        let want = fx["edges"].as_array().unwrap();
        assert_eq!(edges.len(), want.len(), "{name}: edge count");
        for (e, w) in edges.iter().zip(want) {
            assert_eq!(e.src, w["src"].as_u64().unwrap() as usize, "{name}: src");
            assert_eq!(e.dst, w["dst"].as_u64().unwrap() as usize, "{name}: dst");
            assert_eq!(e.lag, w["lag"].as_u64().unwrap() as usize, "{name}: lag");
            close(&format!("{name}: cmi"), e.cmi, w["cmi"].as_f64().unwrap());
            assert_eq!(e.p_value, w["p_value"].as_f64().unwrap(), "{name}: p_value");
        }
        println!("{name}: all {} edges match exactly", edges.len());
    }
}
