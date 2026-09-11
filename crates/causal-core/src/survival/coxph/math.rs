use crate::lapack_cholesky::{dpotrf, dpotrs, Triangle};
use crate::lapack_lu::{dgetrf, dgetrs, Transpose};

use super::fit::CoxFitError;

pub(crate) fn means_and_standard_deviations(
    values: &[f64],
    rows: usize,
    columns: usize,
) -> (Vec<f64>, Vec<f64>) {
    let mut means = vec![0.0; columns];
    for row in 0..rows {
        for column in 0..columns {
            means[column] += values[row * columns + column];
        }
    }
    for value in &mut means {
        *value /= rows as f64;
    }
    let mut deviations = vec![0.0; columns];
    for row in 0..rows {
        for column in 0..columns {
            let difference = values[row * columns + column] - means[column];
            deviations[column] += difference * difference;
        }
    }
    for value in &mut deviations {
        *value = (*value / (rows - 1) as f64).sqrt();
    }
    (means, deviations)
}

pub(crate) fn normalize(
    values: &[f64],
    rows: usize,
    columns: usize,
    means: &[f64],
    deviations: &[f64],
) -> Vec<f64> {
    let mut normalized = vec![0.0; values.len()];
    for row in 0..rows {
        for column in 0..columns {
            normalized[row * columns + column] =
                (values[row * columns + column] - means[column]) / deviations[column];
        }
    }
    normalized
}

fn column_major(values: &[f64], order: usize) -> Vec<f64> {
    let mut result = vec![0.0; values.len()];
    for row in 0..order {
        for column in 0..order {
            result[row + column * order] = values[row * order + column];
        }
    }
    result
}

pub(crate) fn positive_solve(matrix: &[f64], rhs: &[f64]) -> Result<Vec<f64>, CoxFitError> {
    let order = rhs.len();
    let mut factor = column_major(matrix, order);
    let info = dpotrf(Triangle::Upper, order, &mut factor, order)
        .map_err(|_| CoxFitError::SingularInformation)?;
    if info != 0 {
        return Err(CoxFitError::SingularInformation);
    }
    let mut solution = rhs.to_vec();
    dpotrs(
        Triangle::Upper,
        order,
        1,
        &factor,
        order,
        &mut solution,
        order,
    )
    .map_err(|_| CoxFitError::SingularInformation)?;
    Ok(solution)
}

pub(crate) fn inverse(matrix: &[f64], order: usize) -> Result<Vec<f64>, CoxFitError> {
    let mut factor = column_major(matrix, order);
    let mut pivots = vec![0; order];
    let info = dgetrf(order, order, &mut factor, order, &mut pivots)
        .map_err(|_| CoxFitError::SingularInformation)?;
    if info != 0 {
        return Err(CoxFitError::SingularInformation);
    }
    let mut inverse = vec![0.0; order * order];
    for index in 0..order {
        inverse[index + index * order] = 1.0;
    }
    dgetrs(
        Transpose::None,
        order,
        order,
        &factor,
        order,
        &pivots,
        &mut inverse,
        order,
    )
    .map_err(|_| CoxFitError::SingularInformation)?;
    let mut row_major = vec![0.0; inverse.len()];
    for row in 0..order {
        for column in 0..order {
            row_major[row * order + column] = inverse[row + column * order];
        }
    }
    Ok(row_major)
}

pub(crate) struct StepSizer {
    initial: f64,
    current: f64,
    temper_back_up: bool,
    norms: Vec<f64>,
}

impl StepSizer {
    pub(crate) fn new(initial: f64) -> Self {
        Self {
            initial,
            current: initial,
            temper_back_up: false,
            norms: Vec::new(),
        }
    }

    pub(crate) fn current(&self) -> f64 {
        self.current
    }

    pub(crate) fn update(&mut self, norm: f64) -> f64 {
        const SCALE: f64 = 1.3;
        self.norms.push(norm);
        if self.temper_back_up {
            self.current = (self.current * SCALE).min(self.initial);
        }
        if norm >= 15.0 {
            self.current *= 0.1;
            self.temper_back_up = true;
        } else if norm > 5.0 {
            self.current *= 0.25;
            self.temper_back_up = true;
        }
        if self.norms.len() >= 3 {
            let recent = &self.norms[self.norms.len() - 3..];
            let decreasing = recent[1] < recent[0] && recent[2] < recent[1];
            if decreasing {
                self.current = (self.current * SCALE).min(1.0);
            } else {
                self.current *= 0.98;
            }
        }
        self.current
    }
}

pub(crate) fn norm(values: &[f64]) -> f64 {
    values.iter().map(|value| value * value).sum::<f64>().sqrt()
}

pub(crate) fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(a, b)| a * b).sum()
}
