//! Exhaustive parity over 27,648 ID*/IDC* cases emitted by pinned y0.

use std::collections::BTreeSet;

use hirmos_causal_core::counterfactual_query::{
    BinaryValue, ConditionalCounterfactualQuery, CounterfactualExpression, CounterfactualVariable,
    Event, EventAtom, Intervention,
};
use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::id_star::{identify_counterfactual_event, IdStarError};
use hirmos_causal_core::idc_star::{identify_conditional_counterfactual, IdcStarError};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct OracleIntervention {
    variable: String,
    value: bool,
}

#[derive(Deserialize)]
struct OracleAtom {
    variable: String,
    interventions: Vec<OracleIntervention>,
    event_value: bool,
}

#[derive(Deserialize)]
struct OracleCase {
    directed: Vec<(String, String)>,
    bidirected: Vec<(String, String)>,
    algorithm: String,
    template: String,
    event: Option<Vec<OracleAtom>>,
    outcomes: Option<Vec<OracleAtom>>,
    conditions: Option<Vec<OracleAtom>>,
    status: String,
    expression: Option<Value>,
}

fn binary(value: bool) -> BinaryValue {
    value.into()
}

fn event(atoms: Vec<OracleAtom>) -> Event {
    Event::new(atoms.into_iter().map(|atom| {
        let variable = if atom.interventions.is_empty() {
            CounterfactualVariable::factual(atom.variable).unwrap()
        } else {
            CounterfactualVariable::in_world(
                atom.variable,
                atom.interventions.into_iter().map(|intervention| {
                    Intervention::new(intervention.variable, binary(intervention.value)).unwrap()
                }),
            )
            .unwrap()
        };
        EventAtom::new(variable, binary(atom.event_value))
    }))
    .unwrap()
}

fn variable_json(variable: &CounterfactualVariable) -> Value {
    json!({
        "variable": variable.variable,
        "value": Value::Null,
        "interventions": variable.interventions.iter().map(|intervention| json!({
            "variable": intervention.variable,
            "value": intervention.value == BinaryValue::One,
        })).collect::<Vec<_>>(),
    })
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap()
}

fn expression_json(expression: &CounterfactualExpression) -> Value {
    match expression {
        CounterfactualExpression::One => json!({"kind": "one"}),
        CounterfactualExpression::Zero => json!({"kind": "zero"}),
        CounterfactualExpression::Probability(variables) => json!({
            "kind": "probability",
            "variables": variables.iter().map(variable_json).collect::<Vec<_>>(),
        }),
        CounterfactualExpression::Product(factors) => {
            let mut factors = factors.iter().map(expression_json).collect::<Vec<_>>();
            factors.sort_by_key(canonical_json);
            json!({"kind": "product", "factors": factors})
        }
        CounterfactualExpression::Sum { ranges, expression } => json!({
            "kind": "sum",
            "ranges": ranges.iter().collect::<Vec<_>>(),
            "expression": expression_json(expression),
        }),
        CounterfactualExpression::Ratio {
            numerator,
            denominator,
        } => json!({
            "kind": "ratio",
            "numerator": expression_json(numerator),
            "denominator": expression_json(denominator),
        }),
    }
}

#[test]
fn exhaustive_id_star_and_idc_star_match_y0() {
    let cases: Vec<OracleCase> = serde_json::from_str(include_str!(
        "../oracle/fixtures/id_star_exhaustive_y0.json"
    ))
    .unwrap();
    assert_eq!(cases.len(), 27_648);
    let mut expression_mismatches = Vec::new();

    for (index, case) in cases.into_iter().enumerate() {
        let graph = Admg::new(
            BTreeSet::from(["A".to_owned(), "B".to_owned(), "C".to_owned()]),
            case.directed,
            case.bidirected,
        )
        .unwrap();
        let actual = match case.algorithm.as_str() {
            "id_star" => match identify_counterfactual_event(&graph, &event(case.event.unwrap())) {
                Ok(expression) => ("identified", Some(expression_json(&expression))),
                Err(IdStarError::Conflict { .. }) => ("conflict", None),
                Err(error) => panic!(
                    "case {index} ({}) unexpected ID* error: {error:?}",
                    case.template
                ),
            },
            "idc_star" => {
                let query = ConditionalCounterfactualQuery::new(
                    event(case.outcomes.unwrap()),
                    event(case.conditions.unwrap()),
                )
                .unwrap();
                match identify_conditional_counterfactual(&graph, &query) {
                    Ok(expression) => ("identified", Some(expression_json(&expression))),
                    Err(IdcStarError::InconsistentConditioningEvent) => {
                        ("inconsistent_condition", None)
                    }
                    Err(IdcStarError::ZeroDenominatorExpression) => ("zero_denominator", None),
                    Err(IdcStarError::Identification(IdStarError::Conflict { .. })) => {
                        ("conflict", None)
                    }
                    Err(error) => panic!(
                        "case {index} ({}) unexpected IDC* error: {error:?}",
                        case.template
                    ),
                }
            }
            algorithm => panic!("case {index}: unknown algorithm {algorithm}"),
        };
        assert_eq!(
            actual.0, case.status,
            "case {index} ({}, {}) status",
            case.algorithm, case.template
        );
        if actual.1 != case.expression {
            expression_mismatches.push((index, case.algorithm, case.template));
        }
    }
    assert!(
        expression_mismatches.is_empty(),
        "{} expression mismatches; first: {:?}",
        expression_mismatches.len(),
        &expression_mismatches[..expression_mismatches.len().min(20)]
    );
}
