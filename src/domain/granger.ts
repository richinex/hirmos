import { z } from 'zod'
import { brand, err, isNonEmpty, ok, type Brand, type Result } from './dop'
import type { ColumnId, NumericColumnSelection } from './dataset'
import { GRANGER_SSR_F_METHOD_ID, type CaveatEvaluation, type MethodDefinition, type MethodEligibility } from './methods'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId, StationarityEvidenceArtifact } from './preprocessing'
import { levelModelVerdict } from './stationarityAssessment'

/**
 * The Granger SSR F test: whether past values of one series add predictive information about
 * another beyond its own past. A data diagnostic beside the stationarity, break and seasonality
 * checks; it says nothing about intervention, so it never feeds the DAG as evidence.
 */

export const grangerSsrEvidenceSchema = z.object({
  kind: z.literal('grangerSsrF'),
  observations: z.number().int().positive(),
  maxLag: z.number().int().min(1).max(20),
  tests: z.array(z.object({
    lag: z.number().int().positive(),
    statistic: z.number().finite(),
    pValue: z.number().finite().min(0).max(1),
  }).strict()),
}).strict()

export type GrangerSsrEvidence = z.infer<typeof grangerSsrEvidenceSchema>

export type GrangerBoundaryProblem = {
  readonly kind: 'invalid-granger-result'
  readonly detail: string
}

export function parseGrangerSsrEvidence(
  value: unknown,
): Result<GrangerSsrEvidence, GrangerBoundaryProblem> {
  const parsed = grangerSsrEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-granger-result', detail: z.prettifyError(parsed.error) })
  }
  if (parsed.data.tests.length !== parsed.data.maxLag
    || parsed.data.tests.some((test, index) => test.lag !== index + 1)) {
    return err({ kind: 'invalid-granger-result', detail: 'Granger evidence must contain one ordered result per requested lag.' })
  }
  return ok(parsed.data)
}

export const GRANGER_LAG_OPTIONS = [1, 2, 3, 4, 6, 8, 12] as const
export type GrangerLag = (typeof GRANGER_LAG_OPTIONS)[number]

export type GrangerEvidenceId = Brand<string, 'GrangerEvidenceId'>
export const newGrangerEvidenceId = (): GrangerEvidenceId => brand<string, 'GrangerEvidenceId'>(crypto.randomUUID())

export interface GrangerEvidenceArtifact {
  readonly kind: 'granger-evidence'
  readonly id: GrangerEvidenceId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly method: typeof GRANGER_SSR_F_METHOD_ID
  readonly target: NumericColumnSelection
  readonly candidateCause: NumericColumnSelection
  readonly maxLag: GrangerLag
  readonly eligibility: Exclude<MethodEligibility, { readonly kind: 'refused' }>
  readonly result: GrangerSsrEvidence
}

export interface GrangerSpecification {
  readonly target: ColumnId
  readonly candidateCause: ColumnId
  readonly maxLag: GrangerLag
}

export type GrangerReadinessProblem =
  | { readonly kind: 'time-series-required' }
  | { readonly kind: 'target-required' }
  | { readonly kind: 'candidate-cause-required' }
  | { readonly kind: 'distinct-pair-required' }
  | { readonly kind: 'too-few-observations'; readonly required: number; readonly available: number }

/** The pair and lag order the test can run with; the observation floor is the statsmodels one, 3 × lag + 2. */
export function readyGrangerSpecification(
  target: ColumnId | null,
  candidateCause: ColumnId | null,
  maxLag: GrangerLag,
  prepared: PreparedDatasetArtifact,
): Result<GrangerSpecification, GrangerReadinessProblem> {
  if (prepared.kind !== 'prepared-time-series') return err({ kind: 'time-series-required' })
  if (target === null) return err({ kind: 'target-required' })
  if (candidateCause === null) return err({ kind: 'candidate-cause-required' })
  if (target === candidateCause) return err({ kind: 'distinct-pair-required' })
  const required = 3 * maxLag + 2
  return prepared.observations < required
    ? err({ kind: 'too-few-observations', required, available: prepared.observations })
    : ok({ target, candidateCause, maxLag })
}

export function describeGrangerReadiness(problem: GrangerReadinessProblem): string {
  switch (problem.kind) {
    case 'time-series-required': return 'This test needs a regular time series. The prepared dataset holds independent rows.'
    case 'target-required': return 'Choose the target series whose future values are being predicted.'
    case 'candidate-cause-required': return 'Choose the candidate cause whose past values are added to the model.'
    case 'distinct-pair-required': return 'Target and candidate cause must be different variables.'
    case 'too-few-observations': return `This lag order needs at least ${problem.required} rows; ${problem.available} are available. Lower the lag order or use more rows.`
    default: { const exhaustive: never = problem; return exhaustive }
  }
}

/** Orders whose test rejects at α: the sentence a reader takes away, with its boundary. */
export function describeGrangerVerdict(artifact: GrangerEvidenceArtifact, alpha = 0.05): string {
  const rejecting = artifact.result.tests.filter((test) => test.pValue < alpha).map((test) => test.lag)
  const pair = `past ${artifact.candidateCause.name} values`
  if (rejecting.length === 0) return `At α ${alpha}, ${pair} add no predictive information about ${artifact.target.name} at any order up to ${artifact.maxLag}.`
  const orders = rejecting.length === 1 ? `lag order ${rejecting[0]}` : `lag orders ${rejecting.join(', ')}`
  return `At α ${alpha}, ${pair} add predictive information about ${artifact.target.name} at ${orders}.`
}

type Satisfied = Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>
type Unresolved = Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>
type Violated = Extract<CaveatEvaluation, { readonly kind: 'violated' }>

/**
 * The test's conditions against the project's evidence: the sampling grid from the prepared version,
 * the stationarity of the chosen pair from the recorded battery. Reading rules are not conditions
 * and are left out; the card shows them without a status.
 */
export function evaluateGrangerEligibility(
  method: MethodDefinition,
  prepared: PreparedDatasetArtifact,
  stationarity: StationarityEvidenceArtifact | null,
  pair: { readonly target: NumericColumnSelection; readonly candidateCause: NumericColumnSelection } | null,
): MethodEligibility {
  const satisfied: Satisfied[] = []
  const unresolved: Unresolved[] = []
  const violations: Violated[] = []
  for (const caveat of method.caveats) {
    switch (caveat.category) {
      case 'interpretation': break
      case 'sampling-structure':
        if (prepared.kind === 'prepared-time-series') satisfied.push({ kind: 'satisfied', caveat, evidence: `Prepared as a regular ${prepared.sampling.frequency} time series with an explicit time column.` })
        else violations.push({ kind: 'violated', caveat, evidence: 'The prepared dataset is not a regular time series, so row order carries no lag.' })
        break
      case 'stationarity-and-dynamics': {
        if (stationarity === null) { unresolved.push({ kind: 'unresolved', caveat, missingEvidence: 'Run the stationarity tests above for this prepared dataset version.' }); break }
        if (stationarity.transform.kind === 'difference') { satisfied.push({ kind: 'satisfied', caveat, evidence: 'The stationarity view is the first difference; the test reads the prepared levels, so difference the series in the recipe before trusting a level result.' }); break }
        if (pair === null) { unresolved.push({ kind: 'unresolved', caveat, missingEvidence: 'Choose the candidate cause and the target; their stationarity verdicts are checked here.' }); break }
        const verdicts = [pair.candidateCause, pair.target].map((column) => {
          const assessment = stationarity.variables.find((variable) => variable.column === column.id)?.assessment ?? null
          // The shared verdict names a level regression and points at the Data Studio; this card is the Data Studio, and the regression is the test's own.
          return assessment !== null && assessment.kind === 'differenceStationary'
            ? { kind: 'unresolved' as const, reason: `${column.name} is I(1) in levels, so the F test can reject spuriously; a differenced version is the safer test.` }
            : levelModelVerdict(column.name, assessment)
        })
        // A stationarity verdict is itself a hypothesis test, so it warns rather than refuses.
        const open = verdicts.filter((verdict) => verdict.kind !== 'allowed')
        if (open.length > 0) unresolved.push({ kind: 'unresolved', caveat, missingEvidence: open.map((verdict) => verdict.reason).join(' ') })
        else satisfied.push({ kind: 'satisfied', caveat, evidence: verdicts.map((verdict) => verdict.reason).join(' ') })
        break
      }
      default:
        unresolved.push({ kind: 'unresolved', caveat, missingEvidence: 'No recorded evidence covers this condition.' })
    }
  }
  if (isNonEmpty(violations)) return { kind: 'refused', violations }
  if (isNonEmpty(unresolved)) return { kind: 'caution', satisfied, unresolved }
  return { kind: 'eligible', satisfied }
}
