//! `tigramite.causal_effects.CausalEffects` for `graph_type="stationary_dag"`: the latent
//! projection into a time series ADMG, the optimal adjustment set of Runge (NeurIPS 2021),
//! and the total effect estimator.

use std::collections::{BTreeMap, BTreeSet, HashSet};

/// A variable index paired with a lag, which is zero or negative.
pub type Node = (usize, i32);
/// A three character edge mark, `[0, 0, 0]` when absent.
pub type Edge = [u8; 3];

pub const EMPTY: Edge = [0, 0, 0];

/// Tigramite's `get_optimal_set(minimize=...)` choices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptimalSetMinimization {
    None,
    All,
    CollidersOnly,
}

/// Tigramite's `fit_total_effect(adjustment_set=...)` choices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdjustmentSetSelection {
    Optimal,
    MinimizedOptimal,
    CollidersMinimizedOptimal,
    Explicit(Vec<Node>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplicitAdjustmentProblem {
    QueryTreatment(Node),
    QueryOutcome(Node),
    LaterTreatmentOccurrence(Node),
    ForbiddenNode(Node),
    OpenNonCausalPath,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdjustmentSetError {
    NotIdentifiable,
    InvalidExplicitSet {
        problems: Vec<ExplicitAdjustmentProblem>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Optimality {
    NotIdentifiable,
    Checked {
        unique_set: bool,
        condition_i: bool,
        condition_ii: bool,
    },
}

impl Optimality {
    pub fn established(&self) -> bool {
        match self {
            Self::NotIdentifiable => false,
            Self::Checked {
                unique_set,
                condition_i,
                condition_ii,
            } => *unique_set || (*condition_i && *condition_ii),
        }
    }
}

pub const fn mark(s: &str) -> Edge {
    let b = s.as_bytes();
    [b[0], b[1], b[2]]
}

/// `_match_link`: `*` is a wildcard, and an absent edge matches only an absent pattern.
pub fn match_link(pattern: Edge, link: Edge) -> bool {
    if pattern == EMPTY || link == EMPTY {
        return pattern == link;
    }
    if pattern[0] != b'*' && link[0] != pattern[0] {
        return false;
    }
    if pattern[2] != b'*' && link[2] != pattern[2] {
        return false;
    }
    if pattern[1] != b'*' && link[1] != pattern[1] {
        return false;
    }
    true
}

/// `_reverse_link`.
pub fn reverse_link(link: Edge) -> Edge {
    if link == EMPTY {
        return EMPTY;
    }
    let left = if link[2] == b'>' { b'<' } else { link[2] };
    let right = if link[0] == b'<' { b'>' } else { link[0] };
    [left, link[1], right]
}

/// The stationary input graph, indexed `[i][j][tau]`.
#[derive(Clone)]
pub struct StationaryGraph {
    pub n: usize,
    pub stat_lag: usize,
    pub edges: Vec<Edge>,
}

impl StationaryGraph {
    pub fn new(n: usize, stat_lag: usize) -> Self {
        Self {
            n,
            stat_lag,
            edges: vec![EMPTY; n * n * (stat_lag + 1)],
        }
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize, tau: usize) -> Edge {
        self.edges[(i * self.n + j) * (self.stat_lag + 1) + tau]
    }

    pub fn set(&mut self, i: usize, j: usize, tau: usize, e: Edge) {
        let idx = (i * self.n + j) * (self.stat_lag + 1) + tau;
        self.edges[idx] = e;
    }
}

/// The projected time series graph, indexed `[i][j][taui][tauj]`.
#[derive(Clone)]
pub struct TsgGraph {
    pub n: usize,
    pub tau_max: usize,
    pub edges: Vec<Edge>,
}

impl TsgGraph {
    fn new(n: usize, tau_max: usize) -> Self {
        Self {
            n,
            tau_max,
            edges: vec![EMPTY; n * n * (tau_max + 1) * (tau_max + 1)],
        }
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize, ti: usize, tj: usize) -> Edge {
        let t = self.tau_max + 1;
        self.edges[((i * self.n + j) * t + ti) * t + tj]
    }

    fn set(&mut self, i: usize, j: usize, ti: usize, tj: usize, e: Edge) {
        let t = self.tau_max + 1;
        let idx = ((i * self.n + j) * t + ti) * t + tj;
        self.edges[idx] = e;
    }
}

// NonCausal is reached by tigramite's other estimators, not by the two entry points here.
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
enum PathType {
    Any,
    Causal,
    NonCausal,
}

pub struct CausalEffects {
    pub n: usize,
    pub tau_max: usize,
    pub graph: TsgGraph,
    stationary: StationaryGraph,
    pub x: BTreeSet<Node>,
    pub y: BTreeSet<Node>,
    pub s: BTreeSet<Node>,
    pub listx: Vec<Node>,
    pub listy: Vec<Node>,
    pub mediators: BTreeSet<Node>,
    pub no_causal_path: bool,
    anc_x: BTreeSet<Node>,
    anc_y: BTreeSet<Node>,
    anc_s: BTreeSet<Node>,
    descendants: BTreeSet<Node>,
    forbidden_nodes: BTreeSet<Node>,
    vancs: BTreeSet<Node>,
}

fn sorted_unique<T: Ord + Clone>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v.dedup();
    v
}

impl CausalEffects {
    /// `_find_adj` on the projected graph.
    fn find_adj(
        &self,
        node: Node,
        patterns: &[Edge],
        exclude: &[Node],
        graph: &TsgGraph,
    ) -> Vec<(Edge, Node)> {
        let (i, lag_i_signed) = node;
        let lag_i = lag_i_signed.unsigned_abs() as usize;
        let mut adj = Vec::new();
        if lag_i > graph.tau_max {
            return adj;
        }
        // Forward and contemporaneous.
        for k in 0..graph.n {
            for lag_ik in 0..=graph.tau_max {
                let link = graph.get(i, k, lag_i, lag_ik);
                if link == EMPTY {
                    continue;
                }
                for &patt in patterns {
                    if match_link(patt, link) {
                        let m = (k, -(lag_ik as i32));
                        if !exclude.contains(&m) {
                            adj.push((link, m));
                        }
                        break;
                    }
                }
            }
        }
        // Backward and contemporaneous, with the edge read in reverse.
        for k in 0..graph.n {
            for lag_ki in 0..=graph.tau_max {
                let link = graph.get(k, i, lag_ki, lag_i);
                if link == EMPTY {
                    continue;
                }
                for &patt in patterns {
                    if match_link(reverse_link(patt), link) {
                        let m = (k, -(lag_ki as i32));
                        if !exclude.contains(&m) {
                            adj.push((reverse_link(link), m));
                        }
                        break;
                    }
                }
            }
        }
        sorted_unique(adj)
    }

    /// `_get_adjacents_stationary_graph`, which walks the unexpanded stationary graph.
    fn adj_stationary(
        &self,
        node: Node,
        patterns: &[Edge],
        max_lag: i32,
        exclude: &[Node],
    ) -> Vec<(Edge, Node)> {
        let (i, lag_i) = node;
        let g = &self.stationary;
        let mut adj = Vec::new();
        for k in 0..g.n {
            for lag_ik in 0..=g.stat_lag {
                let link = g.get(i, k, lag_ik);
                if link == EMPTY {
                    continue;
                }
                if patterns.iter().any(|&p| match_link(p, link)) {
                    let lag = lag_i + lag_ik as i32;
                    let m = (k, lag);
                    if !exclude.contains(&m) && -max_lag <= lag && lag <= 0 {
                        adj.push((link, m));
                    }
                }
            }
        }
        for k in 0..g.n {
            for lag_ki in 0..=g.stat_lag {
                let link = g.get(k, i, lag_ki);
                if link == EMPTY {
                    continue;
                }
                if patterns.iter().any(|&p| match_link(reverse_link(p), link)) {
                    let lag = lag_i - lag_ki as i32;
                    let m = (k, lag);
                    if !exclude.contains(&m) && -max_lag <= lag && lag <= 0 {
                        adj.push((reverse_link(link), m));
                    }
                }
            }
        }
        sorted_unique(adj)
    }

    fn get_parents(&self, v: Node) -> Vec<Node> {
        self.find_adj(v, &[mark("<*-"), mark("<*+")], &[], &self.graph)
            .into_iter()
            .map(|(_, n)| n)
            .collect::<Vec<_>>()
    }

    fn get_children(&self, v: Node) -> Vec<Node> {
        sorted_unique(
            self.find_adj(v, &[mark("-*>"), mark("+*>")], &[], &self.graph)
                .into_iter()
                .map(|(_, n)| n)
                .collect(),
        )
    }

    fn get_spouses(&self, v: Node) -> Vec<Node> {
        sorted_unique(
            self.find_adj(
                v,
                &[mark("<*>"), mark("+*>"), mark("<*+")],
                &[],
                &self.graph,
            )
            .into_iter()
            .map(|(_, n)| n)
            .collect(),
        )
    }

    /// `_get_ancestors`, which includes the nodes themselves.
    fn get_ancestors(&self, w: &BTreeSet<Node>) -> BTreeSet<Node> {
        let mut ancestors: BTreeSet<Node> = w.clone();
        for &start in w {
            let mut this_level = vec![start];
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    for par in self.get_parents(varlag) {
                        let tau = par.1;
                        if !ancestors.contains(&par) && -(self.tau_max as i32) <= tau && tau <= 0 {
                            ancestors.insert(par);
                            next_level.push(par);
                        }
                    }
                }
                this_level = next_level;
            }
        }
        ancestors
    }

    /// `_get_all_parents`, one step only, including the nodes themselves.
    fn get_all_parents(&self, w: &BTreeSet<Node>) -> BTreeSet<Node> {
        let mut parents: BTreeSet<Node> = w.clone();
        for &node in w {
            for par in self.get_parents(node) {
                let tau = par.1;
                if !parents.contains(&par) && -(self.tau_max as i32) <= tau && tau <= 0 {
                    parents.insert(par);
                }
            }
        }
        parents
    }

    /// `_get_descendants`, which includes the nodes themselves.
    fn get_descendants(&self, w: &BTreeSet<Node>) -> BTreeSet<Node> {
        let mut descendants: BTreeSet<Node> = w.clone();
        for &start in w {
            let mut this_level = vec![start];
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    for child in self.get_children(varlag) {
                        let tau = child.1;
                        if !descendants.contains(&child)
                            && -(self.tau_max as i32) <= tau
                            && tau <= 0
                        {
                            descendants.insert(child);
                            next_level.push(child);
                        }
                    }
                }
                this_level = next_level;
            }
        }
        descendants
    }

    /// `_get_descendants_stationary_graph`, which has no time bound.
    fn get_descendants_stationary(&self, w: &BTreeSet<Node>, max_lag: i32) -> BTreeSet<Node> {
        let mut descendants: BTreeSet<Node> = w.clone();
        for &start in w {
            let mut this_level = vec![start];
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    for (_, child) in
                        self.adj_stationary(varlag, &[mark("-*>"), mark("-*+")], max_lag, &[])
                    {
                        if !descendants.contains(&child) {
                            descendants.insert(child);
                            next_level.push(child);
                        }
                    }
                }
                this_level = next_level;
            }
        }
        descendants
    }

    /// `get_mediators`: nodes on proper causal paths, walked back from the end.
    fn get_mediators(&self, start: &BTreeSet<Node>, end: &BTreeSet<Node>) -> BTreeSet<Node> {
        let des_x = self.get_descendants(start);
        let mut mediators = BTreeSet::new();
        for &y in end {
            let mut this_level = vec![y];
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    for parent in self.get_parents(varlag) {
                        let tau = parent.1;
                        if des_x.contains(&parent)
                            && !mediators.contains(&parent)
                            && !start.contains(&parent)
                            && !end.contains(&parent)
                            && -(self.tau_max as i32) <= tau
                            && tau <= 0
                        {
                            mediators.insert(parent);
                            next_level.push(parent);
                        }
                    }
                }
                this_level = next_level;
            }
        }
        mediators
    }

    /// `_get_mediators_stationary_graph`.
    fn get_mediators_stationary(
        &self,
        start: &BTreeSet<Node>,
        end: &BTreeSet<Node>,
        max_lag: i32,
    ) -> BTreeSet<Node> {
        let des_x = self.get_descendants_stationary(start, max_lag);
        let mut mediators = BTreeSet::new();
        for &y in end {
            let mut this_level = vec![y];
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    for (_, parent) in
                        self.adj_stationary(varlag, &[mark("<*-"), mark("<*+")], max_lag, &[])
                    {
                        if des_x.contains(&parent)
                            && !mediators.contains(&parent)
                            && !start.contains(&parent)
                            && !end.contains(&parent)
                        {
                            mediators.insert(parent);
                            next_level.push(parent);
                        }
                    }
                }
                this_level = next_level;
            }
        }
        mediators
    }

    /// `_check_path`: is there an open path from start to end given the conditions?
    #[allow(clippy::too_many_arguments)]
    fn check_path(
        &self,
        start: &BTreeSet<Node>,
        end: &BTreeSet<Node>,
        conditions: &BTreeSet<Node>,
        starts_with: &[Edge],
        ends_with: &[Edge],
        path_type: PathType,
        stationary_graph: bool,
        hidden_by_taumax: bool,
        hidden_variables: Option<&HashSet<Node>>,
    ) -> bool {
        let max_lag = 10 * self.tau_max as i32;
        let causal_children: BTreeSet<Node> = if stationary_graph {
            let mut m = self.get_mediators_stationary(start, end, max_lag);
            m.extend(end.iter().copied());
            m
        } else {
            let mut m = self.get_mediators(start, end);
            m.extend(end.iter().copied());
            m
        };

        // The latent projection hides everything beyond tau_max, out to the search horizon.
        let mut hidden_owned: HashSet<Node>;
        let hidden: Option<&HashSet<Node>> = if hidden_by_taumax {
            hidden_owned = hidden_variables.cloned().unwrap_or_default();
            for k in 0..self.n {
                for tauk in (self.tau_max as i32 + 1)..=max_lag {
                    hidden_owned.insert((k, -tauk));
                }
            }
            Some(&hidden_owned)
        } else {
            hidden_variables
        };

        let start_list: Vec<Node> = start.iter().copied().collect();
        let mut start_from: BTreeSet<(Node, Edge, Node)> = BTreeSet::new();
        for &x in start {
            let link_neighbors = if stationary_graph {
                self.adj_stationary(x, starts_with, max_lag, &start_list)
            } else {
                self.find_adj(x, starts_with, &start_list, &self.graph)
            };
            for (link, neighbor) in link_neighbors {
                if let Some(h) = hidden {
                    if !end.contains(&neighbor) && !h.contains(&neighbor) {
                        continue;
                    }
                }
                match path_type {
                    PathType::NonCausal => {
                        if causal_children.contains(&neighbor)
                            && match_link(mark("-*>"), link)
                            && !match_link(mark("+*>"), link)
                        {
                            continue;
                        }
                    }
                    PathType::Causal => {
                        if !causal_children.contains(&neighbor) {
                            continue;
                        }
                    }
                    PathType::Any => {}
                }
                start_from.insert((x, link, neighbor));
            }
        }

        let mut visited: HashSet<(Edge, Node)> = HashSet::new();
        for &(_, link_ik, varlag_k) in &start_from {
            visited.insert((link_ik, varlag_k));
        }

        // Traverse motifs i *-* k *-* j.
        while !start_from.is_empty() {
            let mut removables = Vec::new();
            for &(vi, link_ik, vk) in &start_from {
                if end.contains(&vk) {
                    if ends_with.iter().any(|&p| match_link(p, link_ik)) {
                        return true;
                    }
                    removables.push((vi, link_ik, vk));
                }
            }
            for r in removables {
                start_from.remove(&r);
            }
            if start_from.is_empty() {
                return false;
            }

            let popped = *start_from.iter().next_back().expect("non-empty");
            start_from.remove(&popped);
            let (_, link_ik, varlag_k) = popped;

            let link_neighbors = if stationary_graph {
                self.adj_stationary(varlag_k, &[mark("***")], max_lag, &start_list)
            } else {
                self.find_adj(varlag_k, &[mark("***")], &start_list, &self.graph)
            };
            for (link_kj, varlag_j) in link_neighbors {
                if visited.contains(&(link_kj, varlag_j)) {
                    continue;
                }
                if path_type == PathType::Causal
                    && !(match_link(mark("-*>"), link_kj) || match_link(mark("+*>"), link_kj))
                {
                    continue;
                }
                let left_mark = link_ik[2];
                let right_mark = link_kj[0];
                // A conditioned node with a tail on either side blocks the motif.
                if conditions.contains(&varlag_k) && (left_mark == b'-' || right_mark == b'-') {
                    continue;
                }
                // An unconditioned collider blocks the motif.
                if !conditions.contains(&varlag_k) && left_mark == b'>' && right_mark == b'<' {
                    continue;
                }
                if let Some(h) = hidden {
                    if !end.contains(&varlag_j) && !h.contains(&varlag_j) {
                        continue;
                    }
                }
                visited.insert((link_kj, varlag_j));
                start_from.insert((varlag_k, link_kj, varlag_j));
            }
        }
        false
    }

    /// `_get_latent_projection_graph(stationary=True)`: every ordered pair of nodes in the
    /// window is classified by whether a directed or a common-cause path connects them
    /// through nodes that are hidden, which here means outside the window.
    fn latent_projection(
        n: usize,
        tau_max: usize,
        stationary: &StationaryGraph,
        hidden_variables: &HashSet<Node>,
    ) -> TsgGraph {
        // A scaffold carrying only what check_path needs while the projection is built.
        let scaffold = CausalEffects {
            n,
            tau_max,
            graph: TsgGraph::new(n, tau_max),
            stationary: StationaryGraph {
                n,
                stat_lag: stationary.stat_lag,
                edges: stationary.edges.clone(),
            },
            x: BTreeSet::new(),
            y: BTreeSet::new(),
            s: BTreeSet::new(),
            listx: Vec::new(),
            listy: Vec::new(),
            mediators: BTreeSet::new(),
            no_causal_path: false,
            anc_x: BTreeSet::new(),
            anc_y: BTreeSet::new(),
            anc_s: BTreeSet::new(),
            descendants: BTreeSet::new(),
            forbidden_nodes: BTreeSet::new(),
            vancs: BTreeSet::new(),
        };

        let mut aux = TsgGraph::new(n, tau_max);
        let empty_conditions = BTreeSet::new();
        for i in 0..n {
            for j in 0..n {
                for tauj in 0..=tau_max {
                    for taui in 0..=tau_max {
                        if taui == tauj && i == j {
                            continue;
                        }
                        let ni = (i, -(taui as i32));
                        let nj = (j, -(tauj as i32));
                        if hidden_variables.contains(&ni) || hidden_variables.contains(&nj) {
                            continue;
                        }
                        let si: BTreeSet<Node> = [ni].into_iter().collect();
                        let sj: BTreeSet<Node> = [nj].into_iter().collect();

                        let cond_i_xy = scaffold.check_path(
                            &si,
                            &sj,
                            &empty_conditions,
                            &[mark("-*>"), mark("+*>")],
                            &[mark("-*>"), mark("+*>")],
                            PathType::Causal,
                            true,
                            false,
                            Some(hidden_variables),
                        );
                        let cond_i_yx = scaffold.check_path(
                            &sj,
                            &si,
                            &empty_conditions,
                            &[mark("-*>"), mark("+*>")],
                            &[mark("-*>"), mark("+*>")],
                            PathType::Causal,
                            true,
                            false,
                            Some(hidden_variables),
                        );
                        let cond_ii = scaffold.check_path(
                            &si,
                            &sj,
                            &empty_conditions,
                            &[mark("<**"), mark("+**")],
                            &[mark("**>"), mark("**+")],
                            PathType::Any,
                            true,
                            true,
                            Some(hidden_variables),
                        );

                        let (fwd, rev) = match (cond_i_xy, cond_i_yx, cond_ii) {
                            (true, false, false) => (mark("-->"), mark("<--")),
                            (false, true, false) => (mark("<--"), mark("-->")),
                            (false, false, true) => (mark("<->"), mark("<->")),
                            (true, false, true) => (mark("+->"), mark("<-+")),
                            (false, true, true) => (mark("<-+"), mark("+->")),
                            (true, true, _) => {
                                panic!("Cycle between ({i}, -{taui}) and ({j}, -{tauj})")
                            }
                            _ => continue,
                        };
                        aux.set(i, j, taui, tauj, fwd);
                        aux.set(j, i, tauj, taui, rev);
                    }
                }
            }
        }
        aux
    }

    /// `CausalEffects(graph, graph_type="stationary_dag", X, Y, S, hidden_variables)`.
    pub fn new(
        stationary: StationaryGraph,
        x: &[Node],
        y: &[Node],
        s: &[Node],
        hidden_variables: &[Node],
    ) -> Self {
        let n = stationary.n;
        let xs: BTreeSet<Node> = x.iter().copied().collect();
        let ys: BTreeSet<Node> = y.iter().copied().collect();
        let ss: BTreeSet<Node> = s.iter().copied().collect();
        let hidden: HashSet<Node> = hidden_variables.iter().copied().collect();

        // tau_max covers the stationary lag depth plus the deepest lag among X, Y and S.
        let maxlag_xys = xs
            .iter()
            .chain(&ys)
            .chain(&ss)
            .map(|v| v.1.unsigned_abs() as usize)
            .max()
            .unwrap_or(0);
        let tau_max = maxlag_xys + stationary.stat_lag;

        let graph = Self::latent_projection(n, tau_max, &stationary, &hidden);
        let mut ce = CausalEffects {
            n,
            tau_max,
            graph,
            stationary,
            x: xs,
            y: ys,
            s: ss,
            listx: x.to_vec(),
            listy: y.to_vec(),
            mediators: BTreeSet::new(),
            no_causal_path: false,
            anc_x: BTreeSet::new(),
            anc_y: BTreeSet::new(),
            anc_s: BTreeSet::new(),
            descendants: BTreeSet::new(),
            forbidden_nodes: BTreeSet::new(),
            vancs: BTreeSet::new(),
        };

        ce.anc_x = ce.get_ancestors(&ce.x.clone());
        ce.anc_y = ce.get_ancestors(&ce.y.clone());
        ce.anc_s = ce.get_ancestors(&ce.s.clone());
        ce.no_causal_path = ce.anc_y.intersection(&ce.x).next().is_none();

        ce.mediators = ce.get_mediators(&ce.x.clone(), &ce.y.clone());

        let des_y = ce.get_descendants(&ce.y.clone());
        let des_m = ce.get_descendants(&ce.mediators.clone());
        ce.descendants = des_y.union(&des_m).copied().collect();
        ce.forbidden_nodes = ce.descendants.union(&ce.x).copied().collect();
        ce.vancs = ce
            .anc_x
            .union(&ce.anc_y)
            .copied()
            .collect::<BTreeSet<_>>()
            .union(&ce.anc_s)
            .copied()
            .collect::<BTreeSet<_>>()
            .difference(&ce.forbidden_nodes)
            .copied()
            .collect();
        ce
    }

    /// `get_optimal_set()`. Returns `None` where Tigramite returns `False`.
    pub fn get_optimal_set(&self) -> Option<Vec<Node>> {
        self.get_optimal_set_with_minimization(OptimalSetMinimization::None)
    }

    /// `get_optimal_set(minimize=...)`, including the two minimization variants accepted
    /// by `fit_total_effect`.
    pub fn get_optimal_set_with_minimization(
        &self,
        minimization: OptimalSetMinimization,
    ) -> Option<Vec<Node>> {
        let (parents, colliders, collider_parents, s) = self.optimal_set_parts()?;
        let mut oset = parents.clone();
        oset.extend(colliders);
        oset.extend(collider_parents);

        if minimization != OptimalSetMinimization::None {
            let candidates = |set: &BTreeSet<Node>| match minimization {
                OptimalSetMinimization::CollidersOnly => {
                    set.difference(&parents).copied().collect::<Vec<_>>()
                }
                OptimalSetMinimization::All => set.iter().copied().collect(),
                OptimalSetMinimization::None => Vec::new(),
            };

            let removable: Vec<Node> = candidates(&oset)
                .into_iter()
                .filter(|node| {
                    let mut conditions = oset.clone();
                    conditions.remove(node);
                    conditions.extend(s.iter().copied());
                    let target = [*node].into_iter().collect();
                    !self.check_path(
                        &self.x,
                        &target,
                        &conditions,
                        &[mark("***")],
                        &[mark("***")],
                        PathType::Any,
                        false,
                        false,
                        None,
                    )
                })
                .collect();
            for node in removable {
                oset.remove(&node);
            }

            let removable: Vec<Node> = candidates(&oset)
                .into_iter()
                .filter(|node| {
                    let mut conditions = oset.clone();
                    conditions.remove(node);
                    conditions.extend(s.iter().copied());
                    conditions.extend(self.x.iter().copied());
                    let start = [*node].into_iter().collect();
                    !self.check_path(
                        &start,
                        &self.y,
                        &conditions,
                        &[mark("***")],
                        &[mark("**>"), mark("**+")],
                        PathType::Any,
                        false,
                        false,
                        None,
                    )
                })
                .collect();
            for node in removable {
                oset.remove(&node);
            }
        }

        oset.extend(s);
        Some(oset.into_iter().collect())
    }

    /// Tigramite's path-only check; explicit sets also require member validation.
    pub fn is_valid_adjustment_set(&self, adjustment_set: &[Node]) -> bool {
        let conditions = adjustment_set.iter().copied().collect();
        !self.check_path(
            &self.x,
            &self.y,
            &conditions,
            &[mark("***")],
            &[mark("***")],
            PathType::NonCausal,
            false,
            false,
            None,
        )
    }

    /// Check explicit members and noncausal-path blocking.
    pub fn explicit_adjustment_problems(
        &self,
        adjustment_set: &[Node],
    ) -> Vec<ExplicitAdjustmentProblem> {
        let mut problems = Vec::new();
        for &node in adjustment_set {
            if self.x.contains(&node) {
                problems.push(ExplicitAdjustmentProblem::QueryTreatment(node));
            } else if self.y.contains(&node) {
                problems.push(ExplicitAdjustmentProblem::QueryOutcome(node));
            } else if self
                .x
                .iter()
                .any(|&(variable, lag)| variable == node.0 && node.1 > lag)
            {
                problems.push(ExplicitAdjustmentProblem::LaterTreatmentOccurrence(node));
            } else if self.forbidden_nodes.contains(&node) {
                problems.push(ExplicitAdjustmentProblem::ForbiddenNode(node));
            }
        }
        if !self.is_valid_adjustment_set(adjustment_set) {
            problems.push(ExplicitAdjustmentProblem::OpenNonCausalPath);
        }
        problems
    }

    pub fn resolve_adjustment_set(
        &self,
        selection: &AdjustmentSetSelection,
    ) -> Result<Vec<Node>, AdjustmentSetError> {
        match selection {
            AdjustmentSetSelection::Optimal => self
                .get_optimal_set()
                .ok_or(AdjustmentSetError::NotIdentifiable),
            AdjustmentSetSelection::MinimizedOptimal => self
                .get_optimal_set_with_minimization(OptimalSetMinimization::All)
                .ok_or(AdjustmentSetError::NotIdentifiable),
            AdjustmentSetSelection::CollidersMinimizedOptimal => self
                .get_optimal_set_with_minimization(OptimalSetMinimization::CollidersOnly)
                .ok_or(AdjustmentSetError::NotIdentifiable),
            AdjustmentSetSelection::Explicit(adjustment_set) => {
                let problems = self.explicit_adjustment_problems(adjustment_set);
                if problems.is_empty() {
                    Ok(adjustment_set.clone())
                } else {
                    Err(AdjustmentSetError::InvalidExplicitSet { problems })
                }
            }
        }
    }

    /// The same computation, returning the parents, the collider path nodes, their parents
    /// and the conditions separately, as `return_separate_sets=True` does.
    pub fn optimal_set_parts(
        &self,
    ) -> Option<(
        BTreeSet<Node>,
        BTreeSet<Node>,
        BTreeSet<Node>,
        BTreeSet<Node>,
    )> {
        self.optimal_set_parts_given(&self.s, &self.vancs)
    }

    fn optimal_set_parts_given(
        &self,
        conditions: &BTreeSet<Node>,
        vancs: &BTreeSet<Node>,
    ) -> Option<(
        BTreeSet<Node>,
        BTreeSet<Node>,
        BTreeSet<Node>,
        BTreeSet<Node>,
    )> {
        // Overlap between X and the descendants is sufficient for non-identifiability.
        if self.x.intersection(&self.descendants).next().is_some() {
            return None;
        }

        let ym: BTreeSet<Node> = self.y.union(&self.mediators).copied().collect();
        let parents: BTreeSet<Node> = self
            .get_all_parents(&ym)
            .difference(&self.forbidden_nodes)
            .copied()
            .collect();

        let mut colliders: BTreeSet<Node> = BTreeSet::new();
        for &w in &ym {
            let mut this_level = vec![w];
            let mut non_suitable_nodes: Vec<Node> = Vec::new();
            while !this_level.is_empty() {
                let mut next_level = Vec::new();
                for varlag in this_level {
                    let suitable: Vec<Node> = self
                        .get_spouses(varlag)
                        .into_iter()
                        .filter(|sp| !non_suitable_nodes.contains(sp))
                        .collect();
                    for spouse in suitable {
                        let tau = spouse.1;
                        if self.x.contains(&spouse) {
                            return None;
                        }
                        let in_bounds = -(self.tau_max as i32) <= tau && tau <= 0;
                        let reachable = if vancs.contains(&spouse) {
                            true
                        } else {
                            let mut path_conditions: BTreeSet<Node> =
                                parents.union(vancs).copied().collect();
                            path_conditions.extend(conditions.iter().copied());
                            let target: BTreeSet<Node> = [spouse].into_iter().collect();
                            !self.check_path(
                                &self.x,
                                &target,
                                &path_conditions,
                                &[mark("***")],
                                &[mark("***")],
                                PathType::Any,
                                false,
                                false,
                                None,
                            )
                        };
                        if !colliders.contains(&spouse)
                            && !self.forbidden_nodes.contains(&spouse)
                            && in_bounds
                            && reachable
                        {
                            colliders.insert(spouse);
                            next_level.push(spouse);
                        } else if !colliders.contains(&spouse) {
                            non_suitable_nodes.push(spouse);
                        }
                    }
                }
                this_level = next_level
                    .into_iter()
                    .filter(|v| !non_suitable_nodes.contains(v))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect();
            }
        }

        let collider_parents = self.get_all_parents(&colliders);
        if self.x.intersection(&collider_parents).next().is_some() {
            return None;
        }
        Some((parents, colliders, collider_parents, conditions.clone()))
    }

    fn any_path(
        &self,
        start: &BTreeSet<Node>,
        end: &BTreeSet<Node>,
        conditions: &BTreeSet<Node>,
    ) -> bool {
        self.check_path(
            start,
            end,
            conditions,
            &[mark("***")],
            &[mark("***")],
            PathType::Any,
            false,
            false,
            None,
        )
    }

    fn has_unique_adjustment_set(&self) -> bool {
        let allowed: BTreeSet<_> = (0..self.n)
            .flat_map(|i| (0..=self.tau_max).map(move |t| (i, -(t as i32))))
            .filter(|v| !self.forbidden_nodes.contains(v))
            .collect();
        let mut stack = vec![(self.s.clone(), allowed)];
        let mut count = 0;
        while let Some((included, remaining)) = stack.pop() {
            let roots = self
                .x
                .union(&self.y)
                .copied()
                .chain(included.iter().copied())
                .collect();
            let ancestors = self.get_ancestors(&roots);
            let separator: Vec<_> = ancestors
                .intersection(&remaining)
                .copied()
                .filter(|v| !self.x.contains(v) && !self.y.contains(v))
                .collect();
            if !self.is_valid_adjustment_set(&separator) {
                continue;
            }
            if included == remaining {
                count += 1;
                if count == 2 {
                    return false;
                }
            } else if let Some(&v) = remaining.difference(&included).next() {
                let mut excluded = remaining.clone();
                excluded.remove(&v);
                stack.push((included.clone(), excluded));
                let mut added = included;
                added.insert(v);
                stack.push((added, remaining));
            }
        }
        count == 1
    }

    fn optimality_collider_paths(
        &self,
        sources: &BTreeSet<Node>,
        targets: &BTreeSet<Node>,
        inside: &BTreeSet<Node>,
        condition_i: bool,
    ) -> bool {
        for &source in sources {
            let mut stack = vec![(source, Vec::<Node>::new())];
            let mut invalid_subsets = Vec::<BTreeSet<Node>>::new();
            while let Some((node, mut path)) = stack.pop() {
                path.push(node);
                let mut suitable: BTreeSet<_> = self.get_spouses(node).into_iter().collect();
                if !condition_i && path.len() == 1 {
                    suitable.extend(self.get_children(node));
                }
                for next in suitable {
                    if next.1 < -(self.tau_max as i32) || next.1 > 0 || path.contains(&next) {
                        continue;
                    }
                    if !condition_i && !targets.contains(&next) && !self.vancs.contains(&next) {
                        continue;
                    }
                    if inside.contains(&next) {
                        let extended: BTreeSet<_> = path.iter().copied().chain([next]).collect();
                        if condition_i && invalid_subsets.iter().any(|s| s.is_subset(&extended)) {
                            continue;
                        }
                        stack.push((next, path.clone()));
                    }
                    if targets.contains(&next) {
                        if !condition_i {
                            return true;
                        }
                        let conditions: BTreeSet<_> =
                            self.s.iter().copied().chain(path.iter().copied()).collect();
                        let new_ancestors = self.get_ancestors(&conditions);
                        let vancs = self
                            .anc_x
                            .union(&self.anc_y)
                            .copied()
                            .chain(new_ancestors)
                            .filter(|v| !self.forbidden_nodes.contains(v))
                            .collect();
                        if self.optimal_set_parts_given(&conditions, &vancs).is_some() {
                            return false;
                        }
                        let pathset: BTreeSet<_> = path.iter().copied().collect();
                        stack.retain(|(q, p)| {
                            !pathset.is_subset(&p.iter().copied().chain([*q]).collect())
                        });
                        invalid_subsets.push(pathset);
                    }
                }
            }
        }
        condition_i
    }

    /// Runge (2021), Theorem 3; mirrors Tigramite's `check_optimality`.
    pub fn check_optimality(&self) -> Optimality {
        let Some((parents, colliders, collider_parents, _)) = self.optimal_set_parts() else {
            return Optimality::NotIdentifiable;
        };
        let unique_set = self.has_unique_adjustment_set();
        let oset: BTreeSet<_> = parents
            .union(&colliders)
            .copied()
            .chain(collider_parents)
            .collect();
        let targets: BTreeSet<_> = self.y.union(&self.mediators).copied().collect();
        let n_nodes = targets
            .union(&colliders)
            .flat_map(|&v| self.get_spouses(v))
            .filter(|v| {
                !self.forbidden_nodes.contains(v)
                    && !oset.contains(v)
                    && !self.s.contains(v)
                    && !targets.contains(v)
                    && !colliders.contains(v)
            })
            .collect();
        let inside = oset.union(&self.s).copied().collect();
        let condition_i = self.optimality_collider_paths(&n_nodes, &targets, &inside, true);
        let mut condition_ii = true;
        for &e in oset.difference(&parents) {
            let conditions = self
                .s
                .iter()
                .copied()
                .chain(oset.iter().copied().filter(|&v| v != e))
                .collect();
            let source = [e].into_iter().collect();
            if self.any_path(&self.x, &source, &conditions)
                && !self.optimality_collider_paths(&source, &targets, &inside, false)
            {
                condition_ii = false;
                break;
            }
        }
        Optimality::Checked {
            unique_set,
            condition_i,
            condition_ii,
        }
    }
}

// ---------------------------------------------------------------------------
// Total effect estimation: `fit_total_effect` and `predict_total_effect`.
// ---------------------------------------------------------------------------

use crate::nprandom::NpRng;
use crate::parcorr::{construct_array_general, CutOff, TimeSeries};
use nalgebra::{DMatrix, DVector};

/// The estimators accepted by `fit_total_effect` and its nested conditional model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Estimator {
    /// `sklearn.linear_model.LinearRegression()`.
    Linear,
    /// `sklearn.neighbors.KNeighborsRegressor(n_neighbors=k)` with uniform weights and the
    /// Euclidean metric.
    KNeighbors { k: usize },
}

#[derive(Clone)]
enum Fitted {
    Linear {
        intercept: f64,
        coef: DVector<f64>,
    },
    KNeighbors {
        k: usize,
        predictors: Vec<Vec<f64>>,
        targets: Vec<f64>,
    },
}

#[derive(Clone)]
pub struct TotalEffectModel {
    fitted: Fitted,
    /// The adjustment set columns of the observation array, rows by variables.
    z_array: Vec<Vec<f64>>,
    /// The observed conditioning-set design used to fit each nested model, sample-major.
    s_predictors: Vec<Vec<f64>>,
    len_x: usize,
    len_s: usize,
    conditional_estimator: Estimator,
    pub adjustment_set: Vec<Node>,
    pub n_obs: usize,
}

/// Tigramite's `boot_blocklength` options that are implemented by `construct_array`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootstrapBlockLength {
    Fixed(usize),
    CubeRoot,
}

/// Settings for `CausalEffects.fit_bootstrap_of("fit_total_effect", ...)`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TotalEffectBootstrapOptions {
    pub samples: usize,
    pub block_length: BootstrapBlockLength,
    /// Hirmos requires the optional Tigramite seed to be supplied so the run is replayable.
    pub seed: u64,
}

impl Default for TotalEffectBootstrapOptions {
    fn default() -> Self {
        Self {
            samples: 100,
            block_length: BootstrapBlockLength::Fixed(1),
            seed: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TotalEffectBootstrapError {
    NotIdentifiable,
    ZeroSamples,
    ZeroBlockLength,
    TooFewBlocks { blocks: usize },
    SeedOverflow,
}

/// Fitted original and bootstrap models. `block_starts` is retained in the exact-run record.
/// and makes Tigramite's resampling path directly testable.
pub struct TotalEffectBootstrap {
    pub original_model: TotalEffectModel,
    bootstrap_models: Vec<TotalEffectModel>,
    pub block_starts: Vec<Vec<usize>>,
    pub resolved_block_length: usize,
    pub options: TotalEffectBootstrapOptions,
}

/// Output of `predict_bootstrap_of("predict_total_effect", ...)`.
pub struct TotalEffectBootstrapPrediction {
    /// One prediction vector per fitted bootstrap model.
    pub individual_predictions: Vec<Vec<f64>>,
    /// Equal-tail percentile bounds, ordered `[lower, upper][intervention]`.
    pub confidence_interval: Vec<Vec<f64>>,
}

/// Which proper directed paths contribute to a Wright effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WrightMediation {
    /// Every proper directed path from X to Y.
    Total,
    /// Only the direct X -> Y path.
    Direct,
    /// Paths passing through at least one of the supplied time-indexed mediators.
    Through(Vec<Node>),
}

/// The two working coefficient sources in Tigramite's `fit_wright_effect`.
#[derive(Clone, Debug, PartialEq)]
pub enum WrightCoefficientMethod {
    /// Fit every mediator/outcome on all of its parents. Valid only when those nodes have no
    /// bidirected spouse in the projected graph.
    Parents,
    /// Use the stationary link coefficients supplied by the caller, keyed by child variable.
    Links(BTreeMap<usize, Vec<(Node, f64)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WrightCoefficient {
    pub parent: Node,
    pub child: Node,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WrightPathContribution {
    pub source: Node,
    pub target: Node,
    pub path: Vec<Node>,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WrightEffectError {
    ParentMethodHasBidirectedLink { node: Node },
    MissingLinkCoefficients { variable: usize },
    MissingPathCoefficient { parent: Node, child: Node },
    ZeroSamples,
    ZeroBlockLength,
    TooFewBlocks { blocks: usize },
    SeedOverflow,
}

/// A fitted Wright model. The effect for each `(X, Y)` pair is the sum of the retained path
/// contributions; prediction is the dot product of intervention values and those effects.
#[derive(Clone, Debug)]
pub struct WrightEffectModel {
    pub effects: BTreeMap<(Node, Node), f64>,
    pub coefficients: Vec<WrightCoefficient>,
    pub paths: Vec<WrightPathContribution>,
    pub mediation: WrightMediation,
    pub n_obs: usize,
    listx: Vec<Node>,
    listy: Vec<Node>,
}

pub struct WrightEffectBootstrap {
    pub original_model: WrightEffectModel,
    bootstrap_models: Vec<WrightEffectModel>,
    pub block_starts: Vec<Vec<usize>>,
    pub resolved_block_length: usize,
    pub options: TotalEffectBootstrapOptions,
}

pub struct WrightEffectBootstrapPrediction {
    pub individual_predictions: Vec<Vec<Vec<f64>>>,
    /// Equal-tail percentile bounds, ordered `[lower, upper][intervention][outcome]`.
    pub confidence_interval: Vec<Vec<Vec<f64>>>,
}

struct TotalEffectDesign {
    predictors: Vec<Vec<f64>>,
    targets: Vec<f64>,
    z_rows: Vec<Vec<f64>>,
    s_predictors: Vec<Vec<f64>>,
    len_x: usize,
    len_s: usize,
    adjustment_set: Vec<Node>,
}

/// `LinearRegression().fit()`: centre, solve by least squares, recover the intercept.
fn linear_fit(predictors: &[Vec<f64>], targets: &[f64]) -> Fitted {
    let n = predictors.len();
    let k = predictors[0].len();
    let design = DMatrix::from_fn(n, k, |row, column| predictors[row][column]);
    let target = DVector::from_column_slice(targets);
    let fit = crate::sklearn_linear::fit_sklearn_linear_regression(
        &design,
        &target,
        crate::sklearn_linear::SKLEARN_LINEAR_TOLERANCE,
    )
    .expect("finite total-effect linear-regression inputs");
    Fitted::Linear {
        intercept: fit.intercept,
        coef: fit.coefficients,
    }
}

/// Uniform weights, so the prediction is the mean of the k nearest targets. Ties keep the lower
/// row, which is what `KNeighborsRegressor` does.
pub fn k_neighbors_predict(predictors: &[Vec<f64>], targets: &[f64], k: usize, row: &[f64]) -> f64 {
    let mut d: Vec<(f64, usize)> = predictors
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let sq: f64 = p.iter().zip(row).map(|(a, b)| (a - b) * (a - b)).sum();
            (sq, i)
        })
        .collect();
    d.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .expect("no NaN distances")
            .then(a.1.cmp(&b.1))
    });
    d[..k].iter().map(|&(_, i)| targets[i]).sum::<f64>() / k as f64
}

fn predict_one(fitted: &Fitted, row: &[f64]) -> f64 {
    match fitted {
        Fitted::Linear { intercept, coef } => {
            intercept + (0..coef.len()).map(|j| coef[j] * row[j]).sum::<f64>()
        }
        Fitted::KNeighbors {
            k,
            predictors,
            targets,
        } => k_neighbors_predict(predictors, targets, *k, row),
    }
}

fn fit_estimator(estimator: Estimator, predictors: &[Vec<f64>], targets: &[f64]) -> Fitted {
    match estimator {
        Estimator::Linear => linear_fit(predictors, targets),
        Estimator::KNeighbors { k } => {
            assert!(
                k > 0 && k <= predictors.len(),
                "invalid number of neighbors"
            );
            Fitted::KNeighbors {
                k,
                predictors: predictors.to_vec(),
                targets: targets.to_vec(),
            }
        }
    }
}

impl CausalEffects {
    /// `fit_total_effect(dataframe, estimator, adjustment_set="optimal")` for a single Y.
    pub fn fit_total_effect(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
    ) -> Option<TotalEffectModel> {
        self.fit_total_effect_with_conditional_estimator(data, estimator, None)
    }

    /// `fit_total_effect(..., conditional_estimator=...)`. When no conditional estimator is
    /// supplied, Tigramite clones the first-stage estimator for the nested regression.
    pub fn fit_total_effect_with_conditional_estimator(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
    ) -> Option<TotalEffectModel> {
        let design = self.total_effect_design(data)?;
        Some(fit_total_effect_design(
            &design,
            estimator,
            conditional_estimator.unwrap_or(estimator),
            None,
        ))
    }

    /// `fit_total_effect(..., adjustment_set=...)` with Tigramite's complete adjustment-set
    /// selection surface. Explicit sets are rejected when `_check_validity` rejects them.
    pub fn fit_total_effect_with_adjustment_set(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
        selection: &AdjustmentSetSelection,
    ) -> Result<TotalEffectModel, AdjustmentSetError> {
        let design = self.total_effect_design_with_adjustment_set(data, selection)?;
        Ok(fit_total_effect_design(
            &design,
            estimator,
            conditional_estimator.unwrap_or(estimator),
            None,
        ))
    }

    fn total_effect_design(&self, data: &TimeSeries) -> Option<TotalEffectDesign> {
        self.total_effect_design_with_adjustment_set(data, &AdjustmentSetSelection::Optimal)
            .ok()
    }

    fn total_effect_design_with_adjustment_set(
        &self,
        data: &TimeSeries,
        selection: &AdjustmentSetSelection,
    ) -> Result<TotalEffectDesign, AdjustmentSetError> {
        if self.no_causal_path {
            return Err(AdjustmentSetError::NotIdentifiable);
        }
        let adjustment_set = self.resolve_adjustment_set(selection)?;
        // construct_array takes S as Z and the adjustment set as extraZ, so the rows come
        // back ordered X, Y, S, adjustment set.
        let s_list: Vec<Node> = self.s.iter().copied().collect();
        let ((array, cleaned), extra_z) = construct_array_general(
            data,
            &self.listx,
            &self.listy,
            &s_list,
            &adjustment_set,
            self.tau_max,
            CutOff::TauMax,
            None,
            None,
        );
        let len_x = cleaned.x.len();
        let len_y = cleaned.y.len();
        let len_s = cleaned.z.len();
        let n_obs = array[0].len();

        // Predictors are X, then the adjustment set, then the conditions.
        let z_rows: Vec<Vec<f64>> = (0..extra_z.len())
            .map(|i| array[len_x + len_y + len_s + i].clone())
            .collect();
        let s_rows: Vec<Vec<f64>> = (0..len_s)
            .map(|i| array[len_x + len_y + i].clone())
            .collect();
        let predictors: Vec<Vec<f64>> = (0..n_obs)
            .map(|t| {
                let mut row: Vec<f64> = (0..len_x).map(|i| array[i][t]).collect();
                row.extend(z_rows.iter().map(|r| r[t]));
                row.extend(s_rows.iter().map(|r| r[t]));
                row
            })
            .collect();
        let s_predictors: Vec<Vec<f64>> = (0..n_obs)
            .map(|t| s_rows.iter().map(|row| row[t]).collect())
            .collect();
        let targets: Vec<f64> = (0..n_obs).map(|t| array[len_x][t]).collect();

        Ok(TotalEffectDesign {
            predictors,
            targets,
            z_rows,
            s_predictors,
            len_x,
            len_s,
            // Tigramite exposes the O-set before Models removes its overlap with S.
            adjustment_set,
        })
    }

    /// Seeded block bootstrap for `fit_total_effect`, matching Tigramite's
    /// `fit_bootstrap_of` seed schedule and `DataFrame.construct_array` resampling order.
    pub fn fit_bootstrap_total_effect(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
        options: TotalEffectBootstrapOptions,
    ) -> Result<TotalEffectBootstrap, TotalEffectBootstrapError> {
        self.fit_bootstrap_total_effect_with_progress(
            data,
            estimator,
            conditional_estimator,
            options,
            |_, _| {},
        )
    }

    /// Bootstrap `fit_total_effect` while preserving the selected generated or explicit
    /// adjustment set in every refit.
    pub fn fit_bootstrap_total_effect_with_adjustment_set(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
        selection: &AdjustmentSetSelection,
        options: TotalEffectBootstrapOptions,
    ) -> Result<TotalEffectBootstrap, TotalEffectBootstrapError> {
        self.fit_bootstrap_total_effect_with_adjustment_set_and_progress(
            data,
            estimator,
            conditional_estimator,
            selection,
            options,
            |_, _| {},
        )
    }

    /// Progress-aware form of [`Self::fit_bootstrap_total_effect`]. The callback receives
    /// `(completed_bootstrap_models, total_bootstrap_models)` after each successful refit.
    pub fn fit_bootstrap_total_effect_with_progress<F>(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
        options: TotalEffectBootstrapOptions,
        progress: F,
    ) -> Result<TotalEffectBootstrap, TotalEffectBootstrapError>
    where
        F: FnMut(usize, usize),
    {
        self.fit_bootstrap_total_effect_with_adjustment_set_and_progress(
            data,
            estimator,
            conditional_estimator,
            &AdjustmentSetSelection::Optimal,
            options,
            progress,
        )
    }

    /// Progress-aware form of [`Self::fit_bootstrap_total_effect_with_adjustment_set`].
    pub fn fit_bootstrap_total_effect_with_adjustment_set_and_progress<F>(
        &self,
        data: &TimeSeries,
        estimator: Estimator,
        conditional_estimator: Option<Estimator>,
        selection: &AdjustmentSetSelection,
        options: TotalEffectBootstrapOptions,
        mut progress: F,
    ) -> Result<TotalEffectBootstrap, TotalEffectBootstrapError>
    where
        F: FnMut(usize, usize),
    {
        if options.samples == 0 {
            return Err(TotalEffectBootstrapError::ZeroSamples);
        }
        let design = self
            .total_effect_design_with_adjustment_set(data, selection)
            .map_err(|error| match error {
                AdjustmentSetError::NotIdentifiable => TotalEffectBootstrapError::NotIdentifiable,
                AdjustmentSetError::InvalidExplicitSet { .. } => {
                    TotalEffectBootstrapError::NotIdentifiable
                }
            })?;
        let n_obs = design.predictors.len();
        let resolved_block_length = match options.block_length {
            BootstrapBlockLength::Fixed(0) => {
                return Err(TotalEffectBootstrapError::ZeroBlockLength)
            }
            BootstrapBlockLength::Fixed(value) => value,
            BootstrapBlockLength::CubeRoot => ((n_obs as f64).powf(1.0 / 3.0) as usize).max(1),
        };
        let blocks = n_obs.div_ceil(resolved_block_length);
        if blocks < 2 {
            return Err(TotalEffectBootstrapError::TooFewBlocks { blocks });
        }

        let conditional_estimator = conditional_estimator.unwrap_or(estimator);
        let original_model =
            fit_total_effect_design(&design, estimator, conditional_estimator, None);
        let mut bootstrap_models = Vec::with_capacity(options.samples);
        let mut all_starts = Vec::with_capacity(options.samples);
        for bootstrap_index in 0..options.samples {
            let seed = options
                .seed
                .checked_mul(options.samples as u64)
                .and_then(|value| value.checked_add(bootstrap_index as u64))
                .ok_or(TotalEffectBootstrapError::SeedOverflow)?;
            let starts = bootstrap_block_starts(n_obs, resolved_block_length, seed)?;
            let sample_indices = bootstrap_sample_indices(n_obs, resolved_block_length, &starts);
            bootstrap_models.push(fit_total_effect_design(
                &design,
                estimator,
                conditional_estimator,
                Some(&sample_indices),
            ));
            all_starts.push(starts);
            progress(bootstrap_index + 1, options.samples);
        }

        Ok(TotalEffectBootstrap {
            original_model,
            bootstrap_models,
            block_starts: all_starts,
            resolved_block_length,
            options,
        })
    }

    /// Tigramite's working `fit_wright_effect` branches: `method="parents"` and
    /// `method="links_coeffs"`. The upstream `method="optimal"` branch stores a coefficient
    /// vector where a scalar edge coefficient is required and is deliberately not represented by
    /// this parity API.
    pub fn fit_wright_effect(
        &self,
        data: &TimeSeries,
        method: WrightCoefficientMethod,
        mediation: WrightMediation,
    ) -> Result<WrightEffectModel, WrightEffectError> {
        self.fit_wright_effect_indexed(data, &method, &mediation, None)
    }

    fn fit_wright_effect_indexed(
        &self,
        data: &TimeSeries,
        method: &WrightCoefficientMethod,
        mediation: &WrightMediation,
        sample_indices: Option<&[usize]>,
    ) -> Result<WrightEffectModel, WrightEffectError> {
        let paths = self.wright_paths(mediation);
        let mut coefficient_map: BTreeMap<Node, BTreeMap<Node, f64>> = BTreeMap::new();
        let mut fitted_observations = data.t.saturating_sub(self.tau_max);
        let fitted_nodes: BTreeSet<Node> = self
            .mediators
            .iter()
            .copied()
            .chain(self.listy.iter().copied())
            .collect();

        match method {
            WrightCoefficientMethod::Parents => {
                for child in fitted_nodes {
                    if !self.get_spouses(child).is_empty() {
                        return Err(WrightEffectError::ParentMethodHasBidirectedLink {
                            node: child,
                        });
                    }
                    let parents: Vec<Node> = self.get_parents(child);
                    if parents.is_empty() {
                        coefficient_map.insert(child, BTreeMap::new());
                        continue;
                    }
                    let (coefficients, n_obs) = fit_wright_edge_regression(
                        data,
                        &parents,
                        child,
                        &[],
                        self.tau_max,
                        sample_indices,
                    );
                    fitted_observations = n_obs;
                    coefficient_map.insert(child, coefficients);
                }
            }
            WrightCoefficientMethod::Links(links) => {
                for child in fitted_nodes {
                    let supplied = links
                        .get(&child.0)
                        .ok_or(WrightEffectError::MissingLinkCoefficients { variable: child.0 })?;
                    let shifted = supplied
                        .iter()
                        .map(|&((parent, lag), coefficient)| ((parent, lag + child.1), coefficient))
                        .collect();
                    coefficient_map.insert(child, shifted);
                }
            }
        }

        let mut effects = BTreeMap::new();
        let mut contributions = Vec::new();
        for &source in &self.listx {
            for &target in &self.listy {
                let mut total = 0.0;
                for path in paths.get(&(source, target)).into_iter().flatten() {
                    let mut value = 1.0;
                    for edge in path.windows(2) {
                        let parent = edge[0];
                        let child = edge[1];
                        let coefficient = coefficient_map
                            .get(&child)
                            .and_then(|parents| parents.get(&parent))
                            .copied()
                            .ok_or(WrightEffectError::MissingPathCoefficient { parent, child })?;
                        value *= coefficient;
                    }
                    total += value;
                    contributions.push(WrightPathContribution {
                        source,
                        target,
                        path: path.clone(),
                        value,
                    });
                }
                effects.insert((source, target), total);
            }
        }

        let coefficients = coefficient_map
            .iter()
            .flat_map(|(&child, parents)| {
                parents
                    .iter()
                    .map(move |(&parent, &value)| WrightCoefficient {
                        parent,
                        child,
                        value,
                    })
            })
            .collect();
        Ok(WrightEffectModel {
            effects,
            coefficients,
            paths: contributions,
            mediation: mediation.clone(),
            n_obs: fitted_observations,
            listx: self.listx.clone(),
            listy: self.listy.clone(),
        })
    }

    fn wright_paths(&self, mediation: &WrightMediation) -> BTreeMap<(Node, Node), Vec<Vec<Node>>> {
        let mut result = BTreeMap::new();
        if *mediation == WrightMediation::Direct {
            for &source in &self.listx {
                for &target in &self.listy {
                    let paths = if self.get_parents(target).contains(&source) {
                        vec![vec![source, target]]
                    } else {
                        Vec::new()
                    };
                    result.insert((source, target), paths);
                }
            }
            return result;
        }

        let inside: BTreeSet<Node> = self
            .mediators
            .union(&self.y)
            .copied()
            .filter(|node| !self.x.contains(node))
            .collect();
        let through: BTreeSet<Node> = match mediation {
            WrightMediation::Through(nodes) => nodes.iter().copied().collect(),
            WrightMediation::Total => BTreeSet::new(),
            WrightMediation::Direct => unreachable!("handled above"),
        };

        for &source in &self.listx {
            for &target in &self.listy {
                let mut found = Vec::new();
                let mut queue = vec![(source, Vec::<Node>::new())];
                while let Some((node, prefix)) = queue.pop() {
                    let mut path = prefix;
                    path.push(node);
                    let children: BTreeSet<Node> = self
                        .get_children(node)
                        .into_iter()
                        .filter(|child| inside.contains(child))
                        .collect();
                    for child in children {
                        if child.1 < -(self.tau_max as i32) || child.1 > 0 || path.contains(&child)
                        {
                            continue;
                        }
                        queue.push((child, path.clone()));
                        if child == target
                            && (through.is_empty() || path.iter().any(|n| through.contains(n)))
                        {
                            let mut completed = path.clone();
                            completed.push(child);
                            found.push(completed);
                        }
                    }
                }
                found.sort();
                result.insert((source, target), found);
            }
        }
        result
    }

    pub fn fit_bootstrap_wright_effect(
        &self,
        data: &TimeSeries,
        method: WrightCoefficientMethod,
        mediation: WrightMediation,
        options: TotalEffectBootstrapOptions,
    ) -> Result<WrightEffectBootstrap, WrightEffectError> {
        self.fit_bootstrap_wright_effect_with_progress(data, method, mediation, options, |_, _| {})
    }

    pub fn fit_bootstrap_wright_effect_with_progress<F>(
        &self,
        data: &TimeSeries,
        method: WrightCoefficientMethod,
        mediation: WrightMediation,
        options: TotalEffectBootstrapOptions,
        mut progress: F,
    ) -> Result<WrightEffectBootstrap, WrightEffectError>
    where
        F: FnMut(usize, usize),
    {
        if options.samples == 0 {
            return Err(WrightEffectError::ZeroSamples);
        }
        let n_obs = data.t.saturating_sub(self.tau_max);
        let resolved_block_length = match options.block_length {
            BootstrapBlockLength::Fixed(0) => return Err(WrightEffectError::ZeroBlockLength),
            BootstrapBlockLength::Fixed(value) => value,
            BootstrapBlockLength::CubeRoot => ((n_obs as f64).powf(1.0 / 3.0) as usize).max(1),
        };
        let blocks = n_obs.div_ceil(resolved_block_length);
        if blocks < 2 {
            return Err(WrightEffectError::TooFewBlocks { blocks });
        }

        let original_model = self.fit_wright_effect_indexed(data, &method, &mediation, None)?;
        let mut bootstrap_models = Vec::with_capacity(options.samples);
        let mut all_starts = Vec::with_capacity(options.samples);
        for bootstrap_index in 0..options.samples {
            let seed = options
                .seed
                .checked_mul(options.samples as u64)
                .and_then(|value| value.checked_add(bootstrap_index as u64))
                .ok_or(WrightEffectError::SeedOverflow)?;
            let starts =
                bootstrap_block_starts(n_obs, resolved_block_length, seed).map_err(|error| {
                    match error {
                        TotalEffectBootstrapError::ZeroBlockLength => {
                            WrightEffectError::ZeroBlockLength
                        }
                        TotalEffectBootstrapError::TooFewBlocks { blocks } => {
                            WrightEffectError::TooFewBlocks { blocks }
                        }
                        TotalEffectBootstrapError::SeedOverflow => WrightEffectError::SeedOverflow,
                        TotalEffectBootstrapError::ZeroSamples
                        | TotalEffectBootstrapError::NotIdentifiable => {
                            unreachable!("block-start validation cannot return this error")
                        }
                    }
                })?;
            let sample_indices = bootstrap_sample_indices(n_obs, resolved_block_length, &starts);
            bootstrap_models.push(self.fit_wright_effect_indexed(
                data,
                &method,
                &mediation,
                Some(&sample_indices),
            )?);
            all_starts.push(starts);
            progress(bootstrap_index + 1, options.samples);
        }
        Ok(WrightEffectBootstrap {
            original_model,
            bootstrap_models,
            block_starts: all_starts,
            resolved_block_length,
            options,
        })
    }
}

fn fit_wright_edge_regression(
    data: &TimeSeries,
    parents: &[Node],
    child: Node,
    adjustment: &[Node],
    tau_max: usize,
    sample_indices: Option<&[usize]>,
) -> (BTreeMap<Node, f64>, usize) {
    let ((array, cleaned), extra_z) = construct_array_general(
        data,
        parents,
        &[child],
        &[],
        adjustment,
        tau_max,
        CutOff::TauMax,
        None,
        None,
    );
    let base_n_obs = array[0].len();
    let indices: Vec<usize> = sample_indices
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| (0..base_n_obs).collect());
    let predictor_rows: Vec<&Vec<f64>> = (0..cleaned.x.len())
        .map(|index| &array[index])
        .chain(
            extra_z
                .iter()
                .enumerate()
                .map(|(index, _)| &array[cleaned.x.len() + cleaned.y.len() + index]),
        )
        .collect();
    let predictors: Vec<Vec<f64>> = indices
        .iter()
        .map(|&sample| predictor_rows.iter().map(|row| row[sample]).collect())
        .collect();
    let targets: Vec<f64> = indices
        .iter()
        .map(|&sample| array[cleaned.x.len()][sample])
        .collect();
    let Fitted::Linear { coef, .. } = linear_fit(&predictors, &targets) else {
        unreachable!("Wright always uses linear regression")
    };
    let names: Vec<Node> = cleaned.x.iter().copied().chain(extra_z).collect();
    (
        names
            .into_iter()
            .enumerate()
            .map(|(index, node)| (node, coef[index]))
            .collect(),
        indices.len(),
    )
}

fn fit_total_effect_design(
    design: &TotalEffectDesign,
    estimator: Estimator,
    conditional_estimator: Estimator,
    sample_indices: Option<&[usize]>,
) -> TotalEffectModel {
    let indices: Vec<usize> = sample_indices
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| (0..design.predictors.len()).collect());
    let predictors: Vec<Vec<f64>> = indices
        .iter()
        .map(|&index| design.predictors[index].clone())
        .collect();
    let targets: Vec<f64> = indices.iter().map(|&index| design.targets[index]).collect();
    let z_array: Vec<Vec<f64>> = design
        .z_rows
        .iter()
        .map(|row| indices.iter().map(|&index| row[index]).collect())
        .collect();
    let s_predictors: Vec<Vec<f64>> = indices
        .iter()
        .map(|&index| design.s_predictors[index].clone())
        .collect();
    let fitted = fit_estimator(estimator, &predictors, &targets);
    TotalEffectModel {
        fitted,
        z_array,
        s_predictors,
        len_x: design.len_x,
        len_s: design.len_s,
        conditional_estimator,
        adjustment_set: design.adjustment_set.clone(),
        n_obs: indices.len(),
    }
}

/// Starting positions from Tigramite's
/// `Generator.choice(arange(n_obs - block_length), size=ceil(n_obs/block_length))`.
pub fn bootstrap_block_starts(
    n_obs: usize,
    block_length: usize,
    seed: u64,
) -> Result<Vec<usize>, TotalEffectBootstrapError> {
    if block_length == 0 {
        return Err(TotalEffectBootstrapError::ZeroBlockLength);
    }
    let blocks = n_obs.div_ceil(block_length);
    if blocks < 2 {
        return Err(TotalEffectBootstrapError::TooFewBlocks { blocks });
    }
    let high = n_obs - block_length;
    let mut rng = NpRng::seeded(seed);
    Ok((0..blocks)
        .map(|_| rng.bounded_uint64(high as u64) as usize)
        .collect())
}

/// Expand circular block starts into the reference-point order used by Tigramite's bootstrap.
pub fn bootstrap_sample_indices(n_obs: usize, block_length: usize, starts: &[usize]) -> Vec<usize> {
    let mut indices = Vec::with_capacity(starts.len() * block_length);
    for &start in starts {
        for offset in 0..block_length {
            indices.push(start + offset);
        }
    }
    indices.truncate(n_obs);
    indices
}

impl TotalEffectModel {
    /// `predict_total_effect(intervention_data)`: hold X at the intervention value across
    /// every observed row of the adjustment set, predict, and take the mean.
    pub fn predict_total_effect(&self, intervention_data: &[Vec<f64>]) -> Vec<f64> {
        assert_eq!(
            self.len_s, 0,
            "S is non-empty; use predict_total_effect_with_conditions"
        );
        self.predict_total_effect_general(intervention_data, None)
    }

    /// `predict_total_effect(intervention_data, conditions_data=...)` for non-empty S.
    /// For each intervention and condition, predict over every observed adjustment row,
    /// regress those predictions on the observed S rows with a fresh conditional estimator,
    /// and evaluate that nested model at the requested condition.
    pub fn predict_total_effect_with_conditions(
        &self,
        intervention_data: &[Vec<f64>],
        conditions_data: &[Vec<f64>],
    ) -> Vec<f64> {
        assert!(self.len_s > 0, "conditions_data specified, but S is empty");
        assert_eq!(
            conditions_data.len(),
            intervention_data.len(),
            "conditions and interventions must have the same row count"
        );
        self.predict_total_effect_general(intervention_data, Some(conditions_data))
    }

    fn predict_total_effect_general(
        &self,
        intervention_data: &[Vec<f64>],
        conditions_data: Option<&[Vec<f64>]>,
    ) -> Vec<f64> {
        intervention_data
            .iter()
            .enumerate()
            .map(|(index, dox)| {
                assert_eq!(dox.len(), self.len_x, "intervention width must match X");
                let condition = conditions_data.map(|rows| &rows[index]);
                if let Some(values) = condition {
                    assert_eq!(values.len(), self.len_s, "condition width must match S");
                }
                let predicted: Vec<f64> = (0..self.n_obs)
                    .map(|t| {
                        let mut row = dox.clone();
                        row.extend(self.z_array.iter().map(|r| r[t]));
                        if let Some(values) = condition {
                            row.extend(values);
                        }
                        predict_one(&self.fitted, &row)
                    })
                    .collect();

                if let Some(values) = condition {
                    let conditional =
                        fit_estimator(self.conditional_estimator, &self.s_predictors, &predicted);
                    predict_one(&conditional, values)
                } else {
                    predicted.iter().sum::<f64>() / self.n_obs as f64
                }
            })
            .collect()
    }
}

impl TotalEffectBootstrap {
    /// Tigramite's `predict_bootstrap_of("predict_total_effect", ...)`, including NumPy's
    /// default linear percentile interpolation.
    pub fn predict_total_effect(
        &self,
        intervention_data: &[Vec<f64>],
        confidence_level: f64,
    ) -> TotalEffectBootstrapPrediction {
        assert!(
            confidence_level > 0.0 && confidence_level < 1.0,
            "confidence_level must be between zero and one"
        );
        let individual_predictions: Vec<Vec<f64>> = self
            .bootstrap_models
            .iter()
            .map(|model| model.predict_total_effect(intervention_data))
            .collect();
        bootstrap_prediction_interval(&individual_predictions, confidence_level)
    }

    /// Conditional counterpart for a non-empty S set.
    pub fn predict_total_effect_with_conditions(
        &self,
        intervention_data: &[Vec<f64>],
        conditions_data: &[Vec<f64>],
        confidence_level: f64,
    ) -> TotalEffectBootstrapPrediction {
        assert!(
            confidence_level > 0.0 && confidence_level < 1.0,
            "confidence_level must be between zero and one"
        );
        let individual_predictions: Vec<Vec<f64>> = self
            .bootstrap_models
            .iter()
            .map(|model| {
                model.predict_total_effect_with_conditions(intervention_data, conditions_data)
            })
            .collect();
        bootstrap_prediction_interval(&individual_predictions, confidence_level)
    }
}

impl WrightEffectModel {
    /// `predict_wright_effect`: linear intervention values multiplied by the path-summed effects.
    /// Like Tigramite, this reports a change model with no intercept.
    pub fn predict_wright_effect(&self, intervention_data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        intervention_data
            .iter()
            .map(|row| {
                assert_eq!(
                    row.len(),
                    self.listx.len(),
                    "intervention width must match X"
                );
                self.listy
                    .iter()
                    .map(|&target| {
                        self.listx
                            .iter()
                            .enumerate()
                            .map(|(index, &source)| {
                                row[index]
                                    * self.effects.get(&(source, target)).copied().unwrap_or(0.0)
                            })
                            .sum()
                    })
                    .collect()
            })
            .collect()
    }
}

impl WrightEffectBootstrap {
    pub fn predict_wright_effect(
        &self,
        intervention_data: &[Vec<f64>],
        confidence_level: f64,
    ) -> WrightEffectBootstrapPrediction {
        assert!(
            confidence_level > 0.0 && confidence_level < 1.0,
            "confidence_level must be between zero and one"
        );
        let individual_predictions: Vec<Vec<Vec<f64>>> = self
            .bootstrap_models
            .iter()
            .map(|model| model.predict_wright_effect(intervention_data))
            .collect();
        let interventions = intervention_data.len();
        let outcomes = self.original_model.listy.len();
        let tail = (1.0 - confidence_level) / 2.0;
        let mut confidence_interval = vec![vec![vec![0.0; outcomes]; interventions]; 2];
        for intervention in 0..interventions {
            for outcome in 0..outcomes {
                let values: Vec<f64> = individual_predictions
                    .iter()
                    .map(|draw| draw[intervention][outcome])
                    .collect();
                confidence_interval[0][intervention][outcome] = numpy_percentile(&values, tail);
                confidence_interval[1][intervention][outcome] =
                    numpy_percentile(&values, 1.0 - tail);
            }
        }
        WrightEffectBootstrapPrediction {
            individual_predictions,
            confidence_interval,
        }
    }
}

fn bootstrap_prediction_interval(
    individual_predictions: &[Vec<f64>],
    confidence_level: f64,
) -> TotalEffectBootstrapPrediction {
    let width = individual_predictions[0].len();
    assert!(
        individual_predictions.iter().all(|row| row.len() == width),
        "bootstrap prediction widths differ"
    );
    let tail = (1.0 - confidence_level) / 2.0;
    let mut confidence_interval = vec![vec![0.0; width]; 2];
    for column in 0..width {
        let values: Vec<f64> = individual_predictions
            .iter()
            .map(|row| row[column])
            .collect();
        confidence_interval[0][column] = numpy_percentile(&values, tail);
        confidence_interval[1][column] = numpy_percentile(&values, 1.0 - tail);
    }
    TotalEffectBootstrapPrediction {
        individual_predictions: individual_predictions.to_vec(),
        confidence_interval,
    }
}

/// NumPy percentile's default `method="linear"`, with probability in `[0, 1]`.
pub fn numpy_percentile(values: &[f64], probability: f64) -> f64 {
    assert!(!values.is_empty(), "percentile requires values");
    assert!(
        (0.0..=1.0).contains(&probability),
        "percentile probability must be in [0, 1]"
    );
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let position = probability * (sorted.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    let weight = position - lower as f64;
    sorted[lower] + weight * (sorted[upper] - sorted[lower])
}
