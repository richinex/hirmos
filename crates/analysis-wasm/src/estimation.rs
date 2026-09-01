//! Estimator façades: regression, count models, CausalEffects, causal impact, DML, ARDL, VECM, synthetic control, panel DID / SC / SDID, NUTS, discrete BN.

use super::*;
use hirmos_causal_core::frontdoor::{
    frontdoor_two_stage_with_progress, FrontdoorBootstrap, FrontdoorInput, FrontdoorOptions,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn binary_ett(
    values: &[f64],
    rows: usize,
    columns: usize,
    observed_nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("binary ETT", values, rows, columns)?;
    let nodes = names.len();
    if nodes < 2
        || names.iter().any(|name| name.trim().is_empty())
        || names
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != nodes
    {
        return Err("binary ETT needs at least two distinctly named graph nodes".to_owned());
    }
    if treatment >= nodes || outcome >= nodes || treatment == outcome {
        return Err("binary ETT needs distinct observed treatment and outcome nodes".to_owned());
    }
    if edges
        .iter()
        .any(|&(source, target)| source >= nodes || target >= nodes || source == target)
    {
        return Err("binary ETT edges must join two distinct graph nodes".to_owned());
    }
    let observed = observed_nodes
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let hidden = unobserved
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if observed_nodes.len() != columns
        || observed.len() != observed_nodes.len()
        || hidden.len() != unobserved.len()
        || observed
            .iter()
            .chain(hidden.iter())
            .any(|&node| node >= nodes)
        || !observed.is_disjoint(&hidden)
        || observed.len() + hidden.len() != nodes
        || !observed.contains(&treatment)
        || !observed.contains(&outcome)
    {
        return Err("binary ETT needs one matrix column for every observed node and an exact partition of observed and unobserved graph nodes".to_owned());
    }
    let named_edges = edges
        .iter()
        .map(|&(source, target)| (names[source].clone(), names[target].clone()))
        .collect::<Vec<_>>();
    let hidden_names = unobserved
        .iter()
        .map(|&node| names[node].clone())
        .collect::<Vec<_>>();
    let graph = latent_projection(names.to_vec(), named_edges, hidden_names)
        .map_err(|error| format!("binary ETT latent projection failed: {error}"))?;
    let mut table_columns = BTreeMap::new();
    for (column, &node) in observed_nodes.iter().enumerate() {
        let mut states = Vec::with_capacity(rows);
        for row in 0..rows {
            let value = values[column * rows + row];
            if value != 0.0 && value != 1.0 {
                return Err(format!(
                    "binary ETT requires every observed graph variable to contain only 0 and 1; {} has {value}",
                    names[node]
                ));
            }
            states.push(if value == 0.0 { "0" } else { "1" }.to_owned());
        }
        table_columns.insert(names[node].clone(), states);
    }
    let table = DiscreteTable::from_columns(table_columns)
        .map_err(|error| format!("binary ETT table failed: {error}"))?;
    let estimate = estimate_binary_ett(&graph, &table, &names[treatment], &names[outcome])
        .map_err(|error| format!("binary ETT failed: {error}"))?;
    Ok(AnalysisResult::BinaryEtt {
        observations: rows,
        treatment,
        outcome,
        treated_potential_outcome_mean: estimate.treated_potential_outcome_mean,
        untreated_potential_outcome_mean: estimate.untreated_potential_outcome_mean,
        effect_on_treated: estimate.effect_on_treated,
        treated_expression: estimate
            .treated_query
            .identified_to_y0(&estimate.treated_expression),
        untreated_expression: estimate
            .untreated_query
            .identified_to_y0(&estimate.untreated_expression),
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn frontdoor_two_stage_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    mediator: usize,
    outcome: usize,
    first_stage_adjustment: &[usize],
    second_stage_adjustment: &[usize],
    control_value: f64,
    treatment_value: f64,
    uncertainty: FrontdoorUncertainty,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    validate_dense_matrix("front-door two-stage estimate", values, rows, columns)?;
    let endpoints = [treatment, mediator, outcome];
    if endpoints.iter().any(|&column| column >= columns)
        || endpoints
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != endpoints.len()
    {
        return Err(
            "front-door estimation needs distinct treatment, mediator, and outcome columns"
                .to_owned(),
        );
    }
    let valid_adjustment = |stage: &str, excluded: &[usize], adjustment: &[usize]| {
        if adjustment.iter().any(|&column| column >= columns) {
            return Err(format!(
                "front-door {stage} adjustment columns must index the numeric matrix"
            ));
        }
        let distinct = adjustment
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if distinct.len() != adjustment.len()
            || excluded.iter().any(|column| distinct.contains(column))
        {
            return Err(format!(
                "front-door {stage} adjustment columns must be distinct and exclude its treatment, response, and outcome columns"
            ));
        }
        Ok(())
    };
    valid_adjustment(
        "first-stage",
        &[treatment, mediator, outcome],
        first_stage_adjustment,
    )?;
    valid_adjustment(
        "second-stage",
        &[mediator, outcome],
        second_stage_adjustment,
    )?;

    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| (0..rows).map(|row| data[(row, index)]).collect::<Vec<_>>();
    let treatment_values = column(treatment);
    let mediator_values = column(mediator);
    let outcome_values = column(outcome);
    let adjustment_matrix = |selected: &[usize]| {
        (!selected.is_empty()).then(|| {
            DMatrix::from_fn(rows, selected.len(), |row, column| {
                data[(row, selected[column])]
            })
        })
    };
    let first_adjustment = adjustment_matrix(first_stage_adjustment);
    let second_adjustment = adjustment_matrix(second_stage_adjustment);
    let bootstrap = match uncertainty {
        FrontdoorUncertainty::None => None,
        FrontdoorUncertainty::Bootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
        } => Some(FrontdoorBootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
        }),
    };
    let result = frontdoor_two_stage_with_progress(
        FrontdoorInput {
            treatment: &treatment_values,
            mediator: &mediator_values,
            outcome: &outcome_values,
            first_stage_adjustment: first_adjustment.as_ref(),
            second_stage_adjustment: second_adjustment.as_ref(),
        },
        FrontdoorOptions {
            control_value,
            treatment_value,
            bootstrap,
        },
        |completed, total| progress("frontdoor-bootstrap", completed, total),
    )
    .map_err(|error| error.to_string())?;
    let uncertainty = match (uncertainty, result.confidence_interval) {
        (FrontdoorUncertainty::None, None) => FrontdoorUncertaintyEvidence::None,
        (
            FrontdoorUncertainty::Bootstrap {
                simulations,
                sample_size_fraction,
                confidence_level,
                seed,
            },
            Some(interval),
        ) => FrontdoorUncertaintyEvidence::Bootstrap {
            simulations,
            sample_size_fraction,
            confidence_level,
            seed,
            interval,
        },
        _ => return Err("front-door uncertainty result did not match its request".to_owned()),
    };
    Ok(AnalysisResult::FrontdoorTwoStage {
        observations: rows,
        treatment,
        mediator,
        outcome,
        first_stage_adjustment: first_stage_adjustment.to_vec(),
        second_stage_adjustment: second_stage_adjustment.to_vec(),
        control_value,
        treatment_value,
        first_stage_params: result.first_stage_params,
        second_stage_params: result.second_stage_params,
        first_stage_effect: result.first_stage_effect,
        second_stage_effect: result.second_stage_effect,
        estimate: result.ate,
        uncertainty,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn backdoor_linear(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    hac_max_lags: Option<usize>,
    level: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("backdoor linear estimate", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err("backdoor linear estimate columns must index the numeric matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err(
            "backdoor linear estimate needs distinct treatment, outcome, and adjustment columns"
                .to_owned(),
        );
    }
    let parameters = 2 + adjustment.len();
    if rows <= parameters + 1 {
        return Err(format!(
            "backdoor linear estimate needs more than {} observations for {parameters} parameters",
            parameters + 1
        ));
    }
    if !(0.0..1.0).contains(&level) || level <= 0.5 {
        return Err("backdoor linear estimate level must lie in (0.5, 1)".to_owned());
    }
    if let Some(lags) = hac_max_lags {
        if lags >= rows {
            return Err(
                "backdoor linear estimate HAC lags must be fewer than the observations".to_owned(),
            );
        }
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    for &column in &used {
        let first = data[(0, column)];
        if (0..rows).all(|row| data[(row, column)] == first) {
            return Err("backdoor linear estimate is undefined for a constant column".to_owned());
        }
    }
    if (level - 0.95).abs() > 1e-12 {
        return Err(
            "backdoor linear estimate reports the 95% intervals of the ported estimation layer"
                .to_owned(),
        );
    }
    // Design [1, treatment, adjustment...]; the classical fit comes from the ported OLS, the HAC fit
    // from the ported 805 estimation layer.
    let design = DMatrix::<f64>::from_fn(rows, parameters, |row, column| match column {
        0 => 1.0,
        1 => data[(row, treatment)],
        _ => data[(row, adjustment[column - 2])],
    });
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let classical = Ols::fit(&design, &DVector::from_column_slice(&y));
    let degrees_of_freedom = rows - parameters;
    let standard_error = classical.params[1] / classical.tvalues()[1];
    let t = stdtri(degrees_of_freedom as isize, 0.975);
    // statsmodels' bandwidth when `maxlags` is not given: floor(4 (n/100)^(2/9)).
    let max_lags = hac_max_lags
        .unwrap_or_else(|| (4.0 * (rows as f64 / 100.0).powf(2.0 / 9.0)).floor() as usize);
    let hac = ols_hac(&y, &design, max_lags);
    Ok(AnalysisResult::BackdoorLinear {
        observations: rows,
        parameters,
        treatment,
        outcome,
        adjustment: adjustment.to_vec(),
        level,
        estimate: classical.params[1],
        standard_error,
        interval: [
            classical.params[1] - t * standard_error,
            classical.params[1] + t * standard_error,
        ],
        degrees_of_freedom,
        residual_sd: (classical.ssr / degrees_of_freedom as f64).sqrt(),
        r_squared: hac.rsquared,
        hac_max_lags: max_lags,
        hac_standard_error: hac.bse[1],
        hac_interval: [hac.conf_int[1].0, hac.conf_int[1].1],
        hac_p_value: hac.pvalues[1],
        durbin_watson: durbin_watson(&hac.resid),
    })
}

pub(crate) fn count_glm(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    family: CountFamily,
) -> Result<AnalysisResult, String> {
    let (_, design, y) = design_columns(
        "count GLM",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
    )?;
    if y.iter().any(|value| *value < 0.0 || value.fract() != 0.0) {
        return Err("count GLM needs a non-negative integer outcome".to_owned());
    }
    if y.iter().all(|value| *value == 0.0) {
        return Err("count GLM is undefined when every outcome is zero".to_owned());
    }
    let z = ndtri(0.975);
    let parameters = design.ncols();
    let (
        coefficient,
        standard_error,
        p_value,
        deviance,
        log_likelihood,
        alpha,
        degrees_of_freedom,
        converged,
        iterations,
    ) = match family {
        CountFamily::Poisson => {
            let fit = poisson_glm(&y, &design);
            (
                fit.params[1],
                fit.bse[1],
                fit.pvalues[1],
                fit.deviance,
                f64::NAN,
                f64::NAN,
                fit.df_resid as usize,
                fit.converged,
                fit.iterations,
            )
        }
        CountFamily::NegativeBinomial => {
            let fit = negative_binomial_p(&y, &design, 2.0);
            let alpha = fit.params[parameters];
            (
                fit.params[1],
                fit.bse[1],
                fit.pvalues[1],
                f64::NAN,
                fit.llf,
                alpha,
                rows - parameters - 1,
                fit.converged,
                fit.nit,
            )
        }
    };
    Ok(AnalysisResult::CountGlm {
        observations: rows,
        parameters,
        treatment,
        outcome,
        adjustment: adjustment.to_vec(),
        family,
        coefficient,
        standard_error,
        p_value,
        incidence_rate_ratio: coefficient.exp(),
        incidence_rate_ratio_interval: [
            (coefficient - z * standard_error).exp(),
            (coefficient + z * standard_error).exp(),
        ],
        level: 0.95,
        deviance,
        log_likelihood,
        alpha,
        degrees_of_freedom,
        converged,
        iterations,
    })
}

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

#[allow(clippy::too_many_arguments)]
pub(crate) fn causal_effects_total(
    values: &[f64],
    rows: usize,
    columns: usize,
    stat_lag: usize,
    graph: &[Vec<Vec<String>>],
    x: &[Node],
    y: &[Node],
    hidden: &[Node],
    estimator: TotalEffectEstimator,
    interventions: [f64; 2],
    uncertainty: CausalEffectsUncertainty,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("causal effects", values, rows, columns)?;
    if graph.len() != columns
        || graph.iter().any(|targets| {
            targets.len() != columns || targets.iter().any(|lags| lags.len() != stat_lag + 1)
        })
    {
        return Err(
            "causal effects graph must be variables by variables by stat_lag + 1 marks".to_owned(),
        );
    }
    if x.is_empty() || y.len() != 1 {
        return Err("causal effects needs at least one X node and exactly one Y node".to_owned());
    }
    for &(variable, lag) in x.iter().chain(y).chain(hidden) {
        if variable >= columns || lag > 0 || lag.unsigned_abs() as usize > stat_lag + 1 {
            return Err(
                "causal effects nodes must name a variable at a non-positive lag within the window"
                    .to_owned(),
            );
        }
    }
    if !interventions.iter().all(|value| value.is_finite()) {
        return Err("causal effects intervention values must be finite".to_owned());
    }
    let mut stationary = StationaryGraph::new(columns, stat_lag);
    for (i, targets) in graph.iter().enumerate() {
        for (j, lags) in targets.iter().enumerate() {
            for (tau, text) in lags.iter().enumerate() {
                if text.is_empty() {
                    continue;
                }
                if text.len() != 3 {
                    return Err(format!(
                        "causal effects mark {text:?} is not a three character tigramite mark"
                    ));
                }
                stationary.set(i, j, tau, mark(text));
            }
        }
    }
    let effects = CausalEffects::new(stationary, x, y, &[], hidden);
    let mediators: Vec<Node> = effects.mediators.iter().copied().collect();
    let no_causal_path = effects.no_causal_path;
    let data = time_series_from_column_major(values, rows, columns);
    let intervention_rows = [vec![interventions[0]], vec![interventions[1]]];
    let (identifiable, fit, predictions, total_effect, fitted, uncertainty_evidence) =
        if no_causal_path {
            (
                false,
                CausalEffectsFitEvidence::Unfitted {
                    requested: estimator,
                },
                Vec::new(),
                f64::NAN,
                0,
                CausalEffectsUncertaintyEvidence::None,
            )
        } else {
            match estimator {
                TotalEffectEstimator::Linear | TotalEffectEstimator::Knn { .. } => {
                    let Some(set) = effects.get_optimal_set() else {
                        return Ok(AnalysisResult::CausalEffectsTotal {
                            observations: rows,
                            tau_max: effects.tau_max,
                            no_causal_path,
                            identifiable: false,
                            mediators,
                            fit: CausalEffectsFitEvidence::Unfitted {
                                requested: estimator,
                            },
                            interventions,
                            predictions: Vec::new(),
                            total_effect: f64::NAN,
                            fitted_observations: 0,
                            uncertainty: CausalEffectsUncertaintyEvidence::None,
                        });
                    };
                    let (chosen, fit) = match estimator {
                        TotalEffectEstimator::Linear => (
                            Estimator::Linear,
                            CausalEffectsFitEvidence::AdjustedLinear {
                                adjustment_set: set.clone(),
                            },
                        ),
                        TotalEffectEstimator::Knn { k } => (
                            Estimator::KNeighbors { k: k.max(1) },
                            CausalEffectsFitEvidence::AdjustedKnn {
                                k: k.max(1),
                                adjustment_set: set.clone(),
                            },
                        ),
                        TotalEffectEstimator::WrightParents => unreachable!("outer match"),
                    };
                    let (model, uncertainty_evidence) = match uncertainty {
                        CausalEffectsUncertainty::None => {
                            let model =
                                effects.fit_total_effect(&data, chosen).ok_or_else(|| {
                                    "causal effects could not fit the total effect model".to_owned()
                                })?;
                            (model, CausalEffectsUncertaintyEvidence::None)
                        }
                        CausalEffectsUncertainty::Bootstrap {
                            samples,
                            block_length,
                            confidence_level,
                            seed,
                        } => {
                            validate_causal_effects_bootstrap(samples, confidence_level)?;
                            let core_block_length = core_block_length(block_length);
                            progress("causal-effects-bootstrap", 0, samples);
                            let bootstrap = effects
                                .fit_bootstrap_total_effect_with_progress(
                                    &data,
                                    chosen,
                                    None,
                                    TotalEffectBootstrapOptions {
                                        samples,
                                        block_length: core_block_length,
                                        seed,
                                    },
                                    |completed, total| {
                                        progress("causal-effects-bootstrap", completed, total)
                                    },
                                )
                                .map_err(causal_effects_bootstrap_error)?;
                            let bootstrap_prediction = bootstrap
                                .predict_total_effect(&intervention_rows, confidence_level);
                            let evidence = uncertainty_evidence_from_flat_predictions(
                                samples,
                                block_length,
                                bootstrap.resolved_block_length,
                                confidence_level,
                                seed,
                                &bootstrap_prediction.individual_predictions,
                                &bootstrap_prediction.confidence_interval,
                            );
                            (bootstrap.original_model, evidence)
                        }
                    };
                    let predicted = model.predict_total_effect(&intervention_rows);
                    let effect = predicted[1] - predicted[0];
                    (
                        true,
                        fit,
                        predicted,
                        effect,
                        model.n_obs,
                        uncertainty_evidence,
                    )
                }
                TotalEffectEstimator::WrightParents => {
                    let (model, uncertainty_evidence) = match uncertainty {
                        CausalEffectsUncertainty::None => (
                            effects
                                .fit_wright_effect(
                                    &data,
                                    WrightCoefficientMethod::Parents,
                                    WrightMediation::Total,
                                )
                                .map_err(causal_effects_wright_error)?,
                            CausalEffectsUncertaintyEvidence::None,
                        ),
                        CausalEffectsUncertainty::Bootstrap {
                            samples,
                            block_length,
                            confidence_level,
                            seed,
                        } => {
                            validate_causal_effects_bootstrap(samples, confidence_level)?;
                            progress("causal-effects-wright-bootstrap", 0, samples);
                            let bootstrap = effects
                                .fit_bootstrap_wright_effect_with_progress(
                                    &data,
                                    WrightCoefficientMethod::Parents,
                                    WrightMediation::Total,
                                    TotalEffectBootstrapOptions {
                                        samples,
                                        block_length: core_block_length(block_length),
                                        seed,
                                    },
                                    |completed, total| {
                                        progress(
                                            "causal-effects-wright-bootstrap",
                                            completed,
                                            total,
                                        )
                                    },
                                )
                                .map_err(causal_effects_wright_error)?;
                            let prediction = bootstrap
                                .predict_wright_effect(&intervention_rows, confidence_level);
                            let draws: Vec<Vec<f64>> = prediction
                                .individual_predictions
                                .iter()
                                .map(|draw| vec![draw[0][0], draw[1][0]])
                                .collect();
                            let interval = vec![
                                vec![prediction.confidence_interval[0][0][0]],
                                vec![prediction.confidence_interval[0][1][0]],
                                vec![prediction.confidence_interval[1][0][0]],
                                vec![prediction.confidence_interval[1][1][0]],
                            ];
                            let flat_interval = vec![
                                vec![interval[0][0], interval[1][0]],
                                vec![interval[2][0], interval[3][0]],
                            ];
                            let evidence = uncertainty_evidence_from_flat_predictions(
                                samples,
                                block_length,
                                bootstrap.resolved_block_length,
                                confidence_level,
                                seed,
                                &draws,
                                &flat_interval,
                            );
                            (bootstrap.original_model, evidence)
                        }
                    };
                    let prediction_rows = model.predict_wright_effect(&intervention_rows);
                    let predicted = vec![prediction_rows[0][0], prediction_rows[1][0]];
                    let effect = predicted[1] - predicted[0];
                    let delta = interventions[1] - interventions[0];
                    let direct_effect = model
                        .paths
                        .iter()
                        .filter(|path| path.path.len() == 2)
                        .map(|path| path.value * delta)
                        .sum::<f64>();
                    let fit = CausalEffectsFitEvidence::WrightParents {
                        coefficients: model
                            .coefficients
                            .iter()
                            .map(|coefficient| WrightCoefficientEvidence {
                                parent: coefficient.parent,
                                child: coefficient.child,
                                coefficient: coefficient.value,
                            })
                            .collect(),
                        paths: model
                            .paths
                            .iter()
                            .map(|path| WrightPathEvidence {
                                nodes: path.path.clone(),
                                coefficient: path.value,
                                contrast: path.value * delta,
                            })
                            .collect(),
                        direct_effect,
                        indirect_effect: effect - direct_effect,
                    };
                    (
                        true,
                        fit,
                        predicted,
                        effect,
                        model.n_obs,
                        uncertainty_evidence,
                    )
                }
            }
        };
    Ok(AnalysisResult::CausalEffectsTotal {
        observations: rows,
        tau_max: effects.tau_max,
        no_causal_path,
        identifiable,
        mediators,
        fit,
        interventions,
        predictions,
        total_effect,
        fitted_observations: fitted,
        uncertainty: uncertainty_evidence,
    })
}

fn validate_causal_effects_bootstrap(samples: usize, confidence_level: f64) -> Result<(), String> {
    if samples == 0 || !(0.0 < confidence_level && confidence_level < 1.0) {
        return Err("causal effects bootstrap needs positive samples and a confidence level between zero and one".to_owned());
    }
    Ok(())
}

fn core_block_length(block_length: CausalEffectsBlockLength) -> BootstrapBlockLength {
    match block_length {
        CausalEffectsBlockLength::Fixed { length } => BootstrapBlockLength::Fixed(length),
        CausalEffectsBlockLength::CubeRoot => BootstrapBlockLength::CubeRoot,
    }
}

fn uncertainty_evidence_from_flat_predictions(
    samples: usize,
    block_length: CausalEffectsBlockLength,
    resolved_block_length: usize,
    confidence_level: f64,
    seed: u64,
    individual_predictions: &[Vec<f64>],
    confidence_interval: &[Vec<f64>],
) -> CausalEffectsUncertaintyEvidence {
    let effect_draws: Vec<f64> = individual_predictions
        .iter()
        .map(|prediction| prediction[1] - prediction[0])
        .collect();
    let tail = (1.0 - confidence_level) / 2.0;
    let effect_interval = [
        hirmos_causal_core::causal_effects::numpy_percentile(&effect_draws, tail),
        hirmos_causal_core::causal_effects::numpy_percentile(&effect_draws, 1.0 - tail),
    ];
    CausalEffectsUncertaintyEvidence::Bootstrap {
        samples,
        block_length,
        resolved_block_length,
        confidence_level,
        seed,
        prediction_intervals: [
            [confidence_interval[0][0], confidence_interval[1][0]],
            [confidence_interval[0][1], confidence_interval[1][1]],
        ],
        effect_interval,
        effect_draws,
    }
}

fn causal_effects_bootstrap_error(error: TotalEffectBootstrapError) -> String {
    match error {
        TotalEffectBootstrapError::NotIdentifiable => {
            "causal effects bootstrap is not identifiable by adjustment".to_owned()
        }
        TotalEffectBootstrapError::ZeroSamples => {
            "causal effects bootstrap needs at least one sample".to_owned()
        }
        TotalEffectBootstrapError::ZeroBlockLength => {
            "causal effects bootstrap block length must be positive".to_owned()
        }
        TotalEffectBootstrapError::TooFewBlocks { blocks } => format!(
            "causal effects bootstrap block length leaves only {blocks} block; choose a shorter block"
        ),
        TotalEffectBootstrapError::SeedOverflow => {
            "causal effects bootstrap seed schedule overflowed".to_owned()
        }
    }
}

fn causal_effects_wright_error(error: WrightEffectError) -> String {
    match error {
        WrightEffectError::ParentMethodHasBidirectedLink { node } => format!(
            "Wright parent coefficients require a DAG without a bidirected link adjacent to ({}, {})",
            node.0, node.1
        ),
        WrightEffectError::MissingLinkCoefficients { variable } => {
            format!("Wright coefficients are missing for variable {variable}")
        }
        WrightEffectError::MissingPathCoefficient { parent, child } => format!(
            "Wright path coefficient is missing for ({}, {}) -> ({}, {})",
            parent.0, parent.1, child.0, child.1
        ),
        WrightEffectError::ZeroSamples => {
            "Wright bootstrap needs at least one sample".to_owned()
        }
        WrightEffectError::ZeroBlockLength => {
            "Wright bootstrap block length must be positive".to_owned()
        }
        WrightEffectError::TooFewBlocks { blocks } => format!(
            "Wright bootstrap block length leaves only {blocks} block; choose a shorter block"
        ),
        WrightEffectError::SeedOverflow => {
            "Wright bootstrap seed schedule overflowed".to_owned()
        }
    }
}

pub(crate) fn causal_impact_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    outcome: usize,
    controls: &[usize],
    n_pre: usize,
    max_iter: usize,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("causal impact", values, rows, columns)?;
    if outcome >= columns
        || controls
            .iter()
            .any(|&column| column >= columns || column == outcome)
    {
        return Err(
            "causal impact outcome and controls must be distinct numeric columns".to_owned(),
        );
    }
    if n_pre < 8 || n_pre >= rows {
        return Err("causal impact needs at least 8 pre-intervention rows and at least one post-intervention row".to_owned());
    }
    if !(1..=2000).contains(&max_iter) {
        return Err("causal impact maxIter must be between 1 and 2000".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let exog: Vec<Vec<f64>> = (0..rows)
        .map(|row| controls.iter().map(|&column| data[(row, column)]).collect())
        .collect();
    let impact = causal_impact(&y, &exog, n_pre, max_iter);
    Ok(AnalysisResult::CausalImpact {
        observations: rows,
        n_pre,
        n_post: rows - n_pre,
        outcome,
        controls: controls.to_vec(),
        counterfactual: impact.counterfactual,
        counterfactual_se: impact.counterfactual_se,
        pointwise: impact.pointwise,
        cumulative: impact.cumulative,
        average: impact.average,
        params: impact.params,
        log_likelihood: impact.llf,
    })
}

/// The DoubleML study frame: covariate rows, outcome, treatment, and the model flags.
pub(crate) fn dml_frame(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: DmlModel,
) -> Result<(Vec<Vec<f64>>, Vec<f64>, Vec<f64>, bool), String> {
    let (data, _, y) =
        design_columns(label, values, rows, columns, treatment, outcome, adjustment)?;
    if adjustment.is_empty() {
        return Err(format!(
            "{label} needs at least one adjustment column for the nuisance learners"
        ));
    }
    if rows < 20 {
        return Err(format!("{label} needs at least 20 rows for cross-fitting"));
    }
    let d: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    let treat_binary = d.iter().all(|&value| value == 0.0 || value == 1.0);
    if model == DmlModel::Irm {
        if !treat_binary {
            return Err(format!(
                "{label} IRM needs a binary treatment coded 0 and 1"
            ));
        }
        let treated = d.iter().filter(|&&value| value == 1.0).count();
        if treated < 5 || rows - treated < 5 {
            return Err(format!(
                "{label} IRM needs at least five treated and five untreated rows"
            ));
        }
    }
    let x: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            adjustment
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    Ok((x, y, d, treat_binary))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn double_ml(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: DmlModel,
    att: bool,
    seed: u32,
) -> Result<AnalysisResult, String> {
    let (x, y, d, treat_binary) = dml_frame(
        "double machine learning",
        values,
        rows,
        columns,
        treatment,
        outcome,
        adjustment,
        model,
    )?;
    let study = WorkerStudy {
        x: &x,
        y: &y,
        d: &d,
        aipw: model == DmlModel::Irm,
        att,
        treat_binary,
    };
    let mut fold_stream = Mt19937::seeded(seed);
    let fit = worker_fit(&study, &mut fold_stream);
    Ok(AnalysisResult::DoubleMl {
        observations: rows,
        model,
        att,
        treat_binary,
        seed,
        estimate: fit.coef,
        standard_error: fit.se,
        interval: (fit.ci_low, fit.ci_high),
        level: 0.95,
    })
}

pub(crate) fn ardl_pss(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    max_lag: usize,
    trend: ArdlTrend,
    case: usize,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("ARDL", values, rows, columns)?;
    if treatment >= columns || outcome >= columns || treatment == outcome {
        return Err(
            "ARDL needs distinct treatment and outcome columns inside the matrix".to_owned(),
        );
    }
    if !(1..=24).contains(&max_lag) {
        return Err("ARDL maximum lag must be between 1 and 24".to_owned());
    }
    let valid_case = match trend {
        ArdlTrend::C => case == 2 || case == 3,
        ArdlTrend::Ct => case == 4 || case == 5,
    };
    if !valid_case || stat_star(1, case, true).is_none() {
        return Err(
            "ARDL PSS case must be 2 or 3 with a constant and 4 or 5 with a trend".to_owned(),
        );
    }
    if rows < 6 * (max_lag + 1) + 10 {
        return Err(format!(
            "ARDL needs at least {} rows for a maximum lag of {max_lag}",
            6 * (max_lag + 1) + 10
        ));
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let y: Vec<f64> = (0..rows).map(|row| data[(row, outcome)]).collect();
    let x: Vec<f64> = (0..rows).map(|row| data[(row, treatment)]).collect();
    let trend_kind = match trend {
        ArdlTrend::C => Trend::C,
        ArdlTrend::Ct => Trend::Ct,
    };
    let selection = ardl_select_order(&y, max_lag, &x, max_lag, "aic", trend_kind);
    let p = selection.ar_lag.max(1);
    let q = selection.dl_lag.unwrap_or(1).max(1);
    let model = uecm(&y, p, &x, q, trend_kind);
    let vector = model.cointegrating_vector(0.05);
    let index = model.n_det + 1;
    let bounds = bounds_test(&model, case);
    Ok(AnalysisResult::ArdlPss {
        observations: rows,
        trend,
        case,
        ar_lag: p,
        dl_lag: q,
        grid: selection.grid,
        long_run_effect: -vector.params[index],
        p_value: vector.pvalues[index],
        interval: (-vector.conf_int[index].1, -vector.conf_int[index].0),
        level: 0.95,
        bounds_statistic: bounds.stat,
        bounds_critical: bounds.crit_vals,
        bounds_p_lower: bounds.p_lower,
        bounds_p_upper: bounds.p_upper,
    })
}

pub(crate) fn vecm(
    values: &[f64],
    rows: usize,
    columns: usize,
    endogenous: &[usize],
    max_lags: usize,
    deterministic: VecmDeterministic,
    significance: usize,
    break_index: Option<usize>,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("VECM", values, rows, columns)?;
    if endogenous.len() < 2 || endogenous.iter().any(|&column| column >= columns) {
        return Err("VECM needs at least two endogenous columns inside the matrix".to_owned());
    }
    let mut distinct = endogenous.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != endogenous.len() {
        return Err("VECM endogenous columns must be distinct".to_owned());
    }
    if !(1..=24).contains(&max_lags) {
        return Err("VECM maximum lags must be between 1 and 24".to_owned());
    }
    if significance > 2 {
        return Err("VECM significance column must be 0, 1 or 2".to_owned());
    }
    if rows < (max_lags + 2) * endogenous.len() * 3 + 10 {
        return Err(format!(
            "VECM needs more rows than {} for {} variables at {max_lags} lags",
            (max_lags + 2) * endogenous.len() * 3 + 10,
            endogenous.len()
        ));
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let endog: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            endogenous
                .iter()
                .map(|&column| data[(row, column)])
                .collect()
        })
        .collect();
    let code = deterministic.code();
    let k_ar_diff = vecm_select_order(&endog, max_lags, code).max(1);
    let rank = select_coint_rank(&endog, deterministic.det_order(), k_ar_diff, significance);
    let chow = match break_index {
        Some(index) => {
            if index < 4 || index + 4 > rows {
                return Err(
                    "VECM Chow break index must leave at least four rows on each side".to_owned(),
                );
            }
            let y: Vec<f64> = endog.iter().map(|row| row[0]).collect();
            let x: Vec<f64> = endog.iter().map(|row| row[1]).collect();
            Some(chow_break(&y, &x, index))
        }
        None => None,
    };
    let to_rows = |matrix: &DMatrix<f64>| -> Vec<Vec<f64>> {
        (0..matrix.nrows())
            .map(|i| (0..matrix.ncols()).map(|j| matrix[(i, j)]).collect())
            .collect()
    };
    if rank == 0 {
        return Ok(AnalysisResult::Vecm {
            observations: rows,
            deterministic,
            k_ar_diff,
            rank,
            significance,
            long_run_effect: None,
            alpha: Vec::new(),
            beta: Vec::new(),
            gamma: Vec::new(),
            pvalues_alpha: Vec::new(),
            chow,
        });
    }
    let fit = vecm_fit(&endog, k_ar_diff, rank, code);
    // beta is identity-normalised on top, so with the outcome first the long-run effect of the
    // second variable is the negated second row of the first relation.
    let long_run_effect = if rank == 1 && fit.beta.nrows() >= 2 {
        Some(-fit.beta[(1, 0)])
    } else {
        None
    };
    Ok(AnalysisResult::Vecm {
        observations: rows,
        deterministic,
        k_ar_diff,
        rank,
        significance,
        long_run_effect,
        alpha: to_rows(&fit.alpha),
        beta: to_rows(&fit.beta),
        gamma: to_rows(&fit.gamma),
        pvalues_alpha: to_rows(&fit.pvalues_alpha),
        chow,
    })
}

pub(crate) fn synthetic_control(
    values: &[f64],
    rows: usize,
    columns: usize,
    treated: usize,
    donors: &[usize],
    n_pre: usize,
    cross_fit_folds: usize,
    alpha: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("synthetic control", values, rows, columns)?;
    let mut used = vec![treated];
    used.extend_from_slice(donors);
    if donors.is_empty() || used.iter().any(|&column| column >= columns) {
        return Err("synthetic control needs a treated column and at least one donor column inside the matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err("synthetic control treated and donor columns must be distinct".to_owned());
    }
    if n_pre < 2 || n_pre >= rows {
        return Err("synthetic control needs at least 2 pre-intervention rows and at least one post-intervention row".to_owned());
    }
    if cross_fit_folds < 2 {
        return Err("synthetic control cross-fitting needs at least 2 folds".to_owned());
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha >= 1.0 {
        return Err("synthetic control inference alpha must be between zero and one".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let n_post = rows - n_pre;
    let y_pre_co = DMatrix::from_fn(n_pre, donors.len(), |row, j| data[(row, donors[j])]);
    let y_post_co = DMatrix::from_fn(n_post, donors.len(), |row, j| {
        data[(n_pre + row, donors[j])]
    });
    let y_pre_tr = DVector::from_fn(n_pre, |row, _| data[(row, treated)]);
    let y_post_tr = DVector::from_fn(n_post, |row, _| data[(n_pre + row, treated)]);
    let (control, effect) = synthetic_effect(&y_pre_co, &y_pre_tr, &y_post_co, &y_post_tr);
    let treated_series: Vec<f64> = (0..rows).map(|row| data[(row, treated)]).collect();
    let synthetic: Vec<f64> = (0..rows)
        .map(|row| {
            donors
                .iter()
                .zip(&control.weights)
                .map(|(&column, weight)| data[(row, column)] * weight)
                .sum()
        })
        .collect();
    let cross_fit = match debiased_synthetic_control(
        &y_pre_co,
        &y_pre_tr,
        &y_post_co,
        &y_post_tr,
        cross_fit_folds,
    ) {
        Ok(inference) => SyntheticCrossFitEvidence::Available {
            att: inference.att,
            standard_error: inference.standard_error,
            t_statistic: inference.t_statistic,
            degrees_of_freedom: inference.degrees_of_freedom,
            p_value: inference.p_value,
            confidence_interval: inference.confidence_interval,
            block_size: inference.block_size,
            folds: inference
                .folds
                .into_iter()
                .map(|fold| SyntheticCrossFitFoldEvidence {
                    held_out: fold.held_out,
                    weights: fold.weights,
                    bias: fold.bias,
                    att: fold.att,
                })
                .collect(),
        },
        Err(error) => SyntheticCrossFitEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let outcome_controls = DMatrix::from_fn(rows, donors.len(), |row, column| {
        data[(row, donors[column])]
    });
    let outcome_treated = DVector::from_fn(rows, |row, _| data[(row, treated)]);
    let donor_placebo = match donor_placebo_mspe_inference(
        &y_pre_co,
        &y_pre_tr,
        &outcome_controls,
        &outcome_treated,
        n_pre,
    ) {
        Ok(inference) => DonorPlaceboInferenceEvidence::Available {
            treated_pre_mspe: inference.treated.pre_mspe,
            treated_post_mspe: inference.treated.post_mspe,
            treated_mspe_ratio: inference
                .treated
                .mspe_ratio
                .is_finite()
                .then_some(inference.treated.mspe_ratio),
            placebos: inference
                .placebos
                .into_iter()
                .map(|placebo| DonorPlaceboEvidence {
                    donor: placebo.donor,
                    pre_mspe: placebo.summary.pre_mspe,
                    post_mspe: placebo.summary.post_mspe,
                    mspe_ratio: placebo
                        .summary
                        .mspe_ratio
                        .is_finite()
                        .then_some(placebo.summary.mspe_ratio),
                })
                .collect(),
            p_value: inference.p_value,
            n_valid_placebos: inference.n_valid_placebos,
        },
        Err(error) => DonorPlaceboInferenceEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let band = |method| match synthetic_control_prediction_band(
        &outcome_controls,
        &outcome_treated,
        &control.weights,
        n_pre,
        alpha,
        method,
    ) {
        Ok(inference) if inference.half_width.is_finite() => {
            SyntheticPredictionBandEvidence::Available {
                alpha: inference.alpha,
                intervals: inference.intervals,
                half_width: inference.half_width,
                pre_mspe: inference.pre_mspe,
                post_mspe: inference.post_mspe,
                mspe_ratio: inference
                    .mspe_ratio
                    .is_finite()
                    .then_some(inference.mspe_ratio),
            }
        }
        Ok(_) => SyntheticPredictionBandEvidence::Unavailable {
            reason: "the requested conformal level needs more pre-intervention residuals"
                .to_owned(),
        },
        Err(error) => SyntheticPredictionBandEvidence::Unavailable {
            reason: synthetic_control_problem(error),
        },
    };
    let conformal_band = band(SyntheticControlBandMethod::Conformal);
    let gaussian_band = band(SyntheticControlBandMethod::ParametricGaussian);
    Ok(AnalysisResult::SyntheticControl {
        observations: rows,
        n_pre,
        n_post,
        weights: control.weights,
        loss: control.loss,
        iterations: control.iterations,
        pre_gap: effect.pre_gap,
        post_gap: effect.post_gap,
        att: effect.att,
        treated: treated_series,
        synthetic,
        cross_fit,
        donor_placebo,
        conformal_band,
        gaussian_band,
    })
}

fn synthetic_control_problem(error: SyntheticControlError) -> String {
    match error {
        SyntheticControlError::EmptyDesign => "the synthetic-control design is empty".to_owned(),
        SyntheticControlError::DimensionMismatch => "the treated and donor matrices do not have compatible dimensions".to_owned(),
        SyntheticControlError::NonFiniteInput => "the synthetic-control design contains a non-finite value".to_owned(),
        SyntheticControlError::InvalidFoldCount => "cross-fitting needs at least two folds".to_owned(),
        SyntheticControlError::InsufficientFoldData => "the pre- and post-intervention periods are too short for the requested cross-fitting folds".to_owned(),
        SyntheticControlError::InvalidPrePeriodBoundary => "prediction inference needs at least two pre-intervention rows and one post-intervention row".to_owned(),
        SyntheticControlError::InsufficientDonors => "donor-placebo inference needs at least two donor series".to_owned(),
        SyntheticControlError::InvalidAlpha => "prediction-band alpha must be between zero and one".to_owned(),
    }
}

fn panel_method_evidence(
    estimate: hirmos_causal_core::panel::PanelEstimate,
) -> PanelMethodEvidence {
    PanelMethodEvidence {
        estimate: estimate.estimate,
        lambda: estimate.lambda,
        omega: estimate.omega,
        effect_curve: estimate.effect_curve,
        lambda_iterations: estimate.lambda_iterations,
        omega_iterations: estimate.omega_iterations,
        lambda_objective: estimate.lambda_objective,
        omega_objective: estimate.omega_objective,
        noise_level: estimate.noise_level,
    }
}

fn panel_problem(error: hirmos_causal_core::panel::PanelError) -> String {
    use hirmos_causal_core::panel::PanelError;
    match error {
        PanelError::Empty => "the panel has no observations".to_owned(),
        PanelError::EmptyUnit { row } => format!("the unit key is empty at row {}", row + 1),
        PanelError::NonFiniteOutcome { row } => format!("the outcome is not finite at row {}", row + 1),
        PanelError::NonFiniteTreatment { row } => format!("the treatment is not finite at row {}", row + 1),
        PanelError::TreatmentNotBinary { row } => format!("the treatment is not binary at row {}", row + 1),
        PanelError::DuplicateCell { unit, time } => format!("unit {unit} has more than one observation at time code {time}"),
        PanelError::MissingCell { unit, time } => format!("unit {unit} has no observation at time code {time}"),
        PanelError::NoTreatmentVariation => "the treatment has no zero-to-one variation".to_owned(),
        PanelError::NoPreTreatmentPeriod => "the panel has no pre-treatment period".to_owned(),
        PanelError::NoPostTreatmentPeriod => "the panel has no post-treatment period".to_owned(),
        PanelError::NoControlUnit => "the panel has no never-treated control unit".to_owned(),
        PanelError::NoTreatedUnit => "the panel has no treated unit".to_owned(),
        PanelError::NonSimultaneousAdoption => "treated units do not adopt simultaneously and remain treated".to_owned(),
        PanelError::InvalidMatrixBoundary => "the panel boundary does not contain controls, treated units, pre-periods, and post-periods".to_owned(),
        PanelError::DegenerateNoise => "the control pre-period has no usable first-difference variation for synthetic weighting".to_owned(),
        PanelError::TooFewControlsForPlacebo => "placebo variance needs more control units than treated units".to_owned(),
        PanelError::InsufficientPlaceboReplications => "placebo variance needs at least two replications".to_owned(),
        PanelError::InvalidPlaceboPermutation { replication } => format!("placebo replication {} is not a permutation of the control units", replication + 1),
        PanelError::InvalidTreatedFraction => "the pre-treatment period is too short for an in-time placebo".to_owned(),
    }
}

fn panel_placebo_evidence(
    result: Result<
        hirmos_causal_core::panel::PanelPlaceboInference,
        hirmos_causal_core::panel::PanelError,
    >,
    replications: usize,
    seed: u64,
) -> PanelPlaceboEvidence {
    match result {
        Ok(inference) => PanelPlaceboEvidence::Available {
            replications,
            seed,
            standard_error: inference.standard_error,
            estimates: inference.estimates,
        },
        Err(error) => PanelPlaceboEvidence::Unavailable {
            reason: panel_problem(error),
        },
    }
}

fn panel_in_time_evidence(
    result: Result<hirmos_causal_core::panel::PanelEstimate, hirmos_causal_core::panel::PanelError>,
) -> PanelInTimeEvidence {
    match result {
        Ok(estimate) => PanelInTimeEvidence::Available {
            estimate: estimate.estimate,
            effect_curve: estimate.effect_curve,
        },
        Err(error) => PanelInTimeEvidence::Unavailable {
            reason: panel_problem(error),
        },
    }
}

pub(crate) fn panel_intervention(
    values: &[f64],
    rows: usize,
    units: &[String],
    times: &[i64],
    placebo_replications: usize,
    seed: u64,
    progress: impl Fn(&'static str, usize, usize),
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("panel intervention", values, rows, 2)?;
    if units.len() != rows || times.len() != rows {
        return Err("panel intervention needs one unit and time key per numeric row".to_owned());
    }
    let observations = (0..rows)
        .map(|row| hirmos_causal_core::panel::PanelObservation {
            unit: units[row].clone(),
            time: times[row],
            outcome: values[row],
            treatment: values[rows + row],
        })
        .collect::<Vec<_>>();
    let panel = hirmos_causal_core::panel::panel_matrices(&observations).map_err(panel_problem)?;
    progress("validated-panel", 1, 9);
    let did = hirmos_causal_core::panel::did_estimate(&panel.y, panel.n0, panel.t0)
        .map_err(panel_problem)?;
    progress("difference-in-differences", 2, 9);
    let synthetic_control =
        hirmos_causal_core::panel::synthdid_sc_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-control", 3, 9);
    let synthetic_did =
        hirmos_causal_core::panel::synthetic_did_estimate(&panel.y, panel.n0, panel.t0)
            .map_err(panel_problem)?;
    progress("synthetic-did", 4, 9);
    let mut rng = NpRng::seeded(seed);
    let permutations = (0..placebo_replications)
        .map(|_| rng.permutation(panel.n0))
        .collect::<Vec<_>>();
    progress("placebo-permutations", 5, 9);
    let synthetic_control_placebo = panel_placebo_evidence(
        hirmos_causal_core::panel::panel_placebo_standard_error(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticControl,
            &permutations,
        ),
        placebo_replications,
        seed,
    );
    progress("synthetic-control-placebos", 6, 9);
    let synthetic_did_placebo = panel_placebo_evidence(
        hirmos_causal_core::panel::panel_placebo_standard_error(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticDifferenceInDifferences,
            &permutations,
        ),
        placebo_replications,
        seed,
    );
    progress("synthetic-did-placebos", 7, 9);
    let synthetic_control_in_time =
        panel_in_time_evidence(hirmos_causal_core::panel::panel_in_time_placebo(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticControl,
            None,
        ));
    progress("synthetic-control-in-time", 8, 9);
    let synthetic_did_in_time =
        panel_in_time_evidence(hirmos_causal_core::panel::panel_in_time_placebo(
            &panel.y,
            panel.n0,
            panel.t0,
            hirmos_causal_core::panel::PanelEstimatorKind::SyntheticDifferenceInDifferences,
            None,
        ));
    progress("synthetic-did-in-time", 9, 9);
    Ok(AnalysisResult::PanelIntervention {
        observations: rows,
        treated_units: panel.y.nrows() - panel.n0,
        control_units: panel.n0,
        n_pre: panel.t0,
        n_post: panel.y.ncols() - panel.t0,
        units: panel.units,
        times: panel.times,
        did: panel_method_evidence(did),
        synthetic_control: panel_method_evidence(synthetic_control),
        synthetic_did: panel_method_evidence(synthetic_did),
        synthetic_control_placebo,
        synthetic_did_placebo,
        synthetic_control_in_time,
        synthetic_did_in_time,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn negbin_nuts(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    confounder: usize,
    warmup: usize,
    samples: usize,
    seed: u64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("negative binomial NUTS", values, rows, columns)?;
    let used = [treatment, outcome, confounder];
    if used.iter().any(|&column| column >= columns)
        || treatment == outcome
        || treatment == confounder
        || outcome == confounder
    {
        return Err("negative binomial NUTS needs distinct treatment, outcome, and confounder columns inside the matrix".to_owned());
    }
    if rows < 20 {
        return Err("negative binomial NUTS needs at least 20 rows".to_owned());
    }
    if !(10..=5000).contains(&warmup) || !(10..=5000).contains(&samples) {
        return Err(
            "negative binomial NUTS warmup and samples must be between 10 and 5000".to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| -> Vec<f64> { (0..rows).map(|row| data[(row, index)]).collect() };
    let y = column(outcome);
    if y.iter().any(|value| *value < 0.0 || value.fract() != 0.0) {
        return Err("negative binomial NUTS outcome must hold non-negative integers".to_owned());
    }
    let d = column(treatment);
    let w = column(confounder);
    let spread = |series: &[f64]| -> bool {
        let mean = series.iter().sum::<f64>() / rows as f64;
        series.iter().any(|value| (value - mean).abs() > 0.0)
    };
    if !spread(&d) || !spread(&w) {
        return Err("negative binomial NUTS treatment and confounder must vary".to_owned());
    }
    let model = PbcNegBinModel::from_raw(&d, &w, &y);
    let options = NutsOptions {
        warmup,
        samples,
        ..NutsOptions::default()
    };
    let result = model.sample(options, seed);
    let draws = model.irr_draws(&result);
    let summary = irr_summary(&draws);
    let betas: Vec<f64> = result.samples.iter().map(|draw| draw[1]).collect();
    let beta_mean = betas.iter().sum::<f64>() / betas.len() as f64;
    let beta_sd = (betas
        .iter()
        .map(|value| (value - beta_mean).powi(2))
        .sum::<f64>()
        / (betas.len() as f64 - 1.0))
        .sqrt();
    let dispersion_mean =
        result.samples.iter().map(|draw| draw[3].exp()).sum::<f64>() / result.samples.len() as f64;
    Ok(AnalysisResult::NegbinNuts {
        observations: rows,
        warmup,
        samples,
        seed,
        irr_median: summary.median,
        irr_lower: summary.lower,
        irr_upper: summary.upper,
        beta_treatment_mean: beta_mean,
        beta_treatment_sd: beta_sd,
        dispersion_mean,
        divergences: result.divergences,
        acceptance_rate: result.acceptance_rate,
        mean_accept_probability: result.mean_accept_probability,
        step_size: result.step_size,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn bayesian_gaussian(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    warmup: usize,
    samples: usize,
    seed: u64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("Bayesian Gaussian regression", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err(
            "Bayesian Gaussian regression columns must index the numeric matrix".to_owned(),
        );
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err("Bayesian Gaussian regression needs distinct treatment, outcome, and adjustment columns".to_owned());
    }
    if rows < 20 {
        return Err("Bayesian Gaussian regression needs at least 20 rows".to_owned());
    }
    if !(10..=5000).contains(&warmup) || !(10..=5000).contains(&samples) {
        return Err(
            "Bayesian Gaussian regression warmup and samples must be between 10 and 5000"
                .to_owned(),
        );
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let column = |index: usize| -> Vec<f64> { (0..rows).map(|row| data[(row, index)]).collect() };
    let d = column(treatment);
    if d.iter().any(|value| *value != 0.0 && *value != 1.0) {
        return Err("Bayesian Gaussian regression needs a 0/1 treatment".to_owned());
    }
    if d.iter().all(|value| *value == d[0]) {
        return Err("Bayesian Gaussian regression treatment must vary".to_owned());
    }
    let y = column(outcome);
    // Non-binary adjustment columns are divided by their population standard deviation so the
    // Normal(0, 1) slope priors are weakly informative regardless of covariate units. The
    // treatment coefficient, and so the effect, is unchanged by this scaling.
    let mut series_by_column: Vec<Vec<f64>> =
        adjustment.iter().map(|&index| column(index)).collect();
    for series in &mut series_by_column {
        if series.iter().all(|value| *value == 0.0 || *value == 1.0) {
            continue;
        }
        let mean = series.iter().sum::<f64>() / rows as f64;
        let sd = (series
            .iter()
            .map(|value| (value - mean).powi(2))
            .sum::<f64>()
            / rows as f64)
            .sqrt();
        if sd > 0.0 {
            for value in series.iter_mut() {
                *value /= sd;
            }
        }
    }
    let covariates: Vec<Vec<f64>> = (0..rows)
        .map(|row| series_by_column.iter().map(|series| series[row]).collect())
        .collect();
    let describe = |error: hirmos_causal_core::bayesian_gaussian::GaussianScmError| {
        format!("Bayesian Gaussian regression failed: {error:?}")
    };
    let model = BayesianGaussianScm::new(d, covariates, y).map_err(describe)?;
    const CHAINS: usize = 3;
    let mut effects = Vec::new();
    let mut sigmas = Vec::new();
    let mut pooled_samples: Vec<Vec<f64>> = Vec::new();
    let mut divergences = 0usize;
    let mut acceptance_sum = 0.0;
    let mut mean_accept_sum = 0.0;
    let mut step_sum = 0.0;
    for chain in 0..CHAINS {
        let options = NutsOptions {
            warmup,
            samples,
            ..NutsOptions::default()
        };
        let result = model.sample_adjustment(options, seed.wrapping_add(chain as u64));
        divergences += result.divergences;
        acceptance_sum += result.acceptance_rate;
        mean_accept_sum += result.mean_accept_probability;
        step_sum += result.step_size;
        effects.extend(
            model
                .adjustment_effect_draws(&result.samples, 0.0, 1.0)
                .map_err(describe)?,
        );
        sigmas.extend(
            result
                .samples
                .iter()
                .map(|draw| draw.last().copied().unwrap_or(0.0).exp()),
        );
        pooled_samples.extend(result.samples);
    }
    let summary = posterior_effect_summary(&effects, 0.94).map_err(describe)?;
    let sigma_mean = sigmas.iter().sum::<f64>() / sigmas.len() as f64;
    // Expected outcome across each covariate under do(0) and do(1), the other covariates at
    // their sample means, summarised per grid point by posterior quantiles across all draws.
    let covariate_count = adjustment.len();
    let column_means: Vec<f64> = (0..covariate_count)
        .map(|index| series_by_column[index].iter().sum::<f64>() / rows as f64)
        .collect();
    let curves: Vec<BayesianGaussianCurve> = (0..covariate_count)
        .map(|target| {
            let series = &series_by_column[target];
            let standardised = !series.iter().all(|value| *value == 0.0 || *value == 1.0);
            let series_low = series.iter().copied().fold(f64::INFINITY, f64::min);
            let series_high = series.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let grid: Vec<f64> = if standardised {
                (0..25)
                    .map(|step| series_low + (series_high - series_low) * step as f64 / 24.0)
                    .collect()
            } else {
                vec![0.0, 1.0]
            };
            let bases: Vec<(f64, f64, f64)> = pooled_samples
                .iter()
                .map(|draw| {
                    let mut base = draw[0];
                    for (index, mean) in column_means.iter().enumerate() {
                        if index != target {
                            base += draw[2 + index] * mean;
                        }
                    }
                    (base, draw[1], draw[2 + target])
                })
                .collect();
            let mut curve = BayesianGaussianCurve {
                standardised,
                grid: grid.clone(),
                control_lower: Vec::new(),
                control_median: Vec::new(),
                control_upper: Vec::new(),
                treated_lower: Vec::new(),
                treated_median: Vec::new(),
                treated_upper: Vec::new(),
            };
            for x in grid {
                let control: Vec<f64> = bases
                    .iter()
                    .map(|(base, _, slope)| base + slope * x)
                    .collect();
                let treated: Vec<f64> = bases
                    .iter()
                    .map(|(base, tau, slope)| base + tau + slope * x)
                    .collect();
                curve.control_lower.push(quantile(&control, 0.03));
                curve.control_median.push(quantile(&control, 0.5));
                curve.control_upper.push(quantile(&control, 0.97));
                curve.treated_lower.push(quantile(&treated, 0.03));
                curve.treated_median.push(quantile(&treated, 0.5));
                curve.treated_upper.push(quantile(&treated, 0.97));
            }
            curve
        })
        .collect();
    let low = effects.iter().copied().fold(f64::INFINITY, f64::min);
    let high = effects.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let bins = 60usize;
    let bin_width = ((high - low) / bins as f64).max(f64::EPSILON);
    let mut counts = vec![0u32; bins];
    for value in &effects {
        let index = (((value - low) / bin_width) as usize).min(bins - 1);
        counts[index] += 1;
    }
    Ok(AnalysisResult::BayesianGaussian {
        observations: rows,
        warmup,
        samples,
        chains: CHAINS,
        seed,
        effect_mean: summary.mean,
        effect_sd: summary.standard_deviation,
        effect_median: summary.median,
        hdi_lower: summary.hdi_lower,
        hdi_upper: summary.hdi_upper,
        probability_positive: summary.probability_positive,
        sigma_mean,
        divergences,
        acceptance_rate: acceptance_sum / CHAINS as f64,
        mean_accept_probability: mean_accept_sum / CHAINS as f64,
        step_size: step_sum / CHAINS as f64,
        histogram_start: low,
        histogram_bin_width: bin_width,
        histogram_counts: counts,
        curves,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn discrete_bn_query(
    values: &[f64],
    rows: usize,
    columns: usize,
    nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    bins: usize,
    equivalent_sample_size: f64,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("discrete BN query", values, rows, columns)?;
    if nodes.len() < 2
        || nodes.len() != names.len()
        || nodes.iter().any(|&column| column >= columns)
    {
        return Err(
            "discrete BN query needs at least two nodes, each with a name and a matrix column"
                .to_owned(),
        );
    }
    if treatment >= nodes.len() || outcome >= nodes.len() || treatment == outcome {
        return Err(
            "discrete BN query treatment and outcome must be distinct node positions".to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= nodes.len() || effect >= nodes.len() || cause == effect)
    {
        return Err("discrete BN query edges must join distinct node positions".to_owned());
    }
    if !(2..=10).contains(&bins) {
        return Err("discrete BN query bins must be between 2 and 10".to_owned());
    }
    if !(equivalent_sample_size > 0.0) || !equivalent_sample_size.is_finite() {
        return Err("discrete BN query equivalent sample size must be positive".to_owned());
    }
    let data = DMatrix::from_column_slice(rows, columns, values);
    let mut frame: HashMap<String, Vec<String>> = HashMap::new();
    let mut means: Vec<std::collections::BTreeMap<String, f64>> = Vec::with_capacity(nodes.len());
    let mut state_counts = Vec::with_capacity(nodes.len());
    for (position, &column) in nodes.iter().enumerate() {
        let series: Vec<f64> = (0..rows).map(|row| data[(row, column)]).collect();
        let discretised = discretize_805(&series, bins).ok_or_else(|| {
            format!(
                "discrete BN query cannot discretise {}: it is constant",
                names[position]
            )
        })?;
        state_counts.push(discretised.means.len());
        frame.insert(names[position].clone(), discretised.labels);
        means.push(discretised.means);
    }
    let named_edges: Vec<(String, String)> = edges
        .iter()
        .map(|&(cause, effect)| (names[cause].clone(), names[effect].clone()))
        .collect();
    let dag = DiscreteDag::new(&named_edges, names);
    let bn = DiscreteBn::fit(dag, &frame, equivalent_sample_size);
    let treatment_name = &names[treatment];
    let outcome_name = &names[outcome];
    let treatment_states = &bn.state_names[treatment_name];
    let low = treatment_states
        .first()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    let high = treatment_states
        .last()
        .cloned()
        .ok_or("discrete BN query treatment has no states")?;
    if low == high {
        return Err(format!(
            "discrete BN query treatment {treatment_name} has a single state after discretisation"
        ));
    }
    let outcome_means = &means[outcome];
    let expectation = |distribution: &HashMap<String, f64>| -> f64 {
        distribution
            .iter()
            .map(|(state, probability)| {
                outcome_means.get(state).copied().unwrap_or(0.0) * probability
            })
            .sum()
    };
    let ordered = |distribution: HashMap<String, f64>| -> Vec<(String, f64)> {
        let mut entries: Vec<(String, f64)> = distribution.into_iter().collect();
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
    };
    let distribution_low = bn.do_query(outcome_name, treatment_name, &low);
    let distribution_high = bn.do_query(outcome_name, treatment_name, &high);
    let expectations = (
        expectation(&distribution_low),
        expectation(&distribution_high),
    );
    let mut parents_adjusted = bn.dag.parents(treatment_name);
    parents_adjusted.sort();
    Ok(AnalysisResult::DiscreteBnQuery {
        observations: rows,
        bins,
        equivalent_sample_size,
        state_counts,
        treatment_states: (low, high),
        expectations,
        effect: expectations.1 - expectations.0,
        distribution_low: ordered(distribution_low),
        distribution_high: ordered(distribution_high),
        minimal_adjustment_set: bn
            .minimal_adjustment_set(treatment_name, outcome_name)
            .map(|set| set.into_iter().collect()),
        parents_adjusted,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn identified_discrete_query(
    values: &[f64],
    rows: usize,
    columns: usize,
    observed_nodes: &[usize],
    names: &[String],
    edges: &[(usize, usize)],
    treatment: usize,
    outcome: usize,
    unobserved: &[usize],
    bins: usize,
    condition: Option<IdentifiedDiscreteCondition>,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("identified discrete query", values, rows, columns)?;
    if names.len() < 2 || !(2..=10).contains(&bins) {
        return Err(
            "identified discrete query needs at least two graph nodes and between 2 and 10 bins"
                .to_owned(),
        );
    }
    if treatment >= names.len() || outcome >= names.len() || treatment == outcome {
        return Err(
            "identified discrete query treatment and outcome must be distinct graph-node positions"
                .to_owned(),
        );
    }
    if edges
        .iter()
        .any(|&(cause, effect)| cause >= names.len() || effect >= names.len() || cause == effect)
        || unobserved.iter().any(|&node| node >= names.len())
    {
        return Err(
            "identified discrete query contains an invalid graph edge or unobserved node"
                .to_owned(),
        );
    }
    let unobserved_set = unobserved
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if unobserved_set.contains(&treatment) || unobserved_set.contains(&outcome) {
        return Err(
            "identified discrete query can only set and read observed variables".to_owned(),
        );
    }
    let observed_positions = (0..names.len())
        .filter(|position| !unobserved_set.contains(position))
        .collect::<Vec<_>>();
    if observed_nodes.len() != observed_positions.len()
        || observed_nodes.iter().any(|&column| column >= columns)
    {
        return Err(
            "identified discrete query needs one matrix column per observed graph node".to_owned(),
        );
    }
    if let Some(condition) = condition {
        if condition.variable >= names.len()
            || condition.variable == treatment
            || condition.variable == outcome
            || unobserved_set.contains(&condition.variable)
        {
            return Err(
                "identified discrete query condition must be a distinct observed variable"
                    .to_owned(),
            );
        }
    }

    let data = DMatrix::from_column_slice(rows, columns, values);
    let mut columns_by_name = BTreeMap::new();
    let mut means_by_position = HashMap::new();
    let mut state_counts = Vec::with_capacity(observed_nodes.len());
    for (&position, &column) in observed_positions.iter().zip(observed_nodes) {
        let series = (0..rows).map(|row| data[(row, column)]).collect::<Vec<_>>();
        let discretised = discretize_805(&series, bins).ok_or_else(|| {
            format!(
                "identified discrete query cannot discretise {}: it is constant",
                names[position]
            )
        })?;
        state_counts.push(discretised.means.len());
        means_by_position.insert(position, discretised.means);
        columns_by_name.insert(names[position].clone(), discretised.labels);
    }
    let table = DiscreteTable::from_columns(columns_by_name).map_err(|error| {
        format!("identified discrete query could not form its observational table: {error}")
    })?;
    let named_edges = edges
        .iter()
        .map(|&(cause, effect)| (names[cause].clone(), names[effect].clone()))
        .collect::<Vec<_>>();
    let latent_names = unobserved
        .iter()
        .map(|&position| names[position].clone())
        .collect::<Vec<_>>();
    let projection = latent_projection(names.iter().cloned(), named_edges, latent_names)
        .map_err(|error| format!("identified discrete query graph is invalid: {error:?}"))?;
    let treatment_name = names[treatment].clone();
    let outcome_name = names[outcome].clone();
    let treatment_states = table.states(&treatment_name).ok_or_else(|| {
        "identified discrete query treatment is absent from the observed table".to_owned()
    })?;
    let low = treatment_states
        .first()
        .cloned()
        .ok_or_else(|| "identified discrete query treatment has no states".to_owned())?;
    let high = treatment_states
        .last()
        .cloned()
        .ok_or_else(|| "identified discrete query treatment has no states".to_owned())?;
    if low == high {
        return Err(format!("identified discrete query treatment {treatment_name} has a single state after discretisation"));
    }

    let (query, condition_name, condition_state) = match condition {
        None => (IdentifiedDiscreteQueryKind::Unconditional, None, None),
        Some(condition) => {
            let name = names[condition.variable].clone();
            let state = condition.state.to_string();
            let means = &means_by_position[&condition.variable];
            let representative_value = means.get(&state).copied().ok_or_else(|| {
                format!("identified discrete query condition state {state} is absent for {name} after discretisation")
            })?;
            (
                IdentifiedDiscreteQueryKind::Conditional {
                    variable: condition.variable,
                    state: state.clone(),
                    representative_value,
                },
                Some(name),
                Some(state),
            )
        }
    };
    let expression = match &condition_name {
        None => identify_outcomes(
            &projection,
            [treatment_name.clone()],
            [outcome_name.clone()],
        ),
        Some(name) => identify_conditional_outcomes(
            &projection,
            [treatment_name.clone()],
            [outcome_name.clone()],
            [name.clone()],
        ),
    };
    let expression = match expression {
        Ok(expression) => expression,
        Err(IdentificationError::Unidentifiable(hedge)) => {
            let index_by_name = names
                .iter()
                .enumerate()
                .map(|(index, name)| (name.as_str(), index))
                .collect::<HashMap<_, _>>();
            return Ok(AnalysisResult::IdentifiedDiscreteQuery {
                observations: rows,
                bins,
                state_counts,
                treatment_states: (low, high),
                query,
                result: IdentifiedDiscreteResult::Unidentifiable {
                    hedge_graph: hedge
                        .graph_district
                        .iter()
                        .map(|name| index_by_name[name.as_str()])
                        .collect(),
                    hedge_subgraph: hedge
                        .treatment_removed_district
                        .iter()
                        .map(|name| index_by_name[name.as_str()])
                        .collect(),
                },
            });
        }
        Err(error) => {
            return Err(format!(
                "identified discrete query could not identify its expression: {error}"
            ))
        }
    };
    let fixed = |treatment_state: String| {
        let mut assignment = BTreeMap::from([(treatment_name.clone(), treatment_state)]);
        if let (Some(name), Some(state)) = (&condition_name, &condition_state) {
            assignment.insert(name.clone(), state.clone());
        }
        assignment
    };
    let low_distribution = evaluate_distribution(
        &expression,
        &table,
        [outcome_name.clone()],
        &fixed(low.clone()),
    )
    .map_err(|error| {
        format!("identified discrete query could not evaluate the low intervention: {error}")
    })?;
    let high_distribution = evaluate_distribution(
        &expression,
        &table,
        [outcome_name.clone()],
        &fixed(high.clone()),
    )
    .map_err(|error| {
        format!("identified discrete query could not evaluate the high intervention: {error}")
    })?;
    let ordered =
        |distribution: &hirmos_causal_core::identified_expression::EvaluatedDistribution| {
            distribution
                .probabilities
                .iter()
                .map(|(states, probability)| (states[0].clone(), *probability))
                .collect::<Vec<_>>()
        };
    let outcome_means = &means_by_position[&outcome];
    let expectation =
        |distribution: &hirmos_causal_core::identified_expression::EvaluatedDistribution| {
            distribution
                .probabilities
                .iter()
                .map(|(states, probability)| outcome_means[&states[0]] * probability)
                .sum::<f64>()
        };
    let expectations = (
        expectation(&low_distribution),
        expectation(&high_distribution),
    );
    Ok(AnalysisResult::IdentifiedDiscreteQuery {
        observations: rows,
        bins,
        state_counts,
        treatment_states: (low, high),
        query,
        result: IdentifiedDiscreteResult::Identified {
            algorithm: if condition_name.is_some() {
                "IDC"
            } else {
                "ID"
            },
            expression: expression.to_y0(),
            latex: expression.to_latex(),
            expectations,
            effect: expectations.1 - expectations.0,
            distribution_low: ordered(&low_distribution),
            distribution_high: ordered(&high_distribution),
            normalization_low: low_distribution.total,
            normalization_high: high_distribution.total,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_binomial_ingarch_facade_matches_the_tscount_forecast() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/ingarch.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let regressors: Vec<Vec<f64>> =
            serde_json::from_value(fixture["regressors"].clone()).unwrap();
        let rows = observations.len();
        let mut values = observations;
        values.extend(regressors.iter().map(|row| row[0]));
        values.extend(regressors.iter().map(|row| row[1]));
        let progress = std::cell::RefCell::new(Vec::new());
        let result = negative_binomial_ingarch(
            &values,
            rows,
            3,
            0,
            IngarchLink::Log,
            &[1, 2],
            &[1],
            &[1],
            &[false, false],
            10,
            &[2.0, 0.15],
            1,
            2.0,
            5.0,
            IngarchInterventionSchedule::Persistent,
            |stage, completed, total| progress.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "negativeBinomialIngarch");
        assert_eq!(encoded["horizon"], 10);
        assert_eq!(encoded["schedule"]["kind"], "persistent");
        let expected = fixture["scenarios"]["persistent"]["mean"]
            .as_array()
            .unwrap();
        for (actual, expected) in encoded["interventionMean"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected)
        {
            assert!((actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() < 1e-8);
        }
        assert_eq!(progress.borrow().last(), Some(&("complete", 2, 2)));
    }

    #[test]
    fn identity_ingarch_facade_matches_tscount_default_forecast() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/ingarch_identity.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let regressors: Vec<Vec<f64>> =
            serde_json::from_value(fixture["regressors"].clone()).unwrap();
        let rows = observations.len();
        let mut values = observations;
        values.extend(regressors.iter().map(|row| row[0]));
        let result = negative_binomial_ingarch(
            &values,
            rows,
            2,
            0,
            IngarchLink::Identity,
            &[1],
            &[1],
            &[1],
            &[false],
            12,
            &[0.0],
            1,
            0.0,
            1.0,
            IngarchInterventionSchedule::Persistent,
            |_, _, _| {},
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["link"], "identity");
        for (actual, expected) in encoded["interventionMean"]
            .as_array()
            .unwrap()
            .iter()
            .zip(fixture["scenarios"]["treated"]["mean"].as_array().unwrap())
        {
            assert!((actual.as_f64().unwrap() - expected.as_f64().unwrap()).abs() < 1e-7);
        }
    }

    #[test]
    fn multi_lag_count_series_scan_matches_tscount_and_reports_progress() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/ingarch_detection.json"
        ))
        .unwrap();
        let observations: Vec<f64> =
            serde_json::from_value(fixture["observations"].clone()).unwrap();
        let candidates: Vec<usize> =
            serde_json::from_value::<Vec<usize>>(fixture["detection"]["candidates"].clone())
                .unwrap()
                .into_iter()
                .map(|tau| tau - 1)
                .collect();
        let rows = observations.len();
        let progress = std::cell::RefCell::new(Vec::new());
        let result = count_series_intervention_scan(
            &observations,
            rows,
            1,
            0,
            IngarchLink::Identity,
            &[1],
            &[7, 13],
            &candidates,
            1.0,
            |stage, completed, total| progress.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "countSeriesInterventionScan");
        assert_eq!(encoded["strongestReferencePoint"].as_u64(), Some(83));
        let scores = encoded["candidates"].as_array().unwrap();
        let oracle = fixture["detection"]["score_statistics"].as_array().unwrap();
        for (actual, expected) in scores.iter().zip(oracle) {
            assert!(
                (actual["scoreStatistic"].as_f64().unwrap() - expected.as_f64().unwrap()).abs()
                    < 1e-7
            );
        }
        assert_eq!(
            progress.borrow().as_slice(),
            &[("fit-and-scan", 0, 1), ("complete", 1, 1)]
        );
    }

    #[test]
    fn backdoor_linear_serializes_both_intervals() {
        let rows = 120;
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform()).collect();
        let t: Vec<f64> = (0..rows).map(|i| 0.8 * w[i] + uniform()).collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 0.5 * t[i] + 0.7 * w[i] + uniform())
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        let json = backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.95)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("estimate should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "backdoorLinear");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["parameters"], 3);
        assert_eq!(value["hacMaxLags"], 4);
        let estimate = value["estimate"].as_f64().unwrap();
        assert!(value["interval"][0].as_f64().unwrap() < estimate);
        assert!(value["hacInterval"][1].as_f64().unwrap() > estimate);
        assert!(value["hacPValue"].as_f64().unwrap() < 0.05);
        assert!(value["durbinWatson"].as_f64().unwrap() > 1.0);

        assert!(backdoor_linear(&values, rows, 3, 0, 0, &[2], None, 0.95).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], Some(rows), 0.95).is_err());
        assert!(backdoor_linear(&values, rows, 3, 0, 1, &[2], None, 0.4).is_err());
    }

    #[test]
    fn frontdoor_two_stage_serializes_stages_interval_and_progress() {
        let rows = 80;
        let treatment: Vec<f64> = (0..rows).map(|row| (row % 7) as f64 - 3.0).collect();
        let mediator: Vec<f64> = treatment
            .iter()
            .enumerate()
            .map(|(row, treatment)| 4.0 + 2.0 * treatment + ((row * 3) % 5) as f64 * 0.01)
            .collect();
        let outcome: Vec<f64> = mediator
            .iter()
            .zip(&treatment)
            .map(|(mediator, treatment)| 8.0 + 3.0 * mediator + 5.0 * treatment)
            .collect();
        let mut values = Vec::with_capacity(rows * 3);
        values.extend(&treatment);
        values.extend(&mediator);
        values.extend(&outcome);
        let mut progress_events = Vec::new();
        let result = frontdoor_two_stage_evidence(
            &values,
            rows,
            3,
            0,
            1,
            2,
            &[],
            &[0],
            0.0,
            1.0,
            FrontdoorUncertainty::Bootstrap {
                simulations: 20,
                sample_size_fraction: 1.0,
                confidence_level: 0.95,
                seed: 0,
            },
            |stage, completed, total| progress_events.push((stage, completed, total)),
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["kind"], "frontdoorTwoStage");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["secondStageAdjustment"], serde_json::json!([0]));
        assert!((value["estimate"].as_f64().unwrap() - 6.0).abs() < 0.02);
        assert_eq!(value["uncertainty"]["kind"], "bootstrap");
        assert_eq!(value["uncertainty"]["simulations"], 20);
        assert_eq!(
            progress_events.first(),
            Some(&("frontdoor-bootstrap", 0, 20))
        );
        assert_eq!(
            progress_events.last(),
            Some(&("frontdoor-bootstrap", 20, 20))
        );

        assert!(frontdoor_two_stage_evidence(
            &values,
            rows,
            3,
            0,
            0,
            2,
            &[],
            &[0],
            0.0,
            1.0,
            FrontdoorUncertainty::None,
            |_, _, _| {},
        )
        .is_err());
    }

    #[test]
    fn count_glm_reports_incidence_rate_ratios_for_both_families() {
        let rows = 150;
        let mut state = 0x1234_5678_9abc_def1u64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let w: Vec<f64> = (0..rows).map(|_| uniform() - 0.5).collect();
        let t: Vec<f64> = (0..rows)
            .map(|i| {
                if uniform() < 0.5 + 0.4 * w[i] {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| (3.0 + 2.0 * t[i] + w[i] + 4.0 * uniform()).floor())
            .collect();
        let mut values = Vec::new();
        values.extend(&t);
        values.extend(&y);
        values.extend(&w);
        for family in [CountFamily::Poisson, CountFamily::NegativeBinomial] {
            let json = count_glm(&values, rows, 3, 0, 1, &[2], family)
                .and_then(|result| {
                    serde_json::to_string(&result).map_err(|error| error.to_string())
                })
                .expect("count model should serialize");
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(value["kind"], "countGlm");
            assert!(value["incidenceRateRatio"].as_f64().unwrap() > 1.0);
            assert!(
                value["incidenceRateRatioInterval"][0].as_f64().unwrap()
                    < value["incidenceRateRatio"].as_f64().unwrap()
            );
            if matches!(family, CountFamily::Poisson) {
                assert_eq!(value["converged"], true);
            }
        }
        let mut fractional = values.clone();
        fractional[rows] = 2.5;
        assert!(count_glm(&fractional, rows, 3, 0, 1, &[2], CountFamily::Poisson).is_err());
    }

    #[test]
    fn causal_effects_total_identifies_and_predicts_a_linear_chain() {
        let rows = 400;
        let mut state = 0xdead_beef_cafe_f00du64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut x = vec![0.0; rows];
        let mut y = vec![0.0; rows];
        let mut z = vec![0.0; rows];
        for i in 1..rows {
            z[i] = 0.5 * z[i - 1] + uniform();
            x[i] = 0.6 * z[i - 1] + uniform();
            y[i] = 0.8 * x[i - 1] + 0.4 * z[i - 1] + uniform();
        }
        let mut values = Vec::new();
        values.extend(&x);
        values.extend(&y);
        values.extend(&z);
        let empty = String::new();
        let mut graph = vec![vec![vec![empty.clone(); 2]; 3]; 3];
        graph[2][2][1] = "-->".to_owned();
        graph[2][0][1] = "-->".to_owned();
        graph[0][1][1] = "-->".to_owned();
        graph[2][1][1] = "-->".to_owned();
        let json = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear,
            [0.0, 1.0],
            CausalEffectsUncertainty::None,
            |_, _, _| {},
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("total effect should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalEffectsTotal");
        assert_eq!(value["identifiable"], true);
        let effect = value["totalEffect"].as_f64().unwrap();
        assert!((effect - 0.8).abs() < 0.15, "total effect {effect}");
        let progress_events = std::cell::RefCell::new(Vec::new());
        let bootstrap_json = causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[(0, -1)],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear,
            [0.0, 1.0],
            CausalEffectsUncertainty::Bootstrap {
                samples: 20,
                block_length: CausalEffectsBlockLength::Fixed { length: 4 },
                confidence_level: 0.9,
                seed: 4,
            },
            |stage, completed, total| progress_events.borrow_mut().push((stage, completed, total)),
        )
        .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
        .expect("bootstrap total effect should serialize");
        let bootstrap: serde_json::Value = serde_json::from_str(&bootstrap_json).unwrap();
        assert_eq!(bootstrap["uncertainty"]["kind"], "bootstrap");
        assert_eq!(bootstrap["uncertainty"]["samples"], 20);
        assert_eq!(bootstrap["uncertainty"]["resolvedBlockLength"], 4);
        assert_eq!(
            bootstrap["uncertainty"]["effectDraws"]
                .as_array()
                .unwrap()
                .len(),
            20
        );
        let events = progress_events.borrow();
        assert_eq!(events.first(), Some(&("causal-effects-bootstrap", 0, 20)));
        assert_eq!(events.last(), Some(&("causal-effects-bootstrap", 20, 20)));
        assert!(causal_effects_total(
            &values,
            rows,
            3,
            1,
            &graph,
            &[],
            &[(1, 0)],
            &[],
            TotalEffectEstimator::Linear,
            [0.0, 1.0],
            CausalEffectsUncertainty::None,
            |_, _, _| {},
        )
        .is_err());
    }

    #[test]
    fn causal_effects_wright_reports_path_decomposition_and_bootstrap() {
        let rows = 180;
        let z: Vec<f64> = (0..rows)
            .map(|row| (row as f64 * 0.31).sin() + (row % 7) as f64 * 0.03)
            .collect();
        let x: Vec<f64> = (0..rows)
            .map(|row| 0.6 * z[row] + (row as f64 * 0.73).cos() * 0.4)
            .collect();
        let mediator: Vec<f64> = (0..rows)
            .map(|row| 0.5 * x[row] + 0.3 * z[row] + (row as f64 * 0.47).sin() * 0.2)
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|row| {
                0.1 * x[row] + 0.4 * mediator[row] + 0.2 * z[row] + (row as f64 * 1.13).cos() * 0.25
            })
            .collect();
        let mut values = Vec::with_capacity(rows * 4);
        values.extend(&x);
        values.extend(&mediator);
        values.extend(&y);
        values.extend(&z);
        let mut graph = vec![vec![vec![String::new(); 1]; 4]; 4];
        for (source, target) in [(3, 0), (3, 1), (3, 2), (0, 1), (1, 2), (0, 2)] {
            graph[source][target][0] = "-->".to_owned();
            graph[target][source][0] = "<--".to_owned();
        }
        let progress_events = std::cell::RefCell::new(Vec::new());
        let result = causal_effects_total(
            &values,
            rows,
            4,
            0,
            &graph,
            &[(0, 0)],
            &[(2, 0)],
            &[],
            TotalEffectEstimator::WrightParents,
            [0.0, 1.0],
            CausalEffectsUncertainty::Bootstrap {
                samples: 20,
                block_length: CausalEffectsBlockLength::Fixed { length: 4 },
                confidence_level: 0.9,
                seed: 19,
            },
            |stage, completed, total| progress_events.borrow_mut().push((stage, completed, total)),
        )
        .unwrap();
        let value = serde_json::to_value(result).unwrap();
        assert_eq!(value["fit"]["kind"], "wrightParents");
        assert_eq!(value["fit"]["coefficients"].as_array().unwrap().len(), 5);
        assert_eq!(value["fit"]["paths"].as_array().unwrap().len(), 2);
        let total = value["totalEffect"].as_f64().unwrap();
        let decomposed = value["fit"]["directEffect"].as_f64().unwrap()
            + value["fit"]["indirectEffect"].as_f64().unwrap();
        assert!((total - decomposed).abs() < 1e-12);
        assert_eq!(value["uncertainty"]["kind"], "bootstrap");
        assert_eq!(
            value["uncertainty"]["effectDraws"]
                .as_array()
                .unwrap()
                .len(),
            20
        );
        let events = progress_events.borrow();
        assert_eq!(
            events.first(),
            Some(&("causal-effects-wright-bootstrap", 0, 20))
        );
        assert_eq!(
            events.last(),
            Some(&("causal-effects-wright-bootstrap", 20, 20))
        );
    }

    #[test]
    fn causal_impact_forecasts_the_post_window() {
        let rows = 120;
        let n_pre = 80;
        let control: Vec<f64> = (0..rows)
            .map(|i| (i as f64 / 9.0).sin() + i as f64 * 0.01)
            .collect();
        let y: Vec<f64> = (0..rows)
            .map(|i| 10.0 + 2.0 * control[i] + if i >= n_pre { 3.0 } else { 0.0 })
            .collect();
        let mut values = Vec::new();
        values.extend(&y);
        values.extend(&control);
        let json = causal_impact_evidence(&values, rows, 2, 0, &[1], n_pre, 100)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("impact should serialize");
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["kind"], "causalImpact");
        assert_eq!(value["nPost"], rows - n_pre);
        assert_eq!(value["pointwise"].as_array().unwrap().len(), rows - n_pre);
        let average = value["average"].as_f64().unwrap();
        assert!((average - 3.0).abs() < 0.5, "average effect {average}");
        assert!(causal_impact_evidence(&values, rows, 2, 0, &[1], 4, 100).is_err());
    }

    /// Two cointegrated random walks: x is a walk and y follows 2x plus stationary noise.
    fn cointegrated_pair(rows: usize) -> (Vec<f64>, Vec<f64>) {
        let mut stream = Mt19937::seeded(11);
        let mut x = Vec::with_capacity(rows);
        let mut level = 0.0;
        for _ in 0..rows {
            level += stream.next_f64() - 0.5;
            x.push(level);
        }
        let y: Vec<f64> = x
            .iter()
            .map(|value| 2.0 * value + 0.3 * (stream.next_f64() - 0.5))
            .collect();
        (x, y)
    }

    #[test]
    fn ardl_reports_the_long_run_effect_and_the_bounds_test() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result =
            serde_json::to_value(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::Ct, 4).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "ardlPss");
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert!(
            result["boundsPUpper"].as_f64().unwrap() < 0.05,
            "bounds p upper {}",
            result["boundsPUpper"]
        );
        assert_eq!(result["boundsCritical"].as_array().unwrap().len(), 4);
        assert!(result["arLag"].as_u64().unwrap() >= 1);
        assert!(ardl_pss(&values, rows, 2, 0, 1, 4, ArdlTrend::C, 4).is_err());
        assert!(ardl_pss(&values, rows, 2, 0, 0, 4, ArdlTrend::Ct, 4).is_err());
    }

    #[test]
    fn vecm_selects_a_rank_and_reports_the_normalised_long_run_relation() {
        let rows = 200;
        let (x, y) = cointegrated_pair(rows);
        let values: Vec<f64> = x.iter().chain(y.iter()).copied().collect();
        let result = serde_json::to_value(
            vecm(
                &values,
                rows,
                2,
                &[1, 0],
                4,
                VecmDeterministic::Co,
                1,
                Some(100),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "vecm");
        assert_eq!(
            result["rank"].as_u64().unwrap(),
            1,
            "rank {}",
            result["rank"]
        );
        let effect = result["longRunEffect"].as_f64().unwrap();
        assert!((effect - 2.0).abs() < 0.3, "long-run effect {effect}");
        assert_eq!(result["beta"].as_array().unwrap().len(), 2);
        assert!(result["chow"].is_array());
        assert!(vecm(&values, rows, 2, &[0], 4, VecmDeterministic::Co, 1, None).is_err());
        assert!(vecm(&values, rows, 2, &[0, 1], 4, VecmDeterministic::Co, 3, None).is_err());
    }

    #[test]
    fn synthetic_control_weights_the_donors_and_reports_the_post_gap() {
        let rows = 40;
        let n_pre = 25;
        let donor_a: Vec<f64> = (0..rows).map(|i| 10.0 + i as f64 * 0.5).collect();
        let donor_b: Vec<f64> = (0..rows).map(|i| 30.0 - i as f64 * 0.2).collect();
        // The treated unit is 0.6 a + 0.4 b before the intervention and jumps by 5 afterwards.
        let treated: Vec<f64> = (0..rows)
            .map(|i| 0.6 * donor_a[i] + 0.4 * donor_b[i] + if i >= n_pre { 5.0 } else { 0.0 })
            .collect();
        let values: Vec<f64> = treated
            .iter()
            .chain(donor_a.iter())
            .chain(donor_b.iter())
            .copied()
            .collect();
        let result = serde_json::to_value(
            synthetic_control(&values, rows, 3, 0, &[1, 2], n_pre, 3, 0.05).unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "syntheticControl");
        let weights: Vec<f64> = result["weights"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64().unwrap())
            .collect();
        assert!(
            (weights[0] - 0.6).abs() < 1e-6 && (weights[1] - 0.4).abs() < 1e-6,
            "weights {weights:?}"
        );
        assert!((result["att"].as_f64().unwrap() - 5.0).abs() < 1e-6);
        assert_eq!(result["synthetic"].as_array().unwrap().len(), rows);
        assert!(synthetic_control(&values, rows, 3, 0, &[0], n_pre, 3, 0.05).is_err());
        assert!(synthetic_control(&values, rows, 3, 0, &[1, 2], rows, 3, 0.05).is_err());
    }

    #[test]
    fn panel_facade_preserves_the_official_california_synthetic_did_result() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/panel_synthdid.json"
        ))
        .unwrap();
        let strings = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        };
        let numbers = |value: &serde_json::Value| {
            value
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item.as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        let units = strings(&fixture["input"]["unit"]);
        let times = numbers(&fixture["input"]["time"])
            .into_iter()
            .map(|time| time as i64)
            .collect::<Vec<_>>();
        let outcome = numbers(&fixture["input"]["outcome"]);
        let treatment = numbers(&fixture["input"]["treatment"]);
        let rows = outcome.len();
        let values = outcome.into_iter().chain(treatment).collect::<Vec<_>>();
        let stages = std::cell::RefCell::new(Vec::new());
        let result = panel_intervention(
            &values,
            rows,
            &units,
            &times,
            12,
            0,
            |stage, completed, total| {
                stages.borrow_mut().push((stage, completed, total));
            },
        )
        .unwrap();
        let encoded = serde_json::to_value(result).unwrap();
        assert_eq!(encoded["kind"], "panelIntervention");
        assert_eq!(encoded["controlUnits"], 38);
        assert_eq!(encoded["treatedUnits"], 1);
        assert_eq!(encoded["nPre"], 19);
        assert_eq!(encoded["nPost"], 12);
        let actual = encoded["syntheticDid"]["estimate"].as_f64().unwrap();
        let expected = fixture["sdid"]["estimate"].as_f64().unwrap();
        assert!(
            (actual - expected).abs() <= 2e-10,
            "{actual} versus {expected}"
        );
        assert_eq!(encoded["syntheticControlPlacebo"]["kind"], "available");
        assert_eq!(encoded["syntheticDidPlacebo"]["kind"], "available");
        assert_eq!(encoded["syntheticControlInTime"]["kind"], "available");
        assert_eq!(encoded["syntheticDidInTime"]["kind"], "available");
        assert_eq!(
            stages.borrow().last(),
            Some(&("synthetic-did-in-time", 9, 9))
        );
    }

    #[test]
    fn negative_binomial_nuts_recovers_a_rate_ratio_above_one() {
        let rows = 120;
        let mut stream = Mt19937::seeded(5);
        let treatment: Vec<f64> = (0..rows).map(|_| stream.next_f64() * 2.0).collect();
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64() - 0.5).collect();
        // Counts whose log mean rises by 0.7 per unit of treatment.
        let outcome: Vec<f64> = (0..rows)
            .map(|i| {
                ((1.0 + 0.7 * treatment[i] + 0.3 * confounder[i]).exp()
                    * (0.75 + 0.5 * stream.next_f64()))
                .round()
            })
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let result =
            serde_json::to_value(negbin_nuts(&values, rows, 3, 0, 1, 2, 150, 300, 9).unwrap())
                .unwrap();
        assert_eq!(result["kind"], "negbinNuts");
        let median = result["irrMedian"].as_f64().unwrap();
        assert!((median.ln() - 0.7).abs() < 0.3, "IRR median {median}");
        assert!(
            result["irrLower"].as_f64().unwrap() < median
                && median < result["irrUpper"].as_f64().unwrap()
        );
        assert!(result["acceptanceRate"].as_f64().unwrap() > 0.5);
        let mut fractional = values.clone();
        fractional[rows] = 0.5;
        assert!(negbin_nuts(&fractional, rows, 3, 0, 1, 2, 150, 300, 9).is_err());
    }

    #[test]
    fn discrete_bn_query_adjusts_for_the_confounder_and_reports_the_do_difference() {
        let rows = 400;
        let mut stream = Mt19937::seeded(21);
        let confounder: Vec<f64> = (0..rows).map(|_| stream.next_f64()).collect();
        let treatment: Vec<f64> = (0..rows)
            .map(|i| confounder[i] + 0.3 * stream.next_f64())
            .collect();
        let outcome: Vec<f64> = (0..rows)
            .map(|i| 2.0 * treatment[i] + 3.0 * confounder[i] + 0.2 * stream.next_f64())
            .collect();
        let values: Vec<f64> = treatment
            .iter()
            .chain(outcome.iter())
            .chain(confounder.iter())
            .copied()
            .collect();
        let names = vec![
            "treatment".to_owned(),
            "outcome".to_owned(),
            "confounder".to_owned(),
        ];
        let edges = vec![(0, 1), (2, 0), (2, 1)];
        let result = serde_json::to_value(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 3, 5.0).unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "discreteBnQuery");
        assert_eq!(result["parentsAdjusted"].as_array().unwrap().len(), 1);
        assert_eq!(result["minimalAdjustmentSet"].as_array().unwrap().len(), 1);
        let effect = result["effect"].as_f64().unwrap();
        let naive = result["expectations"][1].as_f64().unwrap()
            - result["expectations"][0].as_f64().unwrap();
        assert!(
            effect > 0.0 && (effect - naive).abs() < 1e-12,
            "effect {effect}"
        );
        assert_eq!(result["stateCounts"].as_array().unwrap().len(), 3);
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 0, 3, 5.0).is_err()
        );
        assert!(
            discrete_bn_query(&values, rows, 3, &[0, 1, 2], &names, &edges, 0, 1, 1, 5.0).is_err()
        );
    }

    fn complete_binary_columns(variable_count: usize) -> (usize, Vec<f64>) {
        let repeats = 8;
        let rows = (1usize << variable_count) * repeats;
        let mut values = Vec::with_capacity(rows * variable_count);
        for variable in 0..variable_count {
            for pattern in 0..(1usize << variable_count) {
                let value = ((pattern >> variable) & 1) as f64;
                values.extend(std::iter::repeat_n(value, repeats));
            }
        }
        (rows, values)
    }

    #[test]
    fn identified_discrete_query_evaluates_the_frontdoor_id_expression() {
        let (rows, values) = complete_binary_columns(3);
        let names = vec!["U", "X", "M", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 3), (1, 2), (2, 3)],
                1,
                3,
                &[0],
                2,
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "identifiedDiscreteQuery");
        assert_eq!(result["query"]["kind"], "unconditional");
        assert_eq!(result["result"]["kind"], "identified");
        assert_eq!(result["result"]["algorithm"], "ID");
        assert!((result["result"]["normalizationLow"].as_f64().unwrap() - 1.0).abs() < 1e-12);
        assert!((result["result"]["normalizationHigh"].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn identified_discrete_query_evaluates_an_idc_condition() {
        let (rows, values) = complete_binary_columns(3);
        let names = vec!["Z", "X", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[],
                2,
                Some(IdentifiedDiscreteCondition {
                    variable: 0,
                    state: 1,
                }),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["query"]["kind"], "conditional");
        assert_eq!(result["query"]["variable"], 0);
        assert_eq!(result["query"]["state"], "1");
        assert_eq!(result["result"]["kind"], "identified");
        assert_eq!(result["result"]["algorithm"], "IDC");
        assert!((result["result"]["normalizationLow"].as_f64().unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn identified_discrete_query_returns_the_source_hedge_witness() {
        let (rows, values) = complete_binary_columns(2);
        let names = vec!["U", "X", "Y"]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let result = serde_json::to_value(
            identified_discrete_query(
                &values,
                rows,
                2,
                &[0, 1],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[0],
                2,
                None,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["result"]["kind"], "unidentifiable");
        assert!(!result["result"]["hedgeGraph"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(!result["result"]["hedgeSubgraph"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(result["result"].get("effect").is_none());
    }

    #[test]
    fn binary_ett_facade_preserves_the_exact_g_formula_result() {
        let cells = [
            (0.0, 0.0, 0.0, 30),
            (0.0, 0.0, 1.0, 10),
            (0.0, 1.0, 0.0, 5),
            (0.0, 1.0, 1.0, 5),
            (1.0, 0.0, 0.0, 5),
            (1.0, 0.0, 1.0, 5),
            (1.0, 1.0, 0.0, 10),
            (1.0, 1.0, 1.0, 30),
        ];
        let mut z = Vec::new();
        let mut x = Vec::new();
        let mut y = Vec::new();
        for (z_value, x_value, y_value, count) in cells {
            for _ in 0..count {
                z.push(z_value);
                x.push(x_value);
                y.push(y_value);
            }
        }
        let rows = z.len();
        let values = z.into_iter().chain(x).chain(y).collect::<Vec<_>>();
        let names = vec!["Z".to_owned(), "X".to_owned(), "Y".to_owned()];
        let result = serde_json::to_value(
            binary_ett(
                &values,
                rows,
                3,
                &[0, 1, 2],
                &names,
                &[(0, 1), (0, 2), (1, 2)],
                1,
                2,
                &[],
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(result["kind"], "binaryEtt");
        assert!((result["treatedPotentialOutcomeMean"].as_f64().unwrap() - 0.70).abs() < 1e-12);
        assert!((result["untreatedPotentialOutcomeMean"].as_f64().unwrap() - 0.45).abs() < 1e-12);
        assert!((result["effectOnTreated"].as_f64().unwrap() - 0.25).abs() < 1e-12);

        let mut invalid = values;
        invalid[0] = 2.0;
        assert!(binary_ett(
            &invalid,
            rows,
            3,
            &[0, 1, 2],
            &names,
            &[(0, 1), (0, 2), (1, 2)],
            1,
            2,
            &[],
        )
        .is_err());
    }
}
