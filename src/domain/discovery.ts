import { z } from 'zod'
import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { columnNameOf, type NumericColumnSelection } from './dataset'
import {
  DYNOTEARS_METHOD_ID,
  DIRECT_LINGAM_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  RPCMCI_PAR_CORR_METHOD_ID,
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

export const rpcmciEvidenceSchema = z.object({
  kind: z.literal('rpcmci'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(12),
  numRegimes: z.number().int().min(2).max(6),
  maxTransitions: z.number().int().nonnegative(),
  switchThres: z.number().finite().min(0).max(1),
  numIterations: z.number().int().min(1).max(100),
  maxAnneal: z.number().int().min(1).max(50),
  tauMin: z.number().int().nonnegative().max(6),
  tauMax: z.number().int().nonnegative().max(6),
  pcAlpha: z.number().finite().positive().max(1),
  alphaLevel: z.number().finite().positive().max(1),
  seed: z.number().int().nonnegative(),
  regimes: z.array(z.array(z.number().finite().min(-1e-9).max(1 + 1e-9))),
  graphs: z.array(z.array(z.array(z.array(z.string().max(3))))),
  pMatrices: z.array(z.array(z.array(z.array(z.number().finite().min(0).max(1))))),
  valMatrices: z.array(z.array(z.array(z.array(z.number().finite().min(-1).max(1))))),
  diffGAll: z.array(z.array(z.number().finite().nonnegative()).nullable()),
  diffGBest: z.array(z.number().finite().nonnegative()),
  errorFreeAnnealings: z.number().int().positive(),
}).strict()

export type RpcmciEvidence = z.infer<typeof rpcmciEvidenceSchema>

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

export const directLingamEvidenceSchema = z.object({
  kind: z.literal('directLingam'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2).max(12),
  causalOrder: z.array(z.number().int().nonnegative()),
  weights: z.array(z.array(z.number().finite())),
}).strict()

export type DirectLingamEvidence = z.infer<typeof directLingamEvidenceSchema>

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
  readonly method: 'LPCMCI' | 'RPCMCI' | 'DYNOTEARS' | 'DirectLiNGAM' | 'VAR-LiNGAM' | 'oCSE'
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

export function parseRpcmciEvidence(value: unknown): Result<RpcmciEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = rpcmciEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'RPCMCI', detail: z.prettifyError(parsed.error) })
  }
  const evidence = parsed.data
  const matrixShapeIsValid = (matrix: readonly (readonly (readonly unknown[])[])[]) =>
    hasMatrixShape(matrix, evidence.variables, evidence.tauMax + 1)
  if (evidence.tauMin > evidence.tauMax
    || evidence.maxTransitions >= evidence.observations
    || evidence.regimes.length !== evidence.numRegimes
    || evidence.regimes.some((regime) => regime.length !== evidence.observations)
    || evidence.graphs.length !== evidence.numRegimes
    || evidence.pMatrices.length !== evidence.numRegimes
    || evidence.valMatrices.length !== evidence.numRegimes
    || !evidence.graphs.every(matrixShapeIsValid)
    || !evidence.pMatrices.every(matrixShapeIsValid)
    || !evidence.valMatrices.every(matrixShapeIsValid)
    || evidence.diffGAll.length !== evidence.maxAnneal
    || evidence.diffGAll.some((history) => history !== null && history.length > evidence.numIterations)
    || evidence.diffGBest.length > evidence.numIterations
    || evidence.errorFreeAnnealings > evidence.maxAnneal) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'RPCMCI', detail: 'RPCMCI evidence dimensions do not match its declared configuration.' })
  }
  for (let time = 0; time < evidence.observations; time += 1) {
    const membership = evidence.regimes.reduce((sum, regime) => sum + (regime[time] ?? 0), 0)
    if (Math.abs(membership - 1) > 1e-7) {
      return err({ kind: 'invalid-discovery-matrix-result', method: 'RPCMCI', detail: `RPCMCI memberships at observation ${time + 1} do not sum to one.` })
    }
  }
  return ok(evidence)
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

export function parseDirectLingamEvidence(value: unknown): Result<DirectLingamEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = directLingamEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'DirectLiNGAM', detail: z.prettifyError(parsed.error) })
  }
  const square = parsed.data.weights.length === parsed.data.variables
    && parsed.data.weights.every((row) => row.length === parsed.data.variables)
  if (!square) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'DirectLiNGAM', detail: 'DirectLiNGAM returned a weight matrix with inconsistent dimensions.' })
  }
  const order = [...parsed.data.causalOrder].sort((left, right) => left - right)
  if (order.length !== parsed.data.variables || order.some((index, position) => index !== position)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'DirectLiNGAM', detail: 'DirectLiNGAM causal order is not a permutation of the variables.' })
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

export const PCMCI_ALPHA_OPTIONS = [0.01, 0.025, 0.05, 0.1, 0.2] as const
export type PcmciAlpha = (typeof PCMCI_ALPHA_OPTIONS)[number]

export const DYNOTEARS_PENALTY_OPTIONS = [0.01, 0.05, 0.1, 0.2] as const
export type DynotearsPenalty = (typeof DYNOTEARS_PENALTY_OPTIONS)[number]

export const OCSE_SHUFFLE_OPTIONS = [20, 50, 100, 200] as const
export type OcseShuffles = (typeof OCSE_SHUFFLE_OPTIONS)[number]
export type OcseInformationMethod = 'gaussian' | 'knn'

export type DiscoveryMethodChoice = 'direct-lingam' | 'pcmci-plus' | 'lpcmci' | 'rpcmci' | 'dynotears' | 'var-lingam' | 'ocse'
export type AcceptedDiscoveryEligibility = Exclude<MethodEligibility, { readonly kind: 'refused' }>

export type DiscoveryMethodGroupId = 'pcmci-family' | 'lingam-family' | 'continuous-optimization' | 'causation-entropy'

export interface DiscoveryMethodGroup {
  readonly id: DiscoveryMethodGroupId
  readonly name: string
  readonly description: string
  readonly methods: NonEmptyArray<DiscoveryMethodChoice>
}

const PCMCI_FAMILY: DiscoveryMethodGroup = {
  id: 'pcmci-family',
  name: 'PCMCI family',
  description: 'Conditional-independence methods for time-indexed graphs; RPCMCI also estimates persistent regimes.',
  methods: ['pcmci-plus', 'lpcmci', 'rpcmci'],
}

const LINGAM_FAMILY: DiscoveryMethodGroup = {
  id: 'lingam-family',
  name: 'LiNGAM family',
  description: 'Linear structural models identified through non-Gaussian disturbances.',
  methods: ['direct-lingam', 'var-lingam'],
}

const CONTINUOUS_OPTIMIZATION: DiscoveryMethodGroup = {
  id: 'continuous-optimization',
  name: 'DYNOTEARS',
  description: 'Sparse dynamic structural equations fitted under an acyclicity constraint.',
  methods: ['dynotears'],
}

const CAUSATION_ENTROPY: DiscoveryMethodGroup = {
  id: 'causation-entropy',
  name: 'Optimal causation entropy',
  description: 'Lagged-parent selection by conditional mutual information.',
  methods: ['ocse'],
}

export const DISCOVERY_METHOD_GROUPS: NonEmptyArray<DiscoveryMethodGroup> = [
  PCMCI_FAMILY,
  LINGAM_FAMILY,
  CONTINUOUS_OPTIMIZATION,
  CAUSATION_ENTROPY,
]

export function discoveryMethodGroupById(id: DiscoveryMethodGroupId): DiscoveryMethodGroup {
  switch (id) {
    case 'pcmci-family': return PCMCI_FAMILY
    case 'lingam-family': return LINGAM_FAMILY
    case 'continuous-optimization': return CONTINUOUS_OPTIMIZATION
    case 'causation-entropy': return CAUSATION_ENTROPY
    default: return assertNever(id)
  }
}

export function discoveryMethodGroupFor(method: DiscoveryMethodChoice): DiscoveryMethodGroup {
  switch (method) {
    case 'pcmci-plus':
    case 'lpcmci':
    case 'rpcmci':
      return PCMCI_FAMILY
    case 'direct-lingam':
    case 'var-lingam':
      return LINGAM_FAMILY
    case 'dynotears':
      return CONTINUOUS_OPTIMIZATION
    case 'ocse':
      return CAUSATION_ENTROPY
    default:
      return assertNever(method)
  }
}

export type DiscoveryConfiguration =
  | { readonly kind: 'direct-lingam' }
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
      readonly kind: 'rpcmci'
      readonly numRegimes: number
      readonly maxTransitions: number
      readonly switchThres: number
      readonly numIterations: number
      readonly maxAnneal: number
      readonly tauMin: number
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
      readonly alphaLevel: PcmciAlpha
      readonly seed: number
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
      readonly kind: 'direct-lingam-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof DIRECT_LINGAM_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: DirectLingamEvidence
    }
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
      readonly kind: 'rpcmci-run'
      readonly id: DiscoveryRunId
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof RPCMCI_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: RpcmciEvidence
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
  | { readonly type: 'rpcmci-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }> }
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

export const initialDiscoveryDraftFor = (prepared: PreparedDatasetArtifact): DiscoveryDraft => ({
  configuration: prepared.kind === 'prepared-cross-section'
    ? { kind: 'direct-lingam' }
    : INITIAL_DISCOVERY_DRAFT.configuration,
  job: { kind: 'idle' },
})

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
    case 'rpcmci-configured':
      return state.configuration.kind === 'rpcmci'
        ? { configuration: event.configuration, job: { kind: 'idle' } }
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
    case 'direct-lingam': return { kind: 'direct-lingam' }
    case 'pcmci-plus': return { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 }
    case 'lpcmci': return { kind: 'lpcmci', tauMax: 2, pcAlpha: 0.05 }
    case 'rpcmci': return { kind: 'rpcmci', numRegimes: 2, maxTransitions: 4, switchThres: 0.05, numIterations: 20, maxAnneal: 10, tauMin: 1, tauMax: 1, pcAlpha: 0.2, alphaLevel: 0.01, seed: 327 }
    case 'dynotears': return { kind: 'dynotears', maxLag: 2, lambdaW: 0.1, lambdaA: 0.1 }
    case 'var-lingam': return { kind: 'var-lingam', maxLag: 2, prune: true }
    case 'ocse': return { kind: 'ocse', maxLag: 2, alpha: 0.05, nShuffles: 50, method: 'gaussian', k: 5 }
    default: return assertNever(method)
  }
}

export type ReadyDiscoverySpecification =
  | { readonly kind: 'direct-lingam' }
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
  | Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }>
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
  | { readonly kind: 'cross-section-required' }
  | { readonly kind: 'at-least-two-variables-required' }
  | { readonly kind: 'too-few-observations'; readonly required: number; readonly available: number }
  | { readonly kind: 'dense-browser-boundary-required' }
  | { readonly kind: 'browser-variable-limit'; readonly method: 'DirectLiNGAM' | 'PCMCI+' | 'LPCMCI' | 'RPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE'; readonly maximum: number; readonly available: number }
  | { readonly kind: 'browser-lag-limit'; readonly method: 'RPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE'; readonly maximum: number }
  | { readonly kind: 'transition-budget-too-large'; readonly available: number }

export function readyDiscoverySpecification(
  configuration: DiscoveryConfiguration,
  prepared: PreparedDatasetArtifact,
): Result<ReadyDiscoverySpecification, DiscoveryReadinessProblem> {
  if (prepared.missingness.kind !== 'not-present') return err({ kind: 'dense-browser-boundary-required' })
  if (configuration.kind === 'direct-lingam') {
    if (prepared.kind !== 'prepared-cross-section') return err({ kind: 'cross-section-required' })
    if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
    if (prepared.columns.length > 12) return err({ kind: 'browser-variable-limit', method: 'DirectLiNGAM', maximum: 12, available: prepared.columns.length })
    const required = prepared.columns.length + 16
    return prepared.observations < required
      ? err({ kind: 'too-few-observations', required, available: prepared.observations })
      : ok(configuration)
  }
  if (prepared.kind !== 'prepared-time-series') return err({ kind: 'time-series-required' })
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
    case 'rpcmci': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (prepared.columns.length > 12) return err({ kind: 'browser-variable-limit', method: 'RPCMCI', maximum: 12, available: prepared.columns.length })
      if (configuration.tauMax > 6) return err({ kind: 'browser-lag-limit', method: 'RPCMCI', maximum: 6 })
      if (configuration.maxTransitions >= prepared.observations) return err({ kind: 'transition-budget-too-large', available: prepared.observations })
      const required = Math.max(2 * configuration.tauMax + 24, 40)
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
  if (method.id === DIRECT_LINGAM_METHOD_ID) {
    if (prepared.kind !== 'prepared-cross-section') {
      const samplingCaveat = method.caveats.find((caveat) => caveat.category === 'sampling-structure') ?? firstCaveat
      return {
        kind: 'refused',
        satisfied: [],
        unresolved: [],
        violations: [{
          kind: 'violated',
          caveat: samplingCaveat,
          evidence: prepared.kind === 'prepared-time-series'
            ? 'This prepared dataset is a time series; adjacent rows can be serially dependent.'
            : 'This prepared dataset is a panel; repeated observations from the same unit are not independent rows.',
        }],
      }
    }
    const satisfied: Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[] = []
    const unresolved: Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>[] = []
    for (const caveat of method.caveats) {
      if (caveat.category === 'interpretation') continue
      if (caveat.category === 'sampling-structure') {
        satisfied.push({ kind: 'satisfied', caveat, evidence: 'Prepared as independent cross-sectional observations.' })
        continue
      }
      if (caveat.category === 'missingness' && prepared.missingness.kind === 'not-present') {
        satisfied.push({ kind: 'satisfied', caveat, evidence: 'The prepared dataset contains no missing values.' })
        continue
      }
      unresolved.push({ kind: 'unresolved', caveat, missingEvidence: '' })
    }
    return isNonEmpty(unresolved)
      ? { kind: 'caution', satisfied, unresolved }
      : { kind: 'eligible', satisfied }
  }
  if (prepared.kind !== 'prepared-time-series') {
    const samplingCaveat = method.caveats.find((caveat) => caveat.category === 'sampling-structure') ?? firstCaveat
    return {
      kind: 'refused',
      satisfied: [],
      unresolved: [],
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
      const verdicts = prepared.columns.map((column) => ({ name: columnNameOf(column), verdict: levelModelVerdict(columnNameOf(column), stationarity.variables.find((variable) => variable.column === column)?.assessment ?? null) }))
      const integrated = verdicts.filter((entry) => entry.verdict.kind === 'refused').map((entry) => entry.name)
      const open = verdicts.filter((entry) => entry.verdict.kind === 'unresolved').map((entry) => entry.verdict.reason)
      if (integrated.length === 0 && open.length === 0) {
        satisfied.push({ kind: 'satisfied', caveat, evidence: `${verdicts.map((entry) => entry.name).join(', ')} ${verdicts.length === 1 ? 'is' : 'are'} stationary on the prepared scale.` })
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
    case 'time-series-required': return 'Temporal discovery needs a regular time series. This prepared dataset has another observation structure.'
    case 'cross-section-required': return 'DirectLiNGAM needs independent cross-sectional observations. Prepare this dataset as a cross-section.'
    case 'at-least-two-variables-required': return 'Select at least 2 variables.'
    case 'too-few-observations': return `This configuration needs at least ${problem.required} rows; ${problem.available} are available. Use more rows or choose a smaller configuration.`
    case 'dense-browser-boundary-required': return 'Choose a complete interval or imputation. This method needs complete numeric columns in the browser.'
    case 'browser-variable-limit': return `${problem.method} accepts up to ${problem.maximum} variables in the browser; ${problem.available} are selected. Deselect ${problem.available - problem.maximum}.`
    case 'browser-lag-limit': return `${problem.method} accepts a maximum lag of ${problem.maximum} in the browser. Lower the maximum lag.`
    case 'transition-budget-too-large': return `The maximum transition count must be smaller than the ${problem.available} available observations.`
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
