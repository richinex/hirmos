import { z } from 'zod'
import { assertNever, err, ok, type Result } from './dop'
import type { Estimand, StudyVariable } from './study'
import { forestAnalysisSchema, forestAnalysisEvidenceSchema } from './causalForestAnalysis'

/** GRF 2.6.1: causal_forest.R, average_treatment_effect.R and forest_summary.R.
 * Targets are not interchangeable. In particular, a continuous partial effect is
 * not a contrast between arbitrary treatment values. */
export const causalForestTargetSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('binary-average'), population: z.enum(['all', 'treated', 'control', 'overlap']) }).strict(),
  z.object({ kind: z.literal('binary-conditional') }).strict(),
  z.object({ kind: z.literal('continuous-average'), population: z.enum(['all', 'overlap']) }).strict(),
  z.object({ kind: z.literal('continuous-conditional') }).strict(),
])
export type CausalForestTarget = z.infer<typeof causalForestTargetSchema>

export function causalForestTarget(estimand: Estimand): CausalForestTarget | null {
  switch (estimand.kind) {
    case 'average-treatment-effect': return { kind: 'binary-average', population: 'all' }
    case 'average-treatment-effect-on-treated': return { kind: 'binary-average', population: 'treated' }
    case 'average-treatment-effect-on-controls': return { kind: 'binary-average', population: 'control' }
    case 'overlap-weighted-average-treatment-effect': return { kind: 'binary-average', population: 'overlap' }
    case 'conditional-average-treatment-effect-per-row': return { kind: 'binary-conditional' }
    case 'average-partial-effect': return { kind: 'continuous-average', population: 'all' }
    case 'variance-weighted-average-partial-effect': return { kind: 'continuous-average', population: 'overlap' }
    case 'conditional-partial-effect-per-row': return { kind: 'continuous-conditional' }
    case 'local-cutoff-effect':
    case 'conditional-average-treatment-effect': return null
    default: return assertNever(estimand)
  }
}

export function causalForestInputs(adjustment: readonly StudyVariable[], estimand: Estimand | null): readonly StudyVariable[] {
  const modifiers = estimand?.kind === 'conditional-average-treatment-effect-per-row' || estimand?.kind === 'conditional-partial-effect-per-row' ? estimand.modifiers : []
  return [...adjustment, ...modifiers.filter(modifier => !adjustment.some(variable => variable.column === modifier.column))]
}

export function sameCausalForestTarget(left: CausalForestTarget, right: CausalForestTarget): boolean {
  switch (left.kind) {
    case 'binary-average': return right.kind === left.kind && right.population === left.population
    case 'continuous-average': return right.kind === left.kind && right.population === left.population
    case 'binary-conditional':
    case 'continuous-conditional': return right.kind === left.kind
    default: return assertNever(left)
  }
}

const finite = z.number().finite()
const positiveInteger = z.number().int().positive().max(0xffff_ffff)
const fraction = finite.gt(0).lt(1)
export const causalForestHonestySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('disabled') }).strict(),
  z.object({ kind: z.literal('enabled'), fraction, prune: z.boolean() }).strict(),
])
export const causalForestTuningSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('disabled') }).strict(),
  z.object({ kind: z.literal('all'), trees: positiveInteger, repetitions: positiveInteger, draws: positiveInteger }).strict(),
])

export const causalForestConfigurationSchema = z.object({
  kind: z.literal('causal-forest'),
  trees: positiveInteger.min(2),
  seed: z.number().int().min(0).max(0xffff_ffff),
  sampleFraction: finite.gt(0).max(0.5),
  variablesPerSplit: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('automatic') }).strict(),
    z.object({ kind: z.literal('specified'), count: positiveInteger }).strict(),
  ]),
  minimumNodeSize: positiveInteger,
  honesty: causalForestHonestySchema,
  alpha: finite.min(0).lt(0.5),
  imbalancePenalty: finite.nonnegative(),
  stabilizeSplits: z.boolean(),
  groupSize: positiveInteger.min(2),
  tuning: causalForestTuningSchema,
  confidenceLevel: finite.gt(0).lt(1),
  analysis: forestAnalysisSchema.optional(),
  refit: z.object({ nuisanceTrees: positiveInteger, initialTrees: positiveInteger.min(2),
    outcomeSeed: z.number().int().min(0).max(0xffffffff), treatmentSeed: z.number().int().min(0).max(0xffffffff), initialSeed: z.number().int().min(0).max(0xffffffff),
  }).strict().optional(),
}).strict().superRefine((value, context) => {
  if (value.trees < value.groupSize) context.addIssue({ code: 'custom', path: ['trees'], message: 'Grow at least one complete group of trees.' })
})
export type CausalForestConfiguration = z.infer<typeof causalForestConfigurationSchema>

/** Upstream defaults, except for the explicit, reproducible seed and display confidence level. */
export const DEFAULT_CAUSAL_FOREST: CausalForestConfiguration = {
  kind: 'causal-forest', trees: 2000, seed: 42, sampleFraction: 0.5,
  variablesPerSplit: { kind: 'automatic' }, minimumNodeSize: 5,
  honesty: { kind: 'enabled', fraction: 0.5, prune: true },
  alpha: 0.05, imbalancePenalty: 0, stabilizeSplits: true, groupSize: 2,
  tuning: { kind: 'disabled' }, confidenceLevel: 0.95,
}

const intervalSchema = z.object({ lower: finite, upper: finite }).strict()
  .refine(value => value.lower <= value.upper, 'The interval bounds are reversed.')

export const causalForestPredictionSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  z.object({
    kind: z.literal('estimated'), estimate: finite,
    uncertainty: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
      z.object({ kind: z.literal('estimated'), standardError: finite.nonnegative(), interval: intervalSchema }).strict(),
    ]),
  }).strict(),
])
export type CausalForestPrediction = z.infer<typeof causalForestPredictionSchema>

export const causalForestSummarySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('not-requested') }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  z.object({ kind: z.literal('estimated'), estimate: finite, standardError: finite.nonnegative(), interval: intervalSchema }).strict(),
])

const calibrationCoefficientSchema = z.object({
  estimate: finite, standardError: finite.nonnegative(), statistic: finite, pValue: finite.min(0).max(1),
}).strict()
export const causalForestCalibrationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  z.object({ kind: z.literal('estimated'), mean: calibrationCoefficientSchema, differential: calibrationCoefficientSchema, degreesOfFreedom: positiveInteger }).strict(),
])

export const forestFeatureSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('numeric'), column: z.string().min(1), name: z.string().min(1) }).strict(),
  z.object({ kind: z.literal('indicator'), column: z.string().min(1), name: z.string().min(1), level: finite }).strict(),
])
export type ForestFeature = z.infer<typeof forestFeatureSchema>
export const forestFeatureLabel = (feature: ForestFeature): string =>
  feature.kind === 'indicator' ? `${feature.name} = ${feature.level}` : feature.name

export const causalForestEvidenceSchema = z.object({
  target: causalForestTargetSchema,
  confidenceLevel: finite.gt(0).lt(1),
  observations: positiveInteger,
  predictions: z.array(causalForestPredictionSchema),
  summary: causalForestSummarySchema,
  calibration: causalForestCalibrationSchema,
  /** Relative split frequency, not an intervention effect. One value per encoded covariate. */
  variableImportance: z.array(finite.nonnegative()),
  /** Saved at design expansion, in the exact order supplied to the forest. */
  features: z.array(forestFeatureSchema).optional(),
  fitted: causalForestConfigurationSchema,
  analysis: forestAnalysisEvidenceSchema.optional(),
  refit: z.object({ initialImportance: z.array(finite.nonnegative()).min(1), selected: z.array(z.number().int().nonnegative()).min(1),
    outcomePredictions: z.array(finite), treatmentPredictions: z.array(finite),
  }).strict().optional(),
}).strict().superRefine((value, context) => {
  if (value.features !== undefined && value.features.length !== value.variableImportance.length)
    context.addIssue({ code: 'custom', message: 'Forest feature labels must match the encoded importance columns.' })
  if ((value.refit === undefined) !== (value.fitted.refit === undefined)) context.addIssue({ code: 'custom', message: 'The refit evidence must match the fitted specification.' })
  if (value.refit !== undefined) {
    const r = value.refit, n = value.variableImportance.length
    const threshold = r.initialImportance.reduce((a,b) => a+b, 0) / n
    const selected = r.initialImportance.flatMap((v,i) => v > threshold ? [i] : [])
    if (r.initialImportance.length !== n || JSON.stringify(r.selected) !== JSON.stringify(selected)
      || r.outcomePredictions.length !== value.observations || r.treatmentPredictions.length !== value.observations)
      context.addIssue({ code: 'custom', message: 'The retained features or nuisance predictions do not match the refit.' })
  }
  if (value.predictions.length !== value.observations) context.addIssue({ code: 'custom', path: ['predictions'], message: 'Each retained observation needs a prediction record.' })
  if (value.fitted.confidenceLevel !== value.confidenceLevel) context.addIssue({ code: 'custom', path: ['fitted'], message: 'The fitted confidence level must match the reported intervals.' })
  const conditional = value.target.kind === 'binary-conditional' || value.target.kind === 'continuous-conditional'
  if (conditional !== (value.summary.kind === 'not-requested')) context.addIssue({ code: 'custom', path: ['summary'], message: 'The summary does not match the recorded target.' })
})
export type CausalForestEvidence = z.infer<typeof causalForestEvidenceSchema>

/** Selected parameters may differ only when tuning was explicitly requested. */
export function causalForestSettingsMatch(requested: CausalForestConfiguration, evidence: CausalForestEvidence): boolean {
  const fitted = evidence.fitted
  if (JSON.stringify(requested.refit) !== JSON.stringify(fitted.refit)) return false
  if (JSON.stringify(requested.analysis) !== JSON.stringify(evidence.analysis?.specification)
      || JSON.stringify(requested.analysis) !== JSON.stringify(fitted.analysis)) return false
  if (fitted.tuning.kind !== 'disabled' || fitted.trees !== requested.trees || fitted.seed !== requested.seed
      || fitted.groupSize !== requested.groupSize || fitted.stabilizeSplits !== requested.stabilizeSplits
      || fitted.confidenceLevel !== requested.confidenceLevel || evidence.confidenceLevel !== requested.confidenceLevel
      || fitted.honesty.kind !== requested.honesty.kind || fitted.variablesPerSplit.kind !== 'specified') return false
  if (requested.tuning.kind === 'all') return true
  const featureCount = evidence.refit?.selected.length ?? evidence.variableImportance.length
  const expectedMtry = requested.variablesPerSplit.kind === 'specified' ? requested.variablesPerSplit.count
    : Math.min(featureCount, Math.ceil(Math.sqrt(featureCount) + 20))
  return fitted.variablesPerSplit.count === expectedMtry
    && fitted.sampleFraction === requested.sampleFraction && fitted.minimumNodeSize === requested.minimumNodeSize
    && fitted.alpha === requested.alpha && fitted.imbalancePenalty === requested.imbalancePenalty
    && (requested.honesty.kind === 'disabled' || (fitted.honesty.kind === 'enabled'
      && fitted.honesty.fraction === requested.honesty.fraction && fitted.honesty.prune === requested.honesty.prune))
}

export function parseCausalForestEvidence(input: unknown): Result<CausalForestEvidence, { readonly kind: 'invalid-causal-forest-evidence'; readonly detail: string }> {
  const parsed = causalForestEvidenceSchema.safeParse(input)
  return parsed.success ? ok(parsed.data) : err({ kind: 'invalid-causal-forest-evidence', detail: parsed.error.message })
}

/** Wording is paraphrased from the references, not a claim that an observed row has two observed outcomes.
 * Keep citations in the method reference area, not in field labels. */
export const CAUSAL_FOREST_REFERENCES = [
  { title: 'Generalized random forests', locator: 'Athey, Tibshirani and Wager (2019), section 6', url: 'https://doi.org/10.1214/18-AOS1709' },
  { title: 'Causal forest', locator: 'GRF 2.6.1, causal_forest', url: 'https://grf-labs.github.io/grf/reference/causal_forest.html' },
  { title: 'Average treatment effects', locator: 'GRF 2.6.1, average_treatment_effect', url: 'https://grf-labs.github.io/grf/reference/average_treatment_effect.html' },
  { title: 'Calibration test', locator: 'GRF 2.6.1, test_calibration', url: 'https://grf-labs.github.io/grf/reference/test_calibration.html' },
  { title: 'Best linear projection', locator: 'GRF 2.6.1, best_linear_projection', url: 'https://grf-labs.github.io/grf/reference/best_linear_projection.html' },
  { title: 'Rank-weighted average treatment effects', locator: 'GRF 2.6.1, rank_average_treatment_effect', url: 'https://grf-labs.github.io/grf/reference/rank_average_treatment_effect.html' },
  { title: 'Heterogeneous treatment effects', locator: 'Everyday causal inference, section 12', url: 'https://www.everydaycausal.com/heterogeneous-effects.html' },
] as const

export function describeCausalForestTarget(target: CausalForestTarget): string {
  switch (target.kind) {
    case 'binary-average': {
      switch (target.population) {
        case 'all': return 'Average treatment effect'
        case 'treated': return 'Average treatment effect on the treated'
        case 'control': return 'Average treatment effect on the controls'
        case 'overlap': return 'Overlap-weighted average treatment effect'
        default: return assertNever(target)
      }
    }
    case 'binary-conditional': return 'Conditional average treatment effects'
    case 'continuous-average': return target.population === 'all' ? 'Average partial effect' : 'Variance-weighted average partial effect'
    case 'continuous-conditional': return 'Conditional partial effects'
    default: return assertNever(target)
  }
}

export function causalForestPredictionLabel(target: CausalForestTarget): string {
  switch (target.kind) {
    case 'binary-average':
    case 'binary-conditional': return 'Conditional average treatment effect'
    case 'continuous-average':
    case 'continuous-conditional': return 'Conditional partial effect'
    default: return assertNever(target)
  }
}

export const CAUSAL_FOREST_COPY = {
  conditional: 'Each prediction estimates an average treatment effect conditional on the recorded characteristics. It is not an observed individual treatment effect.',
  partial: 'Each prediction estimates conditional outcome-treatment covariance divided by conditional treatment variance. A causal slope interpretation requires the treatment model assumptions.',
  partialContrast: 'These slopes do not describe an unrestricted dose-response curve or a contrast between arbitrary treatment values.',
  intervals: 'Intervals describe uncertainty in each conditional-effect estimate. They are pointwise, not simultaneous intervals for all observations.',
  variation: 'Variation in predictions alone does not establish treatment-effect heterogeneity. Review the calibration test alongside the predictions.',
  calibration: 'A mean coefficient near 1 supports calibration of the average prediction. A differential coefficient near 1 supports calibration of predicted heterogeneity.',
  calibrationTest: 'The differential coefficient uses a one-sided test against zero. A significantly positive coefficient provides evidence of heterogeneity.',
  importance: 'Importance describes how often each variable is used for splits, with deeper splits receiving less weight. It does not estimate a causal effect.',
  honesty: 'Use separate observations to choose splits and estimate leaf effects within each tree.',
  trees: 'More trees may be needed for accurate confidence intervals than for accurate predictions.',
  outOfBag: 'Each training observation is predicted using trees whose training samples excluded it.',
} as const
