//! The command and result contract the worker speaks: serde-tagged enums and the evidence shapes.

use super::*;

#[derive(Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum StructuralVersion { GaussianComponentsV1 }
#[derive(Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum StructuralTrend { Level, Linear, Semilocal }
#[derive(Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum StructuralSeasonality {
    None,
    Seasonal { seasons: usize, duration: usize },
    Harmonic { period: f64, pairs: usize },
}
#[derive(Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StructuralModel {
    pub version: StructuralVersion,
    pub trend: StructuralTrend,
    pub seasonality: StructuralSeasonality,
}
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum StructuralComponent { Trend, Seasonal, Predictor { column: usize } }
#[derive(Serialize)]
pub(crate) struct StructuralContribution {
    pub component: StructuralComponent,
    pub mean: Vec<f64>, pub lower: Vec<f64>, pub upper: Vec<f64>,
}
#[derive(Serialize)]
pub(crate) struct ControlInclusion { pub column: usize, pub probability: f64 }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImpactPosterior {
    pub control_inclusion: Vec<ControlInclusion>,
    pub observations: usize, pub n_pre: usize, pub n_post: usize, pub post_end: usize,
    pub outcome: usize, pub controls: Vec<usize>,
    pub draws: usize, pub warmup: usize, pub seed: u32, pub level: f64,
    pub pre_intervention_path: PreInterventionPath,
    pub counterfactual: Vec<f64>, pub counterfactual_se: Vec<f64>,
    pub counterfactual_lower: Vec<f64>, pub counterfactual_upper: Vec<f64>,
    pub pointwise: Vec<f64>, pub pointwise_lower: Vec<f64>, pub pointwise_upper: Vec<f64>,
    pub cumulative_lower: Vec<f64>, pub cumulative_upper: Vec<f64>,
    pub cumulative: f64, pub average: f64,
    pub average_summary: ImpactSummary, pub cumulative_summary: ImpactSummary,
}

#[derive(Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum AdjustedDidSpecification {
    Regression,
    DoublyRobust { folds: usize, seed: u32, trimming: f64, normalization: DidNormalization },
}
#[derive(Serialize, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum DidNormalization { InSample, Population }
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum AdjustedDidInference {
    IndependentErrors { degrees_of_freedom: usize, coefficients: Vec<f64>, standard_errors: Vec<f64>, intervals: Vec<[f64; 2]> },
    CrossFitted { propensity: Vec<f64>, optimizer_status: Vec<PropensityStatus> },
}
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum PropensityStatus { ProjectedGradient, FunctionTolerance, IterationLimit, LineSearchFailed }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PosteriorQuantity {
    pub mean: f64,
    pub lower: f64,
    pub upper: f64,
    pub sd: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RdEstimateEvidence {
    pub value: f64,
    pub standard_error: f64,
    pub interval: [f64; 2],
}

/// What the model says the outcome was doing before the intervention, so the counterfactual can
/// be judged against the rows it was fitted on. `steps` carries the row each value belongs to,
/// because the least-squares route leaves out its diffuse first row.
#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum PreInterventionPath {
    Fitted { steps: Vec<usize>, observed: Vec<f64>, counterfactual: Vec<f64>, se: Vec<f64> },
    Sampled { steps: Vec<usize>, observed: Vec<f64>, counterfactual: Vec<f64>, lower: Vec<f64>, upper: Vec<f64> },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImpactSummary {
    pub actual: f64,
    pub predicted: PosteriorQuantity,
    pub absolute: PosteriorQuantity,
    pub relative: PosteriorQuantity,
    pub tail_probability: f64,
}

#[derive(Default, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PanelPrimary { Did, #[default] SyntheticDid }


#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum BootstrapMethod { Percentile, Pivot, Normal }

#[derive(Default, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum TLearnerUncertainty {
    #[default]
    None,
    Bootstrap { samples: usize, seed: u32, level: f64, method: BootstrapMethod },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BootstrapAverage {
    pub interval: [f64; 2],
    pub standard_error_bound: f64,
}

/// How the cross-fitted T-learner picks each arm's candidate: by its own grid search, or as chosen
/// by a search that already ran on that arm's rows, such as one spread across a worker pool.
#[derive(Clone, Copy, Default, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(crate) enum ArmSelection {
    #[default]
    Search,
    Chosen { treated: ChosenArm, control: ChosenArm },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ChosenArm {
    pub(crate) learning_rate: f64,
    pub(crate) max_depth: usize,
    pub(crate) n_estimators: usize,
    /// Mean held-out ROC AUC of the candidate over the search folds.
    pub(crate) validation_auc: f64,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum TLearnerUncertaintyEvidence {
    None,
    Bootstrap { samples: usize, seed: u32, level: f64, method: BootstrapMethod,
        intervals: Vec<[f64; 2]>, standard_errors: Vec<f64>, average: BootstrapAverage },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum ArdlLongRun {
    Recorded { observed: Vec<f64>, departures: Vec<f64> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum VecmLongRun {
    NotFitted,
    Recorded { start_row: usize, departures: Vec<Vec<f64>> },
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum VecmForecast {
    NotRequested,
    NotFitted,
    Recorded {confidence:f64, mean:Vec<Vec<f64>>, lower:Vec<Vec<f64>>, upper:Vec<Vec<f64>>, covariance:Vec<Vec<Vec<f64>>>},
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ForestSplitCommand {
    LogRank,
    ExtraTrees,
}

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

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SurvivalFamily {
    Exponential,
    Weibull,
    WeibullPh,
    LogNormal,
    Gamma,
    Gompertz,
    LogLogistic,
    GeneralizedGamma,
    GeneralizedF,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum SurvivalObservationCommand {
    RightCensored {
        duration: usize,
        event: usize,
    },
    StartStop {
        start: usize,
        stop: usize,
        event: usize,
    },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum SurvivalRowFrequencyCommand {
    OneObservationPerRow,
    FrequencyColumn { column: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxEntryCommand {
    NotUsed,
    Column { column: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxWeightsCommand {
    Equal,
    Column { column: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxStrataCommand {
    Unstratified,
    Column { column: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxStandardErrorsCommand {
    ModelBased,
    Robust,
    Clustered { column: usize },
    ClusteredBreslow { column: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxPenaltyCommand {
    Unpenalized,
    ElasticNet { penalizer: f64, l1_ratio: f64 },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxObservationCommand {
    RightCensored {
        duration: usize,
        event: usize,
        entry: CoxEntryCommand,
        standard_errors: CoxStandardErrorsCommand,
        #[serde(default)]
        frailty: CoxFrailtyCommand,
    },
    StartStop {
        subject: usize,
        start: usize,
        stop: usize,
        event: usize,
    },
}

#[derive(Clone, Copy, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CoxTiesCommand {
    #[default]
    Efron,
    Breslow,
}

#[derive(Clone, Copy, Default, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum CoxFrailtyCommand {
    #[default]
    None,
    Gamma {
        column: usize,
        ties: CoxTiesCommand,
    },
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CoxTiesEvidence {
    Efron,
    Breslow,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxFrailtyTermTestEvidence {
    pub(crate) statistic: f64,
    pub(crate) df: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CoxFrailtyEvidence {
    None,
    Gamma {
        groups: usize,
        ties: CoxTiesEvidence,
        theta: f64,
        term_test: CoxFrailtyTermTestEvidence,
        degrees_of_freedom: f64,
        outer_iterations: usize,
        inner_iterations: usize,
        history: Vec<[f64; 3]>,
        standard_errors2: Vec<f64>,
    },
}

#[derive(Clone, Copy, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CoxObservationEvidence {
    RightCensored { delayed_entry: bool },
    StartStop { subjects: usize },
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CoxStandardErrorsEvidence {
    ModelBased,
    Robust,
    Clustered,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum CoxFittingEvidence {
    Efron,
    GammaFrailty,
    ClusteredBreslow {
        clusters: usize,
        convergence: CoxConvergenceEvidence,
        robust_covariance: Vec<f64>,
        score_test: f64,
        robust_score_test: f64,
        wald_test: f64,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CoxConvergenceEvidence {
    Converged,
    ConvergedDuringHalving,
    IterationLimit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxCoefficientEvidence {
    pub(crate) coefficient: f64,
    pub(crate) hazard_ratio: f64,
    pub(crate) standard_error: f64,
    pub(crate) coefficient_interval: [f64; 2],
    pub(crate) hazard_ratio_interval: [f64; 2],
    pub(crate) z: f64,
    pub(crate) p_value: f64,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxBaselineEstimateEvidence {
    pub(crate) time: f64,
    pub(crate) hazard: f64,
    pub(crate) cumulative_hazard: f64,
    pub(crate) survival: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxStratumBaselineEvidence {
    pub(crate) stratum: usize,
    pub(crate) estimates: Vec<CoxBaselineEstimateEvidence>,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CoxBaselineEvidence {
    Shared {
        estimates: Vec<CoxBaselineEstimateEvidence>,
    },
    Stratified {
        curves: Vec<CoxStratumBaselineEvidence>,
    },
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum CoxTimeTransformEvidence {
    EventRank,
    KaplanMeier,
    Identity,
    LogTime,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxProportionalHazardsTestEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CoxProportionalHazardsTransformEvidence {
    pub(crate) transform: CoxTimeTransformEvidence,
    pub(crate) tests: Vec<CoxProportionalHazardsTestEvidence>,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum CoxProportionalHazardsEvidence {
    Recorded {
        transforms: Vec<CoxProportionalHazardsTransformEvidence>,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum AftFamilyCommand {
    Weibull,
    LogLogistic,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum AftFamilyEvidence {
    Weibull,
    LogLogistic,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AftCoefficientEvidence {
    pub(crate) coefficient: f64,
    pub(crate) time_ratio: f64,
    pub(crate) standard_error: f64,
    pub(crate) coefficient_interval: [f64; 2],
    pub(crate) time_ratio_interval: [f64; 2],
    pub(crate) z: f64,
    pub(crate) p_value: f64,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum NelsonAalenTiesCommand {
    Discrete,
    Smoothed,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum WideStateColumnsCommand {
    NotApplicable,
    Recorded { time: usize, status: usize },
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum WideEntryCommand {
    Shared { state: usize, time: f64 },
    Columns { state: usize, time: usize },
}

#[derive(serde::Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub(crate) enum MultiStateInputCommand {
    PreparedRows {
        start: usize,
        stop: usize,
        event: usize,
        from: usize,
        to: usize,
    },
    LongitudinalStates {
        subject: usize,
        time: usize,
        state: usize,
        allowed: Vec<Vec<bool>>,
    },
    WideEvents {
        states: Vec<WideStateColumnsCommand>,
        transitions: Vec<Vec<Option<usize>>>,
        entry: WideEntryCommand,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GroupSurvivalDiagnostics {
    pub(crate) cumulative_hazard: Vec<[f64; 2]>,
    pub(crate) smoothed_hazard: Vec<[f64; 2]>,
    pub(crate) at_risk: Vec<[f64; 2]>,
    pub(crate) censor_times: Vec<f64>,
    pub(crate) restricted_mean: f64,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum ComparisonSurvivalDiagnostics {
    Recorded {
        group_zero: GroupSurvivalDiagnostics,
        group_one: GroupSurvivalDiagnostics,
        crossing_times: Vec<f64>,
    },
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum SurvivalSummary<T: Serialize> {
    Recorded { result: T },
    Unavailable { reason: String },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ObservedConversionEvidence {
    pub(crate) group_zero_rate: f64,
    pub(crate) group_one_rate: f64,
    pub(crate) difference: f64,
    pub(crate) standard_error: f64,
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum FixedPointScaleEvidence {
    Naive,
    Log,
    ComplementaryLogLog,
    ArcsineSquareRoot,
    Logit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FixedPointTestEvidence {
    pub(crate) scale: FixedPointScaleEvidence,
    pub(crate) group_zero_interval: [f64; 2],
    pub(crate) group_one_interval: [f64; 2],
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FixedTimeConversionEvidence {
    pub(crate) time: f64,
    pub(crate) group_zero_rate: f64,
    pub(crate) group_one_rate: f64,
    pub(crate) difference: f64,
    pub(crate) standard_error: f64,
    pub(crate) interval: [f64; 2],
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
    pub(crate) scale_tests: Vec<FixedPointTestEvidence>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GRhoEvidence {
    pub(crate) rho: f64,
    pub(crate) observed: [f64; 2],
    pub(crate) expected: [f64; 2],
    pub(crate) variance: [[f64; 2]; 2],
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ConstraintCiTest {
    FisherZ,
    Kci,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum JpcmciNodeClass {
    System,
    TimeContext,
    SpaceContext,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JpcmciNodeEvidence {
    pub(crate) variable: usize,
    pub(crate) lag: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JpcmciSeparatingSetEvidence {
    pub(crate) source: usize,
    pub(crate) target: usize,
    pub(crate) lag: usize,
    pub(crate) variables: Vec<JpcmciNodeEvidence>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JpcmciAmbiguousTripleEvidence {
    pub(crate) left: JpcmciNodeEvidence,
    pub(crate) middle: usize,
    pub(crate) right: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct BackgroundKnowledgeCommand {
    pub(crate) forbidden: Vec<(usize, usize)>,
    pub(crate) required: Vec<(usize, usize)>,
    pub(crate) forbidden_patterns: Vec<(String, String)>,
    pub(crate) required_patterns: Vec<(String, String)>,
    pub(crate) tiers: Vec<Option<usize>>,
    pub(crate) forbidden_within_tiers: Vec<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConstraintSeparatingSetEvidence {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) variables: Vec<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConstraintCiEvidence {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) conditions: Vec<usize>,
    pub(crate) p_value: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FciEdgePropertyEvidence {
    pub(crate) left: usize,
    pub(crate) right: usize,
    pub(crate) directness: Option<&'static str>,
    pub(crate) latent_confounding: Option<&'static str>,
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

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum InstrumentSetEvidence {
    Identified { instruments: Vec<usize> },
    NotIdentified,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum InstrumentalVariableRoute {
    WaldRatio,
    CovarianceRatio,
    TwoStageLeastSquares,
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

/// Which discretised state a condition fixes: the first, the last, or one by position.
#[derive(Clone, Copy, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum DiscreteConditionState {
    Lowest,
    Index { state: usize },
    Highest,
}

#[derive(Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IdentifiedDiscreteCondition {
    pub(crate) variable: usize,
    pub(crate) state: DiscreteConditionState,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DiscreteStateQuery {
    BayesianNetwork,
    IdentifiedExpression,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DiscreteStateStrategyEvidence {
    ObservedStates { states: usize },
    Quantiles { requested: usize, populated: usize },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DiscreteStatePreparationEvidence {
    pub(crate) node: usize,
    pub(crate) name: String,
    pub(crate) strategy: DiscreteStateStrategyEvidence,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DiscreteStateProblemEvidence {
    NoFiniteObservations {
        observations: usize,
    },
    SingleObservedState {
        value: f64,
        observations: usize,
    },
    QuantileCollapse {
        distinct_values: usize,
        requested_states: usize,
        populated_states: usize,
    },
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
    SurrogatePath { request: crate::surrogate::PathRequest },
    SurrogateDiagnostics { request: crate::surrogate::DiagnosticRequest },
    Surrogate { request: crate::surrogate::Request },
    CountRegression { request: crate::count_regression::Request },
    StaggeredDid { request: crate::staggered_did::Request },
    Bacon { request: crate::bacon::Request },
    PanelRegression { request: crate::panel_regression::Request },
    SunAbraham { request: crate::remix_extensions::SunRequest },
    RidgeAugmentedSynthetic { request: crate::remix_extensions::RidgeRequest },
    RootCause { request: crate::root_cause::Request },
    GcmEffects { request: crate::gcm_effects::Request },
    GcmInfluence { request: crate::gcm_influence::Request },
    RootCauseChecks { request: crate::root_cause_checks::Request },
    ArdlModel { rows: usize, columns: usize, model: crate::ardl_model::Request },
    HonestDid { model: crate::honest_did::Request },
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
    FlexSurv {
        rows: usize,
        columns: usize,
        observation: SurvivalObservationCommand,
        row_frequency: SurvivalRowFrequencyCommand,
        covariates: Vec<usize>,
        family: SurvivalFamily,
        prediction_times: Vec<f64>,
    },
    CoxRegression {
        rows: usize,
        columns: usize,
        observation: CoxObservationCommand,
        weights: CoxWeightsCommand,
        strata: CoxStrataCommand,
        covariates: Vec<usize>,
        penalty: CoxPenaltyCommand,
        confidence_level: f64,
    },
    NonparametricSurvival {
        rows: usize,
        columns: usize,
        duration: usize,
        event: usize,
        row_frequency: SurvivalRowFrequencyCommand,
        prediction_times: Vec<f64>,
        ties: NelsonAalenTiesCommand,
    },
    PenalizedAft {
        rows: usize,
        columns: usize,
        duration: usize,
        event: usize,
        covariates: Vec<usize>,
        family: AftFamilyCommand,
        penalizer: f64,
        confidence_level: f64,
        prediction_times: Vec<f64>,
    },
    Aalen {
        rows: usize,
        columns: usize,
        duration: usize,
        event: usize,
        covariates: Vec<usize>,
    },
    SurvivalForest {
        rows: usize,
        columns: usize,
        duration: usize,
        event: usize,
        covariates: Vec<usize>,
        categorical: Vec<usize>,
        trees: usize,
        mtry: usize,
        seed: u32,
        min_node_size: usize,
        min_bucket: usize,
        prediction_row: usize,
        split_rule: ForestSplitCommand,
    },
    ComparisonSurvival {
        rows: usize,
        columns: usize,
        duration: usize,
        event: usize,
        group: usize,
        truncation_time: f64,
        permutations: usize,
        seed: u32,
    },
    MultiStateSurvival {
        rows: usize,
        columns: usize,
        input: MultiStateInputCommand,
        family: SurvivalFamily,
        prediction_times: Vec<f64>,
    },
    PcmciPlus {
        rows: usize,
        columns: usize,
        tau_max: usize,
        pc_alpha: f64,
        samples: TemporalSamples,
    },
    Jpcmciplus {
        rows: usize,
        datasets: usize,
        periods: usize,
        observed_columns: usize,
        classes: Vec<JpcmciNodeClass>,
        time_dummy: bool,
        space_dummy: bool,
        tau_max: usize,
        pc_alpha: f64,
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
    PcStable {
        rows: usize,
        columns: usize,
        names: Vec<String>,
        alpha: f64,
        max_depth: Option<usize>,
        ci_test: ConstraintCiTest,
        background: BackgroundKnowledgeCommand,
    },
    Fci {
        rows: usize,
        columns: usize,
        names: Vec<String>,
        alpha: f64,
        max_depth: Option<usize>,
        max_path_length: Option<usize>,
        ci_test: ConstraintCiTest,
        background: BackgroundKnowledgeCommand,
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
        /// The error process beside the Newey-West interval: none, or an ARMA fitted by maximum likelihood.
        error_model: LinearErrorModel,
        /// The fixed effects the regression absorbs, when it has any.
        #[serde(default)]
        fixed_effects: Option<FixedEffectsModel>,
    },
    PropensityWeighting {
        #[serde(default)]
        target: PropensityTarget,
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        scale: PropensityWeightScale,
        fit: WeightingFit,
    },
    PropensityMatching {
        #[serde(default)]
        target: PropensityTarget,
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        model: PropensityModel,
    },
    /// One slice of the boosted grid: a single learning rate and depth over every tree count, which
    /// is the unit the prefix scoring already works in.
    PropensityGridSlice {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        learning_rate: f64,
        max_depth: usize,
        n_estimators: Vec<usize>,
        splits: usize,
        min_samples_leaf: usize,
        min_samples_split: usize,
        seed: u32,
    },
    DoublyRobust {
        #[serde(default)]
        target: PropensityTarget,
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        model: LogisticModel,
        bootstrap: Option<PropensityBootstrapRequest>,
    },
    ContinuousGps {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        scale: GpsWeightScale,
        bootstrap: Option<PropensityBootstrapRequest>,
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
        uncertainty: BootstrapUncertainty,
    },
    InstrumentalVariable {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        instruments: Vec<usize>,
        uncertainty: BootstrapUncertainty,
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
    InterruptedSeries {
        rows: usize,
        columns: usize,
        outcome: usize,
        model: InterruptedModel,
        /// The first row after the event, 0-based.
        intervention_row: usize,
        lag: usize,
        impact: InterruptedImpact,
        seasonal: InterruptedSeasonal,
        ljung_box_lags: usize,
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
    SharpRd {
        rows: usize,
        cutoff: f64,
    },
    CausalImpact {
        rows: usize,
        columns: usize,
        outcome: usize,
        controls: Vec<usize>,
        n_pre: usize,
        post_end: usize,
        max_iter: usize,
    },
    StructuralCausalImpact {
        rows: usize, columns: usize, outcome: usize, controls: Vec<usize>,
        n_pre: usize, post_end: usize, draws: usize, warmup: usize, seed: u32,
        model: StructuralModel,
    },
    BayesianCausalImpact {
        rows: usize,
        columns: usize,
        outcome: usize,
        controls: Vec<usize>,
        n_pre: usize,
        post_end: usize,
        draws: usize,
        warmup: usize,
        seed: u32,
        prior_level_sd: f64,
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
        /// Group effects over an effect modifier, or none for the plain average.
        groups: DmlGroups,
    },
    /// EconML's T-learner: one random forest per treatment arm on the adjustment columns, and the
    /// effect at every row as the treated prediction minus the control prediction.
    CausalForest {
        rows: usize, columns: usize, treatment: usize, outcome: usize,
        adjustment: Vec<usize>, target: crate::causal_forest::Target,
        configuration: crate::causal_forest::Configuration,
        #[serde(default)]
        column_names: Vec<String>,
    },
    TLearner {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        /// Seeds both forests, as the reference clones one estimator per arm.
        seed: u32,
        #[serde(default)]
        uncertainty: TLearnerUncertainty,
    },
    /// A T-learner whose outcome model per arm is a gradient-boosted classifier chosen by one grid
    /// search on all of that arm's rows, then refitted in each of two halves stratified on the
    /// outcome to predict the other half.
    CrossFittedTLearner {
        rows: usize,
        columns: usize,
        treatment: usize,
        outcome: usize,
        adjustment: Vec<usize>,
        learning_rate: Vec<f64>,
        max_depth: Vec<usize>,
        n_estimators: Vec<usize>,
        splits: usize,
        min_samples_leaf: usize,
        min_samples_split: usize,
        seed: u32,
        #[serde(default)]
        selection: ArmSelection,
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
    PredictorSyntheticControl { request: crate::predictor_synthetic_control::Request },
    PanelAdjusted {
        rows: usize, columns: usize, units: Vec<String>, times: Vec<i64>, specification: AdjustedDidSpecification,
    },
    PanelIntervention {
        #[serde(default)]
        primary: PanelPrimary,
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
    NetworkQuery { query: crate::network_query::NetworkQuery },
    ConditionalGaussianQuery { query: crate::conditional_gaussian_query::Query },
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
        #[serde(default)]
        forecast_steps: Option<usize>,
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

/// A percentile interval over refitted rounds, with the rounds that stopped early named rather
/// than hidden.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PropensityIntervalEvidence {
    pub(crate) lower: f64,
    pub(crate) upper: f64,
    pub(crate) rounds: usize,
    pub(crate) level: f64,
    pub(crate) unconverged: Vec<usize>,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PropensityBootstrapRequest {
    pub(crate) rounds: usize,
    pub(crate) seed: u32,
    pub(crate) level: f64,
}

/// Which treatment model fits the propensity score. statsmodels and sklearn do not agree to
/// better than their own convergence, so the choice is the reader's rather than an implementation
/// detail.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum LogisticModel {
    /// statsmodels `Logit`, Newton-Raphson over a LAPACK solve.
    Newton,
    /// sklearn `LogisticRegression(penalty=None)`.
    Lbfgsb { max_iter: usize },
}

/// A boosted treatment model: a `GridSearchCV` over the tree grid scored by ROC AUC, then the
/// chosen candidate refitted on the whole sample.
/// Whether the chosen model scores the rows it was fitted on, or each half is scored by the model
/// fitted on the other half.
#[derive(Clone, Copy, PartialEq, Eq, serde::Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum BoostedScoring { OneModel, CrossFitted }

/// The areas under the ROC curve each scoring can report.
#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum BoostedScoringEvidence {
    OneModel {
        /// The chosen candidate's mean cross-validated ROC AUC.
        validation_auc: f64,
        /// ROC AUC of the scores the fitted model gives the rows it was fitted on.
        fitted_auc: f64,
    },
    CrossFitted {
        /// ROC AUC of the scores each half received from the model fitted on the other half.
        auc: f64,
    },
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BoostedTreatmentModel {
    pub(crate) learning_rate: Vec<f64>,
    pub(crate) max_depth: Vec<usize>,
    pub(crate) n_estimators: Vec<usize>,
    pub(crate) splits: usize,
    pub(crate) min_samples_leaf: usize,
    pub(crate) min_samples_split: usize,
    pub(crate) seed: u32,
    pub(crate) scoring: BoostedScoring,
    /// How many candidates the caller scored before choosing this one, which a refit cannot know.
    pub(crate) candidates_searched: Option<usize>,
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum PropensityModel {
    Logistic { model: LogisticModel },
    Boosted { model: BoostedTreatmentModel },
}

/// A bootstrap refits the treatment model on every replicate, which the kernels support for a
/// logistic fit alone. The request sits inside that variant, so the other case cannot carry one.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum WeightingFit {
    Logistic {
        model: LogisticModel,
        bootstrap: Option<PropensityBootstrapRequest>,
    },
    Boosted {
        model: BoostedTreatmentModel,
    },
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArmChoices {
    pub(crate) treated: ArmChoice,
    pub(crate) control: ArmChoice,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArmChoice {
    pub(crate) learning_rate: f64,
    pub(crate) max_depth: usize,
    pub(crate) n_estimators: usize,
    /// Mean held-out ROC AUC of the chosen candidate over the search folds.
    pub(crate) validation_auc: f64,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GridCandidateScore {
    pub(crate) learning_rate: f64,
    pub(crate) max_depth: usize,
    pub(crate) n_estimators: usize,
    pub(crate) mean_score: f64,
}

/// What the run can say about the treatment model, which differs by model.
#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum TreatmentModelEvidence {
    Logistic {
        parameters: usize,
        converged: bool,
    },
    Boosted {
        learning_rate: f64,
        max_depth: usize,
        n_estimators: usize,
        candidates: usize,
        scoring: BoostedScoringEvidence,
    },
}

/// Weights as the inverse conditional density, or stabilized by the marginal density of the
/// treatment. With a continuous treatment the chapter treats stabilizing as necessary.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum GpsWeightScale {
    InverseDensity,
    Stabilized,
}

/// Weights as the inverse propensity, or stabilized by the marginal treatment prevalence.
#[derive(Clone, Copy, Debug, Default, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PropensityTarget { #[default] Ate, Att }
impl PropensityTarget {
    pub(crate) fn kernel(self) -> hirmos_causal_core::propensity::Target {
        match self { Self::Ate => hirmos_causal_core::propensity::Target::Ate, Self::Att => hirmos_causal_core::propensity::Target::Att }
    }
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PropensityWeightScale {
    InverseProbability,
    Stabilized,
}

/// The error process of a linear fit: independent errors with a Newey-West interval, a
/// heteroskedasticity-consistent (HC1) covariance, a covariance clustered by the labels in one
/// column of the matrix, or an ARMA(p, q) process fitted by maximum likelihood with the regression.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum LinearErrorModel {
    NeweyWest,
    Hc1,
    Cluster { column: usize },
    Arma { p: usize, q: usize, max_iter: usize },
}

/// An ARMA(p, q) error process as fitted: its coefficients, variance, likelihood and the
/// optimiser's outcome.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArmaErrorEvidence {
    pub(crate) covariance: ArmaCovarianceEvidence,
    pub(crate) p: usize,
    pub(crate) q: usize,
    pub(crate) ar: Vec<InterruptedTermEvidence>,
    pub(crate) ma: Vec<InterruptedTermEvidence>,
    pub(crate) sigma2: f64,
    pub(crate) log_likelihood: f64,
    pub(crate) aic: f64,
    pub(crate) bic: f64,
    pub(crate) iterations: usize,
    pub(crate) converged: bool,
}

/// Numerical rank retained by the OPG pseudoinverse, independent of optimiser convergence.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(crate) enum ArmaCovarianceEvidence {
    FullRank,
    RankDeficient { rank: usize, parameters: usize },
}

/// The adjusted linear regression's treatment coefficient under the requested error treatment,
/// beside the OLS and Newey-West readings of the same design: the HC1 or clustered interval of
/// the OLS coefficient, or the coefficient refitted with an ARMA error process.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum LinearErrorEvidence {
    NeweyWest,
    Hc1 { standard_error: f64, interval: [f64; 2], p_value: f64 },
    Cluster { clusters: usize, standard_error: f64, interval: [f64; 2], p_value: f64 },
    Arma { estimate: f64, standard_error: f64, interval: [f64; 2], p_value: f64, errors: ArmaErrorEvidence },
}

/// The fixed effects a regression absorbs: one per unit, or one per unit and one per period, from
/// label columns of the matrix.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum FixedEffectsModel {
    Unit { column: usize },
    Time { column: usize },
    UnitAndTime { unit: usize, time: usize },
}

/// Whether the regression absorbed fixed effects, and what they absorbed.
#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum FixedEffectsEvidence {
    None,
    Time { column: usize, periods: usize, absorbed: Vec<usize> },
    /// `absorbed` holds the design columns with no variation within a unit.
    Unit { column: usize, units: usize, absorbed: Vec<usize> },
    /// `absorbed` holds the design columns the unit and period effects together absorb.
    UnitAndTime { unit: usize, time: usize, units: usize, periods: usize, absorbed: Vec<usize> },
}

/// Which fit the series gets, with only the settings that fit needs.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum InterruptedModel {
    Continuous { errors: ContinuousErrors },
    /// The paper's quasi-Poisson model; the exposure column's log is the offset when given.
    Count { exposure: Option<usize> },
}

/// How a continuous series' terms are fitted: OLS with Newey-West errors (statsmodels'
/// bandwidth when none is given), or by maximum likelihood with ARMA(p, q) errors.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum ContinuousErrors {
    NeweyWest { max_lags: Option<usize> },
    Arma { p: usize, q: usize, max_iter: usize },
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum InterruptedModelEvidence {
    Continuous { errors: ContinuousErrorEvidence },
    Count { exposure: Option<usize>, dispersion: f64 },
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum ContinuousErrorEvidence {
    NeweyWest { max_lags: usize },
    Arma(ArmaErrorEvidence),
}

/// The impact model declared before fitting: how the event is assumed to act.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum InterruptedImpact {
    Level,
    LevelAndSlope,
    Slope,
    /// A level change that ends at `until` (exclusive, 0-based row).
    TemporaryLevel { until: usize },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum InterruptedSeasonal {
    None,
    /// `pairs` sine and cosine pairs at `period` rows.
    Harmonic { pairs: usize, period: f64 },
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub(crate) enum InterruptedSeasonalEvidence {
    None,
    /// With the fit and counterfactual at one fixed phase per row: the deseasonalised trend.
    Harmonic { pairs: usize, period: f64, deseasonalised: Vec<InterruptedDeseasonalisedEvidence> },
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InterruptedDeseasonalisedEvidence {
    pub(crate) fitted: f64,
    pub(crate) counterfactual: f64,
}

#[derive(Clone, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InterruptedTermEvidence {
    pub(crate) name: String,
    pub(crate) coefficient: f64,
    pub(crate) standard_error: f64,
    pub(crate) p_value: f64,
    /// Normal 95% limits; exponentiated they are the rate ratio's limits of a count model.
    pub(crate) interval: [f64; 2],
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InterruptedRowEvidence {
    pub(crate) observed: f64,
    pub(crate) fitted: f64,
    /// The fit with the event's terms at zero.
    pub(crate) counterfactual: f64,
    /// The ordinary residual of a continuous fit; the deviance residual of a count.
    pub(crate) residual: f64,
}

#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LjungBoxEvidence {
    pub(crate) statistic: f64,
    pub(crate) p_value: f64,
}

/// A correlation at one lag with the positive 95% limit drawn around zero.
#[derive(Clone, Copy, serde::Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CorrelationEvidence {
    pub(crate) value: f64,
    pub(crate) limit: f64,
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
pub(crate) enum BootstrapUncertainty {
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
pub(crate) enum BootstrapUncertaintyEvidence {
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
    SurrogatePath { evidence: crate::surrogate::PathEvidence },
    SurrogateDiagnostics { evidence: crate::surrogate::DiagnosticEvidence },
    Surrogate { evidence: crate::surrogate::Evidence },
    CountRegression { evidence: crate::count_regression::Evidence },
    StaggeredDid { evidence: crate::staggered_did::Evidence },
    Bacon { evidence: crate::bacon::Evidence },
    PanelRegression { evidence: crate::panel_regression::Evidence },
    SunAbraham { evidence: crate::remix_extensions::SunEvidence },
    RidgeAugmentedSynthetic { evidence: crate::remix_extensions::RidgeEvidence },
    RootCause { evidence: crate::root_cause::Evidence },
    GcmEffects { evidence: crate::gcm_effects::Evidence },
    GcmInfluence { evidence: crate::gcm_influence::Evidence },
    RootCauseChecks { evidence: crate::root_cause_checks::Evidence },
    ArdlModel { evidence: crate::ardl_model::Evidence },
    HonestDid { evidence: crate::honest_did::Evidence },
    DiscreteStateRefused {
        query: DiscreteStateQuery,
        node: usize,
        name: String,
        problem: DiscreteStateProblemEvidence,
    },
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
    FlexSurv {
        observations: usize,
        events: usize,
        family: SurvivalFamily,
        natural_baseline: Vec<f64>,
        coefficients: Vec<f64>,
        parameter_intervals: Vec<Option<[f64; 2]>>,
        log_likelihood: f64,
        aic: f64,
        bic: f64,
        profile: Vec<f64>,
        prediction_times: Vec<f64>,
        survival: Vec<f64>,
        hazard: Vec<f64>,
        median: f64,
        mean: Option<f64>,
    },
    CoxRegression {
        observations: usize,
        events: usize,
        total_weight: f64,
        observation: CoxObservationEvidence,
        standard_errors: CoxStandardErrorsEvidence,
        coefficients: Vec<CoxCoefficientEvidence>,
        covariance: Vec<f64>,
        log_likelihood: f64,
        null_log_likelihood: f64,
        likelihood_ratio: f64,
        likelihood_ratio_p_value: f64,
        partial_aic: f64,
        iterations: usize,
        covariate_means: Vec<f64>,
        covariate_standard_deviations: Vec<f64>,
        baseline: CoxBaselineEvidence,
        concordance: SurvivalSummary<f64>,
        proportional_hazards_tests: CoxProportionalHazardsEvidence,
        frailty: CoxFrailtyEvidence,
        fitting: CoxFittingEvidence,
    },
    PenalizedAft {
        observations: usize,
        events: usize,
        family: AftFamilyEvidence,
        penalizer: f64,
        coefficients: Vec<AftCoefficientEvidence>,
        intercept: AftCoefficientEvidence,
        ancillary: AftCoefficientEvidence,
        covariance: Vec<f64>,
        log_likelihood: f64,
        aic: f64,
        bic: f64,
        iterations: usize,
        concordance: SurvivalSummary<f64>,
        covariate_means: Vec<f64>,
        prediction_times: Vec<f64>,
        survival: Vec<f64>,
        median: f64,
    },
    Aalen {
        observations: usize,
        events: usize,
        fitted_events: usize,
        last_time: f64,
        coefficients: Vec<[f64; 5]>,
        curves: Vec<Vec<[f64; 4]>>,
        chisq: f64,
        degrees_of_freedom: usize,
        p_value: f64,
    },
    SurvivalForest {
        observations: usize,
        events: usize,
        trees: usize,
        importance: Vec<SurvivalSummary<f64>>,
        concordance: SurvivalSummary<f64>,
        prediction_row: usize,
        profile: Vec<f64>,
        prediction_times: Vec<f64>,
        survival: Vec<f64>,
        cumulative_hazard: Vec<f64>,
    },
    NonparametricSurvival {
        observations: usize,
        events: usize,
        prediction_times: Vec<f64>,
        survival: Vec<f64>,
        survival_lower: Vec<f64>,
        survival_upper: Vec<f64>,
        cumulative_density: Vec<f64>,
        cumulative_hazard: Vec<f64>,
        cumulative_hazard_lower: Vec<f64>,
        cumulative_hazard_upper: Vec<f64>,
        hazard_increment: Vec<f64>,
    },
    ComparisonSurvival {
        observations: usize,
        truncation_time: f64,
        group_zero_curve: Vec<[f64; 2]>,
        group_one_curve: Vec<[f64; 2]>,
        diagnostics: ComparisonSurvivalDiagnostics,
        observed_conversion: SurvivalSummary<ObservedConversionEvidence>,
        fixed_time_conversion: SurvivalSummary<FixedTimeConversionEvidence>,
        peto_peto: SurvivalSummary<GRhoEvidence>,
        proportional_hazards_p_value: f64,
        log_rank_p_value: f64,
        gehan_wilcoxon_p_value: f64,
        tarone_ware_p_value: f64,
        weighted_kaplan_meier_p_value: f64,
        absolute_difference_p_value: f64,
        two_stage_p_value: f64,
        squared_difference_p_value: f64,
        restricted_mean_difference: f64,
        restricted_mean_interval: [f64; 2],
    },
    MultiStateSurvival {
        observations: usize,
        states: Vec<f64>,
        transitions: Vec<[usize; 2]>,
        family: SurvivalFamily,
        preparation: MultiStatePreparationEvidence,
        prediction_times: Vec<f64>,
        /// One row-major state-by-state probability matrix per prediction time.
        probabilities: Vec<Vec<f64>>,
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
    Jpcmciplus {
        observations: usize,
        datasets: usize,
        periods: usize,
        observed_variables: usize,
        variables: usize,
        classes: Vec<&'static str>,
        time_dummy: bool,
        space_dummy: bool,
        tau_max: usize,
        pc_alpha: f64,
        graph: Vec<Vec<Vec<String>>>,
        p_matrix: Vec<Vec<Vec<f64>>>,
        val_matrix: Vec<Vec<Vec<f64>>>,
        separating_sets: Vec<JpcmciSeparatingSetEvidence>,
        ambiguous_triples: Vec<JpcmciAmbiguousTripleEvidence>,
        lagged_parents: Vec<Vec<JpcmciNodeEvidence>>,
        context_parents: Vec<Vec<JpcmciNodeEvidence>>,
        dummy_parents: Vec<Vec<JpcmciNodeEvidence>>,
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
    PcStable {
        observations: usize,
        variables: usize,
        alpha: f64,
        max_depth: Option<usize>,
        ci_test: ConstraintCiTest,
        graph: Vec<Vec<Vec<String>>>,
        separating_sets: Vec<ConstraintSeparatingSetEvidence>,
        ci_tests: Vec<ConstraintCiEvidence>,
    },
    Fci {
        observations: usize,
        variables: usize,
        alpha: f64,
        max_depth: Option<usize>,
        max_path_length: Option<usize>,
        ci_test: ConstraintCiTest,
        graph: Vec<Vec<Vec<String>>>,
        separating_sets: Vec<ConstraintSeparatingSetEvidence>,
        ci_tests: Vec<ConstraintCiEvidence>,
        edge_properties: Vec<FciEdgePropertyEvidence>,
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
        instruments: InstrumentSetEvidence,
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
        error_model: LinearErrorEvidence,
        fixed_effects: FixedEffectsEvidence,
    },
    PropensityWeighting {
        #[serde(default)]
        target: PropensityTarget,
        observations: usize,
        treatment_model: TreatmentModelEvidence,
        treated_rows: usize,
        control_rows: usize,
        estimate: f64,
        treated_mean: f64,
        control_mean: f64,
        /// The weights each arm carries in total, which is how overlap shows itself.
        treated_weight_sum: f64,
        control_weight_sum: f64,
        /// P(treated | design), in row order, so the panel can draw the score by arm.
        propensity: Vec<f64>,
        /// Which arm each row is in, paired with the scores above.
        treated: Vec<bool>,
        /// The weight each row carries, in row order, for the weighted distribution.
        weights: Vec<f64>,
        outcome: Vec<f64>,
        interval: Option<PropensityIntervalEvidence>,
    },
    PropensityGridSlice {
        scores: Vec<GridCandidateScore>,
    },
    PropensityMatching {
        #[serde(default)]
        target: PropensityTarget,
        observations: usize,
        treatment_model: TreatmentModelEvidence,
        treated_rows: usize,
        control_rows: usize,
        estimate: f64,
        propensity: Vec<f64>,
        treated: Vec<bool>,
        /// The opposite-arm outcome matched to each row, which the chapter prints.
        matches: Vec<f64>,
    },
    DoublyRobust {
        #[serde(default)]
        target: PropensityTarget,
        observations: usize,
        /// The treatment model's parameter count: the constant plus one per design column.
        parameters: usize,
        estimate: f64,
        /// The two halves of the estimator, so a reader can see which arm moves it.
        treated_term: f64,
        control_term: f64,
        propensity: Vec<f64>,
        treated: Vec<bool>,
        converged: bool,
        interval: Option<PropensityIntervalEvidence>,
    },
    ContinuousGps {
        observations: usize,
        estimate: f64,
        intercept: f64,
        standard_error: f64,
        /// Stabilized weights sum to about the sample size; inverse-density weights need not.
        weight_sum: f64,
        /// The weight each row carries in the outcome regression, in row order.
        weights: Vec<f64>,
        /// The conditional density at each observed treatment, in row order.
        density: Vec<f64>,
        residual_scale: f64,
        treatment_params: Vec<f64>,
        interval: Option<PropensityIntervalEvidence>,
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
        uncertainty: BootstrapUncertaintyEvidence,
    },
    InstrumentalVariable {
        observations: usize,
        treatment: usize,
        outcome: usize,
        instruments: Vec<usize>,
        route: InstrumentalVariableRoute,
        estimate: f64,
        params: Vec<f64>,
        standard_error: Option<f64>,
        uncertainty: BootstrapUncertaintyEvidence,
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
    InterruptedSeries {
        observations: usize,
        outcome: usize,
        model: InterruptedModelEvidence,
        intervention_row: usize,
        lag: usize,
        impact: InterruptedImpact,
        seasonal: InterruptedSeasonalEvidence,
        /// One term per design column: const, time, step, slope_change, sin1.., cos1..; absent shapes leave their column out.
        terms: Vec<InterruptedTermEvidence>,
        /// One row per observation on the scale of the plotted series: itself for a continuous
        /// series, the count standardised to the mean exposure for a count.
        path: Vec<InterruptedRowEvidence>,
        ljung_box: Vec<LjungBoxEvidence>,
        residual_acf: Vec<CorrelationEvidence>,
        residual_pacf: Vec<CorrelationEvidence>,
        converged: bool,
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
        post_end: usize,
        outcome: usize,
        controls: Vec<usize>,
        pre_intervention_path: PreInterventionPath,
        counterfactual: Vec<f64>,
        counterfactual_se: Vec<f64>,
        pointwise: Vec<f64>,
        cumulative: f64,
        average: f64,
        params: Vec<f64>,
        log_likelihood: f64,
    },
    StructuralCausalImpact {
        model: StructuralModel,
        contributions: Vec<StructuralContribution>,
        #[serde(flatten)]
        posterior: ImpactPosterior,
    },
    BayesianCausalImpact {
        observations: usize,
        control_inclusion: Vec<ControlInclusion>,
        n_pre: usize,
        n_post: usize,
        post_end: usize,
        outcome: usize,
        controls: Vec<usize>,
        draws: usize,
        warmup: usize,
        seed: u32,
        prior_level_sd: f64,
        level: f64,
        pre_intervention_path: PreInterventionPath,
        counterfactual: Vec<f64>,
        counterfactual_se: Vec<f64>,
        counterfactual_lower: Vec<f64>,
        counterfactual_upper: Vec<f64>,
        pointwise: Vec<f64>,
        pointwise_lower: Vec<f64>,
        pointwise_upper: Vec<f64>,
        cumulative_lower: Vec<f64>,
        cumulative_upper: Vec<f64>,
        cumulative: f64,
        average: f64,
        average_summary: ImpactSummary,
        cumulative_summary: ImpactSummary,
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
        groups: DmlGroupEvidence,
    },
    CausalForest { evidence: crate::causal_forest::Evidence },
    TLearner {
        observations: usize,
        control_rows: usize,
        treated_rows: usize,
        seed: u32,
        trees: usize,
        min_leaf: usize,
        /// One effect per row, in row order.
        effects: Vec<f64>,
        /// The mean of the row effects, EconML's `ate`.
        average: f64,
        uncertainty: TLearnerUncertaintyEvidence,
    },
    CrossFittedTLearner {
        observations: usize,
        control_rows: usize,
        treated_rows: usize,
        seed: u32,
        splits: usize,
        /// Candidates in the grid; every one of the four searches scored all of them.
        candidates: usize,
        /// Each arm's candidate, chosen on all of that arm's rows.
        selected: ArmChoices,
        /// One effect per row, in row order.
        effects: Vec<f64>,
        /// The mean of the effects in the order the halves are concatenated, as numpy takes it.
        average: f64,
    },
    ArdlPss {
        observations: usize,
        long_run: ArdlLongRun,
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
    PredictorSyntheticControl { evidence: crate::predictor_synthetic_control::Evidence },
    SharpRd {
        cutoff: f64,
        target: String,
        assignment: String,
        kernel: String,
        bandwidth_selection: String,
        polynomial_order: usize,
        bias_order: usize,
        nearest_neighbors: usize,
        bandwidth: f64,
        bias_bandwidth: f64,
        conventional: RdEstimateEvidence,
        bias_corrected: RdEstimateEvidence,
        robust: RdEstimateEvidence,
        observations: [usize; 2],
        effective_observations: [usize; 2],
        left_coefficients: Vec<f64>,
        right_coefficients: Vec<f64>,
        points: Vec<[f64; 2]>,
    },
    PanelAdjusted {
        observations: usize, units: Vec<String>, times: Vec<i64>, control_units: usize, treated_units: usize,
        n_pre: usize, n_post: usize, covariates: usize, specification: AdjustedDidSpecification,
        estimate: f64, standard_error: f64, interval: [f64; 2], group_means: [[f64; 2]; 2], inference: AdjustedDidInference,
    },
    PanelDid {
        observations: usize,
        units: Vec<String>,
        times: Vec<i64>,
        control_units: usize,
        treated_units: usize,
        n_pre: usize,
        n_post: usize,
        did: PanelMethodEvidence,
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
    NetworkQuery {
        observations: usize,
        states: Vec<Vec<(String, f64)>>,
        distribution: Vec<(Vec<String>, f64)>,
    },
    ConditionalGaussianQuery { observations:usize, outcome:usize, mean:f64, std:f64, configuration_rows:usize },
    DiscreteBnQuery {
        observations: usize,
        bins: usize,
        equivalent_sample_size: f64,
        /// State count per node, in node order.
        state_counts: Vec<usize>,
        state_preparations: Vec<DiscreteStatePreparationEvidence>,
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
        state_preparations: Vec<DiscreteStatePreparationEvidence>,
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
        forecast: VecmForecast,
        long_run: VecmLongRun,
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

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum MultiStatePreparationEvidence {
    PreparedRows,
    LongitudinalStates {
        source_rows: usize,
        transition_rows: usize,
        notices: Vec<String>,
    },
    WideEvents {
        source_rows: usize,
        transition_rows: usize,
        notices: Vec<String>,
    },
}

#[derive(Clone, Copy, Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum DmlModel {
    Plr,
    Irm,
}

/// How the effect modifier's column is cut into the groups DoubleML's `gate` receives.
#[derive(Clone, Copy, serde::Deserialize, PartialEq, Debug)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DmlGroups {
    None,
    /// One group per distinct value of the column.
    Levels {
        column: usize,
    },
    /// `bins` quantile groups of the column, right-inclusive as `pandas.qcut` cuts them.
    Quantiles {
        column: usize,
        bins: usize,
    },
}

#[derive(Clone, Copy, Serialize, PartialEq, Debug)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DmlGroupingEvidence {
    Levels,
    Quantiles { bins: usize },
}

#[derive(Clone, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DmlGroupEffectEvidence {
    /// The group's bounds on the modifier: equal for a level, open at the outer quantile edges.
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub observations: usize,
    pub effect: f64,
    pub standard_error: f64,
    pub interval: (f64, f64),
    pub few_observations: bool,
}

#[derive(Clone, Serialize, PartialEq, Debug)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(crate) enum DmlGroupEvidence {
    None,
    Grouped {
        modifier: usize,
        grouping: DmlGroupingEvidence,
        level: f64,
        groups: Vec<DmlGroupEffectEvidence>,
    },
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
