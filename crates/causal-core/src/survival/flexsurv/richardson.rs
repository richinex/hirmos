//! `numDeriv::hessian(method = "Richardson")` as used by flexsurv's `.hessian`.

#[derive(Clone, Debug, PartialEq)]
pub enum RichardsonError {
    EmptyParameters,
    TooFewIterations,
    NonFiniteEvaluation,
}

fn extrapolate(values: &mut [f64], iterations: usize) {
    for level in 1..iterations {
        let scale = 4_f64.powi(level as i32);
        for index in 0..(iterations - level) {
            values[index] = (values[index + 1] * scale - values[index]) / (scale - 1.0);
        }
    }
}

/// Reproduces the scalar-output Hessian path in numDeriv 2016.8-1.1.
pub(crate) fn richardson_hessian<F>(
    parameters: &[f64],
    iterations: usize,
    mut objective: F,
) -> Result<Vec<f64>, RichardsonError>
where
    F: FnMut(&[f64]) -> f64,
{
    if parameters.is_empty() {
        return Err(RichardsonError::EmptyParameters);
    }
    if iterations < 2 {
        return Err(RichardsonError::TooFewIterations);
    }

    const D: f64 = 0.1;
    const EPSILON: f64 = 1e-4;
    let zero_tolerance = (f64::EPSILON / 7e-7).sqrt();
    let center = objective(parameters);
    if !center.is_finite() {
        return Err(RichardsonError::NonFiniteEvaluation);
    }

    let order = parameters.len();
    let initial_steps = parameters
        .iter()
        .map(|value| {
            D * value.abs()
                + if value.abs() < zero_tolerance {
                    EPSILON
                } else {
                    0.0
                }
        })
        .collect::<Vec<_>>();
    let mut diagonal = vec![0.0; order];
    let mut first = vec![0.0; iterations];
    let mut second = vec![0.0; iterations];

    for parameter in 0..order {
        let mut steps = initial_steps.clone();
        for iteration in 0..iterations {
            let mut plus = parameters.to_vec();
            let mut minus = parameters.to_vec();
            plus[parameter] += steps[parameter];
            minus[parameter] -= steps[parameter];
            let right = objective(&plus);
            let left = objective(&minus);
            if !right.is_finite() || !left.is_finite() {
                return Err(RichardsonError::NonFiniteEvaluation);
            }
            first[iteration] = (right - left) / (2.0 * steps[parameter]);
            second[iteration] = (right - 2.0 * center + left) / steps[parameter].powi(2);
            for step in &mut steps {
                *step /= 2.0;
            }
        }
        extrapolate(&mut first, iterations);
        extrapolate(&mut second, iterations);
        diagonal[parameter] = second[0];
    }

    let mut hessian = vec![0.0; order * order];
    for row in 0..order {
        hessian[row * order + row] = diagonal[row];
        for column in 0..row {
            let mut steps = initial_steps.clone();
            for iteration in 0..iterations {
                let mut plus = parameters.to_vec();
                let mut minus = parameters.to_vec();
                plus[row] += steps[row];
                plus[column] += steps[column];
                minus[row] -= steps[row];
                minus[column] -= steps[column];
                let right = objective(&plus);
                let left = objective(&minus);
                if !right.is_finite() || !left.is_finite() {
                    return Err(RichardsonError::NonFiniteEvaluation);
                }
                first[iteration] = (right - 2.0 * center + left
                    - diagonal[row] * steps[row].powi(2)
                    - diagonal[column] * steps[column].powi(2))
                    / (2.0 * steps[row] * steps[column]);
                for step in &mut steps {
                    *step /= 2.0;
                }
            }
            extrapolate(&mut first, iterations);
            hessian[row * order + column] = first[0];
            hessian[column * order + row] = first[0];
        }
    }
    Ok(hessian)
}
