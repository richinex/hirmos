//! Browser façades for the parity-tested flexsurv and ComparisonSurv kernels.

use super::*;
use hirmos_causal_core::data_preparation::{
    prepare_longitudinal_states, prepare_wide_events, EventStatus, LongitudinalObservation,
    LongitudinalStateHistory, NonEmptyVec, StateId, StateObservation, SubjectId, TransitionMatrix,
    WideEventHistory, WideSubject,
};
use hirmos_causal_core::survival::comparison_surv::{
    comparison_curves, crossing_times, describe, fixed_time_conversion, g_rho_test,
    observed_conversion, overall_test, smooth_hazard_curves, ComparisonData, ComparisonTime, Event,
    FixedPointResult, FixedPointScale, GRho, GRhoResult, Group, OverallConfiguration, RmstWindow,
    SignificanceLevel, SurvivalSample,
};
use hirmos_causal_core::survival::flexsurv::fit::{
    fit, FlexSurvFitPlan, SurvivalDataset, SurvivalRecord, UncertaintyEvidence,
};
use hirmos_causal_core::survival::flexsurv::model::{
    CovariateMatrix, FlexSurvFamily, RegressionModel,
};
use hirmos_causal_core::survival::flexsurv::multistate::{
    MarkovControl, MultiStateModel, TransitionGraph, TransitionModel,
};
use hirmos_causal_core::survival::flexsurv::observation::SurvivalObservation;
use hirmos_causal_core::survival::flexsurv::predict::{Prediction, PredictionRequest};
use hirmos_causal_core::survival::flexsurv::r_optim::ROptimControl;
use std::num::NonZeroUsize;

fn fixed_point_scale(scale: FixedPointScale) -> FixedPointScaleEvidence {
    match scale {
        FixedPointScale::Naive => FixedPointScaleEvidence::Naive,
        FixedPointScale::Log => FixedPointScaleEvidence::Log,
        FixedPointScale::ComplementaryLogLog => FixedPointScaleEvidence::ComplementaryLogLog,
        FixedPointScale::ArcsineSquareRoot => FixedPointScaleEvidence::ArcsineSquareRoot,
        FixedPointScale::Logit => FixedPointScaleEvidence::Logit,
    }
}

fn fixed_point_tests(result: &FixedPointResult) -> Result<Vec<FixedPointTestEvidence>, String> {
    result
        .tests
        .iter()
        .map(|test| {
            let group_zero = result
                .group_zero
                .iter()
                .find(|estimate| estimate.scale == test.scale)
                .ok_or_else(|| "fixed-time comparison omitted a group 0 scale".to_owned())?;
            let group_one = result
                .group_one
                .iter()
                .find(|estimate| estimate.scale == test.scale)
                .ok_or_else(|| "fixed-time comparison omitted a group 1 scale".to_owned())?;
            Ok(FixedPointTestEvidence {
                scale: fixed_point_scale(test.scale),
                group_zero_interval: [1.0 - group_zero.upper, 1.0 - group_zero.lower],
                group_one_interval: [1.0 - group_one.upper, 1.0 - group_one.lower],
                statistic: test.statistic,
                p_value: test.p_value,
            })
        })
        .collect()
}

fn g_rho_evidence(result: GRhoResult) -> GRhoEvidence {
    GRhoEvidence {
        rho: result.rho.get(),
        observed: result.observed,
        expected: result.expected,
        variance: result.variance,
        statistic: result.statistic,
        p_value: result.p_value,
    }
}

fn column(values: &[f64], rows: usize, columns: usize, index: usize) -> Result<Vec<f64>, String> {
    if index >= columns || values.len() != rows.saturating_mul(columns) {
        return Err("survival matrix shape does not match its selected columns".to_owned());
    }
    Ok(values[index * rows..(index + 1) * rows].to_vec())
}

fn event_indicator(values: &[f64], row: usize) -> Result<bool, String> {
    match values[row] {
        0.0 => Ok(false),
        1.0 => Ok(true),
        value => Err(format!(
            "survival event row {} must be 0 for censored or 1 for observed; found {value}",
            row + 1
        )),
    }
}

fn family(value: SurvivalFamily) -> FlexSurvFamily {
    match value {
        SurvivalFamily::Exponential => FlexSurvFamily::Exponential,
        SurvivalFamily::Weibull => FlexSurvFamily::Weibull,
        SurvivalFamily::WeibullPh => FlexSurvFamily::WeibullPh,
        SurvivalFamily::LogNormal => FlexSurvFamily::LogNormal,
        SurvivalFamily::Gamma => FlexSurvFamily::Gamma,
        SurvivalFamily::Gompertz => FlexSurvFamily::Gompertz,
        SurvivalFamily::LogLogistic => FlexSurvFamily::LogLogistic,
        SurvivalFamily::GeneralizedGamma => FlexSurvFamily::GeneralizedGamma,
        SurvivalFamily::GeneralizedF => FlexSurvFamily::GeneralizedF,
    }
}

pub(crate) fn flexsurv_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    observation: SurvivalObservationCommand,
    covariates: &[usize],
    requested_family: SurvivalFamily,
    prediction_times: &[f64],
) -> Result<AnalysisResult, String> {
    if rows < 2 || columns == 0 || values.len() != rows.saturating_mul(columns) {
        return Err("parametric survival needs at least two complete rows".to_owned());
    }
    if prediction_times.is_empty()
        || prediction_times
            .iter()
            .any(|time| !time.is_finite() || *time < 0.0)
    {
        return Err(
            "survival prediction times must be a non-empty list of non-negative values".to_owned(),
        );
    }
    let observation_columns = match observation {
        SurvivalObservationCommand::RightCensored { duration, event } => vec![duration, event],
        SurvivalObservationCommand::StartStop { start, stop, event } => vec![start, stop, event],
    };
    let observations = match observation {
        SurvivalObservationCommand::RightCensored { duration, event } => {
            let durations = column(values, rows, columns, duration)?;
            let events = column(values, rows, columns, event)?;
            let mut observations = Vec::with_capacity(rows);
            for row in 0..rows {
                let observed = event_indicator(&events, row)?;
                let value = if observed {
                    SurvivalObservation::exact(durations[row])
                } else {
                    SurvivalObservation::right_censored(durations[row])
                }
                .map_err(|problem| {
                    format!("invalid survival time at row {}: {problem:?}", row + 1)
                })?;
                observations.push(value);
            }
            observations
        }
        SurvivalObservationCommand::StartStop { start, stop, event } => {
            if !matches!(
                requested_family,
                SurvivalFamily::Exponential | SurvivalFamily::WeibullPh | SurvivalFamily::Gompertz
            ) {
                return Err("start-stop covariates are exposed only with proportional-hazards families: exponential, Weibull PH, or Gompertz".to_owned());
            }
            let starts = column(values, rows, columns, start)?;
            let stops = column(values, rows, columns, stop)?;
            let events = column(values, rows, columns, event)?;
            let mut observations = Vec::with_capacity(rows);
            for row in 0..rows {
                let observed = event_indicator(&events, row)?;
                let value = if observed {
                    SurvivalObservation::delayed_event(starts[row], stops[row])
                } else {
                    SurvivalObservation::delayed_right_censored(starts[row], stops[row])
                }
                .map_err(|problem| {
                    format!("invalid start-stop spell at row {}: {problem:?}", row + 1)
                })?;
                observations.push(value);
            }
            observations
        }
    };
    if covariates
        .iter()
        .any(|index| *index >= columns || observation_columns.contains(index))
    {
        return Err("survival covariates must be distinct matrix columns and exclude the duration, event, start, and stop roles".to_owned());
    }
    let distinct = covariates
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if distinct.len() != covariates.len() {
        return Err("survival covariates must not repeat".to_owned());
    }

    let data = SurvivalDataset::new(observations.into_iter().map(SurvivalRecord::new).collect())
        .map_err(|problem| format!("survival data refused: {problem:?}"))?;
    let core_family = family(requested_family);
    let mut model = RegressionModel::new(core_family, rows)
        .map_err(|problem| format!("survival model refused: {problem:?}"))?;
    let profile = if covariates.is_empty() {
        Vec::new()
    } else {
        let mut design = Vec::with_capacity(rows * covariates.len());
        let mut profile = Vec::with_capacity(covariates.len());
        for row in 0..rows {
            for index in covariates {
                design.push(values[index * rows + row]);
            }
        }
        for index in covariates {
            let values = column(values, rows, columns, *index)?;
            profile.push(values.iter().sum::<f64>() / rows as f64);
        }
        model = model
            .with_location_covariates(
                CovariateMatrix::new(rows, covariates.len(), design)
                    .map_err(|problem| format!("survival design refused: {problem:?}"))?,
            )
            .map_err(|problem| format!("survival design refused: {problem:?}"))?;
        profile
    };
    let parameter_count = NonZeroUsize::new(model.parameter_count())
        .ok_or_else(|| "survival model has no parameters".to_owned())?;
    let plan = FlexSurvFitPlan::optimize_automatic(
        model,
        data,
        ROptimControl::bfgs_defaults(parameter_count),
    )
    .map_err(|problem| format!("survival initialization refused: {problem:?}"))?;
    let fit = fit(plan).map_err(|problem| format!("survival fit failed: {problem:?}"))?;
    let prediction_profile = fit
        .prediction_covariates(profile.clone())
        .map_err(|problem| format!("survival prediction profile refused: {problem:?}"))?;
    let mut survival = Vec::with_capacity(prediction_times.len());
    let mut hazard = Vec::with_capacity(prediction_times.len());
    for time in prediction_times {
        let Prediction::AtTime(value) = fit
            .predict(&prediction_profile, PredictionRequest::AtTime(*time))
            .map_err(|problem| format!("survival prediction failed: {problem:?}"))?
        else {
            unreachable!("AtTime request returns an AtTime prediction")
        };
        survival.push(value.survival);
        hazard.push(value.hazard);
    }
    let median = match fit
        .predict(&prediction_profile, PredictionRequest::Quantile(0.5))
        .map_err(|problem| format!("survival median failed: {problem:?}"))?
    {
        Prediction::Quantile(value) => value,
        _ => unreachable!("Quantile request returns a quantile"),
    };
    let mean = match fit.predict(&prediction_profile, PredictionRequest::Mean) {
        Ok(Prediction::Mean(value)) if value.is_finite() => Some(value),
        Ok(Prediction::Mean(_)) | Err(_) => None,
        Ok(_) => unreachable!("Mean request returns a mean"),
    };
    let parameter_intervals = match &fit.uncertainty {
        UncertaintyEvidence::AllParametersFixed => vec![None; fit.transformed_parameters.len()],
        UncertaintyEvidence::Hessian { natural, .. } => natural
            .iter()
            .map(|interval| interval.as_ref().map(|value| [value.lower, value.upper]))
            .collect(),
    };
    let baseline_count = core_family.parameters().len();
    Ok(AnalysisResult::FlexSurv {
        observations: rows,
        events: fit
            .individual_log_likelihood
            .iter()
            .enumerate()
            .filter(|(row, _)| match observation {
                SurvivalObservationCommand::RightCensored { event, .. }
                | SurvivalObservationCommand::StartStop { event, .. } => {
                    values[event * rows + *row] == 1.0
                }
            })
            .count(),
        family: requested_family,
        natural_baseline: fit.natural_baseline,
        coefficients: fit.transformed_parameters[baseline_count..].to_vec(),
        parameter_intervals,
        log_likelihood: fit.log_likelihood,
        aic: fit.aic,
        bic: fit.bic,
        profile,
        prediction_times: prediction_times.to_vec(),
        survival,
        hazard,
        median,
        mean,
    })
}

pub(crate) fn comparison_survival_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    duration: usize,
    event: usize,
    group: usize,
    truncation_time: f64,
    permutations: usize,
    seed: u32,
) -> Result<AnalysisResult, String> {
    let durations = column(values, rows, columns, duration)?;
    let events = column(values, rows, columns, event)?;
    let groups = column(values, rows, columns, group)?;
    let mut samples = Vec::with_capacity(rows);
    for row in 0..rows {
        let observed = if event_indicator(&events, row)? {
            Event::Observed
        } else {
            Event::Censored
        };
        let group = match groups[row] {
            0.0 => Group::Zero,
            1.0 => Group::One,
            value => {
                return Err(format!(
                    "survival group row {} must be 0 or 1; found {value}",
                    row + 1
                ))
            }
        };
        samples.push(
            SurvivalSample::new(durations[row], observed, group)
                .map_err(|problem| format!("comparison row {} refused: {problem:?}", row + 1))?,
        );
    }
    let data = ComparisonData::new(samples)
        .map_err(|problem| format!("two-group survival comparison refused: {problem:?}"))?;
    let permutations = NonZeroUsize::new(permutations)
        .ok_or_else(|| "survival comparison needs at least one permutation".to_owned())?;
    let result = overall_test(
        &data,
        OverallConfiguration::new(truncation_time, permutations, seed)
            .map_err(|problem| format!("survival comparison configuration refused: {problem:?}"))?,
    )
    .map_err(|problem| format!("survival comparison failed: {problem:?}"))?;
    let description = describe(
        &data,
        RmstWindow::At(
            ComparisonTime::new(truncation_time)
                .map_err(|problem| format!("survival comparison horizon refused: {problem:?}"))?,
        ),
        SignificanceLevel::new(0.05)
            .map_err(|problem| format!("survival comparison interval refused: {problem:?}"))?,
    )
    .map_err(|problem| format!("survival comparison description failed: {problem:?}"))?;
    let curves = comparison_curves(&data);
    let smoothed_hazards = smooth_hazard_curves(&data);
    let observed_conversion = match observed_conversion(&data) {
        Ok(result) => SurvivalSummary::Recorded {
            result: ObservedConversionEvidence {
                group_zero_rate: result.control_rate,
                group_one_rate: result.treatment_rate,
                difference: result.difference,
                standard_error: result.standard_error,
                statistic: result.statistic,
                p_value: result.p_value,
            },
        },
        Err(problem) => SurvivalSummary::Unavailable {
            reason: format!("observed conversion comparison unavailable: {problem:?}"),
        },
    };
    let fixed_time_conversion = match fixed_time_conversion(&data, truncation_time) {
        Ok(result) => SurvivalSummary::Recorded {
            result: FixedTimeConversionEvidence {
                time: result.time,
                group_zero_rate: result.control_rate,
                group_one_rate: result.treatment_rate,
                difference: result.difference,
                standard_error: result.standard_error,
                interval: result.interval,
                statistic: result.statistic,
                p_value: result.p_value,
                scale_tests: fixed_point_tests(&result.comparison_surv)?,
            },
        },
        Err(problem) => SurvivalSummary::Unavailable {
            reason: format!("fixed-time conversion comparison unavailable: {problem:?}"),
        },
    };
    let peto_peto = match g_rho_test(&data, GRho::peto_peto()) {
        Ok(result) => SurvivalSummary::Recorded {
            result: g_rho_evidence(result),
        },
        Err(problem) => SurvivalSummary::Unavailable {
            reason: format!("Peto–Peto comparison unavailable: {problem:?}"),
        },
    };
    Ok(AnalysisResult::ComparisonSurvival {
        observations: rows,
        truncation_time,
        group_zero_curve: curves
            .group_zero
            .points
            .iter()
            .map(|point| [point.time, point.survival])
            .collect(),
        group_one_curve: curves
            .group_one
            .points
            .iter()
            .map(|point| [point.time, point.survival])
            .collect(),
        diagnostics: ComparisonSurvivalDiagnostics::Recorded {
            group_zero: GroupSurvivalDiagnostics {
                cumulative_hazard: curves
                    .group_zero
                    .points
                    .iter()
                    .filter(|point| point.cumulative_hazard.is_finite())
                    .map(|point| [point.time, point.cumulative_hazard])
                    .collect(),
                smoothed_hazard: smoothed_hazards
                    .group_zero
                    .points
                    .into_iter()
                    .map(|point| [point.time, point.hazard])
                    .collect(),
                at_risk: curves
                    .group_zero
                    .points
                    .iter()
                    .map(|point| [point.time, point.at_risk as f64])
                    .collect(),
                censor_times: curves
                    .group_zero
                    .points
                    .iter()
                    .filter(|point| point.censored > 0)
                    .map(|point| point.time)
                    .collect(),
                restricted_mean: description.group_zero.restricted_mean.estimate,
            },
            group_one: GroupSurvivalDiagnostics {
                cumulative_hazard: curves
                    .group_one
                    .points
                    .iter()
                    .filter(|point| point.cumulative_hazard.is_finite())
                    .map(|point| [point.time, point.cumulative_hazard])
                    .collect(),
                smoothed_hazard: smoothed_hazards
                    .group_one
                    .points
                    .into_iter()
                    .map(|point| [point.time, point.hazard])
                    .collect(),
                at_risk: curves
                    .group_one
                    .points
                    .iter()
                    .map(|point| [point.time, point.at_risk as f64])
                    .collect(),
                censor_times: curves
                    .group_one
                    .points
                    .iter()
                    .filter(|point| point.censored > 0)
                    .map(|point| point.time)
                    .collect(),
                restricted_mean: description.group_one.restricted_mean.estimate,
            },
            crossing_times: crossing_times(&data),
        },
        observed_conversion,
        fixed_time_conversion,
        peto_peto,
        proportional_hazards_p_value: result.proportional_hazards.p_value,
        log_rank_p_value: result.log_rank.p_value,
        gehan_wilcoxon_p_value: result.gehan_wilcoxon.p_value,
        tarone_ware_p_value: result.tarone_ware.p_value,
        weighted_kaplan_meier_p_value: result.weighted_kaplan_meier.p_value,
        absolute_difference_p_value: result.absolute_difference_permutation_p_value,
        two_stage_p_value: result.two_stage.two_stage_p_value,
        squared_difference_p_value: result.squared_differences.p_value,
        restricted_mean_difference: result.restricted_mean_difference.estimate,
        restricted_mean_interval: [
            result.restricted_mean_difference.lower,
            result.restricted_mean_difference.upper,
        ],
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn multi_state_survival_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    input: MultiStateInputCommand,
    requested_family: SurvivalFamily,
    prediction_times: &[f64],
) -> Result<AnalysisResult, String> {
    if !matches!(
        requested_family,
        SurvivalFamily::Exponential | SurvivalFamily::WeibullPh | SurvivalFamily::Gompertz
    ) {
        return Err("multi-state start-stop models require exponential, Weibull PH, or Gompertz transition hazards".to_owned());
    }
    if prediction_times.is_empty()
        || prediction_times.windows(2).any(|pair| pair[0] > pair[1])
        || prediction_times
            .iter()
            .any(|time| !time.is_finite() || *time < 0.0)
    {
        return Err("multi-state prediction times must be non-negative and ordered".to_owned());
    }
    let prepared = prepare_multi_state_input(values, rows, columns, input)?;
    let rows = prepared.rows;
    let starts = column(&prepared.values, rows, 5, 0)?;
    let stops = column(&prepared.values, rows, 5, 1)?;
    let events = column(&prepared.values, rows, 5, 2)?;
    let origins = column(&prepared.values, rows, 5, 3)?;
    let destinations = column(&prepared.values, rows, 5, 4)?;
    if origins
        .iter()
        .chain(&destinations)
        .any(|value| !value.is_finite() || value.fract() != 0.0)
    {
        return Err(
            "multi-state origin and destination columns must contain finite integer state codes"
                .to_owned(),
        );
    }
    let mut states = origins.clone();
    for row in 0..rows {
        if event_indicator(&events, row)? {
            states.push(destinations[row]);
        }
    }
    states.sort_by(f64::total_cmp);
    states.dedup_by(|left, right| *left == *right);
    if states.len() < 2 {
        return Err("multi-state analysis needs at least two distinct state codes".to_owned());
    }
    let state_index = |value: f64| {
        states
            .binary_search_by(|candidate| candidate.total_cmp(&value))
            .expect("validated state code belongs to the state list")
    };
    let mut transitions = Vec::<(usize, usize)>::new();
    for row in 0..rows {
        if event_indicator(&events, row)? {
            let transition = (state_index(origins[row]), state_index(destinations[row]));
            if transition.0 == transition.1 {
                return Err(format!(
                    "multi-state event row {} cannot remain in the same state",
                    row + 1
                ));
            }
            if !transitions.contains(&transition) {
                transitions.push(transition);
            }
        }
    }
    if transitions.is_empty() {
        return Err("multi-state analysis needs at least one observed transition".to_owned());
    }
    let graph = TransitionGraph::new(
        states.iter().map(|value| value.to_string()).collect(),
        transitions.clone(),
    )
    .map_err(|problem| format!("multi-state graph refused: {problem:?}"))?;
    let core_family = family(requested_family);
    let mut transition_models = Vec::with_capacity(transitions.len());
    for &(origin, destination) in &transitions {
        let selected = (0..rows)
            .filter(|row| state_index(origins[*row]) == origin)
            .collect::<Vec<_>>();
        let mut records = Vec::with_capacity(selected.len());
        for row in selected {
            let observed =
                event_indicator(&events, row)? && state_index(destinations[row]) == destination;
            let observation = if observed {
                SurvivalObservation::delayed_event(starts[row], stops[row])
            } else {
                SurvivalObservation::delayed_right_censored(starts[row], stops[row])
            }
            .map_err(|problem| {
                format!("invalid multi-state spell at row {}: {problem:?}", row + 1)
            })?;
            records.push(SurvivalRecord::new(observation));
        }
        let data = SurvivalDataset::new(records)
            .map_err(|problem| format!("multi-state transition data refused: {problem:?}"))?;
        let model = RegressionModel::new(core_family, data.len())
            .map_err(|problem| format!("multi-state transition model refused: {problem:?}"))?;
        let parameter_count = NonZeroUsize::new(model.parameter_count())
            .ok_or_else(|| "multi-state transition model has no parameters".to_owned())?;
        let fit = fit(FlexSurvFitPlan::optimize_automatic(
            model,
            data,
            ROptimControl::bfgs_defaults(parameter_count),
        )
        .map_err(|problem| {
            format!("multi-state transition initialization refused: {problem:?}")
        })?)
        .map_err(|problem| format!("multi-state transition fit failed: {problem:?}"))?;
        transition_models.push(
            TransitionModel::from_fit(fit, Vec::new()).map_err(|problem| {
                format!("multi-state transition prediction refused: {problem:?}")
            })?,
        );
    }
    let model = MultiStateModel::clock_forward(graph, transition_models)
        .map_err(|problem| format!("multi-state model refused: {problem:?}"))?;
    let result = model
        .transition_probabilities(
            prediction_times.to_vec(),
            MarkovControl::de_solve_rk45_dp7(1e10)
                .map_err(|problem| format!("multi-state integrator refused: {problem:?}"))?,
        )
        .map_err(|problem| format!("multi-state prediction failed: {problem:?}"))?;
    Ok(AnalysisResult::MultiStateSurvival {
        observations: rows,
        states,
        transitions: transitions
            .into_iter()
            .map(|(origin, destination)| [origin, destination])
            .collect(),
        family: requested_family,
        preparation: prepared.evidence,
        prediction_times: result.times,
        probabilities: result.probabilities,
    })
}

struct PreparedMultiStateInput {
    values: Vec<f64>,
    rows: usize,
    evidence: MultiStatePreparationEvidence,
}

fn prepare_multi_state_input(
    values: &[f64],
    rows: usize,
    columns: usize,
    input: MultiStateInputCommand,
) -> Result<PreparedMultiStateInput, String> {
    match input {
        MultiStateInputCommand::PreparedRows {
            start,
            stop,
            event,
            from,
            to,
        } => {
            let selected = [start, stop, event, from, to];
            let mut prepared = Vec::with_capacity(rows.saturating_mul(selected.len()));
            for column_index in selected {
                prepared.extend(column(values, rows, columns, column_index)?);
            }
            Ok(PreparedMultiStateInput {
                values: prepared,
                rows,
                evidence: MultiStatePreparationEvidence::PreparedRows,
            })
        }
        MultiStateInputCommand::LongitudinalStates {
            subject,
            time,
            state,
            allowed,
        } => prepare_longitudinal_input(values, rows, columns, subject, time, state, allowed),
        MultiStateInputCommand::WideEvents {
            states,
            transitions,
            entry,
        } => prepare_wide_input(values, rows, columns, states, transitions, entry),
    }
}

fn prepare_longitudinal_input(
    values: &[f64],
    rows: usize,
    columns: usize,
    subject: usize,
    time: usize,
    state: usize,
    allowed: Vec<Vec<bool>>,
) -> Result<PreparedMultiStateInput, String> {
    let subjects = column(values, rows, columns, subject)?;
    let times = column(values, rows, columns, time)?;
    let states = column(values, rows, columns, state)?;
    let transitions = TransitionMatrix::from_allowed(allowed)
        .map_err(|problem| format!("longitudinal transition structure refused: {problem}"))?;
    let mut observations = Vec::with_capacity(rows);
    for row in 0..rows {
        observations.push(
            LongitudinalObservation::new(
                subject_id(subjects[row], row)?,
                times[row],
                state_id(states[row], row)?,
                Vec::new(),
            )
            .map_err(|problem| format!("longitudinal row {} refused: {problem}", row + 1))?,
        );
    }
    let observations = NonEmptyVec::try_from_vec(observations)
        .map_err(|problem| format!("longitudinal observations refused: {problem}"))?;
    let history = LongitudinalStateHistory::new(transitions, observations)
        .map_err(|problem| format!("longitudinal state history refused: {problem}"))?;
    let prepared = prepare_longitudinal_states(history)
        .map_err(|problem| format!("longitudinal state preparation refused: {problem}"))?;
    prepared_input(
        prepared,
        |source_rows, transition_rows, notices| MultiStatePreparationEvidence::LongitudinalStates {
            source_rows,
            transition_rows,
            notices,
        },
        rows,
    )
}

fn prepare_wide_input(
    values: &[f64],
    rows: usize,
    columns: usize,
    states: Vec<WideStateColumnsCommand>,
    transitions: Vec<Vec<Option<usize>>>,
    entry: WideEntryCommand,
) -> Result<PreparedMultiStateInput, String> {
    let transition_matrix = TransitionMatrix::from_numbered(transitions)
        .map_err(|problem| format!("wide transition structure refused: {problem}"))?;
    if states.len() != transition_matrix.state_count() {
        return Err(format!(
            "wide input defines {} state column pairs for a {}-state transition matrix",
            states.len(),
            transition_matrix.state_count()
        ));
    }

    let mut subjects = Vec::with_capacity(rows);
    for row in 0..rows {
        let state_observations = states
            .iter()
            .map(|state| match state {
                WideStateColumnsCommand::NotApplicable => Ok(StateObservation::NotApplicable),
                WideStateColumnsCommand::Recorded { time, status } => {
                    let time_value = value_at(values, rows, columns, *time, row)?;
                    let status_value = value_at(values, rows, columns, *status, row)?;
                    StateObservation::recorded(time_value, event_status(status_value, row)?)
                        .map_err(|problem| format!("wide row {} refused: {problem}", row + 1))
                }
            })
            .collect::<Result<Vec<_>, String>>()?;
        let (entry_state, entry_time) = match entry {
            WideEntryCommand::Shared { state, time } => (
                StateId::new(state)
                    .map_err(|problem| format!("shared entry state refused: {problem}"))?,
                time,
            ),
            WideEntryCommand::Columns { state, time } => (
                state_id(value_at(values, rows, columns, state, row)?, row)?,
                value_at(values, rows, columns, time, row)?,
            ),
        };
        subjects.push(
            WideSubject::new(
                SubjectId::new((row + 1).to_string())
                    .map_err(|problem| format!("wide subject id refused: {problem}"))?,
                NonEmptyVec::try_from_vec(state_observations)
                    .map_err(|problem| format!("wide state observations refused: {problem}"))?,
                entry_state,
                entry_time,
                Vec::new(),
            )
            .map_err(|problem| format!("wide row {} refused: {problem}", row + 1))?,
        );
    }
    let subjects = NonEmptyVec::try_from_vec(subjects)
        .map_err(|problem| format!("wide subjects refused: {problem}"))?;
    let history = WideEventHistory::new(transition_matrix, subjects)
        .map_err(|problem| format!("wide event history refused: {problem}"))?;
    let prepared = prepare_wide_events(history)
        .map_err(|problem| format!("wide event preparation refused: {problem}"))?;
    prepared_input(
        prepared,
        |source_rows, transition_rows, notices| MultiStatePreparationEvidence::WideEvents {
            source_rows,
            transition_rows,
            notices,
        },
        rows,
    )
}

fn prepared_input(
    prepared: hirmos_causal_core::data_preparation::PreparedMultiStateData,
    evidence: impl FnOnce(usize, usize, Vec<String>) -> MultiStatePreparationEvidence,
    source_rows: usize,
) -> Result<PreparedMultiStateInput, String> {
    let transition_rows = prepared.rows().len();
    let notices = prepared.notices().iter().map(ToString::to_string).collect();
    let mut values = vec![0.0; transition_rows.saturating_mul(5)];
    for (row, transition) in prepared.rows().iter().enumerate() {
        values[row] = transition.start;
        values[transition_rows + row] = transition.stop;
        values[2 * transition_rows + row] = f64::from(transition.status.indicator());
        values[3 * transition_rows + row] = transition.from.get() as f64;
        values[4 * transition_rows + row] = transition.to.get() as f64;
    }
    Ok(PreparedMultiStateInput {
        values,
        rows: transition_rows,
        evidence: evidence(source_rows, transition_rows, notices),
    })
}

fn value_at(
    values: &[f64],
    rows: usize,
    columns: usize,
    column_index: usize,
    row: usize,
) -> Result<f64, String> {
    if values.len() != rows.saturating_mul(columns) {
        return Err(
            "multi-state preparation payload shape does not match rows and columns".to_owned(),
        );
    }
    if column_index >= columns {
        return Err(format!(
            "multi-state preparation column {column_index} is outside the {columns}-column matrix"
        ));
    }
    values
        .get(column_index * rows + row)
        .copied()
        .ok_or_else(|| format!("multi-state preparation row {row} is outside the matrix"))
}

fn subject_id(value: f64, row: usize) -> Result<SubjectId, String> {
    if !value.is_finite() || value.fract() != 0.0 {
        return Err(format!(
            "longitudinal subject code at row {} must be a finite integer",
            row + 1
        ));
    }
    SubjectId::new(format!("{value:.0}"))
        .map_err(|problem| format!("longitudinal subject at row {} refused: {problem}", row + 1))
}

fn state_id(value: f64, row: usize) -> Result<StateId, String> {
    if !value.is_finite() || value.fract() != 0.0 || value < 1.0 || value > usize::MAX as f64 {
        return Err(format!(
            "state at row {} must be a positive integer",
            row + 1
        ));
    }
    StateId::new(value as usize)
        .map_err(|problem| format!("state at row {} refused: {problem}", row + 1))
}

fn event_status(value: f64, row: usize) -> Result<EventStatus, String> {
    match value {
        0.0 => Ok(EventStatus::Censored),
        1.0 => Ok(EventStatus::Observed),
        _ => Err(format!(
            "status at row {} must be 0 for censored or 1 for observed",
            row + 1
        )),
    }
}

#[cfg(test)]
mod upstream_data_tests {
    use super::*;

    fn selected_columns(csv: &str, selected: &[&str]) -> (Vec<f64>, usize, usize) {
        let mut lines = csv.lines();
        let headers = lines
            .next()
            .expect("upstream CSV has a header")
            .split(',')
            .collect::<Vec<_>>();
        let indices = selected
            .iter()
            .map(|name| {
                headers
                    .iter()
                    .position(|header| header == name)
                    .expect("selected column exists")
            })
            .collect::<Vec<_>>();
        let rows = lines
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.split(',').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut values = Vec::with_capacity(rows.len() * indices.len());
        for index in indices {
            values.extend(rows.iter().map(|row| {
                row[index]
                    .parse::<f64>()
                    .expect("selected upstream value is numeric")
            }));
        }
        (values, rows.len(), selected.len())
    }

    #[test]
    fn flexsurv_bc_reaches_the_browser_facade_without_changing_the_source_rows() {
        let (values, rows, columns) = selected_columns(
            include_str!("../../../public/examples/data/survival/flexsurv_bc.csv"),
            &["recyrs", "censrec"],
        );
        let result = flexsurv_evidence(
            &values,
            rows,
            columns,
            SurvivalObservationCommand::RightCensored {
                duration: 0,
                event: 1,
            },
            &[],
            SurvivalFamily::Weibull,
            &[0.0, 1.0, 3.0, 5.0],
        )
        .expect("the exact flexsurv bc data fit through the facade");

        let AnalysisResult::FlexSurv {
            observations,
            events,
            survival,
            median,
            ..
        } = result
        else {
            panic!("flexsurv command returned another result kind")
        };
        assert_eq!(observations, 686);
        assert_eq!(events, 299);
        assert_eq!(survival.len(), 4);
        assert!(survival.windows(2).all(|pair| pair[0] >= pair[1]));
        assert!(median.is_finite() && median > 0.0);
    }

    #[test]
    fn comparison_surv_crossing_data_reaches_every_comparison_lane() {
        let (values, rows, columns) = selected_columns(
            include_str!("../../../public/examples/data/survival/comparison_surv_crossdata.csv"),
            &["time", "status", "group"],
        );
        let result = comparison_survival_evidence(&values, rows, columns, 0, 1, 2, 2.0, 199, 43)
            .expect("the exact ComparisonSurv crossing data run through the facade");

        let AnalysisResult::ComparisonSurvival {
            observations,
            group_zero_curve,
            group_one_curve,
            diagnostics,
            observed_conversion,
            fixed_time_conversion,
            peto_peto,
            two_stage_p_value,
            restricted_mean_interval,
            ..
        } = result
        else {
            panic!("comparison command returned another result kind")
        };
        assert_eq!(observations, 200);
        assert!(!group_zero_curve.is_empty());
        assert!(!group_one_curve.is_empty());
        let ComparisonSurvivalDiagnostics::Recorded {
            group_zero,
            group_one,
            crossing_times,
        } = diagnostics;
        assert!(!group_zero.cumulative_hazard.is_empty());
        assert!(!group_one.cumulative_hazard.is_empty());
        assert_eq!(group_zero.smoothed_hazard.len(), 9);
        assert_eq!(group_one.smoothed_hazard.len(), 9);
        assert!(!group_zero.at_risk.is_empty());
        assert!(!group_one.at_risk.is_empty());
        assert!(group_zero.restricted_mean.is_finite());
        assert!(group_one.restricted_mean.is_finite());
        assert!(!crossing_times.is_empty());
        assert!(matches!(
            observed_conversion,
            SurvivalSummary::Recorded { .. }
        ));
        assert!(matches!(
            fixed_time_conversion,
            SurvivalSummary::Recorded { .. }
        ));
        assert!(matches!(peto_peto, SurvivalSummary::Recorded { .. }));
        assert!((0.0..=1.0).contains(&two_stage_p_value));
        assert!(restricted_mean_interval[0] <= restricted_mean_interval[1]);
    }

    #[test]
    fn comparison_surv_proportional_hazards_data_reaches_the_same_boundary() {
        let (values, rows, columns) = selected_columns(
            include_str!("../../../public/examples/data/survival/comparison_surv_phdata.csv"),
            &["time", "status", "group"],
        );
        let result = comparison_survival_evidence(&values, rows, columns, 0, 1, 2, 1.0, 199, 43)
            .expect("the exact ComparisonSurv proportional-hazards data run through the facade");

        let AnalysisResult::ComparisonSurvival {
            observations,
            proportional_hazards_p_value,
            log_rank_p_value,
            ..
        } = result
        else {
            panic!("comparison command returned another result kind")
        };
        assert_eq!(observations, 200);
        assert!((0.0..=1.0).contains(&proportional_hazards_p_value));
        assert!((0.0..=1.0).contains(&log_rank_p_value));
    }

    #[test]
    fn flexsurv_bosms3_reaches_start_stop_and_multi_state_facades() {
        let csv = include_str!("../../../public/examples/data/survival/flexsurv_bosms3.csv");
        let (start_stop, rows, columns) = selected_columns(csv, &["Tstart", "Tstop", "status"]);
        let start_stop_result = flexsurv_evidence(
            &start_stop,
            rows,
            columns,
            SurvivalObservationCommand::StartStop {
                start: 0,
                stop: 1,
                event: 2,
            },
            &[],
            SurvivalFamily::WeibullPh,
            &[0.0, 3.0, 6.0, 12.0],
        )
        .expect("the exact flexsurv bosms3 spells fit through the start-stop facade");
        assert!(matches!(
            start_stop_result,
            AnalysisResult::FlexSurv {
                observations: 511,
                ..
            }
        ));

        let (multi_state, rows, columns) =
            selected_columns(csv, &["Tstart", "Tstop", "status", "from", "to"]);
        let result = multi_state_survival_evidence(
            &multi_state,
            rows,
            columns,
            MultiStateInputCommand::PreparedRows {
                start: 0,
                stop: 1,
                event: 2,
                from: 3,
                to: 4,
            },
            SurvivalFamily::WeibullPh,
            &[0.0, 3.0, 6.0, 12.0],
        )
        .expect("the exact flexsurv bosms3 data run through the multi-state facade");
        let AnalysisResult::MultiStateSurvival {
            observations,
            states,
            transitions,
            probabilities,
            ..
        } = result
        else {
            panic!("multi-state command returned another result kind")
        };
        assert_eq!(observations, 511);
        assert_eq!(states, vec![1.0, 2.0, 3.0]);
        assert_eq!(transitions.len(), 3);
        assert_eq!(probabilities.len(), 4);
        assert!(probabilities.iter().all(|matrix| matrix.len() == 9));
    }
}
