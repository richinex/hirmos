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
export type DiscoveryCandidateId = Brand<string, 'DiscoveryCandidateId'>

export type DagOrigin =
  | { readonly kind: 'user-authored'; readonly basis: 'domain-knowledge' | 'experimental-design' }
  | { readonly kind: 'discovery-informed'; readonly reports: NonEmptyArray<DiscoveryRunId> }

export interface ObservedDagNode {
  readonly kind: 'observed'
  readonly id: DagNodeId
  readonly column: ColumnId
  readonly name: string
}

export interface LatentDagNode {
  readonly kind: 'latent'
  readonly id: DagNodeId
  readonly name: string
}

export type DagNode = ObservedDagNode | LatentDagNode

export type EdgeTiming =
  | { readonly kind: 'contemporaneous' }
  | { readonly kind: 'lagged'; readonly lag: number }

export interface DiscoveryEdgeEvidenceRef {
  readonly kind: 'discovery'
  readonly run: DiscoveryRunId
  readonly candidate: DiscoveryCandidateId
  readonly semantics: 'endpoint-marked' | 'weighted-directed' | 'lagged-information'
}

/** An `unstated` support is an arrow drawn on the canvas whose justification is still to be recorded. */
export type EdgeSupport =
  | { readonly kind: 'user-assumption'; readonly rationale: EdgeRationale }
  | { readonly kind: 'experimental-design'; readonly rationale: EdgeRationale }
  | { readonly kind: 'unstated' }

export interface DirectedDagEdge {
  readonly kind: 'directed'
  readonly id: DagEdgeId
  readonly cause: DagNodeId
  readonly effect: DagNodeId
  readonly timing: EdgeTiming
  readonly support: EdgeSupport
  readonly evidence: readonly DiscoveryEdgeEvidenceRef[]
}

export interface EditableDag {
  readonly kind: 'editable-dag'
  readonly nodes: NonEmptyArray<DagNode>
  readonly edges: readonly DirectedDagEdge[]
}

export type DagStructuralIssue =
  | { readonly kind: 'no-edges' }
  | { readonly kind: 'duplicate-node'; readonly node: DagNodeId }
  | { readonly kind: 'unknown-endpoint'; readonly edge: DagEdgeId; readonly node: DagNodeId }
  | { readonly kind: 'self-edge'; readonly edge: DagEdgeId; readonly node: DagNodeId }
  | { readonly kind: 'duplicate-edge'; readonly edge: DagEdgeId }
  | { readonly kind: 'directed-cycle'; readonly edges: NonEmptyArray<DagEdgeId> }
  | { readonly kind: 'temporal-edge-on-cross-section'; readonly edge: DagEdgeId }
  | { readonly kind: 'invalid-lag'; readonly edge: DagEdgeId; readonly lag: number }
  | { readonly kind: 'lag-consumes-sample'; readonly edge: DagEdgeId; readonly lag: number; readonly observations: number }
  | { readonly kind: 'missing-rationale'; readonly edge: DagEdgeId }

export type DagStructuralValidation =
  | { readonly kind: 'incomplete'; readonly issues: NonEmptyArray<DagStructuralIssue> }
  | { readonly kind: 'invalid'; readonly issues: NonEmptyArray<DagStructuralIssue> }
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
  readonly dataset: {
    readonly kind: 'time-series' | 'panel' | 'cross-section'
    readonly observations: number
  }
  readonly origin: DagOrigin
  readonly history: readonly DagDraftRevision[]
  readonly future: readonly DagDraftRevision[]
  /** Append-only revision record, including branches no longer reachable through redo. */
  readonly audit: NonEmptyArray<DagDraftRevision>
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
  | { readonly kind: 'temporal-edge-on-cross-section' }
  | { readonly kind: 'invalid-lag'; readonly maximum: number }
  | { readonly kind: 'unknown-edge'; readonly edge: DagEdgeId }

export type DagHistoryProblem =
  | { readonly kind: 'nothing-to-undo' }
  | { readonly kind: 'nothing-to-redo' }

export type DagVariableEditProblem =
  | { readonly kind: 'empty-variable-name' }
  | { readonly kind: 'variable-name-too-long'; readonly maximum: number }
  | { readonly kind: 'duplicate-variable-name' }
  | { readonly kind: 'unknown-variable'; readonly node: DagNodeId }
  | { readonly kind: 'observed-variable-fixed' }
  | { readonly kind: 'latent-variable-has-edges'; readonly count: number }

const MAX_DAG_NAME = 100
const MAX_VARIABLE_NAME = 100
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
const newLatentDagNodeId = (document: DagDocumentId): DagNodeId =>
  brand<string, 'DagNodeId'>(`${document}:latent:${crypto.randomUUID()}`)
const dagNodeId = (prepared: PreparedDatasetVersionId, column: ColumnId): DagNodeId =>
  brand<string, 'DagNodeId'>(`${prepared}:${column}`)
const dagEdgeId = (cause: DagNodeId, effect: DagNodeId, timing: EdgeTiming): DagEdgeId =>
  brand<string, 'DagEdgeId'>(`${cause}->${effect}@${timing.kind === 'contemporaneous' ? 't' : `t-${timing.lag}`}`)

const cycleEdges = (graph: EditableDag): NonEmptyArray<DagEdgeId> | null => {
  const outgoing = new Map<DagNodeId, DirectedDagEdge[]>()
  for (const edge of graph.edges) {
    if (edge.timing.kind !== 'contemporaneous') continue
    outgoing.set(edge.cause, [...(outgoing.get(edge.cause) ?? []), edge])
  }
  const state = new Map<DagNodeId, 'visiting' | 'done'>()
  const trail: DirectedDagEdge[] = []
  let found: DirectedDagEdge[] | null = null
  const visit = (node: DagNodeId): void => {
    if (found !== null) return
    state.set(node, 'visiting')
    for (const edge of outgoing.get(node) ?? []) {
      const seen = state.get(edge.effect)
      if (seen === 'visiting') {
        const start = trail.findIndex((candidate) => candidate.cause === edge.effect)
        found = [...trail.slice(start < 0 ? 0 : start), edge]
        return
      }
      if (seen === undefined) {
        trail.push(edge)
        visit(edge.effect)
        trail.pop()
      }
    }
    state.set(node, 'done')
  }
  for (const node of graph.nodes) {
    if (state.has(node.id)) continue
    visit(node.id)
    if (found !== null) break
  }
  const cycle = found as DirectedDagEdge[] | null
  const ids: DagEdgeId[] = cycle === null ? [] : cycle.map((edge) => edge.id)
  return isNonEmpty(ids) ? ids : null
}

export const inspectDagStructure = (
  graph: EditableDag,
  dataset: DagDocument['dataset'],
): DagStructuralValidation => {
  if (graph.edges.length === 0) return { kind: 'incomplete', issues: [{ kind: 'no-edges' }] }
  const issues: DagStructuralIssue[] = []
  const nodes = new Set<DagNodeId>()
  for (const node of graph.nodes) {
    if (nodes.has(node.id)) issues.push({ kind: 'duplicate-node', node: node.id })
    nodes.add(node.id)
  }
  const edges = new Set<string>()
  for (const edge of graph.edges) {
    if (!nodes.has(edge.cause)) issues.push({ kind: 'unknown-endpoint', edge: edge.id, node: edge.cause })
    if (!nodes.has(edge.effect)) issues.push({ kind: 'unknown-endpoint', edge: edge.id, node: edge.effect })
    if (edge.timing.kind === 'contemporaneous' && edge.cause === edge.effect) {
      issues.push({ kind: 'self-edge', edge: edge.id, node: edge.cause })
    }
    const key = `${edge.cause}\u0000${edge.effect}\u0000${edge.timing.kind === 'contemporaneous' ? 0 : edge.timing.lag}`
    if (edges.has(key)) issues.push({ kind: 'duplicate-edge', edge: edge.id })
    edges.add(key)
    if (edge.timing.kind === 'lagged') {
      if (dataset.kind === 'cross-section') issues.push({ kind: 'temporal-edge-on-cross-section', edge: edge.id })
      if (!Number.isSafeInteger(edge.timing.lag) || edge.timing.lag < 1) {
        issues.push({ kind: 'invalid-lag', edge: edge.id, lag: edge.timing.lag })
      } else if (edge.timing.lag >= dataset.observations) {
        issues.push({ kind: 'lag-consumes-sample', edge: edge.id, lag: edge.timing.lag, observations: dataset.observations })
      }
    }
  }
  const cycle = cycleEdges(graph)
  if (cycle !== null) issues.push({ kind: 'directed-cycle', edges: cycle })
  if (isNonEmpty(issues)) return { kind: 'invalid', issues }
  const unstated: DagStructuralIssue[] = graph.edges
    .filter((edge) => edge.support.kind === 'unstated')
    .map((edge) => ({ kind: 'missing-rationale', edge: edge.id }))
  return isNonEmpty(unstated) ? { kind: 'incomplete', issues: unstated } : { kind: 'structurally-valid' }
}

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
  const dataset: DagDocument['dataset'] = {
    kind: prepared.kind === 'prepared-time-series' ? 'time-series' : prepared.kind === 'prepared-panel' ? 'panel' : 'cross-section',
    observations: prepared.observations,
  }
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: null,
    createdAt: new Date().toISOString(),
    graph,
    validation: inspectDagStructure(graph, dataset),
  }
  return ok({
    kind: 'dag-document',
    id: newDagDocumentId(),
    name: name.value,
    preparedDataset: prepared.id,
    dataset,
    origin: origin.value,
    history: [],
    future: [],
    audit: [current],
    current,
  })
}

const hasContemporaneousPath = (graph: EditableDag, start: DagNodeId, target: DagNodeId): boolean => {
  const pending: DagNodeId[] = [start]
  const visited = new Set<DagNodeId>()
  while (pending.length > 0) {
    const node = pending.pop()
    if (node === undefined || visited.has(node)) continue
    if (node === target) return true
    visited.add(node)
    for (const edge of graph.edges) {
      if (edge.timing.kind === 'contemporaneous' && edge.cause === node) pending.push(edge.effect)
    }
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

/** A null rationale is a canvas-drawn arrow; the justification is recorded later on the selected edge. */
const supportFrom = (
  origin: DagOrigin,
  rawRationale: string | null,
): Result<EdgeSupport, Extract<DagEditProblem, { readonly kind: 'empty-rationale' | 'rationale-too-long' }>> => {
  if (rawRationale === null) return ok({ kind: 'unstated' })
  const rationale = edgeRationale(rawRationale)
  return rationale.ok ? ok(supportFor(origin, rationale.value)) : rationale
}

const sameTiming = (left: EdgeTiming, right: EdgeTiming): boolean =>
  left.kind === right.kind && (left.kind === 'contemporaneous' || (right.kind === 'lagged' && left.lag === right.lag))

const appendRevision = (document: DagDocument, graph: EditableDag): DagDocument => {
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: document.current.id,
    createdAt: new Date().toISOString(),
    graph,
    validation: inspectDagStructure(graph, document.dataset),
  }
  return {
    ...document,
    history: [...document.history, document.current],
    future: [],
    audit: [...document.audit, current],
    current,
  }
}

/** Check whether a directed edge could join the current revision, without recording a rationale. */
export function inspectDagEdgeAddition(
  document: DagDocument,
  cause: DagNodeId | null,
  effect: DagNodeId | null,
  timing: EdgeTiming = { kind: 'contemporaneous' },
): Result<{ readonly cause: DagNodeId; readonly effect: DagNodeId }, DagEditProblem> {
  if (cause === null) return err({ kind: 'cause-required' })
  if (effect === null) return err({ kind: 'effect-required' })
  if (!document.current.graph.nodes.some((node) => node.id === cause)) return err({ kind: 'unknown-node', node: cause })
  if (!document.current.graph.nodes.some((node) => node.id === effect)) return err({ kind: 'unknown-node', node: effect })
  if (timing.kind === 'contemporaneous' && cause === effect) return err({ kind: 'self-edge' })
  if (timing.kind === 'lagged') {
    if (document.dataset.kind === 'cross-section') return err({ kind: 'temporal-edge-on-cross-section' })
    if (!Number.isSafeInteger(timing.lag) || timing.lag < 1 || timing.lag >= document.dataset.observations) {
      return err({ kind: 'invalid-lag', maximum: Math.max(1, document.dataset.observations - 1) })
    }
  }
  if (document.current.graph.edges.some((edge) => edge.cause === cause && edge.effect === effect && sameTiming(edge.timing, timing))) {
    return err({ kind: 'duplicate-edge' })
  }
  if (timing.kind === 'contemporaneous' && hasContemporaneousPath(document.current.graph, effect, cause)) {
    return err({ kind: 'directed-cycle' })
  }
  return ok({ cause, effect })
}

export function reviseDagWithEdge(
  document: DagDocument,
  rawCause: DagNodeId | null,
  rawEffect: DagNodeId | null,
  rawRationale: string | null,
  timing: EdgeTiming = { kind: 'contemporaneous' },
  evidence: readonly DiscoveryEdgeEvidenceRef[] = [],
): Result<DagDocument, DagEditProblem> {
  const endpoints = inspectDagEdgeAddition(document, rawCause, rawEffect, timing)
  if (!endpoints.ok) return endpoints
  const { cause, effect } = endpoints.value
  const support = supportFrom(document.origin, rawRationale)
  if (!support.ok) return support

  const edge: DirectedDagEdge = {
    kind: 'directed',
    id: dagEdgeId(cause, effect, timing),
    cause,
    effect,
    timing,
    support: support.value,
    evidence,
  }
  return ok(appendRevision(document, { ...document.current.graph, edges: [...document.current.graph.edges, edge] }))
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
    validation: inspectDagStructure(graph, document.dataset),
  }
  return ok({
    ...document,
    history: [...document.history, document.current],
    future: [],
    audit: [...document.audit, current],
    current,
  })
}

/** Reconnect one endpoint as a single immutable revision. The original rationale is not reused. */
export function inspectDagEdgeReplacement(
  document: DagDocument,
  edgeId: DagEdgeId,
  cause: DagNodeId,
  effect: DagNodeId,
  timing: EdgeTiming,
): Result<true, DagEditProblem> {
  const original = document.current.graph.edges.find((edge) => edge.id === edgeId)
  if (original === undefined) return err({ kind: 'unknown-edge', edge: edgeId })
  if (!document.current.graph.nodes.some((node) => node.id === cause)) return err({ kind: 'unknown-node', node: cause })
  if (!document.current.graph.nodes.some((node) => node.id === effect)) return err({ kind: 'unknown-node', node: effect })
  if (timing.kind === 'contemporaneous' && cause === effect) return err({ kind: 'self-edge' })
  if (timing.kind === 'lagged') {
    if (document.dataset.kind === 'cross-section') return err({ kind: 'temporal-edge-on-cross-section' })
    if (!Number.isSafeInteger(timing.lag) || timing.lag < 1 || timing.lag >= document.dataset.observations) {
      return err({ kind: 'invalid-lag', maximum: Math.max(1, document.dataset.observations - 1) })
    }
  }
  const retained = document.current.graph.edges.filter((edge) => edge.id !== edgeId)
  if (retained.some((edge) => edge.cause === cause && edge.effect === effect
    && edge.timing.kind === timing.kind
    && (edge.timing.kind === 'contemporaneous' || (timing.kind === 'lagged' && edge.timing.lag === timing.lag)))) {
    return err({ kind: 'duplicate-edge' })
  }
  const withoutOriginal: EditableDag = { ...document.current.graph, edges: retained }
  if (timing.kind === 'contemporaneous' && hasContemporaneousPath(withoutOriginal, effect, cause)) {
    return err({ kind: 'directed-cycle' })
  }
  return ok(true)
}

/** Reconnect one endpoint as a single immutable revision. The original rationale is not reused. */
export function reviseDagByReplacingEdge(
  document: DagDocument,
  edgeId: DagEdgeId,
  cause: DagNodeId,
  effect: DagNodeId,
  rawRationale: string | null,
  timing: EdgeTiming,
): Result<DagDocument, DagEditProblem> {
  const inspected = inspectDagEdgeReplacement(document, edgeId, cause, effect, timing)
  if (!inspected.ok) return inspected
  const retained = document.current.graph.edges.filter((edge) => edge.id !== edgeId)
  const support = supportFrom(document.origin, rawRationale)
  if (!support.ok) return support
  const edge: DirectedDagEdge = {
    kind: 'directed',
    id: dagEdgeId(cause, effect, timing),
    cause,
    effect,
    timing,
    support: support.value,
    evidence: [],
  }
  return ok(appendRevision(document, { ...document.current.graph, edges: [...retained, edge] }))
}

/** Record an edge's rationale and timing as one revision; a changed timing gives the edge a new identity. */
export function reviseDagEdgeDetails(
  document: DagDocument,
  edgeId: DagEdgeId,
  rawRationale: string,
  timing: EdgeTiming,
): Result<DagDocument, DagEditProblem> {
  const original = document.current.graph.edges.find((edge) => edge.id === edgeId)
  if (original === undefined) return err({ kind: 'unknown-edge', edge: edgeId })
  if (!sameTiming(original.timing, timing)) {
    const inspected = inspectDagEdgeReplacement(document, edgeId, original.cause, original.effect, timing)
    if (!inspected.ok) return inspected
  }
  const support = supportFrom(document.origin, rawRationale)
  if (!support.ok) return support
  const edge: DirectedDagEdge = {
    ...original,
    id: dagEdgeId(original.cause, original.effect, timing),
    timing,
    support: support.value,
  }
  const edges = document.current.graph.edges.map((candidate) => candidate.id === edgeId ? edge : candidate)
  return ok(appendRevision(document, { ...document.current.graph, edges }))
}

/**
 * Replace a direct edge with an unmeasured common cause of both endpoints, as one revision: the
 * Octopus "confound" operation. The new latent node and its two arrows carry unstated support.
 */
export function reviseDagWithLatentConfounder(
  document: DagDocument,
  edgeId: DagEdgeId,
): Result<DagDocument, DagEditProblem | DagVariableEditProblem> {
  const edge = document.current.graph.edges.find((candidate) => candidate.id === edgeId)
  if (edge === undefined) return err({ kind: 'unknown-edge', edge: edgeId })
  const existing = new Set(document.current.graph.nodes.map((node) => node.name))
  let index = document.current.graph.nodes.filter((node) => node.kind === 'latent').length + 1
  while (existing.has(`Unmeasured cause ${index}`)) index += 1
  const latent: LatentDagNode = {
    kind: 'latent',
    id: newLatentDagNodeId(document.id),
    name: `Unmeasured cause ${index}`,
  }
  const arrow = (effect: DagNodeId): DirectedDagEdge => ({
    kind: 'directed',
    id: dagEdgeId(latent.id, effect, { kind: 'contemporaneous' }),
    cause: latent.id,
    effect,
    timing: { kind: 'contemporaneous' },
    support: { kind: 'unstated' },
    evidence: [],
  })
  const graph: EditableDag = {
    ...document.current.graph,
    nodes: [...document.current.graph.nodes, latent],
    edges: [...document.current.graph.edges.filter((candidate) => candidate.id !== edgeId), arrow(edge.cause), arrow(edge.effect)],
  }
  return ok(appendRevision(document, graph))
}

/** Move the active revision pointer backward without deleting any revision from the audit. */
export function undoDagRevision(document: DagDocument): Result<DagDocument, Extract<DagHistoryProblem, { readonly kind: 'nothing-to-undo' }>> {
  const previous = document.history.at(-1)
  if (previous === undefined) return err({ kind: 'nothing-to-undo' })
  return ok({
    ...document,
    history: document.history.slice(0, -1),
    current: previous,
    future: [document.current, ...document.future],
  })
}

/** Move the active revision pointer forward along the current branch. */
export function redoDagRevision(document: DagDocument): Result<DagDocument, Extract<DagHistoryProblem, { readonly kind: 'nothing-to-redo' }>> {
  const [next, ...future] = document.future
  if (next === undefined) return err({ kind: 'nothing-to-redo' })
  return ok({
    ...document,
    history: [...document.history, document.current],
    current: next,
    future,
  })
}

const latentVariableName = (
  document: DagDocument,
  rawName: string,
  except: DagNodeId | null = null,
): Result<string, Extract<DagVariableEditProblem, { readonly kind: 'empty-variable-name' | 'variable-name-too-long' | 'duplicate-variable-name' }>> => {
  const name = rawName.trim()
  if (name.length === 0) return err({ kind: 'empty-variable-name' })
  if (name.length > MAX_VARIABLE_NAME) return err({ kind: 'variable-name-too-long', maximum: MAX_VARIABLE_NAME })
  if (document.current.graph.nodes.some((node) => node.id !== except && node.name.trim().toLocaleLowerCase() === name.toLocaleLowerCase())) {
    return err({ kind: 'duplicate-variable-name' })
  }
  return ok(name)
}

const reviseDagGraph = (document: DagDocument, graph: EditableDag): DagDocument => {
  const current: DagDraftRevision = {
    kind: 'draft',
    id: newDagRevisionId(),
    parent: document.current.id,
    createdAt: new Date().toISOString(),
    graph,
    validation: inspectDagStructure(graph, document.dataset),
  }
  return {
    ...document,
    history: [...document.history, document.current],
    future: [],
    audit: [...document.audit, current],
    current,
  }
}

export function reviseDagWithLatentNode(
  document: DagDocument,
  rawName: string,
): Result<DagDocument, DagVariableEditProblem> {
  const name = latentVariableName(document, rawName)
  if (!name.ok) return name
  const node: LatentDagNode = {
    kind: 'latent',
    id: newLatentDagNodeId(document.id),
    name: name.value,
  }
  const graph: EditableDag = {
    ...document.current.graph,
    nodes: [...document.current.graph.nodes, node],
  }
  return ok(reviseDagGraph(document, graph))
}

export function reviseDagRenamingLatentNode(
  document: DagDocument,
  nodeId: DagNodeId,
  rawName: string,
): Result<DagDocument, DagVariableEditProblem> {
  const node = document.current.graph.nodes.find((candidate) => candidate.id === nodeId)
  if (node === undefined) return err({ kind: 'unknown-variable', node: nodeId })
  if (node.kind === 'observed') return err({ kind: 'observed-variable-fixed' })
  const name = latentVariableName(document, rawName, nodeId)
  if (!name.ok) return name
  const nodes = document.current.graph.nodes.map((candidate): DagNode => candidate.id === nodeId
    ? { ...candidate, name: name.value }
    : candidate)
  if (!isNonEmpty(nodes)) return err({ kind: 'unknown-variable', node: nodeId })
  return ok(reviseDagGraph(document, { ...document.current.graph, nodes }))
}

export function reviseDagWithoutLatentNode(
  document: DagDocument,
  nodeId: DagNodeId,
): Result<DagDocument, DagVariableEditProblem> {
  const node = document.current.graph.nodes.find((candidate) => candidate.id === nodeId)
  if (node === undefined) return err({ kind: 'unknown-variable', node: nodeId })
  if (node.kind === 'observed') return err({ kind: 'observed-variable-fixed' })
  const incidentEdges = document.current.graph.edges.filter((edge) => edge.cause === nodeId || edge.effect === nodeId)
  if (incidentEdges.length > 0) return err({ kind: 'latent-variable-has-edges', count: incidentEdges.length })
  const nodes = document.current.graph.nodes.filter((candidate) => candidate.id !== nodeId)
  if (!isNonEmpty(nodes)) return err({ kind: 'unknown-variable', node: nodeId })
  return ok(reviseDagGraph(document, { ...document.current.graph, nodes }))
}

export function describeDagHistoryProblem(problem: DagHistoryProblem): string {
  switch (problem.kind) {
    case 'nothing-to-undo': return 'This DAG has no earlier active revision.'
    case 'nothing-to-redo': return 'This DAG has no later revision on the current branch.'
    default: return assertNever(problem)
  }
}

export function describeDagVariableEditProblem(problem: DagVariableEditProblem): string {
  switch (problem.kind) {
    case 'empty-variable-name': return 'Give the unmeasured variable a name.'
    case 'variable-name-too-long': return `Keep the variable name to ${problem.maximum} characters or fewer.`
    case 'duplicate-variable-name': return 'A variable with that name already exists in this DAG.'
    case 'unknown-variable': return 'That variable is not present in the active DAG revision.'
    case 'observed-variable-fixed': return 'Change observed variables in Data studio. They cannot be removed from the DAG.'
    case 'latent-variable-has-edges': return `Remove the ${problem.count} connected arrow${problem.count === 1 ? '' : 's'} before deleting this unmeasured variable.`
    default: return assertNever(problem)
  }
}

export function describeDagCreateProblem(problem: DagCreateProblem): string {
  switch (problem.kind) {
    case 'empty-name': return 'Give this DAG a name.'
    case 'name-too-long': return `Keep the DAG name to ${problem.maximum} characters or fewer.`
    case 'prepared-column-missing': return 'A prepared variable is missing from the source profile. Create another prepared dataset version.'
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
    case 'duplicate-edge': return 'That arrow already exists.'
    case 'directed-cycle': return 'That arrow would create a directed cycle. Represent feedback with explicit time order or lags.'
    case 'empty-rationale': return 'Record the rationale for this arrow.'
    case 'rationale-too-long': return `Keep the arrow rationale to ${problem.maximum} characters or fewer.`
    case 'temporal-edge-on-cross-section': return 'Lagged arrows need a prepared time series. This dataset holds independent rows.'
    case 'invalid-lag': return `Choose a positive lag no greater than ${problem.maximum}.`
    case 'unknown-edge': return 'That arrow is not present in the current revision. Select another arrow.'
    default: return assertNever(problem)
  }
}

export const describeDagBasis = (basis: 'domain-knowledge' | 'experimental-design'): string =>
  basis === 'domain-knowledge' ? 'substantive and institutional knowledge' : 'experimental assignment mechanism'

export function describeDagOrigin(origin: DagOrigin): string {
  switch (origin.kind) {
    case 'user-authored': return `Basis: ${describeDagBasis(origin.basis)}`
    case 'discovery-informed': return `Basis: substantive review of ${origin.reports.length} discovery result${origin.reports.length === 1 ? '' : 's'}`
    default: return assertNever(origin)
  }
}

export function describeDagValidation(kind: DagStructuralValidation['kind']): string {
  switch (kind) {
    case 'structurally-valid': return 'structurally valid'
    case 'incomplete': return 'incomplete'
    case 'invalid': return 'invalid'
    default: return assertNever(kind)
  }
}

export function nameOfDagNode(document: DagDocument, id: DagNodeId): string {
  return document.current.graph.nodes.find((node) => node.id === id)?.name ?? id
}
