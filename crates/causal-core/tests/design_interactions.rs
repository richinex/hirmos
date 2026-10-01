use hirmos_causal_core::design::{
    hierarchical_design, Column, DesignError, Encoding, Term, Values,
};
use std::collections::BTreeMap;

fn numeric(name: &str, v: Vec<f64>) -> Column {
    Column {
        name: name.into(),
        values: Values::Numeric(v),
        encoding: Encoding::Numeric,
    }
}

#[test]
fn triple_includes_lower_orders_and_keeps_named_reference() {
    let columns = vec![
        numeric("a", vec![0., 1., 0., 1.]),
        numeric("b", vec![0., 0., 1., 1.]),
        numeric("post", vec![0., 1., 1., 0.]),
    ];
    let result = hierarchical_design(&columns, &[Term::Triple(0, 1, 2)], &BTreeMap::new()).unwrap();
    assert_eq!(
        result.names,
        [
            "Intercept",
            "a",
            "b",
            "post",
            "a:b",
            "a:post",
            "b:post",
            "a:b:post"
        ]
    );
    for (i, row) in result.rows.iter().enumerate() {
        let a = if i % 2 == 1 { 1. } else { 0. };
        let b = if i >= 2 { 1. } else { 0. };
        let p = if i == 1 || i == 2 { 1. } else { 0. };
        assert_eq!(row, &vec![1., a, b, p, a * b, a * p, b * p, a * b * p]);
    }
    let mut factors = columns.clone();
    factors[0].encoding = Encoding::TreatmentContrast;
    let d = hierarchical_design(
        &factors,
        &[Term::Triple(0, 1, 2), Term::Pair(1, 0)],
        &BTreeMap::from([(0, "1".into())]),
    )
    .unwrap();
    assert_eq!(d.names[1], "C(a)[T.0]");
    assert_eq!(d.names.len(), 8);
    assert_eq!(d.rows[0][1], 1.);
    assert_eq!(d.rows[1][1], 0.);
}

#[test]
fn invalid_interactions_and_references_are_refused() {
    let columns = vec![numeric("a", vec![1., 2.]), numeric("b", vec![2., 3.])];
    for term in [Term::Pair(0, 0), Term::Triple(0, 1, 2), Term::Main(2)] {
        assert_eq!(
            hierarchical_design(&columns, &[term], &BTreeMap::new()).unwrap_err(),
            DesignError::InvalidTerm { term: 0 }
        );
    }
    for reference in [
        BTreeMap::from([(0, "1".into())]),
        BTreeMap::from([(5, "1".into())]),
    ] {
        assert!(matches!(
            hierarchical_design(&columns, &[Term::Main(0)], &reference),
            Err(DesignError::InvalidReference { .. })
        ));
    }
    let duplicated = vec![columns[0].clone(), columns[0].clone()];
    assert!(matches!(
        hierarchical_design(&duplicated, &[], &BTreeMap::new()),
        Err(DesignError::DuplicateName { .. })
    ));
    let overflow = vec![numeric("a", vec![1e300]), numeric("b", vec![1e300])];
    assert!(matches!(
        hierarchical_design(&overflow, &[Term::Pair(0, 1)], &BTreeMap::new()),
        Err(DesignError::NonFiniteInteraction { .. })
    ));
}
