//! Parallel-world and counterfactual-graph construction for ID*/IDC*.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::counterfactual_query::{
    BinaryValue, CounterfactualVariable, Event, Intervention, QueryError,
};
use crate::do_calculus::Admg;

pub type World = BTreeSet<Intervention>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CounterfactualGraph {
    nodes: BTreeSet<CounterfactualVariable>,
    directed: BTreeSet<(CounterfactualVariable, CounterfactualVariable)>,
    bidirected: BTreeSet<(CounterfactualVariable, CounterfactualVariable)>,
}

impl CounterfactualGraph {
    fn new(
        nodes: impl IntoIterator<Item = CounterfactualVariable>,
        directed: impl IntoIterator<Item = (CounterfactualVariable, CounterfactualVariable)>,
        bidirected: impl IntoIterator<Item = (CounterfactualVariable, CounterfactualVariable)>,
    ) -> Self {
        let mut nodes = nodes.into_iter().collect::<BTreeSet<_>>();
        let directed = directed
            .into_iter()
            .filter(|(source, target)| source != target)
            .inspect(|(source, target)| {
                nodes.insert(source.clone());
                nodes.insert(target.clone());
            })
            .collect();
        let bidirected = bidirected
            .into_iter()
            .filter(|(left, right)| left != right)
            .map(|(left, right)| {
                nodes.insert(left.clone());
                nodes.insert(right.clone());
                normalize_edge(left, right)
            })
            .collect();
        Self {
            nodes,
            directed,
            bidirected,
        }
    }

    pub fn nodes(&self) -> &BTreeSet<CounterfactualVariable> {
        &self.nodes
    }

    pub fn directed_edges(&self) -> &BTreeSet<(CounterfactualVariable, CounterfactualVariable)> {
        &self.directed
    }

    pub fn bidirected_edges(&self) -> &BTreeSet<(CounterfactualVariable, CounterfactualVariable)> {
        &self.bidirected
    }

    pub(crate) fn subgraph(&self, keep: &BTreeSet<CounterfactualVariable>) -> Self {
        Self::new(
            self.nodes.intersection(keep).cloned(),
            self.directed
                .iter()
                .filter(|(source, target)| keep.contains(source) && keep.contains(target))
                .cloned(),
            self.bidirected
                .iter()
                .filter(|(left, right)| keep.contains(left) && keep.contains(right))
                .cloned(),
        )
    }

    pub(crate) fn ancestors_inclusive(
        &self,
        targets: &BTreeSet<CounterfactualVariable>,
    ) -> BTreeSet<CounterfactualVariable> {
        let mut ancestors = targets.clone();
        let mut queue = VecDeque::from_iter(targets.iter().cloned());
        while let Some(node) = queue.pop_front() {
            for (source, target) in &self.directed {
                if target == &node && ancestors.insert(source.clone()) {
                    queue.push_back(source.clone());
                }
            }
        }
        ancestors
    }

    pub(crate) fn parents(
        &self,
        node: &CounterfactualVariable,
    ) -> BTreeSet<CounterfactualVariable> {
        self.directed
            .iter()
            .filter_map(|(source, target)| (target == node).then_some(source.clone()))
            .collect()
    }

    fn bidirected_degree(&self, node: &CounterfactualVariable) -> usize {
        self.bidirected
            .iter()
            .filter(|(left, right)| left == node || right == node)
            .count()
    }

    fn has_bidirected_edge(
        &self,
        left: &CounterfactualVariable,
        right: &CounterfactualVariable,
    ) -> bool {
        self.bidirected
            .contains(&normalize_edge(left.clone(), right.clone()))
    }

    pub(crate) fn districts(&self) -> BTreeSet<BTreeSet<CounterfactualVariable>> {
        let mut remaining = self.nodes.clone();
        let mut districts = BTreeSet::new();
        while let Some(start) = remaining.pop_first() {
            let mut district = BTreeSet::from([start.clone()]);
            let mut queue = VecDeque::from([start]);
            while let Some(node) = queue.pop_front() {
                for (left, right) in &self.bidirected {
                    let neighbor = if left == &node {
                        Some(right)
                    } else if right == &node {
                        Some(left)
                    } else {
                        None
                    };
                    if let Some(neighbor) = neighbor {
                        if remaining.remove(neighbor) {
                            district.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
            districts.insert(district);
        }
        districts
    }

    pub(crate) fn is_connected(&self) -> bool {
        self.nodes.is_empty() || self.districts().len() == 1
    }

    pub(crate) fn markov_pillow(
        &self,
        district: &BTreeSet<CounterfactualVariable>,
    ) -> BTreeSet<CounterfactualVariable> {
        district
            .iter()
            .flat_map(|node| self.parents(node))
            .filter(|parent| !district.contains(parent))
            .collect()
    }

    pub(crate) fn without_outgoing(&self, source: &CounterfactualVariable) -> Self {
        Self::new(
            self.nodes.clone(),
            self.directed
                .iter()
                .filter(|(candidate, _)| candidate != source)
                .cloned(),
            self.bidirected.clone(),
        )
    }

    pub(crate) fn are_d_separated(
        &self,
        left: &CounterfactualVariable,
        right: &CounterfactualVariable,
        conditions: &BTreeSet<CounterfactualVariable>,
    ) -> bool {
        let named = conditions
            .iter()
            .cloned()
            .chain([left.clone(), right.clone()])
            .collect();
        let ancestors = self.ancestors_inclusive(&named);
        let graph = self.subgraph(&ancestors);
        let mut adjacency: BTreeMap<CounterfactualVariable, BTreeSet<CounterfactualVariable>> =
            graph
                .nodes
                .iter()
                .cloned()
                .map(|node| (node, BTreeSet::new()))
                .collect();
        let mut link = |a: &CounterfactualVariable, b: &CounterfactualVariable| {
            adjacency.entry(a.clone()).or_default().insert(b.clone());
            adjacency.entry(b.clone()).or_default().insert(a.clone());
        };
        for (source, target) in &graph.directed {
            link(source, target);
        }
        for (a, b) in &graph.bidirected {
            link(a, b);
        }
        for child in &graph.nodes {
            let parents = graph.parents(child).into_iter().collect::<Vec<_>>();
            for i in 0..parents.len() {
                for j in i + 1..parents.len() {
                    link(&parents[i], &parents[j]);
                }
            }
        }
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([left.clone()]);
        while let Some(node) = queue.pop_front() {
            if conditions.contains(&node) || !visited.insert(node.clone()) {
                continue;
            }
            if &node == right {
                return false;
            }
            if let Some(neighbors) = adjacency.get(&node) {
                queue.extend(
                    neighbors
                        .iter()
                        .filter(|neighbor| !conditions.contains(*neighbor))
                        .cloned(),
                );
            }
        }
        true
    }
}

fn normalize_edge(
    left: CounterfactualVariable,
    right: CounterfactualVariable,
) -> (CounterfactualVariable, CounterfactualVariable) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}

pub(crate) fn is_not_self_intervened(node: &CounterfactualVariable) -> bool {
    !node
        .interventions
        .iter()
        .any(|intervention| intervention.variable == node.variable)
}

fn node_not_intervened_in_world(world: &World, variable: &str) -> bool {
    !world
        .iter()
        .any(|intervention| intervention.variable == variable)
}

pub fn extract_worlds(event: &Event) -> BTreeSet<World> {
    event
        .iter()
        .filter_map(|(variable, _)| {
            (!variable.interventions.is_empty()).then_some(variable.interventions.clone())
        })
        .collect()
}

fn factual(variable: &str) -> CounterfactualVariable {
    CounterfactualVariable::factual(variable).expect("graph variables are non-empty")
}

fn in_world(variable: &str, world: &World) -> CounterfactualVariable {
    CounterfactualVariable::in_world(variable, world.iter().cloned())
        .expect("worlds are validated and non-empty")
}

pub fn make_parallel_worlds_graph(graph: &Admg, worlds: &BTreeSet<World>) -> CounterfactualGraph {
    let mut nodes = graph
        .nodes()
        .iter()
        .map(|node| factual(node))
        .collect::<BTreeSet<_>>();
    for world in worlds {
        nodes.extend(graph.nodes().iter().map(|node| in_world(node, world)));
    }

    let mut directed = graph
        .directed_edges()
        .iter()
        .map(|(source, target)| (factual(source), factual(target)))
        .collect::<BTreeSet<_>>();
    for world in worlds {
        directed.extend(
            graph
                .directed_edges()
                .iter()
                .filter_map(|(source, target)| {
                    node_not_intervened_in_world(world, target)
                        .then_some((in_world(source, world), in_world(target, world)))
                }),
        );
    }

    let mut bidirected = graph
        .bidirected_edges()
        .iter()
        .map(|(left, right)| normalize_edge(factual(left), factual(right)))
        .collect::<BTreeSet<_>>();
    for world in worlds {
        for node in graph.nodes() {
            if node_not_intervened_in_world(world, node) {
                bidirected.insert(normalize_edge(factual(node), in_world(node, world)));
            }
        }
        for (left, right) in graph.bidirected_edges() {
            if node_not_intervened_in_world(world, left)
                && node_not_intervened_in_world(world, right)
            {
                bidirected.insert(normalize_edge(
                    in_world(left, world),
                    in_world(right, world),
                ));
            }
            if node_not_intervened_in_world(world, right) {
                bidirected.insert(normalize_edge(factual(left), in_world(right, world)));
            }
            if node_not_intervened_in_world(world, left) {
                bidirected.insert(normalize_edge(factual(right), in_world(left, world)));
            }
        }
    }
    let worlds = worlds.iter().collect::<Vec<_>>();
    for i in 0..worlds.len() {
        for j in i + 1..worlds.len() {
            for node in graph.nodes() {
                if node_not_intervened_in_world(worlds[i], node)
                    && node_not_intervened_in_world(worlds[j], node)
                {
                    bidirected.insert(normalize_edge(
                        in_world(node, worlds[i]),
                        in_world(node, worlds[j]),
                    ));
                }
            }
            for (left, right) in graph.bidirected_edges() {
                for (source, target) in [(left, right), (right, left)] {
                    if node_not_intervened_in_world(worlds[i], source)
                        && node_not_intervened_in_world(worlds[j], target)
                    {
                        bidirected.insert(normalize_edge(
                            in_world(source, worlds[i]),
                            in_world(target, worlds[j]),
                        ));
                    }
                }
            }
        }
    }
    CounterfactualGraph::new(nodes, directed, bidirected)
}

fn has_same_confounders(
    graph: &CounterfactualGraph,
    left: &CounterfactualVariable,
    right: &CounterfactualVariable,
) -> bool {
    graph.has_bidirected_edge(left, right)
        || (graph.bidirected_degree(left) == 0 && graph.bidirected_degree(right) == 0)
}

fn nodes_attain_same_value(
    graph: &CounterfactualGraph,
    event: &Event,
    left: &CounterfactualVariable,
    right: &CounterfactualVariable,
) -> bool {
    if left == right {
        return true;
    }
    if !has_same_confounders(graph, left, right) || left.variable != right.variable {
        return false;
    }
    match (event.get(left), event.get(right)) {
        (Some(left_value), Some(right_value)) => left_value == right_value,
        (Some(value), None) => right.interventions.contains(&Intervention {
            variable: left.variable.clone(),
            value,
        }),
        (None, Some(value)) => left.interventions.contains(&Intervention {
            variable: right.variable.clone(),
            value,
        }),
        (None, None) => left.is_factual() && right.is_factual(),
    }
}

fn parents_attain_same_values(
    graph: &CounterfactualGraph,
    event: &Event,
    left: &CounterfactualVariable,
    right: &CounterfactualVariable,
) -> bool {
    if !has_same_confounders(graph, left, right) {
        return false;
    }
    let left_parents = graph.parents(left);
    let right_parents = graph.parents(right);
    if left_parents == right_parents {
        return true;
    }
    let left_only = left_parents
        .difference(&right_parents)
        .cloned()
        .collect::<Vec<_>>();
    let right_only = right_parents
        .difference(&left_parents)
        .cloned()
        .collect::<Vec<_>>();
    left_only.len() == right_only.len()
        && left_only
            .iter()
            .zip(&right_only)
            .all(|(a, b)| nodes_attain_same_value(graph, event, a, b))
}

fn self_intervention_value(node: &CounterfactualVariable) -> Option<BinaryValue> {
    node.interventions
        .iter()
        .find(|intervention| intervention.variable == node.variable)
        .map(|intervention| intervention.value)
}

fn nodes_have_same_domain(
    graph: &CounterfactualGraph,
    left: &CounterfactualVariable,
    right: &CounterfactualVariable,
) -> bool {
    if !has_same_confounders(graph, left, right) || left.variable != right.variable {
        return false;
    }
    match (is_not_self_intervened(left), is_not_self_intervened(right)) {
        (true, true) => true,
        (true, false) | (false, true) => false,
        (false, false) => self_intervention_value(left) == self_intervention_value(right),
    }
}

fn equivalent(
    graph: &CounterfactualGraph,
    event: &Event,
    left: &CounterfactualVariable,
    right: &CounterfactualVariable,
) -> bool {
    left.variable == right.variable
        && is_not_self_intervened(left) == is_not_self_intervened(right)
        && parents_attain_same_values(graph, event, left, right)
        && nodes_have_same_domain(graph, left, right)
}

fn merge(
    graph: &CounterfactualGraph,
    mut preferred: CounterfactualVariable,
    mut eliminated: CounterfactualVariable,
) -> (
    CounterfactualGraph,
    CounterfactualVariable,
    CounterfactualVariable,
) {
    if !preferred.is_factual() && eliminated.is_factual()
        || preferred.is_factual() == eliminated.is_factual() && eliminated < preferred
    {
        std::mem::swap(&mut preferred, &mut eliminated);
    }
    let mut directed = graph
        .directed
        .iter()
        .filter(|(source, target)| source != &eliminated && target != &eliminated)
        .cloned()
        .collect::<BTreeSet<_>>();
    directed.extend(
        graph
            .directed
            .iter()
            .filter(|(source, _)| source == &eliminated)
            .map(|(_, target)| (preferred.clone(), target.clone())),
    );
    let mut bidirected = graph
        .bidirected
        .iter()
        .filter(|(left, right)| left != &eliminated && right != &eliminated)
        .cloned()
        .collect::<BTreeSet<_>>();
    for (left, right) in &graph.bidirected {
        if left == &eliminated && right != &preferred {
            bidirected.insert(normalize_edge(preferred.clone(), right.clone()));
        } else if right == &eliminated && left != &preferred {
            bidirected.insert(normalize_edge(left.clone(), preferred.clone()));
        }
    }
    let preferred_parents = graph.parents(&preferred);
    let eliminated_only_parents = graph
        .parents(&eliminated)
        .difference(&preferred_parents)
        .cloned()
        .collect::<BTreeSet<_>>();
    let nodes = graph
        .nodes
        .iter()
        .filter(|node| *node != &eliminated && !eliminated_only_parents.contains(*node))
        .cloned();
    (
        CounterfactualGraph::new(nodes, directed, bidirected),
        preferred,
        eliminated,
    )
}

pub fn make_counterfactual_graph(
    graph: &Admg,
    event: &Event,
) -> Result<(CounterfactualGraph, Option<Event>), QueryError> {
    let worlds = extract_worlds(event);
    let mut cf_graph = make_parallel_worlds_graph(graph, &worlds);
    let mut new_event = event.clone().into_map();

    for base in graph
        .topological_sort()
        .expect("the ADMG constructor validates acyclicity")
    {
        let factual = factual(&base);
        for world in &worlds {
            let counterfactual = in_world(&base, world);
            if cf_graph.nodes.contains(&factual)
                && cf_graph.nodes.contains(&counterfactual)
                && equivalent(
                    &cf_graph,
                    &Event::from_map(new_event.clone()),
                    &factual,
                    &counterfactual,
                )
            {
                let (next_graph, preferred, eliminated) =
                    merge(&cf_graph, factual.clone(), counterfactual.clone());
                if matches!((new_event.get(&preferred), new_event.get(&eliminated)), (Some(a), Some(b)) if a != b)
                {
                    return Ok((next_graph, None));
                }
                if let Some(value) = new_event.remove(&eliminated) {
                    new_event.insert(preferred, value);
                }
                cf_graph = next_graph;
            }
        }
        let worlds = worlds.iter().collect::<Vec<_>>();
        for i in 0..worlds.len() {
            for j in i + 1..worlds.len() {
                let left = in_world(&base, worlds[i]);
                let right = in_world(&base, worlds[j]);
                if cf_graph.nodes.contains(&left)
                    && cf_graph.nodes.contains(&right)
                    && equivalent(
                        &cf_graph,
                        &Event::from_map(new_event.clone()),
                        &left,
                        &right,
                    )
                {
                    let (next_graph, preferred, eliminated) = merge(&cf_graph, left, right);
                    if matches!((new_event.get(&preferred), new_event.get(&eliminated)), (Some(a), Some(b)) if a != b)
                    {
                        return Ok((next_graph, None));
                    }
                    if let Some(value) = new_event.remove(&eliminated) {
                        new_event.insert(preferred, value);
                    }
                    cf_graph = next_graph;
                }
            }
        }
    }
    let event_nodes = new_event.keys().cloned().collect();
    let ancestors = cf_graph.ancestors_inclusive(&event_nodes);
    Ok((
        cf_graph.subgraph(&ancestors),
        Some(Event::from_map(new_event)),
    ))
}
