use hirmos_causal_core::grf::{
    forest::{self, SeedMode, Variance},
    sampling::Clusters,
    tree::{Data, Honesty, Options},
};
use serde_json::Value;
fn number(v: &Value) -> f64 {
    v.as_str()
        .map(|v| v.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap_or(f64::NAN)
}
fn floats(v: &Value) -> Vec<f64> {
    if v.is_string() || v.is_number() || v.is_null() {
        return vec![number(v)];
    }
    v.as_array().unwrap().iter().map(number).collect()
}
fn ids(v: &Value) -> Vec<usize> {
    if let Some(i) = v.as_u64() {
        return vec![i as usize];
    }
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect()
}
fn close(a: Option<f64>, b: &Value, context: &str) {
    if b.is_null() {
        assert!(a.is_none(), "{context}: expected undefined, got {a:?}");
    } else {
        let a = a.expect(context);
        let b = number(b);
        assert!(
            (a - b).abs() <= 2e-10 * b.abs().max(1.0),
            "{context}: {a} != {b}"
        );
    }
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn installed_r_forests_match() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/forest.json")).unwrap();
    assert_eq!(fixture["version"], "2.6.1");
    for c in fixture["cases"].as_array().unwrap() {
        let case = c["case"].as_u64().unwrap();
        let columns: Vec<_> = c["columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(floats)
            .collect();
        let test: Vec<_> = c["test"].as_array().unwrap().iter().map(floats).collect();
        let clusters = Clusters::new(
            &ids(&c["labels"]),
            c["per_cluster"].as_u64().unwrap() as usize,
        )
        .unwrap();
        let group = c["group"].as_u64().unwrap() as usize;
        let options = forest::Options {
            trees: 41,
            group_size: group,
            sample_fraction: 0.4,
            seed: c["seed"].as_u64().unwrap() as u32,
            seed_mode: if c["legacy"].as_bool().unwrap() {
                SeedMode::Legacy
            } else {
                SeedMode::Indexed
            },
            batches: c["threads"].as_u64().unwrap() as usize,
            tree: Options {
                mtry: 3,
                min_node_size: 3,
                alpha: 0.05,
                imbalance_penalty: 0.0,
                honesty: if c["honesty"].as_bool().unwrap() {
                    Honesty::Enabled {
                        fraction: 0.5,
                        prune: c["prune"].as_bool().unwrap(),
                    }
                } else {
                    Honesty::Disabled
                },
            },
        };
        let f = forest::train(
            Data {
                columns: &columns,
                outcome: 4,
                treatment: 5,
                weight: c["weighted"].as_bool().unwrap().then_some(6),
            },
            &clusters,
            &options,
        )
        .unwrap();
        let expected = c["trees"].as_array().unwrap();
        let errors = f.oob_errors(&columns, &columns[4], &columns[5]).unwrap();
        for (row, value) in errors.iter().enumerate() {
            let expected = &c["oob"][row];
            close(
                value.map(|v| v.0),
                &expected["debiased.error"],
                "debiased error",
            );
            close(
                value.map(|v| v.1),
                &expected["excess.error"],
                "excess error",
            );
        }
        for (x, oob, key) in [
            (&columns, true, "weights_oob"),
            (&test, false, "weights_new"),
        ] {
            let weights = f.weights(x, oob).unwrap();
            for (i, row) in weights.iter().take(13).enumerate() {
                let mut dense = vec![0.0; columns[0].len()];
                for &(j, v) in row {
                    dense[j] = v;
                }
                for (j, v) in dense.iter().enumerate() {
                    close(Some(*v), &c[key][i][j], key);
                }
            }
        }
        let importance = f.variable_importance(4, 4, 2.0).unwrap();
        for (i, v) in importance.iter().enumerate() {
            close(Some(*v), &c["importance"][i], "importance");
        }
        let frequencies = f.split_frequencies(4, 4).unwrap();
        for (i, row) in frequencies.iter().enumerate() {
            assert_eq!(*row, ids(&c["frequencies"][i]));
        }
        assert_eq!(f.trees.len(), expected.len(), "case {case}");
        for (i, (a, e)) in f.trees.iter().zip(expected).enumerate() {
            assert_eq!(
                a.root,
                e["root"].as_u64().unwrap() as usize,
                "case {case} tree {i}"
            );
            assert_eq!(a.drawn_samples, ids(&e["drawn"]), "case {case} tree {i}");
            let vars = ids(&e["vars"]);
            let values = floats(&e["values"]);
            let children = e["children"].as_array().unwrap();
            let left = ids(&children[0]);
            let right = ids(&children[1]);
            let samples = e["samples"].as_array().unwrap();
            assert_eq!(a.nodes.len(), vars.len(), "case {case} tree {i}");
            for (j, node) in a.nodes.iter().enumerate() {
                assert_eq!(
                    node.children.unwrap_or([0, 0]),
                    [left[j], right[j]],
                    "case {case} tree {i} node {j}"
                );
                assert_eq!(
                    node.samples,
                    ids(&samples[j]),
                    "case {case} tree {i} node {j}"
                );
                if let Some(split) = node.split {
                    assert_eq!(split.variable, vars[j], "case {case} tree {i} node {j}");
                    assert!(
                        split.value == values[j] || (split.value.is_nan() && values[j].is_nan())
                    );
                    assert_eq!(split.missing_left, e["missing"][j].as_bool().unwrap());
                }
            }
        }
        for (x, oob, key) in [(&columns, true, "oob"), (&test, false, "predictions")] {
            let actual = f.predict(x, oob, group > 1).unwrap();
            let expected = c[key].as_array().unwrap();
            assert_eq!(actual.len(), expected.len());
            for (row, (a, b)) in actual.iter().zip(expected).enumerate() {
                let context = format!("case {case} {key} row {row}");
                close(a.effect, &b["predictions"], &context);
                match a.variance {
                    Variance::NotRequested => assert_eq!(group, 1),
                    Variance::Unavailable => close(None, &b["variance.estimates"], &context),
                    Variance::Estimate(v) => close(Some(v), &b["variance.estimates"], &context),
                }
            }
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn forest_boundaries_and_absent_oob_predictions() {
    let columns = vec![
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        vec![0.0, 1.0, 2.0, 4.0, 5.0, 6.0, 7.0, 9.0],
        vec![0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0],
    ];
    let data = Data {
        columns: &columns,
        outcome: 1,
        treatment: 2,
        weight: None,
    };
    let clusters = Clusters::new(&(0..8).collect::<Vec<_>>(), 1).unwrap();
    let mut options = forest::Options {
        trees: 4,
        group_size: 1,
        sample_fraction: 1.0,
        seed: 7,
        seed_mode: SeedMode::Indexed,
        batches: 1,
        tree: Options {
            mtry: 1,
            min_node_size: 1,
            honesty: Honesty::Disabled,
            alpha: 0.05,
            imbalance_penalty: 0.0,
        },
    };
    let f = forest::train(data, &clusters, &options).unwrap();
    let oob = f.predict(&columns, true, false).unwrap();
    assert!(oob
        .iter()
        .all(|p| p.effect.is_none() && p.contributing_trees == 0));
    assert!(f.predict(&columns, false, true).is_err());
    assert!(f.predict(&[], false, false).is_err());
    assert!(f.predict(&[vec![0.0]], true, false).is_err());
    options.group_size = 2; // grouped inference cannot sample more than half
    assert!(forest::train(data, &clusters, &options).is_err());
    options.sample_fraction = 0.5;
    options.group_size = 0;
    assert!(forest::train(data, &clusters, &options).is_err());
    options.group_size = 2;
    options.batches = 0;
    assert!(forest::train(data, &clusters, &options).is_err());
    options.batches = 1;
    options.trees = 0;
    assert!(forest::train(data, &clusters, &options).is_err());
    options.trees = 4;
    let wrong = Clusters::new(&[0, 1], 1).unwrap();
    assert!(forest::train(data, &wrong, &options).is_err());
}
