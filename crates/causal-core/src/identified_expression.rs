//! Numerical evaluation of observational expressions returned by ID and IDC.
//!
//! Identification and estimation remain separate: `do_calculus` derives a symbolic
//! expression, while this module evaluates that expression against an explicitly supplied
//! discrete observational distribution. No graph factorization is assumed, which is
//! important when the observed graph contains latent confounding.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::do_calculus::{Expression, Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiscreteTableError {
    EmptyTable,
    EmptyVariable,
    UnequalColumnLengths {
        variable: Variable,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for DiscreteTableError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTable => formatter.write_str("a discrete observational table needs rows"),
            Self::EmptyVariable => formatter.write_str("a discrete variable name cannot be empty"),
            Self::UnequalColumnLengths {
                variable,
                expected,
                actual,
            } => write!(
                formatter,
                "column {variable} has {actual} rows; expected {expected}"
            ),
        }
    }
}

impl std::error::Error for DiscreteTableError {}

/// An observed discrete joint distribution represented by equal-length columns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscreteTable {
    columns: BTreeMap<Variable, Vec<String>>,
    states: BTreeMap<Variable, Vec<String>>,
    observations: usize,
}

impl DiscreteTable {
    pub fn from_columns(
        columns: BTreeMap<Variable, Vec<String>>,
    ) -> Result<Self, DiscreteTableError> {
        let observations = columns.values().next().map_or(0, Vec::len);
        if columns.is_empty() || observations == 0 {
            return Err(DiscreteTableError::EmptyTable);
        }
        if columns.keys().any(|variable| variable.trim().is_empty()) {
            return Err(DiscreteTableError::EmptyVariable);
        }
        for (variable, values) in &columns {
            if values.len() != observations {
                return Err(DiscreteTableError::UnequalColumnLengths {
                    variable: variable.clone(),
                    expected: observations,
                    actual: values.len(),
                });
            }
        }
        let states = columns
            .iter()
            .map(|(variable, values)| {
                (
                    variable.clone(),
                    values
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .collect(),
                )
            })
            .collect();
        Ok(Self {
            columns,
            states,
            observations,
        })
    }

    pub fn observations(&self) -> usize {
        self.observations
    }

    pub fn states(&self, variable: &str) -> Option<&[String]> {
        self.states.get(variable).map(Vec::as_slice)
    }

    fn row_matches(
        &self,
        row: usize,
        variables: impl IntoIterator<Item = Variable>,
        assignment: &BTreeMap<Variable, String>,
    ) -> bool {
        variables
            .into_iter()
            .all(|variable| self.columns[&variable][row] == assignment[&variable])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluationError {
    UnknownVariable(Variable),
    UnknownState { variable: Variable, state: String },
    MissingAssignment(Variable),
    OutcomeFixed(Variable),
    EmptyOutcome,
    ZeroConditioningMass { parents: Vec<Variable> },
    ZeroNormalization,
    NonFinite,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownVariable(variable) => write!(formatter, "unknown variable {variable}"),
            Self::UnknownState { variable, state } => {
                write!(formatter, "unknown state {state} for {variable}")
            }
            Self::MissingAssignment(variable) => {
                write!(formatter, "no state was assigned to {variable}")
            }
            Self::OutcomeFixed(variable) => {
                write!(formatter, "outcome {variable} is already fixed")
            }
            Self::EmptyOutcome => formatter.write_str("at least one outcome is required"),
            Self::ZeroConditioningMass { parents } => write!(
                formatter,
                "the conditioning assignment has zero probability for {}",
                parents.join(", ")
            ),
            Self::ZeroNormalization => {
                formatter.write_str("the identified expression has zero normalization")
            }
            Self::NonFinite => formatter.write_str("the identified expression is not finite"),
        }
    }
}

impl std::error::Error for EvaluationError {}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluatedDistribution {
    pub outcomes: Vec<Variable>,
    pub probabilities: BTreeMap<Vec<String>, f64>,
    pub total: f64,
}

fn validate_assignment(
    table: &DiscreteTable,
    assignment: &BTreeMap<Variable, String>,
) -> Result<(), EvaluationError> {
    for (variable, state) in assignment {
        let states = table
            .states(variable)
            .ok_or_else(|| EvaluationError::UnknownVariable(variable.clone()))?;
        if !states.contains(state) {
            return Err(EvaluationError::UnknownState {
                variable: variable.clone(),
                state: state.clone(),
            });
        }
    }
    Ok(())
}

/// Evaluate one identified expression at a complete assignment of its free variables.
pub fn evaluate_expression(
    expression: &Expression,
    table: &DiscreteTable,
    assignment: &BTreeMap<Variable, String>,
) -> Result<f64, EvaluationError> {
    validate_assignment(table, assignment)?;
    evaluate_recursive(expression, table, assignment)
}

fn evaluate_recursive(
    expression: &Expression,
    table: &DiscreteTable,
    assignment: &BTreeMap<Variable, String>,
) -> Result<f64, EvaluationError> {
    let value = match expression {
        Expression::Probability { children, parents } => {
            for variable in children.iter().chain(parents) {
                if table.states(variable).is_none() {
                    return Err(EvaluationError::UnknownVariable(variable.clone()));
                }
                if !assignment.contains_key(variable) {
                    return Err(EvaluationError::MissingAssignment(variable.clone()));
                }
            }
            let parent_set = parents.iter().cloned().collect::<BTreeSet<_>>();
            let denominator = if parents.is_empty() {
                table.observations
            } else {
                (0..table.observations)
                    .filter(|row| table.row_matches(*row, parent_set.iter().cloned(), assignment))
                    .count()
            };
            if denominator == 0 {
                return Err(EvaluationError::ZeroConditioningMass {
                    parents: parents.clone(),
                });
            }
            let scope = children
                .iter()
                .cloned()
                .chain(parents.iter().cloned())
                .collect::<BTreeSet<_>>();
            let numerator = (0..table.observations)
                .filter(|row| table.row_matches(*row, scope.iter().cloned(), assignment))
                .count();
            numerator as f64 / denominator as f64
        }
        Expression::Product(factors) => {
            let mut product = 1.0;
            for factor in factors {
                product *= evaluate_recursive(factor, table, assignment)?;
            }
            product
        }
        Expression::Sum { ranges, expression } => {
            fn sum_ranges(
                table: &DiscreteTable,
                expression: &Expression,
                ranges: &[Variable],
                position: usize,
                assignment: &mut BTreeMap<Variable, String>,
            ) -> Result<f64, EvaluationError> {
                if position == ranges.len() {
                    return evaluate_recursive(expression, table, assignment);
                }
                let variable = &ranges[position];
                let states = table
                    .states(variable)
                    .ok_or_else(|| EvaluationError::UnknownVariable(variable.clone()))?
                    .to_vec();
                let previous = assignment.get(variable).cloned();
                let mut total = 0.0;
                for state in states {
                    assignment.insert(variable.clone(), state);
                    total += sum_ranges(table, expression, ranges, position + 1, assignment)?;
                }
                if let Some(previous) = previous {
                    assignment.insert(variable.clone(), previous);
                } else {
                    assignment.remove(variable);
                }
                Ok(total)
            }

            let mut assignment = assignment.clone();
            let ranges = ranges.iter().cloned().collect::<Vec<_>>();
            sum_ranges(table, expression, &ranges, 0, &mut assignment)?
        }
        Expression::Ratio {
            numerator,
            denominator,
        } => {
            let denominator = evaluate_recursive(denominator, table, assignment)?;
            if denominator == 0.0 {
                return Err(EvaluationError::ZeroNormalization);
            }
            evaluate_recursive(numerator, table, assignment)? / denominator
        }
    };
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EvaluationError::NonFinite)
    }
}

/// Evaluate the complete distribution over one or more outcomes at fixed treatment and
/// condition assignments. The returned total is retained so callers can audit numerical
/// normalization instead of silently correcting it.
pub fn evaluate_distribution(
    expression: &Expression,
    table: &DiscreteTable,
    outcomes: impl IntoIterator<Item = Variable>,
    fixed: &BTreeMap<Variable, String>,
) -> Result<EvaluatedDistribution, EvaluationError> {
    validate_assignment(table, fixed)?;
    let outcomes = outcomes.into_iter().collect::<Vec<_>>();
    if outcomes.is_empty() {
        return Err(EvaluationError::EmptyOutcome);
    }
    for outcome in &outcomes {
        if fixed.contains_key(outcome) {
            return Err(EvaluationError::OutcomeFixed(outcome.clone()));
        }
        if table.states(outcome).is_none() {
            return Err(EvaluationError::UnknownVariable(outcome.clone()));
        }
    }

    fn enumerate(
        expression: &Expression,
        table: &DiscreteTable,
        outcomes: &[Variable],
        position: usize,
        assignment: &mut BTreeMap<Variable, String>,
        probabilities: &mut BTreeMap<Vec<String>, f64>,
    ) -> Result<(), EvaluationError> {
        if position == outcomes.len() {
            let key = outcomes
                .iter()
                .map(|outcome| assignment[outcome].clone())
                .collect();
            probabilities.insert(key, evaluate_recursive(expression, table, assignment)?);
            return Ok(());
        }
        let outcome = &outcomes[position];
        let states = table
            .states(outcome)
            .ok_or_else(|| EvaluationError::UnknownVariable(outcome.clone()))?
            .to_vec();
        for state in states {
            assignment.insert(outcome.clone(), state);
            enumerate(
                expression,
                table,
                outcomes,
                position + 1,
                assignment,
                probabilities,
            )?;
        }
        assignment.remove(outcome);
        Ok(())
    }

    let mut assignment = fixed.clone();
    let mut probabilities = BTreeMap::new();
    enumerate(
        expression,
        table,
        &outcomes,
        0,
        &mut assignment,
        &mut probabilities,
    )?;
    let total = probabilities.values().sum();
    Ok(EvaluatedDistribution {
        outcomes,
        probabilities,
        total,
    })
}
