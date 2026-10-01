import { fontFor, textWidth, lineHeightFor } from '@/lib/textMetrics'
import type { EditableDag, DirectedDagEdge, DagEdgeId } from '@/domain/dag'
import type { DagCardSize } from './dagCanvasModel'
import type { DagLayout, LayoutPoint } from './elkLayout'

export function routeLabelText(edge: DirectedDagEdge): string {
  return [edge.timing?.kind === 'lagged' ? `t−${edge.timing.lag}` : '', edge.support?.kind === 'unstated' ? 'needs rationale' : ''].filter(Boolean).join('; ')
}
export function routeLabelSize(edge: DirectedDagEdge) {
  const font = fontFor('micro', 400, 'mono')
  return { width: Math.max(textWidth(routeLabelText(edge), font), textWidth('cut by do()', font)) + 14, height: lineHeightFor('micro', font) + 6 }
}
type Box = LayoutPoint & { readonly width: number; readonly height: number }
export type RouteLabel = { readonly kind: 'clear' | 'crowded'; readonly centre: LayoutPoint }
const overlap = (a: Box, b: Box): number => Math.max(0, Math.min(a.x + a.width, b.x + b.width) - Math.max(a.x, b.x)) * Math.max(0, Math.min(a.y + a.height, b.y + b.height) - Math.max(a.y, b.y))

/** Label placement only, not an edge router. Test complete label boxes against cards and earlier labels. */
export function placeRouteLabels(graph: EditableDag, layout: DagLayout, size: DagCardSize): ReadonlyMap<DagEdgeId, RouteLabel> {
  const obstacles: Box[] = [...layout.nodes.values()].map(p => ({ x: p.x - 6, y: p.y - 6, width: size.width + 12, height: size.height + 12 }))
  const labels = new Map<DagEdgeId, RouteLabel>()
  for (const edge of graph.edges) {
    const route = layout.routes.get(edge.id)
    if (!route) continue
    const dimensions = routeLabelSize(edge)
    const candidates: Box[] = []
    for (let index = 1; index < route.points.length; index++) {
      const a = route.points[index - 1]!
      const b = route.points[index]!
      const length = Math.hypot(b.x - a.x, b.y - a.y)
      if (length < 1) continue
      for (const t of [0.5, 0.3, 0.7]) for (const distance of [18, -18, 34, -34]) {
        const x = a.x + (b.x - a.x) * t + (b.y - a.y) / length * distance
        const y = a.y + (b.y - a.y) * t - (b.x - a.x) / length * distance
        candidates.push({ x: x - dimensions.width / 2, y: y - dimensions.height / 2, ...dimensions })
      }
    }
    let best: Box | undefined
    let cost = Infinity
    for (const candidate of candidates) {
      const score = obstacles.reduce((sum, obstacle) => sum + overlap(candidate, obstacle), 0)
      if (score < cost) { best = candidate; cost = score }
      if (score === 0) break
    }
    if (!best) continue
    labels.set(edge.id, { kind: cost === 0 ? 'clear' : 'crowded', centre: { x: best.x + best.width / 2, y: best.y + best.height / 2 } })
    obstacles.push({ ...best, x: best.x - 4, y: best.y - 4, width: best.width + 8, height: best.height + 8 })
  }
  return labels
}
