use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::reinforcement::where_to_intervene::{InterventionSet, OutcomeGraph};
use serde_json::Value;
use std::collections::BTreeSet;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../oracle/fixtures/reinforcement_where_to_intervene.json")).unwrap()
}

fn names(value: &Value) -> InterventionSet {
    serde_json::from_value::<Vec<String>>(value.clone()).unwrap().into_iter().collect()
}

fn families(value: &Value) -> BTreeSet<InterventionSet> {
    value.as_array().unwrap().iter().map(names).collect()
}

fn pairs(value: &Value) -> Vec<(String, String)> {
    serde_json::from_value::<Vec<Vec<String>>>(value.clone())
        .unwrap()
        .into_iter()
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

fn outcome_graph(case: &Value) -> OutcomeGraph {
    let graph = Admg::new(names(&case["nodes"]), pairs(&case["directed"]), pairs(&case["bidirected"])).unwrap();
    OutcomeGraph::new(graph, case["outcome"].as_str().unwrap()).unwrap()
}

fn set(members: &[&str]) -> InterventionSet {
    members.iter().map(|name| name.to_string()).collect()
}

#[test]
fn every_case_matches_the_reference_sets() {
    let fixture = fixture();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 201);
    for case in cases {
        let label = case["label"].as_str().unwrap();
        let problem = outcome_graph(case);
        assert_eq!(problem.minimal_intervention_sets(), families(&case["mis"]), "MISs for {label}");
        assert_eq!(problem.possibly_optimal_intervention_sets(), families(&case["pomis"]), "POMISs for {label}");
        assert_eq!(problem.possibly_optimal_by_enumeration(), families(&case["pomis_bruteforce"]), "enumeration for {label}");
        assert_eq!(problem.minimal_unobserved_confounders_territory(), names(&case["muct"]), "MUCT for {label}");
        assert_eq!(problem.interventional_border(), names(&case["ib"]), "IB for {label}");
    }
}

#[test]
fn minimal_do_drops_interventions_that_cannot_reach_the_outcome() {
    let graph = Admg::new(
        ["X1", "X2", "Y"].map(String::from),
        [("X1", "X2"), ("X2", "Y")].map(|(a, b)| (a.to_string(), b.to_string())),
        [("X2", "Y")].map(|(a, b)| (a.to_string(), b.to_string())),
    )
    .unwrap();
    let problem = OutcomeGraph::new(graph, "Y").unwrap();
    assert_eq!(problem.minimal_do(&set(&["X1", "X2"])), set(&["X2"]));
}
