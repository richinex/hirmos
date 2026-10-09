use hirmos_causal_core::synthetic_control::simplex_qp::{solve, Error, Outcome, Settings};
use nalgebra::{DMatrix, DVector};

#[test]
fn rejects_invalid_problems_and_distinguishes_iteration_limits() {
    for (h, c) in [
        (DMatrix::zeros(0, 0), DVector::zeros(0)),
        (-DMatrix::identity(2, 2), DVector::zeros(2)),
    ] {
        assert!(matches!(
            solve(&h, &c, Settings::synthetic_control()),
            Err(Error::InvalidInput)
        ));
    }
    let outcome = solve(
        &DMatrix::identity(2, 2),
        &DVector::zeros(2),
        Settings {
            max_iterations: 0,
            ..Settings::synthetic_control()
        },
    )
    .unwrap();
    assert!(matches!(outcome, Outcome::IterationLimit(_)));
}

#[test]
fn converged_weights_satisfy_the_simplex_and_known_optimum() {
    let outcome = solve(
        &DMatrix::identity(3, 3),
        &DVector::from_vec(vec![-0.2, -0.3, -0.5]),
        Settings::synthetic_control(),
    )
    .unwrap();
    let Outcome::Converged(fit) = outcome else {
        panic!("Expected convergence")
    };
    assert!((fit.weights.iter().sum::<f64>() - 1.0).abs() < 1e-6);
    assert!(fit.weights.iter().all(|w| *w >= 0.0));
    for (actual, expected) in fit.weights.iter().zip([0.2, 0.3, 0.5]) {
        assert!((actual - expected).abs() < 1e-4);
    }
    assert!(fit.primal_residual <= 1e-6 && fit.dual_residual <= 1e-6 && fit.relative_gap <= 1e-5);
}
