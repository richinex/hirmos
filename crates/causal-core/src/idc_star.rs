//! Identification of conditional binary counterfactual events with IDC*.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::counterfactual_graph::{
    is_not_self_intervened, make_counterfactual_graph, CounterfactualGraph,
};
use crate::counterfactual_query::{
    BinaryValue, ConditionalCounterfactualQuery, CounterfactualExpression, CounterfactualVariable,
    Event, Intervention, QueryError,
};
use crate::do_calculus::Admg;
use crate::id_star::{identify_counterfactual_event, IdStarError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdcStarError {
    InvalidQuery(QueryError),
    Identification(IdStarError),
    InconsistentConditioningEvent,
    ZeroDenominatorExpression,
}

impl fmt::Display for IdcStarError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuery(error) => error.fmt(formatter),
            Self::Identification(error) => error.fmt(formatter),
            Self::InconsistentConditioningEvent => formatter.write_str(
                "the conditioning event is inconsistent, so its conditional probability is undefined",
            ),
            Self::ZeroDenominatorExpression => formatter.write_str(
                "the identified joint counterfactual is zero, so it cannot be conditioned",
            ),
        }
    }
}

impl std::error::Error for IdcStarError {}

impl From<QueryError> for IdcStarError {
    fn from(value: QueryError) -> Self {
        Self::InvalidQuery(value)
    }
}

impl From<IdStarError> for IdcStarError {
    fn from(value: IdStarError) -> Self {
        Self::Identification(value)
    }
}

fn union_events(left: &Event, right: &Event) -> Result<Event, QueryError> {
    let atoms = left.iter().chain(right.iter()).map(|(variable, value)| {
        crate::counterfactual_query::EventAtom::new(variable.clone(), *value)
    });
    Event::new(atoms)
}

fn remaining_and_missing(new_event: &Event, old_event: &Event) -> (Event, Event) {
    let remaining = old_event
        .iter()
        .filter(|(variable, _)| new_event.contains_key(variable))
        .map(|(variable, value)| (variable.clone(), *value))
        .collect();
    let missing = old_event
        .iter()
        .filter(|(variable, _)| !new_event.contains_key(variable))
        .map(|(variable, value)| (variable.clone(), *value))
        .collect();
    (Event::from_map(remaining), Event::from_map(missing))
}

fn relabel_events(
    new_event: &Event,
    old_outcomes: &Event,
    old_conditions: &Event,
) -> (Event, Event, BTreeSet<CounterfactualVariable>) {
    let (mut outcomes, missing_outcomes) = remaining_and_missing(new_event, old_outcomes);
    let (mut conditions, missing_conditions) = remaining_and_missing(new_event, old_conditions);
    let retained_conditions = conditions
        .iter()
        .map(|(variable, _)| variable.clone())
        .collect();
    let old_keys = old_outcomes
        .iter()
        .chain(old_conditions.iter())
        .map(|(variable, _)| variable)
        .collect::<BTreeSet<_>>();
    let new_keys = new_event
        .iter()
        .filter(|(variable, _)| !old_keys.contains(variable))
        .map(|(variable, value)| (variable.clone(), *value))
        .collect::<Vec<_>>();
    let missing_outcome_bases = missing_outcomes
        .iter()
        .map(|(variable, _)| variable.variable.clone())
        .collect::<BTreeSet<_>>();
    let missing_condition_bases = missing_conditions
        .iter()
        .map(|(variable, _)| variable.variable.clone())
        .collect::<BTreeSet<_>>();

    let mut outcome_map = outcomes.clone().into_map();
    let mut condition_map = conditions.clone().into_map();
    match (missing_outcomes.is_empty(), missing_conditions.is_empty()) {
        (false, false) => {
            for (variable, value) in new_keys {
                if missing_outcome_bases.contains(&variable.variable) {
                    outcome_map.insert(variable.clone(), value);
                }
                if missing_condition_bases.contains(&variable.variable) {
                    condition_map.insert(variable, value);
                }
            }
        }
        (false, true) => outcome_map.extend(new_keys),
        (true, false) => condition_map.extend(new_keys),
        (true, true) => {}
    }
    outcomes = Event::from_map(outcome_map);
    conditions = Event::from_map(condition_map);
    (outcomes, conditions, retained_conditions)
}

fn rule_two_applies(
    graph: &CounterfactualGraph,
    outcomes: &Event,
    condition: &CounterfactualVariable,
) -> bool {
    let blocked = graph
        .nodes()
        .iter()
        .filter(|node| !is_not_self_intervened(node))
        .cloned()
        .collect::<BTreeSet<_>>();
    let modified = graph.without_outgoing(condition);
    outcomes
        .iter()
        .all(|(outcome, _)| modified.are_d_separated(outcome, condition, &blocked))
}

/// Identify `P(outcome events | condition events)` using IDC*.
pub fn identify_conditional_counterfactual(
    graph: &Admg,
    query: &ConditionalCounterfactualQuery,
) -> Result<CounterfactualExpression, IdcStarError> {
    identify_recursive(graph, &query.outcomes, &query.conditions)
}

fn identify_recursive(
    graph: &Admg,
    outcomes: &Event,
    conditions: &Event,
) -> Result<CounterfactualExpression, IdcStarError> {
    match identify_counterfactual_event(graph, conditions) {
        Ok(CounterfactualExpression::Zero) => {
            return Err(IdcStarError::InconsistentConditioningEvent);
        }
        Ok(_) | Err(IdStarError::Conflict { .. }) => {}
        Err(error) => return Err(error.into()),
    }

    let combined = union_events(outcomes, conditions)?;
    let (cf_graph, new_event) = make_counterfactual_graph(graph, &combined)?;
    let Some(new_event) = new_event else {
        return Ok(CounterfactualExpression::Zero);
    };
    let (mut new_outcomes, mut new_conditions, retained_conditions) =
        relabel_events(&new_event, outcomes, conditions);

    let candidates = new_conditions
        .iter()
        .map(|(variable, _)| variable.clone())
        .collect::<Vec<_>>();
    // y0 considers conditions that survived counterfactual-graph relabeling before
    // replacements for merged variables. Within either group, use downstream-first
    // ordering so the result does not depend on a hash table's iteration order.
    let condition = candidates
        .into_iter()
        .filter(|condition| rule_two_applies(&cf_graph, &new_outcomes, condition))
        .max_by_key(|condition| {
            (
                retained_conditions.contains(condition),
                cf_graph
                    .ancestors_inclusive(&BTreeSet::from([condition.clone()]))
                    .len(),
            )
        });
    if let Some(condition) = condition {
        let mut outcome_map = BTreeMap::new();
        for (outcome, value) in new_outcomes.iter() {
            let ancestors = cf_graph.ancestors_inclusive(&BTreeSet::from([outcome.clone()]));
            let outcome = if ancestors.contains(&condition) {
                outcome.intervene([Intervention {
                    variable: condition.variable.clone(),
                    value: BinaryValue::Zero,
                }])?
            } else {
                outcome.clone()
            };
            outcome_map.insert(outcome, *value);
        }
        new_outcomes = Event::from_map(outcome_map);
        let mut condition_map = new_conditions.into_map();
        condition_map.remove(&condition);
        new_conditions = Event::from_map(condition_map);
        return identify_recursive(graph, &new_outcomes, &new_conditions);
    }

    let joint = union_events(&new_outcomes, &new_conditions)?;
    let expression = identify_counterfactual_event(graph, &joint)?;
    if conditions.is_empty() {
        return Ok(expression);
    }
    if expression == CounterfactualExpression::Zero {
        return Err(IdcStarError::ZeroDenominatorExpression);
    }
    let condition_bases = conditions
        .iter()
        .map(|(variable, _)| variable.variable.clone())
        .collect::<BTreeSet<_>>();
    let ranges = expression
        .conditioning_variables()
        .difference(&condition_bases)
        .cloned()
        .collect::<BTreeSet<_>>();
    Ok(CounterfactualExpression::normalize_marginalize(
        expression, ranges,
    ))
}
