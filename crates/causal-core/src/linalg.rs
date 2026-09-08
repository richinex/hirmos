//! Matrix adapters over the parity-tested LAPACK translations.
//!
//! These functions keep column-major `nalgebra` storage at the boundary while
//! preserving the decomposition selected by the pinned NumPy or SciPy source.

use nalgebra::{DMatrix, DVector};

use crate::lapack_cholesky::{self, Triangle as CholeskyTriangle};
use crate::lapack_dgesdd::{self, SvdJob};
use crate::lapack_dsyevd::{self, EigenJob, Triangle};
use crate::lapack_lu::{self, Transpose};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Error {
    NonSquare,
    NonFinite,
    Singular,
    DidNotConverge,
    InvalidStorage,
    NotPositiveDefinite,
    Svd(crate::lapack_dgesdd::DgesddError),
}

pub(crate) struct Svd {
    pub left: DMatrix<f64>,
    pub singular_values: DVector<f64>,
    pub right_transposed: DMatrix<f64>,
}

pub(crate) struct SymmetricEigen {
    pub values: DVector<f64>,
    pub vectors: DMatrix<f64>,
}

pub(crate) struct PseudoInverse {
    pub matrix: DMatrix<f64>,
    pub singular_values: DVector<f64>,
}

/// Solve a square general system with LAPACK's partial-pivot LU path.
pub(crate) fn solve(system: &DMatrix<f64>, right: &DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    if system.nrows() != system.ncols() {
        return Err(Error::NonSquare);
    }
    if system.nrows() != right.nrows() {
        return Err(Error::InvalidStorage);
    }
    if system
        .iter()
        .chain(right.iter())
        .any(|value| !value.is_finite())
    {
        return Err(Error::NonFinite);
    }

    let order = system.nrows();
    if order == 0 {
        return Ok(right.clone());
    }

    let mut factors = system.as_slice().to_vec();
    let mut pivots = vec![0; order];
    let info = lapack_lu::dgetrf(order, order, &mut factors, order, &mut pivots)
        .map_err(|_| Error::InvalidStorage)?;
    if info != 0 {
        return Err(Error::Singular);
    }

    let mut solution = right.as_slice().to_vec();
    lapack_lu::dgetrs(
        Transpose::None,
        order,
        right.ncols(),
        &factors,
        order,
        &pivots,
        &mut solution,
        order,
    )
    .map_err(|_| Error::InvalidStorage)?;
    Ok(DMatrix::from_column_slice(order, right.ncols(), &solution))
}

/// Form a general inverse by solving against the identity with the same LU.
pub(crate) fn inverse(matrix: &DMatrix<f64>) -> Result<DMatrix<f64>, Error> {
    solve(matrix, &DMatrix::identity(matrix.nrows(), matrix.nrows()))
}

/// `numpy.linalg.pinv` through its economy DGESDD decomposition.
pub(crate) fn pseudo_inverse(
    matrix: &DMatrix<f64>,
    relative_cutoff: f64,
) -> Result<PseudoInverse, Error> {
    if !relative_cutoff.is_finite() || relative_cutoff < 0.0 {
        return Err(Error::NonFinite);
    }
    if matrix.iter().any(|value| !value.is_finite()) {
        return Err(Error::NonFinite);
    }

    let rows = matrix.nrows();
    let columns = matrix.ncols();
    if rows == 0 || columns == 0 {
        return Ok(PseudoInverse {
            matrix: DMatrix::zeros(columns, rows),
            singular_values: DVector::zeros(0),
        });
    }

    let decomposition = svd_some(matrix)?;
    let order = rows.min(columns);
    let singular_values = decomposition.singular_values;
    let cutoff = relative_cutoff * singular_values[0];
    let left = decomposition.left;
    let right_transposed = decomposition.right_transposed;
    if left.ncols() != order || right_transposed.nrows() != order {
        return Err(Error::InvalidStorage);
    }

    // NumPy forms V @ (s^-1[:, None] * U^T), so keep the scaling on U^T
    // before performing the single matrix product.
    let scaled_left_transposed = DMatrix::from_fn(order, rows, |row, column| {
        if singular_values[row] > cutoff {
            left[(column, row)] / singular_values[row]
        } else {
            0.0
        }
    });
    let inverse = right_transposed.transpose() * scaled_left_transposed;
    Ok(PseudoInverse {
        matrix: inverse,
        singular_values,
    })
}

/// `scipy.linalg.svd(matrix, full_matrices=False)` through DGESDD.
pub(crate) fn svd_some(matrix: &DMatrix<f64>) -> Result<Svd, Error> {
    if matrix.iter().any(|value| !value.is_finite()) {
        return Err(Error::NonFinite);
    }

    let rows = matrix.nrows();
    let columns = matrix.ncols();
    if rows == 0 || columns == 0 {
        let order = rows.min(columns);
        return Ok(Svd {
            left: DMatrix::zeros(rows, order),
            singular_values: DVector::zeros(order),
            right_transposed: DMatrix::zeros(order, columns),
        });
    }

    let mut values = matrix.as_slice().to_vec();
    let (info, decomposition) =
        lapack_dgesdd::dgesdd(SvdJob::Some, rows, columns, &mut values, rows)
            .map_err(Error::Svd)?;
    if info != 0 {
        return Err(Error::DidNotConverge);
    }

    let left = DMatrix::from_column_slice(
        decomposition.left_rows,
        decomposition.left_columns,
        decomposition
            .left_vectors
            .as_deref()
            .ok_or(Error::InvalidStorage)?,
    );
    let right_transposed = DMatrix::from_column_slice(
        decomposition.right_rows,
        decomposition.right_columns,
        decomposition
            .right_vectors_transposed
            .as_deref()
            .ok_or(Error::InvalidStorage)?,
    );
    Ok(Svd {
        left,
        singular_values: DVector::from_vec(decomposition.singular_values),
        right_transposed,
    })
}

/// `statsmodels.tools.linalg.logdet_symm` through SciPy's lower DPOTRF path.
pub(crate) fn logdet_positive_definite_lower(matrix: &DMatrix<f64>) -> Result<f64, Error> {
    if matrix.nrows() != matrix.ncols() {
        return Err(Error::NonSquare);
    }
    if matrix.iter().any(|value| !value.is_finite()) {
        return Err(Error::NonFinite);
    }

    let order = matrix.nrows();
    let mut factor = matrix.as_slice().to_vec();
    let info = lapack_cholesky::dpotrf(CholeskyTriangle::Lower, order, &mut factor, order.max(1))
        .map_err(|_| Error::InvalidStorage)?;
    if info != 0 {
        return Err(Error::NotPositiveDefinite);
    }
    Ok(2.0
        * (0..order)
            .map(|index| factor[index + index * order].ln())
            .sum::<f64>())
}

/// R's `chol2inv(chol(matrix))` through the upper DPOTRF/DPOTRS path.
pub(crate) fn inverse_positive_definite_upper(
    matrix: &DMatrix<f64>,
) -> Result<DMatrix<f64>, Error> {
    if matrix.nrows() != matrix.ncols() {
        return Err(Error::NonSquare);
    }
    if matrix.iter().any(|value| !value.is_finite()) {
        return Err(Error::NonFinite);
    }

    let order = matrix.nrows();
    let mut factor = matrix.as_slice().to_vec();
    let info = lapack_cholesky::dpotrf(CholeskyTriangle::Upper, order, &mut factor, order.max(1))
        .map_err(|_| Error::InvalidStorage)?;
    if info != 0 {
        return Err(Error::NotPositiveDefinite);
    }

    let mut inverse = DMatrix::<f64>::identity(order, order);
    lapack_cholesky::dpotrs(
        CholeskyTriangle::Upper,
        order,
        order,
        &factor,
        order.max(1),
        inverse.as_mut_slice(),
        order.max(1),
    )
    .map_err(|_| Error::InvalidStorage)?;
    Ok(inverse)
}

/// `numpy.linalg.eigvalsh(matrix, UPLO="L")` for a real symmetric matrix.
pub(crate) fn eigenvalues_symmetric_lower(matrix: &DMatrix<f64>) -> Result<DVector<f64>, Error> {
    let (values, _) = symmetric_lower(matrix, EigenJob::Values)?;
    Ok(values)
}

/// `numpy.linalg.eigh(matrix, UPLO="L")` for a real symmetric matrix.
pub(crate) fn eigen_symmetric_lower(matrix: &DMatrix<f64>) -> Result<SymmetricEigen, Error> {
    let (values, vectors) = symmetric_lower(matrix, EigenJob::Vectors)?;
    Ok(SymmetricEigen { values, vectors })
}

fn symmetric_lower(
    matrix: &DMatrix<f64>,
    job: EigenJob,
) -> Result<(DVector<f64>, DMatrix<f64>), Error> {
    if matrix.nrows() != matrix.ncols() {
        return Err(Error::NonSquare);
    }
    if matrix.iter().any(|value| !value.is_finite()) {
        return Err(Error::NonFinite);
    }

    let order = matrix.nrows();
    let mut vectors = matrix.as_slice().to_vec();
    let mut values = vec![0.0; order];
    let info = lapack_dsyevd::dsyevd(
        job,
        Triangle::Lower,
        order,
        &mut vectors,
        order.max(1),
        &mut values,
    )
    .map_err(|_| Error::InvalidStorage)?;
    if info != 0 {
        return Err(Error::DidNotConverge);
    }
    Ok((
        DVector::from_vec(values),
        DMatrix::from_column_slice(order, order, &vectors),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_pivoted_general_system() {
        let system = DMatrix::from_row_slice(2, 2, &[0.0, 2.0, 1.0, 3.0]);
        let right = DMatrix::from_column_slice(2, 1, &[4.0, 7.0]);
        let solution = solve(&system, &right).expect("nonsingular system");
        assert!((solution[(0, 0)] - 1.0).abs() < 1e-15);
        assert!((solution[(1, 0)] - 2.0).abs() < 1e-15);
    }

    #[test]
    fn reads_the_lower_symmetric_triangle() {
        let matrix = DMatrix::from_row_slice(2, 2, &[2.0, 999.0, 1.0, 2.0]);
        let decomposition = eigen_symmetric_lower(&matrix).expect("symmetric eigendecomposition");
        assert!((decomposition.values[0] - 1.0).abs() < 1e-14);
        assert!((decomposition.values[1] - 3.0).abs() < 1e-14);
        let diagonalized = decomposition.vectors.transpose()
            * DMatrix::from_row_slice(2, 2, &[2.0, 1.0, 1.0, 2.0])
            * decomposition.vectors;
        assert!((diagonalized[(0, 0)] - 1.0).abs() < 1e-14);
        assert!((diagonalized[(1, 1)] - 3.0).abs() < 1e-14);
    }

    #[test]
    fn pseudo_inverse_handles_a_rank_deficient_design() {
        let matrix = DMatrix::from_row_slice(3, 2, &[1.0, 2.0, 2.0, 4.0, 3.0, 6.0]);
        let inverse = pseudo_inverse(&matrix, 1e-15).expect("pseudoinverse");
        assert_eq!(inverse.singular_values.len(), 2);
        let reconstructed = &matrix * &inverse.matrix * &matrix;
        assert!((&reconstructed - matrix).amax() < 1e-12);
    }

    #[test]
    fn computes_positive_definite_log_determinant_from_the_lower_triangle() {
        let matrix = DMatrix::from_row_slice(2, 2, &[4.0, 999.0, 2.0, 3.0]);
        let logdet = logdet_positive_definite_lower(&matrix).expect("positive definite matrix");
        assert!((logdet - 8.0_f64.ln()).abs() < 1e-14);
    }

    #[test]
    fn inverts_a_positive_definite_matrix_from_the_upper_triangle() {
        let matrix = DMatrix::from_row_slice(2, 2, &[4.0, 2.0, 999.0, 3.0]);
        let inverse = inverse_positive_definite_upper(&matrix).expect("positive definite matrix");
        let expected = DMatrix::from_row_slice(2, 2, &[0.375, -0.25, -0.25, 0.5]);
        assert!((&inverse - expected).amax() < 1e-14);
    }
}
