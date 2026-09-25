//! Graph-adjusted temporal effects: the total effect over a temporal graph, with its bootstrap.

use super::*;

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
    let explicit_adjustment = match &estimator {
        TotalEffectEstimator::Linear { adjustment }
        | TotalEffectEstimator::Knn { adjustment, .. } => match adjustment {
            CausalEffectsAdjustmentSelection::Explicit { nodes } => nodes.as_slice(),
            _ => &[],
        },
        TotalEffectEstimator::WrightParents => &[],
    };
    for &(variable, lag) in x.iter().chain(y) {
        if variable >= columns || lag > 0 {
            return Err(
                "causal effects query nodes must name a variable at a non-positive lag".to_owned(),
            );
        }
    }
    let query_lag = x
        .iter()
        .chain(y)
        .map(|node| node.1.unsigned_abs() as usize)
        .max()
        .unwrap_or(0);
    let tau_max = stat_lag + query_lag;
    for &(variable, lag) in hidden.iter().chain(explicit_adjustment) {
        if variable >= columns || lag > 0 || lag.unsigned_abs() as usize > tau_max {
            return Err(
                "causal effects hidden and adjustment nodes must lie within the projected time window"
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
                    requested: estimator.clone(),
                },
                Vec::new(),
                f64::NAN,
                0,
                CausalEffectsUncertaintyEvidence::None,
            )
        } else {
            match estimator.clone() {
                TotalEffectEstimator::Linear { adjustment }
                | TotalEffectEstimator::Knn { adjustment, .. } => {
                    let selection = match &adjustment {
                        CausalEffectsAdjustmentSelection::Optimal => {
                            AdjustmentSetSelection::Optimal
                        }
                        CausalEffectsAdjustmentSelection::MinimizedOptimal => {
                            AdjustmentSetSelection::MinimizedOptimal
                        }
                        CausalEffectsAdjustmentSelection::CollidersMinimizedOptimal => {
                            AdjustmentSetSelection::CollidersMinimizedOptimal
                        }
                        CausalEffectsAdjustmentSelection::Explicit { nodes } => {
                            AdjustmentSetSelection::Explicit(nodes.clone())
                        }
                    };
                    let set = match effects.resolve_adjustment_set(&selection) {
                        Ok(set) => set,
                        Err(AdjustmentSetError::InvalidExplicitSet { problems }) => {
                            let problems = problems
                                .into_iter()
                                .map(|problem| match problem {
                                    ExplicitAdjustmentProblem::QueryTreatment(node) => {
                                        CausalEffectsAdjustmentProblem::QueryTreatment { node }
                                    }
                                    ExplicitAdjustmentProblem::QueryOutcome(node) => {
                                        CausalEffectsAdjustmentProblem::QueryOutcome { node }
                                    }
                                    ExplicitAdjustmentProblem::LaterTreatmentOccurrence(node) => {
                                        CausalEffectsAdjustmentProblem::LaterTreatmentOccurrence {
                                            node,
                                        }
                                    }
                                    ExplicitAdjustmentProblem::ForbiddenNode(node) => {
                                        CausalEffectsAdjustmentProblem::ForbiddenNode { node }
                                    }
                                    ExplicitAdjustmentProblem::OpenNonCausalPath => {
                                        CausalEffectsAdjustmentProblem::OpenNonCausalPath
                                    }
                                })
                                .collect();
                            return Ok(AnalysisResult::CausalEffectsTotal {
                                observations: rows,
                                tau_max: effects.tau_max,
                                no_causal_path,
                                identifiable: false,
                                mediators,
                                fit: CausalEffectsFitEvidence::InvalidAdjustment {
                                    requested: estimator.clone(),
                                    problems,
                                },
                                interventions,
                                predictions: Vec::new(),
                                total_effect: f64::NAN,
                                fitted_observations: 0,
                                uncertainty: CausalEffectsUncertaintyEvidence::None,
                            });
                        }
                        Err(AdjustmentSetError::NotIdentifiable) => {
                            return Ok(AnalysisResult::CausalEffectsTotal {
                                observations: rows,
                                tau_max: effects.tau_max,
                                no_causal_path,
                                identifiable: false,
                                mediators,
                                fit: CausalEffectsFitEvidence::Unfitted {
                                    requested: estimator.clone(),
                                },
                                interventions,
                                predictions: Vec::new(),
                                total_effect: f64::NAN,
                                fitted_observations: 0,
                                uncertainty: CausalEffectsUncertaintyEvidence::None,
                            });
                        }
                    };
                    let (chosen, fit) = match estimator.clone() {
                        TotalEffectEstimator::Linear { .. } => (
                            Estimator::Linear,
                            CausalEffectsFitEvidence::AdjustedLinear {
                                selection: adjustment.clone(),
                                adjustment_set: set.clone(),
                            },
                        ),
                        TotalEffectEstimator::Knn { k, .. } => (
                            Estimator::KNeighbors { k: k.max(1) },
                            CausalEffectsFitEvidence::AdjustedKnn {
                                k: k.max(1),
                                selection: adjustment.clone(),
                                adjustment_set: set.clone(),
                            },
                        ),
                        TotalEffectEstimator::WrightParents => unreachable!("outer match"),
                    };
                    let (model, uncertainty_evidence) = match uncertainty {
                        CausalEffectsUncertainty::None => {
                            let model = effects
                                .fit_total_effect_with_adjustment_set(
                                    &data,
                                    chosen,
                                    None,
                                    &selection,
                                )
                                .map_err(|error| {
                                    format!("causal effects could not fit the total effect model: {error:?}")
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
                                .fit_bootstrap_total_effect_with_adjustment_set_and_progress(
                                    &data,
                                    chosen,
                                    None,
                                    &selection,
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
        numpy_percentile(&effect_draws, tail),
        numpy_percentile(&effect_draws, 1.0 - tail),
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
