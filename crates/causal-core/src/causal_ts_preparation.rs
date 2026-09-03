//! Input preparation ported from causal-ts 0.26.0 at
//! `cd20a0d54054f7e852e584c9a0db31191ad56477`.
//!
//! CD-NOTS keeps missing values through lag embedding and removes rows per CI
//! query. GRACE instead requires a dense `f32` lag window, optionally after the
//! source VAR-EM imputer. The distinct types below prevent mixing those inputs.

use crate::tsdiag::var_fit_trend_c;
use nalgebra::{DMatrix, DVector};
use spec_math::cephes64::stdtr;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CausalTsPreparationError {
    EmptyMatrix,
    EmptyVariables,
    RaggedMatrix,
    NonFiniteObservedValue,
    InvalidLag,
    InvalidColumn { column: usize, columns: usize },
    SingularCiCovariance,
    AllMissingColumn { column: usize },
    VarDesignFailure,
    InsufficientRowsForGrace { rows: usize, max_lag: usize },
    MissingValuesForGrace { cells: usize },
}

impl fmt::Display for CausalTsPreparationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMatrix => {
                formatter.write_str("causal-ts preparation needs at least one row")
            }
            Self::EmptyVariables => {
                formatter.write_str("causal-ts preparation needs at least one variable")
            }
            Self::RaggedMatrix => {
                formatter.write_str("causal-ts preparation requires a rectangular matrix")
            }
            Self::NonFiniteObservedValue => {
                formatter.write_str("observed causal-ts values must be finite")
            }
            Self::InvalidLag => {
                formatter.write_str("the maximum lag must be smaller than the number of rows")
            }
            Self::InvalidColumn { column, columns } => write!(
                formatter,
                "embedded column {column} is outside 0..{columns}"
            ),
            Self::SingularCiCovariance => {
                formatter.write_str("the causal-ts partial-correlation covariance is singular")
            }
            Self::AllMissingColumn { column } => write!(
                formatter,
                "variable {column} has no observed value for the causal-ts mean fill"
            ),
            Self::VarDesignFailure => {
                formatter.write_str("the causal-ts VAR-EM regression could not be fitted")
            }
            Self::InsufficientRowsForGrace { rows, max_lag } => write!(
                formatter,
                "GRACE needs more than {max_lag} rows, but received {rows}"
            ),
            Self::MissingValuesForGrace { cells } => write!(
                formatter,
                "GRACE dense preparation received {cells} missing values"
            ),
        }
    }
}

impl std::error::Error for CausalTsPreparationError {}

fn validate_nullable(values: &[Vec<Option<f64>>]) -> Result<usize, CausalTsPreparationError> {
    let Some(first) = values.first() else {
        return Err(CausalTsPreparationError::EmptyMatrix);
    };
    if first.is_empty() {
        return Err(CausalTsPreparationError::EmptyVariables);
    }
    if values.iter().any(|row| row.len() != first.len()) {
        return Err(CausalTsPreparationError::RaggedMatrix);
    }
    if values
        .iter()
        .flatten()
        .flatten()
        .any(|value| !value.is_finite())
    {
        return Err(CausalTsPreparationError::NonFiniteObservedValue);
    }
    Ok(first.len())
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdnotsEmbedded {
    /// Rows after causal-ts's `2 * max_lag` leading cutoff. Lag blocks are
    /// ordered current, lag one, lag two, and so on.
    pub values: Vec<Vec<Option<f64>>>,
    /// Original time index for each embedded row.
    pub reference_points: Vec<usize>,
    pub variables: usize,
    pub max_lag: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CdnotsMissingPolicy {
    /// Keep missing cells in the lag matrix. Each CI query then retains only
    /// rows observed for that query's X, Y, and Z variables.
    PairwiseComplete { minimum_samples: usize },
    /// Fill the source matrix with causal-ts VAR-EM before lag embedding.
    VarEm(VarEmConfiguration),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CdnotsPrepared {
    PairwiseComplete {
        embedded: CdnotsEmbedded,
        minimum_samples: usize,
    },
    VarEm {
        embedded: CdnotsEmbedded,
        imputation: VarEmResult,
    },
}

/// `pd.concat([df.shift(i) for i in range(max_lag + 1)], axis=1)` followed by
/// causal-ts's `iloc[2 * max_lag:]` alignment.
pub fn cdnots_embed(
    values: &[Vec<Option<f64>>],
    max_lag: usize,
) -> Result<CdnotsEmbedded, CausalTsPreparationError> {
    let variables = validate_nullable(values)?;
    let leading = 2 * max_lag;
    if leading >= values.len() {
        return Err(CausalTsPreparationError::InvalidLag);
    }

    let reference_points: Vec<usize> = (leading..values.len()).collect();
    let embedded = reference_points
        .iter()
        .map(|&time| {
            let mut row = Vec::with_capacity(variables * (max_lag + 1));
            for lag in 0..=max_lag {
                row.extend(values[time - lag].iter().copied());
            }
            row
        })
        .collect();

    Ok(CdnotsEmbedded {
        values: embedded,
        reference_points,
        variables,
        max_lag,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct PairwiseCompleteSample {
    pub values: Vec<Vec<f64>>,
    pub retained_embedded_rows: Vec<usize>,
    pub retained_reference_points: Vec<usize>,
    pub sufficient: bool,
}

/// causal-ts CI tests remove a row only when X, Y, or one of the current Z
/// columns is NaN. Missing values in unrelated embedded columns do not remove it.
pub fn pairwise_complete_sample(
    embedded: &CdnotsEmbedded,
    x: usize,
    y: usize,
    z: &[usize],
    minimum_samples: usize,
) -> Result<PairwiseCompleteSample, CausalTsPreparationError> {
    let columns = embedded.variables * (embedded.max_lag + 1);
    for &column in std::iter::once(&x).chain(std::iter::once(&y)).chain(z) {
        if column >= columns {
            return Err(CausalTsPreparationError::InvalidColumn { column, columns });
        }
    }

    let selected: Vec<usize> = std::iter::once(x)
        .chain(std::iter::once(y))
        .chain(z.iter().copied())
        .collect();
    let retained_embedded_rows: Vec<usize> = embedded
        .values
        .iter()
        .enumerate()
        .filter_map(|(row, values)| {
            selected
                .iter()
                .all(|&column| values[column].is_some())
                .then_some(row)
        })
        .collect();
    let retained_reference_points = retained_embedded_rows
        .iter()
        .map(|&row| embedded.reference_points[row])
        .collect();
    let values = retained_embedded_rows
        .iter()
        .map(|&row| {
            embedded.values[row]
                .iter()
                .map(|value| value.unwrap_or(f64::NAN))
                .collect()
        })
        .collect();

    Ok(PairwiseCompleteSample {
        sufficient: retained_embedded_rows.len() >= minimum_samples,
        values,
        retained_embedded_rows,
        retained_reference_points,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CausalTsParCorr {
    pub p_value: f64,
    pub statistic: f64,
    pub observations: usize,
}

fn standardize(values: &[f32]) -> Vec<f32> {
    let mean = values.iter().sum::<f32>() / values.len() as f32;
    let sum_squares = values
        .iter()
        .map(|value| {
            let centered = *value - mean;
            centered * centered
        })
        .sum::<f32>();
    let deviation = if values.len() > 1 {
        (sum_squares / (values.len() - 1) as f32).sqrt()
    } else {
        0.0
    };
    let scale = if deviation == 0.0 { 1.0 } else { deviation };
    values.iter().map(|value| (*value - mean) / scale).collect()
}

fn residualize(
    conditions: &DMatrix<f32>,
    target: &[f32],
) -> Result<Vec<f32>, CausalTsPreparationError> {
    if conditions.ncols() == 0 {
        return Ok(target.to_vec());
    }
    let target = DVector::from_column_slice(target);
    let decomposition = conditions.clone().svd(true, true);
    let largest = decomposition
        .singular_values
        .iter()
        .copied()
        .fold(0.0_f32, f32::max);
    let tolerance = f32::EPSILON * conditions.nrows().max(conditions.ncols()) as f32 * largest;
    let coefficients = decomposition
        .solve(&target, tolerance)
        .map_err(|_| CausalTsPreparationError::VarDesignFailure)?;
    Ok((target - conditions * coefficients)
        .iter()
        .copied()
        .collect())
}

/// The source `ParCorrGPU` path, including its `f32` standardization and the
/// `(p=1, statistic=0)` insufficient-complete-row result.
pub fn causal_ts_parcorr(
    embedded: &CdnotsEmbedded,
    x: usize,
    y: usize,
    z: &[usize],
    minimum_samples: usize,
) -> Result<CausalTsParCorr, CausalTsPreparationError> {
    let sample = pairwise_complete_sample(embedded, x, y, z, minimum_samples)?;
    let observations = sample.values.len();
    if !sample.sufficient {
        return Ok(CausalTsParCorr {
            p_value: 1.0,
            statistic: 0.0,
            observations,
        });
    }

    let x_values: Vec<f32> = sample.values.iter().map(|row| row[x] as f32).collect();
    let y_values: Vec<f32> = sample.values.iter().map(|row| row[y] as f32).collect();
    let x_values = standardize(&x_values);
    let y_values = standardize(&y_values);
    let conditions = DMatrix::from_fn(observations, z.len(), |row, column| {
        sample.values[row][z[column]] as f32
    });
    let conditions = DMatrix::from_fn(observations, z.len(), |row, column| {
        let column_values: Vec<f32> = (0..observations)
            .map(|index| conditions[(index, column)])
            .collect();
        standardize(&column_values)[row]
    });
    let x_residual = residualize(&conditions, &x_values)?;
    let y_residual = residualize(&conditions, &y_values)?;
    let x_mean = x_residual.iter().sum::<f32>() / observations as f32;
    let y_mean = y_residual.iter().sum::<f32>() / observations as f32;
    let numerator = x_residual
        .iter()
        .zip(&y_residual)
        .map(|(left, right)| (*left - x_mean) * (*right - y_mean))
        .sum::<f32>();
    let x_sum = x_residual
        .iter()
        .map(|value| (*value - x_mean).powi(2))
        .sum::<f32>();
    let y_sum = y_residual
        .iter()
        .map(|value| (*value - y_mean).powi(2))
        .sum::<f32>();
    let denominator = (x_sum * y_sum).sqrt();
    let statistic = if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    } as f64;
    let degrees_of_freedom = observations - z.len() - 2;
    let clipped = statistic.clamp(-1.0 + 1e-15, 1.0 - 1e-15);
    let transformed = clipped * (degrees_of_freedom as f64 / (1.0 - clipped * clipped)).sqrt();
    let p_value = 2.0 * stdtr(degrees_of_freedom as isize, -transformed.abs());

    Ok(CausalTsParCorr {
        p_value,
        statistic,
        observations,
    })
}

/// causal-ts `ParCorrGPU(method="precision")` batched path. Dense data uses a
/// global float32 covariance and a precision submatrix; nullable data disables
/// batching in the source and therefore falls back to the OLS path above.
pub fn causal_ts_precision_parcorr(
    embedded: &CdnotsEmbedded,
    x: usize,
    y: usize,
    z: &[usize],
    minimum_samples: usize,
) -> Result<CausalTsParCorr, CausalTsPreparationError> {
    if embedded.values.iter().flatten().any(Option::is_none) {
        return causal_ts_parcorr(embedded, x, y, z, minimum_samples);
    }
    let columns = embedded.variables * (embedded.max_lag + 1);
    for &column in std::iter::once(&x).chain(std::iter::once(&y)).chain(z) {
        if column >= columns {
            return Err(CausalTsPreparationError::InvalidColumn { column, columns });
        }
    }
    let observations = embedded.values.len();
    if observations < minimum_samples {
        return Ok(CausalTsParCorr {
            p_value: 1.0,
            statistic: 0.0,
            observations,
        });
    }

    let dense = DMatrix::from_fn(observations, columns, |row, column| {
        embedded.values[row][column].unwrap_or(f64::NAN) as f32
    });
    let means: Vec<f32> = (0..columns)
        .map(|column| dense.column(column).iter().sum::<f32>() / observations as f32)
        .collect();
    let centered = DMatrix::from_fn(observations, columns, |row, column| {
        dense[(row, column)] - means[column]
    });
    let covariance = centered.transpose() * centered / (observations - 1) as f32;
    let selected: Vec<usize> = std::iter::once(x)
        .chain(std::iter::once(y))
        .chain(z.iter().copied())
        .collect();
    let sub = DMatrix::from_fn(selected.len(), selected.len(), |row, column| {
        covariance[(selected[row], selected[column])]
    });
    let statistic = if selected.len() == 2 {
        sub[(0, 1)] / (sub[(0, 0)] * sub[(1, 1)]).sqrt()
    } else {
        let precision = sub
            .try_inverse()
            .ok_or(CausalTsPreparationError::SingularCiCovariance)?;
        -precision[(0, 1)] / (precision[(0, 0)] * precision[(1, 1)]).sqrt()
    };
    let statistic = (statistic as f64).clamp(-1.0 + 1e-15, 1.0 - 1e-15);
    let degrees_of_freedom = observations - z.len() - 2;
    let transformed =
        statistic * (degrees_of_freedom as f64 / (1.0 - statistic * statistic)).sqrt();
    let p_value = 2.0 * stdtr(degrees_of_freedom as isize, -transformed.abs());
    Ok(CausalTsParCorr {
        p_value,
        statistic,
        observations,
    })
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VarEmConfiguration {
    pub lags: usize,
    pub maximum_iterations: usize,
    pub tolerance: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VarEmTermination {
    NoMissingValues,
    ColumnMeanFallback,
    Converged,
    IterationLimit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarEmResult {
    pub values: Vec<Vec<f64>>,
    pub imputed_cells: Vec<(usize, usize)>,
    pub iterations: usize,
    pub termination: VarEmTermination,
}

fn initial_mean_fill(
    values: &[Vec<Option<f64>>],
) -> Result<Vec<Vec<f64>>, CausalTsPreparationError> {
    let columns = validate_nullable(values)?;
    let mut means = vec![0.0; columns];
    let mut counts = vec![0_usize; columns];
    for row in values {
        for (column, value) in row.iter().enumerate() {
            if let Some(value) = value {
                means[column] += value;
                counts[column] += 1;
            }
        }
    }
    for column in 0..columns {
        if counts[column] == 0 {
            return Err(CausalTsPreparationError::AllMissingColumn { column });
        }
        means[column] /= counts[column] as f64;
    }
    Ok(values
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(column, value)| value.unwrap_or(means[column]))
                .collect()
        })
        .collect())
}

fn var_fitted_values(values: &[Vec<f64>], lags: usize) -> Vec<Vec<f64>> {
    let fit = var_fit_trend_c(values, lags);
    let variables = values[0].len();
    (lags..values.len())
        .map(|time| {
            let mut prediction = fit.params[0].clone();
            for lag in 1..=lags {
                for cause in 0..variables {
                    for effect in 0..variables {
                        prediction[effect] += fit.params[1 + (lag - 1) * variables + cause][effect]
                            * values[time - lag][cause];
                    }
                }
            }
            prediction
        })
        .collect()
}

/// causal-ts `var_em_impute`: mean initialization followed by repeated
/// statsmodels-compatible `VAR(p, trend="c")` fitted-value replacement.
pub fn causal_ts_var_em(
    values: &[Vec<Option<f64>>],
    configuration: VarEmConfiguration,
) -> Result<VarEmResult, CausalTsPreparationError> {
    let variables = validate_nullable(values)?;
    if configuration.lags >= values.len() {
        return Err(CausalTsPreparationError::InvalidLag);
    }
    let imputed_cells: Vec<(usize, usize)> = values
        .iter()
        .enumerate()
        .flat_map(|(row, values)| {
            values
                .iter()
                .enumerate()
                .filter_map(move |(column, value)| value.is_none().then_some((row, column)))
        })
        .collect();
    if imputed_cells.is_empty() {
        return Ok(VarEmResult {
            values: values
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|value| value.expect("validated observed value"))
                        .collect()
                })
                .collect(),
            imputed_cells,
            iterations: 0,
            termination: VarEmTermination::NoMissingValues,
        });
    }

    let mut filled = initial_mean_fill(values)?;
    if values.len() - configuration.lags < 2 * variables {
        return Ok(VarEmResult {
            values: filled,
            imputed_cells,
            iterations: 0,
            termination: VarEmTermination::ColumnMeanFallback,
        });
    }

    let mut previous: Vec<f64> = imputed_cells
        .iter()
        .map(|&(row, column)| filled[row][column])
        .collect();
    for iteration in 0..configuration.maximum_iterations {
        let fitted = var_fitted_values(&filled, configuration.lags);
        for &(row, column) in &imputed_cells {
            if row >= configuration.lags {
                filled[row][column] = fitted[row - configuration.lags][column];
            }
        }
        let current: Vec<f64> = imputed_cells
            .iter()
            .map(|&(row, column)| filled[row][column])
            .collect();
        let delta = current
            .iter()
            .zip(&previous)
            .map(|(left, right)| (left - right).abs())
            .fold(0.0_f64, f64::max);
        if delta < configuration.tolerance {
            return Ok(VarEmResult {
                values: filled,
                imputed_cells,
                iterations: iteration + 1,
                termination: VarEmTermination::Converged,
            });
        }
        previous = current;
    }

    Ok(VarEmResult {
        values: filled,
        imputed_cells,
        iterations: configuration.maximum_iterations,
        termination: VarEmTermination::IterationLimit,
    })
}

/// Prepare the two causal-ts missing-data lanes without erasing which lane was
/// chosen. Both CD-NOTS and CD-NOTS+ use this preparation before their
/// different skeleton searches.
pub fn prepare_cdnots(
    values: &[Vec<Option<f64>>],
    max_lag: usize,
    policy: CdnotsMissingPolicy,
) -> Result<CdnotsPrepared, CausalTsPreparationError> {
    match policy {
        CdnotsMissingPolicy::PairwiseComplete { minimum_samples } => {
            let embedded = cdnots_embed(values, max_lag)?;
            Ok(CdnotsPrepared::PairwiseComplete {
                embedded,
                minimum_samples,
            })
        }
        CdnotsMissingPolicy::VarEm(configuration) => {
            let imputation = causal_ts_var_em(values, configuration)?;
            let dense = imputation
                .values
                .iter()
                .map(|row| row.iter().copied().map(Some).collect())
                .collect::<Vec<Vec<_>>>();
            let embedded = cdnots_embed(&dense, max_lag)?;
            Ok(CdnotsPrepared::VarEm {
                embedded,
                imputation,
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CausalTsContextPreset {
    Linear,
    LinearSine,
    LinearExponential,
    LinearQuadratic,
    Step,
    StepLinear,
}

/// causal-ts `make_c_array`, including its one-cycle sine convention.
pub fn causal_ts_context(rows: usize, preset: CausalTsContextPreset) -> Vec<Vec<f64>> {
    (0..rows)
        .map(|time| {
            let time = time as f64;
            match preset {
                CausalTsContextPreset::Linear => vec![time],
                CausalTsContextPreset::LinearSine => vec![
                    time,
                    (2.0 * std::f64::consts::PI * time / rows as f64).sin(),
                ],
                CausalTsContextPreset::LinearExponential => {
                    vec![time, (time / rows as f64).exp() - 1.0]
                }
                CausalTsContextPreset::LinearQuadratic => vec![time, time * time / rows as f64],
                CausalTsContextPreset::Step => vec![(time as usize >= rows / 2) as u8 as f64],
                CausalTsContextPreset::StepLinear => {
                    vec![(time as usize >= rows / 2) as u8 as f64, time]
                }
            }
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct GraceDenseWindows {
    /// `[reference][variable][oldest..current]`, matching torch `unfold`.
    pub windows: Vec<Vec<Vec<f32>>>,
    pub rows: usize,
    pub variables: usize,
    pub max_lag: usize,
    pub normalized: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraceMissingPolicy {
    /// Match `prepare_data`: reject a nullable input rather than constructing
    /// a neural tensor with missing values.
    Reject,
    /// Match `run_cdnots_gated(impute="var_em")`.
    VarEm(VarEmConfiguration),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GracePrepared {
    pub windows: GraceDenseWindows,
    pub imputation: Option<VarEmResult>,
}

/// causal-ts GRACE `prepare_data`: cast to float32, optional population-z-score,
/// then construct overlapping oldest-to-current windows.
pub fn prepare_grace_dense(
    values: &[Vec<f64>],
    max_lag: usize,
    normalize: bool,
) -> Result<GraceDenseWindows, CausalTsPreparationError> {
    let nullable: Vec<Vec<Option<f64>>> = values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| value.is_finite().then_some(*value))
                .collect()
        })
        .collect();
    let variables = validate_nullable(&nullable)?;
    let missing = values
        .iter()
        .flatten()
        .filter(|value| !value.is_finite())
        .count();
    if missing > 0 {
        return Err(CausalTsPreparationError::MissingValuesForGrace { cells: missing });
    }
    if values.len() <= max_lag {
        return Err(CausalTsPreparationError::InsufficientRowsForGrace {
            rows: values.len(),
            max_lag,
        });
    }

    let mut prepared: Vec<Vec<f32>> = values
        .iter()
        .map(|row| row.iter().map(|value| *value as f32).collect())
        .collect();
    if normalize {
        for variable in 0..variables {
            let mean =
                prepared.iter().map(|row| row[variable]).sum::<f32>() / prepared.len() as f32;
            let variance = prepared
                .iter()
                .map(|row| {
                    let centered = row[variable] - mean;
                    centered * centered
                })
                .sum::<f32>()
                / prepared.len() as f32;
            let scale = variance.sqrt() + 1e-8_f32;
            for row in &mut prepared {
                row[variable] = (row[variable] - mean) / scale;
            }
        }
    }

    let windows = (max_lag..prepared.len())
        .map(|reference| {
            (0..variables)
                .map(|variable| {
                    (reference - max_lag..=reference)
                        .map(|time| prepared[time][variable])
                        .collect()
                })
                .collect()
        })
        .collect();

    Ok(GraceDenseWindows {
        windows,
        rows: values.len(),
        variables,
        max_lag,
        normalized: normalize,
    })
}

/// Produce GRACE's dense neural input while recording whether the source was
/// accepted as dense or first filled by causal-ts VAR-EM.
pub fn prepare_grace(
    values: &[Vec<Option<f64>>],
    max_lag: usize,
    normalize: bool,
    policy: GraceMissingPolicy,
) -> Result<GracePrepared, CausalTsPreparationError> {
    validate_nullable(values)?;
    let missing = values
        .iter()
        .flatten()
        .filter(|value| value.is_none())
        .count();

    match policy {
        GraceMissingPolicy::Reject if missing > 0 => {
            Err(CausalTsPreparationError::MissingValuesForGrace { cells: missing })
        }
        GraceMissingPolicy::Reject => {
            let dense = values
                .iter()
                .map(|row| row.iter().flatten().copied().collect())
                .collect::<Vec<Vec<_>>>();
            Ok(GracePrepared {
                windows: prepare_grace_dense(&dense, max_lag, normalize)?,
                imputation: None,
            })
        }
        GraceMissingPolicy::VarEm(configuration) => {
            let imputation = causal_ts_var_em(values, configuration)?;
            Ok(GracePrepared {
                windows: prepare_grace_dense(&imputation.values, max_lag, normalize)?,
                imputation: Some(imputation),
            })
        }
    }
}
