//! Count time-series interventions: the negative-binomial INGARCH fit and the unknown-date scan.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn negative_binomial_ingarch(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    link: IngarchLink,
    regressors: &[usize],
    past_observation_lags: &[usize],
    past_mean_lags: &[usize],
    external_regressors: &[bool],
    horizon: usize,
    baseline_regressors: &[f64],
    intervention_regressor: usize,
    control_value: f64,
    treatment_value: f64,
    schedule: IngarchInterventionSchedule,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("negative-binomial INGARCH", values, rows, columns)?;
    if outcome >= columns
        || regressors.is_empty()
        || regressors
            .iter()
            .any(|&column| column >= columns || column == outcome)
        || baseline_regressors.len() != regressors.len()
        || external_regressors.len() != regressors.len()
        || horizon == 0
    {
        return Err("negative-binomial INGARCH received an invalid outcome, regressor, or horizon configuration".to_owned());
    }
    let treatment_position = regressors
        .iter()
        .position(|&column| column == intervention_regressor)
        .ok_or_else(|| {
            "the INGARCH intervention column must be included among its regressors".to_owned()
        })?;
    let y: Vec<f64> = (0..rows).map(|row| values[outcome * rows + row]).collect();
    let x: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            regressors
                .iter()
                .map(|&column| values[column * rows + row])
                .collect()
        })
        .collect();
    let specification = IngarchSpecification {
        link: match link {
            IngarchLink::Identity => CoreIngarchLink::Identity,
            IngarchLink::Log => CoreIngarchLink::Log,
        },
        past_observation_lags: past_observation_lags.to_vec(),
        past_mean_lags: past_mean_lags.to_vec(),
        external_regressors: external_regressors.to_vec(),
    };
    progress("fit", 0, 2);
    let fit = fit_negative_binomial_ingarch(&y, &x, specification)
        .map_err(|error| format!("negative-binomial INGARCH could not be fitted: {error:?}"))?;
    progress("forecast", 1, 2);
    let baseline = vec![baseline_regressors.to_vec(); horizon];
    let control = intervention_regressors(
        &baseline,
        treatment_position,
        control_value,
        InterventionSchedule::Persistent,
    )
    .map_err(|error| format!("invalid INGARCH control scenario: {error:?}"))?;
    let core_schedule = match schedule {
        IngarchInterventionSchedule::Point => InterventionSchedule::Point,
        IngarchInterventionSchedule::Persistent => InterventionSchedule::Persistent,
        IngarchInterventionSchedule::Decaying { delta } => InterventionSchedule::Decaying { delta },
    };
    let intervention =
        intervention_regressors(&control, treatment_position, treatment_value, core_schedule)
            .map_err(|error| format!("invalid INGARCH intervention scenario: {error:?}"))?;
    let baseline_mean = fit
        .forecast_mean(&y, &x, &control)
        .map_err(|error| format!("INGARCH baseline forecast failed: {error:?}"))?;
    let intervention_mean = fit
        .forecast_mean(&y, &x, &intervention)
        .map_err(|error| format!("INGARCH intervention forecast failed: {error:?}"))?;
    let effect_path: Vec<f64> = intervention_mean
        .iter()
        .zip(&baseline_mean)
        .map(|(treated, control)| treated - control)
        .collect();
    let cumulative_effect = effect_path.iter().sum();
    let average_effect = cumulative_effect / horizon as f64;
    progress("complete", 2, 2);
    Ok(AnalysisResult::NegativeBinomialIngarch {
        observations: rows,
        outcome,
        link,
        regressors: regressors.to_vec(),
        past_observation_lags: past_observation_lags.to_vec(),
        past_mean_lags: past_mean_lags.to_vec(),
        external_regressors: external_regressors.to_vec(),
        horizon,
        intervention_regressor,
        control_value,
        treatment_value,
        schedule,
        parameters: fit.parameters,
        fitted_means: fit.fitted_means,
        residuals: fit.residuals,
        log_likelihood: fit.log_likelihood,
        size: fit.size,
        dispersion: fit.dispersion,
        score: fit.score,
        iterations: fit.iterations,
        function_evaluations: fit.function_evaluations,
        gradient_evaluations: fit.gradient_evaluations,
        baseline_mean,
        intervention_mean,
        effect_path,
        average_effect,
        cumulative_effect,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn count_series_intervention_scan(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    link: IngarchLink,
    past_observation_lags: &[usize],
    past_mean_lags: &[usize],
    candidate_reference_points: &[usize],
    delta: f64,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("count-series intervention scan", values, rows, columns)?;
    if outcome >= columns
        || past_observation_lags.is_empty()
        || past_mean_lags.is_empty()
        || candidate_reference_points.is_empty()
    {
        return Err("count-series intervention scan received an invalid outcome, lag set, or candidate range".to_owned());
    }
    let observations: Vec<f64> = (0..rows).map(|row| values[outcome * rows + row]).collect();
    let no_regressors = vec![Vec::new(); rows];
    let specification = IngarchSpecification {
        link: match link {
            IngarchLink::Identity => CoreIngarchLink::Identity,
            IngarchLink::Log => CoreIngarchLink::Log,
        },
        past_observation_lags: past_observation_lags.to_vec(),
        past_mean_lags: past_mean_lags.to_vec(),
        external_regressors: vec![],
    };
    progress("fit-and-scan", 0, 1);
    let detection = detect_negative_binomial_intervention(
        &observations,
        &no_regressors,
        specification,
        candidate_reference_points,
        delta,
    )
    .map_err(|error| format!("count-series intervention scan failed: {error:?}"))?;
    progress("complete", 1, 1);
    Ok(AnalysisResult::CountSeriesInterventionScan {
        observations: rows,
        outcome,
        link,
        past_observation_lags: past_observation_lags.to_vec(),
        past_mean_lags: past_mean_lags.to_vec(),
        parameters: detection.null_fit.parameters,
        fitted_means: detection.null_fit.fitted_means,
        residuals: detection.null_fit.residuals,
        log_likelihood: detection.null_fit.log_likelihood,
        size: detection.null_fit.size,
        dispersion: detection.null_fit.dispersion,
        candidates: detection
            .candidates
            .into_iter()
            .map(|candidate| IngarchScanCandidateEvidence {
                reference_point: candidate.reference_point,
                score_statistic: candidate.score_statistic,
            })
            .collect(),
        strongest_reference_point: detection.strongest_reference_point,
        delta: detection.delta,
    })
}
