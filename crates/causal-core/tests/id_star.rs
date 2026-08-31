//! Parity cases from y0's upstream ID* test suite at commit
//! cc6644d3b470a1ab2a1eb9da4f0fdb0c843f5d3a.

use hirmos_causal_core::counterfactual_query::{
    BinaryValue, CounterfactualExpression, CounterfactualVariable, Event, EventAtom, Intervention,
};
use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::id_star::{
    identify_counterfactual_event, remove_event_tautologies, violates_axiom_of_effectiveness,
    IdStarError,
};

fn intervention(name: &str, value: BinaryValue) -> Intervention {
    Intervention::new(name, value).unwrap()
}

fn factual(name: &str) -> CounterfactualVariable {
    CounterfactualVariable::factual(name).unwrap()
}

fn cf(name: &str, interventions: &[(&str, BinaryValue)]) -> CounterfactualVariable {
    CounterfactualVariable::in_world(
        name,
        interventions
            .iter()
            .map(|(variable, value)| intervention(variable, *value)),
    )
    .unwrap()
}

fn event(atoms: Vec<(CounterfactualVariable, BinaryValue)>) -> Event {
    Event::new(
        atoms
            .into_iter()
            .map(|(variable, value)| EventAtom::new(variable, value)),
    )
    .unwrap()
}

fn figure_9a() -> Admg {
    Admg::new(
        ["X", "W", "Y", "D", "Z"].map(str::to_owned),
        [("X", "W"), ("W", "Y"), ("D", "Z"), ("Z", "Y")]
            .map(|(source, target)| (source.to_owned(), target.to_owned())),
        [("X".to_owned(), "Y".to_owned())],
    )
    .unwrap()
}

#[test]
fn id_star_lines_one_through_three_match_y0() {
    assert_eq!(
        identify_counterfactual_event(&figure_9a(), &Event::empty()).unwrap(),
        CounterfactualExpression::One
    );
    let contradiction = event(vec![(
        cf("X", &[("X", BinaryValue::Zero)]),
        BinaryValue::One,
    )]);
    assert!(violates_axiom_of_effectiveness(&contradiction));
    assert_eq!(
        identify_counterfactual_event(&figure_9a(), &contradiction).unwrap(),
        CounterfactualExpression::Zero
    );

    let tautology = event(vec![
        (cf("X", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
        (cf("Y", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
    ]);
    assert_eq!(
        remove_event_tautologies(&tautology),
        event(vec![(
            cf("Y", &[("X", BinaryValue::Zero)]),
            BinaryValue::Zero,
        )])
    );
}

#[test]
fn figure_9a_joint_counterfactual_matches_y0() {
    let query = event(vec![
        (
            cf("Y", &[("X", BinaryValue::One), ("Z", BinaryValue::Zero)]),
            BinaryValue::One,
        ),
        (factual("X"), BinaryValue::Zero),
    ]);
    let expression = identify_counterfactual_event(&figure_9a(), &query).unwrap();
    assert_eq!(expression.to_y0(), "Sum[W](P[X](W) * P[W,Z](X, Y))");
}

#[test]
fn figure_9a_four_event_query_matches_y0() {
    let query = event(vec![
        (cf("Y", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("X"), BinaryValue::One),
        (cf("Z", &[("D", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("D"), BinaryValue::Zero),
    ]);
    let expression = identify_counterfactual_event(&figure_9a(), &query).unwrap();
    assert_eq!(
        expression.to_y0(),
        "Sum[W](P(D) * P[X](W) * P[W,Z](X, Y) * P[D](Z))"
    );
}

#[test]
fn conflict_is_reported_as_unidentifiable() {
    let graph = Admg::new(
        ["X", "Y"].map(str::to_owned),
        [("X".to_owned(), "Y".to_owned())],
        [],
    )
    .unwrap();
    let query = event(vec![
        (cf("Y", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
        (cf("Y", &[("X", BinaryValue::One)]), BinaryValue::One),
    ]);
    assert!(matches!(
        identify_counterfactual_event(&graph, &query),
        Err(IdStarError::Conflict { .. })
    ));
}
