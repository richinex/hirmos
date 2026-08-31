//! pgmpy's discrete Bayesian network lane, which is step 3g of `805_dag.py`: quantile
//! discretisation, BDeu conditional probability tables, variable elimination, the minimal
//! adjustment set, and the interventional do-query by parent adjustment.

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

/// The result of 805's `_dc_disc`: string labels for pgmpy and the original-unit
/// mean represented by each non-missing ordinal state.
#[derive(Clone, Debug, PartialEq)]
pub struct Discretized805 {
    pub labels: Vec<String>,
    pub means: BTreeMap<String, f64>,
}

/// The complete discretisation wrapper in step 3g of `805_dag.py`.
///
/// Constant/all-missing columns return `None`. If duplicate qcut edges leave fewer than
/// two populated bins, 805 falls back to `x > median(x)`. Missing values retain qcut's
/// `-1` state unless that fallback is taken, in which case pandas comparison maps them to 0.
pub fn discretize_805(x: &[f64], q: usize) -> Option<Discretized805> {
    let mut valid: Vec<f64> = x.iter().copied().filter(|value| !value.is_nan()).collect();
    valid.sort_by(|left, right| left.partial_cmp(right).expect("no NaN"));
    valid.dedup_by(|left, right| *left == *right);
    if valid.len() < 2 {
        return None;
    }

    let mut codes = qcut_codes(x, q);
    if codes.iter().copied().max().unwrap_or(-1) < 1 {
        let mut ordered: Vec<f64> = x.iter().copied().filter(|value| !value.is_nan()).collect();
        ordered.sort_by(|left, right| left.partial_cmp(right).expect("no NaN"));
        let middle = ordered.len() / 2;
        let median = if ordered.len() % 2 == 0 {
            (ordered[middle - 1] + ordered[middle]) / 2.0
        } else {
            ordered[middle]
        };
        codes = x
            .iter()
            .map(|value| i32::from(!value.is_nan() && *value > median))
            .collect();
    }

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
    Some(Discretized805 { labels, means })
}

/// A directed acyclic graph over named nodes.
#[derive(Clone)]
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
#[derive(Clone)]
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
