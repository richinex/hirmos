import {
  assertNever,
  brand,
  err,
  isNonEmpty,
  ok,
  type Brand,
  type NonEmptyArray,
  type Result,
} from './dop'
import type { ColumnId, DatasetProfile } from './dataset'
import type { DiscoveryRunArtifact, DiscoveryRunId } from './discovery'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'

export type DagDocumentId = Brand<string, 'DagDocumentId'>
export type DagRevisionId = Brand<string, 'DagRevisionId'>
export type DagNodeId = Brand<string, 'DagNodeId'>
export type DagEdgeId = Brand<string, 'DagEdgeId'>
export type DagName = Brand<string, 'DagName'>
export type EdgeRationale = Brand<string, 'EdgeRationale'>

export type DagOrigin =
  | { readonly kind: 'user-authored'; readonly basis: 'domain-knowledge' | 'experimental-design' }
  | { readonly kind: 'discovery-informed'; readonly reports: NonEmptyArray<DiscoveryRunId> }

export interface ObservedDagNode {
  readonly kind: 'observed'
  readonly id: DagNodeId
  readonly column: ColumnId
  readonly name: string
}

export type EdgeSupport =
  | { readonly kind: 'user-assumption'; readonly rationale: EdgeRationale }
  | { readonly kind: 'experimental-design'; readonly rationale: EdgeRationale }

export interface DirectedDagEdge {
  readonly kind: 'directed'
  readonly id: DagEdgeId
  readonly cause: DagNodeId
  readonly effect: DagNodeId
  readonly support: EdgeSupport
}

export interface EditableDag {
  readonly kind: 'editable-dag'
  readonly nodes: NonEmptyArray<ObservedDagNode>
  readonly edges: readonly DirectedDagEdge[]
}

export type DagStructuralIssue = { readonly kind: 'no-edges' }

export type DagStructuralValidation =
  | { readonly kind: 'incomplete'; readonly issues: NonEmptyArray<DagStructuralIssue> }
  | { readonly kind: 'structurally-valid' }

export interface DagDraftRevision {
  readonly kind: 'draft'
  readonly id: DagRevisionId
  readonly parent: DagRevisionId | null
  readonly createdAt: string
  readonly graph: EditableDag
  readonly validation: DagStructuralValidation
}

export interface DagDocument {
  readonly kind: 'dag-document'
  readonly id: DagDocumentId
  readonly name: DagName
  readonly preparedDataset: PreparedDatasetVersionId
  readonly origin: DagOrigin
  readonly history: readonly DagDraftRevision[]
  readonly current: DagDraftRevision
}

export type DagOriginChoice = 'domain-knowledge' | 'experimental-design' | 'discovery-informed'

export type DagCreateProblem =
  | { readonly kind: 'empty-name' }
  | { readonly kind: 'name-too-long'; readonly maximum: number }
  | { readonly kind: 'prepared-column-missing'; readonly column: ColumnId }
  | { readonly kind: 'discovery-evidence-required' }

export type DagEditProblem =
  | { readonly kind: 'cause-required' }
  | { readonly kind: 'effect-required' }
  | { readonly kind: 'unknown-node'; readonly node: DagNodeId }
  | { readonly kind: 'self-edge' }
  | { readonly kind: 'duplicate-edge' }
  | { readonly kind: 'directed-cycle' }
  | { readonly kind: 'empty-rationale' }
  | { readonly kind: 'rationale-too-long'; readonly maximum: number }
  | { readonly kind: 'unknown-edge'; readonly edge: DagEdgeId }

const MAX_DAG_NAME = 100
const MAX_EDGE_RATIONALE = 600

const dagName = (raw: string): Result<DagName, Extract<DagCreateProblem, { readonly kind: 'empty-name' | 'name-too-long' }>> => {
  const value = raw.trim()
  if (value.length === 0) return err({ kind: 'empty-name' })
  if (value.length > MAX_DAG_NAME) return err({ kind: 'name-too-long', maximum: MAX_DAG_NAME })
  return ok(brand<string, 'DagName'>(value))
}

const edgeRationale = (
  raw: string,
): Result<EdgeRationale, Extract<DagEditProblem, { readonly kind: 'empty-rationale' | 'rationale-too-long' }>> => {
  const value = raw.trim()
  if (value.length === 0) return err({ kind: 'empty-rationale' })
  if (value.length > MAX_EDGE_RATIONALE) return err({ kind: 'rationale-too-long', maximum: MAX_EDGE_RATIONALE })
  return ok(brand<string, 'EdgeRationale'>(value))
}

const newDagDocumentId = (): DagDocumentId => brand<string, 'DagDocumentId'>(crypto.randomUUID())
const newDagRevisionId = (): DagRevisionId => brand<string, 'DagRevisionId'>(crypto.randomUUID())
const dagNodeId = (prepared: PreparedDatasetVersionId, column: ColumnId): DagNodeId =>
  brand<string, 'DagNodeId'>(`${prepared}:${column}`)
const dagEdgeId = (cause: DagNodeId, effect: DagNodeId): DagEdgeId =>
  brand<string, 'DagEdgeId'>(`${cause}->${effect}`)

const validateStructure = (graph: EditableDag): DagStructuralValidation =>
  graph.edges.length === 0
    ? { kind: 'incomplete', issues: [{ kind: 'no-edges' }] }
    : { kind: 'structurally-valid' }

const originFromChoice = (
  choice: DagOriginChoice,
  discoveryRuns: readonly DiscoveryRunArtifact[],
): Result<DagOrigin, Extract<DagCreateProblem, { readonly kind: 'discovery-evidence-required' }>> => {
  switch (choice) {
    case 'domain-knowledge': return ok({ kind: 'user-authored', basis: 'domain-knowledge' })
    case 'experimental-design': return ok({ kind: 'user-authored', basis: 'experimental-design' })
    case 'discovery-informed': {
      const reports = discoveryRuns.map((run) => run.id)
      return isNonEmpty(reports)
        ? ok({ kind: 'discovery-informed', reports })
        : err({ kind: 'discovery-evidence-required' })
    }
    default: return assertNever(choice)
  }
}

export function createDagDocument(
  rawName: string,
  originChoice: DagOriginChoice,
  prepared: PreparedDatasetArtifact,
  profile: DatasetProfile,
  discoveryRuns: readonly DiscoveryRunArtifact[],
): Result<DagDocument, DagCreateProblem> {
  const name = dagName(rawName)
  if (!name.ok) return name
  const origin = originFromChoice(originChoice, discoveryRuns)
  if (!origin.ok) return origin

  const nodes: ObservedDagNode[] = []
  for (const column of prepared.columns) {
    const profiled = profile.columns.find((candidate) => candidate.id === column)
    if (profiled === undefined) return err({ kind: 'prepared-column-missing', column })
    nodes.push({ kind: 'observed', id: dagNodeId(prepared.id, column), column, name: profiled.name })
  }
  if (!isNonEmpty(nodes)) return err({ kind: 'prepared-column-missing', column: prepared.columns[0] })

  const graph: EditableDag = { kind: 'editable-dag', nodes, edges: [] }
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: null,
    createdAt: new Date().toISOString(),
    graph,
    validation: validateStructure(graph),
  }
  return ok({
    kind: 'dag-document',
    id: newDagDocumentId(),
    name: name.value,
    preparedDataset: prepared.id,
    origin: origin.value,
    history: [],
    current,
  })
}

const hasPath = (graph: EditableDag, start: DagNodeId, target: DagNodeId): boolean => {
  const pending: DagNodeId[] = [start]
  const visited = new Set<DagNodeId>()
  while (pending.length > 0) {
    const node = pending.pop()
    if (node === undefined || visited.has(node)) continue
    if (node === target) return true
    visited.add(node)
    for (const edge of graph.edges) if (edge.cause === node) pending.push(edge.effect)
  }
  return false
}

const supportFor = (origin: DagOrigin, rationale: EdgeRationale): EdgeSupport => {
  switch (origin.kind) {
    case 'user-authored': return origin.basis === 'experimental-design'
      ? { kind: 'experimental-design', rationale }
      : { kind: 'user-assumption', rationale }
    case 'discovery-informed': return { kind: 'user-assumption', rationale }
    default: return assertNever(origin)
  }
}

export function reviseDagWithEdge(
  document: DagDocument,
  cause: DagNodeId | null,
  effect: DagNodeId | null,
  rawRationale: string,
): Result<DagDocument, DagEditProblem> {
  if (cause === null) return err({ kind: 'cause-required' })
  if (effect === null) return err({ kind: 'effect-required' })
  if (!document.current.graph.nodes.some((node) => node.id === cause)) return err({ kind: 'unknown-node', node: cause })
  if (!document.current.graph.nodes.some((node) => node.id === effect)) return err({ kind: 'unknown-node', node: effect })
  if (cause === effect) return err({ kind: 'self-edge' })
  if (document.current.graph.edges.some((edge) => edge.cause === cause && edge.effect === effect)) {
    return err({ kind: 'duplicate-edge' })
  }
  if (hasPath(document.current.graph, effect, cause)) return err({ kind: 'directed-cycle' })
  const rationale = edgeRationale(rawRationale)
  if (!rationale.ok) return rationale

  const edge: DirectedDagEdge = {
    kind: 'directed',
    id: dagEdgeId(cause, effect),
    cause,
    effect,
    support: supportFor(document.origin, rationale.value),
  }
  const graph: EditableDag = {
    ...document.current.graph,
    edges: [...document.current.graph.edges, edge],
  }
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: document.current.id,
    createdAt: new Date().toISOString(),
    graph,
    validation: validateStructure(graph),
  }
  return { ok: true, value: { ...document, history: [...document.history, document.current], current } }
}

export function reviseDagWithoutEdge(
  document: DagDocument,
  edgeId: DagEdgeId,
): Result<DagDocument, Extract<DagEditProblem, { readonly kind: 'unknown-edge' }>> {
  if (!document.current.graph.edges.some((edge) => edge.id === edgeId)) return err({ kind: 'unknown-edge', edge: edgeId })
  const graph: EditableDag = {
    ...document.current.graph,
    edges: document.current.graph.edges.filter((edge) => edge.id !== edgeId),
  }
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: document.current.id,
    createdAt: new Date().toISOString(),
    graph,
    validation: validateStructure(graph),
  }
  return ok({ ...document, history: [...document.history, document.current], current })
}

export function describeDagCreateProblem(problem: DagCreateProblem): string {
  switch (problem.kind) {
    case 'empty-name': return 'Give this DAG a name.'
    case 'name-too-long': return `Keep the DAG name to ${problem.maximum} characters or fewer.`
    case 'prepared-column-missing': return 'A prepared variable is missing from the source profile.'
    case 'discovery-evidence-required': return 'Run at least one discovery method before choosing a discovery-informed origin.'
    default: return assertNever(problem)
  }
}

export function describeDagEditProblem(problem: DagEditProblem): string {
  switch (problem.kind) {
    case 'cause-required': return 'Choose the proposed direct cause.'
    case 'effect-required': return 'Choose the proposed direct effect.'
    case 'unknown-node': return 'That variable is not present in this DAG revision.'
    case 'self-edge': return 'A variable cannot directly cause itself in a contemporaneous DAG.'
    case 'duplicate-edge': return 'That directed edge already exists.'
    case 'directed-cycle': return 'That edge would create a directed cycle. Represent feedback using explicit time order or lags.'
    case 'empty-rationale': return 'Record why this direct causal relationship is credible.'
    case 'rationale-too-long': return `Keep the edge rationale to ${problem.maximum} characters or fewer.`
    case 'unknown-edge': return 'That edge is not present in the current revision.'
    default: return assertNever(problem)
  }
}

export function describeDagOrigin(origin: DagOrigin): string {
  switch (origin.kind) {
    case 'user-authored': return origin.basis === 'domain-knowledge'
      ? 'User-authored from domain and institutional knowledge'
      : 'User-authored from a known experimental assignment mechanism'
    case 'discovery-informed': return `User-authored after reviewing ${origin.reports.length} discovery run${origin.reports.length === 1 ? '' : 's'}`
    default: return assertNever(origin)
  }
}

export function nameOfDagNode(document: DagDocument, id: DagNodeId): string {
  return document.current.graph.nodes.find((node) => node.id === id)?.name ?? id
}
