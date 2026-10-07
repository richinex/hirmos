//! Propensity-score estimators: weighting, matching, doubly robust estimation, and the generalised score for a continuous treatment.

use super::*;

#[allow(clippy::too_many_arguments)]
/// In[34] to In[39]: a continuous treatment has no propensity, so the treatment is regressed on
/// the covariates and the conditional Gaussian density at each observed value supplies the weight.
pub(crate) fn continuous_gps(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    scale: GpsWeightScale,
    bootstrap: Option<PropensityBootstrapRequest>,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("continuous GPS", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err("continuous GPS columns must index the numeric matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err("continuous GPS needs distinct treatment, outcome, and adjustment columns".to_owned());
    }
    if adjustment.is_empty() {
        return Err("continuous GPS needs at least one adjustment column".to_owned());
    }
    let column = |index: usize, row: usize| values[index * rows + row];
    let design: Vec<Vec<f64>> = (0..rows)
        .map(|row| adjustment.iter().map(|&at| column(at, row)).collect())
        .collect();
    let doses: Vec<f64> = (0..rows).map(|row| column(treatment, row)).collect();
    let responses: Vec<f64> = (0..rows).map(|row| column(outcome, row)).collect();
    let weighting = match scale {
        GpsWeightScale::InverseDensity => hirmos_causal_core::gps::WeightScale::InverseDensity,
        GpsWeightScale::Stabilized => hirmos_causal_core::gps::WeightScale::Stabilized,
    };

    let scores = hirmos_causal_core::gps::fit(&design, &doses)
        .map_err(|cause| format!("continuous GPS could not fit the treatment model: {cause:?}"))?;
    let estimate = hirmos_causal_core::gps::estimate(&design, &doses, &responses, weighting)
        .map_err(|cause| format!("continuous GPS could not weight the sample: {cause:?}"))?;

    let interval = match bootstrap {
        None => None,
        Some(request) => {
            let (lower, upper) = percentile_tails(&request, "continuous GPS")?;
            let drawn = hirmos_causal_core::gps::bootstrap_interval(
                &design, &doses, &responses, weighting,
                request.rounds, request.seed, lower, upper,
            )
            .map_err(|cause| format!("continuous GPS could not resample: {cause:?}"))?;
            Some(PropensityIntervalEvidence {
                lower: drawn.lower, upper: drawn.upper, rounds: request.rounds,
                level: request.level, unconverged: Vec::new(),
            })
        }
    };
    Ok(AnalysisResult::ContinuousGps {
        observations: rows,
        estimate: estimate.effect,
        intercept: estimate.intercept,
        standard_error: estimate.standard_error,
        weight_sum: estimate.weight_sum,
        weights: estimate.weights,
        density: scores.density,
        residual_scale: scores.residual_scale,
        treatment_params: scores.treatment_params,
        interval,
    })
}

/// Validation shared by the propensity commands: a dense matrix, distinct columns that index it,
type PropensityInputs = (Vec<Vec<f64>>, Vec<bool>, Vec<f64>);

/// a non-empty adjustment set and a 0/1 treatment. Returns the design and the arms.
fn propensity_inputs(
    label: &str,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
) -> Result<PropensityInputs, String> {
    validate_dense_matrix(label, values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err(format!("{label} columns must index the numeric matrix"));
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err(format!(
            "{label} needs distinct treatment, outcome, and adjustment columns"
        ));
    }
    if adjustment.is_empty() {
        return Err(format!("{label} needs at least one adjustment column"));
    }
    let column = |index: usize, row: usize| values[index * rows + row];
    if (0..rows).any(|row| {
        let value = column(treatment, row);
        value != 0.0 && value != 1.0
    }) {
        return Err(format!("{label} needs a 0/1 treatment column"));
    }
    Ok((
        (0..rows)
            .map(|row| adjustment.iter().map(|&at| column(at, row)).collect())
            .collect(),
        (0..rows).map(|row| column(treatment, row) > 0.5).collect(),
        (0..rows).map(|row| column(outcome, row)).collect(),
    ))
}

fn treatment_model_of(model: LogisticModel) -> hirmos_causal_core::propensity::TreatmentModel {
    match model {
        LogisticModel::Newton => {
            hirmos_causal_core::propensity::TreatmentModel::Newton(Default::default())
        }
        LogisticModel::Lbfgsb { max_iter } => {
            hirmos_causal_core::propensity::TreatmentModel::Lbfgsb { max_iter }
        }
    }
}

fn boosted_grid(model: &BoostedTreatmentModel) -> hirmos_causal_core::model_selection::Grid {
    hirmos_causal_core::model_selection::Grid {
        learning_rate: model.learning_rate.clone(),
        max_depth: model.max_depth.clone(),
        n_estimators: model.n_estimators.clone(),
    }
}

fn boosted_scores(
    label: &str,
    design: &[Vec<f64>],
    treated: &[bool],
    model: &BoostedTreatmentModel,
) -> Result<(Vec<f64>, TreatmentModelEvidence), String> {
    use hirmos_causal_core::model_selection::candidates;
    let grid = boosted_grid(model);
    let assignment: Vec<f64> = treated.iter().map(|&t| f64::from(t)).collect();
    let labels: Vec<u8> = treated.iter().map(|&t| u8::from(t)).collect();
    let total = candidates(&grid).len();

    let fitted_auc = |scores: &[f64]| hirmos_causal_core::metrics::roc_auc(&labels, scores);
    let (propensity, chosen, scoring) = match model.scoring {
        BoostedScoring::CrossFitted => {
            let fitted = hirmos_causal_core::crossfit::cross_fitted_propensity(
                design, &assignment, &grid, model.splits,
                model.min_samples_leaf, model.min_samples_split, model.seed,
            )
            .map_err(|cause| format!("{label} could not cross-fit the treatment model: {cause:?}"))?;
            let mut ordered = vec![f64::NAN; design.len()];
            for (at, &row) in fitted.rows.iter().enumerate() {
                ordered[row] = fitted.scores[at];
            }
            if ordered.iter().any(|score| score.is_nan()) {
                return Err(format!("{label} cross-fitting left a row unscored"));
            }
            let auc = fitted_auc(&ordered);
            (ordered, fitted.selected[0], BoostedScoringEvidence::CrossFitted { auc })
        }
        BoostedScoring::OneModel => {
            let search = hirmos_causal_core::model_selection::grid_search(
                design, &assignment, &grid, model.splits,
                model.min_samples_leaf, model.min_samples_split, model.seed,
            )
            .map_err(|cause| format!("{label} could not search the tree grid: {cause:?}"))?;
            let fitted = hirmos_causal_core::gradient_boosting::GradientBoostingClassifier::fit(
                design,
                &assignment,
                &hirmos_causal_core::gradient_boosting::Options {
                    n_estimators: search.best.n_estimators,
                    learning_rate: search.best.learning_rate,
                    max_depth: search.best.max_depth,
                    min_samples_leaf: model.min_samples_leaf,
                    min_samples_split: model.min_samples_split,
                    random_state: model.seed,
                },
            )
            .map_err(|cause| format!("{label} could not fit the chosen candidate: {cause:?}"))?;
            let scores = fitted
                .predict_probability(design)
                .map_err(|cause| format!("{label} could not score the rows: {cause:?}"))?;
            let evidence = BoostedScoringEvidence::OneModel { validation_auc: search.best_score, fitted_auc: fitted_auc(&scores) };
            (scores, search.best, evidence)
        }
    };

    Ok((
        propensity,
        TreatmentModelEvidence::Boosted {
            learning_rate: chosen.learning_rate,
            max_depth: chosen.max_depth,
            n_estimators: chosen.n_estimators,
            candidates: model.candidates_searched.unwrap_or(total),
            scoring,
        },
    ))
}

fn propensity_scores(
    label: &str,
    design: &[Vec<f64>],
    treated: &[bool],
    model: &PropensityModel,
) -> Result<(Vec<f64>, TreatmentModelEvidence), String> {
    match model {
        PropensityModel::Logistic { model } => {
            let scores = hirmos_causal_core::propensity::fit(
                design, DESIGN_INTERCEPT, treated, treatment_model_of(*model),
            )
            .map_err(|cause| format!("{label} could not fit the treatment model: {cause:?}"))?;
            let evidence = TreatmentModelEvidence::Logistic {
                parameters: design[0].len() + 1,
                converged: scores.fitted.converged(),
            };
            Ok((scores.propensity, evidence))
        }
        PropensityModel::Boosted { model } => boosted_scores(label, design, treated, model),
    }
}

/// The adjustment set holds covariates only, so the fit supplies the constant.
const DESIGN_INTERCEPT: hirmos_causal_core::propensity::Intercept =
    hirmos_causal_core::propensity::Intercept::Absent;

fn percentile_tails(request: &PropensityBootstrapRequest, label: &str) -> Result<(f64, f64), String> {
    if !(0.0..1.0).contains(&request.level) || request.level <= 0.5 {
        return Err(format!("{label} interval level must lie in (0.5, 1)"));
    }
    if request.rounds < 2 {
        return Err(format!("{label} needs at least two bootstrap rounds"));
    }
    let tail = (1.0 - request.level) / 2.0;
    Ok((tail, 1.0 - tail))
}

/// One slice of the boosted grid, so the slices run in parallel workers. The unit is a single
/// learning rate and depth over every tree count, which is what the prefix scoring already fits
/// once and scores at each count, so slicing changes no arithmetic.
pub(crate) fn propensity_grid_slice(
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    learning_rate: f64,
    max_depth: usize,
    n_estimators: Vec<usize>,
    splits: usize,
    min_samples_leaf: usize,
    min_samples_split: usize,
    seed: u32,
) -> Result<AnalysisResult, String> {
    let (design, treated, _) = propensity_inputs(
        "propensity grid slice", values, rows, columns, treatment, outcome, adjustment,
    )?;
    let assignment: Vec<f64> = treated.iter().map(|&t| f64::from(t)).collect();
    let grid = hirmos_causal_core::model_selection::Grid {
        learning_rate: vec![learning_rate],
        max_depth: vec![max_depth],
        n_estimators,
    };
    let search = hirmos_causal_core::model_selection::grid_search(
        &design, &assignment, &grid, splits, min_samples_leaf, min_samples_split, seed,
    )
    .map_err(|cause| format!("propensity grid slice could not score the candidates: {cause:?}"))?;
    Ok(AnalysisResult::PropensityGridSlice {
        scores: search
            .scores
            .into_iter()
            .map(|entry| GridCandidateScore {
                learning_rate: entry.candidate.learning_rate,
                max_depth: entry.candidate.max_depth,
                n_estimators: entry.candidate.n_estimators,
                mean_score: entry.mean_score,
            })
            .collect(),
    })
}

/// In[10]: one nearest opposite-arm neighbour for every row, averaged over the whole sample, so
/// the estimand is the average effect rather than the effect on the treated.
pub(crate) fn propensity_matching(
    target: PropensityTarget,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: PropensityModel,
) -> Result<AnalysisResult, String> {
    let (design, treated, responses) = propensity_inputs(
        "propensity matching", values, rows, columns, treatment, outcome, adjustment,
    )?;
    let (propensity, treatment_model) =
        propensity_scores("propensity matching", &design, &treated, &model)?;
    let matched =
        hirmos_causal_core::propensity::match_on_score_target(&responses, &treated, &propensity, target.kernel())
            .map_err(|cause| format!("propensity matching could not pair the arms: {cause:?}"))?;
    let identities: Vec<f64> = (0..rows).map(|r| r as f64).collect();
    let paired = hirmos_causal_core::propensity::match_on_score_target(&identities, &treated, &propensity, target.kernel())
        .map_err(|_| "The matched row identities could not be recovered.".to_owned())?;
    let mut weights = vec![0.0; rows];
    for row in 0..rows {
        if matches!(target, PropensityTarget::Ate) || treated[row] {
            weights[row] += 1.0;
            weights[paired.matches[row] as usize] += 1.0;
        }
    }
    let balance = propensity_balance(&design, &treated, &weights, target);
    let treated_rows = treated.iter().filter(|&&t| t).count();
    Ok(AnalysisResult::PropensityMatching {
        balance,
        target,
        observations: rows,
        treatment_model,
        treated_rows,
        control_rows: rows - treated_rows,
        estimate: matched.effect,
        propensity,
        treated,
        matches: matched.matches,
    })
}

/// In[21]: the propensity on one side and a per-arm outcome regression on the other, so only one
/// of the two models has to be right.
pub(crate) fn doubly_robust_estimate(
    target: PropensityTarget,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    model: LogisticModel,
    bootstrap: Option<PropensityBootstrapRequest>,
) -> Result<AnalysisResult, String> {
    let (design, treated, responses) = propensity_inputs(
        "doubly robust estimate", values, rows, columns, treatment, outcome, adjustment,
    )?;
    let treatment_model = treatment_model_of(model);
    let estimate = hirmos_causal_core::propensity::doubly_robust_target(
        &design, &responses, &treated, DESIGN_INTERCEPT, treatment_model, target.kernel(),
    )
    .map_err(|cause| format!("doubly robust estimate could not fit: {cause:?}"))?;
    let scores = hirmos_causal_core::propensity::fit(
        &design, DESIGN_INTERCEPT, &treated, treatment_model,
    )
    .map_err(|cause| format!("doubly robust estimate could not fit the treatment model: {cause:?}"))?;

    let interval = match bootstrap {
        None => None,
        Some(request) => {
            let (lower, upper) = percentile_tails(&request, "doubly robust estimate")?;
            let drawn = hirmos_causal_core::propensity::bootstrap_interval(
                &design, &responses, &treated, DESIGN_INTERCEPT,
                match target { PropensityTarget::Ate => hirmos_causal_core::propensity::Estimand::DoublyRobust,
                    PropensityTarget::Att => hirmos_causal_core::propensity::Estimand::DoublyRobustAtt },
                request.rounds, request.seed, treatment_model, lower, upper,
            )
            .map_err(|cause| format!("doubly robust estimate could not resample: {cause:?}"))?;
            Some(PropensityIntervalEvidence {
                lower: drawn.lower, upper: drawn.upper, rounds: request.rounds,
                level: request.level, unconverged: drawn.unconverged,
            })
        }
    };
    let weights: Vec<f64> = scores.propensity.iter().zip(&treated).map(|(&p, &t)| {
        match (target, t) {
            (PropensityTarget::Att, true) => 1.0,
            (PropensityTarget::Att, false) => p / (1.0 - p),
            (PropensityTarget::Ate, true) => 1.0 / p,
            (PropensityTarget::Ate, false) => 1.0 / (1.0 - p),
        }
    }).collect();
    let balance = propensity_balance(&design, &treated, &weights, target);
    Ok(AnalysisResult::DoublyRobust {
        balance,
        target,
        observations: rows,
        parameters: design[0].len() + 1,
        estimate: estimate.effect,
        treated_term: estimate.treated_term,
        control_term: estimate.control_term,
        propensity: scores.propensity,
        treated,
        converged: estimate.fitted.converged(),
        interval,
    })
}

/// The weighting estimator: fit the propensity on the adjustment set, then average
/// the potential-outcome totals over every row. The design is the caller's, so a covariate that
/// entered as indicators arrives as separate columns.
pub(crate) fn propensity_weighting(
    target: PropensityTarget,
    values: &[f64],
    rows: usize,
    columns: usize,
    treatment: usize,
    outcome: usize,
    adjustment: &[usize],
    scale: PropensityWeightScale,
    fit: WeightingFit,
) -> Result<AnalysisResult, String> {
    validate_dense_matrix("propensity weighting", values, rows, columns)?;
    let mut used = vec![treatment, outcome];
    used.extend_from_slice(adjustment);
    if used.iter().any(|&column| column >= columns) {
        return Err("propensity weighting columns must index the numeric matrix".to_owned());
    }
    let mut distinct = used.clone();
    distinct.sort_unstable();
    distinct.dedup();
    if distinct.len() != used.len() {
        return Err(
            "propensity weighting needs distinct treatment, outcome, and adjustment columns"
                .to_owned(),
        );
    }
    if adjustment.is_empty() {
        return Err("propensity weighting needs at least one adjustment column".to_owned());
    }
    let column = |index: usize, row: usize| values[index * rows + row];
    let treated: Vec<bool> = (0..rows).map(|row| column(treatment, row) > 0.5).collect();
    if (0..rows).any(|row| {
        let value = column(treatment, row);
        value != 0.0 && value != 1.0
    }) {
        return Err("propensity weighting needs a 0/1 treatment column".to_owned());
    }
    let responses: Vec<f64> = (0..rows).map(|row| column(outcome, row)).collect();
    let design: Vec<Vec<f64>> = (0..rows)
        .map(|row| adjustment.iter().map(|&at| column(at, row)).collect())
        .collect();

    let specification = hirmos_causal_core::propensity::Specification {
        normalization: match target {
            PropensityTarget::Ate => hirmos_causal_core::propensity::Normalization::HorvitzThompson,
            PropensityTarget::Att => hirmos_causal_core::propensity::Normalization::Hajek,
        },
        scale: match scale {
            PropensityWeightScale::InverseProbability => {
                hirmos_causal_core::propensity::WeightScale::InverseProbability
            }
            PropensityWeightScale::Stabilized => {
                hirmos_causal_core::propensity::WeightScale::Stabilized
            }
        },
    };

    let model = match &fit {
        WeightingFit::Logistic { model, .. } => PropensityModel::Logistic { model: *model },
        WeightingFit::Boosted { model } => PropensityModel::Boosted { model: model.clone() },
    };
    let (propensity, treatment_model) =
        propensity_scores("propensity weighting", &design, &treated, &model)?;
    let estimate =
        hirmos_causal_core::propensity::weighted_effect(&responses, &treated, &propensity, specification, target.kernel())
            .map_err(|cause| format!("propensity weighting could not weight the sample: {cause:?}"))?;

    let interval = match &fit {
        WeightingFit::Boosted { .. } => None,
        WeightingFit::Logistic { bootstrap: None, .. } => None,
        WeightingFit::Logistic { model, bootstrap: Some(request) } => {
            let (low, high) = percentile_tails(request, "propensity weighting")?;
            let drawn = hirmos_causal_core::propensity::bootstrap_interval(
                &design,
                &responses,
                &treated,
                DESIGN_INTERCEPT,
                match target { PropensityTarget::Ate => hirmos_causal_core::propensity::Estimand::InverseProbability(specification),
                    PropensityTarget::Att => hirmos_causal_core::propensity::Estimand::InverseProbabilityAtt(specification) },
                request.rounds,
                request.seed,
                treatment_model_of(*model),
                low,
                high,
            )
            .map_err(|cause| format!("propensity weighting could not resample: {cause:?}"))?;
            Some(PropensityIntervalEvidence {
                lower: drawn.lower,
                upper: drawn.upper,
                rounds: request.rounds,
                level: request.level,
                unconverged: drawn.unconverged,
            })
        }
    };

    Ok(AnalysisResult::PropensityWeighting {
        target,
        observations: rows,
        treatment_model,
        treated_rows: estimate.treated_rows,
        control_rows: estimate.control_rows,
        estimate: estimate.effect,
        treated_mean: estimate.treated_mean,
        control_mean: estimate.control_mean,
        treated_weight_sum: estimate.treated_weight_sum,
        control_weight_sum: estimate.control_weight_sum,
        propensity,
        balance: propensity_balance(&design, &treated, &estimate.weights, target),
        treated,
        weights: estimate.weights,
        outcome: responses,
        interval,
    })
}

fn propensity_balance(design: &[Vec<f64>], treated: &[bool], weights: &[f64], target: PropensityTarget) -> CovariateBalanceEvidence {
    use hirmos_causal_core::covariate_balance::{balance, CovariateKind, Denominator, Scale};
    let (reference, denominator) = match target {
        PropensityTarget::Ate => ("pooled", Denominator::Pooled),
        PropensityTarget::Att => ("treated", Denominator::Treated),
    };
    let mut rows = Vec::new();
    for column in 0..design[0].len() {
        let values: Vec<_> = design.iter().map(|row| row[column]).collect();
        let kind = if values.iter().all(|&v| v == 0.0 || v == 1.0) { CovariateKind::Binary } else { CovariateKind::Continuous };
        match balance(&values, treated, Some(weights), kind, Scale::Standardized(denominator)) {
            Ok(result) => rows.push(CovariateBalanceRow { column, before: result.before, after: result.after.expect("weights supplied"), denominator: result.denominator, used_full_sample_spread: result.used_full_sample_spread }),
            Err(_) => return CovariateBalanceEvidence::Unavailable { reason: "Balance could not be calculated for these covariates and weights. The effect estimate is unchanged.".to_owned() },
        }
    }
    CovariateBalanceEvidence::Available { reference: reference.to_owned(), rows }
}
