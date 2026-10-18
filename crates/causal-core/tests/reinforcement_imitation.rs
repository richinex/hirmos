use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::reinforcement::imitation::{non_descendant_inputs, pi_backdoor, sequential_pi_backdoor, ConditioningSets};
use serde_json::Value;
use std::collections::BTreeSet;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_imitation.json")).unwrap()
}

fn strings(value: &Value) -> Vec<String> {
    serde_json::from_value(value.clone()).unwrap()
}

fn graph(case: &Value) -> Admg {
    let pairs = |key: &str| -> Vec<(String, String)> {
        serde_json::from_value::<Vec<Vec<String>>>(case[key].clone()).unwrap().into_iter().map(|p| (p[0].clone(), p[1].clone())).collect()
    };
    Admg::new(strings(&case["nodes"]), pairs("directed"), pairs("bidirected")).unwrap()
}

fn starting_with(nodes: &[String], letters: &str) -> BTreeSet<String> {
    nodes.iter().filter(|v| letters.contains(&v[..1])).cloned().collect()
}

fn recorded(value: &Value) -> Option<ConditioningSets> {
    (!value.is_null()).then(|| value.as_object().unwrap().iter().map(|(k, v)| (k.clone(), strings(v).into_iter().collect())).collect())
}

#[test]
fn sequential_sets_match_the_reference_under_both_orderings() {
    let fixture = fixture();
    for (index, case) in fixture["sequential"].as_array().unwrap().iter().enumerate() {
        let nodes = strings(&case["nodes"]);
        let g = graph(case);
        let (actions, observed, rewards) = (starting_with(&nodes, "X"), starting_with(&nodes, "ZX"), starting_with(&nodes, "Y"));
        let outcome = "Y".to_string();
        for (ordering, expected) in [(strings(&case["default_ordering"]), &case["default"]), (nodes.clone(), &case["custom"])] {
            let found = sequential_pi_backdoor(&g, &actions, &outcome, &observed, &rewards, &ordering).unwrap();
            assert_eq!(found, recorded(expected), "case {index}, ordering {ordering:?}");
        }
    }
}

#[test]
fn single_step_sets_are_valid_and_as_small_as_the_reference() {
    let fixture = fixture();
    for (index, case) in fixture["single"].as_array().unwrap().iter().enumerate() {
        let nodes = strings(&case["nodes"]);
        let g = graph(case);
        let (action, outcome) = ("X0".to_string(), "Y".to_string());
        let candidates = non_descendant_inputs(&g, &action, &outcome, &starting_with(&nodes, "ZX"));
        assert_eq!(candidates, strings(&case["candidates"]).into_iter().collect(), "case {index}");
        let valid: Vec<BTreeSet<String>> = case["valid"].as_array().unwrap().iter().map(|z| strings(z).into_iter().collect()).collect();
        match (pi_backdoor(&g, &action, &outcome, &candidates), case["found"].is_null()) {
            (None, true) => {}
            (Some(z), false) => {
                assert!(valid.contains(&z), "case {index}: {z:?} is not a valid set");
                assert_eq!(z.len(), strings(&case["found"]).len(), "case {index}");
            }
            (found, _) => panic!("case {index}: found {found:?}, reference {}", case["found"]),
        }
    }
}

#[test]
fn the_notebook_graph_needs_the_first_state_and_action_only_for_the_second_action() {
    let nodes: Vec<String> = ["Z0", "X0", "Z1", "X1", "Y"].map(String::from).to_vec();
    let edge = |a: &str, b: &str| (a.to_string(), b.to_string());
    let g = Admg::new(
        nodes.clone(),
        [edge("Z1", "X1"), edge("X0", "Y"), edge("X1", "Y"), edge("Z1", "Y")],
        [edge("Z0", "Z1"), edge("Z0", "Y"), edge("Z1", "X1")],
    )
    .unwrap();
    let ordering: Vec<String> = ["Y", "Z0", "X0", "Z1", "X1"].map(String::from).to_vec();
    let sets = sequential_pi_backdoor(&g, &starting_with(&nodes, "X"), &"Y".to_string(), &starting_with(&nodes, "ZX"), &starting_with(&nodes, "Y"), &ordering)
        .unwrap()
        .unwrap();
    assert_eq!(sets["X0"], BTreeSet::new());
    assert_eq!(sets["X1"], ["X0", "Z1"].map(String::from).into_iter().collect());
}
