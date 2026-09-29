use hirmos_causal_core::grf::{
    forest::{Options, SeedMode},
    regression,
    sampling::Clusters,
    tree::{self, Honesty},
    tuning::{self, Parameter, SurfaceReadiness},
};
use serde_json::Value;
fn number(v: &Value) -> f64 {
    v.as_str()
        .map(|s| s.parse().unwrap())
        .or_else(|| v.as_f64())
        .unwrap()
}
fn vector(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| if v.is_null() { f64::NAN } else { number(v) })
        .collect()
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 2e-10 * b.abs().max(1.), "{a} != {b}");
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn seeded_candidates_and_mini_forests_match_r() {
    let fixture: Value =
        serde_json::from_str(include_str!("../oracle/grf/fixtures/tuning.json")).unwrap();
    for c in fixture["cases"].as_array().unwrap() {
        let x: Vec<_> = c["X"].as_array().unwrap().iter().map(vector).collect();
        let y = vector(&c["Y"]);
        let w = vector(&c["W"]);
        let n = y.len();
        let parameters: Vec<_> = c["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Parameter::from_name(v.as_str().unwrap()).unwrap())
            .collect();
        let seed = c["seed"].as_u64().unwrap() as u32;
        let options = Options {
            trees: 50,
            group_size: 1,
            sample_fraction: 0.5,
            seed,
            seed_mode: SeedMode::Indexed,
            batches: 1,
            tree: tree::Options {
                mtry: 4,
                min_node_size: 5,
                honesty: Honesty::Enabled {
                    fraction: 0.5,
                    prune: true,
                },
                alpha: 0.05,
                imbalance_penalty: 0.,
            },
        };
        let clusters = Clusters::new(&(0..n).collect::<Vec<_>>(), 1).unwrap();
        let candidates = tuning::candidates(n, 4, &parameters, 12, seed).unwrap();
        let mut cols = x.clone();
        cols.push(y.clone());
        cols.push(w);
        let weighted = !c["weights"].is_null();
        if weighted {
            cols.push(vector(&c["weights"]));
        }
        let mut regression_cols = x.clone();
        regression_cols.push(y.clone());
        if weighted {
            regression_cols.push(vector(&c["weights"]));
        }
        for candidate in &candidates {
            let i = candidate.index;
            for (j, (_, v)) in candidate.parameters.iter().enumerate() {
                close(*v, number(&c["mapped"][i][j]));
                close(candidate.draws[j], number(&c["draws"][i][j]));
            }
            let opts = candidate.forest_options(&options, 50).unwrap();
            if seed == 31 && i == 6 {
                let trace: Value =
                    serde_json::from_str(include_str!("../oracle/grf/fixtures/tuning-trace.json"))
                        .unwrap();
                let node = trace["trace"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|v| v["node"] == 9)
                    .unwrap();
                let rows: Vec<_> = node["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as usize)
                    .collect();
                let candidates: Vec<_> = node["candidates"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as usize)
                    .collect();
                let response =
                    hirmos_causal_core::grf::prediction::CausalData::new(&cols[4], &cols[5], &cols[6])
                        .unwrap()
                        .relabel(&rows)
                        .unwrap()
                        .unwrap();
                assert_eq!(response, vector(&node["responses"]));
                for values in [response, vector(&node["responses"])] {
                    let mut all = vec![0.; n];
                    for (&r, v) in rows.iter().zip(values) {
                        all[r] = v;
                    }
                    let selected = hirmos_causal_core::grf::splitting::best_split(
                        &cols,
                        &cols[5],
                        &cols[6],
                        &all,
                        &rows,
                        &candidates,
                        opts.tree.min_node_size,
                        opts.tree.alpha,
                        opts.tree.imbalance_penalty,
                    )
                    .unwrap()
                    .unwrap();
                    assert_eq!(
                        selected.variable,
                        node["variable"].as_u64().unwrap() as usize
                    );
                    assert_eq!(selected.value, number(&node["value"]));
                }
                let reference: Value =
                    serde_json::from_str(include_str!("../oracle/grf/fixtures/tuning-tree.json"))
                        .unwrap();
                let fit = hirmos_causal_core::grf::forest::train(
                    tree::Data {
                        columns: &cols,
                        outcome: 4,
                        treatment: 5,
                        weight: weighted.then_some(6),
                    },
                    &clusters,
                    &opts,
                )
                .unwrap();
                for (t, (tree, expected)) in fit
                    .trees
                    .iter()
                    .zip(reference["trees"].as_array().unwrap())
                    .enumerate()
                {
                    assert_eq!(
                        tree.root,
                        expected["root"].as_u64().unwrap() as usize,
                        "tree {t} root"
                    );
                    for (j, node) in tree.nodes.iter().enumerate() {
                        if let Some(split) = node.split {
                            assert_eq!(
                                split.variable,
                                expected["vars"][j].as_u64().unwrap() as usize,
                                "tree {t} node {j} var"
                            );
                            assert_eq!(
                                split.value,
                                number(&expected["values"][j]),
                                "tree {t} node {j} threshold"
                            );
                        }
                    }
                }
            }
            let error = tuning::causal_error(
                tree::Data {
                    columns: &cols,
                    outcome: 4,
                    treatment: 5,
                    weight: weighted.then_some(6),
                },
                &clusters,
                &opts,
                true,
            )
            .unwrap()
            .unwrap();
            assert!(
                (error - number(&c["errors"][i]["causal"])).abs() < 2e-10,
                "causal seed {seed} candidate {i}: {error} != {}",
                c["errors"][i]["causal"]
            );
            let fit =
                regression::train(&regression_cols, 4, weighted.then_some(5), &clusters, &opts)
                    .unwrap();
            let errors = fit.oob_errors(&regression_cols, &y).unwrap();
            for (row, error) in errors.iter().enumerate() {
                let expected = &c["errors"][i]["regression"][row];
                match error {
                    Some((a, b)) => {
                        close(*a, number(&expected["debiased.error"]));
                        close(*b, number(&expected["excess.error"]));
                    }
                    None => assert!(expected["debiased.error"].is_null()),
                }
            }
            let mean = tuning::regression_error(
                &regression_cols,
                4,
                weighted.then_some(5),
                &clusters,
                &opts,
            )
            .unwrap()
            .unwrap();
            let retained: Vec<_> = errors.iter().flatten().map(|v| v.0).collect();
            close(mean, retained.iter().sum::<f64>() / retained.len() as f64);
        }
        let defaults: Vec<_> = parameters
            .iter()
            .map(|p| {
                (
                    *p,
                    match p {
                        Parameter::SampleFraction => 0.5,
                        Parameter::Mtry => 4.,
                        Parameter::MinNodeSize => 5.,
                        Parameter::HonestyFraction => 0.5,
                        Parameter::HonestyPrune => 1.,
                        Parameter::Alpha => 0.05,
                        Parameter::ImbalancePenalty => 0.,
                    },
                )
            })
            .collect();
        let config = tuning::Config {
            parameters: parameters.clone(),
            mini_trees: 50,
            repetitions: 12,
            draws: 100,
        };
        for causal in [false, true] {
            let result = tuning::tune(n, 4, &options, &defaults, &config, |opts| {
                if causal {
                    tuning::causal_error(
                        tree::Data {
                            columns: &cols,
                            outcome: 4,
                            treatment: 5,
                            weight: weighted.then_some(6),
                        },
                        &clusters,
                        opts,
                        true,
                    )
                } else {
                    tuning::regression_error(
                        &regression_cols,
                        4,
                        weighted.then_some(5),
                        &clusters,
                        opts,
                    )
                }
            })
            .unwrap();
            let expected = &c[if causal {
                "tuned_causal"
            } else {
                "tuned_regression"
            }];
            match result.status {
                tuning::Status::Failed(_) => assert_eq!(expected["status"], "failure"),
                tuning::Status::Default { error } => {
                    assert_eq!(
                        expected["status"], "default",
                        "seed {seed}, causal {causal}"
                    );
                    close(error.unwrap(), number(&expected["error"]));
                }
                tuning::Status::Tuned {
                    candidate,
                    error,
                    surface,
                } => {
                    assert_eq!(expected["status"], "tuned", "seed {seed}, causal {causal}");
                    close(error, number(&expected["error"]));
                    for (j, (_, value)) in candidate.parameters.iter().enumerate() {
                        let name = c["parameters"][j].as_str().unwrap();
                        close(*value, number(&expected["params"][name]));
                    }
                    for (i, value) in surface.iter().enumerate() {
                        let target = number(&expected["grid"][i][0]);
                        assert!(
                            (value - target).abs() < 2e-5 * target.abs().max(1.),
                            "surface seed {seed}, causal {causal}, row {i}: {value} != {target}"
                        );
                    }
                }
            }
        }
    }
}
#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn tuning_boundaries() {
    assert_eq!(
        tuning::ordered_errors(3, &[(2, Some(3.)), (0, None), (1, Some(2.))]).unwrap(),
        vec![None, Some(2.), Some(3.)]
    );
    assert!(tuning::ordered_errors(2, &[(0, None), (0, None)]).is_err());
    assert!(tuning::ordered_errors(2, &[(0, None)]).is_err());
    assert!(tuning::ordered_errors(1, &[(1, None)]).is_err());
    assert!(Parameter::from_name("unknown").is_err());
    assert!(tuning::candidates(100, 4, &[Parameter::Mtry, Parameter::Mtry], 10, 7).is_err());
    assert!(tuning::candidates(0, 4, &[Parameter::Mtry], 10, 7).is_err());
    assert_eq!(
        tuning::surface_readiness(&[None; 12]).unwrap(),
        SurfaceReadiness::TooFewUsableForests
    );
    assert_eq!(
        tuning::surface_readiness(&[Some(1.); 12]).unwrap(),
        SurfaceReadiness::NearlyConstantErrors
    );
    let errors: Vec<_> = (0..12).map(|i| Some(i as f64)).collect();
    assert_eq!(
        tuning::surface_readiness(&errors).unwrap(),
        SurfaceReadiness::Ready
    );
    assert!(tuning::surface_readiness(&[Some(f64::INFINITY); 12]).is_err());
}
