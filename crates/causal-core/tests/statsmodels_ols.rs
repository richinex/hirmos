use hirmos_causal_core::ols::{Ols, OlsError};
use nalgebra::{DMatrix, DVector};
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    name: String,
    design: Vec<Vec<f64>>,
    outcome: Vec<f64>,
    params: Vec<f64>,
    resid: Vec<f64>,
    ssr: f64,
    normalized_cov_params: Vec<Vec<f64>>,
    singular_values: Vec<f64>,
    rank: usize,
    df_resid: usize,
    llf: f64,
    aic: f64,
    tvalues: Vec<f64>,
}

fn close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
        "{label}: actual={actual:.17e}, expected={expected:.17e}, difference={:.3e}",
        (actual - expected).abs(),
    );
}

#[test]
fn default_pseudoinverse_fit_matches_statsmodels() {
    let fixture: Fixture =
        serde_json::from_str(include_str!("../oracle/fixtures/statsmodels_ols.json"))
            .expect("statsmodels OLS fixture");

    for case in fixture.cases {
        let rows = case.design.len();
        let columns = case.design[0].len();
        let design = DMatrix::from_fn(rows, columns, |row, column| case.design[row][column]);
        let outcome = DVector::from_column_slice(&case.outcome);
        let fit = Ols::try_fit(&design, &outcome).expect(&case.name);

        assert_eq!(fit.rank, case.rank, "{} rank", case.name);
        assert_eq!(fit.nobs - fit.rank, case.df_resid, "{} df_resid", case.name);
        for (index, (&actual, &expected)) in fit.params.iter().zip(&case.params).enumerate() {
            close(
                &format!("{} param {index}", case.name),
                actual,
                expected,
                2e-12,
            );
        }
        for (index, (&actual, &expected)) in fit.resid.iter().zip(&case.resid).enumerate() {
            close(
                &format!("{} resid {index}", case.name),
                actual,
                expected,
                3e-12,
            );
        }
        for (index, (&actual, &expected)) in fit
            .singular_values
            .iter()
            .zip(&case.singular_values)
            .enumerate()
        {
            close(
                &format!("{} singular value {index}", case.name),
                actual,
                expected,
                2e-12,
            );
        }
        let covariance = fit.xtx_inverse();
        for row in 0..columns {
            for column in 0..columns {
                close(
                    &format!("{} covariance {row},{column}", case.name),
                    covariance[(row, column)],
                    case.normalized_cov_params[row][column],
                    4e-12,
                );
            }
        }
        for (index, (&actual, &expected)) in fit.tvalues().iter().zip(&case.tvalues).enumerate() {
            close(
                &format!("{} tvalue {index}", case.name),
                actual,
                expected,
                5e-12,
            );
        }
        close(&format!("{} ssr", case.name), fit.ssr, case.ssr, 5e-12);
        close(&format!("{} llf", case.name), fit.llf(), case.llf, 5e-12);
        close(&format!("{} aic", case.name), fit.aic(), case.aic, 5e-12);
    }
}

#[test]
fn invalid_input_is_typed() {
    let design = DMatrix::from_row_slice(2, 1, &[1.0, 2.0]);
    assert_eq!(
        Ols::try_fit(&design, &DVector::from_row_slice(&[1.0])).unwrap_err(),
        OlsError::RowMismatch
    );
}
