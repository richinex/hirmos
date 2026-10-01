//! Numerical parity tests for expressions returned by the level-2 ID/IDC algorithms.

use std::collections::BTreeMap;

use hirmos_causal_core::do_calculus::{identify_conditional_outcomes, Admg};
use hirmos_causal_core::do_calculus::Expression;
use hirmos_causal_core::discrete_bn::Dag;
use hirmos_causal_core::identified_expression::{
    evaluate_distribution, evaluate_expression, DiscreteTable, EvaluationError,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct IdcOracleCase {
    directed: Vec<(String, String)>,
    bidirected: Vec<(String, String)>,
    treatment: String,
    outcome: String,
    condition: String,
    expression: Option<String>,
    values: Option<Vec<f64>>,
}

const WEIGHTS: [usize; 8] = [1, 2, 3, 5, 7, 11, 13, 17];

#[test]
fn graph_simplification_retains_real_conditions_and_latent_paths() {
    let graph = |edges: &[(&str, &str)]| Dag::new(&edges.iter().map(|(a,b)| (a.to_string(),b.to_string())).collect::<Vec<_>>(), &[]);
    let expression = Expression::conditional("Y".into(), vec!["W".into(), "X".into()]);
    let candidates = ["W".to_string()].into_iter().collect();
    // W -> X -> Y: conditioning on X blocks W. The query's X is retained.
    let simplified = expression.simplify_irrelevant_conditions(&graph(&[("W","X"),("X","Y")]), &candidates);
    assert_eq!(simplified, Expression::conditional("Y".into(), vec!["X".into()]));
    // Conditioning on a collider opens W -> X <- Y. Do not drop W.
    assert_eq!(expression.simplify_irrelevant_conditions(&graph(&[("W","X"),("Y","X")]), &candidates), expression);
    // A latent common cause is still a path, even though it is not a data column.
    assert_eq!(expression.simplify_irrelevant_conditions(&graph(&[("U","W"),("U","Y"),("X","Y")]), &candidates), expression);
    // Product, sum and ratio traverse the same rule; no new weighting factor.
    let nested = Expression::normalize_marginalize(Expression::marginalize(Expression::product([expression.clone(), Expression::joint(["X".into()])]), ["X".into()]), ["Y".into()]);
    let reduced = nested.simplify_irrelevant_conditions(&graph(&[("W","X"),("X","Y")]), &candidates);
    assert!(!reduced.free_variables().contains("W"));
    assert!(nested.free_variables().contains("W"));
}

fn oracle_table() -> DiscreteTable {
    let mut columns = BTreeMap::from([
        ("A".to_owned(), Vec::new()),
        ("B".to_owned(), Vec::new()),
        ("C".to_owned(), Vec::new()),
    ]);
    for (index, weight) in WEIGHTS.into_iter().enumerate() {
        for _ in 0..weight {
            columns.get_mut("A").unwrap().push((index & 1).to_string());
            columns
                .get_mut("B")
                .unwrap()
                .push(((index >> 1) & 1).to_string());
            columns
                .get_mut("C")
                .unwrap()
                .push(((index >> 2) & 1).to_string());
        }
    }
    DiscreteTable::from_columns(columns).unwrap()
}

#[test]
fn evaluates_every_identifiable_idc_oracle_case() {
    let cases: Vec<IdcOracleCase> =
        serde_json::from_str(include_str!("../oracle/fixtures/idc_exhaustive_y0.json")).unwrap();
    assert_eq!(cases.len(), 384);
    let table = oracle_table();
    assert_eq!(table.observations(), WEIGHTS.iter().sum::<usize>());

    for (case_index, case) in cases.into_iter().enumerate() {
        let graph = Admg::new(
            ["A".to_owned(), "B".to_owned(), "C".to_owned()],
            case.directed,
            case.bidirected,
        )
        .unwrap();
        let identified = identify_conditional_outcomes(
            &graph,
            [case.treatment.clone()],
            [case.outcome.clone()],
            [case.condition.clone()],
        );

        let (Ok(expression), Some(expected_values)) = (identified, case.values) else {
            assert!(
                case.expression.is_none(),
                "oracle case {case_index} contains an expression without values"
            );
            continue;
        };

        for (value_index, (treatment, outcome, condition)) in [
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1),
        ]
        .into_iter()
        .enumerate()
        {
            let assignment = BTreeMap::from([
                (case.treatment.clone(), treatment.to_string()),
                (case.outcome.clone(), outcome.to_string()),
                (case.condition.clone(), condition.to_string()),
            ]);
            let actual = evaluate_expression(&expression, &table, &assignment).unwrap();
            assert!(
                (actual - expected_values[value_index]).abs() < 1e-12,
                "oracle case {case_index}, value {value_index}: Rust {} = {actual}, y0 = {}",
                expression.to_y0(),
                expected_values[value_index]
            );
        }
    }
}

#[test]
fn evaluates_a_complete_conditional_outcome_distribution() {
    let graph = Admg::new(
        ["A".to_owned(), "B".to_owned(), "C".to_owned()],
        [("A".to_owned(), "B".to_owned())],
        [],
    )
    .unwrap();
    let expression =
        identify_conditional_outcomes(&graph, ["A".to_owned()], ["B".to_owned()], ["C".to_owned()])
            .unwrap();
    let fixed = BTreeMap::from([
        ("A".to_owned(), "1".to_owned()),
        ("C".to_owned(), "0".to_owned()),
    ]);
    let distribution =
        evaluate_distribution(&expression, &oracle_table(), ["B".to_owned()], &fixed).unwrap();

    assert!((distribution.total - 1.0).abs() < 1e-12);
    assert_eq!(distribution.outcomes, vec!["B"]);
    for (states, probability) in distribution.probabilities {
        let assignment = BTreeMap::from([
            ("A".to_owned(), "1".to_owned()),
            ("B".to_owned(), states[0].clone()),
            ("C".to_owned(), "0".to_owned()),
        ]);
        let point = evaluate_expression(&expression, &oracle_table(), &assignment).unwrap();
        assert!((point - probability).abs() < 1e-15);
    }
}

#[test]
fn refuses_invalid_assignments_instead_of_fabricating_a_probability() {
    let graph = Admg::new(
        ["A".to_owned(), "B".to_owned()],
        [("A".to_owned(), "B".to_owned())],
        [],
    )
    .unwrap();
    let expression = hirmos_causal_core::do_calculus::identify_outcomes(
        &graph,
        ["A".to_owned()],
        ["B".to_owned()],
    )
    .unwrap();
    let table = oracle_table();

    let error = evaluate_expression(
        &expression,
        &table,
        &BTreeMap::from([
            ("A".to_owned(), "2".to_owned()),
            ("B".to_owned(), "0".to_owned()),
        ]),
    )
    .unwrap_err();
    assert_eq!(
        error,
        EvaluationError::UnknownState {
            variable: "A".to_owned(),
            state: "2".to_owned(),
        }
    );
}
