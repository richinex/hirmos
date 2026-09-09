//! The non-penalized `survival::survreg` Newton kernel used by flexsurv's
//! automatic initial-value functions.
//!
//! This follows survival 3.6-4 `survreg6.c`, `survregc1.c`, `cholesky2.c`,
//! `cholesky3.c`, and `chsolve2.c`. It deliberately remains private: this is
//! an initialization dependency, not a second public survival estimator.

use std::f64::consts::SQRT_2;

const SMALL_LOG_LIKELIHOOD: f64 = -200.0;
const SQRT_TWO_PI: f64 = 2.506_628_274_631_001;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ErrorDistribution {
    ExtremeValue,
    Logistic,
    Gaussian,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Censoring {
    Right,
    Exact,
    Left,
    Interval,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Observation {
    pub lower: f64,
    pub upper: f64,
    pub censoring: Censoring,
    pub weight: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurvregResult {
    pub coefficients: Vec<f64>,
    pub iterations: usize,
    pub log_likelihood: f64,
    pub converged: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SurvregError {
    EmptyData,
    DesignLength { expected: usize, actual: usize },
    InitialLength { expected: usize, actual: usize },
    NonFiniteInput,
}

#[derive(Clone, Copy)]
pub(crate) struct Derivatives {
    log_likelihood: f64,
    pub location: f64,
    pub location_second: f64,
    scale: f64,
    scale_second: f64,
    cross: f64,
}

struct Likelihood {
    value: f64,
    score: Vec<f64>,
    information: Vec<f64>,
    outer_product: Vec<f64>,
}

fn distribution_values(distribution: ErrorDistribution, z: f64, density: bool) -> [f64; 4] {
    match distribution {
        ErrorDistribution::ExtremeValue => {
            let w = if z < SMALL_LOG_LIKELIHOOD {
                SMALL_LOG_LIKELIHOOD.exp()
            } else if -z < SMALL_LOG_LIKELIHOOD {
                (-SMALL_LOG_LIKELIHOOD).exp()
            } else {
                z.exp()
            };
            let survival = (-w).exp();
            if density {
                [0.0, w * survival, 1.0 - w, w * (w - 3.0) + 1.0]
            } else {
                [
                    1.0 - survival,
                    survival,
                    w * survival,
                    w * survival * (1.0 - w),
                ]
            }
        }
        ErrorDistribution::Logistic => {
            let (w, sign, first_index) = if z > 0.0 {
                ((-z).exp(), -1.0, 0)
            } else {
                (z.exp(), 1.0, 1)
            };
            let denominator = 1.0 + w;
            if density {
                [
                    0.0,
                    w / (denominator * denominator),
                    sign * (1.0 - w) / denominator,
                    (w * w - 4.0 * w + 1.0) / (denominator * denominator),
                ]
            } else {
                let mut values = [0.0; 4];
                values[1 - first_index] = w / denominator;
                values[first_index] = 1.0 / denominator;
                values[2] = w / (denominator * denominator);
                values[3] = sign * values[2] * (1.0 - w) / denominator;
                values
            }
        }
        ErrorDistribution::Gaussian => {
            let f = (-z * z / 2.0).exp() / SQRT_TWO_PI;
            if density {
                [0.0, f, -z, z * z - 1.0]
            } else if z > 0.0 {
                [
                    (1.0 + libm::erf(z / SQRT_2)) / 2.0,
                    libm::erfc(z / SQRT_2) / 2.0,
                    f,
                    -z * f,
                ]
            } else {
                [
                    libm::erfc(-z / SQRT_2) / 2.0,
                    (1.0 + libm::erf(-z / SQRT_2)) / 2.0,
                    f,
                    -z * f,
                ]
            }
        }
    }
}

pub(crate) fn row_derivatives(
    observation: Observation,
    eta: f64,
    sigma: f64,
    distribution: ErrorDistribution,
) -> Derivatives {
    let scaled = observation.lower - eta;
    let z = scaled / sigma;
    let inverse_sigma_squared = 1.0 / (sigma * sigma);

    match observation.censoring {
        Censoring::Exact => {
            let values = distribution_values(distribution, z, true);
            if values[1] <= 0.0 {
                return Derivatives {
                    log_likelihood: SMALL_LOG_LIKELIHOOD,
                    location: -z / sigma,
                    location_second: -1.0 / sigma,
                    scale: 0.0,
                    scale_second: 0.0,
                    cross: 0.0,
                };
            }
            let temp = values[2] / sigma;
            let temp_second = values[3] * inverse_sigma_squared;
            let location = -temp;
            let mut scale = -temp * scaled;
            let location_second = temp_second - location * location;
            let cross = scaled * temp_second - location * (scale + 1.0);
            let scale_second = scaled * scaled * temp_second - scale * (1.0 + scale);
            scale -= 1.0;
            Derivatives {
                log_likelihood: values[1].ln() - sigma.ln(),
                location,
                location_second,
                scale,
                scale_second,
                cross,
            }
        }
        Censoring::Right => {
            let values = distribution_values(distribution, z, false);
            if values[1] <= 0.0 {
                return Derivatives {
                    log_likelihood: SMALL_LOG_LIKELIHOOD,
                    location: z / sigma,
                    location_second: 0.0,
                    scale: 0.0,
                    scale_second: 0.0,
                    cross: 0.0,
                };
            }
            let temp = -values[2] / (values[1] * sigma);
            let temp_second = -values[3] * inverse_sigma_squared / values[1];
            let location = -temp;
            let scale = -temp * scaled;
            Derivatives {
                log_likelihood: values[1].ln(),
                location,
                location_second: temp_second - location * location,
                scale,
                scale_second: scaled * scaled * temp_second - scale * (1.0 + scale),
                cross: scaled * temp_second - location * (scale + 1.0),
            }
        }
        Censoring::Left => {
            let values = distribution_values(distribution, z, false);
            if values[0] <= 0.0 {
                return Derivatives {
                    log_likelihood: SMALL_LOG_LIKELIHOOD,
                    location: -z / sigma,
                    location_second: 0.0,
                    scale: 0.0,
                    scale_second: 0.0,
                    cross: 0.0,
                };
            }
            let temp = values[2] / (values[0] * sigma);
            let temp_second = values[3] * inverse_sigma_squared / values[0];
            let location = -temp;
            let scale = -temp * scaled;
            Derivatives {
                log_likelihood: values[0].ln(),
                location,
                location_second: temp_second - location * location,
                scale,
                scale_second: scaled * scaled * temp_second - scale * (1.0 + scale),
                cross: scaled * temp_second - location * (scale + 1.0),
            }
        }
        Censoring::Interval => {
            let upper_z = (observation.upper - eta) / sigma;
            let lower = distribution_values(distribution, z, false);
            let upper = distribution_values(distribution, upper_z, false);
            let probability = if z > 0.0 {
                lower[1] - upper[1]
            } else {
                upper[0] - lower[0]
            };
            if probability <= 0.0 {
                return Derivatives {
                    log_likelihood: SMALL_LOG_LIKELIHOOD,
                    location: 1.0,
                    location_second: 0.0,
                    scale: 0.0,
                    scale_second: 0.0,
                    cross: 0.0,
                };
            }
            let location = -(upper[2] - lower[2]) / (probability * sigma);
            let location_second =
                (upper[3] - lower[3]) * inverse_sigma_squared / probability - location * location;
            let scale = (z * lower[2] - upper_z * upper[2]) / probability;
            let scale_second = (upper_z * upper_z * upper[3] - z * z * lower[3]) / probability
                - scale * (1.0 + scale);
            let cross = (upper_z * upper[3] - z * lower[3]) / (probability * sigma)
                - location * (scale + 1.0);
            Derivatives {
                log_likelihood: probability.ln(),
                location,
                location_second,
                scale,
                scale_second,
                cross,
            }
        }
    }
}

fn likelihood(
    observations: &[Observation],
    design: &[f64],
    columns: usize,
    parameters: &[f64],
    estimated_scale: bool,
    fixed_log_scale: f64,
    distribution: ErrorDistribution,
    value_only: bool,
) -> Likelihood {
    let order = columns + usize::from(estimated_scale);
    let mut result = Likelihood {
        value: 0.0,
        score: vec![0.0; order],
        information: vec![0.0; order * order],
        outer_product: vec![0.0; order * order],
    };
    let log_scale = if estimated_scale {
        parameters[columns]
    } else {
        fixed_log_scale
    };
    let sigma = log_scale.exp();

    for (row, observation) in observations.iter().copied().enumerate() {
        let eta = (0..columns)
            .map(|column| parameters[column] * design[row * columns + column])
            .sum::<f64>();
        let derivatives = row_derivatives(observation, eta, sigma, distribution);
        result.value += derivatives.log_likelihood * observation.weight;
        if value_only {
            continue;
        }

        for left in 0..columns {
            let left_value = design[row * columns + left];
            let score = derivatives.location * left_value * observation.weight;
            result.score[left] += score;
            for right in 0..=left {
                let right_value = design[row * columns + right];
                result.information[left * order + right] -=
                    left_value * right_value * derivatives.location_second * observation.weight;
                result.outer_product[left * order + right] +=
                    score * right_value * derivatives.location;
            }
        }
        if estimated_scale {
            let scale_index = columns;
            result.score[scale_index] += observation.weight * derivatives.scale;
            for column in 0..columns {
                let value = design[row * columns + column];
                result.information[scale_index * order + column] -=
                    derivatives.cross * value * observation.weight;
                result.outer_product[scale_index * order + column] +=
                    derivatives.scale * value * derivatives.location * observation.weight;
            }
            result.information[scale_index * order + scale_index] -=
                derivatives.scale_second * observation.weight;
            result.outer_product[scale_index * order + scale_index] +=
                derivatives.scale * derivatives.scale * observation.weight;
        }
    }
    result
}

fn cholesky3(matrix: &mut [f64], order: usize, tolerance: f64) -> i32 {
    let mut epsilon = 0.0_f64;
    for index in 0..order {
        if matrix[index * order + index] < epsilon {
            epsilon = matrix[index * order + index];
        }
    }
    if epsilon == 0.0 {
        epsilon = tolerance;
    } else {
        epsilon *= tolerance;
    }

    let mut rank = 0_i32;
    let mut nonnegative = 1_i32;
    for column in 0..order {
        let pivot = matrix[column * order + column];
        if !pivot.is_finite() || pivot < epsilon {
            for row in column..order {
                matrix[row * order + column] = 0.0;
            }
            if pivot < -8.0 * epsilon {
                nonnegative = -1;
            }
        } else {
            rank += 1;
            for row in column + 1..order {
                let multiplier = matrix[row * order + column] / pivot;
                matrix[row * order + column] = multiplier;
                matrix[row * order + row] -= multiplier * multiplier * pivot;
                for lower in row + 1..order {
                    matrix[lower * order + row] -= multiplier * matrix[lower * order + column];
                }
            }
        }
    }
    rank * nonnegative
}

fn cholesky2(matrix: &mut [f64], order: usize, tolerance: f64) -> i32 {
    let mut epsilon = 0.0_f64;
    for row in 0..order {
        epsilon = epsilon.max(matrix[row * order + row]);
        for column in row + 1..order {
            matrix[column * order + row] = matrix[row * order + column];
        }
    }
    epsilon = if epsilon == 0.0 {
        tolerance
    } else {
        epsilon * tolerance
    };

    let mut rank = 0_i32;
    let mut nonnegative = 1_i32;
    for column in 0..order {
        let pivot = matrix[column * order + column];
        if !pivot.is_finite() || pivot < epsilon {
            matrix[column * order + column] = 0.0;
            if pivot < -8.0 * epsilon {
                nonnegative = -1;
            }
        } else {
            rank += 1;
            for row in column + 1..order {
                let multiplier = matrix[row * order + column] / pivot;
                matrix[row * order + column] = multiplier;
                matrix[row * order + row] -= multiplier * multiplier * pivot;
                for lower in row + 1..order {
                    matrix[lower * order + row] -= multiplier * matrix[lower * order + column];
                }
            }
        }
    }
    rank * nonnegative
}

fn cholesky_solve(matrix: &[f64], order: usize, right_hand_side: &mut [f64]) {
    for row in 0..order {
        let mut value = right_hand_side[row];
        for column in 0..row {
            value -= right_hand_side[column] * matrix[row * order + column];
        }
        right_hand_side[row] = value;
    }
    for row in (0..order).rev() {
        let diagonal = matrix[row * order + row];
        if diagonal == 0.0 {
            right_hand_side[row] = 0.0;
        } else {
            let mut value = right_hand_side[row] / diagonal;
            for lower in row + 1..order {
                value -= right_hand_side[lower] * matrix[lower * order + row];
            }
            right_hand_side[row] = value;
        }
    }
}

pub(crate) fn symmetric_solve(
    mut matrix: Vec<f64>,
    mut right_hand_side: Vec<f64>,
    tolerance: f64,
) -> Vec<f64> {
    let order = right_hand_side.len();
    cholesky2(&mut matrix, order, tolerance);
    cholesky_solve(&matrix, order, &mut right_hand_side);
    right_hand_side
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn fit(
    observations: &[Observation],
    design: &[f64],
    columns: usize,
    initial: &[f64],
    fixed_scale: Option<f64>,
    distribution: ErrorDistribution,
    maximum_iterations: usize,
    relative_tolerance: f64,
    cholesky_tolerance: f64,
) -> Result<SurvregResult, SurvregError> {
    if observations.is_empty() || columns == 0 {
        return Err(SurvregError::EmptyData);
    }
    let expected_design = observations.len() * columns;
    if design.len() != expected_design {
        return Err(SurvregError::DesignLength {
            expected: expected_design,
            actual: design.len(),
        });
    }
    let estimated_scale = fixed_scale.is_none();
    let order = columns + usize::from(estimated_scale);
    if initial.len() != order {
        return Err(SurvregError::InitialLength {
            expected: order,
            actual: initial.len(),
        });
    }
    if design.iter().chain(initial).any(|value| !value.is_finite())
        || observations.iter().any(|row| {
            !row.lower.is_finite()
                || (row.censoring == Censoring::Interval && !row.upper.is_finite())
                || !row.weight.is_finite()
                || row.weight <= 0.0
        })
    {
        return Err(SurvregError::NonFiniteInput);
    }
    let fixed_log_scale = fixed_scale.unwrap_or(1.0).ln();
    let mut parameters = initial.to_vec();
    let mut current = likelihood(
        observations,
        design,
        columns,
        &parameters,
        estimated_scale,
        fixed_log_scale,
        distribution,
        false,
    );
    let mut step = current.score.clone();
    let mut decomposition = current.information.clone();
    if cholesky3(&mut decomposition, order, cholesky_tolerance) < 0 {
        decomposition = current.outer_product.clone();
        cholesky3(&mut decomposition, order, cholesky_tolerance);
    }
    cholesky_solve(&decomposition, order, &mut step);
    let mut candidate = parameters
        .iter()
        .zip(step)
        .map(|(parameter, step)| parameter + step)
        .collect::<Vec<_>>();
    if maximum_iterations == 0 {
        return Ok(SurvregResult {
            coefficients: parameters,
            iterations: 0,
            log_likelihood: current.value,
            converged: true,
        });
    }

    let mut halving = 0_usize;
    let mut candidate_fit = likelihood(
        observations,
        design,
        columns,
        &candidate,
        estimated_scale,
        fixed_log_scale,
        distribution,
        false,
    );
    for iteration in 1..=maximum_iterations {
        if candidate_fit.value.is_finite()
            && (1.0 - current.value / candidate_fit.value).abs() <= relative_tolerance
            && halving == 0
        {
            return Ok(SurvregResult {
                coefficients: candidate,
                iterations: iteration,
                log_likelihood: candidate_fit.value,
                converged: true,
            });
        }

        if !candidate_fit.value.is_finite() || candidate_fit.value < current.value {
            for _ in 0..5 {
                if !(candidate_fit.value < current.value) {
                    break;
                }
                halving += 1;
                for index in 0..order {
                    candidate[index] = (candidate[index] + parameters[index]) / 2.0;
                }
                if halving == 1 && estimated_scale {
                    let scale_index = columns;
                    if parameters[scale_index] - candidate[scale_index] > 1.1 {
                        candidate[scale_index] = parameters[scale_index] - 1.1;
                    }
                }
                candidate_fit = likelihood(
                    observations,
                    design,
                    columns,
                    &candidate,
                    estimated_scale,
                    fixed_log_scale,
                    distribution,
                    true,
                );
            }
        } else {
            halving = 0;
            current = candidate_fit;
            let mut next_step = current.score.clone();
            let mut next_decomposition = current.information.clone();
            if cholesky3(&mut next_decomposition, order, cholesky_tolerance) < 0 {
                next_decomposition = current.outer_product.clone();
                cholesky3(&mut next_decomposition, order, cholesky_tolerance);
            }
            cholesky_solve(&next_decomposition, order, &mut next_step);
            parameters.clone_from(&candidate);
            for index in 0..order {
                candidate[index] += next_step[index];
            }
        }
        candidate_fit = likelihood(
            observations,
            design,
            columns,
            &candidate,
            estimated_scale,
            fixed_log_scale,
            distribution,
            false,
        );

        if iteration == maximum_iterations {
            return Ok(SurvregResult {
                coefficients: candidate,
                iterations: maximum_iterations,
                log_likelihood: candidate_fit.value,
                converged: false,
            });
        }
    }
    Ok(SurvregResult {
        coefficients: candidate,
        iterations: maximum_iterations,
        log_likelihood: candidate_fit.value,
        converged: false,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{fit, Censoring, ErrorDistribution, Observation};

    fn close(actual: f64, expected: f64, tolerance: f64, label: &str) {
        assert!(
            (actual - expected).abs() <= tolerance * expected.abs().max(1.0),
            "{label}: actual={actual:.17e}, expected={expected:.17e}"
        );
    }

    #[test]
    fn newton_kernel_matches_survival_3_6_4() {
        let fixture: Value =
            serde_json::from_str(include_str!("../../../oracle/fixtures/flexsurv.json")).unwrap();
        let ovarian = &fixture["ovarian"];
        let times = ovarian["futime"].as_array().unwrap();
        let events = ovarian["fustat"].as_array().unwrap();
        let ages = ovarian["age"].as_array().unwrap();
        let observations = times
            .iter()
            .zip(events)
            .map(|(time, event)| Observation {
                lower: time.as_f64().unwrap().ln(),
                upper: 0.0,
                censoring: if event.as_i64().unwrap() == 1 {
                    Censoring::Exact
                } else {
                    Censoring::Right
                },
                weight: 1.0,
            })
            .collect::<Vec<_>>();
        let design = ages
            .iter()
            .flat_map(|age| [1.0, age.as_f64().unwrap()])
            .collect::<Vec<_>>();
        let log_jacobian = times
            .iter()
            .zip(events)
            .filter(|(_, event)| event.as_i64().unwrap() == 1)
            .map(|(time, _)| -time.as_f64().unwrap().ln())
            .sum::<f64>();
        let cases = [
            (
                "weibull_explicit",
                ErrorDistribution::ExtremeValue,
                None,
                vec![6.0, 0.0, 0.0],
            ),
            (
                "exponential_explicit",
                ErrorDistribution::ExtremeValue,
                Some(1.0),
                vec![6.0, 0.0],
            ),
            (
                "lognormal_explicit",
                ErrorDistribution::Gaussian,
                None,
                vec![6.0, 0.0, 0.0],
            ),
            (
                "loglogistic_explicit",
                ErrorDistribution::Logistic,
                None,
                vec![6.0, 0.0, 0.0],
            ),
        ];
        for (name, distribution, fixed_scale, initial) in cases {
            let expected = &fixture["survreg"][name];
            let actual = fit(
                &observations,
                &design,
                2,
                &initial,
                fixed_scale,
                distribution,
                30,
                1e-9,
                1e-10,
            )
            .unwrap();
            let coefficients = expected["coefficients"].as_array().unwrap();
            close(
                actual.coefficients[0],
                coefficients[0].as_f64().unwrap(),
                2e-11,
                name,
            );
            close(
                actual.coefficients[1],
                coefficients[1].as_f64().unwrap(),
                2e-11,
                name,
            );
            if fixed_scale.is_none() {
                close(
                    actual.coefficients[2].exp(),
                    expected["scale"].as_f64().unwrap(),
                    2e-11,
                    name,
                );
            }
            close(
                actual.log_likelihood + log_jacobian,
                expected["log_likelihood"].as_f64().unwrap(),
                2e-10,
                name,
            );
            assert_eq!(
                actual.iterations,
                expected["iterations"].as_u64().unwrap() as usize,
                "{name} iterations"
            );
            assert!(actual.converged, "{name} convergence");
        }
    }
}
