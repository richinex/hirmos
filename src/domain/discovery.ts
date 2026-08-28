import { z } from 'zod'
import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { ColumnId, NumericColumnSelection } from './dataset'
import {
  GRANGER_SSR_F_METHOD_ID,
  DYNOTEARS_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  OCSE_METHOD_ID,
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

export const lpcmciEvidenceSchema = pcmciPlusEvidenceSchema.extend({
  kind: z.literal('lpcmci'),
}).strict()

export type LpcmciEvidence = z.infer<typeof lpcmciEvidenceSchema>

export const dynotearsEvidenceSchema = z.object({
  kind: z.literal('dynotears'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(12),
  maxLag: z.number().int().min(1).max(6),
  lambdaW: z.number().finite().nonnegative(),
  lambdaA: z.number().finite().nonnegative(),
  contemporaneousWeights: z.array(z.array(z.number().finite())),
  laggedWeights: z.array(z.array(z.array(z.number().finite()))),
}).strict()

export type DynotearsEvidence = z.infer<typeof dynotearsEvidenceSchema>

export const ocseEvidenceSchema = z.object({
  kind: z.literal('ocse'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(12),
  maxLag: z.number().int().min(1).max(8),
  alpha: z.number().finite().positive().max(1),
  nShuffles: z.number().int().min(20).max(2_000),
  method: z.enum(['gaussian', 'knn']),
  k: z.number().int().min(1).max(20),
  seed: z.literal(42),
  edges: z.array(z.object({
    source: z.number().int().nonnegative(),
    target: z.number().int().nonnegative(),
    lag: z.number().int().positive(),
    cmi: z.number().finite().nonnegative(),
    pValue: z.number().finite().min(0).max(1),
  }).strict()),
}).strict()

export type OcseEvidence = z.infer<typeof ocseEvidenceSchema>

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

export type DiscoveryMatrixBoundaryProblem = {
  readonly kind: 'invalid-discovery-matrix-result'
  readonly method: 'LPCMCI' | 'DYNOTEARS' | 'oCSE'
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

export function parseLpcmciEvidence(value: unknown): Result<LpcmciEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = lpcmciEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'LPCMCI', detail: z.prettifyError(parsed.error) })
  }
  const lags = parsed.data.tauMax + 1
  if (!hasMatrixShape(parsed.data.graph, parsed.data.variables, lags)
    || !hasMatrixShape(parsed.data.pMatrix, parsed.data.variables, lags)
    || !hasMatrixShape(parsed.data.valMatrix, parsed.data.variables, lags)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'LPCMCI', detail: 'LPCMCI evidence matrices have inconsistent dimensions.' })
  }
  return ok(parsed.data)
}

export function parseDynotearsEvidence(value: unknown): Result<DynotearsEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = dynotearsEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'DYNOTEARS', detail: z.prettifyError(parsed.error) })
  }
  const square = (matrix: readonly (readonly number[])[]) => matrix.length === parsed.data.variables
    && matrix.every((row) => row.length === parsed.data.variables)
  if (!square(parsed.data.contemporaneousWeights)
    || parsed.data.laggedWeights.length !== parsed.data.maxLag
    || !parsed.data.laggedWeights.every(square)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'DYNOTEARS', detail: 'DYNOTEARS weight matrices have inconsistent dimensions.' })
  }
  return ok(parsed.data)
}

export function parseOcseEvidence(value: unknown): Result<OcseEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = ocseEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'oCSE', detail: z.prettifyError(parsed.error) })
  }
  if (parsed.data.edges.some((edge) => edge.source >= parsed.data.variables
    || edge.target >= parsed.data.variables
    || edge.lag > parsed.data.maxLag)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'oCSE', detail: 'oCSE returned an edge outside the declared variable or lag range.' })
  }
  return ok(parsed.data)
}

export type DiscoveryRunId = Brand<string, 'DiscoveryRunId'>

export const DISCOVERY_LAG_OPTIONS = [1, 2, 3, 4, 6, 8, 12, 20] as const
export type DiscoveryLag = (typeof DISCOVERY_LAG_OPTIONS)[number]

export const PCMCI_ALPHA_OPTIONS = [0.01, 0.025, 0.05, 0.1] as const
export type PcmciAlpha = (typeof PCMCI_ALPHA_OPTIONS)[number]

export const DYNOTEARS_PENALTY_OPTIONS = [0.01, 0.05, 0.1, 0.2] as const
export type DynotearsPenalty = (typeof DYNOTEARS_PENALTY_OPTIONS)[number]

export const OCSE_SHUFFLE_OPTIONS = [20, 50, 100, 200] as const
export type OcseShuffles = (typeof OCSE_SHUFFLE_OPTIONS)[number]
export type OcseInformationMethod = 'gaussian' | 'knn'

export type DiscoveryMethodChoice = 'pcmci-plus' | 'lpcmci' | 'dynotears' | 'ocse' | 'granger-ssr-f'
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
      readonly kind: 'lpcmci'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'dynotears'
      readonly maxLag: DiscoveryLag
      readonly lambdaW: DynotearsPenalty
      readonly lambdaA: DynotearsPenalty
    }
  | {
      readonly kind: 'ocse'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly nShuffles: OcseShuffles
      readonly method: OcseInformationMethod
      readonly k: 5
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
      readonly kind: 'lpcmci-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof LPCMCI_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: LpcmciEvidence
    }
  | {
      readonly kind: 'dynotears-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof DYNOTEARS_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: DynotearsEvidence
    }
  | {
      readonly kind: 'ocse-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof OCSE_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: OcseEvidence
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

export interface DiscoveryProgress {
  readonly stage: string
  readonly completed: number
  readonly total: number
}

export type DiscoveryJob =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly progress: DiscoveryProgress | null }
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
  | { readonly type: 'dynotears-lambda-w-selected'; readonly value: DynotearsPenalty }
  | { readonly type: 'dynotears-lambda-a-selected'; readonly value: DynotearsPenalty }
  | { readonly type: 'ocse-alpha-selected'; readonly value: PcmciAlpha }
  | { readonly type: 'ocse-shuffles-selected'; readonly value: OcseShuffles }
  | { readonly type: 'ocse-method-selected'; readonly value: OcseInformationMethod }
  | { readonly type: 'target-selected'; readonly column: ColumnChoice }
  | { readonly type: 'candidate-cause-selected'; readonly column: ColumnChoice }
  | { readonly type: 'max-lag-selected'; readonly value: DiscoveryLag }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-progressed'; readonly progress: DiscoveryProgress }
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
        configuration: initialConfigurationFor(event.method),
        job: { kind: 'idle' },
      }
    case 'tau-max-selected':
      return state.configuration.kind === 'pcmci-plus' || state.configuration.kind === 'lpcmci'
        ? { configuration: { ...state.configuration, tauMax: event.value }, job: { kind: 'idle' } }
        : state
    case 'pc-alpha-selected':
      return state.configuration.kind === 'pcmci-plus' || state.configuration.kind === 'lpcmci'
        ? { configuration: { ...state.configuration, pcAlpha: event.value }, job: { kind: 'idle' } }
        : state
    case 'dynotears-lambda-w-selected':
      return state.configuration.kind === 'dynotears'
        ? { configuration: { ...state.configuration, lambdaW: event.value }, job: { kind: 'idle' } }
        : state
    case 'dynotears-lambda-a-selected':
      return state.configuration.kind === 'dynotears'
        ? { configuration: { ...state.configuration, lambdaA: event.value }, job: { kind: 'idle' } }
        : state
    case 'ocse-alpha-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, alpha: event.value }, job: { kind: 'idle' } }
        : state
    case 'ocse-shuffles-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, nShuffles: event.value }, job: { kind: 'idle' } }
        : state
    case 'ocse-method-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, method: event.value }, job: { kind: 'idle' } }
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
      return state.configuration.kind === 'granger-ssr-f' || state.configuration.kind === 'dynotears' || state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, maxLag: event.value }, job: { kind: 'idle' } }
        : state
    case 'run-started': return { ...state, job: { kind: 'running', progress: null } }
    case 'run-progressed': return state.job.kind === 'running'
      ? { ...state, job: { kind: 'running', progress: event.progress } }
      : state
    case 'run-failed': return { ...state, job: { kind: 'failed', problem: event.problem } }
    case 'run-succeeded': return { ...state, job: { kind: 'succeeded', artifact: event.artifact } }
    default: return assertNever(event)
  }
}

function initialConfigurationFor(method: DiscoveryMethodChoice): DiscoveryConfiguration {
  switch (method) {
    case 'pcmci-plus': return { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 }
    case 'lpcmci': return { kind: 'lpcmci', tauMax: 2, pcAlpha: 0.05 }
    case 'dynotears': return { kind: 'dynotears', maxLag: 2, lambdaW: 0.1, lambdaA: 0.1 }
    case 'ocse': return { kind: 'ocse', maxLag: 2, alpha: 0.05, nShuffles: 50, method: 'gaussian', k: 5 }
    case 'granger-ssr-f':
      return {
        kind: 'granger-ssr-f',
        target: { kind: 'unselected' },
        candidateCause: { kind: 'unselected' },
        maxLag: 4,
      }
    default: return assertNever(method)
  }
}

export type ReadyDiscoverySpecification =
  | {
      readonly kind: 'pcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'lpcmci'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'dynotears'
      readonly maxLag: DiscoveryLag
      readonly lambdaW: DynotearsPenalty
      readonly lambdaA: DynotearsPenalty
    }
  | {
      readonly kind: 'ocse'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly nShuffles: OcseShuffles
      readonly method: OcseInformationMethod
      readonly k: 5
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
  | { readonly kind: 'browser-variable-limit'; readonly method: 'PCMCI+' | 'LPCMCI' | 'DYNOTEARS' | 'oCSE'; readonly maximum: number; readonly available: number }
  | { readonly kind: 'browser-lag-limit'; readonly method: 'DYNOTEARS' | 'oCSE'; readonly maximum: number }

export function readyDiscoverySpecification(
  configuration: DiscoveryConfiguration,
  prepared: PreparedDatasetArtifact,
): Result<ReadyDiscoverySpecification, DiscoveryReadinessProblem> {
  if (prepared.kind !== 'prepared-time-series') return err({ kind: 'time-series-required' })
  if (prepared.missingness.kind !== 'not-present') return err({ kind: 'dense-browser-boundary-required' })
  switch (configuration.kind) {
    case 'pcmci-plus': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 32) return err({ kind: 'browser-variable-limit', method: 'PCMCI+', maximum: 32, available: prepared.columns.length })
      const required = Math.max(2 * configuration.tauMax + 16, 24)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'lpcmci': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 32) return err({ kind: 'browser-variable-limit', method: 'LPCMCI', maximum: 32, available: prepared.columns.length })
      const required = Math.max(2 * configuration.tauMax + 16, 24)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'dynotears': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 12) return err({ kind: 'browser-variable-limit', method: 'DYNOTEARS', maximum: 12, available: prepared.columns.length })
      if (configuration.maxLag > 6) return err({ kind: 'browser-lag-limit', method: 'DYNOTEARS', maximum: 6 })
      const required = configuration.maxLag + 16
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'ocse': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 12) return err({ kind: 'browser-variable-limit', method: 'oCSE', maximum: 12, available: prepared.columns.length })
      if (configuration.maxLag > 8) return err({ kind: 'browser-lag-limit', method: 'oCSE', maximum: 8 })
      const required = configuration.maxLag + 24
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
    case 'at-least-two-variables-required': return 'This multivariate discovery method requires at least two prepared variables.'
    case 'target-required': return 'Choose the target series whose future values are being predicted.'
    case 'candidate-cause-required': return 'Choose the candidate cause whose past values are added to the model.'
    case 'distinct-pair-required': return 'Target and candidate cause must be different variables.'
    case 'too-few-observations': return `This configuration needs at least ${problem.required} observations; ${problem.available} are available.`
    case 'dense-browser-boundary-required': return 'The current browser command needs a dense prepared matrix. Tigramite-mask execution will use its own later boundary.'
    case 'browser-variable-limit': return `${problem.method} accepts at most ${problem.maximum} variables in the current browser boundary; ${problem.available} are selected.`
    case 'browser-lag-limit': return `${problem.method} accepts a maximum lag of ${problem.maximum} in the current browser boundary.`
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
