import { z } from 'zod'
import type { ColumnId } from './dataset'
import type { DagDocument } from './dag'
import { assertNever, brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { CaveatEvaluation, MethodCaveat, MethodDefinition, MethodEligibility, MethodId } from './methods'
import {
  BACKDOOR_LINEAR_REGRESSION_METHOD_ID,
  FRONTDOOR_TWO_STAGE_METHOD_ID,
  CAUSAL_EFFECTS_TOTAL_METHOD_ID,
  CAUSAL_IMPACT_METHOD_ID,
  ARDL_PSS_METHOD_ID,
  DISCRETE_BN_METHOD_ID,
  BINARY_ETT_METHOD_ID,
  DML_IRM_METHOD_ID,
  DML_PLR_METHOD_ID,
  BAYESIAN_GAUSSIAN_METHOD_ID,
  NEGBIN_NUTS_METHOD_ID,
  SYNTHETIC_CONTROL_METHOD_ID,
  VECM_METHOD_ID,
  NEGATIVE_BINOMIAL_METHOD_ID,
  POISSON_GLM_METHOD_ID,
  PANEL_INTERVENTION_METHOD_ID,
} from './methods'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId, StationarityEvidenceArtifact } from './preprocessing'
import { describePanelInterventionPreflight, type PanelInterventionPreflight } from './panel'
import { describeStationarityConflict, levelModelVerdict, type LevelModelVerdict, type StationarityAssessment } from './stationarityAssessment'
import { treatmentDescendants, type Estimand, type Identification, type IdentificationArtifact, type IdentificationId, type StudyId, type StudySpecification, type StudyVariable } from './study'

/**
 * An estimate carries its meaning in the type: the estimand it answers, the scale, a named interval
 * or an explicit absence, the sample it used, and the diagnostics the method reports. A bare number
 * never leaves this module. Every kernel behind these types is a transpiled port with a parity fixture.
 */

export type EstimationRunId = Brand<string, 'EstimationRunId'>

export type CovarianceChoice = 'hac' | 'classical'

export const CONFIDENCE_LEVEL = 0.95

export interface BackdoorLinearConfiguration {
  readonly kind: 'backdoor-linear-regression'
  readonly covariance: CovarianceChoice
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

export interface CountGlmConfiguration {
  readonly kind: 'poisson-glm' | 'negative-binomial-p'
}

export type TotalEffectEstimator = { readonly kind: 'linear' } | { readonly kind: 'knn'; readonly k: number }

export interface CausalEffectsConfiguration {
  readonly kind: 'causal-effects-total'
  readonly estimator: TotalEffectEstimator
  /** Lag of the treatment node relative to the outcome at time t; 0 means the same period. */
  readonly treatmentLag: number
  /** The two treatment values whose predicted outcomes are differenced. */
  readonly interventions: readonly [number, number]
}

export type InterventionStart =
  | { readonly kind: 'from-treatment' }
  | { readonly kind: 'row'; readonly row: number }

export interface CausalImpactConfiguration {
  readonly kind: 'causal-impact'
  readonly start: InterventionStart
  readonly controls: readonly ColumnId[]
  readonly maxIter: number
}

export interface DoubleMlConfiguration {
  readonly kind: 'dml-plr' | 'dml-irm'
  /** IRM only: the effect on the treated instead of the average effect. */
  readonly att: boolean
  /** Seeds the shuffled folds; recorded so the refutation batch can repeat the fit. */
  readonly seed: number
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
}

export interface PanelInterventionConfiguration {
  readonly kind: 'panel-intervention'
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
  | BackdoorLinearConfiguration
  | FrontdoorTwoStageConfiguration
  | CountGlmConfiguration
  | DoubleMlConfiguration
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

export const ESTIMATOR_IDS: NonEmptyArray<EstimatorId> = ['backdoor-linear-regression', 'frontdoor-two-stage', 'bayesian-gaussian', 'poisson-glm', 'negative-binomial-p', 'negbin-nuts', 'dml-plr', 'dml-irm', 'causal-effects-total', 'causal-impact', 'synthetic-control', 'panel-intervention', 'ardl-pss', 'vecm', 'discrete-bn-query', 'binary-ett-idc-star']

export const methodIdOf = (estimator: EstimatorId): MethodId => {
  switch (estimator) {
    case 'backdoor-linear-regression': return BACKDOOR_LINEAR_REGRESSION_METHOD_ID
    case 'frontdoor-two-stage': return FRONTDOOR_TWO_STAGE_METHOD_ID
    case 'poisson-glm': return POISSON_GLM_METHOD_ID
    case 'negative-binomial-p': return NEGATIVE_BINOMIAL_METHOD_ID
    case 'dml-plr': return DML_PLR_METHOD_ID
    case 'dml-irm': return DML_IRM_METHOD_ID
    case 'ardl-pss': return ARDL_PSS_METHOD_ID
    case 'vecm': return VECM_METHOD_ID
    case 'synthetic-control': return SYNTHETIC_CONTROL_METHOD_ID
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
    case 'backdoor-linear-regression': return { kind: estimator, covariance: prepared.kind === 'prepared-time-series' ? 'hac' : 'classical', level: CONFIDENCE_LEVEL }
    case 'frontdoor-two-stage': return { kind: estimator, interventions: [0, 1], simulations: 399, sampleSizeFraction: 1, level: CONFIDENCE_LEVEL, seed: 0 }
    case 'poisson-glm':
    case 'negative-binomial-p': return { kind: estimator }
    case 'dml-plr': return { kind: estimator, att: false, seed: 7 }
    case 'dml-irm': return { kind: estimator, att: study?.estimand.kind === 'average-treatment-effect-on-treated', seed: 7 }
    case 'ardl-pss': return { kind: estimator, maxLag: 4, trend: 'ct', case: 4 }
    case 'vecm': return { kind: estimator, maxLags: 4, deterministic: 'co', significance: 95, breakIndex: null }
    case 'synthetic-control': {
      const affected = affectedColumns(study)
      return {
        kind: estimator,
        start: { kind: 'from-treatment' },
        donors: study === null ? [] : prepared.columns.filter((column) => column !== study.treatment.column && column !== study.outcome.column && !affected.has(column)),
      }
    }
    case 'panel-intervention': return { kind: estimator }
    case 'negbin-nuts': return { kind: estimator, warmup: 500, samples: 1000, seed: 0 }
    case 'bayesian-gaussian': return { kind: estimator, warmup: 500, samples: 1000, seed: 41 }
    case 'discrete-bn-query': return { kind: estimator, bins: 3, equivalentSampleSize: 5 }
    case 'binary-ett-idc-star': return { kind: estimator }
    case 'causal-effects-total': return { kind: estimator, estimator: { kind: 'linear' }, treatmentLag: 0, interventions: [0, 1] }
    case 'causal-impact': {
      // A control the treatment itself moves would absorb the effect, so DAG descendants of the
      // treatment start unticked; columns outside the DAG stay in, as a judgement for the user.
      const affected = affectedColumns(study)
      return {
        kind: estimator,
        start: { kind: 'from-treatment' },
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
}).strict()

export type BackdoorLinearEvidence = z.infer<typeof backdoorLinearEvidenceSchema>

const frontdoorUncertaintySchema = z.discriminatedUnion('kind', [
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
  uncertainty: frontdoorUncertaintySchema,
}).strict()

export type FrontdoorTwoStageEvidence = z.infer<typeof frontdoorTwoStageEvidenceSchema>

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
}).strict()

export type DoubleMlEvidence = z.infer<typeof doubleMlEvidenceSchema>

export const ardlEvidenceSchema = z.object({
  kind: z.literal('ardlPss'),
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
}).strict()

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

export const panelInterventionEvidenceSchema = z.object({
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
}).strict().superRefine((evidence, context) => {
  if (evidence.controlUnits + evidence.treatedUnits !== evidence.units.length) context.addIssue({ code: 'custom', message: 'Panel unit counts do not match the unit labels.' })
  if (evidence.nPre + evidence.nPost !== evidence.times.length) context.addIssue({ code: 'custom', message: 'Panel period counts do not match the time codes.' })
  for (const [name, estimate] of [['DID', evidence.did], ['synthetic control', evidence.syntheticControl], ['synthetic DID', evidence.syntheticDid]] as const) {
    if (estimate.omega.length !== evidence.controlUnits) context.addIssue({ code: 'custom', message: `${name} control weights do not match the controls.` })
    if (estimate.lambda.length !== evidence.nPre) context.addIssue({ code: 'custom', message: `${name} time weights do not match the pre-periods.` })
    if (estimate.effectCurve.length !== evidence.nPost) context.addIssue({ code: 'custom', message: `${name} effect curve does not match the post-periods.` })
  }
})

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

export const discreteBnEvidenceSchema = z.object({
  kind: z.literal('discreteBnQuery'),
  observations: z.number().int().positive(),
  bins: z.number().int().min(2),
  equivalentSampleSize: z.number().positive(),
  stateCounts: z.array(z.number().int().positive()),
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

const nodeSchema = z.tuple([z.number().int().nonnegative(), z.number().int().max(0)])

export const causalEffectsEvidenceSchema = z.object({
  kind: z.literal('causalEffectsTotal'),
  observations: z.number().int().positive(),
  tauMax: z.number().int().nonnegative(),
  noCausalPath: z.boolean(),
  identifiable: z.boolean(),
  adjustmentSet: z.array(nodeSchema),
  mediators: z.array(nodeSchema),
  estimator: z.discriminatedUnion('kind', [z.object({ kind: z.literal('linear') }).strict(), z.object({ kind: z.literal('knn'), k: z.number().int().positive() }).strict()]),
  interventions: z.tuple([z.number().finite(), z.number().finite()]),
  predictions: z.array(z.number().finite()),
  totalEffect: nullableNumber,
  fittedObservations: z.number().int().nonnegative(),
}).strict()

export type CausalEffectsEvidence = z.infer<typeof causalEffectsEvidenceSchema>

export const causalImpactEvidenceSchema = z.object({
  kind: z.literal('causalImpact'),
  observations: z.number().int().positive(),
  nPre: z.number().int().positive(),
  nPost: z.number().int().positive(),
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

export type CausalImpactEvidence = z.infer<typeof causalImpactEvidenceSchema>

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

export const parseCountGlmEvidence = (value: unknown): Result<CountGlmEvidence, EstimationEvidenceProblem> => parseWith(countGlmEvidenceSchema, value)

export function parseCausalEffectsEvidence(value: unknown): Result<CausalEffectsEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(causalEffectsEvidenceSchema, value)
  if (!parsed.ok) return parsed
  if (parsed.value.identifiable && (parsed.value.predictions.length !== 2 || parsed.value.totalEffect === null)) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'An identifiable effect must carry two predictions and a total effect.' })
  }
  return parsed
}

export function parseCausalImpactEvidence(value: unknown): Result<CausalImpactEvidence, EstimationEvidenceProblem> {
  const parsed = parseWith(causalImpactEvidenceSchema, value)
  if (!parsed.ok) return parsed
  const { nPost, counterfactual, counterfactualSe, pointwise } = parsed.value
  if (counterfactual.length !== nPost || counterfactualSe.length !== nPost || pointwise.length !== nPost) {
    return err({ kind: 'invalid-estimation-evidence', detail: 'The counterfactual path does not cover the post-intervention window.' })
  }
  return parsed
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
  | { readonly kind: 'incidenceRateRatio'; readonly value: number }
  | { readonly kind: 'path'; readonly values: NonEmptyArray<TimeEffectPoint>; readonly aggregate: { readonly cumulative: number; readonly average: number } }

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
  readonly adjustmentSet: readonly StudyVariable[]
  readonly sample: { readonly observations: number; readonly parameters: number; readonly degreesOfFreedom: number | null }
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
  | RunIdentity & { readonly kind: 'backdoor-linear-run'; readonly method: typeof BACKDOOR_LINEAR_REGRESSION_METHOD_ID; readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }
  | RunIdentity & { readonly kind: 'frontdoor-two-stage-run'; readonly method: typeof FRONTDOOR_TWO_STAGE_METHOD_ID; readonly configuration: FrontdoorTwoStageConfiguration; readonly evidence: FrontdoorTwoStageEvidence }
  | RunIdentity & { readonly kind: 'count-glm-run'; readonly method: typeof POISSON_GLM_METHOD_ID | typeof NEGATIVE_BINOMIAL_METHOD_ID; readonly configuration: CountGlmConfiguration; readonly evidence: CountGlmEvidence }
  | RunIdentity & { readonly kind: 'double-ml-run'; readonly method: typeof DML_PLR_METHOD_ID | typeof DML_IRM_METHOD_ID; readonly configuration: DoubleMlConfiguration; readonly evidence: DoubleMlEvidence }
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
  if (context.stationarity.transform.kind === 'difference') { satisfy(id, 'The stationarity view is the first difference; the run reads the prepared levels, so difference the series in the recipe before trusting a level coefficient.'); return }
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
    : `${integrated.join(', ')} ${integrated.length === 1 ? 'is' : 'are'} I(1) in levels, so a level regression can show a spurious relation. Differencing in Data studio or a cointegration method might be needed.`
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

const verdict = (satisfied: Satisfied[], unresolved: Unresolved[], violations: Violated[]): MethodEligibility => {
  if (isNonEmpty(violations)) return { kind: 'refused', satisfied, unresolved, violations }
  if (isNonEmpty(unresolved)) return { kind: 'caution', satisfied, unresolved }
  return { kind: 'eligible', satisfied }
}

/** Rules over the identification, the sampling structure, the data shape, and the chosen configuration. */
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
    const targetIsAtt = context.study.estimand.kind === 'average-treatment-effect-on-treated'
    if (targetIsAtt && configuration.kind !== 'dml-irm' && configuration.kind !== 'binary-ett-idc-star') {
      violations.push({ kind: 'violated', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: 'This study targets ATT. DML interactive and the binary IDC* evaluator report ATT.' })
    } else if (!targetIsAtt && configuration.kind === 'binary-ett-idc-star') {
      violations.push({ kind: 'violated', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: 'The binary IDC* evaluator reports ETT/ATT, but this study records ATE.' })
    } else if (configuration.kind === 'dml-irm' && configuration.att !== targetIsAtt) {
      violations.push({ kind: 'violated', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: `The DML configuration reports ${configuration.att ? 'ATT' : 'ATE'}, but the study records ${targetIsAtt ? 'ATT' : 'ATE'}.` })
    } else {
      satisfied.push({ kind: 'satisfied', caveat: TARGET_COMPATIBILITY_CAVEAT, evidence: `Estimator and study both target ${targetIsAtt ? 'ATT' : 'ATE'}.` })
    }
  }

  switch (configuration.kind) {
    case 'backdoor-linear-regression': {
      if (adjustment === null) violate('linear-identified-adjustment', 'No measured back-door adjustment set was found, so this adjusted regression cannot run.')
      else satisfy('linear-identified-adjustment', `Identified by back-door adjustment for ${adjustment}.`)
      if (panel) leave('linear-serial-dependence', 'Rows repeat within units; neither interval accounts for within-unit correlation.')
      else if (timeSeries && configuration.covariance === 'classical') violate('linear-serial-dependence', 'The rows are a time series and the classical interval assumes independent errors. Choose the HAC interval.')
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
          satisfy('panel-balanced-layout', `${layout.controls.length} control and ${layout.treated.length} treated units across ${layout.prePeriods} pre- and ${layout.postPeriods} post-periods; treatment adopts simultaneously at ${layout.adoptionLabel} and remains on.`)
          break
        }
        default: assertNever(context.panelPreflight)
      }
      leave('panel-parallel-trends', 'Parallel untreated trends cannot be established from the panel shape; compare pre-period paths and the three estimators.')
      leave('panel-no-anticipation', 'Confirm that outcomes were not affected before the treatment indicator first turns on.')
      if (context.study?.designAssumptions.noInterference.rationale === null || context.study === null) {
        leave('panel-no-spillovers', 'The study has no recorded rationale for no interference between units.')
      } else {
        satisfy('panel-no-spillovers', `Recorded study rationale: ${context.study.designAssumptions.noInterference.rationale}`)
      }
      if (context.panelPreflight.kind === 'ready') {
        satisfy('panel-pre-fit', `${context.panelPreflight.layout.controls.length} controls and ${context.panelPreflight.layout.prePeriods} pre-periods provide non-constant control changes. Their standard deviation is ${context.panelPreflight.layout.controlPreDifferenceSd.toPrecision(4)}. The run reports all fitted weights.`)
      } else {
        leave('panel-pre-fit', 'The run checks control variation before treatment and reports the fitted unit and time weights.')
      }
      satisfy('panel-no-interval', 'This port reports point estimates without synthdid placebo, jackknife, or bootstrap variance. The result has no interval.')
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
    case 'dml-plr':
    case 'dml-irm': {
      const prefix = configuration.kind
      if (adjustment === null) violate(`${prefix}-identified-adjustment`, 'No measured back-door adjustment set was found, so there is no identified set for the nuisance learners.')
      else if (identification.kind === 'identified' && identification.adjustment.variables.length === 0) violate(`${prefix}-identified-adjustment`, 'The identified adjustment set is empty; double machine learning needs covariates to partial out. Use the adjusted linear regression.')
      else satisfy(`${prefix}-identified-adjustment`, `Nuisance learners see the identified set: ${adjustment}.`)
      if (panel) violate(`${prefix}-independent-rows`, 'The rows are a panel; shuffled folds would split a unit across folds, and unit-blocked cross-fitting is not ported yet.')
      else if (timeSeries) violate(`${prefix}-independent-rows`, 'The rows are a time series and the folds are shuffled; blocked or rolling cross-fitting is not ported yet.')
      else satisfy(`${prefix}-independent-rows`, 'The prepared dataset holds independent rows, so shuffled folds are valid.')
      if (configuration.kind === 'dml-irm') {
        if (context.treatmentIsBinary === null) leave('dml-irm-binary-treatment', 'The treatment column has not been read yet; it is checked when the run starts.')
        else if (context.treatmentIsBinary) satisfy('dml-irm-binary-treatment', 'Every treatment value is 0 or 1.')
        else violate('dml-irm-binary-treatment', 'The treatment holds values other than 0 and 1; use the partially linear model.')
      } else {
        leave('dml-plr-partial-linearity', 'Partial linearity in the treatment is assumed; compare with the interactive model when the treatment is binary.')
      }
      leave(`${prefix}-overlap`, 'Inspect treatment overlap against the adjustment variables in Data studio.')
      satisfy(`${prefix}-learner-settings`, `The run records 5 folds, 200 trees, minimum leaf 5, learner seed 7, and fold seed ${configuration.seed}.`)
      if (prepared.observations < 100) leave(`${prefix}-interval`, `${prepared.observations} rows is a small sample for random-forest nuisances; read the interval as approximate.`)
      else satisfy(`${prefix}-interval`, `${prepared.observations} rows for the sandwich interval.`)
      break
    }
    case 'ardl-pss': {
      if (!timeSeries) violate('ardl-time-series', 'An autoregressive distributed lag model needs an ordered time series. This prepared dataset holds independent rows.')
      else if (prepared.observations < 6 * (configuration.maxLag + 1) + 10) violate('ardl-time-series', `${prepared.observations} rows is too few for a maximum lag of ${configuration.maxLag}.`)
      else satisfy('ardl-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series with ${prepared.observations} rows for a maximum lag of ${configuration.maxLag}.`)
      if (adjustment === null) violate('ardl-single-regressor', 'No measured back-door adjustment set was found for this study.')
      else if (identification.kind === 'identified' && identification.adjustment.variables.length > 0) violate('ardl-single-regressor', `The identified adjustment set (${adjustment}) is not empty, and the port fits one exogenous variable.`)
      else satisfy('ardl-single-regressor', 'No adjustment is needed, so the treatment is the single exogenous variable.')
      applyOrderRule('ardl-orders-assessed', context, 'i0-or-i1', satisfy, leave, violate)
      leave('ardl-bounds-reading', 'The bounds test is read with the run: only a statistic above the I(1) bound establishes a level relation.')
      satisfy('ardl-deterministic-case', `The run records ${configuration.trend === 'ct' ? 'a constant and trend' : 'a constant'} with Pesaran–Shin–Smith case ${configuration.case}.`)
      break
    }
    case 'vecm': {
      if (!timeSeries) violate('vecm-sample', 'A vector error correction model needs an ordered time series. This prepared dataset holds independent rows.')
      else if (prepared.observations < (configuration.maxLags + 2) * (2 + (identification.kind === 'identified' ? identification.adjustment.variables.length : 0)) * 3 + 10) violate('vecm-sample', `${prepared.observations} rows is too few for the variables at ${configuration.maxLags} lags.`)
      else satisfy('vecm-sample', `Prepared as a regular ${prepared.sampling.frequency} time series; deterministic terms “${configuration.deterministic}” and up to ${configuration.maxLags} lags are recorded.`)
      applyOrderRule('vecm-all-i1', context, 'i1-only', satisfy, leave, violate)
      leave('vecm-rank', 'The Johansen trace test decides the rank when the run starts; rank zero reports no effect.')
      leave('vecm-single-relation', 'A long-run effect is read only when the rank is one.')
      leave('vecm-no-interval', 'The port reports the long-run vector without a standard error.')
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
      leave('synthetic-no-interval', 'The port reports the gap without a placebo distribution.')
      break
    }
    case 'negbin-nuts': {
      if (adjustment === null) violate('nuts-model-shape', 'No measured back-door adjustment set was found, so this one-confounder model has no identified covariate.')
      else if (identification.kind === 'identified' && identification.adjustment.variables.length !== 1) violate('nuts-model-shape', `The ported model takes exactly one confounder; the identified set holds ${identification.adjustment.variables.length}.`)
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
      satisfy('bn-discretisation', `Each variable is cut into ${configuration.bins} quantile bins. The run records the mean value for each state.`)
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
        const refused = verdicts.filter((verdict) => verdict.kind === 'refused')
        const open = verdicts.filter((verdict) => verdict.kind === 'unresolved')
        if (refused.length > 0) violate('causal-effects-stationarity', refused.map((verdict) => verdict.reason).join(' '))
        else if (open.length > 0) leave('causal-effects-stationarity', open.map((verdict) => verdict.reason).join(' '))
        else satisfy('causal-effects-stationarity', `All ${verdicts.length} prepared series are stationary in levels.`)
      }
      leave('causal-effects-identifiable', 'Whether the optimal adjustment set exists is decided by the run; a refusal is reported as not identifiable.')
      if (configuration.estimator.kind === 'linear') satisfy('causal-effects-functional-form', 'Linear first-stage estimator.')
      else leave('causal-effects-functional-form', `k-nearest neighbours with k = ${configuration.estimator.k}; the fit is local and reports no interval.`)
      leave('causal-effects-no-interval', 'The port reports a point estimate only; tigramite’s bootstrap is not ported.')
      break
    }
    case 'causal-impact': {
      if (!timeSeries) violate('impact-time-series', 'An intervention analysis needs an ordered time series. This prepared dataset holds independent rows.')
      else satisfy('impact-time-series', `Prepared as a regular ${prepared.sampling.frequency} time series.`)
      if (configuration.start.kind === 'row') satisfy('impact-intervention-time', `Intervention starts at row ${configuration.start.row}.`)
      else leave('impact-intervention-time', 'The intervention start is read from the treatment column when the run starts: it must be zero before and non-zero after one point.')
      leave('impact-pre-period', 'The pre-period length is checked at run time; at least 8 rows are required and more is better.')
      if (configuration.controls.length === 0) leave('impact-controls', 'Choose control series, or accept a counterfactual based only on a local level.')
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

/** The typed estimate from each façade's evidence; the interval follows the configuration. */
export function causalEstimateFrom(
  study: StudySpecification,
  identification: IdentificationArtifact,
  run:
    | { readonly kind: 'backdoor-linear-run'; readonly configuration: BackdoorLinearConfiguration; readonly evidence: BackdoorLinearEvidence }
    | { readonly kind: 'frontdoor-two-stage-run'; readonly configuration: FrontdoorTwoStageConfiguration; readonly evidence: FrontdoorTwoStageEvidence }
    | { readonly kind: 'count-glm-run'; readonly configuration: CountGlmConfiguration; readonly evidence: CountGlmEvidence }
    | { readonly kind: 'double-ml-run'; readonly configuration: DoubleMlConfiguration; readonly evidence: DoubleMlEvidence }
    | { readonly kind: 'ardl-run'; readonly configuration: ArdlConfiguration; readonly evidence: ArdlEvidence }
    | { readonly kind: 'vecm-run'; readonly configuration: VecmConfiguration; readonly evidence: VecmEvidence }
    | { readonly kind: 'synthetic-control-run'; readonly configuration: SyntheticControlConfiguration; readonly evidence: SyntheticControlEvidence }
    | { readonly kind: 'panel-intervention-run'; readonly configuration: PanelInterventionConfiguration; readonly evidence: PanelInterventionEvidence }
    | { readonly kind: 'negbin-nuts-run'; readonly configuration: NegbinNutsConfiguration; readonly evidence: NegbinNutsEvidence }
    | { readonly kind: 'bayesian-gaussian-run'; readonly configuration: BayesianGaussianConfiguration; readonly evidence: BayesianGaussianEvidence }
    | { readonly kind: 'discrete-bn-run'; readonly configuration: DiscreteBnConfiguration; readonly evidence: DiscreteBnEvidence }
    | { readonly kind: 'binary-ett-run'; readonly configuration: BinaryEttConfiguration; readonly evidence: BinaryEttEvidence }
    | { readonly kind: 'causal-effects-run'; readonly configuration: CausalEffectsConfiguration; readonly evidence: CausalEffectsEvidence }
    | { readonly kind: 'causal-impact-run'; readonly configuration: CausalImpactConfiguration; readonly evidence: CausalImpactEvidence },
): CausalEstimate | null {
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
      adjustmentSet: [],
      sample: {
        observations: run.evidence.observations,
        parameters: Math.max(run.evidence.firstStageParams.length, run.evidence.secondStageParams.length),
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
      adjustmentSet: [],
      sample: { observations: run.evidence.observations, parameters: 0, degreesOfFreedom: null },
    }
  }
  if (identification.result.kind !== 'identified') return null
  const adjustmentSet = identification.result.adjustment.variables
  switch (run.kind) {
    case 'backdoor-linear-run': {
      const hac = run.configuration.covariance === 'hac'
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.estimate, unit: '' },
        interval: { kind: 'confidence', level: evidence.level, lower: hac ? evidence.hacInterval[0] : evidence.interval[0], upper: hac ? evidence.hacInterval[1] : evidence.interval[1] },
        standardError: hac ? evidence.hacStandardError : evidence.standardError,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.parameters, degreesOfFreedom: evidence.degreesOfFreedom },
      }
    }
    case 'count-glm-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'incidenceRateRatio', value: evidence.incidenceRateRatio },
        interval: { kind: 'confidence', level: evidence.level, lower: evidence.incidenceRateRatioInterval[0], upper: evidence.incidenceRateRatioInterval[1] },
        standardError: evidence.standardError,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.parameters, degreesOfFreedom: evidence.degreesOfFreedom },
      }
    }
    case 'double-ml-run': {
      const { evidence } = run
      const studyTargetsAtt = study.estimand.kind === 'average-treatment-effect-on-treated'
      if (evidence.att !== studyTargetsAtt || run.configuration.att !== studyTargetsAtt) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.estimate, unit: '' },
        interval: { kind: 'confidence', level: evidence.level, lower: evidence.interval[0], upper: evidence.interval[1] },
        standardError: evidence.standardError,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: 1 + adjustmentSet.length, degreesOfFreedom: null },
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
        adjustmentSet,
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
        interval: { kind: 'none', reason: 'The VECM port reports the outcome-normalised long-run vector without a standard error.' },
        standardError: null,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.beta.length * evidence.rank, degreesOfFreedom: null },
      }
    }
    case 'synthetic-control-run': {
      const { evidence } = run
      const points = evidence.postGap.map((effect, index): TimeEffectPoint => {
        const counterfactual = evidence.synthetic[evidence.nPre + index] ?? Number.NaN
        return { step: evidence.nPre + index + 1, actual: counterfactual + effect, counterfactual, lower: counterfactual, upper: counterfactual, effect }
      })
      if (!isNonEmpty(points)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'path', values: points, aggregate: { cumulative: evidence.postGap.reduce((sum, value) => sum + value, 0), average: evidence.att } },
        interval: { kind: 'none', reason: 'The port reports the gap without a placebo distribution; no interval is available.' },
        standardError: null,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.weights.length, degreesOfFreedom: null },
      }
    }
    case 'panel-intervention-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate', estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.syntheticDid.estimate, unit: '' },
        interval: { kind: 'none', reason: 'The panel port reports DID, synthetic control, and synthetic DID point estimates; resampling variance is not included.' },
        standardError: null,
        adjustmentSet: [],
        sample: { observations: evidence.observations, parameters: evidence.syntheticDid.lambda.length + evidence.syntheticDid.omega.length, degreesOfFreedom: null },
      }
    }
    case 'negbin-nuts-run': {
      const { evidence } = run
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'incidenceRateRatio', value: evidence.irrMedian },
        interval: { kind: 'credible', level: 0.95, summary: 'ETI', lower: evidence.irrLower, upper: evidence.irrUpper },
        standardError: null,
        adjustmentSet,
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
        adjustmentSet,
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
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.stateCounts.reduce((sum, count) => sum + count, 0), degreesOfFreedom: null },
      }
    }
    case 'causal-effects-run': {
      const { evidence } = run
      if (!evidence.identifiable || evidence.totalEffect === null) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'additive', value: evidence.totalEffect, unit: '' },
        interval: { kind: 'none', reason: 'This implementation reports the point estimate; bootstrap uncertainty is not included.' },
        standardError: null,
        adjustmentSet,
        sample: { observations: evidence.observations, parameters: evidence.adjustmentSet.length + 1, degreesOfFreedom: null },
      }
    }
    case 'causal-impact-run': {
      const { evidence } = run
      const z = 1.959963984540054
      const points = evidence.pointwise.map((effect, index): TimeEffectPoint => ({
        step: evidence.nPre + index + 1,
        actual: evidence.counterfactual[index] + effect,
        counterfactual: evidence.counterfactual[index],
        lower: evidence.counterfactual[index] - z * evidence.counterfactualSe[index],
        upper: evidence.counterfactual[index] + z * evidence.counterfactualSe[index],
        effect,
      }))
      if (!isNonEmpty(points)) return null
      return {
        kind: 'causal-estimate',
        estimand: study.estimand,
        effect: { kind: 'path', values: points, aggregate: { cumulative: evidence.cumulative, average: evidence.average } },
        interval: { kind: 'none', reason: 'The port forecasts a pointwise band from the state variance; no interval for the cumulative effect is reported.' },
        standardError: null,
        adjustmentSet: [],
        sample: { observations: evidence.observations, parameters: evidence.params.length, degreesOfFreedom: null },
      }
    }
    default:
      return assertNever(run)
  }
}

export function describeCovariance(choice: CovarianceChoice): string {
  switch (choice) {
    case 'hac': return 'Newey–West HAC'
    case 'classical': return 'Classical'
    default: return assertNever(choice)
  }
}

export function describeEstimator(estimator: EstimatorId): string {
  switch (estimator) {
    case 'backdoor-linear-regression': return 'Adjusted linear regression'
    case 'frontdoor-two-stage': return 'Linear front-door regression'
    case 'poisson-glm': return 'Poisson GLM'
    case 'negative-binomial-p': return 'Negative binomial'
    case 'dml-plr': return 'Double machine learning, partially linear'
    case 'dml-irm': return 'Double machine learning, interactive'
    case 'ardl-pss': return 'ARDL long run'
    case 'vecm': return 'VECM'
    case 'synthetic-control': return 'Synthetic control'
    case 'panel-intervention': return 'Panel DID / synthetic DID'
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
  const nodes = document.current.graph.nodes
  const index = new Map(nodes.map((node, position) => [node.id, position]))
  const statLag = Math.max(0, ...document.current.graph.edges.map((edge) => (edge.timing.kind === 'lagged' ? edge.timing.lag : 0)))
  const marks: string[][][] = nodes.map(() => nodes.map(() => Array.from({ length: statLag + 1 }, () => '')))
  for (const edge of document.current.graph.edges) {
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
