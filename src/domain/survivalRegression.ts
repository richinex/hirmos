import { z } from 'zod'
import type { NumericColumnSelection } from './dataset'
import { err, ok, type NonEmptyArray } from './dop'

export interface AalenConfiguration {
  readonly kind: 'aalen'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly covariates: NonEmptyArray<NumericColumnSelection>
}

export const forestSettingsSchema = z.object({
  trees: z.number().int().min(1).max(5000),
  mtry: z.number().int().positive(),
  seed: z.number().int().min(1).max(0xffffffff),
  minNodeSize: z.number().int().positive(),
  minBucket: z.number().int().positive(),
  splitRule: z.enum(['logRank', 'extraTrees']),
}).strict()

export type ForestSettings = z.infer<typeof forestSettingsSchema>

export interface ForestConfiguration {
  readonly kind: 'survival-forest'
  readonly duration: NumericColumnSelection
  readonly event: NumericColumnSelection
  readonly covariates: NonEmptyArray<NumericColumnSelection>
  readonly categorical: readonly NumericColumnSelection[]
  readonly settings: ForestSettings
  /** Zero-based prepared row; display it to the user as row + 1. */
  readonly predictionRow: number
}

const finite = z.number().finite()
const probability = finite.min(0).max(1)
const available = <T extends z.ZodTypeAny>(value: T) => z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('recorded'), result: value }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
])

// Aalen estimates and bands are signed coefficients, never probabilities or hazard ratios.
export const aalenEvidenceSchema = z.object({
  kind: z.literal('aalen'),
  observations: z.number().int().min(2),
  events: z.number().int().positive(),
  fittedEvents: z.number().int().positive(),
  lastTime: finite.positive(),
  coefficients: z.array(z.tuple([finite, finite, finite.nonnegative(), finite, probability])).min(2),
  curves: z.array(z.array(z.tuple([finite.nonnegative(), finite, finite, finite])).min(2)).min(2),
  chisq: finite.nonnegative(),
  degreesOfFreedom: z.number().int().positive(),
  pValue: probability,
}).strict().superRefine((value, context) => {
  const issue = (message: string) => context.addIssue({ code: 'custom', message })
  if (value.fittedEvents > value.events || value.events > value.observations) issue('Aalen event counts are inconsistent.')
  if (value.coefficients.length !== value.curves.length || value.coefficients.length !== value.degreesOfFreedom + 1) issue('Aalen terms and curves do not agree.')
  for (const curve of value.curves) {
    if (curve.at(-1)?.[0] !== value.lastTime) issue('An Aalen curve ends at a different fitted time.')
    if (curve.some((point, i) => point[2] > point[1] || point[1] > point[3] || (i > 0 && curve[i - 1]![0] >= point[0]))) issue('Aalen curves need ordered times and intervals around their estimates.')
  }
})
export type AalenEvidence = z.infer<typeof aalenEvidenceSchema>

export const forestEvidenceSchema = z.object({
  kind: z.literal('survivalForest'),
  observations: z.number().int().min(2),
  events: z.number().int().positive(),
  trees: z.number().int().positive(),
  importance: z.array(available(finite)).min(1),
  concordance: available(probability),
  predictionRow: z.number().int().nonnegative(),
  profile: z.array(finite).min(1),
  predictionTimes: z.array(finite.nonnegative()).min(1),
  survival: z.array(probability).min(1),
  cumulativeHazard: z.array(finite.nonnegative()).min(1),
}).strict().superRefine((value, context) => {
  const issue = (message: string) => context.addIssue({ code: 'custom', message })
  if (value.events > value.observations || value.predictionRow >= value.observations) issue('Forest observation counts or prediction row are invalid.')
  if (value.profile.length !== value.importance.length) issue('Forest covariates and importance values do not agree.')
  if (value.predictionTimes.length !== value.survival.length || value.predictionTimes.length !== value.cumulativeHazard.length) issue('Forest curve lengths do not agree.')
  for (let i = 0; i < value.predictionTimes.length; i++) {
    if (i > 0 && (value.predictionTimes[i - 1]! >= value.predictionTimes[i]! || value.cumulativeHazard[i - 1]! > value.cumulativeHazard[i]!)) issue('Forest times and cumulative hazard must increase without reversing.')
    if (Math.abs(Math.exp(-value.cumulativeHazard[i]!) - value.survival[i]!) > 1e-12) issue('Forest survival does not match the cumulative hazard.')
  }
})
export type ForestEvidence = z.infer<typeof forestEvidenceSchema>

const parseEvidence = <T>(schema: z.ZodType<T>, value: unknown) => {
  const parsed = schema.safeParse(value)
  return parsed.success ? ok(parsed.data) : err({ kind: 'invalid-survival-evidence' as const, detail: parsed.error.issues.map((issue) => issue.message).join(' ') })
}
export const parseAalenEvidence = (value: unknown) => parseEvidence(aalenEvidenceSchema, value)
export const parseForestEvidence = (value: unknown) => parseEvidence(forestEvidenceSchema, value)
