import dagre from '@dagrejs/dagre'
import type { DagNodeId, EditableDag } from '@/domain/dag'
import { enumeratePaths, type XyPath } from '@/domain/dagFlow'

export interface DagNodePlacement {
  readonly id: DagNodeId
  readonly x: number
  readonly y: number
}

/** One card size for every node in a graph: the width the longest name needs, up to a cap, and the
 *  height its lines need. Measured before layout (dagCardSize.ts) so dagre and the rails place cards
 *  of the size that will be drawn, instead of a fixed box a long name is then clipped to. */
export interface DagCardSize {
  readonly width: number
  readonly height: number
  /** Lines the longest name takes at this width; every card leaves room for that many. */
  readonly nameLines: number
}

export const DEFAULT_CARD_SIZE: DagCardSize = { width: 164, height: 58, nameLines: 1 }

const COLUMN_GAP = 86
const ROW_GAP = 46
const MARGIN = 34

/**
 * The layered left-to-right layout behind Tidy and behind a new document's first drawing: causes
 * left of effects, crossings minimised, long arrows given room between the columns they skip. Lagged
 * arrows count as arrows; self-loops do not.
 */
export type DagLayoutOrientation = 'across' | 'down'

export function layoutEditableDag(graph: EditableDag, orientation: DagLayoutOrientation = 'across', size: DagCardSize = DEFAULT_CARD_SIZE): readonly DagNodePlacement[] {
  return layoutDirectedGraph(graph.nodes, graph.edges, orientation, size)
}

export function layoutDirectedGraph<Id extends string>(nodes: readonly { readonly id: Id }[], edges: readonly { readonly cause: Id; readonly effect: Id }[], orientation: DagLayoutOrientation = 'across', size: DagCardSize = DEFAULT_CARD_SIZE): readonly { readonly id: Id; readonly x: number; readonly y: number }[] {
  const layout = new dagre.graphlib.Graph()
  layout.setGraph({ rankdir: orientation === 'across' ? 'LR' : 'TB', nodesep: ROW_GAP, ranksep: COLUMN_GAP, marginx: MARGIN, marginy: MARGIN })
  layout.setDefaultEdgeLabel(() => ({}))
  for (const node of nodes) layout.setNode(node.id, { width: size.width, height: size.height })
  for (const edge of edges) {
    if (edge.cause !== edge.effect) layout.setEdge(edge.cause, edge.effect)
  }
  dagre.layout(layout)
  return nodes.map((node) => {
    const placed = layout.node(node.id)
    return { id: node.id, x: placed.x - size.width / 2, y: placed.y - size.height / 2 }
  })
}

/** Column and row pitch of the rail layout: the dagre pitch across, more room down so diagonal arrows have length. */
const railColumn = (size: DagCardSize): number => size.width + COLUMN_GAP
const railRow = (size: DagCardSize): number => size.height + 72
/** Two rails of the same role share a row when every card keeps this much clear of the others. */
const railClearance = (size: DagCardSize): number => size.width + 28

export interface DagBinding {
  readonly treatment: DagNodeId
  readonly outcome: DagNodeId
}

const reaches = (edges: readonly { readonly from: string; readonly to: string }[], from: string, target: string): boolean => {
  const pending = [from]
  const visited = new Set<string>()
  while (pending.length > 0) {
    const current = pending.pop()
    if (current === undefined || visited.has(current)) continue
    if (current === target) return true
    visited.add(current)
    for (const edge of edges) if (edge.from === current) pending.push(edge.to)
  }
  return false
}

interface RailMember { readonly id: string; readonly slot: number; readonly of: number }

/**
 * The Octopus path grammar (`dagGrammar.ts`): treatment left and outcome right on a baseline, every
 * X–Y path composed as a rail around it, back-door paths above, causal paths below, shorter paths
 * nearer the line, so no arrow has to cross a card. With no direct X → Y arrow the longest open path
 * itself becomes the baseline, the way the books draw a bare chain flat. Contemporaneous arrows only;
 * lagged arrows are drawn wherever their ends land.
 */
export function layoutBoundDag(graph: EditableDag, binding: DagBinding, size: DagCardSize = DEFAULT_CARD_SIZE): readonly DagNodePlacement[] {
  const RAIL_COLUMN = railColumn(size)
  const RAIL_ROW = railRow(size)
  const RAIL_CLEARANCE = railClearance(size)
  const X = binding.treatment
  const Y = binding.outcome
  const ids = graph.nodes.map((node) => node.id)
  const edges = graph.edges
    .filter((edge) => edge.timing.kind === 'contemporaneous' && edge.cause !== edge.effect)
    .map((edge) => ({ from: edge.cause, to: edge.effect }))
  const paths = enumeratePaths(edges, X, Y)

  const interior = (path: XyPath): readonly string[] => path.nodes.slice(1, -1)
  const direct = edges.some((edge) => edge.from === X && edge.to === Y)
  const onCausal = new Set(paths.filter((path) => path.type === 'causal').flatMap(interior))
  // The baseline must not swallow a mediator, so prefer open paths clear of causal interiors.
  const open = paths.filter((path) => path.type !== 'closed')
  const clear = open.filter((path) => interior(path).every((id) => !onCausal.has(id)))
  const candidates = clear.length > 0 ? clear : open
  const baselinePath = direct || candidates.length === 0 ? null
    : candidates.reduce((longest, path) => (path.nodes.length > longest.nodes.length ? path : longest), candidates[0])
  const railed = paths.filter((path) => path !== baselinePath)
  const causalPaths = railed.filter((path) => path.type === 'causal' && path.nodes.length > 2)
  const backdoorPaths = railed.filter((path) => path.type === 'backdoor')
  const closedPaths = railed.filter((path) => path.type === 'closed')
  const onXyPath = new Set(paths.flatMap(interior))
  const leftoverAbove = ids.filter((id) => id !== X && id !== Y && !onXyPath.has(id) && (reaches(edges, id, X) || reaches(edges, id, Y)))
  const leftoverBelow = ids.filter((id) => id !== X && id !== Y && !onXyPath.has(id) && !leftoverAbove.includes(id))

  const columns = Math.max(1, ...paths.map((path) => path.nodes.length - 1))
  const span = columns * RAIL_COLUMN
  const baseX = MARGIN + size.width / 2

  const placed = new Map<string, { readonly x: number; readonly y: number }>()
  placed.set(X, { x: baseX, y: 0 })
  placed.set(Y, { x: baseX + span, y: 0 })
  const aboveRows: { readonly role: string; readonly y: number; readonly xs: number[] }[] = []
  const belowRows: { readonly role: string; readonly y: number; readonly xs: number[] }[] = []
  const addRail = (side: 1 | -1 | 0, role: string, members: readonly RailMember[]): void => {
    const fresh = members.filter((member) => !placed.has(member.id))
    if (fresh.length === 0) return
    const points = fresh.map((member) => ({ id: member.id, x: baseX + (member.slot / member.of) * span }))
    if (side === 0) {
      for (const point of points) placed.set(point.id, { x: point.x, y: 0 })
      return
    }
    const rows = side === -1 ? aboveRows : belowRows
    let row = rows.find((candidate) => candidate.role === role && points.every((point) => candidate.xs.every((x) => Math.abs(x - point.x) >= RAIL_CLEARANCE)))
    if (row === undefined) {
      row = { role, y: side * (rows.length + 1) * RAIL_ROW, xs: [] }
      rows.push(row)
    }
    for (const point of points) {
      row.xs.push(point.x)
      placed.set(point.id, { x: point.x, y: row.y })
    }
  }
  const railOf = (nodes: readonly string[]): readonly RailMember[] =>
    nodes.slice(1, -1).map((id, index) => ({ id, slot: index + 1, of: nodes.length - 1 }))
  const pathSlots = (path: XyPath, members: readonly string[]): readonly RailMember[] =>
    members.map((id) => ({ id, slot: path.nodes.indexOf(id), of: path.nodes.length - 1 }))

  if (baselinePath !== null) addRail(0, 'baseline', railOf(baselinePath.nodes))
  for (const path of causalPaths) addRail(1, 'causal', railOf(path.nodes))
  for (const path of backdoorPaths) addRail(-1, 'backdoor', railOf(path.nodes))
  addRail(-1, 'leftover', leftoverAbove.map((id, index) => ({ id, slot: index + 1, of: leftoverAbove.length + 1 })))
  for (const path of closedPaths) {
    const forks = interior(path).filter((id) => !path.colliders.includes(id))
    if (forks.length > 0) {
      // Forks are common causes, so the whole segment routes above, colliders nearest the line.
      addRail(-1, 'collider', pathSlots(path, path.colliders))
      addRail(-1, 'fork', pathSlots(path, forks))
    } else {
      // A bare common effect takes the emptier side, and the bottom on a tie.
      addRail(aboveRows.length < belowRows.length ? -1 : 1, 'collider', pathSlots(path, path.colliders))
    }
  }
  addRail(1, 'leftover', leftoverBelow.map((id, index) => ({ id, slot: index + 1, of: leftoverBelow.length + 1 })))

  const ys = [...placed.values()].map((point) => point.y)
  const shiftY = MARGIN + size.height / 2 - Math.min(...ys)
  return graph.nodes.map((node) => {
    const at = placed.get(node.id) ?? { x: baseX, y: 0 }
    return { id: node.id, x: at.x - size.width / 2, y: at.y + shiftY - size.height / 2 }
  })
}

/** The rail layout once treatment and outcome are bound and both are in the graph; dagre until then. */
export function layoutDagForCanvas(graph: EditableDag, binding: DagBinding | null, orientation: DagLayoutOrientation = 'across', size: DagCardSize = DEFAULT_CARD_SIZE): readonly DagNodePlacement[] {
  const bound = binding !== null
    && binding.treatment !== binding.outcome
    && graph.nodes.some((node) => node.id === binding.treatment)
    && graph.nodes.some((node) => node.id === binding.outcome)
  if (!bound) return layoutEditableDag(graph, orientation, size)
  const across = layoutBoundDag(graph, binding, size)
  return orientation === 'across' ? across : transposeRails(across, size)
}

/**
 * The rail layout turned to run down a narrow canvas: each across-column becomes a row, the
 * treatment at the top and the outcome at the bottom, and the cards that shared a column sit side
 * by side, centred, so nothing is wider than two cards.
 */
const transposeRails = (placements: readonly DagNodePlacement[], size: DagCardSize): readonly DagNodePlacement[] => {
  const DOWN_PITCH = size.width + 40
  const RAIL_COLUMN = railColumn(size)
  const RAIL_ROW = railRow(size)
  const columns = new Map<number, DagNodePlacement[]>()
  for (const placed of placements) {
    const column = Math.round((placed.x + size.width / 2) / RAIL_COLUMN)
    columns.set(column, [...(columns.get(column) ?? []), placed])
  }
  const ordered = [...columns.keys()].sort((a, b) => a - b)
  return ordered.flatMap((column, row) => {
    const members = [...(columns.get(column) ?? [])].sort((a, b) => a.y - b.y)
    return members.map((placed, index) => ({
      id: placed.id,
      x: MARGIN + (index - (members.length - 1) / 2) * DOWN_PITCH,
      y: MARGIN + row * RAIL_ROW,
    }))
  })
}
