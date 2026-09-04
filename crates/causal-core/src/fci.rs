//! Fast Causal Inference (FCI), including complete PAG orientation rules R0-R10.
//!
//! Ported from causal-learn commit `9de1d886b7ab45868819aec7132f79378990f0b6`,
//! `causallearn/search/ConstraintBased/FCI.py`. The output remains a PAG: circles and
//! bidirected edges are not coerced into directed DAG edges.

use crate::constraint_discovery::{
    combinations, stable_adjacency_search, BackgroundKnowledge, CiEngine, CiRecord,
    ConstraintDiscoveryError, CrossSectionalCi, Endpoint, EndpointGraph, SeparatingSets,
};
use crate::pc_stable::{public_separating_sets, SeparatedPair};
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FciConfiguration {
    pub alpha: f64,
    pub max_depth: Option<usize>,
    pub max_path_length: Option<usize>,
    pub ci_test: CrossSectionalCi,
}

impl Default for FciConfiguration {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            max_depth: None,
            max_path_length: None,
            ci_test: CrossSectionalCi::FisherZ,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Directness {
    DefinitelyDirect,
    PossiblyDirect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatentConfounding {
    Excluded,
    Possible,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FciEdgeProperty {
    pub left: usize,
    pub right: usize,
    pub directness: Option<Directness>,
    pub latent_confounding: Option<LatentConfounding>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FciResult {
    pub pag: EndpointGraph,
    pub separating_sets: Vec<SeparatedPair>,
    pub ci_tests: Vec<CiRecord>,
    pub edge_properties: Vec<FciEdgeProperty>,
}

pub fn run_fci(
    data: &DMatrix<f64>,
    node_names: Vec<String>,
    configuration: FciConfiguration,
    background: &BackgroundKnowledge,
) -> Result<FciResult, ConstraintDiscoveryError> {
    run_fci_with_progress(data, node_names, configuration, background, |_, _, _| {})
}

pub fn run_fci_with_progress<F>(
    data: &DMatrix<f64>,
    node_names: Vec<String>,
    configuration: FciConfiguration,
    background: &BackgroundKnowledge,
    mut progress: F,
) -> Result<FciResult, ConstraintDiscoveryError>
where
    F: FnMut(&'static str, usize, usize),
{
    let mut ci = CiEngine::new(data, configuration.ci_test)?;
    let (mut graph, mut separating_sets) = stable_adjacency_search(
        node_names,
        configuration.alpha,
        configuration.max_depth,
        background,
        &mut ci,
    )?;
    progress("skeleton", 1, 5);

    graph.reorient_all(Endpoint::Circle);
    rule_zero(&mut graph, &separating_sets, background);
    progress("initial-orientation", 2, 5);
    remove_by_possible_dsep(
        &mut graph,
        &mut separating_sets,
        configuration.alpha,
        &mut ci,
    )?;
    progress("possible-d-separation", 3, 5);
    graph.reorient_all(Endpoint::Circle);
    rule_zero(&mut graph, &separating_sets, background);

    let mut changed = true;
    let mut first_time = true;
    while changed {
        changed = false;
        changed |= rules_one_two(&mut graph, background);
        changed |= rule_three(&mut graph, &separating_sets, background);
        if changed
            || (first_time
                && background.has_explicit_forbidden()
                && background.has_explicit_required()
                && background.has_tiers())
        {
            changed |= rule_four(
                &mut graph,
                &mut separating_sets,
                configuration.max_path_length,
                configuration.alpha,
                background,
                &mut ci,
            )?;
            first_time = false;
        }
        changed |= rule_five(&mut graph);
        changed |= rule_six(&mut graph);
        changed |= rule_seven(&mut graph);
        changed |= rule_eight(&mut graph);
        changed |= rule_nine(&mut graph);
        changed |= rule_ten(&mut graph);
    }
    progress("orientation", 4, 5);

    let edge_properties = color_edges(&mut graph);
    progress("edge-properties", 5, 5);
    Ok(FciResult {
        pag: graph,
        separating_sets: public_separating_sets(&separating_sets),
        ci_tests: ci.records,
        edge_properties,
    })
}

fn orient_fci_background(graph: &mut EndpointGraph, background: &BackgroundKnowledge) {
    for left in 0..graph.nodes() {
        for right in left + 1..graph.nodes() {
            if !graph.adjacent(left, right) {
                continue;
            }
            if background.is_forbidden(left, right, &graph.node_names) {
                graph.set_edge(left, right, Endpoint::Arrow, Endpoint::Tail);
            } else if background.is_forbidden(right, left, &graph.node_names) {
                graph.set_edge(left, right, Endpoint::Tail, Endpoint::Arrow);
            } else if background.is_required(left, right, &graph.node_names) {
                graph.set_edge(left, right, Endpoint::Tail, Endpoint::Arrow);
            } else if background.is_required(right, left, &graph.node_names) {
                graph.set_edge(left, right, Endpoint::Arrow, Endpoint::Tail);
            }
        }
    }
}

fn arrow_allowed(
    graph: &EndpointGraph,
    from: usize,
    to: usize,
    background: &BackgroundKnowledge,
) -> bool {
    match graph.endpoint(from, to) {
        Some(Endpoint::Arrow) => return true,
        Some(Endpoint::Tail) | None => return false,
        Some(Endpoint::Circle) => {}
    }
    if matches!(
        graph.endpoint(to, from),
        Some(Endpoint::Arrow | Endpoint::Tail)
    ) && background.is_forbidden(from, to, &graph.node_names)
    {
        return false;
    }
    true
}

fn rule_zero(
    graph: &mut EndpointGraph,
    separating_sets: &SeparatingSets,
    background: &BackgroundKnowledge,
) {
    graph.reorient_all(Endpoint::Circle);
    orient_fci_background(graph, background);
    for middle in 0..graph.nodes() {
        let adjacent = graph.neighbors(middle);
        for pair in combinations(&adjacent, 2) {
            let left = pair[0];
            let right = pair[1];
            if graph.adjacent(left, right) || graph.is_def_collider(left, middle, right) {
                continue;
            }
            if separating_sets
                .get(&(left, right))
                .is_some_and(|set| !set.contains(&middle))
                && arrow_allowed(graph, left, middle, background)
                && arrow_allowed(graph, right, middle, background)
            {
                let at_left = graph.endpoint_at(left, middle).expect("R0 left endpoint");
                let at_right = graph.endpoint_at(right, middle).expect("R0 right endpoint");
                graph.set_edge(left, middle, at_left, Endpoint::Arrow);
                graph.set_edge(right, middle, at_right, Endpoint::Arrow);
            }
        }
    }
}

fn rule_one(
    graph: &mut EndpointGraph,
    left: usize,
    middle: usize,
    right: usize,
    background: &BackgroundKnowledge,
) -> bool {
    if graph.adjacent(left, right) {
        return false;
    }
    if graph.endpoint(left, middle) == Some(Endpoint::Arrow)
        && graph.endpoint(right, middle) == Some(Endpoint::Circle)
        && arrow_allowed(graph, middle, right, background)
    {
        graph.set_edge(middle, right, Endpoint::Tail, Endpoint::Arrow);
        return true;
    }
    false
}

fn rule_two(
    graph: &mut EndpointGraph,
    left: usize,
    middle: usize,
    right: usize,
    background: &BackgroundKnowledge,
) -> bool {
    if graph.adjacent(left, right)
        && graph.endpoint(left, right) == Some(Endpoint::Circle)
        && graph.endpoint(left, middle) == Some(Endpoint::Arrow)
        && graph.endpoint(middle, right) == Some(Endpoint::Arrow)
        && matches!(graph.endpoint(middle, left), Some(Endpoint::Tail))
        || graph.adjacent(left, right)
            && graph.endpoint(left, right) == Some(Endpoint::Circle)
            && graph.endpoint(left, middle) == Some(Endpoint::Arrow)
            && graph.endpoint(middle, right) == Some(Endpoint::Arrow)
            && graph.endpoint(right, middle) == Some(Endpoint::Tail)
    {
        if !arrow_allowed(graph, left, right, background) {
            return false;
        }
        let at_left = graph.endpoint_at(left, right).expect("R2 distal endpoint");
        graph.set_edge(left, right, at_left, Endpoint::Arrow);
        return true;
    }
    false
}

fn rules_one_two(graph: &mut EndpointGraph, background: &BackgroundKnowledge) -> bool {
    let mut changed = false;
    for middle in 0..graph.nodes() {
        let adjacent = graph.neighbors(middle);
        for pair in combinations(&adjacent, 2) {
            let left = pair[0];
            let right = pair[1];
            changed |= rule_one(graph, left, middle, right, background);
            changed |= rule_one(graph, right, middle, left, background);
            changed |= rule_two(graph, left, middle, right, background);
            changed |= rule_two(graph, right, middle, left, background);
        }
    }
    changed
}

fn rule_three(
    graph: &mut EndpointGraph,
    separating_sets: &SeparatingSets,
    background: &BackgroundKnowledge,
) -> bool {
    let mut changed = false;
    for middle in 0..graph.nodes() {
        let arrows = graph.nodes_into(middle, Endpoint::Arrow);
        let circles = graph.nodes_into(middle, Endpoint::Circle);
        for &candidate in &circles {
            for pair in combinations(&arrows, 2) {
                let left = pair[0];
                let right = pair[1];
                if graph.adjacent(left, right)
                    || !graph.adjacent(left, candidate)
                    || !graph.adjacent(right, candidate)
                    || !separating_sets
                        .get(&(left, right))
                        .is_some_and(|set| set.contains(&candidate))
                    || graph.endpoint(left, candidate) != Some(Endpoint::Circle)
                    || graph.endpoint(right, candidate) != Some(Endpoint::Circle)
                    || !arrow_allowed(graph, candidate, middle, background)
                {
                    continue;
                }
                let at_candidate = graph
                    .endpoint_at(candidate, middle)
                    .expect("R3 distal endpoint");
                graph.set_edge(candidate, middle, at_candidate, Endpoint::Arrow);
                changed = true;
            }
        }
    }
    changed
}

fn semi_directed_next(graph: &EndpointGraph, node: usize, other: usize) -> bool {
    matches!(
        graph.endpoint_at(node, other),
        Some(Endpoint::Tail | Endpoint::Circle)
    )
}

fn potentially_directed_next(graph: &EndpointGraph, node: usize, other: usize) -> bool {
    matches!(
        graph.endpoint_at(node, other),
        Some(Endpoint::Tail | Endpoint::Circle)
    ) && matches!(
        graph.endpoint_at(other, node),
        Some(Endpoint::Arrow | Endpoint::Circle)
    )
}

fn exists_semi_directed_path(graph: &EndpointGraph, start: usize, destination: usize) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();
    for next in graph.neighbors(start) {
        if semi_directed_next(graph, start, next) && visited.insert(next) {
            queue.push_back(next);
        }
    }
    while let Some(node) = queue.pop_front() {
        if node == destination {
            return true;
        }
        for next in graph.neighbors(node) {
            if semi_directed_next(graph, node, next) && visited.insert(next) {
                queue.push_back(next);
            }
        }
    }
    false
}

fn is_uncovered_path(graph: &EndpointGraph, path: &[usize]) -> bool {
    path.windows(3)
        .all(|triple| !graph.adjacent(triple[0], triple[2]))
}

fn exists_uncovered_pd_path(
    graph: &EndpointGraph,
    start: usize,
    next: usize,
    destination: usize,
) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::from([start, next]);
    for candidate in graph.neighbors(next) {
        if potentially_directed_next(graph, next, candidate) && visited.insert(candidate) {
            queue.push_back((candidate, vec![start, next, candidate]));
        }
    }
    while let Some((node, path)) = queue.pop_front() {
        if node == destination && is_uncovered_path(graph, &path) {
            return true;
        }
        for candidate in graph.neighbors(node) {
            if potentially_directed_next(graph, node, candidate) && visited.insert(candidate) {
                let mut extended = path.clone();
                extended.push(candidate);
                queue.push_back((candidate, extended));
            }
        }
    }
    false
}

fn possible_dsep(
    graph: &EndpointGraph,
    x: usize,
    y: usize,
    max_path_length: Option<usize>,
) -> Vec<usize> {
    let mut dsep = BTreeSet::new();
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();
    let mut previous: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for neighbor in graph.neighbors(x) {
        if neighbor == y {
            continue;
        }
        queue.push_back((x, neighbor, 1usize));
        visited.insert((x, neighbor));
        previous.entry(x).or_default().insert(neighbor);
        dsep.insert(neighbor);
    }
    while let Some((left, middle, distance)) = queue.pop_front() {
        if max_path_length.is_some_and(|maximum| distance > maximum) {
            continue;
        }
        if previous.get(&middle).is_some_and(|parents| {
            parents.iter().any(|&parent| {
                parent != middle
                    && parent != x
                    && (exists_semi_directed_path(graph, parent, x)
                        || exists_semi_directed_path(graph, parent, middle))
            })
        }) {
            dsep.insert(middle);
        }
        for right in graph.neighbors(middle) {
            if right == left || right == x || right == y {
                continue;
            }
            previous.entry(right).or_default().insert(middle);
            if (graph.is_def_collider(left, middle, right) || graph.adjacent(left, right))
                && visited.insert((left, right))
            {
                queue.push_back((left, right, distance + 1));
            }
        }
    }
    dsep.remove(&x);
    dsep.remove(&y);
    let mut values: Vec<usize> = dsep.into_iter().collect();
    values.sort_by(|left, right| graph.node_names[*right].cmp(&graph.node_names[*left]));
    values
}

fn remove_by_possible_dsep(
    graph: &mut EndpointGraph,
    separating_sets: &mut SeparatingSets,
    alpha: f64,
    ci: &mut CiEngine<'_>,
) -> Result<(), ConstraintDiscoveryError> {
    let edges: Vec<(usize, usize)> = graph
        .edges()
        .into_iter()
        .map(|edge| (edge.left, edge.right))
        .collect();
    for (left, right) in edges {
        for (x, y) in [(left, right), (right, left)] {
            if !graph.adjacent(left, right) {
                break;
            }
            let candidates = possible_dsep(graph, x, y, None);
            'sets: for size in 2..=candidates.len() {
                for conditions in combinations(&candidates, size) {
                    let adjacent_x: BTreeSet<_> = graph.neighbors(x).into_iter().collect();
                    let adjacent_y: BTreeSet<_> = graph.neighbors(y).into_iter().collect();
                    if conditions.iter().all(|node| adjacent_x.contains(node))
                        || conditions.iter().all(|node| adjacent_y.contains(node))
                    {
                        continue;
                    }
                    if ci.p_value(x, y, &conditions)? > alpha {
                        graph.remove(left, right);
                        separating_sets.insert((x, y), conditions.into_iter().collect());
                        break 'sets;
                    }
                }
            }
        }
    }
    Ok(())
}

fn discriminating_path(endpoint: usize, previous: &BTreeMap<usize, usize>) -> Vec<usize> {
    let mut path = Vec::new();
    let mut node = previous.get(&endpoint).copied();
    while let Some(value) = node {
        path.push(value);
        node = previous.get(&value).copied();
    }
    path
}

#[allow(clippy::too_many_arguments)]
fn orient_discriminating_path(
    graph: &mut EndpointGraph,
    d: usize,
    a: usize,
    b: usize,
    c: usize,
    previous: &BTreeMap<usize, usize>,
    separating_sets: &mut SeparatingSets,
    alpha: f64,
    background: &BackgroundKnowledge,
    ci: &mut CiEngine<'_>,
) -> Result<(bool, bool), ConstraintDiscoveryError> {
    if graph.adjacent(d, c) {
        return Ok((false, false));
    }
    let path = discriminating_path(d, previous);
    let independent_with_b = ci.p_value(d, c, &path)? > alpha;
    let path_without_b: Vec<_> = path.iter().copied().filter(|&node| node != b).collect();
    let independent_without_b = ci.p_value(d, c, &path_without_b)? > alpha;
    let independent = if !independent_with_b && !independent_without_b {
        let Some(set) = separating_sets.get(&(d, c)) else {
            return Ok((false, false));
        };
        set.contains(&b)
    } else {
        independent_with_b
    };

    if independent {
        let at_c = graph.endpoint_at(c, b).expect("R4 endpoint at C");
        graph.set_edge(c, b, at_c, Endpoint::Tail);
        Ok((true, true))
    } else {
        if !arrow_allowed(graph, a, b, background) || !arrow_allowed(graph, c, b, background) {
            return Ok((false, false));
        }
        let at_a = graph.endpoint_at(a, b).expect("R4 endpoint at A");
        let at_c = graph.endpoint_at(c, b).expect("R4 endpoint at C");
        graph.set_edge(a, b, at_a, Endpoint::Arrow);
        graph.set_edge(c, b, at_c, Endpoint::Arrow);
        Ok((true, true))
    }
}

#[allow(clippy::too_many_arguments)]
fn ddp_orient(
    graph: &mut EndpointGraph,
    a: usize,
    b: usize,
    c: usize,
    max_path_length: Option<usize>,
    separating_sets: &mut SeparatingSets,
    alpha: f64,
    background: &BackgroundKnowledge,
    ci: &mut CiEngine<'_>,
) -> Result<bool, ConstraintDiscoveryError> {
    let mut queue = VecDeque::from([a]);
    let mut visited = BTreeSet::from([a, b]);
    let mut previous = BTreeMap::from([(a, b)]);
    let c_parents: BTreeSet<_> = (0..graph.nodes())
        .filter(|&node| graph.is_parent(node, c))
        .collect();
    let mut distance = 0usize;
    let mut level_end = Some(a);
    while let Some(node) = queue.pop_front() {
        if level_end == Some(node) {
            level_end = queue.back().copied();
            distance += 1;
            if max_path_length.is_some_and(|maximum| distance > maximum) {
                return Ok(false);
            }
        }
        for d in graph.nodes_into(node, Endpoint::Arrow) {
            if visited.contains(&d) {
                continue;
            }
            previous.insert(d, node);
            let parent = previous[&node];
            if !graph.is_def_collider(d, node, parent) {
                continue;
            }
            if !graph.adjacent(d, c) && d != c {
                let (stop, changed) = orient_discriminating_path(
                    graph,
                    d,
                    a,
                    b,
                    c,
                    &previous,
                    separating_sets,
                    alpha,
                    background,
                    ci,
                )?;
                if stop {
                    return Ok(changed);
                }
            }
            if c_parents.contains(&d) {
                queue.push_back(d);
                visited.insert(d);
            }
        }
    }
    Ok(false)
}

fn rule_four(
    graph: &mut EndpointGraph,
    separating_sets: &mut SeparatingSets,
    max_path_length: Option<usize>,
    alpha: f64,
    background: &BackgroundKnowledge,
    ci: &mut CiEngine<'_>,
) -> Result<bool, ConstraintDiscoveryError> {
    let mut changed = false;
    for b in 0..graph.nodes() {
        let possible_a = graph.nodes_out_of(b, Endpoint::Arrow);
        let possible_c = graph.nodes_into(b, Endpoint::Circle);
        for &a in &possible_a {
            for &c in &possible_c {
                if graph.is_parent(a, c) && graph.endpoint(b, c) == Some(Endpoint::Arrow) {
                    changed |= ddp_orient(
                        graph,
                        a,
                        b,
                        c,
                        max_path_length,
                        separating_sets,
                        alpha,
                        background,
                        ci,
                    )?;
                }
            }
        }
    }
    Ok(changed)
}

fn uncovered_circle_paths(
    graph: &EndpointGraph,
    start: usize,
    destination: usize,
    excluded: &[usize],
) -> Vec<Vec<usize>> {
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();
    let mut paths = Vec::new();
    for next in graph.neighbors(start) {
        if excluded.contains(&next) {
            continue;
        }
        if graph.endpoint_at(start, next) == Some(Endpoint::Circle)
            && graph.endpoint_at(next, start) == Some(Endpoint::Circle)
            && visited.insert(next)
        {
            queue.push_back((next, vec![start, next]));
        }
    }
    while let Some((node, path)) = queue.pop_front() {
        if node == destination && is_uncovered_path(graph, &path) {
            paths.push(path.clone());
        }
        for next in graph.neighbors(node) {
            if excluded.contains(&next) {
                continue;
            }
            if graph.endpoint_at(node, next) == Some(Endpoint::Circle)
                && graph.endpoint_at(next, node) == Some(Endpoint::Circle)
                && visited.insert(next)
            {
                let mut extended = path.clone();
                extended.push(next);
                queue.push_back((next, extended));
            }
        }
    }
    paths
}

fn rule_five(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for b in 0..graph.nodes() {
        for a in graph.nodes_into(b, Endpoint::Circle) {
            if graph.endpoint(b, a) != Some(Endpoint::Circle) {
                continue;
            }
            let a_candidates: Vec<_> = graph
                .neighbors(a)
                .into_iter()
                .filter(|&c| {
                    c != a
                        && c != b
                        && graph.endpoint_at(a, c) == Some(Endpoint::Circle)
                        && graph.endpoint_at(c, a) == Some(Endpoint::Circle)
                        && !graph.adjacent(b, c)
                })
                .collect();
            let b_candidates: Vec<_> = graph
                .neighbors(b)
                .into_iter()
                .filter(|&d| {
                    d != a
                        && d != b
                        && graph.endpoint_at(b, d) == Some(Endpoint::Circle)
                        && graph.endpoint_at(d, b) == Some(Endpoint::Circle)
                        && !graph.adjacent(a, d)
                })
                .collect();
            let mut found = Vec::new();
            for &c in &a_candidates {
                for &d in &b_candidates {
                    found.extend(uncovered_circle_paths(graph, c, d, &[a, b]));
                }
            }
            for path in found {
                graph.set_edge(a, b, Endpoint::Tail, Endpoint::Tail);
                graph.set_edge(a, path[0], Endpoint::Tail, Endpoint::Tail);
                graph.set_edge(
                    b,
                    *path.last().expect("R5 path"),
                    Endpoint::Tail,
                    Endpoint::Tail,
                );
                for pair in path.windows(2) {
                    graph.set_edge(pair[0], pair[1], Endpoint::Tail, Endpoint::Tail);
                }
                changed = true;
            }
        }
    }
    changed
}

fn rule_six(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for middle in 0..graph.nodes() {
        let has_tail_edge = graph
            .nodes_into(middle, Endpoint::Tail)
            .into_iter()
            .any(|left| graph.endpoint(middle, left) == Some(Endpoint::Tail));
        if !has_tail_edge {
            continue;
        }
        for right in graph.nodes_into(middle, Endpoint::Circle) {
            let at_right = graph.endpoint_at(right, middle).expect("R6 endpoint");
            graph.set_edge(middle, right, Endpoint::Tail, at_right);
            changed = true;
        }
    }
    changed
}

fn rule_seven(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for middle in 0..graph.nodes() {
        let circles = graph.nodes_into(middle, Endpoint::Circle);
        let tails: Vec<_> = circles
            .iter()
            .copied()
            .filter(|&left| graph.endpoint(middle, left) == Some(Endpoint::Tail))
            .collect();
        for &right in &circles {
            for &left in &tails {
                if left != right && !graph.adjacent(left, right) {
                    let at_right = graph.endpoint_at(right, middle).expect("R7 endpoint");
                    graph.set_edge(middle, right, Endpoint::Tail, at_right);
                    changed = true;
                }
            }
        }
    }
    changed
}

fn rule_eight(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for middle in 0..graph.nodes() {
        let adjacent = graph.neighbors(middle);
        for pair in combinations(&adjacent, 2) {
            let left = pair[0];
            let right = pair[1];
            let first_leg = graph.endpoint(left, middle) == Some(Endpoint::Arrow)
                && graph.endpoint(middle, left) == Some(Endpoint::Tail)
                || graph.endpoint(left, middle) == Some(Endpoint::Circle)
                    && graph.endpoint(middle, left) == Some(Endpoint::Tail);
            if first_leg
                && graph.endpoint(middle, right) == Some(Endpoint::Arrow)
                && graph.endpoint(right, middle) == Some(Endpoint::Tail)
                && graph.adjacent(left, right)
                && graph.endpoint(left, right) == Some(Endpoint::Arrow)
                && graph.endpoint(right, left) == Some(Endpoint::Circle)
            {
                graph.set_edge(left, right, Endpoint::Tail, Endpoint::Arrow);
                changed = true;
            }
        }
    }
    changed
}

fn possible_children(
    graph: &EndpointGraph,
    parent: usize,
    candidates: &[usize],
) -> BTreeSet<usize> {
    candidates
        .iter()
        .copied()
        .filter(|&child| {
            child != parent
                && graph.adjacent(parent, child)
                && graph.endpoint(child, parent) != Some(Endpoint::Arrow)
        })
        .collect()
}

fn rule_nine(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for destination in 0..graph.nodes() {
        for start in graph.nodes_into(destination, Endpoint::Arrow) {
            if graph.endpoint(destination, start) != Some(Endpoint::Circle) {
                continue;
            }
            let candidates: Vec<_> = graph
                .neighbors(start)
                .into_iter()
                .filter(|&node| node != start && node != destination)
                .collect();
            for next in possible_children(graph, start, &candidates) {
                if !graph.adjacent(next, destination)
                    && exists_uncovered_pd_path(graph, start, next, destination)
                {
                    graph.set_edge(start, destination, Endpoint::Tail, Endpoint::Arrow);
                    changed = true;
                    break;
                }
            }
        }
    }
    changed
}

fn rule_ten(graph: &mut EndpointGraph) -> bool {
    let mut changed = false;
    for destination in 0..graph.nodes() {
        let into = graph.nodes_into(destination, Endpoint::Arrow);
        if into.len() < 2 {
            continue;
        }
        let starts: Vec<_> = into
            .iter()
            .copied()
            .filter(|&start| graph.endpoint(destination, start) == Some(Endpoint::Circle))
            .collect();
        for start in starts {
            let neighbors: Vec<_> = graph
                .neighbors(start)
                .into_iter()
                .filter(|&node| node != destination)
                .collect();
            let children: Vec<_> = possible_children(graph, start, &neighbors)
                .into_iter()
                .collect();
            if children.len() < 2 {
                continue;
            }
            for pair in combinations(&into, 2) {
                let left_parent = pair[0];
                let right_parent = pair[1];
                if graph.endpoint(destination, left_parent) != Some(Endpoint::Tail)
                    || graph.endpoint(destination, right_parent) != Some(Endpoint::Tail)
                {
                    continue;
                }
                for child_pair in combinations(&children, 2) {
                    let left_child = child_pair[0];
                    let right_child = child_pair[1];
                    if exists_semi_directed_path(graph, left_child, left_parent)
                        && exists_semi_directed_path(graph, right_child, right_parent)
                        && !graph.adjacent(left_child, right_child)
                    {
                        graph.set_edge(start, destination, Endpoint::Tail, Endpoint::Arrow);
                        changed = true;
                        break;
                    }
                }
            }
        }
    }
    changed
}

fn visible_visit(
    graph: &EndpointGraph,
    c: usize,
    a: usize,
    b: usize,
    path: &mut Vec<usize>,
) -> bool {
    if path.contains(&a) {
        return false;
    }
    path.push(a);
    if a == b {
        return true;
    }
    for d in graph.nodes_into(a, Endpoint::Arrow) {
        if graph.is_parent(d, c) {
            return true;
        }
        if !graph.is_def_collider(d, c, a) || !graph.is_parent(c, b) {
            continue;
        }
        if visible_visit(graph, d, c, b, path) {
            return true;
        }
    }
    path.pop();
    false
}

fn visible_edge(graph: &EndpointGraph, cause: usize, effect: usize) -> bool {
    for candidate in graph.neighbors(cause) {
        if candidate != effect
            && !graph.adjacent(candidate, effect)
            && graph.endpoint(cause, candidate) == Some(Endpoint::Arrow)
        {
            return true;
        }
    }

    let mut path = vec![cause];
    for candidate in graph.nodes_into(cause, Endpoint::Arrow) {
        if graph.is_parent(candidate, cause)
            || visible_visit(graph, candidate, cause, effect, &mut path)
        {
            return true;
        }
    }
    false
}

fn color_edges(graph: &mut EndpointGraph) -> Vec<FciEdgeProperty> {
    let edges = graph.edges();
    edges
        .into_iter()
        .map(|edge| {
            let directed = matches!(
                (edge.left_endpoint, edge.right_endpoint),
                (Endpoint::Tail, Endpoint::Arrow) | (Endpoint::Arrow, Endpoint::Tail)
            );
            if !directed {
                return FciEdgeProperty {
                    left: edge.left,
                    right: edge.right,
                    directness: None,
                    latent_confounding: None,
                };
            }
            let (cause, effect) = if edge.left_endpoint == Endpoint::Tail {
                (edge.left, edge.right)
            } else {
                (edge.right, edge.left)
            };
            let saved = (edge.left_endpoint, edge.right_endpoint);
            graph.remove(edge.left, edge.right);
            let directness = if exists_semi_directed_path(graph, cause, effect) {
                Directness::PossiblyDirect
            } else {
                Directness::DefinitelyDirect
            };
            graph.set_edge(edge.left, edge.right, saved.0, saved.1);
            let latent_confounding = if visible_edge(graph, cause, effect) {
                LatentConfounding::Excluded
            } else {
                LatentConfounding::Possible
            };
            FciEdgeProperty {
                left: edge.left,
                right: edge.right,
                directness: Some(directness),
                latent_confounding: Some(latent_confounding),
            }
        })
        .collect()
}

#[cfg(test)]
mod orientation_tests {
    use super::*;

    fn graph(nodes: usize, edges: &[(usize, usize, Endpoint, Endpoint)]) -> EndpointGraph {
        let mut graph = EndpointGraph {
            node_names: (0..nodes).map(|node| node.to_string()).collect(),
            endpoints: vec![vec![0; nodes]; nodes],
        };
        for &(left, right, at_left, at_right) in edges {
            graph.set_edge(left, right, at_left, at_right);
        }
        graph
    }

    #[test]
    fn rules_one_two_and_three_match_endpoint_contract() {
        let background = BackgroundKnowledge::default();
        let mut one = graph(
            3,
            &[
                (0, 1, Endpoint::Circle, Endpoint::Arrow),
                (1, 2, Endpoint::Circle, Endpoint::Circle),
            ],
        );
        assert!(rule_one(&mut one, 0, 1, 2, &background));
        assert!(one.is_directed(1, 2));

        let mut two = graph(
            3,
            &[
                (0, 1, Endpoint::Tail, Endpoint::Arrow),
                (1, 2, Endpoint::Tail, Endpoint::Arrow),
                (0, 2, Endpoint::Circle, Endpoint::Circle),
            ],
        );
        assert!(rule_two(&mut two, 0, 1, 2, &background));
        assert_eq!(two.endpoint_at(0, 2), Some(Endpoint::Circle));
        assert_eq!(two.endpoint_at(2, 0), Some(Endpoint::Arrow));

        let mut three = graph(
            4,
            &[
                (0, 1, Endpoint::Circle, Endpoint::Arrow),
                (2, 1, Endpoint::Circle, Endpoint::Arrow),
                (3, 1, Endpoint::Circle, Endpoint::Circle),
                (0, 3, Endpoint::Circle, Endpoint::Circle),
                (2, 3, Endpoint::Circle, Endpoint::Circle),
            ],
        );
        let separating_sets = BTreeMap::from([((0, 2), BTreeSet::from([3]))]);
        assert!(rule_three(&mut three, &separating_sets, &background));
        assert_eq!(three.endpoint(3, 1), Some(Endpoint::Arrow));
    }

    #[test]
    fn rule_five_matches_causal_learn_reference_case() {
        let circle = Endpoint::Circle;
        let mut graph = graph(
            7,
            &[
                (0, 1, circle, circle),
                (0, 2, circle, circle),
                (0, 5, circle, circle),
                (0, 6, circle, circle),
                (1, 3, circle, circle),
                (2, 4, circle, circle),
                (3, 5, circle, circle),
                (4, 6, circle, circle),
            ],
        );
        assert!(rule_five(&mut graph));
        assert!(graph
            .edges()
            .iter()
            .all(|edge| edge.left_endpoint == Endpoint::Tail
                && edge.right_endpoint == Endpoint::Tail));
    }

    #[test]
    fn rules_six_seven_and_eight_orient_the_documented_endpoint() {
        let mut six = graph(
            3,
            &[
                (0, 1, Endpoint::Tail, Endpoint::Tail),
                (1, 2, Endpoint::Circle, Endpoint::Circle),
            ],
        );
        assert!(rule_six(&mut six));
        assert_eq!(six.endpoint_at(1, 2), Some(Endpoint::Tail));
        assert_eq!(six.endpoint_at(2, 1), Some(Endpoint::Circle));

        let mut seven = graph(
            3,
            &[
                (0, 1, Endpoint::Tail, Endpoint::Circle),
                (1, 2, Endpoint::Circle, Endpoint::Circle),
            ],
        );
        assert!(rule_seven(&mut seven));
        assert_eq!(seven.endpoint_at(1, 2), Some(Endpoint::Tail));

        let mut eight = graph(
            3,
            &[
                (0, 1, Endpoint::Tail, Endpoint::Arrow),
                (1, 2, Endpoint::Tail, Endpoint::Arrow),
                (0, 2, Endpoint::Circle, Endpoint::Arrow),
            ],
        );
        assert!(rule_eight(&mut eight));
        assert!(eight.is_directed(0, 2));
    }

    #[test]
    fn rules_nine_and_ten_complete_long_path_orientations() {
        let mut nine = graph(
            4,
            &[
                (0, 3, Endpoint::Circle, Endpoint::Arrow),
                (0, 1, Endpoint::Circle, Endpoint::Circle),
                (1, 2, Endpoint::Circle, Endpoint::Circle),
                (2, 3, Endpoint::Tail, Endpoint::Arrow),
            ],
        );
        assert!(rule_nine(&mut nine));
        assert!(nine.is_directed(0, 3));

        let mut ten = graph(
            7,
            &[
                (0, 6, Endpoint::Circle, Endpoint::Arrow),
                (1, 6, Endpoint::Tail, Endpoint::Arrow),
                (2, 6, Endpoint::Tail, Endpoint::Arrow),
                (0, 3, Endpoint::Circle, Endpoint::Circle),
                (0, 4, Endpoint::Circle, Endpoint::Circle),
                (3, 1, Endpoint::Tail, Endpoint::Arrow),
                (4, 2, Endpoint::Tail, Endpoint::Arrow),
            ],
        );
        assert!(rule_ten(&mut ten));
        assert!(ten.is_directed(0, 6));
    }
}
