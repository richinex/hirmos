//! Whether every policy in a multi-stage policy space can be evaluated from observational data.

use std::collections::BTreeSet;

use crate::do_calculus::{Admg, GraphError, Variable};

/// One decision: the action and the variables its rule may read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionRule {
    pub action: Variable,
    pub inputs: BTreeSet<Variable>,
}

/// The check at one stage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StageCheck {
    pub action: Variable,
    pub conditioning: BTreeSet<Variable>,
    pub blocked: bool,
}

/// Check every stage of a policy space against the rewards.
pub fn sequential_backdoor(
    graph: &Admg,
    rules: &[DecisionRule],
    rewards: &BTreeSet<Variable>,
) -> Result<Vec<StageCheck>, GraphError> {
    let named = rules
        .iter()
        .flat_map(|rule| rule.inputs.iter().chain([&rule.action]))
        .chain(rewards);
    if let Some(unknown) = named.into_iter().find(|name| !graph.nodes().contains(*name)) {
        return Err(GraphError::UnknownVariable(unknown.clone()));
    }
    let mut checks = Vec::with_capacity(rules.len());
    let mut conditioning = BTreeSet::new();
    for (i, rule) in rules.iter().enumerate() {
        conditioning.extend(rule.inputs.iter().cloned());
        let manipulated = follow_rules(graph, &rules[i + 1..])?
            .without_outgoing(&BTreeSet::from([rule.action.clone()]));
        let blocked = rewards
            .iter()
            .all(|reward| manipulated.are_d_separated(&rule.action, reward, &conditioning));
        checks.push(StageCheck { action: rule.action.clone(), conditioning: conditioning.clone(), blocked });
        conditioning.insert(rule.action.clone());
    }
    Ok(checks)
}

fn follow_rules(graph: &Admg, rules: &[DecisionRule]) -> Result<Admg, GraphError> {
    let actions: BTreeSet<Variable> = rules.iter().map(|rule| rule.action.clone()).collect();
    let cut = graph.without_incoming(&actions);
    let added = rules
        .iter()
        .flat_map(|rule| rule.inputs.iter().map(|input| (input.clone(), rule.action.clone())));
    Admg::new(
        cut.nodes().iter().cloned(),
        cut.directed_edges().iter().cloned().chain(added),
        cut.bidirected_edges().iter().cloned(),
    )
}
