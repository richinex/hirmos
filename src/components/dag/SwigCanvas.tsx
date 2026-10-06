import { useEffect, useRef, useState } from 'react'
import { MarkerType } from '@xyflow/react'
import {
  CanvasControls,
  useCanvasView,
  useDrawingStyle,
  type CanvasNode,
  type DerivedCanvasEdge as CanvasEdge,
} from './DagCanvas'
import { ReadOnlyCanvas, type ReadOnlyCanvasState } from './ReadOnlyCanvas'
import { layoutGraph } from './elkLayout'
import { dagCardSize } from './dagCardSize'
import { useTextMetricsVersion } from '@/lib/textMetrics'
import { swigNodeName, type SwigAnalysis } from '@/domain/swig'

type View = ReadOnlyCanvasState<CanvasEdge>

/** Read-only projection with the same card geometry, routing and theme as the source DAG. */
export function SwigCanvas({ record }: { readonly record: SwigAnalysis }) {
  const hostRef = useRef<HTMLDivElement>(null)
  const canvasView = useCanvasView(hostRef)
  const { orientation } = canvasView
  const [drawing] = useDrawingStyle()
  const metrics = useTextMetricsVersion()
  const [view, setView] = useState<View>({ kind: 'loading' })
  useEffect(() => {
    let active = true
    setView({ kind: 'loading' })
    const names = record.result.nodes.map((_, index) => swigNodeName(record, index))
    const size = dagCardSize(names)
    const graph = {
      nodes: names.map((_, index) => ({ id: `swig-node:${index}` })),
      edges: record.result.edges.map(([cause, effect], index) => ({
        id: `swig-edge:${index}`,
        cause: `swig-node:${cause}`,
        effect: `swig-node:${effect}`,
        labelSize: { width: 0, height: 0 },
      })),
    }
    void layoutGraph(graph, orientation, size).then((result) => {
      if (!active) return
      if (!result.ok) {
        setView({
          kind: 'failed',
          message: 'The derived graph could not be laid out. The saved analysis remains unchanged.',
        })
        return
      }
      const nodes: CanvasNode[] = []
      const edges: CanvasEdge[] = []
      for (const [index, node] of graph.nodes.entries()) {
        const position = result.value.nodes.get(node.id)
        const evidence = record.result.nodes[index]
        const name = names[index]
        if (position === undefined || evidence === undefined || name === undefined) {
          setView({ kind: 'failed', message: 'The graph layout omitted a recorded node.' })
          return
        }
        nodes.push({
          id: node.id,
          type: 'dagVariable',
          position,
          draggable: false,
          connectable: false,
          data: {
            name,
            kind: 'derived',
            role:
              evidence.kind === 'fixed'
                ? 'Fixed intervention'
                : evidence.kind === 'difference'
                  ? 'Outcome difference'
                  : evidence.role === 'exogenous'
                    ? 'Exogenous variable'
                    : 'Random variable',
            evidenceRole: 'none',
            droppable: false,
            intervention: evidence.kind === 'fixed' ? 'set' : null,
            size,
            drawing,
          },
        })
      }
      for (const edge of graph.edges) {
        const route = result.value.routes.get(edge.id)
        if (route === undefined) {
          setView({ kind: 'failed', message: 'The graph layout omitted a recorded arrow.' })
          return
        }
        edges.push({
          id: edge.id,
          type: 'dagEdge',
          source: edge.cause,
          target: edge.effect,
          data: { route, cut: false, drawing, labelPlacement: null },
          style: { stroke: 'var(--color-muted)', strokeWidth: 1.5 },
          markerEnd: {
            type: MarkerType.ArrowClosed,
            color: 'var(--color-muted)',
            width: 18,
            height: 18,
          },
        })
      }
      setView({ kind: 'ready', nodes, edges })
    })
    return () => {
      active = false
    }
  }, [record, orientation, drawing, metrics])
  return (
    <ReadOnlyCanvas
      host={hostRef}
      view={canvasView}
      state={view}
      label="Read-only SWIG graph"
      loading="Laying out the derived graph…"
      layoutKey={`${record.id}\u0000${orientation}`}
      className="h-[max(30rem,60dvh)]"
    >
      <CanvasControls view={canvasView} />
    </ReadOnlyCanvas>
  )
}
