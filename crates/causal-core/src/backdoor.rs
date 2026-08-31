//! DoWhy's default backdoor identification and linear estimator ported 1:1: the surgered-graph
//! d-separation search (largest-first then minimal), instrument-count and set-size tie-breaks,
//! and the backdoor.linear_regression ATE.

use nalgebra::{DMatrix, DVector};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Dag {
    pub n: usize,
    /// edges[i] holds the children of node i.
    pub children: Vec<Vec<usize>>,
    pub parents: Vec<Vec<usize>>,
}

impl Dag {
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut children = vec![Vec::new(); n];
        let mut parents = vec![Vec::new(); n];
        for &(a, b) in edges {
            children[a].push(b);
            parents[b].push(a);
        }
        Dag {
            n,
            children,
            parents,
        }
    }

    pub(crate) fn descendants(&self, start: usize) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut stack = self.children[start].clone();
        while let Some(v) = stack.pop() {
            if seen.insert(v) {
                stack.extend(self.children[v].iter().copied());
            }
        }
        seen
    }

    pub(crate) fn ancestors_of(&self, targets: &[usize]) -> BTreeSet<usize> {
        let mut seen: BTreeSet<usize> = targets.iter().copied().collect();
        let mut stack: Vec<usize> = targets.to_vec();
        while let Some(v) = stack.pop() {
            for &p in &self.parents[v] {
                if seen.insert(p) {
                    stack.push(p);
                }
            }
        }
        seen
    }

    /// d-separation of {x} and {y} by z via moralized ancestor graph reachability.
    pub(crate) fn d_separated(&self, x: usize, y: usize, z: &BTreeSet<usize>) -> bool {
        let mut targets = vec![x, y];
        targets.extend(z.iter().copied());
        let anc = self.ancestors_of(&targets);
        // Build the moral graph over the ancestor set.
        let nodes: Vec<usize> = anc.iter().copied().collect();
        let index = |v: usize| nodes.binary_search(&v).unwrap();
        let m = nodes.len();
        let mut adj = vec![BTreeSet::new(); m];
        for &v in &nodes {
            for &c in &self.children[v] {
                if anc.contains(&c) {
                    adj[index(v)].insert(index(c));
                    adj[index(c)].insert(index(v));
                }
            }
            // Marry co-parents.
            for (a_pos, &pa) in self.parents[v].iter().enumerate() {
                if !anc.contains(&pa) {
                    continue;
                }
                for &pb in self.parents[v].iter().skip(a_pos + 1) {
                    if anc.contains(&pb) {
                        adj[index(pa)].insert(index(pb));
                        adj[index(pb)].insert(index(pa));
                    }
                }
            }
        }
        // Remove z and test connectivity from x to y.
        let blocked: BTreeSet<usize> = z.iter().map(|&v| index(v)).collect();
        let (xi, yi) = (index(x), index(y));
        if blocked.contains(&xi) || blocked.contains(&yi) {
            return true;
        }
        let mut seen = BTreeSet::new();
        seen.insert(xi);
        let mut stack = vec![xi];
        while let Some(v) = stack.pop() {
            if v == yi {
                return false;
            }
            for &w in &adj[v] {
                if !blocked.contains(&w) && seen.insert(w) {
                    stack.push(w);
                }
            }
        }
        true
    }

    /// The backdoor graph: outgoing edges of the treatment removed.
    pub(crate) fn backdoor_graph(&self, treatment: usize) -> Dag {
        let mut edges = Vec::new();
        for (a, chs) in self.children.iter().enumerate() {
            for &b in chs {
                if a != treatment {
                    edges.push((a, b));
                }
            }
        }
        Dag::new(self.n, &edges)
    }

    /// The intervention graph used by DoWhy's first front-door condition: incoming edges of
    /// `treatment` removed.
    pub(crate) fn without_incoming(&self, treatment: usize) -> Dag {
        let edges = self
            .children
            .iter()
            .enumerate()
            .flat_map(|(source, children)| {
                children
                    .iter()
                    .copied()
                    .filter_map(move |target| (target != treatment).then_some((source, target)))
            })
            .collect::<Vec<_>>();
        Dag::new(self.n, &edges)
    }

    /// Graph surgery for a mediator set: remove every outgoing edge of every selected node.
    pub(crate) fn without_outgoing_set(&self, selected: &BTreeSet<usize>) -> Dag {
        let edges = self
            .children
            .iter()
            .enumerate()
            .flat_map(|(source, children)| {
                children.iter().copied().filter_map(move |target| {
                    (!selected.contains(&source)).then_some((source, target))
                })
            })
            .collect::<Vec<_>>();
        Dag::new(self.n, &edges)
    }

    fn instruments(&self, treatment: usize, outcome: usize) -> BTreeSet<usize> {
        let parents_t: BTreeSet<usize> = self.parents[treatment].iter().copied().collect();
        // Graph with incoming edges of treatment removed.
        let mut edges = Vec::new();
        for (a, chs) in self.children.iter().enumerate() {
            for &b in chs {
                if b != treatment {
                    edges.push((a, b));
                }
            }
        }
        let g = Dag::new(self.n, &edges);
        let mut anc_y = g.ancestors_of(&[outcome]);
        anc_y.remove(&outcome);
        let candidates: BTreeSet<usize> = parents_t.difference(&anc_y).copied().collect();
        let mut children_causes: BTreeSet<usize> = BTreeSet::new();
        for &v in &anc_y {
            children_causes.extend(g.descendants(v));
        }
        candidates.difference(&children_causes).copied().collect()
    }
}

pub(crate) fn combinations(pool: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    if k > pool.len() {
        return out;
    }
    let mut idx: Vec<usize> = (0..k).collect();
    loop {
        out.push(idx.iter().map(|&i| pool[i]).collect());
        let mut pos = k;
        loop {
            if pos == 0 {
                return out;
            }
            pos -= 1;
            if idx[pos] != pos + pool.len() - k {
                break;
            }
        }
        idx[pos] += 1;
        for i in pos + 1..k {
            idx[i] = idx[i - 1] + 1;
        }
    }
}

/// DoWhy's default backdoor set: valid sets from the largest-first sweep plus the minimal sweep,
/// then minimum instrument overlap and minimum size decide. Empty both when the empty set is valid
/// and when no set is; `backdoor_adjustment` keeps the two apart.
pub fn backdoor_variables(dag: &Dag, treatment: usize, outcome: usize) -> Vec<usize> {
    backdoor_adjustment(dag, treatment, outcome, &[]).unwrap_or_default()
}

/// The same search with DoWhy's observed-node rule: unobserved nodes stay in the graph for
/// d-separation but never enter a candidate set (`get_all_nodes(include_unobserved=False)`).
/// `None` means no valid backdoor set exists over the observed nodes.
pub fn backdoor_adjustment(
    dag: &Dag,
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
) -> Option<Vec<usize>> {
    let bdoor = dag.backdoor_graph(treatment);
    let mut sets: Vec<BTreeSet<usize>> = Vec::new();

    let empty = BTreeSet::new();
    if bdoor.d_separated(treatment, outcome, &empty) {
        sets.push(empty.clone());
    }

    let mut eligible: BTreeSet<usize> = (0..dag.n).collect();
    eligible.remove(&treatment);
    eligible.remove(&outcome);
    for d in dag.descendants(treatment) {
        eligible.remove(&d);
    }
    for u in unobserved {
        eligible.remove(u);
    }
    let eligible: Vec<usize> = eligible
        .into_iter()
        .filter(|&v| {
            let none = BTreeSet::new();
            !dag.d_separated(treatment, v, &none) || !dag.d_separated(outcome, v, &none)
        })
        .collect();

    // Largest-first sweep: with all variables observed the oracle stops after the largest size,
    // whether or not it succeeded.
    let mut found_largest = false;
    if !eligible.is_empty() {
        let size = eligible.len();
        for cand in combinations(&eligible, size) {
            let z: BTreeSet<usize> = cand.into_iter().collect();
            if bdoor.d_separated(treatment, outcome, &z) {
                sets.push(z);
                found_largest = true;
            }
        }
    }
    // Minimal sweep, only after the default sweep found a valid set.
    if found_largest {
        for size in 1..=eligible.len() {
            let mut found = false;
            for cand in combinations(&eligible, size) {
                let z: BTreeSet<usize> = cand.into_iter().collect();
                if bdoor.d_separated(treatment, outcome, &z) {
                    if !sets.contains(&z) {
                        sets.push(z);
                    }
                    found = true;
                }
            }
            if found {
                break;
            }
        }
    }

    if sets.is_empty() {
        return None;
    }
    let instruments = dag.instruments(treatment, outcome);
    let mut best = 0usize;
    let mut best_key = (usize::MAX, usize::MAX);
    for (k, s) in sets.iter().enumerate() {
        let iv = s.intersection(&instruments).count();
        let key = (iv, s.len());
        if key < best_key {
            best_key = key;
            best = k;
        }
    }
    Some(sets[best].iter().copied().collect())
}

/// backdoor.linear_regression: OLS of outcome on [1, treatment, adjustment set]; ATE is the
/// treatment coefficient.
pub fn backdoor_linear_ate(
    data: &DMatrix<f64>,
    treatment: usize,
    outcome: usize,
    adjust: &[usize],
) -> f64 {
    let n = data.nrows();
    let k = 2 + adjust.len();
    let mut design = DMatrix::<f64>::zeros(n, k);
    for i in 0..n {
        design[(i, 0)] = 1.0;
        design[(i, 1)] = data[(i, treatment)];
        for (c, &a) in adjust.iter().enumerate() {
            design[(i, 2 + c)] = data[(i, a)];
        }
    }
    let y = DVector::from_iterator(n, (0..n).map(|i| data[(i, outcome)]));
    let qr = design.clone().qr();
    let beta = qr
        .r()
        .solve_upper_triangular(&(qr.q().transpose() * &y))
        .expect("rank deficient");
    beta[1]
}

use crate::nprandom::Mt19937;

/// placebo_treatment_refuter with placebo_type "permute": mean ATE over permuted treatments.
pub fn refute_placebo(
    data: &DMatrix<f64>,
    treatment: usize,
    outcome: usize,
    adjust: &[usize],
    num_simulations: usize,
    mt: &mut Mt19937,
) -> f64 {
    let n = data.nrows();
    let mut total = 0.0;
    for _ in 0..num_simulations {
        let perm = mt.permutation(n);
        let mut permuted = data.clone();
        for (row, &src) in perm.iter().enumerate() {
            permuted[(row, treatment)] = data[(src, treatment)];
        }
        total += backdoor_linear_ate(&permuted, treatment, outcome, adjust);
    }
    total / num_simulations as f64
}

/// random_common_cause: mean ATE with a fresh standard-normal covariate joined to the adjustment.
pub fn refute_random_common_cause(
    data: &DMatrix<f64>,
    treatment: usize,
    outcome: usize,
    adjust: &[usize],
    num_simulations: usize,
    mt: &mut Mt19937,
) -> f64 {
    let n = data.nrows();
    let k = data.ncols();
    let mut total = 0.0;
    let mut cache = None;
    for _ in 0..num_simulations {
        let mut with_rc = data.clone().insert_column(k, 0.0);
        for row in 0..n {
            with_rc[(row, k)] = mt.standard_normal(&mut cache);
        }
        let mut adj: Vec<usize> = adjust.to_vec();
        adj.push(k);
        total += backdoor_linear_ate(&with_rc, treatment, outcome, &adj);
    }
    total / num_simulations as f64
}

/// data_subset_refuter: mean ATE over row subsets of the given fraction.
pub fn refute_data_subset(
    data: &DMatrix<f64>,
    treatment: usize,
    outcome: usize,
    adjust: &[usize],
    subset_fraction: f64,
    num_simulations: usize,
    mt: &mut Mt19937,
) -> f64 {
    let n = data.nrows();
    let size = (subset_fraction * n as f64).round() as usize;
    let mut total = 0.0;
    for _ in 0..num_simulations {
        let locs = mt.permutation(n);
        let subset = DMatrix::from_fn(size, data.ncols(), |i, j| data[(locs[i], j)]);
        total += backdoor_linear_ate(&subset, treatment, outcome, adjust);
    }
    total / num_simulations as f64
}
