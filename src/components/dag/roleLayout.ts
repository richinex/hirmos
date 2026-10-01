// The layout module alone: the package entry also loads the D3 adaptors, which the canvas does not use.
import { Layout } from 'webcola/dist/src/layout'
import { err, ok, type Result } from '@/domain/dop'
import type { DagNodeId, EditableDag } from '@/domain/dag'
import type { DagCausalFlow, DagCausalRole } from '@/domain/dagFlow'
import type { DagCardSize, DagLayoutOrientation } from './dagCanvasModel'
import type { LayoutPoint, LayoutProblem } from './elkLayout'

/**
 * Where a variable sits relative to the line from treatment to outcome, the way causal diagrams are
 * drawn in the books: the causal chain on the line, common causes above it, what the treatment
 * produces below it. Variables whose role puts them on neither side are shared out to balance the two.
 */
type Side =
  | { readonly kind: 'line'; readonly at: number }
  | { readonly kind: 'above'; readonly between: boolean }
  | { readonly kind: 'below'; readonly between: boolean }
  | { readonly kind: 'either' }

const sideOf = (role: DagCausalRole): Side => {
  switch (role.kind) {
    case 'treatment':
    case 'outcome': return { kind: 'line', at: 0 }
    case 'confounder':
    case 'backdoor-variable':
    case 'unmeasured': return { kind: 'above', between: true }
    case 'mediator': return { kind: 'below', between: true }
    case 'collider':
    case 'post-treatment': return { kind: 'below', between: false }
    case 'outcome-predictor':
    case 'pre-treatment':
    case 'unrelated': return { kind: 'either' }
  }
}

/** Space between cards along the line, beyond the cards themselves. */
const LINE_GAP = 90
/** Space between the line and the first card on either side. */
const SIDE_GAP = 70
/** Clear space the overlap rule keeps around every card. */
const CARD_PADDING = 22
const MARGIN = 34

/** Constraint shapes the layout engine accepts: one variable at least `gap` before another on an axis, or a shared coordinate. */
type Constraint =
  | { readonly axis: 'x' | 'y'; readonly left: number; readonly right: number; readonly gap: number }
  | { readonly type: 'alignment'; readonly axis: 'x' | 'y'; readonly offsets: readonly { readonly node: number; readonly offset: number }[] }

/**
 * Card positions for a graph with a treatment and an outcome, as top-left corners in the units ELK
 * uses, so the routing step can take them unchanged. The treatment-to-outcome line runs across the
 * canvas, or down it when the canvas is narrow.
 */
export function placeByRole(graph: EditableDag, flow: DagCausalFlow, orientation: DagLayoutOrientation, size: DagCardSize): Result<ReadonlyMap<DagNodeId, LayoutPoint>, LayoutProblem> {
  const index = new Map(graph.nodes.map((node, i) => [node.id, i]))
  const treatment = index.get(flow.treatment)
  const outcome = index.get(flow.outcome)
  if (treatment === undefined || outcome === undefined) return err({ kind: 'invalid-result' })
  const arrows = graph.edges.filter((edge) => edge.timing.kind === 'contemporaneous' && edge.cause !== edge.effect && index.has(edge.cause) && index.has(edge.effect))

  const along = orientation === 'across' ? 'x' : 'y'
  const across = orientation === 'across' ? 'y' : 'x'
  const alongSize = orientation === 'across' ? size.width : size.height
  const acrossSize = orientation === 'across' ? size.height : size.width

  // The longest causal path lies on the line; without one, the line is the treatment and the outcome alone.
  const causal = flow.paths
    .filter((path) => path.type === 'causal' && path.nodes.every((id) => index.has(id)))
    .reduce<readonly DagNodeId[]>((longest, path) => (path.nodes.length > longest.length ? path.nodes : longest), [flow.treatment, flow.outcome])
  const sides = new Map<DagNodeId, Side>()
  causal.forEach((id, at) => sides.set(id, { kind: 'line', at }))
  for (const node of graph.nodes) {
    if (sides.has(node.id)) continue
    const role = flow.roles.get(node.id) ?? { kind: 'unrelated' }
    sides.set(node.id, sideOf(role))
  }
  balanceSides(graph, arrows, sides)

  const constraints: Constraint[] = []
  constraints.push({ type: 'alignment', axis: across, offsets: causal.map((id) => ({ node: index.get(id) ?? 0, offset: 0 })) })
  for (let i = 0; i + 1 < causal.length; i += 1) {
    constraints.push({ axis: along, left: index.get(causal[i]) ?? 0, right: index.get(causal[i + 1]) ?? 0, gap: alongSize + LINE_GAP })
  }
  for (const [id, side] of sides) {
    const at = index.get(id) ?? 0
    if (side.kind === 'line' || side.kind === 'either') continue
    if (side.kind === 'above') constraints.push({ axis: across, left: at, right: treatment, gap: acrossSize + SIDE_GAP })
    else constraints.push({ axis: across, left: treatment, right: at, gap: acrossSize + SIDE_GAP })
    if (side.between) {
      constraints.push({ axis: along, left: treatment, right: at, gap: 0 })
      constraints.push({ axis: along, left: at, right: outcome, gap: 0 })
    }
  }
  for (const arrow of arrows) {
    const cause = sides.get(arrow.cause)
    const effect = sides.get(arrow.effect)
    if (cause === undefined || effect === undefined || (cause.kind === 'line' && effect.kind === 'line')) continue
    // A cause stays before its effect, except a common cause, which sits over the gap it confounds.
    if (!(cause.kind === 'above' && cause.between)) constraints.push({ axis: along, left: index.get(arrow.cause) ?? 0, right: index.get(arrow.effect) ?? 0, gap: alongSize / 2 })
    // On one side of the line, causes sit no nearer the line than their effects above it, and effects no nearer below it.
    if (cause.kind === effect.kind && (cause.kind === 'above' || cause.kind === 'below')) {
      constraints.push({ axis: across, left: index.get(arrow.cause) ?? 0, right: index.get(arrow.effect) ?? 0, gap: 0 })
    }
  }

  const spacing = alongSize + LINE_GAP
  const nodes = graph.nodes.map((node) => {
    const side = sides.get(node.id) ?? { kind: 'either' }
    const offset = side.kind === 'above' ? -1 : side.kind === 'below' ? 1 : 0
    const position = side.kind === 'line' ? side.at : (index.get(node.id) ?? 0) % Math.max(causal.length, 1)
    const start = { along: position * spacing, across: offset * (acrossSize + SIDE_GAP) }
    return {
      width: size.width + 2 * CARD_PADDING,
      height: size.height + 2 * CARD_PADDING,
      x: orientation === 'across' ? start.along : start.across,
      y: orientation === 'across' ? start.across : start.along,
    }
  })
  const links = arrows.map((edge) => ({ source: index.get(edge.cause) ?? 0, target: index.get(edge.effect) ?? 0 }))

  try {
    new Layout()
      .nodes(nodes)
      .links(links)
      .constraints([...constraints])
      .linkDistance(spacing)
      .avoidOverlaps(true)
      .handleDisconnected(false)
      .start(30, 30, 60, 0, false, false)
  } catch (cause) {
    return err({ kind: 'engine', message: cause instanceof Error ? cause.message : String(cause) })
  }
  if (nodes.some((node) => !Number.isFinite(node.x) || !Number.isFinite(node.y))) return err({ kind: 'invalid-result' })

  // The engine places centres; the canvas and the router take top-left corners clear of the margin.
  const left = Math.min(...nodes.map((node) => node.x)) - size.width / 2
  const top = Math.min(...nodes.map((node) => node.y)) - size.height / 2
  return ok(new Map(graph.nodes.map((node, i) => [node.id, { x: nodes[i].x - size.width / 2 - left + MARGIN, y: nodes[i].y - size.height / 2 - top + MARGIN }])))
}

/**
 * Gives each variable that may sit on either side the side with fewer cards. Variables joined by
 * arrows among themselves stay together, so a chain of them does not straddle the line.
 */
function balanceSides(graph: EditableDag, arrows: EditableDag['edges'], sides: Map<DagNodeId, Side>): void {
  const count = { above: 0, below: 0 }
  for (const side of sides.values()) if (side.kind === 'above' || side.kind === 'below') count[side.kind] += 1

  const neighbours = new Map<DagNodeId, DagNodeId[]>()
  for (const arrow of arrows) {
    if (sides.get(arrow.cause)?.kind !== 'either' || sides.get(arrow.effect)?.kind !== 'either') continue
    neighbours.set(arrow.cause, [...(neighbours.get(arrow.cause) ?? []), arrow.effect])
    neighbours.set(arrow.effect, [...(neighbours.get(arrow.effect) ?? []), arrow.cause])
  }

  for (const node of graph.nodes) {
    if (sides.get(node.id)?.kind !== 'either') continue
    const group: DagNodeId[] = []
    const pending = [node.id]
    const seen = new Set<DagNodeId>()
    while (pending.length > 0) {
      const id = pending.pop()
      if (id === undefined || seen.has(id)) continue
      seen.add(id)
      group.push(id)
      pending.push(...(neighbours.get(id) ?? []))
    }
    const kind = count.above <= count.below ? 'above' : 'below'
    for (const id of group) sides.set(id, { kind, between: false })
    count[kind] += group.length
  }
}
