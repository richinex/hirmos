//! Browser façades for the parity-tested flexsurv and ComparisonSurv kernels.

use super::*;
use hirmos_causal_core::survival::comparison_surv::{
    comparison_curves, crossing_times, describe, overall_test, smooth_hazard_curves,
    ComparisonData, ComparisonTime, Event, Group, OverallConfiguration, RmstWindow,
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
    start: usize,
    stop: usize,
    event: usize,
    from: usize,
    to: usize,
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
    let starts = column(values, rows, columns, start)?;
    let stops = column(values, rows, columns, stop)?;
    let events = column(values, rows, columns, event)?;
    let origins = column(values, rows, columns, from)?;
    let destinations = column(values, rows, columns, to)?;
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
        prediction_times: result.times,
        probabilities: result.probabilities,
    })
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
            0,
            1,
            2,
            3,
            4,
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
