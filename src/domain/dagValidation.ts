import { canonicalAdjustment, type DagAdjustmentAnalysis } from './dagFlow'
import { assertNever, isNonEmpty, type NonEmptyArray } from './dop'
import type {
  DagDocument,
  DagEdgeId,
  DagNodeId,
  DagStructuralIssue,
  DirectedDagEdge,
  EditableDag,
} from './dag'

export interface DagConditionalIndependenceImplication {
  readonly x: DagNodeId
  readonly y: DagNodeId
  readonly given: readonly DagNodeId[]
}

export type DagImplicationPlan =
  | {
      readonly kind: 'not-testable'
      readonly reason: 'no-observed-local-markov-implications' | 'all-implications-require-latent-parents'
      readonly skippedLatentImplications: number
    }
  | {
      readonly kind: 'requires-lag-aware-validation'
      readonly engine: 'tigramite-causal-effects'
    }
  | {
      readonly kind: 'test'
      readonly method: 'kernel-conditional-independence'
      readonly correction: 'holm'
      readonly significanceLevel: 0.05
      readonly maxObservationsPerTest: 500
      readonly skippedLatentImplications: number
      readonly implications: NonEmptyArray<DagConditionalIndependenceImplication>
    }

export type DagStudyBindingProblem =
  | { readonly kind: 'unknown-treatment'; readonly node: DagNodeId }
  | { readonly kind: 'unknown-outcome'; readonly node: DagNodeId }
  | { readonly kind: 'same-treatment-and-outcome'; readonly node: DagNodeId }
  | { readonly kind: 'no-causal-path'; readonly treatment: DagNodeId; readonly outcome: DagNodeId }

export type { DagAdjustmentAnalysis } from './dagFlow'

export interface DagStudyBindingAnalysis {
  readonly treatment: DagNodeId
  readonly outcome: DagNodeId
  readonly adjustment: DagAdjustmentAnalysis
  /** Lagged arrows the contemporaneous adjustment analysis leaves to CausalEffects. */
  readonly laggedArrows: number
}

const contemporaneousEdges = (graph: EditableDag): readonly DirectedDagEdge[] =>
  graph.edges.filter((edge) => edge.timing.kind === 'contemporaneous')

const reaches = (
  edges: readonly DirectedDagEdge[],
  start: DagNodeId,
  target: DagNodeId,
  direction: 'forward' | 'backward' = 'forward',
): boolean => {
  const pending: DagNodeId[] = [start]
  const visited = new Set<DagNodeId>()
  while (pending.length > 0) {
    const current = pending.pop()
    if (current === undefined || visited.has(current)) continue
    if (current === target) return true
    visited.add(current)
    for (const edge of edges) {
      const next = direction === 'forward'
        ? (edge.cause === current ? edge.effect : null)
        : (edge.effect === current ? edge.cause : null)
      if (next !== null) pending.push(next)
    }
  }
  return false
}

const reachable = (
  edges: readonly DirectedDagEdge[],
  start: DagNodeId,
  direction: 'forward' | 'backward',
): Set<DagNodeId> => {
  const result = new Set<DagNodeId>()
  const pending: DagNodeId[] = [start]
  while (pending.length > 0) {
    const current = pending.pop()
    if (current === undefined) continue
    for (const edge of edges) {
      const next = direction === 'forward'
        ? (edge.cause === current ? edge.effect : null)
        : (edge.effect === current ? edge.cause : null)
      if (next !== null && !result.has(next)) {
        result.add(next)
        pending.push(next)
      }
    }
  }
  return result
}

const implicationKey = (implication: DagConditionalIndependenceImplication): string => {
  const pair = [implication.x, implication.y].sort().join('\u001f')
  return `${pair}\u001e${[...implication.given].sort().join('\u001f')}`
}

/** Octopus's observed local-Markov planner, with temporal graphs routed to CausalEffects instead. */
export function planDagImplications(document: DagDocument): DagImplicationPlan {
  if (document.dataset.kind === 'time-series' && document.current.graph.edges.some((edge) => edge.timing.kind === 'lagged')) {
    return { kind: 'requires-lag-aware-validation', engine: 'tigramite-causal-effects' }
  }
  const graph = document.current.graph
  const edges = contemporaneousEdges(graph)
  const observed = new Set(graph.nodes.filter((node) => node.kind === 'observed').map((node) => node.id))
  const parents = new Map<DagNodeId, DagNodeId[]>(graph.nodes.map((node) => [node.id, []]))
  const children = new Map<DagNodeId, DagNodeId[]>(graph.nodes.map((node) => [node.id, []]))
  for (const edge of edges) {
    parents.get(edge.effect)?.push(edge.cause)
    children.get(edge.cause)?.push(edge.effect)
  }
  const descendantsOf = (node: DagNodeId): Set<DagNodeId> => {
    const descendants = new Set<DagNodeId>()
    const pending = [...(children.get(node) ?? [])]
    while (pending.length > 0) {
      const current = pending.pop()
      if (current === undefined || descendants.has(current)) continue
      descendants.add(current)
      pending.push(...(children.get(current) ?? []))
    }
    return descendants
  }
  const unique = new Map<string, DagConditionalIndependenceImplication>()
  let skippedLatentImplications = 0
  for (const x of observed) {
    const xParents = parents.get(x) ?? []
    const descendants = descendantsOf(x)
    const nonDescendants = [...observed].filter((candidate) =>
      candidate !== x && !descendants.has(candidate) && !xParents.includes(candidate))
    if (xParents.some((parent) => !observed.has(parent))) {
      skippedLatentImplications += nonDescendants.length
      continue
    }
    const given = [...xParents].sort()
    for (const y of nonDescendants) {
      const implication = { x, y, given }
      unique.set(implicationKey(implication), implication)
    }
  }
  const available = [...unique.values()].sort((left, right) => implicationKey(left).localeCompare(implicationKey(right)))
  if (!isNonEmpty(available)) {
    return {
      kind: 'not-testable',
      reason: skippedLatentImplications > 0
        ? 'all-implications-require-latent-parents'
        : 'no-observed-local-markov-implications',
      skippedLatentImplications,
    }
  }
  return {
    kind: 'test',
    method: 'kernel-conditional-independence',
    correction: 'holm',
    significanceLevel: 0.05,
    maxObservationsPerTest: 500,
    skippedLatentImplications,
    implications: available,
  }
}

export function inspectDagStudyBinding(
  document: DagDocument,
  treatment: DagNodeId,
  outcome: DagNodeId,
): { readonly ok: true; readonly value: DagStudyBindingAnalysis } | { readonly ok: false; readonly error: DagStudyBindingProblem } {
  const nodes = new Set(document.current.graph.nodes.map((node) => node.id))
  if (!nodes.has(treatment)) return { ok: false, error: { kind: 'unknown-treatment', node: treatment } }
  if (!nodes.has(outcome)) return { ok: false, error: { kind: 'unknown-outcome', node: outcome } }
  if (treatment === outcome) return { ok: false, error: { kind: 'same-treatment-and-outcome', node: treatment } }
  if (!reaches(document.current.graph.edges, treatment, outcome)) {
    return { ok: false, error: { kind: 'no-causal-path', treatment, outcome } }
  }
  return {
    ok: true,
    value: {
      treatment,
      outcome,
      adjustment: canonicalAdjustment(document.current.graph, treatment, outcome),
      laggedArrows: document.current.graph.edges.filter((edge) => edge.timing.kind === 'lagged').length,
    },
  }
}

export function affectedDagEdges(issue: DagStructuralIssue): readonly DagEdgeId[] {
  switch (issue.kind) {
    case 'no-edges':
    case 'duplicate-node': return []
    case 'unknown-endpoint':
    case 'self-edge':
    case 'duplicate-edge':
    case 'temporal-edge-on-cross-section':
    case 'invalid-lag':
    case 'lag-consumes-sample':
    case 'missing-rationale': return [issue.edge]
    case 'directed-cycle': return issue.edges
    default: return assertNever(issue)
  }
}

export function describeDagStructuralIssue(issue: DagStructuralIssue): string {
  switch (issue.kind) {
    case 'no-edges': return 'Add at least one arrow.'
    case 'duplicate-node': return 'Two variables share the same graph identity. Rename or remove one variable.'
    case 'unknown-endpoint': return 'An arrow refers to a variable outside this graph revision. Remove the arrow or restore the variable.'
    case 'self-edge': return 'A same-period variable cannot point to itself. Remove the arrow or add a lag.'
    case 'duplicate-edge': return 'The same arrow appears more than once. Remove the duplicate.'
    case 'directed-cycle': return 'Same-period arrows form a directed cycle. Use a lagged arrow to represent feedback.'
    case 'temporal-edge-on-cross-section': return 'Remove the lagged arrow or prepare the data as a time series.'
    case 'invalid-lag': return `Change lag ${issue.lag} to a positive integer.`
    case 'lag-consumes-sample': return `Lag ${issue.lag} leaves no usable rows from ${issue.observations} rows. Choose a smaller lag.`
    case 'missing-rationale': return 'An arrow has no rationale. Select it and record the supporting mechanism, design, or external evidence.'
    default: return assertNever(issue)
  }
}
