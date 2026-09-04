//! PC-stable with causal-learn's default `uc_sepset` collider rule and Meek closure.
//!
//! Ported from causal-learn commit `9de1d886b7ab45868819aec7132f79378990f0b6`.

use crate::constraint_discovery::{
    orient_background, stable_adjacency_search, BackgroundKnowledge, CiEngine, CiRecord,
    ConstraintDiscoveryError, CrossSectionalCi, Endpoint, EndpointGraph, SeparatingSets,
};
use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PcStableConfiguration {
    pub alpha: f64,
    pub max_depth: Option<usize>,
    pub ci_test: CrossSectionalCi,
}

impl Default for PcStableConfiguration {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            max_depth: None,
            ci_test: CrossSectionalCi::FisherZ,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PcStableResult {
    pub graph: EndpointGraph,
    pub separating_sets: Vec<SeparatedPair>,
    pub ci_tests: Vec<CiRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeparatedPair {
    pub x: usize,
    pub y: usize,
    pub variables: Vec<usize>,
}

pub fn run_pc_stable(
    data: &DMatrix<f64>,
    node_names: Vec<String>,
    configuration: PcStableConfiguration,
    background: &BackgroundKnowledge,
) -> Result<PcStableResult, ConstraintDiscoveryError> {
    run_pc_stable_with_progress(data, node_names, configuration, background, |_, _, _| {})
}

pub fn run_pc_stable_with_progress<F>(
    data: &DMatrix<f64>,
    node_names: Vec<String>,
    configuration: PcStableConfiguration,
    background: &BackgroundKnowledge,
    mut progress: F,
) -> Result<PcStableResult, ConstraintDiscoveryError>
where
    F: FnMut(&'static str, usize, usize),
{
    let mut ci = CiEngine::new(data, configuration.ci_test)?;
    let (mut graph, separating_sets) = stable_adjacency_search(
        node_names,
        configuration.alpha,
        configuration.max_depth,
        background,
        &mut ci,
    )?;
    progress("skeleton", 1, 3);
    orient_background(&mut graph, background);
    orient_unshielded_colliders(&mut graph, &separating_sets, background);
    progress("colliders", 2, 3);
    meek_closure(&mut graph, background);
    progress("orientation", 3, 3);
    Ok(PcStableResult {
        graph,
        separating_sets: public_separating_sets(&separating_sets),
        ci_tests: ci.records,
    })
}

pub(crate) fn public_separating_sets(sets: &SeparatingSets) -> Vec<SeparatedPair> {
    sets.iter()
        .filter(|((x, y), _)| x < y)
        .map(|(&(x, y), values)| SeparatedPair {
            x,
            y,
            variables: values.iter().copied().collect(),
        })
        .collect()
}

fn background_allows(
    graph: &EndpointGraph,
    from: usize,
    to: usize,
    background: &BackgroundKnowledge,
) -> bool {
    !background.is_forbidden(from, to, &graph.node_names)
        && !background.is_required(to, from, &graph.node_names)
}

fn unshielded_triples(graph: &EndpointGraph) -> Vec<(usize, usize, usize)> {
    let mut triples = Vec::new();
    for middle in 0..graph.nodes() {
        let neighbors = graph.neighbors(middle);
        for &left in &neighbors {
            for &right in &neighbors {
                if left != right && !graph.adjacent(left, right) {
                    triples.push((left, middle, right));
                }
            }
        }
    }
    triples
}

fn triangles(graph: &EndpointGraph) -> Vec<(usize, usize, usize)> {
    let mut triples = Vec::new();
    for middle in 0..graph.nodes() {
        for left in 0..graph.nodes() {
            if left == middle || !graph.adjacent(left, middle) {
                continue;
            }
            for right in 0..graph.nodes() {
                if right != left
                    && right != middle
                    && graph.adjacent(middle, right)
                    && graph.adjacent(left, right)
                {
                    triples.push((left, middle, right));
                }
            }
        }
    }
    triples
}

fn kites(graph: &EndpointGraph) -> Vec<(usize, usize, usize, usize)> {
    let mut values = Vec::new();
    for center in 0..graph.nodes() {
        for left in 0..graph.nodes() {
            if left == center || !graph.adjacent(center, left) {
                continue;
            }
            for right in left + 1..graph.nodes() {
                if right == center || !graph.adjacent(center, right) || graph.adjacent(left, right)
                {
                    continue;
                }
                for tip in 0..graph.nodes() {
                    if tip == center || tip == left || tip == right {
                        continue;
                    }
                    if graph.adjacent(center, tip)
                        && graph.adjacent(left, tip)
                        && graph.adjacent(right, tip)
                    {
                        values.push((center, left, right, tip));
                    }
                }
            }
        }
    }
    values
}

fn orient_unshielded_colliders(
    graph: &mut EndpointGraph,
    separating_sets: &SeparatingSets,
    background: &BackgroundKnowledge,
) {
    for (left, middle, right) in unshielded_triples(graph)
        .into_iter()
        .filter(|(left, _, right)| left < right)
    {
        if !background_allows(graph, left, middle, background)
            || !background_allows(graph, right, middle, background)
            || separating_sets
                .get(&(left, right))
                .is_some_and(|set| set.contains(&middle))
        {
            continue;
        }
        // causal-learn priority 2: do not overwrite either opposing arrow.
        if !graph.is_directed(middle, left) && !graph.is_directed(middle, right) {
            graph.set_edge(left, middle, Endpoint::Tail, Endpoint::Arrow);
            graph.set_edge(right, middle, Endpoint::Tail, Endpoint::Arrow);
        }
    }
}

fn meek_closure(graph: &mut EndpointGraph, background: &BackgroundKnowledge) {
    let unshielded = unshielded_triples(graph);
    let triangles = triangles(graph);
    let kites = kites(graph);
    loop {
        let mut changed = false;
        for &(left, middle, right) in &unshielded {
            if graph.is_directed(left, middle)
                && graph.is_undirected(middle, right)
                && background_allows(graph, middle, right, background)
                && !graph.directed_path(right, middle)
            {
                graph.set_edge(middle, right, Endpoint::Tail, Endpoint::Arrow);
                changed = true;
            }
        }
        for &(left, middle, right) in &triangles {
            if graph.is_directed(left, middle)
                && graph.is_directed(middle, right)
                && graph.is_undirected(left, right)
                && background_allows(graph, left, right, background)
                && !graph.directed_path(right, left)
            {
                graph.set_edge(left, right, Endpoint::Tail, Endpoint::Arrow);
                changed = true;
            }
        }
        for &(center, left, right, tip) in &kites {
            if graph.is_undirected(center, left)
                && graph.is_undirected(center, right)
                && graph.is_directed(left, tip)
                && graph.is_directed(right, tip)
                && graph.is_undirected(center, tip)
                && background_allows(graph, center, tip, background)
                && !graph.directed_path(tip, center)
            {
                graph.set_edge(center, tip, Endpoint::Tail, Endpoint::Arrow);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}
