//! Ordinary SWIG foundation for Knaus & Pfleiderer (2026), Section 2.3.
//! Fixed nodes supply intervention values; random treatment nodes retain natural
//! assignment. This module does not itself establish conditional parallel trends.
use crate::backdoor::Dag;
use std::collections::{BTreeMap, BTreeSet};

pub mod did;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Variable(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    Endogenous,
    Exogenous,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    EmptyGraph,
    UnknownVariable,
    DuplicateEdge,
    Cycle,
    ExogenousHasParents,
    DuplicateIntervention,
    InvalidIntervention,
    InvalidQuery,
    InvalidEquation,
    ConflictingMechanism,
    WrongInterventionWorld,
}

/// Only the constructor can establish the graph invariants.
pub struct CausalDag {
    graph: Dag,
    roles: Vec<Role>,
}

impl CausalDag {
    pub fn new(roles: Vec<Role>, edges: &[(Variable, Variable)]) -> Result<Self, Error> {
        if roles.is_empty() {
            return Err(Error::EmptyGraph);
        }
        let n = roles.len();
        let mut seen = BTreeSet::new();
        for &(Variable(a), Variable(b)) in edges {
            if a >= n || b >= n {
                return Err(Error::UnknownVariable);
            }
            if !seen.insert((a, b)) {
                return Err(Error::DuplicateEdge);
            }
            if roles[b] == Role::Exogenous {
                return Err(Error::ExogenousHasParents);
            }
        }
        let graph = Dag::new(n, &seen.into_iter().collect::<Vec<_>>());
        let mut degrees: Vec<_> = graph.parents.iter().map(Vec::len).collect();
        let mut ready: Vec<_> = (0..n).filter(|&i| degrees[i] == 0).collect();
        let mut visited = 0;
        while let Some(i) = ready.pop() {
            visited += 1;
            for &child in &graph.children[i] {
                degrees[child] -= 1;
                if degrees[child] == 0 {
                    ready.push(child);
                }
            }
        }
        if visited != n {
            return Err(Error::Cycle);
        }
        Ok(Self { graph, roles })
    }

    pub fn intervene(&self, interventions: &[(Variable, f64)]) -> Result<Swig, Error> {
        let mut settings = BTreeMap::new();
        for &(Variable(i), value) in interventions {
            if i >= self.graph.n {
                return Err(Error::UnknownVariable);
            }
            if self.roles[i] != Role::Endogenous || !value.is_finite() {
                return Err(Error::InvalidIntervention);
            }
            if settings.insert(i, value).is_some() {
                return Err(Error::DuplicateIntervention);
            }
        }
        let n = self.graph.n;
        let fixed: BTreeMap<_, _> = settings
            .keys()
            .enumerate()
            .map(|(k, &i)| (i, n + k))
            .collect();
        let mut edges = Vec::new();
        for (a, children) in self.graph.children.iter().enumerate() {
            for &b in children {
                edges.push((fixed.get(&a).copied().unwrap_or(a), b));
            }
        }
        let graph = Dag::new(n + fixed.len(), &edges);
        let mut nodes: Vec<_> = (0..n)
            .map(|i| Node::Random {
                original: Variable(i),
                role: self.roles[i],
                interventions: Vec::new(),
            })
            .collect();
        for (&i, &value) in &settings {
            nodes.push(Node::Fixed {
                original: Variable(i),
                value,
            });
        }
        // Use the transformed graph: an intervened mediator cuts propagation
        // to its children from upstream natural assignment.
        for (&treatment, &index) in &fixed {
            for descendant in graph.descendants(index) {
                if let Node::Random { interventions, .. } = &mut nodes[descendant] {
                    interventions.push(Variable(treatment));
                }
            }
        }
        Ok(Swig {
            graph,
            nodes,
            originals: n,
            equations: BTreeMap::new(),
            terms: BTreeMap::new(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Random {
        original: Variable,
        role: Role,
        interventions: Vec<Variable>,
    },
    Fixed {
        original: Variable,
        value: f64,
    },
    Difference {
        later: Variable,
        earlier: Variable,
        cancelled_terms: Vec<String>,
    },
}

/// A declared additive function applied to an ordered list of random arguments.
/// Reusing its identity asserts exact equality of functions, including coefficients.
/// This is an SCM assumption, not something inferred or verified from data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdditiveTerm {
    pub identity: String,
    pub arguments: Vec<Variable>,
}

/// Unit-coefficient sum of named terms in the explicitly selected intervention world.
/// Distinct identities never cancel, even if their argument lists are identical.
#[derive(Clone, Debug)]
pub struct Equation {
    pub outcome: Variable,
    pub terms: Vec<AdditiveTerm>,
}

/// Availability of an original variable's random node, not a claim that a
/// conditioning set identifies an effect. Consistency only substitutes observed
/// values for units whose actual assignments match all listed interventions.
#[derive(Clone, Debug, PartialEq)]
pub enum CovariateAvailability {
    Observed,
    UnderConsistency { assignments: Vec<(Variable, f64)> },
    Unmeasured,
}

/// A requirement that cannot be satisfied from this group's observed columns.
#[derive(Clone, Debug, PartialEq)]
pub enum AvailabilityBlocker {
    Unmeasured {
        variable: Variable,
    },
    UnestablishedAssignment {
        variable: Variable,
        treatment: Variable,
        required: f64,
    },
    ConflictingAssignment {
        variable: Variable,
        treatment: Variable,
        required: f64,
        actual: f64,
    },
}

/// Consistency-based availability only. Neither variant asserts parallel trends,
/// overlap, or identification. Group assignments are supplied assumptions about
/// actual treatment histories, not additional interventions on the graph.
#[derive(Clone, Debug, PartialEq)]
pub enum ConditioningAvailability {
    Available,
    Unavailable { blockers: Vec<AvailabilityBlocker> },
}

pub struct Swig {
    graph: Dag,
    nodes: Vec<Node>,
    originals: usize,
    equations: BTreeMap<Variable, BTreeMap<String, Vec<Variable>>>,
    terms: BTreeMap<String, Vec<Variable>>,
}

/// Query-preserving induced ancestor graph. Public queries retain original node
/// indices; removed nodes are rejected rather than silently reinterpreted.
pub struct PrunedSwig {
    graph: Dag,
    retained: Vec<usize>,
    fixed: BTreeSet<usize>,
}

impl PrunedSwig {
    pub fn retained_nodes(&self) -> &[usize] {
        &self.retained
    }
    pub fn separated_nodes(&self, x: usize, y: usize, given: &[usize]) -> Result<bool, Error> {
        let index = |v| {
            self.retained
                .binary_search(&v)
                .map_err(|_| Error::UnknownVariable)
        };
        let x = index(x)?;
        let y = index(y)?;
        if x == y || self.fixed.contains(&x) || self.fixed.contains(&y) {
            return Err(Error::InvalidQuery);
        }
        let mut z = self.fixed.clone();
        for &v in given {
            let v = index(v)?;
            if v == x || v == y || !z.insert(v) {
                return Err(Error::InvalidQuery);
            }
        }
        Ok(self.graph.d_separated(x, y, &z))
    }
}

impl Swig {
    /// Check a whole conditioning set for a group with a declared actual history.
    /// Section 5.3.1: X2(0) is observable for late adopters with D1=0, but not
    /// for early adopters with D1=1. Missing history is not assumed to be zero.
    pub fn conditioning_availability(
        &self,
        variables: &[Variable],
        measured: &[Variable],
        group_assignments: &[(Variable, f64)],
    ) -> Result<ConditioningAvailability, Error> {
        if variables
            .iter()
            .chain(measured)
            .any(|v| v.0 >= self.originals)
        {
            return Err(Error::UnknownVariable);
        }
        let selected: BTreeSet<_> = variables.iter().copied().collect();
        if selected.len() != variables.len()
            || measured.iter().collect::<BTreeSet<_>>().len() != measured.len()
        {
            return Err(Error::InvalidQuery);
        }
        let mut history = BTreeMap::new();
        for &(treatment, actual) in group_assignments {
            if treatment.0 >= self.originals {
                return Err(Error::UnknownVariable);
            }
            if !actual.is_finite()
                || !self.nodes.iter().any(|node| {
                    matches!(node,
                    Node::Fixed { original, .. } if *original == treatment)
                })
                || history.insert(treatment, actual).is_some()
            {
                return Err(Error::InvalidQuery);
            }
        }
        let mut blockers = Vec::new();
        for variable in selected {
            match self.covariate_availability(variable, measured)? {
                CovariateAvailability::Observed => {}
                CovariateAvailability::Unmeasured => {
                    blockers.push(AvailabilityBlocker::Unmeasured { variable });
                }
                CovariateAvailability::UnderConsistency { assignments } => {
                    for (treatment, required) in assignments {
                        match history.get(&treatment) {
                            Some(&actual) if actual == required => {}
                            Some(&actual) => {
                                blockers.push(AvailabilityBlocker::ConflictingAssignment {
                                    variable,
                                    treatment,
                                    required,
                                    actual,
                                })
                            }
                            None => blockers.push(AvailabilityBlocker::UnestablishedAssignment {
                                variable,
                                treatment,
                                required,
                            }),
                        }
                    }
                }
            }
        }
        Ok(if blockers.is_empty() {
            ConditioningAvailability::Available
        } else {
            ConditioningAvailability::Unavailable { blockers }
        })
    }

    pub fn covariate_availability(
        &self,
        variable: Variable,
        measured: &[Variable],
    ) -> Result<CovariateAvailability, Error> {
        if variable.0 >= self.originals || measured.iter().any(|v| v.0 >= self.originals) {
            return Err(Error::UnknownVariable);
        }
        let observed: BTreeSet<_> = measured.iter().copied().collect();
        if observed.len() != measured.len() {
            return Err(Error::InvalidQuery);
        }
        if !observed.contains(&variable) {
            return Ok(CovariateAvailability::Unmeasured);
        }
        let Node::Random { interventions, .. } = &self.nodes[variable.0] else {
            unreachable!()
        };
        if interventions.is_empty() {
            return Ok(CovariateAvailability::Observed);
        }
        let assignments = self
            .nodes
            .iter()
            .filter_map(|n| match n {
                Node::Fixed { original, value } if interventions.contains(original) => {
                    Some((*original, *value))
                }
                _ => None,
            })
            .collect();
        Ok(CovariateAvailability::UnderConsistency { assignments })
    }
    /// Procedure 1 step 8: iteratively discard irrelevant sinks by retaining
    /// ancestors of every node of interest (including any intended conditioners).
    /// Explicit disturbances are retained; this does not perform latent projection.
    pub fn prune_for(&self, interest: &[usize]) -> Result<PrunedSwig, Error> {
        if interest.is_empty() {
            return Err(Error::InvalidQuery);
        }
        let mut keep = BTreeSet::new();
        for &i in interest {
            if i >= self.nodes.len() {
                return Err(Error::UnknownVariable);
            }
            if matches!(self.nodes[i], Node::Fixed { .. }) || !keep.insert(i) {
                return Err(Error::InvalidQuery);
            }
        }
        keep.extend(
            self.nodes
                .iter()
                .enumerate()
                .filter_map(|(i, n)| matches!(n, Node::Fixed { .. }).then_some(i)),
        );
        let retained: Vec<_> = self
            .graph
            .ancestors_of(&keep.into_iter().collect::<Vec<_>>())
            .into_iter()
            .collect();
        let mut edges = Vec::new();
        for (a, b) in self.edges() {
            if let (Ok(a), Ok(b)) = (retained.binary_search(&a), retained.binary_search(&b)) {
                edges.push((a, b));
            }
        }
        let fixed = retained
            .iter()
            .enumerate()
            .filter_map(|(j, &i)| matches!(self.nodes[i], Node::Fixed { .. }).then_some(j))
            .collect();
        Ok(PrunedSwig {
            graph: Dag::new(retained.len(), &edges),
            retained,
            fixed,
        })
    }
    /// Procedure 1 step 6 / Appendix B.2, for declared additive decompositions.
    /// Returns the new node's index. Inputs describe equations AFTER intervention.
    /// No level-to-difference edges are added: the difference inherits surviving
    /// random parents of the levels, including explicitly represented disturbances.
    pub fn add_difference(
        &mut self,
        later: Equation,
        earlier: Equation,
        world: &[(Variable, f64)],
    ) -> Result<usize, Error> {
        let actual: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|n| match n {
                Node::Fixed { original, value } => Some((*original, *value)),
                _ => None,
            })
            .collect();
        let mut requested = world.to_vec();
        requested.sort_by_key(|(v, _)| *v);
        if requested != actual {
            return Err(Error::WrongInterventionWorld);
        }
        if later.outcome == earlier.outcome {
            return Err(Error::InvalidEquation);
        }
        let mut known = self.terms.clone();
        let mut equations = self.equations.clone();
        let mut normalized = Vec::new();
        for equation in [&later, &earlier] {
            let i = equation.outcome.0;
            if i >= self.originals {
                return Err(Error::UnknownVariable);
            }
            if !matches!(
                self.nodes[i],
                Node::Random {
                    role: Role::Endogenous,
                    ..
                }
            ) {
                return Err(Error::InvalidEquation);
            }
            let parents: BTreeSet<_> = self.graph.parents[i]
                .iter()
                .copied()
                .filter(|&p| !matches!(self.nodes[p], Node::Fixed { .. }))
                .map(Variable)
                .collect();
            let mut used = BTreeSet::new();
            let mut terms = BTreeMap::new();
            for term in &equation.terms {
                if term.identity.trim().is_empty() || terms.contains_key(&term.identity) {
                    return Err(Error::InvalidEquation);
                }
                if term.arguments.iter().any(|p| !parents.contains(p)) {
                    return Err(Error::InvalidEquation);
                }
                if let Some(args) = known.get(&term.identity) {
                    if args != &term.arguments {
                        return Err(Error::ConflictingMechanism);
                    }
                }
                used.extend(term.arguments.iter().copied());
                known.insert(term.identity.clone(), term.arguments.clone());
                terms.insert(term.identity.clone(), term.arguments.clone());
            }
            // Missing parents (especially disturbances) may not silently disappear.
            if used != parents {
                return Err(Error::InvalidEquation);
            }
            if let Some(previous) = equations.get(&equation.outcome) {
                if previous != &terms {
                    return Err(Error::ConflictingMechanism);
                }
            }
            equations.insert(equation.outcome, terms.clone());
            normalized.push(terms);
        }
        let cancelled: Vec<_> = normalized[0]
            .keys()
            .filter(|key| normalized[1].contains_key(*key))
            .cloned()
            .collect();
        let parents: BTreeSet<_> = normalized
            .iter()
            .flat_map(|terms| terms.iter())
            .filter(|(key, _)| !cancelled.contains(key))
            .flat_map(|(_, args)| args.iter().map(|v| v.0))
            .collect();
        let index = self.nodes.len();
        let mut edges = self.edges();
        edges.extend(parents.iter().map(|&p| (p, index)));
        self.nodes.push(Node::Difference {
            later: later.outcome,
            earlier: earlier.outcome,
            cancelled_terms: cancelled,
        });
        self.graph = Dag::new(self.nodes.len(), &edges);
        self.terms = known;
        self.equations = equations;
        Ok(index)
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
    pub fn edges(&self) -> Vec<(usize, usize)> {
        self.graph
            .children
            .iter()
            .enumerate()
            .flat_map(|(a, children)| children.iter().map(move |&b| (a, b)))
            .collect()
    }

    /// Query random nodes by original identity. Fixed intervention nodes are
    /// always conditioned on, as required for SWIG separation semantics.
    pub fn separated(&self, x: Variable, y: Variable, given: &[Variable]) -> Result<bool, Error> {
        if x.0 >= self.originals
            || y.0 >= self.originals
            || given.iter().any(|v| v.0 >= self.originals)
        {
            return Err(Error::UnknownVariable);
        }
        self.separated_nodes(x.0, y.0, &given.iter().map(|v| v.0).collect::<Vec<_>>())
    }

    /// Separation for original random nodes and constructed difference nodes.
    pub fn separated_nodes(&self, x: usize, y: usize, given: &[usize]) -> Result<bool, Error> {
        let mut z = BTreeSet::new();
        for &i in [x, y].iter().chain(given) {
            if i >= self.nodes.len() {
                return Err(Error::UnknownVariable);
            }
            if matches!(self.nodes[i], Node::Fixed { .. }) {
                return Err(Error::InvalidQuery);
            }
        }
        if x == y {
            return Err(Error::InvalidQuery);
        }
        for &v in given {
            if v == x || v == y || !z.insert(v) {
                return Err(Error::InvalidQuery);
            }
        }
        z.extend(
            self.nodes
                .iter()
                .enumerate()
                .filter_map(|(i, n)| matches!(n, Node::Fixed { .. }).then_some(i)),
        );
        Ok(self.graph.d_separated(x, y, &z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_conditioning_equals_removing_fixed_nodes_on_all_ordered_four_node_dags() {
        let possible = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        for mask in 0..64 {
            let edges: Vec<_> = possible
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, &(a, b))| (Variable(a), Variable(b)))
                .collect();
            let dag = CausalDag::new(vec![Role::Endogenous; 4], &edges).unwrap();
            for treatment in 0..4 {
                let swig = dag.intervene(&[(Variable(treatment), 0.0)]).unwrap();
                let random_edges: Vec<_> = edges
                    .iter()
                    .filter(|(a, _)| a.0 != treatment)
                    .map(|(a, b)| (a.0, b.0))
                    .collect();
                let random = Dag::new(4, &random_edges);
                for x in 0..4 {
                    for y in x + 1..4 {
                        for given in 0..16 {
                            if given & ((1 << x) | (1 << y)) != 0 {
                                continue;
                            }
                            let z: BTreeSet<_> = (0..4).filter(|i| given & (1 << i) != 0).collect();
                            assert_eq!(
                                swig.separated(
                                    Variable(x),
                                    Variable(y),
                                    &z.iter().copied().map(Variable).collect::<Vec<_>>()
                                )
                                .unwrap(),
                                random.d_separated(x, y, &z)
                            );
                        }
                    }
                }
            }
        }
    }
}
