import { z } from 'zod'
import { sameStructuralModel, structuralModelSchema, structuralImpactSettingsSchema, structuralContributionSchema } from './structuralImpact'
import type { SharpRdConfiguration, SharpRdEvidence } from './sharpRd'
import { SHARP_RD_METHOD_ID } from './methods'
import { matchesTLearnerUncertainty, tLearnerUncertaintyEvidenceSchema } from './tLearner'
import { vecmForecastSchema } from './vecmForecast'
import { ardlLongRunSchema, vecmLongRunSchema } from './longRun'
import type { ColumnId } from './dataset'
import { armaErrorFieldsSchema } from './interruptedSeries'
import type { DagDocument, EditableDag } from './dag'
import { assertNever, brand, err, flattenNonEmpty, mapNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { CaveatEvaluation, MethodCaveat, MethodDefinition, MethodEligibility, MethodId } from './methods'
import {
  BACKDOOR_LINEAR_REGRESSION_METHOD_ID,
  FRONTDOOR_TWO_STAGE_METHOD_ID,
  INSTRUMENTAL_VARIABLE_METHOD_ID,
  CAUSAL_EFFECTS_TOTAL_METHOD_ID,
  CAUSAL_IMPACT_METHOD_ID,
  ARDL_PSS_METHOD_ID,
  DISCRETE_BN_METHOD_ID,
  BINARY_ETT_METHOD_ID,
  DML_IRM_METHOD_ID,
  DML_PLR_METHOD_ID,
  T_LEARNER_METHOD_ID,
  BAYESIAN_GAUSSIAN_METHOD_ID,
  NEGBIN_NUTS_METHOD_ID,
  SYNTHETIC_CONTROL_METHOD_ID,
  VECM_METHOD_ID,
  NEGATIVE_BINOMIAL_METHOD_ID,
  NEGATIVE_BINOMIAL_INGARCH_METHOD_ID,
  POISSON_GLM_METHOD_ID,
  PANEL_INTERVENTION_METHOD_ID,
} from './methods'
import { describeSeriesTransform, seriesTransformFor, type PreparedDatasetArtifact, type PreparedDatasetVersionId, type StationarityEvidenceArtifact } from './preprocessing'
import { describePanelInterventionPreflight, type PanelInterventionPreflight } from './panel'
import { describeStationarityConflict, levelModelVerdict, type LevelModelVerdict, type StationarityAssessment } from './stationarityAssessment'
import { describeGrouping, identifiedInstruments, treatmentDescendants, type Estimand, type Identification, type IdentificationArtifact, type IdentificationId, type ModifierGrouping, type StudyId, type StudySpecification, type StudyVariable } from './study'
import { formatStatistic } from '@/lib/format/number'

/**
 * An estimate carries its meaning in the type: the estimand it answers, the scale, a named interval
 * or an explicit absence, the sample it used, and the diagnostics the method reports. A bare number
 * never leaves this module. Every kernel behind these types is a transpiled port with a parity fixture.
 */

export type EstimationRunId = Brand<string, 'EstimationRunId'>

/**
 * How the adjusted regression's errors are treated: independent (the classical interval),
 * serially correlated with a Newey–West interval, or an ARMA(p, q) process fitted jointly with
 * the coefficients by maximum likelihood, whose coefficient and interval then replace the OLS ones.
 */
export type LinearErrors =
  | { readonly kind: 'classical' }
  | { readonly kind: 'hac' }
  | { readonly kind: 'arma'; readonly p: number; readonly q: number; readonly maxIter: number }
export type CovarianceChoice = LinearErrors['kind']

export const CONFIDENCE_LEVEL = 0.95

export interface BackdoorLinearConfiguration {
  readonly kind: 'backdoor-linear-regression'
  readonly errors: LinearErrors
  readonly level: typeof CONFIDENCE_LEVEL
}

export interface FrontdoorTwoStageConfiguration {
  readonly kind: 'frontdoor-two-stage'
  readonly interventions: readonly [number, number]
  readonly simulations: number
  readonly sampleSizeFraction: number
  readonly level: typeof CONFIDENCE_LEVEL
  readonly seed: number
}

/** DoWhy's estimator ignores the requested intervention values and always reports the 0 → 1 contrast. */
export interface InstrumentalVariableConfiguration {
  readonly kind: 'instrumental-variable'
  readonly simulations: number
  readonly sampleSizeFraction: number
  readonly level: typeof CONFIDENCE_LEVEL
  readonly seed: number
}

export interface CountGlmConfiguration {
  readonly kind: 'poisson-glm' | 'negative-binomial-p'
}

export type IngarchInterventionSchedule =
  | { readonly kind: 'point' }
  | { readonly kind: 'persistent' }
  | { readonly kind: 'decaying'; readonly delta: number }

export interface NegativeBinomialIngarchConfiguration {
  readonly kind: 'negative-binomial-ingarch'
  readonly link: 'identity' | 'log'
  readonly pastObservationLags: NonEmptyArray<number>
  readonly pastMeanLags: NonEmptyArray<number>
  readonly horizon: number
  readonly controlValue: number
  readonly treatmentValue: number
  readonly schedule: IngarchInterventionSchedule
}

export const causalEffectsNodeSchema = z.tuple([z.number().int().nonnegative(), z.number().int().max(0)])
export type CausalEffectsNode = z.infer<typeof causalEffectsNodeSchema>

export const causalEffectsAdjustmentSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('optimal') }).strict(),
  z.object({ kind: z.literal('minimizedOptimal') }).strict(),
  z.object({ kind: z.literal('collidersMinimizedOptimal') }).strict(),
  z.object({ kind: z.literal('explicit'), nodes: z.array(causalEffectsNodeSchema) }).strict(),
])
export type CausalEffectsAdjustment = z.infer<typeof causalEffectsAdjustmentSchema>

export const totalEffectEstimatorSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('linear'), adjustment: causalEffectsAdjustmentSchema }).strict(),
  z.object({ kind: z.literal('knn'), k: z.number().int().min(1).max(100), adjustment: causalEffectsAdjustmentSchema }).strict(),
  z.object({ kind: z.literal('wrightParents') }).strict(),
])
export type TotalEffectEstimator = z.infer<typeof totalEffectEstimatorSchema>

export const causalEffectsBlockLengthSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('fixed'), length: z.number().int().positive() }).strict(),
  z.object({ kind: z.literal('cubeRoot') }).strict(),
])
export type CausalEffectsBlockLength = z.infer<typeof causalEffectsBlockLengthSchema>

export const causalEffectsUncertaintySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('bootstrap'),
    samples: z.number().int().positive(),
    blockLength: causalEffectsBlockLengthSchema,
    confidenceLevel: z.number().gt(0).lt(1),
    seed: z.number().int().nonnegative(),
  }).strict(),
])
export type CausalEffectsUncertainty = z.infer<typeof causalEffectsUncertaintySchema>

export interface CausalEffectsConfiguration {
  readonly kind: 'causal-effects-total'
  readonly estimator: TotalEffectEstimator
  /** Lag of the treatment node relative to the outcome at time t; 0 means the same period. */
  readonly treatmentLag: number
  /** The two treatment values whose predicted outcomes are differenced. */
  readonly interventions: readonly [number, number]
  readonly uncertainty: CausalEffectsUncertainty
}

export type InterventionStart =
  | { readonly kind: 'from-treatment' }
  | { readonly kind: 'row'; readonly row: number }

/** Which post-intervention rows the effect is summarised over. The fit is unaffected. */
export type EvaluationWindow =
  | { readonly kind: 'through-last-row' }
  | { readonly kind: 'to-row'; readonly row: number }

/** The exclusive end of the evaluated window, as a row count from the start of the series. */
export const evaluatedEnd = (window: EvaluationWindow, observations: number): number => {
  switch (window.kind) {
    case 'through-last-row': return observations
    case 'to-row': return Math.min(window.row, observations)
    default: return assertNever(window)
  }
}

export const describeEvaluationWindow = (window: EvaluationWindow): string =>
  window.kind === 'through-last-row' ? 'through the last row' : `through row ${window.row}`

const localLevelImpactSettingsSchema = z.object({
  kind: z.literal('bayesian'),
  draws: z.number().int().min(2),
  warmup: z.number().int().nonnegative(),
  seed: z.number().int().min(0).max(0xffffffff),
  priorLevelSd: z.number().finite().positive(),
}).strict()
export const bayesianImpactSettingsSchema = z.discriminatedUnion('kind', [localLevelImpactSettingsSchema, structuralImpactSettingsSchema])
export type BayesianImpactSettings = z.infer<typeof bayesianImpactSettingsSchema>

export type CausalImpactConfiguration = {
  readonly kind: 'causal-impact'
  readonly start: InterventionStart
  readonly window: EvaluationWindow
  readonly controls: readonly ColumnId[]
} & (
  | { readonly inference?: undefined; readonly maxIter: number }
  | { readonly inference: BayesianImpactSettings }
)

export interface DoubleMlConfiguration {
  readonly kind: 'dml-plr' | 'dml-irm'
  /** IRM only: the effect on the treated instead of the average effect. */
  readonly att: boolean
  /** Seeds the shuffled folds; recorded so the refutation batch can repeat the fit. */
  readonly seed: number
}

export interface TLearnerConfiguration {
  readonly kind: 't-learner'
  /** One seed for both outcome forests. */
  readonly seed: number
  readonly uncertainty: import('./tLearner').TLearnerUncertainty
}

export interface ArdlConfiguration {
  readonly kind: 'ardl-pss'
  readonly maxLag: number
  readonly trend: 'c' | 'ct'
  /** Pesaran–Shin–Smith case: 2 or 3 with a constant, 4 or 5 with a trend. */
  readonly case: 2 | 3 | 4 | 5
}

export type VecmDeterministic = 'n' | 'co' | 'ci' | 'coli'

export interface VecmConfiguration {
  readonly kind: 'vecm'
  readonly maxLags: number
  readonly deterministic: VecmDeterministic
  /** Johansen trace significance. */
  readonly significance: 90 | 95 | 99
  /** Optional Chow break index between the outcome and the treatment. */
  readonly breakIndex: number | null
}

export interface SyntheticControlConfiguration {
  readonly kind: 'synthetic-control'
  readonly start: InterventionStart
  readonly donors: readonly ColumnId[]
  readonly crossFitFolds: number
  readonly alpha: number
}

export type PanelInterventionConfiguration = {
  readonly kind: 'panel-intervention'
  readonly primary: 'adjusted'
  readonly covariates: readonly ColumnId[]
  readonly specification: AdjustedDidSpecification
} | {
  readonly kind: 'panel-intervention'
  /** Absent in older saved runs, whose primary result is synthetic DiD. */
  readonly primary?: 'did' | 'syntheticDid'
  readonly placeboReplications: number
  readonly seed: number
}

export interface NegbinNutsConfiguration {
  readonly kind: 'negbin-nuts'
  readonly warmup: number
  readonly samples: number
  readonly seed: number
}

export interface BayesianGaussianConfiguration {
  readonly kind: 'bayesian-gaussian'
  readonly warmup: number
  readonly samples: number
  readonly seed: number
}

export interface DiscreteBnConfiguration {
  readonly kind: 'discrete-bn-query'
  readonly bins: number
  readonly equivalentSampleSize: number
}

export interface BinaryEttConfiguration {
  readonly kind: 'binary-ett-idc-star'
}

export type EstimatorConfiguration =
  | SharpRdConfiguration
  | BackdoorLinearConfiguration
  | FrontdoorTwoStageConfiguration
  | InstrumentalVariableConfiguration
  | CountGlmConfiguration
  | NegativeBinomialIngarchConfiguration
  | DoubleMlConfiguration
  | TLearnerConfiguration
  | ArdlConfiguration
  | VecmConfiguration
  | SyntheticControlConfiguration
  | NegbinNutsConfiguration
  | BayesianGaussianConfiguration
  | DiscreteBnConfiguration
  | BinaryEttConfiguration
  | CausalEffectsConfiguration
  | CausalImpactConfiguration
  | PanelInterventionConfiguration

export type EstimatorId = EstimatorConfiguration['kind']

export type EstimatorGroupId = 'adjusted-outcome' | 'identified-functional' | 'graph-adjusted-temporal' | 'dynamic-time-series' | 'intervention-comparison'

export interface EstimatorGroup {
  readonly id: EstimatorGroupId
  readonly name: string
  readonly description: string
  readonly estimators: NonEmptyArray<EstimatorId>
}

export const ESTIMATOR_GROUPS: NonEmptyArray<EstimatorGroup> = [
  {
    id: 'adjusted-outcome',
    name: 'Covariate-adjusted outcome models',
    description: 'Regression, count-model and orthogonal-score estimators using an identified adjustment set.',
    estimators: ['backdoor-linear-regression', 'bayesian-gaussian', 'poisson-glm', 'negative-binomial-p', 'negbin-nuts', 'dml-plr', 'dml-irm', 't-learner'],
  },
  {
    id: 'identified-functional',
    name: 'Identified-function estimators',
    description: 'Methods tied to a front-door, instrument, interventional-distribution, or counterfactual identification result.',
    estimators: ['frontdoor-two-stage', 'instrumental-variable', 'discrete-bn-query', 'binary-ett-idc-star'],
  },
  {
    id: 'graph-adjusted-temporal',
    name: 'Graph-adjusted temporal effects',
    description: 'Total-effect estimation using a time-indexed causal graph and lag-resolved adjustment set.',
    estimators: ['causal-effects-total'],
  },
  {
    id: 'dynamic-time-series',
    name: 'Count time-series interventions',
    description: 'Compare count paths under specified treatment schedules.',
    estimators: ['negative-binomial-ingarch'],
  },
  {
    id: 'intervention-comparison',
    name: 'Intervention and comparative designs',
    description: 'Post-intervention comparisons using a forecast counterfactual or untreated comparison units.',
    estimators: ['causal-impact', 'synthetic-control', 'panel-intervention', 'sharp-rd'],
  },
]

export const ESTIMATOR_IDS: NonEmptyArray<EstimatorId> = flattenNonEmpty(mapNonEmpty(ESTIMATOR_GROUPS, (group) => group.estimators))

export const methodIdOf = (estimator: EstimatorId): MethodId => {
  switch (estimator) {
    case 'backdoor-linear-regression': return BACKDOOR_LINEAR_REGRESSION_METHOD_ID
    case 'frontdoor-two-stage': return FRONTDOOR_TWO_STAGE_METHOD_ID
    case 'instrumental-variable': return INSTRUMENTAL_VARIABLE_METHOD_ID
    case 'poisson-glm': return POISSON_GLM_METHOD_ID
    case 'negative-binomial-p': return NEGATIVE_BINOMIAL_METHOD_ID
    case 'negative-binomial-ingarch': return NEGATIVE_BINOMIAL_INGARCH_METHOD_ID
    case 'dml-plr': return DML_PLR_METHOD_ID
    case 'dml-irm': return DML_IRM_METHOD_ID
    case 't-learner': return T_LEARNER_METHOD_ID
    case 'ardl-pss': return ARDL_PSS_METHOD_ID
    case 'vecm': return VECM_METHOD_ID
    case 'synthetic-control': return SYNTHETIC_CONTROL_METHOD_ID
    case 'sharp-rd': return SHARP_RD_METHOD_ID
    case 'panel-intervention': return PANEL_INTERVENTION_METHOD_ID
    case 'negbin-nuts': return NEGBIN_NUTS_METHOD_ID
    case 'bayesian-gaussian': return BAYESIAN_GAUSSIAN_METHOD_ID
    case 'discrete-bn-query': return DISCRETE_BN_METHOD_ID
    case 'binary-ett-idc-star': return BINARY_ETT_METHOD_ID
    case 'causal-effects-total': return CAUSAL_EFFECTS_TOTAL_METHOD_ID
    case 'causal-impact': return CAUSAL_IMPACT_METHOD_ID
    default: return assertNever(estimator)
  }
}

export const defaultConfiguration = (estimator: EstimatorId, prepared: PreparedDatasetArtifact, study: StudySpecification | null): EstimatorConfiguration => {
  switch (estimator) {
    case 'backdoor-linear-regression': return { kind: estimator, errors: { kind: prepared.kind === 'prepared-time-series' ? 'hac' : 'classical' }, level: CONFIDENCE_LEVEL }
    case 'frontdoor-two-stage': return { kind: estimator, interventions: [0, 1], simulations: 399, sampleSizeFraction: 1, level: CONFIDENCE_LEVEL, seed: 0 }
    case 'instrumental-variable': return { kind: estimator, simulations: 399, sampleSizeFraction: 1, level: CONFIDENCE_LEVEL, seed: 0 }
    case 'poisson-glm':
    case 'negative-binomial-p': return { kind: estimator }
    case 'negative-binomial-ingarch': return { kind: estimator, link: 'identity', pastObservationLags: [1], pastMeanLags: [1], horizon: 12, controlValue: 0, treatmentValue: 1, schedule: { kind: 'persistent' } }
    case 'dml-plr': return { kind: estimator, att: false, seed: 7 }
    case 'dml-irm': return { kind: estimator, att: study?.estimand.kind === 'average-treatment-effect-on-treated', seed: 7 }
    case 't-learner': return { kind: estimator, seed: 7, uncertainty: { kind: 'none' } }
    case 'ardl-pss': return { kind: estimator, maxLag: 4, trend: 'ct', case: 4 }
    case 'vecm': return { kind: estimator, maxLags: 4, deterministic: 'co', significance: 95, breakIndex: null }
    case 'synthetic-control': {
      const affected = affectedColumns(study)
      return {
        kind: estimator,
        start: { kind: 'from-treatment' },
        donors: study === null ? [] : prepared.columns.filter((column) => column !== study.treatment.column && column !== study.outcome.column && !affected.has(column)),
        crossFitFolds: 3,
        alpha: 0.05,
      }
    }
    case 'sharp-rd': return { kind: estimator }
    case 'panel-intervention': return { kind: estimator, placeboReplications: 100, seed: 0 }
    case 'negbin-nuts': return { kind: estimator, warmup: 500, samples: 1000, seed: 0 }
    case 'bayesian-gaussian': return { kind: estimator, warmup: 500, samples: 1000, seed: 41 }
    case 'discrete-bn-query': return { kind: estimator, bins: 3, equivalentSampleSize: 5 }
    case 'binary-ett-idc-star': return { kind: estimator }
    case 'causal-effects-total': return { kind: estimator, estimator: { kind: 'linear', adjustment: { kind: 'optimal' } }, treatmentLag: 0, interventions: [0, 1], uncertainty: { kind: 'bootstrap', samples: 100, blockLength: { kind: 'fixed', length: 1 }, confidenceLevel: 0.9, seed: 4 } }
    case 'causal-impact': {
      // A control the treatment itself moves would absorb the effect, so DAG descendants of the
      // treatment start unticked; columns outside the DAG stay in, as a judgement for the user.
      const affected = affectedColumns(study)
      return {
        kind: estimator,
        start: { kind: 'from-treatment' },
        window: { kind: 'through-last-row' },
        controls: study === null ? [] : prepared.columns.filter((column) => column !== study.treatment.column && column !== study.outcome.column && !affected.has(column)),
        maxIter: 100,
      }
    }
    default: return assertNever(estimator)
  }
}

export const backdoorLinearEvidenceSchema = z.object({
  kind: z.literal('backdoorLinear'),
  observations: z.number().int().positive(),
  parameters: z.number().int().min(2),
  treatment: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  adjustment: z.array(z.number().int().nonnegative()),
  level: z.number().gt(0.5).lt(1),
  estimate: z.number().finite(),
  standardError: z.number().finite().nonnegative(),
  interval: z.tuple([z.number().finite(), z.number().finite()]),
  degreesOfFreedom: z.number().int().positive(),
  residualSd: z.number().finite().nonnegative(),
  rSquared: z.number().finite(),
  hacMaxLags: z.number().int().nonnegative(),
  hacStandardError: z.number().finite().nonnegative(),
  hacInterval: z.tuple([z.number().finite(), z.number().finite()]),
  hacPValue: z.number().min(0).max(1),
  durbinWatson: z.number().finite().nonnegative(),
  /** The same design refitted with an ARMA error process, when one was requested. */
  errorModel: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('neweyWest') }).strict(),
    z.object({
      kind: z.literal('arma'),
      estimate: z.number().finite(),
      standardError: z.number().finite().nonnegative(),
      interval: z.tuple([z.number().finite(), z.number().finite()]),
      pValue: z.number().min(0).max(1),
      errors: armaErrorFieldsSchema,
    }).strict(),
  ]),
}).strict()

export type BackdoorLinearEvidence = z.infer<typeof backdoorLinearEvidenceSchema>
export type ArmaReading = Extract<BackdoorLinearEvidence['errorModel'], { kind: 'arma' }>

/** The ARMA reading a run was configured for, and only then. */
export const armaReading = (run: { readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }): ArmaReading | null =>
  run.configuration.errors.kind === 'arma' && run.evidence.errorModel.kind === 'arma' ? run.evidence.errorModel : null

/** DoWhy's generic bootstrap, shared by the front-door and instrumental-variable estimators. */
const bootstrapUncertaintySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('bootstrap'),
    simulations: z.number().int().positive(),
    sampleSizeFraction: z.number().finite().positive(),
    confidenceLevel: z.number().gt(0).lt(1),
    seed: z.number().int().nonnegative(),
    interval: z.tuple([z.number().finite(), z.number().finite()]),
  }).strict(),
])

export const frontdoorTwoStageEvidenceSchema = z.object({
  kind: z.literal('frontdoorTwoStage'),
  observations: z.number().int().positive(),
  treatment: z.number().int().nonnegative(),
  mediator: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  firstStageAdjustment: z.array(z.number().int().nonnegative()),
  secondStageAdjustment: z.array(z.number().int().nonnegative()),
  controlValue: z.number().finite(),
  treatmentValue: z.number().finite(),
  firstStageParams: z.array(z.number().finite()).min(2),
  secondStageParams: z.array(z.number().finite()).min(2),
  firstStageEffect: z.number().finite(),
  secondStageEffect: z.number().finite(),
  estimate: z.number().finite(),
  uncertainty: bootstrapUncertaintySchema,
}).strict()

export type FrontdoorTwoStageEvidence = z.infer<typeof frontdoorTwoStageEvidenceSchema>

export const INSTRUMENTAL_VARIABLE_ROUTES = ['waldRatio', 'covarianceRatio', 'twoStageLeastSquares'] as const
export type InstrumentalVariableRoute = (typeof INSTRUMENTAL_VARIABLE_ROUTES)[number]

export const instrumentalVariableEvidenceSchema = z.object({
  kind: z.literal('instrumentalVariable'),
  observations: z.number().int().positive(),
  treatment: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  instruments: z.array(z.number().int().nonnegative()).min(1),
  route: z.enum(INSTRUMENTAL_VARIABLE_ROUTES),
  estimate: z.number().finite(),
  params: z.array(z.number().finite()).min(1),
  standardError: z.number().finite().nonnegative().nullable(),
  uncertainty: bootstrapUncertaintySchema,
}).strict()

export type InstrumentalVariableEvidence = z.infer<typeof instrumentalVariableEvidenceSchema>

export function describeInstrumentalVariableRoute(route: InstrumentalVariableRoute): string {
  switch (route) {
    case 'waldRatio': return 'Wald ratio'
    case 'covarianceRatio': return 'Covariance ratio'
    case 'twoStageLeastSquares': return 'Two-stage least squares'
    default: return assertNever(route)
  }
}

/** Rust serialises NaN as null; the count families leave each other's statistics null. */
const nullableNumber = z.number().finite().nullable()

export const countGlmEvidenceSchema = z.object({
  kind: z.literal('countGlm'),
  observations: z.number().int().positive(),
  parameters: z.number().int().min(2),
  treatment: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  adjustment: z.array(z.number().int().nonnegative()),
  family: z.enum(['poisson', 'negativeBinomial']),
  coefficient: z.number().finite(),
  standardError: z.number().finite().nonnegative(),
  pValue: z.number().min(0).max(1),
  incidenceRateRatio: z.number().finite().positive(),
  incidenceRateRatioInterval: z.tuple([z.number().finite().positive(), z.number().finite().positive()]),
  level: z.number().gt(0.5).lt(1),
  deviance: nullableNumber,
  logLikelihood: nullableNumber,
  alpha: nullableNumber,
  degreesOfFreedom: z.number().int().nonnegative(),
  converged: z.boolean(),
  iterations: z.number().int().nonnegative(),
}).strict()

export type CountGlmEvidence = z.infer<typeof countGlmEvidenceSchema>

export const ingarchInterventionScheduleSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('point') }).strict(),
  z.object({ kind: z.literal('persistent') }).strict(),
  z.object({ kind: z.literal('decaying'), delta: z.number().min(0).max(1) }).strict(),
])

export const negativeBinomialIngarchEvidenceSchema = z.object({
  kind: z.literal('negativeBinomialIngarch'),
  observations: z.number().int().positive(),
  outcome: z.number().int().nonnegative(),
  link: z.enum(['identity', 'log']),
  regressors: z.array(z.number().int().nonnegative()).min(1),
  pastObservationLags: z.array(z.number().int().positive()).min(1),
  pastMeanLags: z.array(z.number().int().positive()).min(1),
  externalRegressors: z.array(z.boolean()),
  horizon: z.number().int().positive(),
  interventionRegressor: z.number().int().nonnegative(),
  controlValue: z.number().finite(),
  treatmentValue: z.number().finite(),
  schedule: ingarchInterventionScheduleSchema,
  parameters: z.array(z.number().finite()).min(4),
  fittedMeans: z.array(z.number().finite().positive()).min(1),
  residuals: z.array(z.number().finite()).min(1),
  logLikelihood: z.number().finite(),
  size: z.number().finite().positive(),
  dispersion: z.number().finite().positive(),
  score: z.array(z.number().finite()).min(4),
  iterations: z.number().int().positive(),
  functionEvaluations: z.number().int().positive(),
  gradientEvaluations: z.number().int().positive(),
  baselineMean: z.array(z.number().finite().positive()).min(1),
  interventionMean: z.array(z.number().finite().positive()).min(1),
  effectPath: z.array(z.number().finite()).min(1),
  averageEffect: z.number().finite(),
  cumulativeEffect: z.number().finite(),
}).strict().superRefine((value, context) => {
  if (value.regressors.length !== value.externalRegressors.length) context.addIssue({ code: 'custom', message: 'INGARCH regressors and external flags differ in length.' })
  if (value.fittedMeans.length !== value.observations || value.residuals.length !== value.observations) context.addIssue({ code: 'custom', message: 'INGARCH fitted arrays do not match the observation count.' })
  if (value.baselineMean.length !== value.horizon || value.interventionMean.length !== value.horizon || value.effectPath.length !== value.horizon) context.addIssue({ code: 'custom', message: 'INGARCH forecast arrays do not match the horizon.' })
})

export type NegativeBinomialIngarchEvidence = z.infer<typeof negativeBinomialIngarchEvidenceSchema>

/** One group of the effect modifier with DoubleML's group average treatment effect for it. */
export const dmlGroupEffectSchema = z.object({
  lower: z.number().finite().nullable(),
  upper: z.number().finite().nullable(),
  observations: z.number().int().positive(),
  effect: z.number().finite(),
  standardError: z.number().finite().nonnegative(),
  interval: z.tuple([z.number().finite(), z.number().finite()]),
  fewObservations: z.boolean(),
}).strict()

export const dmlGroupEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('grouped'),
    modifier: z.number().int().nonnegative(),
    grouping: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('levels') }).strict(),
      z.object({ kind: z.literal('quantiles'), bins: z.number().int().min(2).max(10) }).strict(),
    ]),
    level: z.number().gt(0.5).lt(1),
    groups: z.array(dmlGroupEffectSchema).min(1),
  }).strict(),
])

export const doubleMlEvidenceSchema = z.object({
  kind: z.literal('doubleMl'),
  observations: z.number().int().positive(),
  model: z.enum(['plr', 'irm']),
  att: z.boolean(),
  treatBinary: z.boolean(),
  seed: z.number().int().nonnegative(),
  estimate: z.number().finite(),
  standardError: z.number().finite().nonnegative(),
  interval: z.tuple([z.number().finite(), z.number().finite()]),
  level: z.number().gt(0.5).lt(1),
  groups: dmlGroupEvidenceSchema,
}).strict()

export type DoubleMlEvidence = z.infer<typeof doubleMlEvidenceSchema>

export const tLearnerEvidenceSchema = z.object({
  kind: z.literal('tLearner'),
  observations: z.number().int().positive(),
  controlRows: z.number().int().positive(),
  treatedRows: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  trees: z.number().int().positive(),
  minLeaf: z.number().int().positive(),
  /** One effect per prepared row, in row order. */
  effects: z.array(z.number().finite()).min(1),
  /** The mean of the row effects, EconML's `ate`. */
  average: z.number().finite(),
  uncertainty: tLearnerUncertaintyEvidenceSchema.default({ kind: 'none' }),
}).strict().superRefine((value, context) => {
  if (value.controlRows + value.treatedRows !== value.observations || value.effects.length !== value.observations) {
    context.addIssue({ code: 'custom', message: 'Treatment groups and row effects must match the observation count.' })
  }
  const uncertainty = value.uncertainty
  if (uncertainty.kind === 'bootstrap' && (uncertainty.intervals.length !== value.observations || uncertainty.standardErrors.length !== value.observations)) {
    context.addIssue({ code: 'custom', message: 'Each row effect must have one interval and standard error.' })
  }
})

export type TLearnerEvidence = z.infer<typeof tLearnerEvidenceSchema>

export const ardlEvidenceSchema = z.object({
  kind: z.literal('ardlPss'),
  longRun: ardlLongRunSchema.optional(),
  observations: z.number().int().positive(),
  trend: z.enum(['c', 'ct']),
  case: z.number().int().min(2).max(5),
  arLag: z.number().int().positive(),
  dlLag: z.number().int().positive(),
  grid: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative().nullable(), z.number(), z.number(), z.number()])),
  longRunEffect: z.number().finite(),
  pValue: z.number().min(0).max(1),
  interval: z.tuple([z.number().finite(), z.number().finite()]),
  level: z.number().gt(0.5).lt(1),
  boundsStatistic: z.number().finite(),
  boundsCritical: z.array(z.tuple([z.number().finite(), z.number().finite()])).length(4),
  boundsPLower: z.number().min(0).max(1),
  boundsPUpper: z.number().min(0).max(1),
}).strict()

export type ArdlEvidence = z.infer<typeof ardlEvidenceSchema>

/** What the bounds test says, read against the 5% critical bounds. */
export type BoundsReading = 'level-relation' | 'no-level-relation' | 'inconclusive'

export const boundsReading = (evidence: ArdlEvidence): BoundsReading => {
  const [lower, upper] = evidence.boundsCritical[1] ?? [Number.NaN, Number.NaN]
  if (evidence.boundsStatistic > upper) return 'level-relation'
  if (evidence.boundsStatistic < lower) return 'no-level-relation'
  return 'inconclusive'
}

export const vecmEvidenceSchema = z.object({
  kind: z.literal('vecm'),
  forecast: vecmForecastSchema.optional(),
  longRun: vecmLongRunSchema.optional(),
  observations: z.number().int().positive(),
  deterministic: z.enum(['n', 'co', 'ci', 'coli']),
  kArDiff: z.number().int().positive(),
  rank: z.number().int().nonnegative(),
  significance: z.number().int().min(0).max(2),
  longRunEffect: z.number().finite().nullable(),
  alpha: z.array(z.array(z.number())),
  beta: z.array(z.array(z.number())),
  gamma: z.array(z.array(z.number())),
  pvaluesAlpha: z.array(z.array(z.number())),
  chow: z.tuple([z.number(), z.number()]).nullable(),
}).strict()

export type VecmEvidence = z.infer<typeof vecmEvidenceSchema>

export const syntheticControlEvidenceSchema = z.object({
  kind: z.literal('syntheticControl'),
  observations: z.number().int().positive(),
  nPre: z.number().int().min(2),
  nPost: z.number().int().positive(),
  weights: z.array(z.number().min(-1e-9).max(1 + 1e-9)).min(1),
  loss: z.number().finite().nonnegative(),
  iterations: z.number().int().nonnegative(),
  preGap: z.array(z.number().finite()),
  postGap: z.array(z.number().finite()).min(1),
  att: z.number().finite(),
  treated: z.array(z.number().finite()),
  synthetic: z.array(z.number().finite()),
  crossFit: z.discriminatedUnion('kind', [
    z.object({
      kind: z.literal('available'),
      att: z.number().finite(),
      standardError: z.number().finite().nonnegative(),
      tStatistic: z.number().finite(),
      degreesOfFreedom: z.number().int().positive(),
      pValue: z.number().min(0).max(1),
      confidenceInterval: z.tuple([z.number().finite(), z.number().finite()]),
      blockSize: z.number().int().min(2),
      folds: z.array(z.object({
        heldOut: z.array(z.number().int().nonnegative()).min(2),
        weights: z.array(z.number().finite()).min(1),
        bias: z.number().finite(),
        att: z.number().finite(),
      }).strict()).min(2),
    }).strict(),
    z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  ]),
  donorPlacebo: z.discriminatedUnion('kind', [
    z.object({
      kind: z.literal('available'),
      treatedPreMspe: z.number().finite().nonnegative(),
      treatedPostMspe: z.number().finite().nonnegative(),
      treatedMspeRatio: z.number().finite().nonnegative().nullable(),
      placebos: z.array(z.object({
        donor: z.number().int().nonnegative(),
        preMspe: z.number().finite().nonnegative(),
        postMspe: z.number().finite().nonnegative(),
        mspeRatio: z.number().finite().nonnegative().nullable(),
      }).strict()).min(2),
      pValue: z.number().min(0).max(1),
      nValidPlacebos: z.number().int().positive(),
    }).strict(),
    z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  ]),
  conformalBand: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('available'), alpha: z.number().gt(0).lt(1), intervals: z.array(z.tuple([z.number().finite(), z.number().finite()])).min(1), halfWidth: z.number().finite().nonnegative(), preMspe: z.number().finite().nonnegative(), postMspe: z.number().finite().nonnegative(), mspeRatio: z.number().finite().nonnegative().nullable() }).strict(),
    z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  ]),
  gaussianBand: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('available'), alpha: z.number().gt(0).lt(1), intervals: z.array(z.tuple([z.number().finite(), z.number().finite()])).min(1), halfWidth: z.number().finite().nonnegative(), preMspe: z.number().finite().nonnegative(), postMspe: z.number().finite().nonnegative(), mspeRatio: z.number().finite().nonnegative().nullable() }).strict(),
    z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  ]),
}).strict().superRefine((evidence, context) => {
  if (evidence.nPre + evidence.nPost !== evidence.observations) context.addIssue({ code: 'custom', message: 'Synthetic-control pre/post counts do not match the observations.' })
  if (evidence.preGap.length !== evidence.nPre) context.addIssue({ code: 'custom', message: 'Synthetic-control pre-period gaps do not match nPre.' })
  if (evidence.postGap.length !== evidence.nPost) context.addIssue({ code: 'custom', message: 'Synthetic-control post-period gaps do not match nPost.' })
  if (evidence.treated.length !== evidence.observations || evidence.synthetic.length !== evidence.observations) context.addIssue({ code: 'custom', message: 'Synthetic-control paths do not match the observation count.' })
  if (evidence.crossFit.kind === 'available') {
    for (const fold of evidence.crossFit.folds) {
      if (fold.weights.length !== evidence.weights.length) context.addIssue({ code: 'custom', message: 'A cross-fit fold does not contain one weight per donor.' })
    }
  }
  if (evidence.donorPlacebo.kind === 'available') {
    if (evidence.donorPlacebo.placebos.length !== evidence.weights.length) context.addIssue({ code: 'custom', message: 'Donor-placebo evidence does not contain one result per donor.' })
    const donors = evidence.donorPlacebo.placebos.map((placebo) => placebo.donor)
    if (new Set(donors).size !== donors.length || donors.some((donor) => donor >= evidence.weights.length)) context.addIssue({ code: 'custom', message: 'Donor-placebo indices are not a distinct in-range donor set.' })
  }
  for (const band of [evidence.conformalBand, evidence.gaussianBand]) {
    if (band.kind === 'available' && band.intervals.length !== evidence.observations) context.addIssue({ code: 'custom', message: 'A synthetic-control prediction band does not match the observation count.' })
  }
})

export type SyntheticControlEvidence = z.infer<typeof syntheticControlEvidenceSchema>

const panelMethodEvidenceSchema = z.object({
  estimate: z.number().finite(),
  lambda: z.array(z.number().finite()),
  omega: z.array(z.number().finite()).min(1),
  effectCurve: z.array(z.number().finite()).min(1),
  lambdaIterations: z.number().int().nonnegative(),
  omegaIterations: z.number().int().nonnegative(),
  lambdaObjective: z.array(z.number().finite().nonnegative()),
  omegaObjective: z.array(z.number().finite().nonnegative()),
  noiseLevel: z.number().finite().nonnegative(),
}).strict()

const panelPlaceboEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('available'), replications: z.number().int().min(2), seed: z.number().int().nonnegative(), standardError: z.number().finite().nonnegative(), estimates: z.array(z.number().finite()).min(2) }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
])

const panelInTimeEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('available'), estimate: z.number().finite(), effectCurve: z.array(z.number().finite()).min(1) }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
])

const syntheticPanelEvidenceSchema = z.object({
  kind: z.literal('panelIntervention'),
  observations: z.number().int().positive(),
  units: z.array(z.string().min(1)).min(2),
  times: z.array(z.number().int().nonnegative()).min(2),
  controlUnits: z.number().int().positive(),
  treatedUnits: z.number().int().positive(),
  nPre: z.number().int().positive(),
  nPost: z.number().int().positive(),
  did: panelMethodEvidenceSchema,
  syntheticControl: panelMethodEvidenceSchema,
  syntheticDid: panelMethodEvidenceSchema,
  syntheticControlPlacebo: panelPlaceboEvidenceSchema,
  syntheticDidPlacebo: panelPlaceboEvidenceSchema,
  syntheticControlInTime: panelInTimeEvidenceSchema,
  syntheticDidInTime: panelInTimeEvidenceSchema,
}).strict().superRefine((evidence, context) => {
  if (evidence.controlUnits + evidence.treatedUnits !== evidence.units.length) context.addIssue({ code: 'custom', message: 'Panel unit counts do not match the unit labels.' })
  if (evidence.nPre + evidence.nPost !== evidence.times.length) context.addIssue({ code: 'custom', message: 'Panel period counts do not match the time codes.' })
  for (const [name, estimate] of [['DID', evidence.did], ['synthetic control', evidence.syntheticControl], ['synthetic DID', evidence.syntheticDid]] as const) {
    if (estimate.omega.length !== evidence.controlUnits) context.addIssue({ code: 'custom', message: `${name} control weights do not match the controls.` })
    if (estimate.lambda.length !== evidence.nPre) context.addIssue({ code: 'custom', message: `${name} time weights do not match the pre-periods.` })
    if (estimate.effectCurve.length !== evidence.nPost) context.addIssue({ code: 'custom', message: `${name} effect curve does not match the post-periods.` })
  }
  for (const [name, inference] of [['synthetic control', evidence.syntheticControlPlacebo], ['synthetic DID', evidence.syntheticDidPlacebo]] as const) {
    if (inference.kind === 'available' && inference.estimates.length !== inference.replications) context.addIssue({ code: 'custom', message: `${name} placebo estimates do not match the replication count.` })
  }
})

const conventionalPanelEvidenceSchema = z.object({
  kind: z.literal('panelDid'),
  observations: z.number().int().positive(),
  units: z.array(z.string().min(1)).min(2),
  times: z.array(z.number().int().nonnegative()).min(2),
  controlUnits: z.number().int().positive(),
  treatedUnits: z.number().int().positive(),
  nPre: z.number().int().positive(),
  nPost: z.number().int().positive(),
  did: panelMethodEvidenceSchema,
}).strict().superRefine((e, ctx) => {
  if (e.controlUnits + e.treatedUnits !== e.units.length || e.nPre + e.nPost !== e.times.length || e.observations !== e.units.length * e.times.length) ctx.addIssue({ code: 'custom', message: 'DiD panel dimensions do not match.' })
  if (e.did.omega.length !== e.controlUnits || e.did.lambda.length !== e.nPre || e.did.effectCurve.length !== e.nPost) ctx.addIssue({ code: 'custom', message: 'DiD weights or period effects do not match the panel.' })
})

export const panelInterventionEvidenceSchema = z.union([syntheticPanelEvidenceSchema, conventionalPanelEvidenceSchema, adjustedDidEvidenceSchema])
export type PanelInterventionEvidence = z.infer<typeof panelInterventionEvidenceSchema>

export const negbinNutsEvidenceSchema = z.object({
  kind: z.literal('negbinNuts'),
  observations: z.number().int().positive(),
  warmup: z.number().int().positive(),
  samples: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  irrMedian: z.number().finite().positive(),
  irrLower: z.number().finite().positive(),
  irrUpper: z.number().finite().positive(),
  betaTreatmentMean: z.number().finite(),
  betaTreatmentSd: z.number().finite().nonnegative(),
  dispersionMean: z.number().finite().positive(),
  divergences: z.number().int().nonnegative(),
  acceptanceRate: z.number().min(0).max(1),
  meanAcceptProbability: z.number().min(0).max(1),
  stepSize: z.number().finite().positive(),
}).strict()

export type NegbinNutsEvidence = z.infer<typeof negbinNutsEvidenceSchema>

export const bayesianGaussianEvidenceSchema = z.object({
  kind: z.literal('bayesianGaussian'),
  observations: z.number().int().positive(),
  warmup: z.number().int().positive(),
  samples: z.number().int().positive(),
  chains: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  effectMean: z.number().finite(),
  effectSd: z.number().finite().nonnegative(),
  effectMedian: z.number().finite(),
  hdiLower: z.number().finite(),
  hdiUpper: z.number().finite(),
  probabilityPositive: z.number().min(0).max(1),
  sigmaMean: z.number().finite().positive(),
  divergences: z.number().int().nonnegative(),
  acceptanceRate: z.number().min(0).max(1),
  meanAcceptProbability: z.number().min(0).max(1),
  stepSize: z.number().finite().positive(),
  histogramStart: z.number().finite(),
  histogramBinWidth: z.number().finite().positive(),
  histogramCounts: z.array(z.number().int().nonnegative()).min(1),
  curves: z.array(z.object({
    standardised: z.boolean(),
    grid: z.array(z.number().finite()).min(2),
    controlLower: z.array(z.number().finite()),
    controlMedian: z.array(z.number().finite()),
    controlUpper: z.array(z.number().finite()),
    treatedLower: z.array(z.number().finite()),
    treatedMedian: z.array(z.number().finite()),
    treatedUpper: z.array(z.number().finite()),
  }).strict()),
}).strict()

export type BayesianGaussianEvidence = z.infer<typeof bayesianGaussianEvidenceSchema>
export type BayesianGaussianCurve = BayesianGaussianEvidence['curves'][number]

export const discreteStateStrategySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('observedStates'), states: z.number().int().min(2) }).strict(),
  z.object({
    kind: z.literal('quantiles'),
    requested: z.number().int().min(2),
    populated: z.number().int().min(2),
  }).strict(),
])

export const discreteStatePreparationSchema = z.object({
  node: z.number().int().nonnegative(),
  name: z.string().trim().min(1),
  strategy: discreteStateStrategySchema,
}).strict()

export type DiscreteStatePreparation = z.infer<typeof discreteStatePreparationSchema>

export const describeDiscreteStatePreparations = (
  preparations: readonly DiscreteStatePreparation[],
): string => preparations.map((preparation) => {
  switch (preparation.strategy.kind) {
    case 'observedStates':
      return `${preparation.name}: ${preparation.strategy.states} observed states`
    case 'quantiles':
      return `${preparation.name}: ${preparation.strategy.populated} quantile states (budget ${preparation.strategy.requested})`
    default:
      return assertNever(preparation.strategy)
  }
}).join('; ')

export const discreteBnEvidenceSchema = z.object({
  kind: z.literal('discreteBnQuery'),
  observations: z.number().int().positive(),
  bins: z.number().int().min(2),
  equivalentSampleSize: z.number().positive(),
  stateCounts: z.array(z.number().int().positive()),
  statePreparations: z.array(discreteStatePreparationSchema).min(2),
  treatmentStates: z.tuple([z.string(), z.string()]),
  expectations: z.tuple([z.number().finite(), z.number().finite()]),
  effect: z.number().finite(),
  distributionLow: z.array(z.tuple([z.string(), z.number().min(0).max(1 + 1e-9)])),
  distributionHigh: z.array(z.tuple([z.string(), z.number().min(0).max(1 + 1e-9)])),
  minimalAdjustmentSet: z.array(z.string()).nullable(),
  parentsAdjusted: z.array(z.string()),
}).strict()

export type DiscreteBnEvidence = z.infer<typeof discreteBnEvidenceSchema>

export const binaryEttEvidenceSchema = z.object({
  kind: z.literal('binaryEtt'),
  observations: z.number().int().positive(),
  treatment: z.number().int().nonnegative(),
  outcome: z.number().int().nonnegative(),
  treatedPotentialOutcomeMean: z.number().min(0).max(1),
  untreatedPotentialOutcomeMean: z.number().min(0).max(1),
  effectOnTreated: z.number().min(-1).max(1),
  treatedExpression: z.string().min(1),
  untreatedExpression: z.string().min(1),
}).strict()

export type BinaryEttEvidence = z.infer<typeof binaryEttEvidenceSchema>
export const parseBinaryEttEvidence = (value: unknown): Result<BinaryEttEvidence, EstimationEvidenceProblem> => parseWith(binaryEttEvidenceSchema, value)

const nodeSchema = causalEffectsNodeSchema

const wrightCoefficientEvidenceSchema = z.object({
  parent: nodeSchema,
  child: nodeSchema,
  coefficient: z.number().finite(),
}).strict()

const wrightPathEvidenceSchema = z.object({
  nodes: z.array(nodeSchema).min(2),
  coefficient: z.number().finite(),
  contrast: z.number().finite(),
}).strict()

export const causalEffectsAdjustmentProblemSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('queryTreatment'), node: nodeSchema }).strict(),
  z.object({ kind: z.literal('queryOutcome'), node: nodeSchema }).strict(),
  z.object({ kind: z.literal('laterTreatmentOccurrence'), node: nodeSchema }).strict(),
  z.object({ kind: z.literal('forbiddenNode'), node: nodeSchema }).strict(),
  z.object({ kind: z.literal('openNonCausalPath') }).strict(),
])
export type CausalEffectsAdjustmentProblem = z.infer<typeof causalEffectsAdjustmentProblemSchema>

const causalEffectsFitEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unfitted'), requested: totalEffectEstimatorSchema }).strict(),
  z.object({
    kind: z.literal('invalidAdjustment'),
    requested: totalEffectEstimatorSchema,
    problems: z.array(causalEffectsAdjustmentProblemSchema).min(1),
  }).strict(),
  z.object({ kind: z.literal('adjustedLinear'), selection: causalEffectsAdjustmentSchema, adjustmentSet: z.array(nodeSchema) }).strict(),
  z.object({ kind: z.literal('adjustedKnn'), k: z.number().int().positive(), selection: causalEffectsAdjustmentSchema, adjustmentSet: z.array(nodeSchema) }).strict(),
  z.object({
    kind: z.literal('wrightParents'),
    coefficients: z.array(wrightCoefficientEvidenceSchema).min(1),
    paths: z.array(wrightPathEvidenceSchema).min(1),
    directEffect: z.number().finite(),
    indirectEffect: z.number().finite(),
  }).strict(),
])
export type CausalEffectsFitEvidence = z.infer<typeof causalEffectsFitEvidenceSchema>

const causalEffectsUncertaintyEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('bootstrap'),
    samples: z.number().int().positive(),
    blockLength: causalEffectsBlockLengthSchema,
    resolvedBlockLength: z.number().int().positive(),
    confidenceLevel: z.number().gt(0).lt(1),
    seed: z.number().int().nonnegative(),
    predictionIntervals: z.tuple([
      z.tuple([z.number().finite(), z.number().finite()]),
      z.tuple([z.number().finite(), z.number().finite()]),
    ]),
    effectInterval: z.tuple([z.number().finite(), z.number().finite()]),
    effectDraws: z.array(z.number().finite()).min(1),
  }).strict(),
])
export type CausalEffectsUncertaintyEvidence = z.infer<typeof causalEffectsUncertaintyEvidenceSchema>

export const causalEffectsEvidenceSchema = z.object({
  kind: z.literal('causalEffectsTotal'),
  observations: z.number().int().positive(),
  tauMax: z.number().int().nonnegative(),
  noCausalPath: z.boolean(),
  identifiable: z.boolean(),
  mediators: z.array(nodeSchema),
  fit: causalEffectsFitEvidenceSchema,
  interventions: z.tuple([z.number().finite(), z.number().finite()]),
  predictions: z.array(z.number().finite()),
  totalEffect: nullableNumber,
  fittedObservations: z.number().int().nonnegative(),
  uncertainty: causalEffectsUncertaintyEvidenceSchema,
}).strict()

export type CausalEffectsEvidence = z.infer<typeof causalEffectsEvidenceSchema>

function causalEffectsUncertaintyMatches(
  requested: CausalEffectsUncertainty,
  returned: CausalEffectsUncertaintyEvidence,
): boolean {
  switch (requested.kind) {
    case 'none':
      return returned.kind === 'none'
    case 'bootstrap':
      if (returned.kind !== 'bootstrap') return false
      return requested.samples === returned.samples
        && requested.confidenceLevel === returned.confidenceLevel
        && requested.seed === returned.seed
        && (requested.blockLength.kind === 'cubeRoot'
          ? returned.blockLength.kind === 'cubeRoot'
          : returned.blockLength.kind === 'fixed' && requested.blockLength.length === returned.blockLength.length)
    default:
      return assertNever(requested)
  }
}

function adjustmentSelectionMatches(left: CausalEffectsAdjustment, right: CausalEffectsAdjustment): boolean {
  if (left.kind !== right.kind) return false
  if (left.kind !== 'explicit' || right.kind !== 'explicit') return true
  return left.nodes.length === right.nodes.length
    && left.nodes.every((node, index) => node[0] === right.nodes[index]?.[0] && node[1] === right.nodes[index]?.[1])
}

function causalEffectsFitMatches(requested: TotalEffectEstimator, returned: CausalEffectsFitEvidence): boolean {
  switch (requested.kind) {
    case 'linear': return (returned.kind === 'adjustedLinear' && adjustmentSelectionMatches(requested.adjustment, returned.selection))
      || ((returned.kind === 'unfitted' || returned.kind === 'invalidAdjustment') && returned.requested.kind === 'linear' && adjustmentSelectionMatches(requested.adjustment, returned.requested.adjustment))
    case 'knn': return (returned.kind === 'adjustedKnn' && returned.k === requested.k && adjustmentSelectionMatches(requested.adjustment, returned.selection))
      || ((returned.kind === 'unfitted' || returned.kind === 'invalidAdjustment') && returned.requested.kind === 'knn' && returned.requested.k === requested.k && adjustmentSelectionMatches(requested.adjustment, returned.requested.adjustment))
    case 'wrightParents': return returned.kind === 'wrightParents' || ((returned.kind === 'unfitted' || returned.kind === 'invalidAdjustment') && returned.requested.kind === 'wrightParents')
    default: return assertNever(requested)
  }
}

/** The fitted window drawn beside the forecast, so the counterfactual can be judged against the
 * rows it was fitted on. Runs saved before it was reported carry `notReported`. */
const preInterventionStepsSchema = {
  steps: z.array(z.number().int().nonnegative()),
  observed: z.array(z.number().finite()),
  counterfactual: z.array(z.number().finite()),
}
const notReportedPathSchema = z.object({ kind: z.literal('notReported') }).strict()
const fittedPreInterventionPathSchema = z.object({
  kind: z.literal('fitted'), ...preInterventionStepsSchema,
  se: z.array(z.number().finite().nonnegative()),
}).strict()
const sampledPreInterventionPathSchema = z.object({
  kind: z.literal('sampled'), ...preInterventionStepsSchema,
  lower: z.array(z.number().finite()), upper: z.array(z.number().finite()),
}).strict()
export const preInterventionPathSchema = z.discriminatedUnion('kind', [
  notReportedPathSchema, fittedPreInterventionPathSchema, sampledPreInterventionPathSchema,
])
export type PreInterventionPath = z.infer<typeof preInterventionPathSchema>

const maximumLikelihoodImpactEvidenceSchema = z.object({
  kind: z.literal('causalImpact'),
  preInterventionPath: z.discriminatedUnion('kind', [notReportedPathSchema, fittedPreInterventionPathSchema]),
  observations: z.number().int().positive(),
  nPre: z.number().int().positive(),
  nPost: z.number().int().positive(),
  postEnd: z.number().int().positive(),
  outcome: z.number().int().nonnegative(),
  controls: z.array(z.number().int().nonnegative()),
  counterfactual: z.array(z.number().finite()),
  counterfactualSe: z.array(z.number().finite().nonnegative()),
  pointwise: z.array(z.number().finite()),
  cumulative: z.number().finite(),
  average: z.number().finite(),
  params: z.array(z.number().finite()),
  logLikelihood: z.number().finite(),
}).strict()

const posteriorQuantitySchema = z.object({
  mean: z.number().finite(), lower: z.number().finite(), upper: z.number().finite(),
  sd: z.number().finite().nonnegative(),
}).strict().refine((value) => value.lower <= value.upper, 'Posterior bounds must be ordered.')
const impactSummarySchema = z.object({
  actual: z.number().finite(), predicted: posteriorQuantitySchema,
  absolute: posteriorQuantitySchema, relative: posteriorQuantitySchema,
  tailProbability: z.number().positive().max(1),
}).strict()
export const bayesianImpactEvidenceSchema = z.object({
  kind: z.literal('bayesianCausalImpact'),
  // Older saved runs did not record inclusion indicators. Absence is not zero inclusion.
  controlInclusion: z.array(z.object({ column:z.number().int().nonnegative(), probability:z.number().finite().min(0).max(1) }).strict()).optional(),
  observations: z.number().int().positive(), nPre: z.number().int().min(8), nPost: z.number().int().positive(),
  postEnd: z.number().int().positive(),
  outcome: z.number().int().nonnegative(), controls: z.array(z.number().int().nonnegative()),
  draws: z.number().int().min(2), warmup: z.number().int().nonnegative(), seed: z.number().int().min(0).max(0xffffffff),
  priorLevelSd: z.number().finite().positive(), level: z.literal(0.95),
  preInterventionPath: z.discriminatedUnion('kind', [notReportedPathSchema, sampledPreInterventionPathSchema]),
  counterfactual: z.array(z.number().finite()), counterfactualSe: z.array(z.number().finite().nonnegative()),
  counterfactualLower: z.array(z.number().finite()), counterfactualUpper: z.array(z.number().finite()),
  pointwise: z.array(z.number().finite()), pointwiseLower: z.array(z.number().finite()), pointwiseUpper: z.array(z.number().finite()),
  cumulativeLower: z.array(z.number().finite()), cumulativeUpper: z.array(z.number().finite()),
  cumulative: z.number().finite(), average: z.number().finite(),
  averageSummary: impactSummarySchema, cumulativeSummary: impactSummarySchema,
}).strict()
export const structuralImpactEvidenceSchema = bayesianImpactEvidenceSchema.omit({ kind:true, priorLevelSd:true }).extend({
  kind: z.literal('structuralCausalImpact'), model: structuralModelSchema,
  contributions: z.array(structuralContributionSchema).min(1),
}).strict()
export const causalImpactEvidenceSchema = z.discriminatedUnion('kind', [maximumLikelihoodImpactEvidenceSchema, bayesianImpactEvidenceSchema, structuralImpactEvidenceSchema])
export type CausalImpactEvidence = z.infer<typeof causalImpactEvidenceSchema>

export function impactInferenceMatches(configuration: { readonly inference?: BayesianImpactSettings }, evidence: CausalImpactEvidence): boolean {
  if (configuration.inference === undefined) return evidence.kind === 'causalImpact'
  const settings = configuration.inference
  if (settings.kind === 'structural') return evidence.kind === 'structuralCausalImpact'
    && settings.draws === evidence.draws && settings.warmup === evidence.warmup && settings.seed === evidence.seed
    && sameStructuralModel(settings.model, evidence.model)
  return evidence.kind === 'bayesianCausalImpact' && settings.draws === evidence.draws
    && settings.warmup === evidence.warmup && settings.seed === evidence.seed
    && settings.priorLevelSd === evidence.priorLevelSd
}

export type EstimationEvidenceProblem = { readonly kind: 'invalid-estimation-evidence'; readonly detail: string }

const parseWith = <Schema extends z.ZodTypeAny>(schema: Schema, value: unknown): Result<z.infer<Schema>, EstimationEvidenceProblem> => {
  const parsed = schema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ kind: 'invalid-estimation-evidence', detail: z.prettifyError(parsed.error) })
}

export function parseBackdoorLinearEvidence(value: unknown): Result<BackdoorLinearEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(backdoorLinearEvidenceSchema, value)
  if (!parsed.ok) return parsed
  if (parsed.value.interval[0] > parsed.value.interval[1] || parsed.value.hacInterval[0] > parsed.value.hacInterval[1]) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'An interval has its bounds reversed.' })
  }
  return parsed
}

export function parseFrontdoorTwoStageEvidence(value: unknown): Result<FrontdoorTwoStageEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(frontdoorTwoStageEvidenceSchema, value)
  if (!parsed.ok) return parsed
  if (parsed.value.uncertainty.kind === 'bootstrap' && parsed.value.uncertainty.interval[0] > parsed.value.uncertainty.interval[1]) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The front-door confidence interval has its bounds reversed.' })
  }
  return parsed
}

export function parseInstrumentalVariableEvidence(value: unknown): Result<InstrumentalVariableEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(instrumentalVariableEvidenceSchema, value)
  if (!parsed.ok) return parsed
  if (parsed.value.uncertainty.kind === 'bootstrap' && parsed.value.uncertainty.interval[0] > parsed.value.uncertainty.interval[1]) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The instrumental-variable confidence interval has its bounds reversed.' })
  }
  if (new Set([parsed.value.treatment, parsed.value.outcome, ...parsed.value.instruments]).size !== 2 + parsed.value.instruments.length) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The instrumental-variable columns are not distinct.' })
  }
  return parsed
}

export const parseCountGlmEvidence = (value: unknown): Result<CountGlmEvidence, EstimationEvidenceProblem> => parseWith(countGlmEvidenceSchema, value)
export const parseNegativeBinomialIngarchEvidence = (value: unknown): Result<NegativeBinomialIngarchEvidence, EstimationEvidenceProblem> => parseWith(negativeBinomialIngarchEvidenceSchema, value)

export function parseCausalEffectsEvidence(value: unknown): Result<CausalEffectsEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(causalEffectsEvidenceSchema, value)
  if (!parsed.ok) return parsed
  if (parsed.value.identifiable && (parsed.value.predictions.length !== 2 || parsed.value.totalEffect === null)) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'An identifiable effect must carry two predictions and a total effect.' })
  }
  const fitted = parsed.value.fit.kind === 'adjustedLinear'
    || parsed.value.fit.kind === 'adjustedKnn'
    || parsed.value.fit.kind === 'wrightParents'
  if (parsed.value.identifiable !== fitted) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The CausalEffects fit state does not agree with its identification state.' })
  }
  if (!parsed.value.identifiable && parsed.value.uncertainty.kind === 'bootstrap') {
    return err({ kind: 'invalid-estimation-evidence', detail: 'An unidentified effect cannot carry bootstrap uncertainty.' })
  }
  if (parsed.value.uncertainty.kind === 'bootstrap') {
    const uncertainty = parsed.value.uncertainty
    if (uncertainty.effectDraws.length !== uncertainty.samples) {
      return err({ kind: 'invalid-estimation-evidence', detail: 'The CausalEffects bootstrap draw count does not match its sample count.' })
    }
    if (uncertainty.effectInterval[0] > uncertainty.effectInterval[1] || uncertainty.predictionIntervals.some(([lower, upper]) => lower > upper)) {
      return err({ kind: 'invalid-estimation-evidence', detail: 'A CausalEffects bootstrap interval has its bounds reversed.' })
    }
  }
  if (parsed.value.fit.kind === 'wrightParents' && parsed.value.totalEffect !== null) {
    const scale = 1 + Math.abs(parsed.value.totalEffect)
    const decomposed = parsed.value.fit.directEffect + parsed.value.fit.indirectEffect
    const paths = parsed.value.fit.paths.reduce((sum, path) => sum + path.contrast, 0)
    if (Math.abs(parsed.value.totalEffect - decomposed) > 1e-9 * scale || Math.abs(parsed.value.totalEffect - paths) > 1e-9 * scale) {
      return err({ kind: 'invalid-estimation-evidence', detail: 'The Wright path contributions do not sum to the reported total effect.' })
    }
  }
  return parsed
}

export function parseCausalImpactEvidence(value: unknown): Result<CausalImpactEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(causalImpactEvidenceSchema, value)
  if (!parsed.ok) return parsed
  const { nPost, postEnd, counterfactual, counterfactualSe, pointwise } = parsed.value
  if (parsed.value.nPre + nPost !== postEnd || postEnd > parsed.value.observations) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The evaluated window does not run from the intervention row to its end inside the observations.' })
  }
  if (parsed.value.kind !== 'causalImpact') {
    const evidence = parsed.value
    const inclusion = evidence.controlInclusion
    if (inclusion !== undefined && (inclusion.length !== evidence.controls.length
      || new Set(inclusion.map(item => item.column)).size !== inclusion.length
      || inclusion.some(item => !evidence.controls.includes(item.column)))) {
      return err({ kind:'invalid-estimation-evidence', detail:'Control inclusion probabilities must match the fitted control columns.' })
    }
    const bands = [[evidence.counterfactualLower,evidence.counterfactualUpper],
      [evidence.pointwiseLower,evidence.pointwiseUpper], [evidence.cumulativeLower,evidence.cumulativeUpper]]
    if (bands.some(([lower,upper]) => lower.length !== nPost || upper.length !== nPost || lower.some((value,index) => value > upper[index]))) {
      return err({ kind: 'invalid-estimation-evidence', detail: 'The posterior bands must cover the post-intervention window with ordered bounds.' })
    }
  }
  if (parsed.value.kind === 'structuralCausalImpact') {
    const evidence = parsed.value
    const keys = evidence.contributions.map(c => c.component.kind === 'predictor' ? `predictor:${c.component.column}` : c.component.kind)
    const expected = ['trend', ...(evidence.model.seasonality.kind === 'none' ? [] : ['seasonal']), ...evidence.controls.map(c => `predictor:${c}`)]
    if (keys.length !== expected.length || new Set(keys).size !== keys.length || expected.some(k => !keys.includes(k))
      || evidence.contributions.some(c => c.mean.length !== postEnd || c.lower.length !== postEnd || c.upper.length !== postEnd || c.lower.some((v,i) => v > c.upper[i]))) {
      return err({ kind:'invalid-estimation-evidence', detail:'The component paths do not match the model, predictors or evaluated window.' })
    }
    const path = evidence.preInterventionPath
    if (path.kind !== 'sampled' || path.steps.length !== evidence.nPre || path.steps.some((v,i) => v !== i)) {
      return err({ kind:'invalid-estimation-evidence', detail:'The composed model needs a complete sampled training path.' })
    }
    const prediction = [...path.counterfactual,...evidence.counterfactual]
    if (prediction.some((value,t) => Math.abs(value-evidence.contributions.reduce((s,c) => s+c.mean[t],0)) > 1e-8*(1+Math.abs(value)))) {
      return err({ kind:'invalid-estimation-evidence', detail:'The component means do not reconstruct the latent prediction.' })
    }
  }
  if (counterfactual.length !== nPost || counterfactualSe.length !== nPost || pointwise.length !== nPost) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The counterfactual path does not cover the post-intervention window.' })
  }
  const before = parsed.value.preInterventionPath
  if (before.kind !== 'notReported') {
    const bands = before.kind === 'fitted' ? [before.se] : [before.lower, before.upper]
    const rows = before.steps.length
    if (rows === 0 || rows > parsed.value.nPre
      || [before.observed, before.counterfactual, ...bands].some((series) => series.length !== rows)
      || before.steps.some((step, index) => step >= parsed.value.nPre || (index > 0 && step <= before.steps[index - 1]))) {
      return err({ kind: 'invalid-estimation-evidence', detail: 'The pre-intervention path must cover rows of the fitted window in order.' })
    }
  }
  return parsed
}

/** One group of the effect modifier and the effect estimated within it. */
export interface GroupEffectEstimate {
  readonly label: string
  readonly lower: number | null
  readonly upper: number | null
  readonly value: number
  readonly standardError: number
  readonly interval: { readonly level: number; readonly lower: number; readonly upper: number }
  readonly observations: number
  /** DoubleML warns below six observations; the group is shown but read with that in mind. */
  readonly fewObservations: boolean
}

export interface TimeEffectPoint {
  /** One-based row in the prepared series. */
  readonly step: number
  readonly actual: number
  readonly counterfactual: number
  readonly lower: number
  readonly upper: number
  readonly effect: number
}

export type EffectEstimate =
  | { readonly kind: 'additive'; readonly value: number; readonly unit: string }
  | { readonly kind: 'expectedCountRatio'; readonly value: number }
  | { readonly kind: 'path'; readonly values: NonEmptyArray<TimeEffectPoint>; readonly aggregate: { readonly cumulative: number; readonly average: number } }
  /** One additive effect per group of an effect modifier, with the whole-population average beside them. */
  | { readonly kind: 'byGroup'; readonly modifier: string; readonly overall: number; readonly groups: NonEmptyArray<GroupEffectEstimate> }
  /** One additive effect per prepared row, in row order, with their mean as the whole-population average. */
  | { readonly kind: 'perRow'; readonly overall: number; readonly effects: NonEmptyArray<number> }

/** The one figure an estimate is filed under: its value, its average, or its cumulative path total. */
export const headlineValue = (effect: EffectEstimate): number => {
  switch (effect.kind) {
    case 'additive':
    case 'expectedCountRatio': return effect.value
    case 'path': return effect.aggregate.cumulative
    case 'byGroup':
    case 'perRow': return effect.overall
    default: return assertNever(effect)
  }
}

/** What a reader can say about a set of per-row effects without an interval: where they sit and how many rows the treatment helps. */
export interface RowEffectSummary {
  readonly rows: number
  readonly minimum: number
  readonly lowerQuartile: number
  readonly median: number
  readonly upperQuartile: number
  readonly maximum: number
  /** The share of rows whose effect is above zero. */
  readonly positiveShare: number
  /** Equal-width bins over the effects, for the histogram; `counts.length + 1` edges. */
  readonly bins: { readonly edges: readonly number[]; readonly counts: readonly number[] }
}

/** NumPy's default linear interpolation between order statistics, the same rule the core's quantile uses. */
const orderQuantile = (sorted: readonly number[], probability: number): number => {
  const position = probability * (sorted.length - 1)
  const lower = Math.floor(position)
  const upper = Math.ceil(position)
  const weight = position - lower
  return sorted[lower] * (1 - weight) + sorted[upper] * weight
}

export function summariseRowEffects(effects: NonEmptyArray<number>, binCount = 20): RowEffectSummary {
  const sorted = [...effects].sort((a, b) => a - b)
  const minimum = sorted[0]
  const maximum = sorted[sorted.length - 1]
  const width = maximum > minimum ? (maximum - minimum) / binCount : 1
  const edges = Array.from({ length: binCount + 1 }, (_, index) => minimum + index * width)
  const counts = new Array<number>(binCount).fill(0)
  for (const value of effects) {
    // The last bin includes its upper edge, as the data studio's histogram does.
    const bin = Math.min(binCount - 1, Math.max(0, Math.floor((value - minimum) / width)))
    counts[bin] += 1
  }
  return {
    rows: effects.length,
    minimum,
    lowerQuartile: orderQuantile(sorted, 0.25),
    median: orderQuantile(sorted, 0.5),
    upperQuartile: orderQuantile(sorted, 0.75),
    maximum,
    positiveShare: effects.filter((value) => value > 0).length / effects.length,
    bins: { edges, counts },
  }
}

export type EstimateInterval =
  | { readonly kind: 'confidence'; readonly level: number; readonly lower: number; readonly upper: number }
  | { readonly kind: 'credible'; readonly level: number; readonly summary: 'HDI' | 'ETI'; readonly lower: number; readonly upper: number }
  | { readonly kind: 'none'; readonly reason: string }

/** The typed interval label for the number formatter: confidence stays a CI, credible carries its summary. */
export const intervalTypeOf = (interval: Exclude<EstimateInterval, { readonly kind: 'none' }>) =>
  interval.kind === 'confidence'
    ? { kind: 'confidence' as const, level: interval.level }
    : { kind: 'credible' as const, level: interval.level, summary: interval.summary }

export interface CausalEstimate {
  readonly kind: 'causal-estimate'
  readonly estimand: Estimand
  readonly effect: EffectEstimate
  readonly interval: EstimateInterval
  readonly standardError: number | null
  readonly adjustment: AppliedAdjustment
  readonly sample: { readonly observations: number; readonly parameters: number; readonly degreesOfFreedom: number | null }
}

export interface TimeIndexedStudyVariable {
  readonly variable: StudyVariable
  /** Zero is the reference period; negative values are prior periods. */
  readonly lag: number
}

/** The covariates actually supplied to an estimator, including their time index when relevant. */
export type AppliedAdjustment =
  | { readonly kind: 'none' }
  | { readonly kind: 'contemporaneous'; readonly variables: NonEmptyArray<StudyVariable> }
  | { readonly kind: 'time-indexed'; readonly variables: NonEmptyArray<TimeIndexedStudyVariable> }
  | { readonly kind: 'structural-parent-model'; readonly coefficients: number; readonly paths: number }

const appliedContemporaneousAdjustment = (variables: readonly StudyVariable[]): AppliedAdjustment =>
  isNonEmpty(variables) ? { kind: 'contemporaneous', variables } : { kind: 'none' }

const appliedTimeIndexedAdjustment = (
  graphVariables: readonly (StudyVariable | null)[],
  nodes: readonly (readonly [number, number])[],
): AppliedAdjustment | null => {
  const variables: TimeIndexedStudyVariable[] = []
  for (const [index, lag] of nodes) {
    const variable = graphVariables[index]
    if (variable === undefined || variable === null || lag > 0) return null
    variables.push({ variable, lag })
  }
  return isNonEmpty(variables) ? { kind: 'time-indexed', variables } : { kind: 'none' }
}

/** Contemporaneous columns are reusable by row-wise estimators; temporal rows are not raw columns. */
export const contemporaneousAdjustmentVariables = (adjustment: AppliedAdjustment): readonly StudyVariable[] | null => {
  switch (adjustment.kind) {
    case 'none': return []
    case 'contemporaneous': return adjustment.variables
    case 'time-indexed': return null
    case 'structural-parent-model': return null
    default: return assertNever(adjustment)
  }
}

export const adjustmentLabels = (adjustment: AppliedAdjustment): readonly string[] => {
  switch (adjustment.kind) {
    case 'none': return []
    case 'contemporaneous': return adjustment.variables.map((variable) => variable.name)
    case 'time-indexed': return adjustment.variables.map(({ variable, lag }) => `${variable.name} (${lag === 0 ? 't' : `t−${Math.abs(lag)}`})`)
    case 'structural-parent-model': return []
    default: return assertNever(adjustment)
  }
}

export type AcceptedEstimatorEligibility = Exclude<MethodEligibility, { readonly kind: 'refused' }>

interface RunIdentity {
  readonly id: EstimationRunId
  readonly study: StudyId
  readonly identification: IdentificationId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly eligibility: AcceptedEstimatorEligibility
  /** Columns in the order the numeric matrix was materialised. */
  readonly columns: NonEmptyArray<StudyVariable>
  readonly estimate: CausalEstimate
}

export type EstimationRunArtifact =
  | RunIdentity & { readonly kind: 'sharp-rd-run'; readonly method: typeof SHARP_RD_METHOD_ID; readonly configuration: SharpRdConfiguration; readonly evidence: SharpRdEvidence }
  | RunIdentity & { readonly kind: 'backdoor-linear-run'; readonly method: typeof BACKDOOR_LINEAR_REGRESSION_METHOD_ID; readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }
  | RunIdentity & { readonly kind: 'frontdoor-two-stage-run'; readonly method: typeof FRONTDOOR_TWO_STAGE_METHOD_ID; readonly configuration: FrontdoorTwoStageConfiguration; readonly evidence: FrontdoorTwoStageEvidence }
  | RunIdentity & { readonly kind: 'instrumental-variable-run'; readonly method: typeof INSTRUMENTAL_VARIABLE_METHOD_ID; readonly configuration: InstrumentalVariableConfiguration; readonly evidence: InstrumentalVariableEvidence }
  | RunIdentity & { readonly kind: 'count-glm-run'; readonly method: typeof POISSON_GLM_METHOD_ID | typeof NEGATIVE_BINOMIAL_METHOD_ID; readonly configuration: CountGlmConfiguration; readonly evidence: CountGlmEvidence }
  | RunIdentity & { readonly kind: 'negative-binomial-ingarch-run'; readonly method: typeof NEGATIVE_BINOMIAL_INGARCH_METHOD_ID; readonly configuration: NegativeBinomialIngarchConfiguration; readonly evidence: NegativeBinomialIngarchEvidence }
  | RunIdentity & { readonly kind: 'double-ml-run'; readonly method: typeof DML_PLR_METHOD_ID | typeof DML_IRM_METHOD_ID; readonly configuration: DoubleMlConfiguration; readonly evidence: DoubleMlEvidence }
  | RunIdentity & { readonly kind: 't-learner-run'; readonly method: typeof T_LEARNER_METHOD_ID; readonly configuration: TLearnerConfiguration; readonly evidence: TLearnerEvidence }
  | RunIdentity & { readonly kind: 'ardl-run'; readonly method: typeof ARDL_PSS_METHOD_ID; readonly configuration: ArdlConfiguration; readonly evidence: ArdlEvidence }
  | RunIdentity & { readonly kind: 'vecm-run'; readonly method: typeof VECM_METHOD_ID; readonly configuration: VecmConfiguration; readonly evidence: VecmEvidence }
  | RunIdentity & { readonly kind: 'synthetic-control-run'; readonly method: typeof SYNTHETIC_CONTROL_METHOD_ID; readonly configuration: SyntheticControlConfiguration; readonly evidence: SyntheticControlEvidence }
  | RunIdentity & { readonly kind: 'panel-intervention-run'; readonly method: typeof PANEL_INTERVENTION_METHOD_ID; readonly configuration: PanelInterventionConfiguration; readonly evidence: PanelInterventionEvidence; readonly timeLabels: NonEmptyArray<string> }
  | RunIdentity & { readonly kind: 'negbin-nuts-run'; readonly method: typeof NEGBIN_NUTS_METHOD_ID; readonly configuration: NegbinNutsConfiguration; readonly evidence: NegbinNutsEvidence }
  | RunIdentity & { readonly kind: 'bayesian-gaussian-run'; readonly method: typeof BAYESIAN_GAUSSIAN_METHOD_ID; readonly configuration: BayesianGaussianConfiguration; readonly evidence: BayesianGaussianEvidence }
  | RunIdentity & { readonly kind: 'discrete-bn-run'; readonly method: typeof DISCRETE_BN_METHOD_ID; readonly configuration: DiscreteBnConfiguration; readonly evidence: DiscreteBnEvidence }
  | RunIdentity & { readonly kind: 'binary-ett-run'; readonly method: typeof BINARY_ETT_METHOD_ID; readonly configuration: BinaryEttConfiguration; readonly evidence: BinaryEttEvidence }
  | RunIdentity & { readonly kind: 'causal-effects-run'; readonly method: typeof CAUSAL_EFFECTS_TOTAL_METHOD_ID; readonly configuration: CausalEffectsConfiguration; readonly evidence: CausalEffectsEvidence }
  | RunIdentity & { readonly kind: 'causal-impact-run'; readonly method: typeof CAUSAL_IMPACT_METHOD_ID; readonly configuration: CausalImpactConfiguration; readonly evidence: CausalImpactEvidence }

export const newEstimationRunId = (): EstimationRunId => brand<string, 'EstimationRunId'>(crypto.randomUUID())

type Satisfied = Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>
type Unresolved = Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>
type Violated = Extract<CaveatEvaluation, { readonly kind: 'violated' }>

/** Complete typed input used to evaluate estimator eligibility. */
export interface EligibilityContext {
  readonly identification: Identification
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly configuration: EstimatorConfiguration
  /** Whether the outcome column holds non-negative integers; null until the data has been read. */
  readonly outcomeIsCount: boolean | null
  /** Whether the treatment column holds only 0 and 1; null until the data has been read. */
  readonly treatmentIsBinary: boolean | null
  /** Whether every observed graph variable holds only 0 and 1; null until those columns have been read. */
  readonly observedGraphIsBinary: boolean | null
  /** The DAG behind the study, for the time-graph rules. */
  readonly document: DagDocument | null
  /** The study whose variables the stationarity rules look up; null when the identification has no study loaded. */
  readonly study: StudySpecification | null
  /** Structural treatment-layout evidence loaded before a panel estimator can run. */
  readonly panelPreflight: PanelInterventionPreflight
}

/** The treatment, the outcome and the adjustment set with their stationarity assessments on a time series. */
const levelReadings = (context: EligibilityContext): readonly { readonly name: string; readonly assessment: StationarityAssessment | null }[] => {
  if (context.study === null || context.identification.kind !== 'identified') return []
  const variables = [context.study.treatment, context.study.outcome, ...context.identification.adjustment.variables]
  return variables.map((variable) => ({ name: variable.name, assessment: context.stationarity?.variables.find((candidate) => candidate.column === variable.column)?.assessment ?? null }))
}

/** Level-model verdicts for the treatment, the outcome and the adjustment set on a time series. */
const levelVerdicts = (context: EligibilityContext): readonly LevelModelVerdict[] =>
  levelReadings(context).map((reading) => levelModelVerdict(reading.name, reading.assessment))

/** Cointegration procedures must receive the study variables in levels, not a prepared transform. */
const transformedStudyVariables = (context: EligibilityContext): readonly { readonly name: string; readonly transform: string }[] => {
  if (context.prepared.kind !== 'prepared-time-series' || context.study === null) return []
  const variables = [context.study.treatment, context.study.outcome, ...(context.identification.kind === 'identified' ? context.identification.adjustment.variables : [])]
  return variables.flatMap((variable) => {
    const transform = seriesTransformFor(context.prepared.kind === 'prepared-time-series' ? context.prepared.seriesTransforms : [], variable.column)
    return transform.kind === 'levels' ? [] : [{ name: variable.name, transform: describeSeriesTransform(transform) }]
  })
}

/** Integration-order rule for the long-run estimators: every study variable I(0) or I(1), or every one I(1). */
const applyOrderRule = (
  id: string,
  context: EligibilityContext,
  mode: 'i0-or-i1' | 'i1-only',
  satisfy: (id: string, evidence: string) => void,
  leave: (id: string, missingEvidence: string) => void,
  violate: (id: string, evidence: string) => void,
): void => {
  if (context.study === null) { leave(id, 'No study is loaded, so the variables cannot be looked up.'); return }
  if (context.stationarity === null) { leave(id, 'Run stationarity tests for this prepared dataset version in Data studio.'); return }
  const variables = [context.study.treatment, context.study.outcome, ...(context.identification.kind === 'identified' ? context.identification.adjustment.variables : [])]
  const readings = variables.map((variable) => ({ name: variable.name, assessment: context.stationarity?.variables.find((candidate) => candidate.column === variable.column)?.assessment ?? null }))
  const refused: string[] = []
  const open: string[] = []
  const accepted: string[] = []
  for (const reading of readings) {
    if (reading.assessment === null) { open.push(`Run stationarity tests for ${reading.name} on this prepared dataset version.`); continue }
    switch (reading.assessment.kind) {
      case 'levelStationary':
      case 'trendStationary':
      case 'breakStationary':
        if (mode === 'i1-only') refused.push(`${reading.name} is stationary in levels and does not belong in a cointegrated system.`)
        else accepted.push(`${reading.name} is I(0).`)
        break
      case 'differenceStationary':
        accepted.push(`${reading.name} is I(1).`)
        break
      case 'higherOrderOrUnresolved':
        refused.push(`${reading.name} is I(2) or unresolved.`)
        break
      case 'inconclusive':
        refused.push(`${reading.name}'s order is unresolved: ${reading.assessment.conflicts.map(describeStationarityConflict).join(' ')}`)
        break
      default:
        assertNever(reading.assessment)
    }
  }
  if (refused.length > 0) { violate(id, refused.join(' ')); return }
  if (open.length > 0) { leave(id, open.join(' ')); return }
  satisfy(id, accepted.join(' '))
}

const applyLevelRule = (
  id: string,
  context: EligibilityContext,
  satisfy: (id: string, evidence: string) => void,
  leave: (id: string, missingEvidence: string) => void,
  violate: (id: string, evidence: string) => void,
): void => {
  if (context.prepared.kind === 'prepared-panel') { leave(id, 'Rows are a panel; trends are assessed within units, which this rule does not cover.'); return }
  if (context.prepared.kind !== 'prepared-time-series') { satisfy(id, 'Independent observations carry no stochastic trend.'); return }
  if (context.stationarity === null) { leave(id, 'Run stationarity tests for this prepared dataset version in Data studio.'); return }
  const readings = levelReadings(context)
  // I(2) or an unsettled order refuses (DESIGN.md, integration table); I(1) warns in one sentence, since the verdict is itself a test.
  const higher = readings.filter((reading) => reading.assessment?.kind === 'higherOrderOrUnresolved')
  if (higher.length > 0) { violate(id, higher.map((reading) => levelModelVerdict(reading.name, reading.assessment).reason).join(' ')); return }
  const integrated = readings.filter((reading) => reading.assessment?.kind === 'differenceStationary').map((reading) => reading.name)
  const open = readings
    .filter((reading) => reading.assessment?.kind !== 'differenceStationary')
    .map((reading) => levelModelVerdict(reading.name, reading.assessment))
    .filter((verdict) => verdict.kind === 'unresolved')
    .map((verdict) => verdict.reason)
  const integratedText = integrated.length === 0
    ? ''
    : `${integrated.join(', ')} ${integrated.length === 1 ? 'is' : 'are'} I(1) on the prepared scale, so a regression on those values can show a spurious relation. Create a differenced prepared version or use a suitable cointegration method.`
  if (integrated.length > 0 || open.length > 0) { leave(id, [integratedText, ...open].filter((text) => text.length > 0).join(' ')); return }
  satisfy(id, levelVerdicts(context).map((verdict) => verdict.reason).join(' ') || 'No study variables to check.')
}

const findCaveat = (method: MethodDefinition, id: string): MethodDefinition['caveats'][number] =>
  method.caveats.find((caveat) => caveat.id === id) ?? (() => {
    throw new Error(`Method catalogue invariant failed: ${method.name} has no condition “${id}”.`)
  })()

const isNonEmpty = <Value>(values: readonly Value[]): values is NonEmptyArray<Value> => values.length > 0

const TARGET_COMPATIBILITY_CAVEAT: MethodCaveat = {
  id: brand<string, 'MethodCaveatId'>('estimand-target-compatibility'),
  category: 'interpretation',
  requirement: 'The estimator must report the target population recorded in the study specification.',
  consequenceIfUnmet: 'An ATE and an ATT answer different causal questions and cannot be substituted for one another.',
  sources: [{ kind: 'paper', title: 'Hernán and Robins, Causal Inference: What If', locator: 'https://www.hsph.harvard.edu/miguel-hernan/causal-inference-book/' }],
}

/** Whether an estimator reports the study's target, decided once per target. */
type TargetVerdict =
  | { readonly kind: 'reported'; readonly evidence: string }
  | { readonly kind: 'not-reported'; readonly evidence: string }

const reported = (evidence: string): TargetVerdict => ({ kind: 'reported', evidence })
const notReported = (evidence: string): TargetVerdict => ({ kind: 'not-reported', evidence })

function targetCompatibility(estimand: Estimand, configuration: EstimatorConfiguration): TargetVerdict {
  if (configuration.kind === 'panel-intervention') return estimand.kind === 'average-treatment-effect-on-treated'
    ? reported('The panel comparison targets the average effect on the treated group.')
    : notReported('Panel DiD targets the treated group. Record an ATT study before running; existing saved runs retain their original labels.')
  if (configuration.kind === 'sharp-rd') return estimand.kind === 'local-cutoff-effect'
    ? reported('The study and estimator both target the local effect at the recorded cutoff.')
    : notReported('Sharp RD estimates the average treatment effect at the cutoff. For a time-based design, this is the event time. In Study design, choose “At an assignment cutoff (sharp RD)”. Then set the running variable and cutoff.')
  // The T-learner reports one effect per row and no average target, so it is decided before the per-target rules.
  if (configuration.kind === 't-learner') {
    return estimand.kind === 'conditional-average-treatment-effect-per-row'
      ? reported('Estimator and study both target the effect for each row.')
      : notReported('The T-learner reports one effect per row, but the study records an average target.')
  }
  switch (estimand.kind) {
    case 'local-cutoff-effect': return notReported('This cutoff-local target requires the sharp RD estimator.')
    case 'average-treatment-effect':
      if (configuration.kind === 'binary-ett-idc-star') return notReported('The binary IDC* evaluator reports ETT/ATT, but this study records ATE.')
      if (configuration.kind === 'dml-irm' && configuration.att) return notReported('The DML configuration reports ATT, but the study records ATE.')
      return reported('Estimator and study both target ATE.')
    case 'average-treatment-effect-on-treated':
      if (configuration.kind === 'binary-ett-idc-star') return reported('Estimator and study both target ATT.')
      if (configuration.kind === 'dml-irm') return configuration.att ? reported('Estimator and study both target ATT.') : notReported('The DML configuration reports ATE, but the study records ATT.')
      return notReported('This study targets ATT. DML interactive and the binary IDC* evaluator report ATT.')
    case 'conditional-average-treatment-effect':
      if (configuration.kind === 'dml-plr' || (configuration.kind === 'dml-irm' && !configuration.att)) {
        return reported('')
      }
      return notReported(`This study targets the effect within groups of ${estimand.modifier.name}. Only the double machine learning estimators report group effects.`)
    case 'conditional-average-treatment-effect-per-row':
      return notReported('This study targets the effect for each row. Only the T-learner reports per-row effects.')
    default: return assertNever(estimand)
  }
}

const verdict = (satisfied: Satisfied[], unresolved: Unresolved[], violations: Violated[]): MethodEligibility => {
  if (isNonEmpty(violations)) return { kind: 'refused', satisfied, unresolved, violations }
  if (isNonEmpty(unresolved)) return { kind: 'caution', satisfied, unresolved }
  return { kind: 'eligible', satisfied }
}

/** Rules over the identification, the sampling structure, the data shape, and the chosen configuration. */
/** What the double machine learning nuisance learners see: the identified set, plus the effect modifier of a conditional target. */
export function dmlNuisanceInputs(adjustment: readonly StudyVariable[], estimand: Estimand | null): readonly StudyVariable[] {
  const modifier = estimand?.kind === 'conditional-average-treatment-effect' ? estimand.modifier : null
  return modifier === null || adjustment.some((variable) => variable.column === modifier.column) ? adjustment : [...adjustment, modifier]
}

/** What the T-learner's outcome forests see, and what each row's effect is conditioned on: the identified set, then the per-row target's modifiers not already in it. */
export function tLearnerInputs(adjustment: readonly StudyVariable[], estimand: Estimand | null): readonly StudyVariable[] {
  const modifiers = estimand?.kind === 'conditional-average-treatment-effect-per-row' ? estimand.modifiers : []
  return [...adjustment, ...modifiers.filter((modifier) => !adjustment.some((variable) => variable.column === modifier.column))]
}

export function evaluateEstimatorEligibility(method: MethodDefinition, context: EligibilityContext): MethodEligibility {
  const { identification, prepared, configuration } = context
  const timeSeries = prepared.kind === 'prepared-time-series'
  const panel = context.prepared.kind === 'prepared-panel'
  const satisfied: Satisfied[] = []
  const unresolved: Unresolved[] = []
  const violations: Violated[] = []
  const satisfy = (id: string, evidence: string) => satisfied.push({ kind: 'satisfied', caveat: findCaveat(method, id), evidence })
  const leave = (id: string, missingEvidence: string) => unresolved.push({ kind: 'unresolved', caveat: findCaveat(method, id), missingEvidence })
  const violate = (id: string, evidence: string) => violations.push({ kind: 'violated', caveat: findCaveat(method, id), evidence })
  const adjustment = identification.kind === 'identified'
    ? identification.adjustment.variables.length === 0 ? 'nothing' : identification.adjustment.variables.map((variable) => variable.name).join(', ')
    : null

  if (context.study !== null) {
    const verdict = targetCompatibility(context.study.estimand, configuration)
    switch (verdict.kind) {
      case 'reported': satisfied.push({ kind: 'satisfied', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: verdict.evidence }); break
      case 'not-reported': violations.push({ kind: 'violated', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: verdict.evidence }); break
      default: assertNever(verdict)
    }
  }

  switch (configuration.kind) {
    case 'sharp-rd': {
      if (identification.kind !== 'cutoff-design' || context.study?.estimand.kind !== 'local-cutoff-effect') violate('rd-design', 'Record a cutoff-local study and its running variable before fitting sharp RD.')
      else leave('rd-design', 'Justify continuity of potential outcomes, no precise manipulation and no other change at the cutoff. The fit checks sharp assignment, not these causal assumptions.')
      if (prepared.kind !== 'prepared-cross-section') violate('rd-sample', 'This RD implementation requires independent observations; it does not provide clustered or time-series uncertainty.')
      else leave('rd-sample', 'The fit checks support on both sides and selects the mserd bandwidth. Inspect observations near the cutoff.')
      break
    }
    case 'backdoor-linear-regression': {
      if (adjustment === null) violate('linear-identified-adjustment', 'No measured back-door adjustment set was found, so this adjusted regression cannot run.')
      else satisfy('linear-identified-adjustment', `Identified by back-door adjustment for ${adjustment}.`)
      if (panel) leave('linear-serial-dependence', 'Rows repeat within units; neither interval accounts for within-unit correlation.')
      else if (timeSeries && configuration.errors.kind === 'classical') violate('linear-serial-dependence', 'The rows are a time series and the classical interval assumes independent errors. Choose the HAC interval or an ARMA error process.')
      else if (configuration.errors.kind === 'arma') satisfy('linear-serial-dependence', `An ARMA(${configuration.errors.p}, ${configuration.errors.q}) error process is fitted with the coefficients by maximum likelihood; the interval comes from that fit.`)
      else satisfy('linear-serial-dependence', timeSeries ? 'Heteroskedasticity and autocorrelation consistent (HAC) Newey–West interval selected for time-series rows.' : 'The prepared dataset holds independent rows, so the classical interval applies.')
      satisfy('linear-hac-bandwidth', 'The run records the bandwidth from the default rule.')
      leave('linear-functional-form', 'Check linearity with the residual diagnostics in the sensitivity chapter.')
      leave('linear-overlap', 'Inspect treatment overlap against the adjustment variables in Data studio.')
      if (timeSeries) leave('linear-not-time-graph', 'Lagged effects are not estimated by this method.')
      else satisfy('linear-not-time-graph', 'Independent observations carry no lag structure.')
      applyLevelRule('linear-level-stationarity', context, satisfy, leave, violate)
      break
    }
    case 'frontdoor-two-stage': {
      if (identification.kind !== 'graphically-identified' || identification.frontdoor.kind !== 'identified') {
        violate('frontdoor-identified-mediator', 'The identification record does not contain a front-door set for this treatment and outcome.')
        violate('frontdoor-single-mediator', 'No front-door mediator is available to the estimator.')
      } else {
        satisfy('frontdoor-identified-mediator', `The graph identifies ${identification.frontdoor.mediators.map((mediator) => mediator.name).join(', ')} as the front-door set.`)
        if (identification.frontdoor.mediators.length === 1) satisfy('frontdoor-single-mediator', `${identification.frontdoor.mediators[0].name} is the single identified mediator.`)
        else violate('frontdoor-single-mediator', `The identified front-door set contains ${identification.frontdoor.mediators.length} mediators; this estimator supports one.`)
      }
      leave('frontdoor-linear-stages', 'Assess whether treatment–mediator and mediator–outcome relations are adequately represented by additive linear regressions over the chosen contrast.')
      if (timeSeries) violate('frontdoor-bootstrap-rows', 'The prepared rows are a time series, but this estimator uses an ordinary row bootstrap and does not preserve temporal dependence.')
      else if (panel) violate('frontdoor-bootstrap-rows', 'The prepared rows repeat units, but this estimator uses an ordinary row bootstrap and does not preserve within-unit dependence.')
      else leave('frontdoor-bootstrap-rows', `The run uses ${configuration.simulations} seeded row resamples; confirm that observations are independently sampled.`)
      break
    }
    case 'instrumental-variable': {
      const instruments = identifiedInstruments(identification)
      const treatmentName = context.study?.treatment.name ?? 'the treatment'
      const outcomeName = context.study?.outcome.name ?? 'the outcome'
      if (instruments === null) {
        violate('iv-identified-instrument', `The identification record names no observed instrument for ${treatmentName} and ${outcomeName}. If the instrumental variables are latent, an instrumental variable estimand cannot be targeted.`)
      } else {
        const names = instruments.map((variable) => variable.name).join(', ')
        satisfy('iv-identified-instrument', `${names} ${instruments.length === 1 ? 'meets' : 'meet'} as-if-random (any backdoor paths between the instrument and ${outcomeName} can be blocked) and exclusion (the instrument is a cause of ${outcomeName} only indirectly through ${treatmentName}) in the recorded graph.`)
      }
      leave('iv-linearity', `Assess whether ${outcomeName} and ${treatmentName} are linear in the instrument: the estimate is the ratio of the coefficients of the linear models ${outcomeName} ~ instrument and ${treatmentName} ~ instrument, fitted without covariates.`)
      leave('iv-effect-homogeneity', `Assess whether each unit’s ${treatmentName} is affected in the same way by the common causes of ${treatmentName} and ${outcomeName}, and each unit’s ${outcomeName} likewise; the estimator reports one effect for every unit.`)
      leave('iv-instrument-strength', `Check that the instrument is strong, meaning it has a strong causal effect on ${treatmentName}; weak instruments can lead to high variance estimates of the ATE. The run reports no first-stage F statistic.`)
      if (timeSeries) violate('iv-bootstrap-rows', 'The prepared rows are a time series, but this estimator uses an ordinary row bootstrap and does not preserve temporal dependence.')
      else if (panel) violate('iv-bootstrap-rows', 'The prepared rows repeat units, but this estimator uses an ordinary row bootstrap and does not preserve within-unit dependence.')
      else leave('iv-bootstrap-rows', `The run uses ${configuration.simulations} seeded row resamples; confirm that observations are independently sampled.`)
      break
    }
    case 'panel-intervention': {
      if (prepared.kind !== 'prepared-panel') {
        violate('panel-balanced-layout', 'This estimator requires a long panel prepared with explicit unit and time keys.')
      } else if (!prepared.panel.balanced) {
        violate('panel-balanced-layout', 'The prepared unit–time grid is not balanced.')
      } else switch (context.panelPreflight.kind) {
        case 'not-applicable':
        case 'pending':
          leave('panel-balanced-layout', 'The treatment layout is being checked against the prepared panel before this estimator can run.')
          break
        case 'refused':
          violate('panel-balanced-layout', describePanelInterventionPreflight(context.panelPreflight))
          break
        case 'ready': {
          const layout = context.panelPreflight.layout
          satisfy('panel-balanced-layout', `${layout.controls.length} control and ${layout.treated.length} treated units across ${layout.prePeriods} pre- and ${layout.postPeriods} post-periods; treatment adopts simultaneously at ${layout.adoption.label} and remains on.`)
          break
        }
        default: assertNever(context.panelPreflight)
      }
      leave('panel-parallel-trends', 'Parallel untreated trends cannot be established from the panel shape. Inspect pre-treatment paths when multiple pre-periods are available and justify the comparison substantively.')
      leave('panel-no-anticipation', 'Confirm that outcomes were not affected before the treatment indicator first turns on.')
      if (context.study?.designAssumptions.noInterference.rationale === null || context.study === null) {
        leave('panel-no-spillovers', 'The study has no recorded rationale for no interference between units.')
      } else {
        satisfy('panel-no-spillovers', `Recorded study rationale: ${context.study.designAssumptions.noInterference.rationale}`)
      }
      if (configuration.primary === 'adjusted') {
        if (!adjustedDidConfigurationSchema.safeParse(configuration).success) violate('panel-pre-fit', 'Review the DiD covariate selection and inference settings before running.')
        const layout = context.panelPreflight.kind === 'ready' ? context.panelPreflight.layout : null
        if (layout !== null && (layout.prePeriods !== 1 || layout.postPeriods !== 1)) violate('panel-balanced-layout', 'Regression and DR DiD require exactly one before and one after period.')
        satisfy('panel-pre-fit', 'This two-period fit does not estimate synthetic weights.')
        if (configuration.specification.kind === 'doublyRobust') {
          if (configuration.covariates.length === 0) violate('panel-pre-fit', 'Select at least one baseline covariate for DR DiD.')
          if (layout !== null && configuration.specification.folds > Math.min(layout.controls.length, layout.treated.length)) violate('panel-pre-fit', 'The fold count cannot exceed the smaller treatment group.')
          leave('panel-parallel-trends', 'DR DiD requires conditional parallel trends and treatment overlap given baseline covariates. Clipping propensity estimates does not establish overlap.')
        }
      } else if (configuration.primary === 'did') {
        satisfy('panel-pre-fit', 'Conventional DiD does not require synthetic weights or pre-period variation.')
      } else if (context.panelPreflight.kind === 'ready' && context.panelPreflight.layout.controlPreDifferenceSd === null) {
        violate('panel-pre-fit', 'Synthetic DiD needs non-constant control changes before adoption. Choose conventional DiD for this panel.')
      } else if (context.panelPreflight.kind === 'ready' && context.panelPreflight.layout.controlPreDifferenceSd !== null) {
        satisfy('panel-pre-fit', `${context.panelPreflight.layout.controls.length} controls and ${context.panelPreflight.layout.prePeriods} pre-periods provide non-constant control changes. Their standard deviation is ${context.panelPreflight.layout.controlPreDifferenceSd.toPrecision(4)}. The run reports all fitted weights.`)
      } else {
        leave('panel-pre-fit', 'The run checks control variation before treatment and reports the fitted unit and time weights.')
      }
      if (configuration.primary === 'adjusted') {
        leave('panel-no-interval', configuration.specification.kind === 'regression'
          ? 'Classical Student-t intervals assume independent, homoskedastic errors. They are not unit-clustered intervals.'
          : 'Cross-fitted score intervals assume independent units and adequate treatment overlap.')
      } else if (configuration.primary === 'did') {
        leave('panel-no-interval', 'This conventional DiD fit reports a point estimate without a confidence interval.')
      } else if (context.panelPreflight.kind === 'ready' && context.panelPreflight.layout.controls.length > context.panelPreflight.layout.treated.length) {
        satisfy('panel-no-interval', '')
      } else {
        leave('panel-no-interval', 'Placebo standard errors need more controls than treated units. The point estimates remain runnable and the result records whether inference was available.')
      }
      break
    }
    case 'poisson-glm':
    case 'negative-binomial-p': {
      const prefix = configuration.kind === 'poisson-glm' ? 'poisson' : 'negbin'
      if (adjustment === null) violate(`${prefix}-identified-adjustment`, 'No measured back-door adjustment set was found, so this count model has no identified set to condition on.')
      else satisfy(`${prefix}-identified-adjustment`, `Identified by back-door adjustment for ${adjustment}.`)
      if (context.outcomeIsCount === null) leave(`${prefix}-count-outcome`, 'The outcome column has not been read yet; it is checked when the run starts.')
      else if (context.outcomeIsCount) satisfy(`${prefix}-count-outcome`, 'Every outcome value is a non-negative integer.')
      else violate(`${prefix}-count-outcome`, 'The outcome holds negative or fractional values, so it is not a count.')
      if (configuration.kind === 'poisson-glm') leave('poisson-dispersion', 'Compare with the negative binomial fit: an alpha well above zero means the Poisson variance assumption fails.')
      else leave('negbin-convergence', 'Convergence is reported with the run; a fit at the alpha boundary is flagged.')
      if (timeSeries) leave(`${prefix}-independence`, 'Rows are a time series; the standard errors assume independent counts.')
      else satisfy(`${prefix}-independence`, 'The prepared dataset holds independent rows.')
      applyLevelRule(`${prefix}-level-stationarity`, context, satisfy, leave, violate)
      break
    }
    case 'negative-binomial-ingarch': {
      if (adjustment === null) violate('ingarch-identified-regressors', 'No measured back-door adjustment set was found for the treatment trajectory.')
      else satisfy('ingarch-identified-regressors', `The treatment and contemporaneous adjustment set are ${adjustment}. The requested future treatment path remains a modelling assumption.`)
      if (!timeSeries) violate('ingarch-regular-count-series', 'The prepared dataset is not a regular time series.')
      else if (context.outcomeIsCount === null) leave('ingarch-regular-count-series', 'The outcome is checked for non-negative integer values when the run starts.')
      else if (context.outcomeIsCount) satisfy('ingarch-regular-count-series', 'The prepared rows are a regular time series and every outcome value is a non-negative integer.')
      else violate('ingarch-regular-count-series', 'The outcome holds negative or fractional values.')
      leave('ingarch-conditional-mean', `Review the selected ${configuration.link === 'identity' ? 'additive' : 'multiplicative'} conditional-mean form, count lags (${configuration.pastObservationLags.join(', ')}), mean lags (${configuration.pastMeanLags.join(', ')}), and contemporaneous regressors.${configuration.link === 'identity' ? ' The run requires their fitted and future values to be non-negative.' : ''}`)
      leave('ingarch-stability', 'The run reports the fitted recursion and forecast path; inspect them for unstable or explosive behaviour.')
      satisfy('ingarch-no-interval', 'This release labels both trajectories as conditional-mean point forecasts and reports no sampling interval.')
      break
    }
    case 'dml-plr':
    case 'dml-irm': {
      const prefix = configuration.kind
      const nuisance = identification.kind === 'identified' ? dmlNuisanceInputs(identification.adjustment.variables, context.study?.estimand ?? null) : []
      if (identification.kind !== 'identified') violate(`${prefix}-identified-adjustment`, 'No measured back-door adjustment set was found, so there is no identified set for the nuisance learners.')
      else if (nuisance.length === 0) violate(`${prefix}-identified-adjustment`, 'The identified adjustment set is empty; double machine learning needs covariates to partial out. Use the adjusted linear regression, or target the effect within groups of a modifier.')
      else satisfy(`${prefix}-identified-adjustment`, `Nuisance learners see ${nuisance.map((variable) => variable.name).join(', ')}.`)
      if (panel) violate(`${prefix}-independent-rows`, 'The rows are a panel; shuffled folds would split a unit across folds, and cross-fitting that keeps each unit together is not available.')
      else if (timeSeries) violate(`${prefix}-independent-rows`, 'The rows are a time series and the folds are shuffled; cross-fitting with time blocks or rolling windows is not available.')
      else satisfy(`${prefix}-independent-rows`, 'The prepared dataset holds independent rows, so shuffled folds are valid.')
      if (configuration.kind === 'dml-irm') {
        if (context.treatmentIsBinary === null) leave('dml-irm-binary-treatment', 'The treatment column has not been read yet; it is checked when the run starts.')
        else if (context.treatmentIsBinary) satisfy('dml-irm-binary-treatment', 'Every treatment value is 0 or 1.')
        else violate('dml-irm-binary-treatment', 'The treatment holds values other than 0 and 1; use the partially linear model.')
      } else {
        leave('dml-plr-partial-linearity', 'Partial linearity in the treatment is assumed; compare with the interactive model when the treatment is binary.')
      }
      leave(`${prefix}-overlap`, 'Inspect treatment overlap against the adjustment variables in Data studio.')
      if (context.study?.estimand.kind === 'conditional-average-treatment-effect') {
        const modifier = context.study.estimand.modifier.name
        leave(`${prefix}-group-effects`, `Assess whether the effect of ${context.study.treatment.name} is linear within each group of ${modifier} (${describeGrouping(context.study.estimand.grouping)}), and whether identification and overlap hold inside every group.`)
      } else {
        satisfy(`${prefix}-group-effects`, 'The study targets one average, so no group effects are estimated.')
      }
      satisfy(`${prefix}-learner-settings`, `The run records 5 folds, 200 trees, minimum leaf 5, learner seed 7, and fold seed ${configuration.seed}.`)
      if (prepared.observations < 100) leave(`${prefix}-interval`, `${prepared.observations} rows is a small sample for random-forest nuisances; read the interval as approximate.`)
      else satisfy(`${prefix}-interval`, `${prepared.observations} rows for the sandwich interval.`)
      break
    }
    case 't-learner': {
      const inputs = identification.kind === 'identified' ? tLearnerInputs(identification.adjustment.variables, context.study?.estimand ?? null) : []
      if (identification.kind !== 'identified') violate('t-learner-identified-adjustment', 'No measured back-door adjustment set was found, so there is no identified set for the outcome forests.')
      else if (inputs.length === 0) violate('t-learner-identified-adjustment', 'The identified adjustment set is empty and the study names no effect modifier, so each row’s effect has nothing to be conditioned on. Name the modifiers in Study design.')
      else satisfy('t-learner-identified-adjustment', `Both forests see ${inputs.map((variable) => variable.name).join(', ')}, and each row’s effect is conditioned on those values.`)
      if (context.treatmentIsBinary === null) leave('t-learner-binary-treatment', 'The treatment column has not been read yet; it is checked when the run starts.')
      else if (context.treatmentIsBinary) satisfy('t-learner-binary-treatment', 'Every treatment value is 0 or 1.')
      else violate('t-learner-binary-treatment', 'The treatment holds values other than 0 and 1; one outcome model per arm needs a binary treatment.')
      if (panel) leave('t-learner-independent-rows', 'Rows repeat within units; the forests treat them as independent draws.')
      else if (timeSeries) leave('t-learner-independent-rows', 'The rows are a time series; the forests treat them as independent draws.')
      else satisfy('t-learner-independent-rows', 'The prepared dataset holds independent rows.')
      leave('t-learner-overlap', 'Inspect treatment overlap against the adjustment variables in Data studio; a row with no nearby rows in one arm carries an extrapolated effect.')
      leave('t-learner-row-effect-reading', 'Each row’s effect is the average for rows with its covariate values; it is not that row’s own observed counterfactual.')
      satisfy('t-learner-learner-settings', `The run records 200 trees, minimum leaf 5, and learner seed ${configuration.seed} for both arms.`)
      satisfy('t-learner-no-interval', configuration.uncertainty.kind === 'none'
        ? 'Uncertainty was not requested. The run reports point estimates.'
        : `Both forests are refitted on ${configuration.uncertainty.samples} resampled datasets. Row intervals are pointwise; the average uses a conservative standard-error bound. Resampling assumes independent observations.`)
      break
    }
    case 'ardl-pss': {
      if (!timeSeries) violate('ardl-time-series', 'An autoregressive distributed lag model needs an ordered time series. This prepared dataset holds independent rows.')
      else if (prepared.observations < 6 * (configuration.maxLag + 1) + 10) violate('ardl-time-series', `${prepared.observations} rows is too few for a maximum lag of ${configuration.maxLag}.`)
      else satisfy('ardl-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series with ${prepared.observations} rows for a maximum lag of ${configuration.maxLag}.`)
      if (adjustment === null) violate('ardl-single-regressor', 'No measured back-door adjustment set was found for this study.')
      else if (identification.kind === 'identified' && identification.adjustment.variables.length > 0) violate('ardl-single-regressor', `The identified adjustment set (${adjustment}) is not empty.`)
      else satisfy('ardl-single-regressor', 'No adjustment is needed, so the treatment is the single exogenous variable.')
      {
        const transformed = transformedStudyVariables(context)
        if (transformed.length > 0) violate('ardl-orders-assessed', `ARDL bounds inference and its long-run coefficient require level variables. This prepared version uses ${transformed.map((variable) => `${variable.transform} for ${variable.name}`).join(', ')}.`)
        else applyOrderRule('ardl-orders-assessed', context, 'i0-or-i1', satisfy, leave, violate)
      }
      leave('ardl-bounds-reading', 'The bounds test is read with the run: only a statistic above the I(1) bound establishes a level relation.')
      satisfy('ardl-deterministic-case', `The run records ${configuration.trend === 'ct' ? 'a constant and trend' : 'a constant'} with Pesaran–Shin–Smith case ${configuration.case}.`)
      break
    }
    case 'vecm': {
      if (!timeSeries) violate('vecm-sample', 'A vector error correction model needs an ordered time series. This prepared dataset holds independent rows.')
      else if (prepared.observations < (configuration.maxLags + 2) * (2 + (identification.kind === 'identified' ? identification.adjustment.variables.length : 0)) * 3 + 10) violate('vecm-sample', `${prepared.observations} rows is too few for the variables at ${configuration.maxLags} lags.`)
      else satisfy('vecm-sample', `Prepared as a regular ${prepared.sampling.frequency} time series; deterministic terms “${configuration.deterministic}” and up to ${configuration.maxLags} lags are recorded.`)
      {
        const transformed = transformedStudyVariables(context)
        if (transformed.length > 0) violate('vecm-all-i1', `VECM estimates cointegration among variables in levels. This prepared version uses ${transformed.map((variable) => `${variable.transform} for ${variable.name}`).join(', ')}.`)
        else applyOrderRule('vecm-all-i1', context, 'i1-only', satisfy, leave, violate)
      }
      leave('vecm-rank', 'The Johansen trace test decides the rank when the run starts; rank zero reports no effect.')
      leave('vecm-single-relation', 'A long-run effect is read only when the rank is one.')
      leave('vecm-no-interval', '')
      break
    }
    case 'synthetic-control': {
      if (!timeSeries) violate('synthetic-panel-layout', 'Synthetic control needs rows ordered by period. This prepared dataset holds independent rows.')
      else satisfy('synthetic-panel-layout', `Rows are a regular ${prepared.sampling.frequency} series; the outcome column is the treated unit and the chosen columns the donors.`)
      if (configuration.donors.length === 0) violate('synthetic-panel-layout', 'Choose at least one donor column.')
      if (configuration.start.kind === 'row') {
        if (configuration.start.row - 1 < 2) violate('synthetic-pre-period', 'At least two pre-intervention rows are needed to fit the weights.')
        else satisfy('synthetic-pre-period', `Intervention at row ${configuration.start.row}: ${configuration.start.row - 1} pre-intervention rows fit the weights; the pre-period loss is reported.`)
      } else leave('synthetic-pre-period', 'The intervention row is read from the treatment column when the run starts: it must be zero before and non-zero after one point.')
      {
        const affected = affectedNodes(context.study).filter((node) => configuration.donors.includes(node.column))
        if (context.study === null) leave('synthetic-donors-untreated', 'No study is loaded, so the donors cannot be checked against the DAG.')
        else if (affected.length > 0) violate('synthetic-donors-untreated', `${affected.map((node) => node.name).join(', ')} ${affected.length === 1 ? 'is a descendant' : 'are descendants'} of ${context.study.treatment.name} in the DAG: a donor the intervention moves absorbs the effect.`)
        else leave('synthetic-donors-untreated', 'No chosen donor is a DAG descendant of the treatment; whether columns outside the DAG were untouched is a judgement recorded with the run.')
      }
      leave('synthetic-convex-hull', 'The pre-period loss is reported with the run; read a large loss as a treated series outside the donors’ reach.')
      if (configuration.donors.length >= 2) {
        leave('synthetic-no-interval', `${configuration.donors.length} donors permit the placebo rank calculation. Cross-fitted inference uses ${configuration.crossFitFolds} folds and prediction bands use alpha ${configuration.alpha}. Each result records whether its sample-size requirement was met.`)
      } else {
        leave('synthetic-no-interval', 'The point estimate can use one donor, but donor-placebo inference needs at least two. Cross-fitted inference and prediction bands remain subject to their pre-period requirements.')
      }
      break
    }
    case 'negbin-nuts': {
      if (adjustment === null) violate('nuts-model-shape', 'No measured back-door adjustment set was found, so this one-confounder model has no identified covariate.')
      else if (identification.kind === 'identified' && identification.adjustment.variables.length !== 1) violate('nuts-model-shape', `The identified set holds ${identification.adjustment.variables.length}.`)
      else satisfy('nuts-model-shape', `One confounder, ${adjustment}, enters the linear predictor beside the treatment.`)
      if (context.outcomeIsCount === null) leave('nuts-count-outcome', 'The outcome column has not been read yet; it is checked when the run starts.')
      else if (context.outcomeIsCount) satisfy('nuts-count-outcome', 'Every outcome value is a non-negative integer.')
      else violate('nuts-count-outcome', 'The outcome holds negative or fractional values, so it is not a count.')
      leave('nuts-convergence', `Divergences and the acceptance rate are reported with the run; warmup ${configuration.warmup}, draws ${configuration.samples}, seed ${configuration.seed}.`)
      if (panel) leave('nuts-independence', 'Rows are a panel; the model assumes independent counts.')
      else if (timeSeries) leave('nuts-independence', 'Rows are a time series; the model assumes independent counts.')
      else satisfy('nuts-independence', 'The prepared dataset holds independent rows.')
      break
    }
    case 'bayesian-gaussian': {
      if (adjustment === null) violate('bayes-gaussian-identified-adjustment', 'No measured back-door adjustment set was found, so the regression has no identified set to condition on.')
      else satisfy('bayes-gaussian-identified-adjustment', `Identified by back-door adjustment for ${adjustment}.`)
      if (context.treatmentIsBinary === null) leave('bayes-gaussian-binary-treatment', 'The treatment column has not been read yet; it is checked before the estimator runs.')
      else if (context.treatmentIsBinary) satisfy('bayes-gaussian-binary-treatment', 'Every treatment value is 0 or 1.')
      else violate('bayes-gaussian-binary-treatment', 'The treatment holds values other than 0 and 1, so do(0) versus do(1) is not the recorded treatment contrast.')
      leave('bayes-gaussian-prior-scale', 'Slope priors are Normal(0, 1) and the residual scale prior is half-normal(10). Non-binary adjustment columns are standardised, but the outcome keeps its units: on a scale where plausible effects lie far outside ±2, the prior pulls the estimate toward zero.')
      leave('bayes-gaussian-convergence', `Divergences and the acceptance rate are reported with the run; warmup ${configuration.warmup} and ${configuration.samples} draws in each of 3 chains, seed ${configuration.seed}.`)
      if (panel) leave('bayes-gaussian-independence', 'Rows are a panel; the model assumes independent rows.')
      else if (timeSeries) leave('bayes-gaussian-independence', 'Rows are a time series; the model assumes independent rows.')
      else satisfy('bayes-gaussian-independence', 'The prepared dataset holds independent rows.')
      break
    }
    case 'discrete-bn-query': {
      if (context.study === null) leave('bn-observed-graph', 'No study is loaded, so the graph cannot be checked.')
      else {
        const latent = context.study.graph.nodes.filter((node) => node.column === null)
        if (latent.length > 0) violate('bn-observed-graph', `${latent.map((node) => node.name).join(', ')} ${latent.length === 1 ? 'is' : 'are'} unmeasured; the network needs every node in the data.`)
        else satisfy('bn-observed-graph', `All ${context.study.graph.nodes.length} DAG nodes are measured; the query adjusts for the treatment’s parents.`)
      }
      satisfy('bn-discretisation', `Each variable has a budget of ${configuration.bins} states. Observed low-cardinality states are preserved; higher-cardinality values are divided at quantiles. The run records the mean value represented by each state.`)
      leave('bn-sample-per-cell', `${prepared.observations} total rows do not establish support in every parent configuration. Cell counts are not reported in this release; sparse cells receive BDeu pseudo-counts with equivalent sample size ${configuration.equivalentSampleSize}.`)
      if (timeSeries) violate('bn-independent-rows', 'The prepared rows are a time series; this discrete network has no lag or serial-dependence model.')
      else if (panel) violate('bn-independent-rows', 'The prepared rows repeat units through time; this discrete network has no unit or serial-dependence model.')
      else leave('bn-independent-rows', 'Cross-sectional structure does not by itself establish independent sampling. Confirm that clustering or repeated observations are absent.')
      break
    }
    case 'binary-ett-idc-star': {
      if (identification.kind === 'counterfactually-identified') satisfy('ett-identified-expression', 'IDC* identified both conditional potential-outcome distributions in this record.')
      else violate('ett-identified-expression', 'This identification record does not contain the two IDC* expressions for binary ETT.')
      if (context.observedGraphIsBinary === null) leave('ett-binary-table', 'The observed graph columns have not been read yet.')
      else if (context.observedGraphIsBinary) satisfy('ett-binary-table', 'Every observed graph variable contains only 0 and 1.')
      else violate('ett-binary-table', 'At least one observed graph variable contains a value other than 0 or 1; no discretisation is applied.')
      leave('ett-positive-conditioning-mass', 'The run evaluates every conditional denominator and refuses zero observed mass.')
      if (timeSeries) violate('ett-independent-rows', 'The prepared rows are a time series; the empirical table would count serially dependent rows as independent.')
      else if (panel) violate('ett-independent-rows', 'The prepared rows repeat units; the empirical table would count dependent rows as independent.')
      else leave('ett-independent-rows', 'Confirm that the cross-sectional rows are independently sampled.')
      satisfy('ett-no-interval', 'The run is recorded as a point estimate with no sampling interval.')
      break
    }
    case 'causal-effects-total': {
      if (!timeSeries) violate('causal-effects-time-series', 'A time-series graph needs a regular time series. This prepared dataset holds independent rows.')
      else satisfy('causal-effects-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series.`)
      if (context.document === null) violate('causal-effects-stationary-dag', 'No DAG document is bound to the study.')
      else if (context.document.current.validation.kind !== 'structurally-valid') violate('causal-effects-stationary-dag', 'The bound DAG revision is not structurally valid.')
      else satisfy('causal-effects-stationary-dag', `Stationary DAG read from “${context.document.name}” with its lagged arrows kept.`)
      if (context.stationarity === null) leave('causal-effects-stationarity', 'Run stationarity tests for this prepared dataset version in Data studio.')
      else {
        const verdicts = context.stationarity.variables.map((variable) => levelModelVerdict(context.document?.current.graph.nodes.find((node) => node.kind === 'observed' && node.column === variable.column)?.name ?? variable.column, variable.assessment))
        const review = verdicts.filter((verdict) => verdict.kind !== 'allowed')
        if (review.length > 0) leave('causal-effects-stationarity', `${review.map((verdict) => verdict.reason).join(' ')} CausalEffects assumes a stationary temporal graph; continuing keeps the prepared values unchanged and records this conflict with the run.`)
        else satisfy('causal-effects-stationarity', `All ${verdicts.length} prepared series are stationary in levels.`)
      }
      if (configuration.estimator.kind === 'wrightParents') {
        leave('causal-effects-identifiable', 'The run checks whether the graph contains directed treatment–outcome paths that Wright path tracing can evaluate.')
      } else if (configuration.estimator.adjustment.kind === 'explicit') {
        leave('causal-effects-identifiable', 'The run checks the supplied time-indexed set against every open non-causal treatment–outcome path and refuses an invalid set.')
      } else {
        leave('causal-effects-identifiable', `Whether the ${configuration.estimator.adjustment.kind === 'optimal' ? 'complete O-set' : configuration.estimator.adjustment.kind === 'minimizedOptimal' ? 'minimized O-set' : 'collider-minimized O-set'} exists is decided by the run; a refusal is reported as not identifiable.`)
      }
      switch (configuration.estimator.kind) {
        case 'linear':
          satisfy('causal-effects-functional-form', 'Linear outcome regression with the selected time-indexed adjustment set.')
          break
        case 'knn':
          leave('causal-effects-functional-form', `k-nearest neighbours with k = ${configuration.estimator.k}; confirm that this local model is suitable for the response surface.`)
          break
        case 'wrightParents':
          leave('causal-effects-functional-form', 'Wright path tracing fits one linear parent regression per structural equation. Review the linearity and directed-graph assumptions before interpreting the path decomposition.')
          break
        default:
          assertNever(configuration.estimator)
      }
      if (configuration.uncertainty.kind === 'none') leave('causal-effects-bootstrap', 'This run does not request a sampling interval.')
      else satisfy('causal-effects-bootstrap', '')
      break
    }
    case 'causal-impact': {
      if (!timeSeries) violate('impact-time-series', 'An intervention analysis needs an ordered time series. This prepared dataset holds independent rows.')
      else satisfy('impact-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series.`)
      if (configuration.inference !== undefined && !bayesianImpactSettingsSchema.safeParse(configuration.inference).success) violate('impact-pre-period', 'Review the sampling settings and the selected model. Seasonal counts and durations must be positive; harmonic pairs must be below half the period.')
      if (configuration.start.kind === 'row') satisfy('impact-intervention-time', `Intervention starts at row ${configuration.start.row}.`)
      else leave('impact-intervention-time', 'The intervention start is read from the treatment column when the run starts: it must be zero before and non-zero after one point.')
      leave('impact-pre-period', 'The pre-period length is checked at run time; at least 8 rows are required and more is better.')
      if (configuration.controls.length === 0) leave('impact-controls', 'Choose control series, or accept a counterfactual based only on the selected state model.')
      else satisfy('impact-controls', `${configuration.controls.length} control series enter the static regression.`)
      {
        const affected = affectedNodes(context.study).filter((node) => configuration.controls.includes(node.column))
        if (context.study === null) leave('impact-controls-unaffected', 'No study is loaded, so the controls cannot be checked against the DAG.')
        else if (affected.length > 0) violate('impact-controls-unaffected', `${affected.map((node) => node.name).join(', ')} ${affected.length === 1 ? 'is a descendant' : 'are descendants'} of ${context.study.treatment.name} in the DAG: a control the intervention moves absorbs the effect. Untick ${affected.length === 1 ? 'it' : 'them'}.`)
        else if (configuration.controls.length === 0) satisfy('impact-controls-unaffected', 'No controls are used.')
        else leave('impact-controls-unaffected', 'No chosen control is a DAG descendant of the treatment; columns outside the DAG remain a judgement recorded with the run.')
      }
      break
    }
    default:
      return assertNever(configuration)
  }
  return verdict(satisfied, unresolved, violations)
}

export const additive = { kind: 'additive', unit: '' } as const

/** Columns of DAG nodes the treatment reaches, with their names; empty without a study. */
function affectedNodes(study: StudySpecification | null): readonly { readonly column: ColumnId; readonly name: string }[] {
  if (study === null) return []
  const descendants = treatmentDescendants(study)
  return study.graph.nodes.flatMap((node) => (node.column !== null && descendants.has(node.node) ? [{ column: node.column, name: node.name }] : []))
}

const affectedColumns = (study: StudySpecification | null): ReadonlySet<ColumnId> => new Set(affectedNodes(study).map((node) => node.column))

/** A group's label from its bounds: the level itself, or the quantile band it spans. */
export const groupLabel = (group: { readonly lower: number | null; readonly upper: number | null }, grouping: ModifierGrouping): string => {
  const text = (value: number) => formatStatistic('raw', value).text
  switch (grouping.kind) {
    case 'levels': return group.lower === null ? 'level' : text(group.lower)
    case 'quantiles':
      if (group.lower === null && group.upper !== null) return `≤ ${text(group.upper)}`
      if (group.upper === null && group.lower !== null) return `> ${text(group.lower)}`
      return group.lower === null || group.upper === null ? 'all' : `${text(group.lower)} to ${text(group.upper)}`
    default: return assertNever(grouping)
  }
}

/** The DML effect in the shape the study asked for: the plain average, or one effect per modifier group. */
const groupedEffectFrom = (study: StudySpecification, evidence: DoubleMlEvidence): CausalEstimate['effect'] | null => {
  if (study.estimand.kind !== 'conditional-average-treatment-effect') {
    return evidence.groups.kind === 'none' ? { kind: 'additive', value: evidence.estimate, unit: '' } : null
  }
  if (evidence.groups.kind !== 'grouped') return null
  const grouping = study.estimand.grouping
  if (evidence.groups.grouping.kind !== grouping.kind || (grouping.kind === 'quantiles' && evidence.groups.grouping.kind === 'quantiles' && evidence.groups.grouping.bins !== grouping.bins)) return null
  const groups = evidence.groups.groups.map((group): GroupEffectEstimate => ({
    label: groupLabel(group, grouping),
    lower: group.lower,
    upper: group.upper,
    value: group.effect,
    standardError: group.standardError,
    interval: { level: evidence.level, lower: group.interval[0], upper: group.interval[1] },
    observations: group.observations,
    fewObservations: group.fewObservations,
  }))
  return isNonEmpty(groups) ? { kind: 'byGroup', modifier: study.estimand.modifier.name, overall: evidence.estimate, groups } : null
}

/** The typed estimate from each façade's evidence; the interval follows the configuration. */
export function causalEstimateFrom(
  study: StudySpecification,
  identification: IdentificationArtifact,
  run:
    | { readonly kind: 'sharp-rd-run'; readonly configuration: SharpRdConfiguration; readonly evidence: SharpRdEvidence }
    | { readonly kind: 'backdoor-linear-run'; readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }
    | { readonly kind: 'frontdoor-two-stage-run'; readonly configuration: FrontdoorTwoStageConfiguration; readonly evidence: FrontdoorTwoStageEvidence }
    | { readonly kind: 'instrumental-variable-run'; readonly configuration: InstrumentalVariableConfiguration; readonly evidence: InstrumentalVariableEvidence }
    | { readonly kind: 'count-glm-run'; readonly configuration: CountGlmConfiguration; readonly evidence: CountGlmEvidence }
    | { readonly kind: 'negative-binomial-ingarch-run'; readonly configuration: NegativeBinomialIngarchConfiguration; readonly evidence: NegativeBinomialIngarchEvidence }
    | { readonly kind: 'double-ml-run'; readonly configuration: DoubleMlConfiguration; readonly evidence: DoubleMlEvidence }
    | { readonly kind: 't-learner-run'; readonly configuration: TLearnerConfiguration; readonly evidence: TLearnerEvidence }
    | { readonly kind: 'ardl-run'; readonly configuration: ArdlConfiguration; readonly evidence: ArdlEvidence }
    | { readonly kind: 'vecm-run'; readonly configuration: VecmConfiguration; readonly evidence: VecmEvidence }
    | { readonly kind: 'synthetic-control-run'; readonly configuration: SyntheticControlConfiguration; readonly evidence: SyntheticControlEvidence }
    | { readonly kind: 'panel-intervention-run'; readonly configuration: PanelInterventionConfiguration; readonly evidence: PanelInterventionEvidence }
    | { readonly kind: 'negbin-nuts-run'; readonly configuration: NegbinNutsConfiguration; readonly evidence: NegbinNutsEvidence }
    | { readonly kind: 'bayesian-gaussian-run'; readonly configuration: BayesianGaussianConfiguration; readonly evidence: BayesianGaussianEvidence }
    | { readonly kind: 'discrete-bn-run'; readonly configuration: DiscreteBnConfiguration; readonly evidence: DiscreteBnEvidence }
    | { readonly kind: 'binary-ett-run'; readonly configuration: BinaryEttConfiguration; readonly evidence: BinaryEttEvidence }
    | { readonly kind: 'causal-effects-run'; readonly configuration: CausalEffectsConfiguration; readonly evidence: CausalEffectsEvidence; readonly graphVariables: readonly (StudyVariable | null)[] }
    | { readonly kind: 'causal-impact-run'; readonly configuration: CausalImpactConfiguration; readonly evidence: CausalImpactEvidence },
): CausalEstimate | null {
  if (run.kind === 'sharp-rd-run') {
    if (study.estimand.kind !== 'local-cutoff-effect' || identification.result.kind !== 'cutoff-design' || study.estimand.cutoff !== run.evidence.cutoff) return null
    const { robust, observations } = run.evidence
    return { kind: 'causal-estimate', estimand: study.estimand,
      effect: { kind: 'additive', value: robust.value, unit: '' },
      interval: { kind: 'confidence', level: 0.95, lower: robust.interval[0], upper: robust.interval[1] },
      standardError: robust.standardError, adjustment: { kind: 'none' },
      sample: { observations: observations[0] + observations[1], parameters: 4, degreesOfFreedom: null },
    }
  }
  if (run.kind === 'frontdoor-two-stage-run') {
    if (identification.result.kind !== 'graphically-identified' || identification.result.frontdoor.kind !== 'identified' || identification.result.frontdoor.mediators.length !== 1) return null
    const interval = run.evidence.uncertainty
    return {
      kind: 'causal-estimate',
      estimand: study.estimand,
      effect: { kind: 'additive', value: run.evidence.estimate, unit: '' },
      interval: interval.kind === 'bootstrap'
        ? { kind: 'confidence', level: interval.confidenceLevel, lower: interval.interval[0], upper: interval.interval[1] }
        : { kind: 'none', reason: 'This run did not request bootstrap uncertainty.' },
      standardError: null,
      adjustment: { kind: 'none' },
      sample: {
        observations: run.evidence.observations,
        parameters: Math.max(run.evidence.firstStageParams.length, run.evidence.secondStageParams.length),
        degreesOfFreedom: null,
      },
    }
  }
  if (run.kind === 'instrumental-variable-run') {
    const instruments = identifiedInstruments(identification.result)
    if (instruments === null || instruments.length !== run.evidence.instruments.length) return null
    const interval = run.evidence.uncertainty
    return {
      kind: 'causal-estimate',
      estimand: study.estimand,
      effect: { kind: 'additive', value: run.evidence.estimate, unit: '' },
      interval: interval.kind === 'bootstrap'
        ? { kind: 'confidence', level: interval.confidenceLevel, lower: interval.interval[0], upper: interval.interval[1] }
        : { kind: 'none', reason: 'This run did not request bootstrap uncertainty.' },
      standardError: run.evidence.standardError,
      adjustment: { kind: 'none' },
      sample: {
        observations: run.evidence.observations,
        parameters: run.evidence.params.length,
        degreesOfFreedom: null,
      },
    }
  }
  if (run.kind === 'binary-ett-run') {
    if (
      identification.result.kind !== 'counterfactually-identified'
      || study.estimand.kind !== 'average-treatment-effect-on-treated'
      || run.evidence.treatedExpression !== identification.result.treatedExpression
      || run.evidence.untreatedExpression !== identification.result.untreatedExpression
    ) return null
    return {
      kind: 'causal-estimate',
      estimand: study.estimand,
      effect: { kind: 'additive', value: run.evidence.effectOnTreated, unit: '' },
      interval: { kind: 'none', reason: 'This empirical IDC* evaluator reports a plug-in point estimate without a sampling interval.' },
      standardError: null,
      adjustment: { kind: 'none' },
      sample: { observations: run.evidence.observations, parameters: 0, degreesOfFreedom: null },
    }
  }
  if (identification.result.kind !== 'identified') return null
  const adjustmentSet = identification.result.adjustment.variables
  const adjustment = appliedContemporaneousAdjustment(adjustmentSet)
  switch (run.kind) {
    case 'backdoor-linear-run': {
      const { evidence } = run
      const reading = linearReading(run)
      if (reading === null) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: reading.estimate, unit: '' },
        interval: { kind: 'confidence', level: evidence.level, lower: reading.interval[0], upper: reading.interval[1] },
        standardError: reading.standardError,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.parameters, degreesOfFreedom: evidence.degreesOfFreedom },
      }
    }
    case 'count-glm-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'expectedCountRatio', value: evidence.incidenceRateRatio },
        interval: { kind: 'confidence', level: evidence.level, lower: evidence.incidenceRateRatioInterval[0], upper: evidence.incidenceRateRatioInterval[1] },
        standardError: evidence.standardError,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.parameters, degreesOfFreedom: evidence.degreesOfFreedom },
      }
    }
    case 'negative-binomial-ingarch-run': {
      const { evidence } = run
      const points = evidence.effectPath.map((effect, index): TimeEffectPoint => ({
        step: index + 1,
        actual: evidence.interventionMean[index] ?? Number.NaN,
        counterfactual: evidence.baselineMean[index] ?? Number.NaN,
        lower: evidence.baselineMean[index] ?? Number.NaN,
        upper: evidence.baselineMean[index] ?? Number.NaN,
        effect,
      }))
      if (!isNonEmpty(points)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'path', values: points, aggregate: { cumulative: evidence.cumulativeEffect, average: evidence.averageEffect } },
        interval: { kind: 'none', reason: 'The INGARCH run reports conditional-mean scenario paths without a sampling interval.' },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.parameters.length, degreesOfFreedom: null },
      }
    }
    case 'double-ml-run': {
      const { evidence } = run
      const studyTargetsAtt = study.estimand.kind === 'average-treatment-effect-on-treated'
      if (evidence.att !== studyTargetsAtt || run.configuration.att !== studyTargetsAtt) return null
      const effect = groupedEffectFrom(study, evidence)
      if (effect === null) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect,
        interval: { kind: 'confidence', level: evidence.level, lower: evidence.interval[0], upper: evidence.interval[1] },
        standardError: evidence.standardError,
        adjustment,
        sample: { observations: evidence.observations, parameters: 1 + adjustmentSet.length, degreesOfFreedom: null },
      }
    }
    case 't-learner-run': {
      // The T-learner reports one effect per row and nothing else, so it binds only to the per-row target.
      if (study.estimand.kind !== 'conditional-average-treatment-effect-per-row') return null
      const { evidence } = run
      if (!isNonEmpty(evidence.effects) || evidence.effects.length !== evidence.observations) return null
      if (!matchesTLearnerUncertainty(run.configuration.uncertainty, evidence.uncertainty)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'perRow', overall: evidence.average, effects: evidence.effects },
        interval: evidence.uncertainty.kind === 'none'
          ? { kind: 'none', reason: 'Uncertainty was not requested. Choose bootstrap intervals to estimate it.' }
          : { kind: 'confidence', level: evidence.uncertainty.level, lower: evidence.uncertainty.average.interval[0], upper: evidence.uncertainty.average.interval[1] },
        standardError: evidence.uncertainty.kind === 'none' ? null : evidence.uncertainty.average.standardErrorBound,
        adjustment,
        sample: { observations: evidence.observations, parameters: 0, degreesOfFreedom: null },
      }
    }
    case 'ardl-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.longRunEffect, unit: '' },
        interval: { kind: 'confidence', level: evidence.level, lower: evidence.interval[0], upper: evidence.interval[1] },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.arLag + evidence.dlLag + 2, degreesOfFreedom: null },
      }
    }
    case 'vecm-run': {
      const { evidence } = run
      if (evidence.longRunEffect === null) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.longRunEffect, unit: '' },
        interval: { kind: 'none', reason: 'No standard error is reported for the long-run vector.' },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.beta.length * evidence.rank, degreesOfFreedom: null },
      }
    }
    case 'synthetic-control-run': {
      const { evidence } = run
      const predictionBand = evidence.conformalBand.kind === 'available'
        ? evidence.conformalBand
        : evidence.gaussianBand.kind === 'available' ? evidence.gaussianBand : null
      const points = evidence.postGap.map((effect, index): TimeEffectPoint => {
        const counterfactual = evidence.synthetic[evidence.nPre + index] ?? Number.NaN
        const interval = predictionBand?.intervals[evidence.nPre + index]
        return { step: evidence.nPre + index + 1, actual: counterfactual + effect, counterfactual, lower: interval?.[0] ?? counterfactual, upper: interval?.[1] ?? counterfactual, effect }
      })
      if (!isNonEmpty(points)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'path', values: points, aggregate: { cumulative: evidence.postGap.reduce((sum, value) => sum + value, 0), average: evidence.att } },
        interval: { kind: 'none', reason: 'Cross-fitted confidence inference, the donor-placebo rank test, and fixed-weight prediction bands are reported with this run; they answer different uncertainty questions and are not combined into one interval.' },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.weights.length, degreesOfFreedom: null },
      }
    }
    case 'panel-intervention-run': {
      const { evidence } = run
      if (run.configuration.primary === 'adjusted') {
        if (study.estimand.kind !== 'average-treatment-effect-on-treated' || evidence.kind !== 'panelAdjusted'
          || !sameDidSpecification(run.configuration.specification, evidence.specification) || run.configuration.covariates.length !== evidence.covariates) return null
        return { kind: 'causal-estimate', estimand: study.estimand, effect: { kind: 'additive', value: evidence.estimate, unit: '' },
          interval: { kind: 'confidence', level: 0.95, lower: evidence.interval[0], upper: evidence.interval[1] }, standardError: evidence.standardError,
          adjustment: { kind: 'none' }, sample: { observations: evidence.observations, parameters: evidence.inference.kind === 'independentErrors' ? 4+evidence.covariates : evidence.covariates, degreesOfFreedom: evidence.inference.kind === 'independentErrors' ? evidence.inference.degreesOfFreedom : null } }
      }
      if (evidence.kind === 'panelAdjusted') return null
      if ((run.configuration.primary === 'did') !== (evidence.kind === 'panelDid')) return null
      if (evidence.kind === 'panelDid') return {
        kind: 'causal-estimate', estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.did.estimate, unit: '' },
        interval: { kind: 'none', reason: 'This conventional DiD result does not yet report an uncertainty interval.' },
        standardError: null, adjustment: { kind: 'none' },
        sample: { observations: evidence.observations, parameters: 4, degreesOfFreedom: null },
      }
      const placeboStandardError = evidence.syntheticDidPlacebo.kind === 'available' ? evidence.syntheticDidPlacebo.standardError : null
      return {
        kind: 'causal-estimate', estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.syntheticDid.estimate, unit: '' },
        interval: { kind: 'none', reason: 'No confidence interval is shown for this result.' },
        standardError: placeboStandardError,
        adjustment: { kind: 'none' },
        sample: { observations: evidence.observations, parameters: evidence.syntheticDid.lambda.length + evidence.syntheticDid.omega.length, degreesOfFreedom: null },
      }
    }
    case 'negbin-nuts-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'expectedCountRatio', value: evidence.irrMedian },
        interval: { kind: 'credible', level: 0.95, summary: 'ETI', lower: evidence.irrLower, upper: evidence.irrUpper },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: 4, degreesOfFreedom: null },
      }
    }
    case 'bayesian-gaussian-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.effectMean, unit: '' },
        interval: { kind: 'credible', level: 0.94, summary: 'HDI', lower: evidence.hdiLower, upper: evidence.hdiUpper },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: adjustmentSet.length + 3, degreesOfFreedom: null },
      }
    }
    case 'discrete-bn-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.effect, unit: '' },
        interval: { kind: 'none', reason: 'The do-query reports expectations from the fitted network; no posterior or sampling interval is available.' },
        standardError: null,
        adjustment,
        sample: { observations: evidence.observations, parameters: evidence.stateCounts.reduce((sum, count) => sum + count, 0), degreesOfFreedom: null },
      }
    }
    case 'causal-effects-run': {
      const { evidence } = run
      if (!evidence.identifiable || evidence.totalEffect === null) return null
      if (!causalEffectsUncertaintyMatches(run.configuration.uncertainty, evidence.uncertainty)) return null
      if (!causalEffectsFitMatches(run.configuration.estimator, evidence.fit)) return null
      let applied: AppliedAdjustment
      let parameters: number
      switch (evidence.fit.kind) {
        case 'unfitted': return null
        case 'invalidAdjustment': return null
        case 'adjustedLinear':
        case 'adjustedKnn': {
          const temporal = appliedTimeIndexedAdjustment(run.graphVariables, evidence.fit.adjustmentSet)
          if (temporal === null) return null
          applied = temporal
          parameters = evidence.fit.adjustmentSet.length + 1
          break
        }
        case 'wrightParents':
          applied = { kind: 'structural-parent-model', coefficients: evidence.fit.coefficients.length, paths: evidence.fit.paths.length }
          parameters = evidence.fit.coefficients.length
          break
        default:
          return assertNever(evidence.fit)
      }
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.totalEffect, unit: '' },
        interval: evidence.uncertainty.kind === 'bootstrap'
          ? { kind: 'confidence', level: evidence.uncertainty.confidenceLevel, lower: evidence.uncertainty.effectInterval[0], upper: evidence.uncertainty.effectInterval[1] }
          : { kind: 'none', reason: 'This run did not request bootstrap uncertainty.' },
        standardError: null,
        adjustment: applied,
        sample: { observations: evidence.fittedObservations, parameters, degreesOfFreedom: null },
      }
    }
    case 'causal-impact-run': {
      const { evidence } = run
      if (!impactInferenceMatches(run.configuration, evidence)) return null
      const z = 1.959963984540054
      const points = evidence.pointwise.map((effect, index): TimeEffectPoint => ({
        step: evidence.nPre + index + 1,
        actual: evidence.counterfactual[index] + effect,
        counterfactual: evidence.counterfactual[index],
        lower: evidence.kind !== 'causalImpact' ? evidence.counterfactualLower[index] : evidence.counterfactual[index] - z * evidence.counterfactualSe[index],
        upper: evidence.kind !== 'causalImpact' ? evidence.counterfactualUpper[index] : evidence.counterfactual[index] + z * evidence.counterfactualSe[index],
        effect,
      }))
      if (!isNonEmpty(points)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'path', values: points, aggregate: { cumulative: evidence.cumulative, average: evidence.average } },
        interval: evidence.kind !== 'causalImpact'
          ? { kind: 'credible', level: evidence.level, summary: 'ETI', lower: evidence.cumulativeSummary.absolute.lower, upper: evidence.cumulativeSummary.absolute.upper }
          : { kind: 'none', reason: 'The shaded range is a pointwise forecast band for each no-intervention period. It is not a confidence interval for the average or cumulative difference.' },
        standardError: null,
        adjustment: { kind: 'none' },
        sample: { observations: evidence.observations, parameters: evidence.kind === 'causalImpact' ? evidence.params.length : evidence.kind === 'bayesianCausalImpact' ? evidence.controls.length + (evidence.controls.length > 0 ? 1 : 0) + 2 : evidence.controls.length + ({ level:2, linear:3, semilocal:5 }[evidence.model.trend]) + (evidence.model.seasonality.kind === 'none' ? 0 : 1), degreesOfFreedom: null },
      }
    }
    default:
      return assertNever(run)
  }
}

/** The fitted window as chart points, so the counterfactual can be seen tracking the outcome
 * before the intervention. Row indices arrive zero-based and are reported as steps. */
export function preInterventionPoints(path: PreInterventionPath): readonly TimeEffectPoint[] {
  if (path.kind === 'notReported') return []
  const z = 1.959963984540054
  return path.steps.map((step, index): TimeEffectPoint => {
    const counterfactual = path.counterfactual[index]
    const margin = path.kind === 'fitted' ? z * path.se[index] : 0
    return {
      step: step + 1,
      actual: path.observed[index],
      counterfactual,
      lower: path.kind === 'fitted' ? counterfactual - margin : path.lower[index],
      upper: path.kind === 'fitted' ? counterfactual + margin : path.upper[index],
      effect: path.observed[index] - counterfactual,
    }
  })
}

export type EffectBand =
  | { readonly kind: 'none' }
  | { readonly kind: 'interval'; readonly lower: number; readonly upper: number }

export interface ImpactEffectPoint {
  readonly step: number
  readonly effect: number
  readonly band: EffectBand
}

const NORMAL_95 = 1.959963984540054
const interval = (lower: number, upper: number): EffectBand => ({ kind: 'interval', lower, upper })

/** The fitted window's differences. The outcome is fixed, so the band on the difference is the
 * band on the counterfactual reflected about it. */
const preInterventionEffects = (path: PreInterventionPath): readonly ImpactEffectPoint[] => {
  if (path.kind === 'notReported') return []
  return path.steps.map((step, index): ImpactEffectPoint => {
    const effect = path.observed[index] - path.counterfactual[index]
    return {
      step: step + 1,
      effect,
      band: path.kind === 'fitted'
        ? interval(effect - NORMAL_95 * path.se[index], effect + NORMAL_95 * path.se[index])
        : interval(path.observed[index] - path.upper[index], path.observed[index] - path.lower[index]),
    }
  })
}

/** The difference at each row, across the fitted window and the evaluated one. */
export function impactPointwisePath(evidence: CausalImpactEvidence): readonly ImpactEffectPoint[] {
  const after = evidence.pointwise.map((effect, index): ImpactEffectPoint => ({
    step: evidence.nPre + index + 1,
    effect,
    band: evidence.kind === 'causalImpact'
      ? interval(effect - NORMAL_95 * evidence.counterfactualSe[index], effect + NORMAL_95 * evidence.counterfactualSe[index])
      : interval(evidence.pointwiseLower[index], evidence.pointwiseUpper[index]),
  }))
  return [...preInterventionEffects(evidence.preInterventionPath), ...after]
}

/** The differences accumulated from the intervention. Before it the total is zero by
 * construction, not by estimation, so it carries no band. The least-squares route carries none
 * either: its forecast errors share the level's uncertainty, so summing their variances would
 * understate the range. */
export function impactCumulativePath(evidence: CausalImpactEvidence): readonly ImpactEffectPoint[] {
  let total = 0
  const after = evidence.pointwise.map((effect, index): ImpactEffectPoint => {
    total += effect
    return {
      step: evidence.nPre + index + 1,
      effect: total,
      band: evidence.kind === 'causalImpact'
        ? { kind: 'none' }
        : interval(evidence.cumulativeLower[index], evidence.cumulativeUpper[index]),
    }
  })
  const before = preInterventionEffects(evidence.preInterventionPath)
    .map((point): ImpactEffectPoint => ({ step: point.step, effect: 0, band: { kind: 'none' } }))
  return [...before, ...after]
}

export function describeCovariance(errors: LinearErrors): string {
  switch (errors.kind) {
    case 'hac': return 'Newey–West HAC'
    case 'classical': return 'Classical'
    case 'arma': return `ARMA(${errors.p}, ${errors.q}) errors`
    default: return assertNever(errors)
  }
}

/** The coefficient, its standard error and interval under the configured error treatment; null when the evidence holds no ARMA fit for an ARMA configuration. */
export function linearReading(run: { readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }): { readonly estimate: number; readonly standardError: number; readonly interval: readonly [number, number] } | null {
  const { evidence } = run
  switch (run.configuration.errors.kind) {
    case 'classical': return { estimate: evidence.estimate, standardError: evidence.standardError, interval: evidence.interval }
    case 'hac': return { estimate: evidence.estimate, standardError: evidence.hacStandardError, interval: evidence.hacInterval }
    case 'arma': {
      const reading = armaReading(run)
      return reading === null ? null : { estimate: reading.estimate, standardError: reading.standardError, interval: reading.interval }
    }
    default: return assertNever(run.configuration.errors)
  }
}

/** The estimator the chapter opens with for a record: the one its strategy calls for, else the plain adjustment route for the row structure. */
export function defaultEstimatorFor(identification: Identification | null, prepared: PreparedDatasetArtifact, study: StudySpecification | null): EstimatorId {
  if (study?.estimand.kind === 'local-cutoff-effect') return 'sharp-rd'
  // A conditional target is reported only by the DML estimators; the partially linear one runs for any treatment.
  if (study?.estimand.kind === 'conditional-average-treatment-effect') return 'dml-plr'
  // The per-row target is reported only by the T-learner.
  if (study?.estimand.kind === 'conditional-average-treatment-effect-per-row') return 't-learner'
  const fallback: EstimatorId = prepared.kind === 'prepared-panel' ? 'panel-intervention' : 'backdoor-linear-regression'
  if (identification === null) return fallback
  switch (identification.kind) {
    case 'cutoff-design': return 'sharp-rd'
    case 'instrument-identified': return 'instrumental-variable'
    case 'graphically-identified': return identification.frontdoor.kind === 'identified' ? 'frontdoor-two-stage' : 'instrumental-variable'
    case 'counterfactually-identified': return 'binary-ett-idc-star'
    case 'identified':
    case 'backdoor-not-identified':
      return fallback
    default: return assertNever(identification)
  }
}

export function describeEstimator(estimator: EstimatorId): string {
  switch (estimator) {
    case 'backdoor-linear-regression': return 'Adjusted linear regression'
    case 'frontdoor-two-stage': return 'Linear front-door regression'
    case 'instrumental-variable': return 'Instrumental variable'
    case 'poisson-glm': return 'Poisson GLM'
    case 'negative-binomial-p': return 'Negative binomial'
    case 'negative-binomial-ingarch': return 'Negative-binomial INGARCH'
    case 'dml-plr': return 'Double machine learning, partially linear'
    case 'dml-irm': return 'Double machine learning, interactive'
    case 't-learner': return 'T-learner'
    case 'ardl-pss': return 'ARDL long run'
    case 'vecm': return 'VECM'
    case 'synthetic-control': return 'Synthetic control'
    case 'sharp-rd': return 'Sharp regression discontinuity'
    case 'panel-intervention': return 'Panel difference-in-differences'
    case 'negbin-nuts': return 'Bayesian negative binomial'
    case 'bayesian-gaussian': return 'Bayesian Gaussian regression'
    case 'discrete-bn-query': return 'Discrete BN do-query'
    case 'binary-ett-idc-star': return 'Binary ETT by IDC*'
    case 'causal-effects-total': return 'CausalEffects total effect'
    case 'causal-impact': return 'Causal impact'
    default: return assertNever(estimator)
  }
}

/** Tigramite marks for a DAG document: `graph[i][j][tau]` is the arrow from i at t−tau to j at t. */
export function stationaryMarksOf(document: DagDocument): { readonly statLag: number; readonly marks: readonly (readonly (readonly string[])[])[]; readonly hidden: readonly number[] } {
  return stationaryMarksFromGraph(document.current.graph)
}

/** Tigramite stationary marks for an immutable DAG revision rather than a document's current draft. */
export function stationaryMarksFromGraph(graph: EditableDag): { readonly statLag: number; readonly marks: readonly (readonly (readonly string[])[])[]; readonly hidden: readonly number[] } {
  const nodes = graph.nodes
  const index = new Map(nodes.map((node, position) => [node.id, position]))
  const statLag = Math.max(0, ...graph.edges.map((edge) => (edge.timing.kind === 'lagged' ? edge.timing.lag : 0)))
  const marks: string[][][] = nodes.map(() => nodes.map(() => Array.from({ length: statLag + 1 }, () => '')))
  for (const edge of graph.edges) {
    const from = index.get(edge.cause)
    const to = index.get(edge.effect)
    if (from === undefined || to === undefined) continue
    const lag = edge.timing.kind === 'lagged' ? edge.timing.lag : 0
    marks[from][to][lag] = '-->'
    if (lag === 0 && marks[to][from][0] === '') marks[to][from][0] = '<--'
  }
  return { statLag, marks, hidden: nodes.flatMap((node, position) => (node.kind === 'latent' ? [position] : [])) }
}

/** The first row at which a step treatment turns on, or the reason it is not a step. */
export function interventionStartFromTreatment(values: readonly number[]): Result<number, { readonly kind: 'not-a-step'; readonly detail: string }> {
  const first = values.findIndex((value) => value !== 0)
  if (first <= 0) return err({ kind: 'not-a-step', detail: first === 0 ? 'The treatment is already on at the first row.' : 'The treatment never turns on.' })
  if (values.slice(first).some((value) => value === 0)) return err({ kind: 'not-a-step', detail: 'The treatment switches off again after it starts.' })
  return ok(first)
}
import { adjustedDidEvidenceSchema, adjustedDidConfigurationSchema, sameDidSpecification, type AdjustedDidSpecification } from './adjustedDid'
