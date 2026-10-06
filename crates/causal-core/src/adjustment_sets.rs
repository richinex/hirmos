//! Canonical and minimal total-effect adjustment sets for a single treatment and outcome in a DAG.
//!
//! The reduction follows the generalized adjustment criterion: remove the first edge of every
//! proper causal path, retain the relevant ancestor graph, moralize it, and enumerate minimal
//! admissible vertex separators. The separator enumeration is an independent Rust implementation
//! of Takata's 2010 algorithm. Dagitty is used as a parity oracle, not as copied source.

use crate::backdoor::Dag;
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjustmentSetAnalysis {
    Identified {
        /// All admissible observed ancestors. Valid whenever an admissible set exists.
        canonical: Vec<usize>,
        /// Inclusion-minimal valid sets, ordered deterministically.
        minimal: Vec<Vec<usize>>,
        /// More minimal sets exist than the caller's result budget permits returning.
        truncated: bool,
    },
    NotIdentified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdjustmentSetError {
    TooFewNodes,
    EndpointOutOfRange,
    SameEndpoint,
    UnobservedOutOfRange,
    ZeroMaxResults,
    AdjustmentOutOfRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuppliedAdjustment {
    Valid,
    Endpoints { nodes: Vec<usize> },
    Unobserved { nodes: Vec<usize> },
    ForbiddenDescendants { nodes: Vec<usize> },
    OpenNoncausalPath,
}

/// Validate a supplied set using the same proper-backdoor reduction as enumeration.
/// Unlike minimal-set enumeration, this also accepts valid nonminimal sets.
pub fn validate_adjustment_set(
    dag: &Dag,
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
    adjustment: &[usize],
) -> Result<SuppliedAdjustment, AdjustmentSetError> {
    if dag.n < 2 {
        return Err(AdjustmentSetError::TooFewNodes);
    }
    if treatment >= dag.n || outcome >= dag.n {
        return Err(AdjustmentSetError::EndpointOutOfRange);
    }
    if treatment == outcome {
        return Err(AdjustmentSetError::SameEndpoint);
    }
    if unobserved.iter().any(|&n| n >= dag.n) {
        return Err(AdjustmentSetError::UnobservedOutOfRange);
    }
    if adjustment.iter().any(|&n| n >= dag.n) {
        return Err(AdjustmentSetError::AdjustmentOutOfRange);
    }
    let selected = adjustment.iter().copied().collect::<BTreeSet<_>>();
    let endpoints = selected
        .iter()
        .copied()
        .filter(|&n| n == treatment || n == outcome)
        .collect::<Vec<_>>();
    if !endpoints.is_empty() {
        return Ok(SuppliedAdjustment::Endpoints { nodes: endpoints });
    }
    let latent = selected
        .iter()
        .copied()
        .filter(|n| unobserved.contains(n))
        .collect::<Vec<_>>();
    if !latent.is_empty() {
        return Ok(SuppliedAdjustment::Unobserved { nodes: latent });
    }
    let causal = proper_causal_path_nodes(dag, treatment, outcome);
    let mut without_treatment = causal.clone();
    without_treatment.remove(&treatment);
    let forbidden = inclusive_descendants(dag, &without_treatment);
    let nodes = selected
        .intersection(&forbidden)
        .copied()
        .collect::<Vec<_>>();
    if !nodes.is_empty() {
        return Ok(SuppliedAdjustment::ForbiddenDescendants { nodes });
    }
    let backdoor = proper_backdoor_graph(dag, treatment, &causal);
    Ok(if backdoor.d_separated(treatment, outcome, &selected) {
        SuppliedAdjustment::Valid
    } else {
        SuppliedAdjustment::OpenNoncausalPath
    })
}

#[derive(Debug, Clone)]
struct UndirectedGraph {
    present: BTreeSet<usize>,
    neighbours: Vec<BTreeSet<usize>>,
}

impl UndirectedGraph {
    fn add_edge(&mut self, a: usize, b: usize) {
        if a == b || !self.present.contains(&a) || !self.present.contains(&b) {
            return;
        }
        self.neighbours[a].insert(b);
        self.neighbours[b].insert(a);
    }

    fn real_neighbours(
        &self,
        vertices: &BTreeSet<usize>,
        forbidden: &BTreeSet<usize>,
    ) -> BTreeSet<usize> {
        let mut visited = vertices.clone();
        let mut queue: VecDeque<usize> = vertices.iter().copied().collect();
        let mut boundary = BTreeSet::new();
        while let Some(node) = queue.pop_front() {
            for &next in &self.neighbours[node] {
                if !visited.insert(next) {
                    continue;
                }
                if forbidden.contains(&next) {
                    queue.push_back(next);
                } else {
                    boundary.insert(next);
                }
            }
        }
        boundary
    }

    fn component_avoiding(
        &self,
        starts: &BTreeSet<usize>,
        avoided: &BTreeSet<usize>,
    ) -> BTreeSet<usize> {
        let mut visited = avoided.clone();
        let mut component = BTreeSet::new();
        let mut queue = VecDeque::new();
        for &start in starts {
            if self.present.contains(&start) {
                visited.insert(start);
                component.insert(start);
                queue.push_back(start);
            }
        }
        while let Some(node) = queue.pop_front() {
            for &next in &self.neighbours[node] {
                if visited.insert(next) {
                    component.insert(next);
                    queue.push_back(next);
                }
            }
        }
        component
    }

    fn near_separator(
        &self,
        source_side: &BTreeSet<usize>,
        target: usize,
        forbidden: &BTreeSet<usize>,
    ) -> BTreeSet<usize> {
        let source_boundary = self.real_neighbours(source_side, forbidden);
        let target_component = self.component_avoiding(&BTreeSet::from([target]), &source_boundary);
        self.real_neighbours(&target_component, forbidden)
    }
}

fn inclusive_descendants(dag: &Dag, starts: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut result = starts.clone();
    let mut stack: Vec<usize> = starts.iter().copied().collect();
    while let Some(node) = stack.pop() {
        for &child in &dag.children[node] {
            if result.insert(child) {
                stack.push(child);
            }
        }
    }
    result
}

fn proper_causal_path_nodes(dag: &Dag, treatment: usize, outcome: usize) -> BTreeSet<usize> {
    let mut descendants = dag.descendants(treatment);
    descendants.insert(treatment);
    let ancestors = dag.ancestors_of(&[outcome]);
    descendants.intersection(&ancestors).copied().collect()
}

fn proper_backdoor_graph(dag: &Dag, treatment: usize, causal_path_nodes: &BTreeSet<usize>) -> Dag {
    let edges = dag
        .children
        .iter()
        .enumerate()
        .flat_map(|(cause, children)| {
            children.iter().copied().filter_map(move |effect| {
                if cause == treatment && causal_path_nodes.contains(&effect) {
                    None
                } else {
                    Some((cause, effect))
                }
            })
        })
        .collect::<Vec<_>>();
    Dag::new(dag.n, &edges)
}

fn moral_ancestor_graph(dag: &Dag, treatment: usize, outcome: usize) -> UndirectedGraph {
    let present = dag.ancestors_of(&[treatment, outcome]);
    let mut graph = UndirectedGraph {
        present: present.clone(),
        neighbours: vec![BTreeSet::new(); dag.n],
    };
    for &node in &present {
        for &child in &dag.children[node] {
            if present.contains(&child) {
                graph.add_edge(node, child);
            }
        }
        let parents = dag.parents[node]
            .iter()
            .copied()
            .filter(|parent| present.contains(parent))
            .collect::<Vec<_>>();
        for left in 0..parents.len() {
            for right in left + 1..parents.len() {
                graph.add_edge(parents[left], parents[right]);
            }
        }
    }
    graph
}

struct SeparatorSearch<'a> {
    graph: &'a UndirectedGraph,
    source: usize,
    target: usize,
    forbidden: &'a BTreeSet<usize>,
    budget: usize,
    results: BTreeSet<Vec<usize>>,
}

impl SeparatorSearch<'_> {
    fn visit(&mut self, source_side: BTreeSet<usize>, excluded: BTreeSet<usize>) {
        if self.results.len() > self.budget {
            return;
        }
        let separator = self
            .graph
            .near_separator(&source_side, self.target, self.forbidden);
        let source_component = self
            .graph
            .component_avoiding(&BTreeSet::from([self.source]), &separator);
        if source_component.iter().any(|node| excluded.contains(node)) {
            return;
        }
        let boundary = self
            .graph
            .real_neighbours(&source_component, self.forbidden);
        if let Some(candidate) = boundary
            .iter()
            .find(|node| !excluded.contains(node))
            .copied()
        {
            let mut expanded_source = source_component.clone();
            expanded_source.insert(candidate);
            self.visit(expanded_source, excluded.clone());

            let mut expanded_excluded = excluded;
            expanded_excluded.insert(candidate);
            self.visit(source_component, expanded_excluded);
        } else {
            let mut set = separator.into_iter().collect::<Vec<_>>();
            set.sort_unstable();
            self.results.insert(set);
        }
    }
}

fn minimal_separators(
    graph: &UndirectedGraph,
    treatment: usize,
    outcome: usize,
    forbidden: &BTreeSet<usize>,
    max_results: usize,
) -> (Vec<Vec<usize>>, bool) {
    let mut initially_excluded = BTreeSet::from([outcome]);
    initially_excluded.extend(graph.neighbours[outcome].iter().copied());
    let mut search = SeparatorSearch {
        graph,
        source: treatment,
        target: outcome,
        forbidden,
        budget: max_results,
        results: BTreeSet::new(),
    };
    search.visit(BTreeSet::from([treatment]), initially_excluded);
    let truncated = search.results.len() > max_results;
    let mut results = search.results.into_iter().collect::<Vec<_>>();
    results.sort_by(|left, right| left.len().cmp(&right.len()).then_with(|| left.cmp(right)));
    results.truncate(max_results);
    (results, truncated)
}

/// Enumerate Dagitty-compatible canonical and inclusion-minimal total-effect adjustment sets.
pub fn dagitty_adjustment_sets(
    dag: &Dag,
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
    max_results: usize,
) -> Result<AdjustmentSetAnalysis, AdjustmentSetError> {
    if dag.n < 2 {
        return Err(AdjustmentSetError::TooFewNodes);
    }
    if treatment >= dag.n || outcome >= dag.n {
        return Err(AdjustmentSetError::EndpointOutOfRange);
    }
    if treatment == outcome {
        return Err(AdjustmentSetError::SameEndpoint);
    }
    if unobserved.iter().any(|&node| node >= dag.n) {
        return Err(AdjustmentSetError::UnobservedOutOfRange);
    }
    if max_results == 0 {
        return Err(AdjustmentSetError::ZeroMaxResults);
    }

    let causal_path_nodes = proper_causal_path_nodes(dag, treatment, outcome);
    let mut path_nodes_without_treatment = causal_path_nodes.clone();
    path_nodes_without_treatment.remove(&treatment);
    let descendants_of_causal_path = inclusive_descendants(dag, &path_nodes_without_treatment);
    let proper_backdoor = proper_backdoor_graph(dag, treatment, &causal_path_nodes);

    let empty = BTreeSet::new();
    if proper_backdoor.d_separated(treatment, outcome, &empty) {
        return Ok(AdjustmentSetAnalysis::Identified {
            canonical: Vec::new(),
            minimal: vec![Vec::new()],
            truncated: false,
        });
    }

    let latent = unobserved.iter().copied().collect::<BTreeSet<_>>();
    let mut forbidden = descendants_of_causal_path;
    forbidden.extend(latent.iter().copied());

    let ancestors = dag.ancestors_of(&[treatment, outcome]);
    let canonical = ancestors
        .into_iter()
        .filter(|node| {
            *node != treatment
                && *node != outcome
                && !latent.contains(node)
                && !forbidden.contains(node)
        })
        .collect::<Vec<_>>();
    let canonical_set = canonical.iter().copied().collect::<BTreeSet<_>>();
    if !proper_backdoor.d_separated(treatment, outcome, &canonical_set) {
        return Ok(AdjustmentSetAnalysis::NotIdentified);
    }

    let moral = moral_ancestor_graph(&proper_backdoor, treatment, outcome);
    let (minimal, truncated) =
        minimal_separators(&moral, treatment, outcome, &forbidden, max_results);
    if minimal.is_empty() {
        return Ok(AdjustmentSetAnalysis::NotIdentified);
    }
    Ok(AdjustmentSetAnalysis::Identified {
        canonical,
        minimal,
        truncated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extended_confounding_triangle_has_two_minimal_sets() {
        // A -> E, A -> Z, B -> D, B -> Z, E -> D, Z -> D, Z -> E.
        let dag = Dag::new(5, &[(0, 3), (0, 4), (1, 2), (1, 4), (3, 2), (4, 2), (4, 3)]);
        let result = dagitty_adjustment_sets(&dag, 3, 2, &[], 32).unwrap();
        assert_eq!(
            result,
            AdjustmentSetAnalysis::Identified {
                canonical: vec![0, 1, 4],
                minimal: vec![vec![0, 4], vec![1, 4]],
                truncated: false,
            }
        );
    }

    #[test]
    fn latent_confounding_is_not_identified() {
        let dag = Dag::new(3, &[(2, 0), (2, 1), (0, 1)]);
        assert_eq!(
            dagitty_adjustment_sets(&dag, 0, 1, &[2], 32).unwrap(),
            AdjustmentSetAnalysis::NotIdentified
        );
    }

    #[test]
    fn no_open_backdoor_path_returns_the_empty_minimal_set() {
        let dag = Dag::new(3, &[(0, 2), (2, 1)]);
        assert_eq!(
            dagitty_adjustment_sets(&dag, 0, 1, &[], 32).unwrap(),
            AdjustmentSetAnalysis::Identified {
                canonical: vec![],
                minimal: vec![vec![]],
                truncated: false,
            }
        );
    }

    #[test]
    fn result_budget_records_truncation() {
        let dag = Dag::new(5, &[(0, 3), (0, 4), (1, 2), (1, 4), (3, 2), (4, 2), (4, 3)]);
        let result = dagitty_adjustment_sets(&dag, 3, 2, &[], 1).unwrap();
        match result {
            AdjustmentSetAnalysis::Identified {
                minimal, truncated, ..
            } => {
                assert_eq!(minimal.len(), 1);
                assert!(truncated);
            }
            AdjustmentSetAnalysis::NotIdentified => panic!("triangle is identified"),
        }
    }

    #[test]
    fn separator_enumeration_matches_exhaustive_search_on_small_dags() {
        let n = 4usize;
        let possible_edges = (0..n)
            .flat_map(|cause| (cause + 1..n).map(move |effect| (cause, effect)))
            .collect::<Vec<_>>();
        for edge_mask in 0usize..(1usize << possible_edges.len()) {
            let edges = possible_edges
                .iter()
                .enumerate()
                .filter_map(|(bit, edge)| ((edge_mask >> bit) & 1 == 1).then_some(*edge))
                .collect::<Vec<_>>();
            let dag = Dag::new(n, &edges);
            for treatment in 0..n {
                for outcome in 0..n {
                    if treatment == outcome {
                        continue;
                    }
                    let remaining = (0..n)
                        .filter(|node| *node != treatment && *node != outcome)
                        .collect::<Vec<_>>();
                    for latent_mask in 0usize..(1usize << remaining.len()) {
                        let unobserved = remaining
                            .iter()
                            .enumerate()
                            .filter_map(|(bit, node)| {
                                ((latent_mask >> bit) & 1 == 1).then_some(*node)
                            })
                            .collect::<Vec<_>>();
                        let causal = proper_causal_path_nodes(&dag, treatment, outcome);
                        let mut without_treatment = causal.clone();
                        without_treatment.remove(&treatment);
                        let mut forbidden = inclusive_descendants(&dag, &without_treatment);
                        forbidden.extend(unobserved.iter().copied());
                        let proper = proper_backdoor_graph(&dag, treatment, &causal);
                        let eligible = remaining
                            .iter()
                            .copied()
                            .filter(|node| !forbidden.contains(node))
                            .collect::<Vec<_>>();
                        let valid = (0usize..(1usize << eligible.len()))
                            .filter_map(|mask| {
                                let set = eligible
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(bit, node)| {
                                        ((mask >> bit) & 1 == 1).then_some(*node)
                                    })
                                    .collect::<BTreeSet<_>>();
                                proper.d_separated(treatment, outcome, &set).then_some(set)
                            })
                            .collect::<Vec<_>>();
                        let mut expected = valid
                            .iter()
                            .filter(|set| {
                                !valid
                                    .iter()
                                    .any(|other| other.len() < set.len() && other.is_subset(set))
                            })
                            .map(|set| set.iter().copied().collect::<Vec<_>>())
                            .collect::<Vec<_>>();
                        expected.sort_by(|left, right| {
                            left.len().cmp(&right.len()).then_with(|| left.cmp(right))
                        });

                        let actual =
                            dagitty_adjustment_sets(&dag, treatment, outcome, &unobserved, 64)
                                .unwrap();
                        match actual {
                            AdjustmentSetAnalysis::NotIdentified => assert!(
                                expected.is_empty(),
                                "edges={edges:?}, treatment={treatment}, outcome={outcome}, latent={unobserved:?}"
                            ),
                            AdjustmentSetAnalysis::Identified {
                                minimal,
                                truncated,
                                ..
                            } => {
                                assert!(!truncated);
                                assert_eq!(
                                    minimal, expected,
                                    "edges={edges:?}, treatment={treatment}, outcome={outcome}, latent={unobserved:?}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
