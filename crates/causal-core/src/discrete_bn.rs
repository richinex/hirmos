//! Discrete Bayesian-network operations ported from pgmpy. The input adapter preserves observed
//! states for already-discrete columns and quantile-bins higher-cardinality measurements. The
//! remaining operations cover BDeu tables, variable elimination, minimal adjustment, and
//! parent-adjusted do-queries. The chapter 12 lane adds checked explicit kernels and ideal
//! graph-surgery queries, including additional evidence and simultaneous interventions.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// `pandas.qcut(s, q, duplicates="drop").cat.codes`.
pub fn qcut_codes(x: &[f64], q: usize) -> Vec<i32> {
    assert!(q > 0, "qcut requires at least one quantile");
    let mut sorted: Vec<f64> = x.iter().copied().filter(|value| !value.is_nan()).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN"));
    let n = sorted.len();
    if n == 0 {
        return vec![-1; x.len()];
    }
    // numpy's linear interpolation between order statistics.
    let quantile = |p: f64| -> f64 {
        let pos = p * (n - 1) as f64;
        let lo = pos.floor() as usize;
        let hi = pos.ceil() as usize;
        if lo == hi {
            sorted[lo]
        } else {
            sorted[lo] + (pos - lo as f64) * (sorted[hi] - sorted[lo])
        }
    };
    let step = 1.0 / q as f64;
    let raw: Vec<f64> = (0..=q)
        .map(|i| {
            // qcut builds these with linspace. pandas' ndarray quantile path then
            // converts to percent and NumPy converts back; both roundings matter
            // when an observation lies exactly on a tied bin edge.
            let probability = if i == q { 1.0 } else { i as f64 * step };
            quantile((probability * 100.0) / 100.0)
        })
        .collect();
    // duplicates="drop" keeps the first of each repeated edge.
    let mut bins: Vec<f64> = Vec::new();
    for v in raw {
        if bins.last().map(|&b| b != v).unwrap_or(true) {
            bins.push(v);
        }
    }
    if bins.len() < 2 {
        return vec![-1; x.len()];
    }
    x.iter()
        .map(|&v| {
            if v.is_nan() {
                return -1;
            }
            // pd.cut with right=True and include_lowest=True.
            let mut idx = bins.partition_point(|&b| b < v);
            if v == bins[0] {
                idx = 1;
            }
            if idx == 0 || idx >= bins.len() + 1 || v > *bins.last().expect("non-empty") {
                -1
            } else {
                (idx - 1) as i32
            }
        })
        .collect()
}

/// Discrete state labels and the original-unit mean represented by each non-missing state.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteStates {
    pub labels: Vec<String>,
    pub means: BTreeMap<String, f64>,
    pub strategy: DiscreteStateStrategy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateBudget(usize);

impl StateBudget {
    pub const MINIMUM: usize = 2;

    pub fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateBudgetError {
    pub requested: usize,
    pub minimum: usize,
}

impl TryFrom<usize> for StateBudget {
    type Error = StateBudgetError;

    fn try_from(requested: usize) -> Result<Self, Self::Error> {
        if requested < Self::MINIMUM {
            Err(StateBudgetError {
                requested,
                minimum: Self::MINIMUM,
            })
        } else {
            Ok(Self(requested))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscreteStateStrategy {
    ObservedStates { states: usize },
    Quantiles { requested: usize, populated: usize },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiscreteStateProblem {
    NoFiniteObservations {
        observations: usize,
    },
    SingleObservedState {
        value: f64,
        observations: usize,
    },
    QuantileCollapse {
        distinct_values: usize,
        requested_states: usize,
        populated_states: usize,
    },
}

/// Prepare one column for a discrete Bayesian network.
///
/// pgmpy treats the sorted observed values as states when no explicit `state_names` are supplied.
/// Preserve that state space when it fits the requested budget. Higher-cardinality measurements
/// use qcut-compatible quantile bins. A constant column or a quantile result with fewer than two
/// populated states is refused instead of being reported as a zero contrast.
pub fn discretize_for_discrete_bn(
    x: &[f64],
    state_budget: StateBudget,
) -> Result<DiscreteStates, DiscreteStateProblem> {
    let mut states: Vec<f64> = x
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect();
    states.sort_by(|left, right| left.partial_cmp(right).expect("finite values"));
    states.dedup_by(|left, right| *left == *right);
    if states.is_empty() {
        return Err(DiscreteStateProblem::NoFiniteObservations {
            observations: x.len(),
        });
    }
    if states.len() == 1 {
        return Err(DiscreteStateProblem::SingleObservedState {
            value: states[0],
            observations: x.len(),
        });
    }

    let (codes, strategy) = if states.len() <= state_budget.get() {
        (
            x.iter()
                .map(|value| {
                    if !value.is_finite() {
                        return -1;
                    }
                    states
                        .iter()
                        .position(|state| state == value)
                        .expect("every finite value is an observed state")
                        as i32
                })
                .collect(),
            DiscreteStateStrategy::ObservedStates {
                states: states.len(),
            },
        )
    } else {
        let codes = qcut_codes(x, state_budget.get());
        let populated = codes
            .iter()
            .copied()
            .filter(|code| *code >= 0)
            .collect::<BTreeSet<_>>()
            .len();
        if populated < 2 {
            return Err(DiscreteStateProblem::QuantileCollapse {
                distinct_values: states.len(),
                requested_states: state_budget.get(),
                populated_states: populated,
            });
        }
        (
            codes,
            DiscreteStateStrategy::Quantiles {
                requested: state_budget.get(),
                populated,
            },
        )
    };

    let labels: Vec<String> = codes.iter().map(ToString::to_string).collect();
    let mut sums: BTreeMap<i32, (f64, usize)> = BTreeMap::new();
    for (&value, &code) in x.iter().zip(&codes) {
        if code >= 0 && !value.is_nan() {
            let entry = sums.entry(code).or_insert((0.0, 0));
            entry.0 += value;
            entry.1 += 1;
        }
    }
    let means = sums
        .into_iter()
        .map(|(code, (sum, count))| (code.to_string(), sum / count as f64))
        .collect();
    Ok(DiscreteStates {
        labels,
        means,
        strategy,
    })
}

/// A directed acyclic graph over named nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dag {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
}

impl Dag {
    pub fn new(edges: &[(String, String)], extra_nodes: &[String]) -> Self {
        let mut nodes: Vec<String> = Vec::new();
        for (a, b) in edges {
            for v in [a, b] {
                if !nodes.contains(v) {
                    nodes.push(v.clone());
                }
            }
        }
        for v in extra_nodes {
            if !nodes.contains(v) {
                nodes.push(v.clone());
            }
        }
        Dag {
            nodes,
            edges: edges.to_vec(),
        }
    }

    pub fn parents(&self, node: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(_, b)| b == node)
            .map(|(a, _)| a.clone())
            .collect()
    }

    pub fn children(&self, node: &str) -> Vec<String> {
        self.edges
            .iter()
            .filter(|(a, _)| a == node)
            .map(|(_, b)| b.clone())
            .collect()
    }

    /// `get_ancestors`, which includes the nodes themselves.
    pub fn ancestors(&self, nodes: &[String]) -> BTreeSet<String> {
        let mut out: BTreeSet<String> = nodes.iter().cloned().collect();
        let mut stack: Vec<String> = nodes.to_vec();
        while let Some(v) = stack.pop() {
            for p in self.parents(&v) {
                if out.insert(p.clone()) {
                    stack.push(p);
                }
            }
        }
        out
    }

    /// `active_trail_nodes`: the Bayes-ball reachable set from `start` given `observed`.
    pub fn active_trail_nodes(&self, start: &str, observed: &[String]) -> BTreeSet<String> {
        let observed_set: HashSet<&String> = observed.iter().collect();
        let ancestors_list = self.ancestors(observed);
        let mut visit: BTreeSet<(String, bool)> = BTreeSet::new();
        visit.insert((start.to_string(), true)); // true is "up"
        let mut traversed: HashSet<(String, bool)> = HashSet::new();
        let mut active: BTreeSet<String> = BTreeSet::new();
        while let Some(item) = visit.iter().next_back().cloned() {
            visit.remove(&item);
            let (node, up) = item;
            if traversed.contains(&(node.clone(), up)) {
                continue;
            }
            if !observed_set.contains(&node) {
                active.insert(node.clone());
            }
            traversed.insert((node.clone(), up));
            if up && !observed_set.contains(&node) {
                for p in self.parents(&node) {
                    visit.insert((p, true));
                }
                for c in self.children(&node) {
                    visit.insert((c, false));
                }
            } else if !up {
                if !observed_set.contains(&node) {
                    for c in self.children(&node) {
                        visit.insert((c, false));
                    }
                }
                if ancestors_list.contains(&node) {
                    for p in self.parents(&node) {
                        visit.insert((p, true));
                    }
                }
            }
        }
        active
    }

    pub fn is_dconnected(&self, start: &str, end: &str, observed: &[String]) -> bool {
        self.active_trail_nodes(start, observed).contains(end)
    }

    /// `get_ancestral_graph`: the subgraph induced on the ancestors of the given nodes.
    pub fn ancestral_graph(&self, nodes: &[String]) -> Dag {
        let keep = self.ancestors(nodes);
        let edges: Vec<(String, String)> = self
            .edges
            .iter()
            .filter(|(a, b)| keep.contains(a) && keep.contains(b))
            .cloned()
            .collect();
        Dag {
            nodes: keep.into_iter().collect(),
            edges,
        }
    }

    /// `get_proper_backdoor_graph`: drop the first edge of every simple path from X to Y.
    pub fn proper_backdoor_graph(&self, x: &str, y: &str) -> Dag {
        let mut to_remove: HashSet<(String, String)> = HashSet::new();
        // Every simple path out of X that reaches Y starts with an edge X -> c where c can
        // still reach Y, so those are exactly the first edges to drop.
        for c in self.children(x) {
            if c == y || self.reaches(&c, y, x) {
                to_remove.insert((x.to_string(), c));
            }
        }
        let edges: Vec<(String, String)> = self
            .edges
            .iter()
            .filter(|e| !to_remove.contains(*e))
            .cloned()
            .collect();
        Dag {
            nodes: self.nodes.clone(),
            edges,
        }
    }

    /// Is `target` reachable from `from` along directed edges without revisiting `avoid`?
    fn reaches(&self, from: &str, target: &str, avoid: &str) -> bool {
        let mut seen: HashSet<String> = HashSet::new();
        let mut stack = vec![from.to_string()];
        while let Some(v) = stack.pop() {
            if v == target {
                return true;
            }
            if !seen.insert(v.clone()) {
                continue;
            }
            for c in self.children(&v) {
                if c != avoid {
                    stack.push(c);
                }
            }
        }
        false
    }

    /// `minimal_dseparator`: start from the parents of both endpoints, then drop members
    /// while separation still holds.
    pub fn minimal_dseparator(&self, start: &str, end: &str) -> Option<BTreeSet<String>> {
        let an_graph = self.ancestral_graph(&[start.to_string(), end.to_string()]);
        let mut separator: BTreeSet<String> = self
            .parents(start)
            .into_iter()
            .chain(self.parents(end))
            .collect();
        separator.remove(start);
        separator.remove(end);
        let as_vec = |s: &BTreeSet<String>| -> Vec<String> { s.iter().cloned().collect() };
        if an_graph.is_dconnected(start, end, &as_vec(&separator)) {
            return None;
        }
        let mut minimal = separator.clone();
        for u in &separator {
            let mut trial = minimal.clone();
            trial.remove(u);
            if !an_graph.is_dconnected(start, end, &as_vec(&trial)) {
                minimal.remove(u);
            }
        }
        Some(minimal)
    }
}

/// A conditional probability table. `values[state * n_cols + col]`, where the columns run
/// over the parent configurations with the first parent varying slowest.
#[derive(Clone, Debug, PartialEq)]
pub struct Cpd {
    pub variable: String,
    pub states: Vec<String>,
    pub parents: Vec<String>,
    pub parent_states: Vec<Vec<String>>,
    pub values: Vec<f64>,
}

impl Cpd {
    fn n_cols(&self) -> usize {
        self.parent_states
            .iter()
            .map(|s| s.len())
            .product::<usize>()
            .max(1)
    }

    /// The column index for one assignment of the parents.
    fn column(&self, assignment: &HashMap<String, usize>) -> usize {
        let mut col = 0;
        for (p, states) in self.parents.iter().zip(&self.parent_states) {
            col = col * states.len() + assignment[p];
        }
        col
    }
}

/// Failures at the checked discrete Bayesian-network boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum DiscreteBnError {
    EmptyNode,
    DuplicateNode(String),
    UnknownEdgeNode(String),
    DuplicateEdge(String, String),
    Cycle,
    MissingCpd(String),
    DuplicateCpd(String),
    UnknownCpdVariable(String),
    ParentMismatch {
        variable: String,
        graph: Vec<String>,
        cpd: Vec<String>,
    },
    ParentStateMismatch {
        variable: String,
        parent: String,
    },
    EmptyStates(String),
    DuplicateState {
        variable: String,
        state: String,
    },
    InvalidCpdLength {
        variable: String,
        expected: usize,
        actual: usize,
    },
    InvalidProbability {
        variable: String,
        index: usize,
        value: f64,
    },
    UnnormalizedCpdColumn {
        variable: String,
        column: usize,
        sum: f64,
    },
    EmptyTargets,
    DuplicateTarget(String),
    UnknownVariable(String),
    UnknownState {
        variable: String,
        state: String,
    },
    DuplicateIntervention(String),
    ConflictingAssignment {
        variable: String,
        intervention: String,
        evidence: String,
    },
    ZeroProbabilityEvidence,
    StateSpaceTooLarge,
    InvalidTolerance(f64),
}

impl std::fmt::Display for DiscreteBnError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for DiscreteBnError {}

/// One ideal intervention fixing a named variable to a named state.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intervention {
    pub variable: String,
    pub state: String,
}

/// A normalized distribution over one or more query variables.
///
/// Each entry in `states` is one joint assignment in `variables` order.
#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteDistribution {
    pub variables: Vec<String>,
    pub states: Vec<Vec<String>>,
    pub probabilities: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscreteBn {
    pub dag: Dag,
    pub cpds: Vec<Cpd>,
    pub state_names: HashMap<String, Vec<String>>,
}

/// `BayesianEstimator(model, data).estimate_cpd(node, prior_type="BDeu", ess)`.
pub fn estimate_cpd(
    dag: &Dag,
    data: &HashMap<String, Vec<String>>,
    state_names: &HashMap<String, Vec<String>>,
    node: &str,
    equivalent_sample_size: f64,
) -> Cpd {
    let states = state_names[node].clone();
    let mut parents = dag.parents(node);
    parents.sort();
    let parent_states: Vec<Vec<String>> = parents.iter().map(|p| state_names[p].clone()).collect();
    let node_card = states.len();
    let n_cols: usize = parent_states
        .iter()
        .map(|s| s.len())
        .product::<usize>()
        .max(1);

    let alpha = equivalent_sample_size / (node_card * n_cols) as f64;
    let mut counts = vec![alpha; node_card * n_cols];

    let index_of =
        |v: &str, list: &[String]| list.iter().position(|s| s == v).expect("known state");
    let n_rows = data[node].len();
    for r in 0..n_rows {
        let row_state = index_of(&data[node][r], &states);
        let mut col = 0;
        for (p, ps) in parents.iter().zip(&parent_states) {
            col = col * ps.len() + index_of(&data[p][r], ps);
        }
        counts[row_state * n_cols + col] += 1.0;
    }
    // Normalise each parent configuration to a distribution.
    for col in 0..n_cols {
        let total: f64 = (0..node_card).map(|s| counts[s * n_cols + col]).sum();
        if total > 0.0 {
            for s in 0..node_card {
                counts[s * n_cols + col] /= total;
            }
        } else {
            for s in 0..node_card {
                counts[s * n_cols + col] = 1.0 / node_card as f64;
            }
        }
    }
    Cpd {
        variable: node.to_string(),
        states,
        parents,
        parent_states,
        values: counts,
    }
}

impl DiscreteBn {
    /// Build a Bayesian network from explicitly supplied causal Markov kernels.
    ///
    /// This is the checked boundary used by the chapter 12 decision examples.
    pub fn from_cpds(dag: Dag, cpds: Vec<Cpd>) -> Result<Self, DiscreteBnError> {
        Self::from_cpds_with_tolerance(dag, cpds, 1e-12)
    }

    pub fn from_cpds_with_tolerance(
        dag: Dag,
        cpds: Vec<Cpd>,
        normalization_tolerance: f64,
    ) -> Result<Self, DiscreteBnError> {
        if !normalization_tolerance.is_finite() || normalization_tolerance < 0.0 {
            return Err(DiscreteBnError::InvalidTolerance(normalization_tolerance));
        }
        validate_dag(&dag)?;

        let mut cpds_by_variable: HashMap<&str, &Cpd> = HashMap::new();
        for cpd in &cpds {
            if !dag.nodes.contains(&cpd.variable) {
                return Err(DiscreteBnError::UnknownCpdVariable(cpd.variable.clone()));
            }
            if cpds_by_variable.insert(&cpd.variable, cpd).is_some() {
                return Err(DiscreteBnError::DuplicateCpd(cpd.variable.clone()));
            }
        }
        for node in &dag.nodes {
            if !cpds_by_variable.contains_key(node.as_str()) {
                return Err(DiscreteBnError::MissingCpd(node.clone()));
            }
        }

        let mut state_names = HashMap::new();
        for cpd in &cpds {
            validate_states(&cpd.variable, &cpd.states)?;
            state_names.insert(cpd.variable.clone(), cpd.states.clone());
        }

        for cpd in &cpds {
            let graph_parents: BTreeSet<String> = dag.parents(&cpd.variable).into_iter().collect();
            let cpd_parents: BTreeSet<String> = cpd.parents.iter().cloned().collect();
            if cpd.parents.len() != cpd_parents.len() || graph_parents != cpd_parents {
                return Err(DiscreteBnError::ParentMismatch {
                    variable: cpd.variable.clone(),
                    graph: graph_parents.into_iter().collect(),
                    cpd: cpd.parents.clone(),
                });
            }
            if cpd.parents.len() != cpd.parent_states.len() {
                return Err(DiscreteBnError::ParentMismatch {
                    variable: cpd.variable.clone(),
                    graph: graph_parents.into_iter().collect(),
                    cpd: cpd.parents.clone(),
                });
            }
            for (parent, supplied_states) in cpd.parents.iter().zip(&cpd.parent_states) {
                if state_names.get(parent) != Some(supplied_states) {
                    return Err(DiscreteBnError::ParentStateMismatch {
                        variable: cpd.variable.clone(),
                        parent: parent.clone(),
                    });
                }
            }

            let columns = cpd.n_cols();
            let expected = cpd.states.len() * columns;
            if cpd.values.len() != expected {
                return Err(DiscreteBnError::InvalidCpdLength {
                    variable: cpd.variable.clone(),
                    expected,
                    actual: cpd.values.len(),
                });
            }
            for (index, &value) in cpd.values.iter().enumerate() {
                if !value.is_finite() || value < 0.0 {
                    return Err(DiscreteBnError::InvalidProbability {
                        variable: cpd.variable.clone(),
                        index,
                        value,
                    });
                }
            }
            for column in 0..columns {
                let sum: f64 = (0..cpd.states.len())
                    .map(|state| cpd.values[state * columns + column])
                    .sum();
                if (sum - 1.0).abs() > normalization_tolerance {
                    return Err(DiscreteBnError::UnnormalizedCpdColumn {
                        variable: cpd.variable.clone(),
                        column,
                        sum,
                    });
                }
            }
        }

        Ok(Self {
            dag,
            cpds,
            state_names,
        })
    }

    /// Fits every node's CPD with a BDeu prior.
    pub fn fit(dag: Dag, data: &HashMap<String, Vec<String>>, equivalent_sample_size: f64) -> Self {
        // pgmpy's state names are the sorted unique observed values.
        let mut state_names: HashMap<String, Vec<String>> = HashMap::new();
        for node in &dag.nodes {
            let mut s: Vec<String> = data[node]
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            s.sort();
            state_names.insert(node.clone(), s);
        }
        let cpds = dag
            .nodes
            .iter()
            .map(|n| estimate_cpd(&dag, data, &state_names, n, equivalent_sample_size))
            .collect();
        DiscreteBn {
            dag,
            cpds,
            state_names,
        }
    }

    fn cpd(&self, node: &str) -> &Cpd {
        self.cpds
            .iter()
            .find(|c| c.variable == node)
            .expect("node has a CPD")
    }

    /// `VariableElimination.query(variables, evidence)`: reduce every factor by the
    /// evidence, sum out everything that is not queried, and normalise.
    pub fn query(
        &self,
        variables: &[String],
        evidence: &HashMap<String, String>,
    ) -> HashMap<String, f64> {
        assert_eq!(
            variables.len(),
            1,
            "only single variable queries are used here"
        );
        let target = &variables[0];

        // Pruning: keep only what is d-connected to the query given the evidence, then
        // restrict to the ancestral graph of the query and evidence.
        let observed: Vec<String> = evidence.keys().cloned().collect();
        let mut d_connected = self.dag.active_trail_nodes(target, &observed);
        d_connected.extend(evidence.keys().cloned());
        let sub_edges: Vec<(String, String)> = self
            .dag
            .edges
            .iter()
            .filter(|(a, b)| d_connected.contains(a) && d_connected.contains(b))
            .cloned()
            .collect();
        let sub = Dag {
            nodes: d_connected.iter().cloned().collect(),
            edges: sub_edges,
        };
        let mut roots = vec![target.clone()];
        roots.extend(
            evidence
                .keys()
                .filter(|k| d_connected.contains(*k))
                .cloned(),
        );
        let pruned = sub.ancestral_graph(&roots);

        // Enumerate over the free variables, accumulating the product of the reduced CPDs.
        let free: Vec<String> = pruned
            .nodes
            .iter()
            .filter(|v| *v != target && !evidence.contains_key(*v))
            .cloned()
            .collect();
        let free_cards: Vec<usize> = free.iter().map(|v| self.state_names[v].len()).collect();
        let n_combos: usize = free_cards.iter().product::<usize>().max(1);

        let target_states = &self.state_names[target];
        let mut out = vec![0.0f64; target_states.len()];
        for (ti, _) in target_states.iter().enumerate() {
            let mut total = 0.0;
            for combo in 0..n_combos {
                let mut assignment: HashMap<String, usize> = HashMap::new();
                assignment.insert(target.clone(), ti);
                for (k, v) in evidence {
                    if pruned.nodes.contains(k) {
                        let idx = self.state_names[k]
                            .iter()
                            .position(|s| s == v)
                            .expect("evidence state exists");
                        assignment.insert(k.clone(), idx);
                    }
                }
                let mut rest = combo;
                for (i, v) in free.iter().enumerate() {
                    assignment.insert(v.clone(), rest % free_cards[i]);
                    rest /= free_cards[i];
                }
                let mut product = 1.0;
                for node in &pruned.nodes {
                    let cpd = self.cpd(node);
                    // A CPD whose parents left the pruned graph has been marginalised out
                    // of that scope, which for a normalised table means it contributes one.
                    if cpd.parents.iter().any(|p| !pruned.nodes.contains(p)) {
                        continue;
                    }
                    let col = cpd.column(&assignment);
                    product *= cpd.values[assignment[node] * cpd.n_cols() + col];
                }
                total += product;
            }
            out[ti] = total;
        }
        let norm: f64 = out.iter().sum();
        target_states
            .iter()
            .cloned()
            .zip(out.into_iter().map(|v| v / norm))
            .collect()
    }

    /// Exact enumeration equivalent to pgmpy's variable elimination for a checked model.
    pub fn query_checked(
        &self,
        variables: &[String],
        evidence: &HashMap<String, String>,
    ) -> Result<DiscreteDistribution, DiscreteBnError> {
        self.query_under_intervention(variables, &[], evidence)
    }

    /// Evaluate `P(variables | do(interventions), evidence)` by ideal graph surgery.
    ///
    /// During enumeration an intervened node's original causal Markov kernel is replaced
    /// by a point mass. This is algebraically identical to removing its incoming edges and
    /// installing the one-column kernel used by AltDeep's `pgmpy_do.py`.
    pub fn query_under_intervention(
        &self,
        variables: &[String],
        interventions: &[Intervention],
        evidence: &HashMap<String, String>,
    ) -> Result<DiscreteDistribution, DiscreteBnError> {
        if variables.is_empty() {
            return Err(DiscreteBnError::EmptyTargets);
        }

        let mut targets = HashSet::new();
        for variable in variables {
            self.validate_variable(variable)?;
            if !targets.insert(variable) {
                return Err(DiscreteBnError::DuplicateTarget(variable.clone()));
            }
        }
        for (variable, state) in evidence {
            self.validate_state(variable, state)?;
        }

        let mut fixed: HashMap<String, usize> = HashMap::new();
        for intervention in interventions {
            self.validate_state(&intervention.variable, &intervention.state)?;
            if fixed.contains_key(&intervention.variable) {
                return Err(DiscreteBnError::DuplicateIntervention(
                    intervention.variable.clone(),
                ));
            }
            let state = self.state_index(&intervention.variable, &intervention.state);
            fixed.insert(intervention.variable.clone(), state);
        }
        for (variable, evidence_state) in evidence {
            if let Some(&intervention_state) = fixed.get(variable) {
                let intervention_name = &self.state_names[variable][intervention_state];
                if intervention_name != evidence_state {
                    return Err(DiscreteBnError::ConflictingAssignment {
                        variable: variable.clone(),
                        intervention: intervention_name.clone(),
                        evidence: evidence_state.clone(),
                    });
                }
            }
            fixed.insert(variable.clone(), self.state_index(variable, evidence_state));
        }

        let surgery: Vec<(String, String)> = self
            .dag
            .edges
            .iter()
            .filter(|(_, child)| !fixed_by_intervention(interventions, child))
            .cloned()
            .collect();
        let roots: Vec<String> = variables.iter().chain(evidence.keys()).cloned().collect();
        let relevant = Dag::new(&surgery, &self.dag.nodes).ancestral_graph(&roots).nodes;
        let cards: Vec<usize> = relevant
            .iter()
            .map(|node| self.state_names[node].len())
            .collect();
        let combinations = cards
            .iter()
            .try_fold(1usize, |total, card| total.checked_mul(*card))
            .ok_or(DiscreteBnError::StateSpaceTooLarge)?;

        let target_cards: Vec<usize> = variables
            .iter()
            .map(|variable| self.state_names[variable].len())
            .collect();
        let target_combinations: usize = target_cards.iter().product();
        let mut probabilities = vec![0.0; target_combinations];

        let intervention_variables: HashSet<&str> = interventions
            .iter()
            .map(|intervention| intervention.variable.as_str())
            .collect();
        for combination in 0..combinations {
            let assignment = decode_assignment(&relevant, &cards, combination);
            if fixed
                .iter()
                .any(|(variable, state)| assignment[variable] != *state)
            {
                continue;
            }

            let mut probability = 1.0;
            for node in &relevant {
                if intervention_variables.contains(node.as_str()) {
                    continue;
                }
                let cpd = self.cpd(node);
                let column = cpd.column(&assignment);
                probability *= cpd.values[assignment[node] * cpd.n_cols() + column];
            }
            if probability == 0.0 {
                continue;
            }

            let mut output_index = 0;
            for (variable, card) in variables.iter().zip(&target_cards) {
                output_index = output_index * card + assignment[variable];
            }
            probabilities[output_index] += probability;
        }

        let normalization: f64 = probabilities.iter().sum();
        if !normalization.is_finite() || normalization == 0.0 {
            return Err(DiscreteBnError::ZeroProbabilityEvidence);
        }
        for probability in &mut probabilities {
            *probability /= normalization;
        }

        let states = (0..target_combinations)
            .map(|combination| {
                let indices = decode_indices_left_major(&target_cards, combination);
                variables
                    .iter()
                    .zip(indices)
                    .map(|(variable, state)| self.state_names[variable][state].clone())
                    .collect()
            })
            .collect();

        Ok(DiscreteDistribution {
            variables: variables.to_vec(),
            states,
            probabilities,
        })
    }

    fn validate_variable(&self, variable: &str) -> Result<(), DiscreteBnError> {
        if self.state_names.contains_key(variable) {
            Ok(())
        } else {
            Err(DiscreteBnError::UnknownVariable(variable.to_string()))
        }
    }

    fn validate_state(&self, variable: &str, state: &str) -> Result<(), DiscreteBnError> {
        self.validate_variable(variable)?;
        if self.state_names[variable]
            .iter()
            .any(|candidate| candidate == state)
        {
            Ok(())
        } else {
            Err(DiscreteBnError::UnknownState {
                variable: variable.to_string(),
                state: state.to_string(),
            })
        }
    }

    fn state_index(&self, variable: &str, state: &str) -> usize {
        self.state_names[variable]
            .iter()
            .position(|candidate| candidate == state)
            .expect("state was validated")
    }

    /// `CausalInference.get_minimal_adjustment_set(X, Y)`.
    pub fn minimal_adjustment_set(&self, x: &str, y: &str) -> Option<BTreeSet<String>> {
        self.dag
            .proper_backdoor_graph(x, y)
            .minimal_dseparator(x, y)
    }

    /// `CausalInference.query([Y], do={X: state})`: adjust for the parents of the
    /// intervened variable and average the conditional over their distribution.
    pub fn do_query(&self, target: &str, do_var: &str, do_state: &str) -> HashMap<String, f64> {
        let mut adjustment: Vec<String> = self.dag.parents(do_var);
        adjustment.sort();
        if adjustment.is_empty() {
            let mut evidence = HashMap::new();
            evidence.insert(do_var.to_string(), do_state.to_string());
            return self.query(&[target.to_string()], &evidence);
        }

        // p(z) over the adjustment set, from the same inference engine.
        let adj_states: Vec<Vec<String>> = adjustment
            .iter()
            .map(|v| self.state_names[v].clone())
            .collect();
        let p_z = self.joint(&adjustment);

        let target_states = &self.state_names[target];
        let mut acc: HashMap<String, f64> =
            target_states.iter().map(|s| (s.clone(), 0.0)).collect();
        let n_combos: usize = adj_states.iter().map(|s| s.len()).product();
        for combo in 0..n_combos {
            let mut rest = combo;
            let mut assign: Vec<String> = Vec::with_capacity(adjustment.len());
            for states in &adj_states {
                assign.push(states[rest % states.len()].clone());
                rest /= states.len();
            }
            let weight = p_z[&assign];
            if weight == 0.0 {
                continue;
            }
            let mut evidence: HashMap<String, String> = HashMap::new();
            evidence.insert(do_var.to_string(), do_state.to_string());
            for (v, s) in adjustment.iter().zip(&assign) {
                evidence.insert(v.clone(), s.clone());
            }
            let cond = self.query(&[target.to_string()], &evidence);
            for (state, p) in cond {
                *acc.get_mut(&state).expect("known state") += p * weight;
            }
        }
        let norm: f64 = acc.values().sum();
        acc.into_iter().map(|(k, v)| (k, v / norm)).collect()
    }

    /// The joint distribution over a set of variables, by enumeration over the network.
    fn joint(&self, vars: &[String]) -> HashMap<Vec<String>, f64> {
        let all: Vec<String> = self.dag.nodes.clone();
        let cards: Vec<usize> = all.iter().map(|v| self.state_names[v].len()).collect();
        let n_combos: usize = cards.iter().product();
        let mut out: HashMap<Vec<String>, f64> = HashMap::new();
        for combo in 0..n_combos {
            let mut rest = combo;
            let mut assignment: HashMap<String, usize> = HashMap::new();
            for (i, v) in all.iter().enumerate() {
                assignment.insert(v.clone(), rest % cards[i]);
                rest /= cards[i];
            }
            let mut product = 1.0;
            for node in &all {
                let cpd = self.cpd(node);
                let col = cpd.column(&assignment);
                product *= cpd.values[assignment[node] * cpd.n_cols() + col];
            }
            if product == 0.0 {
                continue;
            }
            let key: Vec<String> = vars
                .iter()
                .map(|v| self.state_names[v][assignment[v]].clone())
                .collect();
            *out.entry(key).or_insert(0.0) += product;
        }
        out
    }
}

fn validate_states(variable: &str, states: &[String]) -> Result<(), DiscreteBnError> {
    if states.is_empty() {
        return Err(DiscreteBnError::EmptyStates(variable.to_string()));
    }
    let mut seen = HashSet::new();
    for state in states {
        if !seen.insert(state) {
            return Err(DiscreteBnError::DuplicateState {
                variable: variable.to_string(),
                state: state.clone(),
            });
        }
    }
    Ok(())
}

fn validate_dag(dag: &Dag) -> Result<(), DiscreteBnError> {
    let mut nodes = HashSet::new();
    for node in &dag.nodes {
        if node.is_empty() {
            return Err(DiscreteBnError::EmptyNode);
        }
        if !nodes.insert(node) {
            return Err(DiscreteBnError::DuplicateNode(node.clone()));
        }
    }

    let mut edges = HashSet::new();
    let mut indegree: HashMap<&str, usize> =
        dag.nodes.iter().map(|node| (node.as_str(), 0)).collect();
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    for (cause, effect) in &dag.edges {
        if !nodes.contains(cause) {
            return Err(DiscreteBnError::UnknownEdgeNode(cause.clone()));
        }
        if !nodes.contains(effect) {
            return Err(DiscreteBnError::UnknownEdgeNode(effect.clone()));
        }
        if !edges.insert((cause, effect)) {
            return Err(DiscreteBnError::DuplicateEdge(
                cause.clone(),
                effect.clone(),
            ));
        }
        *indegree.get_mut(effect.as_str()).expect("known node") += 1;
        children.entry(cause).or_default().push(effect);
    }

    let mut ready: Vec<&str> = indegree
        .iter()
        .filter_map(|(&node, &degree)| (degree == 0).then_some(node))
        .collect();
    let mut visited = 0;
    while let Some(node) = ready.pop() {
        visited += 1;
        if let Some(next) = children.get(node) {
            for child in next {
                let degree = indegree.get_mut(child).expect("known child");
                *degree -= 1;
                if *degree == 0 {
                    ready.push(child);
                }
            }
        }
    }
    if visited != dag.nodes.len() {
        return Err(DiscreteBnError::Cycle);
    }
    Ok(())
}

fn fixed_by_intervention(interventions: &[Intervention], variable: &str) -> bool {
    interventions.iter().any(|intervention| intervention.variable == variable)
}

fn decode_assignment(
    variables: &[String],
    cards: &[usize],
    mut combination: usize,
) -> HashMap<String, usize> {
    let mut assignment = HashMap::with_capacity(variables.len());
    for (variable, card) in variables.iter().zip(cards) {
        assignment.insert(variable.clone(), combination % card);
        combination /= card;
    }
    assignment
}

fn decode_indices_left_major(cards: &[usize], mut combination: usize) -> Vec<usize> {
    let mut indices = vec![0; cards.len()];
    for position in (0..cards.len()).rev() {
        indices[position] = combination % cards[position];
        combination /= cards[position];
    }
    indices
}
