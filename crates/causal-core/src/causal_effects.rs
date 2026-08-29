//! `tigramite.causal_effects.CausalEffects` for `graph_type="stationary_dag"`: the latent
//! projection into a time series ADMG, the optimal adjustment set of Runge (NeurIPS 2021),
//! and the total effect estimator.

use std::collections::{BTreeSet, HashSet};

/// A variable index paired with a lag, which is zero or negative.
pub type Node = (usize, i32);
/// A three character edge mark, `[0, 0, 0]` when absent.
pub type Edge = [u8; 3];

pub const EMPTY: Edge = [0, 0, 0];

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

    /// `get_optimal_set()`. Returns `None` where tigramite returns `False`, meaning the
    /// effect is not identifiable from this graph.
    pub fn get_optimal_set(&self) -> Option<Vec<Node>> {
        self.optimal_set_parts()
            .map(|(parents, colliders, collider_parents, s)| {
                let mut oset: BTreeSet<Node> = parents;
                oset.extend(colliders);
                oset.extend(collider_parents);
                oset.extend(s);
                oset.into_iter().collect()
            })
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
                        let reachable = if self.vancs.contains(&spouse) {
                            true
                        } else {
                            let mut conditions: BTreeSet<Node> =
                                parents.union(&self.vancs).copied().collect();
                            conditions.extend(self.s.iter().copied());
                            let target: BTreeSet<Node> = [spouse].into_iter().collect();
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
        Some((parents, colliders, collider_parents, self.s.clone()))
    }
}

// ---------------------------------------------------------------------------
// Total effect estimation: `fit_total_effect` and `predict_total_effect`.
// ---------------------------------------------------------------------------

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

/// `LinearRegression().fit()`: centre, solve by least squares, recover the intercept.
fn linear_fit(predictors: &[Vec<f64>], targets: &[f64]) -> Fitted {
    let n = predictors.len();
    let k = predictors[0].len();
    let mut means = vec![0.0; k];
    for row in predictors {
        for j in 0..k {
            means[j] += row[j] / n as f64;
        }
    }
    let y_mean = targets.iter().sum::<f64>() / n as f64;
    let xc = DMatrix::from_fn(n, k, |r, c| predictors[r][c] - means[c]);
    let yc = DVector::from_fn(n, |r, _| targets[r] - y_mean);
    let svd = xc.svd(true, true);
    let eps = 1e-15 * svd.singular_values.max() * n.max(k) as f64;
    let coef = svd.solve(&yc, eps).expect("total effect design solve");
    let intercept = y_mean - (0..k).map(|j| coef[j] * means[j]).sum::<f64>();
    Fitted::Linear { intercept, coef }
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
        } => {
            // Uniform weights, so the prediction is the mean of the k nearest targets.
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
            d[..*k].iter().map(|&(_, i)| targets[i]).sum::<f64>() / *k as f64
        }
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
        if self.no_causal_path {
            return None;
        }
        let adjustment_set = self.get_optimal_set()?;
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

        let fitted = fit_estimator(estimator, &predictors, &targets);
        Some(TotalEffectModel {
            fitted,
            z_array: z_rows,
            s_predictors,
            len_x,
            len_s,
            conditional_estimator: conditional_estimator.unwrap_or(estimator),
            // Tigramite exposes the O-set before Models removes its overlap with S.
            adjustment_set,
            n_obs,
        })
    }
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
