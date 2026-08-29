// Parity for backdoor identification with unobserved nodes (DoWhy).
use hirmos_causal_core::{backdoor_adjustment, Dag};
use serde_json::Value;

#[test]
fn identification_respects_unobserved_nodes() {
    let root: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/backdoor_identification.json")).unwrap();
    for fx in root["identifications"].as_array().unwrap() {
        let name = fx["name"].as_str().unwrap();
        let nodes: Vec<String> = serde_json::from_value(fx["nodes"].clone()).unwrap();
        let observed: Vec<String> = serde_json::from_value(fx["observed"].clone()).unwrap();
        let idx = |s: &str| nodes.iter().position(|n| n == s).unwrap();
        let edges: Vec<(String, String)> = serde_json::from_value(fx["edges"].clone()).unwrap();
        let edge_idx: Vec<(usize, usize)> = edges.iter().map(|(a, b)| (idx(a), idx(b))).collect();
        let unobserved: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| !observed.contains(n))
            .map(|(i, _)| i)
            .collect();
        let dag = Dag::new(nodes.len(), &edge_idx);
        let got = backdoor_adjustment(&dag, idx("T"), idx("Y"), &unobserved);
        let identified = fx["identified"].as_bool().unwrap();
        assert_eq!(got.is_some(), identified, "{name} identified");
        if let Some(set) = got {
            let mut names: Vec<String> = set.iter().map(|&v| nodes[v].clone()).collect();
            names.sort();
            let want: Vec<String> = serde_json::from_value(fx["backdoor"].clone()).unwrap();
            assert_eq!(names, want, "{name} backdoor set");
        }
        println!("{name}: identification matches");
    }
}
