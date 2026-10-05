import { useEffect, useRef, useState } from 'react'
import { createPortal } from 'react-dom'
import { MarkerType, ReactFlow } from '@xyflow/react'
import {
  CanvasControls,
  DagGrid,
  DagVariableCard,
  DagEdgePath,
  RefitOnResize,
  useCanvasView,
  useDrawingStyle,
  type CanvasNode,
  type DerivedCanvasEdge as CanvasEdge,
} from './DagCanvas'
import { layoutGraph } from './elkLayout'
import { dagCardSize } from './dagCardSize'
import { useTextMetricsVersion } from '@/lib/textMetrics'
import { swigNodeName, type SwigAnalysis } from '@/domain/swig'
import { Alert } from '@/components/ui/Alert'

const NODE_TYPES = { dagVariable: DagVariableCard }
const EDGE_TYPES = { dagEdge: DagEdgePath }
const FIT = { padding: 0.2, maxZoom: 1 }
type View =
  | { readonly kind: 'loading' }
  | { readonly kind: 'failed'; readonly message: string }
  | { readonly kind: 'ready'; readonly nodes: CanvasNode[]; readonly edges: CanvasEdge[] }

/** Read-only projection with the same card geometry, routing and theme as the source DAG. */
export function SwigCanvas({ record }: { readonly record: SwigAnalysis }) {
  const hostRef = useRef<HTMLDivElement>(null)
  const canvasView = useCanvasView(hostRef)
  const { expanded, orientation, viewLocked } = canvasView
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
  const canvas = (
    <div
      ref={hostRef}
      aria-label="Read-only SWIG graph"
      className={
        expanded
          ? 'fixed inset-3 z-(--z-dialog) flex flex-col overflow-hidden rounded-xl border border-edge bg-well float'
          : 'relative h-[max(30rem,60dvh)] min-w-0 overflow-hidden rounded-xl border border-edge bg-well'
      }
    >
      {view.kind === 'loading' && (
        <p role="status" className="p-4 text-body text-muted">
          Laying out the derived graph…
        </p>
      )}
      {view.kind === 'failed' && <Alert tone="danger">{view.message}</Alert>}
      {view.kind === 'ready' && (
        <ReactFlow
          nodes={view.nodes}
          edges={view.edges}
          nodeTypes={NODE_TYPES}
          edgeTypes={EDGE_TYPES}
          nodesDraggable={false}
          nodesConnectable={false}
          edgesReconnectable={false}
          elementsSelectable={false}
          deleteKeyCode={null}
          fitView
          fitViewOptions={FIT}
          minZoom={0.1}
          maxZoom={2}
          zoomOnScroll={!viewLocked}
          preventScrolling={!viewLocked}
          proOptions={{ hideAttribution: true }}
        >
          <DagGrid />
          <CanvasControls view={canvasView} />
          <RefitOnResize host={hostRef} layoutKey={`${record.id}\u0000${orientation}`} />
        </ReactFlow>
      )}
    </div>
  )
  // Expanded, the canvas leaves the workbench pane, whose size containment would clip a fixed layer.
  return expanded ? createPortal(canvas, globalThis.document.body) : canvas
}
