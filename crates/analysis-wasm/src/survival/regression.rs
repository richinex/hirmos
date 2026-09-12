//! Aalen coefficient estimates and ranger predictions keep their distinct statistical meanings.

use super::*;
use hirmos_causal_core::survival::{aalen, forest};

fn regression_rows(values: &[f64], rows: usize, columns: usize, duration: usize, event: usize, covariates: &[usize]) -> Result<Vec<Vec<f64>>, String> {
    if rows < 2 || covariates.is_empty() || values.len() != rows.saturating_mul(columns) {
        return Err("Choose at least one covariate and provide at least two complete observations.".into());
    }
    let mut roles = vec![duration, event];
    roles.extend_from_slice(covariates);
    require_distinct_cox_roles(columns, &roles)?;
    if values.iter().any(|v| !v.is_finite()) {
        return Err("The selected columns contain missing or non-finite values. Resolve them in data preparation before fitting.".into());
    }
    Ok((0..rows).map(|i| covariates.iter().map(|&j| values[j * rows + i]).collect()).collect())
}

fn aalen_problem(error: aalen::Error) -> String {
    match error {
        aalen::Error::TooFewEvents => "Too few events remain with enough observations at risk to fit the selected covariates.".into(),
        aalen::Error::SingularRiskSet => "The selected covariates cannot be estimated separately in an event-time risk set. Check for redundant columns or choose fewer covariates.".into(),
        aalen::Error::InvalidInterval => "Each duration must be above zero and the event flag must be 0 or 1.".into(),
        _ => "The Aalen input or settings are invalid. Check the selected numeric columns.".into(),
    }
}

pub(crate) fn aalen_evidence(values: &[f64], rows: usize, columns: usize, duration: usize, event: usize, covariates: &[usize]) -> Result<AnalysisResult, String> {
    let x = regression_rows(values, rows, columns, duration, event, covariates)?;
    let times = column(values, rows, columns, duration)?;
    let status = column(values, rows, columns, event)?;
    for row in 0..rows { event_indicator(&status, row)?; }
    let events = status.iter().filter(|&&v| v == 1.0).count();
    let intervals = times.iter().zip(&status).map(|(&t, &e)| [0.0, t, e]).collect();
    let data = aalen::Data::new(x, intervals, vec![1.0; rows]).map_err(aalen_problem)?;
    let fit = aalen::fit(&data, &aalen::Options::default()).map_err(aalen_problem)?;
    let summary = aalen::summarize(&fit).map_err(aalen_problem)?;
    let coefficients = match summary.table {
        aalen::SummaryTable::Model(table) => table,
        aalen::SummaryTable::Robust(_) => return Err("Unexpected robust summary for a model-based Aalen fit.".into()),
    };
    let curves = aalen::coefficient_curves(&fit).iter().map(|curve| curve.iter().map(|p| [p.time, p.estimate, p.lower, p.upper]).collect()).collect();
    Ok(AnalysisResult::Aalen {
        observations: rows, events, fitted_events: fit.times.len(), last_time: *fit.times.last().ok_or("No fitted event times.")?,
        coefficients, curves, chisq: summary.chisq, degrees_of_freedom: summary.degrees_of_freedom, p_value: summary.p_value,
    })
}

fn forest_problem(error: forest::Error) -> String {
    match error {
        forest::Error::InvalidCategory => "A categorical covariate must use whole-number level codes from 1 to 53. Prepare those codes or treat the column as numeric.".into(),
        forest::Error::InvalidSettings => "Check the tree count, candidate covariates, node sizes and positive random seed.".into(),
        forest::Error::InvalidTimes => "The forest needs non-negative follow-up times and at least one observed event.".into(),
        _ => "The forest could not be fitted to these rows. Check durations, event flags and covariate encoding.".into(),
    }
}

pub(crate) fn forest_evidence(values: &[f64], rows: usize, columns: usize, duration: usize, event: usize, covariates: &[usize], categorical: &[usize], trees: usize, mtry: usize, seed: u32, min_node_size: usize, min_bucket: usize, prediction_row: usize, split_rule: ForestSplitCommand) -> Result<AnalysisResult, String> {
    let x = regression_rows(values, rows, columns, duration, event, covariates)?;
    if prediction_row >= rows { return Err(format!("Choose a prediction row between 1 and {rows}.")); }
    if categorical.iter().any(|c| !covariates.contains(c)) || categorical.iter().collect::<std::collections::HashSet<_>>().len() != categorical.len() {
        return Err("Each categorical column must be a selected covariate, listed once.".into());
    }
    let profile = x[prediction_row].clone();
    let times = column(values, rows, columns, duration)?;
    let status = column(values, rows, columns, event)?;
    let events = (0..rows).map(|row| event_indicator(&status, row)).collect::<Result<Vec<_>, _>>()?;
    let event_count = events.iter().filter(|&&e| e).count();
    let data = forest::Data::new(x, times, events, covariates.iter().map(|c| !categorical.contains(c)).collect()).map_err(forest_problem)?;
    let rule = match split_rule { ForestSplitCommand::LogRank => forest::SplitRule::LogRank, ForestSplitCommand::ExtraTrees => forest::SplitRule::ExtraTrees { candidates: 1 } };
    let fitted = forest::train(&data, &forest::Settings { trees, mtry, seed, min_node_size, min_bucket, max_depth: None, rule }).map_err(forest_problem)?;
    let prediction = fitted.forest.predict(&profile).map_err(forest_problem)?;
    let available = |value| match value {
        Some(result) => SurvivalSummary::Recorded { result },
        None => SurvivalSummary::Unavailable { reason: "No comparable out-of-bag observation pairs were available.".into() },
    };
    Ok(AnalysisResult::SurvivalForest {
        observations: rows, events: event_count, trees,
        importance: fitted.permutation_importance.into_iter().map(available).collect(),
        concordance: available(fitted.prediction_error.map(|v| 1.0 - v)),
        prediction_row, profile, prediction_times: fitted.forest.times().to_vec(),
        survival: prediction.survival, cumulative_hazard: prediction.hazard,
    })
}
