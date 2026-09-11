//! Calculate residuals for fitted right-censored Cox models.

use super::data::{Event, RightCensoredData};
use super::fit::CoxFit;
use super::math::dot;
use crate::survival::nonparametric::{kaplan_meier, EventStatus, WeightedObservation};
use spec_math::cephes64::chdtrc;

#[derive(Clone, Debug, PartialEq)]
pub struct CoxResiduals {
    pub martingale: Vec<f64>,
    pub deviance: Vec<f64>,
    pub schoenfeld: Vec<f64>,
    pub scaled_schoenfeld: Vec<f64>,
    pub score: Vec<f64>,
    pub delta_beta: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResidualError {
    DelayedEntryUnsupported,
    FitDimensionMismatch,
    InvalidTimeTransform,
    KaplanMeierFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimeTransform {
    EventRank,
    KaplanMeier,
    Identity,
    LogTime,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProportionalHazardsTest {
    pub statistic: f64,
    pub p_value: f64,
}

/// Shared preparation for testing several time transformations of one fit.
pub struct ProportionalHazardsDiagnostics {
    rows: SortedRows,
    scaled_schoenfeld: Vec<f64>,
    standard_errors: Vec<f64>,
}

impl ProportionalHazardsDiagnostics {
    pub fn new(data: &RightCensoredData, fit: &CoxFit) -> Result<Self, ResidualError> {
        validate_residual_fit(data, fit)?;
        let rows = sorted_rows(data);
        let coefficients = fit
            .coefficients
            .iter()
            .map(|value| value.coefficient)
            .collect::<Vec<_>>();
        let residuals = schoenfeld(&rows, &coefficients);
        let scaled_schoenfeld = scale_schoenfeld(&rows, fit, &residuals);
        Ok(Self {
            rows,
            scaled_schoenfeld,
            standard_errors: fit
                .coefficients
                .iter()
                .map(|value| value.standard_error)
                .collect(),
        })
    }

    pub fn test(
        &self,
        transform: TimeTransform,
    ) -> Result<Vec<ProportionalHazardsTest>, ResidualError> {
        test_prepared(self, transform)
    }
}

struct SortedRows {
    rows: usize,
    columns: usize,
    durations: Vec<f64>,
    events: Vec<Event>,
    weights: Vec<f64>,
    strata: Option<Vec<usize>>,
    covariates: Vec<f64>,
}

fn sorted_rows(data: &RightCensoredData) -> SortedRows {
    let mut order = (0..data.rows()).collect::<Vec<_>>();
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
    let mut covariates = Vec::with_capacity(data.rows() * data.columns());
    for row in &order {
        covariates.extend_from_slice(data.covariates().row(*row));
    }
    SortedRows {
        rows: data.rows(),
        columns: data.columns(),
        durations: order.iter().map(|row| data.durations()[*row]).collect(),
        events: order.iter().map(|row| data.events()[*row]).collect(),
        weights: order.iter().map(|row| data.weights()[*row]).collect(),
        strata: data
            .strata()
            .map(|strata| order.iter().map(|row| strata[*row]).collect()),
        covariates,
    }
}

fn slice_rows(rows: &SortedRows, start: usize, end: usize) -> SortedRows {
    SortedRows {
        rows: end - start,
        columns: rows.columns,
        durations: rows.durations[start..end].to_vec(),
        events: rows.events[start..end].to_vec(),
        weights: rows.weights[start..end].to_vec(),
        strata: None,
        covariates: rows.covariates[start * rows.columns..end * rows.columns].to_vec(),
    }
}

fn cumulative_baseline_at(fit: &CoxFit, time: f64, stratum: Option<usize>) -> Option<f64> {
    let estimates = match stratum {
        Some(stratum) => {
            &fit.baseline
                .stratified()?
                .iter()
                .find(|curve| curve.stratum == stratum)?
                .estimates
        }
        None => fit.baseline.shared()?,
    };
    estimates
        .iter()
        .find(|estimate| estimate.time == time)
        .map(|estimate| estimate.cumulative_hazard)
}

fn martingale_and_deviance(rows: &SortedRows, fit: &CoxFit) -> (Vec<f64>, Vec<f64>) {
    let coefficients = fit
        .coefficients
        .iter()
        .map(|estimate| estimate.coefficient)
        .collect::<Vec<_>>();
    let mut martingale = Vec::with_capacity(rows.rows);
    let mut deviance = Vec::with_capacity(rows.rows);
    for row in 0..rows.rows {
        let x = &rows.covariates[row * rows.columns..(row + 1) * rows.columns];
        let centered = (0..rows.columns)
            .map(|column| x[column] - fit.covariate_means[column])
            .collect::<Vec<_>>();
        let partial_hazard = dot(&centered, &coefficients).exp();
        let cumulative = cumulative_baseline_at(
            fit,
            rows.durations[row],
            rows.strata.as_ref().map(|strata| strata[row]),
        )
        .expect("the baseline includes every observed duration");
        let observed = f64::from(rows.events[row] == Event::Observed);
        let residual = observed - partial_hazard * cumulative;
        let event_minus_residual = observed - residual;
        let log_term = if event_minus_residual <= 0.0 {
            0.0
        } else {
            observed * event_minus_residual.ln()
        };
        martingale.push(residual);
        deviance.push(residual.signum() * (-2.0 * (residual + log_term)).sqrt());
    }
    (martingale, deviance)
}

fn schoenfeld_group(rows: &SortedRows, coefficients: &[f64]) -> Vec<f64> {
    let d = rows.columns;
    let scores = (0..rows.rows)
        .map(|row| {
            rows.weights[row] * dot(&rows.covariates[row * d..(row + 1) * d], coefficients).exp()
        })
        .collect::<Vec<_>>();
    let mut risk_weight = 0.0;
    let mut tie_weight = 0.0;
    let mut risk_first = vec![0.0; d];
    let mut tie_first = vec![0.0; d];
    let mut reversed = Vec::with_capacity(rows.rows * d);
    let mut group = Vec::new();
    let mut tied_deaths = 0usize;

    for row in (0..rows.rows).rev() {
        let x = &rows.covariates[row * d..(row + 1) * d];
        let score = scores[row];
        risk_weight += score;
        for column in 0..d {
            risk_first[column] += score * x[column];
        }
        group.push(row);
        if rows.events[row] == Event::Observed {
            tie_weight += score;
            for column in 0..d {
                tie_first[column] += score * x[column];
            }
            tied_deaths += 1;
        }
        if row > 0 && rows.durations[row - 1] == rows.durations[row] {
            continue;
        }
        if tied_deaths == 0 {
            reversed.extend(std::iter::repeat_n(0.0, group.len() * d));
        } else {
            let mut weighted_mean = vec![0.0; d];
            for tied in 0..tied_deaths {
                let fraction = tied as f64 / tied_deaths as f64;
                let denominator = risk_weight - fraction * tie_weight;
                for column in 0..d {
                    weighted_mean[column] += (risk_first[column] - fraction * tie_first[column])
                        / (denominator * tied_deaths as f64);
                }
            }
            for group_row in &group {
                let x = &rows.covariates[*group_row * d..(*group_row + 1) * d];
                let observed = f64::from(rows.events[*group_row] == Event::Observed);
                for column in 0..d {
                    reversed.push(observed * (x[column] - weighted_mean[column]));
                }
            }
        }
        group.clear();
        tie_weight = 0.0;
        tie_first.fill(0.0);
        tied_deaths = 0;
    }

    let mut all_rows = reversed
        .chunks_exact(d)
        .rev()
        .flat_map(|row| row.iter().copied())
        .collect::<Vec<_>>();
    let mut observed_only = Vec::new();
    for row in 0..rows.rows {
        if rows.events[row] == Event::Observed {
            observed_only.extend_from_slice(&all_rows[row * d..(row + 1) * d]);
        }
    }
    all_rows.clear();
    observed_only
}

fn schoenfeld(rows: &SortedRows, coefficients: &[f64]) -> Vec<f64> {
    let Some(strata) = &rows.strata else {
        return schoenfeld_group(rows, coefficients);
    };
    let mut residuals = Vec::new();
    let mut start = 0;
    while start < rows.rows {
        let mut end = start + 1;
        while end < rows.rows && strata[end] == strata[start] {
            end += 1;
        }
        residuals.extend(schoenfeld_group(
            &slice_rows(rows, start, end),
            coefficients,
        ));
        start = end;
    }
    residuals
}

fn score_values(
    rows: usize,
    columns: usize,
    covariates: &[f64],
    events: &[Event],
    weights: &[f64],
    beta: &[f64],
) -> Vec<f64> {
    let d = columns;
    let phi = (0..rows)
        .map(|row| dot(&covariates[row * d..(row + 1) * d], beta).exp())
        .collect::<Vec<_>>();
    let mut risk_weight = vec![0.0; rows];
    let mut risk_first = vec![0.0; rows * d];
    let mut running_weight = 0.0;
    let mut running_first = vec![0.0; d];
    for row in (0..rows).rev() {
        let weighted_phi = weights[row] * phi[row];
        running_weight += weighted_phi;
        let x = &covariates[row * d..(row + 1) * d];
        for column in 0..d {
            running_first[column] += x[column] * weighted_phi;
            risk_first[row * d + column] = running_first[column];
        }
        risk_weight[row] = running_weight;
    }

    let mut residuals = vec![0.0; rows * d];
    for row in 0..rows {
        let x = &covariates[row * d..(row + 1) * d];
        for earlier in 0..=row {
            if events[earlier] != Event::Observed {
                continue;
            }
            let scale = weights[earlier] / risk_weight[earlier];
            for column in 0..d {
                residuals[row * d + column] -= phi[row]
                    * scale
                    * (x[column] - risk_first[earlier * d + column] / risk_weight[earlier]);
            }
        }
        if events[row] == Event::Observed {
            for column in 0..d {
                residuals[row * d + column] +=
                    x[column] - risk_first[row * d + column] / risk_weight[row];
            }
        }
        for column in 0..d {
            residuals[row * d + column] *= weights[row];
        }
    }
    residuals
}

fn score_group(rows: &SortedRows, fit: &CoxFit) -> Vec<f64> {
    let d = rows.columns;
    let normalized_coefficients = fit
        .coefficients
        .iter()
        .enumerate()
        .map(|(column, estimate)| estimate.coefficient * fit.covariate_standard_deviations[column])
        .collect::<Vec<_>>();
    score_values(
        rows.rows,
        d,
        &rows.covariates,
        &rows.events,
        &rows.weights,
        &normalized_coefficients,
    )
}

fn score(rows: &SortedRows, fit: &CoxFit) -> Vec<f64> {
    let Some(strata) = &rows.strata else {
        return score_group(rows, fit);
    };
    let mut residuals = Vec::with_capacity(rows.rows * rows.columns);
    let mut start = 0;
    while start < rows.rows {
        let mut end = start + 1;
        while end < rows.rows && strata[end] == strata[start] {
            end += 1;
        }
        residuals.extend(score_group(&slice_rows(rows, start, end), fit));
        start = end;
    }
    residuals
}

pub(crate) fn sandwich_standard_errors(
    normalized: &[f64],
    events: &[Event],
    weights: &[f64],
    covariance: &[f64],
    deviations: &[f64],
    strata: Option<&[usize]>,
    clusters: Option<&[usize]>,
    rows: usize,
    columns: usize,
    beta: &[f64],
) -> Vec<f64> {
    let score = if let Some(strata) = strata {
        let mut values = vec![0.0; rows * columns];
        let mut start = 0;
        while start < rows {
            let mut end = start + 1;
            while end < rows && strata[end] == strata[start] {
                end += 1;
            }
            values[start * columns..end * columns].copy_from_slice(&score_values(
                end - start,
                columns,
                &normalized[start * columns..end * columns],
                &events[start..end],
                &weights[start..end],
                beta,
            ));
            start = end;
        }
        values
    } else {
        score_values(rows, columns, normalized, events, weights, beta)
    };
    let mut changes = vec![0.0; rows * columns];
    for row in 0..rows {
        for column in 0..columns {
            changes[row * columns + column] = (0..columns)
                .map(|inner| {
                    score[row * columns + inner]
                        * covariance[inner * columns + column]
                        * deviations[inner]
                })
                .sum();
        }
    }
    let grouped = if let Some(cluster_ids) = clusters {
        let mut groups = cluster_ids.to_vec();
        groups.sort_unstable();
        groups.dedup();
        groups
            .into_iter()
            .flat_map(|group| {
                (0..columns)
                    .map(|column| {
                        (0..rows)
                            .filter(|row| cluster_ids[*row] == group)
                            .map(|row| changes[row * columns + column])
                            .sum::<f64>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    } else {
        changes
    };
    let group_count = grouped.len() / columns;
    (0..columns)
        .map(|column| {
            (0..group_count)
                .map(|row| grouped[row * columns + column].powi(2))
                .sum::<f64>()
                .sqrt()
        })
        .collect()
}

fn validate_residual_fit(data: &RightCensoredData, fit: &CoxFit) -> Result<(), ResidualError> {
    if data.entries().is_some() {
        return Err(ResidualError::DelayedEntryUnsupported);
    }
    let d = data.columns();
    if fit.coefficients.len() != d
        || fit.covariance.len() != d * d
        || fit.covariate_means.len() != d
        || fit.covariate_standard_deviations.len() != d
    {
        return Err(ResidualError::FitDimensionMismatch);
    }
    Ok(())
}

pub fn compute_right_censored_residuals(
    data: &RightCensoredData,
    fit: &CoxFit,
) -> Result<CoxResiduals, ResidualError> {
    validate_residual_fit(data, fit)?;
    let d = data.columns();
    let rows = sorted_rows(data);
    let coefficients = fit
        .coefficients
        .iter()
        .map(|estimate| estimate.coefficient)
        .collect::<Vec<_>>();
    let (martingale, deviance) = martingale_and_deviance(&rows, fit);
    let schoenfeld = schoenfeld(&rows, &coefficients);
    let scaled_schoenfeld = scale_schoenfeld(&rows, fit, &schoenfeld);
    let score = score(&rows, fit);
    let mut delta_beta = vec![0.0; score.len()];
    for row in 0..rows.rows {
        for column in 0..d {
            delta_beta[row * d + column] = (0..d)
                .map(|inner| {
                    score[row * d + inner]
                        * fit.covariance[inner * d + column]
                        * fit.covariate_standard_deviations[inner]
                })
                .sum::<f64>();
        }
    }
    Ok(CoxResiduals {
        martingale,
        deviance,
        schoenfeld,
        scaled_schoenfeld,
        score,
        delta_beta,
    })
}

fn scale_schoenfeld(rows: &SortedRows, fit: &CoxFit, residuals: &[f64]) -> Vec<f64> {
    let d = rows.columns;
    let deaths = rows
        .events
        .iter()
        .filter(|event| **event == Event::Observed)
        .count() as f64;
    let mut scaled = vec![0.0; residuals.len()];
    for row in 0..residuals.len() / d {
        for column in 0..d {
            scaled[row * d + column] = deaths
                * (0..d)
                    .map(|inner| residuals[row * d + inner] * fit.covariance[inner * d + column])
                    .sum::<f64>();
        }
    }
    scaled
}

pub fn test_proportional_hazards(
    data: &RightCensoredData,
    fit: &CoxFit,
    transform: TimeTransform,
) -> Result<Vec<ProportionalHazardsTest>, ResidualError> {
    ProportionalHazardsDiagnostics::new(data, fit)?.test(transform)
}

fn test_prepared(
    diagnostics: &ProportionalHazardsDiagnostics,
    transform: TimeTransform,
) -> Result<Vec<ProportionalHazardsTest>, ResidualError> {
    let rows = &diagnostics.rows;
    let mut transformed = match transform {
        TimeTransform::EventRank => {
            let mut rank = 0.0;
            rows.events
                .iter()
                .map(|event| {
                    if *event == Event::Observed {
                        rank += 1.0;
                    }
                    rank
                })
                .collect::<Vec<_>>()
        }
        TimeTransform::Identity => rows.durations.clone(),
        TimeTransform::LogTime => {
            if rows.durations.iter().any(|time| *time <= 0.0) {
                return Err(ResidualError::InvalidTimeTransform);
            }
            rows.durations.iter().map(|time| time.ln()).collect()
        }
        TimeTransform::KaplanMeier => {
            let observations = (0..rows.rows)
                .map(|row| {
                    WeightedObservation::new(
                        rows.durations[row],
                        if rows.events[row] == Event::Observed {
                            EventStatus::Observed
                        } else {
                            EventStatus::Censored
                        },
                        rows.weights[row],
                    )
                    .map_err(|_| ResidualError::KaplanMeierFailed)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let mut timeline = rows.durations.clone();
            timeline.sort_by(f64::total_cmp);
            timeline.dedup_by(|left, right| left.total_cmp(right).is_eq());
            let estimates = kaplan_meier(&observations, &timeline, 0.05)
                .map_err(|_| ResidualError::KaplanMeierFailed)?;
            rows.durations
                .iter()
                .map(|time| {
                    let index = estimates.partition_point(|estimate| estimate.time < *time);
                    let estimate = &estimates[index];
                    1.0 - estimate.survival
                })
                .collect()
        }
    };
    transformed = transformed
        .into_iter()
        .enumerate()
        .filter(|(row, _)| rows.events[*row] == Event::Observed)
        .map(|(_, value)| value)
        .collect();
    let mean = transformed.iter().sum::<f64>() / transformed.len() as f64;
    let centered = transformed
        .iter()
        .map(|value| value - mean)
        .collect::<Vec<_>>();
    let time_sum_squares = centered.iter().map(|value| value * value).sum::<f64>();
    let deaths = transformed.len() as f64;
    Ok((0..rows.columns)
        .map(|column| {
            let association = centered
                .iter()
                .enumerate()
                .map(|(row, time)| {
                    time * diagnostics.scaled_schoenfeld[row * rows.columns + column]
                })
                .sum::<f64>();
            let standard_error = diagnostics.standard_errors[column];
            let statistic = association * association
                / (deaths * standard_error * standard_error * time_sum_squares);
            ProportionalHazardsTest {
                statistic,
                p_value: chdtrc(1.0, statistic),
            }
        })
        .collect())
}
