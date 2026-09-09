use hirmos_causal_core::survival::flexsurv::r_optim::{
    r_optim_bfgs, r_optim_bfgs_numeric, ROptimControl,
};
use std::num::NonZeroUsize;

fn count(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).unwrap()
}

fn rosenbrock(parameters: &[f64]) -> f64 {
    let x = parameters[0];
    let y = parameters[1];
    let curve_error = y - x * x;
    let location_error = 1.0 - x;
    100.0 * curve_error * curve_error + location_error * location_error
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn base_r_bfgs_numeric_rosenbrock() {
    let result = r_optim_bfgs_numeric(
        &[-1.2, 1.0],
        &ROptimControl::bfgs_defaults(count(2)),
        rosenbrock,
    )
    .unwrap();

    // R 4.4.1: optim(c(-1.2, 1), rosenbrock, method = "BFGS")
    assert!((result.parameters[0] - 0.999804433231344).abs() < 5e-12);
    assert!((result.parameters[1] - 0.999608380623381).abs() < 5e-12);
    assert!((result.value - 3.827383e-8).abs() < 5e-13);
    assert_eq!(result.function_count, 120);
    assert_eq!(result.gradient_count, 38);
    assert_eq!(result.convergence, 0);
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
pub fn base_r_bfgs_analytic_rosenbrock() {
    let result = r_optim_bfgs(
        &[-1.2, 1.0],
        &ROptimControl::bfgs_defaults(count(2)),
        rosenbrock,
        |parameters| {
            let x = parameters[0];
            let y = parameters[1];
            vec![
                -400.0 * x * (y - x * x) - 2.0 * (1.0 - x),
                200.0 * (y - x * x),
            ]
        },
    )
    .unwrap();

    // R 4.4.1 with the matching analytic gradient.
    assert!((result.parameters[0] - 1.0).abs() < 5e-8);
    assert!((result.parameters[1] - 1.0).abs() < 5e-8);
    assert!(result.value < 1e-15);
    assert_eq!(result.function_count, 110);
    assert_eq!(result.gradient_count, 43);
    assert_eq!(result.convergence, 0);
}
