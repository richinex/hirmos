use hirmos_causal_core::grf::{
    sampling::{Clusters, Sampler},
    tree::{self, Data, Honesty, Options},
};
use serde_json::Value;
fn floats(v: &Value) -> Vec<f64> {
    v.as_array().unwrap().iter().map(number).collect()
}
fn number(v: &Value) -> f64 {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap_or(f64::NAN)
}
fn ids(v: &Value) -> Vec<usize> {
    // jsonlite auto_unbox represents a one-row leaf by a scalar.
    if let Some(i) = v.as_u64() {
        return vec![i as usize];
    }
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect()
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn complete_trees_match_pinned_grf() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/tree.json")).unwrap();
    assert_eq!(fixture["version"], "2.6.1");
    let mut promoted = false;
    let mut empty = false;
    for c in fixture["cases"].as_array().unwrap() {
        let seed = c["seed"].as_u64().unwrap() as u32;
        let columns: Vec<_> = c["columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(floats)
            .collect();
        let clusters = Clusters::new(
            &ids(&c["labels"]),
            c["per_cluster"].as_u64().unwrap() as usize,
        )
        .unwrap();
        let mut rng = Sampler::new(seed);
        let selected = rng
            .sample_clusters(&clusters, c["fraction"].as_f64().unwrap())
            .unwrap();
        let options = Options {
            mtry: c["mtry"].as_u64().unwrap() as u32,
            min_node_size: c["min_size"].as_u64().unwrap() as usize,
            honesty: if c["honesty"].as_bool().unwrap() {
                Honesty::Enabled {
                    fraction: c["honesty_fraction"].as_f64().unwrap(),
                    prune: c["prune"].as_bool().unwrap(),
                }
            } else {
                Honesty::Disabled
            },
            alpha: c["alpha"].as_f64().unwrap(),
            imbalance_penalty: c["penalty"].as_f64().unwrap(),
        };
        let actual = tree::train(
            Data {
                columns: &columns,
                outcome: 4,
                treatment: 5,
                weight: c["weight"].as_u64().map(|v| v as usize),
            },
            &mut rng,
            &clusters,
            &selected,
            &options,
        )
        .unwrap();
        let expected = &c["expected"];
        assert_eq!(
            actual.root,
            expected["root"].as_u64().unwrap() as usize,
            "seed {seed}"
        );
        promoted |= actual.root != 0;
        assert_eq!(actual.drawn_samples, ids(&expected["drawn"]), "seed {seed}");
        let nodes = expected["nodes"].as_array().unwrap();
        assert_eq!(actual.nodes.len(), nodes.len(), "seed {seed}");
        for (i, (a, e)) in actual.nodes.iter().zip(nodes).enumerate() {
            assert_eq!(
                a.children.unwrap_or([0, 0]).to_vec(),
                ids(&e["children"]),
                "seed {seed} node {i}"
            );
            assert_eq!(a.samples, ids(&e["samples"]), "seed {seed} node {i}");
            empty |= a.children.is_none() && a.samples.is_empty();
            if let Some(s) = a.split {
                assert_eq!(
                    s.variable,
                    e["variable"].as_u64().unwrap() as usize,
                    "seed {seed} node {i}"
                );
                assert_eq!(s.missing_left, e["missing_left"].as_bool().unwrap());
                if e["value"].is_null() {
                    assert!(s.value.is_nan());
                } else {
                    assert_eq!(s.value, number(&e["value"]));
                }
            }
            match (a.moments, &e["moments"]) {
                (None, e) => assert!(e.is_null()),
                (Some(a), e) => {
                    for (a, b) in a.iter().zip(floats(e)) {
                        assert!(
                            (a - b).abs() <= 2e-12 * b.abs().max(1.0),
                            "seed {seed} node {i}: {a} != {b}"
                        );
                    }
                }
            }
        }
        let leaves: Vec<_> = (0..columns[0].len())
            .map(|i| actual.leaf(&columns, i).unwrap())
            .collect();
        assert_eq!(leaves, ids(&expected["leaves"]), "seed {seed}");
    }
    assert!(empty, "fixtures must exercise empty honest leaves");
    assert!(promoted, "fixtures must exercise root promotion");
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_training_inputs_are_rejected() {
    let columns = vec![
        vec![0.0, 1.0, 2.0, 3.0],
        vec![0.0, 1.0, 2.0, 4.0],
        vec![0.0, 1.0, 0.0, 1.0],
    ];
    let clusters = Clusters::new(&[0, 1, 2, 3], 1).unwrap();
    let mut rng = Sampler::new(1);
    let mut options = Options {
        mtry: 1,
        min_node_size: 1,
        honesty: Honesty::Disabled,
        alpha: 0.05,
        imbalance_penalty: 0.0,
    };
    assert!(tree::train(
        Data {
            columns: &columns,
            outcome: 1,
            treatment: 1,
            weight: None
        },
        &mut rng,
        &clusters,
        &[0, 1],
        &options
    )
    .is_err());
    assert!(tree::train(
        Data {
            columns: &columns,
            outcome: 1,
            treatment: 2,
            weight: Some(1)
        },
        &mut rng,
        &clusters,
        &[0, 1],
        &options
    )
    .is_err());
    assert!(tree::train(
        Data {
            columns: &columns,
            outcome: 1,
            treatment: 2,
            weight: None
        },
        &mut rng,
        &clusters,
        &[],
        &options
    )
    .is_err());
    assert!(tree::train(
        Data {
            columns: &columns,
            outcome: 1,
            treatment: 2,
            weight: None
        },
        &mut rng,
        &clusters,
        &[4],
        &options
    )
    .is_err());
    options.honesty = Honesty::Enabled {
        fraction: f64::NAN,
        prune: true,
    };
    assert!(tree::train(
        Data {
            columns: &columns,
            outcome: 1,
            treatment: 2,
            weight: None
        },
        &mut rng,
        &clusters,
        &[0, 1],
        &options
    )
    .is_err());
}
