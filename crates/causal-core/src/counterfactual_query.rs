//! Typed binary counterfactual query/event language and symbolic level-3 expressions.
//!
//! This module mirrors the event language used by y0's ID*/IDC* implementation. A
//! counterfactual random variable records its base variable and the intervention world in
//! which it is evaluated. An event separately assigns a binary value to that random
//! variable. Keeping those concepts separate prevents an intervention such as `do(X=0)`
//! from being confused with the event `X=0`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::do_calculus::Variable;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryValue {
    Zero,
    One,
}

impl BinaryValue {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }

    fn sign(self) -> char {
        match self {
            Self::Zero => '-',
            Self::One => '+',
        }
    }
}

impl From<bool> for BinaryValue {
    fn from(value: bool) -> Self {
        if value {
            Self::One
        } else {
            Self::Zero
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Intervention {
    pub variable: Variable,
    pub value: BinaryValue,
}

impl Intervention {
    pub fn new(variable: impl Into<Variable>, value: BinaryValue) -> Result<Self, QueryError> {
        let variable = variable.into();
        validate_name(&variable)?;
        Ok(Self { variable, value })
    }

    pub fn to_y0(&self) -> String {
        format!("{}{}", self.value.sign(), self.variable)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CounterfactualVariable {
    pub variable: Variable,
    pub interventions: BTreeSet<Intervention>,
}

impl CounterfactualVariable {
    pub fn factual(variable: impl Into<Variable>) -> Result<Self, QueryError> {
        let variable = variable.into();
        validate_name(&variable)?;
        Ok(Self {
            variable,
            interventions: BTreeSet::new(),
        })
    }

    pub fn in_world(
        variable: impl Into<Variable>,
        interventions: impl IntoIterator<Item = Intervention>,
    ) -> Result<Self, QueryError> {
        let variable = variable.into();
        validate_name(&variable)?;
        let interventions = collect_interventions(interventions)?;
        if interventions.is_empty() {
            return Err(QueryError::EmptyInterventionWorld);
        }
        Ok(Self {
            variable,
            interventions,
        })
    }

    pub fn base(&self) -> &str {
        &self.variable
    }

    pub fn is_factual(&self) -> bool {
        self.interventions.is_empty()
    }

    pub fn intervene(
        &self,
        interventions: impl IntoIterator<Item = Intervention>,
    ) -> Result<Self, QueryError> {
        let additions = collect_interventions(interventions)?;
        let combined = self
            .interventions
            .iter()
            .cloned()
            .chain(additions)
            .collect::<Vec<_>>();
        Ok(Self {
            variable: self.variable.clone(),
            interventions: collect_interventions(combined)?,
        })
    }

    pub fn to_y0(&self) -> String {
        match self.interventions.len() {
            0 => self.variable.clone(),
            1 => format!(
                "{} @ {}",
                self.variable,
                self.interventions.iter().next().unwrap().to_y0()
            ),
            _ => format!(
                "{} @ ({})",
                self.variable,
                self.interventions
                    .iter()
                    .map(Intervention::to_y0)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventAtom {
    pub variable: CounterfactualVariable,
    pub value: BinaryValue,
}

impl EventAtom {
    pub fn new(variable: CounterfactualVariable, value: BinaryValue) -> Self {
        Self { variable, value }
    }

    pub fn to_y0(&self) -> String {
        format!(
            "{}: {}{}",
            self.variable.to_y0(),
            self.value.sign(),
            self.variable.variable
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Event(BTreeMap<CounterfactualVariable, BinaryValue>);

impl Event {
    pub fn new(atoms: impl IntoIterator<Item = EventAtom>) -> Result<Self, QueryError> {
        let mut values = BTreeMap::new();
        for atom in atoms {
            if let Some(previous) = values.insert(atom.variable.clone(), atom.value) {
                if previous != atom.value {
                    return Err(QueryError::ConflictingEventValue(atom.variable));
                }
            }
        }
        Ok(Self(values))
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CounterfactualVariable, &BinaryValue)> {
        self.0.iter()
    }

    pub fn get(&self, variable: &CounterfactualVariable) -> Option<BinaryValue> {
        self.0.get(variable).copied()
    }

    pub fn contains_key(&self, variable: &CounterfactualVariable) -> bool {
        self.0.contains_key(variable)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_y0(&self) -> String {
        self.iter()
            .map(|(variable, value)| EventAtom::new(variable.clone(), *value).to_y0())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(crate) fn from_map(values: BTreeMap<CounterfactualVariable, BinaryValue>) -> Self {
        Self(values)
    }

    pub(crate) fn into_map(self) -> BTreeMap<CounterfactualVariable, BinaryValue> {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryError {
    EmptyVariable,
    EmptyInterventionWorld,
    ConflictingIntervention {
        variable: Variable,
        first: BinaryValue,
        second: BinaryValue,
    },
    ConflictingEventValue(CounterfactualVariable),
    OverlappingOutcomeAndCondition(CounterfactualVariable),
}

impl fmt::Display for QueryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariable => formatter.write_str("a variable name cannot be empty"),
            Self::EmptyInterventionWorld => {
                formatter.write_str("a counterfactual world needs at least one intervention")
            }
            Self::ConflictingIntervention {
                variable,
                first,
                second,
            } => write!(
                formatter,
                "{variable} cannot be assigned both {} and {} in one intervention world",
                first.as_u8(),
                second.as_u8()
            ),
            Self::ConflictingEventValue(variable) => write!(
                formatter,
                "{} cannot have two values in one event",
                variable.to_y0()
            ),
            Self::OverlappingOutcomeAndCondition(variable) => write!(
                formatter,
                "{} cannot be both an outcome and a condition",
                variable.to_y0()
            ),
        }
    }
}

impl std::error::Error for QueryError {}

fn validate_name(variable: &str) -> Result<(), QueryError> {
    if variable.trim().is_empty() {
        Err(QueryError::EmptyVariable)
    } else {
        Ok(())
    }
}

fn collect_interventions(
    interventions: impl IntoIterator<Item = Intervention>,
) -> Result<BTreeSet<Intervention>, QueryError> {
    let mut by_variable = BTreeMap::new();
    for intervention in interventions {
        if let Some(previous) =
            by_variable.insert(intervention.variable.clone(), intervention.value)
        {
            if previous != intervention.value {
                return Err(QueryError::ConflictingIntervention {
                    variable: intervention.variable,
                    first: previous,
                    second: intervention.value,
                });
            }
        }
    }
    Ok(by_variable
        .into_iter()
        .map(|(variable, value)| Intervention { variable, value })
        .collect())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConditionalCounterfactualQuery {
    pub outcomes: Event,
    pub conditions: Event,
}

impl ConditionalCounterfactualQuery {
    pub fn new(outcomes: Event, conditions: Event) -> Result<Self, QueryError> {
        if let Some((variable, _)) = outcomes
            .iter()
            .find(|(variable, _)| conditions.contains_key(variable))
        {
            return Err(QueryError::OverlappingOutcomeAndCondition(
                (*variable).clone(),
            ));
        }
        Ok(Self {
            outcomes,
            conditions,
        })
    }

    /// Canonical query plus the identifying functional. The expression alone can be identical
    /// for two assignments, so the event values must remain part of the serialized record.
    pub fn identified_to_y0(&self, expression: &CounterfactualExpression) -> String {
        format!(
            "P({} | {}) = {}",
            self.outcomes.to_y0(),
            self.conditions.to_y0(),
            expression.to_y0()
        )
    }
}

/// A symbolic distribution returned by ID* or IDC*.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CounterfactualExpression {
    One,
    Zero,
    Probability(BTreeSet<CounterfactualVariable>),
    Product(Vec<CounterfactualExpression>),
    Sum {
        ranges: BTreeSet<Variable>,
        expression: Box<CounterfactualExpression>,
    },
    Ratio {
        numerator: Box<CounterfactualExpression>,
        denominator: Box<CounterfactualExpression>,
    },
}

impl CounterfactualExpression {
    pub fn probability(variables: impl IntoIterator<Item = CounterfactualVariable>) -> Self {
        Self::Probability(variables.into_iter().collect())
    }

    pub fn product(expressions: impl IntoIterator<Item = Self>) -> Self {
        let mut factors = Vec::new();
        for expression in expressions {
            match expression {
                Self::One => {}
                Self::Zero => return Self::Zero,
                Self::Product(parts) => factors.extend(parts),
                other => factors.push(other),
            }
        }
        factors.sort_by_key(Self::sort_key);
        match factors.len() {
            0 => Self::One,
            1 => factors.pop().unwrap(),
            _ => Self::Product(factors),
        }
    }

    pub fn marginalize(expression: Self, ranges: impl IntoIterator<Item = Variable>) -> Self {
        let ranges = ranges.into_iter().collect::<BTreeSet<_>>();
        if ranges.is_empty() || matches!(expression, Self::Zero | Self::One) {
            expression
        } else {
            Self::Sum {
                ranges,
                expression: Box::new(expression),
            }
        }
    }

    pub fn normalize_marginalize(
        expression: Self,
        ranges: impl IntoIterator<Item = Variable>,
    ) -> Self {
        let denominator = Self::marginalize(expression.clone(), ranges);
        Self::Ratio {
            numerator: Box::new(expression),
            denominator: Box::new(denominator),
        }
    }

    pub fn to_y0(&self) -> String {
        match self {
            Self::One => "One()".to_owned(),
            Self::Zero => "Zero()".to_owned(),
            Self::Probability(variables) => {
                let worlds = variables
                    .iter()
                    .map(|variable| &variable.interventions)
                    .collect::<BTreeSet<_>>();
                if worlds.len() == 1 {
                    let world = *worlds.iter().next().unwrap();
                    if !world.is_empty() {
                        let interventions = world
                            .iter()
                            .map(|intervention| match intervention.value {
                                BinaryValue::Zero => intervention.variable.clone(),
                                BinaryValue::One => format!("+{}", intervention.variable),
                            })
                            .collect::<Vec<_>>()
                            .join(",");
                        let bases = variables
                            .iter()
                            .map(|variable| variable.variable.clone())
                            .collect::<Vec<_>>()
                            .join(", ");
                        return format!("P[{interventions}]({bases})");
                    }
                }
                format!(
                    "P({})",
                    variables
                        .iter()
                        .map(CounterfactualVariable::to_y0)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Product(factors) => factors
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

    fn sort_key(&self) -> String {
        match self {
            Self::Probability(variables) => format!(
                "0:{}",
                variables
                    .iter()
                    .map(|variable| variable.variable.clone())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Sum { ranges, expression } => format!(
                "1:{}:{}",
                ranges.iter().cloned().collect::<Vec<_>>().join(","),
                expression.sort_key()
            ),
            Self::Product(factors) => format!(
                "2:{}",
                factors
                    .iter()
                    .map(Self::sort_key)
                    .collect::<Vec<_>>()
                    .join(":")
            ),
            Self::Ratio {
                numerator,
                denominator,
            } => format!("3:{}:{}", numerator.sort_key(), denominator.sort_key()),
            Self::One => "4".to_owned(),
            Self::Zero => "5".to_owned(),
        }
    }

    pub(crate) fn base_variables(&self) -> BTreeSet<Variable> {
        match self {
            Self::One | Self::Zero => BTreeSet::new(),
            Self::Probability(variables) => variables
                .iter()
                .flat_map(|variable| {
                    [variable.variable.clone()].into_iter().chain(
                        variable
                            .interventions
                            .iter()
                            .map(|intervention| intervention.variable.clone()),
                    )
                })
                .collect(),
            Self::Product(factors) => factors.iter().flat_map(Self::base_variables).collect(),
            Self::Sum { ranges, expression } => {
                expression.base_variables().union(ranges).cloned().collect()
            }
            Self::Ratio {
                numerator,
                denominator,
            } => numerator
                .base_variables()
                .union(&denominator.base_variables())
                .cloned()
                .collect(),
        }
    }

    /// Match y0's dynamic `conditional()` dispatch. A bare Probability excludes its
    /// intervention subscripts when finding normalization ranges; composite expressions
    /// use the base `Expression` behavior and include every iterated variable.
    pub(crate) fn conditioning_variables(&self) -> BTreeSet<Variable> {
        match self {
            Self::Probability(variables) => variables
                .iter()
                .map(|variable| variable.variable.clone())
                .collect(),
            _ => self.base_variables(),
        }
    }

    pub(crate) fn free_variables(&self) -> BTreeSet<Variable> {
        match self {
            Self::One | Self::Zero => BTreeSet::new(),
            Self::Probability(variables) => variables
                .iter()
                .flat_map(|variable| {
                    [variable.variable.clone()].into_iter().chain(
                        variable
                            .interventions
                            .iter()
                            .map(|intervention| intervention.variable.clone()),
                    )
                })
                .collect(),
            Self::Product(factors) => factors.iter().flat_map(Self::free_variables).collect(),
            Self::Sum { ranges, expression } => expression
                .free_variables()
                .difference(ranges)
                .cloned()
                .collect(),
            Self::Ratio {
                numerator,
                denominator,
            } => numerator
                .free_variables()
                .union(&denominator.free_variables())
                .cloned()
                .collect(),
        }
    }
}
