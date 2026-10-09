//! flexsurv's Hessian-to-covariance recipe, including `Matrix::nearPD` defaults.

use crate::lapack_dsyevd::{dsyevd, DsyevdError, EigenJob, Triangle};
use crate::lapack_lu::{dgetrf, dgetrs, LuError, Transpose};

#[derive(Clone, Debug, PartialEq)]
pub struct CovarianceResult {
    pub values: Vec<f64>,
    pub order: usize,
    pub smallest_unrepaired_eigenvalue: f64,
    pub repaired: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CovarianceError {
    EmptyHessian,
    HessianLength { expected: usize, actual: usize },
    NonFiniteHessian { index: usize },
    SingularHessian { pivot: usize },
    InfiniteInverse,
    NegativeSemidefinite,
    Lu(LuError),
    Eigen(DsyevdError),
}

impl From<LuError> for CovarianceError {
    fn from(value: LuError) -> Self {
        Self::Lu(value)
    }
}

impl From<DsyevdError> for CovarianceError {
    fn from(value: DsyevdError) -> Self {
        Self::Eigen(value)
    }
}

fn row_to_column(values: &[f64], order: usize) -> Vec<f64> {
    let mut column_major = vec![0.0; values.len()];
    for row in 0..order {
        for column in 0..order {
            column_major[row + column * order] = values[row * order + column];
        }
    }
    column_major
}

fn column_to_row(values: &[f64], order: usize) -> Vec<f64> {
    let mut row_major = vec![0.0; values.len()];
    for row in 0..order {
        for column in 0..order {
            row_major[row * order + column] = values[row + column * order];
        }
    }
    row_major
}

fn symmetric_eigen(
    values: &[f64],
    order: usize,
    vectors: bool,
) -> Result<(Vec<f64>, Vec<f64>), CovarianceError> {
    let mut matrix = row_to_column(values, order);
    let mut eigenvalues = vec![0.0; order];
    let info = dsyevd(
        if vectors {
            EigenJob::Vectors
        } else {
            EigenJob::Values
        },
        Triangle::Upper,
        order,
        &mut matrix,
        order,
        &mut eigenvalues,
    )?;
    if info != 0 {
        return Err(CovarianceError::Eigen(DsyevdError::InvalidArgument(info)));
    }
    Ok((eigenvalues, matrix))
}

pub(crate) fn eigen_covariance_root(
    covariance: &[f64],
    order: usize,
) -> Result<Vec<f64>, CovarianceError> {
    if order == 0 {
        return Err(CovarianceError::EmptyHessian);
    }
    let expected = order * order;
    if covariance.len() != expected {
        return Err(CovarianceError::HessianLength {
            expected,
            actual: covariance.len(),
        });
    }
    if let Some(index) = covariance.iter().position(|value| !value.is_finite()) {
        return Err(CovarianceError::NonFiniteHessian { index });
    }

    let (eigenvalues, eigenvectors) = symmetric_eigen(covariance, order, true)?;
    let mut root = vec![0.0; expected];
    for component in 0..order {
        let scale = eigenvalues[component].max(0.0).sqrt();
        for row in 0..order {
            let left = eigenvectors[row + component * order];
            for column in 0..order {
                root[row * order + column] +=
                    left * scale * eigenvectors[column + component * order];
            }
        }
    }
    Ok(root)
}

fn reconstruct(
    eigenvalues: &[f64],
    eigenvectors: &[f64],
    order: usize,
    retain: impl Fn(f64) -> bool,
) -> Vec<f64> {
    let mut result = vec![0.0; order * order];
    for component in 0..order {
        let value = eigenvalues[component];
        if !retain(value) {
            continue;
        }
        for row in 0..order {
            let left = eigenvectors[row + component * order];
            for column in 0..order {
                result[row * order + column] +=
                    left * value * eigenvectors[column + component * order];
            }
        }
    }
    result
}

fn infinity_norm(values: &[f64], order: usize) -> f64 {
    (0..order)
        .map(|row| {
            (0..order)
                .map(|column| values[row * order + column].abs())
                .sum::<f64>()
        })
        .fold(0.0, f64::max)
}

fn near_positive_definite(original: &[f64], order: usize) -> Result<Vec<f64>, CovarianceError> {
    const EIGEN_TOLERANCE: f64 = 1e-6;
    const CONVERGENCE_TOLERANCE: f64 = 1e-7;
    const POSITIVE_DEFINITE_TOLERANCE: f64 = 1e-8;
    const MAXIMUM_ITERATIONS: usize = 100;

    let mut x = original.to_vec();
    let mut correction = vec![0.0; x.len()];
    for _ in 0..MAXIMUM_ITERATIONS {
        let previous = x.clone();
        let residual = previous
            .iter()
            .zip(&correction)
            .map(|(value, correction)| value - correction)
            .collect::<Vec<_>>();
        let (eigenvalues, eigenvectors) = symmetric_eigen(&residual, order, true)?;
        let largest = eigenvalues[order - 1];
        if !eigenvalues
            .iter()
            .any(|value| *value > EIGEN_TOLERANCE * largest)
        {
            return Err(CovarianceError::NegativeSemidefinite);
        }
        x = reconstruct(&eigenvalues, &eigenvectors, order, |value| {
            value > EIGEN_TOLERANCE * largest
        });
        for index in 0..correction.len() {
            correction[index] = x[index] - residual[index];
        }
        let denominator = infinity_norm(&previous, order);
        let difference = previous
            .iter()
            .zip(&x)
            .map(|(left, right)| left - right)
            .collect::<Vec<_>>();
        let relative = infinity_norm(&difference, order) / denominator;
        if relative <= CONVERGENCE_TOLERANCE {
            break;
        }
    }

    let original_diagonal = (0..order)
        .map(|index| x[index * order + index])
        .collect::<Vec<_>>();
    let (mut eigenvalues, eigenvectors) = symmetric_eigen(&x, order, true)?;
    let floor = POSITIVE_DEFINITE_TOLERANCE * eigenvalues[order - 1].abs();
    if eigenvalues[0] < floor {
        for value in &mut eigenvalues {
            if *value < floor {
                *value = floor;
            }
        }
        x = reconstruct(&eigenvalues, &eigenvectors, order, |_| true);
        let scale = (0..order)
            .map(|index| (original_diagonal[index].max(floor) / x[index * order + index]).sqrt())
            .collect::<Vec<_>>();
        for row in 0..order {
            for column in 0..order {
                x[row * order + column] *= scale[row] * scale[column];
            }
        }
    }
    Ok(x)
}

pub fn hessian_to_covariance(
    hessian: &[f64],
    order: usize,
) -> Result<CovarianceResult, CovarianceError> {
    if order == 0 {
        return Err(CovarianceError::EmptyHessian);
    }
    let expected = order * order;
    if hessian.len() != expected {
        return Err(CovarianceError::HessianLength {
            expected,
            actual: hessian.len(),
        });
    }
    if let Some(index) = hessian.iter().position(|value| !value.is_finite()) {
        return Err(CovarianceError::NonFiniteHessian { index });
    }

    let mut lu = row_to_column(hessian, order);
    let mut pivots = vec![0; order];
    let info = dgetrf(order, order, &mut lu, order, &mut pivots)?;
    if info != 0 {
        return Err(CovarianceError::SingularHessian { pivot: info });
    }
    let mut inverse = vec![0.0; expected];
    for index in 0..order {
        inverse[index + index * order] = 1.0;
    }
    dgetrs(
        Transpose::None,
        order,
        order,
        &lu,
        order,
        &pivots,
        &mut inverse,
        order,
    )?;
    if inverse.iter().any(|value| value.is_infinite()) {
        return Err(CovarianceError::InfiniteInverse);
    }
    let mut inverse = column_to_row(&inverse, order);
    for row in 0..order {
        for column in 0..row {
            let value = 0.5 * (inverse[row * order + column] + inverse[column * order + row]);
            inverse[row * order + column] = value;
            inverse[column * order + row] = value;
        }
    }
    let (eigenvalues, _) = symmetric_eigen(&inverse, order, false)?;
    let smallest = eigenvalues[0];
    let repaired = near_positive_definite(&inverse, order)?;
    Ok(CovarianceResult {
        repaired: repaired != inverse,
        values: repaired,
        order,
        smallest_unrepaired_eigenvalue: smallest,
    })
}
