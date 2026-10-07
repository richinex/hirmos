use hirmos_causal_core::causal_effects::*;
use serde_json::Value;

fn nodes(value: &Value) -> Vec<Node> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            (
                v[0].as_u64().unwrap() as usize,
                v[1].as_i64().unwrap() as i32,
            )
        })
        .collect()
}

#[test]
fn optimality_matches_tigramite_conditions_and_sets() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../oracle/fixtures/causal_effects_optimality.json"
    ))
    .unwrap();
    assert_eq!(fixture["tigramite"], "5.2.10.1");
    let mut unestablished = 0;
    let mut unidentified = 0;
    let mut condition_failures = [0, 0];
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 673);
    for c in fixture["cases"].as_array().unwrap() {
        let mut graph = StationaryGraph::new(
            c["n"].as_u64().unwrap() as usize,
            c["stat_lag"].as_u64().unwrap() as usize,
        );
        for edge in c["edges"].as_array().unwrap() {
            let (a, b, l) = (
                edge[0].as_u64().unwrap() as usize,
                edge[1].as_u64().unwrap() as usize,
                edge[2].as_u64().unwrap() as usize,
            );
            graph.set(a, b, l, mark("-->"));
            if l == 0 {
                graph.set(b, a, 0, mark("<--"));
            }
        }
        let ce = CausalEffects::new(
            graph,
            &nodes(&c["X"]),
            &nodes(&c["Y"]),
            &nodes(&c["S"]),
            &nodes(&c["hidden"]),
        );
        let actual = ce.check_optimality();
        if c["result"]["kind"] == "notIdentifiable" {
            // Deliberate departure: upstream unpacks False and raises TypeError.
            assert_eq!(c["upstream_optimality"], "TypeError");
            assert_eq!(actual, Optimality::NotIdentifiable, "{}", c["name"]);
            unidentified += 1;
        } else {
            let flags = &c["result"]["conditions"];
            condition_failures[0] += usize::from(!flags[1].as_bool().unwrap());
            condition_failures[1] += usize::from(!flags[2].as_bool().unwrap());
            assert_eq!(
                actual,
                Optimality::Checked {
                    unique_set: flags[0].as_bool().unwrap(),
                    condition_i: flags[1].as_bool().unwrap(),
                    condition_ii: flags[2].as_bool().unwrap()
                },
                "{}",
                c["name"]
            );
            assert_eq!(
                actual.established(),
                c["result"]["established"].as_bool().unwrap(),
                "{}",
                c["name"]
            );
            assert_eq!(
                ce.get_optimal_set(),
                Some(nodes(&c["result"]["oset"])),
                "{}",
                c["name"]
            );
            unestablished += usize::from(!actual.established());
        }
    }
    assert!(unestablished > 0);
    assert!(unidentified > 0);
    assert!(condition_failures.iter().all(|&count| count > 0));
}

