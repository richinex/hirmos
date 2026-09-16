import type { DagEdgeId, DagNodeId, DirectedDagEdge, EditableDag } from './dag'
import { assertNever } from './dop'

/**
 * What a bound treatment and outcome make of a DAG: which arrows carry the effect and which leak
 * bias (dagitty's overlays, as Octopus draws them), the role every variable plays relative to the
 * pair, every treatment-to-outcome path with its open-or-closed status, and the canonical
 * observed adjustment set. Roles are derived from paths, never stored.
 */

export interface PathEdge { readonly from: string; readonly to: string }

export type PathType = 'causal' | 'backdoor' | 'closed'

export interface XyPath {
  readonly nodes: readonly string[]
  readonly type: PathType
  readonly colliders: readonly string[]
}

const PATH_CAP = 64

export const classifyPath = (nodes: readonly string[], dirs: readonly ('f' | 'b')[]): XyPath => {
  const colliders = nodes.filter((_, index) => index > 0 && index < nodes.length - 1 && dirs[index - 1] === 'f' && dirs[index] === 'b')
  const type: PathType = colliders.length > 0 ? 'closed' : dirs.every((direction) => direction === 'f') ? 'causal' : 'backdoor'
  return { nodes, type, colliders }
}

/** Every simple path between X and Y, ignoring arrow direction, typed by Pearl's rule; capped so a dense graph stays responsive. */
export const enumeratePaths = (edges: readonly PathEdge[], X: string, Y: string): XyPath[] => {
  const paths: XyPath[] = []
  const walk = (node: string, visited: Set<string>, trail: string[], dirs: ('f' | 'b')[]): void => {
    if (paths.length >= PATH_CAP) return
    if (node === Y) { paths.push(classifyPath([...trail], [...dirs])); return }
    for (const edge of edges) {
      const next = edge.from === node && !visited.has(edge.to) ? { id: edge.to, dir: 'f' as const }
        : edge.to === node && !visited.has(edge.from) ? { id: edge.from, dir: 'b' as const } : null
      if (!next) continue
      visited.add(next.id); trail.push(next.id); dirs.push(next.dir)
      walk(next.id, visited, trail, dirs)
      visited.delete(next.id); trail.pop(); dirs.pop()
    }
  }
  walk(X, new Set([X]), [X], [])
  paths.sort((a, b) => a.nodes.length - b.nodes.length || a.nodes.join('').localeCompare(b.nodes.join('')))
  return paths
}

export type DagCausalRole =
  | { readonly kind: 'treatment' }
  | { readonly kind: 'outcome' }
  /** Common ancestor of treatment and outcome on an open back-door path. */
  | { readonly kind: 'confounder'; readonly paths: number }
  /** Other interior non-collider of an open back-door path. */
  | { readonly kind: 'backdoor-variable'; readonly paths: number }
  /** Interior of a causal path; sometimes also a collider on another path. */
  | { readonly kind: 'mediator'; readonly alsoCollider: boolean }
  /** Collision point of a closed path. */
  | { readonly kind: 'collider'; readonly paths: number }
  /** Descendant of the treatment off every causal path. */
  | { readonly kind: 'post-treatment' }
  /** Ancestor of the outcome only. */
  | { readonly kind: 'outcome-predictor' }
  /** Ancestor of the treatment only. */
  | { readonly kind: 'pre-treatment' }
  | { readonly kind: 'unmeasured' }
  | { readonly kind: 'unrelated' }

export interface DagEdgeFlow {
  readonly causal: boolean
  readonly biasing: boolean
}

export type DagPathStatus =
  | { readonly kind: 'causal' }
  | { readonly kind: 'open' }
  | { readonly kind: 'closed-by-adjustment'; readonly by: readonly DagNodeId[] }
  | { readonly kind: 'closed-at-collider'; readonly colliders: readonly DagNodeId[] }
  /** Open because the only variables that could close it are unmeasured. */
  | { readonly kind: 'open-through-unmeasured'; readonly unmeasured: readonly DagNodeId[] }

export interface DagPathFact {
  readonly nodes: readonly DagNodeId[]
  readonly type: PathType
  readonly status: DagPathStatus
}

export type DagAdjustmentAnalysis =
  | { readonly kind: 'unnecessary' }
  | { readonly kind: 'sufficient'; readonly variables: readonly DagNodeId[] }
  | { readonly kind: 'none' }

export interface DagCausalFlow {
  readonly treatment: DagNodeId
  readonly outcome: DagNodeId
  readonly edges: ReadonlyMap<DagEdgeId, DagEdgeFlow>
  readonly roles: ReadonlyMap<DagNodeId, DagCausalRole>
  readonly paths: readonly DagPathFact[]
  readonly adjustment: DagAdjustmentAnalysis
  /** Lagged arrows left out of the contemporaneous analysis. */
  readonly laggedArrows: number
  readonly pathCapReached: boolean
}

const contemporaneous = (graph: EditableDag): readonly DirectedDagEdge[] =>
  graph.edges.filter((edge) => edge.timing.kind === 'contemporaneous' && edge.cause !== edge.effect)

const reachable = (edges: readonly DirectedDagEdge[], start: DagNodeId, direction: 'forward' | 'backward'): Set<DagNodeId> => {
  const out = new Set<DagNodeId>()
  const frontier = [start]
  while (frontier.length > 0) {
    const current = frontier.pop()
    if (current === undefined) continue
    for (const edge of edges) {
      const next = direction === 'forward' ? (edge.cause === current ? edge.effect : null) : (edge.effect === current ? edge.cause : null)
      if (next !== null && !out.has(next)) { out.add(next); frontier.push(next) }
    }
  }
  return out
}

type WalkMode = 'head' | 'tail'
const stateKey = (node: DagNodeId, mode: WalkMode): string => `${node}\u0000${mode}`

const walkTransitions = (
  edges: readonly DirectedDagEdge[],
  node: DagNodeId,
  mode: WalkMode,
  conditioned: ReadonlySet<DagNodeId>,
  opensCollider: ReadonlySet<DagNodeId>,
): readonly { readonly edge: DirectedDagEdge; readonly node: DagNodeId; readonly mode: WalkMode }[] => {
  const moves: { edge: DirectedDagEdge; node: DagNodeId; mode: WalkMode }[] = []
  for (const edge of edges) {
    if (edge.cause === node && !conditioned.has(node)) moves.push({ edge, node: edge.effect, mode: 'head' })
    if (edge.effect === node) {
      const legal = mode === 'tail' ? !conditioned.has(node) : opensCollider.has(node)
      if (legal) moves.push({ edge, node: edge.cause, mode: 'tail' })
    }
  }
  return moves
}

/** Every edge on an open back-door walk from the treatment to the outcome given the conditioning set. */
export const backdoorEdges = (
  edges: readonly DirectedDagEdge[],
  treatment: DagNodeId,
  outcome: DagNodeId,
  conditioned: ReadonlySet<DagNodeId>,
): Set<DagEdgeId> => {
  const opensCollider = new Set<DagNodeId>(conditioned)
  for (const z of conditioned) for (const ancestor of reachable(edges, z, 'backward')) opensCollider.add(ancestor)
  // A simple back-door path never revisits the treatment, so a walk may not re-enter it; otherwise a
  // walk could leave through the back door and return through the front, marking causal edges as biasing.
  const moves = (node: DagNodeId, mode: WalkMode) =>
    walkTransitions(edges, node, mode, conditioned, opensCollider).filter((move) => move.node !== treatment)

  const forward = new Set<string>()
  const seed = edges.filter((edge) => edge.effect === treatment).map((edge) => ({ node: edge.cause, mode: 'tail' as const, via: edge }))
  const frontier: { node: DagNodeId; mode: WalkMode }[] = seed.map((entry) => ({ node: entry.node, mode: entry.mode }))
  for (const entry of seed) forward.add(stateKey(entry.node, entry.mode))
  while (frontier.length > 0) {
    const current = frontier.pop()
    if (current === undefined || current.node === outcome) continue
    for (const move of moves(current.node, current.mode)) {
      const key = stateKey(move.node, move.mode)
      if (!forward.has(key)) { forward.add(key); frontier.push({ node: move.node, mode: move.mode }) }
    }
  }

  const canFinish = new Set<string>()
  const nodes = new Set<DagNodeId>()
  for (const edge of edges) { nodes.add(edge.cause); nodes.add(edge.effect) }
  const allStates = [...nodes].flatMap((node) => [{ node, mode: 'head' as const }, { node, mode: 'tail' as const }])
  let grew = true
  while (grew) {
    grew = false
    for (const state of allStates) {
      const key = stateKey(state.node, state.mode)
      if (canFinish.has(key)) continue
      const done = state.node === outcome
        || moves(state.node, state.mode).some((move) => move.node === outcome || canFinish.has(stateKey(move.node, move.mode)))
      if (done) { canFinish.add(key); grew = true }
    }
  }

  const marked = new Set<DagEdgeId>()
  for (const entry of seed) {
    if (entry.node === outcome || canFinish.has(stateKey(entry.node, entry.mode))) marked.add(entry.via.id)
  }
  for (const key of forward) {
    const [node, mode] = key.split('\u0000') as [DagNodeId, WalkMode]
    if (node === outcome) continue
    for (const move of moves(node, mode)) {
      if (move.node === outcome || canFinish.has(stateKey(move.node, move.mode))) marked.add(move.edge.id)
    }
  }
  return marked
}

const backdoorOpen = (edges: readonly DirectedDagEdge[], treatment: DagNodeId, outcome: DagNodeId, conditioned: ReadonlySet<DagNodeId>): boolean =>
  backdoorEdges(edges, treatment, outcome, conditioned).size > 0

/**
 * The canonical adjustment set: ancestors of the treatment or the outcome, minus both endpoints,
 * minus every node on or descended from a proper causal path, minus unmeasured nodes. Sufficient
 * whenever any observed set is (Perković and others, 2018); latent confounding shows as no valid set.
 */
export function canonicalAdjustment(graph: EditableDag, treatment: DagNodeId, outcome: DagNodeId): DagAdjustmentAnalysis {
  const edges = contemporaneous(graph)
  if (!backdoorOpen(edges, treatment, outcome, new Set())) return { kind: 'unnecessary' }
  const descendantsOfTreatment = reachable(edges, treatment, 'forward')
  const ancestorsOfOutcome = reachable(edges, outcome, 'backward')
  const onCausalPath = new Set<DagNodeId>()
  for (const node of descendantsOfTreatment) if (node === outcome || ancestorsOfOutcome.has(node)) onCausalPath.add(node)
  const forbidden = new Set<DagNodeId>([treatment, outcome])
  for (const node of onCausalPath) {
    forbidden.add(node)
    for (const descendant of reachable(edges, node, 'forward')) forbidden.add(descendant)
  }
  const latent = new Set(graph.nodes.filter((node) => node.kind === 'latent').map((node) => node.id))
  const candidates = new Set<DagNodeId>()
  for (const node of [...reachable(edges, treatment, 'backward'), ...ancestorsOfOutcome]) {
    if (!forbidden.has(node) && !latent.has(node)) candidates.add(node)
  }
  const backdoorGraph = edges.filter((edge) => !(edge.cause === treatment && onCausalPath.has(edge.effect)))
  return backdoorOpen(backdoorGraph, treatment, outcome, candidates)
    ? { kind: 'none' }
    : { kind: 'sufficient', variables: [...candidates].sort() }
}

export function analyseDagCausalFlow(graph: EditableDag, treatment: DagNodeId, outcome: DagNodeId): DagCausalFlow {
  const edges = contemporaneous(graph)
  const laggedArrows = graph.edges.length - edges.length - graph.edges.filter((edge) => edge.cause === edge.effect && edge.timing.kind === 'contemporaneous').length
  const latent = new Set(graph.nodes.filter((node) => node.kind === 'latent').map((node) => node.id))
  const descendantsOfTreatment = reachable(edges, treatment, 'forward')
  const ancestorsOfOutcome = reachable(edges, outcome, 'backward')
  const ancestorsOfTreatment = reachable(edges, treatment, 'backward')
  const biasing = backdoorEdges(edges, treatment, outcome, new Set())

  const flows = new Map<DagEdgeId, DagEdgeFlow>()
  for (const edge of edges) {
    const causal = (edge.cause === treatment || descendantsOfTreatment.has(edge.cause)) && (edge.effect === outcome || ancestorsOfOutcome.has(edge.effect))
    flows.set(edge.id, { causal, biasing: biasing.has(edge.id) })
  }

  const adjustment = canonicalAdjustment(graph, treatment, outcome)
  const adjusted = new Set<DagNodeId>(adjustment.kind === 'sufficient' ? adjustment.variables : [])
  const raw = enumeratePaths(edges.map((edge) => ({ from: edge.cause, to: edge.effect })), treatment, outcome)
  const paths: DagPathFact[] = raw.map((path) => {
    const nodes = path.nodes as readonly DagNodeId[]
    const interior = nodes.slice(1, -1)
    if (path.type === 'causal') return { nodes, type: path.type, status: { kind: 'causal' } }
    if (path.type === 'closed') return { nodes, type: path.type, status: { kind: 'closed-at-collider', colliders: path.colliders as readonly DagNodeId[] } }
    const closers = interior.filter((node) => adjusted.has(node))
    if (closers.length > 0) return { nodes, type: path.type, status: { kind: 'closed-by-adjustment', by: closers } }
    const unmeasured = interior.filter((node) => latent.has(node))
    return { nodes, type: path.type, status: unmeasured.length > 0 ? { kind: 'open-through-unmeasured', unmeasured } : { kind: 'open' } }
  })

  const roles = new Map<DagNodeId, DagCausalRole>()
  const onCausalPath = new Set<DagNodeId>()
  const colliderCounts = new Map<DagNodeId, number>()
  const backdoorCounts = new Map<DagNodeId, number>()
  for (const path of paths) {
    const interior = path.nodes.slice(1, -1)
    if (path.type === 'causal') for (const node of interior) onCausalPath.add(node)
    if (path.status.kind === 'closed-at-collider') for (const node of path.status.colliders) colliderCounts.set(node, (colliderCounts.get(node) ?? 0) + 1)
    if (path.type === 'backdoor') for (const node of interior) backdoorCounts.set(node, (backdoorCounts.get(node) ?? 0) + 1)
  }
  for (const node of graph.nodes) {
    const id = node.id
    let role: DagCausalRole
    if (id === treatment) role = { kind: 'treatment' }
    else if (id === outcome) role = { kind: 'outcome' }
    else if (onCausalPath.has(id)) role = { kind: 'mediator', alsoCollider: colliderCounts.has(id) }
    else if (latent.has(id)) role = { kind: 'unmeasured' }
    else if (backdoorCounts.has(id) && ancestorsOfTreatment.has(id) && ancestorsOfOutcome.has(id)) role = { kind: 'confounder', paths: backdoorCounts.get(id) ?? 0 }
    else if (backdoorCounts.has(id)) role = { kind: 'backdoor-variable', paths: backdoorCounts.get(id) ?? 0 }
    else if (colliderCounts.has(id)) role = { kind: 'collider', paths: colliderCounts.get(id) ?? 0 }
    else if (descendantsOfTreatment.has(id)) role = { kind: 'post-treatment' }
    else if (ancestorsOfOutcome.has(id)) role = { kind: 'outcome-predictor' }
    else if (ancestorsOfTreatment.has(id)) role = { kind: 'pre-treatment' }
    else role = { kind: 'unrelated' }
    roles.set(id, role)
  }

  return { treatment, outcome, edges: flows, roles, paths, adjustment, laggedArrows, pathCapReached: raw.length >= PATH_CAP }
}

export function roleDetail(role: DagCausalRole): string | null {
  switch (role.kind) {
    case 'treatment':
    case 'outcome':
    case 'unmeasured': return null
    case 'confounder': return role.paths === 1 ? 'Common cause on one open back-door path' : `Common cause on ${role.paths} open back-door paths`
    case 'backdoor-variable': return role.paths === 1 ? 'On one open back-door path' : `On ${role.paths} open back-door paths`
    case 'mediator': return role.alsoCollider ? 'Adjusting would both remove part of the total effect and introduce collider bias' : 'Carries part of the total effect; exclude it from total-effect adjustment'
    case 'collider': return role.paths === 1 ? 'Adjusting would open a closed path and introduce collider bias' : `Adjusting would open ${role.paths} closed paths and introduce collider bias`
    case 'post-treatment': return 'Keep out of adjustment'
    case 'outcome-predictor': return 'Not required to close a back-door path; may improve precision'
    case 'pre-treatment': return 'Not on any path'
    case 'unrelated': return 'Not connected to the effect'
    default: return assertNever(role)
  }
}

export function describeDagCausalRole(role: DagCausalRole): string {
  const detail = roleDetail(role)
  return detail === null ? roleWord(role) : `${roleWord(role)}: ${detail}`
}

/** Short role word for a card label. */
export function roleWord(role: DagCausalRole): string {
  switch (role.kind) {
    case 'treatment': return 'Treatment'
    case 'outcome': return 'Outcome'
    case 'confounder': return 'Confounder'
    case 'backdoor-variable': return 'Back-door variable'
    case 'mediator': return role.alsoCollider ? 'Mediator and collider' : 'Mediator'
    case 'collider': return 'Collider'
    case 'post-treatment': return 'Post-treatment'
    case 'outcome-predictor': return 'Outcome predictor'
    case 'pre-treatment': return 'Pre-treatment'
    case 'unmeasured': return 'Unmeasured'
    case 'unrelated': return 'Unrelated'
    default: return assertNever(role)
  }
}
