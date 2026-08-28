//! LPCMCI ported 1:1 from tigramite/lpcmci.py at default parameters, ParCorr with analytic
//! significance. Method names mirror the oracle for line-by-line auditing.

use crate::parcorr::{CiKind, Node, ParCorrCi, TimeSeries};
use std::collections::{BTreeMap, BTreeSet};

/// A link is three ASCII marks, or absent.
pub type Link = Option<[u8; 3]>;

fn lk(s: &str) -> Link {
    let b = s.as_bytes();
    Some([b[0], b[1], b[2]])
}

fn match_link(pattern: &str, link: Link) -> bool {
    let Some(link) = link else {
        return pattern.is_empty();
    };
    if pattern.is_empty() {
        return false;
    }
    let p = pattern.as_bytes();
    let (left, middle, right) = (p[0], p[1], p[2]);
    if left != b'*' {
        if left == b'+' {
            if link[0] != b'<' && link[0] != b'o' {
                return false;
            }
        } else if link[0] != left {
            return false;
        }
    }
    if right != b'*' {
        if right == b'+' {
            if link[2] != b'>' && link[2] != b'o' {
                return false;
            }
        } else if link[2] != right {
            return false;
        }
    }
    middle == b'*' || link[1] == middle
}

fn reverse_link(link: Link) -> Link {
    let link = link?;
    let left = if link[2] == b'>' { b'<' } else { link[2] };
    let right = if link[0] == b'<' { b'>' } else { link[0] };
    Some([left, link[1], right])
}

/// Lexicographic k-combinations of a slice, like itertools.combinations.
fn combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let mut out = Vec::new();
    if k > items.len() {
        return out;
    }
    let mut idx: Vec<usize> = (0..k).collect();
    loop {
        out.push(idx.iter().map(|&i| items[i].clone()).collect());
        let mut i = k;
        loop {
            if i == 0 {
                return out;
            }
            i -= 1;
            if idx[i] != i + items.len() - k {
                break;
            }
            if i == 0 {
                return out;
            }
        }
        idx[i] += 1;
        for j in i + 1..k {
            idx[j] = idx[j - 1] + 1;
        }
    }
}

type Sepset = BTreeSet<Node>;
type PairKey = (usize, usize, i32);

pub struct LpcmciResult {
    pub graph: Vec<Vec<Vec<String>>>,
    pub p_matrix: Vec<Vec<Vec<f64>>>,
    pub val_matrix: Vec<Vec<Vec<f64>>>,
}

pub struct Lpcmci<'a> {
    data: &'a TimeSeries,
    ci: ParCorrCi,
    n: usize,
    tau_max: i32,
    pc_alpha: f64,
    n_preliminary_iterations: usize,
    no_apr: i64,
    orient_contemp: u8,
    graph_dict: Vec<BTreeMap<Node, Link>>,
    graph_full_dict: Vec<BTreeMap<Node, Link>>,
    sepsets: Vec<BTreeMap<Node, BTreeSet<(Sepset, bool)>>>,
    def_ancs: Vec<BTreeSet<Node>>,
    def_non_ancs: Vec<BTreeSet<Node>>,
    ambiguous_ancestorships: Vec<BTreeSet<Node>>,
    pval_max: Vec<BTreeMap<Node, f64>>,
    pval_max_val: Vec<BTreeMap<Node, f64>>,
    pval_max_card: Vec<BTreeMap<Node, f64>>,
    na_pds_t: BTreeMap<Node, BTreeMap<Node, Sepset>>,
    rules_prelim: Vec<&'static str>,
    rules_all: Vec<&'static str>,
}

pub fn run_lpcmci(data: &TimeSeries, tau_max: usize, pc_alpha: f64) -> LpcmciResult {
    run_lpcmci_traced(data, tau_max, pc_alpha).0
}

/// Run LPCMCI while reporting completed algorithm phases.
pub fn run_lpcmci_with_progress<F>(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    progress: F,
) -> LpcmciResult
where
    F: FnMut(&'static str, usize, usize),
{
    let mut this = Lpcmci::new(data, tau_max as i32, pc_alpha, CiKind::ParCorr);
    this.run_inner_with_progress(progress)
}

pub fn run_lpcmci_with(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
    kind: CiKind,
) -> LpcmciResult {
    Lpcmci::new(data, tau_max as i32, pc_alpha, kind)
        .run_traced()
        .0
}

pub fn run_lpcmci_traced(
    data: &TimeSeries,
    tau_max: usize,
    pc_alpha: f64,
) -> (LpcmciResult, Vec<(Node, Node, Vec<Node>, f64, f64)>) {
    let this = Lpcmci::new(data, tau_max as i32, pc_alpha, CiKind::ParCorr);
    this.run_traced()
}

impl<'a> Lpcmci<'a> {
    fn new(data: &'a TimeSeries, tau_max: i32, pc_alpha: f64, kind: CiKind) -> Self {
        let n = data.n;
        let mut this = Lpcmci {
            data,
            ci: ParCorrCi::with_kind(kind),
            n,
            tau_max,
            pc_alpha,
            n_preliminary_iterations: 1,
            no_apr: 0,
            orient_contemp: 1,
            graph_dict: Vec::new(),
            graph_full_dict: Vec::new(),
            sepsets: Vec::new(),
            def_ancs: Vec::new(),
            def_non_ancs: Vec::new(),
            ambiguous_ancestorships: Vec::new(),
            pval_max: Vec::new(),
            pval_max_val: Vec::new(),
            pval_max_card: Vec::new(),
            na_pds_t: BTreeMap::new(),
            rules_prelim: vec!["APR", "ER-08", "ER-02", "ER-01", "ER-09", "ER-10"],
            rules_all: vec![
                "APR", "ER-08", "ER-02", "ER-01", "ER-00-d", "ER-00-c", "ER-03", "R-04", "ER-09",
                "ER-10", "ER-00-b", "ER-00-a",
            ],
        };
        this.initialize_run_memory();
        this
    }

    fn initialize_run_memory(&mut self) {
        let n = self.n;
        self.graph_dict = (0..n)
            .map(|j| {
                let mut links: BTreeMap<Node, Link> = (0..n)
                    .filter(|&i| i != j)
                    .map(|i| ((i, 0), lk("o?o")))
                    .collect();
                for i in 0..n {
                    for tau in 1..=self.tau_max {
                        // max_cond_px == 0 and update_middle_marks hold at defaults.
                        links.insert((i, -tau), lk("oL>"));
                    }
                }
                links
            })
            .collect();
        self.sepsets = (0..n)
            .map(|j| {
                let mut m = BTreeMap::new();
                for i in 0..n {
                    for tau in 0..=self.tau_max {
                        if tau > 0 || i < j {
                            m.insert((i, -tau), BTreeSet::new());
                        }
                    }
                }
                m
            })
            .collect();
        self.def_ancs = vec![BTreeSet::new(); n];
        self.def_non_ancs = vec![BTreeSet::new(); n];
        self.ambiguous_ancestorships = vec![BTreeSet::new(); n];
        let pval_init = |value: f64| {
            (0..n)
                .map(|j| {
                    let mut m = BTreeMap::new();
                    for i in 0..n {
                        for tau in 0..=self.tau_max {
                            if tau > 0 || i < j {
                                m.insert((i, -tau), value);
                            }
                        }
                    }
                    m
                })
                .collect::<Vec<_>>()
        };
        self.pval_max = pval_init(f64::NEG_INFINITY);
        self.pval_max_val = pval_init(f64::INFINITY);
        self.pval_max_card = pval_init(f64::NEG_INFINITY);
        self.na_pds_t = (0..n)
            .flat_map(|j| (0..=self.tau_max).map(move |t| ((j, -t), BTreeMap::new())))
            .collect();
    }

    fn run_traced(self) -> (LpcmciResult, Vec<(Node, Node, Vec<Node>, f64, f64)>) {
        let mut this = self;
        let result = this.run_inner_with_progress(|_, _, _| {});
        let trace = std::mem::take(&mut this.ci.trace);
        (result, trace)
    }

    fn run_inner_with_progress<F>(&mut self, mut progress: F) -> LpcmciResult
    where
        F: FnMut(&'static str, usize, usize),
    {
        const PHASES: usize = 5;
        progress("preliminary ancestral removal", 0, PHASES);
        for i in 0..self.n_preliminary_iterations {
            self.run_ancestral_removal_phase(true);
            if i == self.n_preliminary_iterations - 1 {
                // prelim_only is false; fall through with remembered parents.
            }
            // remember_only_parents = true.
            let def_ancs: Vec<BTreeSet<Node>> = (0..self.n)
                .map(|j| {
                    self.def_ancs[j]
                        .iter()
                        .copied()
                        .filter(|&(v, lag)| self.get_link((v, lag), (j, 0)).is_some())
                        .collect()
                })
                .collect();
            self.initialize_run_memory();
            self.apply_new_ancestral_information(&vec![BTreeSet::new(); self.n], &def_ancs);
        }
        progress("ancestral removal", 1, PHASES);
        self.run_ancestral_removal_phase(false);
        progress("non-ancestral removal", 2, PHASES);
        self.run_non_ancestral_removal_phase();
        // fix_all_edges_before_final_orientation = true.
        self.fix_all_edges();
        let rules = self.rules_all.clone();
        progress("orientation rules", 3, PHASES);
        self.run_orientation_phase(&rules, false);
        self.fix_all_edges();
        progress("materializing PAG matrices", 4, PHASES);
        let graph = self.dict2graph();
        let p_matrix = self.dict_to_matrix(&self.pval_max, 0.0);
        let val_matrix = self.dict_to_matrix(&self.pval_max_val, 0.0);
        let result = LpcmciResult {
            graph,
            p_matrix,
            val_matrix,
        };
        progress("complete", PHASES, PHASES);
        result
    }

    // ------------------------------------------------------------------ phases

    fn link_groups(&self) -> Vec<Vec<(Node, Node)>> {
        // auto_first = true: first the autodependency links, then per-lag groups skipping them.
        let mut groups: Vec<Vec<(Node, Node)>> = Vec::new();
        let mut auto = Vec::new();
        for v in 0..self.n {
            for lag in -self.tau_max..0 {
                auto.push(((v, lag), (v, 0)));
            }
        }
        groups.push(auto);
        for lag in 0..=self.tau_max {
            let mut group = Vec::new();
            for a in 0..self.n {
                for b in 0..self.n {
                    if a == b {
                        continue;
                    }
                    group.push(((a, -lag), (b, 0)));
                }
            }
            groups.push(group);
        }
        groups
    }

    fn run_ancestral_removal_phase(&mut self, prelim: bool) {
        let mut p_pc: usize = 0;
        loop {
            let mut has_converged = true;
            let mut any_removal = false;
            for links in self.link_groups() {
                let mut to_remove: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); self.n];
                for (x, y) in links {
                    if x.1 == 0 && x.0 == y.0 {
                        continue;
                    }
                    if self.is_smaller(y, x) {
                        continue;
                    }
                    let Some(link) = self.get_link(x, y) else {
                        continue;
                    };
                    if link[1] == b'-' {
                        continue;
                    }
                    let test_y = link[1] != b'R' && link[1] != b'!';
                    // max_cond_px = 0: X-side tests only for contemporaneous X.
                    let test_x = link[1] != b'L' && link[1] != b'!' && x.1 == 0;
                    let mut s_default_yx = BTreeSet::new();
                    let mut s_search_yx = Vec::new();
                    let mut s_default_xy = BTreeSet::new();
                    let mut s_search_xy = Vec::new();
                    if test_y {
                        (s_default_yx, s_search_yx) =
                            self.get_default_and_search_sets_ancestral(y, x);
                    }
                    if test_x {
                        (s_default_xy, s_search_xy) =
                            self.get_default_and_search_sets_ancestral(x, y);
                    }
                    if test_y {
                        if s_search_yx.len() < p_pc {
                            self.apply_middle_mark(x, y, b'R');
                        } else {
                            has_converged = false;
                        }
                    }
                    if test_x {
                        if s_search_xy.len() < p_pc {
                            self.apply_middle_mark(x, y, b'L');
                        } else {
                            has_converged = false;
                        }
                    }
                    // break_once_separated = true: sort the search sets.
                    if test_y {
                        s_search_yx = self.sort_search_set(s_search_yx, y);
                    }
                    if test_x {
                        s_search_xy = self.sort_search_set(s_search_xy, x);
                    }
                    if test_y {
                        for s_pc in combinations(&s_search_yx, p_pc) {
                            let mut z: Sepset = s_pc.iter().copied().collect();
                            z.extend(s_default_yx.iter().copied());
                            let (val, pval) = self.run_ci(x, y, &z);
                            self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                            if pval > self.pc_alpha {
                                to_remove[y.0].insert(x);
                                self.save_sepset(x, y, z.clone(), true);
                                break;
                            }
                        }
                    }
                    if test_x {
                        for s_pc in combinations(&s_search_xy, p_pc) {
                            let mut z: Sepset = s_pc.iter().copied().collect();
                            z.extend(s_default_xy.iter().copied());
                            let (val, pval) = self.run_ci(x, y, &z);
                            self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                            if pval > self.pc_alpha {
                                to_remove[y.0].insert(x);
                                self.save_sepset(x, y, z.clone(), true);
                                break;
                            }
                        }
                    }
                }
                for j in 0..self.n {
                    for &x in &to_remove[j].clone() {
                        any_removal = true;
                        self.write_link(x, (j, 0), None);
                    }
                }
            }
            if any_removal {
                let only_lagged = self.orient_contemp != 2;
                let rules = self.rules_prelim.clone();
                let any_update = self.run_orientation_phase(&rules, only_lagged);
                if any_update {
                    self.update_middle_marks();
                    p_pc = 0;
                } else {
                    p_pc += 1;
                }
            } else if has_converged {
                break;
            } else {
                p_pc += 1;
            }
        }
        // Turn remaining middle marks into '!'.
        for j in 0..self.n {
            let keys: Vec<Node> = self.graph_dict[j].keys().copied().collect();
            for (i, lag_i) in keys {
                let x = (i, lag_i);
                let y = (j, 0);
                if self.is_smaller(y, x) {
                    continue;
                }
                if let Some(link) = self.get_link(x, y) {
                    if link[1] != b'-' && link[1] != b'!' {
                        self.write_link(x, y, Some([link[0], b'!', link[2]]));
                    }
                }
            }
        }
        // prelim_with_collider_rules = true: both the prelim and final phases end with the full
        // rule set, unlagged; only the no_apr decrement distinguishes the final phase.
        if !prelim {
            self.no_apr -= 1;
        }
        let rules = self.rules_all.clone();
        let any_update = self.run_orientation_phase(&rules, false);
        if any_update {
            self.update_middle_marks();
        }
    }

    fn run_non_ancestral_removal_phase(&mut self) {
        self.update_middle_marks();
        self.initialize_full_graph();
        let mut p_pc: usize = 0;
        loop {
            let mut has_converged = true;
            let mut any_removal = false;
            for links in self.link_groups() {
                let mut to_remove: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); self.n];
                for (x, y) in links {
                    if x.1 == 0 && x.0 == y.0 {
                        continue;
                    }
                    if self.is_smaller(y, x) {
                        continue;
                    }
                    let Some(link) = self.get_link(x, y) else {
                        continue;
                    };
                    if link[1] == b'-' {
                        continue;
                    }
                    let test_x = x.1 == 0;
                    let (s_default_yx, s_search_yx) =
                        self.get_default_and_search_sets_non_ancestral(y, x);
                    let mut s_default_xy = BTreeSet::new();
                    let mut s_search_xy = Vec::new();
                    if test_x {
                        (s_default_xy, s_search_xy) =
                            self.get_default_and_search_sets_non_ancestral(x, y);
                    }
                    if s_search_yx.len() < p_pc || (test_x && s_search_xy.len() < p_pc) {
                        self.write_link(x, y, Some([link[0], b'-', link[2]]));
                        continue;
                    } else {
                        has_converged = false;
                    }
                    let s_search_yx = self.sort_search_set(s_search_yx, y);
                    let s_search_xy = if test_x {
                        self.sort_search_set(s_search_xy, x)
                    } else {
                        Vec::new()
                    };
                    let mut separated = false;
                    for s_pc in combinations(&s_search_yx, p_pc) {
                        let mut z: Sepset = s_pc.iter().copied().collect();
                        z.extend(s_default_yx.iter().copied());
                        let (val, pval) = self.run_ci(x, y, &z);
                        self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                        if pval > self.pc_alpha {
                            to_remove[y.0].insert(x);
                            self.save_sepset(x, y, z.clone(), true);
                            separated = true;
                            break;
                        }
                    }
                    let _ = separated;
                    if test_x {
                        for s_pc in combinations(&s_search_xy, p_pc) {
                            let mut z: Sepset = s_pc.iter().copied().collect();
                            z.extend(s_default_xy.iter().copied());
                            let (val, pval) = self.run_ci(x, y, &z);
                            self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                            if pval > self.pc_alpha {
                                to_remove[y.0].insert(x);
                                self.save_sepset(x, y, z.clone(), true);
                                break;
                            }
                        }
                    }
                }
                let mut any_removal_this = false;
                for j in 0..self.n {
                    for &x in &to_remove[j].clone() {
                        any_removal = true;
                        any_removal_this = true;
                        self.write_link(x, (j, 0), None);
                    }
                }
                if any_removal_this {
                    self.initialize_full_graph();
                    for m in self.na_pds_t.values_mut() {
                        m.clear();
                    }
                }
            }
            if any_removal {
                let rules = self.rules_all.clone();
                let any_update = self.run_orientation_phase(&rules, false);
                if any_update {
                    self.initialize_full_graph();
                    for m in self.na_pds_t.values_mut() {
                        m.clear();
                    }
                    p_pc = 0;
                } else {
                    p_pc += 1;
                }
            } else if has_converged {
                break;
            } else {
                p_pc += 1;
            }
        }
        let rules = self.rules_all.clone();
        self.run_orientation_phase(&rules, false);
    }

    fn run_orientation_phase(&mut self, rule_list: &[&'static str], only_lagged: bool) -> bool {
        let mut restarted_once = false;
        let mut idx = 0;
        while idx < rule_list.len() {
            if idx == 0 {
                self.initialize_full_graph();
            }
            let mut restart = false;
            let rule = rule_list[idx];
            let to_orient = self.apply_rule(rule, only_lagged);
            let mut links_to_remove: BTreeSet<PairKey> = BTreeSet::new();
            let mut links_to_fix: BTreeSet<PairKey> = BTreeSet::new();
            let mut new_ancs: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); self.n];
            let mut new_non_ancs: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); self.n];
            for ((i, j, lag_i), new_link) in &to_orient {
                let (i, j, lag_i) = (*i, *j, *lag_i);
                let old_link = self.get_link((i, lag_i), (j, 0));
                match new_link {
                    None => {
                        if old_link.is_some() {
                            links_to_remove.insert((i, j, lag_i));
                        }
                        continue;
                    }
                    Some(new_link) => {
                        let old = old_link.expect("orienting an existing link");
                        if new_link[1] == b'-' && old[1] != b'-' {
                            links_to_fix.insert((i, j, lag_i));
                        }
                        if new_link[0] == b'-' && old[0] != b'-' {
                            new_ancs[j].insert((i, lag_i));
                        } else if new_link[0] == b'<' && old[0] != b'<' {
                            new_non_ancs[j].insert((i, lag_i));
                        }
                        if lag_i == 0 {
                            if new_link[2] == b'-' && old[2] != b'-' {
                                new_ancs[i].insert((j, 0));
                            } else if new_link[2] == b'>' && old[2] != b'>' {
                                new_non_ancs[i].insert((j, 0));
                            }
                        }
                    }
                }
            }
            let ambiguous_links: BTreeSet<PairKey> = links_to_fix
                .intersection(&links_to_remove)
                .copied()
                .collect();
            links_to_fix = links_to_fix.difference(&ambiguous_links).copied().collect();
            links_to_remove = links_to_remove
                .difference(&ambiguous_links)
                .copied()
                .collect();
            for &(i, j, lag_i) in &links_to_remove {
                self.write_link((i, lag_i), (j, 0), None);
                restart = true;
            }
            for &(i, j, lag_i) in &links_to_fix {
                let old = self
                    .get_link((i, lag_i), (j, 0))
                    .expect("fixing an existing link");
                self.write_link((i, lag_i), (j, 0), Some([old[0], b'-', old[2]]));
                restart = true;
            }
            for &(i, j, lag_i) in &ambiguous_links {
                let old = self
                    .get_link((i, lag_i), (j, 0))
                    .expect("marking an existing link");
                self.write_link((i, lag_i), (j, 0), Some([old[0], b'x', old[2]]));
            }
            restart = self.apply_new_ancestral_information(&new_non_ancs, &new_ancs) || restart;
            if !links_to_remove.is_empty() {
                for &(i, j, lag_i) in &links_to_remove {
                    let x = (i, lag_i);
                    let y = (j, 0);
                    let mut ancs_xy = self.get_ancs(&[x, y]);
                    ancs_xy.remove(&x);
                    ancs_xy.remove(&y);
                    let old_all: BTreeSet<Sepset> =
                        self.get_sepsets(x, y).into_iter().map(|(z, _)| z).collect();
                    let min_size = old_all
                        .iter()
                        .map(|z| z.len())
                        .min()
                        .expect("removed link has sepsets");
                    let smallest: BTreeSet<Sepset> = old_all
                        .into_iter()
                        .filter(|z| z.len() == min_size)
                        .collect();
                    self.delete_sepsets(x, y);
                    self.make_sepset_weakly_minimal(x, y, &smallest, &ancs_xy);
                }
            }
            if restart {
                idx = 0;
                restarted_once = true;
            } else {
                idx += 1;
            }
        }
        restarted_once
    }

    // ---------------------------------------------------------- search sets

    fn get_default_and_search_sets_ancestral(&mut self, a: Node, b: Node) -> (Sepset, Vec<Node>) {
        let s_raw = self.get_a_pds_t(a, b);
        let mut s_default = self.get_parents(a, b);
        s_default.remove(&a);
        s_default.remove(&b);
        let s_search: Vec<Node> = s_raw.difference(&s_default).copied().collect();
        (s_default, s_search)
    }

    fn get_default_and_search_sets_non_ancestral(
        &mut self,
        a: Node,
        b: Node,
    ) -> (Sepset, Vec<Node>) {
        let s_raw = self.get_na_pds_t(a, b);
        let ancs = self.get_ancs(&[a, b]);
        let mut s_default: Sepset = s_raw.intersection(&ancs).copied().collect();
        s_default.extend(self.get_parents(a, b));
        s_default.remove(&a);
        s_default.remove(&b);
        let s_search: Vec<Node> = s_raw.difference(&s_default).copied().collect();
        (s_default, s_search)
    }

    fn get_a_pds_t(&self, a: Node, b: Node) -> Sepset {
        let (var_a, lag_a) = a;
        self.graph_dict[var_a]
            .iter()
            .filter_map(|(&(var, lag), &link)| {
                let link = link?;
                let node = (var, lag + lag_a);
                if node != b && node.1 >= -self.tau_max && link[0] != b'<' {
                    Some(node)
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_na_pds_t(&mut self, a: Node, b: Node) -> Sepset {
        if let Some(memo) = self.na_pds_t.get(&a).and_then(|m| m.get(&b)) {
            return memo.clone();
        }
        let (var_a, lag_a) = a;
        let (var_b, lag_b) = b;
        let mut na_pds_t_1: Sepset = BTreeSet::new();
        for (&(var, lag), &link) in &self.graph_dict[var_a] {
            let Some(link) = link else { continue };
            let node = (var, lag + lag_a);
            if node != b
                && node.1 >= -self.tau_max
                && node.1 <= 0
                && link[0] != b'<'
                && !self.def_ancs[var].contains(&(var_b, lag_b - node.1))
            {
                na_pds_t_1.insert(node);
            }
        }
        let mut c1_list: Sepset = BTreeSet::new();
        for (&(var, lag), &link) in &self.graph_full_dict[var_a] {
            let node = (var, lag + lag_a);
            if node == b || node.1 < -self.tau_max || node.1 > lag_b {
                continue;
            }
            let link = link.expect("full graph holds only links");
            if link[0] == b'-' || link[2] == b'-' {
                continue;
            }
            if self.def_non_ancs[var_b].contains(&(var, node.1 - lag_b)) {
                continue;
            }
            c1_list.insert(node);
        }
        let mut visited: BTreeSet<(Node, Node)> = BTreeSet::new();
        let mut start_from: BTreeSet<(Node, Node)> = c1_list.iter().map(|&c1| (c1, a)).collect();
        while !start_from.is_empty() {
            let mut new_start_from = BTreeSet::new();
            for &(current, previous) in &start_from {
                visited.insert((current, previous));
                let neighbours: Vec<Node> =
                    self.graph_full_dict[current.0].keys().copied().collect();
                for (var, lag) in neighbours {
                    let next = (var, lag + current.1);
                    if next.1 < -self.tau_max || next.1 > 0 {
                        continue;
                    }
                    if visited.contains(&(next, current))
                        || next == previous
                        || next == b
                        || next == a
                    {
                        continue;
                    }
                    let link_l = self.get_link(next, current).expect("adjacent");
                    let link_r = self.get_link(previous, current).expect("adjacent");
                    if link_l[2] == b'-' || link_r[2] == b'-' {
                        continue;
                    }
                    if self.get_link(next, previous).is_none()
                        && (link_l[2] == b'o' || link_r[2] == b'o')
                    {
                        continue;
                    }
                    if self.def_ancs[next.0].contains(&(var_a, lag_a - next.1))
                        || self.def_ancs[next.0].contains(&(var_b, lag_b - next.1))
                    {
                        continue;
                    }
                    let non_anc_a = next.1 - lag_a > 0
                        || self.def_non_ancs[var_a].contains(&(next.0, next.1 - lag_a));
                    let non_anc_b = next.1 - lag_b > 0
                        || self.def_non_ancs[var_b].contains(&(next.0, next.1 - lag_b));
                    if non_anc_a && non_anc_b {
                        continue;
                    }
                    new_start_from.insert((next, current));
                }
            }
            start_from = new_start_from;
        }
        let mut out: Sepset = na_pds_t_1;
        out.extend(visited.iter().map(|&(node, _)| node));
        out.remove(&a);
        out.remove(&b);
        self.na_pds_t
            .get_mut(&a)
            .expect("memo row")
            .insert(b, out.clone());
        out
    }

    fn get_parents(&self, a: Node, b: Node) -> Sepset {
        // parents_of_lagged = true.
        let collect = |node: Node| -> Sepset {
            self.graph_dict[node.0]
                .iter()
                .filter_map(|(&(var, lag), &link)| {
                    let link = link?;
                    if link[0] == b'-' && lag + node.1 >= -self.tau_max {
                        Some((var, lag + node.1))
                    } else {
                        None
                    }
                })
                .collect()
        };
        let mut out = collect(a);
        out.extend(collect(b));
        out
    }

    fn get_ancs(&self, nodes: &[Node]) -> Sepset {
        let mut out = BTreeSet::new();
        for &(var_a, lag_a) in nodes {
            out.extend(self.def_ancs[var_a].iter().filter_map(|&(var, lag)| {
                if lag + lag_a >= -self.tau_max {
                    Some((var, lag + lag_a))
                } else {
                    None
                }
            }));
        }
        out
    }

    // ----------------------------------------------------------- sepset logic

    fn make_sepset_weakly_minimal(
        &mut self,
        x: Node,
        y: Node,
        z_list: &BTreeSet<Sepset>,
        ancs: &Sepset,
    ) {
        let mut any_weakly_minimal = false;
        for z in z_list {
            if z.len() <= 1 || z.is_subset(ancs) {
                self.save_sepset(x, y, z.clone(), true);
                any_weakly_minimal = true;
            }
        }
        if any_weakly_minimal {
            return;
        }
        let mut sepsets_next_call: BTreeSet<Sepset> = BTreeSet::new();
        for z in z_list {
            let removable: Vec<Node> = z.difference(ancs).copied().collect();
            let mut new_sepsets: Vec<Sepset> = Vec::new();
            let mut val_values: Vec<f64> = Vec::new();
            for &a in &removable {
                let z_a: Sepset = z.iter().copied().filter(|&node| node != a).collect();
                let (val, pval) = self.run_ci(x, y, &z_a);
                self.update_pval_val_card_dicts(x, y, pval, val, z_a.len());
                if pval > self.pc_alpha {
                    new_sepsets.push(z_a);
                    val_values.push(val);
                }
            }
            if new_sepsets.is_empty() {
                self.save_sepset(x, y, z.clone(), true);
                any_weakly_minimal = true;
            }
            if !any_weakly_minimal {
                // Sepsets sort by val descending, but the equality chain
                // walks the UNSORTED values and indexing starts at -1, Python-style.
                let mut order: Vec<usize> = (0..new_sepsets.len()).collect();
                order.sort_by(|&p, &q| val_values[q].partial_cmp(&val_values[p]).unwrap());
                let sorted_sepsets: Vec<Sepset> =
                    order.iter().map(|&k| new_sepsets[k].clone()).collect();
                let len = val_values.len();
                let mut i: isize = -1;
                while i <= len as isize - 2 && val_values[(i + 1) as usize] == val_values[0] {
                    let idx = if i < 0 { len - 1 } else { i as usize };
                    sepsets_next_call.insert(sorted_sepsets[idx].clone());
                    i += 1;
                }
                assert!(i >= 0);
            }
        }
        if !any_weakly_minimal {
            self.make_sepset_weakly_minimal(x, y, &sepsets_next_call, ancs);
        }
    }

    fn b_not_in_sepset_ac(&mut self, a: Node, b: Node, c: Node) -> bool {
        self.majority_vote(a, b, c, false)
    }

    fn b_in_sepset_ac(&mut self, a: Node, b: Node, c: Node) -> bool {
        self.majority_vote(a, b, c, true)
    }

    /// Shared body of _B_in_SepSet_AC and _B_not_in_SepSet_AC at default parameters.
    fn majority_vote(&mut self, a: Node, b: Node, c: Node, in_variant: bool) -> bool {
        if c.1 < a.1 || (c.1 == a.1 && c.0 < a.0) {
            return self.majority_vote(c, b, a, in_variant);
        }
        if in_variant {
            let link_ab = self.get_link(a, b);
            let link_cb = self.get_link(c, b);
            let fixed = |l: Link| l.map(|l| l[1] == b'-').unwrap_or(false);
            if link_ab.is_none() || link_cb.is_none() || !fixed(link_ab) || !fixed(link_cb) {
                let all_sepsets: BTreeSet<Sepset> =
                    self.get_sepsets(a, c).into_iter().map(|(z, _)| z).collect();
                let n_sepsets = all_sepsets.len();
                let with_b = all_sepsets.iter().filter(|z| z.contains(&b)).count();
                return 2 * with_b > n_sepsets;
            }
        }
        let mut all_sepsets: BTreeSet<Sepset> = BTreeSet::new();
        // use_a_pds_t_for_majority = true.
        let mut adj_a = self.get_a_pds_t(a, c);
        adj_a.remove(&a);
        adj_a.remove(&c);
        let mut adj_c = self.get_a_pds_t(c, a);
        adj_c.remove(&a);
        adj_c.remove(&c);
        let mut z_add = self.get_parents(a, c);
        z_add.remove(&a);
        z_add.remove(&c);
        let search_a: Vec<Node> = adj_a.difference(&z_add).copied().collect();
        let search_c: Vec<Node> = adj_c.difference(&z_add).copied().collect();
        // max_q_global is infinite, so no sorting of the search sets here.
        let max_p_a = if a.1 < c.1 {
            // max_cond_px = 0.
            search_a.len().min(0) + 1
        } else {
            search_a.len() + 1
        };
        let search_a: Vec<Node> = search_a
            .iter()
            .map(|&(var, lag)| (var, lag - c.1))
            .collect();
        let search_c: Vec<Node> = search_c
            .iter()
            .map(|&(var, lag)| (var, lag - c.1))
            .collect();
        let z_add: Sepset = z_add.iter().map(|&(var, lag)| (var, lag - c.1)).collect();
        let x = (a.0, a.1 - c.1);
        let y = (c.0, 0);
        for p in 0..max_p_a {
            for z_raw in combinations(&search_a, p) {
                let mut z: Sepset = z_raw
                    .iter()
                    .copied()
                    .filter(|&node| node != x && node != y)
                    .collect();
                z.extend(z_add.iter().copied());
                let (val, pval) = self.run_ci(x, y, &z);
                self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                if pval > self.pc_alpha {
                    all_sepsets.insert(z);
                }
            }
        }
        for p in 0..=search_c.len() {
            for z_raw in combinations(&search_c, p) {
                let mut z: Sepset = z_raw
                    .iter()
                    .copied()
                    .filter(|&node| node != x && node != y)
                    .collect();
                z.extend(z_add.iter().copied());
                let (val, pval) = self.run_ci(x, y, &z);
                self.update_pval_val_card_dicts(x, y, pval, val, z.len());
                if pval > self.pc_alpha {
                    all_sepsets.insert(z);
                }
            }
        }
        all_sepsets.extend(self.get_sepsets(x, y).into_iter().map(|(z, _)| z));
        let n_sepsets = all_sepsets.len();
        let b_shifted = (b.0, b.1 - c.1);
        let with_b = all_sepsets
            .iter()
            .filter(|z| z.contains(&b_shifted))
            .count();
        if in_variant {
            2 * with_b > n_sepsets
        } else {
            2 * with_b < n_sepsets
        }
    }

    // -------------------------------------------------------- ancestral info

    fn apply_new_ancestral_information(
        &mut self,
        new_non_ancs: &[BTreeSet<Node>],
        new_ancs: &[BTreeSet<Node>],
    ) -> bool {
        let n = self.n;
        let mut new_non_ancs: Vec<BTreeSet<Node>> = new_non_ancs.to_vec();
        let new_ancs: Vec<BTreeSet<Node>> = new_ancs.to_vec();
        let mut add_def_non_ancs: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); n];
        let mut add_def_ancs: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); n];
        let mut add_ambiguous: Vec<BTreeSet<Node>> = vec![BTreeSet::new(); n];
        let mut put_head_or_tail = false;
        for j in 0..n {
            for &(i, lag_i) in &new_ancs[j] {
                if lag_i == 0 {
                    new_non_ancs[i].insert((j, 0));
                }
            }
        }
        for j in 0..n {
            for &node in &new_non_ancs[j] {
                if self.ambiguous_ancestorships[j].contains(&node) {
                } else if self.def_ancs[j].contains(&node) || new_ancs[j].contains(&node) {
                    add_ambiguous[j].insert(node);
                } else {
                    add_def_non_ancs[j].insert(node);
                }
            }
        }
        for j in 0..n {
            for &node in &new_ancs[j] {
                let (i, lag_i) = node;
                if self.ambiguous_ancestorships[j].contains(&node) {
                } else if lag_i == 0 && self.ambiguous_ancestorships[i].contains(&(j, 0)) {
                } else if self.def_non_ancs[j].contains(&node) || new_non_ancs[j].contains(&node) {
                    add_ambiguous[j].insert(node);
                } else {
                    add_def_ancs[j].insert(node);
                }
            }
        }
        for j in 0..n {
            for &node in &add_ambiguous[j] {
                let (i, lag_i) = node;
                if let Some(old) = self.get_link(node, (j, 0)) {
                    if old[0] != b'x' {
                        self.write_link(node, (j, 0), Some([b'x', old[1], old[2]]));
                    }
                }
                self.def_ancs[j].remove(&node);
                self.def_non_ancs[j].remove(&node);
                if lag_i == 0 {
                    self.def_ancs[i].remove(&(j, 0));
                }
                self.ambiguous_ancestorships[j].insert(node);
            }
        }
        for j in 0..n {
            for &node in &add_def_non_ancs[j] {
                if let Some(old) = self.get_link(node, (j, 0)) {
                    if old[0] != b'<' {
                        self.write_link(node, (j, 0), Some([b'<', old[1], old[2]]));
                        put_head_or_tail = true;
                    }
                }
                self.def_non_ancs[j].insert(node);
            }
            for &node in &add_def_ancs[j] {
                let (_, lag_i) = node;
                if let Some(old) = self.get_link(node, (j, 0)) {
                    if old[0] != b'-' || old[2] != b'>' {
                        self.write_link(node, (j, 0), Some([b'-', old[1], b'>']));
                        put_head_or_tail = true;
                    }
                }
                self.def_ancs[j].insert(node);
                if lag_i == 0 {
                    self.def_non_ancs[node.0].insert((j, 0));
                }
            }
        }
        put_head_or_tail
    }

    // -------------------------------------------------------------- the rules

    fn apply_rule(&mut self, rule: &str, only_lagged: bool) -> Vec<(PairKey, Link)> {
        match rule {
            "APR" => self.apply_apr(only_lagged),
            "ER-00-a" => self.apply_er00a(only_lagged),
            "ER-00-b" => self.apply_er00b(only_lagged),
            "ER-00-c" => self.apply_er00c(only_lagged),
            "ER-00-d" => self.apply_er00d(only_lagged),
            "ER-01" => self.apply_er01(only_lagged),
            "ER-02" => self.apply_er02(only_lagged),
            "ER-03" => self.apply_er03(only_lagged),
            "R-04" => self.apply_r04(only_lagged),
            "ER-08" => self.apply_er08(only_lagged),
            "ER-09" => self.apply_er09(only_lagged),
            "ER-10" => self.apply_er10(only_lagged),
            _ => unreachable!("unknown rule"),
        }
    }

    fn apply_apr(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        if self.no_apr > 0 {
            return out;
        }
        for j in 0..self.n {
            let keys: Vec<Node> = self.graph_dict[j].keys().copied().collect();
            for (i, lag_i) in keys {
                let a = (i, lag_i);
                let b = (j, 0);
                if only_lagged && lag_i == 0 {
                    continue;
                }
                let link_ab = self.get_link(a, b);
                if match_link("-!>", link_ab)
                    || (match_link("-R>", link_ab) && self.is_smaller(a, b))
                    || (match_link("-L>", link_ab) && self.is_smaller(b, a))
                {
                    out.push(self.pair_key_and_new_link(a, b, lk("-->")));
                }
            }
        }
        out
    }

    fn apply_er01(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let triples = self.find_triples("**>", "o*+", "");
        for (a, b, c) in triples {
            if only_lagged && b.1 == c.1 {
                continue;
            }
            if self.b_in_sepset_ac(a, b, c) {
                let link_bc = self.get_link(b, c).expect("triple link");
                out.push(self.pair_key_and_new_link(b, c, Some([b'-', link_bc[1], b'>'])));
            }
        }
        out
    }

    fn apply_er02(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("-*>", "**>", "+*o").into_iter().collect();
        triples.extend(self.find_triples("**>", "-*>", "+*o"));
        for (a, _, c) in triples {
            if only_lagged && a.1 == c.1 {
                continue;
            }
            let link_ac = self.get_link(a, c).expect("triple link");
            out.push(self.pair_key_and_new_link(a, c, Some([link_ac[0], link_ac[1], b'>'])));
        }
        out
    }

    fn apply_er03(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let quadruples = self.find_quadruples("**>", "<**", "", "+*o", "o*+", "+*o");
        for (a, b, c, d) in quadruples {
            if only_lagged && b.1 == d.1 {
                continue;
            }
            if self.b_in_sepset_ac(a, d, c) {
                let link_db = self.get_link(d, b).expect("quadruple link");
                out.push(self.pair_key_and_new_link(d, b, Some([link_db[0], link_db[1], b'>'])));
            }
        }
        out
    }

    fn apply_r04(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let triples = self.find_triples("<-*", "o-+", "-->");
        for triple in triples {
            let (w, v, y) = triple;
            if only_lagged && v.1 == y.1 && w.1 == v.1 {
                continue;
            }
            let link_wv = self.get_link(w, v);
            let paths = self.r4_discriminating_paths(triple);
            for path in paths {
                let x_1 = *path.last().expect("path nonempty");
                if self.b_in_sepset_ac(x_1, v, y) {
                    out.push(self.pair_key_and_new_link(v, y, lk("-->")));
                } else if link_wv != lk("<-x") && self.b_not_in_sepset_ac(x_1, v, y) {
                    out.push(self.pair_key_and_new_link(v, y, lk("<->")));
                    if link_wv != lk("<->") {
                        out.push(self.pair_key_and_new_link(w, v, lk("<->")));
                    }
                }
            }
        }
        out
    }

    fn apply_er08(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let triples = self.find_triples("-*>", "-*>", "o*+");
        for (a, _, c) in triples {
            if only_lagged && a.1 == c.1 {
                continue;
            }
            let link_ac = self.get_link(a, c).expect("triple link");
            out.push(self.pair_key_and_new_link(a, c, Some([b'-', link_ac[1], b'>'])));
        }
        out
    }

    fn apply_er09(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("o*o", "o*>", "").into_iter().collect();
        triples.extend(self.find_triples("<*o", "o*>", ""));
        triples.extend(self.find_triples("<*-", "o*>", ""));
        for (b_1, a, c) in triples {
            if only_lagged && a.1 == c.1 {
                continue;
            }
            if !self.b_in_sepset_ac(b_1, a, c) {
                continue;
            }
            let link_ac = self.get_link(a, c).expect("triple link");
            let (pair_key, new_link) =
                self.pair_key_and_new_link(a, c, Some([b'-', link_ac[1], b'>']));
            let first_link = self.get_link(a, b_1).expect("triple link");
            let initial: Vec<&str> = if match_link("o*o", Some(first_link)) {
                vec!["-*>", "o*>", "o*o"]
            } else {
                vec!["-*>"]
            };
            let paths = self.potentially_directed_uncovered_paths(b_1, c, &initial);
            for upd_path in paths {
                if upd_path.len() < 3
                    || upd_path.contains(&a)
                    || self.get_link(a, upd_path[1]).is_some()
                {
                    continue;
                }
                if first_link[2] == b'>' {
                    out.push((pair_key, new_link));
                    break;
                }
                if !self.b_in_sepset_ac(a, b_1, upd_path[1]) {
                    continue;
                }
                let mut qualifies = true;
                for i in 0..upd_path.len() - 2 {
                    let left = self
                        .get_link(upd_path[i], upd_path[i + 1])
                        .expect("path link");
                    if left[2] == b'>' {
                        break;
                    }
                    if !self.b_in_sepset_ac(upd_path[i], upd_path[i + 1], upd_path[i + 2]) {
                        qualifies = false;
                        break;
                    }
                }
                if qualifies {
                    out.push((pair_key, new_link));
                    break;
                }
            }
        }
        out
    }

    fn apply_er10(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("o*>", "<*-", "").into_iter().collect();
        triples.extend(self.find_triples("o*>", "<*-", "***"));
        let mut sorting: BTreeMap<(Node, Node), Vec<Node>> = BTreeMap::new();
        for (a, c, p_c) in triples {
            sorting.entry((a, c)).or_default().push(p_c);
        }
        let keys: Vec<(Node, Node)> = sorting.keys().copied().collect();
        for (a, c) in keys {
            if only_lagged && a.1 == c.1 {
                continue;
            }
            let mut relevant_paths: Vec<Vec<Node>> = Vec::new();
            for &p_c in &sorting[&(a, c)] {
                for mut upd_path in
                    self.potentially_directed_uncovered_paths(a, p_c, &["-*>", "o*>", "o*o"])
                {
                    if upd_path.len() < 3
                        || upd_path.contains(&c)
                        || self.get_link(upd_path[upd_path.len() - 2], c).is_some()
                    {
                        continue;
                    }
                    upd_path.push(c);
                    let mut qualifies = true;
                    for i in 0..upd_path.len() - 2 {
                        let left = self
                            .get_link(upd_path[i], upd_path[i + 1])
                            .expect("path link");
                        if left[2] == b'>' {
                            break;
                        }
                        if !self.b_in_sepset_ac(upd_path[i], upd_path[i + 1], upd_path[i + 2]) {
                            qualifies = false;
                            break;
                        }
                    }
                    if qualifies {
                        relevant_paths.push(upd_path);
                    }
                }
            }
            let second_nodes: Vec<Node> = relevant_paths
                .iter()
                .map(|p| p[1])
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            'pairs: for i in 0..second_nodes.len() {
                for j in 0..second_nodes.len() {
                    if i < j
                        && self.get_link(second_nodes[i], second_nodes[j]).is_none()
                        && self.b_in_sepset_ac(second_nodes[i], a, second_nodes[j])
                    {
                        let link_ac = self.get_link(a, c).expect("pair link");
                        out.push(self.pair_key_and_new_link(a, c, Some([b'-', link_ac[1], b'>'])));
                        break 'pairs;
                    }
                }
            }
        }
        out
    }

    /// Shared removal body of ER-00-a and ER-00-b, testing the sepsets of (A, C) against link A-B.
    fn er00_removal(&mut self, a: Node, b: Node, sepsets: &BTreeSet<Sepset>) -> bool {
        let (i, lag_i) = a;
        let (j, lag_j) = b;
        let mut z_add = self.get_parents(a, b);
        z_add.remove(&a);
        z_add.remove(&b);
        let (x, y, delta_lag) = if lag_i <= lag_j {
            ((i, lag_i - lag_j), (j, 0), lag_j)
        } else {
            ((j, lag_j - lag_i), (i, 0), lag_i)
        };
        let mut removed = false;
        for z in sepsets {
            let mut z_test: Sepset = z.union(&z_add).copied().collect();
            z_test.remove(&a);
            z_test.remove(&b);
            let z_test: Sepset = z_test
                .into_iter()
                .filter_map(|(var, lag)| {
                    let shifted = lag - delta_lag;
                    if (-self.tau_max..=0).contains(&shifted) {
                        Some((var, shifted))
                    } else {
                        None
                    }
                })
                .collect();
            let (val, pval) = self.run_ci(x, y, &z_test);
            self.update_pval_val_card_dicts(x, y, pval, val, z_test.len());
            if pval > self.pc_alpha {
                removed = true;
                self.save_sepset(x, y, z_test, false);
            }
        }
        removed
    }

    fn apply_er00a(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let triples = self.find_triples("***", "***", "");
        for (a, b, c) in triples {
            if only_lagged && (a.1 == b.1 || b.1 == c.1) {
                continue;
            }
            let sepsets: BTreeSet<Sepset> = self
                .get_sepsets(a, c)
                .into_iter()
                .filter(|(_, wm)| *wm)
                .map(|(z, _)| z)
                .collect();
            let link_ab = self.get_link(a, b).expect("triple link");
            let mut remove_ab = false;
            if link_ab[1] != b'-' && link_ab[1] != b'x' {
                remove_ab = self.er00_removal(a, b, &sepsets);
                if remove_ab {
                    out.push(self.pair_key_and_new_link(a, b, None));
                }
            }
            let link_cb = self.get_link(c, b).expect("triple link");
            let mut remove_cb = false;
            if link_cb[1] != b'-' && link_cb[1] != b'x' {
                remove_cb = self.er00_removal(c, b, &sepsets);
                if remove_cb {
                    out.push(self.pair_key_and_new_link(c, b, None));
                }
            }
            if remove_ab
                || remove_cb
                || link_ab[2] == b'-'
                || link_ab[2] == b'x'
                || link_cb[2] == b'-'
                || link_cb[2] == b'x'
                || link_ab[1] == b'x'
                || link_cb[1] == b'x'
                || (link_ab[2] == b'>' && link_cb[2] == b'>')
            {
                continue;
            }
            if self.b_not_in_sepset_ac(a, b, c) {
                if link_ab[2] != b'>' {
                    out.push(self.pair_key_and_new_link(
                        a,
                        b,
                        Some([link_ab[0], link_ab[1], b'>']),
                    ));
                }
                if link_cb[2] != b'>' {
                    out.push(self.pair_key_and_new_link(
                        c,
                        b,
                        Some([link_cb[0], link_cb[1], b'>']),
                    ));
                }
            }
        }
        out
    }

    fn apply_er00b(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("**>", "o!+", "").into_iter().collect();
        triples.extend(
            self.find_triples("**>", "oR+", "")
                .into_iter()
                .filter(|t| self.is_smaller(t.1, t.2)),
        );
        triples.extend(
            self.find_triples("**>", "oL+", "")
                .into_iter()
                .filter(|t| self.is_smaller(t.2, t.1)),
        );
        for (a, b, c) in triples {
            if only_lagged && a.1 == b.1 {
                continue;
            }
            let sepsets: BTreeSet<Sepset> = self
                .get_sepsets(a, c)
                .into_iter()
                .filter(|(_, wm)| *wm)
                .map(|(z, _)| z)
                .collect();
            let link_ab = self.get_link(a, b).expect("triple link");
            let mut remove_ab = false;
            if link_ab[1] != b'-' && link_ab[1] != b'x' {
                remove_ab = self.er00_removal(a, b, &sepsets);
                if remove_ab {
                    out.push(self.pair_key_and_new_link(a, b, None));
                }
            }
            if only_lagged && b.1 == c.1 {
                continue;
            }
            if remove_ab || link_ab[1] == b'x' {
                continue;
            }
            if self.b_not_in_sepset_ac(a, b, c) {
                let link_cb = self.get_link(c, b).expect("triple link");
                out.push(self.pair_key_and_new_link(c, b, Some([link_cb[0], link_cb[1], b'>'])));
            }
        }
        out
    }

    fn apply_er00c(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("*-*", "o!+", "").into_iter().collect();
        triples.extend(
            self.find_triples("*-*", "oR+", "")
                .into_iter()
                .filter(|t| self.is_smaller(t.1, t.2)),
        );
        triples.extend(
            self.find_triples("*-*", "oL+", "")
                .into_iter()
                .filter(|t| self.is_smaller(t.2, t.1)),
        );
        for (a, b, c) in triples {
            if only_lagged && b.1 == c.1 {
                continue;
            }
            if self.b_not_in_sepset_ac(a, b, c) {
                let link_cb = self.get_link(c, b).expect("triple link");
                out.push(self.pair_key_and_new_link(c, b, Some([link_cb[0], link_cb[1], b'>'])));
            }
        }
        out
    }

    fn apply_er00d(&mut self, only_lagged: bool) -> Vec<(PairKey, Link)> {
        let mut out = Vec::new();
        let mut triples: BTreeSet<(Node, Node, Node)> =
            self.find_triples("*-o", "o-*", "").into_iter().collect();
        triples.extend(self.find_triples("*->", "o-*", ""));
        for (a, b, c) in triples {
            if only_lagged && a.1 == b.1 && b.1 == c.1 {
                continue;
            }
            if self.b_not_in_sepset_ac(a, b, c) {
                if !only_lagged || b.1 != c.1 {
                    let link_cb = self.get_link(c, b).expect("triple link");
                    out.push(self.pair_key_and_new_link(
                        c,
                        b,
                        Some([link_cb[0], link_cb[1], b'>']),
                    ));
                }
                let link_ab = self.get_link(a, b).expect("triple link");
                if (!only_lagged || a.1 != b.1) && link_ab[2] == b'o' {
                    out.push(self.pair_key_and_new_link(
                        a,
                        b,
                        Some([link_ab[0], link_ab[1], b'>']),
                    ));
                }
            }
        }
        out
    }

    // --------------------------------------------------- graph pattern search

    fn snapshot(&self) -> Vec<Vec<Vec<Link>>> {
        let t = self.tau_max as usize + 1;
        let mut graph = vec![vec![vec![None; t]; self.n]; self.n];
        for j in 0..self.n {
            for (&(i, lag_i), &link) in &self.graph_dict[j] {
                graph[i][j][lag_i.unsigned_abs() as usize] = link;
            }
        }
        graph
    }

    fn find_adj(
        &self,
        graph: &[Vec<Vec<Link>>],
        node: Node,
        patterns: &[&str],
        exclude: &[Node],
    ) -> Vec<Node> {
        let (i, lag_i) = node;
        let mut adj: Vec<Node> = Vec::new();
        for k in 0..self.n {
            for lag_ik in 0..=self.tau_max as usize {
                if graph[i][k][lag_ik].is_none() {
                    continue;
                }
                if patterns.iter().any(|p| match_link(p, graph[i][k][lag_ik])) {
                    let m = (k, lag_i + lag_ik as i32);
                    if !adj.contains(&m) && !exclude.contains(&m) {
                        adj.push(m);
                    }
                }
            }
        }
        for k in 0..self.n {
            for lag_ki in 0..=self.tau_max as usize {
                if graph[k][i][lag_ki].is_none() {
                    continue;
                }
                let reversed: Vec<String> = patterns
                    .iter()
                    .map(|p| {
                        let r = reverse_link(lk(p)).expect("pattern reverses");
                        String::from_utf8(r.to_vec()).expect("ascii")
                    })
                    .collect();
                if reversed.iter().any(|p| match_link(p, graph[k][i][lag_ki])) {
                    let m = (k, lag_i - lag_ki as i32);
                    if !adj.contains(&m) && !exclude.contains(&m) {
                        adj.push(m);
                    }
                }
            }
        }
        adj
    }

    fn is_match(&self, graph: &[Vec<Vec<Link>>], x: Node, y: Node, pattern_ij: &str) -> bool {
        let (i, lag_i) = x;
        let (j, lag_j) = y;
        let tauij = lag_j - lag_i;
        if tauij.unsigned_abs() as usize >= graph[0][0].len() {
            return false;
        }
        if tauij >= 0 {
            match_link(pattern_ij, graph[i][j][tauij as usize])
        } else {
            let r = reverse_link(lk_or_empty(pattern_ij));
            let pattern = r
                .map(|r| String::from_utf8(r.to_vec()).expect("ascii"))
                .unwrap_or_default();
            match_link(&pattern, graph[j][i][(-tauij) as usize])
        }
    }

    fn find_triples(
        &self,
        pattern_ij: &str,
        pattern_jk: &str,
        pattern_ik: &str,
    ) -> Vec<(Node, Node, Node)> {
        let graph = self.snapshot();
        let mut matched = Vec::new();
        for i in 0..self.n {
            let node_i = (i, 0);
            for (j, lag_j) in self.find_adj(&graph, node_i, &[pattern_ij], &[]) {
                for (k, lag_k) in self.find_adj(&graph, (j, lag_j), &[pattern_jk], &[node_i]) {
                    if self.is_match(&graph, node_i, (k, lag_k), pattern_ik) {
                        let rightmost = 0.max(lag_j).max(lag_k);
                        let m = (
                            (i, -rightmost),
                            (j, lag_j - rightmost),
                            (k, lag_k - rightmost),
                        );
                        let largest = (0 - rightmost)
                            .min(lag_j - rightmost)
                            .min(lag_k - rightmost);
                        if !matched.contains(&m) && largest >= -self.tau_max {
                            matched.push(m);
                        }
                    }
                }
            }
        }
        matched
    }

    #[allow(clippy::too_many_arguments)]
    fn find_quadruples(
        &self,
        pattern_ij: &str,
        pattern_jk: &str,
        pattern_ik: &str,
        pattern_il: &str,
        pattern_jl: &str,
        pattern_kl: &str,
    ) -> Vec<(Node, Node, Node, Node)> {
        let graph = self.snapshot();
        let mut matched = Vec::new();
        for (a, b, c) in self.find_triples(pattern_ij, pattern_jk, pattern_ik) {
            let mut adjacencies: BTreeSet<Node> = self
                .find_adj(&graph, a, &[pattern_il], &[b, c])
                .into_iter()
                .collect();
            if pattern_jl.is_empty() {
                adjacencies.retain(|&adj| self.is_match(&graph, b, adj, ""));
            } else {
                let with_j: BTreeSet<Node> = self
                    .find_adj(&graph, b, &[pattern_jl], &[a, c])
                    .into_iter()
                    .collect();
                adjacencies = adjacencies.intersection(&with_j).copied().collect();
            }
            if pattern_kl.is_empty() {
                adjacencies.retain(|&adj| self.is_match(&graph, c, adj, ""));
            } else {
                let with_k: BTreeSet<Node> = self
                    .find_adj(&graph, c, &[pattern_kl], &[a, b])
                    .into_iter()
                    .collect();
                adjacencies = adjacencies.intersection(&with_k).copied().collect();
            }
            for (l, lag_l) in adjacencies {
                let rightmost = a.1.max(b.1).max(c.1).max(lag_l);
                let m = (
                    (a.0, a.1 - rightmost),
                    (b.0, b.1 - rightmost),
                    (c.0, c.1 - rightmost),
                    (l, lag_l - rightmost),
                );
                let largest = (a.1 - rightmost)
                    .min(b.1 - rightmost)
                    .min(c.1 - rightmost)
                    .min(lag_l - rightmost);
                if !matched.contains(&m) && largest >= -self.tau_max {
                    matched.push(m);
                }
            }
        }
        matched
    }

    fn r4_discriminating_paths(&self, triple: (Node, Node, Node)) -> Vec<Vec<Node>> {
        let (w, v, y) = triple;
        self.r4_search(vec![y, v, w])
    }

    fn r4_search(&self, path_taken: Vec<Node>) -> Vec<Vec<Node>> {
        let last = *path_taken.last().expect("path nonempty");
        let link_to_y = self.get_link(last, path_taken[0]);
        if path_taken.len() > 3 && link_to_y.is_none() {
            return vec![path_taken];
        }
        let mut paths = Vec::new();
        let into_prev = self.get_link(last, path_taken[path_taken.len() - 2]);
        if into_prev.map(|l| l[0] == b'<').unwrap_or(false) && link_to_y == lk("-->") {
            for (&(var, lag), _) in &self.graph_full_dict[last.0] {
                let next = (var, lag + last.1);
                let next_link = self.get_link(next, last);
                if next.1 <= 0
                    && next.1 >= -self.tau_max
                    && !path_taken.contains(&next)
                    && match_link("*->", next_link)
                {
                    let mut extended = path_taken.clone();
                    extended.push(next);
                    paths.extend(self.r4_search(extended));
                }
            }
        }
        paths
    }

    fn potentially_directed_uncovered_paths(
        &self,
        start: Node,
        end: Node,
        initial: &[&str],
    ) -> Vec<Vec<Node>> {
        assert!(start != end);
        let paths = self.upd_search(end, vec![start], initial);
        paths.into_iter().filter(|p| p.len() > 2).collect()
    }

    fn upd_search(&self, end: Node, path_taken: Vec<Node>, allowed: &[&str]) -> Vec<Vec<Node>> {
        let mut paths = Vec::new();
        let start = *path_taken.last().expect("path nonempty");
        if start == end {
            paths.push(path_taken);
            return paths;
        }
        for (&(var, lag), _) in &self.graph_full_dict[start.0] {
            let next = (var, lag + start.1);
            if next.1 < -self.tau_max || next.1 > 0 || path_taken.contains(&next) {
                continue;
            }
            if path_taken.len() >= 2
                && self
                    .get_link(path_taken[path_taken.len() - 2], next)
                    .is_some()
            {
                continue;
            }
            let link = self.get_link(start, next);
            if !allowed.iter().any(|p| match_link(p, link)) {
                continue;
            }
            let new_allowed: &[&str] = if match_link("o*o", link) {
                &["o*o", "o*>", "-*>"]
            } else {
                &["-*>"]
            };
            let mut new_path = path_taken.clone();
            new_path.push(next);
            paths.extend(self.upd_search(end, new_path, new_allowed));
        }
        paths
    }

    // ----------------------------------------------------------- bookkeeping

    fn run_ci(&mut self, x: Node, y: Node, z: &Sepset) -> (f64, f64) {
        let z_list: Vec<Node> = z.iter().copied().collect();
        self.ci
            .run_test(self.data, &[x], &[y], &z_list, self.tau_max as usize)
    }

    fn is_smaller(&self, x: Node, y: Node) -> bool {
        x.1 < y.1 || (x.1 == y.1 && x.0 < y.0)
    }

    fn get_link(&self, a: Node, b: Node) -> Link {
        let (var_a, lag_a) = a;
        let (var_b, lag_b) = b;
        if (lag_a - lag_b).abs() > self.tau_max {
            return None;
        }
        if lag_a <= lag_b {
            self.graph_dict[var_b][&(var_a, lag_a - lag_b)]
        } else {
            reverse_link(self.graph_dict[var_a][&(var_b, lag_b - lag_a)])
        }
    }

    fn write_link(&mut self, a: Node, b: Node, new_link: Link) {
        let (var_a, lag_a) = a;
        let (var_b, lag_b) = b;
        use std::cmp::Ordering;
        match lag_a.cmp(&lag_b) {
            Ordering::Less => {
                self.graph_dict[var_b].insert((var_a, lag_a - lag_b), new_link);
            }
            Ordering::Equal => {
                self.graph_dict[var_b].insert((var_a, 0), new_link);
                self.graph_dict[var_a].insert((var_b, 0), reverse_link(new_link));
            }
            Ordering::Greater => {
                self.graph_dict[var_a].insert((var_b, lag_b - lag_a), reverse_link(new_link));
            }
        }
    }

    fn apply_middle_mark(&mut self, x: Node, y: Node, mark: u8) {
        let old = self.get_link(x, y).expect("marking an existing link");
        let new_mark = if old[1] == b'?' {
            mark
        } else if (old[1] == b'L' && mark == b'R') || (old[1] == b'R' && mark == b'L') {
            b'!'
        } else {
            unreachable!("invalid middle mark transition")
        };
        self.write_link(x, y, Some([old[0], new_mark, old[2]]));
    }

    fn update_middle_marks(&mut self) {
        for j in 0..self.n {
            let entries: Vec<Node> = self.graph_dict[j]
                .iter()
                .filter_map(|(&node, &link)| link.map(|_| node))
                .collect();
            for (i, lag_i) in entries {
                let x = (i, lag_i);
                let y = (j, 0);
                if self.get_link(x, y).is_none() {
                    continue;
                }
                for (from, to) in [(x, y), (y, x)] {
                    let Some(link) = self.get_link(from, to) else {
                        continue;
                    };
                    if link[2] != b'>' {
                        continue;
                    }
                    let smaller = self.is_smaller(from, to);
                    if link[1] == b'?' {
                        let mark = if smaller { b'L' } else { b'R' };
                        self.write_link(from, to, Some([link[0], mark, b'>']));
                    } else if (link[1] == b'R' && smaller) || (link[1] == b'L' && !smaller) {
                        self.write_link(from, to, Some([link[0], b'!', b'>']));
                    }
                }
            }
        }
    }

    fn initialize_full_graph(&mut self) {
        self.graph_full_dict = vec![BTreeMap::new(); self.n];
        for j in 0..self.n {
            let entries: Vec<(Node, Link)> = self.graph_dict[j]
                .iter()
                .map(|(&node, &link)| (node, link))
                .collect();
            for ((var, lag), link) in entries {
                if let Some(link) = link {
                    self.graph_full_dict[j].insert((var, lag), Some(link));
                    if lag < 0 {
                        self.graph_full_dict[var].insert((j, -lag), reverse_link(Some(link)));
                    }
                }
            }
        }
    }

    fn pair_key_and_new_link(&self, a: Node, b: Node, link_ab: Link) -> (PairKey, Link) {
        let (var_a, lag_a) = a;
        let (var_b, lag_b) = b;
        if lag_a <= lag_b {
            ((var_a, var_b, lag_a - lag_b), link_ab)
        } else {
            ((var_b, var_a, lag_b - lag_a), reverse_link(link_ab))
        }
    }

    fn update_pval_val_card_dicts(&mut self, x: Node, y: Node, pval: f64, val: f64, card: usize) {
        let (row, key) = if x.1 < 0 || x.0 < y.0 {
            (y.0, x)
        } else {
            (x.0, y)
        };
        if pval > self.pval_max[row][&key] {
            self.pval_max[row].insert(key, pval);
            self.pval_max_val[row].insert(key, val);
            self.pval_max_card[row].insert(key, card as f64);
        }
    }

    fn get_pval_max_val(&self, x: Node, y: Node) -> f64 {
        if x.1 < 0 || x.0 < y.0 {
            self.pval_max_val[y.0][&x]
        } else {
            self.pval_max_val[x.0][&y]
        }
    }

    fn sort_search_set(&self, search_set: Vec<Node>, reference: Node) -> Vec<Node> {
        let mut decorated: Vec<(f64, Node)> = search_set
            .into_iter()
            .map(|node| {
                let value = self.get_pval_max_val(node, reference);
                let key = if value == f64::NEG_INFINITY {
                    0.0
                } else {
                    value.abs()
                };
                (key, node)
            })
            .collect();
        // Python sorts (value, node) tuples descending: ties break by node, descending.
        decorated.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(b.1.cmp(&a.1)));
        decorated.into_iter().map(|(_, node)| node).collect()
    }

    fn save_sepset(&mut self, x: Node, y: Node, z: Sepset, weakly_minimal: bool) {
        let (i, lag_i) = x;
        let (j, lag_j) = y;
        assert!(lag_j == 0);
        if lag_i < 0 || i < j {
            self.sepsets[j]
                .get_mut(&x)
                .expect("sepset slot")
                .insert((z, weakly_minimal));
        } else {
            self.sepsets[i]
                .get_mut(&y)
                .expect("sepset slot")
                .insert((z, weakly_minimal));
        }
    }

    fn delete_sepsets(&mut self, x: Node, y: Node) {
        let (i, lag_i) = x;
        let (j, lag_j) = y;
        assert!(lag_j == 0);
        if lag_i < 0 || i < j {
            self.sepsets[j].insert(x, BTreeSet::new());
        } else {
            self.sepsets[i].insert(y, BTreeSet::new());
        }
    }

    fn get_sepsets(&self, a: Node, b: Node) -> BTreeSet<(Sepset, bool)> {
        let (var_a, lag_a) = a;
        let (var_b, lag_b) = b;
        let shift = |z: &Sepset, by: i32| -> Sepset {
            z.iter().map(|&(var, lag)| (var, lag + by)).collect()
        };
        use std::cmp::Ordering;
        let (row, key, by) = match lag_a.cmp(&lag_b) {
            Ordering::Less => (var_b, (var_a, lag_a - lag_b), lag_b),
            Ordering::Greater => (var_a, (var_b, lag_b - lag_a), lag_a),
            Ordering::Equal => (var_a.max(var_b), (var_a.min(var_b), 0), lag_a),
        };
        self.sepsets[row][&key]
            .iter()
            .map(|(z, wm)| (shift(z, by), *wm))
            .collect()
    }

    fn fix_all_edges(&mut self) {
        for j in 0..self.n {
            let entries: Vec<(Node, Link)> = self.graph_dict[j]
                .iter()
                .map(|(&node, &link)| (node, link))
                .collect();
            for (node, link) in entries {
                if let Some(link) = link {
                    self.graph_dict[j].insert(node, Some([link[0], b'-', link[2]]));
                }
            }
        }
    }

    fn dict2graph(&self) -> Vec<Vec<Vec<String>>> {
        let t = self.tau_max as usize + 1;
        let mut graph = vec![vec![vec![String::new(); t]; self.n]; self.n];
        for j in 0..self.n {
            for (&(i, lag_i), &link) in &self.graph_dict[j] {
                graph[i][j][lag_i.unsigned_abs() as usize] = link
                    .map(|l| String::from_utf8(l.to_vec()).expect("ascii"))
                    .unwrap_or_default();
            }
        }
        graph
    }

    fn dict_to_matrix(&self, dict: &[BTreeMap<Node, f64>], default: f64) -> Vec<Vec<Vec<f64>>> {
        let t = self.tau_max as usize + 1;
        let mut matrix = vec![vec![vec![default; t]; self.n]; self.n];
        for (j, row) in dict.iter().enumerate() {
            for (&(k, tau), &value) in row {
                if tau == 0 {
                    matrix[k][j][0] = value;
                    matrix[j][k][0] = value;
                } else {
                    matrix[k][j][tau.unsigned_abs() as usize] = value;
                }
            }
        }
        matrix
    }
}

fn lk_or_empty(pattern: &str) -> Link {
    if pattern.is_empty() {
        None
    } else {
        lk(pattern)
    }
}
