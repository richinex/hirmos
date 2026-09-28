import { z } from 'zod'
import { assertNever, brand, err, isNonEmpty, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import { columnNameOf, type ColumnId, type NumericColumnSelection } from './dataset'
import {
  DYNOTEARS_METHOD_ID,
  DIRECT_LINGAM_METHOD_ID,
  FCI_METHOD_ID,
  PC_STABLE_METHOD_ID,
  LPCMCI_PAR_CORR_METHOD_ID,
  RPCMCI_PAR_CORR_METHOD_ID,
  CDNOTS_PAR_CORR_METHOD_ID,
  CDNOTS_PLUS_PAR_CORR_METHOD_ID,
  GRACE_METHOD_ID,
  CMLP_METHOD_ID,
  CLSTM_METHOD_ID,
  VAR_LINGAM_METHOD_ID,
  OCSE_METHOD_ID,
  PCMCI_PLUS_PAR_CORR_METHOD_ID,
  JPCMCI_PLUS_PAR_CORR_METHOD_ID,
  type CaveatEvaluation,
  type MethodDefinition,
  type MethodEligibility,
} from './methods'
import type {
  PreparedDatasetArtifact,
  PreparedDatasetVersionId,
  StationarityEvidenceArtifact,
} from './preprocessing'
import { levelEvidence, nonEmptyGroups, summaries } from './levelEvidence'

export const pcmciPlusEvidenceSchema = z.object({
  kind: z.literal('pcmciPlus'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
  tauMax: z.number().int().min(1).max(20),
  pcAlpha: z.number().finite().positive().max(1),
  graph: z.array(z.array(z.array(z.string().max(3)))),
  pMatrix: z.array(z.array(z.array(z.number().finite().min(0).max(1)))),
  valMatrix: z.array(z.array(z.array(z.number().finite().min(-1).max(1)))),
}).strict()

export type PcmciPlusEvidence = z.infer<typeof pcmciPlusEvidenceSchema>

const jpcmciNodeSchema = z.object({
  variable: z.number().int().nonnegative(),
  lag: z.number().int(),
}).strict()

export const jpcmciPlusEvidenceSchema = z.object({
  kind: z.literal('jpcmciplus'),
  observations: z.number().int().positive(),
  datasets: z.number().int().min(2),
  periods: z.number().int().min(2),
  observedVariables: z.number().int().min(2),
  variables: z.number().int().min(2),
  classes: z.array(z.enum(['system', 'timeContext', 'spaceContext', 'timeDummy', 'spaceDummy'])),
  timeDummy: z.boolean(),
  spaceDummy: z.boolean(),
  tauMax: z.number().int().min(1).max(20),
  pcAlpha: z.number().finite().positive().max(1),
  graph: z.array(z.array(z.array(z.string().max(3)))),
  // ParCorrMult's analytic p-value for a dummy block against a system variable exceeds 1 in the reference, so the bound is the reference's.
  pMatrix: z.array(z.array(z.array(z.number().finite().min(0)))),
  valMatrix: z.array(z.array(z.array(z.number().finite().min(-1).max(1)))),
  separatingSets: z.array(z.object({
    source: z.number().int().nonnegative(),
    target: z.number().int().nonnegative(),
    lag: z.number().int().nonnegative(),
    variables: z.array(jpcmciNodeSchema),
  }).strict()),
  ambiguousTriples: z.array(z.object({
    left: jpcmciNodeSchema,
    middle: z.number().int().nonnegative(),
    right: z.number().int().nonnegative(),
  }).strict()),
  laggedParents: z.array(z.array(jpcmciNodeSchema)),
  contextParents: z.array(z.array(jpcmciNodeSchema)),
  dummyParents: z.array(z.array(jpcmciNodeSchema)),
}).strict()

export type JpcmciPlusEvidence = z.infer<typeof jpcmciPlusEvidenceSchema>

export const lpcmciEvidenceSchema = pcmciPlusEvidenceSchema.extend({
  kind: z.literal('lpcmci'),
}).strict()

export type LpcmciEvidence = z.infer<typeof lpcmciEvidenceSchema>

export const rpcmciEvidenceSchema = z.object({
  kind: z.literal('rpcmci'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
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
  variables: z.number().int().min(2),
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
  variables: z.number().int().min(2),
  causalOrder: z.array(z.number().int().nonnegative()),
  weights: z.array(z.array(z.number().finite())),
}).strict()

export type DirectLingamEvidence = z.infer<typeof directLingamEvidenceSchema>

export const constraintCiTestSchema = z.enum(['fisherZ', 'kci'])
export type ConstraintCiTest = z.infer<typeof constraintCiTestSchema>

const constraintSeparatingSetSchema = z.object({
  x: z.number().int().nonnegative(),
  y: z.number().int().nonnegative(),
  variables: z.array(z.number().int().nonnegative()),
}).strict()

const constraintCiEvidenceSchema = z.object({
  x: z.number().int().nonnegative(),
  y: z.number().int().nonnegative(),
  conditions: z.array(z.number().int().nonnegative()),
  pValue: z.number().finite().min(0).max(1),
}).strict()

const constraintEvidenceBaseSchema = z.object({
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
  alpha: z.number().finite().positive().max(1),
  maxDepth: z.number().int().nonnegative().nullable(),
  ciTest: constraintCiTestSchema,
  graph: z.array(z.array(z.array(z.string().max(3)))),
  separatingSets: z.array(constraintSeparatingSetSchema),
  ciTests: z.array(constraintCiEvidenceSchema),
})

export const pcStableEvidenceSchema = constraintEvidenceBaseSchema.extend({
  kind: z.literal('pcStable'),
}).strict()

export type PcStableEvidence = z.infer<typeof pcStableEvidenceSchema>

const fciEdgePropertySchema = z.object({
  left: z.number().int().nonnegative(),
  right: z.number().int().nonnegative(),
  directness: z.enum(['definitelyDirect', 'possiblyDirect']).nullable(),
  latentConfounding: z.enum(['excluded', 'possible']).nullable(),
}).strict()

export const fciEvidenceSchema = constraintEvidenceBaseSchema.extend({
  kind: z.literal('fci'),
  maxPathLength: z.number().int().nonnegative().nullable(),
  edgeProperties: z.array(fciEdgePropertySchema),
}).strict()

export type FciEvidence = z.infer<typeof fciEvidenceSchema>

export const varLingamEvidenceSchema = z.object({
  kind: z.literal('varLingam'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
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
  variables: z.number().int().min(2),
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

const neuralScoreMatrixSchema = z.array(z.array(z.number().finite().nonnegative()))
const neuralActiveMatrixSchema = z.array(z.array(z.boolean()))
const neuralStandardizationSchema = z.object({
  means: z.array(z.number().finite()),
  scales: z.array(z.number().finite().positive()),
}).strict()

export const cmlpEvidenceSchema = z.object({
  kind: z.literal('cmlp'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
  lag: z.number().int().min(1).max(20),
  hidden: z.array(z.number().int().min(1).max(256)).min(1).max(4),
  activation: z.enum(['sigmoid', 'tanh', 'relu', 'leakyRelu', 'identity']),
  penalty: z.enum(['groupLasso', 'groupSparseGroupLasso', 'hierarchical']),
  lambda: z.number().finite().nonnegative(),
  ridgeLambda: z.number().finite().nonnegative(),
  learningRate: z.number().finite().positive(),
  maxIter: z.number().int().min(1).max(50_000),
  checkEvery: z.number().int().positive(),
  lookback: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  standardization: neuralStandardizationSchema,
  summaryScores: neuralScoreMatrixSchema,
  summaryActive: neuralActiveMatrixSchema,
  lagScores: z.array(z.array(z.array(z.number().finite().nonnegative()))),
  lagActive: z.array(z.array(z.array(z.boolean()))),
  lagOrder: z.array(z.number().int().positive()),
  loss: z.array(z.number().finite().nonnegative()),
  iterations: z.number().int().positive(),
}).strict()

export type CmlpEvidence = z.infer<typeof cmlpEvidenceSchema>

export const clstmEvidenceSchema = z.object({
  kind: z.literal('clstm'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
  context: z.number().int().min(1).max(100),
  hidden: z.number().int().min(1).max(256),
  lambda: z.number().finite().nonnegative(),
  ridgeLambda: z.number().finite().nonnegative(),
  learningRate: z.number().finite().positive(),
  maxIter: z.number().int().min(1).max(20_000),
  checkEvery: z.number().int().positive(),
  lookback: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  standardization: neuralStandardizationSchema,
  summaryScores: neuralScoreMatrixSchema,
  summaryActive: neuralActiveMatrixSchema,
  loss: z.array(z.number().finite().nonnegative()),
  iterations: z.number().int().positive(),
}).strict()

export type ClstmEvidence = z.infer<typeof clstmEvidenceSchema>

export const cdnotsMissingStrategySchema = z.enum(['pairwiseComplete', 'varEm'])
export type CdnotsMissingStrategy = z.infer<typeof cdnotsMissingStrategySchema>

export const cdnotsContextSchema = z.enum(['none', 'linear', 'linearSine', 'linearExponential', 'linearQuadratic', 'step', 'stepLinear'])
export type CdnotsContext = z.infer<typeof cdnotsContextSchema>

const cdnotsEvidenceBaseSchema = z.object({
  observations: z.number().int().positive(),
  observedVariables: z.number().int().min(2),
  contextVariables: z.array(z.string().trim().min(1)).max(2),
  maxLag: z.number().int().min(1).max(20),
  alpha: z.number().finite().positive().max(1),
  missing: cdnotsMissingStrategySchema,
  context: cdnotsContextSchema,
  graph: z.array(z.array(z.array(z.string().max(3)))),
  pMatrix: z.array(z.array(z.array(z.number().finite().min(0).max(1)))),
  valMatrix: z.array(z.array(z.array(z.number().finite().min(-1).max(1)))),
})

export const cdnotsEvidenceSchema = cdnotsEvidenceBaseSchema.extend({ kind: z.literal('cdnots') }).strict()
export const cdnotsPlusEvidenceSchema = cdnotsEvidenceBaseSchema.extend({ kind: z.literal('cdnotsPlus') }).strict()
export type CdnotsEvidence = z.infer<typeof cdnotsEvidenceSchema>
export type CdnotsPlusEvidence = z.infer<typeof cdnotsPlusEvidenceSchema>

export const graceEvidenceSchema = z.object({
  kind: z.literal('grace'),
  observations: z.number().int().positive(),
  variables: z.number().int().min(2),
  maxLag: z.number().int().min(1).max(20),
  alpha: z.number().finite().positive().max(1),
  context: cdnotsContextSchema,
  gateThreshold: z.number().finite().min(0).max(1),
  lambdaL0: z.number().finite().nonnegative(),
  epochs: z.number().int().positive(),
  patience: z.number().int().positive(),
  seed: z.number().int().nonnegative(),
  imputedCells: z.number().int().nonnegative(),
  skeleton: z.array(z.array(z.array(z.boolean()))),
  gateValues: z.array(z.array(z.array(z.number().finite().min(0).max(1)))),
  graph: z.array(z.array(z.array(z.boolean()))),
  loss: z.array(z.number().finite()),
  rmse: z.array(z.number().finite().nonnegative()),
}).strict()
export type GraceEvidence = z.infer<typeof graceEvidenceSchema>


export type PcmciPlusBoundaryProblem = {
  readonly kind: 'invalid-pcmci-plus-result'
  readonly detail: string
}

export type DiscoveryMatrixBoundaryProblem = {
  readonly kind: 'invalid-discovery-matrix-result'
  readonly method: 'J-PCMCI+' | 'LPCMCI' | 'RPCMCI' | 'CD-NOTS' | 'CD-NOTS+' | 'GRACE' | 'DYNOTEARS' | 'DirectLiNGAM' | 'PC-stable' | 'FCI' | 'VAR-LiNGAM' | 'oCSE' | 'cMLP' | 'cLSTM'
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

export function parseJpcmciPlusEvidence(value: unknown): Result<JpcmciPlusEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = jpcmciPlusEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'J-PCMCI+', detail: z.prettifyError(parsed.error) })
  }
  const result = parsed.data
  const lags = result.tauMax + 1
  const nodeIsValid = (node: { readonly variable: number; readonly lag: number }) =>
    node.variable < result.variables && node.lag <= 0 && node.lag >= -result.tauMax
  const parentRows = [result.laggedParents, result.contextParents, result.dummyParents]
  const expectedVariables = result.observedVariables + Number(result.timeDummy) + Number(result.spaceDummy)
  const expectedClasses = [
    ...result.classes.slice(0, result.observedVariables),
    ...(result.timeDummy ? ['timeDummy'] as const : []),
    ...(result.spaceDummy ? ['spaceDummy'] as const : []),
  ]
  if (result.observations !== result.datasets * result.periods
    || result.variables !== expectedVariables
    || result.classes.length !== result.variables
    || result.classes.some((role, index) => role !== expectedClasses[index])
    || result.classes.slice(0, result.observedVariables).some((role) => role === 'timeDummy' || role === 'spaceDummy')
    || !hasMatrixShape(result.graph, result.variables, lags)
    || !hasMatrixShape(result.pMatrix, result.variables, lags)
    || !hasMatrixShape(result.valMatrix, result.variables, lags)
    || parentRows.some((rows) => rows.length !== result.variables || rows.some((nodes) => nodes.some((node) => !nodeIsValid(node))))
    || result.separatingSets.some((set) => set.source >= result.variables || set.target >= result.variables || set.lag > result.tauMax || set.variables.some((node) => !nodeIsValid(node)))
    || result.ambiguousTriples.some((triple) => triple.middle >= result.variables || triple.right >= result.variables || !nodeIsValid(triple.left))) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'J-PCMCI+', detail: 'J-PCMCI+ evidence dimensions, node classes, or indexed evidence are inconsistent.' })
  }
  return ok(result)
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

function parseConstraintEvidence<ResultValue extends PcStableEvidence | FciEvidence>(
  value: unknown,
  method: 'PC-stable' | 'FCI',
  schema: z.ZodType<ResultValue>,
): Result<ResultValue, DiscoveryMatrixBoundaryProblem> {
  const parsed = schema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method, detail: z.prettifyError(parsed.error) })
  }
  const evidence = parsed.data
  if (!hasMatrixShape(evidence.graph, evidence.variables, 1)) {
    return err({ kind: 'invalid-discovery-matrix-result', method, detail: `${method} returned an endpoint matrix with inconsistent dimensions.` })
  }
  const validIndex = (index: number) => index < evidence.variables
  if (evidence.separatingSets.some((set) => !validIndex(set.x) || !validIndex(set.y) || set.variables.some((index) => !validIndex(index)))
    || evidence.ciTests.some((test) => !validIndex(test.x) || !validIndex(test.y) || test.conditions.some((index) => !validIndex(index)))) {
    return err({ kind: 'invalid-discovery-matrix-result', method, detail: `${method} returned a variable index outside the selected matrix.` })
  }
  return ok(evidence)
}

export function parsePcStableEvidence(value: unknown): Result<PcStableEvidence, DiscoveryMatrixBoundaryProblem> {
  return parseConstraintEvidence(value, 'PC-stable', pcStableEvidenceSchema)
}

export function parseFciEvidence(value: unknown): Result<FciEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = parseConstraintEvidence(value, 'FCI', fciEvidenceSchema)
  if (!parsed.ok) return parsed
  if (parsed.value.edgeProperties.some((edge) => edge.left >= parsed.value.variables || edge.right >= parsed.value.variables)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'FCI', detail: 'FCI returned an edge property outside the selected matrix.' })
  }
  return parsed
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

const isSquare = <Value>(matrix: readonly (readonly Value[])[], variables: number): boolean =>
  matrix.length === variables && matrix.every((row) => row.length === variables)

export function parseCmlpEvidence(value: unknown): Result<CmlpEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = cmlpEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-discovery-matrix-result', method: 'cMLP', detail: z.prettifyError(parsed.error) })
  const result = parsed.data
  const lagShape = (matrix: readonly (readonly (readonly unknown[])[])[]) =>
    isSquare(matrix, result.variables) && matrix.every((row) => row.every((lags) => lags.length === result.lag))
  if (!isSquare(result.summaryScores, result.variables)
    || !isSquare(result.summaryActive, result.variables)
    || !lagShape(result.lagScores)
    || !lagShape(result.lagActive)
    || result.lagOrder.length !== result.lag
    || result.standardization.means.length !== result.variables
    || result.standardization.scales.length !== result.variables
    || [...result.lagOrder].sort((left, right) => left - right).some((lag, index) => lag !== index + 1)
    || result.checkEvery > result.maxIter
    || result.iterations > result.maxIter) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'cMLP', detail: 'cMLP evidence dimensions do not match its declared configuration.' })
  }
  return ok(result)
}

export function parseClstmEvidence(value: unknown): Result<ClstmEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = clstmEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-discovery-matrix-result', method: 'cLSTM', detail: z.prettifyError(parsed.error) })
  const result = parsed.data
  if (!isSquare(result.summaryScores, result.variables)
    || !isSquare(result.summaryActive, result.variables)
    || result.standardization.means.length !== result.variables
    || result.standardization.scales.length !== result.variables
    || result.checkEvery > result.maxIter
    || result.iterations > result.maxIter) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'cLSTM', detail: 'cLSTM evidence dimensions do not match its declared configuration.' })
  }
  return ok(result)
}

function cdnotsMatricesAreValid(result: CdnotsEvidence | CdnotsPlusEvidence): boolean {
  const variables = result.observedVariables + result.contextVariables.length
  const lags = result.maxLag + 1
  return hasMatrixShape(result.graph, variables, lags)
    && hasMatrixShape(result.pMatrix, variables, lags)
    && hasMatrixShape(result.valMatrix, variables, lags)
}

export function parseCdnotsResult(value: unknown): Result<CdnotsEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = cdnotsEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'CD-NOTS', detail: z.prettifyError(parsed.error) })
  }
  if (!cdnotsMatricesAreValid(parsed.data)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'CD-NOTS', detail: 'CD-NOTS evidence matrices have inconsistent dimensions.' })
  }
  return ok(parsed.data)
}

export function parseCdnotsPlusResult(value: unknown): Result<CdnotsPlusEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = cdnotsPlusEvidenceSchema.safeParse(value)
  if (!parsed.success) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'CD-NOTS+', detail: z.prettifyError(parsed.error) })
  }
  if (!cdnotsMatricesAreValid(parsed.data)) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'CD-NOTS+', detail: 'CD-NOTS+ evidence matrices have inconsistent dimensions.' })
  }
  return ok(parsed.data)
}

export function parseGraceEvidence(value: unknown): Result<GraceEvidence, DiscoveryMatrixBoundaryProblem> {
  const parsed = graceEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-discovery-matrix-result', method: 'GRACE', detail: z.prettifyError(parsed.error) })
  const result = parsed.data
  const lags = result.maxLag + 1
  if (!hasMatrixShape(result.skeleton, result.variables, lags)
    || !hasMatrixShape(result.gateValues, result.variables, lags)
    || !hasMatrixShape(result.graph, result.variables, lags)
    || result.loss.length !== result.epochs
    || result.rmse.length !== result.epochs) {
    return err({ kind: 'invalid-discovery-matrix-result', method: 'GRACE', detail: 'GRACE evidence dimensions do not match its declared configuration.' })
  }
  return ok(result)
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

export type DiscoveryMethodChoice = 'direct-lingam' | 'pc-stable' | 'fci' | 'pcmci-plus' | 'jpcmci-plus' | 'lpcmci' | 'rpcmci' | 'cdnots' | 'cdnots-plus' | 'grace' | 'dynotears' | 'var-lingam' | 'ocse' | 'cmlp' | 'clstm'
export type AcceptedDiscoveryEligibility = Exclude<MethodEligibility, { readonly kind: 'refused' }>

export type DiscoveryMethodGroupId = 'cross-sectional-constraint' | 'pcmci-family' | 'nonstationary-constraint' | 'lingam-family' | 'continuous-optimization' | 'causation-entropy' | 'neural-granger'

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
  methods: ['pcmci-plus', 'jpcmci-plus', 'lpcmci', 'rpcmci'],
}

const CROSS_SECTIONAL_CONSTRAINT: DiscoveryMethodGroup = {
  id: 'cross-sectional-constraint',
  name: 'Cross-sectional constraint',
  description: 'Conditional-independence methods for independent observations; PC-stable returns a CPDAG and FCI returns a PAG.',
  methods: ['pc-stable', 'fci'],
}

const LINGAM_FAMILY: DiscoveryMethodGroup = {
  id: 'lingam-family',
  name: 'LiNGAM family',
  description: 'Linear structural models identified through non-Gaussian disturbances.',
  methods: ['direct-lingam', 'var-lingam'],
}

const NONSTATIONARY_CONSTRAINT: DiscoveryMethodGroup = {
  id: 'nonstationary-constraint',
  name: 'Nonstationary constraint',
  description: 'Constraint-based temporal discovery that represents changing mechanisms with an explicit time-context variable.',
  methods: ['cdnots', 'cdnots-plus'],
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

const NEURAL_GRANGER: DiscoveryMethodGroup = {
  id: 'neural-granger',
  name: 'Neural Granger',
  description: 'Component-wise neural forecasting models with structured sparsity for nonlinear Granger-causality selection.',
  methods: ['grace', 'cmlp', 'clstm'],
}

export const DISCOVERY_METHOD_GROUPS: NonEmptyArray<DiscoveryMethodGroup> = [
  CROSS_SECTIONAL_CONSTRAINT,
  PCMCI_FAMILY,
  NONSTATIONARY_CONSTRAINT,
  LINGAM_FAMILY,
  CONTINUOUS_OPTIMIZATION,
  CAUSATION_ENTROPY,
  NEURAL_GRANGER,
]

export function discoveryMethodGroupById(id: DiscoveryMethodGroupId): DiscoveryMethodGroup {
  switch (id) {
    case 'cross-sectional-constraint': return CROSS_SECTIONAL_CONSTRAINT
    case 'pcmci-family': return PCMCI_FAMILY
    case 'nonstationary-constraint': return NONSTATIONARY_CONSTRAINT
    case 'lingam-family': return LINGAM_FAMILY
    case 'continuous-optimization': return CONTINUOUS_OPTIMIZATION
    case 'causation-entropy': return CAUSATION_ENTROPY
    case 'neural-granger': return NEURAL_GRANGER
    default: return assertNever(id)
  }
}

export function discoveryMethodGroupFor(method: DiscoveryMethodChoice): DiscoveryMethodGroup {
  switch (method) {
    case 'pc-stable':
    case 'fci':
      return CROSS_SECTIONAL_CONSTRAINT
    case 'pcmci-plus':
    case 'jpcmci-plus':
    case 'lpcmci':
    case 'rpcmci':
      return PCMCI_FAMILY
    case 'cdnots':
    case 'cdnots-plus':
      return NONSTATIONARY_CONSTRAINT
    case 'direct-lingam':
    case 'var-lingam':
      return LINGAM_FAMILY
    case 'dynotears':
      return CONTINUOUS_OPTIMIZATION
    case 'ocse':
      return CAUSATION_ENTROPY
    case 'grace':
    case 'cmlp':
    case 'clstm':
      return NEURAL_GRANGER
    default:
      return assertNever(method)
  }
}

export interface ConstraintBackgroundKnowledge {
  readonly forbidden: readonly (readonly [number, number])[]
  readonly required: readonly (readonly [number, number])[]
  readonly forbiddenPatterns: readonly (readonly [string, string])[]
  readonly requiredPatterns: readonly (readonly [string, string])[]
  readonly tiers: readonly (number | null)[]
  readonly forbiddenWithinTiers: readonly number[]
}

export type JpcmciObservedRole = 'system' | 'timeContext' | 'spaceContext'

export interface JpcmciRoleAssignment {
  readonly column: ColumnId
  readonly role: JpcmciObservedRole
}

export type JpcmciRunNode =
  | { readonly kind: 'observed'; readonly column: NumericColumnSelection; readonly role: JpcmciObservedRole }
  | { readonly kind: 'generated'; readonly role: 'timeDummy'; readonly name: 'Time context (generated)' }
  | { readonly kind: 'generated'; readonly role: 'spaceDummy'; readonly name: 'Unit context (generated)' }

export type DiscoveryConfiguration =
  | { readonly kind: 'direct-lingam' }
  | {
      readonly kind: 'pc-stable'
      readonly alpha: PcmciAlpha
      readonly maxDepth: number | null
      readonly ciTest: ConstraintCiTest
      readonly background: ConstraintBackgroundKnowledge
    }
  | {
      readonly kind: 'fci'
      readonly alpha: PcmciAlpha
      readonly maxDepth: number | null
      readonly maxPathLength: number | null
      readonly ciTest: ConstraintCiTest
      readonly background: ConstraintBackgroundKnowledge
    }
  | {
      readonly kind: 'pcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'jpcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
      readonly assignments: readonly JpcmciRoleAssignment[]
      readonly timeDummy: boolean
      readonly spaceDummy: boolean
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
      readonly kind: 'cdnots' | 'cdnots-plus'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly missing: CdnotsMissingStrategy
      readonly context: CdnotsContext
    }
  | {
      readonly kind: 'grace'
      readonly maxLag: DiscoveryLag
      readonly alpha: PcmciAlpha
      readonly context: CdnotsContext
      readonly gateThreshold: number
      readonly epochs: number
      readonly patience: number
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
  | {
      readonly kind: 'cmlp'
      readonly lag: DiscoveryLag
      readonly hidden: NonEmptyArray<number>
      readonly activation: 'sigmoid' | 'tanh' | 'relu' | 'leakyRelu' | 'identity'
      readonly penalty: 'groupLasso' | 'groupSparseGroupLasso' | 'hierarchical'
      readonly lambda: number
      readonly ridgeLambda: number
      readonly learningRate: number
      readonly maxIter: number
      readonly checkEvery: number
      readonly lookback: number
      readonly seed: number
    }
  | {
      readonly kind: 'clstm'
      readonly context: number
      readonly hidden: number
      readonly lambda: number
      readonly ridgeLambda: number
      readonly learningRate: number
      readonly maxIter: number
      readonly checkEvery: number
      readonly lookback: number
      readonly seed: number
    }

type ConfigurationWithKind<C, K> = C extends { readonly kind: infer CK } ? (K extends CK ? C : never) : never
/** The configuration member whose kind covers K, so a run carries exactly the settings of its own method. */
export type DiscoveryConfigurationOf<K extends DiscoveryConfiguration['kind']> = ConfigurationWithKind<DiscoveryConfiguration, K>

export type DiscoveryRunArtifact =
  | {
      readonly kind: 'direct-lingam-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'direct-lingam'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof DIRECT_LINGAM_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: DirectLingamEvidence
    }
  | {
      readonly kind: 'pc-stable-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'pc-stable'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof PC_STABLE_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: PcStableEvidence
    }
  | {
      readonly kind: 'fci-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'fci'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof FCI_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: FciEvidence
    }
  | {
      readonly kind: 'pcmci-plus-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'pcmci-plus'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof PCMCI_PLUS_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: PcmciPlusEvidence
    }
  | {
      readonly kind: 'jpcmci-plus-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'jpcmci-plus'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof JPCMCI_PLUS_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly nodes: NonEmptyArray<JpcmciRunNode>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: JpcmciPlusEvidence
    }
  | {
      readonly kind: 'lpcmci-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'lpcmci'>
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
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'rpcmci'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof RPCMCI_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: RpcmciEvidence
    }
  | {
      readonly kind: 'cdnots-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'cdnots'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof CDNOTS_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: CdnotsEvidence
    }
  | {
      readonly kind: 'cdnots-plus-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'cdnots-plus'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof CDNOTS_PLUS_PAR_CORR_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: CdnotsPlusEvidence
    }
  | {
      readonly kind: 'grace-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'grace'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof GRACE_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: GraceEvidence
    }
  | {
      readonly kind: 'dynotears-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'dynotears'>
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
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'var-lingam'>
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
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'ocse'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof OCSE_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: OcseEvidence
    }
  | {
      readonly kind: 'cmlp-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'cmlp'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof CMLP_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: CmlpEvidence
    }
  | {
      readonly kind: 'clstm-run'
      readonly id: DiscoveryRunId
      /** The settings the run was made with, so a bundle replays the chapter exactly as its author left it. */
      readonly configuration: DiscoveryConfigurationOf<'clstm'>
      readonly preparedDataset: PreparedDatasetVersionId
      readonly createdAt: string
      readonly method: typeof CLSTM_METHOD_ID
      readonly variables: NonEmptyArray<NumericColumnSelection>
      readonly eligibility: AcceptedDiscoveryEligibility
      readonly result: ClstmEvidence
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

export interface DiscoveryDraft {
  readonly configuration: DiscoveryConfiguration
}

export type DiscoverySession =
  | { readonly kind: 'without-prepared-dataset' }
  | {
      readonly kind: 'with-prepared-dataset'
      readonly preparedDataset: PreparedDatasetVersionId
      readonly draft: DiscoveryDraft
    }

export type DiscoveryEvent =
  | { readonly type: 'method-selected'; readonly method: DiscoveryMethodChoice }
  | { readonly type: 'tau-max-selected'; readonly value: DiscoveryLag }
  | { readonly type: 'pc-alpha-selected'; readonly value: PcmciAlpha }
  | { readonly type: 'constraint-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'pc-stable' | 'fci' }> }
  | { readonly type: 'jpcmci-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'jpcmci-plus' }> }
  | { readonly type: 'rpcmci-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }> }
  | { readonly type: 'cdnots-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'cdnots' | 'cdnots-plus' }> }
  | { readonly type: 'grace-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'grace' }> }
  | { readonly type: 'dynotears-lambda-w-selected'; readonly value: DynotearsPenalty }
  | { readonly type: 'dynotears-lambda-a-selected'; readonly value: DynotearsPenalty }
  | { readonly type: 'var-lingam-prune-selected'; readonly value: boolean }
  | { readonly type: 'ocse-alpha-selected'; readonly value: PcmciAlpha }
  | { readonly type: 'ocse-shuffles-selected'; readonly value: OcseShuffles }
  | { readonly type: 'ocse-method-selected'; readonly value: OcseInformationMethod }
  | { readonly type: 'neural-configured'; readonly configuration: Extract<DiscoveryConfiguration, { readonly kind: 'cmlp' | 'clstm' }> }
  | { readonly type: 'max-lag-selected'; readonly value: DiscoveryLag }

export type DiscoveryRunEvent =
  | { readonly type: 'run-cancelled' }
  | { readonly type: 'run-progressed'; readonly progress: DiscoveryProgress }
  | { readonly type: 'run-failed'; readonly problem: DiscoveryRunProblem }

export type DiscoverySessionEvent =
  | { readonly type: 'prepared-dataset-changed'; readonly prepared: PreparedDatasetArtifact | null; readonly recorded?: DiscoveryRunArtifact | null }
  | { readonly type: 'discovery-event-received'; readonly event: DiscoveryEvent }

export const INITIAL_DISCOVERY_DRAFT: DiscoveryDraft = {
  configuration: { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 },
}

/**
 * A run as saved before runs carried their configuration, completed from what the method reported
 * back: the lags, the alphas and the J-PCMCI+ roles. What a method did not report stays at its
 * default. A run that already carries its configuration is returned as it is. The saved artifact is
 * left alone, so reopening a project does not rewrite it.
 */
export function withRecordedConfiguration(run: DiscoveryRunArtifact): DiscoveryRunArtifact {
  if (run.configuration !== undefined) return run
  // The controls offer a fixed list of lags and alphas; a recorded value off the list keeps the default.
  const lag = (value: number, fallback: DiscoveryLag): DiscoveryLag => DISCOVERY_LAG_OPTIONS.find((option) => option === value) ?? fallback
  const alpha = (value: number, fallback: PcmciAlpha): PcmciAlpha => PCMCI_ALPHA_OPTIONS.find((option) => option === value) ?? fallback
  switch (run.kind) {
    case 'direct-lingam-run': return { ...run, configuration: { kind: 'direct-lingam' } }
    case 'pc-stable-run': return { ...run, configuration: { kind: 'pc-stable', alpha: alpha(run.result.alpha, 0.05), maxDepth: null, ciTest: 'fisherZ', background: emptyConstraintBackgroundKnowledge() } }
    case 'fci-run': return { ...run, configuration: { kind: 'fci', alpha: alpha(run.result.alpha, 0.05), maxDepth: null, maxPathLength: null, ciTest: 'fisherZ', background: emptyConstraintBackgroundKnowledge() } }
    case 'pcmci-plus-run': return { ...run, configuration: { kind: 'pcmci-plus', tauMax: lag(run.result.tauMax, 2), pcAlpha: alpha(run.result.pcAlpha, 0.05) } }
    case 'jpcmci-plus-run': return {
      ...run,
      configuration: {
        kind: 'jpcmci-plus',
        tauMax: lag(run.result.tauMax, 2),
        pcAlpha: alpha(run.result.pcAlpha, 0.05),
        assignments: run.nodes.flatMap((node) => node.kind === 'observed' && node.role !== 'system' ? [{ column: node.column.id, role: node.role }] : []),
        timeDummy: run.nodes.some((node) => node.kind === 'generated' && node.role === 'timeDummy'),
        spaceDummy: run.nodes.some((node) => node.kind === 'generated' && node.role === 'spaceDummy'),
      },
    }
    case 'lpcmci-run': return { ...run, configuration: { kind: 'lpcmci', tauMax: lag(run.result.tauMax, 2), pcAlpha: alpha(run.result.pcAlpha, 0.05) } }
    case 'rpcmci-run': return { ...run, configuration: { kind: 'rpcmci', numRegimes: 2, maxTransitions: 4, switchThres: 0.05, numIterations: 20, maxAnneal: 10, tauMin: 1, tauMax: lag(run.result.tauMax, 1), pcAlpha: alpha(run.result.pcAlpha, 0.2), alphaLevel: alpha(run.result.alphaLevel, 0.01), seed: 327 } }
    case 'cdnots-run': return { ...run, configuration: { kind: 'cdnots', maxLag: lag(run.result.maxLag, 2), alpha: alpha(run.result.alpha, 0.05), missing: 'pairwiseComplete', context: 'linear' } }
    case 'cdnots-plus-run': return { ...run, configuration: { kind: 'cdnots-plus', maxLag: lag(run.result.maxLag, 2), alpha: alpha(run.result.alpha, 0.01), missing: 'pairwiseComplete', context: 'linear' } }
    case 'grace-run': return { ...run, configuration: { kind: 'grace', maxLag: lag(run.result.maxLag, 2), alpha: alpha(run.result.alpha, 0.05), context: 'linear', gateThreshold: 0.5, epochs: 150, patience: 20, seed: 0 } }
    case 'dynotears-run': return { ...run, configuration: { kind: 'dynotears', maxLag: lag(run.result.maxLag, 2), lambdaW: 0.1, lambdaA: 0.1 } }
    case 'var-lingam-run': return { ...run, configuration: { kind: 'var-lingam', maxLag: 2, prune: true } }
    case 'ocse-run': return { ...run, configuration: { kind: 'ocse', maxLag: lag(run.result.maxLag, 2), alpha: alpha(run.result.alpha, 0.05), nShuffles: 50, method: run.result.method, k: 5 } }
    case 'cmlp-run': return { ...run, configuration: { kind: 'cmlp', lag: 3, hidden: [100], activation: 'relu', penalty: 'hierarchical', lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 50_000, checkEvery: 100, lookback: 5, seed: 0 } }
    case 'clstm-run': return { ...run, configuration: { kind: 'clstm', context: 10, hidden: 100, lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 20_000, checkEvery: 50, lookback: 5, seed: 0 } }
    default: return assertNever(run)
  }
}

export const initialDiscoveryDraftFor = (prepared: PreparedDatasetArtifact, recorded: DiscoveryRunArtifact | null = null): DiscoveryDraft => ({
  configuration: recorded !== null && recorded.preparedDataset === prepared.id
    ? withRecordedConfiguration(recorded).configuration
    : prepared.kind === 'prepared-cross-section'
      ? { kind: 'direct-lingam' }
      : prepared.kind === 'prepared-panel'
        ? initialConfigurationFor('jpcmci-plus')
        : INITIAL_DISCOVERY_DRAFT.configuration,
})

export const initialDiscoverySessionFor = (prepared: PreparedDatasetArtifact | null, recorded: DiscoveryRunArtifact | null = null): DiscoverySession =>
  prepared === null
    ? { kind: 'without-prepared-dataset' }
    : {
        kind: 'with-prepared-dataset',
        preparedDataset: prepared.id,
        draft: initialDiscoveryDraftFor(prepared, recorded),
      }

export function stepDiscoverySession(state: DiscoverySession, event: DiscoverySessionEvent): DiscoverySession {
  switch (event.type) {
    case 'prepared-dataset-changed':
      if (event.prepared === null) return { kind: 'without-prepared-dataset' }
      if (state.kind === 'with-prepared-dataset' && state.preparedDataset === event.prepared.id) return state
      return initialDiscoverySessionFor(event.prepared, event.recorded ?? null)
    case 'discovery-event-received':
      return state.kind === 'with-prepared-dataset'
        ? { ...state, draft: stepDiscovery(state.draft, event.event) }
        : state
    default: return assertNever(event)
  }
}

export function stepDiscovery(state: DiscoveryDraft, event: DiscoveryEvent): DiscoveryDraft {
  switch (event.type) {
    case 'method-selected':
      return {
        configuration: initialConfigurationFor(event.method),
      }
    case 'tau-max-selected':
      return state.configuration.kind === 'pcmci-plus' || state.configuration.kind === 'jpcmci-plus' || state.configuration.kind === 'lpcmci'
        ? { configuration: { ...state.configuration, tauMax: event.value } }
        : state
    case 'pc-alpha-selected':
      return state.configuration.kind === 'pcmci-plus' || state.configuration.kind === 'jpcmci-plus' || state.configuration.kind === 'lpcmci'
        ? { configuration: { ...state.configuration, pcAlpha: event.value } }
        : state
    case 'constraint-configured':
      return state.configuration.kind === event.configuration.kind
        ? { configuration: event.configuration }
        : state
    case 'jpcmci-configured':
      return state.configuration.kind === 'jpcmci-plus'
        ? { configuration: event.configuration }
        : state
    case 'rpcmci-configured':
      return state.configuration.kind === 'rpcmci'
        ? { configuration: event.configuration }
        : state
    case 'cdnots-configured':
      return state.configuration.kind === event.configuration.kind
        ? { configuration: event.configuration }
        : state
    case 'grace-configured':
      return state.configuration.kind === 'grace'
        ? { configuration: event.configuration }
        : state
    case 'dynotears-lambda-w-selected':
      return state.configuration.kind === 'dynotears'
        ? { configuration: { ...state.configuration, lambdaW: event.value } }
        : state
    case 'dynotears-lambda-a-selected':
      return state.configuration.kind === 'dynotears'
        ? { configuration: { ...state.configuration, lambdaA: event.value } }
        : state
    case 'var-lingam-prune-selected':
      return state.configuration.kind === 'var-lingam'
        ? { configuration: { ...state.configuration, prune: event.value } }
        : state
    case 'ocse-alpha-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, alpha: event.value } }
        : state
    case 'ocse-shuffles-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, nShuffles: event.value } }
        : state
    case 'ocse-method-selected':
      return state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, method: event.value } }
        : state
    case 'neural-configured':
      return state.configuration.kind === event.configuration.kind
        ? { configuration: event.configuration }
        : state
    case 'max-lag-selected':
      return state.configuration.kind === 'dynotears' || state.configuration.kind === 'var-lingam' || state.configuration.kind === 'ocse'
        ? { configuration: { ...state.configuration, maxLag: event.value } }
        : state
    default: return assertNever(event)
  }
}

export function initialConfigurationFor(method: DiscoveryMethodChoice): DiscoveryConfiguration {
  switch (method) {
    case 'direct-lingam': return { kind: 'direct-lingam' }
    case 'pc-stable': return { kind: 'pc-stable', alpha: 0.05, maxDepth: null, ciTest: 'fisherZ', background: emptyConstraintBackgroundKnowledge() }
    case 'fci': return { kind: 'fci', alpha: 0.05, maxDepth: null, maxPathLength: null, ciTest: 'fisherZ', background: emptyConstraintBackgroundKnowledge() }
    case 'pcmci-plus': return { kind: 'pcmci-plus', tauMax: 2, pcAlpha: 0.05 }
    case 'jpcmci-plus': return { kind: 'jpcmci-plus', tauMax: 2, pcAlpha: 0.05, assignments: [], timeDummy: true, spaceDummy: true }
    case 'lpcmci': return { kind: 'lpcmci', tauMax: 2, pcAlpha: 0.05 }
    case 'rpcmci': return { kind: 'rpcmci', numRegimes: 2, maxTransitions: 4, switchThres: 0.05, numIterations: 20, maxAnneal: 10, tauMin: 1, tauMax: 1, pcAlpha: 0.2, alphaLevel: 0.01, seed: 327 }
    case 'cdnots': return { kind: 'cdnots', maxLag: 2, alpha: 0.05, missing: 'pairwiseComplete', context: 'linear' }
    case 'cdnots-plus': return { kind: 'cdnots-plus', maxLag: 2, alpha: 0.01, missing: 'pairwiseComplete', context: 'linear' }
    case 'grace': return { kind: 'grace', maxLag: 2, alpha: 0.05, context: 'linear', gateThreshold: 0.5, epochs: 150, patience: 20, seed: 0 }
    case 'dynotears': return { kind: 'dynotears', maxLag: 2, lambdaW: 0.1, lambdaA: 0.1 }
    case 'var-lingam': return { kind: 'var-lingam', maxLag: 2, prune: true }
    case 'ocse': return { kind: 'ocse', maxLag: 2, alpha: 0.05, nShuffles: 50, method: 'gaussian', k: 5 }
    case 'cmlp': return { kind: 'cmlp', lag: 3, hidden: [100], activation: 'relu', penalty: 'hierarchical', lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 50_000, checkEvery: 100, lookback: 5, seed: 0 }
    case 'clstm': return { kind: 'clstm', context: 10, hidden: 100, lambda: 0.1, ridgeLambda: 0.01, learningRate: 0.01, maxIter: 20_000, checkEvery: 50, lookback: 5, seed: 0 }
    default: return assertNever(method)
  }
}

function emptyConstraintBackgroundKnowledge(): ConstraintBackgroundKnowledge {
  return {
    forbidden: [],
    required: [],
    forbiddenPatterns: [],
    requiredPatterns: [],
    tiers: [],
    forbiddenWithinTiers: [],
  }
}

export type ReadyDiscoverySpecification =
  | { readonly kind: 'direct-lingam' }
  | Extract<DiscoveryConfiguration, { readonly kind: 'pc-stable' | 'fci' }>
  | {
      readonly kind: 'pcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | {
      readonly kind: 'jpcmci-plus'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
      readonly roles: NonEmptyArray<JpcmciObservedRole>
      readonly timeDummy: boolean
      readonly spaceDummy: boolean
    }
  | {
      readonly kind: 'lpcmci'
      readonly tauMax: DiscoveryLag
      readonly pcAlpha: PcmciAlpha
    }
  | Extract<DiscoveryConfiguration, { readonly kind: 'rpcmci' }>
  | Extract<DiscoveryConfiguration, { readonly kind: 'cdnots' | 'cdnots-plus' | 'grace' }>
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
  | Extract<DiscoveryConfiguration, { readonly kind: 'cmlp' | 'clstm' }>

export type DiscoveryReadinessProblem =
  | { readonly kind: 'time-series-required' }
  | { readonly kind: 'panel-required' }
  | { readonly kind: 'cross-section-required' }
  | { readonly kind: 'at-least-two-variables-required' }
  | { readonly kind: 'too-few-observations'; readonly required: number; readonly available: number }
  | { readonly kind: 'dense-browser-boundary-required' }
  | { readonly kind: 'browser-lag-limit'; readonly method: 'RPCMCI' | 'DYNOTEARS' | 'VAR-LiNGAM' | 'oCSE' | 'cMLP'; readonly maximum: number }
  | { readonly kind: 'transition-budget-too-large'; readonly available: number }
  | { readonly kind: 'balanced-panel-required' }
  | { readonly kind: 'at-least-two-panel-units-required'; readonly available: number }
  | { readonly kind: 'at-least-two-system-variables-required'; readonly available: number }
  | { readonly kind: 'duplicate-context-assignment'; readonly column: string }
  | { readonly kind: 'unknown-context-assignment'; readonly column: string }
  | { readonly kind: 'complete-interval-unsupported' }

export function readyDiscoverySpecification(
  configuration: DiscoveryConfiguration,
  prepared: PreparedDatasetArtifact,
): Result<ReadyDiscoverySpecification, DiscoveryReadinessProblem> {
  if (configuration.kind === 'jpcmci-plus') {
    if (prepared.kind !== 'prepared-panel') return err({ kind: 'panel-required' })
    if (!prepared.panel.balanced) return err({ kind: 'balanced-panel-required' })
    if (prepared.resolution.kind === 'window') return err({ kind: 'complete-interval-unsupported' })
    if (prepared.panel.units < 2) return err({ kind: 'at-least-two-panel-units-required', available: prepared.panel.units })
    const roles = new Map<string, JpcmciObservedRole>()
    for (const assignment of configuration.assignments) {
      if (!prepared.columns.some((column) => column === assignment.column)) return err({ kind: 'unknown-context-assignment', column: assignment.column })
      if (roles.has(assignment.column)) return err({ kind: 'duplicate-context-assignment', column: assignment.column })
      roles.set(assignment.column, assignment.role)
    }
    const orderedRoles = prepared.columns.map((column) => roles.get(column) ?? 'system')
    if (!isNonEmpty(orderedRoles)) return err({ kind: 'at-least-two-variables-required' })
    const systems = orderedRoles.filter((role) => role === 'system').length
    if (systems < 2) return err({ kind: 'at-least-two-system-variables-required', available: systems })
    const required = Math.max(2 * configuration.tauMax + 16, 24)
    if (prepared.panel.periods < required) return err({ kind: 'too-few-observations', required, available: prepared.panel.periods })
    return ok({ ...configuration, roles: orderedRoles })
  }
  if (prepared.missingness.kind === 'lag-aware-exclusion'
    && configuration.kind !== 'pcmci-plus'
    && configuration.kind !== 'lpcmci'
    && configuration.kind !== 'cdnots'
    && configuration.kind !== 'cdnots-plus'
    && configuration.kind !== 'grace') {
    return err({ kind: 'dense-browser-boundary-required' })
  }
  if (configuration.kind === 'direct-lingam' || configuration.kind === 'pc-stable' || configuration.kind === 'fci') {
    if (prepared.kind !== 'prepared-cross-section') return err({ kind: 'cross-section-required' })
    if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
    const required = prepared.columns.length + 16
    return prepared.observations < required
      ? err({ kind: 'too-few-observations', required, available: prepared.observations })
      : ok(configuration)
  }
  if (prepared.kind !== 'prepared-time-series') return err({ kind: 'time-series-required' })
  switch (configuration.kind) {
    case 'pcmci-plus': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = Math.max(2 * configuration.tauMax + 16, 24)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'lpcmci': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = Math.max(2 * configuration.tauMax + 16, 24)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'rpcmci': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (configuration.tauMax > 6) return err({ kind: 'browser-lag-limit', method: 'RPCMCI', maximum: 6 })
      if (configuration.maxTransitions >= prepared.observations) return err({ kind: 'transition-budget-too-large', available: prepared.observations })
      const required = Math.max(2 * configuration.tauMax + 24, 40)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'cdnots':
    case 'cdnots-plus': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = Math.max(2 * configuration.maxLag + 30, 40)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'grace': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = Math.max(2 * configuration.maxLag + 30, configuration.maxLag + 32)
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'dynotears': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (configuration.maxLag > 6) return err({ kind: 'browser-lag-limit', method: 'DYNOTEARS', maximum: 6 })
      const required = configuration.maxLag + 16
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'var-lingam': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (configuration.maxLag > 6) return err({ kind: 'browser-lag-limit', method: 'VAR-LiNGAM', maximum: 6 })
      const required = prepared.columns.length * (configuration.maxLag + 1) + 16
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'ocse': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (configuration.maxLag > 8) return err({ kind: 'browser-lag-limit', method: 'oCSE', maximum: 8 })
      const required = configuration.maxLag + 24
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'cmlp': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      if (configuration.lag > 20) return err({ kind: 'browser-lag-limit', method: 'cMLP', maximum: 20 })
      const required = configuration.lag + 16
      return prepared.observations < required
        ? err({ kind: 'too-few-observations', required, available: prepared.observations })
        : ok(configuration)
    }
    case 'clstm': {
      if (prepared.columns.length < 2) return err({ kind: 'at-least-two-variables-required' })
      const required = configuration.context + 16
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
  const supportsRoleAwareSamples = method.id === PCMCI_PLUS_PAR_CORR_METHOD_ID
    || method.id === LPCMCI_PAR_CORR_METHOD_ID
    || method.id === CDNOTS_PAR_CORR_METHOD_ID
    || method.id === CDNOTS_PLUS_PAR_CORR_METHOD_ID
    || method.id === GRACE_METHOD_ID
  if (prepared.missingness.kind === 'lag-aware-exclusion' && !supportsRoleAwareSamples) {
    const missingnessCaveat = method.caveats.find((caveat) => caveat.category === 'missingness') ?? firstCaveat
    return {
      kind: 'refused',
      satisfied: [],
      unresolved: [],
      violations: [{
        kind: 'violated',
        caveat: missingnessCaveat,
        evidence: 'This method requires a dense prepared matrix. Lag-aware sample handling is available for PCMCI+, LPCMCI, CD-NOTS, CD-NOTS+ and GRACE.',
      }],
    }
  }
  if (method.id === JPCMCI_PLUS_PAR_CORR_METHOD_ID) {
    if (prepared.kind !== 'prepared-panel' || !prepared.panel.balanced || prepared.panel.units < 2) {
      const samplingCaveat = method.caveats.find((caveat) => caveat.category === 'sampling-structure') ?? firstCaveat
      return {
        kind: 'refused',
        satisfied: [],
        unresolved: [],
        violations: [{
          kind: 'violated',
          caveat: samplingCaveat,
          evidence: prepared.kind !== 'prepared-panel'
            ? 'J-PCMCI+ requires repeated observations arranged as a regular panel.'
            : 'J-PCMCI+ requires at least two units and exactly one row for every shared unit-period cell.',
        }],
      }
    }
    const satisfied: Extract<CaveatEvaluation, { readonly kind: 'satisfied' }>[] = []
    const unresolved: Extract<CaveatEvaluation, { readonly kind: 'unresolved' }>[] = []
    for (const caveat of method.caveats) {
      if (caveat.category === 'interpretation') continue
      if (caveat.category === 'sampling-structure') {
        satisfied.push({ kind: 'satisfied', caveat, evidence: `Prepared as a balanced panel with ${prepared.panel.units} units and ${prepared.panel.periods} shared periods.` })
      } else if (caveat.category === 'missingness' && prepared.missingness.kind === 'not-present') {
        satisfied.push({ kind: 'satisfied', caveat, evidence: 'The prepared dataset contains no missing values.' })
      } else {
        unresolved.push({ kind: 'unresolved', caveat, missingEvidence: '' })
      }
    }
    return isNonEmpty(unresolved)
      ? { kind: 'caution', satisfied, unresolved }
      : { kind: 'eligible', satisfied }
  }
  if (method.id === DIRECT_LINGAM_METHOD_ID || method.id === PC_STABLE_METHOD_ID || method.id === FCI_METHOD_ID) {
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
      const readings = prepared.columns.map((column) => ({ name: columnNameOf(column), assessment: stationarity.variables.find((variable) => variable.column === column)?.assessment ?? null }))
      const { refused, cautions } = levelEvidence(readings, { integrated: 'links found on levels can be spurious; a differenced version is the safer input.' })
      const review = [...refused, ...cautions]
      if (review.length === 0) {
        satisfied.push({ kind: 'satisfied', caveat, evidence: `${readings.map((entry) => entry.name).join(', ')} ${readings.length === 1 ? 'is' : 'are'} stationary on the prepared scale.` })
        continue
      }
      unresolved.push({ kind: 'unresolved', caveat, missingEvidence: summaries(review), groups: nonEmptyGroups(review) })
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
    case 'panel-required': return 'J-PCMCI+ needs a balanced regular panel with repeated periods for at least two units.'
    case 'cross-section-required': return 'This method needs independent cross-sectional observations. Prepare this dataset as a cross-section.'
    case 'at-least-two-variables-required': return 'Select at least 2 variables.'
    case 'too-few-observations': return `This configuration needs at least ${problem.required} rows; ${problem.available} are available. Use more rows or choose a smaller configuration.`
    case 'dense-browser-boundary-required': return 'Choose a complete interval or imputation. This method needs complete numeric columns in the browser.'
    case 'browser-lag-limit': return `${problem.method} accepts a maximum lag of ${problem.maximum} in the browser. Lower the maximum lag.`
    case 'transition-budget-too-large': return `The maximum transition count must be smaller than the ${problem.available} available observations.`
    case 'balanced-panel-required': return 'J-PCMCI+ needs exactly one row for every unit-period cell. Repair duplicate or missing panel cells in Data studio.'
    case 'at-least-two-panel-units-required': return `J-PCMCI+ needs at least two panel units; ${problem.available} is available.`
    case 'at-least-two-system-variables-required': return `J-PCMCI+ needs at least two system variables; ${problem.available} remains after the context assignments.`
    case 'duplicate-context-assignment': return `The J-PCMCI+ role for ${problem.column} is recorded more than once.`
    case 'unknown-context-assignment': return `The J-PCMCI+ role assignment refers to ${problem.column}, which is not in this prepared version.`
    case 'complete-interval-unsupported': return 'J-PCMCI+ cannot use a complete-row interval because that can remove different unit-period cells. Prepare the panel with complete data or a recorded row-preserving imputation.'
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

/**
 * The settings a reader changed from the method's defaults, by field name. The method itself is not
 * a setting, and background knowledge is a statement about the world rather than a knob, so neither
 * counts.
 */
export const changedSettings = (configuration: DiscoveryConfiguration): readonly string[] => {
  const defaults = initialConfigurationFor(configuration.kind) as unknown as Record<string, unknown>
  return Object.keys(configuration)
    .filter((key) => key !== 'kind' && key !== 'background')
    .filter((key) => JSON.stringify(Reflect.get(configuration, key)) !== JSON.stringify(defaults[key]))
}

/** How a neural Granger run standardises its inputs, as both result cards state it. */
export const STANDARDISED_INPUTS_NOTE = 'Each selected column was centered and scaled by its recorded population standard deviation before training.'
