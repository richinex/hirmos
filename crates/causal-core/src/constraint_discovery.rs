//! Shared cross-sectional constraint-discovery primitives ported from causal-learn.
//!
//! The source of truth is causal-learn commit `9de1d886b7ab45868819aec7132f79378990f0b6`:
//! `utils/FAS.py`, `utils/PCUtils/SkeletonDiscovery.py`, `utils/cit.py`,
//! `utils/PCUtils/BackgroundKnowledge.py`, and the Tetrad-style graph classes.

use crate::kci::{causal_learn_kernel_conditional_independence, KciError};
use nalgebra::DMatrix;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt;

pub const NONE: i8 = 0;
pub const TAIL: i8 = -1;
pub const ARROW: i8 = 1;
pub const CIRCLE: i8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum Endpoint {
    Tail = TAIL,
    Arrow = ARROW,
    Circle = CIRCLE,
}

impl Endpoint {
    pub fn from_code(code: i8) -> Option<Self> {
        match code {
            TAIL => Some(Self::Tail),
            ARROW => Some(Self::Arrow),
            CIRCLE => Some(Self::Circle),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkedEdge {
    pub left: usize,
    pub right: usize,
    pub left_endpoint: Endpoint,
    pub right_endpoint: Endpoint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointGraph {
    pub node_names: Vec<String>,
    /// causal-learn/Tetrad convention: `endpoints[i][j]` is the endpoint at `i`.
    pub endpoints: Vec<Vec<i8>>,
}

impl EndpointGraph {
    pub fn complete(node_names: Vec<String>, endpoint: Endpoint) -> Self {
        let nodes = node_names.len();
        let mut endpoints = vec![vec![endpoint as i8; nodes]; nodes];
        for (node, row) in endpoints.iter_mut().enumerate() {
            row[node] = NONE;
        }
        Self {
            node_names,
            endpoints,
        }
    }

    pub fn nodes(&self) -> usize {
        self.endpoints.len()
    }

    pub fn adjacent(&self, left: usize, right: usize) -> bool {
        self.endpoints[left][right] != NONE && self.endpoints[right][left] != NONE
    }

    pub fn neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.nodes())
            .filter(|&other| other != node && self.adjacent(node, other))
            .collect()
    }

    pub fn max_degree(&self) -> usize {
        (0..self.nodes())
            .map(|node| self.neighbors(node).len())
            .max()
            .unwrap_or(0)
    }

    pub fn remove(&mut self, left: usize, right: usize) {
        self.endpoints[left][right] = NONE;
        self.endpoints[right][left] = NONE;
    }

    pub fn set_edge(&mut self, left: usize, right: usize, at_left: Endpoint, at_right: Endpoint) {
        self.endpoints[left][right] = at_left as i8;
        self.endpoints[right][left] = at_right as i8;
    }

    pub fn set_endpoint_at(&mut self, node: usize, other: usize, endpoint: Endpoint) {
        debug_assert!(self.adjacent(node, other));
        self.endpoints[node][other] = endpoint as i8;
    }

    /// Matches causal-learn `get_endpoint(from, to)`: the endpoint proximal to `to`.
    pub fn endpoint(&self, from: usize, to: usize) -> Option<Endpoint> {
        Endpoint::from_code(self.endpoints[to][from])
    }

    pub fn endpoint_at(&self, node: usize, other: usize) -> Option<Endpoint> {
        Endpoint::from_code(self.endpoints[node][other])
    }

    pub fn is_undirected(&self, left: usize, right: usize) -> bool {
        self.endpoint_at(left, right) == Some(Endpoint::Tail)
            && self.endpoint_at(right, left) == Some(Endpoint::Tail)
    }

    pub fn is_directed(&self, cause: usize, effect: usize) -> bool {
        self.endpoint_at(cause, effect) == Some(Endpoint::Tail)
            && self.endpoint_at(effect, cause) == Some(Endpoint::Arrow)
    }

    pub fn is_parent(&self, parent: usize, child: usize) -> bool {
        self.is_directed(parent, child)
    }

    pub fn is_def_collider(&self, left: usize, middle: usize, right: usize) -> bool {
        self.endpoint(left, middle) == Some(Endpoint::Arrow)
            && self.endpoint(right, middle) == Some(Endpoint::Arrow)
    }

    pub fn nodes_into(&self, node: usize, endpoint: Endpoint) -> Vec<usize> {
        (0..self.nodes())
            .filter(|&other| other != node && self.endpoint_at(node, other) == Some(endpoint))
            .collect()
    }

    pub fn nodes_out_of(&self, node: usize, endpoint: Endpoint) -> Vec<usize> {
        (0..self.nodes())
            .filter(|&other| other != node && self.endpoint_at(other, node) == Some(endpoint))
            .collect()
    }

    pub fn directed_path(&self, start: usize, destination: usize) -> bool {
        let mut stack = vec![start];
        let mut visited = vec![false; self.nodes()];
        visited[start] = true;
        while let Some(node) = stack.pop() {
            for next in 0..self.nodes() {
                if self.is_directed(node, next) && !visited[next] {
                    if next == destination {
                        return true;
                    }
                    visited[next] = true;
                    stack.push(next);
                }
            }
        }
        false
    }

    pub fn edges(&self) -> Vec<MarkedEdge> {
        let mut edges = Vec::new();
        for left in 0..self.nodes() {
            for right in left + 1..self.nodes() {
                if self.adjacent(left, right) {
                    edges.push(MarkedEdge {
                        left,
                        right,
                        left_endpoint: self.endpoint_at(left, right).expect("adjacent endpoint"),
                        right_endpoint: self.endpoint_at(right, left).expect("adjacent endpoint"),
                    });
                }
            }
        }
        edges
    }

    pub(crate) fn reorient_all(&mut self, endpoint: Endpoint) {
        for left in 0..self.nodes() {
            for right in left + 1..self.nodes() {
                if self.adjacent(left, right) {
                    self.set_edge(left, right, endpoint, endpoint);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BackgroundKnowledge {
    forbidden: BTreeSet<(usize, usize)>,
    required: BTreeSet<(usize, usize)>,
    forbidden_patterns: Vec<(Regex, Regex)>,
    required_patterns: Vec<(Regex, Regex)>,
    tiers: BTreeMap<usize, usize>,
    forbidden_within_tiers: BTreeSet<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackgroundKnowledgeError {
    InvalidPattern(String),
    UnknownNode(usize),
}

impl fmt::Display for BackgroundKnowledgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPattern(pattern) => {
                write!(formatter, "invalid background-knowledge pattern: {pattern}")
            }
            Self::UnknownNode(node) => {
                write!(formatter, "background knowledge names unknown node {node}")
            }
        }
    }
}

impl std::error::Error for BackgroundKnowledgeError {}

impl BackgroundKnowledge {
    pub fn forbid(&mut self, from: usize, to: usize) -> &mut Self {
        self.forbidden.insert((from, to));
        self
    }

    pub fn require(&mut self, from: usize, to: usize) -> &mut Self {
        self.required.insert((from, to));
        self
    }

    pub fn forbid_pattern(
        &mut self,
        from: &str,
        to: &str,
    ) -> Result<&mut Self, BackgroundKnowledgeError> {
        let from = Regex::new(from)
            .map_err(|_| BackgroundKnowledgeError::InvalidPattern(from.to_owned()))?;
        let to =
            Regex::new(to).map_err(|_| BackgroundKnowledgeError::InvalidPattern(to.to_owned()))?;
        self.forbidden_patterns.push((from, to));
        Ok(self)
    }

    pub fn require_pattern(
        &mut self,
        from: &str,
        to: &str,
    ) -> Result<&mut Self, BackgroundKnowledgeError> {
        let from = Regex::new(from)
            .map_err(|_| BackgroundKnowledgeError::InvalidPattern(from.to_owned()))?;
        let to =
            Regex::new(to).map_err(|_| BackgroundKnowledgeError::InvalidPattern(to.to_owned()))?;
        self.required_patterns.push((from, to));
        Ok(self)
    }

    pub fn add_to_tier(&mut self, node: usize, tier: usize) -> &mut Self {
        self.tiers.insert(node, tier);
        self
    }

    pub fn forbid_within_tier(&mut self, tier: usize) -> &mut Self {
        self.forbidden_within_tiers.insert(tier);
        self
    }

    pub fn allow_within_tier(&mut self, tier: usize) -> &mut Self {
        self.forbidden_within_tiers.remove(&tier);
        self
    }

    pub fn is_forbidden(&self, from: usize, to: usize, names: &[String]) -> bool {
        if self.forbidden.contains(&(from, to)) {
            return true;
        }
        if self.forbidden_patterns.iter().any(|(left, right)| {
            left.find(&names[from]).is_some_and(|m| m.start() == 0)
                && right.find(&names[to]).is_some_and(|m| m.start() == 0)
        }) {
            return true;
        }
        match (self.tiers.get(&from), self.tiers.get(&to)) {
            (Some(&from_tier), Some(&to_tier)) => {
                from_tier > to_tier
                    || (from_tier == to_tier && self.forbidden_within_tiers.contains(&from_tier))
            }
            _ => false,
        }
    }

    pub fn is_required(&self, from: usize, to: usize, names: &[String]) -> bool {
        self.required.contains(&(from, to))
            || self.required_patterns.iter().any(|(left, right)| {
                left.find(&names[from]).is_some_and(|m| m.start() == 0)
                    && right.find(&names[to]).is_some_and(|m| m.start() == 0)
            })
    }

    pub(crate) fn has_explicit_forbidden(&self) -> bool {
        !self.forbidden.is_empty()
    }
    pub(crate) fn has_explicit_required(&self) -> bool {
        !self.required.is_empty()
    }
    pub(crate) fn has_tiers(&self) -> bool {
        !self.tiers.is_empty()
    }

    pub(crate) fn validate(&self, nodes: usize) -> Result<(), BackgroundKnowledgeError> {
        for &(from, to) in self.forbidden.iter().chain(self.required.iter()) {
            if from >= nodes {
                return Err(BackgroundKnowledgeError::UnknownNode(from));
            }
            if to >= nodes {
                return Err(BackgroundKnowledgeError::UnknownNode(to));
            }
        }
        for &node in self.tiers.keys() {
            if node >= nodes {
                return Err(BackgroundKnowledgeError::UnknownNode(node));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossSectionalCi {
    FisherZ,
    /// causal-learn's default Gaussian KCI with empirical widths and gamma approximation.
    Kci,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CiRecord {
    pub x: usize,
    pub y: usize,
    pub conditions: Vec<usize>,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintDiscoveryError {
    EmptyData,
    NameCount { names: usize, columns: usize },
    InvalidAlpha,
    NonFiniteData,
    SingularCorrelation { variables: Vec<usize> },
    InsufficientSamples { samples: usize, conditions: usize },
    BackgroundKnowledge(BackgroundKnowledgeError),
    Kci(KciError),
}

impl fmt::Display for ConstraintDiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyData => formatter
                .write_str("constraint discovery requires at least one row and two columns"),
            Self::NameCount { names, columns } => {
                write!(formatter, "received {names} names for {columns} columns")
            }
            Self::InvalidAlpha => {
                formatter.write_str("alpha must be strictly between zero and one")
            }
            Self::NonFiniteData => {
                formatter.write_str("constraint discovery requires finite values")
            }
            Self::SingularCorrelation { variables } => write!(
                formatter,
                "the correlation matrix is singular for variables {variables:?}"
            ),
            Self::InsufficientSamples {
                samples,
                conditions,
            } => write!(
                formatter,
                "{samples} samples are insufficient for {conditions} conditioning variables"
            ),
            Self::BackgroundKnowledge(problem) => problem.fmt(formatter),
            Self::Kci(problem) => write!(formatter, "KCI failed: {problem:?}"),
        }
    }
}

impl std::error::Error for ConstraintDiscoveryError {}

impl From<BackgroundKnowledgeError> for ConstraintDiscoveryError {
    fn from(value: BackgroundKnowledgeError) -> Self {
        Self::BackgroundKnowledge(value)
    }
}

impl From<KciError> for ConstraintDiscoveryError {
    fn from(value: KciError) -> Self {
        Self::Kci(value)
    }
}

pub(crate) struct CiEngine<'a> {
    data: &'a DMatrix<f64>,
    kind: CrossSectionalCi,
    correlations: Option<DMatrix<f64>>,
    cache: HashMap<(usize, usize, Vec<usize>), f64>,
    pub records: Vec<CiRecord>,
}

impl<'a> CiEngine<'a> {
    pub(crate) fn new(
        data: &'a DMatrix<f64>,
        kind: CrossSectionalCi,
    ) -> Result<Self, ConstraintDiscoveryError> {
        if data.nrows() == 0 || data.ncols() < 2 {
            return Err(ConstraintDiscoveryError::EmptyData);
        }
        if data.iter().any(|value| !value.is_finite()) {
            return Err(ConstraintDiscoveryError::NonFiniteData);
        }
        let correlations = (kind == CrossSectionalCi::FisherZ).then(|| correlation_matrix(data));
        Ok(Self {
            data,
            kind,
            correlations,
            cache: HashMap::new(),
            records: Vec::new(),
        })
    }

    pub(crate) fn p_value(
        &mut self,
        x: usize,
        y: usize,
        conditions: &[usize],
    ) -> Result<f64, ConstraintDiscoveryError> {
        let mut conditions = conditions.to_vec();
        conditions.sort_unstable();
        conditions.dedup();
        let (left, right) = if x <= y { (x, y) } else { (y, x) };
        let key = (left, right, conditions.clone());
        if let Some(&p_value) = self.cache.get(&key) {
            return Ok(p_value);
        }
        let p_value = match self.kind {
            CrossSectionalCi::FisherZ => self.fisher_z(left, right, &conditions)?,
            CrossSectionalCi::Kci => self.kci(left, right, &conditions)?,
        };
        self.cache.insert(key, p_value);
        self.records.push(CiRecord {
            x: left,
            y: right,
            conditions,
            p_value,
        });
        Ok(p_value)
    }

    fn fisher_z(
        &self,
        x: usize,
        y: usize,
        conditions: &[usize],
    ) -> Result<f64, ConstraintDiscoveryError> {
        if self.data.nrows() <= conditions.len() + 3 {
            return Err(ConstraintDiscoveryError::InsufficientSamples {
                samples: self.data.nrows(),
                conditions: conditions.len(),
            });
        }
        let mut variables = vec![x, y];
        variables.extend_from_slice(conditions);
        let correlations = self.correlations.as_ref().expect("Fisher-Z correlations");
        let sub = DMatrix::from_fn(variables.len(), variables.len(), |row, column| {
            correlations[(variables[row], variables[column])]
        });
        let inverse =
            sub.try_inverse()
                .ok_or_else(|| ConstraintDiscoveryError::SingularCorrelation {
                    variables: variables.clone(),
                })?;
        let mut r = -inverse[(0, 1)] / (inverse[(0, 0)] * inverse[(1, 1)]).abs().sqrt();
        if r.abs() >= 1.0 {
            r = (1.0 - f64::EPSILON).copysign(r);
        }
        let z = 0.5 * ((1.0 + r) / (1.0 - r)).ln();
        let statistic = ((self.data.nrows() - conditions.len() - 3) as f64).sqrt() * z.abs();
        Ok((2.0 * (1.0 - spec_math::cephes64::ndtr(statistic.abs()))).clamp(0.0, 1.0))
    }

    fn kci(
        &self,
        x: usize,
        y: usize,
        conditions: &[usize],
    ) -> Result<f64, ConstraintDiscoveryError> {
        let x_values = DMatrix::from_fn(self.data.nrows(), 1, |row, _| self.data[(row, x)]);
        let y_values = DMatrix::from_fn(self.data.nrows(), 1, |row, _| self.data[(row, y)]);
        let z_values = (!conditions.is_empty()).then(|| {
            DMatrix::from_fn(self.data.nrows(), conditions.len(), |row, column| {
                self.data[(row, conditions[column])]
            })
        });
        Ok(
            causal_learn_kernel_conditional_independence(&x_values, &y_values, z_values.as_ref())?
                .p_value,
        )
    }
}

pub fn cross_sectional_ci_test(
    data: &DMatrix<f64>,
    kind: CrossSectionalCi,
    x: usize,
    y: usize,
    conditions: &[usize],
) -> Result<f64, ConstraintDiscoveryError> {
    CiEngine::new(data, kind)?.p_value(x, y, conditions)
}

fn correlation_matrix(data: &DMatrix<f64>) -> DMatrix<f64> {
    let means: Vec<f64> = (0..data.ncols())
        .map(|column| data.column(column).sum() / data.nrows() as f64)
        .collect();
    let scales: Vec<f64> = (0..data.ncols())
        .map(|column| {
            (0..data.nrows())
                .map(|row| {
                    let delta = data[(row, column)] - means[column];
                    delta * delta
                })
                .sum::<f64>()
                .sqrt()
        })
        .collect();
    DMatrix::from_fn(data.ncols(), data.ncols(), |left, right| {
        let numerator = (0..data.nrows())
            .map(|row| (data[(row, left)] - means[left]) * (data[(row, right)] - means[right]))
            .sum::<f64>();
        numerator / (scales[left] * scales[right])
    })
}

pub(crate) type SeparatingSets = BTreeMap<(usize, usize), BTreeSet<usize>>;

pub(crate) fn combinations(values: &[usize], size: usize) -> Vec<Vec<usize>> {
    fn visit(
        values: &[usize],
        size: usize,
        start: usize,
        current: &mut Vec<usize>,
        output: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == size {
            output.push(current.clone());
            return;
        }
        let needed = size - current.len();
        if values.len().saturating_sub(start) < needed {
            return;
        }
        for index in start..values.len() {
            current.push(values[index]);
            visit(values, size, index + 1, current, output);
            current.pop();
        }
    }
    let mut output = Vec::new();
    visit(values, size, 0, &mut Vec::new(), &mut output);
    output
}

pub(crate) fn stable_adjacency_search(
    names: Vec<String>,
    alpha: f64,
    max_depth: Option<usize>,
    knowledge: &BackgroundKnowledge,
    ci: &mut CiEngine<'_>,
) -> Result<(EndpointGraph, SeparatingSets), ConstraintDiscoveryError> {
    if !(0.0 < alpha && alpha < 1.0) {
        return Err(ConstraintDiscoveryError::InvalidAlpha);
    }
    if names.len() != ci.data.ncols() {
        return Err(ConstraintDiscoveryError::NameCount {
            names: names.len(),
            columns: ci.data.ncols(),
        });
    }
    knowledge.validate(names.len())?;
    let mut graph = EndpointGraph::complete(names, Endpoint::Tail);
    let mut separating_sets = SeparatingSets::new();
    let mut depth = 0usize;
    while graph.max_degree().saturating_sub(1) >= depth
        && max_depth.is_none_or(|maximum| depth <= maximum)
    {
        let mut removals = BTreeSet::new();
        let mut separated_at_depth: BTreeMap<(usize, usize), BTreeSet<usize>> = BTreeMap::new();
        for x in 0..graph.nodes() {
            let neighbors = graph.neighbors(x);
            for &y in &neighbors {
                if knowledge.is_forbidden(x, y, &graph.node_names)
                    && knowledge.is_forbidden(y, x, &graph.node_names)
                {
                    removals.insert((x.min(y), x.max(y)));
                }
                let candidates: Vec<usize> = neighbors
                    .iter()
                    .copied()
                    .filter(|&node| node != y)
                    .collect();
                for conditions in combinations(&candidates, depth) {
                    if ci.p_value(x, y, &conditions)? > alpha {
                        removals.insert((x.min(y), x.max(y)));
                        separated_at_depth
                            .entry((x, y))
                            .or_default()
                            .extend(conditions);
                    }
                }
            }
        }
        for (left, right) in removals {
            graph.remove(left, right);
            let mut union = BTreeSet::new();
            union.extend(
                separated_at_depth
                    .remove(&(left, right))
                    .unwrap_or_default(),
            );
            union.extend(
                separated_at_depth
                    .remove(&(right, left))
                    .unwrap_or_default(),
            );
            separating_sets.insert((left, right), union.clone());
            separating_sets.insert((right, left), union);
        }
        depth += 1;
    }
    Ok((graph, separating_sets))
}

pub(crate) fn orient_background(graph: &mut EndpointGraph, knowledge: &BackgroundKnowledge) {
    for left in 0..graph.nodes() {
        for right in left + 1..graph.nodes() {
            if !graph.is_undirected(left, right) {
                continue;
            }
            if knowledge.is_forbidden(right, left, &graph.node_names)
                || knowledge.is_required(left, right, &graph.node_names)
            {
                graph.set_edge(left, right, Endpoint::Tail, Endpoint::Arrow);
            } else if knowledge.is_forbidden(left, right, &graph.node_names)
                || knowledge.is_required(right, left, &graph.node_names)
            {
                graph.set_edge(left, right, Endpoint::Arrow, Endpoint::Tail);
            }
        }
    }
}
