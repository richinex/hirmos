//! Which sets of variables are worth experimenting on to raise an outcome, from the causal graph alone.

use std::collections::BTreeSet;
use std::fmt;

use crate::do_calculus::{Admg, Variable};

/// A set of variables to intervene on together.
pub type InterventionSet = BTreeSet<Variable>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WhereToInterveneError {
    UnknownOutcome(Variable),
}

impl fmt::Display for WhereToInterveneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOutcome(name) => write!(f, "the outcome {name} is not a node of the graph"),
        }
    }
}

impl std::error::Error for WhereToInterveneError {}

/// An acyclic mixed graph paired with the outcome the interventions aim to raise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutcomeGraph {
    graph: Admg,
    outcome: Variable,
}

impl OutcomeGraph {
    /// Pair a graph with its outcome, refusing an outcome that is not one of its nodes.
    pub fn new(graph: Admg, outcome: impl Into<Variable>) -> Result<Self, WhereToInterveneError> {
        let outcome = outcome.into();
        if graph.nodes().contains(&outcome) {
            Ok(Self { graph, outcome })
        } else {
            Err(WhereToInterveneError::UnknownOutcome(outcome))
        }
    }

    pub fn graph(&self) -> &Admg {
        &self.graph
    }

    pub fn outcome(&self) -> &str {
        &self.outcome
    }

    /// Every intervention set that changes the outcome's distribution with no redundant member.
    pub fn minimal_intervention_sets(&self) -> BTreeSet<InterventionSet> {
        let candidates: InterventionSet = self
            .graph
            .nodes()
            .iter()
            .filter(|node| **node != self.outcome)
            .cloned()
            .collect();
        let restricted = ancestral(&self.graph, &self.outcome);
        let order = only(&backward_order(&restricted), &candidates);
        minimal_sets_below(&restricted, &self.outcome, &InterventionSet::new(), &order)
    }

    /// The minimal intervention sets that are optimal for some model compatible with the graph.
    pub fn possibly_optimal_intervention_sets(&self) -> BTreeSet<InterventionSet> {
        let restricted = ancestral(&self.graph, &self.outcome);
        let (territory, border) = territory_and_border(&restricted, &self.outcome);
        let reduced = intervene_and_restrict(&restricted, &border, &territory);
        let mut candidates = territory.clone();
        candidates.remove(&self.outcome);
        let order = only(&backward_order(&reduced), &candidates);
        let mut found = possibly_optimal_below(&reduced, &self.outcome, &order, &InterventionSet::new());
        found.insert(border);
        found
    }

    /// The possibly optimal sets found by trying every subset of the other variables.
    pub fn possibly_optimal_by_enumeration(&self) -> BTreeSet<InterventionSet> {
        let candidates: Vec<Variable> = self
            .graph
            .nodes()
            .iter()
            .filter(|node| **node != self.outcome)
            .cloned()
            .collect();
        (0u64..(1u64 << candidates.len()))
            .map(|mask| {
                let intervened: InterventionSet = candidates
                    .iter()
                    .enumerate()
                    .filter(|(bit, _)| mask >> bit & 1 == 1)
                    .map(|(_, name)| name.clone())
                    .collect();
                territory_and_border(&self.graph.without_incoming(&intervened), &self.outcome).1
            })
            .collect()
    }

    /// The outcome together with every variable it shares a latent confounder with, closed under
    /// descendants inside the outcome's ancestors.
    pub fn minimal_unobserved_confounders_territory(&self) -> InterventionSet {
        territory(&self.graph, &self.outcome)
    }

    /// The parents of that territory outside it: the one intervention set that is always possibly
    /// optimal.
    pub fn interventional_border(&self) -> InterventionSet {
        territory_and_border(&self.graph, &self.outcome).1
    }

    /// The members of `intervened` that can still reach the outcome once all of them are fixed.
    pub fn minimal_do(&self, intervened: &InterventionSet) -> InterventionSet {
        let reachable = self
            .graph
            .without_incoming(intervened)
            .ancestors_inclusive(&singleton(&self.outcome));
        intervened.intersection(&reachable).cloned().collect()
    }
}

fn singleton(name: &str) -> InterventionSet {
    BTreeSet::from([name.to_owned()])
}

fn ancestral(graph: &Admg, outcome: &str) -> Admg {
    graph.subgraph(&graph.ancestors_inclusive(&singleton(outcome)))
}

fn intervene_and_restrict(graph: &Admg, intervened: &InterventionSet, territory: &InterventionSet) -> Admg {
    let keep: InterventionSet = territory.union(intervened).cloned().collect();
    graph.without_incoming(intervened).subgraph(&keep)
}

fn parents(graph: &Admg, nodes: &InterventionSet) -> InterventionSet {
    graph
        .directed_edges()
        .iter()
        .filter(|(_, target)| nodes.contains(target))
        .map(|(source, _)| source.clone())
        .collect()
}

fn descendants(graph: &Admg, nodes: &InterventionSet) -> InterventionSet {
    let mut found = InterventionSet::new();
    let mut frontier: Vec<Variable> = nodes.iter().cloned().collect();
    while let Some(node) = frontier.pop() {
        for (source, target) in graph.directed_edges() {
            if *source == node && found.insert(target.clone()) {
                frontier.push(target.clone());
            }
        }
    }
    found
}

fn c_component(graph: &Admg, node: &str) -> InterventionSet {
    graph
        .districts()
        .into_iter()
        .find(|district| district.contains(node))
        .unwrap_or_else(|| singleton(node))
}

fn backward_order(graph: &Admg) -> Vec<Variable> {
    let mut order = graph.topological_sort().expect("subgraphs of an acyclic graph are acyclic");
    order.reverse();
    order
}

fn only(order: &[Variable], keep: &InterventionSet) -> Vec<Variable> {
    order.iter().filter(|node| keep.contains(*node)).cloned().collect()
}

fn territory(graph: &Admg, outcome: &str) -> InterventionSet {
    let restricted = ancestral(graph, outcome);
    let mut queue = singleton(outcome);
    let mut territory = singleton(outcome);
    while let Some(next) = queue.pop_first() {
        let component = c_component(&restricted, &next);
        territory.extend(component.iter().cloned());
        queue.extend(descendants(&restricted, &component));
        queue.retain(|node| !territory.contains(node));
    }
    territory
}

fn territory_and_border(graph: &Admg, outcome: &str) -> (InterventionSet, InterventionSet) {
    let territory = territory(graph, outcome);
    let border = parents(graph, &territory).difference(&territory).cloned().collect();
    (territory, border)
}

fn minimal_sets_below(
    graph: &Admg,
    outcome: &str,
    chosen: &InterventionSet,
    order: &[Variable],
) -> BTreeSet<InterventionSet> {
    let mut found = BTreeSet::from([chosen.clone()]);
    for (i, node) in order.iter().enumerate() {
        let reduced = ancestral(&graph.without_incoming(&singleton(node)), outcome);
        let mut extended = chosen.clone();
        extended.insert(node.clone());
        let remaining = only(&order[i + 1..], reduced.nodes());
        found.extend(minimal_sets_below(&reduced, outcome, &extended, &remaining));
    }
    found
}

fn possibly_optimal_below(
    graph: &Admg,
    outcome: &str,
    order: &[Variable],
    passed_over: &InterventionSet,
) -> BTreeSet<InterventionSet> {
    let mut found = BTreeSet::new();
    for (i, node) in order.iter().enumerate() {
        let (territory, border) = territory_and_border(&graph.without_incoming(&singleton(node)), outcome);
        let mut skipped = passed_over.clone();
        skipped.extend(order[..i].iter().cloned());
        if !border.is_disjoint(&skipped) {
            continue;
        }
        found.insert(border.clone());
        let remaining = only(&order[i + 1..], &territory);
        if !remaining.is_empty() {
            let reduced = intervene_and_restrict(graph, &border, &territory);
            found.extend(possibly_optimal_below(&reduced, outcome, &remaining, &skipped));
        }
    }
    found
}
