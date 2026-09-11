//! Fit semi-parametric Cox proportional-hazards models.

use spec_math::cephes64::{chdtrc, ndtri};

use super::baseline::{right_censored_baseline, BaselineCurves};
use super::concordance::{concordance_index, ConcordanceIndex};
use super::data::{CoxCovariates, Event, RightCensoredData};
use super::math::{
    dot, inverse, means_and_standard_deviations, norm, normalize, positive_solve, StepSizer,
};
use super::residuals::sandwich_standard_errors;
use super::time_varying::evaluate_time_varying_arrays;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchMode {
    Automatic,
    Single,
    Batch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StandardErrorMethod {
    ModelBased,
    Sandwich,
    Clustered,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoxPenalty {
    Uniform(f64),
    ByCoefficient(Vec<f64>),
}

impl CoxPenalty {
    fn value(&self, column: usize) -> f64 {
        match self {
            Self::Uniform(value) => *value,
            Self::ByCoefficient(values) => values[column],
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Self::Uniform(value) => *value == 0.0,
            Self::ByCoefficient(values) => values.iter().all(|value| *value == 0.0),
        }
    }

    pub(crate) fn is_valid(&self, columns: usize) -> bool {
        match self {
            Self::Uniform(value) => value.is_finite() && *value >= 0.0,
            Self::ByCoefficient(values) => {
                values.len() == columns
                    && values
                        .iter()
                        .all(|value| value.is_finite() && *value >= 0.0)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoxFitOptions {
    pub step_size: f64,
    pub precision: f64,
    pub relative_precision: f64,
    pub maximum_steps: usize,
    pub penalizer: CoxPenalty,
    pub l1_ratio: f64,
    pub batch_mode: BatchMode,
    pub initial_point: Option<Vec<f64>>,
    pub alpha: f64,
    pub standard_errors: StandardErrorMethod,
}

impl Default for CoxFitOptions {
    fn default() -> Self {
        Self {
            step_size: 0.95,
            precision: 1e-7,
            relative_precision: 1e-9,
            maximum_steps: 500,
            penalizer: CoxPenalty::Uniform(0.0),
            l1_ratio: 0.0,
            batch_mode: BatchMode::Automatic,
            initial_point: None,
            alpha: 0.05,
            standard_errors: StandardErrorMethod::ModelBased,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoxFitError {
    InvalidConfiguration,
    InitialPointLength,
    SingularInformation,
    NonFiniteIteration,
    DidNotConverge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EfronEvaluation {
    pub hessian: Vec<f64>,
    pub gradient: Vec<f64>,
    pub log_likelihood: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoefficientEstimate {
    pub coefficient: f64,
    pub hazard_ratio: f64,
    pub standard_error: f64,
    pub lower: f64,
    pub upper: f64,
    pub z: f64,
    pub p_value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoxFit {
    pub coefficients: Vec<CoefficientEstimate>,
    pub covariance: Vec<f64>,
    pub log_likelihood: f64,
    pub null_log_likelihood: f64,
    pub likelihood_ratio: f64,
    pub likelihood_ratio_p_value: f64,
    pub partial_aic: f64,
    pub iterations: usize,
    pub covariate_means: Vec<f64>,
    pub covariate_standard_deviations: Vec<f64>,
    pub log_partial_hazards: Vec<f64>,
    pub baseline: BaselineCurves,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RightCensoredFit {
    pub model: CoxFit,
    pub concordance_index: Option<ConcordanceIndex>,
}

#[derive(Clone)]
pub(crate) struct PreparedRightCensored {
    pub rows: usize,
    pub columns: usize,
    pub durations: Vec<f64>,
    pub events: Vec<Event>,
    pub weights: Vec<f64>,
    pub entries: Option<Vec<f64>>,
    pub strata: Option<Vec<usize>>,
    pub clusters: Option<Vec<usize>>,
    pub normalized: Vec<f64>,
    pub means: Vec<f64>,
    pub deviations: Vec<f64>,
}

fn validate_options(options: &CoxFitOptions, columns: usize) -> Result<(), CoxFitError> {
    if !options.step_size.is_finite()
        || options.step_size <= 0.0
        || !options.precision.is_finite()
        || options.precision <= 0.0
        || options.precision > 1.0
        || !options.relative_precision.is_finite()
        || options.relative_precision <= 0.0
        || options.maximum_steps == 0
        || !options.l1_ratio.is_finite()
        || !(0.0..=1.0).contains(&options.l1_ratio)
        || !options.alpha.is_finite()
        || !(0.0..1.0).contains(&options.alpha)
    {
        return Err(CoxFitError::InvalidConfiguration);
    }
    if !options.penalizer.is_valid(columns) {
        return Err(CoxFitError::InvalidConfiguration);
    }
    if let Some(point) = &options.initial_point {
        if point.len() != columns {
            return Err(CoxFitError::InitialPointLength);
        }
        if point.iter().any(|value| !value.is_finite()) {
            return Err(CoxFitError::InvalidConfiguration);
        }
    }
    Ok(())
}

pub(crate) fn prepare_right_censored(data: &RightCensoredData) -> PreparedRightCensored {
    let rows = data.rows();
    let columns = data.columns();
    let (means, deviations) =
        means_and_standard_deviations(data.covariates().values(), rows, columns);
    let original_normalized = normalize(
        data.covariates().values(),
        rows,
        columns,
        &means,
        &deviations,
    );
    let mut order = (0..rows).collect::<Vec<_>>();
    order.sort_by(|left, right| {
        data.strata()
            .map(|strata| strata[*left].cmp(&strata[*right]))
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                data.durations()[*left]
                    .total_cmp(&data.durations()[*right])
                    .then_with(|| data.events()[*left].cmp(&data.events()[*right]))
            })
    });
    let mut normalized = Vec::with_capacity(rows * columns);
    for row in &order {
        normalized.extend_from_slice(&original_normalized[*row * columns..(*row + 1) * columns]);
    }
    PreparedRightCensored {
        rows,
        columns,
        durations: order.iter().map(|row| data.durations()[*row]).collect(),
        events: order.iter().map(|row| data.events()[*row]).collect(),
        weights: order.iter().map(|row| data.weights()[*row]).collect(),
        entries: data
            .entries()
            .map(|entries| order.iter().map(|row| entries[*row]).collect()),
        strata: data
            .strata()
            .map(|strata| order.iter().map(|row| strata[*row]).collect()),
        clusters: data
            .clusters()
            .map(|clusters| order.iter().map(|row| clusters[*row]).collect()),
        normalized,
        means,
        deviations,
    }
}

fn slice_rows(prepared: &PreparedRightCensored, start: usize, end: usize) -> PreparedRightCensored {
    let columns = prepared.columns;
    PreparedRightCensored {
        rows: end - start,
        columns,
        durations: prepared.durations[start..end].to_vec(),
        events: prepared.events[start..end].to_vec(),
        weights: prepared.weights[start..end].to_vec(),
        entries: prepared
            .entries
            .as_ref()
            .map(|entries| entries[start..end].to_vec()),
        strata: None,
        clusters: prepared
            .clusters
            .as_ref()
            .map(|clusters| clusters[start..end].to_vec()),
        normalized: prepared.normalized[start * columns..end * columns].to_vec(),
        means: prepared.means.clone(),
        deviations: prepared.deviations.clone(),
    }
}

fn add_evaluation(total: &mut EfronEvaluation, contribution: EfronEvaluation) {
    for (value, addition) in total.hessian.iter_mut().zip(contribution.hessian) {
        *value += addition;
    }
    for (value, addition) in total.gradient.iter_mut().zip(contribution.gradient) {
        *value += addition;
    }
    total.log_likelihood += contribution.log_likelihood;
}

fn evaluate_unstratified(
    prepared: &PreparedRightCensored,
    beta: &[f64],
    batch: bool,
) -> EfronEvaluation {
    if let Some(entries) = &prepared.entries {
        evaluate_time_varying_arrays(
            &prepared.normalized,
            &prepared.events,
            entries,
            &prepared.durations,
            &prepared.weights,
            beta,
            prepared.columns,
        )
    } else if batch {
        evaluate_batch(prepared, beta)
    } else {
        evaluate_single(prepared, beta)
    }
}

fn evaluate_prepared(
    prepared: &PreparedRightCensored,
    beta: &[f64],
    batch: bool,
) -> EfronEvaluation {
    let Some(strata) = &prepared.strata else {
        return evaluate_unstratified(prepared, beta, batch);
    };
    let mut total = EfronEvaluation {
        hessian: vec![0.0; prepared.columns * prepared.columns],
        gradient: vec![0.0; prepared.columns],
        log_likelihood: 0.0,
    };
    let mut start = 0;
    while start < prepared.rows {
        let mut end = start + 1;
        while end < prepared.rows && strata[end] == strata[start] {
            end += 1;
        }
        add_evaluation(
            &mut total,
            evaluate_unstratified(&slice_rows(prepared, start, end), beta, batch),
        );
        start = end;
    }
    total
}

fn refit_null_log_likelihood(data: &RightCensoredData) -> Result<f64, CoxFitError> {
    let durations = data
        .events()
        .iter()
        .map(|event| f64::from(*event == Event::Observed))
        .collect::<Vec<_>>();
    let events = vec![Event::Observed; data.rows()];
    let covariates = CoxCovariates::new(data.rows(), 1, data.durations().to_vec())
        .map_err(|_| CoxFitError::SingularInformation)?;
    let trivial = RightCensoredData::with_options(
        durations,
        events,
        data.weights().to_vec(),
        None,
        covariates,
    )
    .map_err(|_| CoxFitError::InvalidConfiguration)?;
    Ok(fit_right_censored(&trivial, &CoxFitOptions::default())?
        .model
        .log_likelihood)
}

fn choose_batch(mode: BatchMode, durations: &[f64], columns: usize) -> bool {
    match mode {
        BatchMode::Single => false,
        BatchMode::Batch => true,
        BatchMode::Automatic => {
            let mut unique = durations.to_vec();
            unique.sort_by(f64::total_cmp);
            unique.dedup_by(|left, right| left.total_cmp(right).is_eq());
            let total = durations.len() as f64;
            let log_fraction_duplicates = (unique.len() as f64 / total).ln();
            1.537_271
                + total * 4.771_387e-6
                + log_fraction_duplicates * 2.610_877e-1
                + total * log_fraction_duplicates * -3.830_987e-11
                + log_fraction_duplicates.powi(2) * 1.389_890e-2
                + total.powi(2) * 3.129_870e-14
                + columns as f64 * 3.196_517e-3
                + columns as f64 * total * -7.356_722e-7
                < 1.0
        }
    }
}

fn symmetric_outer_add(target: &mut [f64], values: &[f64], scale: f64) {
    let columns = values.len();
    for row in 0..columns {
        let scaled_row = scale * values[row];
        for column in row..columns {
            target[row * columns + column] += scaled_row * values[column];
        }
    }
}

fn mirror_upper_triangle(matrix: &mut [f64], order: usize) {
    for row in 1..order {
        for column in 0..row {
            matrix[row * order + column] = matrix[column * order + row];
        }
    }
}

fn evaluate_single(prepared: &PreparedRightCensored, beta: &[f64]) -> EfronEvaluation {
    let d = prepared.columns;
    let n = prepared.rows;
    let scores = (0..n)
        .map(|row| {
            prepared.weights[row] * dot(&prepared.normalized[row * d..(row + 1) * d], beta).exp()
        })
        .collect::<Vec<_>>();
    let mut hessian = vec![0.0; d * d];
    let mut gradient = vec![0.0; d];
    let mut log_likelihood = 0.0;
    let mut death_sum = vec![0.0; d];
    let mut risk_weight = 0.0;
    let mut tie_weight = 0.0;
    let mut risk_first = vec![0.0; d];
    let mut tie_first = vec![0.0; d];
    let mut risk_second = vec![0.0; d * d];
    let mut tie_second = vec![0.0; d * d];
    let mut summand = vec![0.0; d];
    let mut death_weights = 0.0;
    let mut tied_deaths = 0usize;

    for row in (0..n).rev() {
        let x = &prepared.normalized[row * d..(row + 1) * d];
        let score = scores[row];
        risk_weight += score;
        for column in 0..d {
            risk_first[column] += score * x[column];
        }
        symmetric_outer_add(&mut risk_second, x, score);
        if prepared.events[row] == Event::Observed {
            for column in 0..d {
                death_sum[column] += prepared.weights[row] * x[column];
                tie_first[column] += score * x[column];
            }
            tie_weight += score;
            symmetric_outer_add(&mut tie_second, x, score);
            tied_deaths += 1;
            death_weights += prepared.weights[row];
        }
        if row > 0 && prepared.durations[row - 1] == prepared.durations[row] {
            continue;
        }
        if tied_deaths == 0 {
            continue;
        }
        let average_weight = death_weights / tied_deaths as f64;
        for column in 0..d {
            gradient[column] += death_sum[column];
        }
        log_likelihood += dot(&death_sum, beta);
        for tied in 0..tied_deaths {
            let fraction = tied as f64 / tied_deaths as f64;
            let denominator = risk_weight - fraction * tie_weight;
            let reciprocal = 1.0 / denominator;
            for column in 0..d {
                summand[column] = (risk_first[column] - fraction * tie_first[column]) * reciprocal;
                gradient[column] -= average_weight * summand[column];
            }
            log_likelihood += average_weight * reciprocal.ln();
            for left in 0..d {
                for right in left..d {
                    let first = (risk_second[left * d + right]
                        - fraction * tie_second[left * d + right])
                        * reciprocal;
                    hessian[left * d + right] +=
                        average_weight * (summand[left] * summand[right] - first);
                }
            }
        }
        death_sum.fill(0.0);
        tie_weight = 0.0;
        tie_first.fill(0.0);
        tie_second.fill(0.0);
        death_weights = 0.0;
        tied_deaths = 0;
    }
    mirror_upper_triangle(&mut hessian, d);
    EfronEvaluation {
        hessian,
        gradient,
        log_likelihood,
    }
}

fn evaluate_batch(prepared: &PreparedRightCensored, beta: &[f64]) -> EfronEvaluation {
    let d = prepared.columns;
    let n = prepared.rows;
    let scores = (0..n)
        .map(|row| {
            prepared.weights[row] * dot(&prepared.normalized[row * d..(row + 1) * d], beta).exp()
        })
        .collect::<Vec<_>>();
    let mut hessian = vec![0.0; d * d];
    let mut gradient = vec![0.0; d];
    let mut log_likelihood = 0.0;
    let mut risk_weight = 0.0;
    let mut risk_first = vec![0.0; d];
    let mut risk_second = vec![0.0; d * d];
    let mut death_sum = vec![0.0; d];
    let mut tie_first = vec![0.0; d];
    let mut tie_second = vec![0.0; d * d];
    let mut end = n;

    while end > 0 {
        let duration = prepared.durations[end - 1];
        let mut start = end - 1;
        while start > 0 && prepared.durations[start - 1] == duration {
            start -= 1;
        }

        for row in start..end {
            let x = &prepared.normalized[row * d..(row + 1) * d];
            let score = scores[row];
            risk_weight += score;
            for column in 0..d {
                risk_first[column] += score * x[column];
            }
            symmetric_outer_add(&mut risk_second, x, score);
        }

        let death_start = (start..end)
            .find(|row| prepared.events[*row] == Event::Observed)
            .unwrap_or(end);
        let tied_deaths = end - death_start;
        if tied_deaths == 0 {
            end = start;
            continue;
        }

        let mut tie_weight = 0.0;
        let mut death_weights = 0.0;
        for row in death_start..end {
            let x = &prepared.normalized[row * d..(row + 1) * d];
            let score = scores[row];
            death_weights += prepared.weights[row];
            for column in 0..d {
                death_sum[column] += prepared.weights[row] * x[column];
            }
            if tied_deaths > 1 {
                tie_weight += score;
                for column in 0..d {
                    tie_first[column] += score * x[column];
                }
                symmetric_outer_add(&mut tie_second, x, score);
            }
        }

        let average_weight = death_weights / tied_deaths as f64;
        for column in 0..d {
            gradient[column] += death_sum[column];
        }
        log_likelihood += dot(&death_sum, beta);
        let mut reciprocal_sum = 0.0;
        let mut fraction_reciprocal_sum = 0.0;
        let mut reciprocal_squared_sum = 0.0;
        let mut fraction_reciprocal_squared_sum = 0.0;
        let mut fraction_squared_reciprocal_squared_sum = 0.0;
        for tied in 0..tied_deaths {
            let fraction = tied as f64 / tied_deaths as f64;
            let reciprocal = 1.0 / (risk_weight - fraction * tie_weight);
            log_likelihood += average_weight * reciprocal.ln();
            let reciprocal_squared = reciprocal * reciprocal;
            reciprocal_sum += reciprocal;
            fraction_reciprocal_sum += fraction * reciprocal;
            reciprocal_squared_sum += reciprocal_squared;
            fraction_reciprocal_squared_sum += fraction * reciprocal_squared;
            fraction_squared_reciprocal_squared_sum += fraction * fraction * reciprocal_squared;
        }
        for column in 0..d {
            let summed =
                risk_first[column] * reciprocal_sum - tie_first[column] * fraction_reciprocal_sum;
            gradient[column] -= average_weight * summed;
        }
        for left in 0..d {
            for right in left..d {
                let index = left * d + right;
                let summed_outer = risk_first[left] * risk_first[right] * reciprocal_squared_sum
                    - (risk_first[left] * tie_first[right] + tie_first[left] * risk_first[right])
                        * fraction_reciprocal_squared_sum
                    + tie_first[left] * tie_first[right] * fraction_squared_reciprocal_squared_sum;
                let summed_first = risk_second[index] * reciprocal_sum
                    - tie_second[index] * fraction_reciprocal_sum;
                hessian[index] += average_weight * (summed_outer - summed_first);
            }
        }
        death_sum.fill(0.0);
        tie_first.fill(0.0);
        tie_second.fill(0.0);
        end = start;
    }

    mirror_upper_triangle(&mut hessian, d);

    EfronEvaluation {
        hessian,
        gradient,
        log_likelihood,
    }
}

pub fn evaluate_right_censored(
    data: &RightCensoredData,
    beta: &[f64],
    mode: BatchMode,
) -> Result<EfronEvaluation, CoxFitError> {
    if beta.len() != data.columns() || beta.iter().any(|value| !value.is_finite()) {
        return Err(CoxFitError::InitialPointLength);
    }
    let prepared = prepare_right_censored(data);
    let batch = choose_batch(mode, &prepared.durations, prepared.columns);
    Ok(evaluate_prepared(&prepared, beta, batch))
}

pub(crate) fn apply_penalty(
    evaluation: &mut EfronEvaluation,
    beta: &[f64],
    rows: usize,
    penalizer: &CoxPenalty,
    l1_ratio: f64,
    smoothing: f64,
) {
    if penalizer.is_zero() {
        return;
    }
    for (column, value) in beta.iter().copied().enumerate() {
        let penalty = penalizer.value(column);
        let half = smoothing * value / 2.0;
        let tanh = half.tanh();
        let scaled = smoothing * value;
        let soft_absolute = (scaled.max(0.0)
            + (-scaled.abs()).exp().ln_1p()
            + (-scaled).max(0.0)
            + (-scaled.abs()).exp().ln_1p())
            / smoothing;
        evaluation.log_likelihood -= rows as f64
            * penalty
            * (l1_ratio * soft_absolute + 0.5 * (1.0 - l1_ratio) * value * value);
        evaluation.gradient[column] -=
            rows as f64 * penalty * (l1_ratio * tanh + (1.0 - l1_ratio) * value);
        evaluation.hessian[column * beta.len() + column] -= rows as f64
            * penalty
            * (l1_ratio * smoothing * 0.5 * (1.0 - tanh * tanh) + (1.0 - l1_ratio));
    }
}

pub fn fit_right_censored(
    data: &RightCensoredData,
    options: &CoxFitOptions,
) -> Result<RightCensoredFit, CoxFitError> {
    validate_options(options, data.columns())?;
    if options.standard_errors == StandardErrorMethod::Clustered && data.clusters().is_none() {
        return Err(CoxFitError::InvalidConfiguration);
    }
    let prepared = prepare_right_censored(data);
    if prepared
        .deviations
        .iter()
        .any(|value| !value.is_finite() || *value == 0.0)
    {
        return Err(CoxFitError::SingularInformation);
    }
    let d = prepared.columns;
    let batch = choose_batch(options.batch_mode, &prepared.durations, d);
    let mut beta = options
        .initial_point
        .clone()
        .unwrap_or_else(|| vec![0.0; d]);
    let mut delta = vec![0.0; d];
    let mut previous_log_likelihood = 0.0;
    let mut null_log_likelihood = None;
    let mut step_sizer = StepSizer::new(options.step_size);
    let mut final_evaluation = None;
    let mut iterations = 0;
    let mut converged = false;

    while iterations < options.maximum_steps {
        let step = step_sizer.current();
        for column in 0..d {
            beta[column] += step * delta[column];
        }
        iterations += 1;
        let mut evaluation = evaluate_prepared(&prepared, &beta, batch);
        if iterations == 1 && beta.iter().all(|value| *value == 0.0) {
            null_log_likelihood = Some(evaluation.log_likelihood);
        }
        apply_penalty(
            &mut evaluation,
            &beta,
            prepared.rows,
            &options.penalizer,
            options.l1_ratio,
            1.3_f64.powi(iterations as i32),
        );
        let positive_information = evaluation
            .hessian
            .iter()
            .map(|value| -*value)
            .collect::<Vec<_>>();
        delta = positive_solve(&positive_information, &evaluation.gradient)?;
        if delta.iter().any(|value| !value.is_finite()) {
            return Err(CoxFitError::NonFiniteIteration);
        }
        let delta_norm = norm(&delta);
        let decrement = dot(&evaluation.gradient, &delta) / 2.0;
        let relative_change = if previous_log_likelihood == 0.0 {
            f64::INFINITY
        } else {
            (evaluation.log_likelihood - previous_log_likelihood).abs() / -previous_log_likelihood
        };
        converged = delta_norm < options.precision
            || relative_change < options.relative_precision
            || decrement < options.precision;
        previous_log_likelihood = evaluation.log_likelihood;
        step_sizer.update(delta_norm);
        final_evaluation = Some(evaluation);
        if converged || step_sizer.current() <= 0.00001 {
            break;
        }
    }
    if !converged {
        return Err(CoxFitError::DidNotConverge);
    }
    let evaluation = final_evaluation.expect("at least one configured iteration");
    let inverse_hessian = inverse(&evaluation.hessian, d)?;
    let mut covariance = vec![0.0; d * d];
    for row in 0..d {
        for column in 0..d {
            covariance[row * d + column] = -inverse_hessian[row * d + column]
                / (prepared.deviations[row] * prepared.deviations[column]);
        }
    }
    let coefficients = beta
        .iter()
        .zip(&prepared.deviations)
        .map(|(value, deviation)| value / deviation)
        .collect::<Vec<_>>();
    let standard_errors = match options.standard_errors {
        StandardErrorMethod::ModelBased => (0..d)
            .map(|column| covariance[column * d + column].sqrt())
            .collect::<Vec<_>>(),
        StandardErrorMethod::Sandwich => sandwich_standard_errors(
            &prepared.normalized,
            &prepared.events,
            &prepared.weights,
            &covariance,
            &prepared.deviations,
            prepared.strata.as_deref(),
            None,
            prepared.rows,
            d,
            &beta,
        ),
        StandardErrorMethod::Clustered => sandwich_standard_errors(
            &prepared.normalized,
            &prepared.events,
            &prepared.weights,
            &covariance,
            &prepared.deviations,
            prepared.strata.as_deref(),
            prepared.clusters.as_deref(),
            prepared.rows,
            d,
            &beta,
        ),
    };
    let z_quantile = ndtri(1.0 - options.alpha / 2.0);
    let estimates = coefficients
        .iter()
        .copied()
        .enumerate()
        .map(|(column, coefficient)| {
            let standard_error = standard_errors[column];
            let z = coefficient / standard_error;
            CoefficientEstimate {
                coefficient,
                hazard_ratio: coefficient.exp(),
                standard_error,
                lower: coefficient - z_quantile * standard_error,
                upper: coefficient + z_quantile * standard_error,
                z,
                p_value: chdtrc(1.0, z * z),
            }
        })
        .collect::<Vec<_>>();
    let log_partial_hazards = (0..data.rows())
        .map(|row| {
            (0..d)
                .map(|column| {
                    (data.covariates().row(row)[column] - prepared.means[column])
                        * coefficients[column]
                })
                .sum::<f64>()
        })
        .collect::<Vec<_>>();
    let null_log_likelihood = if let Some(value) = null_log_likelihood {
        value
    } else if options.initial_point.is_some() {
        refit_null_log_likelihood(data)?
    } else {
        evaluate_prepared(&prepared, &vec![0.0; d], batch).log_likelihood
    };
    let likelihood_ratio = 2.0 * (evaluation.log_likelihood - null_log_likelihood);
    let concordance_index = concordance_index(
        data.durations(),
        data.events(),
        &log_partial_hazards,
        data.strata(),
    );
    Ok(RightCensoredFit {
        concordance_index,
        model: CoxFit {
            coefficients: estimates,
            covariance,
            log_likelihood: evaluation.log_likelihood,
            null_log_likelihood,
            likelihood_ratio,
            likelihood_ratio_p_value: chdtrc(d as f64, likelihood_ratio),
            partial_aic: 2.0 * d as f64 - 2.0 * evaluation.log_likelihood,
            iterations,
            covariate_means: prepared.means.clone(),
            covariate_standard_deviations: prepared.deviations,
            log_partial_hazards,
            baseline: right_censored_baseline(data, &prepared.means, &coefficients),
        },
    })
}
