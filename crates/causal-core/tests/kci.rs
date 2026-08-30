use hirmos_causal_core::kernel_conditional_independence;
use nalgebra::DMatrix;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture {
    name: String,
    x: Vec<f64>,
    y: Vec<f64>,
    z: Option<Vec<Vec<f64>>>,
    p_value: f64,
    statistic: f64,
}

fn matrix(values: &[f64]) -> DMatrix<f64> {
    DMatrix::from_column_slice(values.len(), 1, values)
}

#[test]
fn matches_dowhy_kernel_based_gamma_approximation() {
    let fixtures: Vec<Fixture> =
        serde_json::from_str(include_str!("../oracle/fixtures/kci.json")).unwrap();
    for fixture in fixtures {
        let z = fixture.z.as_ref().map(|rows| {
            DMatrix::from_fn(rows.len(), rows[0].len(), |row, column| rows[row][column])
        });
        let result =
            kernel_conditional_independence(&matrix(&fixture.x), &matrix(&fixture.y), z.as_ref())
                .unwrap();
        assert!(
            (result.p_value - fixture.p_value).abs() < 2e-8,
            "{}: Rust {}, DoWhy {}",
            fixture.name,
            result.p_value,
            fixture.p_value,
        );
        assert!(
            (result.statistic - fixture.statistic).abs() < 3e-9,
            "{} statistic: Rust {}, DoWhy {}",
            fixture.name,
            result.statistic,
            fixture.statistic,
        );
    }
}

#[test]
fn rejects_non_finite_values_instead_of_imputing() {
    assert!(kernel_conditional_independence(
        &matrix(&[0.0, 1.0, f64::NAN]),
        &matrix(&[1.0, 2.0, 3.0]),
        None,
    )
    .is_err());
}
