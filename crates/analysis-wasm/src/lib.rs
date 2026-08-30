//! Thin browser command façade over the parity-tested causal kernels.
//!
//! The façade owns browser-facing validation and serialization. It does not reinterpret test
//! evidence or copy the numerical implementations out of the Hirmos causal core.

use hirmos_causal_core::ardl::{ardl_select_order, bounds_test, uecm, Trend};
use hirmos_causal_core::causal_effects::{mark, CausalEffects, Estimator, Node, StationaryGraph};
use hirmos_causal_core::causal_impact::causal_impact;
use hirmos_causal_core::counterfactual::{Equation, LinearScm};
use hirmos_causal_core::discrete_bn::{discretize_805, Dag as DiscreteDag, DiscreteBn};
use hirmos_causal_core::dynotears::dynotears_with_progress;
use hirmos_causal_core::glm::{negative_binomial_p, poisson_glm};
use hirmos_causal_core::lpcmci::run_lpcmci_with_progress;
use hirmos_causal_core::bayesian_gaussian::{posterior_effect_summary, BayesianGaussianScm};
use hirmos_causal_core::negbin_nuts::{irr_summary, quantile, PbcNegBinModel};
use hirmos_causal_core::nprandom::Mt19937;
use hirmos_causal_core::nuts::NutsOptions;
use hirmos_causal_core::ocse::{discover_network_with_progress, CmiMethod};
use hirmos_causal_core::ols::Ols;
use hirmos_causal_core::parcorr::{CiKind, TimeSeries};
use hirmos_causal_core::pcmciplus::run_pcmciplus;
use hirmos_causal_core::pelt::pelt_l2;
use hirmos_causal_core::pss_tables::stat_star;
use hirmos_causal_core::refute_dml::{
    placebo_refute, random_common_cause_refute, unobserved_refute, worker_fit, WorkerStudy,
};
use hirmos_causal_core::stl::{stl, strength, StlConfig};
use hirmos_causal_core::synthetic_control::synthetic_effect;
use hirmos_causal_core::tsdiag::granger_ssr_ftest;
use hirmos_causal_core::tsdiag::ljung_box;
use hirmos_causal_core::var_lingam::run_var_lingam;
use hirmos_causal_core::vecm::{chow_break, select_coint_rank, vecm_fit, vecm_select_order};
use hirmos_causal_core::{
    adfuller, kpss, zivot_andrews, AdfResult, KpssResult, Regression, ZaModel, ZaResult,
};
use hirmos_causal_core::{
    backdoor_linear_ate, dagitty_adjustment_sets, durbin_watson, infer_kappa_t, infer_kappa_y,
    ols_hac, refute_data_subset, refute_placebo, refute_random_common_cause, shapiro,
    unobserved_common_cause_grid, AdjustmentSetAnalysis, Dag,
};
use nalgebra::DMatrix;
use nalgebra::DVector;
use serde::Serialize;
use spec_math::cephes64::{ndtri, stdtri};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod missingness;

use missingness::{resolve_missingness, MissingnessExecution, MissingnessResolution};

mod counterfactual;
mod dag_check;
mod discovery;
mod estimation;
mod identification;
mod matrix;
mod preparation;
mod protocol;
mod sensitivity;
mod stationarity;

use counterfactual::*;
use dag_check::*;
use discovery::*;
use estimation::*;
use identification::*;
use matrix::*;
use preparation::*;
use protocol::*;
use sensitivity::*;
use stationarity::*;

/// Execute one validated analysis command over a transferred dense numeric column.
#[wasm_bindgen(js_name = runAnalysis)]
pub fn run_analysis(
    command_json: &str,
    values: &[f64],
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
        AnalysisCommand::StationarityBattery => stationarity_battery(values),
        AnalysisCommand::PcmciPlus {
            rows,
            columns,
            tau_max,
            pc_alpha,
        } => pcmci_plus(values, rows, columns, tau_max, pc_alpha),
        AnalysisCommand::Lpcmci {
            rows,
            columns,
            tau_max,
            pc_alpha,
        } => lpcmci_evidence(values, rows, columns, tau_max, pc_alpha, progress),
        AnalysisCommand::Dynotears {
            rows,
            columns,
            max_lag,
            lambda_w,
            lambda_a,
        } => dynotears_evidence(values, rows, columns, max_lag, lambda_w, lambda_a, progress),
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
        AnalysisCommand::GrangerSsrF { rows, max_lag } => granger_evidence(values, rows, max_lag),
        AnalysisCommand::BackdoorIdentify {
            nodes,
            edges,
            treatment,
            outcome,
            unobserved,
        } => backdoor_identification(nodes, &edges, treatment, outcome, &unobserved),
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
        } => backdoor_linear(
            values,
            rows,
            columns,
            treatment,
            outcome,
            &adjustment,
            hac_max_lags,
            level,
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
        ),
        AnalysisCommand::CausalImpact {
            rows,
            columns,
            outcome,
            controls,
            n_pre,
            max_iter,
        } => causal_impact_evidence(values, rows, columns, outcome, &controls, n_pre, max_iter),
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
            pelt_min_size,
            pelt_jump,
            pelt_penalty,
        } => series_structure(
            values,
            rows,
            columns,
            period,
            robust,
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
        } => vecm(
            values,
            rows,
            columns,
            &endogenous,
            max_lags,
            deterministic,
            significance,
            break_index,
        ),
        AnalysisCommand::SyntheticControl {
            rows,
            columns,
            treated,
            donors,
            n_pre,
        } => synthetic_control(values, rows, columns, treated, &donors, n_pre),
        AnalysisCommand::PanelIntervention { rows, units, times } => {
            panel_intervention(values, rows, &units, &times, progress)
        }
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
            values, rows, columns, treatment, outcome, &adjustment, warmup, samples, seed,
        ),
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
