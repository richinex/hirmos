//! Cross-sectional and time-series discovery façades.

use super::*;
use hirmos_causal_core::constraint_discovery::{
    BackgroundKnowledge, CrossSectionalCi, EndpointGraph,
};
use hirmos_causal_core::fci::{
    run_fci_with_progress, Directness, FciConfiguration, LatentConfounding,
};
use hirmos_causal_core::pc_stable::{run_pc_stable_with_progress, PcStableConfiguration};

fn role_aware_policy(
    cut_off: TemporalCutOff,
    propagate_through_max_lag: bool,
    mask_type: TemporalMaskType,
) -> RoleAwareSamplePolicy {
    let cut_off = match cut_off {
        TemporalCutOff::MethodDefault | TemporalCutOff::TwoTauMax => CutOff::TwoTauMax,
        TemporalCutOff::TauMax => CutOff::TauMax,
        TemporalCutOff::MaxLag => CutOff::MaxLag,
        TemporalCutOff::MaxLagOrTauMax => CutOff::MaxLagOrTauMax,
        TemporalCutOff::TwoTauMaxFuture => CutOff::TwoTauMaxFuture,
    };
    let mask_type = match mask_type {
        TemporalMaskType::None => MaskType::NONE,
        TemporalMaskType::X => MaskType::X,
        TemporalMaskType::Y => MaskType::Y,
        TemporalMaskType::Z => MaskType::Z,
        TemporalMaskType::Xy => MaskType::XY,
        TemporalMaskType::Xz => MaskType::XZ,
        TemporalMaskType::Yz => MaskType::YZ,
        TemporalMaskType::Xyz => MaskType::XYZ,
    };
    RoleAwareSamplePolicy {
        cut_off,
        remove_missing_upto_maxlag: propagate_through_max_lag,
        mask_type,
    }
}

fn role_aware_frame(
    values: &[f64],
    validity: &[u8],
    analysis_mask: &[u8],
    rows: usize,
    columns: usize,
) -> Result<TigramiteFrame, String> {
    let cells = rows.saturating_mul(columns);
    if values.len() != cells || validity.len() != cells || analysis_mask.len() != cells {
        return Err(
            "role-aware values, validity, and analysis mask must have identical matrix shapes"
                .to_owned(),
        );
    }
    if validity.iter().chain(analysis_mask).any(|&value| value > 1) {
        return Err("validity and analysis-mask cells must be encoded as 0 or 1".to_owned());
    }
    let matrix: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let valid: Vec<Vec<bool>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| validity[column * rows + row] == 1)
                .collect()
        })
        .collect();
    let mask: Vec<Vec<bool>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| analysis_mask[column * rows + row] == 1)
                .collect()
        })
        .collect();
    TigramiteFrame::from_validity(matrix, valid, Some(mask))
        .map_err(|problem| format!("role-aware sample matrix refused: {problem}"))
}

fn causal_ts_context(value: CdnotsContext) -> Option<CausalTsContextPreset> {
    match value {
        CdnotsContext::None => None,
        CdnotsContext::Linear => Some(CausalTsContextPreset::Linear),
        CdnotsContext::LinearSine => Some(CausalTsContextPreset::LinearSine),
        CdnotsContext::LinearExponential => Some(CausalTsContextPreset::LinearExponential),
        CdnotsContext::LinearQuadratic => Some(CausalTsContextPreset::LinearQuadratic),
        CdnotsContext::Step => Some(CausalTsContextPreset::Step),
        CdnotsContext::StepLinear => Some(CausalTsContextPreset::StepLinear),
    }
}

fn causal_ts_context_names(value: CdnotsContext) -> Vec<String> {
    match value {
        CdnotsContext::None => vec![],
        CdnotsContext::Linear => vec!["C_lin".to_owned()],
        CdnotsContext::LinearSine => vec!["C_lin".to_owned(), "C_sin".to_owned()],
        CdnotsContext::LinearExponential => vec!["C_lin".to_owned(), "C_exp".to_owned()],
        CdnotsContext::LinearQuadratic => vec!["C_lin".to_owned(), "C_quad".to_owned()],
        CdnotsContext::Step => vec!["C_step".to_owned()],
        CdnotsContext::StepLinear => vec!["C_step".to_owned(), "C_lin".to_owned()],
    }
}

fn causal_ts_rows(
    values: &[f64],
    validity: &[u8],
    rows: usize,
    columns: usize,
) -> Result<Vec<Vec<Option<f64>>>, String> {
    let cells = rows.saturating_mul(columns);
    if values.len() != cells || (!validity.is_empty() && validity.len() != cells) {
        return Err("causal-ts values and validity must have identical matrix shapes".to_owned());
    }
    if validity.iter().any(|&value| value > 1) {
        return Err("causal-ts validity cells must be encoded as 0 or 1".to_owned());
    }

    Ok((0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| {
                    let index = column * rows + row;
                    if validity.is_empty() || validity[index] == 1 {
                        values[index].is_finite().then_some(values[index])
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect())
}

fn cdnots_missing(value: CdnotsMissingStrategy) -> CdnotsMissingPolicy {
    match value {
        CdnotsMissingStrategy::PairwiseComplete => CdnotsMissingPolicy::PairwiseComplete {
            minimum_samples: 30,
        },
        CdnotsMissingStrategy::VarEm => CdnotsMissingPolicy::VarEm(VarEmConfiguration {
            lags: 1,
            maximum_iterations: 50,
            tolerance: 1e-4,
        }),
    }
}

fn endpoint_mark(left: i8, right: i8) -> Option<String> {
    if left == 0 && right == 0 {
        return None;
    }
    let left = match left {
        -1 => '-',
        1 => '<',
        _ => 'x',
    };
    let right = match right {
        -1 => '-',
        1 => '>',
        _ => 'x',
    };
    Some(format!("{left}-{right}"))
}

struct CdnotsMatrices {
    graph: Vec<Vec<Vec<String>>>,
    p_matrix: Vec<Vec<Vec<f64>>>,
    val_matrix: Vec<Vec<Vec<f64>>>,
}

fn cdnots_matrices(
    result: &hirmos_causal_core::cdnots::CdnotsResult,
    variables: usize,
    max_lag: usize,
) -> CdnotsMatrices {
    let mut graph = vec![vec![vec![String::new(); max_lag + 1]; variables]; variables];
    let mut p_matrix = vec![vec![vec![1.0; max_lag + 1]; variables]; variables];
    let mut val_matrix = vec![vec![vec![0.0; max_lag + 1]; variables]; variables];

    for source in 0..variables {
        for target in 0..variables {
            if source != target {
                if let Some(mark) =
                    endpoint_mark(result.graph[source][target], result.graph[target][source])
                {
                    graph[source][target][0] = mark;
                }
            }
            for lag in 1..=max_lag {
                let lagged_source = lag * variables + source;
                if let Some(mark) = endpoint_mark(
                    result.graph[lagged_source][target],
                    result.graph[target][lagged_source],
                ) {
                    graph[source][target][lag] = mark;
                }
            }
        }
    }

    for test in &result.pvalues {
        let first_lag = test.first / variables;
        let second_lag = test.second / variables;
        let mapped = if first_lag == 0 && second_lag <= max_lag {
            Some((test.second % variables, test.first % variables, second_lag))
        } else if second_lag == 0 && first_lag <= max_lag {
            Some((test.first % variables, test.second % variables, first_lag))
        } else {
            None
        };
        let Some((source, target, lag)) = mapped else {
            continue;
        };
        if test.p_value <= p_matrix[source][target][lag] {
            p_matrix[source][target][lag] = test.p_value;
            val_matrix[source][target][lag] = test.statistic.clamp(-1.0, 1.0);
        }
        if lag == 0 && test.p_value <= p_matrix[target][source][0] {
            p_matrix[target][source][0] = test.p_value;
            val_matrix[target][source][0] = test.statistic.clamp(-1.0, 1.0);
        }
    }

    CdnotsMatrices {
        graph,
        p_matrix,
        val_matrix,
    }
}

pub(crate) fn cdnots_evidence<F>(
    values: &[f64],
    validity: &[u8],
    rows: usize,
    columns: usize,
    max_lag: usize,
    alpha: f64,
    missing: CdnotsMissingStrategy,
    context: CdnotsContext,
    plus: bool,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=32).contains(&columns) || !(1..=20).contains(&max_lag) {
        return Err("CD-NOTS requires 2–32 variables and a maximum lag from 1 to 20".to_owned());
    }
    let input = causal_ts_rows(values, validity, rows, columns)?;
    let context_preset = causal_ts_context(context);
    let result = if plus {
        run_cdnots_plus_with_progress(
            &input,
            CdnotsPlusConfiguration {
                max_lag,
                alpha,
                max_degree: None,
                max_conds_y: None,
                max_conds_x: None,
                priority: ColliderPriority::MarkConflict,
                missing: cdnots_missing(missing),
                context: context_preset,
                legacy_mci_conditions: false,
                orient_margin: 0.1,
            },
            progress,
        )
    } else {
        run_cdnots_with_progress(
            &input,
            CdnotsConfiguration {
                max_lag,
                alpha,
                max_degree: None,
                max_combinations: Some(20),
                priority: ColliderPriority::KeepFirst,
                missing: cdnots_missing(missing),
                context: context_preset,
                orient_margin: 0.0,
            },
            progress,
        )
    }
    .map_err(|problem| format!("CD-NOTS refused: {problem}"))?;
    let context_variables = causal_ts_context_names(context);
    let variables = columns + context_variables.len();
    let matrices = cdnots_matrices(&result, variables, max_lag);
    let common = (
        rows,
        columns,
        context_variables,
        max_lag,
        alpha,
        missing,
        context,
        matrices,
    );
    Ok(if plus {
        AnalysisResult::CdnotsPlus {
            observations: common.0,
            observed_variables: common.1,
            context_variables: common.2,
            max_lag: common.3,
            alpha: common.4,
            missing: common.5,
            context: common.6,
            graph: common.7.graph,
            p_matrix: common.7.p_matrix,
            val_matrix: common.7.val_matrix,
        }
    } else {
        AnalysisResult::Cdnots {
            observations: common.0,
            observed_variables: common.1,
            context_variables: common.2,
            max_lag: common.3,
            alpha: common.4,
            missing: common.5,
            context: common.6,
            graph: common.7.graph,
            p_matrix: common.7.p_matrix,
            val_matrix: common.7.val_matrix,
        }
    })
}

pub(crate) fn grace_evidence<F>(
    values: &[f64],
    validity: &[u8],
    rows: usize,
    columns: usize,
    max_lag: usize,
    alpha: f64,
    context: CdnotsContext,
    gate_threshold: f64,
    epochs: usize,
    patience: usize,
    seed: u64,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    let nullable = causal_ts_rows(values, validity, rows, columns)?;
    let missing_cells = nullable
        .iter()
        .flatten()
        .filter(|value| value.is_none())
        .count();
    let imputation = causal_ts_var_em(
        &nullable,
        VarEmConfiguration {
            lags: 1,
            maximum_iterations: 50,
            tolerance: 1e-4,
        },
    )
    .map_err(|problem| format!("GRACE VAR-EM preparation refused: {problem}"))?;
    progress("imputation", 1, 3);
    let dense = imputation
        .values
        .iter()
        .map(|row| row.iter().copied().map(Some).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let skeleton_result = run_cdnots_with_progress(
        &dense,
        CdnotsConfiguration {
            max_lag,
            alpha,
            max_degree: None,
            max_combinations: Some(20),
            priority: ColliderPriority::KeepFirst,
            missing: CdnotsMissingPolicy::PairwiseComplete {
                minimum_samples: 30,
            },
            context: causal_ts_context(context),
            orient_margin: 0.0,
        },
        |_, completed, total| progress("skeleton", completed, total),
    )
    .map_err(|problem| format!("GRACE skeleton refused: {problem}"))?;
    let all_variables = skeleton_result.time_graph.len();
    let lags = max_lag + 1;
    let mut skeleton = vec![0_u8; columns * columns * lags];
    for source in 0..columns {
        for target in 0..columns {
            for lag in 0..lags {
                skeleton[(source * columns + target) * lags + lag] =
                    skeleton_result.time_graph[source][target][lag];
            }
        }
    }
    debug_assert!(all_variables >= columns);
    let windows = prepare_grace_dense(&imputation.values, max_lag, true)
        .map_err(|problem| format!("GRACE dense preparation refused: {problem}"))?;
    let possible = columns * columns * lags - columns;
    let density = if possible == 0 {
        0.0
    } else {
        skeleton.iter().map(|&value| value as usize).sum::<usize>() as f64 / possible as f64
    };
    let lambda_l0 = (0.007 + 0.16 * density)
        .max(1.0 / columns as f64 + 4.0 / rows as f64 - 0.4 * density.powf(0.8));
    let configuration = GraceConfiguration {
        max_lag,
        lambda_l0,
        gate_threshold,
        epochs,
        patience,
        seed,
        ..GraceConfiguration::default()
    };
    let result = fit_grace_with_progress(&windows, &skeleton, &configuration, |state| {
        progress("neural-training", state.epoch, state.maximum_epochs)
    })
    .map_err(|problem| format!("GRACE refused: {problem}"))?;
    progress("complete", 3, 3);
    let matrix = |values: &[u8]| {
        (0..columns)
            .map(|source| {
                (0..columns)
                    .map(|target| {
                        (0..lags)
                            .map(|lag| values[(source * columns + target) * lags + lag] != 0)
                            .collect()
                    })
                    .collect()
            })
            .collect()
    };
    let gates = (0..columns)
        .map(|source| {
            (0..columns)
                .map(|target| {
                    (0..lags)
                        .map(|lag| result.gate_values[(source * columns + target) * lags + lag])
                        .collect()
                })
                .collect()
        })
        .collect();
    Ok(AnalysisResult::Grace {
        observations: rows,
        variables: columns,
        max_lag,
        alpha,
        context,
        gate_threshold,
        lambda_l0,
        epochs: result.epochs,
        patience,
        seed,
        imputed_cells: missing_cells,
        skeleton: matrix(&result.skeleton),
        gate_values: gates,
        graph: matrix(&result.time_graph),
        loss: result.trajectory.iter().map(|step| step.loss).collect(),
        rmse: result.trajectory.iter().map(|step| step.rmse).collect(),
    })
}

pub(crate) fn pcmci_plus(
    values: &[f64],
    validity: &[u8],
    analysis_mask: &[u8],
    rows: usize,
    columns: usize,
    tau_max: usize,
    pc_alpha: f64,
    samples: TemporalSamples,
) -> Result<AnalysisResult, String> {
    if !(2..=32).contains(&columns) {
        return Err("PCMCI+ requires between 2 and 32 selected variables".to_owned());
    }
    if !(1..=20).contains(&tau_max) {
        return Err("PCMCI+ tauMax must be between 1 and 20".to_owned());
    }
    if !pc_alpha.is_finite() || !(0.0..=1.0).contains(&pc_alpha) || pc_alpha == 0.0 {
        return Err("PCMCI+ pcAlpha must be finite and in (0, 1]".to_owned());
    }
    if rows < (2 * tau_max + 16).max(24) {
        return Err("PCMCI+ has too few observations for the selected maximum lag".to_owned());
    }
    let result = match samples {
        TemporalSamples::Dense => {
            validate_dense_matrix("PCMCI+", values, rows, columns)?;
            let data = time_series_from_column_major(values, rows, columns);
            run_pcmciplus(&data, tau_max, pc_alpha, CiKind::ParCorr)
        }
        TemporalSamples::RoleAware {
            cut_off,
            propagate_through_max_lag,
            mask_type,
        } => {
            let frame = role_aware_frame(values, validity, analysis_mask, rows, columns)?;
            run_pcmciplus_frame(
                frame,
                tau_max,
                pc_alpha,
                CiKind::ParCorr,
                role_aware_policy(cut_off, propagate_through_max_lag, mask_type),
            )
            .map_err(|problem| format!("PCMCI+ sample construction refused: {problem}"))?
        }
    };
    Ok(AnalysisResult::PcmciPlus {
        observations: rows,
        variables: columns,
        tau_max,
        pc_alpha,
        graph: result.graph,
        p_matrix: result.p_matrix,
        val_matrix: result.val_matrix,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn jpcmciplus_evidence(
    values: &[f64],
    rows: usize,
    datasets: usize,
    periods: usize,
    observed_columns: usize,
    classes: Vec<JpcmciNodeClass>,
    time_dummy: bool,
    space_dummy: bool,
    tau_max: usize,
    pc_alpha: f64,
) -> Result<AnalysisResult, String> {
    if datasets < 2 {
        return Err("J-PCMCI+ requires at least two panel units (datasets)".to_owned());
    }
    if periods < 2 || rows != datasets.saturating_mul(periods) {
        return Err(
            "J-PCMCI+ requires one balanced period grid shared by every panel unit".to_owned(),
        );
    }
    if !(2..=32).contains(&observed_columns) || classes.len() != observed_columns {
        return Err(
            "J-PCMCI+ requires 2 to 32 observed variables and one class for each".to_owned(),
        );
    }
    if classes
        .iter()
        .filter(|class| matches!(class, JpcmciNodeClass::System))
        .count()
        < 2
    {
        return Err("J-PCMCI+ requires at least two system variables".to_owned());
    }
    if !(1..=20).contains(&tau_max) {
        return Err("J-PCMCI+ tauMax must be between 1 and 20".to_owned());
    }
    if !pc_alpha.is_finite() || !(0.0..=1.0).contains(&pc_alpha) || pc_alpha == 0.0 {
        return Err("J-PCMCI+ pcAlpha must be finite and in (0, 1]".to_owned());
    }
    if periods < (2 * tau_max + 16).max(24) {
        return Err(
            "J-PCMCI+ has too few periods per dataset for the selected maximum lag".to_owned(),
        );
    }
    validate_dense_matrix("J-PCMCI+", values, rows, observed_columns)?;

    for (column, class) in classes.iter().enumerate() {
        match class {
            JpcmciNodeClass::System => {}
            JpcmciNodeClass::TimeContext => {
                for period in 0..periods {
                    let expected = values[column * rows + period];
                    if (1..datasets).any(|dataset| {
                        values[column * rows + dataset * periods + period] != expected
                    }) {
                        return Err(format!(
                            "J-PCMCI+ time-context variable {} differs between panel units at period {}",
                            column + 1,
                            period + 1
                        ));
                    }
                }
            }
            JpcmciNodeClass::SpaceContext => {
                for dataset in 0..datasets {
                    let start = dataset * periods;
                    let expected = values[column * rows + start];
                    if (1..periods).any(|period| values[column * rows + start + period] != expected)
                    {
                        return Err(format!(
                            "J-PCMCI+ space-context variable {} changes within panel unit {}",
                            column + 1,
                            dataset + 1
                        ));
                    }
                }
            }
        }
    }

    let time_start = observed_columns;
    let space_start = time_start + usize::from(time_dummy) * periods;
    let components = space_start + usize::from(space_dummy) * datasets;
    let mut joint = vec![vec![vec![0.0; components]; periods]; datasets];
    for dataset in 0..datasets {
        for period in 0..periods {
            let row = dataset * periods + period;
            for column in 0..observed_columns {
                joint[dataset][period][column] = values[column * rows + row];
            }
            if time_dummy {
                joint[dataset][period][time_start + period] = 1.0;
            }
            if space_dummy {
                joint[dataset][period][space_start + dataset] = 1.0;
            }
        }
    }

    let mut vectors: Vec<Vec<(usize, i32)>> = (0..observed_columns)
        .map(|column| vec![(column, 0)])
        .collect();
    let mut core_classes: Vec<CoreJpcmciNodeClass> = classes
        .iter()
        .map(|class| match class {
            JpcmciNodeClass::System => CoreJpcmciNodeClass::System,
            JpcmciNodeClass::TimeContext => CoreJpcmciNodeClass::TimeContext,
            JpcmciNodeClass::SpaceContext => CoreJpcmciNodeClass::SpaceContext,
        })
        .collect();
    let mut result_classes: Vec<&'static str> = classes
        .iter()
        .map(|class| match class {
            JpcmciNodeClass::System => "system",
            JpcmciNodeClass::TimeContext => "timeContext",
            JpcmciNodeClass::SpaceContext => "spaceContext",
        })
        .collect();
    if time_dummy {
        vectors.push(
            (0..periods)
                .map(|period| (time_start + period, 0))
                .collect(),
        );
        core_classes.push(CoreJpcmciNodeClass::TimeDummy);
        result_classes.push("timeDummy");
    }
    if space_dummy {
        vectors.push(
            (0..datasets)
                .map(|dataset| (space_start + dataset, 0))
                .collect(),
        );
        core_classes.push(CoreJpcmciNodeClass::SpaceDummy);
        result_classes.push("spaceDummy");
    }

    let data = JointData::new(joint, vectors)
        .map_err(|problem| format!("J-PCMCI+ joint sample construction refused: {problem:?}"))?;
    let ci = data.analytic(MultCorrelation::MaxCorrelation);
    let result = jpcmciplus::run(&ci, &core_classes, tau_max, pc_alpha)
        .map_err(|problem| format!("J-PCMCI+ discovery refused: {problem:?}"))?;
    let node = |(variable, lag)| JpcmciNodeEvidence { variable, lag };
    let separating_sets = result
        .sepsets
        .iter()
        .enumerate()
        .flat_map(|(source, targets)| {
            targets.iter().enumerate().flat_map(move |(target, lags)| {
                lags.iter().enumerate().filter_map(move |(lag, variables)| {
                    (!variables.is_empty()).then(|| JpcmciSeparatingSetEvidence {
                        source,
                        target,
                        lag,
                        variables: variables.iter().copied().map(node).collect(),
                    })
                })
            })
        })
        .collect();
    let ambiguous_triples = result
        .ambiguous_triples
        .iter()
        .map(|&(left, middle, right)| JpcmciAmbiguousTripleEvidence {
            left: node(left),
            middle,
            right,
        })
        .collect();
    let parent_rows = |parents: Vec<Vec<(usize, i32)>>| {
        parents
            .into_iter()
            .map(|row| row.into_iter().map(node).collect())
            .collect()
    };
    let variables = core_classes.len();
    Ok(AnalysisResult::Jpcmciplus {
        observations: rows,
        datasets,
        periods,
        observed_variables: observed_columns,
        variables,
        classes: result_classes,
        time_dummy,
        space_dummy,
        tau_max,
        pc_alpha,
        graph: result.graph,
        p_matrix: result.p_matrix,
        val_matrix: result.val_matrix,
        separating_sets,
        ambiguous_triples,
        lagged_parents: parent_rows(result.lagged_parents),
        context_parents: parent_rows(result.context_parents),
        dummy_parents: parent_rows(result.dummy_parents),
    })
}

pub(crate) fn lpcmci_evidence<F>(
    values: &[f64],
    validity: &[u8],
    analysis_mask: &[u8],
    rows: usize,
    columns: usize,
    tau_max: usize,
    pc_alpha: f64,
    samples: TemporalSamples,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=32).contains(&columns) {
        return Err("LPCMCI requires between 2 and 32 selected variables".to_owned());
    }
    if !(1..=20).contains(&tau_max) {
        return Err("LPCMCI tauMax must be between 1 and 20".to_owned());
    }
    if !pc_alpha.is_finite() || !(0.0..=1.0).contains(&pc_alpha) || pc_alpha == 0.0 {
        return Err("LPCMCI pcAlpha must be finite and in (0, 1]".to_owned());
    }
    if rows < (2 * tau_max + 16).max(24) {
        return Err("LPCMCI has too few observations for the selected maximum lag".to_owned());
    }
    let result = match samples {
        TemporalSamples::Dense => {
            validate_dense_matrix("LPCMCI", values, rows, columns)?;
            run_lpcmci_with_progress(
                &time_series_from_column_major(values, rows, columns),
                tau_max,
                pc_alpha,
                progress,
            )
        }
        TemporalSamples::RoleAware {
            cut_off,
            propagate_through_max_lag,
            mask_type,
        } => {
            let frame = role_aware_frame(values, validity, analysis_mask, rows, columns)?;
            run_lpcmci_frame_with_progress(
                frame,
                tau_max,
                pc_alpha,
                CiKind::ParCorr,
                role_aware_policy(cut_off, propagate_through_max_lag, mask_type),
                progress,
            )
            .map_err(|problem| format!("LPCMCI sample construction refused: {problem}"))?
        }
    };
    Ok(AnalysisResult::Lpcmci {
        observations: rows,
        variables: columns,
        tau_max,
        pc_alpha,
        graph: result.graph,
        p_matrix: result.p_matrix,
        val_matrix: result.val_matrix,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn rpcmci_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    num_regimes: usize,
    max_transitions: usize,
    switch_thres: f64,
    num_iterations: usize,
    max_anneal: usize,
    tau_min: usize,
    tau_max: usize,
    pc_alpha: f64,
    alpha_level: f64,
    seed: u64,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "RPCMCI requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    if !(2..=6).contains(&num_regimes) {
        return Err("RPCMCI numRegimes must be between 2 and 6".to_owned());
    }
    if max_transitions >= rows {
        return Err(
            "RPCMCI maxTransitions must be smaller than the number of observations".to_owned(),
        );
    }
    if !switch_thres.is_finite() || !(0.0..=1.0).contains(&switch_thres) {
        return Err("RPCMCI switchThres must be finite and in [0, 1]".to_owned());
    }
    if !(1..=100).contains(&num_iterations) || !(1..=50).contains(&max_anneal) {
        return Err("RPCMCI iterations must be in 1..=100 and annealings in 1..=50".to_owned());
    }
    if tau_min > tau_max || tau_max > 6 {
        return Err("RPCMCI requires 0 <= tauMin <= tauMax <= 6 in the browser".to_owned());
    }
    if !pc_alpha.is_finite()
        || !(0.0..=1.0).contains(&pc_alpha)
        || pc_alpha == 0.0
        || !alpha_level.is_finite()
        || !(0.0..=1.0).contains(&alpha_level)
        || alpha_level == 0.0
    {
        return Err("RPCMCI pcAlpha and alphaLevel must be finite and in (0, 1]".to_owned());
    }
    validate_dense_matrix("RPCMCI", values, rows, columns)?;
    if rows < (2 * tau_max + 24).max(40) {
        return Err("RPCMCI has too few observations for the selected maximum lag".to_owned());
    }

    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let result = run_rpcmci_with_progress(
        &data,
        num_regimes,
        max_transitions,
        switch_thres,
        num_iterations,
        max_anneal,
        tau_min,
        tau_max,
        pc_alpha,
        alpha_level,
        seed,
        CiKind::ParCorr,
        progress,
    )
    .ok_or_else(|| "RPCMCI produced no error-free annealing for this configuration".to_owned())?;

    Ok(AnalysisResult::Rpcmci {
        observations: rows,
        variables: columns,
        num_regimes,
        max_transitions,
        switch_thres,
        num_iterations,
        max_anneal,
        tau_min,
        tau_max,
        pc_alpha,
        alpha_level,
        seed,
        regimes: result.regimes,
        graphs: result.graphs,
        val_matrices: result.val_matrices,
        p_matrices: result.p_matrices,
        diff_g_all: result.diff_g_f.0,
        diff_g_best: result.diff_g_f.1,
        error_free_annealings: result.error_free_annealings,
    })
}

pub(crate) fn dynotears_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    max_lag: usize,
    lambda_w: f64,
    lambda_a: f64,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "DYNOTEARS requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    if !(1..=6).contains(&max_lag) {
        return Err("DYNOTEARS maxLag must be between 1 and 6".to_owned());
    }
    if !lambda_w.is_finite() || lambda_w < 0.0 || !lambda_a.is_finite() || lambda_a < 0.0 {
        return Err("DYNOTEARS penalties must be finite and non-negative".to_owned());
    }
    validate_dense_matrix("DYNOTEARS", values, rows, columns)?;
    if rows < max_lag + 16 {
        return Err("DYNOTEARS has too few observations for the selected maximum lag".to_owned());
    }
    let samples = rows - max_lag;
    let x = DMatrix::from_fn(samples, columns, |row, column| {
        values[column * rows + max_lag + row]
    });
    let x_lags = DMatrix::from_fn(samples, columns * max_lag, |row, column| {
        let lag = column / columns + 1;
        let variable = column % columns;
        values[variable * rows + max_lag + row - lag]
    });
    let result = dynotears_with_progress(&x, &x_lags, lambda_w, lambda_a, progress);
    let contemporaneous_weights = (0..columns)
        .map(|source| {
            (0..columns)
                .map(|target| result.w[(source, target)])
                .collect()
        })
        .collect();
    let lagged_weights = (0..max_lag)
        .map(|lag| {
            (0..columns)
                .map(|source| {
                    (0..columns)
                        .map(|target| result.a[(lag * columns + source, target)])
                        .collect()
                })
                .collect()
        })
        .collect();
    Ok(AnalysisResult::Dynotears {
        observations: rows,
        variables: columns,
        max_lag,
        lambda_w,
        lambda_a,
        contemporaneous_weights,
        lagged_weights,
    })
}

/// DirectLiNGAM over independent observations in a dense column-major matrix.
///
/// The core matrix is target-by-source, matching lingam's `adjacency_matrix_`; the browser
/// contract is source-by-target so every discovery plot and DAG adapter uses one orientation.
pub(crate) fn direct_lingam_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "DirectLiNGAM requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    validate_dense_matrix("DirectLiNGAM", values, rows, columns)?;
    if rows < columns + 16 {
        return Err(
            "DirectLiNGAM has too few independent observations for the selected variables"
                .to_owned(),
        );
    }
    for column in 0..columns {
        let series = &values[column * rows..(column + 1) * rows];
        let first = series[0];
        if series.iter().all(|value| *value == first) {
            return Err(format!(
                "DirectLiNGAM requires every variable to vary; column {column} is constant"
            ));
        }
    }
    let matrix = DMatrix::from_fn(rows, columns, |row, column| values[column * rows + row]);
    let (causal_order, adjacency) = direct_lingam_with_progress(&matrix, progress);
    let weights = (0..columns)
        .map(|source| {
            (0..columns)
                .map(|target| adjacency[(target, source)])
                .collect()
        })
        .collect();
    Ok(AnalysisResult::DirectLingam {
        observations: rows,
        variables: columns,
        causal_order,
        weights,
    })
}

fn constraint_ci(value: ConstraintCiTest) -> CrossSectionalCi {
    match value {
        ConstraintCiTest::FisherZ => CrossSectionalCi::FisherZ,
        ConstraintCiTest::Kci => CrossSectionalCi::Kci,
    }
}

fn constraint_background(
    command: BackgroundKnowledgeCommand,
    variables: usize,
) -> Result<BackgroundKnowledge, String> {
    if command.tiers.len() != variables {
        return Err("background-knowledge tiers must contain one entry per variable".to_owned());
    }
    let mut background = BackgroundKnowledge::default();
    for (from, to) in command.forbidden {
        background.forbid(from, to);
    }
    for (from, to) in command.required {
        background.require(from, to);
    }
    for (from, to) in command.forbidden_patterns {
        background
            .forbid_pattern(&from, &to)
            .map_err(|problem| problem.to_string())?;
    }
    for (from, to) in command.required_patterns {
        background
            .require_pattern(&from, &to)
            .map_err(|problem| problem.to_string())?;
    }
    for (node, tier) in command.tiers.into_iter().enumerate() {
        if let Some(tier) = tier {
            background.add_to_tier(node, tier);
        }
    }
    for tier in command.forbidden_within_tiers {
        background.forbid_within_tier(tier);
    }
    Ok(background)
}

fn pag_endpoint_mark(at_source: i8, at_target: i8) -> Option<String> {
    if at_source == 0 && at_target == 0 {
        return None;
    }
    let source = match at_source {
        -1 => '-',
        1 => '<',
        2 => 'o',
        _ => return None,
    };
    let target = match at_target {
        -1 => '-',
        1 => '>',
        2 => 'o',
        _ => return None,
    };
    Some(format!("{source}-{target}"))
}

fn constraint_graph(graph: &EndpointGraph) -> Vec<Vec<Vec<String>>> {
    (0..graph.nodes())
        .map(|source| {
            (0..graph.nodes())
                .map(|target| {
                    vec![pag_endpoint_mark(
                        graph.endpoints[source][target],
                        graph.endpoints[target][source],
                    )
                    .unwrap_or_default()]
                })
                .collect()
        })
        .collect()
}

fn separating_set_evidence(
    sets: Vec<hirmos_causal_core::pc_stable::SeparatedPair>,
) -> Vec<ConstraintSeparatingSetEvidence> {
    sets.into_iter()
        .map(|set| ConstraintSeparatingSetEvidence {
            x: set.x,
            y: set.y,
            variables: set.variables,
        })
        .collect()
}

fn constraint_ci_evidence(
    tests: Vec<hirmos_causal_core::constraint_discovery::CiRecord>,
) -> Vec<ConstraintCiEvidence> {
    tests
        .into_iter()
        .map(|test| ConstraintCiEvidence {
            x: test.x,
            y: test.y,
            conditions: test.conditions,
            p_value: test.p_value,
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn cross_sectional_constraint_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    names: Vec<String>,
    alpha: f64,
    max_depth: Option<usize>,
    max_path_length: Option<usize>,
    ci_test: ConstraintCiTest,
    background: BackgroundKnowledgeCommand,
    fci: bool,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=32).contains(&columns) {
        return Err("PC-stable and FCI require between 2 and 32 selected variables".to_owned());
    }
    if names.len() != columns {
        return Err("PC-stable and FCI require one variable name per matrix column".to_owned());
    }
    if matches!(ci_test, ConstraintCiTest::Kci) && columns > 12 {
        return Err("KCI is limited to 12 selected variables in the browser".to_owned());
    }
    validate_dense_matrix(if fci { "FCI" } else { "PC-stable" }, values, rows, columns)?;
    let matrix = DMatrix::from_fn(rows, columns, |row, column| values[column * rows + row]);
    let background = constraint_background(background, columns)?;
    if fci {
        let result = run_fci_with_progress(
            &matrix,
            names,
            FciConfiguration {
                alpha,
                max_depth,
                max_path_length,
                ci_test: constraint_ci(ci_test),
            },
            &background,
            progress,
        )
        .map_err(|problem| format!("FCI refused: {problem}"))?;
        let edge_properties = result
            .edge_properties
            .into_iter()
            .map(|edge| FciEdgePropertyEvidence {
                left: edge.left,
                right: edge.right,
                directness: edge.directness.map(|value| match value {
                    Directness::DefinitelyDirect => "definitelyDirect",
                    Directness::PossiblyDirect => "possiblyDirect",
                }),
                latent_confounding: edge.latent_confounding.map(|value| match value {
                    LatentConfounding::Excluded => "excluded",
                    LatentConfounding::Possible => "possible",
                }),
            })
            .collect();
        return Ok(AnalysisResult::Fci {
            observations: rows,
            variables: columns,
            alpha,
            max_depth,
            max_path_length,
            ci_test,
            graph: constraint_graph(&result.pag),
            separating_sets: separating_set_evidence(result.separating_sets),
            ci_tests: constraint_ci_evidence(result.ci_tests),
            edge_properties,
        });
    }

    let result = run_pc_stable_with_progress(
        &matrix,
        names,
        PcStableConfiguration {
            alpha,
            max_depth,
            ci_test: constraint_ci(ci_test),
        },
        &background,
        progress,
    )
    .map_err(|problem| format!("PC-stable refused: {problem}"))?;
    Ok(AnalysisResult::PcStable {
        observations: rows,
        variables: columns,
        alpha,
        max_depth,
        ci_test,
        graph: constraint_graph(&result.graph),
        separating_sets: separating_set_evidence(result.separating_sets),
        ci_tests: constraint_ci_evidence(result.ci_tests),
    })
}

/// VAR-LiNGAM over a dense column-major matrix; `lags` bounds the BIC lag sweep.
///
/// The kernel panics on a rank-deficient VAR design or a singular residual covariance, which a
/// wasm module cannot recover from, so constant columns and undersized samples are refused here.
pub(crate) fn var_lingam_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    lags: usize,
    prune: bool,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err(
            "VAR-LiNGAM requires between 2 and 12 selected variables in the browser".to_owned(),
        );
    }
    if !(1..=6).contains(&lags) {
        return Err("VAR-LiNGAM lags must be between 1 and 6".to_owned());
    }
    validate_dense_matrix("VAR-LiNGAM", values, rows, columns)?;
    if rows < columns * (lags + 1) + 16 {
        return Err(
            "VAR-LiNGAM has too few observations for the selected variables and lag order"
                .to_owned(),
        );
    }
    for column in 0..columns {
        let series = &values[column * rows..(column + 1) * rows];
        let first = series[0];
        if series.iter().all(|value| *value == first) {
            return Err(format!(
                "VAR-LiNGAM requires every variable to vary; column {column} is constant"
            ));
        }
    }
    progress("var-fit", 0, 2);
    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let result = run_var_lingam(&data, lags, prune);
    progress("complete", 2, 2);
    let matrix = |tau: usize| -> Vec<Vec<f64>> {
        (0..columns)
            .map(|source| {
                (0..columns)
                    .map(|target| result.adjacency_matrices[tau][(target, source)])
                    .collect()
            })
            .collect()
    };
    Ok(AnalysisResult::VarLingam {
        observations: rows,
        variables: columns,
        lags,
        selected_lag: result.k_ar,
        prune,
        causal_order: result.causal_order.clone(),
        contemporaneous_weights: matrix(0),
        lagged_weights: (1..=result.k_ar).map(matrix).collect(),
    })
}

pub(crate) fn ocse_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    max_lag: usize,
    alpha: f64,
    n_shuffles: usize,
    method: OcseMethod,
    k: usize,
    progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err("oCSE requires between 2 and 12 selected variables in the browser".to_owned());
    }
    if !(1..=8).contains(&max_lag) {
        return Err("oCSE maxLag must be between 1 and 8".to_owned());
    }
    if !alpha.is_finite() || alpha <= 0.0 || alpha > 1.0 {
        return Err("oCSE alpha must be finite and in (0, 1]".to_owned());
    }
    if !(20..=2_000).contains(&n_shuffles) {
        return Err("oCSE nShuffles must be between 20 and 2000".to_owned());
    }
    if !(1..=20).contains(&k) {
        return Err("oCSE k must be between 1 and 20".to_owned());
    }
    validate_dense_matrix("oCSE", values, rows, columns)?;
    if rows < max_lag + 24 || matches!(method, OcseMethod::Knn) && k >= rows - max_lag {
        return Err("oCSE has too few observations for the selected lag and estimator".to_owned());
    }
    let data: Vec<Vec<f64>> = (0..rows)
        .map(|row| {
            (0..columns)
                .map(|column| values[column * rows + row])
                .collect()
        })
        .collect();
    let cmi_method = match method {
        OcseMethod::Gaussian => CmiMethod::Gaussian,
        OcseMethod::Knn => CmiMethod::Knn,
    };
    let edges = discover_network_with_progress(
        &data, max_lag, alpha, alpha, n_shuffles, cmi_method, k, progress,
    )
    .into_iter()
    .map(|edge| OcseEdgeEvidence {
        source: edge.src,
        target: edge.dst,
        lag: edge.lag,
        cmi: edge.cmi,
        p_value: edge.p_value,
    })
    .collect();
    Ok(AnalysisResult::Ocse {
        observations: rows,
        variables: columns,
        max_lag,
        alpha,
        n_shuffles,
        method,
        k,
        seed: 42,
        edges,
    })
}

fn neural_row_major(values: &[f64], rows: usize, columns: usize) -> Vec<f64> {
    (0..rows)
        .flat_map(|row| (0..columns).map(move |column| values[column * rows + row]))
        .collect()
}

fn standardize_neural_columns(
    values: &[f64],
    rows: usize,
    columns: usize,
) -> Result<(Vec<f64>, NeuralStandardizationEvidence), String> {
    let mut standardized = vec![0.0; values.len()];
    let mut means = Vec::with_capacity(columns);
    let mut scales = Vec::with_capacity(columns);

    for column in 0..columns {
        let offset = column * rows;
        let source = &values[offset..offset + rows];
        let mean = source.iter().sum::<f64>() / rows as f64;
        let variance = source
            .iter()
            .map(|value| {
                let centered = value - mean;
                centered * centered
            })
            .sum::<f64>()
            / rows as f64;
        let scale = variance.sqrt();
        if !mean.is_finite() || !scale.is_finite() || scale == 0.0 {
            return Err(format!(
                "neural Granger variable {} is constant or cannot be standardized",
                column + 1
            ));
        }

        for row in 0..rows {
            standardized[offset + row] = (source[row] - mean) / scale;
        }
        means.push(mean);
        scales.push(scale);
    }

    Ok((
        standardized,
        NeuralStandardizationEvidence { means, scales },
    ))
}

fn neural_scores(values: &[f64], variables: usize) -> Vec<Vec<f64>> {
    (0..variables)
        .map(|source| {
            (0..variables)
                .map(|target| values[target * variables + source])
                .collect()
        })
        .collect()
}

fn neural_active(values: &[bool], variables: usize) -> Vec<Vec<bool>> {
    (0..variables)
        .map(|source| {
            (0..variables)
                .map(|target| values[target * variables + source])
                .collect()
        })
        .collect()
}

fn neural_lag_scores(values: &[f64], variables: usize, lags: usize) -> Vec<Vec<Vec<f64>>> {
    (0..variables)
        .map(|source| {
            (0..variables)
                .map(|target| {
                    (0..lags)
                        .map(|lag| values[(target * variables + source) * lags + lag])
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn neural_lag_active(values: &[bool], variables: usize, lags: usize) -> Vec<Vec<Vec<bool>>> {
    (0..variables)
        .map(|source| {
            (0..variables)
                .map(|target| {
                    (0..lags)
                        .map(|lag| values[(target * variables + source) * lags + lag])
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn cmlp_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    lag: usize,
    hidden: Vec<usize>,
    activation: NeuralActivation,
    penalty: NeuralCmlpPenalty,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    max_iter: usize,
    check_every: usize,
    lookback: usize,
    seed: u64,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err("cMLP requires between 2 and 12 selected variables in the browser".to_owned());
    }
    if !(1..=20).contains(&lag) {
        return Err("cMLP lag must be between 1 and 20 in the browser".to_owned());
    }
    if hidden.is_empty()
        || hidden.len() > 4
        || hidden.iter().any(|width| !(1..=256).contains(width))
    {
        return Err("cMLP requires one to four hidden layers with widths from 1 to 256".to_owned());
    }
    if !(1..=50_000).contains(&max_iter)
        || check_every == 0
        || check_every > max_iter
        || lookback == 0
    {
        return Err("cMLP training counts are outside the supported range".to_owned());
    }
    validate_dense_matrix("cMLP", values, rows, columns)?;
    if rows < lag + 16 {
        return Err("cMLP has too few observations for the selected lag window".to_owned());
    }

    let core_activation = match activation {
        NeuralActivation::Sigmoid => CoreNeuralActivation::Sigmoid,
        NeuralActivation::Tanh => CoreNeuralActivation::Tanh,
        NeuralActivation::Relu => CoreNeuralActivation::Relu,
        NeuralActivation::LeakyRelu => CoreNeuralActivation::LeakyRelu,
        NeuralActivation::Identity => CoreNeuralActivation::Identity,
    };
    let core_penalty = match penalty {
        NeuralCmlpPenalty::GroupLasso => CoreCmlpPenalty::GroupLasso,
        NeuralCmlpPenalty::GroupSparseGroupLasso => CoreCmlpPenalty::GroupSparseGroupLasso,
        NeuralCmlpPenalty::Hierarchical => CoreCmlpPenalty::Hierarchical,
    };
    let configuration = CmlpConfig {
        lag,
        hidden: hidden.clone(),
        activation: core_activation,
        penalty: core_penalty,
        lambda,
        ridge_lambda,
        learning_rate,
        max_iter,
        check_every,
        lookback,
        seed,
    };
    let (standardized, standardization) = standardize_neural_columns(values, rows, columns)?;
    let result = fit_cmlp_with(
        &neural_row_major(&standardized, rows, columns),
        rows,
        columns,
        &configuration,
        None,
        |state| progress("neural-training", state.iteration, state.max_iterations),
    )
    .map_err(|problem| format!("cMLP refused: {problem}"))?;
    progress("complete", result.iterations, result.iterations);

    Ok(AnalysisResult::Cmlp {
        observations: rows,
        variables: columns,
        lag,
        hidden,
        activation,
        penalty,
        lambda,
        ridge_lambda,
        learning_rate,
        max_iter,
        check_every,
        lookback,
        seed,
        standardization,
        summary_scores: neural_scores(&result.summary_scores, columns),
        summary_active: neural_active(&result.summary_active, columns),
        lag_scores: neural_lag_scores(&result.lag_scores, columns, lag),
        lag_active: neural_lag_active(&result.lag_active, columns, lag),
        lag_order: result.lag_order,
        loss: result.loss,
        iterations: result.iterations,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn clstm_evidence<F>(
    values: &[f64],
    rows: usize,
    columns: usize,
    context: usize,
    hidden: usize,
    lambda: f64,
    ridge_lambda: f64,
    learning_rate: f64,
    max_iter: usize,
    check_every: usize,
    lookback: usize,
    seed: u64,
    mut progress: F,
) -> Result<AnalysisResult, String>
where
    F: FnMut(&'static str, usize, usize),
{
    if !(2..=12).contains(&columns) {
        return Err("cLSTM requires between 2 and 12 selected variables in the browser".to_owned());
    }
    if !(1..=100).contains(&context) {
        return Err("cLSTM context must be between 1 and 100 in the browser".to_owned());
    }
    if !(1..=256).contains(&hidden) {
        return Err("cLSTM hidden width must be between 1 and 256".to_owned());
    }
    if !(1..=20_000).contains(&max_iter)
        || check_every == 0
        || check_every > max_iter
        || lookback == 0
    {
        return Err("cLSTM training counts are outside the supported range".to_owned());
    }
    validate_dense_matrix("cLSTM", values, rows, columns)?;
    if rows < context + 16 {
        return Err("cLSTM has too few observations for the selected context window".to_owned());
    }

    let configuration = ClstmConfig {
        context,
        hidden,
        lambda,
        ridge_lambda,
        learning_rate,
        max_iter,
        check_every,
        lookback,
        seed,
    };
    let (standardized, standardization) = standardize_neural_columns(values, rows, columns)?;
    let result = fit_clstm_with(
        &neural_row_major(&standardized, rows, columns),
        rows,
        columns,
        &configuration,
        None,
        |state| progress("neural-training", state.iteration, state.max_iterations),
    )
    .map_err(|problem| format!("cLSTM refused: {problem}"))?;
    progress("complete", result.iterations, result.iterations);

    Ok(AnalysisResult::Clstm {
        observations: rows,
        variables: columns,
        context,
        hidden,
        lambda,
        ridge_lambda,
        learning_rate,
        max_iter,
        check_every,
        lookback,
        seed,
        standardization,
        summary_scores: neural_scores(&result.summary_scores, columns),
        summary_active: neural_active(&result.summary_active, columns),
        loss: result.loss,
        iterations: result.iterations,
    })
}

pub(crate) fn granger_evidence(
    values: &[f64],
    rows: usize,
    max_lag: usize,
) -> Result<AnalysisResult, String> {
    if !(1..=20).contains(&max_lag) {
        return Err("Granger maxLag must be between 1 and 20".to_owned());
    }
    let expected = rows
        .checked_mul(2)
        .ok_or_else(|| "Granger matrix dimensions overflowed".to_owned())?;
    if values.len() != expected {
        return Err(format!(
            "Granger received {} values for a {rows} by 2 matrix",
            values.len()
        ));
    }
    if rows <= 3 * max_lag + 1 {
        return Err(
            "Granger has insufficient observations for the selected maximum lag".to_owned(),
        );
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err("Granger requires finite paired values; resolve missingness first".to_owned());
    }
    let target = &values[..rows];
    let cause = &values[rows..];
    let tests = granger_ssr_ftest(target, cause, max_lag)
        .into_iter()
        .enumerate()
        .map(|(index, (statistic, p_value))| GrangerLagEvidence {
            lag: index + 1,
            statistic,
            p_value,
        })
        .collect();
    Ok(AnalysisResult::GrangerSsrF {
        observations: rows,
        max_lag,
        tests,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hirmos_causal_core::constraint_discovery::{ARROW, CIRCLE, NONE, TAIL};

    #[test]
    fn constraint_endpoint_matrix_preserves_every_pag_mark() {
        let marks = [
            (TAIL, ARROW, "-->"),
            (ARROW, TAIL, "<--"),
            (ARROW, ARROW, "<->"),
            (CIRCLE, ARROW, "o->"),
            (ARROW, CIRCLE, "<-o"),
            (CIRCLE, CIRCLE, "o-o"),
            (TAIL, TAIL, "---"),
            (TAIL, CIRCLE, "--o"),
            (CIRCLE, TAIL, "o--"),
        ];
        for (left, right, expected) in marks {
            let graph = EndpointGraph {
                node_names: vec!["X".to_owned(), "Y".to_owned()],
                endpoints: vec![vec![NONE, left], vec![right, NONE]],
            };
            let encoded = constraint_graph(&graph);
            assert_eq!(encoded[0][1][0], expected);
        }
    }

    #[test]
    fn cross_sectional_constraint_facade_serializes_pc_and_fci_results() {
        let rows = 240;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        let mut uniform = || {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            (state.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / ((1u64 << 53) as f64) - 0.5
        };
        for row in 0..rows {
            let x = uniform();
            let z = 1.2 * x + 0.3 * uniform();
            let y = -0.8 * z + 0.3 * uniform();
            values[row] = x;
            values[rows + row] = z;
            values[2 * rows + row] = y;
        }
        let background = || BackgroundKnowledgeCommand {
            forbidden: vec![],
            required: vec![],
            forbidden_patterns: vec![],
            required_patterns: vec![],
            tiers: vec![None; columns],
            forbidden_within_tiers: vec![],
        };
        let names = vec!["X".to_owned(), "Z".to_owned(), "Y".to_owned()];
        let pc = cross_sectional_constraint_evidence(
            &values,
            rows,
            columns,
            names.clone(),
            0.05,
            None,
            None,
            ConstraintCiTest::FisherZ,
            background(),
            false,
            |_, _, _| {},
        )
        .unwrap();
        let fci = cross_sectional_constraint_evidence(
            &values,
            rows,
            columns,
            names,
            0.05,
            None,
            None,
            ConstraintCiTest::FisherZ,
            background(),
            true,
            |_, _, _| {},
        )
        .unwrap();
        let pc = serde_json::to_value(pc).unwrap();
        let fci = serde_json::to_value(fci).unwrap();
        assert_eq!(pc["kind"], "pcStable");
        assert_eq!(fci["kind"], "fci");
        assert_eq!(pc["graph"].as_array().unwrap().len(), columns);
        assert!(!fci["edgeProperties"].as_array().unwrap().is_empty());
    }

    #[test]
    fn pcmci_plus_accepts_column_major_input_and_serializes_raw_evidence() {
        let rows = 120;
        let mut values = vec![0.0; rows * 3];
        for row in 0..rows {
            let time = row as f64;
            let x = (time / 4.0).sin() + 0.1 * (time / 1.7).cos();
            let y = if row == 0 { 0.0 } else { 0.8 * values[row - 1] } + 0.05 * (time / 2.3).sin();
            let z = if row == 0 {
                0.0
            } else {
                -0.6 * values[rows + row - 1]
            } + 0.04 * (time / 3.1).cos();
            values[row] = x;
            values[rows + row] = y;
            values[2 * rows + row] = z;
        }
        let json = pcmci_plus(&values, &[], &[], rows, 3, 2, 0.05, TemporalSamples::Dense)
            .and_then(|result| serde_json::to_string(&result).map_err(|error| error.to_string()))
            .expect("fixture should produce PCMCI+ evidence");
        let value: serde_json::Value = serde_json::from_str(&json).expect("valid result JSON");
        assert_eq!(value["kind"], "pcmciPlus");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["variables"], 3);
        assert_eq!(value["graph"].as_array().unwrap().len(), 3);
        assert_eq!(value["pMatrix"][0][0].as_array().unwrap().len(), 3);
    }

    #[test]
    fn pcmci_plus_command_accepts_the_browser_camel_case_contract() {
        let command: AnalysisCommand = serde_json::from_str(
            r#"{"kind":"pcmciPlus","rows":120,"columns":3,"tauMax":2,"pcAlpha":0.05,"samples":{"kind":"dense"}}"#,
        )
        .expect("browser command should parse");
        match command {
            AnalysisCommand::PcmciPlus {
                rows,
                columns,
                tau_max,
                pc_alpha,
                samples,
            } => {
                assert_eq!(rows, 120);
                assert_eq!(columns, 3);
                assert_eq!(tau_max, 2);
                assert_eq!(pc_alpha, 0.05);
                assert!(matches!(samples, TemporalSamples::Dense));
            }
            AnalysisCommand::StationarityBattery
            | AnalysisCommand::Multicollinearity { .. }
            | AnalysisCommand::PandasResampleDaily { .. }
            | AnalysisCommand::FlexSurv { .. }
            | AnalysisCommand::CoxRegression { .. }
            | AnalysisCommand::PenalizedAft { .. }
            | AnalysisCommand::Aalen { .. }
            | AnalysisCommand::SurvivalForest { .. }
            | AnalysisCommand::NonparametricSurvival { .. }
            | AnalysisCommand::ComparisonSurvival { .. }
            | AnalysisCommand::MultiStateSurvival { .. }
            | AnalysisCommand::Jpcmciplus { .. }
            | AnalysisCommand::Lpcmci { .. }
            | AnalysisCommand::Rpcmci { .. }
            | AnalysisCommand::Cdnots { .. }
            | AnalysisCommand::CdnotsPlus { .. }
            | AnalysisCommand::Grace { .. }
            | AnalysisCommand::Dynotears { .. }
            | AnalysisCommand::Cmlp { .. }
            | AnalysisCommand::Clstm { .. }
            | AnalysisCommand::DirectLingam { .. }
            | AnalysisCommand::PcStable { .. }
            | AnalysisCommand::Fci { .. }
            | AnalysisCommand::VarLingam { .. }
            | AnalysisCommand::Ocse { .. }
            | AnalysisCommand::GrangerSsrF { .. }
            | AnalysisCommand::BackdoorIdentify { .. }
            | AnalysisCommand::DagCheck { .. }
            | AnalysisCommand::BackdoorLinear { .. }
            | AnalysisCommand::FrontdoorTwoStage { .. }
            | AnalysisCommand::InstrumentalVariable { .. }
            | AnalysisCommand::CountGlm { .. }
            | AnalysisCommand::NegativeBinomialIngarch { .. }
            | AnalysisCommand::CountSeriesInterventionScan { .. }
            | AnalysisCommand::CausalEffectsTotal { .. }
            | AnalysisCommand::CausalImpact { .. }
            | AnalysisCommand::LinearRefutation { .. }
            | AnalysisCommand::UnobservedConfounding { .. }
            | AnalysisCommand::SeriesStructure { .. }
            | AnalysisCommand::SeasonalAdjust { .. }
            | AnalysisCommand::DoubleMl { .. }
            | AnalysisCommand::TLearner { .. }
            | AnalysisCommand::DmlRefutationBatch { .. }
            | AnalysisCommand::ArdlPss { .. }
            | AnalysisCommand::Vecm { .. }
            | AnalysisCommand::SyntheticControl { .. }
            | AnalysisCommand::PanelIntervention { .. }
            | AnalysisCommand::LinearScmCounterfactual { .. }
            | AnalysisCommand::DynamicLinearScmCounterfactual { .. }
            | AnalysisCommand::NegbinNuts { .. }
            | AnalysisCommand::BayesianGaussian { .. }
            | AnalysisCommand::DiscreteBnQuery { .. }
            | AnalysisCommand::IdentifiedDiscreteQuery { .. }
            | AnalysisCommand::BinaryEtt { .. }
            | AnalysisCommand::ResolveMissingness { .. } => {
                panic!("parsed the wrong command variant")
            }
        }
    }

    #[test]
    fn jpcmciplus_accepts_balanced_dataset_major_panel_and_serializes_joint_evidence() {
        let datasets = 3;
        let periods = 36;
        let rows = datasets * periods;
        let mut values = vec![0.0; rows * 2];
        for dataset in 0..datasets {
            for period in 0..periods {
                let row = dataset * periods + period;
                let time = period as f64;
                values[row] = (time / 3.0).sin() + dataset as f64 * 0.03;
                values[rows + row] = if period == 0 {
                    0.0
                } else {
                    0.7 * values[row - 1]
                } + 0.05 * (time / 2.0).cos();
            }
        }

        let result = jpcmciplus_evidence(
            &values,
            rows,
            datasets,
            periods,
            2,
            vec![JpcmciNodeClass::System, JpcmciNodeClass::System],
            true,
            true,
            1,
            0.05,
        )
        .expect("balanced panel should produce J-PCMCI+ evidence");
        let value = serde_json::to_value(result).expect("valid result JSON");
        assert_eq!(value["kind"], "jpcmciplus");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["datasets"], datasets);
        assert_eq!(value["periods"], periods);
        assert_eq!(value["observedVariables"], 2);
        assert_eq!(value["variables"], 4);
        assert_eq!(
            value["classes"],
            serde_json::json!(["system", "system", "timeDummy", "spaceDummy"])
        );
        assert_eq!(value["graph"].as_array().unwrap().len(), 4);
        assert_eq!(value["laggedParents"].as_array().unwrap().len(), 4);
        assert_eq!(value["contextParents"].as_array().unwrap().len(), 4);
        assert_eq!(value["dummyParents"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn jpcmciplus_refuses_context_roles_that_break_panel_invariance() {
        let datasets = 2;
        let periods = 24;
        let rows = datasets * periods;
        let mut values = vec![0.0; rows * 3];
        for row in 0..rows {
            values[row] = row as f64;
            values[rows + row] = (row as f64 / 3.0).sin();
            values[2 * rows + row] = row as f64;
        }
        let result = jpcmciplus_evidence(
            &values,
            rows,
            datasets,
            periods,
            3,
            vec![
                JpcmciNodeClass::System,
                JpcmciNodeClass::System,
                JpcmciNodeClass::TimeContext,
            ],
            false,
            false,
            1,
            0.05,
        );
        assert!(result.is_err());
    }

    #[test]
    fn jpcmciplus_command_accepts_the_browser_contract() {
        let command: AnalysisCommand = serde_json::from_str(
            r#"{"kind":"jpcmciplus","rows":48,"datasets":2,"periods":24,"observedColumns":2,"classes":["system","system"],"timeDummy":true,"spaceDummy":true,"tauMax":1,"pcAlpha":0.05}"#,
        )
        .expect("browser command should parse");
        let AnalysisCommand::Jpcmciplus {
            rows,
            datasets,
            periods,
            observed_columns,
            classes,
            time_dummy,
            space_dummy,
            tau_max,
            pc_alpha,
        } = command
        else {
            panic!("parsed the wrong command variant")
        };
        assert_eq!((rows, datasets, periods, observed_columns), (48, 2, 24, 2));
        assert_eq!(classes.len(), 2);
        assert!(time_dummy && space_dummy);
        assert_eq!(tau_max, 1);
        assert_eq!(pc_alpha, 0.05);
    }

    #[test]
    fn var_lingam_command_serializes_ordered_weights() {
        let rows = 160;
        let columns = 3;
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut uniform = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 11) as f64 / (1u64 << 53) as f64 - 0.5
        };
        let mut series = vec![vec![0.0; columns]; rows];
        for row in 1..rows {
            let e: Vec<f64> = (0..columns).map(|_| uniform()).collect();
            series[row][0] = 0.6 * series[row - 1][0] + e[0];
            series[row][1] = 0.5 * series[row][0] + 0.3 * series[row - 1][1] + e[1];
            series[row][2] = 0.4 * series[row][1] + 0.2 * series[row - 1][0] + e[2];
        }
        let mut values = vec![0.0; rows * columns];
        for column in 0..columns {
            for row in 0..rows {
                values[column * rows + row] = series[row][column];
            }
        }
        let mut stages = Vec::new();
        let result = var_lingam_evidence(&values, rows, columns, 2, true, |stage, done, total| {
            stages.push((stage, done, total))
        })
        .expect("VAR-LiNGAM fixture should run");
        let json = serde_json::to_value(result).expect("VAR-LiNGAM result serializes");
        assert_eq!(json["kind"], "varLingam");
        assert_eq!(json["causalOrder"].as_array().unwrap().len(), columns);
        assert_eq!(
            json["contemporaneousWeights"].as_array().unwrap().len(),
            columns
        );
        assert_eq!(
            json["laggedWeights"].as_array().unwrap().len(),
            json["selectedLag"].as_u64().unwrap() as usize
        );
        assert_eq!(stages.last(), Some(&("complete", 2, 2)));

        let mut constant = values.clone();
        for row in 0..rows {
            constant[row] = 1.0;
        }
        match var_lingam_evidence(&constant, rows, columns, 1, true, |_, _, _| {}) {
            Err(message) => assert!(message.contains("constant")),
            Ok(_) => panic!("a constant column must be refused"),
        }
    }

    #[test]
    fn direct_lingam_serializes_cross_sectional_order_and_weights() {
        let rows = 96;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            let u = ((row * 37 % 101) as f64 - 50.0) / 25.0;
            let v = ((row * 61 % 103) as f64 - 51.0) / 30.0;
            let w = ((row * 73 % 107) as f64 - 53.0) / 35.0;
            values[row] = u;
            values[rows + row] = 0.8 * u + v;
            values[2 * rows + row] = -0.5 * values[rows + row] + w;
        }
        let mut stages = Vec::new();
        let result = direct_lingam_evidence(&values, rows, columns, |stage, done, total| {
            stages.push((stage, done, total));
        })
        .expect("DirectLiNGAM fixture should run");
        let json = serde_json::to_value(result).expect("DirectLiNGAM result serializes");
        assert_eq!(json["kind"], "directLingam");
        assert_eq!(json["causalOrder"].as_array().unwrap().len(), columns);
        assert_eq!(json["weights"].as_array().unwrap().len(), columns);
        assert!(stages.iter().any(|(stage, _, _)| *stage == "causal-order"));
        assert_eq!(stages.last(), Some(&("complete", columns + 1, columns + 1)));
    }

    #[test]
    fn granger_command_serializes_every_lag_result() {
        let rows = 80;
        let mut values = vec![0.0; rows * 2];
        for row in 0..rows {
            let time = row as f64;
            values[rows + row] = (time / 3.0).sin() + 0.1 * (time / 1.7).cos();
            values[row] = if row == 0 {
                0.0
            } else {
                0.7 * values[rows + row - 1]
            } + 0.05 * (time / 2.1).sin();
        }
        let result = granger_evidence(&values, rows, 4).expect("fixture should produce evidence");
        let value = serde_json::to_value(result).expect("valid result JSON");
        assert_eq!(value["kind"], "grangerSsrF");
        assert_eq!(value["observations"], rows);
        assert_eq!(value["maxLag"], 4);
        assert_eq!(value["tests"].as_array().unwrap().len(), 4);
        assert_eq!(value["tests"][0]["lag"], 1);
        assert!(value["tests"][0]["statistic"].is_number());
        assert!(value["tests"][0]["pValue"].is_number());
    }

    #[test]
    fn pcmci_plus_refuses_bad_shape_and_missing_values() {
        assert!(pcmci_plus(
            &[0.0; 100],
            &[],
            &[],
            40,
            3,
            2,
            0.05,
            TemporalSamples::Dense
        )
        .is_err());
        let mut values = vec![0.0; 120];
        values[7] = f64::NAN;
        assert!(pcmci_plus(&values, &[], &[], 40, 3, 2, 0.05, TemporalSamples::Dense).is_err());
    }

    #[test]
    fn role_aware_pcmci_plus_accepts_an_explicitly_invalid_cell() {
        let rows = 40;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            values[row] = (row as f64 / 3.0).sin();
            values[rows + row] = (row as f64 / 5.0).cos();
            values[2 * rows + row] = (row as f64 / 7.0).sin();
        }
        let mut validity = vec![1; values.len()];
        let analysis_mask = vec![0; values.len()];
        values[11] = f64::NAN;
        validity[11] = 0;

        let result = pcmci_plus(
            &values,
            &validity,
            &analysis_mask,
            rows,
            columns,
            1,
            0.05,
            TemporalSamples::RoleAware {
                cut_off: TemporalCutOff::TwoTauMax,
                propagate_through_max_lag: false,
                mask_type: TemporalMaskType::Xyz,
            },
        );

        assert!(result.is_ok());
    }

    #[test]
    fn new_discovery_adapters_serialize_complete_outputs_and_report_real_progress() {
        let rows = 60;
        let columns = 3;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            let time = row as f64;
            values[row] = (time / 3.7).sin() + 0.07 * (time / 1.9).cos();
            values[rows + row] = if row == 0 {
                0.0
            } else {
                0.72 * values[row - 1]
            } + 0.05 * (time / 2.1).sin();
            values[2 * rows + row] = if row == 0 {
                0.0
            } else {
                -0.55 * values[rows + row - 1]
            } + 0.04 * (time / 2.9).cos();
        }

        let mut lpcmci_progress = Vec::new();
        let lpcmci = lpcmci_evidence(
            &values,
            &[],
            &[],
            rows,
            columns,
            1,
            0.05,
            TemporalSamples::Dense,
            |stage, done, total| {
                lpcmci_progress.push((stage, done, total));
            },
        )
        .expect("LPCMCI fixture should run");
        let lpcmci_json = serde_json::to_value(lpcmci).expect("LPCMCI result serializes");
        assert_eq!(lpcmci_json["kind"], "lpcmci");
        assert_eq!(lpcmci_json["graph"].as_array().unwrap().len(), columns);
        assert_eq!(lpcmci_progress.last(), Some(&("complete", 5, 5)));

        let mut dynotears_progress = Vec::new();
        let dynotears =
            dynotears_evidence(&values, rows, columns, 1, 0.1, 0.1, |stage, done, total| {
                dynotears_progress.push((stage, done, total))
            })
            .expect("DYNOTEARS fixture should run");
        let dynotears_json = serde_json::to_value(dynotears).expect("DYNOTEARS result serializes");
        assert_eq!(dynotears_json["kind"], "dynotears");
        assert_eq!(
            dynotears_json["contemporaneousWeights"]
                .as_array()
                .unwrap()
                .len(),
            columns
        );
        let (_, done, total) = dynotears_progress
            .last()
            .expect("DYNOTEARS reports completion");
        assert_eq!(done, total);

        let mut ocse_progress = Vec::new();
        let ocse = ocse_evidence(
            &values,
            rows,
            columns,
            1,
            0.05,
            20,
            OcseMethod::Gaussian,
            5,
            |stage, done, total| ocse_progress.push((stage, done, total)),
        )
        .expect("oCSE fixture should run");
        let ocse_json = serde_json::to_value(ocse).expect("oCSE result serializes");
        assert_eq!(ocse_json["kind"], "ocse");
        assert_eq!(ocse_json["seed"], 42);
        assert_eq!(ocse_progress.last(), Some(&("complete", columns, columns)));
    }

    #[test]
    fn neural_adapters_record_standardization_and_return_finite_losses_across_units() {
        let rows = 48;
        let columns = 2;
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            let time = row as f64;
            values[row] = 15_000.0 + 2_500.0 * (time / 4.0).sin();
            values[rows + row] = 0.1
                + if row == 0 {
                    0.0
                } else {
                    0.02 * (values[row - 1] - 15_000.0) / 2_500.0
                };
        }

        let cmlp = cmlp_evidence(
            &values,
            rows,
            columns,
            2,
            vec![3],
            NeuralActivation::Relu,
            NeuralCmlpPenalty::Hierarchical,
            0.005,
            0.01,
            0.01,
            5,
            1,
            5,
            0,
            |_, _, _| {},
        )
        .expect("cMLP should run after recorded per-column standardization");
        match cmlp {
            AnalysisResult::Cmlp {
                standardization,
                loss,
                ..
            } => {
                assert_eq!(standardization.means.len(), columns);
                assert_eq!(standardization.scales.len(), columns);
                assert!(standardization.scales[0] > 1_000.0);
                assert!(standardization.scales[1] < 0.1);
                assert!(loss.iter().all(|value| value.is_finite()));
            }
            _ => panic!("cMLP adapter returned another result variant"),
        }

        let clstm = clstm_evidence(
            &values,
            rows,
            columns,
            3,
            3,
            0.005,
            0.01,
            0.01,
            5,
            1,
            5,
            0,
            |_, _, _| {},
        )
        .expect("cLSTM should run after recorded per-column standardization");
        match clstm {
            AnalysisResult::Clstm {
                standardization,
                loss,
                ..
            } => {
                assert_eq!(standardization.means.len(), columns);
                assert_eq!(standardization.scales.len(), columns);
                assert!(loss.iter().all(|value| value.is_finite()));
            }
            _ => panic!("cLSTM adapter returned another result variant"),
        }

        let mut constant = values;
        constant[..rows].fill(1.0);
        let refusal = match cmlp_evidence(
            &constant,
            rows,
            columns,
            2,
            vec![3],
            NeuralActivation::Relu,
            NeuralCmlpPenalty::Hierarchical,
            0.005,
            0.01,
            0.01,
            1,
            1,
            1,
            0,
            |_, _, _| {},
        ) {
            Ok(_) => panic!("a constant selected variable should be refused before training"),
            Err(problem) => problem,
        };
        assert!(refusal.contains("variable 1 is constant"));
    }

    #[test]
    fn rpcmci_serializes_the_parity_fixture_and_reports_annealing_progress() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/rpcmci.json"
        ))
        .expect("RPCMCI fixture parses");
        let data: Vec<Vec<f64>> =
            serde_json::from_value(fixture["data"].clone()).expect("RPCMCI data parses");
        let rows = data.len();
        let columns = data[0].len();
        let mut values = vec![0.0; rows * columns];
        for row in 0..rows {
            for column in 0..columns {
                values[column * rows + row] = data[row][column];
            }
        }

        let num_iterations = fixture["num_iterations"].as_u64().unwrap() as usize;
        let max_anneal = fixture["max_anneal"].as_u64().unwrap() as usize;
        let mut progress = Vec::new();
        let result = rpcmci_evidence(
            &values,
            rows,
            columns,
            fixture["num_regimes"].as_u64().unwrap() as usize,
            fixture["max_transitions"].as_u64().unwrap() as usize,
            fixture["switch_thres"].as_f64().unwrap(),
            num_iterations,
            max_anneal,
            fixture["tau_min"].as_u64().unwrap() as usize,
            fixture["tau_max"].as_u64().unwrap() as usize,
            fixture["pc_alpha"].as_f64().unwrap(),
            fixture["alpha_level"].as_f64().unwrap(),
            fixture["seed"].as_u64().unwrap(),
            |stage, done, total| progress.push((stage, done, total)),
        )
        .expect("RPCMCI parity fixture should run");
        let json = serde_json::to_value(result).expect("RPCMCI result serializes");
        assert_eq!(json["kind"], "rpcmci");
        assert_eq!(json["regimes"].as_array().unwrap().len(), 2);
        assert_eq!(json["graphs"].as_array().unwrap().len(), 2);
        assert_eq!(
            progress.last(),
            Some(&(
                "complete",
                num_iterations * max_anneal,
                num_iterations * max_anneal
            ))
        );
    }
}
