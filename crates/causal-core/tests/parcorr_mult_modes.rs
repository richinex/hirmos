use hirmos_causal_core::parcorr_mult::{wilks_lambda, Correlation, Role, Samples};
use serde_json::Value;

fn check(actual: f64, expected: &Value, label: &str) {
    match expected.as_str() {
        Some("nan") => assert!(actual.is_nan(), "{label}: {actual}"),
        Some("inf") => assert_eq!(actual, f64::INFINITY, "{label}"),
        Some("-inf") => assert_eq!(actual, f64::NEG_INFINITY, "{label}"),
        Some(_) => unreachable!(),
        None => assert!(
            (actual - expected.as_f64().unwrap()).abs() < 2e-9,
            "{label}: {actual} != {expected}"
        ),
    }
}

#[test]
fn wilks_lambda_and_analytic_significance_match_tigramite() {
    let bases: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/parcorr_mult.json")).unwrap();
    let cases: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/parcorr_mult_modes.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let name = case["base"].as_str().unwrap();
        let base = bases["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|base| base["name"] == case["base"])
            .unwrap();
        let rows: Vec<Vec<f64>> = serde_json::from_value(base["array"].clone()).unwrap();
        let roles: Vec<_> = base["roles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|role| match role.as_u64().unwrap() {
                0 => Role::X,
                1 => Role::Y,
                2 => Role::Z,
                _ => unreachable!(),
            })
            .collect();
        let samples = Samples::new(rows.clone(), roles.clone()).unwrap();
        let correlation = Correlation::PccaWilksLambda;
        let (value, p_value) = samples.run_test_with(correlation).unwrap();
        check(value, &case["value"], &format!("{name} value"));
        check(p_value, &case["p"], &format!("{name} p"));
        let raw = samples.dependence_with(correlation).unwrap();
        check(raw, &case["raw"], &format!("{name} raw"));
        check(
            samples.significance_with(raw, correlation),
            &case["raw_p"],
            &format!("{name} raw p"),
        );
        let x: Vec<_> = rows
            .iter()
            .zip(&roles)
            .filter(|(_, role)| **role == Role::X)
            .map(|(row, _)| row.clone())
            .collect();
        let y: Vec<_> = rows
            .iter()
            .zip(&roles)
            .filter(|(_, role)| **role == Role::Y)
            .map(|(row, _)| row.clone())
            .collect();
        for (index, components) in [None, Some(0), Some(1), Some(2)].into_iter().enumerate() {
            check(
                wilks_lambda(&x, &y, components).unwrap(),
                &case["components"][index],
                &format!("{name} components {components:?}"),
            );
        }
    }
}
