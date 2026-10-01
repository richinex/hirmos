import { init, routeEdges, type ElkGraph } from '@mr_mint/elkjs-libavoid'
import wasmUrl from '../../../node_modules/libavoid-js/dist/libavoid.wasm?url'

// The main-thread adapter validates the response before any geometry enters the canvas.
self.onmessage = async (event: MessageEvent<{ readonly request: number; readonly graph: ElkGraph }>) => {
  const { request, graph } = event.data
  try {
    await init(wasmUrl)
    const routes = await routeEdges(graph, { routingType: 'polyline', shapeBufferDistance: 12, selfLoopHandling: 'skip' })
    self.postMessage({ request, kind: 'routed', routes: [...routes].map(([id, route]) => ({ id, points: [route.sourcePoint, ...route.bendPoints, route.targetPoint] })) })
  } catch (cause) {
    self.postMessage({ request, kind: 'failed', message: cause instanceof Error ? cause.message : String(cause) })
  }
}
