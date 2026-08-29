import { z } from 'zod'
import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { columnNameOf, type NumericColumnSelection } from './dataset'
import {
  DYNOTEARS_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  VAR_LINGAM_METHOD_ID,
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
import { levelModelVerdict } from './stationarityAssessment'

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

export const varLingamEvidenceSchema = z.object({
  kind: z.literal('varLingam'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(12),
  lags: z.number().int().min(1).max(6),
  selectedLag: z.number().int().min(1).max(6),
  prune: z.boolean(),
  causalOrder: z.array(z.number().int().nonnegative()),
  contemporaneousWeights: z.array(z.array(z.number().finite())),
  laggedWeights: z.array(z.array(z.array(z.number().finite()))),
}).strict()

export type VarLingamEvidence = z.infer<typeof varLingamEvidenceSchema>

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


export type PcmciPlusBoundaryProblem = {
  readonly kind: 'invalid-pcmci-plus-result'
  readonly detail: string
}

export type DiscoveryMatrixBoundaryProblem = {
  readonly kind: 'invalid-discovery-matrix-result'
  readonly method: 'LPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE'
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

export function parseVarLingamEvidence(value: unknown): Result<VarLingamEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = varLingamEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'VAR-LiNGAM', detail: z.prettifyError(parsed.error) })
  }
  const square = (matrix: readonly (readonly number[])[]) => matrix.length === parsed.data.variables
    && matrix.every((row) => row.length === parsed.data.variables)
  if (!square(parsed.data.contemporaneousWeights)
    || parsed.data.laggedWeights.length !== parsed.data.selectedLag
    || parsed.data.selectedLag > parsed.data.lags
    || !parsed.data.laggedWeights.every(square)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'VAR-LiNGAM', detail: 'VAR-LiNGAM adjacency matrices have inconsistent dimensions.' })
  }
  const order = [...parsed.data.causalOrder].sort((left, right) => left - right)
  if (order.length !== parsed.data.variables || order.some((index, position) => index !== position)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'VAR-LiNGAM', detail: 'VAR-LiNGAM causal order is not a permutation of the variables.' })
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

export type DiscoveryMethodChoice = 'pcmci-plus' | 'lpcmci' | 'dynotears' | 'var-lingam' | 'ocse'
export type AcceptedDiscoveryEligibility = Exclude<MethodEligibility, { readonly kind: 'refused' }>

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
      readonly kind: 'var-lingam'
      readonly maxLag: DiscoveryLag
      readonly prune: boolean
    }
  | {
      readonly kind: 'ocse'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly nShuffles: OcseShuffles
      readonly method: OcseInformationMethod
      readonly k: 5
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
      readonly kind: 'var-lingam-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof VAR_LINGAM_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: VarLingamEvidence
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
  | { readonly type: 'var-lingam-prune-selected'; readonly value: boolean }
  | { readonly type: 'ocse-alpha-selected'; readonly value: PcmciAlpha }
  | { readonly type: 'ocse-shuffles-selected'; readonly value: OcseShuffles }
  | { readonly type: 'ocse-method-selected'; readonly value: OcseInformationMethod }
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
    case 'var-lingam-prune-selected':
      return state.configuration.kind === 'var-lingam'
        ? { configuration: { ...state.configuration, prune: event.value }, job: { kind: 'idle' } }
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
    case 'max-lag-selected':
      return state.configuration.kind === 'dynotears' || state.configuration.kind === 'var-lingam' || state.configuration.kind === 'ocse'
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
    case 'var-lingam': return { kind: 'var-lingam', maxLag: 2, prune: true }
    case 'ocse': return { kind: 'ocse', maxLag: 2, alpha: 0.05, nShuffles: 50, method: 'gaussian', k: 5 }
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
      readonly kind: 'var-lingam'
      readonly maxLag: DiscoveryLag
      readonly prune: boolean
    }
  | {
      readonly kind: 'ocse'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly nShuffles: OcseShuffles
      readonly method: OcseInformationMethod
      readonly k: 5
    }

export type DiscoveryReadinessProblem =
  | { readonly kind: 'time-series-required' }
  | { readonly kind: 'at-least-two-variables-required' }
  | { readonly kind: 'too-few-observations'; readonly required: number; readonly available: number }
  | { readonly kind: 'dense-browser-boundary-required' }
  | { readonly kind: 'browser-variable-limit'; readonly method: 'PCMCI+' | 'LPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE'; readonly maximum: number; readonly available: number }
  | { readonly kind: 'browser-lag-limit'; readonly method: 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE'; readonly maximum: number }

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
    case 'var-lingam': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 12) return err({ kind: 'browser-variable-limit', method: 'VAR-LiNGAM', maximum: 12, available: prepared.columns.length })
      if (configuration.maxLag > 6) return err({ kind: 'browser-lag-limit', method: 'VAR-LiNGAM', maximum: 6 })
      const required = prepared.columns.length * (configuration.maxLag + 1) + 16
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
        evidence: 'This prepared dataset holds independent rows. Their order does not represent time.',
      }],
    }
  }

  const satisfied: Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[] = []
  const unresolved: Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>[] = []
  for (const caveat of method.caveats) {
    // Reading rules say how to read a result; they are not conditions the data can meet or fail.
    if (caveat.category === 'interpretation') continue
    if (caveat.category === 'sampling-structure') {
      satisfied.push({
        kind: 'satisfied',
        caveat,
        evidence: `Prepared as a regular ${prepared.sampling.frequency} time series with an explicit time column.`,
      })
      continue
    }
    if (caveat.category === 'missingness' && prepared.missingness.kind === 'not-present') {
      satisfied.push({ kind: 'satisfied', caveat, evidence: 'The prepared dataset contains no missing values.' })
      continue
    }
    if (caveat.category === 'stationarity-and-dynamics') {
      // The battery holds a verdict per series; an integrated series in levels warns rather than refuses, since the verdict is itself a test.
      if (stationarity === null) {
        unresolved.push({ kind: 'unresolved', caveat, missingEvidence: 'Run stationarity tests for this prepared dataset version in Data studio.' })
        continue
      }
      if (stationarity.transform.kind === 'difference') {
        satisfied.push({ kind: 'satisfied', caveat, evidence: 'The stationarity view is the first difference; the run reads the prepared levels, so difference the series in the recipe before trusting links found on levels.' })
        continue
      }
      const verdicts = prepared.columns.map((column) => ({ name: columnNameOf(column), verdict: levelModelVerdict(columnNameOf(column), stationarity.variables.find((variable) => variable.column === column)?.assessment ?? null) }))
      const integrated = verdicts.filter((entry) => entry.verdict.kind === 'refused').map((entry) => entry.name)
      const open = verdicts.filter((entry) => entry.verdict.kind === 'unresolved').map((entry) => entry.verdict.reason)
      if (integrated.length === 0 && open.length === 0) {
        satisfied.push({ kind: 'satisfied', caveat, evidence: `${verdicts.map((entry) => entry.name).join(', ')} ${verdicts.length === 1 ? 'is' : 'are'} stationary in levels.` })
        continue
      }
      const integratedText = integrated.length === 0 ? '' : `${integrated.join(', ')} ${integrated.length === 1 ? 'is' : 'are'} I(1) in levels, so links found on levels can be spurious; a differenced version is the safer input.`
      unresolved.push({ kind: 'unresolved', caveat, missingEvidence: [integratedText, ...open].filter((text) => text.length > 0).join(' ') })
      continue
    }
    unresolved.push({ kind: 'unresolved', caveat, missingEvidence: '' })
  }
  if (!isNonEmpty(unresolved)) return { kind: 'eligible', satisfied }
  return { kind: 'caution', satisfied, unresolved }
}

export const newDiscoveryRunId = (): DiscoveryRunId =>
  brand<string, 'DiscoveryRunId'>(crypto.randomUUID())

export function describeDiscoveryReadiness(problem: DiscoveryReadinessProblem): string {
  switch (problem.kind) {
    case 'time-series-required': return 'Temporal discovery needs a time series. This prepared dataset holds independent rows.'
    case 'at-least-two-variables-required': return 'Select at least 2 variables.'
    case 'too-few-observations': return `This configuration needs at least ${problem.required} rows; ${problem.available} are available. Lower the maximum lag or use more rows.`
    case 'dense-browser-boundary-required': return 'Choose a complete interval or imputation. This method needs complete numeric columns in the browser.'
    case 'browser-variable-limit': return `${problem.method} accepts up to ${problem.maximum} variables in the browser; ${problem.available} are selected. Deselect ${problem.available - problem.maximum}.`
    case 'browser-lag-limit': return `${problem.method} accepts a maximum lag of ${problem.maximum} in the browser. Lower the maximum lag.`
    default: return assertNever(problem)
  }
}

export function describeDiscoveryRunProblem(problem: DiscoveryRunProblem): string {
  switch (problem.kind) {
    case 'materialization-refused': return problem.detail
    case 'missing-values-remain': return `${problem.cells} missing values remain. Choose a missing-value policy in Data studio.`
    case 'analysis-refused': return problem.detail
    case 'execution-unavailable': return problem.detail
    default: return assertNever(problem)
  }
}
