use hirmos_causal_core::survival::forest::{
    train, train_with_sampling, Data, Forest, Node, Replacement, Sampling, Settings, Split,
    SplitRule,
};
use serde_json::Value;

fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12 + 1e-12 * b.abs(), "{a} != {b}");
}

fn element(value: &Value, index: usize) -> &Value {
    match value.as_array() {
        Some(values) => &values[index],
        None => {
            assert_eq!(index, 0);
            value
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn invalid_forest_inputs_are_rejected() {
    use hirmos_causal_core::survival::forest::Error;
    assert!(matches!(
        Data::new(vec![vec![1.5]], vec![1.0], vec![true], vec![false]),
        Err(Error::InvalidCategory)
    ));
    assert!(matches!(
        Data::new(vec![vec![f64::NAN]], vec![1.0], vec![true], vec![true]),
        Err(Error::InvalidRow)
    ));
    let data = Data::new(vec![vec![1.0]; 4], vec![1.0; 4], vec![false; 4], vec![true]).unwrap();
    let mut settings = Settings {
        trees: 1,
        mtry: 1,
        seed: 1,
        min_node_size: 3,
        min_bucket: 3,
        max_depth: None,
        rule: SplitRule::LogRank,
    };
    assert!(matches!(train(&data, &settings), Err(Error::InvalidTimes)));
    settings.mtry = 2;
    assert!(matches!(
        train(&data, &settings),
        Err(Error::InvalidSettings)
    ));
    assert!(matches!(
        Forest::new(
            vec![1.0],
            1,
            vec![vec![Node::Branch {
                split: Split::Ordered {
                    column: 0,
                    threshold: 1.0
                },
                left: 0,
                right: 1
            }]]
        ),
        Err(Error::InvalidTree)
    ));
    let forest = Forest::new(
        vec![1.0],
        1,
        vec![vec![Node::Terminal { hazard: vec![0.0] }]],
    )
    .unwrap();
    assert!(forest.predict_out_of_bag(&[1.0], &[1]).unwrap().is_none());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ranger_training_and_predictions_match_reference() {
    let fixture: Value = serde_json::from_str(
        &std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/oracle/fixtures/ranger.json"
        ))
        .unwrap(),
    )
    .unwrap();
    verify_ranger(&fixture);
}

pub fn verify_ranger(fixture: &Value) {
    for case in fixture["cases"].as_array().unwrap() {
        let expected = &case["expected"];
        let columns: Vec<String> = case["columns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .filter(|name| name != "time" && name != "status")
            .collect();
        let trees = (0..case["trees"].as_u64().unwrap() as usize)
            .map(|tree| {
                let children = &expected["child_nodes"][tree];
                (0..children[0].as_array().map_or(1, Vec::len))
                    .map(|node| {
                        let left = element(&children[0], node).as_u64().unwrap() as usize;
                        let right = element(&children[1], node).as_u64().unwrap() as usize;
                        if left == 0 && right == 0 {
                            return Node::Terminal {
                                hazard: serde_json::from_value(
                                    expected["terminal_hazards"][tree][node].clone(),
                                )
                                .unwrap(),
                            };
                        }
                        let column =
                            expected["split_variables"][tree][node].as_u64().unwrap() as usize;
                        let value = expected["split_values"][tree][node].as_f64().unwrap();
                        let split = if expected["ordered"][column].as_bool().unwrap() {
                            Split::Ordered {
                                column,
                                threshold: value,
                            }
                        } else {
                            Split::Categorical {
                                column,
                                right_levels: value as u64,
                            }
                        };
                        Node::Branch { split, left, right }
                    })
                    .collect()
            })
            .collect();
        let forest = Forest::new(
            serde_json::from_value(expected["death_times"].clone()).unwrap(),
            columns.len(),
            trees,
        )
        .unwrap();
        let n = case["data"][&columns[0]]["values"]
            .as_array()
            .unwrap()
            .len();
        let rows: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                columns
                    .iter()
                    .map(|name| case["data"][name]["values"][i].as_f64().unwrap())
                    .collect()
            })
            .collect();
        let data = Data::new(
            rows,
            serde_json::from_value(case["data"]["time"]["values"].clone()).unwrap(),
            case["data"]["status"]["values"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() == 1)
                .collect(),
            serde_json::from_value(expected["ordered"].clone()).unwrap(),
        )
        .unwrap();
        let sampling = &case["sampling"];
        let replacement = if sampling["replacement"] == true {
            Replacement::With
        } else {
            Replacement::Without
        };
        let sampling = match sampling["kind"].as_str().unwrap() {
            "uniform" => Sampling::Uniform {
                fraction: sampling["fraction"].as_f64().unwrap(),
                replacement,
            },
            "weighted" => Sampling::Weighted {
                fraction: sampling["fraction"].as_f64().unwrap(),
                replacement,
                weights: serde_json::from_value(sampling["weights"].clone()).unwrap(),
            },
            "manual" => Sampling::Manual {
                counts: serde_json::from_value(sampling["counts"].clone()).unwrap(),
            },
            _ => panic!("unknown oracle sampling"),
        };
        let trained = train_with_sampling(
            &data,
            &Settings {
                trees: case["trees"].as_u64().unwrap() as usize,
                mtry: case["mtry"].as_u64().unwrap() as usize,
                seed: case["seed"].as_u64().unwrap() as u32,
                min_node_size: case["min_node_size"].as_u64().unwrap() as usize,
                min_bucket: case["min_bucket"].as_u64().unwrap() as usize,
                max_depth: case["max_depth"].as_u64().map(|v| v as usize),
                rule: match case["splitrule"].as_str().unwrap() {
                    "logrank" => SplitRule::LogRank,
                    "auc" => SplitRule::Auc,
                    "auc_ignore_ties" => SplitRule::AucIgnoreTies,
                    "maxstat" => SplitRule::MaxStat { alpha: 0.5 },
                    "extratrees" => SplitRule::ExtraTrees {
                        candidates: case["candidates"].as_u64().unwrap() as usize,
                    },
                    _ => panic!("unknown oracle split rule"),
                },
            },
            sampling,
        )
        .unwrap();
        let expected_inbag: Vec<Vec<usize>> =
            serde_json::from_value(expected["inbag"].clone()).unwrap();
        assert_eq!(trained.inbag, expected_inbag, "{} inbag", case["name"]);
        for (i, (actual, oracle)) in trained
            .forest
            .trees()
            .iter()
            .zip(forest.trees())
            .enumerate()
        {
            assert_eq!(actual.len(), oracle.len(), "{} tree {i}", case["name"]);
            for (j, (actual, oracle)) in actual.iter().zip(oracle).enumerate() {
                close(
                    trained.split_statistics[i][j],
                    element(&expected["split_statistics"][i], j)
                        .as_f64()
                        .unwrap(),
                );
                assert_eq!(
                    trained.node_sample_counts[i][j],
                    element(&expected["node_counts"][i], j).as_u64().unwrap() as usize
                );
                match (actual, oracle) {
                    (Node::Terminal { hazard: a }, Node::Terminal { hazard: b }) => {
                        for (&a, &b) in a.iter().zip(b) {
                            close(a, b);
                        }
                    }
                    (
                        Node::Branch {
                            split: a,
                            left: al,
                            right: ar,
                        },
                        Node::Branch {
                            split: b,
                            left: bl,
                            right: br,
                        },
                    ) => {
                        assert_eq!((al, ar), (bl, br), "{} tree {i} node {j}", case["name"]);
                        match (a, b) {
                            (
                                Split::Ordered {
                                    column: ac,
                                    threshold: av,
                                },
                                Split::Ordered {
                                    column: bc,
                                    threshold: bv,
                                },
                            ) => {
                                assert_eq!(ac, bc, "{} tree {i} node {j}", case["name"]);
                                close(*av, *bv);
                            }
                            (
                                Split::Categorical {
                                    column: ac,
                                    right_levels: av,
                                },
                                Split::Categorical {
                                    column: bc,
                                    right_levels: bv,
                                },
                            ) => {
                                assert_eq!((ac, av), (bc, bv), "{} tree {i} node {j}", case["name"])
                            }
                            _ => panic!(
                                "split kind {} tree {i} node {j}: {a:?} vs {b:?}",
                                case["name"]
                            ),
                        }
                    }
                    _ => panic!("node kind {} tree {i} node {j}", case["name"]),
                }
            }
        }
        close(
            trained.prediction_error.unwrap(),
            expected["prediction_error"].as_f64().unwrap(),
        );
        for (i, value) in trained.permutation_importance.iter().enumerate() {
            close(value.unwrap(), expected["importance"][i].as_f64().unwrap());
        }
        for i in 0..n {
            let row: Vec<f64> = columns
                .iter()
                .map(|name| case["data"][name]["values"][i].as_f64().unwrap())
                .collect();
            let result = forest.predict(&row).unwrap();
            for (tree, node) in result.terminal_nodes.iter().enumerate() {
                assert_eq!(
                    *node,
                    expected["terminal_nodes"][i][tree].as_u64().unwrap() as usize
                );
            }
            for j in 0..result.hazard.len() {
                close(result.hazard[j], expected["hazard"][i][j].as_f64().unwrap());
                close(
                    result.survival[j],
                    expected["survival"][i][j].as_f64().unwrap(),
                );
            }
            let inbag: Vec<usize> = expected["inbag"]
                .as_array()
                .unwrap()
                .iter()
                .map(|tree| tree[i].as_u64().unwrap() as usize)
                .collect();
            if let Some(result) = forest.predict_out_of_bag(&row, &inbag).unwrap() {
                for j in 0..result.hazard.len() {
                    close(
                        result.hazard[j],
                        expected["out_of_bag_hazard"][i][j].as_f64().unwrap(),
                    );
                    close(
                        result.survival[j],
                        expected["out_of_bag_survival"][i][j].as_f64().unwrap(),
                    );
                }
            }
        }
    }
}
