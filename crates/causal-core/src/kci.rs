//! Numeric-only port of DoWhy 0.14's `kernel_based` test and causal-learn's Gaussian KCI.
//!
//! This is intentionally separate from `hsic.rs`: that module reproduces LiNGAM's
//! unconditional HSIC diagnostic, while this module supports both marginal and conditional
//! independence and follows the KCI gamma approximation used by DoWhy's graph falsifier.

use nalgebra::{DMatrix, SymmetricEigen};

#[derive(Clone, Debug, PartialEq)]
pub enum KciError {
    EmptySample,
    RowMismatch,
    NonFiniteValue,
    SingularRegression,
    InvalidGammaApproximation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KciResult {
    pub p_value: f64,
    pub statistic: f64,
    pub observations: usize,
    pub conditioning_columns: usize,
}

fn finite(matrix: &DMatrix<f64>) -> bool {
    matrix.iter().all(|value| value.is_finite())
}

fn remove_constant_columns(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let kept: Vec<usize> = (0..matrix.ncols())
        .filter(|&column| {
            let first = matrix[(0, column)];
            (1..matrix.nrows()).any(|row| matrix[(row, column)] != first)
        })
        .collect();
    DMatrix::from_fn(matrix.nrows(), kept.len(), |row, column| {
        matrix[(row, kept[column])]
    })
}

/// SciPy `stats.zscore(..., ddof=1, axis=0)`, with constant columns replaced by zero.
fn zscore(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let n = matrix.nrows();
    DMatrix::from_fn(n, matrix.ncols(), |row, column| {
        let mean = (0..n).map(|r| matrix[(r, column)]).sum::<f64>() / n as f64;
        let variance = (0..n)
            .map(|r| {
                let delta = matrix[(r, column)] - mean;
                delta * delta
            })
            .sum::<f64>()
            / (n.saturating_sub(1)) as f64;
        let standard_deviation = variance.sqrt();
        if standard_deviation == 0.0 || !standard_deviation.is_finite() {
            0.0
        } else {
            (matrix[(row, column)] - mean) / standard_deviation
        }
    })
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|left, right| left.total_cmp(right));
    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

/// causal-learn's `GaussianKernel.set_width_median` for at most 500 observations.
fn median_kernel_precision(matrix: &DMatrix<f64>) -> f64 {
    let mut distances = Vec::with_capacity(matrix.nrows() * matrix.nrows() / 2);
    for left in 0..matrix.nrows() {
        for right in left + 1..matrix.nrows() {
            let squared = (0..matrix.ncols())
                .map(|column| {
                    let delta = matrix[(left, column)] - matrix[(right, column)];
                    delta * delta
                })
                .sum::<f64>();
            if squared > 0.0 {
                distances.push(squared.sqrt());
            }
        }
    }
    if distances.is_empty() {
        return 1.0;
    }
    let width = 2.0_f64.sqrt() * median(&mut distances);
    1.0 / (width * width)
}

fn gaussian_gram(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let precision = median_kernel_precision(matrix);
    gaussian_gram_with_precision(matrix, precision)
}

fn gaussian_gram_with_precision(matrix: &DMatrix<f64>, precision: f64) -> DMatrix<f64> {
    DMatrix::from_fn(matrix.nrows(), matrix.nrows(), |left, right| {
        let squared = (0..matrix.ncols())
            .map(|column| {
                let delta = matrix[(left, column)] - matrix[(right, column)];
                delta * delta
            })
            .sum::<f64>();
        (-0.5 * squared * precision).exp()
    })
}

/// causal-learn's O(n^2) expansion of H K H.
fn center(matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let n = matrix.nrows();
    let nf = n as f64;
    let column_sums: Vec<f64> = (0..n).map(|column| matrix.column(column).sum()).collect();
    let all_sum = column_sums.iter().sum::<f64>();
    DMatrix::from_fn(n, n, |row, column| {
        matrix[(row, column)] - (column_sums[row] + column_sums[column]) / nf + all_sum / (nf * nf)
    })
}

fn gamma_survival(statistic: f64, mean: f64, variance: f64) -> Result<f64, KciError> {
    if !(statistic.is_finite() && mean.is_finite() && variance.is_finite())
        || mean <= 0.0
        || variance <= 0.0
    {
        return Err(KciError::InvalidGammaApproximation);
    }
    let shape = mean * mean / variance;
    let scale = variance / mean;
    Ok(spec_math::cephes64::igamc(shape, statistic / scale).clamp(0.0, 1.0))
}

fn unconditional(x: &DMatrix<f64>, y: &DMatrix<f64>) -> Result<KciResult, KciError> {
    // KCI_UInd chooses its median width on the incoming values and only then
    // standardises the values used to build the Gram matrix.
    let x_precision = median_kernel_precision(x);
    let y_precision = median_kernel_precision(y);
    let x = zscore(x);
    let y = zscore(y);
    let kx = center(&gaussian_gram_with_precision(&x, x_precision));
    let ky = center(&gaussian_gram_with_precision(&y, y_precision));
    let statistic = kx.component_mul(&ky).sum();
    let n = x.nrows() as f64;
    let mean = kx.trace() * ky.trace() / n;
    let variance = 2.0 * kx.component_mul(&kx).sum() * ky.component_mul(&ky).sum() / (n * n);
    Ok(KciResult {
        p_value: gamma_survival(statistic, mean, variance)?,
        statistic,
        observations: x.nrows(),
        conditioning_columns: 0,
    })
}

fn empirical_hsic_precision(rows: usize, columns: usize) -> f64 {
    let width: f64 = if rows < 200 {
        0.8
    } else if rows < 1200 {
        0.5
    } else {
        0.3
    };
    columns as f64 / width.powi(2)
}

fn empirical_kci_precision(rows: usize, conditioning_columns: usize) -> f64 {
    let width: f64 = if rows < 200 {
        1.2
    } else if rows < 1200 {
        0.7
    } else {
        0.4
    };
    1.0 / (width.powi(2) * conditioning_columns as f64)
}

/// causal-learn's default Gaussian KCI (`est_width="empirical"`, `approx=True`).
///
/// Unlike [`kernel_conditional_independence`], this preserves constant columns as zero after
/// SciPy-style standardisation because that is what causal-learn's `KCI_UInd` and `KCI_CInd` do.
pub fn causal_learn_kernel_conditional_independence(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
) -> Result<KciResult, KciError> {
    if x.nrows() == 0 || y.nrows() == 0 {
        return Err(KciError::EmptySample);
    }
    if x.nrows() != y.nrows() || z.is_some_and(|values| values.nrows() != x.nrows()) {
        return Err(KciError::RowMismatch);
    }
    if !finite(x) || !finite(y) || z.is_some_and(|values| !finite(values)) {
        return Err(KciError::NonFiniteValue);
    }

    match z {
        None if x.ncols() > 0 && y.ncols() > 0 => {
            let x_precision = empirical_hsic_precision(x.nrows(), x.ncols());
            let y_precision = empirical_hsic_precision(y.nrows(), y.ncols());
            let x = zscore(x);
            let y = zscore(y);
            let kx = center(&gaussian_gram_with_precision(&x, x_precision));
            let ky = center(&gaussian_gram_with_precision(&y, y_precision));
            let statistic = kx.component_mul(&ky).sum();
            let n = x.nrows() as f64;
            let mean = kx.trace() * ky.trace() / n;
            let variance =
                2.0 * kx.component_mul(&kx).sum() * ky.component_mul(&ky).sum() / (n * n);
            Ok(KciResult {
                p_value: gamma_survival(statistic, mean, variance)?,
                statistic,
                observations: x.nrows(),
                conditioning_columns: 0,
            })
        }
        Some(z) if z.ncols() > 0 && x.ncols() > 0 && y.ncols() > 0 => {
            let x = zscore(x);
            let y = zscore(y);
            let z = zscore(z);
            let xz = DMatrix::from_fn(x.nrows(), x.ncols() + z.ncols(), |row, column| {
                if column < x.ncols() {
                    x[(row, column)]
                } else {
                    0.5 * z[(row, column - x.ncols())]
                }
            });
            let precision = empirical_kci_precision(z.nrows(), z.ncols());
            let kx = center(&gaussian_gram_with_precision(&xz, precision));
            let ky = center(&gaussian_gram_with_precision(&y, precision));
            let kz = center(&gaussian_gram_with_precision(&z, precision));
            let n = x.nrows();
            let epsilon = 1e-3;
            let residualizer =
                symmetric_pseudo_inverse(kz + DMatrix::identity(n, n) * epsilon, 1e-15)? * epsilon;
            let kxr = &residualizer * kx * &residualizer;
            let kyr = &residualizer * ky * &residualizer;
            let statistic = kxr.component_mul(&kyr).sum();
            let vx = retained_eigen_features(&kxr, 1e-5);
            let vy = retained_eigen_features(&kyr, 1e-5);
            let size_u = vx.ncols() * vy.ncols();
            if size_u == 0 {
                return Ok(KciResult {
                    p_value: 1.0,
                    statistic,
                    observations: n,
                    conditioning_columns: z.ncols(),
                });
            }
            let u = DMatrix::from_fn(n, size_u, |row, column| {
                vx[(row, column / vy.ncols())] * vy[(row, column % vy.ncols())]
            });
            let uu_product = if size_u > n {
                &u * u.transpose()
            } else {
                u.transpose() * &u
            };
            let mean = uu_product.trace();
            let variance = 2.0 * (&uu_product * &uu_product).trace();
            Ok(KciResult {
                p_value: gamma_survival(statistic, mean, variance)?,
                statistic,
                observations: n,
                conditioning_columns: z.ncols(),
            })
        }
        _ => Ok(KciResult {
            p_value: 1.0,
            statistic: 0.0,
            observations: x.nrows(),
            conditioning_columns: z.map_or(0, DMatrix::ncols),
        }),
    }
}

fn symmetric_pseudo_inverse(
    matrix: DMatrix<f64>,
    tolerance: f64,
) -> Result<DMatrix<f64>, KciError> {
    let svd = matrix.svd(true, true);
    svd.pseudo_inverse(tolerance)
        .map_err(|_| KciError::SingularRegression)
}

fn retained_eigen_features(matrix: &DMatrix<f64>, threshold: f64) -> DMatrix<f64> {
    let symmetric = (matrix + matrix.transpose()) * 0.5;
    let decomposition = SymmetricEigen::new(symmetric);
    let maximum = decomposition
        .eigenvalues
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let kept: Vec<usize> = decomposition
        .eigenvalues
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| (value > maximum * threshold).then_some(index))
        .collect();
    DMatrix::from_fn(matrix.nrows(), kept.len(), |row, column| {
        let index = kept[column];
        decomposition.eigenvectors[(row, index)] * decomposition.eigenvalues[index].sqrt()
    })
}

fn conditional(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: &DMatrix<f64>,
) -> Result<KciResult, KciError> {
    let x = zscore(x);
    let y = zscore(y);
    let z = zscore(z);
    let xz = DMatrix::from_fn(x.nrows(), x.ncols() + z.ncols(), |row, column| {
        if column < x.ncols() {
            x[(row, column)]
        } else {
            0.5 * z[(row, column - x.ncols())]
        }
    });
    let kx = center(&gaussian_gram(&xz));
    let ky = center(&gaussian_gram(&y));
    let kz = center(&gaussian_gram(&z));
    let n = x.nrows();
    let epsilon = 1e-3;
    let regularized = kz + DMatrix::identity(n, n) * epsilon;
    // NumPy's `pinv` default is 1e-15. Multiplication by epsilon is the
    // causal-learn regression residual operator Rz.
    let residualizer = symmetric_pseudo_inverse(regularized, 1e-15)? * epsilon;
    let kxr = &residualizer * kx * &residualizer;
    let kyr = &residualizer * ky * &residualizer;
    let statistic = kxr.component_mul(&kyr).sum();

    let vx = retained_eigen_features(&kxr, 1e-5);
    let vy = retained_eigen_features(&kyr, 1e-5);
    let size_u = vx.ncols() * vy.ncols();
    if size_u == 0 {
        return Ok(KciResult {
            p_value: 1.0,
            statistic,
            observations: n,
            conditioning_columns: z.ncols(),
        });
    }
    let u = DMatrix::from_fn(n, size_u, |row, column| {
        let y_column = column % vy.ncols();
        let x_column = column / vy.ncols();
        vx[(row, x_column)] * vy[(row, y_column)]
    });
    let uu_product = if size_u > n {
        &u * u.transpose()
    } else {
        u.transpose() * &u
    };
    let mean = uu_product.trace();
    let variance = 2.0 * (&uu_product * &uu_product).trace();
    Ok(KciResult {
        p_value: gamma_survival(statistic, mean, variance)?,
        statistic,
        observations: n,
        conditioning_columns: z.ncols(),
    })
}

/// DoWhy's numeric KCI path with `use_bootstrap=False` and median kernel widths.
pub fn kernel_conditional_independence(
    x: &DMatrix<f64>,
    y: &DMatrix<f64>,
    z: Option<&DMatrix<f64>>,
) -> Result<KciResult, KciError> {
    if x.nrows() == 0 || y.nrows() == 0 {
        return Err(KciError::EmptySample);
    }
    if x.nrows() != y.nrows() || z.is_some_and(|values| values.nrows() != x.nrows()) {
        return Err(KciError::RowMismatch);
    }
    if !finite(x) || !finite(y) || z.is_some_and(|values| !finite(values)) {
        return Err(KciError::NonFiniteValue);
    }
    let x = remove_constant_columns(x);
    let y = remove_constant_columns(y);
    if x.ncols() == 0 || y.ncols() == 0 {
        return Ok(KciResult {
            p_value: 1.0,
            statistic: 0.0,
            observations: x.nrows(),
            conditioning_columns: z.map_or(0, DMatrix::ncols),
        });
    }
    let z = z.map(remove_constant_columns);
    match z.as_ref() {
        Some(conditioning) if conditioning.ncols() > 0 => conditional(&x, &y, conditioning),
        _ => unconditional(&x, &y),
    }
}
