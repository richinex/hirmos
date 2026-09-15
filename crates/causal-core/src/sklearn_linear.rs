//! Dense `sklearn.linear_model.LinearRegression` compatibility for the pinned 1.9 reference.
//!
//! Scikit-learn centers dense predictors and the target when `fit_intercept=true`, then calls
//! `scipy.linalg.lstsq` with `cond=tol`.  SciPy interprets `cond` relative to the largest singular
//! value and returns the minimum-norm solution when the centered design is rank deficient.

use std::fmt;

use nalgebra::{DMatrix, DVector};

pub const SKLEARN_LINEAR_TOLERANCE: f64 = 1e-6;

#[derive(Clone, Debug, PartialEq)]
pub struct SklearnLinearFit {
    pub intercept: f64,
    pub coefficients: DVector<f64>,
    pub predictions: DVector<f64>,
    pub residuals: DVector<f64>,
    /// Root mean squared residual, matching Tigramite's dynamic-SCM convention.
    pub residual_scale: f64,
    /// Numerical rank of the centered predictor matrix.
    pub rank: usize,
    pub singular_values: DVector<f64>,
    /// Absolute cutoff implied by scikit-learn's relative tolerance.
    pub singular_value_cutoff: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SklearnLinearError {
    Shape,
    EmptyObservations,
    NonFinite,
    InvalidTolerance,
    SvdSolve,
}

impl fmt::Display for SklearnLinearError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shape => formatter
                .write_str("linear-regression predictors and target must have matching rows"),
            Self::EmptyObservations => {
                formatter.write_str("linear regression needs at least one observation")
            }
            Self::NonFinite => formatter.write_str("linear-regression inputs must be finite"),
            Self::InvalidTolerance => {
                formatter.write_str("linear-regression tolerance must be finite and non-negative")
            }
            Self::SvdSolve => formatter.write_str("the centered SVD least-squares solve failed"),
        }
    }
}

impl std::error::Error for SklearnLinearError {}

pub fn fit_sklearn_linear_regression(
    predictors: &DMatrix<f64>,
    target: &DVector<f64>,
    tolerance: f64,
) -> Result<SklearnLinearFit, SklearnLinearError> {
    fit_with_means(predictors, target, tolerance, MeanOrder::Sequential)
}

/// Dense column-contiguous inputs, as produced by the GCM pandas column selection.
pub fn fit_sklearn_fortran(
    predictors: &DMatrix<f64>,
    target: &DVector<f64>,
    tolerance: f64,
) -> Result<SklearnLinearFit, SklearnLinearError> {
    fit_with_means(predictors, target, tolerance, MeanOrder::NumpyContiguous)
}

#[derive(Clone, Copy)]
enum MeanOrder {
    Sequential,
    NumpyContiguous,
}

pub(crate) fn predict_fortran(
    predictors: &DMatrix<f64>,
    coefficients: &DVector<f64>,
    intercept: f64,
) -> DVector<f64> {
    use crate::lapack_dgelsd::blas::{dgemv, Transpose};
    let mut values = DVector::zeros(predictors.nrows());
    dgemv(
        Transpose::None,
        predictors.nrows(),
        predictors.ncols(),
        1.0,
        predictors.as_slice(),
        predictors.nrows().max(1),
        coefficients.as_slice(),
        1,
        0.0,
        values.as_mut_slice(),
        1,
    )
    .expect("fitted coefficients match the validated predictor matrix");
    values.add_scalar_mut(intercept);
    values
}

fn fit_with_means(
    predictors: &DMatrix<f64>,
    target: &DVector<f64>,
    tolerance: f64,
    order: MeanOrder,
) -> Result<SklearnLinearFit, SklearnLinearError> {
    let observations = predictors.nrows();
    if observations == 0 {
        return Err(SklearnLinearError::EmptyObservations);
    }
    if target.len() != observations {
        return Err(SklearnLinearError::Shape);
    }
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(SklearnLinearError::InvalidTolerance);
    }
    if predictors
        .iter()
        .chain(target.iter())
        .any(|value| !value.is_finite())
    {
        return Err(SklearnLinearError::NonFinite);
    }

    let mean = |values: &[f64]| match order {
        MeanOrder::Sequential => values.iter().sum::<f64>() / observations as f64,
        MeanOrder::NumpyContiguous => crate::numpy_reduce::numpy_mean(values),
    };
    let target_mean = mean(target.as_slice());
    let predictor_means = DVector::from_iterator(
        predictors.ncols(),
        (0..predictors.ncols()).map(|column| mean(predictors.column(column).as_slice())),
    );
    let mut centered_predictors = predictors.clone();
    for column in 0..centered_predictors.ncols() {
        for row in 0..observations {
            centered_predictors[(row, column)] -= predictor_means[column];
        }
    }
    let centered_target = target.map(|value| value - target_mean);

    let (coefficients, rank, singular_values, singular_value_cutoff) = if predictors.ncols() == 0 {
        (DVector::zeros(0), 0, DVector::zeros(0), 0.0)
    } else {
    let fit = crate::least_squares::solve(
        &centered_predictors,
        &DMatrix::from_column_slice(observations, 1, centered_target.as_slice()),
        tolerance,
    )
    .map_err(|_| SklearnLinearError::SvdSolve)?;
    (fit.coefficients.column(0).into_owned(), fit.rank, fit.singular_values, fit.cutoff)
    };
    let intercept = target_mean - predictor_means.dot(&coefficients);
    let predictions = match order {
        MeanOrder::Sequential => {
            predictors * &coefficients + DVector::from_element(observations, intercept)
        }
        MeanOrder::NumpyContiguous => predict_fortran(predictors, &coefficients, intercept),
    };
    let residuals = target - &predictions;
    let residual_scale = (residuals.dot(&residuals) / observations as f64).sqrt();

    Ok(SklearnLinearFit {
        intercept,
        coefficients,
        predictions,
        residuals,
        residual_scale,
        rank,
        singular_values,
        singular_value_cutoff,
    })
}
