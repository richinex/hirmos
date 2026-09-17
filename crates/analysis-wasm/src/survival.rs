//! Browser façades for the parity-tested flexsurv and ComparisonSurv kernels.

use super::*;
mod regression;
pub(crate) use regression::{aalen_evidence, forest_evidence};
use hirmos_causal_core::data_preparation::{
    prepare_longitudinal_states, prepare_wide_events, EventStatus, LongitudinalObservation,
    LongitudinalStateHistory, NonEmptyVec, StateId, StateObservation, SubjectId, TransitionMatrix,
    WideEventHistory, WideSubject,
};
use hirmos_causal_core::survival::comparison_surv::{
    comparison_curves, crossing_times, describe, fixed_time_conversion, g_rho_test,
    observed_conversion, overall_test, smooth_hazard_curves, ComparisonData, ComparisonError,
    ComparisonTime, DescriptiveError, Event, FixedPointResult, FixedPointScale, GRho, GRhoResult,
    Group, OverallConfiguration, OverallError, RmstWindow, SignificanceLevel, SurvivalSample,
};
use hirmos_causal_core::survival::aft::{AftFamily, AftPreparation};
use hirmos_causal_core::survival::coxph::{
    fit_clustered_breslow, ClusteredBreslowOptions, ClusteredConvergence,
    fit_gamma_frailty, fit_right_censored, fit_time_varying, BaselineCurves, CoxCovariates, CoxFit,
    CoxFitOptions, CoxPenalty, Event as CoxEvent, GammaFrailtyFit, GammaFrailtyOptions,
    ProportionalHazardsDiagnostics, RightCensoredData, StandardErrorMethod, TieMethod, TimeTransform,
    TimeVaryingData,
};
use spec_math::cephes64::{chdtrc, ndtri};
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
use hirmos_causal_core::survival::nonparametric::{
    kaplan_meier, nelson_aalen, EventStatus as NonparametricEventStatus, NelsonAalenTies,
    WeightedObservation,
};
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

fn cox_events(values: &[f64]) -> Result<Vec<CoxEvent>, String> {
    values
        .iter()
        .enumerate()
        .map(|(row, _)| {
            event_indicator(values, row).map(|observed| {
                if observed {
                    CoxEvent::Observed
                } else {
                    CoxEvent::Censored
                }
            })
        })
        .collect()
}

fn cox_group_column(
    values: &[f64],
    rows: usize,
    columns: usize,
    index: usize,
    role: &str,
) -> Result<Vec<usize>, String> {
    column(values, rows, columns, index)?
        .into_iter()
        .enumerate()
        .map(|(row, value)| {
            if value.is_finite()
                && value >= 0.0
                && value.fract() == 0.0
                && value <= usize::MAX as f64
            {
                Ok(value as usize)
            } else {
                Err(format!(
                    "Cox {role} row {} must be a non-negative whole-number code",
                    row + 1
                ))
            }
        })
        .collect()
}

fn cox_weights(
    values: &[f64],
    rows: usize,
    columns: usize,
    command: CoxWeightsCommand,
) -> Result<Vec<f64>, String> {
    let weights = match command {
        CoxWeightsCommand::Equal => vec![1.0; rows],
        CoxWeightsCommand::Column { column: index } => column(values, rows, columns, index)?,
    };
    if let Some(row) = weights
        .iter()
        .position(|weight| !weight.is_finite() || *weight <= 0.0)
    {
        return Err(format!(
            "Cox observation weight row {} must be finite and greater than zero",
            row + 1
        ));
    }
    Ok(weights)
}

fn cox_strata(
    values: &[f64],
    rows: usize,
    columns: usize,
    command: CoxStrataCommand,
) -> Result<Option<Vec<usize>>, String> {
    match command {
        CoxStrataCommand::Unstratified => Ok(None),
        CoxStrataCommand::Column { column: index } => {
            cox_group_column(values, rows, columns, index, "stratum").map(Some)
        }
    }
}

fn cox_covariates(
    values: &[f64],
    rows: usize,
    columns: usize,
    selected: &[usize],
    reserved: &[usize],
) -> Result<CoxCovariates, String> {
    if selected.is_empty() {
        return Err("Cox regression needs at least one covariate".to_owned());
    }
    let distinct = selected
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if distinct.len() != selected.len()
        || selected
            .iter()
            .any(|index| *index >= columns || reserved.contains(index))
    {
        return Err(
            "Cox covariates must be distinct matrix columns with no other analysis role".to_owned(),
        );
    }
    let mut design = Vec::with_capacity(rows.saturating_mul(selected.len()));
    for row in 0..rows {
        for index in selected {
            design.push(values[index * rows + row]);
        }
    }
    CoxCovariates::new(rows, selected.len(), design)
        .map_err(|problem| format!("Cox covariates refused: {problem:?}"))
}

fn cox_options(
    penalty: CoxPenaltyCommand,
    confidence_level: f64,
    standard_errors: StandardErrorMethod,
) -> Result<CoxFitOptions, String> {
    if !confidence_level.is_finite() || !(0.0..1.0).contains(&confidence_level) {
        return Err("Cox confidence level must be between 0 and 1".to_owned());
    }
    let (penalizer, l1_ratio) = match penalty {
        CoxPenaltyCommand::Unpenalized => (0.0, 0.0),
        CoxPenaltyCommand::ElasticNet {
            penalizer,
            l1_ratio,
        } => {
            if !penalizer.is_finite()
                || penalizer < 0.0
                || !l1_ratio.is_finite()
                || !(0.0..=1.0).contains(&l1_ratio)
            {
                return Err(
                    "Cox elastic-net penalizer must be non-negative and its L1 ratio must be between 0 and 1"
                        .to_owned(),
                );
            }
            (penalizer, l1_ratio)
        }
    };
    Ok(CoxFitOptions {
        penalizer: CoxPenalty::Uniform(penalizer),
        l1_ratio,
        alpha: 1.0 - confidence_level,
        standard_errors,
        ..CoxFitOptions::default()
    })
}

fn cox_baseline(curves: &BaselineCurves) -> CoxBaselineEvidence {
    let estimates = |values: &[hirmos_causal_core::survival::coxph::BaselineEstimate]| {
        values
            .iter()
            .map(|value| CoxBaselineEstimateEvidence {
                time: value.time,
                hazard: value.hazard,
                cumulative_hazard: value.cumulative_hazard,
                survival: value.survival,
            })
            .collect()
    };
    match curves {
        BaselineCurves::Shared(values) => CoxBaselineEvidence::Shared {
            estimates: estimates(values),
        },
        BaselineCurves::Stratified(values) => CoxBaselineEvidence::Stratified {
            curves: values
                .iter()
                .map(|curve| CoxStratumBaselineEvidence {
                    stratum: curve.stratum,
                    estimates: estimates(&curve.estimates),
                })
                .collect(),
        },
    }
}

fn cox_coefficients(fit: &CoxFit) -> Vec<CoxCoefficientEvidence> {
    fit.coefficients
        .iter()
        .map(|estimate| CoxCoefficientEvidence {
            coefficient: estimate.coefficient,
            hazard_ratio: estimate.hazard_ratio,
            standard_error: estimate.standard_error,
            coefficient_interval: [estimate.lower, estimate.upper],
            hazard_ratio_interval: [estimate.lower.exp(), estimate.upper.exp()],
            z: estimate.z,
            p_value: estimate.p_value,
        })
        .collect()
}

fn cox_proportional_hazards_tests(
    data: &RightCensoredData,
    fit: &CoxFit,
    delayed_entry: bool,
) -> CoxProportionalHazardsEvidence {
    if delayed_entry {
        return CoxProportionalHazardsEvidence::Unavailable {
            reason: "Proportional-hazards tests are unavailable with delayed entry.".to_owned(),
        };
    }
    let Ok(diagnostics) = ProportionalHazardsDiagnostics::new(data, fit) else {
        return CoxProportionalHazardsEvidence::Unavailable {
            reason: "Proportional-hazards tests could not be calculated for these observations."
                .to_owned(),
        };
    };
    let requested = [
        (
            CoxTimeTransformEvidence::EventRank,
            TimeTransform::EventRank,
        ),
        (
            CoxTimeTransformEvidence::KaplanMeier,
            TimeTransform::KaplanMeier,
        ),
        (CoxTimeTransformEvidence::Identity, TimeTransform::Identity),
        (CoxTimeTransformEvidence::LogTime, TimeTransform::LogTime),
    ];
    let transforms = requested
        .into_iter()
        .filter_map(|(evidence, transform)| {
            diagnostics.test(transform)
                .ok()
                .map(|tests| CoxProportionalHazardsTransformEvidence {
                    transform: evidence,
                    tests: tests
                        .into_iter()
                        .map(|test| CoxProportionalHazardsTestEvidence {
                            statistic: test.statistic,
                            p_value: test.p_value,
                        })
                        .collect(),
                })
        })
        .collect::<Vec<_>>();
    if transforms.is_empty() {
        CoxProportionalHazardsEvidence::Unavailable {
            reason: "Proportional-hazards tests could not be calculated for these observations."
                .to_owned(),
        }
    } else {
        CoxProportionalHazardsEvidence::Recorded { transforms }
    }
}

fn cox_standard_errors_evidence(method: StandardErrorMethod) -> CoxStandardErrorsEvidence {
    match method {
        StandardErrorMethod::ModelBased => CoxStandardErrorsEvidence::ModelBased,
        StandardErrorMethod::Sandwich => CoxStandardErrorsEvidence::Robust,
        StandardErrorMethod::Clustered => CoxStandardErrorsEvidence::Clustered,
    }
}


/// lifelines' `WeibullAFTFitter` and `LogLogisticAFTFitter` with an L2 penalty: the covariates
/// enter the location parameter with an intercept, the ancillary parameter is an intercept alone,
/// the design is scaled by sample standard deviations, and the fit starts from the univariate
/// distribution, as the source does.
#[allow(clippy::too_many_arguments)]
pub(crate) fn penalized_aft_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    duration: usize,
    event: usize,
    covariates: &[usize],
    family: AftFamilyCommand,
    penalizer: f64,
    confidence_level: f64,
    prediction_times: &[f64],
) -> Result<AnalysisResult, String> {
    if rows < 2 || columns == 0 || values.len() != rows.saturating_mul(columns) {
        return Err("A penalized AFT model needs at least two complete rows".to_owned());
    }
    if covariates.is_empty() {
        return Err("A penalized AFT model needs at least one covariate".to_owned());
    }
    let mut roles = vec![duration, event];
    roles.extend_from_slice(covariates);
    require_distinct_cox_roles(columns, &roles)?;
    if !penalizer.is_finite() || penalizer < 0.0 {
        return Err("The AFT penalizer must be a non-negative number".to_owned());
    }
    if !confidence_level.is_finite() || !(0.0..1.0).contains(&confidence_level) {
        return Err("AFT confidence level must be between 0 and 1".to_owned());
    }
    if prediction_times.is_empty() || prediction_times.iter().any(|t| !t.is_finite() || *t < 0.0) {
        return Err("AFT prediction times must be a non-empty list of non-negative values".to_owned());
    }
    let durations = column(values, rows, columns, duration)?;
    if let Some(row) = durations.iter().position(|t| !t.is_finite() || *t <= 0.0) {
        return Err(format!(
            "AFT row {} has a duration of zero or less; lifelines' AFT models take the logarithm of every duration",
            row + 1
        ));
    }
    let events = cox_events(&column(values, rows, columns, event)?)?
        .into_iter()
        .map(|e| e == CoxEvent::Observed)
        .collect::<Vec<_>>();
    let event_count = events.iter().filter(|e| **e).count();
    let k = covariates.len();
    let mut design = Vec::with_capacity(rows * (k + 2));
    // The matrix arrives column-major, as `column` reads it.
    for row in 0..rows {
        for &c in covariates {
            let value = values[c * rows + row];
            if !value.is_finite() {
                return Err(format!("AFT row {} has a non-finite covariate value", row + 1));
            }
            design.push(value);
        }
        design.extend([1.0, 1.0]);
    }
    let prepared = AftPreparation::new(k + 1, k + 2, design.clone(), durations.clone(), events.clone())
        .map_err(|problem| format!("AFT preparation refused: {problem:?}"))?;
    let aft_family = match family {
        AftFamilyCommand::Weibull => AftFamily::Weibull,
        AftFamilyCommand::LogLogistic => AftFamily::LogLogistic,
    };
    let fit = prepared
        .fit(aft_family, penalizer, 1.0 - confidence_level)
        .map_err(|problem| format!("AFT fit failed: {problem:?}"))?;
    let evidence = |c: &hirmos_causal_core::survival::aft::AftCoefficient| AftCoefficientEvidence {
        coefficient: c.estimate,
        time_ratio: c.estimate.exp(),
        standard_error: c.standard_error,
        coefficient_interval: [c.lower, c.upper],
        time_ratio_interval: [c.lower.exp(), c.upper.exp()],
        z: c.z,
        p_value: c.p_value,
    };
    let coefficients = fit.coefficients[..k].iter().map(evidence).collect::<Vec<_>>();
    let intercept = evidence(&fit.coefficients[k]);
    let ancillary = evidence(&fit.coefficients[k + 1]);
    let concordance = match fit
        .concordance(&design, &durations, &events)
        .map_err(|problem| format!("AFT concordance failed: {problem:?}"))?
    {
        Some(value) => SurvivalSummary::Recorded {
            result: value.value(),
        },
        None => SurvivalSummary::Unavailable {
            reason: "Concordance needs at least one comparable pair of observations.".to_owned(),
        },
    };
    // The curve at the covariate means: the location at the means and the fitted ancillary parameter.
    let covariate_means = (0..k)
        .map(|j| design.chunks_exact(k + 2).map(|row| row[j]).sum::<f64>() / rows as f64)
        .collect::<Vec<_>>();
    let location = (covariate_means
        .iter()
        .zip(&fit.coefficients[..k])
        .map(|(mean, c)| mean * c.estimate)
        .sum::<f64>()
        + fit.coefficients[k].estimate)
        .exp();
    let shape = fit.coefficients[k + 1].estimate.exp();
    let survival_at = |t: f64| match aft_family {
        AftFamily::Weibull => (-(t / location).powf(shape)).exp(),
        AftFamily::LogLogistic => 1.0 / (1.0 + (t / location).powf(shape)),
    };
    let survival = prediction_times.iter().map(|&t| survival_at(t)).collect::<Vec<_>>();
    let median = match aft_family {
        AftFamily::Weibull => location * std::f64::consts::LN_2.powf(1.0 / shape),
        AftFamily::LogLogistic => location,
    };
    Ok(AnalysisResult::PenalizedAft {
        observations: rows,
        events: event_count,
        family: match family {
            AftFamilyCommand::Weibull => AftFamilyEvidence::Weibull,
            AftFamilyCommand::LogLogistic => AftFamilyEvidence::LogLogistic,
        },
        penalizer,
        coefficients,
        intercept,
        ancillary,
        covariance: fit.covariance.clone(),
        log_likelihood: fit.log_likelihood,
        aic: fit.aic,
        bic: fit.bic,
        iterations: fit.iterations.max(1),
        concordance,
        covariate_means,
        prediction_times: prediction_times.to_vec(),
        survival,
        median,
    })
}

/// The Breslow or Efron baseline at the fitted linear predictors, as `survfit.coxph` forms it, for
/// the frailty model whose kernel does not return curves.
fn frailty_baseline(
    durations: &[f64],
    events: &[CoxEvent],
    weights: &[f64],
    strata: Option<&[usize]>,
    linear_predictors: &[f64],
    ties: TieMethod,
) -> CoxBaselineEvidence {
    let rows = durations.len();
    let curve = |rows: &[usize]| {
        let mut order = rows.to_vec();
        order.sort_by(|&a, &b| durations[a].total_cmp(&durations[b]));
        // The risk set at each time is every row from that position on; sum it once from the end.
        let mut at_risk = vec![0.0; order.len() + 1];
        for position in (0..order.len()).rev() {
            let row = order[position];
            at_risk[position] = at_risk[position + 1] + weights[row] * linear_predictors[row].exp();
        }
        let mut estimates = Vec::new();
        let mut cumulative = 0.0;
        let mut index = 0;
        while index < order.len() {
            let time = durations[order[index]];
            let mut end = index;
            while end < order.len() && durations[order[end]] == time {
                end += 1;
            }
            let tied = &order[index..end];
            let deaths = tied
                .iter()
                .filter(|&&row| events[row] == CoxEvent::Observed)
                .collect::<Vec<_>>();
            if !deaths.is_empty() {
                let denominator = at_risk[index];
                let death_weight = deaths.iter().map(|&&row| weights[row]).sum::<f64>();
                let hazard = match ties {
                    TieMethod::Breslow => death_weight / denominator,
                    TieMethod::Efron => {
                        let count = deaths.len() as f64;
                        let death_risk = deaths
                            .iter()
                            .map(|&&row| weights[row] * linear_predictors[row].exp())
                            .sum::<f64>();
                        let average = death_weight / count;
                        (0..deaths.len())
                            .map(|k| average / (denominator - death_risk * k as f64 / count))
                            .sum::<f64>()
                    }
                };
                cumulative += hazard;
                estimates.push(CoxBaselineEstimateEvidence {
                    time,
                    hazard,
                    cumulative_hazard: cumulative,
                    survival: (-cumulative).exp(),
                });
            }
            index = end;
        }
        estimates
    };
    match strata {
        None => CoxBaselineEvidence::Shared {
            estimates: curve(&(0..rows).collect::<Vec<_>>()),
        },
        Some(strata) => {
            let mut levels = strata.to_vec();
            levels.sort_unstable();
            levels.dedup();
            CoxBaselineEvidence::Stratified {
                curves: levels
                    .into_iter()
                    .map(|stratum| CoxStratumBaselineEvidence {
                        stratum,
                        estimates: curve(
                            &(0..rows)
                                .filter(|&row| strata[row] == stratum)
                                .collect::<Vec<_>>(),
                        ),
                    })
                    .collect(),
            }
        }
    }
}

/// Sample standard deviation of each covariate column, for the summaries the evidence carries.
fn covariate_standard_deviations(data: &RightCensoredData) -> Vec<f64> {
    let rows = data.rows();
    let columns = data.columns();
    let covariates = data.covariates();
    (0..columns)
        .map(|column| {
            let mean = (0..rows).map(|row| covariates.row(row)[column]).sum::<f64>() / rows as f64;
            let sum = (0..rows)
                .map(|row| (covariates.row(row)[column] - mean).powi(2))
                .sum::<f64>();
            (sum / (rows as f64 - 1.0)).sqrt()
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn cox_frailty_result(
    fit: &GammaFrailtyFit,
    data: &RightCensoredData,
    raw: (&[f64], &[CoxEvent], &[f64]),
    groups: &[usize],
    ties: TieMethod,
    observations: usize,
    events: usize,
    total_weight: f64,
    confidence_level: f64,
) -> AnalysisResult {
    // coxph reports Wald intervals with qnorm((1 + conf.int) / 2).
    let quantile = ndtri((1.0 + confidence_level) / 2.0);
    let coefficients = fit
        .coefficients
        .iter()
        .zip(&fit.standard_errors)
        .map(|(&coefficient, &standard_error)| {
            let z = coefficient / standard_error;
            CoxCoefficientEvidence {
                coefficient,
                hazard_ratio: coefficient.exp(),
                standard_error,
                coefficient_interval: [
                    coefficient - quantile * standard_error,
                    coefficient + quantile * standard_error,
                ],
                hazard_ratio_interval: [
                    (coefficient - quantile * standard_error).exp(),
                    (coefficient + quantile * standard_error).exp(),
                ],
                z,
                p_value: chdtrc(1.0, z * z),
            }
        })
        .collect::<Vec<_>>();
    let mut levels = groups.to_vec();
    levels.sort_unstable();
    levels.dedup();
    let count = fit.coefficients.len() as f64;
    AnalysisResult::CoxRegression {
        observations,
        events,
        total_weight,
        observation: CoxObservationEvidence::RightCensored {
            delayed_entry: false,
        },
        standard_errors: CoxStandardErrorsEvidence::ModelBased,
        fitting: CoxFittingEvidence::GammaFrailty,
        coefficients,
        covariance: fit.var.clone(),
        log_likelihood: fit.loglik[1],
        null_log_likelihood: fit.loglik[0],
        likelihood_ratio: fit.likelihood_ratio,
        likelihood_ratio_p_value: fit.likelihood_ratio_p_value,
        partial_aic: 2.0 * count - 2.0 * fit.loglik[1],
        iterations: fit.inner_iterations.max(1),
        covariate_means: fit.means.clone(),
        covariate_standard_deviations: covariate_standard_deviations(data),
        baseline: frailty_baseline(raw.0, raw.1, raw.2, data.strata(), &fit.linear_predictors, ties),
        concordance: match fit.concordance {
            Some(value) => SurvivalSummary::Recorded { result: value },
            None => SurvivalSummary::Unavailable {
                reason: "Concordance needs at least one comparable pair of observations.".to_owned(),
            },
        },
        proportional_hazards_tests: CoxProportionalHazardsEvidence::Unavailable {
            reason: "Proportional-hazards tests are not reported for a shared frailty model.".to_owned(),
        },
        frailty: CoxFrailtyEvidence::Gamma {
            groups: levels.len(),
            ties: match ties {
                TieMethod::Efron => CoxTiesEvidence::Efron,
                TieMethod::Breslow => CoxTiesEvidence::Breslow,
            },
            theta: fit.history.last().map_or(f64::NAN, |row| row[0]),
            term_test: CoxFrailtyTermTestEvidence {
                statistic: fit.frailty_test.statistic,
                df: fit.frailty_test.df,
                p_value: fit.frailty_test.p_value,
            },
            degrees_of_freedom: fit.likelihood_ratio_df,
            outer_iterations: fit.outer_iterations,
            inner_iterations: fit.inner_iterations,
            history: fit.history.clone(),
            standard_errors2: fit.standard_errors2.clone(),
        },
    }
}

fn cox_result(
    fit: CoxFit,
    observations: usize,
    events: usize,
    total_weight: f64,
    observation: CoxObservationEvidence,
    standard_errors: CoxStandardErrorsEvidence,
    concordance: SurvivalSummary<f64>,
    proportional_hazards_tests: CoxProportionalHazardsEvidence,
) -> AnalysisResult {
    AnalysisResult::CoxRegression {
        frailty: CoxFrailtyEvidence::None,
        fitting: CoxFittingEvidence::Efron,
        observations,
        events,
        total_weight,
        observation,
        standard_errors,
        coefficients: cox_coefficients(&fit),
        covariance: fit.covariance.clone(),
        log_likelihood: fit.log_likelihood,
        null_log_likelihood: fit.null_log_likelihood,
        likelihood_ratio: fit.likelihood_ratio,
        likelihood_ratio_p_value: fit.likelihood_ratio_p_value,
        partial_aic: fit.partial_aic,
        iterations: fit.iterations,
        covariate_means: fit.covariate_means.clone(),
        covariate_standard_deviations: fit.covariate_standard_deviations.clone(),
        baseline: cox_baseline(&fit.baseline),
        concordance,
        proportional_hazards_tests,
    }
}

fn clustered_breslow_result(
    durations: Vec<f64>,
    events: Vec<CoxEvent>,
    clusters: Option<Vec<usize>>,
    covariates: CoxCovariates,
    confidence_level: f64,
) -> Result<AnalysisResult, String> {
    if !confidence_level.is_finite() || confidence_level <= 0.0 || confidence_level >= 1.0 {
        return Err("Cox confidence level must be between 0 and 1.".to_owned());
    }
    let rows = durations.len();
    let event_count = events.iter().filter(|&&event| event == CoxEvent::Observed).count();
    let weights = vec![1.0; rows];
    let data = RightCensoredData::with_grouping(
        durations, events.clone(), weights.clone(), None, None, clusters, covariates,
    ).map_err(|problem| format!("Cox right-censored data refused: {problem:?}"))?;
    let fit = fit_clustered_breslow(&data, ClusteredBreslowOptions::default())
        .map_err(|problem| format!("Clustered Breslow regression failed: {problem:?}"))?;
    let p = fit.coefficients.len();
    let quantile = ndtri((1.0 + confidence_level) / 2.0);
    let coefficients = fit.coefficients.iter().enumerate().map(|(j, &coefficient)| {
        let standard_error = fit.covariance[j * p + j].sqrt();
        let z = coefficient / standard_error;
        let lower = coefficient - quantile * standard_error;
        let upper = coefficient + quantile * standard_error;
        CoxCoefficientEvidence {
            coefficient, hazard_ratio: coefficient.exp(), standard_error,
            coefficient_interval: [lower, upper], hazard_ratio_interval: [lower.exp(), upper.exp()],
            z, p_value: chdtrc(1.0, z * z),
        }
    }).collect();
    let baseline = frailty_baseline(&fit.times, &events, &weights, None, &fit.linear_predictors, TieMethod::Breslow);
    let likelihood_ratio = 2.0 * (fit.log_likelihood[1] - fit.log_likelihood[0]);
    Ok(AnalysisResult::CoxRegression {
        observations: rows, events: event_count, total_weight: rows as f64,
        observation: CoxObservationEvidence::RightCensored { delayed_entry: false },
        standard_errors: CoxStandardErrorsEvidence::Clustered,
        coefficients,
        // The shared result contract retains inverse-information covariance; the fitting record
        // explicitly stores the sandwich covariance used for clustered intervals.
        covariance: fit.naive_covariance,
        log_likelihood: fit.log_likelihood[1], null_log_likelihood: fit.log_likelihood[0],
        likelihood_ratio, likelihood_ratio_p_value: chdtrc(p as f64, likelihood_ratio),
        partial_aic: 2.0 * p as f64 - 2.0 * fit.log_likelihood[1],
        iterations: fit.iterations, covariate_means: fit.means,
        covariate_standard_deviations: covariate_standard_deviations(&data),
        baseline,
        concordance: match fit.concordance {
            Some(result) => SurvivalSummary::Recorded { result },
            None => SurvivalSummary::Unavailable { reason: "Concordance needs at least one comparable pair of observations.".to_owned() },
        },
        proportional_hazards_tests: CoxProportionalHazardsEvidence::Unavailable {
            reason: "Proportional-hazards tests are not available for the clustered Breslow fit.".to_owned(),
        },
        frailty: CoxFrailtyEvidence::None,
        fitting: CoxFittingEvidence::ClusteredBreslow {
            clusters: fit.clusters,
            convergence: match fit.convergence {
                ClusteredConvergence::Converged => CoxConvergenceEvidence::Converged,
                ClusteredConvergence::ConvergedDuringHalving => CoxConvergenceEvidence::ConvergedDuringHalving,
                ClusteredConvergence::IterationLimit => CoxConvergenceEvidence::IterationLimit,
            },
            robust_covariance: fit.covariance,
            score_test: fit.score_test, robust_score_test: fit.robust_score_test, wald_test: fit.wald_test,
        },
    })
}

fn cox_optional_column(command: CoxEntryCommand) -> Option<usize> {
    match command {
        CoxEntryCommand::NotUsed => None,
        CoxEntryCommand::Column { column } => Some(column),
    }
}

fn cox_weight_column(command: CoxWeightsCommand) -> Option<usize> {
    match command {
        CoxWeightsCommand::Equal => None,
        CoxWeightsCommand::Column { column } => Some(column),
    }
}

fn cox_stratum_column(command: CoxStrataCommand) -> Option<usize> {
    match command {
        CoxStrataCommand::Unstratified => None,
        CoxStrataCommand::Column { column } => Some(column),
    }
}

fn require_distinct_cox_roles(columns: usize, roles: &[usize]) -> Result<(), String> {
    if roles.iter().any(|index| *index >= columns) {
        return Err("Cox analysis roles must refer to columns inside the matrix".to_owned());
    }
    let distinct = roles
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if distinct.len() != roles.len() {
        return Err(
            "Cox duration, event, entry, weight, start, and stop roles must use different columns"
                .to_owned(),
        );
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn cox_regression_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    observation: CoxObservationCommand,
    weights_command: CoxWeightsCommand,
    strata_command: CoxStrataCommand,
    covariates: &[usize],
    penalty: CoxPenaltyCommand,
    confidence_level: f64,
) -> Result<AnalysisResult, String> {
    if rows < 2 || columns == 0 || values.len() != rows.saturating_mul(columns) {
        return Err("Cox regression needs at least two complete rows".to_owned());
    }
    let weights = cox_weights(values, rows, columns, weights_command)?;
    let total_weight = weights.iter().sum::<f64>();
    let strata = cox_strata(values, rows, columns, strata_command)?;
    let weight_column = cox_weight_column(weights_command);
    let stratum_column = cox_stratum_column(strata_command);

    match observation {
        CoxObservationCommand::RightCensored {
            duration,
            event,
            entry,
            standard_errors,
            frailty,
        } => {
            let entry_column = cox_optional_column(entry);
            let frailty_column = match frailty {
                CoxFrailtyCommand::None => None,
                CoxFrailtyCommand::Gamma { column, .. } => Some(column),
            };
            let (standard_error_method, cluster_column) = match standard_errors {
                CoxStandardErrorsCommand::ModelBased => (StandardErrorMethod::ModelBased, None),
                CoxStandardErrorsCommand::Robust => (StandardErrorMethod::Sandwich, None),
                CoxStandardErrorsCommand::Clustered { column } => {
                    (StandardErrorMethod::Clustered, Some(column))
                }
                CoxStandardErrorsCommand::ClusteredBreslow { column } => {
                    (StandardErrorMethod::Clustered, Some(column))
                }
            };
            let mut observation_roles = vec![duration, event];
            observation_roles.extend(entry_column);
            observation_roles.extend(weight_column);
            observation_roles.extend(frailty_column);
            require_distinct_cox_roles(columns, &observation_roles)?;
            let mut reserved = observation_roles;
            reserved.extend(stratum_column);
            reserved.extend(cluster_column);
            let covariates = cox_covariates(values, rows, columns, covariates, &reserved)?;
            let durations = column(values, rows, columns, duration)?;
            let events = cox_events(&column(values, rows, columns, event)?)?;
            let event_count = events
                .iter()
                .filter(|event| **event == CoxEvent::Observed)
                .count();
            let entries = entry_column
                .map(|index| column(values, rows, columns, index))
                .transpose()?;
            let clusters = cluster_column
                .map(|index| cox_group_column(values, rows, columns, index, "cluster"))
                .transpose()?;
            let raw = match frailty {
                CoxFrailtyCommand::Gamma { .. } => Some((durations.clone(), events.clone(), weights.clone())),
                CoxFrailtyCommand::None => None,
            };
            if matches!(standard_errors, CoxStandardErrorsCommand::ClusteredBreslow { .. }) {
                if !matches!(entry, CoxEntryCommand::NotUsed)
                    || !matches!(frailty, CoxFrailtyCommand::None)
                    || !matches!(weights_command, CoxWeightsCommand::Equal)
                    || !matches!(strata_command, CoxStrataCommand::Unstratified)
                    || !matches!(penalty, CoxPenaltyCommand::Unpenalized)
                {
                    return Err("Clustered Breslow requires equal weights, no delayed entry, no strata, no frailty and no penalty.".to_owned());
                }
                return clustered_breslow_result(durations, events, clusters, covariates, confidence_level);
            }
            let data = RightCensoredData::with_grouping(
                durations, events, weights, entries, strata, clusters, covariates,
            )
            .map_err(|problem| format!("Cox right-censored data refused: {problem:?}"))?;
            if let CoxFrailtyCommand::Gamma { column, ties } = frailty {
                let raw = raw.expect("raw copies are kept for the frailty path");
                if entry_column.is_some() {
                    return Err("A shared frailty model does not take a delayed-entry column.".to_owned());
                }
                if !matches!(standard_errors, CoxStandardErrorsCommand::ModelBased) {
                    return Err("A shared frailty model reports model-based standard errors only.".to_owned());
                }
                if !matches!(penalty, CoxPenaltyCommand::Unpenalized) {
                    return Err("A shared frailty model cannot also carry an elastic-net penalty.".to_owned());
                }
                if !confidence_level.is_finite() || !(0.0..1.0).contains(&confidence_level) {
                    return Err("Cox confidence level must be between 0 and 1".to_owned());
                }
                let groups = cox_group_column(values, rows, columns, column, "frailty group")?;
                let ties = match ties {
                    CoxTiesCommand::Efron => TieMethod::Efron,
                    CoxTiesCommand::Breslow => TieMethod::Breslow,
                };
                let options = GammaFrailtyOptions {
                    ties,
                    ..GammaFrailtyOptions::default()
                };
                let fit = fit_gamma_frailty(&data, &groups, &options)
                    .map_err(|problem| format!("Shared frailty Cox regression failed: {problem:?}"))?;
                return Ok(cox_frailty_result(
                    &fit,
                    &data,
                    (&raw.0, &raw.1, &raw.2),
                    &groups,
                    ties,
                    rows,
                    event_count,
                    total_weight,
                    confidence_level,
                ));
            }
            let options = cox_options(penalty, confidence_level, standard_error_method)?;
            let fit = fit_right_censored(&data, &options)
                .map_err(|problem| format!("Cox regression failed: {problem:?}"))?;
            let delayed_entry = entry_column.is_some();
            let concordance = match fit.concordance_index {
                Some(value) => SurvivalSummary::Recorded {
                    result: value.value(),
                },
                None => SurvivalSummary::Unavailable {
                    reason: "Concordance needs at least one comparable pair of observations."
                        .to_owned(),
                },
            };
            let proportional_hazards_tests =
                cox_proportional_hazards_tests(&data, &fit.model, delayed_entry);
            Ok(cox_result(
                fit.model,
                rows,
                event_count,
                total_weight,
                CoxObservationEvidence::RightCensored { delayed_entry },
                cox_standard_errors_evidence(standard_error_method),
                concordance,
                proportional_hazards_tests,
            ))
        }
        CoxObservationCommand::StartStop {
            subject,
            start,
            stop,
            event,
        } => {
            let mut observation_roles = vec![subject, start, stop, event];
            observation_roles.extend(weight_column);
            require_distinct_cox_roles(columns, &observation_roles)?;
            let mut reserved = observation_roles;
            reserved.extend(stratum_column);
            let covariates = cox_covariates(values, rows, columns, covariates, &reserved)?;
            let subjects = cox_group_column(values, rows, columns, subject, "subject")?;
            let subject_count = subjects
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            let starts = column(values, rows, columns, start)?;
            let stops = column(values, rows, columns, stop)?;
            let events = cox_events(&column(values, rows, columns, event)?)?;
            let event_count = events
                .iter()
                .filter(|event| **event == CoxEvent::Observed)
                .count();
            let data = TimeVaryingData::with_strata(
                subjects, starts, stops, events, weights, strata, covariates,
            )
            .map_err(|problem| format!("Cox start-stop data refused: {problem:?}"))?;
            let options = cox_options(penalty, confidence_level, StandardErrorMethod::ModelBased)?;
            let fit = fit_time_varying(&data, &options)
                .map_err(|problem| format!("Cox regression failed: {problem:?}"))?;
            Ok(cox_result(
                fit,
                rows,
                event_count,
                total_weight,
                CoxObservationEvidence::StartStop {
                    subjects: subject_count,
                },
                CoxStandardErrorsEvidence::ModelBased,
                SurvivalSummary::Unavailable {
                    reason: "Concordance is not calculated for start-stop Cox regression."
                        .to_owned(),
                },
                CoxProportionalHazardsEvidence::Unavailable {
                    reason: "Proportional-hazards tests are not calculated for start-stop Cox regression."
                        .to_owned(),
                },
            ))
        }
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

fn survival_row_weights(
    values: &[f64],
    rows: usize,
    columns: usize,
    command: SurvivalRowFrequencyCommand,
    reserved: &[usize],
) -> Result<Vec<f64>, String> {
    match command {
        SurvivalRowFrequencyCommand::OneObservationPerRow => Ok(vec![1.0; rows]),
        SurvivalRowFrequencyCommand::FrequencyColumn { column: index } => {
            if reserved.contains(&index) {
                return Err(
                    "the survival frequency must use a column with no other analysis role"
                        .to_owned(),
                );
            }
            let values = column(values, rows, columns, index)?;
            for (row, value) in values.iter().enumerate() {
                if !value.is_finite() || *value <= 0.0 || value.fract() != 0.0 {
                    return Err(format!(
                        "invalid survival frequency at row {}: use a positive whole-number count",
                        row + 1
                    ));
                }
            }
            Ok(values)
        }
    }
}

pub(crate) fn flexsurv_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    observation: SurvivalObservationCommand,
    row_frequency: SurvivalRowFrequencyCommand,
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
    let weights = survival_row_weights(
        values,
        rows,
        columns,
        row_frequency,
        &[observation_columns.as_slice(), covariates].concat(),
    )?;
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

    let data = SurvivalDataset::new(
        observations
            .into_iter()
            .zip(weights.iter().copied())
            .map(|(observation, weight)| SurvivalRecord::weighted(observation, weight))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|problem| format!("survival weights refused: {problem:?}"))?,
    )
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
        let weight_total = weights.iter().sum::<f64>();
        for index in covariates {
            let values = column(values, rows, columns, *index)?;
            profile.push(
                values
                    .iter()
                    .zip(&weights)
                    .map(|(value, weight)| value * weight)
                    .sum::<f64>()
                    / weight_total,
            );
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
        observations: weights.iter().sum::<f64>() as usize,
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
            .map(|(row, _)| weights[row] as usize)
            .sum(),
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

pub(crate) fn nonparametric_survival_evidence(
    values: &[f64],
    rows: usize,
    columns: usize,
    duration: usize,
    event: usize,
    row_frequency: SurvivalRowFrequencyCommand,
    prediction_times: &[f64],
    ties: NelsonAalenTiesCommand,
) -> Result<AnalysisResult, String> {
    if rows < 2 || columns < 2 || values.len() != rows.saturating_mul(columns) {
        return Err("nonparametric survival needs at least two complete rows".to_owned());
    }
    if duration == event || duration >= columns || event >= columns {
        return Err("duration and event must be different columns inside the matrix".to_owned());
    }
    if prediction_times.is_empty()
        || prediction_times.windows(2).any(|pair| pair[0] > pair[1])
        || prediction_times
            .iter()
            .any(|time| !time.is_finite() || *time < 0.0)
    {
        return Err("nonparametric prediction times must be non-negative and ordered".to_owned());
    }
    let durations = column(values, rows, columns, duration)?;
    let events = column(values, rows, columns, event)?;
    let weights = survival_row_weights(values, rows, columns, row_frequency, &[duration, event])?;
    let mut observations = Vec::with_capacity(rows);
    for row in 0..rows {
        observations.push(
            WeightedObservation::new(
                durations[row],
                if event_indicator(&events, row)? {
                    NonparametricEventStatus::Observed
                } else {
                    NonparametricEventStatus::Censored
                },
                weights[row],
            )
            .map_err(|problem| {
                format!(
                    "invalid nonparametric survival row {}: {problem:?}",
                    row + 1
                )
            })?,
        );
    }
    let survival = kaplan_meier(&observations, prediction_times, 0.05)
        .map_err(|problem| format!("Kaplan-Meier failed: {problem:?}"))?;
    let cumulative_hazard = nelson_aalen(
        &observations,
        prediction_times,
        0.05,
        match ties {
            NelsonAalenTiesCommand::Discrete => NelsonAalenTies::Discrete,
            NelsonAalenTiesCommand::Smoothed => NelsonAalenTies::Smoothed,
        },
    )
    .map_err(|problem| format!("Nelson-Aalen failed: {problem:?}"))?;
    let mut previous = 0.0;
    let hazard_increment = cumulative_hazard
        .iter()
        .map(|estimate| {
            let increment = estimate.cumulative_hazard - previous;
            previous = estimate.cumulative_hazard;
            increment.max(0.0)
        })
        .collect();
    Ok(AnalysisResult::NonparametricSurvival {
        observations: weights.iter().sum::<f64>() as usize,
        events: events
            .iter()
            .zip(&weights)
            .filter(|(event, _)| **event == 1.0)
            .map(|(_, weight)| *weight as usize)
            .sum(),
        prediction_times: prediction_times.to_vec(),
        survival: survival.iter().map(|estimate| estimate.survival).collect(),
        survival_lower: survival.iter().map(|estimate| estimate.lower).collect(),
        survival_upper: survival.iter().map(|estimate| estimate.upper).collect(),
        cumulative_density: survival
            .iter()
            .map(|estimate| estimate.cumulative_density)
            .collect(),
        cumulative_hazard: cumulative_hazard
            .iter()
            .map(|estimate| estimate.cumulative_hazard)
            .collect(),
        cumulative_hazard_lower: cumulative_hazard
            .iter()
            .map(|estimate| estimate.lower)
            .collect(),
        cumulative_hazard_upper: cumulative_hazard
            .iter()
            .map(|estimate| estimate.upper)
            .collect(),
        hazard_increment,
    })
}

/// The last observed time of the group that is followed for less long: the latest time the
/// restricted mean can be taken to.
fn shortest_group_follow_up(durations: &[f64], groups: &[f64]) -> f64 {
    let latest = |wanted: f64| {
        durations
            .iter()
            .zip(groups)
            .filter(|(_, group)| **group == wanted)
            .map(|(time, _)| *time)
            .fold(f64::NEG_INFINITY, f64::max)
    };
    latest(0.0).min(latest(1.0))
}

/// A refusal from the two-group comparison as a sentence that names the value to change.
fn comparison_problem(problem: &OverallError, truncation_time: f64, follow_up: f64) -> String {
    match problem {
        OverallError::InvalidTruncationTime
        | OverallError::Description(DescriptiveError::InvalidTruncationTime) => format!(
            "Compare through time is {truncation_time}, but follow-up in the shorter group ends at {follow_up}; choose a time at or before that."
        ),
        OverallError::Comparison(
            ComparisonError::ComparisonTimeTooEarly
            | ComparisonError::ComparisonTimeTooLate
            | ComparisonError::FixedTimeOutsideEventRange,
        ) => format!(
            "Compare through time is {truncation_time}; the comparison needs a time after the first event and before the last one."
        ),
        other => format!("The two-group comparison could not be computed ({other:?})."),
    }
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
    let follow_up = shortest_group_follow_up(&durations, &groups);
    let result = overall_test(
        &data,
        OverallConfiguration::new(truncation_time, permutations, seed)
            .map_err(|problem| comparison_problem(&problem, truncation_time, follow_up))?,
    )
    .map_err(|problem| comparison_problem(&problem, truncation_time, follow_up))?;
    let description = describe(
        &data,
        RmstWindow::At(
            ComparisonTime::new(truncation_time)
                .map_err(|problem| comparison_problem(&OverallError::Description(problem), truncation_time, follow_up))?,
        ),
        SignificanceLevel::new(0.05)
            .map_err(|problem| format!("survival comparison interval refused: {problem:?}"))?,
    )
    .map_err(|problem| comparison_problem(&OverallError::Description(problem), truncation_time, follow_up))?;
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
        // One prepared row per permitted move per spell: the fit for a transition takes the rows
        // addressed to it, so a spell at risk of several moves is counted once in each fit.
        let selected = (0..rows)
            .filter(|row| {
                state_index(origins[*row]) == origin
                    && state_index(destinations[*row]) == destination
            })
            .collect::<Vec<_>>();
        let mut records = Vec::with_capacity(selected.len());
        for row in selected {
            let observed = event_indicator(&events, row)?;
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

    #[test]
    fn clustered_breslow_matches_r_through_the_browser_contract() {
        use std::{fs, path::Path};
        let numbers = |path: &Path| -> Vec<f64> {
            fs::read_to_string(path).unwrap().split(|c: char| c == ',' || c.is_whitespace())
                .filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect()
        };
        for name in ["kidney", "tied", "near_tied"] {
            let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../causal-core/oracle/fixtures/clustered_breslow").join(name);
            let csv = fs::read_to_string(path.join("input.csv")).unwrap();
            let rows = csv.lines().map(|line| line.split(',').map(|v| v.parse::<f64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
            let columns = rows[0].len();
            let values = (0..columns).flat_map(|j| rows.iter().map(move |row| row[j])).collect::<Vec<_>>();
            let result = cox_regression_evidence(
                &values, rows.len(), columns,
                CoxObservationCommand::RightCensored {
                    duration: 0, event: 1, entry: CoxEntryCommand::NotUsed,
                    standard_errors: CoxStandardErrorsCommand::ClusteredBreslow { column: 2 },
                    frailty: CoxFrailtyCommand::None,
                },
                CoxWeightsCommand::Equal, CoxStrataCommand::Unstratified,
                &(3..columns).collect::<Vec<_>>(), CoxPenaltyCommand::Unpenalized, 0.95,
            ).unwrap();
            let json = serde_json::to_value(result).unwrap();
            let beta = numbers(&path.join("coefficients.csv"));
            let covariance = numbers(&path.join("covariance.csv"));
            let naive = numbers(&path.join("naive.csv"));
            for (j, &expected) in beta.iter().enumerate() {
                let row = &json["coefficients"][j];
                assert!((row["coefficient"].as_f64().unwrap() - expected).abs() < 2e-8, "{name} beta {j}");
                assert!((row["standardError"].as_f64().unwrap().powi(2) - covariance[j * beta.len() + j]).abs() < 2e-8);
            }
            for j in 0..covariance.len() {
                assert!((json["fitting"]["robustCovariance"][j].as_f64().unwrap() - covariance[j]).abs() < 2e-8);
                assert!((json["covariance"][j].as_f64().unwrap() - naive[j]).abs() < 2e-8);
            }
            assert_eq!(json["fitting"]["kind"], "clusteredBreslow");
            assert_eq!(json["fitting"]["convergence"], "converged");
            assert_eq!(json["concordance"]["kind"], "recorded");
        }
    }

    fn cox_case(section: &str, name: &str) -> serde_json::Value {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../causal-core/oracle/fixtures/lifelines_coxph.json"
        ))
        .expect("Cox fixture is valid JSON");
        fixture[section]
            .as_array()
            .expect("Cox fixture section is an array")
            .iter()
            .find(|case| case["name"] == name)
            .expect("named Cox fixture exists")
            .clone()
    }

    fn numeric_values(value: &serde_json::Value) -> Vec<f64> {
        value
            .as_array()
            .expect("fixture value is an array")
            .iter()
            .map(|value| value.as_f64().expect("fixture value is numeric"))
            .collect()
    }

    fn event_values(value: &serde_json::Value) -> Vec<f64> {
        value
            .as_array()
            .expect("fixture events are an array")
            .iter()
            .map(|value| f64::from(value.as_bool().expect("fixture event is Boolean")))
            .collect()
    }

    fn cox_matrix(input: &serde_json::Value, observation_columns: &[&str]) -> (Vec<f64>, usize) {
        let mut values = Vec::new();
        for name in observation_columns {
            if *name == "event" {
                values.extend(event_values(&input[*name]));
            } else {
                values.extend(numeric_values(&input[*name]));
            }
        }
        let covariates = input["covariates"]
            .as_array()
            .expect("fixture covariates are rows");
        let rows = covariates.len();
        let columns = covariates[0]
            .as_array()
            .expect("fixture covariate row is an array")
            .len();
        for column in 0..columns {
            values.extend(
                covariates
                    .iter()
                    .map(|row| row[column].as_f64().expect("fixture covariate is numeric")),
            );
        }
        (values, rows)
    }

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
    fn right_censored_cox_fixture_reaches_the_browser_facade() {
        let case = cox_case("right_censored", "one_covariate_no_forced_batch");
        let input = &case["input"];
        let (values, rows) = cox_matrix(input, &["duration", "event"]);
        let result = cox_regression_evidence(
            &values,
            rows,
            3,
            CoxObservationCommand::RightCensored {
                duration: 0,
                event: 1,
                entry: CoxEntryCommand::NotUsed,
                standard_errors: CoxStandardErrorsCommand::ModelBased,
                frailty: CoxFrailtyCommand::None,
            },
            CoxWeightsCommand::Equal,
            CoxStrataCommand::Unstratified,
            &[2],
            CoxPenaltyCommand::Unpenalized,
            0.95,
        )
        .expect("the lifelines right-censored fixture fits through the facade");
        let serialized = serde_json::to_value(&result).expect("Cox evidence serializes");
        assert_eq!(serialized["kind"], "coxRegression");
        assert_eq!(serialized["observation"]["kind"], "rightCensored");
        assert_eq!(serialized["baseline"]["kind"], "shared");
        assert_eq!(serialized["concordance"]["kind"], "recorded");
        assert_eq!(serialized["proportionalHazardsTests"]["kind"], "recorded");
        assert_eq!(serialized["coefficients"].as_array().unwrap().len(), 1);
        let actual = serialized["coefficients"][0]["coefficient"]
            .as_f64()
            .unwrap();
        let expected = case["fit"]["coefficients"][0].as_f64().unwrap();
        assert!((actual - expected).abs() <= 2e-9);
    }

    #[test]
    fn cox_command_accepts_the_browser_contract() {
        let command: AnalysisCommand = serde_json::from_str(
            r#"{"kind":"coxRegression","rows":18,"columns":4,"observation":{"kind":"rightCensored","duration":0,"event":1,"entry":{"kind":"notUsed"},"standardErrors":{"kind":"clustered","column":2}},"weights":{"kind":"equal"},"strata":{"kind":"unstratified"},"covariates":[3],"penalty":{"kind":"elasticNet","penalizer":0.1,"l1Ratio":0.25},"confidenceLevel":0.95}"#,
        )
        .expect("browser Cox command should parse");
        let AnalysisCommand::CoxRegression {
            rows,
            columns,
            observation:
                CoxObservationCommand::RightCensored {
                    duration,
                    event,
                    entry: CoxEntryCommand::NotUsed,
                    standard_errors: CoxStandardErrorsCommand::Clustered { column: cluster },
                    frailty: _,
                },
            weights: CoxWeightsCommand::Equal,
            strata: CoxStrataCommand::Unstratified,
            covariates,
            penalty:
                CoxPenaltyCommand::ElasticNet {
                    penalizer,
                    l1_ratio,
                },
            confidence_level,
        } = command
        else {
            panic!("browser command parsed as another Cox state")
        };
        assert_eq!((rows, columns, duration, event, cluster), (18, 4, 0, 1, 2));
        assert_eq!(covariates, vec![3]);
        assert_eq!((penalizer, l1_ratio, confidence_level), (0.1, 0.25, 0.95));
    }

    #[test]
    fn start_stop_cox_fixture_reaches_the_browser_facade() {
        let case = cox_case("time_varying", "two_covariates");
        let input = &case["input"];
        let (mut values, rows) = cox_matrix(input, &["start", "stop", "event"]);
        let subjects = input["subject"]
            .as_array()
            .expect("fixture subjects are an array")
            .iter()
            .map(|value| value.as_u64().expect("fixture subject is numeric") as f64)
            .collect::<Vec<_>>();
        values.splice(0..0, subjects);
        let result = cox_regression_evidence(
            &values,
            rows,
            6,
            CoxObservationCommand::StartStop {
                subject: 0,
                start: 1,
                stop: 2,
                event: 3,
            },
            CoxWeightsCommand::Equal,
            CoxStrataCommand::Unstratified,
            &[4, 5],
            CoxPenaltyCommand::Unpenalized,
            0.95,
        )
        .expect("the lifelines start-stop fixture fits through the facade");
        let serialized = serde_json::to_value(&result).expect("Cox evidence serializes");
        assert_eq!(serialized["kind"], "coxRegression");
        assert_eq!(serialized["observation"]["kind"], "startStop");
        assert_eq!(serialized["baseline"]["kind"], "shared");
        assert_eq!(serialized["concordance"]["kind"], "unavailable");
        assert_eq!(
            serialized["proportionalHazardsTests"]["kind"],
            "unavailable"
        );
        assert_eq!(serialized["coefficients"].as_array().unwrap().len(), 2);
        for (actual, expected) in serialized["coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .zip(case["fit"]["coefficients"].as_array().unwrap())
        {
            assert!(
                (actual["coefficient"].as_f64().unwrap() - expected.as_f64().unwrap()).abs()
                    <= 2e-7
            );
        }
    }

    #[test]
    fn delayed_entry_and_clustered_errors_keep_distinct_evidence() {
        let delayed = cox_case("right_censored", "delayed_entry");
        let delayed_input = &delayed["input"];
        let (mut delayed_values, rows) = cox_matrix(delayed_input, &["duration", "event"]);
        delayed_values.splice(2 * rows..2 * rows, numeric_values(&delayed_input["entry"]));
        let delayed_result = cox_regression_evidence(
            &delayed_values,
            rows,
            6,
            CoxObservationCommand::RightCensored {
                duration: 0,
                event: 1,
                entry: CoxEntryCommand::Column { column: 2 },
                standard_errors: CoxStandardErrorsCommand::ModelBased,
                frailty: CoxFrailtyCommand::None,
            },
            CoxWeightsCommand::Equal,
            CoxStrataCommand::Unstratified,
            &[3, 4, 5],
            CoxPenaltyCommand::Unpenalized,
            0.95,
        )
        .expect("the lifelines delayed-entry fixture fits through the facade");
        let delayed_json = serde_json::to_value(delayed_result).unwrap();
        assert_eq!(delayed_json["observation"]["delayedEntry"], true);
        assert_eq!(
            delayed_json["proportionalHazardsTests"]["kind"],
            "unavailable"
        );

        let clustered = cox_case("right_censored", "clustered");
        let clustered_input = &clustered["input"];
        let (mut clustered_values, rows) = cox_matrix(clustered_input, &["duration", "event"]);
        clustered_values.splice(
            2 * rows..2 * rows,
            numeric_values(&clustered_input["cluster"]),
        );
        let clustered_result = cox_regression_evidence(
            &clustered_values,
            rows,
            6,
            CoxObservationCommand::RightCensored {
                duration: 0,
                event: 1,
                entry: CoxEntryCommand::NotUsed,
                standard_errors: CoxStandardErrorsCommand::Clustered { column: 2 },
                frailty: CoxFrailtyCommand::None,
            },
            CoxWeightsCommand::Equal,
            CoxStrataCommand::Unstratified,
            &[3, 4, 5],
            CoxPenaltyCommand::Unpenalized,
            0.95,
        )
        .expect("the lifelines clustered fixture fits through the facade");
        let clustered_json = serde_json::to_value(clustered_result).unwrap();
        assert_eq!(clustered_json["standardErrors"], "clustered");
        for (actual, expected) in clustered_json["coefficients"]
            .as_array()
            .unwrap()
            .iter()
            .zip(clustered["fit"]["standard_errors"].as_array().unwrap())
        {
            assert!(
                (actual["standardError"].as_f64().unwrap() - expected.as_f64().unwrap()).abs()
                    <= 2e-7
            );
        }
    }

    #[test]
    fn weights_strata_and_elastic_net_reach_their_typed_lanes() {
        let weighted = cox_case("right_censored", "weighted");
        let input = &weighted["input"];
        let (mut values, rows) = cox_matrix(input, &["duration", "event"]);
        values.splice(2 * rows..2 * rows, numeric_values(&input["weights"]));
        let result = cox_regression_evidence(
            &values,
            rows,
            6,
            CoxObservationCommand::RightCensored {
                duration: 0,
                event: 1,
                entry: CoxEntryCommand::NotUsed,
                standard_errors: CoxStandardErrorsCommand::ModelBased,
                frailty: CoxFrailtyCommand::None,
            },
            CoxWeightsCommand::Column { column: 2 },
            CoxStrataCommand::Unstratified,
            &[3, 4, 5],
            CoxPenaltyCommand::Unpenalized,
            0.95,
        )
        .expect("the lifelines weighted fixture fits through the facade");
        let result = serde_json::to_value(result).unwrap();
        assert!(result["totalWeight"].as_f64().unwrap() > rows as f64);

        let stratified = cox_case("right_censored", "stratified_robust");
        let input = &stratified["input"];
        let (mut values, rows) = cox_matrix(input, &["duration", "event"]);
        let strata = input["strata"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| if value == "north" { 0.0 } else { 1.0 })
            .collect::<Vec<_>>();
        values.splice(2 * rows..2 * rows, strata);
        let result = cox_regression_evidence(
            &values,
            rows,
            6,
            CoxObservationCommand::RightCensored {
                duration: 0,
                event: 1,
                entry: CoxEntryCommand::NotUsed,
                standard_errors: CoxStandardErrorsCommand::Robust,
                frailty: CoxFrailtyCommand::None,
            },
            CoxWeightsCommand::Equal,
            CoxStrataCommand::Column { column: 2 },
            &[3, 4, 5],
            CoxPenaltyCommand::ElasticNet {
                penalizer: 0.0,
                l1_ratio: 0.0,
            },
            0.95,
        )
        .expect("the lifelines stratified robust fixture fits through the facade");
        let result = serde_json::to_value(result).unwrap();
        assert_eq!(result["standardErrors"], "robust");
        assert_eq!(result["baseline"]["kind"], "stratified");
        assert_eq!(result["baseline"]["curves"].as_array().unwrap().len(), 2);
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
            SurvivalRowFrequencyCommand::OneObservationPerRow,
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
    fn grouped_nonparametric_rows_match_expanded_observations_at_the_browser_facade() {
        let grouped_values = vec![1.0, 2.0, 3.0, 4.0, 1.0, 1.0, 0.0, 0.0, 2.0, 1.0, 3.0, 4.0];
        let expanded_values = vec![
            1.0, 1.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        let times = [0.0, 1.0, 2.0, 3.0, 4.0];
        let grouped = nonparametric_survival_evidence(
            &grouped_values,
            4,
            3,
            0,
            1,
            SurvivalRowFrequencyCommand::FrequencyColumn { column: 2 },
            &times,
            NelsonAalenTiesCommand::Discrete,
        )
        .expect("grouped observations should be accepted");
        let expanded = nonparametric_survival_evidence(
            &expanded_values,
            10,
            2,
            0,
            1,
            SurvivalRowFrequencyCommand::OneObservationPerRow,
            &times,
            NelsonAalenTiesCommand::Discrete,
        )
        .expect("expanded observations should be accepted");

        let AnalysisResult::NonparametricSurvival {
            observations: grouped_observations,
            events: grouped_events,
            survival: grouped_survival,
            cumulative_hazard: grouped_hazard,
            ..
        } = grouped
        else {
            panic!("grouped command returned another result kind")
        };
        let AnalysisResult::NonparametricSurvival {
            observations: expanded_observations,
            events: expanded_events,
            survival: expanded_survival,
            cumulative_hazard: expanded_hazard,
            ..
        } = expanded
        else {
            panic!("expanded command returned another result kind")
        };
        assert_eq!(grouped_observations, expanded_observations);
        assert_eq!(grouped_events, expanded_events);
        assert_eq!(grouped_survival, expanded_survival);
        assert_eq!(grouped_hazard, expanded_hazard);
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
    fn comparison_past_the_shorter_follow_up_names_the_day_to_stay_within() {
        // Group 0 of the crossing data is last observed at 4.103844950351483; group 1 runs to 4.67.
        let (values, rows, columns) = selected_columns(
            include_str!("../../../public/examples/data/survival/comparison_surv_crossdata.csv"),
            &["time", "status", "group"],
        );
        let Err(problem) = comparison_survival_evidence(&values, rows, columns, 0, 1, 2, 4.5, 19, 43) else {
            panic!("a comparison time past group 0's last observation is refused")
        };
        assert_eq!(
            problem,
            "Compare through time is 4.5, but follow-up in the shorter group ends at 4.103844950351483; choose a time at or before that."
        );
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
            SurvivalRowFrequencyCommand::OneObservationPerRow,
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

    #[test]
    fn grouped_frequency_rows_match_the_equivalent_expanded_flexsurv_data() {
        let grouped = vec![1.0, 2.0, 3.0, 4.0, 1.0, 1.0, 0.0, 0.0, 2.0, 3.0, 4.0, 1.0];
        let expanded = vec![
            1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ];
        let grouped_result = flexsurv_evidence(
            &grouped,
            4,
            3,
            SurvivalObservationCommand::RightCensored {
                duration: 0,
                event: 1,
            },
            SurvivalRowFrequencyCommand::FrequencyColumn { column: 2 },
            &[],
            SurvivalFamily::Weibull,
            &[1.0, 2.0, 4.0],
        )
        .expect("positive integer frequency rows fit");
        let expanded_result = flexsurv_evidence(
            &expanded,
            10,
            2,
            SurvivalObservationCommand::RightCensored {
                duration: 0,
                event: 1,
            },
            SurvivalRowFrequencyCommand::OneObservationPerRow,
            &[],
            SurvivalFamily::Weibull,
            &[1.0, 2.0, 4.0],
        )
        .expect("the expanded observations fit");
        let (
            AnalysisResult::FlexSurv {
                observations: grouped_observations,
                events: grouped_events,
                natural_baseline: grouped_parameters,
                log_likelihood: grouped_likelihood,
                ..
            },
            AnalysisResult::FlexSurv {
                observations: expanded_observations,
                events: expanded_events,
                natural_baseline: expanded_parameters,
                log_likelihood: expanded_likelihood,
                ..
            },
        ) = (grouped_result, expanded_result)
        else {
            panic!("flexsurv commands returned another result kind")
        };
        assert_eq!((grouped_observations, grouped_events), (10, 5));
        assert_eq!(
            (grouped_observations, grouped_events),
            (expanded_observations, expanded_events)
        );
        assert!((grouped_likelihood - expanded_likelihood).abs() <= 1e-10);
        for (grouped, expanded) in grouped_parameters.iter().zip(expanded_parameters) {
            assert!((grouped - expanded).abs() <= 1e-9);
        }
    }
}
