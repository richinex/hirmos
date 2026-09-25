use hirmos_causal_core::propensity::{ate, Error, Normalization, Specification, WeightScale};

fn spec(normalization: Normalization, scale: WeightScale) -> Specification {
    Specification {
        normalization,
        scale,
    }
}

#[test]
fn distinct_normalizations_and_stabilization() {
    let y = [2., 4., 1., 3.];
    let t = [true, true, false, false];
    let p = [0.2, 0.8, 0.3, 0.6];
    for scale in [WeightScale::InverseProbability, WeightScale::Stabilized] {
        let ht = ate(&y, &t, &p, spec(Normalization::HorvitzThompson, scale)).unwrap();
        let hajek = ate(&y, &t, &p, spec(Normalization::Hajek, scale)).unwrap();
        assert!((ht.effect - 1.5178571428571428).abs() < 1e-12);
        assert!((hajek.effect - 0.12727272727272743).abs() < 1e-12);
        assert_eq!(ht.treated_rows, 2);
        assert_eq!(ht.control_rows, 2);
        assert_eq!(ht.weights, hajek.weights);
    }
}

#[test]
fn unequal_arm_stabilization_preserves_each_estimand() {
    let y = [2., 4., 1., 3.];
    let t = [true, false, false, false];
    let p = [0.2, 0.8, 0.3, 0.6];
    for normalization in [Normalization::HorvitzThompson, Normalization::Hajek] {
        let raw = ate(
            &y,
            &t,
            &p,
            spec(normalization, WeightScale::InverseProbability),
        )
        .unwrap();
        let stabilized = ate(&y, &t, &p, spec(normalization, WeightScale::Stabilized)).unwrap();
        assert!((raw.effect - stabilized.effect).abs() < 1e-12);
        let oracle = match normalization {
            Normalization::HorvitzThompson => -4.732142857142858,
            Normalization::Hajek => -1.2399999999999993,
        };
        assert!((raw.effect - oracle).abs() < 1e-12);
        assert!((stabilized.weights[0] - raw.weights[0] * 0.25).abs() < 1e-12);
        assert!((stabilized.weights[1] - raw.weights[1] * 0.75).abs() < 1e-12);
    }
}

#[test]
fn invalid_inputs_are_not_clipped_or_dropped() {
    let s = spec(
        Normalization::HorvitzThompson,
        WeightScale::InverseProbability,
    );
    assert_eq!(ate(&[], &[], &[], s).unwrap_err(), Error::EmptySample);
    assert_eq!(
        ate(&[1., 2.], &[true], &[0.5, 0.5], s).unwrap_err(),
        Error::LengthMismatch
    );
    assert_eq!(
        ate(&[1., 2.], &[true, true], &[0.5, 0.5], s).unwrap_err(),
        Error::MissingArm { treated: false }
    );
    assert_eq!(
        ate(&[1., 2.], &[false, false], &[0.5, 0.5], s).unwrap_err(),
        Error::MissingArm { treated: true }
    );
    for p in [0., 1., -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert_eq!(
            ate(&[1., 2.], &[true, false], &[p, 0.5], s).unwrap_err(),
            Error::InvalidProbability { row: 0 }
        );
    }
    assert_eq!(
        ate(&[f64::NAN, 2.], &[true, false], &[0.5, 0.5], s).unwrap_err(),
        Error::NonFiniteOutcome { row: 0 }
    );
    assert_eq!(
        ate(&[f64::MAX, 2.], &[true, false], &[0.1, 0.5], s).unwrap_err(),
        Error::NumericalOverflow
    );
}
