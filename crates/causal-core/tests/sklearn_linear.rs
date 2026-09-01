use hirmos_causal_core::sklearn_linear::fit_sklearn_linear_regression;
use nalgebra::{DMatrix, DVector};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    sklearn_version: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    x: Vec<Vec<f64>>,
    y: Vec<f64>,
    tolerance: f64,
    intercept: f64,
    coefficients: Vec<f64>,
    rank: usize,
    singular_values: Vec<f64>,
    predictions: Vec<f64>,
    residual_scale: f64,
}

fn maxdev(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .map(|(actual, expected)| (actual - expected).abs())
        .fold(0.0, f64::max)
}

#[test]
fn passes_sklearn_upstream_dense_single_target_regression_test() {
    // Ported from scikit-learn 1.9.0:
    // sklearn/linear_model/tests/test_base.py::test_linear_regression.
    // These are the upstream inputs and expected values for the dense,
    // fit-intercept, single-target lane used by Tigramite.
    let predictors = DMatrix::from_row_slice(2, 1, &[1.0, 2.0]);
    let target = DVector::from_vec(vec![1.0, 2.0]);
    let fit = fit_sklearn_linear_regression(&predictors, &target, 1e-6).unwrap();

    assert!((fit.coefficients[0] - 1.0).abs() <= 1e-12);
    assert!(fit.intercept.abs() <= 1e-12);
    assert!(maxdev(fit.predictions.as_slice(), &[1.0, 2.0]) <= 1e-12);

    // The second half of the same upstream test is deliberately degenerate:
    // one observation and one constant predictor. Scikit-learn returns the
    // minimum-norm coefficient instead of refusing the rank-deficient fit.
    let degenerate_predictors = DMatrix::from_row_slice(1, 1, &[1.0]);
    let degenerate_target = DVector::from_vec(vec![0.0]);
    let degenerate =
        fit_sklearn_linear_regression(&degenerate_predictors, &degenerate_target, 1e-6).unwrap();

    assert!(degenerate.coefficients[0].abs() <= 1e-12);
    assert!(degenerate.intercept.abs() <= 1e-12);
    assert!(degenerate.predictions[0].abs() <= 1e-12);
}

#[test]
fn centered_svd_regression_matches_sklearn_1_9_including_singular_designs() {
    let fixture: Fixture = serde_json::from_str(include_str!(
        "../oracle/fixtures/sklearn_linear_regression.json"
    ))
    .unwrap();
    assert_eq!(fixture.sklearn_version, "1.9.0");

    for case in fixture.cases {
        let rows = case.x.len();
        let columns = case.x.first().map_or(0, Vec::len);
        let flat: Vec<f64> = case.x.iter().flatten().copied().collect();
        let predictors = DMatrix::from_row_slice(rows, columns, &flat);
        let target = DVector::from_vec(case.y);
        let fit = fit_sklearn_linear_regression(&predictors, &target, case.tolerance).unwrap();
        let coefficient_deviation = maxdev(fit.coefficients.as_slice(), &case.coefficients);
        let prediction_deviation = maxdev(fit.predictions.as_slice(), &case.predictions);
        let singular_deviation = maxdev(fit.singular_values.as_slice(), &case.singular_values);
        let intercept_deviation = (fit.intercept - case.intercept).abs();
        let residual_deviation = (fit.residual_scale - case.residual_scale).abs();
        println!(
            "{}: coef {coefficient_deviation:.3e}, intercept {intercept_deviation:.3e}, prediction {prediction_deviation:.3e}, singular {singular_deviation:.3e}, residual {residual_deviation:.3e}, rank {}",
            case.name, fit.rank
        );
        assert_eq!(fit.rank, case.rank, "{} rank", case.name);
        assert!(coefficient_deviation <= 2e-10, "{} coefficients", case.name);
        assert!(intercept_deviation <= 2e-10, "{} intercept", case.name);
        assert!(prediction_deviation <= 2e-10, "{} predictions", case.name);
        assert!(singular_deviation <= 2e-9, "{} singular values", case.name);
        assert!(residual_deviation <= 2e-12, "{} residual scale", case.name);
    }
}
