use std::collections::BTreeMap;

use hirmos_causal_core::counterfactual_evaluator::{
    estimate_binary_ett, evaluate_counterfactual_expression, CounterfactualEvaluationError,
};
use hirmos_causal_core::counterfactual_query::{
    BinaryValue, CounterfactualExpression, CounterfactualVariable, Event, EventAtom, Intervention,
};
use hirmos_causal_core::do_calculus::Admg;
use hirmos_causal_core::identified_expression::DiscreteTable;

fn confounded_binary_table() -> DiscreteTable {
    // Exact cell counts for Z, X, Y. The treatment group has
    // P(Z=1 | X=1)=0.8, E[Y(1)|X=1]=0.7, and E[Y(0)|X=1]=0.45.
    let cells = [
        (0, 0, 0, 30),
        (0, 0, 1, 10),
        (0, 1, 0, 5),
        (0, 1, 1, 5),
        (1, 0, 0, 5),
        (1, 0, 1, 5),
        (1, 1, 0, 10),
        (1, 1, 1, 30),
    ];
    let mut columns = BTreeMap::from([
        ("Z".to_owned(), Vec::new()),
        ("X".to_owned(), Vec::new()),
        ("Y".to_owned(), Vec::new()),
    ]);
    for (z, x, y, count) in cells {
        for _ in 0..count {
            columns.get_mut("Z").unwrap().push(z.to_string());
            columns.get_mut("X").unwrap().push(x.to_string());
            columns.get_mut("Y").unwrap().push(y.to_string());
        }
    }
    DiscreteTable::from_columns(columns).unwrap()
}

fn confounded_graph() -> Admg {
    Admg::new(
        ["Z", "X", "Y"].map(str::to_owned),
        [("Z", "X"), ("Z", "Y"), ("X", "Y")]
            .map(|(source, target)| (source.to_owned(), target.to_owned())),
        [],
    )
    .unwrap()
}

#[test]
fn binary_ett_matches_the_backdoor_g_formula_exactly() {
    let result =
        estimate_binary_ett(&confounded_graph(), &confounded_binary_table(), "X", "Y").unwrap();

    let treated_record = result
        .treated_query
        .identified_to_y0(&result.treated_expression);
    let untreated_record = result
        .untreated_query
        .identified_to_y0(&result.untreated_expression);
    assert_ne!(treated_record, untreated_record);
    assert!(treated_record.contains("Y @ +X: +Y"), "{treated_record}");
    assert!(
        untreated_record.contains("Y @ -X: +Y"),
        "{untreated_record}"
    );

    assert!(
        (result.treated_potential_outcome_mean - 0.70).abs() < 1e-12,
        "{result:?}"
    );
    assert!(
        (result.untreated_potential_outcome_mean - 0.45).abs() < 1e-12,
        "{result:?}"
    );
    assert!(
        (result.effect_on_treated - 0.25).abs() < 1e-12,
        "{result:?}"
    );
}

#[test]
fn cross_world_events_with_two_values_for_one_base_are_not_silently_collapsed() {
    let y_zero =
        CounterfactualVariable::in_world("Y", [Intervention::new("X", BinaryValue::Zero).unwrap()])
            .unwrap();
    let y_one =
        CounterfactualVariable::in_world("Y", [Intervention::new("X", BinaryValue::One).unwrap()])
            .unwrap();
    let event = Event::new([
        EventAtom::new(y_zero.clone(), BinaryValue::Zero),
        EventAtom::new(y_one.clone(), BinaryValue::One),
    ])
    .unwrap();
    let expression = CounterfactualExpression::product([
        CounterfactualExpression::probability([y_zero]),
        CounterfactualExpression::probability([y_one]),
    ]);

    assert_eq!(
        evaluate_counterfactual_expression(
            &confounded_graph(),
            &expression,
            &event,
            &confounded_binary_table(),
        ),
        Err(CounterfactualEvaluationError::ConflictingBaseValues {
            variable: "Y".to_owned(),
            first: BinaryValue::Zero,
            second: BinaryValue::One,
        })
    );
}
