import RoutingWorker from './fixedRouting.worker?worker'
import { z } from 'zod'
import { err, ok, type Result } from '@/domain/dop'
import type { EditableDag, DagEdgeId, DagNodeId } from '@/domain/dag'
import type { DagCardSize } from './dagCanvasModel'
import { type DagLayout, type DagRoute, type LayoutPoint, type LayoutProblem } from './elkLayout'

const point = z.object({ x: z.number().finite(), y: z.number().finite() })
const response = z.discriminatedUnion('kind', [
  z.object({ request: z.number().int(), kind: z.literal('routed'), routes: z.array(z.object({ id: z.string(), points: z.array(point).min(2) })) }),
  z.object({ request: z.number().int(), kind: z.literal('failed'), message: z.string() }),
])
let worker: Worker | undefined
let sequence = 0
const pending = new Map<number, (result: Result<z.infer<typeof response>, LayoutProblem>) => void>()

function requestRoutes(graph: unknown): Promise<Result<z.infer<typeof response>, LayoutProblem>> {
  worker ??= new RoutingWorker()
  worker.onmessage = (event: MessageEvent<unknown>) => {
    const parsed = response.safeParse(event.data)
    if (!parsed.success) {
      for (const finish of pending.values()) finish(err({ kind: 'invalid-result' }))
      pending.clear()
      return
    }
    pending.get(parsed.data.request)?.(ok(parsed.data))
    pending.delete(parsed.data.request)
  }
  worker.onerror = event => {
    for (const finish of pending.values()) finish(err({ kind: 'engine', message: event.message }))
    pending.clear()
    worker?.terminate()
    worker = undefined
  }
  const request = ++sequence
  return new Promise(resolve => { pending.set(request, resolve); worker!.postMessage({ request, graph }) })
}

/** libavoid routes around cards at their actual positions. No synthetic fallback routes. */
export async function routeFixedDag(graph: EditableDag, nodes: ReadonlyMap<DagNodeId, LayoutPoint>, size: DagCardSize, automatic: DagLayout): Promise<Result<DagLayout, LayoutProblem>> {
  try {
    const result = await requestRoutes({
      id: 'dag',
      children: graph.nodes.map(node => ({ id: node.id, ...nodes.get(node.id), width: size.width, height: size.height })),
      edges: graph.edges.filter(edge => edge.cause !== edge.effect).map(edge => ({ id: edge.id, sources: [edge.cause], targets: [edge.effect] })),
    })
    if (!result.ok) return result
    if (result.value.kind === 'failed') return err({ kind: 'engine', message: result.value.message })
    const routed = new Map(result.value.routes.map(route => [route.id, route]))
    const routes = new Map<DagEdgeId, DagRoute>()
    for (const edge of graph.edges) {
      const sourcePosition = nodes.get(edge.cause)
      const targetPosition = nodes.get(edge.effect)
      if (!sourcePosition || !targetPosition) return err({ kind: 'invalid-result' })
      if (edge.cause === edge.effect) {
        const loop = automatic.routes.get(edge.id)
        if (!loop) return err({ kind: 'invalid-result' })
        // Preserve ELK's self-loop shape, translated to the held card, not libavoid's synthetic loop.
        const delta = { x: sourcePosition.x - loop.sourcePosition.x, y: sourcePosition.y - loop.sourcePosition.y }
        const [first, ...rest] = loop.points
        routes.set(edge.id, { points: [{ x: first.x + delta.x, y: first.y + delta.y }, ...rest.map(p => ({ x: p.x + delta.x, y: p.y + delta.y }))], sourcePosition, targetPosition })
      } else {
        const points = routed.get(edge.id)?.points
        const first = points?.[0]
        if (!points || !first) return err({ kind: 'invalid-result' })
        routes.set(edge.id, { points: [first, ...points.slice(1)], sourcePosition, targetPosition })
      }
    }
    return ok({ nodes, routes })
  } catch (cause) {
    return err({ kind: 'engine', message: cause instanceof Error ? cause.message : String(cause) })
  }
}
