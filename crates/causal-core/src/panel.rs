//! Balanced-panel preparation and the simultaneous-adoption estimators from `synthdid`.
//!
//! The reference is `synth-inference/synthdid` at commit
//! `70c1ce3eac58e28c30b67435ca377bb48baa9b8a`.  A long panel is validated before
//! it is reshaped: every unit-time cell must occur exactly once, outcomes must be
//! finite, treatment must be binary, and treated units must adopt simultaneously
//! and remain treated.  The resulting matrix has control units first and treated
//! units last, matching `panel.matrices(..., treated.last = TRUE)`.

use std::collections::{BTreeMap, BTreeSet};

use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, PartialEq)]
pub struct PanelObservation {
    pub unit: String,
    /// Ordered integer time coordinate. Hirmos can map dates/timestamps to this
    /// representation without discarding the original display value.
    pub time: i64,
    pub outcome: f64,
    /// Kept numeric at the import boundary so values other than zero and one are
    /// rejected instead of being coerced to `bool`.
    pub treatment: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelError {
    Empty,
    EmptyUnit { row: usize },
    NonFiniteOutcome { row: usize },
    NonFiniteTreatment { row: usize },
    TreatmentNotBinary { row: usize },
    DuplicateCell { unit: String, time: i64 },
    MissingCell { unit: String, time: i64 },
    NoTreatmentVariation,
    NoPreTreatmentPeriod,
    NoPostTreatmentPeriod,
    NoControlUnit,
    NoTreatedUnit,
    NonSimultaneousAdoption,
    InvalidMatrixBoundary,
    DegenerateNoise,
    TooFewControlsForPlacebo,
    InsufficientPlaceboReplications,
    InvalidPlaceboPermutation { replication: usize },
    InvalidTreatedFraction,
}

#[derive(Debug, Clone)]
pub struct PanelMatrices {
    /// Unit-by-time outcome matrix.
    pub y: DMatrix<f64>,
    /// Unit-by-time treatment matrix containing only zero and one.
    pub w: DMatrix<f64>,
    /// Control units followed by treated units; alphabetical within each block.
    pub units: Vec<String>,
    /// Strictly increasing time coordinates.
    pub times: Vec<i64>,
    /// Number of control units at the start of `y`.
    pub n0: usize,
    /// Number of pre-treatment periods at the start of `y`.
    pub t0: usize,
}

/// Rust equivalent of `synthdid::panel.matrices` for a typed long panel.
pub fn panel_matrices(rows: &[PanelObservation]) -> Result<PanelMatrices, PanelError> {
    if rows.is_empty() {
        return Err(PanelError::Empty);
    }

    let mut units = BTreeSet::new();
    let mut times = BTreeSet::new();
    let mut cells = BTreeMap::new();
    let mut saw_zero = false;
    let mut saw_one = false;

    for (row, observation) in rows.iter().enumerate() {
        if observation.unit.is_empty() {
            return Err(PanelError::EmptyUnit { row });
        }
        if !observation.outcome.is_finite() {
            return Err(PanelError::NonFiniteOutcome { row });
        }
        if !observation.treatment.is_finite() {
            return Err(PanelError::NonFiniteTreatment { row });
        }
        if observation.treatment != 0.0 && observation.treatment != 1.0 {
            return Err(PanelError::TreatmentNotBinary { row });
        }
        saw_zero |= observation.treatment == 0.0;
        saw_one |= observation.treatment == 1.0;
        units.insert(observation.unit.clone());
        times.insert(observation.time);
        let key = (observation.unit.clone(), observation.time);
        if cells.insert(key.clone(), observation).is_some() {
            return Err(PanelError::DuplicateCell {
                unit: key.0,
                time: key.1,
            });
        }
    }
    if !saw_zero || !saw_one {
        return Err(PanelError::NoTreatmentVariation);
    }

    let alphabetical_units: Vec<String> = units.into_iter().collect();
    let times: Vec<i64> = times.into_iter().collect();
    for unit in &alphabetical_units {
        for &time in &times {
            if !cells.contains_key(&(unit.clone(), time)) {
                return Err(PanelError::MissingCell {
                    unit: unit.clone(),
                    time,
                });
            }
        }
    }

    let first_treated = times
        .iter()
        .position(|&time| {
            alphabetical_units
                .iter()
                .any(|unit| cells[&(unit.clone(), time)].treatment == 1.0)
        })
        .ok_or(PanelError::NoTreatedUnit)?;
    if first_treated == 0 {
        return Err(PanelError::NoPreTreatmentPeriod);
    }
    if first_treated >= times.len() {
        return Err(PanelError::NoPostTreatmentPeriod);
    }

    let mut controls = Vec::new();
    let mut treated = Vec::new();
    for unit in alphabetical_units {
        let ever_treated = times
            .iter()
            .any(|&time| cells[&(unit.clone(), time)].treatment == 1.0);
        if ever_treated {
            treated.push(unit);
        } else {
            controls.push(unit);
        }
    }
    if controls.is_empty() {
        return Err(PanelError::NoControlUnit);
    }
    if treated.is_empty() {
        return Err(PanelError::NoTreatedUnit);
    }

    for unit in &controls {
        if times
            .iter()
            .any(|&time| cells[&(unit.clone(), time)].treatment != 0.0)
        {
            return Err(PanelError::NonSimultaneousAdoption);
        }
    }
    for unit in &treated {
        for (time_index, &time) in times.iter().enumerate() {
            let expected = if time_index < first_treated { 0.0 } else { 1.0 };
            if cells[&(unit.clone(), time)].treatment != expected {
                return Err(PanelError::NonSimultaneousAdoption);
            }
        }
    }

    let n0 = controls.len();
    controls.extend(treated);
    let ordered_units = controls;
    let y = DMatrix::from_fn(ordered_units.len(), times.len(), |i, t| {
        cells[&(ordered_units[i].clone(), times[t])].outcome
    });
    let w = DMatrix::from_fn(ordered_units.len(), times.len(), |i, t| {
        cells[&(ordered_units[i].clone(), times[t])].treatment
    });

    Ok(PanelMatrices {
        y,
        w,
        units: ordered_units,
        times,
        n0,
        t0: first_treated,
    })
}

#[derive(Debug, Clone)]
pub struct PanelEstimate {
    pub estimate: f64,
    /// Weights on the pre-treatment periods.
    pub lambda: Vec<f64>,
    /// Weights on the control units.
    pub omega: Vec<f64>,
    /// Post-period effect after subtracting the weighted pre-period gap.
    pub effect_curve: Vec<f64>,
    pub lambda_iterations: usize,
    pub omega_iterations: usize,
    /// Penalised-MSE trace from the final (post-sparsification) reference run.
    pub lambda_objective: Vec<f64>,
    /// Penalised-MSE trace from the final (post-sparsification) reference run.
    pub omega_objective: Vec<f64>,
    pub noise_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelEstimatorKind {
    DifferenceInDifferences,
    SyntheticControl,
    SyntheticDifferenceInDifferences,
}

#[derive(Debug, Clone)]
pub struct PanelPlaceboInference {
    pub estimates: Vec<f64>,
    pub standard_error: f64,
}

#[derive(Clone, Copy)]
struct FitConfiguration {
    noise_level: f64,
    zeta_lambda: f64,
    zeta_omega: f64,
    min_decrease: f64,
    lambda_intercept: bool,
    omega_intercept: bool,
}

fn validate_boundary(y: &DMatrix<f64>, n0: usize, t0: usize) -> Result<(), PanelError> {
    if y.nrows() < 2
        || y.ncols() < 2
        || n0 == 0
        || n0 >= y.nrows()
        || t0 == 0
        || t0 >= y.ncols()
        || y.iter().any(|value| !value.is_finite())
    {
        return Err(PanelError::InvalidMatrixBoundary);
    }
    Ok(())
}

fn collapsed_form(y: &DMatrix<f64>, n0: usize, t0: usize) -> DMatrix<f64> {
    let n = y.nrows();
    let t = y.ncols();
    let n1 = n - n0;
    let t1 = t - t0;
    DMatrix::from_fn(n0 + 1, t0 + 1, |i, j| match (i < n0, j < t0) {
        (true, true) => y[(i, j)],
        (true, false) => (t0..t).map(|k| y[(i, k)]).sum::<f64>() / t1 as f64,
        (false, true) => (n0..n).map(|k| y[(k, j)]).sum::<f64>() / n1 as f64,
        (false, false) => {
            let total = (n0..n)
                .flat_map(|i| (t0..t).map(move |j| y[(i, j)]))
                .sum::<f64>();
            total / (n1 * t1) as f64
        }
    })
}

fn sample_noise_level(y: &DMatrix<f64>, n0: usize, t0: usize) -> Result<f64, PanelError> {
    let mut differences = Vec::with_capacity(n0 * t0.saturating_sub(1));
    for i in 0..n0 {
        for t in 1..t0 {
            differences.push(y[(i, t)] - y[(i, t - 1)]);
        }
    }
    if differences.len() < 2 {
        return Err(PanelError::DegenerateNoise);
    }
    let mean = differences.iter().sum::<f64>() / differences.len() as f64;
    let variance = differences
        .iter()
        .map(|value| (value - mean) * (value - mean))
        .sum::<f64>()
        / (differences.len() - 1) as f64;
    let noise = variance.sqrt();
    if !noise.is_finite() || noise == 0.0 {
        return Err(PanelError::DegenerateNoise);
    }
    Ok(noise)
}

fn centered_columns(y: &DMatrix<f64>) -> DMatrix<f64> {
    DMatrix::from_fn(y.nrows(), y.ncols(), |i, j| {
        let mean = (0..y.nrows()).map(|row| y[(row, j)]).sum::<f64>() / y.nrows() as f64;
        y[(i, j)] - mean
    })
}

fn fw_step(a: &DMatrix<f64>, x: &DVector<f64>, b: &DVector<f64>, eta: f64) -> DVector<f64> {
    let ax = a * x;
    let residual = &ax - b;
    let half_gradient = a.transpose() * residual + x * eta;
    let vertex = (0..half_gradient.len())
        .min_by(|&left, &right| half_gradient[left].total_cmp(&half_gradient[right]))
        .expect("simplex is non-empty");
    let mut direction = -x;
    direction[vertex] = 1.0 - x[vertex];
    if direction.iter().all(|value| *value == 0.0) {
        return x.clone();
    }
    let error_direction = a.column(vertex) - ax;
    let denominator = error_direction.norm_squared() + eta * direction.norm_squared();
    if denominator == 0.0 {
        return x.clone();
    }
    let step = (-half_gradient.dot(&direction) / denominator).clamp(0.0, 1.0);
    x + direction * step
}

struct WeightFit {
    weights: Vec<f64>,
    values: Vec<f64>,
}

fn sc_weight_fw(
    input: &DMatrix<f64>,
    zeta: f64,
    intercept: bool,
    initial: Option<&[f64]>,
    min_decrease: f64,
    max_iterations: usize,
) -> WeightFit {
    let target_column = input.ncols() - 1;
    let rows = input.nrows();
    let y = if intercept {
        centered_columns(input)
    } else {
        input.clone()
    };
    let a = y.columns(0, target_column).into_owned();
    let b = y.column(target_column).into_owned();
    let mut weights = initial
        .map(DVector::from_column_slice)
        .unwrap_or_else(|| DVector::from_element(target_column, 1.0 / target_column as f64));
    let eta = rows as f64 * zeta * zeta;
    let threshold = min_decrease * min_decrease;
    let mut values = Vec::with_capacity(max_iterations);

    while values.len() < max_iterations
        && (values.len() < 2 || values[values.len() - 2] - values[values.len() - 1] > threshold)
    {
        weights = fw_step(&a, &weights, &b, eta);
        let mut coefficients = Vec::with_capacity(weights.len() + 1);
        coefficients.extend(weights.iter().copied());
        coefficients.push(-1.0);
        let error = &y * DVector::from_vec(coefficients);
        values.push(zeta * zeta * weights.norm_squared() + error.norm_squared() / rows as f64);
    }
    WeightFit {
        weights: weights.iter().copied().collect(),
        values,
    }
}

fn sparsify(weights: &[f64]) -> Vec<f64> {
    let maximum = weights.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut sparse: Vec<f64> = weights
        .iter()
        .map(|&weight| if weight <= maximum / 4.0 { 0.0 } else { weight })
        .collect();
    let total = sparse.iter().sum::<f64>();
    for weight in &mut sparse {
        *weight /= total;
    }
    sparse
}

fn estimate_from_weights(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    lambda: Vec<f64>,
    omega: Vec<f64>,
    lambda_iterations: usize,
    omega_iterations: usize,
    lambda_objective: Vec<f64>,
    omega_objective: Vec<f64>,
    noise_level: f64,
) -> PanelEstimate {
    let n1 = y.nrows() - n0;
    let t1 = y.ncols() - t0;
    let mut unit_contrast = DVector::from_element(y.nrows(), 1.0 / n1 as f64);
    for i in 0..n0 {
        unit_contrast[i] = -omega[i];
    }
    let mut time_contrast = DVector::from_element(y.ncols(), 1.0 / t1 as f64);
    for t in 0..t0 {
        time_contrast[t] = -lambda[t];
    }
    let unit_gap = y.transpose() * &unit_contrast;
    let baseline = (0..t0).map(|t| unit_gap[t] * lambda[t]).sum::<f64>();
    let effect_curve = (t0..y.ncols()).map(|t| unit_gap[t] - baseline).collect();
    let estimate = unit_contrast.dot(&(y * time_contrast));
    PanelEstimate {
        estimate,
        lambda,
        omega,
        effect_curve,
        lambda_iterations,
        omega_iterations,
        lambda_objective,
        omega_objective,
        noise_level,
    }
}

/// Conventional simultaneous-adoption difference in differences. This is the
/// `synthdid::did_estimate` weighting system: uniform control-unit weights and
/// uniform pre-treatment time weights.
pub fn did_estimate(y: &DMatrix<f64>, n0: usize, t0: usize) -> Result<PanelEstimate, PanelError> {
    validate_boundary(y, n0, t0)?;
    // DiD uses fixed uniform weights, so the noise scale is diagnostic only. A
    // deterministic panel remains a valid DiD input even though SDID's ridge
    // calibration would be undefined.
    let noise_level = sample_noise_level(y, n0, t0).unwrap_or(0.0);
    Ok(estimate_from_weights(
        y,
        n0,
        t0,
        vec![1.0 / t0 as f64; t0],
        vec![1.0 / n0 as f64; n0],
        0,
        0,
        Vec::new(),
        Vec::new(),
        noise_level,
    ))
}

fn fitted_weights(
    design: &DMatrix<f64>,
    zeta: f64,
    intercept: bool,
    min_decrease: f64,
) -> WeightFit {
    fitted_weights_from_initial(design, zeta, intercept, min_decrease, None)
}

fn fitted_weights_from_initial(
    design: &DMatrix<f64>,
    zeta: f64,
    intercept: bool,
    min_decrease: f64,
    initial: Option<&[f64]>,
) -> WeightFit {
    let preliminary = sc_weight_fw(design, zeta, intercept, initial, min_decrease, 100);
    let initial = sparsify(&preliminary.weights);
    sc_weight_fw(
        design,
        zeta,
        intercept,
        Some(&initial),
        min_decrease,
        10_000,
    )
}

fn fit_configuration(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    estimator: PanelEstimatorKind,
) -> Result<FitConfiguration, PanelError> {
    validate_boundary(y, n0, t0)?;
    let noise_level = match estimator {
        PanelEstimatorKind::DifferenceInDifferences => sample_noise_level(y, n0, t0).unwrap_or(0.0),
        _ => sample_noise_level(y, n0, t0)?,
    };
    let post_units = y.nrows() - n0;
    let post_periods = y.ncols() - t0;
    Ok(FitConfiguration {
        noise_level,
        zeta_lambda: 1e-6 * noise_level,
        zeta_omega: match estimator {
            PanelEstimatorKind::DifferenceInDifferences => 0.0,
            PanelEstimatorKind::SyntheticControl => 1e-6 * noise_level,
            PanelEstimatorKind::SyntheticDifferenceInDifferences => {
                ((post_units * post_periods) as f64).powf(0.25) * noise_level
            }
        },
        min_decrease: 1e-5 * noise_level,
        lambda_intercept: estimator != PanelEstimatorKind::SyntheticControl,
        omega_intercept: estimator != PanelEstimatorKind::SyntheticControl,
    })
}

fn normalize_or_uniform(weights: &[f64]) -> Vec<f64> {
    let total = weights.iter().sum::<f64>();
    if total != 0.0 {
        weights.iter().map(|weight| weight / total).collect()
    } else {
        vec![1.0 / weights.len() as f64; weights.len()]
    }
}

fn refit_with_configuration(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    estimator: PanelEstimatorKind,
    configuration: FitConfiguration,
    initial_lambda: Option<&[f64]>,
    initial_omega: Option<&[f64]>,
) -> Result<PanelEstimate, PanelError> {
    validate_boundary(y, n0, t0)?;
    let collapsed = collapsed_form(y, n0, t0);

    let lambda = match estimator {
        PanelEstimatorKind::DifferenceInDifferences => WeightFit {
            weights: initial_lambda
                .map(normalize_or_uniform)
                .unwrap_or_else(|| vec![1.0 / t0 as f64; t0]),
            values: Vec::new(),
        },
        PanelEstimatorKind::SyntheticControl => WeightFit {
            weights: vec![0.0; t0],
            values: Vec::new(),
        },
        PanelEstimatorKind::SyntheticDifferenceInDifferences => {
            let design = collapsed.rows(0, n0).into_owned();
            fitted_weights_from_initial(
                &design,
                configuration.zeta_lambda,
                configuration.lambda_intercept,
                configuration.min_decrease,
                initial_lambda,
            )
        }
    };

    let omega = match estimator {
        PanelEstimatorKind::DifferenceInDifferences => WeightFit {
            weights: initial_omega
                .map(normalize_or_uniform)
                .unwrap_or_else(|| vec![1.0 / n0 as f64; n0]),
            values: Vec::new(),
        },
        PanelEstimatorKind::SyntheticControl
        | PanelEstimatorKind::SyntheticDifferenceInDifferences => {
            let design = collapsed.columns(0, t0).transpose();
            fitted_weights_from_initial(
                &design,
                configuration.zeta_omega,
                configuration.omega_intercept,
                configuration.min_decrease,
                initial_omega,
            )
        }
    };

    Ok(estimate_from_weights(
        y,
        n0,
        t0,
        lambda.weights,
        omega.weights,
        lambda.values.len(),
        omega.values.len(),
        lambda.values,
        omega.values,
        configuration.noise_level,
    ))
}

fn fit_by_kind(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    estimator: PanelEstimatorKind,
) -> Result<PanelEstimate, PanelError> {
    match estimator {
        PanelEstimatorKind::DifferenceInDifferences => did_estimate(y, n0, t0),
        PanelEstimatorKind::SyntheticControl => synthdid_sc_estimate(y, n0, t0),
        PanelEstimatorKind::SyntheticDifferenceInDifferences => synthetic_did_estimate(y, n0, t0),
    }
}

/// Algorithm 4 of Arkhangelsky et al., matching `synthdid::placebo_se`.
///
/// `control_permutations` holds zero-based permutations of the original control
/// rows. Keeping the random permutations at the boundary separates parity of the
/// numerical refits from R's RNG implementation.
pub fn panel_placebo_standard_error(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    estimator: PanelEstimatorKind,
    control_permutations: &[Vec<usize>],
) -> Result<PanelPlaceboInference, PanelError> {
    validate_boundary(y, n0, t0)?;
    if control_permutations.len() < 2 {
        return Err(PanelError::InsufficientPlaceboReplications);
    }
    let treated_units = y.nrows() - n0;
    if n0 <= treated_units {
        return Err(PanelError::TooFewControlsForPlacebo);
    }
    let placebo_n0 = n0 - treated_units;
    let base = fit_by_kind(y, n0, t0, estimator)?;
    let configuration = fit_configuration(y, n0, t0, estimator)?;
    let mut estimates = Vec::with_capacity(control_permutations.len());

    for (replication, permutation) in control_permutations.iter().enumerate() {
        let mut sorted = permutation.clone();
        sorted.sort_unstable();
        if permutation.len() != n0 || sorted != (0..n0).collect::<Vec<_>>() {
            return Err(PanelError::InvalidPlaceboPermutation { replication });
        }
        let placebo_y =
            DMatrix::from_fn(n0, y.ncols(), |row, column| y[(permutation[row], column)]);
        let initial_omega = normalize_or_uniform(
            &permutation[..placebo_n0]
                .iter()
                .map(|&index| base.omega[index])
                .collect::<Vec<_>>(),
        );
        let estimate = refit_with_configuration(
            &placebo_y,
            placebo_n0,
            t0,
            estimator,
            configuration,
            Some(&base.lambda),
            Some(&initial_omega),
        )?;
        estimates.push(estimate.estimate);
    }

    let mean = estimates.iter().sum::<f64>() / estimates.len() as f64;
    // sqrt((B-1)/B) * sample_sd is algebraically sqrt(sum((x-mean)^2)/B).
    let standard_error = (estimates
        .iter()
        .map(|estimate| (estimate - mean).powi(2))
        .sum::<f64>()
        / estimates.len() as f64)
        .sqrt();
    Ok(PanelPlaceboInference {
        estimates,
        standard_error,
    })
}

/// Pre-treatment-only placebo diagnostic from `synthdid_placebo`.
pub fn panel_in_time_placebo(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
    estimator: PanelEstimatorKind,
    treated_fraction: Option<f64>,
) -> Result<PanelEstimate, PanelError> {
    validate_boundary(y, n0, t0)?;
    let fraction = treated_fraction.unwrap_or(1.0 - t0 as f64 / y.ncols() as f64);
    if !fraction.is_finite() || !(0.0..1.0).contains(&fraction) {
        return Err(PanelError::InvalidTreatedFraction);
    }
    let placebo_t0 = (t0 as f64 * (1.0 - fraction)).floor() as usize;
    if placebo_t0 == 0 || placebo_t0 >= t0 {
        return Err(PanelError::InvalidTreatedFraction);
    }
    let placebo_y = y.columns(0, t0).into_owned();
    let configuration = fit_configuration(y, n0, t0, estimator)?;
    refit_with_configuration(
        &placebo_y,
        n0,
        placebo_t0,
        estimator,
        configuration,
        None,
        None,
    )
}

/// The `synthdid::sc_estimate` comparator. This differs from the crate's exact
/// active-set synthetic-control solver because the reference uses infinitesimal
/// ridge regularisation and its Frank-Wolfe stopping rule.
pub fn synthdid_sc_estimate(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
) -> Result<PanelEstimate, PanelError> {
    validate_boundary(y, n0, t0)?;
    let noise = sample_noise_level(y, n0, t0)?;
    let min_decrease = 1e-5 * noise;
    let collapsed = collapsed_form(y, n0, t0);
    let omega_design = collapsed.columns(0, t0).transpose();
    let omega = fitted_weights(&omega_design, 1e-6 * noise, false, min_decrease);
    let omega_iterations = omega.values.len();
    Ok(estimate_from_weights(
        y,
        n0,
        t0,
        vec![0.0; t0],
        omega.weights,
        0,
        omega_iterations,
        Vec::new(),
        omega.values,
        noise,
    ))
}

/// Synthetic difference in differences, Algorithm 1 of Arkhangelsky et al.,
/// matching the default no-covariate path of `synthdid::synthdid_estimate`.
pub fn synthetic_did_estimate(
    y: &DMatrix<f64>,
    n0: usize,
    t0: usize,
) -> Result<PanelEstimate, PanelError> {
    validate_boundary(y, n0, t0)?;
    let noise = sample_noise_level(y, n0, t0)?;
    let n1 = y.nrows() - n0;
    let t1 = y.ncols() - t0;
    let min_decrease = 1e-5 * noise;
    let collapsed = collapsed_form(y, n0, t0);

    let lambda_design = collapsed.rows(0, n0).into_owned();
    let lambda = fitted_weights(&lambda_design, 1e-6 * noise, true, min_decrease);

    let omega_design = collapsed.columns(0, t0).transpose();
    let eta_omega = (n1 * t1) as f64;
    let zeta_omega = eta_omega.sqrt().sqrt() * noise;
    let omega = fitted_weights(&omega_design, zeta_omega, true, min_decrease);

    let lambda_iterations = lambda.values.len();
    let omega_iterations = omega.values.len();
    Ok(estimate_from_weights(
        y,
        n0,
        t0,
        lambda.weights,
        omega.weights,
        lambda_iterations,
        omega_iterations,
        lambda.values,
        omega.values,
        noise,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_panel() -> Vec<PanelObservation> {
        let mut rows = Vec::new();
        for (unit, offset, treated) in [("A", 0.0, false), ("B", 2.0, false), ("C", 1.0, true)] {
            for time in 0..4 {
                rows.push(PanelObservation {
                    unit: unit.to_owned(),
                    time,
                    outcome: offset + time as f64 + if treated && time >= 2 { 3.0 } else { 0.0 },
                    treatment: if treated && time >= 2 { 1.0 } else { 0.0 },
                });
            }
        }
        rows.reverse();
        rows
    }

    #[test]
    fn panel_is_sorted_and_did_recovers_the_shift() {
        let panel = panel_matrices(&small_panel()).unwrap();
        assert_eq!(panel.units, ["A", "B", "C"]);
        assert_eq!(panel.times, [0, 1, 2, 3]);
        assert_eq!((panel.n0, panel.t0), (2, 2));
        let estimate = did_estimate(&panel.y, panel.n0, panel.t0).unwrap();
        assert!((estimate.estimate - 3.0).abs() < 1e-12);
        assert!(estimate
            .effect_curve
            .iter()
            .all(|effect| (*effect - 3.0).abs() < 1e-12));
    }

    #[test]
    fn duplicate_and_missing_cells_are_rejected() {
        let rows = small_panel();
        let mut duplicate = rows.clone();
        duplicate.push(rows[0].clone());
        assert!(matches!(
            panel_matrices(&duplicate),
            Err(PanelError::DuplicateCell { .. })
        ));
        assert!(matches!(
            panel_matrices(&rows[1..]),
            Err(PanelError::MissingCell { .. })
        ));
    }

    #[test]
    fn treatment_must_be_simultaneous_and_absorbing() {
        let mut rows = small_panel();
        let row = rows
            .iter_mut()
            .find(|row| row.unit == "C" && row.time == 3)
            .unwrap();
        row.treatment = 0.0;
        assert!(matches!(
            panel_matrices(&rows),
            Err(PanelError::NonSimultaneousAdoption)
        ));
    }
}
