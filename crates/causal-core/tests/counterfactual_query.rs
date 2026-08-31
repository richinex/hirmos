use hirmos_causal_core::counterfactual_query::{
    BinaryValue, ConditionalCounterfactualQuery, CounterfactualExpression, CounterfactualVariable,
    Event, EventAtom, Intervention, QueryError,
};

fn intervention(name: &str, value: BinaryValue) -> Intervention {
    Intervention::new(name, value).unwrap()
}

#[test]
fn distinguishes_event_values_from_intervention_values() {
    let y_under_x_zero =
        CounterfactualVariable::in_world("Y", [intervention("X", BinaryValue::Zero)]).unwrap();
    let event = Event::new([EventAtom::new(y_under_x_zero.clone(), BinaryValue::One)]).unwrap();

    assert_eq!(y_under_x_zero.to_y0(), "Y @ -X");
    assert_eq!(
        event
            .iter()
            .next()
            .map(|(variable, value)| (variable.to_y0(), value.as_u8())),
        Some(("Y @ -X".to_owned(), 1))
    );
}

#[test]
fn rejects_contradictory_interventions_within_one_world() {
    let error = CounterfactualVariable::in_world(
        "Y",
        [
            intervention("X", BinaryValue::Zero),
            intervention("X", BinaryValue::One),
        ],
    )
    .unwrap_err();
    assert_eq!(
        error,
        QueryError::ConflictingIntervention {
            variable: "X".to_owned(),
            first: BinaryValue::Zero,
            second: BinaryValue::One,
        }
    );
}

#[test]
fn refuses_the_same_event_as_outcome_and_condition() {
    let treated = CounterfactualVariable::factual("X").unwrap();
    let outcomes = Event::new([EventAtom::new(treated.clone(), BinaryValue::One)]).unwrap();
    let conditions = Event::new([EventAtom::new(treated.clone(), BinaryValue::One)]).unwrap();

    assert_eq!(
        ConditionalCounterfactualQuery::new(outcomes, conditions),
        Err(QueryError::OverlappingOutcomeAndCondition(treated))
    );
}

#[test]
fn renders_symbolic_counterfactual_expressions_stably() {
    let x = CounterfactualVariable::in_world(
        "X",
        [
            intervention("W", BinaryValue::Zero),
            intervention("Z", BinaryValue::Zero),
        ],
    )
    .unwrap();
    let y = CounterfactualVariable::in_world(
        "Y",
        [
            intervention("W", BinaryValue::Zero),
            intervention("Z", BinaryValue::Zero),
        ],
    )
    .unwrap();
    let w = CounterfactualVariable::in_world("W", [intervention("X", BinaryValue::Zero)]).unwrap();
    let expression = CounterfactualExpression::marginalize(
        CounterfactualExpression::product([
            CounterfactualExpression::probability([x, y]),
            CounterfactualExpression::probability([w]),
        ]),
        ["W".to_owned()],
    );

    assert_eq!(expression.to_y0(), "Sum[W](P[X](W) * P[W,Z](X, Y))");
}
