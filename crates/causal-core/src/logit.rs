//! statsmodels `Logit` at its Newton-Raphson default, whose every step is a LAPACK LU solve.
//!
//! The `logistic` module ports sklearn's `LogisticRegression`, which is a different estimator.

use nalgebra::DMatrix;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogitError {
    EmptySample,
    NoColumns,
    RowMismatch,
    NonFiniteValue,
    StepSolveFailed { iteration: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Termination {
    Converged,
    IterationLimit,
}

impl Termination {
    pub fn converged(self) -> bool {
        matches!(self, Self::Converged)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub max_iter: usize,
    pub tolerance: f64,
    pub ridge_factor: f64,
}

/// `Logit.fit`'s own defaults.
impl Default for Settings {
    fn default() -> Self {
        Self {
            max_iter: 35,
            tolerance: 1e-8,
            ridge_factor: 1e-10,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Logit {
    /// One per design column, in column order.
    pub params: Vec<f64>,
    pub iterations: usize,
    pub termination: Termination,
}

impl Logit {
    pub fn predict_probability(&self, design: &[Vec<f64>]) -> Vec<f64> {
        probabilities(design, &self.params)
    }

    pub fn predict_linear(&self, design: &[Vec<f64>]) -> Vec<f64> {
        design.iter().map(|row| linear(row, &self.params)).collect()
    }

    /// `Logit.loglike`.
    pub fn log_likelihood(&self, design: &[Vec<f64>], outcome: &[f64]) -> f64 {
        design
            .iter()
            .zip(outcome)
            .map(|(row, y)| {
                let sign = 2.0 * y - 1.0;
                (1.0 / (1.0 + (-(sign * linear(row, &self.params))).exp())).ln()
            })
            .sum()
    }
}

fn linear(row: &[f64], params: &[f64]) -> f64 {
    row.iter().zip(params).map(|(value, param)| value * param).sum()
}

/// `Logit.cdf`, which has no sign split.
fn probabilities(design: &[Vec<f64>], params: &[f64]) -> Vec<f64> {
    design
        .iter()
        .map(|row| 1.0 / (1.0 + (-linear(row, params)).exp()))
        .collect()
}

/// The design carries its own intercept column.
pub fn fit(
    design: &[Vec<f64>],
    outcome: &[f64],
    settings: Settings,
) -> Result<Logit, LogitError> {
    let n = design.len();
    if n == 0 {
        return Err(LogitError::EmptySample);
    }
    if outcome.len() != n {
        return Err(LogitError::RowMismatch);
    }
    let columns = design[0].len();
    if columns == 0 {
        return Err(LogitError::NoColumns);
    }
    for row in design {
        if row.len() != columns {
            return Err(LogitError::RowMismatch);
        }
        if row.iter().any(|value| !value.is_finite()) {
            return Err(LogitError::NonFiniteValue);
        }
    }
    if outcome.iter().any(|value| !value.is_finite()) {
        return Err(LogitError::NonFiniteValue);
    }

    let nobs = n as f64;
    let mut params = vec![0.0; columns];
    let mut previous = vec![f64::INFINITY; columns];
    let mut iterations = 0;
    while iterations < settings.max_iter
        && params
            .iter()
            .zip(&previous)
            .any(|(new, old)| (new - old).abs() > settings.tolerance)
    {
        let fitted = probabilities(design, &params);
        let curvature: Vec<f64> = fitted.iter().map(|p| p * (1.0 - p)).collect();

        // statsmodels divides the Hessian and the score by the sample size before the solve, so
        // the ridge term lands on the scaled diagonal.
        let mut hessian = DMatrix::zeros(columns, columns);
        for column in 0..columns {
            for other in 0..columns {
                let mut total = 0.0;
                for row in 0..n {
                    total += curvature[row] * design[row][column] * design[row][other];
                }
                hessian[(column, other)] = -total / nobs;
            }
            hessian[(column, column)] += settings.ridge_factor;
        }

        let mut score = DMatrix::zeros(columns, 1);
        for column in 0..columns {
            let mut total = 0.0;
            for row in 0..n {
                total += (outcome[row] - fitted[row]) * design[row][column];
            }
            score[(column, 0)] = total / nobs;
        }

        let step = crate::linalg::solve(&hessian, &score)
            .map_err(|_| LogitError::StepSolveFailed { iteration: iterations })?;
        previous.copy_from_slice(&params);
        for column in 0..columns {
            params[column] = previous[column] - step[(column, 0)];
        }
        iterations += 1;
    }

    // statsmodels reads the iteration count alone, so a fit that lands on the limit is reported
    // as unconverged even when the last step moved nothing.
    let termination = if iterations == settings.max_iter {
        Termination::IterationLimit
    } else {
        Termination::Converged
    };
    Ok(Logit {
        params,
        iterations,
        termination,
    })
}
