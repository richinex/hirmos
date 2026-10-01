import ELK, { type ELK as ElkEngine } from 'elkjs/lib/elk-api'
import ElkWorker from 'elkjs/lib/elk-worker.min.js?worker'
import { z } from 'zod'
import { err, ok, type Result, type NonEmptyArray } from '@/domain/dop'
import type { EditableDag, DagEdgeId, DagNodeId } from '@/domain/dag'
import type { DagCardSize, DagLayoutOrientation } from './dagCanvasModel'

export type LayoutPoint = { readonly x: number; readonly y: number }
export interface DagRoute {
  readonly points: NonEmptyArray<LayoutPoint>
  readonly sourcePosition: LayoutPoint
  readonly targetPosition: LayoutPoint
}
export interface DagLayout {
  readonly nodes: ReadonlyMap<DagNodeId, LayoutPoint>
  readonly routes: ReadonlyMap<DagEdgeId, DagRoute>
}
export type LayoutProblem =
  | { readonly kind: 'engine'; readonly message: string }
  | { readonly kind: 'invalid-result' }

const point = z.object({ x: z.number().finite(), y: z.number().finite() })
const output = z.object({
  children: z.array(point.extend({ id: z.string() })),
  edges: z.array(z.object({ id: z.string(), sections: z.array(z.object({
    startPoint: point, endPoint: point, bendPoints: z.array(point).optional(),
  })).length(1) })).optional(),
})

// Vite serves a separate worker chunk. Layout never runs on React's rendering thread.
let engine: ElkEngine | undefined
export async function layoutDag(graph: EditableDag, orientation: DagLayoutOrientation, size: DagCardSize): Promise<Result<DagLayout, LayoutProblem>> {
  try {
    engine ??= new ELK({ workerFactory: () => new ElkWorker() })
    const raw = await engine.layout({
      id: 'dag',
      layoutOptions: {
        'elk.algorithm': 'layered',
        'elk.direction': orientation === 'across' ? 'RIGHT' : 'DOWN',
        'elk.edgeRouting': 'POLYLINE',
        'elk.spacing.nodeNode': '46',
        'elk.layered.spacing.nodeNodeBetweenLayers': '86',
        'elk.spacing.edgeNode': '18',
        'elk.padding': '[top=34,left=34,bottom=34,right=34]',
        'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
        'elk.layered.nodePlacement.favorStraightEdges': 'true',
        'elk.randomSeed': '1',
      },
      children: graph.nodes.map(node => ({ id: node.id, width: size.width, height: size.height })),
      edges: graph.edges.map(edge => ({ id: edge.id, sources: [edge.cause], targets: [edge.effect] })),
    })
    const parsed = output.safeParse(raw)
    if (!parsed.success) return err({ kind: 'invalid-result' })
    const placedNodes = new Map(parsed.data.children.map(node => [node.id, node]))
    const placedEdges = new Map((parsed.data.edges ?? []).map(edge => [edge.id, edge]))
    const nodes = new Map<DagNodeId, LayoutPoint>()
    for (const node of graph.nodes) {
      const placed = placedNodes.get(node.id)
      if (placed === undefined) return err({ kind: 'invalid-result' })
      nodes.set(node.id, { x: placed.x, y: placed.y })
    }
    const routes = new Map<DagEdgeId, DagRoute>()
    for (const edge of graph.edges) {
      const routed = placedEdges.get(edge.id)
      const section = routed?.sections[0]
      const sourcePosition = nodes.get(edge.cause)
      const targetPosition = nodes.get(edge.effect)
      if (section === undefined || sourcePosition === undefined || targetPosition === undefined) return err({ kind: 'invalid-result' })
      routes.set(edge.id, { points: [section.startPoint, ...(section.bendPoints ?? []), section.endPoint], sourcePosition, targetPosition })
    }
    return ok({ nodes, routes })
  } catch (cause) {
    return err({ kind: 'engine', message: cause instanceof Error ? cause.message : String(cause) })
  }
}

/** Keep the engine's route attached during card movement, without inventing another router. */
export function movedRoute(route: DagRoute, source: LayoutPoint, target: LayoutPoint): NonEmptyArray<LayoutPoint> {
  const last = route.points.length - 1
  const move = (p: LayoutPoint, index: number): LayoutPoint => {
    const t = last === 0 ? 0 : index / last
    return {
      x: p.x + (source.x - route.sourcePosition.x) * (1 - t) + (target.x - route.targetPosition.x) * t,
      y: p.y + (source.y - route.sourcePosition.y) * (1 - t) + (target.y - route.targetPosition.y) * t,
    }
  }
  const [first, ...rest] = route.points
  const points: NonEmptyArray<LayoutPoint> = [move(first, 0), ...rest.map((p, i) => move(p, i + 1))]
  // ELK ends at the card border. Leave room for the arrowhead and the reconnect grip outside it.
  const inset = (end: LayoutPoint, toward: LayoutPoint): LayoutPoint => {
    const length = Math.hypot(toward.x - end.x, toward.y - end.y)
    const scale = length === 0 ? 0 : Math.min(5.5, length / 3) / length
    return { x: end.x + (toward.x - end.x) * scale, y: end.y + (toward.y - end.y) * scale }
  }
  const next = points[1]
  const end = points[last]
  const previous = points[last - 1]
  if (next === undefined || end === undefined || previous === undefined) return points
  return [inset(points[0], next), ...points.slice(1, -1), inset(end, previous)]
}
