//! SciPy 1.17.1 `_minimize_neldermead` with standard coefficients, default
//! simplex/tolerances and a finite iteration budget (no evaluation limit).
//! This is the configuration used by lifelines' univariate initialization.

#[derive(Debug, PartialEq)]
pub(crate) enum Termination {
    Converged,
    IterationLimit,
}

pub(crate) struct Minimum {
    pub parameters: Vec<f64>,
    pub value: f64,
    pub iterations: usize,
    pub evaluations: usize,
    pub termination: Termination,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::survival::aft::{AftFamily, AftPreparation};
    use serde_json::Value;

    fn numbers(value: &Value) -> Vec<f64> {
        value
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect()
    }

    #[test]
    fn initialization_trajectories_match_lifelines() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../reproductions/clsa/data/aft-optimizer.json"
        ))
        .unwrap();
        let times = numbers(&fixture["data"]["duration"]);
        let events: Vec<bool> = numbers(&fixture["data"]["event"])
            .iter()
            .map(|v| *v == 1.0)
            .collect();
        let data = AftPreparation::new(1, 2, vec![1.0; times.len() * 2], times, events).unwrap();
        let close = |actual: f64, expected: f64| {
            assert!(
                (actual - expected).abs() <= 2e-13 * expected.abs().max(1.0),
                "{actual} != {expected}"
            );
        };
        for case in fixture["cases"].as_array().unwrap() {
            let family = match case["model"].as_str().unwrap() {
                "WeibullAFTFitter" => AftFamily::Weibull,
                "LogLogisticAFTFitter" => AftFamily::LogLogistic,
                other => panic!("unexpected model {other}"),
            };
            let call = case["optimizer_calls"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["method"] == "Nelder-Mead")
                .unwrap();
            let trials = call["evaluations"].as_array().unwrap();
            let callbacks = call["iterations"].as_array().unwrap();
            let mut trial = 0;
            let mut iteration = 0;
            let fit = minimize(
                &numbers(&call["initial"]),
                &[1e-9; 2],
                &[f64::INFINITY; 2],
                400,
                |point| {
                    for (actual, expected) in
                        point.iter().zip(numbers(&trials[trial]["parameters"]))
                    {
                        close(*actual, expected);
                    }
                    let value = data
                        .evaluate(
                            family,
                            &point.iter().map(|p| p.ln()).collect::<Vec<_>>(),
                            0.0,
                        )?
                        .objective;
                    close(value, trials[trial]["objective"].as_f64().unwrap());
                    trial += 1;
                    Ok::<_, crate::survival::aft::AftError>(value)
                },
                |point| {
                    for (actual, expected) in point.iter().zip(numbers(&callbacks[iteration])) {
                        close(*actual, expected);
                    }
                    iteration += 1;
                },
            )
            .unwrap();
            assert_eq!(trial, trials.len());
            assert_eq!(iteration, callbacks.len());
            assert_eq!(fit.evaluations, trials.len());
            assert_eq!(
                fit.iterations as u64,
                call["result"]["iterations"].as_u64().unwrap()
            );
            assert_eq!(fit.termination, Termination::Converged);
            for (actual, expected) in fit
                .parameters
                .iter()
                .zip(numbers(&call["result"]["parameters"]))
            {
                close(*actual, expected);
            }
            close(fit.value, call["result"]["objective"].as_f64().unwrap());
        }
    }
}

#[derive(Debug)]
pub(crate) enum Error<E> {
    InvalidInput,
    Evaluation(E),
}

fn sort(simplex: &mut Vec<Vec<f64>>, values: &mut Vec<f64>) {
    let order = crate::numpy_argsort::argsort(values);
    *simplex = order.iter().map(|&i| simplex[i].clone()).collect();
    *values = order.iter().map(|&i| values[i]).collect();
}

pub(crate) fn minimize<E>(
    initial: &[f64],
    lower: &[f64],
    upper: &[f64],
    max_iterations: usize,
    mut objective: impl FnMut(&[f64]) -> Result<f64, E>,
    mut on_iteration: impl FnMut(&[f64]),
) -> Result<Minimum, Error<E>> {
    let n = initial.len();
    if n == 0
        || lower.len() != n
        || upper.len() != n
        || initial.iter().any(|v| !v.is_finite())
        || lower
            .iter()
            .zip(upper)
            .any(|(lo, hi)| lo.is_nan() || hi.is_nan() || lo > hi)
    {
        return Err(Error::InvalidInput);
    }
    let clip = |point: &mut [f64]| {
        for i in 0..n {
            point[i] = point[i].max(lower[i]).min(upper[i]);
        }
    };
    let mut start = initial.to_vec();
    clip(&mut start);
    let mut simplex = vec![start.clone(); n + 1];
    for i in 0..n {
        simplex[i + 1][i] = if start[i] != 0.0 {
            1.05 * start[i]
        } else {
            0.00025
        };
    }
    for point in &mut simplex {
        for i in 0..n {
            if point[i] > upper[i] {
                point[i] = 2.0 * upper[i] - point[i];
            }
        }
        clip(point);
    }
    let mut evaluations = 0;
    let mut evaluate = |point: &[f64]| {
        evaluations += 1;
        objective(point).map_err(Error::Evaluation)
    };
    let mut values = simplex
        .iter()
        .map(|p| evaluate(p))
        .collect::<Result<Vec<_>, _>>()?;
    sort(&mut simplex, &mut values);
    sort(&mut simplex, &mut values);
    let mut iterations = 1;
    while iterations < max_iterations {
        let small_simplex = simplex[1..].iter().all(|p| {
            p.iter()
                .zip(&simplex[0])
                .all(|(a, b)| (a - b).abs() <= 1e-4)
        });
        let small_values = values[1..].iter().all(|v| (values[0] - v).abs() <= 1e-4);
        if small_simplex && small_values {
            break;
        }
        let centroid: Vec<f64> = (0..n)
            .map(|i| simplex[..n].iter().map(|p| p[i]).sum::<f64>() / n as f64)
            .collect();
        let mut reflected: Vec<f64> = (0..n).map(|i| 2.0 * centroid[i] - simplex[n][i]).collect();
        clip(&mut reflected);
        let reflection_value = evaluate(&reflected)?;
        let mut shrink = false;
        if reflection_value < values[0] {
            let mut expanded: Vec<f64> = (0..n)
                .map(|i| 3.0 * centroid[i] - 2.0 * simplex[n][i])
                .collect();
            clip(&mut expanded);
            let expansion_value = evaluate(&expanded)?;
            if expansion_value < reflection_value {
                simplex[n] = expanded;
                values[n] = expansion_value;
            } else {
                simplex[n] = reflected;
                values[n] = reflection_value;
            }
        } else if reflection_value < values[n - 1] {
            simplex[n] = reflected;
            values[n] = reflection_value;
        } else if reflection_value < values[n] {
            let mut contracted: Vec<f64> = (0..n)
                .map(|i| 1.5 * centroid[i] - 0.5 * simplex[n][i])
                .collect();
            clip(&mut contracted);
            let contraction_value = evaluate(&contracted)?;
            if contraction_value <= reflection_value {
                simplex[n] = contracted;
                values[n] = contraction_value;
            } else {
                shrink = true;
            }
        } else {
            let mut contracted: Vec<f64> = (0..n)
                .map(|i| 0.5 * centroid[i] + 0.5 * simplex[n][i])
                .collect();
            clip(&mut contracted);
            let contraction_value = evaluate(&contracted)?;
            if contraction_value < values[n] {
                simplex[n] = contracted;
                values[n] = contraction_value;
            } else {
                shrink = true;
            }
        }
        if shrink {
            for j in 1..=n {
                for i in 0..n {
                    simplex[j][i] = simplex[0][i] + 0.5 * (simplex[j][i] - simplex[0][i]);
                }
                clip(&mut simplex[j]);
                values[j] = evaluate(&simplex[j])?;
            }
        }
        iterations += 1;
        sort(&mut simplex, &mut values);
        on_iteration(&simplex[0]);
    }
    Ok(Minimum {
        parameters: simplex[0].clone(),
        value: values[0],
        iterations,
        evaluations,
        termination: if iterations >= max_iterations {
            Termination::IterationLimit
        } else {
            Termination::Converged
        },
    })
}
