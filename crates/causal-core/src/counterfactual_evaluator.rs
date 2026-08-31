//! Evaluation of identified binary counterfactual expressions from observational data.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::counterfactual_query::{
    BinaryValue, ConditionalCounterfactualQuery, CounterfactualExpression, CounterfactualVariable,
    Event, EventAtom, Intervention, QueryError,
};
use crate::do_calculus::{identify_outcomes, Admg, IdentificationError, Variable};
use crate::idc_star::{identify_conditional_counterfactual, IdcStarError};
use crate::identified_expression::{evaluate_expression, DiscreteTable, EvaluationError};

#[derive(Clone, Debug, PartialEq)]
pub enum CounterfactualEvaluationError {
    Query(QueryError),
    Identification(IdcStarError),
    LevelTwoIdentification(IdentificationError),
    Evaluation(EvaluationError),
    MixedInterventionWorlds,
    ConflictingBaseValues {
        variable: Variable,
        first: BinaryValue,
        second: BinaryValue,
    },
    MissingEventValue(Variable),
    ZeroDenominator,
    NonFinite,
}

impl fmt::Display for CounterfactualEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Query(error) => error.fmt(formatter),
            Self::Identification(error) => error.fmt(formatter),
            Self::LevelTwoIdentification(error) => error.fmt(formatter),
            Self::Evaluation(error) => error.fmt(formatter),
            Self::MixedInterventionWorlds => formatter.write_str(
                "one probability factor contains variables from different intervention worlds",
            ),
            Self::ConflictingBaseValues {
                variable,
                first,
                second,
            } => write!(
                formatter,
                "this evaluator cannot collapse {variable}={} and {variable}={} into one base assignment",
                first.as_u8(),
                second.as_u8()
            ),
            Self::MissingEventValue(variable) => {
                write!(formatter, "no event or summation value is available for {variable}")
            }
            Self::ZeroDenominator => {
                formatter.write_str("the conditional counterfactual has zero denominator")
            }
            Self::NonFinite => formatter.write_str("the counterfactual result is not finite"),
        }
    }
}

impl std::error::Error for CounterfactualEvaluationError {}

impl From<QueryError> for CounterfactualEvaluationError {
    fn from(value: QueryError) -> Self {
        Self::Query(value)
    }
}

impl From<IdcStarError> for CounterfactualEvaluationError {
    fn from(value: IdcStarError) -> Self {
        Self::Identification(value)
    }
}

impl From<IdentificationError> for CounterfactualEvaluationError {
    fn from(value: IdentificationError) -> Self {
        Self::LevelTwoIdentification(value)
    }
}

impl From<EvaluationError> for CounterfactualEvaluationError {
    fn from(value: EvaluationError) -> Self {
        Self::Evaluation(value)
    }
}

fn base_event_values(
    event: &Event,
) -> Result<BTreeMap<Variable, BinaryValue>, CounterfactualEvaluationError> {
    let mut values = BTreeMap::new();
    for (variable, value) in event.iter() {
        if let Some(previous) = values.insert(variable.variable.clone(), *value) {
            if previous != *value {
                return Err(CounterfactualEvaluationError::ConflictingBaseValues {
                    variable: variable.variable.clone(),
                    first: previous,
                    second: *value,
                });
            }
        }
    }
    Ok(values)
}

fn designated_intervention_values(
    event: &Event,
) -> Result<BTreeMap<Variable, BinaryValue>, CounterfactualEvaluationError> {
    let mut values = BTreeMap::new();
    for (variable, _) in event.iter() {
        for intervention in &variable.interventions {
            if let Some(previous) = values.insert(intervention.variable.clone(), intervention.value)
            {
                if previous != intervention.value {
                    return Err(CounterfactualEvaluationError::ConflictingBaseValues {
                        variable: intervention.variable.clone(),
                        first: previous,
                        second: intervention.value,
                    });
                }
            }
        }
    }
    Ok(values)
}

fn evaluate_recursive(
    graph: &Admg,
    expression: &CounterfactualExpression,
    table: &DiscreteTable,
    assignment: &mut BTreeMap<Variable, String>,
    designated_interventions: &BTreeMap<Variable, BinaryValue>,
) -> Result<f64, CounterfactualEvaluationError> {
    let value = match expression {
        CounterfactualExpression::One => 1.0,
        CounterfactualExpression::Zero => 0.0,
        CounterfactualExpression::Probability(variables) => {
            let worlds = variables
                .iter()
                .map(|variable| &variable.interventions)
                .collect::<BTreeSet<_>>();
            if worlds.len() != 1 {
                return Err(CounterfactualEvaluationError::MixedInterventionWorlds);
            }
            let world = *worlds.iter().next().unwrap();
            let treatments = world
                .iter()
                .map(|intervention| intervention.variable.clone())
                .collect::<BTreeSet<_>>();
            let outcomes = variables
                .iter()
                .map(|variable| variable.variable.clone())
                .collect::<BTreeSet<_>>();
            let identified = identify_outcomes(graph, treatments, outcomes.clone())?;
            let mut leaf_assignment = assignment.clone();
            for intervention in world {
                if let Some(value) = designated_interventions.get(&intervention.variable) {
                    leaf_assignment
                        .insert(intervention.variable.clone(), value.as_u8().to_string());
                } else if !leaf_assignment.contains_key(&intervention.variable) {
                    leaf_assignment.insert(
                        intervention.variable.clone(),
                        intervention.value.as_u8().to_string(),
                    );
                }
            }
            for outcome in outcomes {
                if !leaf_assignment.contains_key(&outcome) {
                    return Err(CounterfactualEvaluationError::MissingEventValue(outcome));
                }
            }
            evaluate_expression(&identified, table, &leaf_assignment)?
        }
        CounterfactualExpression::Product(factors) => {
            let mut product = 1.0;
            for factor in factors {
                product *=
                    evaluate_recursive(graph, factor, table, assignment, designated_interventions)?;
            }
            product
        }
        CounterfactualExpression::Sum { ranges, expression } => {
            fn sum_ranges(
                graph: &Admg,
                expression: &CounterfactualExpression,
                table: &DiscreteTable,
                ranges: &[Variable],
                position: usize,
                assignment: &mut BTreeMap<Variable, String>,
                designated_interventions: &BTreeMap<Variable, BinaryValue>,
            ) -> Result<f64, CounterfactualEvaluationError> {
                if position == ranges.len() {
                    return evaluate_recursive(
                        graph,
                        expression,
                        table,
                        assignment,
                        designated_interventions,
                    );
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
                    total += sum_ranges(
                        graph,
                        expression,
                        table,
                        ranges,
                        position + 1,
                        assignment,
                        designated_interventions,
                    )?;
                }
                if let Some(previous) = previous {
                    assignment.insert(variable.clone(), previous);
                } else {
                    assignment.remove(variable);
                }
                Ok(total)
            }
            // y0's unsimplified IDC* output can repeat a variable in an outer sum
            // after an inner sum has already bound it. Rebinding that name would
            // multiply the denominator by its state count. Evaluate only genuinely
            // free ranges, which is the algebraic meaning of marginalization.
            let free = expression.free_variables();
            let ranges = ranges.intersection(&free).cloned().collect::<Vec<_>>();
            sum_ranges(
                graph,
                expression,
                table,
                &ranges,
                0,
                assignment,
                designated_interventions,
            )?
        }
        CounterfactualExpression::Ratio {
            numerator,
            denominator,
        } => {
            let denominator = evaluate_recursive(
                graph,
                denominator,
                table,
                assignment,
                designated_interventions,
            )?;
            if denominator == 0.0 {
                return Err(CounterfactualEvaluationError::ZeroDenominator);
            }
            evaluate_recursive(
                graph,
                numerator,
                table,
                assignment,
                designated_interventions,
            )? / denominator
        }
    };
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CounterfactualEvaluationError::NonFinite)
    }
}

/// Evaluate an ID*/IDC* result. This first identifies every level-2 probability factor
/// from `P(V)` with ordinary ID, then evaluates those factors against the observed table.
pub fn evaluate_counterfactual_expression(
    graph: &Admg,
    expression: &CounterfactualExpression,
    event: &Event,
    table: &DiscreteTable,
) -> Result<f64, CounterfactualEvaluationError> {
    let mut assignment = base_event_values(event)?
        .into_iter()
        .map(|(variable, value)| (variable, value.as_u8().to_string()))
        .collect();
    let designated_interventions = designated_intervention_values(event)?;
    evaluate_recursive(
        graph,
        expression,
        table,
        &mut assignment,
        &designated_interventions,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryEtt {
    pub treated_potential_outcome_mean: f64,
    pub untreated_potential_outcome_mean: f64,
    pub effect_on_treated: f64,
    pub treated_query: ConditionalCounterfactualQuery,
    pub untreated_query: ConditionalCounterfactualQuery,
    pub treated_expression: CounterfactualExpression,
    pub untreated_expression: CounterfactualExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryEttIdentification {
    pub treated_query: ConditionalCounterfactualQuery,
    pub untreated_query: ConditionalCounterfactualQuery,
    pub treated_expression: CounterfactualExpression,
    pub untreated_expression: CounterfactualExpression,
}

fn intervention(name: &str, value: BinaryValue) -> Result<Intervention, QueryError> {
    Intervention::new(name, value)
}

fn potential_outcome_event(
    outcome: &str,
    treatment: &str,
    treatment_value: BinaryValue,
) -> Result<Event, QueryError> {
    Event::new([EventAtom::new(
        CounterfactualVariable::in_world(outcome, [intervention(treatment, treatment_value)?])?,
        BinaryValue::One,
    )])
}

/// Identify the two conditional potential-outcome distributions needed for binary ETT:
/// `P(Y(1) | X=1)` and `P(Y(0) | X=1)`.
pub fn identify_binary_ett(
    graph: &Admg,
    treatment: &str,
    outcome: &str,
) -> Result<BinaryEttIdentification, IdcStarError> {
    let conditions = Event::new([EventAtom::new(
        CounterfactualVariable::factual(treatment)?,
        BinaryValue::One,
    )])?;
    let treated_outcome = potential_outcome_event(outcome, treatment, BinaryValue::One)?;
    let untreated_outcome = potential_outcome_event(outcome, treatment, BinaryValue::Zero)?;
    let treated_query = ConditionalCounterfactualQuery::new(treated_outcome, conditions.clone())?;
    let untreated_query = ConditionalCounterfactualQuery::new(untreated_outcome, conditions)?;
    let treated_expression = identify_conditional_counterfactual(graph, &treated_query)?;
    let untreated_expression = identify_conditional_counterfactual(graph, &untreated_query)?;
    Ok(BinaryEttIdentification {
        treated_query,
        untreated_query,
        treated_expression,
        untreated_expression,
    })
}

/// Estimate the binary effect of treatment on the treated,
/// `E[Y(1) - Y(0) | X=1]`.
pub fn estimate_binary_ett(
    graph: &Admg,
    table: &DiscreteTable,
    treatment: &str,
    outcome: &str,
) -> Result<BinaryEtt, CounterfactualEvaluationError> {
    let identification = identify_binary_ett(graph, treatment, outcome)?;
    let conditions = Event::new([EventAtom::new(
        CounterfactualVariable::factual(treatment)?,
        BinaryValue::One,
    )])?;
    let treated_outcome = potential_outcome_event(outcome, treatment, BinaryValue::One)?;
    let untreated_outcome = potential_outcome_event(outcome, treatment, BinaryValue::Zero)?;
    let treated_event = Event::new(
        treated_outcome
            .iter()
            .chain(conditions.iter())
            .map(|(variable, value)| EventAtom::new(variable.clone(), *value)),
    )?;
    let untreated_event = Event::new(
        untreated_outcome
            .iter()
            .chain(conditions.iter())
            .map(|(variable, value)| EventAtom::new(variable.clone(), *value)),
    )?;
    let treated_potential_outcome_mean = evaluate_counterfactual_expression(
        graph,
        &identification.treated_expression,
        &treated_event,
        table,
    )?;
    let untreated_potential_outcome_mean = evaluate_counterfactual_expression(
        graph,
        &identification.untreated_expression,
        &untreated_event,
        table,
    )?;
    Ok(BinaryEtt {
        treated_potential_outcome_mean,
        untreated_potential_outcome_mean,
        effect_on_treated: treated_potential_outcome_mean - untreated_potential_outcome_mean,
        treated_query: identification.treated_query,
        untreated_query: identification.untreated_query,
        treated_expression: identification.treated_expression,
        untreated_expression: identification.untreated_expression,
    })
}
