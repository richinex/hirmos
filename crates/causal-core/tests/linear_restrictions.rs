use hirmos_causal_core::estimation::inference::{contrast, joint, Error, JointTest, Reference};
use nalgebra::DMatrix;
use serde_json::Value;

fn matrix(v: &Value) -> DMatrix<f64> {
    let rows: Vec<Vec<f64>> = serde_json::from_value(v.clone()).unwrap();
    DMatrix::from_fn(rows.len(), rows[0].len(), |i, j| rows[i][j])
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-8 * b.abs().max(1.0), "{a} != {b}");
}

#[test]
fn restrictions_match_statsmodels() {
    let fixtures: Value =
        serde_json::from_str(include_str!("../oracle/fixtures/estimation.json")).unwrap();
    for f in fixtures["restrictions"].as_array().unwrap() {
        let beta: Vec<f64> = serde_json::from_value(f["params"].clone()).unwrap();
        let null: Vec<f64> = serde_json::from_value(f["q"].clone()).unwrap();
        let cov = matrix(&f["covariance"]);
        let r = matrix(&f["R"]);
        let reference = if f["use_t"].as_bool().unwrap() {
            Reference::Student {
                degrees_of_freedom: f["df"].as_f64().unwrap(),
            }
        } else {
            Reference::Asymptotic
        };
        let c: Vec<f64> = r.row(0).iter().copied().collect();
        let result = contrast(&beta, &cov, &c, null[0], 0.9, reference).unwrap();
        for (actual, key) in [
            (result.estimate, "estimate"),
            (result.standard_error, "se"),
            (result.statistic, "statistic"),
            (result.p_value, "p"),
        ] {
            close(actual, f[key].as_f64().unwrap());
        }
        for i in 0..2 {
            close(result.interval[i], f["interval"][i].as_f64().unwrap());
        }
        let (statistic, p) = match joint(&beta, &cov, &r, &null, reference).unwrap() {
            JointTest::ChiSquared {
                statistic,
                degrees_of_freedom,
                p_value,
            } => {
                assert_eq!(degrees_of_freedom, 2);
                (statistic, p_value)
            }
            JointTest::F {
                statistic,
                numerator_df,
                denominator_df,
                p_value,
            } => {
                assert_eq!(numerator_df, 2);
                close(denominator_df, f["df"].as_f64().unwrap());
                (statistic, p_value)
            }
        };
        close(statistic, f["joint_statistic"].as_f64().unwrap());
        close(p, f["joint_p"].as_f64().unwrap());
    }
}

#[test]
fn invalid_restrictions_are_refused() {
    let cov = DMatrix::identity(2, 2);
    let beta = [1.0, 2.0];
    let normal = Reference::Asymptotic;
    assert_eq!(
        contrast(&beta, &cov, &[0.0, 0.0], 0.0, 0.95, normal).err(),
        Some(Error::SingularRestriction)
    );
    assert_eq!(
        contrast(&beta, &cov, &[1.0], 0.0, 0.95, normal).err(),
        Some(Error::Shape)
    );
    assert_eq!(
        contrast(&beta, &cov, &[1.0, 0.0], 0.0, 1.0, normal).err(),
        Some(Error::Confidence)
    );
    assert_eq!(
        contrast(
            &beta,
            &cov,
            &[1.0, 0.0],
            0.0,
            0.95,
            Reference::Student {
                degrees_of_freedom: 0.0
            }
        )
        .err(),
        Some(Error::DegreesOfFreedom)
    );
    let bad = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 2.0, 1.0]);
    assert_eq!(
        contrast(&beta, &bad, &[1.0, 0.0], 0.0, 0.95, normal).err(),
        Some(Error::InvalidCovariance)
    );
    let redundant = DMatrix::from_row_slice(2, 2, &[1.0, 0.0, 2.0, 0.0]);
    assert_eq!(
        joint(&beta, &cov, &redundant, &[0.0, 0.0], normal).err(),
        Some(Error::SingularRestriction)
    );
    assert_eq!(
        joint(&beta, &cov, &cov, &[f64::NAN, 0.0], normal).err(),
        Some(Error::NonFinite)
    );
    let asymmetric = DMatrix::from_row_slice(2, 2, &[1.0, 0.1, 0.0, 1.0]);
    assert_eq!(
        joint(&beta, &asymmetric, &cov, &[0.0, 0.0], normal).err(),
        Some(Error::InvalidCovariance)
    );
    let zero = DMatrix::zeros(2, 2);
    assert_eq!(
        joint(&beta, &zero, &cov, &[0.0, 0.0], normal).err(),
        Some(Error::SingularRestriction)
    );
}

#[test]
fn scalar_and_single_restriction_tests_agree() {
    let covariance = DMatrix::from_row_slice(2, 2, &[2.0, 0.5, 0.5, 1.0]);
    let beta = [0.3, 0.8];
    let r = DMatrix::from_row_slice(1, 2, &[1.0, -0.5]);
    for reference in [
        Reference::Asymptotic,
        Reference::Student {
            degrees_of_freedom: 7.5,
        },
    ] {
        let scalar = contrast(&beta, &covariance, &[1.0, -0.5], 0.2, 0.95, reference).unwrap();
        let (statistic, p) = match joint(&beta, &covariance, &r, &[0.2], reference).unwrap() {
            JointTest::ChiSquared {
                statistic, p_value, ..
            }
            | JointTest::F {
                statistic, p_value, ..
            } => (statistic, p_value),
        };
        close(statistic, scalar.statistic.powi(2));
        close(p, scalar.p_value);
    }
}
