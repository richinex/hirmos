//! Parity cases from y0's upstream IDC* suite at commit
//! cc6644d3b470a1ab2a1eb9da4f0fdb0c843f5d3a.

use hirmos_causal_core::counterfactual_query::{
    BinaryValue, ConditionalCounterfactualQuery, CounterfactualExpression, CounterfactualVariable,
    Event, EventAtom, Intervention,
};
use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::idc_star::{identify_conditional_counterfactual, IdcStarError};

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

fn query(outcomes: Event, conditions: Event) -> ConditionalCounterfactualQuery {
    ConditionalCounterfactualQuery::new(outcomes, conditions).unwrap()
}

fn chain() -> Admg {
    Admg::new(
        ["D", "Z", "Y"].map(str::to_owned),
        [("D", "Z"), ("Z", "Y")].map(|(source, target)| (source.to_owned(), target.to_owned())),
        [],
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
fn refuses_an_inconsistent_conditioning_event() {
    let outcomes = event(vec![(
        cf("Y", &[("D", BinaryValue::Zero)]),
        BinaryValue::Zero,
    )]);
    let conditions = event(vec![
        (cf("Z", &[("D", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("Z"), BinaryValue::One),
        (factual("D"), BinaryValue::Zero),
    ]);
    assert_eq!(
        identify_conditional_counterfactual(&chain(), &query(outcomes, conditions)),
        Err(IdcStarError::InconsistentConditioningEvent)
    );
}

#[test]
fn returns_zero_for_an_inconsistent_joint_event() {
    let outcomes = event(vec![(
        cf("Z", &[("D", BinaryValue::Zero)]),
        BinaryValue::Zero,
    )]);
    let conditions = event(vec![
        (factual("Z"), BinaryValue::One),
        (factual("D"), BinaryValue::Zero),
    ]);
    assert_eq!(
        identify_conditional_counterfactual(&chain(), &query(outcomes, conditions)).unwrap(),
        CounterfactualExpression::Zero
    );
}

#[test]
fn tikka_figure_2_matches_y0() {
    let graph = Admg::new(
        ["X", "Y", "Z"].map(str::to_owned),
        [("X", "Z"), ("X", "Y"), ("Z", "Y")]
            .map(|(source, target)| (source.to_owned(), target.to_owned())),
        [("X".to_owned(), "Z".to_owned())],
    )
    .unwrap();
    let outcomes = event(vec![(
        cf("Y", &[("X", BinaryValue::Zero)]),
        BinaryValue::Zero,
    )]);
    let conditions = event(vec![
        (cf("Z", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("X"), BinaryValue::One),
    ]);
    assert_eq!(
        identify_conditional_counterfactual(&graph, &query(outcomes, conditions))
            .unwrap()
            .to_y0(),
        // y0 stores intervention sets in hash iteration order and emitted P[Z,X](Y)
        // in this environment. Rust uses a stable canonical order for the same set.
        "P[X,Z](Y)"
    );
}

#[test]
fn figure_9a_conditional_query_matches_y0() {
    let outcomes = event(vec![(
        cf("Y", &[("X", BinaryValue::Zero)]),
        BinaryValue::Zero,
    )]);
    let conditions = event(vec![
        (factual("X"), BinaryValue::One),
        (cf("Z", &[("D", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("D"), BinaryValue::Zero),
    ]);
    assert_eq!(
        identify_conditional_counterfactual(&figure_9a(), &query(outcomes, conditions))
            .unwrap()
            .to_y0(),
        "((Sum[D, W](P(D) * P[X](W) * P[W,Z](X, Y) * P[D](Z)) / Sum[D, W, Y](Sum[D, W](P(D) * P[X](W) * P[W,Z](X, Y) * P[D](Z)))))"
    );
}
