use hirmos_causal_core::estimation::{
    weighted_within_convention, WithinConvention, WithinErrors, WithinProblem,
};
use nalgebra::DMatrix;

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 2e-8 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

#[test]
fn changing_predictor_units_preserves_the_fit_and_transforms_the_full_covariance() {
    let n = 120;
    let units: Vec<u64> = (0..n).map(|i| (i / 10) as u64).collect();
    let weights: Vec<f64> = (0..n).map(|i| 1.0 + (i % 7) as f64).collect();
    let x = DMatrix::from_fn(n, 5, |i, j| {
        let t = (i % 10) as f64;
        let z = ((i * 13) % 17) as f64;
        match j {
            0 => t,
            1 => 2.0 * t,
            2 => units[i] as f64 + 0.1,
            3 => z,
            _ => t * t,
        }
    });
    let y: Vec<f64> = (0..n)
        .map(|i| {
            2.0 * x[(i, 0)] - 0.5 * x[(i, 3)]
                + 0.1 * x[(i, 4)]
                + units[i] as f64
                + ((i * 7) % 11) as f64
        })
        .collect();
    let factors = [-1e-80, 1e70, 1e-40, 1e80, 1e-70];
    let changed = DMatrix::from_fn(n, 5, |i, j| x[(i, j)] * factors[j]);
    for errors in [
        WithinErrors::Classical,
        WithinErrors::Hc1,
        WithinErrors::Cluster,
    ] {
        let baseline = weighted_within_convention(
            &y,
            &x,
            &units,
            None,
            &weights,
            errors,
            WithinConvention::Xtreg,
        )
        .unwrap();
        let scaled = weighted_within_convention(
            &y,
            &changed,
            &units,
            None,
            &weights,
            errors,
            WithinConvention::Xtreg,
        )
        .unwrap();
        assert_eq!(baseline.kept, vec![0, 3, 4]);
        assert_eq!(baseline.kept, scaled.kept);
        assert_eq!(baseline.dropped, scaled.dropped);
        assert_eq!(baseline.cluster_parameters, scaled.cluster_parameters);
        close(baseline.ssr, scaled.ssr);
        close(baseline.rsquared_within, scaled.rsquared_within);
        for i in 0..n {
            close(baseline.resid[i], scaled.resid[i]);
        }
        for (i, &column) in baseline.kept.iter().enumerate() {
            close(baseline.params[i], scaled.params[i] * factors[column]);
            close(baseline.bse[i], scaled.bse[i] * factors[column].abs());
            close(baseline.pvalues[i], scaled.pvalues[i]);
            for (j, &other) in baseline.kept.iter().enumerate() {
                close(
                    baseline.covariance[(i, j)],
                    scaled.covariance[(i, j)] * factors[column] * factors[other],
                );
            }
            for row in 0..n {
                close(
                    baseline.within_design[(row, i)],
                    scaled.within_design[(row, i)] / factors[column],
                );
            }
        }
    }
}

#[test]
fn a_fully_absorbed_design_is_refused_even_with_projection_roundoff() {
    let n = 60;
    let units: Vec<u64> = (0..n).map(|i| (i / 10) as u64).collect();
    let x = DMatrix::from_fn(n, 2, |i, j| (units[i] as f64 + 0.1) * [1e-80, 1e80][j]);
    let result = weighted_within_convention(
        &vec![1.0; n],
        &x,
        &units,
        None,
        &vec![0.7; n],
        WithinErrors::Cluster,
        WithinConvention::Xtreg,
    );
    assert!(matches!(result, Err(WithinProblem::AbsorbedDesign)));
}
