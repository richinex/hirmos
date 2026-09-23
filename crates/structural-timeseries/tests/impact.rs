use hirmos_structural_timeseries::{
    impact::{self, Plan, Seasonality, Trend},
    Error,
};
use nalgebra::DMatrix;
use std::ops::ControlFlow;

fn inputs() -> (Vec<f64>, DMatrix<f64>) {
    let x = DMatrix::from_fn(54, 2, |i, j| {
        if j == 0 {
            (i % 12 < 2) as u8 as f64
        } else {
            (i as f64 * 0.7).cos()
        }
    });
    let y = (0..48)
        .map(|i| 20. + 0.08 * i as f64 + (i as f64 * 0.4).sin() + 2. * x[(i, 0)] + x[(i, 1)])
        .collect();
    (y, x)
}

#[test]
fn all_trend_seasonality_combinations_reconstruct_and_repeat() {
    let (y, x) = inputs();
    for trend in [Trend::Level, Trend::Linear, Trend::Semilocal] {
        for seasonality in [
            Seasonality::None,
            Seasonality::seasonal(12, 1).unwrap(),
            Seasonality::harmonic(12., 2).unwrap(),
        ] {
            let plan = Plan::new(&y, x.clone(), trend, seasonality, 12, 8, 17).unwrap();
            let a = impact::fit(&plan, |_| ControlFlow::Continue(())).unwrap();
            let b = impact::fit(&plan, |_| ControlFlow::Continue(())).unwrap();
            assert_eq!(a.paths, b.paths);
            assert_eq!(a.inclusion_probabilities, b.inclusion_probabilities);
            assert_eq!(a.inclusion_probabilities.len(), 2);
            assert!(a.inclusion_probabilities.iter().all(|p| (0.0..=1.0).contains(p)));
            for t in 0..54 {
                let reconstructed = a
                    .contributions
                    .iter()
                    .map(|c| c.paths[t].iter().sum::<f64>() / 12.)
                    .sum::<f64>();
                assert!((reconstructed - a.means[t]).abs() < 1e-10);
                assert_eq!(a.paths[t].len(), 12);
                assert!(a.paths[t].iter().all(|v| v.is_finite()));
            }
        }
    }
}

#[test]
fn future_predictors_do_not_change_training_draws() {
    let (y, x) = inputs();
    let mut changed = x.clone();
    for t in 48..54 {
        changed[(t, 0)] += 50.;
    }
    let fit = |x| {
        impact::fit(
            &Plan::new(
                &y,
                x,
                Trend::Semilocal,
                Seasonality::harmonic(12., 2).unwrap(),
                8,
                8,
                7,
            )
            .unwrap(),
            |_| ControlFlow::Continue(()),
        )
        .unwrap()
    };
    let a = fit(x);
    let b = fit(changed);
    assert_eq!(&a.paths[..48], &b.paths[..48]);
    for (a, b) in a.contributions.iter().zip(&b.contributions).take(2) {
        assert_eq!(a.paths, b.paths);
    }
    assert_ne!(&a.means[48..], &b.means[48..]);
}

#[test]
fn cancellation_and_invalid_training_are_explicit() {
    let (y, x) = inputs();
    let plan = Plan::new(&y, x.clone(), Trend::Level, Seasonality::None, 8, 8, 7).unwrap();
    assert!(matches!(
        impact::fit(&plan, |_| ControlFlow::Break(())),
        Err(Error::Cancelled)
    ));
    let mut holiday_only_after = x;
    for t in 0..48 {
        holiday_only_after[(t, 0)] = 0.;
    }
    assert!(Plan::new(
        &y,
        holiday_only_after,
        Trend::Level,
        Seasonality::None,
        8,
        8,
        7
    )
    .is_err());
    assert!(Seasonality::harmonic(12., 6).is_err());
    assert!(Seasonality::seasonal(12, 0).is_err());
    assert!(Plan::new(
        &vec![1.; 48],
        DMatrix::zeros(54, 0),
        Trend::Level,
        Seasonality::None,
        8,
        8,
        7
    )
    .is_err());
}

#[test]
fn no_predictor_model_runs_without_a_regression_placeholder() {
    let (y, _) = inputs();
    let plan = Plan::new(
        &y,
        DMatrix::zeros(54, 0),
        Trend::Linear,
        Seasonality::seasonal(12, 1).unwrap(),
        8,
        8,
        7,
    )
    .unwrap();
    let result = impact::fit(&plan, |_| ControlFlow::Continue(())).unwrap();
    assert_eq!(result.contributions.len(), 2);
    assert!(result.inclusion_probabilities.is_empty());
}
