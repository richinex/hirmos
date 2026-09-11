//! Fit Cox models to start-stop observations.

use spec_math::cephes64::{chdtrc, ndtri};

use super::baseline::time_varying_baseline;
use super::data::{Event, TimeVaryingData};
use super::fit::{
    apply_penalty, CoefficientEstimate, CoxFit, CoxFitError, CoxFitOptions, EfronEvaluation,
    StandardErrorMethod,
};
use super::math::{
    dot, inverse, means_and_standard_deviations, norm, normalize, positive_solve, StepSizer,
};

fn add_outer(target: &mut [f64], left: &[f64], right: &[f64], scale: f64) {
    let columns = left.len();
    for row in 0..columns {
        for column in 0..columns {
            target[row * columns + column] += scale * left[row] * right[column];
        }
    }
}

pub(crate) fn evaluate_time_varying_arrays(
    covariates: &[f64],
    events: &[Event],
    starts: &[f64],
    stops: &[f64],
    weights: &[f64],
    beta: &[f64],
    columns: usize,
) -> EfronEvaluation {
    let rows = events.len();
    let mut event_times = (0..rows)
        .filter(|row| events[*row] == Event::Observed)
        .map(|row| stops[row])
        .collect::<Vec<_>>();
    event_times.sort_by(f64::total_cmp);
    event_times.dedup_by(|left, right| left.total_cmp(right).is_eq());
    let mut hessian = vec![0.0; columns * columns];
    let mut gradient = vec![0.0; columns];
    let mut log_likelihood = 0.0;

    for time in event_times {
        let risk_rows = (0..rows)
            .filter(|row| starts[*row] < time && time <= stops[*row])
            .collect::<Vec<_>>();
        let death_rows = risk_rows
            .iter()
            .copied()
            .filter(|row| events[*row] == Event::Observed && stops[*row] == time)
            .collect::<Vec<_>>();
        let tied_deaths = death_rows.len();
        let mut risk_weight = 0.0;
        let mut risk_first = vec![0.0; columns];
        let mut risk_second = vec![0.0; columns * columns];
        let mut tie_weight = 0.0;
        let mut tie_first = vec![0.0; columns];
        let mut tie_second = vec![0.0; columns * columns];
        let mut death_sum = vec![0.0; columns];
        let mut death_weight = 0.0;
        for row in risk_rows {
            let x = &covariates[row * columns..(row + 1) * columns];
            let score = weights[row] * dot(x, beta).exp();
            risk_weight += score;
            for column in 0..columns {
                risk_first[column] += score * x[column];
            }
            add_outer(&mut risk_second, x, x, score);
            if events[row] == Event::Observed && stops[row] == time {
                tie_weight += score;
                death_weight += weights[row];
                for column in 0..columns {
                    tie_first[column] += score * x[column];
                    death_sum[column] += weights[row] * x[column];
                }
                add_outer(&mut tie_second, x, x, score);
            }
        }
        let average_weight = death_weight / tied_deaths as f64;
        for column in 0..columns {
            gradient[column] += death_sum[column];
        }
        log_likelihood += dot(&death_sum, beta);
        for tied in 0..tied_deaths {
            let fraction = tied as f64 / tied_deaths as f64;
            let reciprocal = 1.0 / (risk_weight - fraction * tie_weight);
            let mut summand = vec![0.0; columns];
            for column in 0..columns {
                summand[column] = (risk_first[column] - fraction * tie_first[column]) * reciprocal;
                gradient[column] -= average_weight * summand[column];
            }
            log_likelihood += average_weight * reciprocal.ln();
            for left in 0..columns {
                for right in 0..columns {
                    let first = (risk_second[left * columns + right]
                        - fraction * tie_second[left * columns + right])
                        * reciprocal;
                    hessian[left * columns + right] +=
                        average_weight * (summand[left] * summand[right] - first);
                }
            }
        }
    }
    EfronEvaluation {
        hessian,
        gradient,
        log_likelihood,
    }
}

fn evaluate_strata(
    covariates: &[f64],
    events: &[Event],
    starts: &[f64],
    stops: &[f64],
    weights: &[f64],
    strata: Option<&[usize]>,
    beta: &[f64],
    columns: usize,
) -> EfronEvaluation {
    let Some(strata) = strata else {
        return evaluate_time_varying_arrays(
            covariates, events, starts, stops, weights, beta, columns,
        );
    };
    let mut groups = strata.to_vec();
    groups.sort_unstable();
    groups.dedup();
    let mut total = EfronEvaluation {
        hessian: vec![0.0; columns * columns],
        gradient: vec![0.0; columns],
        log_likelihood: 0.0,
    };
    for group in groups {
        let selected = (0..events.len())
            .filter(|row| strata[*row] == group)
            .collect::<Vec<_>>();
        let mut group_covariates = Vec::with_capacity(selected.len() * columns);
        for row in &selected {
            group_covariates.extend_from_slice(&covariates[*row * columns..(*row + 1) * columns]);
        }
        let contribution = evaluate_time_varying_arrays(
            &group_covariates,
            &selected.iter().map(|row| events[*row]).collect::<Vec<_>>(),
            &selected.iter().map(|row| starts[*row]).collect::<Vec<_>>(),
            &selected.iter().map(|row| stops[*row]).collect::<Vec<_>>(),
            &selected.iter().map(|row| weights[*row]).collect::<Vec<_>>(),
            beta,
            columns,
        );
        for (value, addition) in total.hessian.iter_mut().zip(contribution.hessian) {
            *value += addition;
        }
        for (value, addition) in total.gradient.iter_mut().zip(contribution.gradient) {
            *value += addition;
        }
        total.log_likelihood += contribution.log_likelihood;
    }
    total
}

pub fn evaluate_time_varying(
    data: &TimeVaryingData,
    beta: &[f64],
) -> Result<EfronEvaluation, CoxFitError> {
    if beta.len() != data.columns() || beta.iter().any(|value| !value.is_finite()) {
        return Err(CoxFitError::InitialPointLength);
    }
    let (means, deviations) =
        means_and_standard_deviations(data.covariates().values(), data.rows(), data.columns());
    if deviations
        .iter()
        .any(|value| !value.is_finite() || *value == 0.0)
    {
        return Err(CoxFitError::SingularInformation);
    }
    let normalized = normalize(
        data.covariates().values(),
        data.rows(),
        data.columns(),
        &means,
        &deviations,
    );
    Ok(evaluate_strata(
        &normalized,
        data.events(),
        data.starts(),
        data.stops(),
        data.weights(),
        data.strata(),
        beta,
        data.columns(),
    ))
}

fn refit_null_log_likelihood(data: &TimeVaryingData) -> f64 {
    let mut events = Vec::new();
    let mut starts = Vec::new();
    let mut stops = Vec::new();
    let mut weights = Vec::new();
    let mut strata = Vec::new();
    for row in 0..data.rows() {
        let copies = (0..data.rows())
            .filter(|other| {
                data.subjects()[*other] == data.subjects()[row]
                    && data
                        .strata()
                        .map(|values| values[*other] == values[row])
                        .unwrap_or(true)
            })
            .count();
        for _ in 0..copies {
            events.push(data.events()[row]);
            starts.push(data.starts()[row]);
            stops.push(data.stops()[row]);
            weights.push(data.weights()[row]);
            if let Some(values) = data.strata() {
                strata.push(values[row]);
            }
        }
    }
    evaluate_strata(
        &[],
        &events,
        &starts,
        &stops,
        &weights,
        data.strata().map(|_| strata.as_slice()),
        &[],
        0,
    )
    .log_likelihood
}

pub fn fit_time_varying(
    data: &TimeVaryingData,
    options: &CoxFitOptions,
) -> Result<CoxFit, CoxFitError> {
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
        || options.standard_errors != StandardErrorMethod::ModelBased
        || !options.penalizer.is_valid(data.columns())
    {
        return Err(CoxFitError::InvalidConfiguration);
    }
    let rows = data.rows();
    let columns = data.columns();
    let (means, deviations) =
        means_and_standard_deviations(data.covariates().values(), rows, columns);
    if deviations
        .iter()
        .any(|value| !value.is_finite() || *value == 0.0)
    {
        return Err(CoxFitError::SingularInformation);
    }
    let normalized = normalize(
        data.covariates().values(),
        rows,
        columns,
        &means,
        &deviations,
    );
    let mut beta = options
        .initial_point
        .clone()
        .unwrap_or_else(|| vec![0.0; columns]);
    if beta.len() != columns {
        return Err(CoxFitError::InitialPointLength);
    }
    if beta.iter().any(|value| !value.is_finite()) {
        return Err(CoxFitError::InvalidConfiguration);
    }
    let mut previous_log_likelihood = 0.0;
    let mut null_log_likelihood = None;
    let mut step_sizer = StepSizer::new(options.step_size);
    let mut final_evaluation = None;
    let mut iterations = 0;
    let mut converged = false;

    while iterations < options.maximum_steps {
        iterations += 1;
        let mut evaluation = evaluate_strata(
            &normalized,
            data.events(),
            data.starts(),
            data.stops(),
            data.weights(),
            data.strata(),
            &beta,
            columns,
        );
        if iterations == 1 && beta.iter().all(|value| *value == 0.0) {
            null_log_likelihood = Some(evaluation.log_likelihood);
        }
        apply_penalty(
            &mut evaluation,
            &beta,
            rows,
            &options.penalizer,
            options.l1_ratio,
            1.5_f64.powi(iterations as i32),
        );
        let positive_information = evaluation
            .hessian
            .iter()
            .map(|value| -*value)
            .collect::<Vec<_>>();
        let solution = positive_solve(&positive_information, &evaluation.gradient)?;
        let delta = solution
            .iter()
            .map(|value| step_sizer.current() * value)
            .collect::<Vec<_>>();
        if delta.iter().any(|value| !value.is_finite()) {
            return Err(CoxFitError::NonFiniteIteration);
        }
        let delta_norm = norm(&delta);
        let decrement = dot(&evaluation.gradient, &solution) / 2.0;
        let relative_change = if previous_log_likelihood > 0.0 {
            (evaluation.log_likelihood - previous_log_likelihood).abs() / -previous_log_likelihood
        } else {
            f64::INFINITY
        };
        converged = delta_norm < options.precision
            || relative_change < options.relative_precision
            || decrement < options.precision;
        previous_log_likelihood = evaluation.log_likelihood;
        step_sizer.update(delta_norm);
        for column in 0..columns {
            beta[column] += delta[column];
        }
        final_evaluation = Some(evaluation);
        if converged || step_sizer.current() <= 0.0001 {
            break;
        }
    }
    if !converged {
        return Err(CoxFitError::DidNotConverge);
    }
    let evaluation = final_evaluation.expect("at least one configured iteration");
    let inverse_hessian = inverse(&evaluation.hessian, columns)?;
    let mut covariance = vec![0.0; columns * columns];
    for row in 0..columns {
        for column in 0..columns {
            covariance[row * columns + column] =
                -inverse_hessian[row * columns + column] / (deviations[row] * deviations[column]);
        }
    }
    let coefficients = beta
        .iter()
        .zip(&deviations)
        .map(|(value, deviation)| value / deviation)
        .collect::<Vec<_>>();
    let quantile = ndtri(1.0 - options.alpha / 2.0);
    let estimates = coefficients
        .iter()
        .copied()
        .enumerate()
        .map(|(column, coefficient)| {
            let standard_error = covariance[column * columns + column].sqrt();
            let z = coefficient / standard_error;
            CoefficientEstimate {
                coefficient,
                hazard_ratio: coefficient.exp(),
                standard_error,
                lower: coefficient - quantile * standard_error,
                upper: coefficient + quantile * standard_error,
                z,
                p_value: chdtrc(1.0, z * z),
            }
        })
        .collect::<Vec<_>>();
    let log_partial_hazards = (0..rows)
        .map(|row| {
            let centered = (0..columns)
                .map(|column| data.covariates().row(row)[column] - means[column])
                .collect::<Vec<_>>();
            dot(&centered, &coefficients)
        })
        .collect::<Vec<_>>();
    let null_log_likelihood = if let Some(value) = null_log_likelihood {
        value
    } else if options.initial_point.is_some() {
        refit_null_log_likelihood(data)
    } else {
        evaluate_strata(
            &normalized,
            data.events(),
            data.starts(),
            data.stops(),
            data.weights(),
            data.strata(),
            &vec![0.0; columns],
            columns,
        )
        .log_likelihood
    };
    let likelihood_ratio = 2.0 * (evaluation.log_likelihood - null_log_likelihood);
    Ok(CoxFit {
        coefficients: estimates,
        covariance,
        log_likelihood: evaluation.log_likelihood,
        null_log_likelihood,
        likelihood_ratio,
        likelihood_ratio_p_value: chdtrc(columns as f64, likelihood_ratio),
        partial_aic: 2.0 * columns as f64 - 2.0 * evaluation.log_likelihood,
        iterations,
        covariate_means: means.clone(),
        covariate_standard_deviations: deviations,
        log_partial_hazards,
        baseline: time_varying_baseline(data, &means, &coefficients),
    })
}
