//! PCMCI+ ported 1:1 from tigramite's run_pcmciplus at 902 defaults: PC1 lagged parents, the
//! contemp_conds skeleton with MCI conditions, majority collider rule with conflict resolution,
//! and the timeseries Meek rules. Also the fdr_bh correction of get_corrected_pvalues.

use crate::missing_data::{PreprocessingError, TigramiteFrame};
use crate::parcorr::{CiKind, Node, ParCorrCi, RoleAwareSamplePolicy, TimeSeries};
use crate::pcmci::pc_stable_single;

pub(crate) type Mark = Option<[u8; 3]>;

fn mk(s: &str) -> Mark {
    let b = s.as_bytes();
    Some([b[0], b[1], b[2]])
}

pub struct PcmciPlusResult {
    pub graph: Vec<Vec<Vec<String>>>,
    pub p_matrix: Vec<Vec<Vec<f64>>>,
    pub val_matrix: Vec<Vec<Vec<f64>>>,
}

pub(crate) fn combinations(pool: &[Node], k: usize) -> Vec<Vec<Node>> {
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

struct Plus<'a> {
    data: &'a TimeSeries,
    ci: ParCorrCi,
    tau_max: usize,
    pc_alpha: f64,
    graph: Vec<Vec<Vec<Mark>>>,
    lagged_parents: Vec<Vec<Node>>,
}

impl<'a> Plus<'a> {
    /// The MCI test of _run_pcalg_test: S plus lagged parents of j and shifted parents of i.
    fn run_pcalg_test(
        &mut self,
        i: usize,
        abstau: usize,
        j: usize,
        s: &[Node],
    ) -> (f64, f64, bool) {
        if let Some(mark) = self.graph[i][j][abstau] {
            if mark[1] == b'-' {
                return (1.0, 0.0, true);
            }
        }
        let mut z: Vec<Node> = s.to_vec();
        for &node in &self.lagged_parents[j] {
            if node != (i, -(abstau as i32)) && !z.contains(&node) {
                z.push(node);
            }
        }
        for &(k, k_tau) in &self.lagged_parents[i] {
            let shifted = (k, -(abstau as i32) + k_tau);
            if !z.contains(&shifted) {
                z.push(shifted);
            }
        }
        let (val, pval) = self.ci.run_test(
            self.data,
            &[(i, -(abstau as i32))],
            &[(j, 0)],
            &z,
            self.tau_max,
        );
        (val, pval, pval <= self.pc_alpha)
    }

    /// Adjacencies of j from the graph in np.where order: i-major, tau ascending.
    fn adj(&self, j: usize, include_conflicts: bool) -> Vec<Node> {
        adjacencies(&self.graph, j, include_conflicts)
    }

    fn adj_contemp(&self, j: usize, include_conflicts: bool) -> Vec<Node> {
        self.adj(j, include_conflicts)
            .into_iter()
            .filter(|a| a.1 == 0)
            .collect()
    }

    /// Contemp adjacencies sorted by |val_min| descending, stable over val_min insertion order.
    fn adj_contemp_sorted(&self, j: usize, val_min: &[(Node, f64)]) -> Vec<Node> {
        let adjt = self.adj_contemp(j, true);
        let members: Vec<(Node, f64)> = val_min
            .iter()
            .filter(|(node, _)| adjt.contains(node))
            .map(|&(node, v)| (node, v.abs()))
            .collect();
        let mut order: Vec<usize> = (0..members.len()).collect();
        order.sort_by(|&a, &b| members[b].1.partial_cmp(&members[a].1).unwrap());
        order.into_iter().map(|k| members[k].0).collect()
    }

    /// Full adjacencies sorted by |val_min| descending, the standard-mode counterpart.
    fn adj_sorted(&self, j: usize, val_min: &[(Node, f64)]) -> Vec<Node> {
        let adjt = self.adj(j, true);
        let members: Vec<(Node, f64)> = val_min
            .iter()
            .filter(|(node, _)| adjt.contains(node))
            .map(|&(node, v)| (node, v.abs()))
            .collect();
        let mut order: Vec<usize> = (0..members.len()).collect();
        order.sort_by(|&a, &b| members[b].1.partial_cmp(&members[a].1).unwrap());
        order.into_iter().map(|k| members[k].0).collect()
    }
}

pub fn run_pcmciplus(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
) -> PcmciPlusResult {
    run_pcmciplus_windowed(data, tau_max, pc_alpha, kind, None)
}

/// run_sliding_window_of: fresh CI cache per reference window, lags reaching before its start.
pub fn run_sliding_window_pcmciplus(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
    window_length: usize,
    window_step: usize,
) -> Vec<PcmciPlusResult> {
    let mut out = Vec::new();
    let mut start = 0usize;
    while start < data.t - window_length {
        out.push(run_pcmciplus_windowed(
            data,
            tau_max,
            pc_alpha,
            kind,
            Some((start, start + window_length)),
        ));
        start += window_step;
    }
    out
}

pub fn run_pcmciplus_windowed(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
    window: Option<(usize, usize)>,
) -> PcmciPlusResult {
    let mut ci = ParCorrCi::with_kind(kind);
    ci.window = window;
    run_pcmciplus_with_ci(data, tau_max, pc_alpha, ci)
        .expect("dense PCMCI+ sample construction must succeed")
}

/// PCMCI+ over a nullable frame using Tigramite's role-aware sample construction.
pub fn run_pcmciplus_frame(
    frame: TigramiteFrame,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
    sample_policy: RoleAwareSamplePolicy,
) -> Result<PcmciPlusResult, PreprocessingError> {
    let data = frame.data.clone();
    let ci = ParCorrCi::with_frame(kind, frame, sample_policy);
    run_pcmciplus_with_ci(&data, tau_max, pc_alpha, ci)
}

fn run_pcmciplus_with_ci(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    ci: ParCorrCi,
) -> Result<PcmciPlusResult, PreprocessingError> {
    let n = data.n;
    let mut ci = ci;

    // Phase 1: PC1 lagged parent supersets with their max p-values and statistics.
    let mut lagged_parents: Vec<Vec<Node>> = Vec::new();
    let mut p_full = vec![vec![vec![1.0; tau_max + 1]; n]; n];
    let mut val_full = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    for j in 0..n {
        let single = pc_stable_single(&mut ci, data, j, n, 1, tau_max, pc_alpha);
        for &((i, tau), pval, val) in &single.pval_max {
            p_full[i][j][(-tau) as usize] = pval;
            val_full[i][j][(-tau) as usize] = val;
        }
        lagged_parents.push(single.parents);
    }

    // links_for_pc: PC1 lagged parents as -?> plus every contemporaneous pair as o?o.
    let mut links_for_pc: Vec<Vec<(Node, [u8; 3])>> = Vec::new();
    for j in 0..n {
        let mut links: Vec<(Node, [u8; 3])> =
            lagged_parents[j].iter().map(|&p| (p, *b"-?>")).collect();
        for i in 0..n {
            if i != j {
                links.push(((i, 0), *b"o?o"));
            }
        }
        links_for_pc.push(links);
    }

    // Initial graph for the skeleton phase.
    let mut graph = vec![vec![vec![None as Mark; tau_max + 1]; n]; n];
    for (j, links) in links_for_pc.iter().enumerate() {
        for &((i, tau), mark) in links {
            graph[i][j][(-tau) as usize] = Some(mark);
        }
    }
    for i in 0..n {
        graph[i][i][0] = None;
    }

    let mut this = Plus {
        data,
        ci,
        tau_max,
        pc_alpha,
        graph,
        lagged_parents,
    };

    // Phase 2: skeleton discovery with contemporaneous condition sets.
    let mut adjt: Vec<Vec<Node>> = (0..n).map(|j| this.adj_contemp(j, true)).collect();
    let mut val_min: Vec<Vec<(Node, f64)>> = (0..n)
        .map(|j| {
            let mut init = Vec::new();
            for i in 0..n {
                for tau in 0..=tau_max {
                    if this.graph[i][j][tau].is_some() {
                        init.push(((i, -(tau as i32)), f64::INFINITY));
                    }
                }
            }
            init
        })
        .collect();
    let mut p_skel = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    let mut val_skel = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=tau_max {
                if this.graph[i][j][tau].is_none() {
                    p_skel[i][j][tau] = 1.0;
                }
            }
        }
    }

    let max_conds_dim = n;
    let mut p = 0usize;
    loop {
        let mut remaining = Vec::new();
        for i in 0..n {
            for j in 0..n {
                for abstau in 0..=tau_max {
                    if this.graph[i][j][abstau].is_some()
                        && adjt[j]
                            .iter()
                            .filter(|&&a| a != (i, -(abstau as i32)))
                            .count()
                            >= p
                    {
                        remaining.push((i, j, abstau));
                    }
                }
            }
        }
        if remaining.is_empty() || p > max_conds_dim {
            break;
        }
        for (i, j, abstau) in remaining {
            if this.graph[i][j][abstau].is_none() {
                continue;
            }
            let pool: Vec<Node> = adjt[j]
                .iter()
                .copied()
                .filter(|&a| a != (i, -(abstau as i32)))
                .collect();
            for s in combinations(&pool, p) {
                let (val, pval, dependent) = this.run_pcalg_test(i, abstau, j, &s);
                if let Some(entry) = val_min[j]
                    .iter_mut()
                    .find(|(node, _)| *node == (i, -(abstau as i32)))
                {
                    entry.1 = entry.1.min(val.abs());
                }
                if pval >= p_skel[i][j][abstau] {
                    p_skel[i][j][abstau] = pval;
                    val_skel[i][j][abstau] = val;
                }
                if !dependent {
                    if abstau == 0 {
                        this.graph[i][j][0] = None;
                        this.graph[j][i][0] = None;
                        p_skel[j][i][0] = p_skel[i][j][0];
                    } else {
                        this.graph[i][j][abstau] = None;
                    }
                    break;
                }
            }
        }
        p += 1;
        adjt = (0..n)
            .map(|j| this.adj_contemp_sorted(j, &val_min[j]))
            .collect();
    }

    // Symmetrize the skeleton matrices in the oracle's in-place iteration order.
    for i in 0..n {
        for j in 0..n {
            if links_for_pc[j]
                .iter()
                .any(|&((a, tau), mark)| (a, tau) == (i, 0) && &mark == b"o?o")
                && p_skel[i][j][0] >= p_skel[j][i][0]
            {
                p_skel[j][i][0] = p_skel[i][j][0];
                val_skel[j][i][0] = val_skel[i][j][0];
            }
        }
    }

    // Merge: contemp entries wholesale, then every tested link.
    for i in 0..n {
        for j in 0..n {
            p_full[i][j][0] = p_skel[i][j][0];
            val_full[i][j][0] = val_skel[i][j][0];
        }
    }
    for (j, links) in links_for_pc.iter().enumerate() {
        for &((i, tau), _) in links {
            let abstau = (-tau) as usize;
            p_full[i][j][abstau] = p_skel[i][j][abstau];
            val_full[i][j][abstau] = val_skel[i][j][abstau];
        }
    }

    // Phase 3: collider orientation, majority rule with conflict resolution.
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=tau_max {
                match this.graph[i][j][tau] {
                    Some(m) if &m == b"o?o" => this.graph[i][j][tau] = mk("o-o"),
                    Some(m) if &m == b"-?>" => this.graph[i][j][tau] = mk("-->"),
                    Some(m) if &m == b"<?-" => this.graph[i][j][tau] = mk("<--"),
                    _ => {}
                }
            }
        }
    }

    // Unshielded triples (i,taui) *-* k o-o j with i and j non-adjacent.
    let mut triples: Vec<(Node, usize, usize)> = Vec::new();
    for j in 0..n {
        for (k, tauk) in this.adj(j, false) {
            if tauk == 0 && this.graph[k][j][0] == mk("o-o") {
                for (i, taui) in this.adj(k, false) {
                    if (i, taui) != (j, 0)
                        && this.graph[i][j][(-taui) as usize].is_none()
                        && (this.graph[i][k][(-taui) as usize] == mk("o-o")
                            || this.graph[i][k][(-taui) as usize] == mk("-->"))
                    {
                        triples.push(((i, taui), k, j));
                    }
                }
            }
        }
    }

    let adjt: Vec<Vec<Node>> = (0..n).map(|j| this.adj_contemp(j, true)).collect();
    let mut v_structures = Vec::new();
    let mut ambiguous: Vec<(Node, usize, usize)> = Vec::new();
    for &((i, tau), k, j) in &triples {
        // Subsets of contemp neighbors of j (and of i when tau == 0), deduplicated.
        let mut neighbor_subsets: Vec<Vec<Node>> = Vec::new();
        let side_j: Vec<Node> = adjt[j]
            .iter()
            .copied()
            .filter(|&(l, taul)| !(l == i && tau == taul))
            .collect();
        if !side_j.is_empty() {
            for card in 0..=side_j.len() {
                for sub in combinations(&side_j, card) {
                    if !neighbor_subsets.contains(&sub) {
                        neighbor_subsets.push(sub);
                    }
                }
            }
        }
        if tau == 0 {
            let side_i: Vec<Node> = adjt[i]
                .iter()
                .copied()
                .filter(|&(l, taul)| !(l == j && taul == 0))
                .collect();
            if !side_i.is_empty() {
                for card in 0..=side_i.len() {
                    for sub in combinations(&side_i, card) {
                        if !neighbor_subsets.contains(&sub) {
                            neighbor_subsets.push(sub);
                        }
                    }
                }
            }
        }

        let mut sepset_count = 0usize;
        let mut with_k = 0usize;
        for s in &neighbor_subsets {
            let (_val, _pval, dependent) = this.run_pcalg_test(i, (-tau) as usize, j, s);
            if !dependent {
                sepset_count += 1;
                if s.contains(&(k, 0)) {
                    with_k += 1;
                }
            }
        }
        if sepset_count == 0 {
            ambiguous.push(((i, tau), k, j));
        } else {
            let fraction = with_k as f64 / sepset_count as f64;
            if fraction == 0.5 {
                ambiguous.push(((i, tau), k, j));
            } else if fraction < 0.5 {
                v_structures.push(((i, tau), k, j));
            }
        }
    }

    let mut oriented: Vec<(usize, usize)> = Vec::new();
    for &((i, tau), k, j) in &v_structures {
        if !oriented.contains(&(k, j)) && !oriented.contains(&(j, k)) {
            this.graph[k][j][0] = mk("<--");
            this.graph[j][k][0] = mk("-->");
            oriented.push((j, k));
        }
        if oriented.contains(&(k, j)) {
            this.graph[j][k][0] = mk("x-x");
            this.graph[k][j][0] = mk("x-x");
        }
        if tau == 0 {
            if !oriented.contains(&(i, k)) && !oriented.contains(&(k, i)) {
                this.graph[k][i][0] = mk("<--");
                this.graph[i][k][0] = mk("-->");
                oriented.push((i, k));
            }
            if oriented.contains(&(k, i)) {
                this.graph[i][k][0] = mk("x-x");
                this.graph[k][i][0] = mk("x-x");
            }
        }
    }

    // Phase 4: Meek rules until none fires, with conflict marking.
    meek_rules(&mut this.graph, &ambiguous);

    let graph_out = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    (0..=tau_max)
                        .map(|tau| match this.graph[i][j][tau] {
                            Some(m) => String::from_utf8_lossy(&m).into_owned(),
                            None => String::new(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let result = PcmciPlusResult {
        graph: graph_out,
        p_matrix: p_full,
        val_matrix: val_full,
    };
    match this.ci.preprocessing_error().cloned() {
        Some(error) => Err(error),
        None => Ok(result),
    }
}

/// Meek rules until none fires, with conflict marking (tigramite _pcalg_rules_timeseries).
pub(crate) fn adjacencies(
    graph: &[Vec<Vec<Mark>>],
    j: usize,
    include_conflicts: bool,
) -> Vec<Node> {
    let mut out = Vec::new();
    for (i, targets) in graph.iter().enumerate() {
        for (tau, mark) in targets[j].iter().enumerate() {
            if let Some(mark) = mark {
                if !include_conflicts && (mark == b"x-x" || mark == b"x?x") {
                    continue;
                }
                out.push((i, -(tau as i32)));
            }
        }
    }
    out
}

struct Orientation<'a> {
    graph: &'a mut Vec<Vec<Vec<Mark>>>,
}
impl Orientation<'_> {
    fn adj(&self, j: usize, conflicts: bool) -> Vec<Node> {
        adjacencies(self.graph, j, conflicts)
    }
    fn adj_contemp(&self, j: usize, conflicts: bool) -> Vec<Node> {
        self.adj(j, conflicts)
            .into_iter()
            .filter(|node| node.1 == 0)
            .collect()
    }
}

pub(crate) fn meek_rules(graph: &mut Vec<Vec<Vec<Mark>>>, ambiguous: &[(Node, usize, usize)]) {
    meek_rules_with_conflicts(graph, ambiguous, true);
}

pub(crate) fn meek_rules_with_conflicts(
    graph: &mut Vec<Vec<Vec<Mark>>>,
    ambiguous: &[(Node, usize, usize)],
    conflicts: bool,
) {
    let n = graph.len();
    let this = Orientation { graph };
    let mut oriented: Vec<(usize, usize)> = Vec::new();
    loop {
        // Rule 1: i --> k o-o j with i,j non-adjacent: orient k --> j.
        let mut any1 = false;
        let mut r1 = Vec::new();
        for j in 0..n {
            for (k, tauk) in this.adj(j, false) {
                if tauk == 0 && this.graph[j][k][0] == mk("o-o") {
                    for (i, taui) in this.adj(k, false) {
                        if (i, taui) != (j, 0)
                            && this.graph[i][j][(-taui) as usize].is_none()
                            && this.graph[i][k][(-taui) as usize] == mk("-->")
                        {
                            r1.push(((i, taui), k, j));
                        }
                    }
                }
            }
        }
        for ((i, taui), k, j) in r1 {
            if ambiguous.contains(&((i, taui), k, j)) {
                continue;
            }
            any1 = true;
            if !oriented.contains(&(j, k)) && !oriented.contains(&(k, j)) {
                this.graph[k][j][0] = mk("-->");
                this.graph[j][k][0] = mk("<--");
                oriented.push((k, j));
            }
            if conflicts && oriented.contains(&(j, k)) {
                this.graph[j][k][0] = mk("x-x");
                this.graph[k][j][0] = mk("x-x");
            }
        }

        // Rule 2: i --> k --> j with i o-o j: orient i --> j.
        let mut any2 = false;
        let mut r2 = Vec::new();
        for j in 0..n {
            for (k, _) in this.adj_contemp(j, false) {
                if this.graph[k][j][0] == mk("-->") {
                    for (i, taui) in this.adj_contemp(k, false) {
                        if this.graph[i][k][0] == mk("-->")
                            && (i, taui) != (j, 0)
                            && this.graph[i][j][0] == mk("o-o")
                            && this.graph[j][i][0] == mk("o-o")
                        {
                            r2.push(((i, 0i32), k, j));
                        }
                    }
                }
            }
        }
        for ((i, taui), k, j) in r2 {
            if ambiguous.contains(&((i, taui), k, j)) {
                continue;
            }
            any2 = true;
            if !oriented.contains(&(j, i)) && !oriented.contains(&(i, j)) {
                this.graph[i][j][0] = mk("-->");
                this.graph[j][i][0] = mk("<--");
                oriented.push((i, j));
            }
            if conflicts && oriented.contains(&(j, i)) {
                this.graph[j][i][0] = mk("x-x");
                this.graph[i][j][0] = mk("x-x");
            }
        }

        // Rule 3: i o-o k --> j and i o-o l --> j with i o-o j, k and l non-adjacent: i --> j.
        let mut any3 = false;
        let mut r3 = Vec::new();
        for j in 0..n {
            let contemp = this.adj_contemp(j, false);
            for &(i, _) in &contemp {
                if this.graph[j][i][0] == mk("o-o") {
                    for &(k, _) in &contemp {
                        for &(l, _) in &contemp {
                            if k != l
                                && k != i
                                && l != i
                                && this.graph[k][j][0] == mk("-->")
                                && this.graph[l][j][0] == mk("-->")
                                && this.graph[k][i][0] == mk("o-o")
                                && this.graph[l][i][0] == mk("o-o")
                                && this.graph[k][l][0].is_none()
                            {
                                r3.push((((i, 0i32), k, j), ((i, 0i32), l, j)));
                            }
                        }
                    }
                }
            }
        }
        for (t_k, t_l) in r3 {
            if ambiguous.contains(&t_k) || ambiguous.contains(&t_l) {
                continue;
            }
            any3 = true;
            let ((i, _), _, j) = t_k;
            if !oriented.contains(&(j, i)) && !oriented.contains(&(i, j)) {
                this.graph[i][j][0] = mk("-->");
                this.graph[j][i][0] = mk("<--");
                oriented.push((i, j));
            }
            if conflicts && oriented.contains(&(j, i)) {
                this.graph[j][i][0] = mk("x-x");
                this.graph[i][j][0] = mk("x-x");
            }
        }

        if !any1 && !any2 && !any3 {
            break;
        }
    }
}

/// run_pcalg in tigramite's 'standard' mode: the PC algorithm over every lagged and
/// contemporaneous link, condition sets drawn from full adjacencies, no MCI padding.
pub fn run_pcalg_standard(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
) -> PcmciPlusResult {
    let n = data.n;
    let ci = ParCorrCi::with_kind(kind);

    let mut graph = vec![vec![vec![None as Mark; tau_max + 1]; n]; n];
    for j in 0..n {
        for i in 0..n {
            if i != j {
                graph[i][j][0] = Some(*b"o?o");
            }
            for tau in 1..=tau_max {
                graph[i][j][tau] = Some(*b"-?>");
            }
        }
    }

    let lagged_parents: Vec<Vec<Node>> = vec![Vec::new(); n];
    let mut this = Plus {
        data,
        ci,
        tau_max,
        pc_alpha,
        graph,
        lagged_parents,
    };

    let mut adjt: Vec<Vec<Node>> = (0..n).map(|j| this.adj(j, true)).collect();
    let mut val_min: Vec<Vec<(Node, f64)>> = (0..n)
        .map(|j| {
            let mut init = Vec::new();
            for i in 0..n {
                for tau in 0..=tau_max {
                    if this.graph[i][j][tau].is_some() {
                        init.push(((i, -(tau as i32)), f64::INFINITY));
                    }
                }
            }
            init
        })
        .collect();
    let mut p_skel = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    let mut val_skel = vec![vec![vec![0.0; tau_max + 1]; n]; n];
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=tau_max {
                if this.graph[i][j][tau].is_none() {
                    p_skel[i][j][tau] = 1.0;
                }
            }
        }
    }

    let max_conds_dim = n * (tau_max + 1);
    let mut p = 0usize;
    loop {
        let mut remaining = Vec::new();
        for i in 0..n {
            for j in 0..n {
                for abstau in 0..=tau_max {
                    if this.graph[i][j][abstau].is_some()
                        && adjt[j]
                            .iter()
                            .filter(|&&a| a != (i, -(abstau as i32)))
                            .count()
                            >= p
                    {
                        remaining.push((i, j, abstau));
                    }
                }
            }
        }
        if remaining.is_empty() || p > max_conds_dim {
            break;
        }
        for (i, j, abstau) in remaining {
            if this.graph[i][j][abstau].is_none() {
                continue;
            }
            let pool: Vec<Node> = adjt[j]
                .iter()
                .copied()
                .filter(|&a| a != (i, -(abstau as i32)))
                .collect();
            for s in combinations(&pool, p) {
                let (val, pval, dependent) = this.run_pcalg_test(i, abstau, j, &s);
                if let Some(entry) = val_min[j]
                    .iter_mut()
                    .find(|(node, _)| *node == (i, -(abstau as i32)))
                {
                    entry.1 = entry.1.min(val.abs());
                }
                if pval >= p_skel[i][j][abstau] {
                    p_skel[i][j][abstau] = pval;
                    val_skel[i][j][abstau] = val;
                }
                if !dependent {
                    if abstau == 0 {
                        this.graph[i][j][0] = None;
                        this.graph[j][i][0] = None;
                        p_skel[j][i][0] = p_skel[i][j][0];
                    } else {
                        this.graph[i][j][abstau] = None;
                    }
                    break;
                }
            }
        }
        p += 1;
        adjt = (0..n).map(|j| this.adj_sorted(j, &val_min[j])).collect();
    }

    // Symmetrize the contemporaneous skeleton matrices in the oracle's iteration order.
    for i in 0..n {
        for j in 0..n {
            if i != j && p_skel[i][j][0] >= p_skel[j][i][0] {
                p_skel[j][i][0] = p_skel[i][j][0];
                val_skel[j][i][0] = val_skel[i][j][0];
            }
        }
    }

    // Phase 3: collider orientation, majority rule with conflict resolution.
    for i in 0..n {
        for j in 0..n {
            for tau in 0..=tau_max {
                match this.graph[i][j][tau] {
                    Some(m) if &m == b"o?o" => this.graph[i][j][tau] = mk("o-o"),
                    Some(m) if &m == b"-?>" => this.graph[i][j][tau] = mk("-->"),
                    Some(m) if &m == b"<?-" => this.graph[i][j][tau] = mk("<--"),
                    _ => {}
                }
            }
        }
    }

    let mut triples: Vec<(Node, usize, usize)> = Vec::new();
    for j in 0..n {
        for (k, tauk) in this.adj(j, false) {
            if tauk == 0 && this.graph[k][j][0] == mk("o-o") {
                for (i, taui) in this.adj(k, false) {
                    if (i, taui) != (j, 0)
                        && this.graph[i][j][(-taui) as usize].is_none()
                        && (this.graph[i][k][(-taui) as usize] == mk("o-o")
                            || this.graph[i][k][(-taui) as usize] == mk("-->"))
                    {
                        triples.push(((i, taui), k, j));
                    }
                }
            }
        }
    }

    let adjt: Vec<Vec<Node>> = (0..n).map(|j| this.adj(j, true)).collect();
    let mut v_structures = Vec::new();
    let mut ambiguous: Vec<(Node, usize, usize)> = Vec::new();
    for &((i, tau), k, j) in &triples {
        let mut neighbor_subsets: Vec<Vec<Node>> = Vec::new();
        let side_j: Vec<Node> = adjt[j]
            .iter()
            .copied()
            .filter(|&(l, taul)| !(l == i && tau == taul))
            .collect();
        if !side_j.is_empty() {
            for card in 0..=side_j.len() {
                for sub in combinations(&side_j, card) {
                    if !neighbor_subsets.contains(&sub) {
                        neighbor_subsets.push(sub);
                    }
                }
            }
        }
        if tau == 0 {
            let side_i: Vec<Node> = adjt[i]
                .iter()
                .copied()
                .filter(|&(l, taul)| !(l == j && taul == 0))
                .collect();
            if !side_i.is_empty() {
                for card in 0..=side_i.len() {
                    for sub in combinations(&side_i, card) {
                        if !neighbor_subsets.contains(&sub) {
                            neighbor_subsets.push(sub);
                        }
                    }
                }
            }
        }

        let mut sepset_count = 0usize;
        let mut with_k = 0usize;
        for s in &neighbor_subsets {
            let (_val, _pval, dependent) = this.run_pcalg_test(i, (-tau) as usize, j, s);
            if !dependent {
                sepset_count += 1;
                if s.contains(&(k, 0)) {
                    with_k += 1;
                }
            }
        }
        if sepset_count == 0 {
            ambiguous.push(((i, tau), k, j));
        } else {
            let fraction = with_k as f64 / sepset_count as f64;
            if fraction == 0.5 {
                ambiguous.push(((i, tau), k, j));
            } else if fraction < 0.5 {
                v_structures.push(((i, tau), k, j));
            }
        }
    }

    let mut oriented: Vec<(usize, usize)> = Vec::new();
    for &((i, tau), k, j) in &v_structures {
        if !oriented.contains(&(k, j)) && !oriented.contains(&(j, k)) {
            this.graph[k][j][0] = mk("<--");
            this.graph[j][k][0] = mk("-->");
            oriented.push((j, k));
        }
        if oriented.contains(&(k, j)) {
            this.graph[j][k][0] = mk("x-x");
            this.graph[k][j][0] = mk("x-x");
        }
        if tau == 0 {
            if !oriented.contains(&(i, k)) && !oriented.contains(&(k, i)) {
                this.graph[k][i][0] = mk("<--");
                this.graph[i][k][0] = mk("-->");
                oriented.push((i, k));
            }
            if oriented.contains(&(k, i)) {
                this.graph[i][k][0] = mk("x-x");
                this.graph[k][i][0] = mk("x-x");
            }
        }
    }

    meek_rules(&mut this.graph, &ambiguous);

    let graph_out = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    (0..=tau_max)
                        .map(|tau| match this.graph[i][j][tau] {
                            Some(m) => String::from_utf8_lossy(&m).into_owned(),
                            None => String::new(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    PcmciPlusResult {
        graph: graph_out,
        p_matrix: p_skel,
        val_matrix: val_skel,
    }
}

/// get_corrected_pvalues with fdr_bh: Benjamini-Hochberg over all off-diagonal entries.
pub fn fdr_bh(p_matrix: &[Vec<Vec<f64>>], exclude_contemporaneous: bool) -> Vec<Vec<Vec<f64>>> {
    let n = p_matrix.len();
    let tau_len = p_matrix[0][0].len();
    let mut q = p_matrix.to_vec();
    let mut cells = Vec::new();
    for i in 0..n {
        for j in 0..n {
            for tau in 0..tau_len {
                if i == j && tau == 0 {
                    continue;
                }
                if exclude_contemporaneous && tau == 0 {
                    continue;
                }
                cells.push((i, j, tau));
            }
        }
    }
    let pvs: Vec<f64> = cells
        .iter()
        .map(|&(i, j, tau)| p_matrix[i][j][tau])
        .collect();
    let corrected = bh_values(&pvs);
    for (&(i, j, tau), value) in cells.iter().zip(corrected) {
        q[i][j][tau] = value;
    }
    q
}

pub(crate) fn bh_values(pvs: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..pvs.len()).collect();
    order.sort_by(|&a, &b| pvs[a].partial_cmp(&pvs[b]).unwrap());
    let nobs = pvs.len() as f64;
    let mut corrected: Vec<f64> = order
        .iter()
        .enumerate()
        .map(|(rank, &idx)| pvs[idx] / ((rank + 1) as f64 / nobs))
        .collect();
    for r in (0..corrected.len().saturating_sub(1)).rev() {
        corrected[r] = corrected[r].min(corrected[r + 1]);
    }
    for v in &mut corrected {
        if *v > 1.0 {
            *v = 1.0;
        }
    }
    let mut out = vec![0.0; pvs.len()];
    for (rank, &idx) in order.iter().enumerate() {
        out[idx] = corrected[rank];
    }
    out
}
