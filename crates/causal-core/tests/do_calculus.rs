//! Parity tests for y0's level-2 ID algorithm.
//!
//! Oracle: y0 commit cc6644d3b470a1ab2a1eb9da4f0fdb0c843f5d3a, whose source is
//! vendored under reference/y0. The fixture expressions were emitted by
//! oracle/do_calculus_y0.py and are intentionally compared exactly in y0's DSL form.

use hirmos_causal_core::do_calculus::{
    identify_conditional_outcomes, identify_outcomes, latent_projection, Admg, Expression,
    GraphError, IdentificationError,
};
use serde::Deserialize;

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn directed(values: &[(&str, &str)]) -> Vec<(String, String)> {
    values
        .iter()
        .map(|(source, target)| ((*source).to_owned(), (*target).to_owned()))
        .collect()
}

#[test]
fn backdoor_matches_y0() {
    let graph = Admg::new(
        names(&["X", "Y", "Z"]),
        directed(&[("Z", "X"), ("Z", "Y"), ("X", "Y")]),
        [],
    )
    .unwrap();
    let expression = identify_outcomes(&graph, names(&["X"]), names(&["Y"])).unwrap();
    assert_eq!(
        expression.to_y0(),
        "Sum[Z](P(Y | X, Z) * Sum[X, Y](P(X, Y, Z)))"
    );
}

#[test]
fn frontdoor_matches_y0() {
    let graph = Admg::new(
        names(&["X", "Y", "Z"]),
        directed(&[("X", "Z"), ("Z", "Y")]),
        directed(&[("X", "Y")]),
    )
    .unwrap();
    let expression = identify_outcomes(&graph, names(&["X"]), names(&["Y"])).unwrap();
    assert_eq!(
        expression.to_y0(),
        "Sum[Z](P(Z | X) * Sum[X](P(X) * P(Y | X, Z)))"
    );
    assert_eq!(
        expression.to_latex(),
        "\\sum_{Z} P(Z \\mid X) \\; \\sum_{X} P(X) \\; P(Y \\mid X, Z)"
    );
}

#[test]
fn idc_figure_6a_matches_y0() {
    let graph = Admg::new(
        names(&["X", "Y", "Z"]),
        directed(&[("X", "Z"), ("Z", "Y")]),
        directed(&[("X", "Z")]),
    )
    .unwrap();
    let expression =
        identify_conditional_outcomes(&graph, names(&["X"]), names(&["Y"]), names(&["Z"])).unwrap();
    assert_eq!(expression.to_y0(), "((P(Y | X, Z) / Sum[Y](P(Y | X, Z))))");
    assert_eq!(
        expression.to_latex(),
        "\\frac{P(Y \\mid X, Z)}{\\sum_{Y} P(Y \\mid X, Z)}"
    );
}

#[test]
fn recursive_line_7_case_matches_y0() {
    // Shpitser and Pearl (2008), y0's line_7_example.
    let graph = Admg::new(
        names(&["W1", "X", "Y1"]),
        directed(&[("X", "Y1"), ("W1", "X")]),
        directed(&[("W1", "Y1")]),
    )
    .unwrap();
    let expression = identify_outcomes(&graph, names(&["X", "W1"]), names(&["Y1"])).unwrap();
    assert_eq!(expression.to_y0(), "Sum[W1](P(W1) * P(Y1 | W1, X))");
}

#[test]
fn complete_hierarchy_examples_match_y0() {
    let cases = [
        (
            "figure_2a",
            Admg::new(names(&["X", "Y"]), directed(&[("X", "Y")]), []).unwrap(),
            names(&["Y"]),
            "P(Y | X)",
        ),
        (
            "figure_2b",
            Admg::new(
                names(&["X", "Y", "Z"]),
                directed(&[("X", "Y"), ("X", "Z"), ("Z", "Y")]),
                directed(&[("Y", "Z")]),
            )
            .unwrap(),
            names(&["Y"]),
            "Sum[Z](P(Y | X, Z) * P(Z | X))",
        ),
        (
            "figure_2c",
            Admg::new(
                names(&["X", "Y", "Z"]),
                directed(&[("X", "Y"), ("Z", "X"), ("Z", "Y")]),
                directed(&[("X", "Z")]),
            )
            .unwrap(),
            names(&["Y"]),
            "Sum[Z](P(Y | X, Z) * Sum[X, Y](P(X, Y, Z)))",
        ),
        (
            "figure_2e",
            Admg::new(
                names(&["X", "Y", "Z"]),
                directed(&[("X", "Z"), ("Z", "Y")]),
                directed(&[("X", "Y")]),
            )
            .unwrap(),
            names(&["Y"]),
            "Sum[Z](P(Z | X) * Sum[X](P(X) * P(Y | X, Z)))",
        ),
    ];

    for (name, graph, outcomes, expected) in cases {
        let actual = identify_outcomes(&graph, names(&["X"]), outcomes).unwrap();
        assert_eq!(actual.to_y0(), expected, "{name}");
    }
}

#[test]
fn multi_outcome_recursive_example_matches_y0() {
    let graph = Admg::new(
        names(&["W1", "W2", "X", "Y1", "Y2"]),
        directed(&[("X", "Y1"), ("W1", "X"), ("W2", "Y2")]),
        directed(&[("W1", "W2"), ("W1", "Y1"), ("W1", "Y2"), ("X", "W2")]),
    )
    .unwrap();
    let actual = identify_outcomes(&graph, names(&["X"]), names(&["Y1", "Y2"])).unwrap();
    assert_eq!(
        actual.to_y0(),
        "Sum[W2](P(Y2 | W2) * Sum[W1, X, Y1, Y2](P(W1, W2, X, Y1, Y2)) * Sum[W1](P(W1) * P(Y1 | W1, X)))"
    );
}

#[test]
fn hedge_is_reported_as_unidentifiable() {
    // X -> Y together with X <-> Y is the bow-arc hedge.
    let graph = Admg::new(
        names(&["X", "Y"]),
        directed(&[("X", "Y")]),
        directed(&[("X", "Y")]),
    )
    .unwrap();
    let error = identify_outcomes(&graph, names(&["X"]), names(&["Y"])).unwrap_err();
    let IdentificationError::Unidentifiable(hedge) = error else {
        panic!("expected a hedge witness");
    };
    assert_eq!(
        hedge.graph_district,
        names(&["X", "Y"]).into_iter().collect()
    );
    assert_eq!(
        hedge.treatment_removed_district,
        names(&["Y"]).into_iter().collect()
    );
}

#[test]
fn validates_query_and_directed_graph() {
    let graph = Admg::new(names(&["X", "Y"]), directed(&[("X", "Y")]), []).unwrap();
    assert_eq!(
        identify_outcomes(&graph, names(&["X"]), Vec::<String>::new()),
        Err(IdentificationError::InvalidGraph(GraphError::EmptyOutcome))
    );
    assert!(matches!(
        Admg::new(names(&["X", "Y"]), directed(&[("X", "Y"), ("Y", "X")]), []),
        Err(GraphError::DirectedCycle)
    ));
}

#[test]
fn latex_escapes_user_variable_names() {
    let graph = Admg::new(
        names(&["treat_rate", "net income"]),
        directed(&[("treat_rate", "net income")]),
        [],
    )
    .unwrap();
    assert_eq!(
        identify_outcomes(&graph, names(&["treat_rate"]), names(&["net income"]),)
            .unwrap()
            .to_latex(),
        "P(\\text{net income} \\mid \\text{treat\\_rate})"
    );
}

#[test]
fn latent_projection_preserves_observed_mediators() {
    // U confounds X and Y, while M remains an observed mediator on X -> M -> Y.
    let graph = latent_projection(
        names(&["U", "X", "M", "Y"]),
        directed(&[("U", "X"), ("U", "Y"), ("X", "M"), ("M", "Y")]),
        names(&["U"]),
    )
    .unwrap();
    assert_eq!(
        graph.nodes(),
        &names(&["M", "X", "Y"]).into_iter().collect()
    );
    assert_eq!(
        graph.directed_edges(),
        &directed(&[("X", "M"), ("M", "Y")]).into_iter().collect()
    );
    assert_eq!(
        graph.bidirected_edges(),
        &directed(&[("X", "Y")]).into_iter().collect()
    );
    assert_eq!(
        identify_outcomes(&graph, names(&["X"]), names(&["Y"]))
            .unwrap()
            .to_y0(),
        "Sum[M](P(M | X) * Sum[X](P(X) * P(Y | M, X)))"
    );
}

#[test]
fn latent_projection_follows_latent_chains_but_stops_at_observed_nodes() {
    let graph = latent_projection(
        names(&["U0", "U1", "U2", "A", "M", "B", "C"]),
        directed(&[
            ("U0", "U1"),
            ("U0", "U2"),
            ("U1", "A"),
            ("U2", "B"),
            ("A", "M"),
            ("M", "C"),
        ]),
        names(&["U0", "U1", "U2"]),
    )
    .unwrap();
    assert!(graph
        .bidirected_edges()
        .contains(&("A".to_owned(), "B".to_owned())));
    assert!(!graph
        .bidirected_edges()
        .contains(&("B".to_owned(), "C".to_owned())));
    assert!(graph
        .directed_edges()
        .contains(&("A".to_owned(), "M".to_owned())));
    assert!(!graph
        .directed_edges()
        .contains(&("A".to_owned(), "C".to_owned())));
}

#[derive(Deserialize)]
struct OracleCase {
    directed: Vec<(String, String)>,
    bidirected: Vec<(String, String)>,
    treatment: String,
    outcome: String,
    expression: Option<String>,
    values: Option<Vec<f64>>,
}

const WEIGHTS: [f64; 8] = [1.0, 2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0];

fn observational_probability(assignment: &std::collections::BTreeMap<String, u8>) -> f64 {
    let total: f64 = WEIGHTS.iter().sum();
    WEIGHTS
        .iter()
        .enumerate()
        .filter(|(index, _)| {
            assignment.iter().all(|(name, value)| {
                let actual = match name.as_str() {
                    "A" => index & 1,
                    "B" => (index >> 1) & 1,
                    "C" => (index >> 2) & 1,
                    _ => panic!("unexpected oracle variable {name}"),
                };
                actual == usize::from(*value)
            })
        })
        .map(|(_, weight)| *weight)
        .sum::<f64>()
        / total
}

fn evaluate(
    expression: &Expression,
    assignment: &mut std::collections::BTreeMap<String, u8>,
) -> f64 {
    match expression {
        Expression::Probability { children, parents } => {
            let numerator_assignment = assignment
                .iter()
                .filter(|(name, _)| children.contains(*name) || parents.contains(*name))
                .map(|(name, value)| (name.clone(), *value))
                .collect();
            let numerator = observational_probability(&numerator_assignment);
            if parents.is_empty() {
                numerator
            } else {
                let denominator_assignment = assignment
                    .iter()
                    .filter(|(name, _)| parents.contains(*name))
                    .map(|(name, value)| (name.clone(), *value))
                    .collect();
                numerator / observational_probability(&denominator_assignment)
            }
        }
        Expression::Product(factors) => factors
            .iter()
            .map(|factor| evaluate(factor, assignment))
            .product(),
        Expression::Sum { ranges, expression } => {
            fn sum_ranges(
                ranges: &[String],
                position: usize,
                expression: &Expression,
                assignment: &mut std::collections::BTreeMap<String, u8>,
            ) -> f64 {
                if position == ranges.len() {
                    return evaluate(expression, assignment);
                }
                let variable = &ranges[position];
                let previous = assignment.get(variable).copied();
                let mut result = 0.0;
                for value in [0, 1] {
                    assignment.insert(variable.clone(), value);
                    result += sum_ranges(ranges, position + 1, expression, assignment);
                }
                if let Some(value) = previous {
                    assignment.insert(variable.clone(), value);
                } else {
                    assignment.remove(variable);
                }
                result
            }

            let ranges = ranges.iter().cloned().collect::<Vec<_>>();
            sum_ranges(&ranges, 0, expression, assignment)
        }
        Expression::Ratio {
            numerator,
            denominator,
        } => evaluate(numerator, assignment) / evaluate(denominator, assignment),
    }
}

#[test]
fn exhaustive_three_variable_graphs_match_y0() {
    let cases: Vec<OracleCase> = serde_json::from_str(include_str!(
        "../oracle/fixtures/do_calculus_exhaustive_y0.json"
    ))
    .unwrap();
    assert_eq!(cases.len(), 384);

    for (index, case) in cases.into_iter().enumerate() {
        let graph = Admg::new(names(&["A", "B", "C"]), case.directed, case.bidirected).unwrap();
        let actual = identify_outcomes(&graph, [case.treatment.clone()], [case.outcome.clone()]);
        match (actual, case.expression, case.values) {
            (Ok(expression), Some(expected), Some(expected_values)) => {
                for (value_index, (treatment_value, outcome_value)) in
                    [(0, 0), (0, 1), (1, 0), (1, 1)].into_iter().enumerate()
                {
                    let mut assignment = std::collections::BTreeMap::from([
                        (case.treatment.clone(), treatment_value),
                        (case.outcome.clone(), outcome_value),
                    ]);
                    let actual_value = evaluate(&expression, &mut assignment);
                    assert!(
                        (actual_value - expected_values[value_index]).abs() < 1e-12,
                        "oracle case {index}, value {value_index}: Rust {} = {actual_value}, y0 {expected} = {}",
                        expression.to_y0(),
                        expected_values[value_index]
                    );
                }
            }
            (Err(IdentificationError::Unidentifiable(_)), None, None) => {}
            (actual, expected, values) => {
                panic!(
                    "oracle case {index} disagreed: Rust={actual:?}, y0={expected:?}, values={values:?}"
                )
            }
        }
    }
}

#[derive(Deserialize)]
struct IdcOracleCase {
    directed: Vec<(String, String)>,
    bidirected: Vec<(String, String)>,
    treatment: String,
    outcome: String,
    condition: String,
    expression: Option<String>,
    latex: Option<String>,
    values: Option<Vec<f64>>,
}

#[test]
fn exhaustive_three_variable_idc_matches_y0() {
    let cases: Vec<IdcOracleCase> =
        serde_json::from_str(include_str!("../oracle/fixtures/idc_exhaustive_y0.json")).unwrap();
    assert_eq!(cases.len(), 384);

    for (index, case) in cases.into_iter().enumerate() {
        let graph = Admg::new(names(&["A", "B", "C"]), case.directed, case.bidirected).unwrap();
        let actual = identify_conditional_outcomes(
            &graph,
            [case.treatment.clone()],
            [case.outcome.clone()],
            [case.condition.clone()],
        );
        match (actual, case.expression, case.latex, case.values) {
            (Ok(expression), Some(expected), Some(_expected_latex), Some(expected_values)) => {
                for (value_index, (treatment_value, outcome_value, condition_value)) in [
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
                    let mut assignment = std::collections::BTreeMap::from([
                        (case.treatment.clone(), treatment_value),
                        (case.outcome.clone(), outcome_value),
                        (case.condition.clone(), condition_value),
                    ]);
                    let actual_value = evaluate(&expression, &mut assignment);
                    assert!(
                        (actual_value - expected_values[value_index]).abs() < 1e-12,
                        "oracle case {index}, value {value_index}: Rust {} = {actual_value}, y0 {expected} = {}",
                        expression.to_y0(),
                        expected_values[value_index]
                    );
                }
            }
            (Err(IdentificationError::Unidentifiable(_)), None, None, None) => {}
            (actual, expected, latex, values) => {
                panic!(
                    "oracle case {index} disagreed: Rust={actual:?}, y0={expected:?}, latex={latex:?}, values={values:?}"
                )
            }
        }
    }
}
