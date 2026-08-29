import { z } from 'zod'
import { assertNever, brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { EstimationRunArtifact, EstimationRunId } from './estimation'
import type { MethodId } from './methods'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { StudyVariable } from './study'

/**
 * Sensitivity keeps every probe as its own fact: a refuter's shifted estimate, a simulated confounder
 * grid, a change point. Nothing here folds the facts into one score; that is a human's reading.
 */

export type SensitivityRunId = Brand<string, 'SensitivityRunId'>

export const newSensitivityRunId = (): SensitivityRunId => brand<string, 'SensitivityRunId'>(crypto.randomUUID())

export interface RefutationConfiguration {
  readonly kind: 'linear-refutation'
  readonly simulations: number
  readonly subsetFraction: number
  readonly seed: number
  readonly ljungBoxLags: number
}

/** A strength grid for the simulated confounder: DoWhy's inferred range, or an explicit even spacing. */
export type KappaRange =
  | { readonly kind: 'inferred' }
  | { readonly kind: 'range'; readonly from: number; readonly to: number; readonly steps: number }

export interface UnobservedConfiguration {
  readonly kind: 'unobserved-confounding'
  readonly seed: number
  readonly kappaT: KappaRange
  readonly kappaY: KappaRange
}

export const kappaValues = (range: KappaRange): readonly number[] | null => {
  if (range.kind === 'inferred') return null
  const steps = Math.max(1, Math.min(200, Math.floor(range.steps)))
  if (steps === 1) return [range.from]
  return Array.from({ length: steps }, (_, index) => range.from + ((range.to - range.from) * index) / (steps - 1))
}

export interface DmlRefutationConfiguration {
  readonly kind: 'dml-refutation'
  /** The fold seed; the batch's main fit equals the estimation run at the same seed. */
  readonly seed: number
}

export type SensitivityConfiguration = RefutationConfiguration | UnobservedConfiguration | DmlRefutationConfiguration

export type SensitivityProbe = SensitivityConfiguration['kind']

export const DEFAULT_REFUTATION: RefutationConfiguration = { kind: 'linear-refutation', simulations: 50, subsetFraction: 0.8, seed: 555, ljungBoxLags: 12 }
export const DEFAULT_UNOBSERVED: UnobservedConfiguration = { kind: 'unobserved-confounding', seed: 100, kappaT: { kind: 'inferred' }, kappaY: { kind: 'inferred' } }
export const DEFAULT_DML_REFUTATION: DmlRefutationConfiguration = { kind: 'dml-refutation', seed: 7 }

export const linearRefutationEvidenceSchema = z.object({
  kind: z.literal('linearRefutation'),
  observations: z.number().int().positive(),
  estimate: z.number().finite(),
  simulations: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  placeboEffect: z.number().finite(),
  subsetFraction: z.number().gt(0).lt(1),
  subsetEffect: z.number().finite(),
  randomCommonCauseEffect: z.number().finite(),
  ljungBoxLags: z.array(z.number().int().positive()),
  ljungBoxStatistics: z.array(z.number().finite()),
  ljungBoxPValues: z.array(z.number().min(0).max(1)),
  shapiroW: z.number().finite().nullable(),
  shapiroP: z.number().min(0).max(1).nullable(),
  durbinWatson: z.number().finite().nonnegative(),
}).strict()

export type LinearRefutationEvidence = z.infer<typeof linearRefutationEvidenceSchema>

export const unobservedConfoundingEvidenceSchema = z.object({
  kind: z.literal('unobservedConfounding'),
  observations: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  kappaT: z.array(z.number().finite()).min(1),
  kappaY: z.array(z.number().finite()).min(1),
  effects: z.array(z.array(z.number().finite())),
  originalEffect: z.number().finite(),
}).strict()

export type UnobservedConfoundingEvidence = z.infer<typeof unobservedConfoundingEvidenceSchema>

const dmlRefutationOutcomeSchema = z.object({
  originalEffect: z.number().finite(),
  refutedEffect: z.number().finite(),
  pValue: z.number().min(0).max(1),
}).strict()

export const dmlRefutationEvidenceSchema = z.object({
  kind: z.literal('dmlRefutationBatch'),
  observations: z.number().int().positive(),
  model: z.enum(['plr', 'irm']),
  att: z.boolean(),
  seed: z.number().int().nonnegative(),
  order: z.tuple([z.literal('mainFit'), z.literal('placebo'), z.literal('randomCommonCause'), z.literal('unobservedSensitivity')]),
  mainEstimate: z.number().finite(),
  placebo: dmlRefutationOutcomeSchema,
  randomCommonCause: dmlRefutationOutcomeSchema,
  sensitivity: z.object({
    scenarios: z.array(z.object({
      confounding: z.number().finite(),
      effectLower: z.number().finite(),
      effectUpper: z.number().finite(),
      ciLower: z.number().finite(),
      ciUpper: z.number().finite(),
    }).strict()).min(1),
    robustnessValue: z.number().min(0).max(1),
    robustnessValueCi: z.number().min(0).max(1),
  }).strict(),
}).strict()

export type DmlRefutationEvidence = z.infer<typeof dmlRefutationEvidenceSchema>

export const seriesStructureEvidenceSchema = z.object({
  kind: z.literal('seriesStructure'),
  observations: z.number().int().positive(),
  period: z.number().int().min(2).nullable(),
  series: z.array(z.object({
    column: z.number().int().nonnegative(),
    trendStrength: z.number().finite().nullable(),
    seasonalStrength: z.number().finite().nullable(),
    changePoints: z.array(z.number().int().positive()),
    peltPenalty: z.number().finite().nonnegative(),
  }).strict()),
}).strict()

export type SeriesStructureEvidence = z.infer<typeof seriesStructureEvidenceSchema>

export type SensitivityEvidenceProblem = { readonly kind: 'invalid-sensitivity-evidence'; readonly detail: string }

const parseWith = <Schema extends z.ZodTypeAny>(schema: Schema, value: unknown): Result<z.infer<Schema>, SensitivityEvidenceProblem> => {
  const parsed = schema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ kind: 'invalid-sensitivity-evidence', detail: z.prettifyError(parsed.error) })
}

export function parseLinearRefutationEvidence(value: unknown): Result<LinearRefutationEvidence, SensitivityEvidenceProblem> {
  const parsed = parseWith(linearRefutationEvidenceSchema, value)
  if (!parsed.ok) return parsed
  const { ljungBoxLags, ljungBoxStatistics, ljungBoxPValues } = parsed.value
  if (ljungBoxLags.length !== ljungBoxStatistics.length || ljungBoxLags.length !== ljungBoxPValues.length) {
    return err({ kind: 'invalid-sensitivity-evidence', detail: 'The Ljung-Box lags, statistics, and p-values differ in length.' })
  }
  return parsed
}

export const parseDmlRefutationEvidence = (value: unknown): Result<DmlRefutationEvidence, SensitivityEvidenceProblem> => parseWith(dmlRefutationEvidenceSchema, value)

export function parseUnobservedConfoundingEvidence(value: unknown): Result<UnobservedConfoundingEvidence, SensitivityEvidenceProblem> {
  const parsed = parseWith(unobservedConfoundingEvidenceSchema, value)
  if (!parsed.ok) return parsed
  const { kappaT, kappaY, effects } = parsed.value
  if (effects.length !== kappaT.length || effects.some((row) => row.length !== kappaY.length)) {
    return err({ kind: 'invalid-sensitivity-evidence', detail: 'The simulated grid does not match its kappa axes.' })
  }
  return parsed
}

export const parseSeriesStructureEvidence = (value: unknown): Result<SeriesStructureEvidence, SensitivityEvidenceProblem> => parseWith(seriesStructureEvidenceSchema, value)

/** A refuter's reading: what it should show if the estimate is real, and what it did show. */
interface RefuterFactBase {
  readonly id: 'placebo' | 'data-subset' | 'random-common-cause'
  readonly method: MethodId
  readonly original: number
  readonly refuted: number
}

/** Each refuter carries only the interpretation its procedure actually supports. */
export type RefuterFact =
  | RefuterFactBase & {
      readonly interpretation: {
        readonly kind: 'reference-distance'
        readonly reference: 'zero' | 'original-estimate'
        readonly reading: string
      }
    }
  | RefuterFactBase & {
      readonly interpretation: {
        readonly kind: 'mean-shift-test'
        readonly nullHypothesis: string
        readonly pValue: number
        readonly alpha: 0.05
      }
    }

export interface DiagnosticFact {
  readonly id: 'ljung-box' | 'shapiro-wilk' | 'durbin-watson'
  readonly method: MethodId
  readonly reading: string
}

interface RunIdentity {
  readonly id: SensitivityRunId
  readonly estimationRun: EstimationRunId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly columns: NonEmptyArray<StudyVariable>
}

export type SensitivityRunArtifact =
  | RunIdentity & { readonly kind: 'linear-refutation-run'; readonly configuration: RefutationConfiguration; readonly evidence: LinearRefutationEvidence; readonly refuters: NonEmptyArray<RefuterFact>; readonly diagnostics: NonEmptyArray<DiagnosticFact> }
  | RunIdentity & { readonly kind: 'unobserved-confounding-run'; readonly configuration: UnobservedConfiguration; readonly evidence: UnobservedConfoundingEvidence }
  | RunIdentity & { readonly kind: 'dml-refutation-run'; readonly configuration: DmlRefutationConfiguration; readonly evidence: DmlRefutationEvidence; readonly refuters: NonEmptyArray<RefuterFact> }

export type ProbeEligibility =
  | { readonly kind: 'eligible' }
  | { readonly kind: 'refused'; readonly reason: string }

/** Which probes an estimation run supports; a probe never reads a run it was not written for. */
export function probeEligibility(probe: SensitivityProbe, run: EstimationRunArtifact | null, treatmentIsBinary: boolean | null): ProbeEligibility {
  if (run === null) return { kind: 'refused', reason: 'Choose an estimation run first.' }
  switch (probe) {
    case 'linear-refutation':
      return run.kind === 'backdoor-linear-run'
        ? { kind: 'eligible' }
        : { kind: 'refused', reason: 'The DoWhy refuters refit the linear back-door estimate; choose an adjusted linear regression run.' }
    case 'unobserved-confounding':
      if (run.kind !== 'backdoor-linear-run') return { kind: 'refused', reason: 'The simulated confounder refits the linear back-door estimate; choose an adjusted linear regression run.' }
      if (run.estimate.adjustmentSet.length === 0) return { kind: 'refused', reason: 'The simulated confounder is sized from the observed common causes, and this study adjusts for none.' }
      if (treatmentIsBinary === false) return { kind: 'refused', reason: 'The simulation flips a binary treatment; this treatment is not 0 or 1.' }
      return { kind: 'eligible' }
    case 'dml-refutation':
      return run.kind === 'double-ml-run'
        ? { kind: 'eligible' }
        : { kind: 'refused', reason: 'The DML batch repeats a double machine learning fit; choose a DML run.' }
    default:
      return assertNever(probe)
  }
}

export const describeProbe = (probe: SensitivityProbe): string => {
  switch (probe) {
    case 'linear-refutation': return 'Perturbation and residual probes'
    case 'unobserved-confounding': return 'Simulated unmeasured confounder'
    case 'dml-refutation': return 'Double machine learning probe batch'
    default: return assertNever(probe)
  }
}

/** ruptures' rule of thumb for the L2 penalty: log(n) times the noise variance, here the series variance. */
export function defaultPeltPenalty(values: readonly number[]): number {
  const n = values.length
  if (n < 2) return 0
  const mean = values.reduce((sum, value) => sum + value, 0) / n
  const variance = values.reduce((sum, value) => sum + (value - mean) ** 2, 0) / n
  return Math.log(n) * variance
}
