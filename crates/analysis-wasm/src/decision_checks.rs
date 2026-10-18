//! Browser boundary for the graph checks of a decision: whether logged data can evaluate the policy space, and where intervening can pay off.
use hirmos_causal_core::do_calculus::latent_projection;
use hirmos_causal_core::reinforcement::imitation::sequential_pi_backdoor;
use hirmos_causal_core::reinforcement::sequential_backdoor::{sequential_backdoor, DecisionRule};
use hirmos_causal_core::reinforcement::where_to_intervene::OutcomeGraph;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// One decision in time order and the nodes its rule may read.
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Decision {
    node: usize,
    reads: Vec<usize>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Request {
    names: Vec<String>,
    edges: Vec<(usize, usize)>,
    unobserved: Vec<usize>,
    decisions: Vec<Decision>,
    objective: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Stage {
    decision: usize,
    conditioning: Vec<usize>,
    blocked: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Evidence {
    /// One check per decision; every stage blocked means logged data can evaluate any rule in the space.
    stages: Vec<Stage>,
    admissible: bool,
    /// Sets of variables whose joint intervention is optimal for some model compatible with the graph.
    possibly_optimal: Vec<Vec<usize>>,
    /// For each decision, the recorded variables a copier must read to keep the demonstrator's value; `None` when no such sets exist.
    imitation: Option<Vec<Vec<usize>>>,
}

pub(crate) fn run(request: Request) -> Result<Evidence, String> {
    let n = request.names.len();
    let distinct: BTreeSet<&String> = request.names.iter().collect();
    if n < 2 || distinct.len() != n || request.names.iter().any(|name| name.trim().is_empty()) {
        return Err("decision checks need at least two distinct, non-empty node names".to_owned());
    }
    let named = |index: usize| request.names.get(index).cloned().ok_or_else(|| format!("node {index} is not in the graph"));
    if request.edges.iter().any(|&(a, b)| a >= n || b >= n || a == b) {
        return Err("every edge must join two distinct graph nodes".to_owned());
    }
    if request.decisions.is_empty() {
        return Err("mark at least one decision".to_owned());
    }
    let latent: BTreeSet<usize> = request.unobserved.iter().copied().collect();
    let mentioned = request.decisions.iter().flat_map(|d| d.reads.iter().chain([&d.node])).chain([&request.objective]);
    if let Some(index) = mentioned.into_iter().find(|i| latent.contains(i)) {
        return Err(format!("{} is unobserved, so it cannot be a decision, the objective or something a rule reads", named(*index)?));
    }
    let directed = request.edges.iter().map(|&(a, b)| Ok((named(a)?, named(b)?))).collect::<Result<Vec<_>, String>>()?;
    let latent_names = latent.iter().map(|&i| named(i)).collect::<Result<Vec<_>, String>>()?;
    let projection = latent_projection(request.names.clone(), directed, latent_names).map_err(|e| format!("latent projection failed: {e}"))?;

    let rules = request
        .decisions
        .iter()
        .map(|d| Ok(DecisionRule { action: named(d.node)?, inputs: d.reads.iter().map(|&i| named(i)).collect::<Result<_, String>>()? }))
        .collect::<Result<Vec<_>, String>>()?;
    let objective = named(request.objective)?;
    let index: BTreeMap<&str, usize> = request.names.iter().enumerate().map(|(i, name)| (name.as_str(), i)).collect();
    let indices = |set: &BTreeSet<String>| {
        let mut found: Vec<usize> = set.iter().map(|name| index[name.as_str()]).collect();
        found.sort_unstable();
        found
    };

    let checks = sequential_backdoor(&projection, &rules, &BTreeSet::from([objective.clone()])).map_err(|e| format!("sequential backdoor check failed: {e:?}"))?;
    let stages: Vec<Stage> = checks
        .iter()
        .zip(&request.decisions)
        .map(|(check, d)| Stage { decision: d.node, conditioning: indices(&check.conditioning), blocked: check.blocked })
        .collect();
    let admissible = stages.iter().all(|s| s.blocked);

    let actions: BTreeSet<String> = rules.iter().map(|rule| rule.action.clone()).collect();
    let observed: BTreeSet<String> =
        (0..n).filter(|i| !latent.contains(i) && *i != request.objective).map(|i| request.names[i].clone()).collect();
    let ordering = projection.topological_sort().map_err(|e| format!("the graph has no time order: {e:?}"))?;
    let imitation = sequential_pi_backdoor(&projection, &actions, &objective, &observed, &BTreeSet::from([objective.clone()]), &ordering)
        .map_err(|e| format!("imitation check failed: {e:?}"))?
        .map(|sets| rules.iter().map(|rule| indices(&sets[&rule.action])).collect());

    let outcome = OutcomeGraph::new(projection, objective).map_err(|e| format!("intervention search failed: {e:?}"))?;
    let possibly_optimal = outcome.possibly_optimal_intervention_sets().iter().map(indices).collect();
    Ok(Evidence { stages, admissible, possibly_optimal, imitation })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(edges: Vec<(usize, usize)>, unobserved: Vec<usize>) -> Request {
        Request {
            names: ["S1", "X1", "S2", "X2", "Y", "U"].map(String::from).to_vec(),
            edges,
            unobserved,
            decisions: vec![Decision { node: 1, reads: vec![0] }, Decision { node: 3, reads: vec![0, 1, 2] }],
            objective: 4,
        }
    }

    const CAUSES: [(usize, usize); 8] = [(0, 1), (0, 2), (1, 2), (2, 3), (0, 4), (1, 4), (3, 4), (2, 4)];

    #[test]
    fn an_unrecorded_cause_of_both_treatments_and_the_outcome_leaves_logged_care_unable_to_evaluate_the_plans() {
        let mut edges = CAUSES.to_vec();
        edges.extend([(5, 1), (5, 3), (5, 4)]);
        let evidence = run(request(edges, vec![5])).unwrap();
        assert!(!evidence.admissible);
        assert_eq!(evidence.stages.iter().map(|s| s.blocked).collect::<Vec<_>>(), [false, false]);
    }

    #[test]
    fn without_that_factor_every_stage_is_blocked() {
        let evidence = run(request(CAUSES.to_vec(), vec![])).unwrap();
        assert!(evidence.admissible);
        assert_eq!(evidence.stages[1].conditioning, [0, 1, 2]);
    }

    #[test]
    fn a_decision_confounded_with_its_objective_cannot_be_copied_safely_but_an_unconfounded_one_can() {
        let mut edges = CAUSES.to_vec();
        edges.extend([(5, 1), (5, 4)]);
        assert_eq!(run(request(edges, vec![5])).unwrap().imitation, None);
        let evidence = run(request(CAUSES.to_vec(), vec![])).unwrap();
        assert!(evidence.imitation.is_some());
    }

    #[test]
    fn an_unobserved_decision_is_refused() {
        let mut edges = CAUSES.to_vec();
        edges.push((5, 1));
        let mut r = request(edges, vec![5]);
        r.decisions[0].node = 5;
        assert!(run(r).is_err());
    }
}
