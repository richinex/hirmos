//! Which observed variables each imitated action may read so that copying the demonstrator keeps its reward.

use std::collections::{BTreeMap, BTreeSet};

use crate::do_calculus::{Admg, GraphError, Variable};

/// For each action, the variables its imitating rule conditions on.
pub type ConditioningSets = BTreeMap<Variable, BTreeSet<Variable>>;

/// For each action, the variables its imitating rule reads, given what is observed, the reward signals and every node in time order, or `None` when no such sets exist.
pub fn sequential_pi_backdoor(
    graph: &Admg,
    actions: &BTreeSet<Variable>,
    outcome: &Variable,
    observed: &BTreeSet<Variable>,
    rewards: &BTreeSet<Variable>,
    ordering: &[Variable],
) -> Result<Option<ConditioningSets>, GraphError> {
    let named = actions.iter().chain([outcome]).chain(observed).chain(rewards).chain(ordering);
    if let Some(unknown) = named.into_iter().find(|v| !graph.nodes().contains(*v)) {
        return Err(GraphError::UnknownVariable(unknown.clone()));
    }
    let ancestral = Links::new(graph.subgraph(&graph.ancestors_inclusive(&BTreeSet::from([outcome.clone()]))));
    let order: Vec<&Variable> = ordering.iter().filter(|v| ancestral.graph.nodes().contains(*v)).collect();
    let seen: BTreeSet<Variable> = observed.union(rewards).cloned().collect();

    let matched = match_observations(&ancestral, actions, &seen, &order);
    if !actions.iter().all(|a| matched.contains_key(a)) {
        return Ok(None);
    }
    let matched_nodes: BTreeSet<Variable> = matched.keys().cloned().collect();
    let boundary = markov_boundary(&ancestral, &matched_nodes, observed);
    let edge_actions: BTreeSet<Variable> = actions
        .iter()
        .filter(|&a| {
            let (mut reached, _) = ancestral.plus(a, observed, Direction::Down);
            reached.extend(ancestral.children(a).cloned());
            !reached.is_subset(&matched_nodes)
        })
        .cloned()
        .collect();
    Ok(Some(
        actions
            .iter()
            .map(|a| {
                let before: BTreeSet<&Variable> = ordering.iter().take_while(|v| *v != a).collect();
                let inputs = if edge_actions.contains(a) {
                    boundary.union(&edge_actions).filter(|v| before.contains(v)).cloned().collect()
                } else {
                    BTreeSet::new()
                };
                (a.clone(), inputs)
            })
            .collect(),
    ))
}

/// The observed variables that are neither `action`, `outcome` nor descendants of `action`.
pub fn non_descendant_inputs(graph: &Admg, action: &Variable, outcome: &Variable, observed: &BTreeSet<Variable>) -> BTreeSet<Variable> {
    let links = Links::new(graph.clone());
    let mut descendants = BTreeSet::new();
    let mut stack = vec![action];
    while let Some(v) = stack.pop() {
        for child in links.children(v) {
            if descendants.insert(child.clone()) {
                stack.push(child);
            }
        }
    }
    observed.iter().filter(|v| *v != action && *v != outcome && !descendants.contains(*v)).cloned().collect()
}

/// The smallest subset of `candidates`, first in sorted order, that blocks every backdoor path from `action` to `outcome`.
pub fn pi_backdoor(graph: &Admg, action: &Variable, outcome: &Variable, candidates: &BTreeSet<Variable>) -> Option<BTreeSet<Variable>> {
    let cut = graph.without_outgoing(&BTreeSet::from([action.clone()]));
    let pool: Vec<&Variable> = candidates.iter().collect();
    let mut chosen: Vec<usize> = Vec::with_capacity(pool.len());
    let mut subset = BTreeSet::new();
    for size in 0..=pool.len() {
        chosen.clear();
        chosen.extend(0..size);
        loop {
            subset.clear();
            subset.extend(chosen.iter().map(|&i| pool[i].clone()));
            if cut.are_d_separated(action, outcome, &subset) {
                return Some(subset);
            }
            let Some(i) = (0..size).rev().find(|&i| chosen[i] < pool.len() - size + i) else { break };
            chosen[i] += 1;
            for j in i + 1..size {
                chosen[j] = chosen[j - 1] + 1;
            }
        }
    }
    None
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
}

struct Links {
    graph: Admg,
    parents: BTreeMap<Variable, Vec<Variable>>,
    children: BTreeMap<Variable, Vec<Variable>>,
    districts: Vec<BTreeSet<Variable>>,
}

impl Links {
    fn new(graph: Admg) -> Self {
        let mut parents: BTreeMap<Variable, Vec<Variable>> = BTreeMap::new();
        let mut children: BTreeMap<Variable, Vec<Variable>> = BTreeMap::new();
        for (from, to) in graph.directed_edges() {
            parents.entry(to.clone()).or_default().push(from.clone());
            children.entry(from.clone()).or_default().push(to.clone());
        }
        let districts = graph.districts().into_iter().collect();
        Self { graph, parents, children, districts }
    }

    fn children(&self, v: &Variable) -> impl Iterator<Item = &Variable> {
        self.children.get(v).into_iter().flatten()
    }

    fn district(&self, v: &Variable) -> &BTreeSet<Variable> {
        self.districts.iter().find(|d| d.contains(v)).expect("every node lies in a district")
    }

    /// Observed variables reached from `v` along one direction, passing through unobserved ones, and the unobserved ones passed.
    fn plus(&self, v: &Variable, observed: &BTreeSet<Variable>, direction: Direction) -> (BTreeSet<Variable>, BTreeSet<Variable>) {
        let step = |v: &Variable| match direction {
            Direction::Up => self.parents.get(v),
            Direction::Down => self.children.get(v),
        };
        let (mut reached, mut hidden) = (BTreeSet::new(), BTreeSet::new());
        let mut stack: Vec<&Variable> = step(v).into_iter().flatten().collect();
        while let Some(next) = stack.pop() {
            if observed.contains(next) {
                reached.insert(next.clone());
            } else {
                hidden.insert(next.clone());
                stack.extend(step(next).into_iter().flatten());
            }
        }
        (reached, hidden)
    }
}

/// Pairs observed variables with the earliest action whose adjustment they can stand behind, working back from the outcome.
fn match_observations(
    links: &Links,
    actions: &BTreeSet<Variable>,
    seen: &BTreeSet<Variable>,
    order: &[&Variable],
) -> BTreeMap<Variable, Variable> {
    let position: BTreeMap<&Variable, usize> = order.iter().enumerate().map(|(i, v)| (*v, i)).collect();
    let mut matched: BTreeMap<Variable, Variable> = BTreeMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        for &oi in order.iter().rev() {
            if !seen.contains(oi) {
                continue;
            }
            let (reached, _) = links.plus(oi, seen, Direction::Down);
            let target = if !reached.is_empty() && reached.iter().all(|c| matched.contains_key(c)) {
                reached.iter().map(|c| &matched[c]).filter(|a| actions.contains(*a)).min_by_key(|a| position[a]).cloned()
            } else if actions.contains(oi) {
                Some(oi.clone())
            } else {
                None
            };
            if let Some(xi) = target {
                if matched.get(oi) != Some(&xi) && adjustable(links, &matched, oi, &xi, seen, order) {
                    matched.insert(oi.clone(), xi);
                    changed = true;
                }
            }
        }
    }
    matched
}

fn adjustable(
    links: &Links,
    matched: &BTreeMap<Variable, Variable>,
    oi: &Variable,
    xi: &Variable,
    seen: &BTreeSet<Variable>,
    order: &[&Variable],
) -> bool {
    let district = links.district(oi);
    let mut kept = district.clone();
    for v in district {
        let (reached, hidden) = links.plus(v, seen, Direction::Up);
        kept.extend(reached);
        kept.extend(hidden);
    }
    let local = links.graph.subgraph(&kept);
    let before: BTreeSet<&Variable> = order.iter().copied().take_while(|v| *v != xi).collect();
    let others: Vec<&Variable> = district.iter().filter(|v| *v != oi && !matched.contains_key(*v)).collect();
    let conditions: BTreeSet<Variable> = others.iter().filter(|v| before.contains(*v)).map(|v| (*v).clone()).collect();
    others.iter().all(|o| local.are_d_separated(oi, o, &conditions))
}

fn markov_boundary(links: &Links, matched: &BTreeSet<Variable>, observed: &BTreeSet<Variable>) -> BTreeSet<Variable> {
    let mut reach = matched.clone();
    for v in matched {
        reach.extend(links.plus(v, observed, Direction::Down).0);
    }
    let mut members = BTreeSet::new();
    for v in &reach {
        members.extend(links.district(v).iter().filter(|m| observed.contains(*m)).cloned());
    }
    let mut boundary = members.clone();
    for v in &members {
        boundary.extend(links.plus(v, observed, Direction::Up).0);
    }
    boundary.retain(|v| !matched.contains(v));
    boundary
}
