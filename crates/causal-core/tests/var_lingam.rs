// Parity against lingam golden fixtures: adaptive-lasso unit cases, DirectLiNGAM, and full
// VARLiNGAM runs pruned and unpruned, including US macro growth rates.
use hirmos_causal_core::lars::predict_adaptive_lasso;
use hirmos_causal_core::{direct_lingam, run_var_lingam};
use nalgebra::DMatrix;
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-8 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

fn to_matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j])
}

#[test]
fn adaptive_lasso_matches_lingam() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/var_lingam.json")).unwrap();
    for (k, case) in root["adaptive_lasso"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let data = to_matrix(&case["data"]);
        let p = case["n_predictors"].as_u64().unwrap() as usize;
        let want: Vec<f64> = serde_json::from_value(case["coef"].clone()).unwrap();
        let predictors: Vec<usize> = (0..p).collect();
        let coef = predict_adaptive_lasso(&data, &predictors, p);
        for j in 0..p {
            close(&format!("lasso case {k} coef[{j}]"), coef[j], want[j]);
        }
    }
    println!("adaptive_lasso: all cases match");
}

#[test]
fn direct_lingam_matches_lingam() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/var_lingam.json")).unwrap();
    let fx = &root["direct_lingam"];
    let data = to_matrix(&fx["data"]);
    let want_order: Vec<usize> = serde_json::from_value(fx["causal_order"].clone()).unwrap();
    let want_b = to_matrix(&fx["adjacency_matrix"]);
    let (order, b) = direct_lingam(&data);
    assert_eq!(order, want_order, "causal order");
    for i in 0..b.nrows() {
        for j in 0..b.ncols() {
            close(&format!("B[{i}][{j}]"), b[(i, j)], want_b[(i, j)]);
        }
    }
    println!("direct_lingam: order and adjacency match");
}

#[test]
fn var_lingam_matches_lingam() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/var_lingam.json")).unwrap();
    for name in ["var_lingam", "var_lingam_noprune", "var_lingam_macro"] {
        let fx = &root[name];
        let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
        let lags = fx["lags"].as_u64().unwrap() as usize;
        let prune = fx["prune"].as_bool().unwrap();
        let result = run_var_lingam(&data, lags, prune);

        assert_eq!(
            result.k_ar,
            fx["k_ar"].as_u64().unwrap() as usize,
            "{name}: k_ar"
        );
        let want_order: Vec<usize> = serde_json::from_value(fx["causal_order"].clone()).unwrap();
        assert_eq!(result.causal_order, want_order, "{name}: causal order");

        let want_ar: Vec<Vec<Vec<f64>>> = serde_json::from_value(fx["ar_coefs"].clone()).unwrap();
        for (tau, m) in want_ar.iter().enumerate() {
            for i in 0..m.len() {
                for j in 0..m[0].len() {
                    close(
                        &format!("{name}: M[{tau}][{i}][{j}]"),
                        result.ar_coefs[tau][(i, j)],
                        m[i][j],
                    );
                }
            }
        }
        let want_res: Vec<Vec<f64>> = serde_json::from_value(fx["residuals"].clone()).unwrap();
        for (i, row) in want_res.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                close(
                    &format!("{name}: resid[{i}][{j}]"),
                    result.residuals[(i, j)],
                    v,
                );
            }
        }
        let want_b: Vec<Vec<Vec<f64>>> =
            serde_json::from_value(fx["adjacency_matrices"].clone()).unwrap();
        for (tau, m) in want_b.iter().enumerate() {
            for i in 0..m.len() {
                for j in 0..m[0].len() {
                    close(
                        &format!("{name}: B[{tau}][{i}][{j}]"),
                        result.adjacency_matrices[tau][(i, j)],
                        m[i][j],
                    );
                }
            }
        }
        println!("{name}: k_ar, order, coefs, residuals and adjacency all match");
    }
}

#[test]
fn mt19937_and_bootstrap_match_lingam() {
    use hirmos_causal_core::nprandom::Mt19937;
    use hirmos_causal_core::var_lingam_bootstrap;
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/var_lingam.json")).unwrap();

    let want_stream: Vec<u64> = serde_json::from_value(root["mt_stream"].clone()).unwrap();
    let mut mt = Mt19937::seeded(97);
    let got: Vec<u64> = (0..want_stream.len()).map(|_| mt.randint(233)).collect();
    assert_eq!(got, want_stream, "mt19937 randint stream");
    println!("mt19937: randint stream matches bitwise");

    let fx = &root["bootstrap"];
    let data: Vec<Vec<f64>> = serde_json::from_value(fx["data"].clone()).unwrap();
    let lags = fx["lags"].as_u64().unwrap() as usize;
    let n_sampling = fx["n_sampling"].as_u64().unwrap() as usize;
    let seed = fx["seed"].as_u64().unwrap() as u32;
    let result = var_lingam_bootstrap(&data, lags, n_sampling, seed);
    let probs = result.get_probabilities(0.05);
    let want: Vec<Vec<Vec<f64>>> = serde_json::from_value(fx["probabilities"].clone()).unwrap();
    assert_eq!(probs.len(), want.len(), "block count");
    for (b, block) in want.iter().enumerate() {
        for i in 0..block.len() {
            for j in 0..block[0].len() {
                assert_eq!(probs[b][(i, j)], block[i][j], "prob[{b}][{i}][{j}]");
            }
        }
    }
    println!("bootstrap: probabilities match exactly");
}
