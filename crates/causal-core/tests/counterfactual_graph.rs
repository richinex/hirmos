//! Counterfactual-graph parity with Shpitser and Pearl figure 9 as encoded by y0.

use std::collections::BTreeSet;

use hirmos_causal_core::counterfactual_graph::make_counterfactual_graph;
use hirmos_causal_core::counterfactual_query::{
    BinaryValue, CounterfactualVariable, Event, EventAtom, Intervention,
};
use hirmos_causal_core::do_calculus::Admg;

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
fn make_counterfactual_graph_matches_y0_figure_9c() {
    let query = event(vec![
        (cf("Y", &[("X", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("X"), BinaryValue::One),
        (cf("Z", &[("D", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("D"), BinaryValue::Zero),
    ]);
    let (graph, relabeled) = make_counterfactual_graph(&figure_9a(), &query).unwrap();

    let x_world = [("X", BinaryValue::Zero)];
    assert_eq!(
        graph.nodes(),
        &BTreeSet::from([
            factual("X"),
            factual("D"),
            factual("Z"),
            cf("W", &x_world),
            cf("Y", &x_world),
            cf("X", &x_world),
        ])
    );
    assert_eq!(
        graph.directed_edges(),
        &BTreeSet::from([
            (cf("X", &x_world), cf("W", &x_world)),
            (cf("W", &x_world), cf("Y", &x_world)),
            (factual("D"), factual("Z")),
            (factual("Z"), cf("Y", &x_world)),
        ])
    );
    assert_eq!(
        graph.bidirected_edges(),
        &BTreeSet::from([(factual("X"), cf("Y", &x_world))])
    );
    assert_eq!(
        relabeled.unwrap(),
        event(vec![
            (cf("Y", &x_world), BinaryValue::Zero),
            (factual("X"), BinaryValue::One),
            (factual("Z"), BinaryValue::Zero),
            (factual("D"), BinaryValue::Zero),
        ])
    );
}

#[test]
fn merged_worlds_with_different_event_values_are_inconsistent() {
    let graph = Admg::new(
        ["D", "Z", "Y"].map(str::to_owned),
        [("D", "Z"), ("Z", "Y")].map(|(source, target)| (source.to_owned(), target.to_owned())),
        [],
    )
    .unwrap();
    let query = event(vec![
        (cf("Z", &[("D", BinaryValue::Zero)]), BinaryValue::Zero),
        (factual("Z"), BinaryValue::One),
        (factual("D"), BinaryValue::Zero),
    ]);
    let (_, relabeled) = make_counterfactual_graph(&graph, &query).unwrap();
    assert!(relabeled.is_none());
}
