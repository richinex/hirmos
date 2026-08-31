//! Complete identification of unconditional binary counterfactual events with ID*.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::counterfactual_graph::{
    is_not_self_intervened, make_counterfactual_graph, CounterfactualGraph,
};
use crate::counterfactual_query::{
    BinaryValue, CounterfactualExpression, CounterfactualVariable, Event, Intervention, QueryError,
};
use crate::do_calculus::{Admg, Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdStarError {
    InvalidQuery(QueryError),
    UnknownVariable(Variable),
    Conflict {
        interventions: Vec<(Intervention, Intervention)>,
    },
    InternalInvariant(&'static str),
}

impl fmt::Display for IdStarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuery(error) => error.fmt(formatter),
            Self::UnknownVariable(variable) => write!(formatter, "unknown variable {variable}"),
            Self::Conflict { .. } => formatter.write_str(
                "the counterfactual event contains conflicting evidence and interventions",
            ),
            Self::InternalInvariant(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for IdStarError {}

impl From<QueryError> for IdStarError {
    fn from(value: QueryError) -> Self {
        Self::InvalidQuery(value)
    }
}

pub fn violates_axiom_of_effectiveness(event: &Event) -> bool {
    event.iter().any(|(variable, value)| {
        variable.interventions.iter().any(|intervention| {
            intervention.variable == variable.variable && intervention.value != *value
        })
    })
}

pub fn remove_event_tautologies(event: &Event) -> Event {
    Event::from_map(
        event
            .iter()
            .filter(|(variable, value)| {
                !variable.interventions.iter().any(|intervention| {
                    intervention.variable == variable.variable && intervention.value == **value
                })
            })
            .map(|(variable, value)| (variable.clone(), *value))
            .collect(),
    )
}

fn interventions_in_nodes(
    nodes: impl IntoIterator<Item = CounterfactualVariable>,
) -> BTreeSet<Intervention> {
    nodes
        .into_iter()
        .flat_map(|node| node.interventions)
        .collect()
}

fn evidence(event: &Event) -> BTreeSet<Intervention> {
    event
        .iter()
        .flat_map(|(variable, value)| {
            variable.interventions.iter().cloned().chain([Intervention {
                variable: variable.variable.clone(),
                value: *value,
            }])
        })
        .collect()
}

fn conflicts(graph: &CounterfactualGraph, event: &Event) -> Vec<(Intervention, Intervention)> {
    let interventions = interventions_in_nodes(graph.nodes().iter().cloned());
    let evidence = evidence(event);
    interventions
        .iter()
        .flat_map(|intervention| {
            evidence.iter().filter_map(move |observed| {
                (intervention.variable == observed.variable && intervention.value != observed.value)
                    .then_some((intervention.clone(), observed.clone()))
            })
        })
        .collect()
}

fn free_variables(graph: &CounterfactualGraph, event: &Event) -> BTreeSet<Variable> {
    let graph_bases = graph
        .nodes()
        .iter()
        .filter(|node| is_not_self_intervened(node))
        .map(|node| node.variable.clone())
        .collect::<BTreeSet<_>>();
    let event_bases = event
        .iter()
        .map(|(variable, _)| variable.variable.clone())
        .collect::<BTreeSet<_>>();
    graph_bases.difference(&event_bases).cloned().collect()
}

fn node_event(node: &CounterfactualVariable, event: &Event) -> BinaryValue {
    event.get(node).unwrap_or(BinaryValue::Zero)
}

fn event_for_district(
    graph: &CounterfactualGraph,
    district: &BTreeSet<CounterfactualVariable>,
    event: &Event,
) -> Result<Event, IdStarError> {
    let pillow = graph.markov_pillow(district);
    let pillow_interventions = pillow
        .iter()
        .map(|node| Intervention {
            variable: node.variable.clone(),
            value: BinaryValue::Zero,
        })
        .collect::<Vec<_>>();
    let mut mapped = BTreeMap::new();
    for node in district {
        let variable = if pillow_interventions.is_empty() {
            CounterfactualVariable::factual(&node.variable)?
        } else {
            CounterfactualVariable::in_world(&node.variable, pillow_interventions.iter().cloned())?
        };
        mapped.insert(variable, node_event(node, event));
    }
    Ok(Event::from_map(mapped))
}

fn line_nine(graph: &CounterfactualGraph) -> Result<CounterfactualExpression, IdStarError> {
    let interventions = interventions_in_nodes(graph.nodes().iter().cloned());
    let mut variables = BTreeSet::new();
    for base in graph.nodes().iter().map(|node| &node.variable) {
        variables.insert(if interventions.is_empty() {
            CounterfactualVariable::factual(base)?
        } else {
            CounterfactualVariable::in_world(base, interventions.iter().cloned())?
        });
    }
    Ok(CounterfactualExpression::probability(variables))
}

/// Identify the probability of a binary counterfactual event using ID*.
pub fn identify_counterfactual_event(
    graph: &Admg,
    event: &Event,
) -> Result<CounterfactualExpression, IdStarError> {
    for (variable, _) in event.iter() {
        if !graph.nodes().contains(&variable.variable) {
            return Err(IdStarError::UnknownVariable(variable.variable.clone()));
        }
        for intervention in &variable.interventions {
            if !graph.nodes().contains(&intervention.variable) {
                return Err(IdStarError::UnknownVariable(intervention.variable.clone()));
            }
        }
    }
    identify_recursive(graph, event)
}

fn identify_recursive(
    graph: &Admg,
    event: &Event,
) -> Result<CounterfactualExpression, IdStarError> {
    if event.is_empty() {
        return Ok(CounterfactualExpression::One);
    }
    if violates_axiom_of_effectiveness(event) {
        return Ok(CounterfactualExpression::Zero);
    }
    let reduced = remove_event_tautologies(event);
    if &reduced != event {
        return identify_recursive(graph, &reduced);
    }

    let (cf_graph, new_event) = make_counterfactual_graph(graph, event)?;
    let Some(new_event) = new_event else {
        return Ok(CounterfactualExpression::Zero);
    };
    let active_nodes = cf_graph
        .nodes()
        .iter()
        .filter(|node| is_not_self_intervened(node))
        .cloned()
        .collect();
    let active_graph = cf_graph.subgraph(&active_nodes);
    if !active_graph.is_connected() {
        let summand = free_variables(&cf_graph, &new_event);
        let districts = active_graph.districts();
        if districts.len() <= 1 {
            return Err(IdStarError::InternalInvariant(
                "a disconnected counterfactual graph must contain multiple districts",
            ));
        }
        let factors = districts
            .iter()
            .map(|district| {
                event_for_district(&cf_graph, district, &new_event)
                    .and_then(|district_event| identify_recursive(graph, &district_event))
            })
            .collect::<Result<Vec<_>, _>>()?;
        return Ok(CounterfactualExpression::marginalize(
            CounterfactualExpression::product(factors),
            summand,
        ));
    }

    let conflicts = conflicts(&active_graph, &new_event);
    if !conflicts.is_empty() {
        return Err(IdStarError::Conflict {
            interventions: conflicts,
        });
    }
    line_nine(&active_graph)
}
