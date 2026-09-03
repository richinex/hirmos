//! The command and result contract the worker speaks: serde-tagged enums and the evidence shapes.

use super::*;

pub(crate) const MIN_STATIONARITY_OBSERVATIONS: usize = 24;
pub(crate) const MAX_MINIMAL_ADJUSTMENT_SETS: usize = 128;

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum TemporalCutOff {
    MethodDefault,
    TwoTauMax,
    TauMax,
    MaxLag,
    MaxLagOrTauMax,
    TwoTauMaxFuture,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TemporalMaskType {
    None,
    X,
    Y,
    Z,
    Xy,
    Xz,
    Yz,
    Xyz,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum TemporalSamples {
    Dense,
    RoleAware {
        cut_off: TemporalCutOff,
        propagate_through_max_lag: bool,
        mask_type: TemporalMaskType,
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CdnotsMissingStrategy {
    PairwiseComplete,
    VarEm,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CdnotsContext {
    None,
    Linear,
    LinearSine,
    LinearExponential,
    LinearQuadratic,
    Step,
    StepLinear,
}

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LatentProjectionEvidence {
    pub(crate) directed_edges: Vec<(usize, usize)>,
    pub(crate) bidirected_edges: Vec<(usize, usize)>,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum GraphicalIdentificationEvidence {
    Identified {
        expression: String,
        latex: String,
        projection: LatentProjectionEvidence,
    },
    Unidentifiable {
        hedge_graph: Vec<usize>,
        hedge_subgraph: Vec<usize>,
        projection: LatentProjectionEvidence,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum FrontdoorSetEvidence {
    Identified { mediators: Vec<usize> },
    NotIdentified,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum IdentificationEstimand {
    Ate,
    Att,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CounterfactualIdentificationEvidence {
    NotApplicable,
    Identified {
        treated_expression: String,
        untreated_expression: String,
    },
    Unidentifiable {
        reason: String,
    },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IdentifiedDiscreteCondition {
    pub(crate) variable: usize,
    pub(crate) state: usize,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum IdentifiedDiscreteQueryKind {
    Unconditional,
    Conditional {
        variable: usize,
        state: String,
        representative_value: f64,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum IdentifiedDiscreteResult {
    Identified {
        algorithm: &'static str,
        expression: String,
        latex: String,
        expectations: (f64, f64),
        effect: f64,
        distribution_low: Vec<(String, f64)>,
        distribution_high: Vec<(String, f64)>,
        normalization_low: f64,
        normalization_high: f64,
    },
    Unidentifiable {
        hedge_graph: Vec<usize>,
        hedge_subgraph: Vec<usize>,
    },
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
    Multicollinearity {
        rows: usize,
        columns: usize,
        correlation_threshold: f64,
        vif_threshold: f64,
    },
    PandasResampleDaily {
        rows: usize,
        columns: usize,
        target: ResamplingTarget,
        incomplete_bins: ResamplingIncompleteBins,
        aggregations: Vec<ResamplingAggregation>,
        imputed_cells: Vec<[usize; 2]>,
    },
    PcmciPlus {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
        samples: TemporalSamples,
    },
    Lpcmci {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
        samples: TemporalSamples,
    },
    Rpcmci {
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
    },
    Cdnots {
        rows: usize,
        columns: usize,
        max_lag: usize,
        alpha: f64,
        missing: CdnotsMissingStrategy,
        context: CdnotsContext,
    },
    CdnotsPlus {
        rows: usize,
        columns: usize,
        max_lag: usize,
        alpha: f64,
        missing: CdnotsMissingStrategy,
        context: CdnotsContext,
    },
    Grace {
        rows: usize,
        columns: usize,
        max_lag: usize,
        alpha: f64,
        context: CdnotsContext,
        gate_threshold: f64,
        epochs: usize,
        patience: usize,
        seed: u64,
    },
    Dynotears {
        rows: usize,
        columns: usize,
        max_lag: usize,
        lambda_w: f64,
        lambda_a: f64,
    },
    DirectLingam {
        rows: usize,
        columns: usize,
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
    Cmlp {
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
    },
    Clstm {
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
    },
    GrangerSsrF {
        rows: usize,
        max_lag: usize,
    },
    BackdoorIdentify {
        nodes: usize,
        names: Vec<String>,
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        unobserved: Vec<usize>,
        estimand: IdentificationEstimand,
    },
    DagCheck {
        rows: usize,
        columns: usize,
        /// One numeric matrix column per observed DAG node, in node order.
        node_columns: Vec<usize>,
        /// Directed edges as node positions.
        edges: Vec<(usize, usize)>,
        implications: Vec<DagImplicationCommand>,
        maximum_observations: usize,
        permutations: usize,
        significance_level: f64,
        /// Whole-graph relabeling is invalid after projecting latent nodes out of the DAG.
        run_falsification: bool,
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
    FrontdoorTwoStage {
        rows: usize,
        columns: usize,
        treatment: usize,
        mediator: usize,
        outcome: usize,
        first_stage_adjustment: Vec<usize>,
        second_stage_adjustment: Vec<usize>,
        control_value: f64,
        treatment_value: f64,
        uncertainty: FrontdoorUncertainty,
    },
    CountGlm {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        family: CountFamily,
    },
    NegativeBinomialIngarch {
        rows: usize,
        columns: usize,
        outcome: usize,
        link: IngarchLink,
        /// Contemporaneous regressor columns, including the treatment column.
        regressors: Vec<usize>,
        past_observation_lags: Vec<usize>,
        past_mean_lags: Vec<usize>,
        external_regressors: Vec<bool>,
        horizon: usize,
        /// One future baseline value per regressor; repeated over the forecast horizon.
        baseline_regressors: Vec<f64>,
        /// Matrix column whose future path is changed by the intervention schedule.
        intervention_regressor: usize,
        control_value: f64,
        treatment_value: f64,
        schedule: IngarchInterventionSchedule,
    },
    CountSeriesInterventionScan {
        rows: usize,
        columns: usize,
        outcome: usize,
        link: IngarchLink,
        past_observation_lags: Vec<usize>,
        past_mean_lags: Vec<usize>,
        candidate_reference_points: Vec<usize>,
        delta: f64,
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
        uncertainty: CausalEffectsUncertainty,
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
        correlation_max_lag: usize,
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
        cross_fit_folds: usize,
        alpha: f64,
    },
    PanelIntervention {
        rows: usize,
        /// One unit label and ordered time code per long-form observation.
        units: Vec<String>,
        times: Vec<i64>,
        placebo_replications: usize,
        seed: u64,
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
    BayesianGaussian {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
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
    IdentifiedDiscreteQuery {
        rows: usize,
        columns: usize,
        /// One matrix column per observed graph node, in full graph-node order.
        observed_nodes: Vec<usize>,
        /// Names and directed edges cover the full graph, including unobserved nodes.
        names: Vec<String>,
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        unobserved: Vec<usize>,
        bins: usize,
        condition: Option<IdentifiedDiscreteCondition>,
    },
    BinaryEtt {
        rows: usize,
        columns: usize,
        /// One matrix column per observed graph node, in the same order.
        observed_nodes: Vec<usize>,
        /// Names and directed edges cover the full graph, including unobserved nodes.
        names: Vec<String>,
        edges: Vec<(usize, usize)>,
        treatment: usize,
        outcome: usize,
        unobserved: Vec<usize>,
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
    DynamicLinearScmCounterfactual {
        rows: usize,
        columns: usize,
        /// One prepared-matrix column per observed graph node, in graph-node order.
        nodes: Vec<usize>,
        stat_lag: usize,
        /// Stationary directed marks indexed source, target, lag.
        graph: Vec<Vec<Vec<String>>>,
        treatment: usize,
        outcome: usize,
        timing: DynamicInterventionTiming,
        /// Number of returned time points, including the intervention time.
        steps: usize,
        interventions: (f64, f64),
        uncertainty: DynamicCounterfactualUncertainty,
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

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct DagImplicationCommand {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) given: Vec<usize>,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CountFamily {
    Poisson,
    NegativeBinomial,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum NeuralActivation {
    Sigmoid,
    Tanh,
    Relu,
    LeakyRelu,
    Identity,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum NeuralCmlpPenalty {
    GroupLasso,
    GroupSparseGroupLasso,
    Hierarchical,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NeuralStandardizationEvidence {
    pub(crate) means: Vec<f64>,
    pub(crate) scales: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VifEliminationEvidence {
    pub(crate) column: usize,
    /// Exact collinearity produces an infinite VIF, represented as null in JSON.
    pub(crate) vif: Option<f64>,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum IngarchInterventionSchedule {
    Point,
    Persistent,
    Decaying { delta: f64 },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum IngarchLink {
    Identity,
    Log,
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum TotalEffectEstimator {
    Linear {
        adjustment: CausalEffectsAdjustmentSelection,
    },
    Knn {
        k: usize,
        adjustment: CausalEffectsAdjustmentSelection,
    },
    WrightParents,
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsAdjustmentSelection {
    Optimal,
    MinimizedOptimal,
    CollidersMinimizedOptimal,
    Explicit { nodes: Vec<(usize, i32)> },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DynamicInterventionTiming {
    Point { time: usize },
    Persistent { start: usize },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsFitEvidence {
    Unfitted {
        requested: TotalEffectEstimator,
    },
    InvalidAdjustment {
        requested: TotalEffectEstimator,
        problems: Vec<CausalEffectsAdjustmentProblem>,
    },
    AdjustedLinear {
        selection: CausalEffectsAdjustmentSelection,
        adjustment_set: Vec<(usize, i32)>,
    },
    AdjustedKnn {
        k: usize,
        selection: CausalEffectsAdjustmentSelection,
        adjustment_set: Vec<(usize, i32)>,
    },
    WrightParents {
        coefficients: Vec<WrightCoefficientEvidence>,
        paths: Vec<WrightPathEvidence>,
        direct_effect: f64,
        indirect_effect: f64,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsAdjustmentProblem {
    QueryTreatment { node: (usize, i32) },
    QueryOutcome { node: (usize, i32) },
    LaterTreatmentOccurrence { node: (usize, i32) },
    ForbiddenNode { node: (usize, i32) },
    OpenNonCausalPath,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WrightCoefficientEvidence {
    pub(crate) parent: (usize, i32),
    pub(crate) child: (usize, i32),
    pub(crate) coefficient: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WrightPathEvidence {
    pub(crate) nodes: Vec<(usize, i32)>,
    pub(crate) coefficient: f64,
    pub(crate) contrast: f64,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsBlockLength {
    Fixed { length: usize },
    CubeRoot,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsUncertainty {
    None,
    Bootstrap {
        samples: usize,
        block_length: CausalEffectsBlockLength,
        confidence_level: f64,
        seed: u64,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CausalEffectsUncertaintyEvidence {
    None,
    Bootstrap {
        samples: usize,
        block_length: CausalEffectsBlockLength,
        resolved_block_length: usize,
        confidence_level: f64,
        seed: u64,
        prediction_intervals: [[f64; 2]; 2],
        effect_interval: [f64; 2],
        effect_draws: Vec<f64>,
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DynamicCounterfactualUncertainty {
    None,
    BlockBootstrap {
        samples: usize,
        block_length: CausalEffectsBlockLength,
        confidence_level: f64,
        seed: u64,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DynamicCounterfactualUncertaintyEvidence {
    None,
    BlockBootstrap {
        samples: usize,
        block_length: CausalEffectsBlockLength,
        resolved_block_length: usize,
        confidence_level: f64,
        seed: u64,
        effect_draws: Vec<Vec<f64>>,
        /// Equal-tail pointwise bounds, ordered lower then upper.
        pointwise_effect_interval: [Vec<f64>; 2],
        average_draws: Vec<f64>,
        average_interval: [f64; 2],
        cumulative_draws: Vec<f64>,
        cumulative_interval: [f64; 2],
    },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum FrontdoorUncertainty {
    None,
    Bootstrap {
        simulations: usize,
        sample_size_fraction: f64,
        confidence_level: f64,
        seed: u32,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum FrontdoorUncertaintyEvidence {
    None,
    Bootstrap {
        simulations: usize,
        sample_size_fraction: f64,
        confidence_level: f64,
        seed: u32,
        interval: [f64; 2],
    },
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
pub(crate) struct IngarchScanCandidateEvidence {
    pub(crate) reference_point: usize,
    pub(crate) score_statistic: f64,
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
#[serde(rename_all = "camelCase")]
pub(crate) struct SyntheticCrossFitFoldEvidence {
    pub(crate) held_out: Vec<usize>,
    pub(crate) weights: Vec<f64>,
    pub(crate) bias: f64,
    pub(crate) att: f64,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum SyntheticCrossFitEvidence {
    Available {
        att: f64,
        standard_error: f64,
        t_statistic: f64,
        degrees_of_freedom: usize,
        p_value: f64,
        confidence_interval: (f64, f64),
        block_size: usize,
        folds: Vec<SyntheticCrossFitFoldEvidence>,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DonorPlaceboEvidence {
    pub(crate) donor: usize,
    pub(crate) pre_mspe: f64,
    pub(crate) post_mspe: f64,
    pub(crate) mspe_ratio: Option<f64>,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DonorPlaceboInferenceEvidence {
    Available {
        treated_pre_mspe: f64,
        treated_post_mspe: f64,
        treated_mspe_ratio: Option<f64>,
        placebos: Vec<DonorPlaceboEvidence>,
        p_value: f64,
        n_valid_placebos: usize,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum SyntheticPredictionBandEvidence {
    Available {
        alpha: f64,
        intervals: Vec<(f64, f64)>,
        half_width: f64,
        pre_mspe: f64,
        post_mspe: f64,
        mspe_ratio: Option<f64>,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum PanelPlaceboEvidence {
    Available {
        replications: usize,
        seed: u64,
        standard_error: f64,
        estimates: Vec<f64>,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum PanelInTimeEvidence {
    Available {
        estimate: f64,
        effect_curve: Vec<f64>,
    },
    Unavailable {
        reason: String,
    },
}

/// Posterior expected-outcome curve over one adjustment covariate under both interventions.
/// Bands are the 3% and 97% posterior quantiles, matching the 94% summary interval.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BayesianGaussianCurve {
    pub standardised: bool,
    pub grid: Vec<f64>,
    pub control_lower: Vec<f64>,
    pub control_median: Vec<f64>,
    pub control_upper: Vec<f64>,
    pub treated_lower: Vec<f64>,
    pub treated_median: Vec<f64>,
    pub treated_upper: Vec<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DagImplicationEvidence {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) given: Vec<usize>,
    pub(crate) p_value: f64,
    pub(crate) adjusted_p_value: f64,
    pub(crate) observations: usize,
    pub(crate) decision: DagImplicationDecision,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DagImplicationDecision {
    Contradicted,
    NotRefuted,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DagUniformityEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
    pub(crate) tests: usize,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DagFalsificationEvidence {
    Completed {
        permutations: usize,
        given_lmc_violations: usize,
        given_lmc_tests: usize,
        given_lmc_violation_fraction: f64,
        permutation_lmc_violation_fractions: Vec<f64>,
        permutation_tpa_violation_fractions: Vec<f64>,
        p_value_lmc: f64,
        p_value_tpa: f64,
        permutations_in_markov_equivalence_class: usize,
        falsifiable: bool,
        falsified: bool,
    },
    Skipped {
        reason: &'static str,
    },
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
    Multicollinearity {
        observations: usize,
        variables: usize,
        correlation_threshold: f64,
        vif_threshold: f64,
        correlation: Vec<Vec<f64>>,
        correlation_keep: Vec<usize>,
        correlation_drop: Vec<usize>,
        correlation_clusters: Vec<Vec<usize>>,
        vif_keep: Vec<usize>,
        vif_drop: Vec<usize>,
        vif_history: Vec<VifEliminationEvidence>,
    },
    PandasResampled {
        values: Vec<f64>,
        timestamps_ms: Vec<i64>,
        imputed_cells: Vec<(usize, usize)>,
        source_rows: usize,
        output_rows: usize,
        incomplete_bins: usize,
        bins_dropped: usize,
        source_rows_dropped: usize,
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
    Rpcmci {
        observations: usize,
        variables: usize,
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
        regimes: Vec<Vec<f64>>,
        graphs: Vec<Vec<Vec<Vec<String>>>>,
        val_matrices: Vec<Vec<Vec<Vec<f64>>>>,
        p_matrices: Vec<Vec<Vec<Vec<f64>>>>,
        diff_g_all: Vec<Option<Vec<f64>>>,
        diff_g_best: Vec<f64>,
        error_free_annealings: usize,
    },
    Cdnots {
        observations: usize,
        observed_variables: usize,
        context_variables: Vec<String>,
        max_lag: usize,
        alpha: f64,
        missing: CdnotsMissingStrategy,
        context: CdnotsContext,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    CdnotsPlus {
        observations: usize,
        observed_variables: usize,
        context_variables: Vec<String>,
        max_lag: usize,
        alpha: f64,
        missing: CdnotsMissingStrategy,
        context: CdnotsContext,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
    },
    Grace {
        observations: usize,
        variables: usize,
        max_lag: usize,
        alpha: f64,
        context: CdnotsContext,
        gate_threshold: f64,
        lambda_l0: f64,
        epochs: usize,
        patience: usize,
        seed: u64,
        imputed_cells: usize,
        skeleton: Vec<Vec<Vec<bool>>>,
        gate_values: Vec<Vec<Vec<f64>>>,
        graph: Vec<Vec<Vec<bool>>>,
        loss: Vec<f64>,
        rmse: Vec<f64>,
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
    DirectLingam {
        observations: usize,
        variables: usize,
        causal_order: Vec<usize>,
        weights: Vec<Vec<f64>>,
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
    Cmlp {
        observations: usize,
        variables: usize,
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
        standardization: NeuralStandardizationEvidence,
        summary_scores: Vec<Vec<f64>>,
        summary_active: Vec<Vec<bool>>,
        lag_scores: Vec<Vec<Vec<f64>>>,
        lag_active: Vec<Vec<Vec<bool>>>,
        lag_order: Vec<usize>,
        loss: Vec<f64>,
        iterations: usize,
    },
    Clstm {
        observations: usize,
        variables: usize,
        context: usize,
        hidden: usize,
        lambda: f64,
        ridge_lambda: f64,
        learning_rate: f64,
        max_iter: usize,
        check_every: usize,
        lookback: usize,
        seed: u64,
        standardization: NeuralStandardizationEvidence,
        summary_scores: Vec<Vec<f64>>,
        summary_active: Vec<Vec<bool>>,
        loss: Vec<f64>,
        iterations: usize,
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
        frontdoor: FrontdoorSetEvidence,
        graphical_identification: GraphicalIdentificationEvidence,
        counterfactual_identification: CounterfactualIdentificationEvidence,
    },
    DagCheck {
        observations: usize,
        significance_level: f64,
        correction: &'static str,
        implications: Vec<DagImplicationEvidence>,
        uniformity: DagUniformityEvidence,
        falsification: DagFalsificationEvidence,
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
    FrontdoorTwoStage {
        observations: usize,
        treatment: usize,
        mediator: usize,
        outcome: usize,
        first_stage_adjustment: Vec<usize>,
        second_stage_adjustment: Vec<usize>,
        control_value: f64,
        treatment_value: f64,
        first_stage_params: Vec<f64>,
        second_stage_params: Vec<f64>,
        first_stage_effect: f64,
        second_stage_effect: f64,
        estimate: f64,
        uncertainty: FrontdoorUncertaintyEvidence,
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
    NegativeBinomialIngarch {
        observations: usize,
        outcome: usize,
        link: IngarchLink,
        regressors: Vec<usize>,
        past_observation_lags: Vec<usize>,
        past_mean_lags: Vec<usize>,
        external_regressors: Vec<bool>,
        horizon: usize,
        intervention_regressor: usize,
        control_value: f64,
        treatment_value: f64,
        schedule: IngarchInterventionSchedule,
        parameters: Vec<f64>,
        fitted_means: Vec<f64>,
        residuals: Vec<f64>,
        log_likelihood: f64,
        size: f64,
        dispersion: f64,
        score: Vec<f64>,
        iterations: usize,
        function_evaluations: usize,
        gradient_evaluations: usize,
        baseline_mean: Vec<f64>,
        intervention_mean: Vec<f64>,
        effect_path: Vec<f64>,
        average_effect: f64,
        cumulative_effect: f64,
    },
    CountSeriesInterventionScan {
        observations: usize,
        outcome: usize,
        link: IngarchLink,
        past_observation_lags: Vec<usize>,
        past_mean_lags: Vec<usize>,
        parameters: Vec<f64>,
        fitted_means: Vec<f64>,
        residuals: Vec<f64>,
        log_likelihood: f64,
        size: f64,
        dispersion: f64,
        candidates: Vec<IngarchScanCandidateEvidence>,
        strongest_reference_point: usize,
        delta: f64,
    },
    CausalEffectsTotal {
        observations: usize,
        tau_max: usize,
        no_causal_path: bool,
        identifiable: bool,
        mediators: Vec<(usize, i32)>,
        fit: CausalEffectsFitEvidence,
        interventions: [f64; 2],
        /// Predicted Y at each intervention value; empty when not identifiable.
        predictions: Vec<f64>,
        /// predictions[1] - predictions[0]; NaN when not identifiable.
        total_effect: f64,
        fitted_observations: usize,
        uncertainty: CausalEffectsUncertaintyEvidence,
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
        cross_fit: SyntheticCrossFitEvidence,
        donor_placebo: DonorPlaceboInferenceEvidence,
        conformal_band: SyntheticPredictionBandEvidence,
        gaussian_band: SyntheticPredictionBandEvidence,
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
        synthetic_control_placebo: PanelPlaceboEvidence,
        synthetic_did_placebo: PanelPlaceboEvidence,
        synthetic_control_in_time: PanelInTimeEvidence,
        synthetic_did_in_time: PanelInTimeEvidence,
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
    BayesianGaussian {
        observations: usize,
        warmup: usize,
        samples: usize,
        chains: usize,
        seed: u64,
        effect_mean: f64,
        effect_sd: f64,
        effect_median: f64,
        hdi_lower: f64,
        hdi_upper: f64,
        probability_positive: f64,
        sigma_mean: f64,
        divergences: usize,
        acceptance_rate: f64,
        mean_accept_probability: f64,
        step_size: f64,
        histogram_start: f64,
        histogram_bin_width: f64,
        histogram_counts: Vec<u32>,
        curves: Vec<BayesianGaussianCurve>,
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
    IdentifiedDiscreteQuery {
        observations: usize,
        bins: usize,
        /// State count per observed node, in full graph-node order with latent nodes omitted.
        state_counts: Vec<usize>,
        treatment_states: (String, String),
        query: IdentifiedDiscreteQueryKind,
        result: IdentifiedDiscreteResult,
    },
    BinaryEtt {
        observations: usize,
        treatment: usize,
        outcome: usize,
        treated_potential_outcome_mean: f64,
        untreated_potential_outcome_mean: f64,
        effect_on_treated: f64,
        treated_expression: String,
        untreated_expression: String,
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
    DynamicLinearScmCounterfactual {
        observations: usize,
        fitted_observations: usize,
        max_lag: usize,
        order: Vec<usize>,
        equations: Vec<DynamicScmEquationEvidence>,
        timing: DynamicInterventionTiming,
        interventions: (f64, f64),
        start: usize,
        factual_outcome: Vec<f64>,
        counterfactual_low: Vec<f64>,
        counterfactual_high: Vec<f64>,
        effects: Vec<f64>,
        average_effect: f64,
        cumulative_effect: f64,
        uncertainty: DynamicCounterfactualUncertaintyEvidence,
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
pub(crate) enum ResamplingTarget {
    Weekly,
    Monthly,
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ResamplingIncompleteBins {
    Keep,
    Drop,
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ResamplingAggregation {
    Mean,
    Sum,
    Median,
    Minimum,
    Maximum,
    First,
    Last,
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
pub(crate) struct DynamicScmParentEvidence {
    pub(crate) variable: usize,
    pub(crate) lag: usize,
    pub(crate) coefficient: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DynamicScmEquationEvidence {
    pub(crate) variable: usize,
    pub(crate) intercept: f64,
    pub(crate) parents: Vec<DynamicScmParentEvidence>,
    pub(crate) residual_scale: f64,
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
    pub(crate) observed: Vec<f64>,
    pub(crate) trend: Vec<f64>,
    pub(crate) seasonal: Vec<f64>,
    pub(crate) remainder: Vec<f64>,
    pub(crate) robust_weights: Vec<f64>,
    pub(crate) trend_strength: f64,
    pub(crate) seasonal_strength_before: f64,
    pub(crate) seasonal_strength_after: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SeriesStructureEvidence {
    pub(crate) column: usize,
    pub(crate) trend_strength: Option<f64>,
    pub(crate) seasonal_strength: Option<f64>,
    pub(crate) correlation_max_lag: usize,
    pub(crate) acf: Vec<f64>,
    pub(crate) acf_limits: Vec<f64>,
    pub(crate) pacf: Vec<f64>,
    pub(crate) pacf_limits: Vec<f64>,
    /// Segment ends as one-based row numbers, without the final row.
    pub(crate) change_points: Vec<usize>,
    pub(crate) pelt_penalty: f64,
}
