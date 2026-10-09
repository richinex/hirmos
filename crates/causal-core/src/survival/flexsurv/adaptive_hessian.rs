//! Estimate a scalar function's Hessian and its numerical error.
// Numdifftools 0.11.1 central Hessian, Richardson, DEA3 and estimate selection.
// Copyright (c) 2009-2026 Per A. Brodtkorb, John D'Errico.
// BSD-3-Clause; see licenses/upstream/numdifftools-LICENSE.txt.

#[derive(Clone, Debug, PartialEq)]
pub struct Estimate {
    pub values: Vec<f64>,
    pub errors: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum HessianError {
    InvalidParameters,
    NonFiniteCenter,
    NoFiniteSteps { row: usize, column: usize },
    InvalidSequence,
    NonFiniteExtrapolation,
}

#[derive(Debug, PartialEq)]
pub enum AccuracyError {
    InvalidTolerance,
    Insufficient {
        entry: usize,
        estimated_error: f64,
        allowed_error: f64,
    },
}

impl Estimate {
    pub fn check_accuracy(&self, absolute: f64, relative: f64) -> Result<(), AccuracyError> {
        if !absolute.is_finite()
            || !relative.is_finite()
            || absolute < 0.0
            || relative < 0.0
            || absolute + relative == 0.0
        {
            return Err(AccuracyError::InvalidTolerance);
        }
        for (entry, (value, error)) in self.values.iter().zip(&self.errors).enumerate() {
            let allowed_error = absolute + relative * value.abs();
            if !value.is_finite() || !error.is_finite() || *error > allowed_error {
                return Err(AccuracyError::Insufficient {
                    entry,
                    estimated_error: *error,
                    allowed_error,
                });
            }
        }
        Ok(())
    }
}

pub fn select(sequence: &[f64]) -> Result<(f64, f64), HessianError> {
    if sequence.len() < 5 || sequence.iter().any(|x| !x.is_finite()) {
        return Err(HessianError::InvalidSequence);
    }
    let r = 1.6_f64.powi(2);
    let denominator = (r - 1.0) * (r * r - 1.0);
    let rule = [
        1.0 / denominator,
        -(r + r * r) / denominator,
        r.powi(3) / denominator,
    ];
    let richardson = sequence
        .windows(3)
        .map(|w| w.iter().zip(rule).map(|(v, c)| v * c).sum::<f64>())
        .collect::<Vec<_>>();
    let mut estimates = Vec::new();
    let mut errors = Vec::new();
    for w in richardson.windows(3) {
        let d1 = w[1] - w[0];
        let d2 = w[2] - w[1];
        let e1 = d1.abs();
        let e2 = d2.abs();
        let t1 = w[0].abs().max(w[1].abs()) * f64::EPSILON;
        let t2 = w[1].abs().max(w[2].abs()) * f64::EPSILON;
        let d1 = if e1 < f64::MIN_POSITIVE {
            f64::MIN_POSITIVE
        } else {
            d1
        };
        let d2 = if e2 < f64::MIN_POSITIVE {
            f64::MIN_POSITIVE
        } else {
            d2
        };
        let ss = 1.0 / d2 - 1.0 / d1;
        let converged = e1 <= t1 || e2 <= t2 || (ss * w[1]).abs() <= 1e-4;
        let result = if converged { w[2] } else { w[1] + 1.0 / ss };
        let error = e1
            + e2
            + if converged {
                t2 * 10.0
            } else {
                (result - w[2]).abs()
            };
        if !result.is_finite() || !error.is_finite() {
            return Err(HessianError::NonFiniteExtrapolation);
        }
        estimates.push(result);
        errors.push(error);
    }
    let mut sorted = estimates.clone();
    sorted.sort_by(f64::total_cmp);
    let quantile = |p: f64| {
        let position = p * (sorted.len() - 1) as f64;
        let lo = position.floor() as usize;
        let hi = position.ceil() as usize;
        sorted[lo] + (sorted[hi] - sorted[lo]) * (position - lo as f64)
    };
    let q1 = quantile(0.25);
    let median = quantile(0.5);
    let q3 = quantile(0.75);
    let iqr = (q3 - q1).abs();
    for (value, error) in estimates.iter().zip(&mut errors) {
        let magnitude_outlier = median.abs() > 1e-8
            && (value.abs() < median.abs() / 10.0 || value.abs() > median.abs() * 10.0);
        let iqr_outlier = *value < q1 - 1.5 * iqr || *value > q3 + 1.5 * iqr;
        *error += usize::from(magnitude_outlier || iqr_outlier) as f64 * (value - median).abs();
    }
    let best = errors.iter().copied().fold(f64::INFINITY, f64::min);
    let ties = errors
        .iter()
        .enumerate()
        .filter(|(_, e)| **e == best)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let index = ties[ties.len() / 2];
    Ok((estimates[index], errors[index]))
}

pub fn hessian(x: &[f64], f: impl Fn(&[f64]) -> f64) -> Result<Estimate, HessianError> {
    if x.is_empty() || x.iter().any(|v| !v.is_finite()) {
        return Err(HessianError::InvalidParameters);
    }
    let n = x.len();
    let center = f(x);
    if !center.is_finite() {
        return Err(HessianError::NonFiniteCenter);
    }
    let base = x
        .iter()
        .map(|v| {
            let h = 2.0 * (1.718281828459045 + v.abs()).ln().max(1.0);
            (h + 1.0) - 1.0
        })
        .collect::<Vec<_>>();
    let mut sequences = vec![Vec::new(); n * n];
    for k in 0..15 {
        let steps = base
            .iter()
            .map(|h| h * 1.6_f64.powi(-k))
            .collect::<Vec<_>>();
        for i in 0..n {
            for j in i..n {
                let eval = |a: f64, b: f64| {
                    let mut p = x.to_vec();
                    p[i] += a * steps[i];
                    p[j] += b * steps[j];
                    f(&p)
                };
                let value = if i == j {
                    let mut plus = x.to_vec();
                    let mut minus = x.to_vec();
                    plus[i] += 2.0 * steps[i];
                    minus[i] -= 2.0 * steps[i];
                    (f(&plus) - 2.0 * center + f(&minus)) / (4.0 * steps[i] * steps[i])
                } else {
                    (eval(1.0, 1.0) - eval(1.0, -1.0) - eval(-1.0, 1.0) + eval(-1.0, -1.0))
                        / (4.0 * steps[i] * steps[j])
                };
                sequences[i * n + j].push(value);
            }
        }
    }
    let mut result = Estimate {
        values: vec![0.0; n * n],
        errors: vec![0.0; n * n],
    };
    for i in 0..n {
        for j in i..n {
            let sequence = &sequences[i * n + j];
            let start = sequence
                .iter()
                .position(|v| v.is_finite())
                .ok_or(HessianError::NoFiniteSteps { row: i, column: j })?;
            let (value, error) = select(&sequence[start..])?;
            result.values[i * n + j] = value;
            result.values[j * n + i] = value;
            result.errors[i * n + j] = error;
            result.errors[j * n + i] = error;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn numbers(v: &serde_json::Value) -> Vec<f64> {
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_f64().unwrap())
            .collect()
    }
    fn scaled_error(a: &[f64], b: &[f64]) -> f64 {
        assert_eq!(a.len(), b.len());
        a.iter()
            .zip(b)
            .map(|(x, y)| {
                assert!(x.is_finite() && y.is_finite());
                (x - y).abs() / y.abs().max(1.0)
            })
            .fold(0.0, f64::max)
    }
    #[test]
    fn adaptive_accuracy_is_explicit() {
        let estimate = Estimate {
            values: vec![1.0],
            errors: vec![0.1],
        };
        assert!(matches!(
            estimate.check_accuracy(0.01, 0.01),
            Err(AccuracyError::Insufficient { entry: 0, .. })
        ));
        assert!(estimate.check_accuracy(0.2, 0.0).is_ok());
        assert_eq!(
            estimate.check_accuracy(-1.0, 0.0),
            Err(AccuracyError::InvalidTolerance)
        );
    }

    #[test]
    fn adaptive_matches_pinned_reference() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/adaptive-hessian/adaptive_reference.json"
        ))
        .unwrap();
        for case in fixture["sequences"].as_array().unwrap() {
            let seq = numbers(&case["values"]);
            let (value, error) = select(&seq).unwrap();
            assert!((value - case["estimate"].as_f64().unwrap()).abs() < 1e-11);
            assert!((error - case["error"].as_f64().unwrap()).abs() < 1e-11);
        }
        for case in fixture["cases"].as_array().unwrap() {
            let x = numbers(&case["x"]);
            let name = case["name"].as_str().unwrap();
            let result = hessian(&x, |p| match name {
                "quadratic" => 3.0 * p[0] * p[0] + 2.0 * p[0] * p[1] + 7.0 * p[1] * p[1],
                "rosenbrock" => (1.0 - p[0]).powi(2) + 100.0 * (p[1] - p[0] * p[0]).powi(2),
                "smooth" => p[0].exp() + (p[0] * p[1]).sin() + (p[1] * p[1]).ln_1p(),
                _ => panic!("Unknown fixture"),
            })
            .unwrap();
            assert!(scaled_error(&result.values, &numbers(&case["values"])) < 1e-9);
            assert!(scaled_error(&result.errors, &numbers(&case["errors"])) < 1e-9);
        }
    }

    #[test]
    fn adaptive_rejects_invalid_input() {
        assert!(hessian(&[], |_| 0.0).is_err());
        assert!(hessian(&[0.0], |_| f64::NAN).is_err());
        assert!(select(&[1.0; 4]).is_err());
        assert!(select(&[1.0, 2.0, f64::NAN, 3.0, 4.0]).is_err());
    }

    #[test]
    fn adaptive_domain_boundaries() {
        let bounded = hessian(&[0.0], |p| {
            if p[0].abs() < 0.1 {
                p[0] * p[0]
            } else {
                f64::NAN
            }
        })
        .unwrap();
        assert!((bounded.values[0] - 2.0).abs() < 1e-10);
        assert!(hessian(&[0.0], |p| if p[0] == 0.0 { 0.0 } else { f64::NAN }).is_err());
        assert!(hessian(&[0.0], |p| if (0.01..0.02).contains(&p[0].abs()) {
            f64::NAN
        } else {
            p[0] * p[0]
        })
        .is_err());
    }

    #[test]
    fn adaptive_quadratic_scales_and_permutations() {
        for offset in [0.0, 1e4] {
            for scale in [0.01, 1.0, 100.0] {
                let f = |x: &[f64]| {
                    offset + scale * x[0] * x[0] + 0.3 * x[0] * x[1] + x[1] * x[1] / scale
                };
                let x = [0.4, -0.7];
                let expected = [2.0 * scale, 0.3, 0.3, 2.0 / scale];
                let result = hessian(&x, f).unwrap();
                assert!(scaled_error(&result.values, &expected) < 1e-7);
                let permuted = hessian(&[x[1], x[0]], |p| f(&[p[1], p[0]])).unwrap();
                let restored = [
                    permuted.values[3],
                    permuted.values[2],
                    permuted.values[1],
                    permuted.values[0],
                ];
                assert!(scaled_error(&result.values, &restored) < 1e-7);
            }
        }
    }

    #[test]
    fn adaptive_singular_curvature_is_not_an_identified_covariance() {
        use super::super::covariance::{hessian_to_covariance, CovarianceError};
        let estimate = hessian(&[0.0, 0.0], |p| 0.5 * p[0] * p[0]).unwrap();
        assert!(matches!(
            hessian_to_covariance(&estimate.values, 2),
            Err(CovarianceError::SingularHessian { .. })
        ));
    }

    #[test]
    fn adaptive_nearly_singular_covariance_matches_r_repair() {
        use super::super::covariance::hessian_to_covariance;
        let reference: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/adaptive-hessian/near-singular.json"
        ))
        .unwrap();
        for case in reference["cases"].as_array().unwrap() {
            let epsilon = case["epsilon"].as_f64().unwrap();
            let estimate = hessian(&[0.0, 0.0], |p| {
                0.5 * (p[0] + p[1]).powi(2) + 0.5 * epsilon * p[1].powi(2)
            })
            .unwrap();
            let covariance = hessian_to_covariance(&estimate.values, 2).unwrap();
            let expected = numbers(&case["covariance"]);
            assert!(
                scaled_error(&covariance.values, &expected) < 1e-7,
                "epsilon={epsilon}, covariance={:?}",
                covariance.values
            );
        }
    }
}
