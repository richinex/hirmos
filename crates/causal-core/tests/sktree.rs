// Parity for the sklearn tree/forest port: complete node structures and exact predictions.
use hirmos_causal_core::sktree::{build_tree, fit_forest, DecisionTree, TreeParams};
use serde_json::Value;

fn close(label: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-10 * want.abs().max(1.0),
        "{label}: got {got}, oracle {want}",
    );
}

fn assert_structure(got: &DecisionTree, want: &Value, label: &str) {
    let left: Vec<i64> = serde_json::from_value(want["children_left"].clone()).unwrap();
    let right: Vec<i64> = serde_json::from_value(want["children_right"].clone()).unwrap();
    let features: Vec<i64> = serde_json::from_value(want["feature"].clone()).unwrap();
    let thresholds: Vec<f64> = serde_json::from_value(want["threshold"].clone()).unwrap();
    let values: Vec<Vec<f64>> = serde_json::from_value(want["value"].clone()).unwrap();

    assert_eq!(got.nodes.len(), left.len(), "{label} node count");
    for (i, node) in got.nodes.iter().enumerate() {
        assert_eq!(node.left, left[i], "{label} node {i} left child");
        assert_eq!(node.right, right[i], "{label} node {i} right child");
        if left[i] >= 0 {
            assert_eq!(node.feature as i64, features[i], "{label} node {i} feature");
            close(
                &format!("{label} node {i} threshold"),
                node.threshold,
                thresholds[i],
            );
        }
        assert_eq!(
            node.value.len(),
            values[i].len(),
            "{label} node {i} value width"
        );
        for (c, (&g, &w)) in node.value.iter().zip(&values[i]).enumerate() {
            close(&format!("{label} node {i} value {c}"), g, w);
        }
    }
}

#[test]
fn sktree_matches_sklearn() {
    let root: Value = serde_json::from_str(include_str!("../oracle/fixtures/sktree.json")).unwrap();
    let x: Vec<Vec<f64>> = serde_json::from_value(root["X"].clone()).unwrap();
    let y: Vec<f64> = serde_json::from_value(root["y"].clone()).unwrap();
    let yc: Vec<f64> = serde_json::from_value(root["yc"].clone()).unwrap();
    let xt: Vec<Vec<f64>> = serde_json::from_value(root["Xt"].clone()).unwrap();
    let n_features = x[0].len();
    let x32: Vec<f32> = x.iter().flat_map(|r| r.iter().map(|&v| v as f32)).collect();

    let params = TreeParams {
        max_features: n_features,
        min_samples_leaf: 5,
        min_samples_split: 2,
    };
    let tree = build_tree(&x32, n_features, &y, None, None, &params, 3);
    let want: Vec<f64> = serde_json::from_value(root["tree_reg"]["pred"].clone()).unwrap();
    assert_structure(&tree, &root["tree_reg"]["structure"], "tree_reg");
    for (i, row) in xt.iter().enumerate() {
        let row32: Vec<f32> = row.iter().map(|&v| v as f32).collect();
        close(
            &format!("tree_reg pred[{i}]"),
            tree.predict_row(&row32)[0],
            want[i],
        );
    }
    println!("tree_reg: structure and predictions match");

    let params_c = TreeParams {
        max_features: ((n_features as f64).sqrt() as usize).max(1),
        min_samples_leaf: 5,
        min_samples_split: 2,
    };
    let treec = build_tree(&x32, n_features, &yc, None, Some(2), &params_c, 3);
    let want: Vec<f64> = serde_json::from_value(root["tree_clf"]["proba"].clone()).unwrap();
    assert_structure(&treec, &root["tree_clf"]["structure"], "tree_clf");
    for (i, row) in xt.iter().enumerate() {
        let row32: Vec<f32> = row.iter().map(|&v| v as f32).collect();
        let v = treec.predict_row(&row32);
        close(
            &format!("tree_clf proba[{i}]"),
            v[1] / (v[0] + v[1]),
            want[i],
        );
    }
    println!("tree_clf: structure and probabilities match");

    let rf = fit_forest(&x, &y, 30, 5, None, 7);
    let structures = root["rf_reg"]["structures"].as_array().unwrap();
    assert_eq!(rf.trees.len(), structures.len(), "rf_reg tree count");
    for (i, (tree, want)) in rf.trees.iter().zip(structures).enumerate() {
        assert_structure(tree, want, &format!("rf_reg tree {i}"));
    }
    let want: Vec<f64> = serde_json::from_value(root["rf_reg"]["pred"].clone()).unwrap();
    let got = rf.predict(&xt);
    for i in 0..want.len() {
        close(&format!("rf_reg pred[{i}]"), got[i], want[i]);
    }
    println!("rf_reg: all tree structures and predictions match");

    let rfc = fit_forest(&x, &yc, 30, 5, Some(2), 7);
    let structures = root["rf_clf"]["structures"].as_array().unwrap();
    assert_eq!(rfc.trees.len(), structures.len(), "rf_clf tree count");
    for (i, (tree, want)) in rfc.trees.iter().zip(structures).enumerate() {
        assert_structure(tree, want, &format!("rf_clf tree {i}"));
    }
    let want: Vec<f64> = serde_json::from_value(root["rf_clf"]["proba"].clone()).unwrap();
    let got = rfc.predict_proba(&xt);
    for i in 0..want.len() {
        close(&format!("rf_clf proba[{i}]"), got[i][1], want[i]);
    }
    println!("rf_clf: all tree structures and probabilities match");
}
