//! Source-derived fixed-alpha J-PCMCI+ orchestration (GPL-3.0).
use super::*;
use crate::joint_samples::{AnalyticCi, JointData, JointError, Residuals, ShuffleCi};
use crate::parcorr::dedup;
use crate::parcorr_mult::{AnalyticResult, Correlation, FixedDecision, FixedStatistic};
use crate::pcmci::pc_stable_decisions;
use crate::pcmciplus::{adjacencies, combinations, meek_rules_with_conflicts, Mark};
use std::{cell::RefCell, collections::BTreeMap};

/// Statistical evidence and fixed-threshold markers have different meanings.
pub enum TestResult {
    Constant,
    Probability { value: f64, p_value: f64 },
    Fixed(FixedDecision),
}

impl TestResult {
    fn parts(self, alpha: f64) -> (f64, f64, bool) {
        match self {
            Self::Constant => (0.0, 1.0, false),
            Self::Probability { value, p_value } => (value, p_value, p_value <= alpha),
            Self::Fixed(FixedDecision::Dependent { value }) => (value, 0.0, true),
            Self::Fixed(FixedDecision::Independent { value }) => (value, 1.0, false),
            // NumPy stores source None in floating result matrices as NaN.
            Self::Fixed(FixedDecision::Constant) => (0.0, f64::NAN, false),
        }
    }
}

impl From<AnalyticResult> for TestResult {
    fn from(result: AnalyticResult) -> Self {
        match result {
            AnalyticResult::Constant => Self::Constant,
            AnalyticResult::Tested { value, p_value } => Self::Probability { value, p_value },
        }
    }
}

pub struct FixedCi<'a> {
    data: &'a JointData,
    correlation: Correlation,
    cache: RefCell<BTreeMap<[[u8; 20]; 3], FixedStatistic>>,
    residuals: Residuals,
}

impl FixedCi<'_> {
    pub fn with_residual_recycling(mut self) -> Self {
        self.residuals = self.data.recycling_policy();
        self
    }
}

impl JointData {
    pub fn fixed(&self, correlation: Correlation) -> FixedCi<'_> {
        FixedCi {
            data: self,
            correlation,
            cache: RefCell::new(BTreeMap::new()),
            residuals: Residuals::Fresh,
        }
    }
}

type Matrix = Vec<Vec<Vec<f64>>>;
type Graph = Vec<Vec<Vec<Mark>>>;
type Sepsets = Vec<Vec<Vec<Vec<Node>>>>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColliderRule {
    #[default]
    Majority,
    Conservative,
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub collider_rule: ColliderRule,
    pub conflict_resolution: bool,
    pub reset_lagged_links: bool,
    pub tau_min: usize,
    pub max_conds_dim: Option<usize>,
    pub max_combinations: usize,
    pub max_conds_py: Option<usize>,
    pub max_conds_px: Option<usize>,
    pub max_conds_px_lagged: Option<usize>,
    pub fdr: Fdr,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fdr {
    #[default]
    None,
    BenjaminiHochberg,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            collider_rule: ColliderRule::Majority,
            conflict_resolution: true,
            reset_lagged_links: false,
            tau_min: 0,
            max_conds_dim: None,
            max_combinations: 1,
            max_conds_py: None,
            max_conds_px: None,
            max_conds_px_lagged: None,
            fdr: Fdr::None,
        }
    }
}

#[derive(Debug)]
pub enum RunError {
    Shape,
    Configuration,
    InconsistentLinks,
    CyclicAssumptions,
    Constraints(ConstraintError),
    UnsupportedScore,
    ReferenceQueryMissing,
    ThresholdRequired,
    UndefinedThresholdMarker,
    Data(JointError),
}

/// J-PCMCI+ is CI-test agnostic, as in Tigramite. Tests and alternative CI ports
/// implement this boundary; the discovery engine does not assume a scalar test.
pub trait ConditionalIndependence {
    fn variables(&self) -> usize;
    fn test(&self, x: &[Node], y: &[Node], z: &[Node], tau: usize) -> Result<(f64, f64), RunError>;
    fn decision(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
        _alpha: f64,
    ) -> Result<TestResult, RunError> {
        let (value, p) = self.test(x, y, z, tau)?;
        Ok(TestResult::Probability { value, p_value: p })
    }
    fn model_score(&self, _j: usize, _parents: &[Node], _tau: usize) -> Result<f64, RunError> {
        Err(RunError::UnsupportedScore)
    }
}
impl ConditionalIndependence for FixedCi<'_> {
    fn variables(&self) -> usize {
        self.data.variables()
    }
    fn test(
        &self,
        _x: &[Node],
        _y: &[Node],
        _z: &[Node],
        _tau: usize,
    ) -> Result<(f64, f64), RunError> {
        Err(RunError::ThresholdRequired)
    }
    fn decision(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
        alpha: f64,
    ) -> Result<TestResult, RunError> {
        let array = self.data.test_array(x, y, z, tau)?;
        let key = array.ci_key();
        let mut cache = self.cache.borrow_mut();
        let statistic = match cache.get(&key) {
            Some(statistic) => *statistic,
            None => {
                let samples = array.samples()?;
                self.data.check_recycling(&samples, self.residuals)?;
                let statistic = samples
                    .fixed_statistic(self.correlation)
                    .map_err(JointError::Samples)?;
                cache.insert(key, statistic);
                statistic
            }
        };
        Ok(TestResult::Fixed(statistic.decide(alpha)))
    }
    fn model_score(&self, j: usize, parents: &[Node], tau: usize) -> Result<f64, RunError> {
        ConditionalIndependence::model_score(self.data, j, parents, tau)
    }
}
impl ConditionalIndependence for JointData {
    fn variables(&self) -> usize {
        JointData::variables(self)
    }
    fn test(&self, x: &[Node], y: &[Node], z: &[Node], tau: usize) -> Result<(f64, f64), RunError> {
        Ok(JointData::test(self, x, y, z, tau)?)
    }
    fn decision(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
        _alpha: f64,
    ) -> Result<TestResult, RunError> {
        analytic_decision(self, x, y, z, tau, Correlation::MaxCorrelation)
    }
    fn model_score(&self, j: usize, parents: &[Node], tau: usize) -> Result<f64, RunError> {
        self.construct(&[(j, 0)], &[(j, 0)], parents, tau)?
            .samples()?
            .model_score(false)
            .map_err(|e| RunError::Data(JointError::Samples(e)))
    }
}
impl From<JointError> for RunError {
    fn from(e: JointError) -> Self {
        Self::Data(e)
    }
}

impl ConditionalIndependence for AnalyticCi<'_> {
    fn variables(&self) -> usize {
        self.data.variables()
    }

    fn test(&self, x: &[Node], y: &[Node], z: &[Node], tau: usize) -> Result<(f64, f64), RunError> {
        Ok(AnalyticCi::test(self, x, y, z, tau)?)
    }
    fn decision(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
        _alpha: f64,
    ) -> Result<TestResult, RunError> {
        Ok(self.evidence(x, y, z, tau)?.into())
    }

    fn model_score(&self, j: usize, parents: &[Node], tau: usize) -> Result<f64, RunError> {
        // ParCorrMult's AIC does not depend on its correlation statistic.
        ConditionalIndependence::model_score(self.data, j, parents, tau)
    }
}

impl ConditionalIndependence for ShuffleCi<'_> {
    fn variables(&self) -> usize {
        self.data.variables()
    }
    fn test(&self, x: &[Node], y: &[Node], z: &[Node], tau: usize) -> Result<(f64, f64), RunError> {
        Ok(ShuffleCi::test(self, x, y, z, tau)?)
    }
    fn decision(
        &self,
        x: &[Node],
        y: &[Node],
        z: &[Node],
        tau: usize,
        _alpha: f64,
    ) -> Result<TestResult, RunError> {
        let constant = self
            .data
            .test_array(x, y, z, tau)?
            .samples()?
            .nonconstant()
            .map_err(JointError::Samples)?
            .is_none();
        let (value, p_value) = ShuffleCi::test(self, x, y, z, tau)?;
        Ok(if constant {
            TestResult::Constant
        } else {
            TestResult::Probability { value, p_value }
        })
    }
    fn model_score(&self, j: usize, parents: &[Node], tau: usize) -> Result<f64, RunError> {
        ConditionalIndependence::model_score(self.data, j, parents, tau)
    }
}

fn analytic_decision(
    data: &JointData,
    x: &[Node],
    y: &[Node],
    z: &[Node],
    tau: usize,
    correlation: Correlation,
) -> Result<TestResult, RunError> {
    let samples = data.test_array(x, y, z, tau)?.samples()?;
    Ok(samples
        .analytic_result(correlation)
        .map_err(JointError::Samples)?
        .into())
}

pub struct Discovery {
    pub graph: Vec<Vec<Vec<String>>>,
    pub p_matrix: Matrix,
    pub val_matrix: Matrix,
    /// [source][target][absolute lag], including initially empty separating sets.
    pub sepsets: Sepsets,
    pub ambiguous_triples: Vec<(Node, usize, usize)>,
    pub lagged_parents: Vec<Vec<Node>>,
    pub context_parents: Vec<Vec<Node>>,
    pub dummy_parents: Vec<Vec<Node>>,
}

enum Phase<'a> {
    Context,
    Dummy {
        contexts: &'a [Vec<Node>],
    },
    System {
        contexts: &'a [Vec<Node>],
        dummies: &'a [Vec<Node>],
    },
}

fn reciprocal(links: &LinkAssumptions) -> Result<LinkAssumptions, RunError> {
    let mut out = links.clone();
    for j in 0..out.len() {
        for ((i, t), m) in out[j].clone() {
            if t == 0 {
                let reverse = match m {
                    [b'-', c, b'>'] => [b'<', c, b'-'],
                    [b'<', c, b'-'] => [b'-', c, b'>'],
                    other => other,
                };
                match out[i].iter().find(|(node, _)| *node == (j, 0)) {
                    Some((_, mark)) if *mark != reverse => return Err(RunError::InconsistentLinks),
                    Some(_) => {}
                    None => out[i].push(((j, 0), reverse)),
                }
            }
        }
    }
    Ok(out)
}

/// `_set_link_assumptions`: reciprocal endpoints, time ordering, then cycle check.
fn normalize(links: &LinkAssumptions) -> Result<LinkAssumptions, RunError> {
    let mut out = reciprocal(links)?;
    for parents in &mut out {
        for (node, mark) in parents {
            if node.1 != 0 {
                *mark = [b'-', mark[1], b'>'];
            }
        }
    }
    fn visit(j: usize, links: &LinkAssumptions, path: &mut [bool], visited: &mut [bool]) -> bool {
        if visited[j] {
            return false;
        }
        visited[j] = true;
        path[j] = true;
        for &((i, lag), mark) in &links[j] {
            if lag == 0 && mark[0] == b'-' && mark[2] == b'>' {
                if path[i] || visit(i, links, path, visited) {
                    return true;
                }
            }
        }
        path[j] = false;
        false
    }
    let mut path = vec![false; out.len()];
    let mut visited = path.clone();
    for j in 0..out.len() {
        if visit(j, &out, &mut path, &mut visited) {
            return Err(RunError::CyclicAssumptions);
        }
    }
    Ok(out)
}

struct Search<'a> {
    data: &'a dyn ConditionalIndependence,
    classes: &'a [NodeClass],
    tau: usize,
    alpha: f64,
    options: Options,
}
struct Skeleton {
    // Tigramite corrects a copy, retaining its raw matrix for the next phase.
    uncorrected_p: Matrix,
    graph: Graph,
    p: Matrix,
    val: Matrix,
    sepsets: Sepsets,
}

impl Search<'_> {
    fn test(
        &self,
        graph: &Graph,
        i: usize,
        j: usize,
        t: usize,
        s: &[Node],
        parents: &[Vec<Node>],
        phase: &Phase,
    ) -> Result<TestResult, RunError> {
        let mut z = s.to_vec();
        match phase {
            Phase::Context => {}
            Phase::Dummy { contexts } => z.extend(&contexts[j]),
            Phase::System { contexts, dummies } => {
                z.extend(&dummies[j]);
                z.extend(&contexts[j]);
            }
        }
        z = dedup(&z);
        let default_max = self.classes.len() * (self.tau - self.options.tau_min + 1);
        for &node in parents[j]
            .iter()
            .take(self.options.max_conds_py.unwrap_or(default_max))
        {
            if node != (i, -(t as i32)) && !z.contains(&node) {
                z.push(node);
            }
        }
        let max_x = if t == 0 {
            self.options.max_conds_px
        } else {
            self.options
                .max_conds_px_lagged
                .or(self.options.max_conds_px)
        };
        for &(k, lag) in parents[i].iter().take(max_x.unwrap_or(default_max)) {
            let node = (k, lag - t as i32);
            if !z.contains(&node) {
                z.push(node);
            }
        }
        if graph[i][j][t].is_some_and(|m| m[1] == b'-') {
            return Ok(TestResult::Probability {
                value: 1.0,
                p_value: 0.0,
            });
        }
        Ok(self
            .data
            .decision(&[(i, -(t as i32))], &[(j, 0)], &z, self.tau, self.alpha)?)
    }

    fn skeleton(
        &self,
        links: &LinkAssumptions,
        parents: &[Vec<Node>],
        phase: &Phase,
        mut p: Matrix,
        mut val: Matrix,
    ) -> Result<Skeleton, RunError> {
        let n = self.classes.len();
        let links = normalize(links)?;
        let mut selected = vec![Vec::new(); n];
        for j in 0..n {
            for &node in &parents[j] {
                if let Some(&(_, mark)) = links[j].iter().find(|(p, _)| *p == node) {
                    if mark == *b"-?>" || mark == *b"-->" {
                        selected[j].push((node, mark));
                    }
                }
            }
            for &(node, mark) in &links[j] {
                if node.1 == 0 {
                    if let Some(entry) = selected[j].iter_mut().find(|(p, _)| *p == node) {
                        entry.1 = mark;
                    } else {
                        selected[j].push((node, mark));
                    }
                }
            }
        }
        if self.options.reset_lagged_links {
            selected = links.clone();
        }
        let mut graph = vec![vec![vec![None; self.tau + 1]; n]; n];
        for j in 0..n {
            for &((i, t), m) in &selected[j] {
                if (i, t) != (j, 0) {
                    graph[i][j][(-t) as usize] = Some(m);
                }
            }
        }
        let mut sp = vec![vec![vec![0.0; self.tau + 1]; n]; n];
        let mut sv = sp.clone();
        for i in 0..n {
            for j in 0..n {
                for t in 0..=self.tau {
                    if graph[i][j][t].is_none() {
                        sp[i][j][t] = 1.0;
                    }
                }
            }
        }
        let mut sepsets = vec![vec![vec![Vec::new(); self.tau + 1]; n]; n];
        let mut minima: Vec<Vec<(Node, f64)>> = (0..n)
            .map(|j| {
                adjacencies(&graph, j, true)
                    .into_iter()
                    .map(|node| (node, f64::INFINITY))
                    .collect()
            })
            .collect();
        let mut adj: Vec<Vec<Node>> = (0..n)
            .map(|j| {
                adjacencies(&graph, j, true)
                    .into_iter()
                    .filter(|node| node.1 == 0)
                    .collect()
            })
            .collect();
        for card in 0..=self.options.max_conds_dim.unwrap_or(n) {
            let mut pairs = Vec::new();
            for i in 0..n {
                let eligible = match phase {
                    Phase::Context => self.classes[i].context(),
                    Phase::Dummy { .. } => self.classes[i].dummy(),
                    Phase::System { .. } => true,
                };
                if !eligible {
                    continue;
                }
                for j in 0..n {
                    for t in self.options.tau_min..=self.tau {
                        if graph[i][j][t].is_some()
                            && adj[j]
                                .iter()
                                .filter(|&&node| node != (i, -(t as i32)))
                                .count()
                                >= card
                        {
                            pairs.push((i, j, t));
                        }
                    }
                }
            }
            if pairs.is_empty() {
                break;
            }
            for (i, j, t) in pairs {
                if graph[i][j][t].is_none() {
                    continue;
                }
                let node = (i, -(t as i32));
                let pool: Vec<_> = adj[j].iter().copied().filter(|p| *p != node).collect();
                for s in combinations(&pool, card) {
                    let result = self.test(&graph, i, j, t, &s, parents, phase)?;
                    // Source _pcalg_skeleton compares a None marker with float
                    // here and raises TypeError. Do not fabricate a p-value.
                    if matches!(result, TestResult::Fixed(FixedDecision::Constant)) {
                        return Err(RunError::UndefinedThresholdMarker);
                    }
                    let (v, q, dependent) = result.parts(self.alpha);
                    if let Some(entry) = minima[j].iter_mut().find(|(p, _)| *p == node) {
                        entry.1 = entry.1.min(v.abs());
                    }
                    if q >= sp[i][j][t] {
                        sp[i][j][t] = q;
                        sv[i][j][t] = v;
                    }
                    if !dependent {
                        graph[i][j][t] = None;
                        sepsets[i][j][t] = s.clone();
                        if t == 0 {
                            graph[j][i][0] = None;
                            sepsets[j][i][0] = s;
                            sp[j][i][0] = sp[i][j][0];
                        }
                        break;
                    }
                }
            }
            for j in 0..n {
                let mut sorted: Vec<_> = minima[j]
                    .iter()
                    .filter(|(node, _)| node.1 == 0 && graph[node.0][j][0].is_some())
                    .copied()
                    .collect();
                sorted.sort_by(|a, b| b.1.total_cmp(&a.1));
                adj[j] = sorted.into_iter().map(|(p, _)| p).collect();
            }
        }
        for i in 0..n {
            for j in 0..n {
                if let Some((_, m)) = selected[j].iter().find(|(node, _)| *node == (i, 0)) {
                    if (m[0] == b'o' && sp[i][j][0] >= sp[j][i][0]) || m[2] == b'>' {
                        sp[j][i][0] = sp[i][j][0];
                        sv[j][i][0] = sv[i][j][0];
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                p[i][j][0] = sp[i][j][0];
                val[i][j][0] = sv[i][j][0];
            }
        }
        for j in 0..n {
            for &((i, t), m) in &selected[j] {
                if m[0] != b'<' {
                    p[i][j][(-t) as usize] = sp[i][j][(-t) as usize];
                    val[i][j][(-t) as usize] = sv[i][j][(-t) as usize];
                }
            }
        }
        let uncorrected_p = p.clone();
        if self.options.fdr == Fdr::BenjaminiHochberg {
            let mut cells = Vec::new();
            for i in 0..n {
                for j in 0..n {
                    for t in 1..=self.tau {
                        if links[j]
                            .iter()
                            .any(|(node, m)| *node == (i, -(t as i32)) && m[0] != b'<')
                        {
                            cells.push((i, j, t));
                        }
                    }
                }
            }
            let values: Vec<_> = cells.iter().map(|&(i, j, t)| p[i][j][t]).collect();
            for ((i, j, t), q) in cells.into_iter().zip(crate::pcmciplus::bh_values(&values)) {
                p[i][j][t] = q;
            }
        }
        Ok(Skeleton {
            uncorrected_p,
            graph,
            p,
            val,
            sepsets,
        })
    }

    fn orient(
        &self,
        skel: &mut Skeleton,
        parents: &[Vec<Node>],
        phase: &Phase,
    ) -> Result<Vec<(Node, usize, usize)>, RunError> {
        let n = self.classes.len();
        for m in skel.graph.iter_mut().flatten().flatten().flatten() {
            if m[1] == b'?' {
                m[1] = b'-';
            }
        }
        let mut triples = Vec::new();
        for j in 0..n {
            for (k, t) in adjacencies(&skel.graph, j, false) {
                if t == 0 && skel.graph[k][j][0] == Some(*b"o-o") {
                    for (i, lag) in adjacencies(&skel.graph, k, false) {
                        if (i, lag) != (j, 0)
                            && skel.graph[i][j][(-lag) as usize].is_none()
                            && [Some(*b"o-o"), Some(*b"-->")]
                                .contains(&skel.graph[i][k][(-lag) as usize])
                        {
                            triples.push(((i, lag), k, j));
                        }
                    }
                }
            }
        }
        let mut ambiguous = Vec::new();
        let mut colliders = Vec::new();
        for &((i, lag), k, j) in &triples {
            if self.options.collider_rule == ColliderRule::None {
                if !skel.sepsets[i][j][(-lag) as usize].contains(&(k, 0)) {
                    colliders.push(((i, lag), k, j));
                }
                continue;
            }
            let mut subsets = Vec::new();
            let sides = if lag == 0 {
                vec![(j, (i, lag)), (i, (j, 0))]
            } else {
                vec![(j, (i, lag))]
            };
            for (target, exclude) in sides {
                let pool: Vec<_> = adjacencies(&skel.graph, target, true)
                    .into_iter()
                    .filter(|p| p.1 == 0 && *p != exclude)
                    .collect();
                if pool.is_empty() {
                    continue;
                }
                for card in 0..=pool.len() {
                    for s in combinations(&pool, card) {
                        if !subsets.contains(&s) {
                            subsets.push(s);
                        }
                    }
                }
            }
            let mut count = 0;
            let mut with_k = 0;
            for s in subsets {
                let (_, _, dependent) = self
                    .test(&skel.graph, i, j, (-lag) as usize, &s, parents, phase)?
                    .parts(self.alpha);
                if !dependent {
                    count += 1;
                    if s.contains(&(k, 0)) {
                        with_k += 1;
                    }
                }
            }
            let is_ambiguous = match self.options.collider_rule {
                ColliderRule::Majority => count == 0 || 2 * with_k == count,
                ColliderRule::Conservative => count == 0 || (with_k > 0 && with_k < count),
                ColliderRule::None => unreachable!(),
            };
            if is_ambiguous {
                ambiguous.push(((i, lag), k, j));
            } else {
                let collider = 2 * with_k < count;
                if collider {
                    colliders.push(((i, lag), k, j));
                }
                let keys = if lag == 0 {
                    vec![(i, j, 0), (j, i, 0)]
                } else {
                    vec![(i, j, (-lag) as usize)]
                };
                for (a, b, t) in keys {
                    let set = &mut skel.sepsets[a][b][t];
                    if collider {
                        set.retain(|p| *p != (k, 0));
                    } else if !set.contains(&(k, 0)) {
                        set.push((k, 0));
                    }
                }
            }
        }
        let mut oriented = Vec::new();
        for ((i, t), k, j) in colliders {
            let edges = if t == 0 {
                vec![(j, k), (i, k)]
            } else {
                vec![(j, k)]
            };
            for (a, b) in edges {
                if !oriented.contains(&(a, b)) && !oriented.contains(&(b, a)) {
                    skel.graph[a][b][0] = Some(*b"-->");
                    skel.graph[b][a][0] = Some(*b"<--");
                    oriented.push((a, b));
                }
                if self.options.conflict_resolution && oriented.contains(&(b, a)) {
                    skel.graph[a][b][0] = Some(*b"x-x");
                    skel.graph[b][a][0] = Some(*b"x-x");
                }
            }
        }
        meek_rules_with_conflicts(
            &mut skel.graph,
            &ambiguous,
            self.options.conflict_resolution,
        );
        Ok(ambiguous)
    }
}

/// Default link assumptions, analytic max_corr, fixed alpha, majority colliders,
/// conflict resolution enabled, PC1 max_combinations=1. Other source options are
/// not silently accepted by this entry point.
pub fn run(
    data: &dyn ConditionalIndependence,
    classes: &[NodeClass],
    tau: usize,
    alpha: f64,
) -> Result<Discovery, RunError> {
    run_with_options(data, classes, tau, alpha, Options::default())
}

pub fn run_with_options(
    data: &dyn ConditionalIndependence,
    classes: &[NodeClass],
    tau: usize,
    alpha: f64,
    options: Options,
) -> Result<Discovery, RunError> {
    run_with_history(data, classes, tau, alpha, options, None)
}

pub(super) fn run_with_history(
    data: &dyn ConditionalIndependence,
    classes: &[NodeClass],
    tau: usize,
    alpha: f64,
    options: Options,
    history: Option<&AlphaHistory>,
) -> Result<Discovery, RunError> {
    let n = classes.len();
    if n == 0 || data.variables() != n {
        return Err(RunError::Shape);
    }
    if tau > i32::MAX as usize / 2
        || options.tau_min > tau
        || options.max_combinations == 0
        || !alpha.is_finite()
        || !(0.0..=1.0).contains(&alpha)
    {
        return Err(RunError::Configuration);
    }
    let raw: LinkAssumptions = (0..n)
        .map(|j| {
            (0..n)
                .flat_map(|i| {
                    (options.tau_min..=tau).filter_map(move |t| {
                        if i == j && t == 0 {
                            None
                        } else {
                            Some(((i, -(t as i32)), if t == 0 { *b"o?o" } else { *b"-?>" }))
                        }
                    })
                })
                .collect()
        })
        .collect();
    run_prepared(
        data,
        classes,
        tau,
        alpha,
        options,
        prepare_context_links(classes, &raw),
        history,
    )
}

/// Explicit incoming assumptions bundled with their node classifications.
/// Unlisted edges are absent, matching Tigramite's custom-assumption contract.
pub fn run_with_assumptions(
    data: &dyn ConditionalIndependence,
    nodes: Vec<NodeInput>,
    tau: usize,
    alpha: f64,
    options: Options,
) -> Result<Discovery, RunError> {
    run_assumptions_with_history(data, nodes, tau, alpha, options, None)
}

pub(super) fn run_assumptions_with_history(
    data: &dyn ConditionalIndependence,
    nodes: Vec<NodeInput>,
    tau: usize,
    alpha: f64,
    options: Options,
    history: Option<&AlphaHistory>,
) -> Result<Discovery, RunError> {
    if tau > i32::MAX as usize / 2
        || options.tau_min > tau
        || options.max_combinations == 0
        || !alpha.is_finite()
        || !(0.0..=1.0).contains(&alpha)
    {
        return Err(RunError::Configuration);
    }
    let context = ContextSearch::new(nodes, tau).map_err(RunError::Constraints)?;
    let prepared = context.prepared;
    if data.variables() != prepared.classes.len() {
        return Err(RunError::Shape);
    }
    if prepared
        .links
        .iter()
        .flatten()
        .any(|(node, _)| (node.1.unsigned_abs() as usize) < options.tau_min)
    {
        return Err(RunError::Configuration);
    }
    run_prepared(
        data,
        &prepared.classes,
        tau,
        alpha,
        options,
        prepared.links,
        history,
    )
}

fn run_prepared(
    data: &dyn ConditionalIndependence,
    classes: &[NodeClass],
    tau: usize,
    alpha: f64,
    options: Options,
    prepared: LinkAssumptions,
    history: Option<&AlphaHistory>,
) -> Result<Discovery, RunError> {
    let n = classes.len();
    // PC1 removes contemporaneous entries before source normalization.
    let lagged_links: LinkAssumptions = prepared
        .iter()
        .map(|parents| {
            parents
                .iter()
                .copied()
                .filter(|(node, _)| node.1 < 0)
                .collect()
        })
        .collect();
    let lagged_links = normalize(&lagged_links)?;
    let search = Search {
        data,
        classes,
        tau,
        alpha,
        options,
    };
    let mut p = vec![vec![vec![1.0; tau + 1]; n]; n];
    let mut val = vec![vec![vec![0.0; tau + 1]; n]; n];
    let mut lagged = Vec::new();
    for j in 0..n {
        let candidates = lagged_links[j]
            .iter()
            .filter(|(node, m)| node.1 < 0 && m[0] != b'<')
            .map(|(node, _)| *node)
            .collect();
        let fit = pc_stable_decisions(
            candidates,
            options
                .max_conds_dim
                .unwrap_or(n * (tau + 1 - options.tau_min.max(1))),
            options.max_combinations,
            |node, z| -> Result<_, RunError> {
                if lagged_links[j]
                    .iter()
                    .any(|(p, mark)| *p == node && *mark == *b"-->")
                {
                    Ok((1.0, 0.0, true))
                } else {
                    Ok(data
                        .decision(&[node], &[(j, 0)], z, tau, alpha)?
                        .parts(alpha))
                }
            },
        )?;
        for ((i, t), q, v) in fit.pval_max {
            p[i][j][(-t) as usize] = q;
            val[i][j][(-t) as usize] = v;
        }
        lagged.push(fit.parents);
    }
    let context = search.skeleton(
        &without_dummy_links(classes, &prepared),
        &lagged,
        &Phase::Context,
        p,
        val,
    )?;
    let sources: Vec<_> = [NodeClass::SpaceContext, NodeClass::TimeContext]
        .into_iter()
        .flat_map(|c| (0..n).filter(move |&i| classes[i] == c))
        .collect();
    let mut contexts = vec![Vec::new(); n];
    for j in 0..n {
        if !classes[j].dummy() {
            for &i in &sources {
                for t in 0..=tau {
                    if context.graph[i][j][t].is_some_and(|m| m[0] != b'<') {
                        contexts[j].push((i, -(t as i32)));
                    }
                }
            }
        }
    }
    for parents in &mut lagged {
        parents.retain(|(i, _)| classes[*i] != NodeClass::TimeContext);
    }
    let mut dummies = vec![Vec::new(); n];
    let dummy = if classes.iter().any(|c| c.dummy()) {
        let parents: Vec<_> = (0..n)
            .map(|j| dedup(&[contexts[j].clone(), lagged[j].clone()].concat()))
            .collect();
        let mut d = search.skeleton(
            &with_context_parents(classes, &prepared, &contexts),
            &parents,
            &Phase::Dummy {
                contexts: &contexts,
            },
            context.uncorrected_p.clone(),
            context.val.clone(),
        )?;
        for j in 0..n {
            if classes[j] == NodeClass::System {
                for t in 0..=tau {
                    for c in [NodeClass::TimeDummy, NodeClass::SpaceDummy] {
                        for i in 0..n {
                            if classes[i] == c && d.graph[i][j][t].is_some_and(|m| m[0] != b'<') {
                                dummies[j].push((i, t as i32));
                            }
                        }
                    }
                    for &i in &sources {
                        d.p[i][j][t] = context.p[i][j][t];
                        d.p[j][i][t] = context.p[j][i][t];
                        if options.fdr == Fdr::None {
                            d.uncorrected_p[i][j][t] = context.p[i][j][t];
                            d.uncorrected_p[j][i][t] = context.p[j][i][t];
                        }
                        d.val[i][j][t] = context.val[i][j][t];
                        d.val[j][i][t] = context.val[j][i][t];
                    }
                }
            }
        }
        d
    } else {
        context
    };
    let parents: Vec<_> = (0..n)
        .map(|j| {
            if classes[j] == NodeClass::System {
                dedup(&[lagged[j].clone(), contexts[j].clone(), dummies[j].clone()].concat())
            } else {
                Vec::new()
            }
        })
        .collect();
    let phase = Phase::System {
        contexts: history.map_or(&contexts, |h| &h.contexts),
        dummies: history.map_or(&dummies, |h| &h.dummies),
    };
    let mut final_skeleton = search.skeleton(
        &system_links(classes, &prepared, &contexts, &dummies),
        &parents,
        &phase,
        dummy.uncorrected_p.clone(),
        dummy.val.clone(),
    )?;
    let ambiguous = search.orient(&mut final_skeleton, &parents, &phase)?;
    for i in 0..n {
        if classes[i] != NodeClass::System {
            for j in 0..n {
                for t in 0..=tau {
                    final_skeleton.p[i][j][t] = dummy.p[i][j][t];
                    final_skeleton.p[j][i][t] = dummy.p[j][i][t];
                    final_skeleton.val[i][j][t] = dummy.val[i][j][t];
                    final_skeleton.val[j][i][t] = dummy.val[j][i][t];
                }
            }
        }
    }
    let graph = final_skeleton
        .graph
        .into_iter()
        .map(|targets| {
            targets
                .into_iter()
                .map(|lags| {
                    lags.into_iter()
                        .map(|m| {
                            m.map_or_else(String::new, |m| String::from_utf8_lossy(&m).into_owned())
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    Ok(Discovery {
        graph,
        p_matrix: final_skeleton.p,
        val_matrix: final_skeleton.val,
        sepsets: final_skeleton.sepsets,
        ambiguous_triples: ambiguous,
        lagged_parents: lagged,
        context_parents: history.map_or(contexts, |h| h.contexts.clone()),
        dummy_parents: history.map_or(dummies, |h| h.dummies.clone()),
    })
}
