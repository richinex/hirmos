use hirmos_causal_core::do_calculus::Admg;
use serde_json::Value;
use std::collections::BTreeSet;

fn strings(value: &Value) -> Vec<String> {
    serde_json::from_value(value.clone()).unwrap()
}

fn pairs(value: &Value) -> Vec<(String, String)> {
    serde_json::from_value::<Vec<Vec<String>>>(value.clone())
        .unwrap()
        .into_iter()
        .map(|pair| (pair[0].clone(), pair[1].clone()))
        .collect()
}

#[test]
fn m_separation_matches_d_separation_with_explicit_latents() {
    let fixture: Value = serde_json::from_str(include_str!("../oracle/fixtures/m_separation.json")).unwrap();
    let mut wrong = Vec::new();
    for (index, case) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let graph = Admg::new(strings(&case["nodes"]), pairs(&case["directed"]), pairs(&case["bidirected"])).unwrap();
        let conditions: BTreeSet<String> = strings(&case["conditions"]).into_iter().collect();
        let left = case["left"].as_str().unwrap().to_string();
        let right = case["right"].as_str().unwrap().to_string();
        if graph.are_d_separated(&left, &right, &conditions) != case["separated"].as_bool().unwrap() {
            wrong.push(index);
        }
    }
    assert!(wrong.is_empty(), "{} of 400 cases disagree: {:?}", wrong.len(), &wrong[..wrong.len().min(10)]);
}
