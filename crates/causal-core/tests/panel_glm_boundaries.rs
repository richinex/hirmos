use nalgebra::DMatrix;
use hirmos_causal_core::{
    panel_design::{Error as DesignError, Panel},
    panel_glm::{self as regression, Error, Family, Uncertainty},
};
use std::collections::BTreeMap;
#[test]
fn keyed_lags_do_not_cross_units_or_gaps() {
    let keys = vec![(2, 3), (1, 1), (2, 1), (1, 3), (1, 2), (2, 2)];
    let values = DMatrix::from_column_slice(6, 1, &[23., 11., 21., 13., 12., 22.]);
    let p = Panel::new(&keys, values).unwrap();
    let d = p.distributed_lags(&[(0, 1, "lag1".into())]).unwrap();
    assert_eq!(d.keys, vec![(1, 2), (1, 3), (2, 2), (2, 3)]);
    assert_eq!(d.matrix.column(1).as_slice(), &[11., 12., 21., 22.]);
    assert_eq!(d.original_rows, vec![4, 3, 5, 0]);
    assert_eq!(d.omitted_rows, vec![1, 2]);
    assert!(matches!(
        Panel::new(&[(1, 1), (1, 3)], DMatrix::zeros(2, 1)),
        Err(DesignError::Gap)
    ));
    assert!(matches!(
        Panel::new(&[(1, 1), (1, 1)], DMatrix::zeros(2, 1)),
        Err(DesignError::DuplicateKey)
    ));
}
#[test]
fn event_design_uses_only_selected_cohort_and_never_controls() {
    let keys: Vec<_> = (0..3u64)
        .flat_map(|u| (0..5i64).map(move |t| (u, t)))
        .collect();
    let p = Panel::new(&keys, DMatrix::zeros(15, 0)).unwrap();
    let adoption = BTreeMap::from([(0, None), (1, Some(2)), (2, Some(3))]);
    let d = p.cohort_events(&adoption, 2, &[]).unwrap();
    assert_eq!(d.keys.len(), 10);
    assert!(!d.terms.iter().any(|s| s == "event[-1]"));
    assert_eq!(
        &d.terms[1..5],
        &["event[-2]", "event[0]", "event[1]", "event[2]"]
    );
    assert_eq!(d.matrix[(5, 1)], 1.0);
    assert_eq!(d.matrix[(6, 1)], 0.0);
    assert_eq!(d.matrix[(7, 2)], 1.0);
    assert_eq!(d.omitted_rows, vec![10, 11, 12, 13, 14]);
}
#[test]
fn invalid_model_inputs_are_refused() {
    let x = DMatrix::from_element(6, 1, 1.0);
    let y = vec![0., 1., 2., 3., 4., 5.];
    let n = vec![8.; 6];
    assert!(matches!(
        regression::fit(
            &x,
            Family::Binomial {
                successes: &y,
                trials: &[1.; 6]
            },
            100,
            1e-8
        ),
        Err(Error::Trials)
    ));
    assert!(matches!(
        regression::fit(
            &x,
            Family::NegativeBinomial {
                counts: &y,
                exposure: &[0.; 6]
            },
            100,
            1e-8
        ),
        Err(Error::Exposure)
    ));
    assert!(matches!(
        regression::fit(
            &DMatrix::from_element(6, 2, 1.0),
            Family::Binomial {
                successes: &y,
                trials: &n
            },
            100,
            1e-8
        ),
        Err(Error::Rank)
    ));
    assert!(matches!(
        regression::fit(
            &x,
            Family::Binomial {
                successes: &[0.; 6],
                trials: &n
            },
            100,
            1e-8
        ),
        Err(Error::Separation)
    ));
    let f = regression::fit(
        &x,
        Family::Binomial {
            successes: &y,
            trials: &n,
        },
        100,
        1e-8,
    )
    .unwrap();
    assert!(matches!(
        regression::covariance(
            &f,
            Uncertainty::Cluster {
                ids: &[1; 6],
                correction: true
            }
        ),
        Err(Error::Clusters)
    ));
    assert!(matches!(
        regression::covariance(
            &f,
            Uncertainty::Hac {
                times: &[0, 1, 2, 0, 1, 2],
                lags: 1,
                correction: true
            }
        ),
        Err(Error::TimeOrder)
    ));
    assert!(matches!(
        regression::contrast(&f.coefficients, &f.covariance, &[0.], 0.95),
        Err(Error::Contrast)
    ));
}

#[test]
fn panel_design_is_invariant_to_input_order() {
    let keys: Vec<_> = (0..3u64)
        .flat_map(|u| (0..6i64).map(move |t| (u, t)))
        .collect();
    let values = DMatrix::from_fn(keys.len(), 1, |i, _| i as f64);
    let expected = Panel::new(&keys, values.clone())
        .unwrap()
        .distributed_lags(&[(0, 2, "lag2".into())])
        .unwrap();
    for shift in 0..keys.len() {
        let order: Vec<_> = (0..keys.len())
            .map(|i| (i + shift) % keys.len())
            .rev()
            .collect();
        let shuffled: Vec<_> = order.iter().map(|i| keys[*i]).collect();
        let actual = Panel::new(
            &shuffled,
            DMatrix::from_fn(keys.len(), 1, |i, j| values[(order[i], j)]),
        )
        .unwrap()
        .distributed_lags(&[(0, 2, "lag2".into())])
        .unwrap();
        assert_eq!(actual.matrix, expected.matrix);
        assert_eq!(actual.keys, expected.keys);
        for (key, row) in actual.keys.iter().zip(actual.original_rows) {
            assert_eq!(*key, shuffled[row]);
        }
    }
}

#[test]
fn common_calendar_covariate_is_collinear_with_week_effects() {
    let keys: Vec<_> = (0..4u64)
        .flat_map(|u| (0..8i64).map(move |t| (u, t)))
        .collect();
    let holiday = DMatrix::from_fn(keys.len(), 1, |i, _| f64::from(keys[i].1 % 3 == 0));
    let design = Panel::new(&keys, holiday)
        .unwrap()
        .distributed_lags(&[(0, 0, "holiday".into())])
        .unwrap();
    assert!(matches!(
        regression::fit(
            &design.matrix,
            Family::NegativeBinomial {
                counts: &[2.; 32],
                exposure: &[10.; 32]
            },
            100,
            1e-8
        ),
        Err(Error::Rank)
    ));
}

#[test]
fn sum_interval_uses_cross_coefficient_covariance() {
    let covariance = DMatrix::from_row_slice(2, 2, &[4., -1., -1., 9.]);
    let result = regression::contrast(&[0.2, 0.3], &covariance, &[1., 1.], 0.95).unwrap();
    assert_eq!(result.estimate, 0.5);
    assert!((result.standard_error - 11f64.sqrt()).abs() < 1e-14);
}
