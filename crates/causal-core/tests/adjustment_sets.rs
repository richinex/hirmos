//! Parity with the adjustment sets generated from Dagitty's bundled example DAGs.

use hirmos_causal_core::{dagitty_adjustment_sets, AdjustmentSetAnalysis, Dag};
use serde_json::Value;

fn sorted_sets(mut sets: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for set in &mut sets {
        set.sort();
    }
    sets.sort();
    sets
}

#[test]
fn minimal_and_canonical_sets_match_dagitty_examples() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/dagitty/examples.json"
    ))
    .unwrap();
    let cases = fixtures.as_array().unwrap();
    assert!(
        cases.len() >= 10,
        "expected the full Dagitty example corpus"
    );

    for fixture in cases {
        let label = fixture["label"].as_str().unwrap();
        let nodes = fixture["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|node| node["name"].as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let index = |name: &str| nodes.iter().position(|node| node == name).unwrap();
        let edges = fixture["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|edge| {
                (
                    index(edge["from"].as_str().unwrap()),
                    index(edge["to"].as_str().unwrap()),
                )
            })
            .collect::<Vec<_>>();
        let unobserved = fixture["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(position, node)| {
                (node["kind"].as_str() == Some("latent")).then_some(position)
            })
            .collect::<Vec<_>>();
        let expected_minimal = fixture["expected"]["msas"]
            .as_array()
            .unwrap()
            .iter()
            .map(|set| {
                set.as_array()
                    .unwrap()
                    .iter()
                    .map(|name| name.as_str().unwrap().to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let expected_canonical = fixture["expected"]["canonical"]
            .as_array()
            .unwrap()
            .iter()
            .map(|name| name.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();

        let dag = Dag::new(nodes.len(), &edges);
        let result = dagitty_adjustment_sets(
            &dag,
            index(fixture["exposure"].as_str().unwrap()),
            index(fixture["outcome"].as_str().unwrap()),
            &unobserved,
            2_048,
        )
        .unwrap();
        match result {
            AdjustmentSetAnalysis::NotIdentified => {
                assert!(
                    expected_minimal.is_empty(),
                    "{label}: expected minimal sets"
                );
            }
            AdjustmentSetAnalysis::Identified {
                canonical,
                minimal,
                truncated,
            } => {
                assert!(!truncated, "{label}: result budget unexpectedly exhausted");
                let canonical_names = canonical
                    .iter()
                    .map(|&node| nodes[node].clone())
                    .collect::<Vec<_>>();
                let minimal_names = minimal
                    .iter()
                    .map(|set| set.iter().map(|&node| nodes[node].clone()).collect())
                    .collect::<Vec<Vec<String>>>();
                assert_eq!(
                    sorted_sets(vec![canonical_names]),
                    sorted_sets(vec![expected_canonical]),
                    "{label}: canonical set"
                );
                assert_eq!(
                    sorted_sets(minimal_names),
                    sorted_sets(expected_minimal),
                    "{label}: minimal sets"
                );
            }
        }
    }
}
