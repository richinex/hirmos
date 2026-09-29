use hirmos_causal_core::continuous_did::response_curve::{self, Uncertainty};
use serde_json::Value;
fn vector(v: &Value) -> Vec<f64> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_f64().unwrap())
        .collect()
}
#[test]
fn saved_paper_results_and_independent_designs_match() {
    let cases: Value =
        serde_json::from_str(include_str!("fixtures/continuous_did/response-curve.json")).unwrap();
    for (name, c) in cases.as_object().unwrap() {
        let f = response_curve::fit(
            &vector(&c["dose"]),
            &vector(&c["contrast"]),
            &vector(&c["grid"]),
            c["degree"].as_u64().unwrap() as usize,
            vector(&c["knots"]),
        )
        .unwrap();
        assert_eq!(f.uncertainty, Uncertainty::ConditionalRowInfluence);
        for (key, actual) in [
            ("coefficients", f.coefficients),
            ("fitted", f.fitted),
            ("residuals", f.residuals),
            ("curve", f.curve),
            ("se", f.standard_errors),
        ] {
            let expected = vector(&c[key]);
            assert_eq!(actual.len(), expected.len());
            for (i, (a, e)) in actual.iter().zip(expected).enumerate() {
                assert!(
                    (a - e).abs() < 1e-9 * (1.0 + e.abs()),
                    "{name} {key}[{i}] {a} != {e}"
                );
            }
        }
    }
}

#[test]
fn row_order_and_outcome_units_do_not_change_the_specification() {
    let mut x: Vec<_> = (1..=50).map(|i| i as f64 / 51.0).collect();
    let mut y: Vec<_> = x.iter().map(|x| x.sin() + 0.2 * (x * 70.0).cos()).collect();
    let grid = vec![0.2, 0.5, 0.8];
    let a = response_curve::fit(&x, &y, &grid, 3, vec![0.3, 0.7]).unwrap();
    x.reverse();
    y.reverse();
    let b = response_curve::fit(&x, &y, &grid, 3, vec![0.3, 0.7]).unwrap();
    let transformed: Vec<_> = y.iter().map(|y| 7.0 - 3.0 * y).collect();
    let c = response_curve::fit(&x, &transformed, &grid, 3, vec![0.3, 0.7]).unwrap();
    for i in 0..grid.len() {
        assert!((a.curve[i] - b.curve[i]).abs() < 1e-12);
        assert!((a.standard_errors[i] - b.standard_errors[i]).abs() < 1e-12);
        assert!((c.curve[i] - (7.0 - 3.0 * a.curve[i])).abs() < 1e-11);
        assert!((c.standard_errors[i] - 3.0 * a.standard_errors[i]).abs() < 1e-11);
    }
}

#[test]
fn invalid_or_unidentified_curves_are_not_fitted() {
    use response_curve::Error;
    assert!(matches!(
        response_curve::fit(&[], &[], &[0.5], 3, vec![]),
        Err(Error::Input)
    ));
    assert!(response_curve::fit(&[0.5; 20], &[1.0; 20], &[0.5], 3, vec![]).is_err());
    let x: Vec<_> = (0..20)
        .map(|i| if i % 2 == 0 { 0.1 } else { 0.9 })
        .collect();
    assert!(matches!(
        response_curve::fit(&x, &x, &[0.5], 3, vec![]),
        Err(Error::RankDeficient)
    ));
    assert!(matches!(
        response_curve::fit(&x, &x, &[f64::NAN], 1, vec![]),
        Err(Error::Input)
    ));
}
