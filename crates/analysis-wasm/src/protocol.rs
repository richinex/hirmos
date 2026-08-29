//! The command and result contract the worker speaks: serde-tagged enums and the evidence shapes.

use super::*;

pub(crate) const MIN_STATIONARITY_OBSERVATIONS: usize = 24;
pub(crate) const MAX_MINIMAL_ADJUSTMENT_SETS: usize = 128;

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum BackdoorAdjustmentSetEvidence {
    Identified {
        canonical_set: Vec<usize>,
        minimal_sets: Vec<Vec<usize>>,
        truncated: bool,
    },
    NotIdentified,
}

#[derive(serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum AnalysisCommand {
    StationarityBattery,
    PcmciPlus {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
    },
    Lpcmci {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
    },
    Dynotears {
        rows: usize,
        columns: usize,
        max_lag: usize,
        lambda_w: f64,
        lambda_a: f64,
    },
    VarLingam {
        rows: usize,
        columns: usize,
        lags: usize,
        prune: bool,
    },
    Ocse {
        rows: usize,
        columns: usize,
        max_lag: usize,
        alpha: f64,
        n_shuffles: usize,
        method: OcseMethod,
        k: usize,
    },
    GrangerSsrF {
        rows: usize,
        max_lag: usize,
    },
    BackdoorIdentify {
        nodes: usize,
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        unobserved: Vec<usize>,
    },
    BackdoorLinear {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        hac_max_lags: Option<usize>,
        level: f64,
    },
    CountGlm {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        family: CountFamily,
    },
    CausalEffectsTotal {
        rows: usize,
        columns: usize,
        stat_lag: usize,
        /// Tigramite marks `graph[i][j][tau]`; empty strings for no link.
        graph: Vec<Vec<Vec<String>>>,
        x: Vec<(usize, i32)>,
        y: Vec<(usize, i32)>,
        hidden: Vec<(usize, i32)>,
        estimator: TotalEffectEstimator,
        /// Two intervention values for X; the effect is the prediction difference.
        interventions: [f64; 2],
    },
    CausalImpact {
        rows: usize,
        columns: usize,
        outcome: usize,
        controls: Vec<usize>,
        n_pre: usize,
        max_iter: usize,
    },
    LinearRefutation {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        simulations: usize,
        subset_fraction: f64,
        seed: u32,
        ljung_box_lags: usize,
    },
    UnobservedConfounding {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        seed: u32,
        /// Explicit strength grids, as DoWhy's `kappa_t` and `kappa_y`; null infers them from the observed common causes.
        kappa_t: Option<Vec<f64>>,
        kappa_y: Option<Vec<f64>>,
    },
    SeriesStructure {
        rows: usize,
        columns: usize,
        period: Option<usize>,
        robust: bool,
        pelt_min_size: usize,
        pelt_jump: usize,
        pelt_penalty: f64,
    },
    SeasonalAdjust {
        rows: usize,
        columns: usize,
        period: usize,
        robust: bool,
        /// Column positions to adjust; the others pass through unchanged.
        adjust: Vec<usize>,
    },
    DoubleMl {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        model: DmlModel,
        /// IRM only: the effect on the treated instead of the average effect.
        att: bool,
        /// Seeds the fold stream; the learner seed is the worker's fixed 7.
        seed: u32,
    },
    DmlRefutationBatch {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        model: DmlModel,
        att: bool,
        seed: u32,
    },
    ArdlPss {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        max_lag: usize,
        trend: ArdlTrend,
        /// Pesaran, Shin and Smith case: 2 or 3 with a constant, 4 or 5 with a trend.
        case: usize,
    },
    SyntheticControl {
        rows: usize,
        columns: usize,
        treated: usize,
        donors: Vec<usize>,
        n_pre: usize,
    },
    PanelIntervention {
        rows: usize,
        /// One unit label and ordered time code per long-form observation.
        units: Vec<String>,
        times: Vec<i64>,
    },
    NegbinNuts {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        confounder: usize,
        warmup: usize,
        samples: usize,
        seed: u64,
    },
    DiscreteBnQuery {
        rows: usize,
        columns: usize,
        /// One matrix column per node, in node order.
        nodes: Vec<usize>,
        names: Vec<String>,
        /// Directed edges as node positions.
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        bins: usize,
        equivalent_sample_size: f64,
    },
    LinearScmCounterfactual {
        rows: usize,
        columns: usize,
        /// One matrix column per node, in node order.
        nodes: Vec<usize>,
        names: Vec<String>,
        /// Directed edges as node positions.
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        /// The two treatment values every row is set to; the effect is the difference.
        interventions: (f64, f64),
        /// Observation noise scale for abduction; null abducts exactly.
        observation_noise: Option<f64>,
    },
    Vecm {
        rows: usize,
        columns: usize,
        /// Endogenous columns in order; the first is the outcome the long-run relation is normalised on.
        endogenous: Vec<usize>,
        max_lags: usize,
        deterministic: VecmDeterministic,
        /// Johansen significance column: 0 for 90%, 1 for 95%, 2 for 99%.
        significance: usize,
        /// Optional Chow break index for the first two endogenous columns.
        break_index: Option<usize>,
    },
    ResolveMissingness {
        rows: usize,
        columns: usize,
        /// One byte per cell, column-major like the values: 1 observed, 0 missing.
        validity: Vec<u8>,
        resolution: MissingnessResolution,
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CountFamily {
    Poisson,
    NegativeBinomial,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum TotalEffectEstimator {
    Linear,
    Knn { k: usize },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum OcseMethod {
    Gaussian,
    Knn,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AdfEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
    pub(crate) used_lag: usize,
    pub(crate) observations: usize,
    pub(crate) critical_values: [f64; 3],
}

impl From<AdfResult> for AdfEvidence {
    fn from(value: AdfResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            used_lag: value.usedlag,
            observations: value.nobs,
            critical_values: value.crit,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KpssEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
    pub(crate) used_lag: usize,
    pub(crate) critical_values: [f64; 4],
}

impl From<KpssResult> for KpssEvidence {
    fn from(value: KpssResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            used_lag: value.nlags,
            critical_values: value.crit,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ZivotAndrewsEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
    pub(crate) critical_values: [f64; 3],
    pub(crate) base_lags: usize,
    pub(crate) break_index: usize,
}

impl From<ZaResult> for ZivotAndrewsEvidence {
    fn from(value: ZaResult) -> Self {
        Self {
            statistic: value.stat,
            p_value: value.pvalue,
            critical_values: value.crit,
            base_lags: value.baselags,
            break_index: value.bpidx,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DeterministicEvidence<T> {
    pub(crate) constant: T,
    pub(crate) constant_and_trend: T,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BreakEvidence {
    pub(crate) level: ZivotAndrewsEvidence,
    pub(crate) trend: ZivotAndrewsEvidence,
    pub(crate) level_and_trend: ZivotAndrewsEvidence,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GrangerLagEvidence {
    pub(crate) lag: usize,
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OcseEdgeEvidence {
    pub(crate) source: usize,
    pub(crate) target: usize,
    pub(crate) lag: usize,
    pub(crate) cmi: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PanelMethodEvidence {
    pub(crate) estimate: f64,
    pub(crate) lambda: Vec<f64>,
    pub(crate) omega: Vec<f64>,
    pub(crate) effect_curve: Vec<f64>,
    pub(crate) lambda_iterations: usize,
    pub(crate) omega_iterations: usize,
    pub(crate) lambda_objective: Vec<f64>,
    pub(crate) omega_objective: Vec<f64>,
    pub(crate) noise_level: f64,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum AnalysisResult {
    StationarityBattery {
        observations: usize,
        adf: DeterministicEvidence<AdfEvidence>,
        kpss: DeterministicEvidence<KpssEvidence>,
        zivot_andrews: BreakEvidence,
    },
    PcmciPlus {
        observations: usize,
        variables: usize,
        tau_max: usize,
        pc_alpha: f64,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    Lpcmci {
        observations: usize,
        variables: usize,
        tau_max: usize,
        pc_alpha: f64,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    Dynotears {
        observations: usize,
        variables: usize,
        max_lag: usize,
        lambda_w: f64,
        lambda_a: f64,
        contemporaneous_weights: Vec<Vec<f64>>,
        lagged_weights: Vec<Vec<Vec<f64>>>,
    },
    VarLingam {
        observations: usize,
        variables: usize,
        lags: usize,
        selected_lag: usize,
        prune: bool,
        causal_order: Vec<usize>,
        contemporaneous_weights: Vec<Vec<f64>>,
        lagged_weights: Vec<Vec<Vec<f64>>>,
    },
    Ocse {
        observations: usize,
        variables: usize,
        max_lag: usize,
        alpha: f64,
        n_shuffles: usize,
        method: OcseMethod,
        k: usize,
        seed: u64,
        edges: Vec<OcseEdgeEvidence>,
    },
    GrangerSsrF {
        observations: usize,
        max_lag: usize,
        tests: Vec<GrangerLagEvidence>,
    },
    BackdoorIdentification {
        nodes: usize,
        treatment: usize,
        outcome: usize,
        unobserved: Vec<usize>,
        result: BackdoorAdjustmentSetEvidence,
    },
    BackdoorLinear {
        observations: usize,
        parameters: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        level: f64,
        estimate: f64,
        standard_error: f64,
        interval: [f64; 2],
        degrees_of_freedom: usize,
        residual_sd: f64,
        r_squared: f64,
        hac_max_lags: usize,
        hac_standard_error: f64,
        hac_interval: [f64; 2],
        hac_p_value: f64,
        durbin_watson: f64,
    },
    CountGlm {
        observations: usize,
        parameters: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        family: CountFamily,
        coefficient: f64,
        standard_error: f64,
        p_value: f64,
        /// exp(coefficient) and the exponentiated normal interval, as 805 reports.
        incidence_rate_ratio: f64,
        incidence_rate_ratio_interval: [f64; 2],
        level: f64,
        /// Poisson deviance; NaN for the negative binomial.
        deviance: f64,
        /// Negative binomial log likelihood; NaN for Poisson.
        log_likelihood: f64,
        /// Negative binomial dispersion alpha; NaN for Poisson.
        alpha: f64,
        degrees_of_freedom: usize,
        converged: bool,
        iterations: usize,
    },
    CausalEffectsTotal {
        observations: usize,
        tau_max: usize,
        no_causal_path: bool,
        identifiable: bool,
        adjustment_set: Vec<(usize, i32)>,
        mediators: Vec<(usize, i32)>,
        estimator: TotalEffectEstimator,
        interventions: [f64; 2],
        /// Predicted Y at each intervention value; empty when not identifiable.
        predictions: Vec<f64>,
        /// predictions[1] - predictions[0]; NaN when not identifiable.
        total_effect: f64,
        fitted_observations: usize,
    },
    CausalImpact {
        observations: usize,
        n_pre: usize,
        n_post: usize,
        outcome: usize,
        controls: Vec<usize>,
        counterfactual: Vec<f64>,
        counterfactual_se: Vec<f64>,
        pointwise: Vec<f64>,
        cumulative: f64,
        average: f64,
        params: Vec<f64>,
        log_likelihood: f64,
    },
    LinearRefutation {
        observations: usize,
        estimate: f64,
        simulations: usize,
        seed: u32,
        placebo_effect: f64,
        subset_fraction: f64,
        subset_effect: f64,
        random_common_cause_effect: f64,
        ljung_box_lags: Vec<usize>,
        ljung_box_statistics: Vec<f64>,
        ljung_box_p_values: Vec<f64>,
        shapiro_w: Option<f64>,
        shapiro_p: Option<f64>,
        durbin_watson: f64,
    },
    UnobservedConfounding {
        observations: usize,
        seed: u32,
        kappa_t: Vec<f64>,
        kappa_y: Vec<f64>,
        /// Simulated effects, rows by kappa_t and columns by kappa_y.
        effects: Vec<Vec<f64>>,
        original_effect: f64,
    },
    SeriesStructure {
        observations: usize,
        period: Option<usize>,
        series: Vec<SeriesStructureEvidence>,
    },
    DoubleMl {
        observations: usize,
        model: DmlModel,
        att: bool,
        treat_binary: bool,
        seed: u32,
        estimate: f64,
        standard_error: f64,
        interval: (f64, f64),
        level: f64,
    },
    ArdlPss {
        observations: usize,
        trend: ArdlTrend,
        case: usize,
        ar_lag: usize,
        dl_lag: usize,
        /// `(ar_lag, dl_lag_or_null, aic, bic, hqic)` for every candidate in search order.
        grid: Vec<(usize, Option<usize>, f64, f64, f64)>,
        long_run_effect: f64,
        p_value: f64,
        interval: (f64, f64),
        level: f64,
        bounds_statistic: f64,
        /// Rows are 10%, 5%, 2.5%, 1%; columns the I(0) then the I(1) bound.
        bounds_critical: Vec<(f64, f64)>,
        bounds_p_lower: f64,
        bounds_p_upper: f64,
    },
    SyntheticControl {
        observations: usize,
        n_pre: usize,
        n_post: usize,
        weights: Vec<f64>,
        loss: f64,
        iterations: usize,
        pre_gap: Vec<f64>,
        post_gap: Vec<f64>,
        att: f64,
        /// The treated series and its synthetic counterpart over every row.
        treated: Vec<f64>,
        synthetic: Vec<f64>,
    },
    PanelIntervention {
        observations: usize,
        units: Vec<String>,
        times: Vec<i64>,
        control_units: usize,
        treated_units: usize,
        n_pre: usize,
        n_post: usize,
        did: PanelMethodEvidence,
        synthetic_control: PanelMethodEvidence,
        synthetic_did: PanelMethodEvidence,
    },
    NegbinNuts {
        observations: usize,
        warmup: usize,
        samples: usize,
        seed: u64,
        irr_median: f64,
        irr_lower: f64,
        irr_upper: f64,
        beta_treatment_mean: f64,
        beta_treatment_sd: f64,
        dispersion_mean: f64,
        divergences: usize,
        acceptance_rate: f64,
        mean_accept_probability: f64,
        step_size: f64,
    },
    DiscreteBnQuery {
        observations: usize,
        bins: usize,
        equivalent_sample_size: f64,
        /// State count per node, in node order.
        state_counts: Vec<usize>,
        treatment_states: (String, String),
        /// Outcome expectation under do(low) and do(high), from each state's original-unit mean.
        expectations: (f64, f64),
        effect: f64,
        /// P(outcome state | do(treatment)) for the low and the high state.
        distribution_low: Vec<(String, f64)>,
        distribution_high: Vec<(String, f64)>,
        /// pgmpy's minimal adjustment set, or null when none exists.
        minimal_adjustment_set: Option<Vec<String>>,
        /// The parents the do-query itself adjusted for.
        parents_adjusted: Vec<String>,
    },
    LinearScmCounterfactual {
        observations: usize,
        /// Node positions in the topological order the equations use.
        order: Vec<usize>,
        equations: Vec<ScmEquation>,
        interventions: (f64, f64),
        observation_noise: Option<f64>,
        factual_outcome: Vec<f64>,
        /// The outcome each row would have shown under the first and the second intervention.
        counterfactual_low: Vec<f64>,
        counterfactual_high: Vec<f64>,
        /// Per-row difference between the two counterfactual outcomes.
        effects: Vec<f64>,
        average_effect: f64,
        share_positive: f64,
    },
    Vecm {
        observations: usize,
        deterministic: VecmDeterministic,
        k_ar_diff: usize,
        rank: usize,
        significance: usize,
        /// Null when the trace test finds no cointegration, so nothing was fitted.
        long_run_effect: Option<f64>,
        alpha: Vec<Vec<f64>>,
        beta: Vec<Vec<f64>>,
        gamma: Vec<Vec<f64>>,
        pvalues_alpha: Vec<Vec<f64>>,
        chow: Option<(f64, f64)>,
    },
    DmlRefutationBatch {
        observations: usize,
        model: DmlModel,
        att: bool,
        seed: u32,
        order: Vec<&'static str>,
        main_estimate: f64,
        placebo: DmlRefutationOutcome,
        random_common_cause: DmlRefutationOutcome,
        sensitivity: DmlSensitivity,
    },
    SeasonalAdjusted {
        rows: usize,
        columns: usize,
        period: usize,
        /// Dense column-major values with the STL seasonal component removed from the adjusted columns.
        values: Vec<f64>,
        adjusted: Vec<SeasonalAdjustedColumn>,
    },
    MissingnessResolved {
        rows: usize,
        columns: usize,
        resolution: MissingnessResolution,
        outcome: MissingnessExecution,
    },
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DmlModel {
    Plr,
    Irm,
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ArdlTrend {
    C,
    Ct,
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum VecmDeterministic {
    /// No deterministic terms.
    N,
    /// Constant outside the cointegration relation.
    Co,
    /// Constant inside the cointegration relation.
    Ci,
    /// Constant outside and linear trend inside.
    Coli,
}

impl VecmDeterministic {
    pub(crate) fn code(self) -> &'static str {
        match self {
            VecmDeterministic::N => "n",
            VecmDeterministic::Co => "co",
            VecmDeterministic::Ci => "ci",
            VecmDeterministic::Coli => "coli",
        }
    }

    /// Johansen's `det_order`: -1 without deterministic terms, 0 with a constant, 1 with a trend.
    pub(crate) fn det_order(self) -> i32 {
        match self {
            VecmDeterministic::N => -1,
            VecmDeterministic::Co | VecmDeterministic::Ci => 0,
            VecmDeterministic::Coli => 1,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScmEquation {
    pub(crate) node: usize,
    pub(crate) intercept: f64,
    /// `(parent node position, coefficient)`.
    pub(crate) parents: Vec<(usize, f64)>,
    pub(crate) residual_sd: f64,
    pub(crate) r_squared: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DmlRefutationOutcome {
    pub(crate) original_effect: f64,
    pub(crate) refuted_effect: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DmlSensitivityScenario {
    pub(crate) confounding: f64,
    pub(crate) effect_lower: f64,
    pub(crate) effect_upper: f64,
    pub(crate) ci_lower: f64,
    pub(crate) ci_upper: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DmlSensitivity {
    pub(crate) scenarios: Vec<DmlSensitivityScenario>,
    pub(crate) robustness_value: f64,
    pub(crate) robustness_value_ci: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SeasonalAdjustedColumn {
    pub(crate) column: usize,
    pub(crate) seasonal_strength_before: f64,
    pub(crate) seasonal_strength_after: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SeriesStructureEvidence {
    pub(crate) column: usize,
    pub(crate) trend_strength: Option<f64>,
    pub(crate) seasonal_strength: Option<f64>,
    /// Segment ends as one-based row numbers, without the final row.
    pub(crate) change_points: Vec<usize>,
    pub(crate) pelt_penalty: f64,
}
