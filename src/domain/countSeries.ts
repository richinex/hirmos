import { z } from 'zod'
import type { ColumnId, NumericColumnSelection } from './dataset'
import { brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { PreparedDatasetVersionId } from './preprocessing'

export type CountSeriesModelId = Brand<string, 'CountSeriesModelId'>
export const newCountSeriesModelId = (): CountSeriesModelId => brand<string, 'CountSeriesModelId'>(crypto.randomUUID())

export type CountSeriesLink = 'identity' | 'log'

export interface CountSeriesInterventionScanSpecification {
  readonly outcome: ColumnId
  readonly link: CountSeriesLink
  readonly pastObservationLags: NonEmptyArray<number>
  readonly pastMeanLags: NonEmptyArray<number>
  readonly candidateStart: number
  readonly candidateEnd: number
  readonly delta: number
}

export const countSeriesInterventionScanEvidenceSchema = z.object({
  kind: z.literal('countSeriesInterventionScan'),
  observations: z.number().int().positive(),
  outcome: z.number().int().nonnegative(),
  link: z.enum(['identity', 'log']),
  pastObservationLags: z.array(z.number().int().positive()).min(1),
  pastMeanLags: z.array(z.number().int().positive()).min(1),
  parameters: z.array(z.number().finite()).min(3),
  fittedMeans: z.array(z.number().finite()),
  residuals: z.array(z.number().finite()),
  logLikelihood: z.number().finite(),
  size: z.number().finite().positive(),
  dispersion: z.number().finite().positive(),
  candidates: z.array(z.object({
    referencePoint: z.number().int().nonnegative(),
    scoreStatistic: z.number().finite().nonnegative(),
  }).strict()).min(1),
  strongestReferencePoint: z.number().int().nonnegative(),
  delta: z.number().finite().min(0).max(1),
}).strict()

export type CountSeriesInterventionScanEvidence = z.infer<typeof countSeriesInterventionScanEvidenceSchema>

export type CountSeriesBoundaryProblem = { readonly kind: 'invalid-count-series-result'; readonly detail: string }

export function parseCountSeriesInterventionScanEvidence(value: unknown): Result<CountSeriesInterventionScanEvidence, CountSeriesBoundaryProblem> {
  const parsed = countSeriesInterventionScanEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-count-series-result', detail: z.prettifyError(parsed.error) })
  const evidence = parsed.data
  if (evidence.fittedMeans.length !== evidence.observations || evidence.residuals.length !== evidence.observations) {
    return err({ kind: 'invalid-count-series-result', detail: 'The fitted and residual series must match the observation count.' })
  }
  if (!evidence.candidates.some((candidate) => candidate.referencePoint === evidence.strongestReferencePoint)) {
    return err({ kind: 'invalid-count-series-result', detail: 'The strongest reference point must occur in the candidate scan.' })
  }
  const maximum = evidence.candidates.reduce((value, candidate) => Math.max(value, candidate.scoreStatistic), Number.NEGATIVE_INFINITY)
  const strongest = evidence.candidates.find((candidate) => candidate.referencePoint === evidence.strongestReferencePoint)
  if (strongest === undefined || Math.abs(strongest.scoreStatistic - maximum) > 1e-12 * Math.max(1, Math.abs(maximum))) {
    return err({ kind: 'invalid-count-series-result', detail: 'The strongest reference point does not carry the maximum score statistic.' })
  }
  return ok(evidence)
}

export interface CountSeriesModelArtifact {
  readonly kind: 'count-series-model'
  readonly id: CountSeriesModelId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly outcome: NumericColumnSelection
  readonly specification: Omit<CountSeriesInterventionScanSpecification, 'outcome'>
  readonly result: CountSeriesInterventionScanEvidence
}

export type CountSeriesReadinessProblem =
  | { readonly kind: 'outcome-required' }
  | { readonly kind: 'count-outcome-required' }
  | { readonly kind: 'invalid-lags' }
  | { readonly kind: 'invalid-candidate-window' }
  | { readonly kind: 'invalid-delta' }

export function readyCountSeriesSpecification(
  specification: CountSeriesInterventionScanSpecification,
  observations: number,
  outcomeValues: readonly number[] | null,
): Result<CountSeriesInterventionScanSpecification, CountSeriesReadinessProblem> {
  if (specification.outcome.length === 0) return err({ kind: 'outcome-required' })
  if (outcomeValues !== null && outcomeValues.some((value) => !Number.isInteger(value) || value < 0)) return err({ kind: 'count-outcome-required' })
  if (!isNonEmpty(specification.pastObservationLags) || !isNonEmpty(specification.pastMeanLags)) return err({ kind: 'invalid-lags' })
  if (specification.candidateStart < 1 || specification.candidateEnd >= observations || specification.candidateStart > specification.candidateEnd) return err({ kind: 'invalid-candidate-window' })
  if (!Number.isFinite(specification.delta) || specification.delta < 0 || specification.delta > 1) return err({ kind: 'invalid-delta' })
  return ok(specification)
}

export function describeCountSeriesReadiness(problem: CountSeriesReadinessProblem): string {
  switch (problem.kind) {
    case 'outcome-required': return 'Choose the count series to model.'
    case 'count-outcome-required': return 'INGARCH requires non-negative whole-number observations.'
    case 'invalid-lags': return 'Enter at least one positive count lag and one positive mean lag.'
    case 'invalid-candidate-window': return 'The candidate window must lie inside the prepared series and start after the first row.'
    case 'invalid-delta': return 'The intervention shape δ must be between 0 and 1.'
  }
}
