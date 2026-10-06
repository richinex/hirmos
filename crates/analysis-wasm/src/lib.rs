#![allow(clippy::too_many_arguments)]
//! Thin browser command façade over the parity-tested causal kernels.
//!
//! The façade owns browser-facing validation and serialization. It does not reinterpret test
//! evidence or copy the numerical implementations out of the Hirmos causal core.

mod causal_forest;
mod swig_analysis;
mod honest_did;
mod did_sensitivity;
mod network_query;
mod conditional_gaussian_query;
mod causal_forest_analysis;
use hirmos_causal_core::ardl::{ardl_select_order, bounds_test, uecm, Trend};
use hirmos_causal_core::bayesian_gaussian::{posterior_effect_summary, BayesianGaussianScm};
use hirmos_causal_core::causal_effects::{
    mark, AdjustmentSetError, AdjustmentSetSelection, BootstrapBlockLength, CausalEffects,
    Estimator, ExplicitAdjustmentProblem, Node, StationaryGraph, TotalEffectBootstrapError,
    TotalEffectBootstrapOptions, WrightCoefficientMethod, WrightEffectError, WrightMediation,
};
use hirmos_causal_core::causal_impact::causal_impact;
use hirmos_causal_core::causal_ts_preparation::{
    causal_ts_var_em, prepare_grace_dense, CausalTsContextPreset, CdnotsMissingPolicy,
    VarEmConfiguration,
};
use hirmos_causal_core::cdnots::{
    run_cdnots_plus_with_progress, run_cdnots_with_progress, CdnotsConfiguration,
    CdnotsPlusConfiguration, ColliderPriority,
};
use hirmos_causal_core::counterfactual::{Equation, LinearScm};
use hirmos_causal_core::counterfactual_evaluator::{estimate_binary_ett, identify_binary_ett};
use hirmos_causal_core::discrete_bn::{
    discretize_for_discrete_bn, Dag as DiscreteDag, DiscreteBn,
    DiscreteStateProblem as CoreDiscreteStateProblem,
    DiscreteStateStrategy as CoreDiscreteStateStrategy, StateBudget,
};
use hirmos_causal_core::dynotears::dynotears_with_progress;
use hirmos_causal_core::glm::{negative_binomial_p, poisson_glm};
use hirmos_causal_core::grace::{fit_grace_with_progress, GraceConfiguration};
use hirmos_causal_core::identified_expression::{evaluate_distribution, DiscreteTable};
use hirmos_causal_core::ingarch::{
    detect_negative_binomial_intervention, fit_negative_binomial_ingarch, intervention_regressors,
    IngarchLink as CoreIngarchLink, IngarchSpecification, InterventionSchedule,
};
use hirmos_causal_core::joint_samples::JointData;
use hirmos_causal_core::jpcmciplus::{self, NodeClass as CoreJpcmciNodeClass};
use hirmos_causal_core::lpcmci::{run_lpcmci_frame_with_progress, run_lpcmci_with_progress};
use hirmos_causal_core::missing_data::{MaskType, TigramiteFrame};
use hirmos_causal_core::negbin_nuts::{irr_summary, quantile, PbcNegBinModel};
use hirmos_causal_core::neural_granger::{
    fit_clstm_with, fit_cmlp_with, Activation as CoreNeuralActivation, ClstmConfig, CmlpConfig,
    CmlpPenalty as CoreCmlpPenalty,
};
use hirmos_causal_core::nprandom::{Mt19937, NpRng};
use hirmos_causal_core::nuts::NutsOptions;
use hirmos_causal_core::ocse::{discover_network_with_progress, CmiMethod};
use hirmos_causal_core::ols::Ols;
use hirmos_causal_core::parcorr::{CiKind, CutOff, RoleAwareSamplePolicy, TimeSeries};
use hirmos_causal_core::parcorr_mult::Correlation as MultCorrelation;
use hirmos_causal_core::pcmciplus::{run_pcmciplus, run_pcmciplus_frame};
use hirmos_causal_core::pelt::pelt_l2;
use hirmos_causal_core::pss_tables::stat_star;
use hirmos_causal_core::refute_dml::{
    placebo_refute, random_common_cause_refute, unobserved_refute, worker_fit, WorkerStudy,
};
use hirmos_causal_core::resampling::{
    pandas_resample_daily, Aggregation as CoreResamplingAggregation,
    IncompleteBins as CoreIncompleteBins, ResampleFrequency as CoreResampleFrequency,
};
use hirmos_causal_core::rpcmci::run_rpcmci_with_progress;
use hirmos_causal_core::stl::{stl, strength, StlConfig};
use hirmos_causal_core::synthetic_control::{
    debiased_synthetic_control, donor_placebo_mspe_inference, synthetic_control_prediction_band,
    synthetic_effect, SyntheticControlBandMethod, SyntheticControlError,
};
use hirmos_causal_core::tsdiag::granger_ssr_ftest;
use hirmos_causal_core::tsdiag::ljung_box;
use hirmos_causal_core::var_lingam::{direct_lingam_with_progress, run_var_lingam};
use hirmos_causal_core::vecm::{chow_break, select_coint_rank, vecm_fit, vecm_select_order};
use hirmos_causal_core::{
    adfuller, kpss, AdfResult, KpssResult, Regression, ZaResult,
};
use hirmos_causal_core::{
    backdoor_linear_ate, dagitty_adjustment_sets, durbin_watson, identify_conditional_outcomes,
    identify_frontdoor_set, identify_instrument_set, identify_outcomes, infer_kappa_t,
    infer_kappa_y, latent_projection, ols_cluster, ols_hac, ols_hc1, refute_data_subset, refute_placebo,
    refute_random_common_cause, shapiro, unobserved_common_cause_grid, AdjustmentSetAnalysis, Dag,
    IdentificationError, WithinErrors,
};
use hirmos_causal_core::{cluster_redundant, correlation_matrix, vif_redundant};
use nalgebra::DMatrix;
use nalgebra::DVector;
use serde::Serialize;
use spec_math::cephes64::{ndtri, stdtri};
use std::collections::{BTreeMap, HashMap};
use wasm_bindgen::prelude::*;

mod causal_forest_pool;

#[wasm_bindgen(js_name = planForestTuning)]
pub fn plan_forest_tuning(values: &[f64], request: &str) -> Result<String, JsValue> {
    let request = serde_json::from_str(request).map_err(|e| JsValue::from_str(&format!("Invalid forest tuning request: {e}")))?;
    let reply = causal_forest_pool::plan(values, request).map_err(|e| JsValue::from_str(&e))?;
    serde_json::to_string(&reply).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = scoreForestTuning)]
pub fn score_forest_tuning(batch: &str, index: usize) -> Result<String, JsValue> {
    let batch = serde_json::from_str(batch).map_err(|e| JsValue::from_str(&format!("Invalid forest tuning batch: {e}")))?;
    let score = causal_forest_pool::score(&batch, index).map_err(|e| JsValue::from_str(&e))?;
    serde_json::to_string(&score).map_err(|e| JsValue::from_str(&e.to_string()))
}

mod missingness;
mod staggered_did;
mod bacon;
mod predictor_synthetic_control;
mod panel_regression;
mod remix_extensions;
mod surrogate;
mod count_regression;
mod calendar;

use missingness::{resolve_missingness, MissingnessExecution, MissingnessResolution};

mod counterfactual;
mod dag_check;
mod root_cause;
mod gcm_effects;
mod gcm_influence;
mod root_cause_checks;
mod discovery;
mod dynamic_counterfactual;
mod estimation;
mod ardl_model;
mod identification;
mod matrix;
mod preparation;
mod protocol;
mod sensitivity;
mod stationarity;
mod survival;

use counterfactual::*;
use dag_check::*;
use discovery::*;
use dynamic_counterfactual::*;
use estimation::*;
use identification::*;
use matrix::*;
use preparation::*;
use protocol::*;
use sensitivity::*;
use stationarity::*;
use survival::*;

/// Execute one validated analysis command over a transferred dense numeric column.
#[wasm_bindgen(js_name = runAnalysis)]
pub fn run_analysis(
    command_json: &str,
    values: &[f64],
    validity: &[u8],
    analysis_mask: &[u8],
    progress_callback: &js_sys::Function,
) -> Result<String, JsError> {
    let command: AnalysisCommand = serde_json::from_str(command_json)
        .map_err(|error| JsError::new(&format!("invalid analysis command: {error}")))?;
    let progress = |stage: &'static str, completed: usize, total: usize| {
        let _ = progress_callback.call3(
            &JsValue::NULL,
            &JsValue::from_str(stage),
            &JsValue::from_f64(completed as f64),
            &JsValue::from_f64(total as f64),
        );
    };
    let result = match command {
        AnalysisCommand::SurrogatePath { request } => surrogate::path(request).map(|evidence| AnalysisResult::SurrogatePath { evidence }),
        AnalysisCommand::SurrogateDiagnostics { request } => surrogate::diagnose(request).map(|evidence| AnalysisResult::SurrogateDiagnostics { evidence }),
        AnalysisCommand::Surrogate { request } => surrogate::run(request).map(|evidence| AnalysisResult::Surrogate { evidence }),
        AnalysisCommand::GcmEffects { request } => gcm_effects::run(request, values, &progress)
            .map(|evidence| AnalysisResult::GcmEffects { evidence }),
        AnalysisCommand::GcmInfluence { request } => gcm_influence::run(request, values, &progress)
            .map(|evidence| AnalysisResult::GcmInfluence { evidence }),
        AnalysisCommand::RootCause { request } => root_cause::run(request, values, &progress)
            .map(|evidence| AnalysisResult::RootCause { evidence }),
        AnalysisCommand::RootCauseChecks { request } => root_cause_checks::run(request, values, &progress)
            .map(|evidence| AnalysisResult::RootCauseChecks { evidence }),
        AnalysisCommand::StationarityBattery => stationarity_battery(values),
        AnalysisCommand::Multicollinearity {
            rows,
            columns,
            correlation_threshold,
            vif_threshold,
        } => multicollinearity(values, rows, columns, correlation_threshold, vif_threshold),
        AnalysisCommand::PandasResampleDaily {
            rows,
            columns,
            target,
            incomplete_bins,
            aggregations,
            imputed_cells,
        } => {
            if values.len() != rows.saturating_mul(columns + 1) {
                return Err(JsError::new(
                    "resampling payload shape does not match rows and columns",
                ));
            }
            let timestamps: Result<Vec<i64>, JsError> = values[..rows]
                .iter()
                .map(|&value| {
                    if !value.is_finite()
                        || value.fract() != 0.0
                        || value < i64::MIN as f64
                        || value > i64::MAX as f64
                    {
                        Err(JsError::new(
                            "resampling timestamps must be finite integer milliseconds",
                        ))
                    } else {
                        Ok(value as i64)
                    }
                })
                .collect();
            let target = match target {
                ResamplingTarget::Weekly => CoreResampleFrequency::WeeklyMonday,
                ResamplingTarget::Monthly => CoreResampleFrequency::MonthStart,
            };
            let incomplete = match incomplete_bins {
                ResamplingIncompleteBins::Keep => CoreIncompleteBins::Keep,
                ResamplingIncompleteBins::Drop => CoreIncompleteBins::Drop,
            };
            let methods: Vec<_> = aggregations
                .into_iter()
                .map(|method| match method {
                    ResamplingAggregation::Mean => CoreResamplingAggregation::Mean,
                    ResamplingAggregation::Sum => CoreResamplingAggregation::Sum,
                    ResamplingAggregation::Median => CoreResamplingAggregation::Median,
                    ResamplingAggregation::Minimum => CoreResamplingAggregation::Minimum,
                    ResamplingAggregation::Maximum => CoreResamplingAggregation::Maximum,
                    ResamplingAggregation::First => CoreResamplingAggregation::First,
                    ResamplingAggregation::Last => CoreResamplingAggregation::Last,
                })
                .collect();
            let imputed: Vec<_> = imputed_cells
                .into_iter()
                .map(|cell| (cell[0], cell[1]))
                .collect();
            let result = pandas_resample_daily(
                &values[rows..],
                rows,
                columns,
                &timestamps?,
                &methods,
                &imputed,
                target,
                incomplete,
            )
            .map_err(|problem| JsError::new(&format!("pandas resampling refused: {problem:?}")))?;
            Ok(AnalysisResult::PandasResampled {
                values: result.values,
                timestamps_ms: result.timestamps_ms,
                imputed_cells: result.imputed_cells,
                source_rows: result.source_rows,
                output_rows: result.output_rows,
                incomplete_bins: result.incomplete_bins,
                bins_dropped: result.bins_dropped,
                source_rows_dropped: result.source_rows_dropped,
            })
        }
        AnalysisCommand::FlexSurv {
            rows,
            columns,
            observation,
            row_frequency,
            covariates,
            family,
            prediction_times,
        } => flexsurv_evidence(
            values,
            rows,
            columns,
            observation,
            row_frequency,
            &covariates,
            family,
            &prediction_times,
        ),
        AnalysisCommand::CoxRegression {
            rows,
            columns,
            observation,
            weights,
            strata,
            covariates,
            penalty,
            confidence_level,
        } => cox_regression_evidence(
            values,
            rows,
            columns,
            observation,
            weights,
            strata,
            &covariates,
            penalty,
            confidence_level,
        ),
        AnalysisCommand::PenalizedAft {
            rows,
            columns,
            duration,
            event,
            covariates,
            family,
            penalizer,
            confidence_level,
            prediction_times,
        } => penalized_aft_evidence(
            values,
            rows,
            columns,
            duration,
            event,
            &covariates,
            family,
            penalizer,
            confidence_level,
            &prediction_times,
        ),
        AnalysisCommand::Aalen { rows, columns, duration, event, covariates } =>
            aalen_evidence(values, rows, columns, duration, event, &covariates),
        AnalysisCommand::SurvivalForest { rows, columns, duration, event, covariates, categorical, trees, mtry, seed, min_node_size, min_bucket, prediction_row, split_rule } =>
            forest_evidence(values, rows, columns, duration, event, &covariates, &categorical, trees, mtry, seed, min_node_size, min_bucket, prediction_row, split_rule),
        AnalysisCommand::NonparametricSurvival {
            rows,
            columns,
            duration,
            event,
            row_frequency,
            prediction_times,
            ties,
        } => nonparametric_survival_evidence(
            values,
            rows,
            columns,
            duration,
            event,
            row_frequency,
            &prediction_times,
            ties,
        ),
        AnalysisCommand::ComparisonSurvival {
            rows,
            columns,
            duration,
            event,
            group,
            truncation_time,
            permutations,
            seed,
        } => comparison_survival_evidence(
            values,
            rows,
            columns,
            duration,
            event,
            group,
            truncation_time,
            permutations,
            seed,
        ),
        AnalysisCommand::MultiStateSurvival {
            rows,
            columns,
            input,
            family,
            prediction_times,
        } => multi_state_survival_evidence(values, rows, columns, input, family, &prediction_times),
        AnalysisCommand::PcmciPlus {
            rows,
            columns,
            tau_max,
            pc_alpha,
            samples,
        } => pcmci_plus(
            values,
            validity,
            analysis_mask,
            rows,
            columns,
            tau_max,
            pc_alpha,
            samples,
        ),
        AnalysisCommand::Jpcmciplus {
            rows,
            datasets,
            periods,
            observed_columns,
            classes,
            time_dummy,
            space_dummy,
            tau_max,
            pc_alpha,
        } => jpcmciplus_evidence(
            values,
            rows,
            datasets,
            periods,
            observed_columns,
            classes,
            time_dummy,
            space_dummy,
            tau_max,
            pc_alpha,
        ),
        AnalysisCommand::Lpcmci {
            rows,
            columns,
            tau_max,
            pc_alpha,
            samples,
        } => lpcmci_evidence(
            values,
            validity,
            analysis_mask,
            rows,
            columns,
            tau_max,
            pc_alpha,
            samples,
            progress,
        ),
        AnalysisCommand::Rpcmci {
            rows,
            columns,
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
        } => rpcmci_evidence(
            values,
            rows,
            columns,
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
            progress,
        ),
        AnalysisCommand::Cdnots {
            rows,
            columns,
            max_lag,
            alpha,
            missing,
            context,
        } => cdnots_evidence(
            values, validity, rows, columns, max_lag, alpha, missing, context, false, progress,
        ),
        AnalysisCommand::CdnotsPlus {
            rows,
            columns,
            max_lag,
            alpha,
            missing,
            context,
        } => cdnots_evidence(
            values, validity, rows, columns, max_lag, alpha, missing, context, true, progress,
        ),
        AnalysisCommand::Grace {
            rows,
            columns,
            max_lag,
            alpha,
            context,
            gate_threshold,
            epochs,
            patience,
            seed,
        } => grace_evidence(
            values,
            validity,
            rows,
            columns,
            max_lag,
            alpha,
            context,
            gate_threshold,
            epochs,
            patience,
            seed,
            progress,
        ),
        AnalysisCommand::Dynotears {
            rows,
            columns,
            max_lag,
            lambda_w,
            lambda_a,
        } => dynotears_evidence(values, rows, columns, max_lag, lambda_w, lambda_a, progress),
        AnalysisCommand::DirectLingam { rows, columns } => {
            direct_lingam_evidence(values, rows, columns, progress)
        }
        AnalysisCommand::PcStable {
            rows,
            columns,
            names,
            alpha,
            max_depth,
            ci_test,
            background,
        } => cross_sectional_constraint_evidence(
            values, rows, columns, names, alpha, max_depth, None, ci_test, background, false,
            progress,
        ),
        AnalysisCommand::Fci {
            rows,
            columns,
            names,
            alpha,
            max_depth,
            max_path_length,
            ci_test,
            background,
        } => cross_sectional_constraint_evidence(
            values,
            rows,
            columns,
            names,
            alpha,
            max_depth,
            max_path_length,
            ci_test,
            background,
            true,
            progress,
        ),
        AnalysisCommand::VarLingam {
            rows,
            columns,
            lags,
            prune,
        } => var_lingam_evidence(values, rows, columns, lags, prune, progress),
        AnalysisCommand::Ocse {
            rows,
            columns,
            max_lag,
            alpha,
            n_shuffles,
            method,
            k,
        } => ocse_evidence(
            values, rows, columns, max_lag, alpha, n_shuffles, method, k, progress,
        ),
        AnalysisCommand::Cmlp {
            rows,
            columns,
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
        } => cmlp_evidence(
            values,
            rows,
            columns,
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
            progress,
        ),
        AnalysisCommand::Clstm {
            rows,
            columns,
            context,
            hidden,
            lambda,
            ridge_lambda,
            learning_rate,
            max_iter,
            check_every,
            lookback,
            seed,
        } => clstm_evidence(
            values,
            rows,
            columns,
            context,
            hidden,
            lambda,
            ridge_lambda,
            learning_rate,
            max_iter,
            check_every,
            lookback,
            seed,
            progress,
        ),
        AnalysisCommand::GrangerSsrF { rows, max_lag } => granger_evidence(values, rows, max_lag),
        AnalysisCommand::BackdoorIdentify {
            nodes,
            names,
            edges,
            treatment,
            outcome,
            unobserved,
            estimand,
        } => backdoor_identification(
            nodes,
            &names,
            &edges,
            treatment,
            outcome,
            &unobserved,
            estimand,
        ),
        AnalysisCommand::AdjustmentValidate { nodes, edges, treatment, outcome, unobserved, sets } => {
            identification::validate_supplied(nodes, &edges, treatment, outcome, &unobserved, &sets)
        }
        AnalysisCommand::SwigAnalysis { specification, names } => {
            swig_analysis::analyse(specification, &names).map(|result| AnalysisResult::SwigAnalysis { result })
        },
        AnalysisCommand::DagCheck {
            rows,
            columns,
            node_columns,
            edges,
            implications,
            maximum_observations,
            permutations,
            significance_level,
            run_falsification,
        } => dag_check(
            values,
            rows,
            columns,
            &node_columns,
            &edges,
            &implications,
            maximum_observations,
            permutations,
            significance_level,
            run_falsification,
            progress,
        ),
        AnalysisCommand::BackdoorLinear {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            hac_max_lags,
            level,
            error_model,
            fixed_effects,
        } => backdoor_linear(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            hac_max_lags,
            level,
            error_model,
            fixed_effects,
        ),
        AnalysisCommand::PropensityWeighting {
            target, rows,
            columns,
            treatment,
            outcome,
            adjustment,
            scale,
            fit,
        } => propensity_weighting(
            target, values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            scale,
            fit,
        ),
        AnalysisCommand::PropensityMatching {
            target, rows, columns, treatment, outcome, adjustment, model,
        } => propensity_matching(target, values, rows, columns, treatment, outcome, &adjustment, model),
        AnalysisCommand::PropensityGridSlice {
            rows, columns, treatment, outcome, adjustment, learning_rate, max_depth,
            n_estimators, splits, min_samples_leaf, min_samples_split, seed,
        } => propensity_grid_slice(
            values, rows, columns, treatment, outcome, &adjustment, learning_rate, max_depth,
            n_estimators, splits, min_samples_leaf, min_samples_split, seed,
        ),
        AnalysisCommand::DoublyRobust {
            target, rows, columns, treatment, outcome, adjustment, model, bootstrap,
        } => doubly_robust_estimate(
            target, values, rows, columns, treatment, outcome, &adjustment, model, bootstrap,
        ),
        AnalysisCommand::ContinuousGps {
            rows, columns, treatment, outcome, adjustment, scale, bootstrap,
        } => continuous_gps(
            values, rows, columns, treatment, outcome, &adjustment, scale, bootstrap,
        ),
        AnalysisCommand::FrontdoorTwoStage {
            rows,
            columns,
            treatment,
            mediator,
            outcome,
            first_stage_adjustment,
            second_stage_adjustment,
            control_value,
            treatment_value,
            uncertainty,
        } => frontdoor_two_stage_evidence(
            values,
            rows,
            columns,
            treatment,
            mediator,
            outcome,
            &first_stage_adjustment,
            &second_stage_adjustment,
            control_value,
            treatment_value,
            uncertainty,
            progress,
        ),
        AnalysisCommand::InstrumentalVariable {
            rows,
            columns,
            treatment,
            outcome,
            instruments,
            uncertainty,
        } => instrumental_variable_evidence(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &instruments,
            uncertainty,
            progress,
        ),
        AnalysisCommand::CountGlm {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            family,
        } => count_glm(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            family,
        ),
        AnalysisCommand::NegativeBinomialIngarch {
            rows,
            columns,
            outcome,
            link,
            regressors,
            past_observation_lags,
            past_mean_lags,
            external_regressors,
            horizon,
            baseline_regressors,
            intervention_regressor,
            control_value,
            treatment_value,
            schedule,
        } => negative_binomial_ingarch(
            values,
            rows,
            columns,
            outcome,
            link,
            &regressors,
            &past_observation_lags,
            &past_mean_lags,
            &external_regressors,
            horizon,
            &baseline_regressors,
            intervention_regressor,
            control_value,
            treatment_value,
            schedule,
            progress,
        ),
        AnalysisCommand::CountSeriesInterventionScan {
            rows,
            columns,
            outcome,
            link,
            past_observation_lags,
            past_mean_lags,
            candidate_reference_points,
            delta,
        } => count_series_intervention_scan(
            values,
            rows,
            columns,
            outcome,
            link,
            &past_observation_lags,
            &past_mean_lags,
            &candidate_reference_points,
            delta,
            progress,
        ),
        AnalysisCommand::InterruptedSeries {
            rows,
            columns,
            outcome,
            model,
            intervention_row,
            lag,
            impact,
            seasonal,
            ljung_box_lags,
        } => interrupted_series(
            values,
            rows,
            columns,
            outcome,
            model,
            intervention_row,
            lag,
            impact,
            seasonal,
            ljung_box_lags,
        ),
        AnalysisCommand::CausalEffectsTotal {
            rows,
            columns,
            stat_lag,
            graph,
            x,
            y,
            hidden,
            estimator,
            interventions,
            uncertainty,
        } => causal_effects_total(
            values,
            rows,
            columns,
            stat_lag,
            &graph,
            &x,
            &y,
            &hidden,
            estimator,
            interventions,
            uncertainty,
            progress,
        ),
        AnalysisCommand::SharpRd { rows, cutoff } => estimation::sharp_rd(values, rows, cutoff),
        AnalysisCommand::CausalImpact {
            rows,
            columns,
            outcome,
            controls,
            n_pre,
            post_end,
            max_iter,
        } => causal_impact_evidence(values, rows, columns, outcome, &controls, n_pre, post_end, max_iter),
        AnalysisCommand::StructuralCausalImpact { rows, columns, outcome, controls, n_pre, post_end, draws, warmup, seed, model } =>
            structural_causal_impact_evidence(values, rows, columns, outcome, &controls, n_pre, post_end, draws, warmup, seed, model),
        AnalysisCommand::BayesianCausalImpact { rows, columns, outcome, controls, n_pre, post_end, draws, warmup, seed, prior_level_sd } =>
            bayesian_causal_impact_evidence(values, rows, columns, outcome, &controls, n_pre, post_end, draws, warmup, seed, prior_level_sd),
        AnalysisCommand::LinearRefutation {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            simulations,
            subset_fraction,
            seed,
            ljung_box_lags,
        } => linear_refutation(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            simulations,
            subset_fraction,
            seed,
            ljung_box_lags,
        ),
        AnalysisCommand::UnobservedConfounding {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            seed,
            kappa_t,
            kappa_y,
        } => unobserved_confounding(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            seed,
            kappa_t,
            kappa_y,
        ),
        AnalysisCommand::SeriesStructure {
            rows,
            columns,
            period,
            robust,
            correlation_max_lag,
            pelt_min_size,
            pelt_jump,
            pelt_penalty,
        } => series_structure(
            values,
            rows,
            columns,
            period,
            robust,
            correlation_max_lag,
            pelt_min_size,
            pelt_jump,
            pelt_penalty,
        ),
        AnalysisCommand::SeasonalAdjust {
            rows,
            columns,
            period,
            robust,
            adjust,
        } => seasonal_adjust(values, rows, columns, period, robust, &adjust),
        AnalysisCommand::DoubleMl {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            model,
            att,
            seed,
            groups,
        } => double_ml(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            model,
            att,
            seed,
            groups,
        ),
        AnalysisCommand::CausalForest { rows, columns, treatment, outcome, adjustment, target, configuration, column_names } =>
            causal_forest::fit_named(values, rows, columns, treatment, outcome, &adjustment, target, configuration, &column_names)
                .map(|evidence| AnalysisResult::CausalForest { evidence }),
        AnalysisCommand::TLearner {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            seed,
            uncertainty,
        } => t_learner_with_uncertainty(values, rows, columns, treatment, outcome, &adjustment, seed, uncertainty),
        AnalysisCommand::CrossFittedTLearner {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            learning_rate,
            max_depth,
            n_estimators,
            splits,
            min_samples_leaf,
            min_samples_split,
            seed,
            selection,
        } => cross_fitted_t_learner(
            values, rows, columns, treatment, outcome, &adjustment,
            hirmos_causal_core::model_selection::Grid { learning_rate, max_depth, n_estimators },
            splits, min_samples_leaf, min_samples_split, seed, selection,
        ),
        AnalysisCommand::DmlRefutationBatch {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            model,
            att,
            seed,
        } => dml_refutation_batch(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            model,
            att,
            seed,
        ),
        AnalysisCommand::ArdlModel { rows, columns, model } => ardl_model::fit(values, rows, columns, model),
        AnalysisCommand::HonestDid { model } => honest_did::fit(model),
        AnalysisCommand::DidSensitivity { rows, columns, units, times, model } => did_sensitivity::fit(values, rows, columns, &units, &times, model),
        AnalysisCommand::ArdlPss {
            rows,
            columns,
            treatment,
            outcome,
            max_lag,
            trend,
            case,
        } => ardl_pss(
            values, rows, columns, treatment, outcome, max_lag, trend, case,
        ),
        AnalysisCommand::Vecm {
            rows,
            columns,
            endogenous,
            max_lags,
            deterministic,
            significance,
            break_index,
            forecast_steps,
        } => estimation::vecm_with_forecast(
            values,
            rows,
            columns,
            &endogenous,
            max_lags,
            deterministic,
            significance,
            break_index,
            forecast_steps,
        ),
        AnalysisCommand::SyntheticControl {
            rows,
            columns,
            treated,
            donors,
            n_pre,
            cross_fit_folds,
            alpha,
        } => synthetic_control(
            values,
            rows,
            columns,
            treated,
            &donors,
            n_pre,
            cross_fit_folds,
            alpha,
        ),
        AnalysisCommand::PanelAdjusted { rows, columns, units, times, specification } => panel_adjusted(values, rows, columns, &units, &times, specification),
        AnalysisCommand::CountRegression { request } => count_regression::run(values, request).map(|evidence| AnalysisResult::CountRegression { evidence }),
        AnalysisCommand::StaggeredDid { request } => staggered_did::run(values, request).map(|evidence| AnalysisResult::StaggeredDid { evidence }),
        AnalysisCommand::PredictorSyntheticControl { request } => predictor_synthetic_control::run(values, request).map(|evidence| AnalysisResult::PredictorSyntheticControl { evidence }),
        AnalysisCommand::Bacon { request } => bacon::run(values, request).map(|evidence| AnalysisResult::Bacon { evidence }),
        AnalysisCommand::SunAbraham { request } => remix_extensions::sun_run(values, request).map(|evidence| AnalysisResult::SunAbraham { evidence }),
        AnalysisCommand::RidgeAugmentedSynthetic { request } => remix_extensions::ridge_run(values, request).map(|evidence| AnalysisResult::RidgeAugmentedSynthetic { evidence }),
        AnalysisCommand::PanelRegression { request } => panel_regression::run(values, request).map(|evidence| AnalysisResult::PanelRegression { evidence }),
        AnalysisCommand::PanelIntervention {
            primary,
            rows,
            units,
            times,
            placebo_replications,
            seed,
        } => panel_intervention_selected(
            primary,
            values,
            rows,
            &units,
            &times,
            placebo_replications,
            seed,
            progress,
        ),
        AnalysisCommand::LinearScmCounterfactual {
            rows,
            columns,
            nodes,
            names,
            edges,
            treatment,
            outcome,
            interventions,
            observation_noise,
        } => linear_scm_counterfactual(
            values,
            rows,
            columns,
            &nodes,
            &names,
            &edges,
            treatment,
            outcome,
            interventions,
            observation_noise,
        ),
        AnalysisCommand::DynamicLinearScmCounterfactual {
            rows,
            columns,
            nodes,
            stat_lag,
            graph,
            treatment,
            outcome,
            timing,
            steps,
            interventions,
            uncertainty,
        } => dynamic_linear_scm_counterfactual(
            values,
            rows,
            columns,
            &nodes,
            stat_lag,
            &graph,
            treatment,
            outcome,
            timing,
            steps,
            interventions,
            uncertainty,
            progress,
        ),
        AnalysisCommand::NegbinNuts {
            rows,
            columns,
            treatment,
            outcome,
            confounder,
            warmup,
            samples,
            seed,
        } => negbin_nuts(
            values, rows, columns, treatment, outcome, confounder, warmup, samples, seed,
        ),
        AnalysisCommand::BayesianGaussian {
            rows,
            columns,
            treatment,
            outcome,
            adjustment,
            warmup,
            samples,
            seed,
        } => bayesian_gaussian(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            warmup,
            samples,
            seed,
        ),
        AnalysisCommand::NetworkQuery { query } => network_query::run(values, query),
        AnalysisCommand::ConditionalGaussianQuery { query } => conditional_gaussian_query::run(values, query),
        AnalysisCommand::DiscreteBnQuery {
            rows,
            columns,
            nodes,
            names,
            edges,
            treatment,
            outcome,
            bins,
            equivalent_sample_size,
        } => discrete_bn_query(
            values,
            rows,
            columns,
            &nodes,
            &names,
            &edges,
            treatment,
            outcome,
            bins,
            equivalent_sample_size,
        ),
        AnalysisCommand::IdentifiedDiscreteQuery {
            rows,
            columns,
            observed_nodes,
            names,
            edges,
            treatment,
            outcome,
            unobserved,
            bins,
            condition,
        } => identified_discrete_query(
            values,
            rows,
            columns,
            &observed_nodes,
            &names,
            &edges,
            treatment,
            outcome,
            &unobserved,
            bins,
            condition,
        ),
        AnalysisCommand::BinaryEtt {
            rows,
            columns,
            observed_nodes,
            names,
            edges,
            treatment,
            outcome,
            unobserved,
        } => binary_ett(
            values,
            rows,
            columns,
            &observed_nodes,
            &names,
            &edges,
            treatment,
            outcome,
            &unobserved,
        ),
        AnalysisCommand::ResolveMissingness {
            rows,
            columns,
            validity,
            resolution,
        } => resolve_missingness(values, rows, columns, &validity, resolution),
    }
    .map_err(|error| JsError::new(&error))?;
    serde_json::to_string(&result)
        .map_err(|error| JsError::new(&format!("could not serialize analysis result: {error}")))
}
