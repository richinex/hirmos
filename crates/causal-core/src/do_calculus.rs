//! Graphical identification of level-2 interventional distributions.
//!
//! This is a Rust port of the standard ID algorithm implemented by y0's
//! `y0.algorithm.identify.id_std` (Shpitser and Pearl, 2006). The input is an acyclic
//! directed mixed graph (ADMG): directed edges describe observed causal relations and
//! bidirected edges describe latent common causes. The output is a symbolic expression
//! containing only observational probabilities, products, and marginalizations.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

pub type Variable = String;

fn set(values: impl IntoIterator<Item = Variable>) -> BTreeSet<Variable> {
    values.into_iter().collect()
}

fn latex_name(name: &str) -> String {
    if name
        .chars()
        .all(|character| character.is_ascii_alphanumeric())
    {
        return name.to_owned();
    }
    let mut escaped = String::new();
    for character in name.chars() {
        match character {
            '\\' => escaped.push_str("\\textbackslash{}"),
            '{' | '}' | '$' | '&' | '#' | '_' | '%' => {
                escaped.push('\\');
                escaped.push(character);
            }
            '^' | '~' => {
                escaped.push('\\');
                escaped.push(character);
                escaped.push_str("{}");
            }
            _ => escaped.push(character),
        }
    }
    format!("\\text{{{escaped}}}")
}

/// An observational expression returned by the ID algorithm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    /// `P(children | parents)`. An empty parent set denotes a marginal/joint probability.
    Probability {
        children: BTreeSet<Variable>,
        parents: Vec<Variable>,
    },
    Product(Vec<Expression>),
    Sum {
        ranges: BTreeSet<Variable>,
        expression: Box<Expression>,
    },
    /// A normalized conditional expression. IDC returns a ratio whose denominator is
    /// the numerator marginalized over the requested outcomes.
    Ratio {
        numerator: Box<Expression>,
        denominator: Box<Expression>,
    },
}

impl Expression {
    pub fn joint(variables: impl IntoIterator<Item = Variable>) -> Self {
        Self::Probability {
            children: set(variables),
            parents: Vec::new(),
        }
    }

    pub fn conditional(child: Variable, parents: Vec<Variable>) -> Self {
        Self::Probability {
            children: BTreeSet::from([child]),
            parents,
        }
    }

    pub fn product(expressions: impl IntoIterator<Item = Expression>) -> Self {
        let mut flattened = Vec::new();
        for expression in expressions {
            match expression {
                Self::Product(parts) => flattened.extend(parts),
                other => flattened.push(other),
            }
        }
        // y0's Product.safe is commutative and stores factors canonically. Sorting by the
        // stable wire form gives Rust the same deterministic presentation while retaining
        // an explicitly ordered Vec for inexpensive WASM serialization later.
        flattened.sort_by_key(Self::to_y0);
        match flattened.len() {
            1 => flattened.pop().expect("one expression"),
            _ => Self::Product(flattened),
        }
    }

    pub fn marginalize(expression: Expression, ranges: impl IntoIterator<Item = Variable>) -> Self {
        let ranges = set(ranges);
        if ranges.is_empty() {
            return expression;
        }
        Self::Sum {
            ranges,
            expression: Box::new(expression),
        }
    }

    pub fn ratio(numerator: Expression, denominator: Expression) -> Self {
        Self::Ratio {
            numerator: Box::new(numerator),
            denominator: Box::new(denominator),
        }
    }

    pub fn normalize_marginalize(
        expression: Expression,
        ranges: impl IntoIterator<Item = Variable>,
    ) -> Self {
        let denominator = Self::marginalize(expression.clone(), ranges);
        Self::ratio(expression, denominator)
    }

    /// Stable y0-style text used in parity fixtures and audit records.
    pub fn to_y0(&self) -> String {
        match self {
            Self::Probability { children, parents } => {
                let children = children.iter().cloned().collect::<Vec<_>>().join(", ");
                if parents.is_empty() {
                    format!("P({children})")
                } else {
                    let mut parents = parents.clone();
                    parents.sort();
                    format!("P({children} | {})", parents.join(", "))
                }
            }
            Self::Product(expressions) => expressions
                .iter()
                .map(Self::to_y0)
                .collect::<Vec<_>>()
                .join(" * "),
            Self::Sum { ranges, expression } => format!(
                "Sum[{}]({})",
                ranges.iter().cloned().collect::<Vec<_>>().join(", "),
                expression.to_y0()
            ),
            Self::Ratio {
                numerator,
                denominator,
            } => format!("(({} / {}))", numerator.to_y0(), denominator.to_y0()),
        }
    }

    pub fn to_latex(&self) -> String {
        match self {
            Self::Probability { children, parents } => {
                let children = children
                    .iter()
                    .map(|name| latex_name(name))
                    .collect::<Vec<_>>()
                    .join(", ");
                if parents.is_empty() {
                    format!("P({children})")
                } else {
                    let mut parents = parents.clone();
                    parents.sort();
                    format!(
                        "P({children} \\mid {})",
                        parents
                            .iter()
                            .map(|name| latex_name(name))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            Self::Product(expressions) => expressions
                .iter()
                .map(Self::to_latex)
                .collect::<Vec<_>>()
                .join(" \\; "),
            Self::Sum { ranges, expression } => format!(
                "\\sum_{{{}}} {}",
                ranges
                    .iter()
                    .map(|name| latex_name(name))
                    .collect::<Vec<_>>()
                    .join(", "),
                expression.to_latex()
            ),
            Self::Ratio {
                numerator,
                denominator,
            } => format!(
                "\\frac{{{}}}{{{}}}",
                numerator.to_latex(),
                denominator.to_latex()
            ),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_y0())
    }
}

/// An acyclic directed mixed graph. Bidirected edges are stored as normalized pairs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Admg {
    nodes: BTreeSet<Variable>,
    directed: BTreeSet<(Variable, Variable)>,
    bidirected: BTreeSet<(Variable, Variable)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphError {
    SelfLoop(Variable),
    DirectedCycle,
    UnknownVariable(Variable),
    TreatmentOutcomeOverlap(Variable),
    QueryRoleOverlap(Variable),
    EmptyOutcome,
}

/// Project a DAG with explicitly marked latent variables onto its observed variables.
///
/// A directed projected edge represents a directed path whose internal vertices are all
/// latent. A bidirected projected edge represents a latent common cause connected to both
/// observed endpoints through latent-only directed paths. Traversal stops at the first
/// observed vertex, so observed mediators are never silently removed.
pub fn latent_projection(
    nodes: impl IntoIterator<Item = Variable>,
    directed: impl IntoIterator<Item = (Variable, Variable)>,
    latent: impl IntoIterator<Item = Variable>,
) -> Result<Admg, GraphError> {
    let nodes = set(nodes);
    let directed: BTreeSet<_> = directed.into_iter().collect();
    let latent = set(latent);
    for variable in &latent {
        if !nodes.contains(variable) {
            return Err(GraphError::UnknownVariable(variable.clone()));
        }
    }
    let full = Admg::new(nodes.clone(), directed.clone(), [])?;
    let observed: BTreeSet<_> = nodes.difference(&latent).cloned().collect();
    let mut children: BTreeMap<Variable, BTreeSet<Variable>> = BTreeMap::new();
    for (source, target) in &directed {
        children
            .entry(source.clone())
            .or_default()
            .insert(target.clone());
    }

    let observed_frontier = |start: &Variable| {
        let mut frontier = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([start.clone()]);
        while let Some(node) = queue.pop_front() {
            if !visited.insert(node.clone()) {
                continue;
            }
            if &node != start && observed.contains(&node) {
                frontier.insert(node);
                continue;
            }
            if let Some(next) = children.get(&node) {
                queue.extend(next.iter().cloned());
            }
        }
        frontier
    };

    let mut projected_directed = BTreeSet::new();
    for source in &observed {
        for target in observed_frontier(source) {
            projected_directed.insert((source.clone(), target));
        }
    }

    let mut projected_bidirected = BTreeSet::new();
    for common_cause in &latent {
        let descendants = observed_frontier(common_cause)
            .into_iter()
            .collect::<Vec<_>>();
        for left_index in 0..descendants.len() {
            for right_index in left_index + 1..descendants.len() {
                projected_bidirected.insert((
                    descendants[left_index].clone(),
                    descendants[right_index].clone(),
                ));
            }
        }
    }

    // The construction above is a projection of an already validated DAG. Calling the
    // constructor again checks this invariant and normalizes bidirected pairs.
    let projected = Admg::new(observed, projected_directed, projected_bidirected)?;
    debug_assert!(full.topological_sort().is_ok());
    Ok(projected)
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SelfLoop(v) => write!(f, "self-loop at {v}"),
            Self::DirectedCycle => f.write_str("the directed component must be acyclic"),
            Self::UnknownVariable(v) => write!(f, "unknown variable {v}"),
            Self::TreatmentOutcomeOverlap(v) => {
                write!(f, "{v} cannot be both treatment and outcome")
            }
            Self::QueryRoleOverlap(v) => {
                write!(f, "{v} cannot have more than one query role")
            }
            Self::EmptyOutcome => f.write_str("at least one outcome is required"),
        }
    }
}

impl std::error::Error for GraphError {}

impl Admg {
    pub fn new(
        nodes: impl IntoIterator<Item = Variable>,
        directed: impl IntoIterator<Item = (Variable, Variable)>,
        bidirected: impl IntoIterator<Item = (Variable, Variable)>,
    ) -> Result<Self, GraphError> {
        let mut nodes = set(nodes);
        let mut directed_set = BTreeSet::new();
        let mut bidirected_set = BTreeSet::new();
        for (source, target) in directed {
            if source == target {
                return Err(GraphError::SelfLoop(source));
            }
            nodes.insert(source.clone());
            nodes.insert(target.clone());
            directed_set.insert((source, target));
        }
        for (left, right) in bidirected {
            if left == right {
                return Err(GraphError::SelfLoop(left));
            }
            nodes.insert(left.clone());
            nodes.insert(right.clone());
            bidirected_set.insert(if left < right {
                (left, right)
            } else {
                (right, left)
            });
        }
        let graph = Self {
            nodes,
            directed: directed_set,
            bidirected: bidirected_set,
        };
        graph.topological_sort()?;
        Ok(graph)
    }

    pub fn nodes(&self) -> &BTreeSet<Variable> {
        &self.nodes
    }

    pub fn directed_edges(&self) -> &BTreeSet<(Variable, Variable)> {
        &self.directed
    }

    pub fn bidirected_edges(&self) -> &BTreeSet<(Variable, Variable)> {
        &self.bidirected
    }

    pub fn topological_sort(&self) -> Result<Vec<Variable>, GraphError> {
        let mut indegree: BTreeMap<Variable, usize> =
            self.nodes.iter().cloned().map(|v| (v, 0)).collect();
        let mut children: BTreeMap<Variable, BTreeSet<Variable>> = BTreeMap::new();
        for (source, target) in &self.directed {
            *indegree.get_mut(target).expect("edge endpoint is a node") += 1;
            children
                .entry(source.clone())
                .or_default()
                .insert(target.clone());
        }
        let mut ready: BTreeSet<Variable> = indegree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(node, _)| node.clone())
            .collect();
        let mut order = Vec::with_capacity(self.nodes.len());
        while let Some(node) = ready.pop_first() {
            order.push(node.clone());
            if let Some(next) = children.get(&node) {
                for child in next {
                    let degree = indegree.get_mut(child).expect("child is a node");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }
        if order.len() == self.nodes.len() {
            Ok(order)
        } else {
            Err(GraphError::DirectedCycle)
        }
    }

    fn subgraph(&self, keep: &BTreeSet<Variable>) -> Self {
        Self {
            nodes: self.nodes.intersection(keep).cloned().collect(),
            directed: self
                .directed
                .iter()
                .filter(|(u, v)| keep.contains(u) && keep.contains(v))
                .cloned()
                .collect(),
            bidirected: self
                .bidirected
                .iter()
                .filter(|(u, v)| keep.contains(u) && keep.contains(v))
                .cloned()
                .collect(),
        }
    }

    fn without_nodes(&self, removed: &BTreeSet<Variable>) -> Self {
        self.subgraph(&self.nodes.difference(removed).cloned().collect())
    }

    fn without_incoming(&self, targets: &BTreeSet<Variable>) -> Self {
        Self {
            nodes: self.nodes.clone(),
            directed: self
                .directed
                .iter()
                .filter(|(_, target)| !targets.contains(target))
                .cloned()
                .collect(),
            bidirected: self
                .bidirected
                .iter()
                .filter(|(u, v)| !targets.contains(u) && !targets.contains(v))
                .cloned()
                .collect(),
        }
    }

    fn without_outgoing(&self, sources: &BTreeSet<Variable>) -> Self {
        Self {
            nodes: self.nodes.clone(),
            directed: self
                .directed
                .iter()
                .filter(|(source, _)| !sources.contains(source))
                .cloned()
                .collect(),
            // Rule 2 removes directed arrows out of Z, not latent-confounding edges.
            bidirected: self.bidirected.clone(),
        }
    }

    /// Match y0's ADMG separation check: retain ancestors of the named variables,
    /// moralize directed co-parents, disorient all edge types, remove the conditions,
    /// and test undirected reachability.
    fn are_d_separated(
        &self,
        left: &Variable,
        right: &Variable,
        conditions: &BTreeSet<Variable>,
    ) -> bool {
        let named = conditions
            .iter()
            .cloned()
            .chain([left.clone(), right.clone()])
            .collect();
        let ancestors = self.ancestors_inclusive(&named);
        let graph = self.subgraph(&ancestors);
        let mut adjacency: BTreeMap<Variable, BTreeSet<Variable>> = graph
            .nodes
            .iter()
            .cloned()
            .map(|node| (node, BTreeSet::new()))
            .collect();
        let mut link = |a: &Variable, b: &Variable| {
            adjacency.entry(a.clone()).or_default().insert(b.clone());
            adjacency.entry(b.clone()).or_default().insert(a.clone());
        };
        for (source, target) in &graph.directed {
            link(source, target);
        }
        for (left, right) in &graph.bidirected {
            link(left, right);
        }
        for child in &graph.nodes {
            let parents = graph
                .directed
                .iter()
                .filter_map(|(source, target)| (target == child).then_some(source))
                .collect::<Vec<_>>();
            for left_index in 0..parents.len() {
                for right_index in left_index + 1..parents.len() {
                    link(parents[left_index], parents[right_index]);
                }
            }
        }
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([left.clone()]);
        while let Some(node) = queue.pop_front() {
            if conditions.contains(&node) || !visited.insert(node.clone()) {
                continue;
            }
            if &node == right {
                return false;
            }
            if let Some(neighbors) = adjacency.get(&node) {
                queue.extend(
                    neighbors
                        .iter()
                        .filter(|neighbor| !conditions.contains(*neighbor))
                        .cloned(),
                );
            }
        }
        true
    }

    fn ancestors_inclusive(&self, targets: &BTreeSet<Variable>) -> BTreeSet<Variable> {
        let mut ancestors = targets.clone();
        let mut queue: VecDeque<_> = targets.iter().cloned().collect();
        while let Some(node) = queue.pop_front() {
            for (source, target) in &self.directed {
                if target == &node && ancestors.insert(source.clone()) {
                    queue.push_back(source.clone());
                }
            }
        }
        ancestors
    }

    fn no_effect_on_outcomes(
        &self,
        treatments: &BTreeSet<Variable>,
        outcomes: &BTreeSet<Variable>,
    ) -> BTreeSet<Variable> {
        let ancestors = self
            .without_incoming(treatments)
            .ancestors_inclusive(outcomes);
        self.nodes
            .difference(treatments)
            .filter(|node| !ancestors.contains(*node))
            .cloned()
            .collect()
    }

    /// Bidirected connected components, including singleton observed variables.
    pub fn districts(&self) -> BTreeSet<BTreeSet<Variable>> {
        let mut remaining = self.nodes.clone();
        let mut districts = BTreeSet::new();
        while let Some(start) = remaining.pop_first() {
            let mut component = BTreeSet::from([start.clone()]);
            let mut queue = VecDeque::from([start]);
            while let Some(node) = queue.pop_front() {
                for (left, right) in &self.bidirected {
                    let neighbor = if left == &node {
                        Some(right)
                    } else if right == &node {
                        Some(left)
                    } else {
                        None
                    };
                    if let Some(neighbor) = neighbor {
                        if remaining.remove(neighbor) {
                            component.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
            districts.insert(component);
        }
        districts
    }

    fn is_single_district(&self) -> bool {
        self.districts().len() == 1
    }
}

/// A pair of C-forests witnessing that a query is not identifiable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hedge {
    pub graph_district: BTreeSet<Variable>,
    pub treatment_removed_district: BTreeSet<Variable>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentificationError {
    InvalidGraph(GraphError),
    Unidentifiable(Hedge),
    InternalInvariant(&'static str),
}

impl fmt::Display for IdentificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidGraph(error) => error.fmt(f),
            Self::Unidentifiable(_) => f.write_str("the effect is not identifiable from P(V)"),
            Self::InternalInvariant(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for IdentificationError {}

impl From<GraphError> for IdentificationError {
    fn from(value: GraphError) -> Self {
        Self::InvalidGraph(value)
    }
}

/// Identify `P(outcomes | do(treatments))` from the observational distribution `P(V)`.
pub fn identify_outcomes(
    graph: &Admg,
    treatments: impl IntoIterator<Item = Variable>,
    outcomes: impl IntoIterator<Item = Variable>,
) -> Result<Expression, IdentificationError> {
    let treatments = set(treatments);
    let outcomes = set(outcomes);
    if outcomes.is_empty() {
        return Err(GraphError::EmptyOutcome.into());
    }
    for variable in treatments.union(&outcomes) {
        if !graph.nodes.contains(variable) {
            return Err(GraphError::UnknownVariable(variable.clone()).into());
        }
    }
    if let Some(variable) = treatments.intersection(&outcomes).next() {
        return Err(GraphError::TreatmentOutcomeOverlap(variable.clone()).into());
    }
    identify_recursive(
        graph,
        &treatments,
        &outcomes,
        Expression::joint(graph.nodes.iter().cloned()),
        None,
    )
}

/// Identify a conditional interventional distribution
/// `P(outcomes | do(treatments), conditions)` from `P(V)` using IDC.
///
/// IDC first applies do-calculus rule 2 whenever an observed condition can be
/// exchanged for an action. If no further exchange is possible, it identifies the
/// joint intervention distribution for outcomes and remaining conditions with ID and
/// normalizes it over the outcomes.
pub fn identify_conditional_outcomes(
    graph: &Admg,
    treatments: impl IntoIterator<Item = Variable>,
    outcomes: impl IntoIterator<Item = Variable>,
    conditions: impl IntoIterator<Item = Variable>,
) -> Result<Expression, IdentificationError> {
    let treatments = set(treatments);
    let outcomes = set(outcomes);
    let conditions = set(conditions);
    if outcomes.is_empty() {
        return Err(GraphError::EmptyOutcome.into());
    }
    for variable in treatments.iter().chain(&outcomes).chain(&conditions) {
        if !graph.nodes.contains(variable) {
            return Err(GraphError::UnknownVariable(variable.clone()).into());
        }
    }
    if let Some(variable) = treatments
        .intersection(&outcomes)
        .chain(treatments.intersection(&conditions))
        .chain(outcomes.intersection(&conditions))
        .next()
    {
        return Err(GraphError::QueryRoleOverlap(variable.clone()).into());
    }
    if conditions.is_empty() {
        return identify_outcomes(graph, treatments, outcomes);
    }
    identify_conditional_recursive(graph, treatments, outcomes, conditions)
}

fn identify_conditional_recursive(
    graph: &Admg,
    treatments: BTreeSet<Variable>,
    outcomes: BTreeSet<Variable>,
    conditions: BTreeSet<Variable>,
) -> Result<Expression, IdentificationError> {
    for condition in &conditions {
        let remaining_conditions = conditions
            .iter()
            .filter(|candidate| *candidate != condition)
            .cloned()
            .collect::<BTreeSet<_>>();
        let separation_conditions = treatments
            .union(&remaining_conditions)
            .cloned()
            .collect::<BTreeSet<_>>();
        let reduced = graph
            .without_incoming(&treatments)
            .without_outgoing(&BTreeSet::from([condition.clone()]));
        let rule_two_applies = outcomes
            .iter()
            .all(|outcome| reduced.are_d_separated(outcome, condition, &separation_conditions));
        if rule_two_applies {
            let treatments = treatments
                .union(&BTreeSet::from([condition.clone()]))
                .cloned()
                .collect();
            return identify_conditional_recursive(
                graph,
                treatments,
                outcomes,
                remaining_conditions,
            );
        }
    }

    let joint_outcomes = outcomes
        .union(&conditions)
        .cloned()
        .collect::<BTreeSet<_>>();
    let joint = identify_outcomes(graph, treatments, joint_outcomes)?;
    Ok(Expression::normalize_marginalize(joint, outcomes))
}

fn identify_recursive(
    graph: &Admg,
    treatments: &BTreeSet<Variable>,
    outcomes: &BTreeSet<Variable>,
    estimand: Expression,
    ordering: Option<&[Variable]>,
) -> Result<Expression, IdentificationError> {
    let nodes = &graph.nodes;

    // ID line 1: an observational marginal.
    if treatments.is_empty() {
        return Ok(Expression::marginalize(
            estimand,
            nodes.difference(outcomes).cloned(),
        ));
    }

    // ID line 2: variables outside An(Y) can be marginalized before recursing.
    let ancestors = graph.ancestors_inclusive(outcomes);
    let outside_ancestors: BTreeSet<_> = nodes.difference(&ancestors).cloned().collect();
    if !outside_ancestors.is_empty() {
        return identify_recursive(
            &graph.subgraph(&ancestors),
            &treatments.intersection(&ancestors).cloned().collect(),
            outcomes,
            Expression::marginalize(estimand, outside_ancestors),
            ordering,
        );
    }

    // ID line 3: intervene on variables that cannot affect Y after do(X).
    let no_effect = graph.no_effect_on_outcomes(treatments, outcomes);
    if !no_effect.is_empty() {
        return identify_recursive(
            graph,
            &treatments.union(&no_effect).cloned().collect(),
            outcomes,
            estimand,
            ordering,
        );
    }

    let graph_without_treatments = graph.without_nodes(treatments);
    let districts_without_treatments = graph_without_treatments.districts();

    // ID line 4: c-component factorization.
    if districts_without_treatments.len() > 1 {
        let mut factors = Vec::with_capacity(districts_without_treatments.len());
        for district in districts_without_treatments {
            factors.push(identify_recursive(
                graph,
                &nodes.difference(&district).cloned().collect(),
                &district,
                estimand.clone(),
                ordering,
            )?);
        }
        return Ok(Expression::marginalize(
            Expression::product(factors),
            nodes
                .difference(&outcomes.union(treatments).cloned().collect())
                .cloned(),
        ));
    }

    let treatment_removed_district = districts_without_treatments.iter().next().cloned().ok_or(
        IdentificationError::InternalInvariant("G\\X must contain an outcome district"),
    )?;

    // ID line 5: failure with a hedge witness.
    if graph.is_single_district() {
        return Err(IdentificationError::Unidentifiable(Hedge {
            graph_district: nodes.clone(),
            treatment_removed_district,
        }));
    }

    // ID line 6: the remaining district is already a district of G.
    let graph_districts = graph.districts();
    if graph_districts.contains(&treatment_removed_district) {
        return Ok(Expression::marginalize(
            district_product(
                &treatment_removed_district,
                ordering.unwrap_or(&graph.topological_sort()?),
            ),
            treatment_removed_district.difference(outcomes).cloned(),
        ));
    }

    // ID line 7: recurse inside the unique larger district containing S.
    if let Some(district) = graph_districts
        .iter()
        .find(|district| treatment_removed_district.is_subset(district))
    {
        let owned_ordering;
        let ordering = if let Some(ordering) = ordering {
            ordering
        } else {
            owned_ordering = graph.topological_sort()?;
            &owned_ordering
        };
        return identify_recursive(
            &graph.subgraph(district),
            &treatments.intersection(district).cloned().collect(),
            outcomes,
            district_product(district, ordering),
            Some(ordering),
        );
    }

    Err(IdentificationError::InternalInvariant(
        "ID line 7 could not find a containing district",
    ))
}

fn district_product(district: &BTreeSet<Variable>, ordering: &[Variable]) -> Expression {
    Expression::product(district.iter().map(|child| {
        let position = ordering
            .iter()
            .position(|node| node == child)
            .expect("district node occurs in topological order");
        Expression::conditional(child.clone(), ordering[..position].to_vec())
    }))
}
