// Parity for pgmpy's discrete Bayesian-network operations, with a corrected input adapter.
use hirmos_causal_core::discrete_bn::*;
use serde_json::Value;
use std::collections::HashMap;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/discrete_bn.json")).unwrap()
}

fn strings(v: &Value) -> Vec<String> {
    serde_json::from_value(v.clone()).unwrap()
}

fn budget(value: usize) -> StateBudget {
    StateBudget::try_from(value).unwrap()
}

fn build(root: &Value) -> (DiscreteBn, HashMap<String, Vec<String>>) {
    let raw = root["raw"].as_object().unwrap();
    let bins = root["bins"].as_u64().unwrap() as usize;
    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    for (col, vals) in raw {
        let x: Vec<f64> = serde_json::from_value(vals.clone()).unwrap();
        data.insert(
            col.clone(),
            discretize_for_discrete_bn(&x, budget(bins))
                .expect("fixture columns are discretizable")
                .labels,
        );
    }
    let edges: Vec<Vec<String>> = serde_json::from_value(root["edges"].clone()).unwrap();
    let edges: Vec<(String, String)> = edges
        .into_iter()
        .map(|e| (e[0].clone(), e[1].clone()))
        .collect();
    let dag = Dag::new(&edges, &[]);
    (DiscreteBn::fit(dag, &data, 5.0), data)
}

#[test]
fn discretisation_matches() {
    let root = fixture();
    let bins = root["bins"].as_u64().unwrap() as usize;
    for (col, want) in root["codes"].as_object().unwrap() {
        let x: Vec<f64> = serde_json::from_value(root["raw"][col].clone()).unwrap();
        let got = discretize_for_discrete_bn(&x, budget(bins))
            .expect("fixture column is discretizable")
            .labels;
        assert_eq!(got, strings(want), "qcut codes for {col}");
    }
    println!(
        "qcut codes identical for all {} columns",
        root["codes"].as_object().unwrap().len()
    );
}

#[test]
fn discretisation_preserves_states_and_handles_missing_values() {
    let root = fixture();
    let bins = root["bins"].as_u64().unwrap() as usize;
    for case in root["discretization_cases"].as_array().unwrap() {
        let raw: Vec<Option<f64>> = serde_json::from_value(case["raw"].clone()).unwrap();
        let raw: Vec<f64> = raw
            .into_iter()
            .map(|value| value.unwrap_or(f64::NAN))
            .collect();
        let got = discretize_for_discrete_bn(&raw, budget(bins));
        if case["labels"].is_null() {
            assert!(got.is_err(), "{} should be refused", case["name"]);
            continue;
        }
        let got = got.expect("case is discretizable");
        assert_eq!(
            got.labels,
            strings(&case["labels"]),
            "{} labels",
            case["name"]
        );
        let expected_means: std::collections::BTreeMap<String, f64> =
            serde_json::from_value(case["means"].clone()).unwrap();
        assert_eq!(
            got.means.keys().collect::<Vec<_>>(),
            expected_means.keys().collect::<Vec<_>>()
        );
        for (state, expected) in expected_means {
            assert!(
                (got.means[&state] - expected).abs() < 1e-15,
                "{} mean for state {state}",
                case["name"]
            );
        }
    }

    let imbalanced_binary = [0.0, 1.0, 1.0, 1.0, 1.0];
    let got = discretize_for_discrete_bn(&imbalanced_binary, budget(bins)).unwrap();
    assert_eq!(got.labels, ["0", "1", "1", "1", "1"]);
    assert_eq!(got.means.len(), 2);
    let pgmpy_states: Vec<f64> =
        serde_json::from_value(root["pgmpy_observed_binary_states"].clone()).unwrap();
    assert_eq!(pgmpy_states, vec![got.means["0"], got.means["1"]]);

    let tied_continuous = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0];
    assert_eq!(
        discretize_for_discrete_bn(&tied_continuous, budget(bins)),
        Err(DiscreteStateProblem::QuantileCollapse {
            distinct_values: 4,
            requested_states: 3,
            populated_states: 1,
        })
    );
    assert_eq!(
        discretize_for_discrete_bn(&[4.0, 4.0], budget(bins)),
        Err(DiscreteStateProblem::SingleObservedState {
            value: 4.0,
            observations: 2,
        })
    );
    assert_eq!(
        discretize_for_discrete_bn(&[f64::NAN], budget(bins)),
        Err(DiscreteStateProblem::NoFiniteObservations { observations: 1 })
    );
    assert_eq!(
        StateBudget::try_from(1),
        Err(StateBudgetError {
            requested: 1,
            minimum: 2,
        })
    );
}

#[test]
fn bdeu_cpds_match() {
    let root = fixture();
    let (bn, _) = build(&root);
    let mut worst = 0.0f64;
    for (node, want) in root["cpds"].as_object().unwrap() {
        let cpd = bn
            .cpds
            .iter()
            .find(|c| c.variable == *node)
            .expect("cpd exists");
        assert_eq!(
            cpd.parents,
            strings(&want["parents"]),
            "{node}: parent order"
        );
        let values: Vec<Vec<f64>> = serde_json::from_value(want["values"].clone()).unwrap();
        let n_cols = values[0].len();
        assert_eq!(
            cpd.values.len(),
            values.len() * n_cols,
            "{node}: table shape"
        );
        for (s, row) in values.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                worst = worst.max((cpd.values[s * n_cols + c] - v).abs());
            }
        }
    }
    println!("BDeu conditional probability tables maxdev {worst:.3e}");
    assert!(worst <= 1e-14, "CPD deviation {worst}");
}

#[test]
fn variable_elimination_matches() {
    let root = fixture();
    let (bn, _) = build(&root);
    let mut worst = 0.0f64;
    for q in root["ve_queries"].as_array().unwrap() {
        let evidence: HashMap<String, String> =
            serde_json::from_value(q["evidence"].clone()).unwrap();
        let dist = bn.query(&["outcome".to_string()], &evidence);
        let states = strings(&q["states"]);
        let values: Vec<f64> = serde_json::from_value(q["values"].clone()).unwrap();
        for (s, &v) in states.iter().zip(&values) {
            worst = worst.max((dist[s] - v).abs());
        }
        println!("  evidence {:?}: maxdev so far {worst:.3e}", q["evidence"]);
    }
    assert!(worst <= 1e-12, "variable elimination deviation {worst}");
}

#[test]
fn minimal_adjustment_set_matches() {
    let root = fixture();
    let (bn, _) = build(&root);
    let got = bn.minimal_adjustment_set("treatment", "outcome");
    let want: Option<Vec<String>> =
        serde_json::from_value(root["minimal_adjustment_set"].clone()).unwrap();
    match (got, want) {
        (Some(g), Some(w)) => {
            let g: Vec<String> = g.into_iter().collect();
            println!("minimal adjustment set {g:?} against {w:?}");
            assert_eq!(g, w, "minimal adjustment set");
        }
        (None, None) => println!("no adjustment set, as pgmpy reports"),
        (g, w) => panic!("adjustment set differs: {g:?} against {w:?}"),
    }
}

#[test]
fn interventional_query_matches() {
    let root = fixture();
    let (bn, _) = build(&root);
    let means: HashMap<String, HashMap<String, f64>> =
        serde_json::from_value(root["means"].clone()).unwrap();
    let om = &means["outcome"];
    let expectation =
        |dist: &HashMap<String, f64>| -> f64 { dist.iter().map(|(s, p)| p * om[s]).sum() };

    let mut worst = 0.0f64;
    for (label, curve) in [("do", "do_curve"), ("observational", "ob_curve")] {
        for entry in root[curve].as_array().unwrap() {
            let state = entry["state"].as_str().unwrap();
            let dist = if label == "do" {
                bn.do_query("outcome", "treatment", state)
            } else {
                let mut ev = HashMap::new();
                ev.insert("treatment".to_string(), state.to_string());
                bn.query(&["outcome".to_string()], &ev)
            };
            let states = strings(&entry["states"]);
            let values: Vec<f64> = serde_json::from_value(entry["values"].clone()).unwrap();
            for (s, &v) in states.iter().zip(&values) {
                worst = worst.max((dist[s] - v).abs());
            }
            println!(
                "  {label}(treatment={state}): E[outcome] = {:.6}",
                expectation(&dist)
            );
        }
    }
    println!("do and observational distributions maxdev {worst:.3e}");
    assert!(worst <= 1e-12, "do-query deviation {worst}");

    // The per-unit ATE 805 prints.
    let tm = &means["treatment"];
    let tstates: Vec<String> = {
        let mut s: Vec<String> = tm.keys().cloned().collect();
        s.sort_by_key(|k| k.parse::<i32>().unwrap());
        s
    };
    let (lo, hi) = (tstates.first().unwrap(), tstates.last().unwrap());
    let span = tm[hi] - tm[lo];
    let do_ate = (expectation(&bn.do_query("outcome", "treatment", hi))
        - expectation(&bn.do_query("outcome", "treatment", lo)))
        / span;
    let obs = |s: &str| {
        let mut ev = HashMap::new();
        ev.insert("treatment".to_string(), s.to_string());
        expectation(&bn.query(&["outcome".to_string()], &ev))
    };
    let ob_ate = (obs(hi) - obs(lo)) / span;
    println!(
        "do ATE {do_ate:+.6} against {:+.6}, observational {ob_ate:+.6} against {:+.6}",
        root["do_ate"].as_f64().unwrap(),
        root["ob_ate"].as_f64().unwrap()
    );
    assert!(
        (do_ate - root["do_ate"].as_f64().unwrap()).abs() <= 1e-12,
        "do ATE"
    );
    assert!(
        (ob_ate - root["ob_ate"].as_f64().unwrap()).abs() <= 1e-12,
        "observational ATE"
    );
}
