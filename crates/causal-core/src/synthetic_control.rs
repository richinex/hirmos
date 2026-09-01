//! The synthetic control weight problem of 134: minimise `||Xw - y||^2` subject to
//! `sum(w) == 1` and `w >= 0`.
//!
//! 134 solves this with cvxpy and SCS, a first order solver whose default tolerance is 1e-4.
//! This is a primal active set method, which returns the exact optimum instead.

use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntheticControlError {
    EmptyDesign,
    DimensionMismatch,
    NonFiniteInput,
    InvalidFoldCount,
    InsufficientFoldData,
    InvalidPrePeriodBoundary,
    InsufficientDonors,
    InvalidAlpha,
}

pub struct SyntheticControl {
    pub weights: Vec<f64>,
    /// `problem.solve()`, which returns the objective value `||Xw - y||^2`.
    pub loss: f64,
    pub iterations: usize,
}

/// Least squares on the KKT system, through the pseudo-inverse so a rank deficient
/// `X'X` block still gives a descent direction.
fn kkt_solve(g: &DMatrix<f64>, rhs: &DVector<f64>) -> DVector<f64> {
    let m = g.nrows();
    let mut kkt = DMatrix::<f64>::zeros(m + 1, m + 1);
    kkt.view_mut((0, 0), (m, m)).copy_from(g);
    for i in 0..m {
        kkt[(i, m)] = 1.0;
        kkt[(m, i)] = 1.0;
    }
    let mut b = DVector::<f64>::zeros(m + 1);
    b.rows_mut(0, m).copy_from(rhs);
    let svd = kkt.svd(true, true);
    let eps = 1e-13 * svd.singular_values.max();
    svd.solve(&b, eps).expect("KKT solve")
}

pub fn fit_synthetic_control(x: &DMatrix<f64>, y: &DVector<f64>) -> SyntheticControl {
    let n = x.ncols();
    // A common rescaling leaves the minimizer unchanged but prevents the KKT
    // equality row (whose entries are one) from being lost beside level-valued
    // outcomes such as cigarette sales. The unscaled California placebo systems
    // differ by five orders of magnitude at this boundary.
    let scale = x
        .iter()
        .chain(y.iter())
        .map(|value| value.abs())
        .fold(1.0, f64::max);
    let scaled_x = x / scale;
    let scaled_y = y / scale;
    let g = scaled_x.transpose() * &scaled_x * 2.0;
    let c = scaled_x.transpose() * &scaled_y * -2.0;

    // The uniform weights are interior, so the working set starts empty.
    let mut w = DVector::from_element(n, 1.0 / n as f64);
    let mut active = vec![false; n];
    let tol = 1e-12;
    let mut iterations = 0;

    for _ in 0..200 * n.max(1) {
        iterations += 1;
        let free: Vec<usize> = (0..n).filter(|&i| !active[i]).collect();
        let grad = &g * &w + &c;

        let gff = DMatrix::from_fn(free.len(), free.len(), |i, j| g[(free[i], free[j])]);
        let rhs = DVector::from_fn(free.len(), |i, _| -grad[free[i]]);
        let sol = kkt_solve(&gff, &rhs);
        let lambda = sol[free.len()];

        let mut p = DVector::<f64>::zeros(n);
        for (i, &idx) in free.iter().enumerate() {
            p[idx] = sol[i];
        }

        if p.amax() <= tol {
            // Multipliers on the bounds. A negative one means that bound is holding the
            // objective up, so it leaves the working set.
            let mut worst = 0.0;
            let mut drop = None;
            for i in 0..n {
                if active[i] {
                    let mu = grad[i] + lambda;
                    if mu < worst {
                        worst = mu;
                        drop = Some(i);
                    }
                }
            }
            match drop {
                Some(i) => active[i] = false,
                None => break,
            }
            continue;
        }

        // Step to the first bound the direction runs into, capped at the full step.
        let mut alpha = 1.0;
        let mut blocking = None;
        for &i in &free {
            if p[i] < -tol {
                let limit = -w[i] / p[i];
                if limit < alpha {
                    alpha = limit;
                    blocking = Some(i);
                }
            }
        }
        w += alpha * p;
        for i in 0..n {
            if w[i] < 0.0 {
                w[i] = 0.0;
            }
        }
        if let Some(i) = blocking {
            active[i] = true;
            w[i] = 0.0;
        }
    }

    let resid = x * &w - y;
    SyntheticControl {
        weights: w.iter().copied().collect(),
        loss: resid.iter().map(|v| v * v).sum(),
        iterations,
    }
}

/// The treatment effect 134 reports: the gap between the treated series and its synthetic
/// counterpart, post-intervention.
pub struct SyntheticEffect {
    pub pre_gap: Vec<f64>,
    pub post_gap: Vec<f64>,
    pub att: f64,
}

#[derive(Debug, Clone)]
pub struct DebiasedSyntheticControlFold {
    pub held_out: Vec<usize>,
    pub weights: Vec<f64>,
    pub bias: f64,
    pub att: f64,
}

/// Cross-fitted, bias-corrected synthetic-control inference used by 134.
///
/// The last `folds * block_size` pre-treatment observations are divided into
/// contiguous blocks. Each block is held out in turn, its mean prediction error
/// estimates bias, and that bias is subtracted from the corresponding post-period
/// effect. The reported uncertainty is the t procedure over those fold estimates.
#[derive(Debug, Clone)]
pub struct DebiasedSyntheticControl {
    pub att: f64,
    pub standard_error: f64,
    pub t_statistic: f64,
    pub degrees_of_freedom: usize,
    pub p_value: f64,
    pub confidence_interval: (f64, f64),
    pub block_size: usize,
    pub folds: Vec<DebiasedSyntheticControlFold>,
}

#[derive(Debug, Clone)]
pub struct SyntheticControlMspeSummary {
    pub weights: Vec<f64>,
    pub gap: Vec<f64>,
    pub pre_mspe: f64,
    pub post_mspe: f64,
    pub mspe_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct DonorPlacebo {
    /// Zero-based column of the donor promoted to the treated position.
    pub donor: usize,
    pub summary: SyntheticControlMspeSummary,
}

#[derive(Debug, Clone)]
pub struct DonorPlaceboInference {
    pub treated: SyntheticControlMspeSummary,
    pub placebos: Vec<DonorPlacebo>,
    pub p_value: f64,
    pub n_valid_placebos: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntheticControlBandMethod {
    Conformal,
    ParametricGaussian,
}

#[derive(Debug, Clone)]
pub struct SyntheticControlPredictionBand {
    pub method: SyntheticControlBandMethod,
    pub alpha: f64,
    pub synthetic: Vec<f64>,
    pub effect: Vec<f64>,
    pub intervals: Vec<(f64, f64)>,
    pub half_width: f64,
    pub pre_mspe: f64,
    pub post_mspe: f64,
    pub mspe_ratio: f64,
}

fn t_cdf(t: f64, degrees_of_freedom: f64) -> f64 {
    if t == 0.0 {
        return 0.5;
    }
    let tail = 0.5
        * spec_math::cephes64::incbet(
            0.5 * degrees_of_freedom,
            0.5,
            degrees_of_freedom / (degrees_of_freedom + t * t),
        );
    if t < 0.0 {
        tail
    } else {
        1.0 - tail
    }
}

fn t_quantile(probability: f64, degrees_of_freedom: f64) -> f64 {
    let (tail, sign) = if probability < 0.5 {
        (probability, -1.0)
    } else {
        (1.0 - probability, 1.0)
    };
    let beta = spec_math::cephes64::incbi(0.5 * degrees_of_freedom, 0.5, 2.0 * tail);
    sign * (degrees_of_freedom * (1.0 - beta) / beta).sqrt()
}

fn validate_effect_matrices(
    y_pre_co: &DMatrix<f64>,
    y_pre_tr: &DVector<f64>,
    y_post_co: &DMatrix<f64>,
    y_post_tr: &DVector<f64>,
) -> Result<(), SyntheticControlError> {
    if y_pre_co.nrows() == 0 || y_pre_co.ncols() == 0 || y_post_co.nrows() == 0 {
        return Err(SyntheticControlError::EmptyDesign);
    }
    if y_pre_co.nrows() != y_pre_tr.len()
        || y_post_co.nrows() != y_post_tr.len()
        || y_pre_co.ncols() != y_post_co.ncols()
    {
        return Err(SyntheticControlError::DimensionMismatch);
    }
    if y_pre_co
        .iter()
        .chain(y_pre_tr.iter())
        .chain(y_post_co.iter())
        .chain(y_post_tr.iter())
        .any(|value| !value.is_finite())
    {
        return Err(SyntheticControlError::NonFiniteInput);
    }
    Ok(())
}

fn mspe_summary(
    matching_controls: &DMatrix<f64>,
    matching_treated: &DVector<f64>,
    outcome_controls: &DMatrix<f64>,
    outcome_treated: &DVector<f64>,
    pre_periods: usize,
) -> SyntheticControlMspeSummary {
    let fit = fit_synthetic_control(matching_controls, matching_treated);
    let weights = DVector::from_column_slice(&fit.weights);
    let synthetic = outcome_controls * weights;
    let gap: Vec<f64> = outcome_treated
        .iter()
        .zip(synthetic.iter())
        .map(|(treated, control)| treated - control)
        .collect();
    let pre_mspe = gap[..pre_periods]
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        / pre_periods as f64;
    let post_mspe = gap[pre_periods..]
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        / (gap.len() - pre_periods) as f64;
    let mspe_ratio = if pre_mspe == 0.0 {
        f64::INFINITY
    } else {
        post_mspe / pre_mspe
    };
    SyntheticControlMspeSummary {
        weights: fit.weights,
        gap,
        pre_mspe,
        post_mspe,
        mspe_ratio,
    }
}

/// In-space donor placebos and the post/pre-MSPE rank test used by Synth.
///
/// The fit may use pre-outcome paths or any supplied predictor design. For each
/// placebo, donor `i` becomes treated and the real treated unit takes donor `i`'s
/// slot in both the matching design and the outcome panel. This preserves the
/// donor-pool size exactly as `Synth::generate_placebos` does.
pub fn donor_placebo_mspe_inference(
    matching_controls: &DMatrix<f64>,
    matching_treated: &DVector<f64>,
    outcome_controls: &DMatrix<f64>,
    outcome_treated: &DVector<f64>,
    pre_periods: usize,
) -> Result<DonorPlaceboInference, SyntheticControlError> {
    if matching_controls.nrows() == 0
        || matching_controls.ncols() == 0
        || outcome_controls.nrows() == 0
    {
        return Err(SyntheticControlError::EmptyDesign);
    }
    if matching_controls.ncols() < 2 {
        return Err(SyntheticControlError::InsufficientDonors);
    }
    if matching_controls.nrows() != matching_treated.len()
        || outcome_controls.nrows() != outcome_treated.len()
        || matching_controls.ncols() != outcome_controls.ncols()
    {
        return Err(SyntheticControlError::DimensionMismatch);
    }
    if pre_periods == 0 || pre_periods >= outcome_controls.nrows() {
        return Err(SyntheticControlError::InvalidPrePeriodBoundary);
    }
    if matching_controls
        .iter()
        .chain(matching_treated.iter())
        .chain(outcome_controls.iter())
        .chain(outcome_treated.iter())
        .any(|value| !value.is_finite())
    {
        return Err(SyntheticControlError::NonFiniteInput);
    }

    let treated = mspe_summary(
        matching_controls,
        matching_treated,
        outcome_controls,
        outcome_treated,
        pre_periods,
    );
    let mut placebos = Vec::with_capacity(matching_controls.ncols());
    for donor in 0..matching_controls.ncols() {
        let mut swapped_matching_controls = matching_controls.clone();
        swapped_matching_controls
            .column_mut(donor)
            .copy_from(matching_treated);
        let swapped_matching_treated = matching_controls.column(donor).into_owned();
        let mut swapped_outcome_controls = outcome_controls.clone();
        swapped_outcome_controls
            .column_mut(donor)
            .copy_from(outcome_treated);
        let swapped_outcome_treated = outcome_controls.column(donor).into_owned();
        placebos.push(DonorPlacebo {
            donor,
            summary: mspe_summary(
                &swapped_matching_controls,
                &swapped_matching_treated,
                &swapped_outcome_controls,
                &swapped_outcome_treated,
                pre_periods,
            ),
        });
    }
    let n_valid_placebos = placebos
        .iter()
        .filter(|placebo| !placebo.summary.mspe_ratio.is_nan())
        .count();
    let extreme = 1 + placebos
        .iter()
        .filter(|placebo| {
            !placebo.summary.mspe_ratio.is_nan() && placebo.summary.mspe_ratio >= treated.mspe_ratio
        })
        .count();
    let p_value = extreme as f64 / (n_valid_placebos + 1) as f64;
    Ok(DonorPlaceboInference {
        treated,
        placebos,
        p_value,
        n_valid_placebos,
    })
}

/// Prediction bands from `Synth::synth_inference` for an already fitted set of
/// donor weights. The conformal method uses the finite-sample `(n + 1)` rank;
/// the parametric method uses the sample SD of pre-period residuals and a normal
/// quantile. Their validity assumptions remain exchangeability and i.i.d.
/// Gaussian residuals, respectively.
pub fn synthetic_control_prediction_band(
    outcome_controls: &DMatrix<f64>,
    outcome_treated: &DVector<f64>,
    weights: &[f64],
    pre_periods: usize,
    alpha: f64,
    method: SyntheticControlBandMethod,
) -> Result<SyntheticControlPredictionBand, SyntheticControlError> {
    if outcome_controls.nrows() == 0 || outcome_controls.ncols() == 0 {
        return Err(SyntheticControlError::EmptyDesign);
    }
    if outcome_controls.nrows() != outcome_treated.len()
        || outcome_controls.ncols() != weights.len()
    {
        return Err(SyntheticControlError::DimensionMismatch);
    }
    if pre_periods < 2 || pre_periods >= outcome_controls.nrows() {
        return Err(SyntheticControlError::InvalidPrePeriodBoundary);
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err(SyntheticControlError::InvalidAlpha);
    }
    if outcome_controls
        .iter()
        .chain(outcome_treated.iter())
        .chain(weights.iter())
        .any(|value| !value.is_finite())
    {
        return Err(SyntheticControlError::NonFiniteInput);
    }
    let synthetic_vector = outcome_controls * DVector::from_column_slice(weights);
    let synthetic: Vec<f64> = synthetic_vector.iter().copied().collect();
    let effect: Vec<f64> = outcome_treated
        .iter()
        .zip(&synthetic)
        .map(|(treated, control)| treated - control)
        .collect();
    let pre_mspe = effect[..pre_periods]
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        / pre_periods as f64;
    let post_mspe = effect[pre_periods..]
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        / (effect.len() - pre_periods) as f64;
    let mspe_ratio = if pre_mspe == 0.0 {
        f64::INFINITY
    } else {
        post_mspe / pre_mspe
    };
    let half_width = match method {
        SyntheticControlBandMethod::Conformal => {
            let mut residuals: Vec<f64> = effect[..pre_periods]
                .iter()
                .map(|value| value.abs())
                .collect();
            residuals.sort_by(f64::total_cmp);
            let rank = ((pre_periods + 1) as f64 * (1.0 - alpha)).ceil() as usize;
            if rank > pre_periods {
                f64::INFINITY
            } else {
                residuals[rank - 1]
            }
        }
        SyntheticControlBandMethod::ParametricGaussian => {
            let mean = effect[..pre_periods].iter().sum::<f64>() / pre_periods as f64;
            let standard_deviation = (effect[..pre_periods]
                .iter()
                .map(|value| (value - mean).powi(2))
                .sum::<f64>()
                / (pre_periods - 1) as f64)
                .sqrt();
            spec_math::cephes64::ndtri(1.0 - alpha / 2.0) * standard_deviation
        }
    };
    let intervals = synthetic
        .iter()
        .map(|center| (center - half_width, center + half_width))
        .collect();
    Ok(SyntheticControlPredictionBand {
        method,
        alpha,
        synthetic,
        effect,
        intervals,
        half_width,
        pre_mspe,
        post_mspe,
        mspe_ratio,
    })
}

/// `debiased_sc` from 134, with the same fold construction, bias correction,
/// Chernozhukov standard-error formula, Student-t p-value, and 95% interval.
pub fn debiased_synthetic_control(
    y_pre_co: &DMatrix<f64>,
    y_pre_tr: &DVector<f64>,
    y_post_co: &DMatrix<f64>,
    y_post_tr: &DVector<f64>,
    fold_count: usize,
) -> Result<DebiasedSyntheticControl, SyntheticControlError> {
    validate_effect_matrices(y_pre_co, y_pre_tr, y_post_co, y_post_tr)?;
    if fold_count < 2 {
        return Err(SyntheticControlError::InvalidFoldCount);
    }

    let pre_periods = y_pre_co.nrows();
    let post_periods = y_post_co.nrows();
    let block_size = (pre_periods / fold_count).min(post_periods);
    if block_size < 2 || pre_periods.saturating_sub(block_size) < 4 {
        return Err(SyntheticControlError::InsufficientFoldData);
    }
    let first_held_out = pre_periods - fold_count * block_size;
    let mut fitted_folds = Vec::with_capacity(fold_count);

    for fold in 0..fold_count {
        let held_start = first_held_out + fold * block_size;
        let held_end = held_start + block_size;
        let held_out: Vec<usize> = (held_start..held_end).collect();
        let training: Vec<usize> = (0..pre_periods)
            .filter(|row| *row < held_start || *row >= held_end)
            .collect();
        if training.len() < 4 {
            continue;
        }
        let design = DMatrix::from_fn(training.len(), y_pre_co.ncols(), |row, column| {
            y_pre_co[(training[row], column)]
        });
        let target = DVector::from_fn(training.len(), |row, _| y_pre_tr[training[row]]);
        let fit = fit_synthetic_control(&design, &target);
        let weights = DVector::from_column_slice(&fit.weights);

        let bias = held_out
            .iter()
            .map(|&row| {
                let prediction = (0..weights.len())
                    .map(|column| y_pre_co[(row, column)] * weights[column])
                    .sum::<f64>();
                y_pre_tr[row] - prediction
            })
            .sum::<f64>()
            / block_size as f64;
        let post_gap = (0..post_periods)
            .map(|row| {
                let prediction = (0..weights.len())
                    .map(|column| y_post_co[(row, column)] * weights[column])
                    .sum::<f64>();
                y_post_tr[row] - prediction
            })
            .sum::<f64>()
            / post_periods as f64;
        fitted_folds.push(DebiasedSyntheticControlFold {
            held_out,
            weights: fit.weights,
            bias,
            att: post_gap - bias,
        });
    }
    if fitted_folds.len() < 2 {
        return Err(SyntheticControlError::InsufficientFoldData);
    }

    let att = fitted_folds.iter().map(|fold| fold.att).sum::<f64>() / fitted_folds.len() as f64;
    let variance = fitted_folds
        .iter()
        .map(|fold| (fold.att - att).powi(2))
        .sum::<f64>()
        / (fitted_folds.len() - 1) as f64;
    // 134 deliberately uses the requested K in this formula and in the degrees
    // of freedom. With the exact solver every valid fold is retained.
    let standard_error = (1.0 + (fold_count * block_size) as f64 / post_periods as f64).sqrt()
        * variance.sqrt()
        / (fold_count as f64).sqrt();
    let t_statistic = if standard_error > 1e-10 {
        att / standard_error
    } else {
        0.0
    };
    let degrees_of_freedom = fold_count - 1;
    let p_value = 2.0 * t_cdf(-t_statistic.abs(), degrees_of_freedom as f64);
    let critical = t_quantile(0.975, degrees_of_freedom as f64);

    Ok(DebiasedSyntheticControl {
        att,
        standard_error,
        t_statistic,
        degrees_of_freedom,
        p_value,
        confidence_interval: (
            att - critical * standard_error,
            att + critical * standard_error,
        ),
        block_size,
        folds: fitted_folds,
    })
}

pub fn synthetic_effect(
    y_pre_co: &DMatrix<f64>,
    y_pre_tr: &DVector<f64>,
    y_post_co: &DMatrix<f64>,
    y_post_tr: &DVector<f64>,
) -> (SyntheticControl, SyntheticEffect) {
    let sc = fit_synthetic_control(y_pre_co, y_pre_tr);
    let w = DVector::from_row_slice(&sc.weights);
    let pre = y_pre_tr - y_pre_co * &w;
    let post = y_post_tr - y_post_co * &w;
    let att = post.iter().sum::<f64>() / post.len() as f64;
    let effect = SyntheticEffect {
        pre_gap: pre.iter().copied().collect(),
        post_gap: post.iter().copied().collect(),
        att,
    };
    (sc, effect)
}
