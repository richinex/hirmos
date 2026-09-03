//! CD-NOTS discovery ported from causal-ts 0.26.0 at
//! `cd20a0d54054f7e852e584c9a0db31191ad56477`.

use crate::causal_ts_preparation::{
    causal_ts_context, causal_ts_parcorr, causal_ts_precision_parcorr, causal_ts_var_em,
    cdnots_embed, CausalTsContextPreset, CausalTsPreparationError, CdnotsEmbedded,
    CdnotsMissingPolicy,
};
use nalgebra::DMatrix;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

const NONE: i8 = 0;
const TAIL: i8 = -1;
const ARROW: i8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColliderPriority {
    Overwrite,
    MarkConflict,
    KeepFirst,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CdnotsConfiguration {
    pub max_lag: usize,
    pub alpha: f64,
    pub max_degree: Option<usize>,
    pub max_combinations: Option<usize>,
    pub priority: ColliderPriority,
    pub missing: CdnotsMissingPolicy,
    pub context: Option<CausalTsContextPreset>,
    pub orient_margin: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CdnotsPlusConfiguration {
    pub max_lag: usize,
    pub alpha: f64,
    pub max_degree: Option<usize>,
    pub max_conds_y: Option<usize>,
    pub max_conds_x: Option<usize>,
    pub priority: ColliderPriority,
    pub missing: CdnotsMissingPolicy,
    pub context: Option<CausalTsContextPreset>,
    pub legacy_mci_conditions: bool,
    pub orient_margin: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdnotsPValue {
    pub first: usize,
    pub second: usize,
    pub conditions: Vec<usize>,
    pub p_value: f64,
    pub statistic: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdnotsResult {
    /// causal-learn endpoint matrix retained for parity inspection.
    pub graph: Vec<Vec<i8>>,
    /// `[cause][effect][lag]`, matching causal-ts `cg_tig`.
    pub time_graph: Vec<Vec<Vec<u8>>>,
    pub pvalue_matrix: Vec<Vec<Vec<Option<f64>>>>,
    pub pvalues: Vec<CdnotsPValue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CdnotsError {
    InvalidAlpha,
    Preparation(CausalTsPreparationError),
}

impl fmt::Display for CdnotsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAlpha => formatter.write_str("CD-NOTS alpha must be between zero and one"),
            Self::Preparation(problem) => problem.fmt(formatter),
        }
    }
}

impl std::error::Error for CdnotsError {}

impl From<CausalTsPreparationError> for CdnotsError {
    fn from(value: CausalTsPreparationError) -> Self {
        Self::Preparation(value)
    }
}

#[derive(Clone)]
struct Graph {
    endpoints: Vec<Vec<i8>>,
    separating_sets: Vec<Vec<Vec<Vec<usize>>>>,
}

impl Graph {
    fn complete(nodes: usize) -> Self {
        let mut endpoints = vec![vec![TAIL; nodes]; nodes];
        for (node, row) in endpoints.iter_mut().enumerate() {
            row[node] = NONE;
        }
        Self {
            endpoints,
            separating_sets: vec![vec![Vec::new(); nodes]; nodes],
        }
    }

    fn nodes(&self) -> usize {
        self.endpoints.len()
    }

    fn adjacent(&self, left: usize, right: usize) -> bool {
        self.endpoints[left][right] != NONE || self.endpoints[right][left] != NONE
    }

    fn neighbors(&self, node: usize) -> Vec<usize> {
        (0..self.nodes())
            .filter(|&other| other != node && self.adjacent(node, other))
            .collect()
    }

    fn max_degree(&self) -> usize {
        (0..self.nodes())
            .map(|node| self.neighbors(node).len())
            .max()
            .unwrap_or(0)
    }

    fn remove(&mut self, left: usize, right: usize) {
        self.endpoints[left][right] = NONE;
        self.endpoints[right][left] = NONE;
    }

    fn undirected(&self, left: usize, right: usize) -> bool {
        self.endpoints[left][right] == TAIL && self.endpoints[right][left] == TAIL
    }

    fn directed(&self, cause: usize, effect: usize) -> bool {
        self.endpoints[cause][effect] == TAIL && self.endpoints[effect][cause] == ARROW
    }

    fn set_directed(&mut self, cause: usize, effect: usize) {
        self.endpoints[cause][effect] = TAIL;
        self.endpoints[effect][cause] = ARROW;
    }

    fn set_bidirected(&mut self, left: usize, right: usize) {
        self.endpoints[left][right] = ARROW;
        self.endpoints[right][left] = ARROW;
    }

    fn directed_path(&self, start: usize, destination: usize) -> bool {
        let mut queue = VecDeque::from([start]);
        let mut visited = vec![false; self.nodes()];
        visited[start] = true;
        while let Some(node) = queue.pop_front() {
            for next in 0..self.nodes() {
                if self.directed(node, next) && !visited[next] {
                    if next == destination {
                        return true;
                    }
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        false
    }
}

fn combinations(values: &[usize], size: usize, limit: Option<usize>) -> Vec<Vec<usize>> {
    fn visit(
        values: &[usize],
        size: usize,
        start: usize,
        current: &mut Vec<usize>,
        output: &mut Vec<Vec<usize>>,
        limit: Option<usize>,
    ) {
        if limit.is_some_and(|maximum| output.len() >= maximum) {
            return;
        }
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
            visit(values, size, index + 1, current, output, limit);
            current.pop();
            if limit.is_some_and(|maximum| output.len() >= maximum) {
                break;
            }
        }
    }

    let mut output = Vec::new();
    visit(values, size, 0, &mut Vec::new(), &mut output, limit);
    output
}

fn remove_stationary_edge(
    graph: &mut Graph,
    current_variables: usize,
    max_lag: usize,
    x: usize,
    y: usize,
) {
    graph.remove(x, y);
    let y_step = y / current_variables;
    for shift in 1..=max_lag.saturating_sub(y_step) {
        graph.remove(x + shift * current_variables, y + shift * current_variables);
    }
}

fn remove_shifted_edge(
    graph: &mut Graph,
    current_variables: usize,
    max_lag: usize,
    left: usize,
    right: usize,
) {
    graph.remove(left, right);
    let maximum_step = (left / current_variables).max(right / current_variables);
    for shift in 1..=max_lag.saturating_sub(maximum_step) {
        let shifted_left = left + shift * current_variables;
        let shifted_right = right + shift * current_variables;
        if shifted_left < graph.nodes() && shifted_right < graph.nodes() {
            graph.remove(shifted_left, shifted_right);
        }
    }
}

fn constrained_complete_graph(
    current_variables: usize,
    max_lag: usize,
    context_columns: usize,
) -> Graph {
    let mut graph = Graph::complete(current_variables * (max_lag + 1));
    if context_columns == 0 {
        return graph;
    }

    for left_lag in 0..=max_lag {
        for right_lag in 0..=max_lag {
            if left_lag == right_lag {
                continue;
            }
            for context in 0..context_columns {
                let context_node = (left_lag + 1) * current_variables - context_columns + context;
                for variable in right_lag * current_variables
                    ..(right_lag + 1) * current_variables - context_columns
                {
                    graph.remove(context_node, variable);
                }
            }
        }
    }

    let contexts: Vec<usize> = (0..=max_lag)
        .flat_map(|lag| {
            (0..context_columns)
                .map(move |context| (lag + 1) * current_variables - context_columns + context)
        })
        .collect();
    for left in 0..contexts.len() {
        for right in left + 1..contexts.len() {
            graph.remove(contexts[left], contexts[right]);
        }
    }
    graph
}

fn sort_neighbors(
    neighbors: &mut [usize],
    node: usize,
    strengths: &BTreeMap<(usize, usize), f64>,
    limited: bool,
) {
    if !limited || strengths.is_empty() {
        return;
    }
    neighbors.sort_by(|left, right| {
        let left_key = (node.min(*left), node.max(*left));
        let right_key = (node.min(*right), node.max(*right));
        strengths
            .get(&right_key)
            .copied()
            .unwrap_or(0.0)
            .total_cmp(&strengths.get(&left_key).copied().unwrap_or(0.0))
    });
}

fn update_strength(
    strengths: &mut BTreeMap<(usize, usize), f64>,
    x: usize,
    y: usize,
    statistic: f64,
) {
    let strength = if statistic == 0.0 {
        0.0
    } else {
        statistic.abs()
    };
    let key = (x.min(y), x.max(y));
    strengths
        .entry(key)
        .and_modify(|current| *current = current.min(strength))
        .or_insert(strength);
}

#[derive(Clone)]
struct PairTest {
    conditions: Vec<usize>,
    p_value: f64,
    statistic: f64,
}

fn test_pair(
    embedded: &CdnotsEmbedded,
    x: usize,
    y: usize,
    conditions: &[usize],
    minimum_samples: usize,
) -> Result<PairTest, CdnotsError> {
    let result = causal_ts_precision_parcorr(embedded, x, y, conditions, minimum_samples)?;
    Ok(PairTest {
        conditions: conditions.to_vec(),
        p_value: result.p_value,
        statistic: result.statistic,
    })
}

fn test_pair_ols(
    embedded: &CdnotsEmbedded,
    x: usize,
    y: usize,
    conditions: &[usize],
    minimum_samples: usize,
) -> Result<PairTest, CdnotsError> {
    let result = causal_ts_parcorr(embedded, x, y, conditions, minimum_samples)?;
    Ok(PairTest {
        conditions: conditions.to_vec(),
        p_value: result.p_value,
        statistic: result.statistic,
    })
}

fn stable_skeleton(
    mut graph: Graph,
    embedded: &CdnotsEmbedded,
    configuration: CdnotsConfiguration,
    minimum_samples: usize,
) -> Result<(Graph, Vec<CdnotsPValue>), CdnotsError> {
    let current_variables = embedded.variables;
    let mut depth = 0;
    let mut pvalues = Vec::new();
    let mut strengths = BTreeMap::new();
    let has_missing = embedded.values.iter().flatten().any(Option::is_none);

    while graph.max_degree() > depth {
        if configuration
            .max_degree
            .is_some_and(|maximum| depth > maximum)
        {
            break;
        }

        let snapshot = graph.clone();
        if has_missing {
            let mut removals = BTreeSet::new();
            for x in 0..current_variables {
                let neighbors_x = snapshot.neighbors(x);
                for &y in &neighbors_x {
                    let mut separating = BTreeSet::new();
                    let mut from_x: Vec<usize> = neighbors_x
                        .iter()
                        .copied()
                        .filter(|&neighbor| neighbor != y)
                        .collect();
                    sort_neighbors(
                        &mut from_x,
                        x,
                        &strengths,
                        configuration.max_combinations.is_some(),
                    );
                    for conditions in combinations(&from_x, depth, configuration.max_combinations) {
                        let test = test_pair(embedded, x, y, &conditions, minimum_samples)?;
                        pvalues.push(CdnotsPValue {
                            first: x,
                            second: y,
                            conditions: test.conditions.clone(),
                            p_value: test.p_value,
                            statistic: test.statistic,
                        });
                        update_strength(&mut strengths, x, y, test.statistic);
                        if test.p_value > configuration.alpha {
                            removals.insert((x, y));
                            separating.extend(test.conditions);
                        }
                    }
                    graph.separating_sets[x][y].push(separating.iter().copied().collect());
                    graph.separating_sets[y][x].push(separating.iter().copied().collect());

                    let mut from_y: Vec<usize> = snapshot
                        .neighbors(y)
                        .into_iter()
                        .filter(|&neighbor| neighbor != x)
                        .collect();
                    sort_neighbors(
                        &mut from_y,
                        y,
                        &strengths,
                        configuration.max_combinations.is_some(),
                    );
                    for conditions in combinations(&from_y, depth, configuration.max_combinations) {
                        let test = test_pair(embedded, x, y, &conditions, minimum_samples)?;
                        pvalues.push(CdnotsPValue {
                            first: x,
                            second: y,
                            conditions: test.conditions.clone(),
                            p_value: test.p_value,
                            statistic: test.statistic,
                        });
                        update_strength(&mut strengths, x, y, test.statistic);
                        if test.p_value > configuration.alpha {
                            removals.insert((x, y));
                            separating.extend(test.conditions);
                        }
                    }
                    graph.separating_sets[x][y].push(separating.iter().copied().collect());
                    graph.separating_sets[y][x].push(separating.iter().copied().collect());
                }
            }
            for (x, y) in removals {
                remove_stationary_edge(&mut graph, current_variables, configuration.max_lag, x, y);
            }
            depth += 1;
            continue;
        }

        let mut pair_results: BTreeMap<(usize, usize), (Vec<PairTest>, Vec<PairTest>)> =
            BTreeMap::new();
        for x in 0..current_variables {
            let neighbors_x = snapshot.neighbors(x);
            for &y in &neighbors_x {
                let mut from_x: Vec<usize> = neighbors_x
                    .iter()
                    .copied()
                    .filter(|&neighbor| neighbor != y)
                    .collect();
                sort_neighbors(
                    &mut from_x,
                    x,
                    &strengths,
                    configuration.max_combinations.is_some(),
                );
                let mut first = Vec::new();
                for conditions in combinations(&from_x, depth, configuration.max_combinations) {
                    first.push(test_pair(embedded, x, y, &conditions, minimum_samples)?);
                }

                let mut from_y: Vec<usize> = snapshot
                    .neighbors(y)
                    .into_iter()
                    .filter(|&neighbor| neighbor != x)
                    .collect();
                sort_neighbors(
                    &mut from_y,
                    y,
                    &strengths,
                    configuration.max_combinations.is_some(),
                );
                let mut second = Vec::new();
                for conditions in combinations(&from_y, depth, configuration.max_combinations) {
                    second.push(test_pair(embedded, x, y, &conditions, minimum_samples)?);
                }
                if !first.is_empty() || !second.is_empty() {
                    pair_results.insert((x, y), (first, second));
                }
            }
        }

        let mut removals = BTreeSet::new();
        for x in 0..current_variables {
            for y in snapshot.neighbors(x) {
                let Some((first, second)) = pair_results.get(&(x, y)) else {
                    continue;
                };
                let mut separating = BTreeSet::new();
                for phase in [first, second] {
                    for test in phase {
                        pvalues.push(CdnotsPValue {
                            first: x,
                            second: y,
                            conditions: test.conditions.clone(),
                            p_value: test.p_value,
                            statistic: test.statistic,
                        });
                        update_strength(&mut strengths, x, y, test.statistic);
                        if test.p_value > configuration.alpha {
                            removals.insert((x, y));
                            separating.extend(test.conditions.iter().copied());
                        }
                    }
                    graph.separating_sets[x][y].push(separating.iter().copied().collect());
                    graph.separating_sets[y][x].push(separating.iter().copied().collect());
                }
            }
        }

        for (x, y) in removals {
            remove_stationary_edge(&mut graph, current_variables, configuration.max_lag, x, y);
        }
        depth += 1;
    }

    Ok((graph, pvalues))
}

fn per_variable_skeleton(
    mut graph: Graph,
    embedded: &CdnotsEmbedded,
    alpha: f64,
    max_degree: Option<usize>,
    minimum_samples: usize,
) -> Result<(Graph, Vec<CdnotsPValue>, Vec<Vec<usize>>), CdnotsError> {
    let current_variables = embedded.variables;
    let initial_parents = (0..current_variables)
        .map(|target| {
            graph
                .neighbors(target)
                .into_iter()
                .filter(|&node| node >= current_variables)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut surviving = vec![BTreeSet::new(); current_variables];
    let mut pvalues = Vec::new();

    for target in 0..current_variables {
        let mut parents = initial_parents[target].clone();
        let mut strengths = parents
            .iter()
            .map(|&parent| (parent, f64::INFINITY))
            .collect::<BTreeMap<_, _>>();
        let maximum_depth = max_degree.unwrap_or(parents.len());
        for depth in 0..=maximum_depth {
            if parents.len().saturating_sub(1) < depth || parents.is_empty() {
                break;
            }
            let mut nonsignificant = Vec::new();
            for &parent in &parents {
                let others = parents
                    .iter()
                    .copied()
                    .filter(|&candidate| candidate != parent)
                    .collect::<Vec<_>>();
                for conditions in combinations(&others, depth, Some(1)) {
                    let test =
                        test_pair_ols(embedded, parent, target, &conditions, minimum_samples)?;
                    pvalues.push(CdnotsPValue {
                        first: parent,
                        second: target,
                        conditions: conditions.clone(),
                        p_value: test.p_value,
                        statistic: test.statistic,
                    });
                    let strength = if test.statistic == 0.0 {
                        0.0
                    } else {
                        test.statistic.abs()
                    };
                    strengths
                        .entry(parent)
                        .and_modify(|current| *current = current.min(strength));
                    if test.p_value > alpha {
                        nonsignificant.push(parent);
                        graph.separating_sets[parent][target].push(conditions.clone());
                        graph.separating_sets[target][parent].push(conditions);
                        break;
                    }
                }
            }
            parents.retain(|parent| !nonsignificant.contains(parent));
            for parent in nonsignificant {
                strengths.remove(&parent);
            }
            parents.sort_by(|left, right| {
                strengths
                    .get(right)
                    .copied()
                    .unwrap_or(0.0)
                    .total_cmp(&strengths.get(left).copied().unwrap_or(0.0))
            });
        }
        surviving[target].extend(parents);
    }

    for target in 0..current_variables {
        for &parent in &initial_parents[target] {
            if !surviving[target].contains(&parent) {
                remove_shifted_edge(
                    &mut graph,
                    current_variables,
                    embedded.max_lag,
                    parent,
                    target,
                );
            }
        }
    }
    let surviving = surviving
        .into_iter()
        .map(|parents| parents.into_iter().collect())
        .collect();
    Ok((graph, pvalues, surviving))
}

#[allow(clippy::too_many_arguments)]
fn mci_skeleton(
    mut graph: Graph,
    embedded: &CdnotsEmbedded,
    extended: &CdnotsEmbedded,
    alpha: f64,
    lagged_parents: &[Vec<usize>],
    context_columns: usize,
    max_conds_y: Option<usize>,
    max_conds_x: Option<usize>,
    legacy_conditions: bool,
    minimum_samples: usize,
) -> Result<(Graph, Vec<CdnotsPValue>), CdnotsError> {
    let current_variables = embedded.variables;
    let current_nodes = current_variables * (embedded.max_lag + 1);
    let context_start = current_variables - context_columns;
    let is_context = |node: usize| node % current_variables >= context_start;
    let mut parents = vec![Vec::new(); current_variables];
    let mut contemporaneous = vec![Vec::new(); current_variables];
    let mut strengths = vec![BTreeMap::new(); current_variables];
    let mut base_conditions = vec![Vec::new(); current_variables];

    for target in 0..current_variables {
        if is_context(target) {
            continue;
        }
        parents[target] = graph
            .neighbors(target)
            .into_iter()
            .filter(|&node| !is_context(node))
            .collect();
        contemporaneous[target] = parents[target]
            .iter()
            .copied()
            .filter(|&node| node < current_variables)
            .collect();
        strengths[target] = parents[target]
            .iter()
            .map(|&parent| (parent, f64::INFINITY))
            .collect();
        let mut base = lagged_parents[target]
            .iter()
            .copied()
            .filter(|&node| !is_context(node))
            .collect::<Vec<_>>();
        if let Some(maximum) = max_conds_y {
            base.truncate(maximum);
        }
        base.extend(
            graph
                .neighbors(target)
                .into_iter()
                .filter(|&node| is_context(node)),
        );
        base_conditions[target] = base;
    }

    let mut pvalues = Vec::new();
    let mut depth = 0;
    loop {
        let mut removals = Vec::new();
        let mut tested_any = false;
        for target in 0..current_variables {
            if is_context(target) {
                continue;
            }
            for &parent in &parents[target].clone() {
                let parent_variable = parent % current_variables;
                let parent_lag = parent / current_variables;
                let other_contemporaneous = contemporaneous[target]
                    .iter()
                    .copied()
                    .filter(|&candidate| candidate != parent)
                    .collect::<Vec<_>>();
                if depth > other_contemporaneous.len() {
                    continue;
                }
                tested_any = true;
                let base_y = base_conditions[target]
                    .iter()
                    .copied()
                    .filter(|&condition| {
                        if legacy_conditions {
                            condition % current_variables != parent_variable
                        } else {
                            condition != parent
                        }
                    })
                    .collect::<Vec<_>>();
                let mut base_x = Vec::new();
                for &condition in &lagged_parents[parent_variable] {
                    let condition_variable = condition % current_variables;
                    let shifted_lag = condition / current_variables + parent_lag;
                    if legacy_conditions && condition_variable == target {
                        continue;
                    }
                    if condition_variable < context_start {
                        let shifted = condition_variable + shifted_lag * current_variables;
                        if shifted < extended.variables * (extended.max_lag + 1)
                            && shifted != target
                        {
                            base_x.push(shifted);
                        }
                    }
                }
                if let Some(maximum) = max_conds_x {
                    base_x.truncate(maximum);
                }
                for subset in combinations(&other_contemporaneous, depth, None) {
                    let mut conditions = subset;
                    for condition in base_y.iter().chain(&base_x) {
                        if !conditions.contains(condition) {
                            conditions.push(*condition);
                        }
                    }
                    let test =
                        test_pair_ols(extended, parent, target, &conditions, minimum_samples)?;
                    pvalues.push(CdnotsPValue {
                        first: parent,
                        second: target,
                        conditions: conditions.clone(),
                        p_value: test.p_value,
                        statistic: test.statistic,
                    });
                    let strength = if test.statistic == 0.0 {
                        0.0
                    } else {
                        test.statistic.abs()
                    };
                    strengths[target]
                        .entry(parent)
                        .and_modify(|current| *current = current.min(strength));
                    if test.p_value > alpha {
                        removals.push((parent, target, conditions.clone()));
                        graph.separating_sets[parent][target].push(conditions.clone());
                        graph.separating_sets[target][parent].push(conditions);
                        break;
                    }
                }
            }
        }

        for (parent, target, _) in removals {
            parents[target].retain(|&candidate| candidate != parent);
            strengths[target].remove(&parent);
            contemporaneous[target].retain(|&candidate| candidate != parent);
            if parent < current_variables {
                parents[parent].retain(|&candidate| candidate != target);
                strengths[parent].remove(&target);
                contemporaneous[parent].retain(|&candidate| candidate != target);
            }
            remove_shifted_edge(
                &mut graph,
                current_variables,
                embedded.max_lag,
                parent,
                target,
            );
        }
        for target in 0..current_variables {
            if is_context(target) {
                continue;
            }
            parents[target].sort_by(|left, right| {
                strengths[target]
                    .get(right)
                    .copied()
                    .unwrap_or(0.0)
                    .total_cmp(&strengths[target].get(left).copied().unwrap_or(0.0))
            });
            contemporaneous[target].sort_by(|left, right| {
                strengths[target]
                    .get(right)
                    .copied()
                    .unwrap_or(0.0)
                    .total_cmp(&strengths[target].get(left).copied().unwrap_or(0.0))
            });
        }
        if !tested_any {
            break;
        }
        depth += 1;
        if depth > current_nodes {
            break;
        }
    }
    Ok((graph, pvalues))
}

fn orient_stationary(
    graph: &mut Graph,
    cause: usize,
    effect: usize,
    current_variables: usize,
    max_lag: usize,
    conflict: bool,
) {
    fn orient_one(graph: &mut Graph, cause: usize, effect: usize, conflict: bool) {
        if conflict && graph.directed(effect, cause) {
            graph.set_bidirected(cause, effect);
        } else if !conflict || graph.undirected(cause, effect) || !graph.adjacent(cause, effect) {
            graph.set_directed(cause, effect);
        }
    }

    orient_one(graph, cause, effect, conflict);
    let cause_step = cause / current_variables;
    let effect_step = effect / current_variables;
    for shift in 1..=max_lag.saturating_sub(cause_step.max(effect_step)) {
        orient_one(
            graph,
            cause + shift * current_variables,
            effect + shift * current_variables,
            conflict,
        );
    }
}

fn unshielded_triples(graph: &Graph) -> Vec<(usize, usize, usize)> {
    let mut triples = Vec::new();
    for middle in 0..graph.nodes() {
        let neighbors = graph.neighbors(middle);
        for &left in &neighbors {
            for &right in &neighbors {
                if left != right && !graph.adjacent(left, right) {
                    triples.push((left, middle, right));
                }
            }
        }
    }
    triples
}

fn majority_collider(
    graph: &Graph,
    embedded: &CdnotsEmbedded,
    x: usize,
    middle: usize,
    z: usize,
    alpha: f64,
    minimum_samples: usize,
    lagged_parents: Option<&[Vec<usize>]>,
) -> Result<bool, CdnotsError> {
    let current_variables = embedded.variables;
    let endpoint = if z < current_variables { z } else { x };
    let other = if z < current_variables { x } else { z };
    let mut subsets = Vec::new();
    let endpoint_neighbors: Vec<usize> = graph
        .neighbors(endpoint)
        .into_iter()
        .filter(|&node| node < current_variables && node != other)
        .collect();
    for size in 0..=endpoint_neighbors.len() {
        for subset in combinations(&endpoint_neighbors, size, None) {
            if !subsets.contains(&subset) {
                subsets.push(subset);
            }
        }
    }
    if other < current_variables {
        let other_neighbors: Vec<usize> = graph
            .neighbors(other)
            .into_iter()
            .filter(|&node| node < current_variables && node != endpoint)
            .collect();
        for size in 0..=other_neighbors.len() {
            for subset in combinations(&other_neighbors, size, None) {
                if !subsets.contains(&subset) {
                    subsets.push(subset);
                }
            }
        }
    }

    let base_endpoint = lagged_parents
        .and_then(|parents| parents.get(endpoint % current_variables))
        .cloned()
        .unwrap_or_default();
    let base_other = lagged_parents
        .and_then(|parents| parents.get(other % current_variables))
        .cloned()
        .unwrap_or_default();
    let mut separating = Vec::new();
    for subset in subsets.into_iter().take(1000) {
        let mut conditions = subset.clone();
        for condition in base_endpoint.iter().chain(&base_other) {
            if !conditions.contains(condition) {
                conditions.push(*condition);
            }
        }
        let test = test_pair_ols(embedded, x, z, &conditions, minimum_samples)?;
        if test.p_value > alpha {
            separating.push(subset);
        }
    }
    if separating.is_empty() {
        return Ok(false);
    }
    let with_middle = separating
        .iter()
        .filter(|subset| subset.contains(&middle))
        .count();
    Ok((with_middle as f64 / separating.len() as f64) < 0.5)
}

fn orient_colliders(
    mut graph: Graph,
    embedded: &CdnotsEmbedded,
    configuration: CdnotsConfiguration,
    minimum_samples: usize,
    lagged_parents: Option<&[Vec<usize>]>,
) -> Result<Graph, CdnotsError> {
    let current_variables = embedded.variables;
    let triples: Vec<_> = unshielded_triples(&graph)
        .into_iter()
        .filter(|(left, middle, right)| left < right && *middle < current_variables)
        .collect();
    for (left, middle, right) in triples {
        if graph.directed(left, middle) && graph.directed(right, middle) {
            continue;
        }
        if !majority_collider(
            &graph,
            embedded,
            left,
            middle,
            right,
            configuration.alpha,
            minimum_samples,
            lagged_parents,
        )? {
            continue;
        }
        match configuration.priority {
            ColliderPriority::Overwrite => {
                orient_stationary(
                    &mut graph,
                    left,
                    middle,
                    current_variables,
                    configuration.max_lag,
                    false,
                );
                orient_stationary(
                    &mut graph,
                    right,
                    middle,
                    current_variables,
                    configuration.max_lag,
                    false,
                );
            }
            ColliderPriority::MarkConflict => {
                orient_stationary(
                    &mut graph,
                    left,
                    middle,
                    current_variables,
                    configuration.max_lag,
                    true,
                );
                orient_stationary(
                    &mut graph,
                    right,
                    middle,
                    current_variables,
                    configuration.max_lag,
                    true,
                );
            }
            ColliderPriority::KeepFirst => {
                if !graph.directed(middle, left) && !graph.directed(middle, right) {
                    orient_stationary(
                        &mut graph,
                        left,
                        middle,
                        current_variables,
                        configuration.max_lag,
                        false,
                    );
                    orient_stationary(
                        &mut graph,
                        right,
                        middle,
                        current_variables,
                        configuration.max_lag,
                        false,
                    );
                }
            }
        }
    }
    Ok(graph)
}

fn lag_constraints(graph: &mut Graph, current_variables: usize, max_lag: usize) {
    for newer_step in 0..max_lag {
        for newer in newer_step * current_variables..(newer_step + 1) * current_variables {
            for older in (newer_step + 1) * current_variables..(max_lag + 1) * current_variables {
                if graph.adjacent(newer, older) {
                    graph.set_directed(older, newer);
                }
            }
        }
    }
}

fn context_and_lag_constraints(
    graph: &mut Graph,
    current_variables: usize,
    max_lag: usize,
    context_columns: usize,
) {
    if context_columns > 0 {
        for lag in 0..=max_lag {
            for context in 0..context_columns {
                let context_node = (lag + 1) * current_variables - context_columns + context;
                for neighbor in graph.neighbors(context_node) {
                    graph.set_directed(context_node, neighbor);
                }
            }
        }
    }
    lag_constraints(graph, current_variables, max_lag);
}

fn standardize_columns(values: &DMatrix<f32>) -> DMatrix<f32> {
    DMatrix::from_fn(values.nrows(), values.ncols(), |row, column| {
        let mean = values.column(column).iter().sum::<f32>() / values.nrows() as f32;
        let sum_squares = values
            .column(column)
            .iter()
            .map(|value| (*value - mean).powi(2))
            .sum::<f32>();
        let deviation = (sum_squares / (values.nrows() - 1) as f32).sqrt();
        (values[(row, column)] - mean) / deviation
    })
}

fn lower_median(mut values: Vec<f32>) -> f32 {
    values.sort_by(f32::total_cmp);
    values[(values.len() - 1) / 2]
}

fn rbf_kernel(values: &DMatrix<f32>, width: Option<f32>) -> DMatrix<f32> {
    let width = width.unwrap_or_else(|| {
        let limit = values.nrows().min(500);
        let mut distances = Vec::new();
        for left in 0..limit {
            for right in left + 1..limit {
                let distance = (0..values.ncols())
                    .map(|column| (values[(left, column)] - values[(right, column)]).powi(2))
                    .sum::<f32>()
                    .sqrt();
                if distance > 0.0 {
                    distances.push(distance);
                }
            }
        }
        if distances.is_empty() {
            1.0
        } else {
            (values.ncols() as f32).sqrt() * lower_median(distances)
        }
    });
    let inverse_width_square = 1.0 / (width * width);
    DMatrix::from_fn(values.nrows(), values.nrows(), |left, right| {
        let squared_distance = (0..values.ncols())
            .map(|column| (values[(left, column)] - values[(right, column)]).powi(2))
            .sum::<f32>();
        (-inverse_width_square * squared_distance).exp()
    })
}

fn squared_distances(values: &DMatrix<f32>) -> DMatrix<f32> {
    DMatrix::from_fn(values.nrows(), values.ncols(), |row, column| {
        values[(row, row)] + values[(column, column)] - 2.0 * values[(row, column)]
    })
}

fn gaussian_from_squared_distances(distances: &DMatrix<f32>) -> DMatrix<f32> {
    let positive = (0..distances.nrows())
        .flat_map(|row| (0..row).map(move |column| distances[(row, column)]))
        .filter(|value| *value > 0.0)
        .collect::<Vec<_>>();
    if positive.is_empty() {
        return DMatrix::from_element(distances.nrows(), distances.ncols(), f32::NAN);
    }
    let scale = lower_median(positive);
    distances.map(|distance| (-distance / scale / 2.0).exp())
}

fn center_kernel(kernel: &DMatrix<f32>) -> DMatrix<f32> {
    let rows = kernel.nrows();
    let sums: Vec<f32> = (0..rows)
        .map(|column| kernel.column(column).iter().sum())
        .collect();
    let total = sums.iter().sum::<f32>();
    DMatrix::from_fn(rows, rows, |row, column| {
        kernel[(row, column)] - (sums[column] + sums[row]) / rows as f32
            + total / (rows * rows) as f32
    })
}

fn infer_nonstationary_direction(
    data: &CdnotsEmbedded,
    causes: &[usize],
    effect: usize,
    context_start: usize,
    context_columns: usize,
) -> f64 {
    let rows = data.values.len();
    let x = DMatrix::from_fn(rows, causes.len(), |row, column| {
        data.values[row][causes[column]].unwrap_or(f64::NAN) as f32
    });
    let y = DMatrix::from_fn(rows, 1, |row, _| {
        data.values[row][effect].unwrap_or(f64::NAN) as f32
    });
    let context = DMatrix::from_fn(rows, context_columns, |row, column| {
        data.values[row][context_start + column].unwrap_or(f64::NAN) as f32
    });
    if x.iter()
        .chain(y.iter())
        .chain(context.iter())
        .any(|v| !v.is_finite())
    {
        return f64::NAN;
    }
    let x = standardize_columns(&x);
    let y = standardize_columns(&y);
    let k_y = rbf_kernel(&y, None);
    let k_x = rbf_kernel(&x, None);
    let k_t = rbf_kernel(&context, Some(1.0));
    let identity = DMatrix::<f32>::identity(rows, rows);

    let first_system = k_x.component_mul(&k_t) + identity.clone() * 2.0;
    let Some(first_inverse) = first_system.try_inverse() else {
        return f64::NAN;
    };
    let x_cube = k_x.map(|value| value.powi(3));
    let product = &first_inverse * &k_y * &first_inverse;
    let linear = k_t
        .component_mul(&x_cube.component_mul(&product))
        .component_mul(&k_t)
        / (rows * rows) as f32;
    let first_gaussian = gaussian_from_squared_distances(&squared_distances(&linear));

    let second_system = &k_t + identity * 2.0;
    let Some(second_inverse) = second_system.try_inverse() else {
        return f64::NAN;
    };
    let second_linear = &k_t * &second_inverse * &k_x * &second_inverse * &k_t;
    let second_gaussian = gaussian_from_squared_distances(&squared_distances(&second_linear));
    let first_centered = center_kernel(&first_gaussian);
    let second_centered = center_kernel(&second_gaussian);
    (first_centered
        .transpose()
        .component_mul(&second_centered)
        .iter()
        .sum::<f32>()
        / (rows * rows) as f32) as f64
}

fn phase_three(
    mut graph: Graph,
    embedded: &CdnotsEmbedded,
    context_columns: usize,
    configuration: CdnotsConfiguration,
) -> Graph {
    if context_columns == 0 {
        return meek(graph, embedded.variables, configuration.max_lag);
    }
    let context_start = embedded.variables - context_columns;
    let mut candidates = BTreeSet::new();
    for context in context_start..embedded.variables {
        for variable in 0..context_start {
            if graph.endpoints[context][variable] == TAIL
                && graph.endpoints[variable][context] == ARROW
            {
                candidates.insert(variable);
            }
        }
    }
    let mut candidates: Vec<usize> = candidates
        .into_iter()
        .filter(|&variable| (0..context_start).any(|other| graph.undirected(variable, other)))
        .collect();

    while candidates.len() > 1 {
        let hypotheses: Vec<Vec<usize>> = candidates
            .iter()
            .map(|&effect| {
                let mut causes = (0..context_start)
                    .filter(|&cause| {
                        graph.undirected(cause, effect) || graph.directed(cause, effect)
                    })
                    .collect::<Vec<_>>();
                causes.sort_unstable();
                causes
            })
            .collect();
        let scores: Vec<f64> = candidates
            .iter()
            .zip(&hypotheses)
            .map(|(&effect, causes)| {
                infer_nonstationary_direction(
                    embedded,
                    causes,
                    effect,
                    context_start,
                    context_columns,
                )
            })
            .collect();
        let best_index = scores
            .iter()
            .position(|score| score.is_nan())
            .unwrap_or_else(|| {
                scores
                    .iter()
                    .enumerate()
                    .min_by(|left, right| left.1.total_cmp(right.1))
                    .map(|(index, _)| index)
                    .unwrap_or(0)
            });
        if configuration.orient_margin > 0.0 {
            let best = scores[best_index];
            if !best.is_finite() {
                break;
            }
            let mut finite = scores
                .iter()
                .copied()
                .filter(|score| score.is_finite())
                .collect::<Vec<_>>();
            finite.sort_by(f64::total_cmp);
            if finite.len() < 2 {
                break;
            }
            let scale = finite[0].abs().max(finite[1].abs()).max(1e-300);
            if (finite[1] - finite[0]) / scale < configuration.orient_margin {
                break;
            }
        }
        let sink = candidates[best_index];
        for &cause in &hypotheses[best_index] {
            orient_stationary(
                &mut graph,
                cause,
                sink,
                embedded.variables,
                configuration.max_lag,
                false,
            );
        }
        candidates.remove(best_index);
    }
    meek(graph, embedded.variables, configuration.max_lag)
}

fn triangles(graph: &Graph) -> Vec<(usize, usize, usize)> {
    let mut output = Vec::new();
    for middle in 0..graph.nodes() {
        let neighbors = graph.neighbors(middle);
        for &left in &neighbors {
            for &right in &neighbors {
                if left != right && graph.adjacent(left, right) {
                    output.push((left, middle, right));
                }
            }
        }
    }
    output
}

fn kites(graph: &Graph, triangles: &[(usize, usize, usize)]) -> Vec<(usize, usize, usize, usize)> {
    let mut grouped: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
    for &(anchor, middle, apex) in triangles {
        grouped.entry((anchor, apex)).or_default().push(middle);
    }
    let mut output = Vec::new();
    for ((anchor, apex), middles) in grouped {
        for pair in combinations(&middles, 2, None) {
            let left = pair[0].min(pair[1]);
            let right = pair[0].max(pair[1]);
            if !graph.adjacent(left, right) {
                output.push((anchor, left, right, apex));
            }
        }
    }
    output
}

fn meek(mut graph: Graph, current_variables: usize, max_lag: usize) -> Graph {
    let unshielded = unshielded_triples(&graph);
    let triangle_list = triangles(&graph);
    let kite_list = kites(&graph, &triangle_list);
    let mut changed = true;
    let mut iteration: i32 = 0;
    let mut added: BTreeMap<(usize, usize), i32> = BTreeMap::new();
    while changed && iteration < 500 {
        changed = false;
        for &(left, middle, right) in &unshielded {
            if left < current_variables
                && middle < current_variables
                && graph.directed(left, middle)
                && graph.undirected(middle, right)
            {
                let previous = added.get(&(middle, right)).copied();
                if !graph.directed_path(right, middle) {
                    orient_stationary(&mut graph, middle, right, current_variables, max_lag, false);
                }
                changed |= previous != Some(iteration - 1);
                added.insert((middle, right), iteration);
            }
        }
        for &(left, middle, right) in &triangle_list {
            if left < current_variables
                && right < current_variables
                && graph.directed(left, middle)
                && graph.directed(middle, right)
                && graph.undirected(left, right)
            {
                let previous = added.get(&(left, right)).copied();
                if !graph.directed_path(right, left) {
                    orient_stationary(&mut graph, left, right, current_variables, max_lag, false);
                }
                changed |= previous != Some(iteration - 1);
                added.insert((left, right), iteration);
            }
        }
        for &(anchor, left, right, apex) in &kite_list {
            if [anchor, left, right, apex]
                .iter()
                .all(|&node| node < current_variables)
                && graph.undirected(anchor, left)
                && graph.undirected(anchor, right)
                && graph.directed(left, apex)
                && graph.directed(right, apex)
                && graph.undirected(anchor, apex)
            {
                let previous = added.get(&(anchor, apex)).copied();
                if !graph.directed_path(apex, anchor) {
                    orient_stationary(&mut graph, anchor, apex, current_variables, max_lag, false);
                }
                changed |= previous != Some(iteration - 1);
                added.insert((anchor, apex), iteration);
            }
        }
        iteration += 1;
    }
    graph
}

fn convert_time_graph(
    graph: &Graph,
    variables: usize,
    max_lag: usize,
    keep_undirected: bool,
) -> Vec<Vec<Vec<u8>>> {
    let mut output = vec![vec![vec![0; max_lag + 1]; variables]; variables];
    for cause in 0..variables {
        for effect in 0..variables {
            if cause == effect {
                continue;
            }
            let forward = graph.endpoints[cause][effect];
            let backward = graph.endpoints[effect][cause];
            if (forward == TAIL && backward == ARROW)
                || (keep_undirected && forward == TAIL && backward == TAIL)
            {
                output[cause][effect][0] = 1;
            }
        }
        for lag in 1..=max_lag {
            for cause_variable in 0..variables {
                let lagged = cause_variable + lag * variables;
                if graph.directed(lagged, cause)
                    || (keep_undirected && graph.undirected(lagged, cause))
                {
                    output[cause_variable][cause][lag] = 1;
                }
            }
        }
    }
    output
}

fn pvalue_matrix(
    pvalues: &[CdnotsPValue],
    variables: usize,
    max_lag: usize,
) -> Vec<Vec<Vec<Option<f64>>>> {
    let mut output: Vec<Vec<Vec<Option<f64>>>> =
        vec![vec![vec![None; max_lag + 1]; variables]; variables];
    let mut seen = BTreeSet::new();
    for record in pvalues {
        let key = (
            record.first,
            record.second,
            record.conditions.clone(),
            record.p_value.to_bits(),
        );
        if !seen.insert(key) {
            continue;
        }
        let effect_variable = record.first % variables;
        let effect_lag = record.first / variables;
        let cause_variable = record.second % variables;
        let cause_lag = record.second / variables;
        if effect_lag != 0 || cause_lag > max_lag {
            continue;
        }
        let slot = &mut output[cause_variable][effect_variable][cause_lag];
        *slot = Some(slot.map_or(record.p_value, |current| current.min(record.p_value)));
    }
    output
}

fn prepare_values(
    values: &[Vec<Option<f64>>],
    missing: CdnotsMissingPolicy,
    context_preset: Option<CausalTsContextPreset>,
) -> Result<(Vec<Vec<Option<f64>>>, usize, usize), CdnotsError> {
    let context = context_preset
        .map(|preset| causal_ts_context(values.len(), preset))
        .unwrap_or_default();
    let context_columns = context.first().map_or(0, Vec::len);
    let (mut prepared, minimum_samples) = match missing {
        CdnotsMissingPolicy::PairwiseComplete { minimum_samples } => {
            (values.to_vec(), minimum_samples)
        }
        CdnotsMissingPolicy::VarEm(configuration) => {
            let imputation = causal_ts_var_em(values, configuration)?;
            (
                imputation
                    .values
                    .into_iter()
                    .map(|row| row.into_iter().map(Some).collect())
                    .collect(),
                30,
            )
        }
    };
    if context_columns > 0 {
        for (row, context_row) in prepared.iter_mut().zip(context) {
            row.extend(context_row.into_iter().map(Some));
        }
    }
    Ok((prepared, minimum_samples, context_columns))
}

fn embed_at_cutoff(
    values: &[Vec<Option<f64>>],
    embedded_max_lag: usize,
    cutoff: usize,
) -> Result<CdnotsEmbedded, CdnotsError> {
    let variables = values
        .first()
        .map(Vec::len)
        .ok_or(CausalTsPreparationError::EmptyMatrix)?;
    if cutoff >= values.len() || embedded_max_lag > cutoff {
        return Err(CausalTsPreparationError::InvalidLag.into());
    }
    let reference_points = (cutoff..values.len()).collect::<Vec<_>>();
    let embedded = reference_points
        .iter()
        .map(|&time| {
            let mut row = Vec::with_capacity(variables * (embedded_max_lag + 1));
            for lag in 0..=embedded_max_lag {
                row.extend(values[time - lag].iter().copied());
            }
            row
        })
        .collect();
    Ok(CdnotsEmbedded {
        values: embedded,
        reference_points,
        variables,
        max_lag: embedded_max_lag,
    })
}

fn run_cdnots_inner<F>(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsConfiguration,
    mut progress: F,
) -> Result<CdnotsResult, CdnotsError>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(0.0 < configuration.alpha && configuration.alpha < 1.0) {
        return Err(CdnotsError::InvalidAlpha);
    }
    let (prepared_values, minimum_samples, context_columns) =
        prepare_values(values, configuration.missing, configuration.context)?;
    progress("preparation", 1, 5);
    let embedded = cdnots_embed(&prepared_values, configuration.max_lag)?;
    let nodes = embedded.variables * (configuration.max_lag + 1);
    let initial =
        constrained_complete_graph(embedded.variables, configuration.max_lag, context_columns);
    debug_assert_eq!(initial.nodes(), nodes);
    let (skeleton, pvalues) = stable_skeleton(initial, &embedded, configuration, minimum_samples)?;
    progress("skeleton", 2, 5);
    let mut constrained = skeleton;
    context_and_lag_constraints(
        &mut constrained,
        embedded.variables,
        configuration.max_lag,
        context_columns,
    );
    let colliders = orient_colliders(constrained, &embedded, configuration, minimum_samples, None)?;
    progress("colliders", 3, 5);
    let oriented = meek(colliders, embedded.variables, configuration.max_lag);
    progress("orientation", 4, 5);
    let oriented = phase_three(oriented, &embedded, context_columns, configuration);
    progress("nonstationarity", 5, 5);
    let time_graph = convert_time_graph(&oriented, embedded.variables, configuration.max_lag, true);
    let pvalue_matrix = pvalue_matrix(&pvalues, embedded.variables, configuration.max_lag);
    Ok(CdnotsResult {
        graph: oriented.endpoints,
        time_graph,
        pvalue_matrix,
        pvalues,
    })
}

pub fn run_cdnots(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsConfiguration,
) -> Result<CdnotsResult, CdnotsError> {
    run_cdnots_inner(values, configuration, |_, _, _| {})
}

pub fn run_cdnots_with_progress<F>(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsConfiguration,
    progress: F,
) -> Result<CdnotsResult, CdnotsError>
where
    F: FnMut(&'static str, usize, usize),
{
    run_cdnots_inner(values, configuration, progress)
}

fn run_cdnots_plus_inner<F>(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsPlusConfiguration,
    mut progress: F,
) -> Result<CdnotsResult, CdnotsError>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(0.0 < configuration.alpha && configuration.alpha < 1.0) {
        return Err(CdnotsError::InvalidAlpha);
    }
    let (prepared_values, minimum_samples, context_columns) =
        prepare_values(values, configuration.missing, configuration.context)?;
    progress("preparation", 1, 6);
    let embedded = cdnots_embed(&prepared_values, configuration.max_lag)?;
    let extended = embed_at_cutoff(
        &prepared_values,
        2 * configuration.max_lag,
        2 * configuration.max_lag,
    )?;
    let initial =
        constrained_complete_graph(embedded.variables, configuration.max_lag, context_columns);
    let (phase_one, mut pvalues, lagged_parents) = per_variable_skeleton(
        initial,
        &embedded,
        configuration.alpha,
        configuration.max_degree,
        minimum_samples,
    )?;
    progress("lagged-skeleton", 2, 6);
    let (skeleton, phase_two_pvalues) = mci_skeleton(
        phase_one,
        &embedded,
        &extended,
        configuration.alpha,
        &lagged_parents,
        context_columns,
        configuration.max_conds_y,
        configuration.max_conds_x,
        configuration.legacy_mci_conditions,
        minimum_samples,
    )?;
    progress("mci-skeleton", 3, 6);
    pvalues.extend(phase_two_pvalues);
    let mut constrained = skeleton;
    context_and_lag_constraints(
        &mut constrained,
        embedded.variables,
        configuration.max_lag,
        context_columns,
    );
    let orientation = CdnotsConfiguration {
        max_lag: configuration.max_lag,
        alpha: configuration.alpha,
        max_degree: configuration.max_degree,
        max_combinations: Some(1),
        priority: configuration.priority,
        missing: configuration.missing,
        context: configuration.context,
        orient_margin: configuration.orient_margin,
    };
    let colliders = orient_colliders(
        constrained,
        &embedded,
        orientation,
        minimum_samples,
        Some(&lagged_parents),
    )?;
    progress("colliders", 4, 6);
    let oriented = meek(colliders, embedded.variables, configuration.max_lag);
    progress("orientation", 5, 6);
    let oriented = phase_three(oriented, &embedded, context_columns, orientation);
    progress("nonstationarity", 6, 6);
    let time_graph =
        convert_time_graph(&oriented, embedded.variables, configuration.max_lag, false);
    let pvalue_matrix = pvalue_matrix(&pvalues, embedded.variables, configuration.max_lag);
    Ok(CdnotsResult {
        graph: oriented.endpoints,
        time_graph,
        pvalue_matrix,
        pvalues,
    })
}

pub fn run_cdnots_plus(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsPlusConfiguration,
) -> Result<CdnotsResult, CdnotsError> {
    run_cdnots_plus_inner(values, configuration, |_, _, _| {})
}

pub fn run_cdnots_plus_with_progress<F>(
    values: &[Vec<Option<f64>>],
    configuration: CdnotsPlusConfiguration,
    progress: F,
) -> Result<CdnotsResult, CdnotsError>
where
    F: FnMut(&'static str, usize, usize),
{
    run_cdnots_plus_inner(values, configuration, progress)
}
