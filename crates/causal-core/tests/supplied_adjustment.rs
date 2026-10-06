use hirmos_causal_core::backdoor::Dag;
use hirmos_causal_core::{validate_adjustment_set, SuppliedAdjustment};
use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    name: String,
    nodes: usize,
    edges: Vec<(usize, usize)>,
    unobserved: Vec<usize>,
    treatment: usize,
    outcome: usize,
    set: Vec<usize>,
    valid: bool,
}
#[derive(Deserialize)]
struct Fixture {
    cases: Vec<Case>,
}

#[test]
fn supplied_sets_match_dagitty() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../oracle/fixtures/dagitty_supplied_sets.json"
    ))
    .unwrap();
    assert_eq!(fixture.cases.len(), 820);
    for case in fixture.cases {
        for reverse in [false, true] {
            let mut edges = case.edges.clone();
            if reverse {
                edges.reverse();
            }
            let result = validate_adjustment_set(
                &Dag::new(case.nodes, &edges),
                case.treatment,
                case.outcome,
                &case.unobserved,
                &case.set,
            )
            .unwrap();
            assert_eq!(
                result == SuppliedAdjustment::Valid,
                case.valid,
                "{} {:?}",
                case.name,
                case.set
            );
        }
    }
}

#[test]
fn explains_forbidden_descendants_and_open_paths() {
    // X=0, M=1, Y=2, Z=3.
    let model12 = Dag::new(4, &[(0, 1), (1, 2), (1, 3)]);
    assert_eq!(
        validate_adjustment_set(&model12, 0, 2, &[], &[3]).unwrap(),
        SuppliedAdjustment::ForbiddenDescendants { nodes: vec![3] }
    );
    let model14 = Dag::new(4, &[(0, 2), (0, 3)]);
    assert_eq!(
        validate_adjustment_set(&model14, 0, 2, &[], &[3]).unwrap(),
        SuppliedAdjustment::Valid
    );
    // A descendant of a collider opens a path that starts OUT of treatment.
    let graph = Dag::new(5, &[(0, 2), (0, 1), (4, 1), (4, 2), (1, 3)]);
    assert_eq!(
        validate_adjustment_set(&graph, 0, 2, &[4], &[3]).unwrap(),
        SuppliedAdjustment::OpenNoncausalPath
    );
    assert!(validate_adjustment_set(&graph, 0, 2, &[], &[5]).is_err());
    assert_eq!(
        validate_adjustment_set(&graph, 0, 2, &[], &[0]).unwrap(),
        SuppliedAdjustment::Endpoints { nodes: vec![0] }
    );
    assert_eq!(
        validate_adjustment_set(&graph, 0, 2, &[4], &[4]).unwrap(),
        SuppliedAdjustment::Unobserved { nodes: vec![4] }
    );
}
