use super::*;
use serde_json::Value;

fn links(value: &Value) -> LinkAssumptions {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|parents| {
            parents
                .as_array()
                .unwrap()
                .iter()
                .map(|entry| {
                    (
                        (
                            entry[0].as_u64().unwrap() as usize,
                            entry[1].as_i64().unwrap() as i32,
                        ),
                        entry[2].as_str().unwrap().as_bytes().try_into().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

#[test]
fn context_constraints_match_vendored_tigramite() {
    let cases: Value = serde_json::from_str(include_str!(
        "../oracle/fixtures/jpcmciplus_constraints.json"
    ))
    .unwrap();
    assert_eq!(cases.as_array().unwrap().len(), 250);
    for case in cases.as_array().unwrap() {
        let classes: Vec<NodeClass> = case["classes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| match c.as_str().unwrap() {
                "system" => NodeClass::System,
                "time_context" => NodeClass::TimeContext,
                "space_context" => NodeClass::SpaceContext,
                "time_dummy" => NodeClass::TimeDummy,
                "space_dummy" => NodeClass::SpaceDummy,
                other => panic!("unknown class {other}"),
            })
            .collect();
        let contexts: Vec<Vec<(usize, i32)>> =
            serde_json::from_value(case["contexts"].clone()).unwrap();
        let dummies: Vec<Vec<(usize, i32)>> =
            serde_json::from_value(case["dummies"].clone()).unwrap();
        let prepared = prepare_context_links(&classes, &links(&case["original"]));
        assert_eq!(prepared, links(&case["prepared"]), "{classes:?}");
        assert_eq!(
            without_dummy_links(&classes, &prepared),
            links(&case["without_dummy"])
        );
        assert_eq!(
            with_context_parents(&classes, &prepared, &contexts),
            links(&case["with_contexts"])
        );
        assert_eq!(
            system_links(&classes, &prepared, &contexts, &dummies),
            links(&case["system"])
        );
        let nodes = links(&case["original"])
            .into_iter()
            .enumerate()
            .map(|(i, parents)| NodeInput {
                class: classes[i],
                parents: parents
                    .into_iter()
                    .map(|(node, mark)| {
                        (
                            node,
                            Assumption::parse(std::str::from_utf8(&mark).unwrap()).unwrap(),
                        )
                    })
                    .collect(),
            })
            .collect();
        let context_search = ContextSearch::new(nodes, 2).unwrap();
        assert_eq!(context_search.links(), links(&case["without_dummy"]));
        let phase = &case["phase"];
        let dummy_search = context_search
            .with_contexts(serde_json::from_value(phase["contexts"].clone()).unwrap())
            .unwrap();
        assert_eq!(dummy_search.links(), links(&phase["with_contexts"]));
        let system_search = dummy_search
            .with_dummies(serde_json::from_value(phase["dummies"].clone()).unwrap())
            .unwrap();
        assert_eq!(system_search.links(), links(&phase["system"]));
    }
}
