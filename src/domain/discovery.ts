import { z } from 'zod'
import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { ColumnId, NumericColumnSelection } from './dataset'
import {
  GRANGER_SSR_F_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  type CaveatEvaluation,
  type MethodDefinition,
  type MethodEligibility,
} from './methods'
import type {
  PreparedDatasetArtifact,
  PreparedDatasetVersionId,
  StationarityEvidenceArtifact,
} from './preprocessing'

export const pcmciPlusEvidenceSchema = z.object({
  kind: z.literal('pcmciPlus'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(32),
  tauMax: z.number().int().min(1).max(20),
  pcAlpha: z.number().finite().positive().max(1),
  graph: z.array(z.array(z.array(z.string().max(3)))),
  pMatrix: z.array(z.array(z.array(z.number().finite().min(0).max(1)))),
  valMatrix: z.array(z.array(z.array(z.number().finite().min(-1).max(1)))),
}).strict()

export type PcmciPlusEvidence = z.infer<typeof pcmciPlusEvidenceSchema>

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

export type PcmciPlusBoundaryProblem = {
  readonly kind: 'invalid-pcmci-plus-result'
  readonly detail: string
}

export type GrangerBoundaryProblem = {
  readonly kind: 'invalid-granger-result'
  readonly detail: string
}

const hasMatrixShape = <Value>(
  matrix: readonly (readonly (readonly Value[])[])[],
  variables: number,
  lags: number,
): boolean => matrix.length === variables
  && matrix.every((targets) => targets.length === variables
    && targets.every((lagValues) => lagValues.length === lags))

export function parsePcmciPlusEvidence(value: unknown): Result<PcmciPlusEvidence, PcmciPlusBoundaryProblem> {
  const parsed = pcmciPlusEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-pcmci-plus-result', detail: z.prettifyError(parsed.error) })
  }
  const lags = parsed.data.tauMax + 1
  if (!hasMatrixShape(parsed.data.graph, parsed.data.variables, lags)
    || !hasMatrixShape(parsed.data.pMatrix, parsed.data.variables, lags)
    || !hasMatrixShape(parsed.data.valMatrix, parsed.data.variables, lags)) {
    return err({ kind: 'invalid-pcmci-plus-result', detail: 'PCMCI+ evidence matrices have inconsistent dimensions.' })
  }
  return ok(parsed.data)
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

export type DiscoveryRunId = Brand<string, 'DiscoveryRunId'>

export const DISCOVERY_LAG_OPTIONS = [1, 2, 3, 4, 6, 8, 12, 20] as const
export type DiscoveryLag = (typeof DISCOVERY_LAG_OPTIONS)[number]

export const PCMCI_ALPHA_OPTIONS = [0.01, 0.025, 0.05, 0.1] as const
export type PcmciAlpha = (typeof PCMCI_ALPHA_OPTIONS)[number]

export type DiscoveryMethodChoice = 'pcmci-plus' | 'granger-ssr-f'
export type AcceptedDiscoveryEligibility = Exclude<MethodEligibility, { readonly kind: 'refused' }>

export type ColumnChoice =
  | { readonly kind: 'unselected' }
  | { readonly kind: 'selected'; readonly column: ColumnId }

export type DiscoveryConfiguration =
  | {
      readonly kind: 'pcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'granger-ssr-f'
      readonly target: ColumnChoice
      readonly candidateCause: ColumnChoice
      readonly maxLag: DiscoveryLag
    }

export type DiscoveryRunArtifact =
  | {
      readonly kind: 'pcmci-plus-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof PCMCI_PLUS_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: PcmciPlusEvidence
    }
  | {
      readonly kind: 'granger-ssr-f-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof GRANGER_SSR_F_METHOD_ID
      readonly target: NumericColumnSelection
      readonly candidateCause: NumericColumnSelection
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: GrangerSsrEvidence
    }

export type DiscoveryRunProblem =
  | { readonly kind: 'materialization-refused'; readonly detail: string }
  | { readonly kind: 'missing-values-remain'; readonly cells: number }
  | { readonly kind: 'analysis-refused'; readonly detail: string }
  | { readonly kind: 'execution-unavailable'; readonly detail: string }

export type DiscoveryJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running' }
  | { readonly kind: 'failed'; readonly problem: DiscoveryRunProblem }
  | { readonly kind: 'succeeded'; readonly artifact: DiscoveryRunArtifact }

export interface DiscoveryDraft {
  readonly configuration: DiscoveryConfiguration
  readonly job: DiscoveryJob
}

export type DiscoveryEvent =
  | { readonly type: 'method-selected'; readonly method: DiscoveryMethodChoice }
  | { readonly type: 'tau-max-selected'; readonly value: DiscoveryLag }
  | { readonly type: 'pc-alpha-selected'; readonly value: PcmciAlpha }
  | { readonly type: 'target-selected'; readonly column: ColumnChoice }
  | { readonly type: 'candidate-cause-selected'; readonly column: ColumnChoice }
  | { readonly type: 'max-lag-selected'; readonly value: DiscoveryLag }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-failed'; readonly problem: DiscoveryRunProblem }
  | { readonly type: 'run-succeeded'; readonly artifact: DiscoveryRunArtifact }

export const INITIAL_DISCOVERY_DRAFT: DiscoveryDraft = {
  configuration: { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 },
  job: { kind: 'idle' },
}

export function stepDiscovery(state: DiscoveryDraft, event: DiscoveryEvent): DiscoveryDraft {
  switch (event.type) {
    case 'method-selected':
      return {
        configuration: event.method === 'pcmci-plus'
          ? { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 }
          : {
              kind: 'granger-ssr-f',
              target: { kind: 'unselected' },
              candidateCause: { kind: 'unselected' },
              maxLag: 4,
            },
        job: { kind: 'idle' },
      }
    case 'tau-max-selected':
      return state.configuration.kind === 'pcmci-plus'
        ? { configuration: { ...state.configuration, tauMax: event.value }, job: { kind: 'idle' } }
        : state
    case 'pc-alpha-selected':
      return state.configuration.kind === 'pcmci-plus'
        ? { configuration: { ...state.configuration, pcAlpha: event.value }, job: { kind: 'idle' } }
        : state
    case 'target-selected':
      return state.configuration.kind === 'granger-ssr-f'
        ? { configuration: { ...state.configuration, target: event.column }, job: { kind: 'idle' } }
        : state
    case 'candidate-cause-selected':
      return state.configuration.kind === 'granger-ssr-f'
        ? { configuration: { ...state.configuration, candidateCause: event.column }, job: { kind: 'idle' } }
        : state
    case 'max-lag-selected':
      return state.configuration.kind === 'granger-ssr-f'
        ? { configuration: { ...state.configuration, maxLag: event.value }, job: { kind: 'idle' } }
        : state
    case 'run-started': return { ...state, job: { kind: 'running' } }
    case 'run-failed': return { ...state, job: { kind: 'failed', problem: event.problem } }
    case 'run-succeeded': return { ...state, job: { kind: 'succeeded', artifact: event.artifact } }
    default: return assertNever(event)
  }
}

export type ReadyDiscoverySpecification =
  | {
      readonly kind: 'pcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'granger-ssr-f'
      readonly target: ColumnId
      readonly candidateCause: ColumnId
      readonly maxLag: DiscoveryLag
    }

export type DiscoveryReadinessProblem =
  | { readonly kind: 'time-series-required' }
  | { readonly kind: 'at-least-two-variables-required' }
  | { readonly kind: 'target-required' }
  | { readonly kind: 'candidate-cause-required' }
  | { readonly kind: 'distinct-pair-required' }
  | { readonly kind: 'too-few-observations'; readonly required: number; readonly available: number }
  | { readonly kind: 'dense-browser-boundary-required' }

export function readyDiscoverySpecification(
  configuration: DiscoveryConfiguration,
  prepared: PreparedDatasetArtifact,
): Result<ReadyDiscoverySpecification, DiscoveryReadinessProblem> {
  if (prepared.kind !== 'prepared-time-series') return err({ kind: 'time-series-required' })
  if (prepared.missingness.kind !== 'not-present') return err({ kind: 'dense-browser-boundary-required' })
  switch (configuration.kind) {
    case 'pcmci-plus': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = Math.max(2 * configuration.tauMax + 16, 24)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'granger-ssr-f': {
      if (configuration.target.kind === 'unselected') return err({ kind: 'target-required' })
      if (configuration.candidateCause.kind === 'unselected') return err({ kind: 'candidate-cause-required' })
      if (configuration.target.column === configuration.candidateCause.column) {
        return err({ kind: 'distinct-pair-required' })
      }
      const required = 3 * configuration.maxLag + 2
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok({
            kind: 'granger-ssr-f',
            target: configuration.target.column,
            candidateCause: configuration.candidateCause.column,
            maxLag: configuration.maxLag,
          })
    }
    default: return assertNever(configuration)
  }
}

export function evaluateDiscoveryEligibility(
  method: MethodDefinition,
  prepared: PreparedDatasetArtifact,
  stationarity: StationarityEvidenceArtifact | null,
): MethodEligibility {
  const [firstCaveat] = method.caveats
  if (prepared.kind !== 'prepared-time-series') {
    const samplingCaveat = method.caveats.find((caveat) => caveat.category === 'sampling-structure') ?? firstCaveat
    return {
      kind: 'refused',
      violations: [{
        kind: 'violated',
        caveat: samplingCaveat,
        evidence: 'This prepared artifact declares independent observations, so row order has no temporal meaning.',
      }],
    }
  }

  const satisfied: Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[] = []
  const unresolved: Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>[] = []
  for (const caveat of method.caveats) {
    if (caveat.category === 'sampling-structure') {
      satisfied.push({
        kind: 'satisfied',
        caveat,
        evidence: `Prepared as a regular ${prepared.sampling.frequency} time series with an explicit time column.`,
      })
      continue
    }
    if (caveat.category === 'missingness' && prepared.missingness.kind === 'not-present') {
      satisfied.push({ kind: 'satisfied', caveat, evidence: 'The prepared numeric matrix contains no missing cells.' })
      continue
    }
    unresolved.push({
      kind: 'unresolved',
      caveat,
      missingEvidence: caveat.category === 'stationarity-and-dynamics'
        ? stationarity === null
          ? 'No stationarity evidence has been run for this prepared version. The run remains available with caution.'
          : `Stationarity tests exist for ${stationarity.observations} observations, but Hirmos does not turn them into an automatic verdict.`
        : 'This requirement needs scientific judgment or evidence beyond the structural dataset checks.',
    })
  }
  if (!isNonEmpty(unresolved)) return { kind: 'eligible', satisfied }
  return { kind: 'caution', satisfied, unresolved }
}

export const newDiscoveryRunId = (): DiscoveryRunId =>
  brand<string, 'DiscoveryRunId'>(crypto.randomUUID())

export function describeDiscoveryReadiness(problem: DiscoveryReadinessProblem): string {
  switch (problem.kind) {
    case 'time-series-required': return 'Temporal discovery is refused because this dataset declares independent observations.'
    case 'at-least-two-variables-required': return 'PCMCI+ requires at least two prepared variables.'
    case 'target-required': return 'Choose the target series whose future values are being predicted.'
    case 'candidate-cause-required': return 'Choose the candidate cause whose past values are added to the model.'
    case 'distinct-pair-required': return 'Target and candidate cause must be different variables.'
    case 'too-few-observations': return `This configuration needs at least ${problem.required} observations; ${problem.available} are available.`
    case 'dense-browser-boundary-required': return 'The current browser command needs a dense prepared matrix. Tigramite-mask execution will use its own later boundary.'
    default: return assertNever(problem)
  }
}

export function describeDiscoveryRunProblem(problem: DiscoveryRunProblem): string {
  switch (problem.kind) {
    case 'materialization-refused': return problem.detail
    case 'missing-values-remain': return `${problem.cells} missing cells remain in the numeric matrix.`
    case 'analysis-refused': return problem.detail
    case 'execution-unavailable': return problem.detail
    default: return assertNever(problem)
  }
}
