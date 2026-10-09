import type { ReactNode } from 'react'
import { ReactFlow, type Edge } from '@xyflow/react'
import {
  DagGrid,
  DagVariableCard,
  DagEdgePath,
  RefitOnResize,
  type CanvasNode,
  type useCanvasView,
} from './DagCanvas'
import { Alert } from '@/components/ui/Alert'
import { FloatingWindow } from '@/components/ui/FloatingWindow'
import { button } from '@/components/ui/recipes'
import { FLOW_DEFAULTS } from '@/components/flow/defaults'

const NODE_TYPES = { dagVariable: DagVariableCard }
const EDGE_TYPES = { dagEdge: DagEdgePath }
const FIT = { padding: 0.2, maxZoom: 1 }

export type ReadOnlyCanvasState<E extends Edge> =
  | {
      readonly kind: 'loading'
      readonly previous?: { readonly nodes: CanvasNode[]; readonly edges: E[] }
    }
  | {
      readonly kind: 'failed'
      readonly message: string
      readonly previous?: { readonly nodes: CanvasNode[]; readonly edges: E[] }
    }
  | { readonly kind: 'ready'; readonly nodes: CanvasNode[]; readonly edges: E[] }

/** A graph drawn with the DAG workspace's cards and routing, without editing. */
export function ReadOnlyCanvas<E extends Edge>({
  host,
  view,
  state,
  label,
  loading,
  layoutKey,
  className,
  frame = 'well',
  onRetry,
  children,
}: {
  readonly host: React.RefObject<HTMLDivElement | null>
  readonly view: ReturnType<typeof useCanvasView>
  readonly state: ReadOnlyCanvasState<E>
  readonly label: string
  readonly loading: string
  readonly layoutKey: string
  readonly className: string
  readonly frame?: 'well' | 'none'
  readonly onRetry?: () => void
  readonly children?: ReactNode
}) {
  const drawing = state.kind === 'ready' ? state : state.previous
  const canvas = (
    <div
      ref={host}
      aria-label={label}
      className={
        view.expanded
          ? 'relative min-h-0 flex-1 overflow-hidden'
          : `relative ${className} min-w-0 overflow-hidden ${frame === 'well' ? 'rounded-xl border border-edge bg-well' : ''}`
      }
    >
      {state.kind === 'loading' && (
        <p role="status" className="p-4 text-body text-muted">
          {loading}
        </p>
      )}
      {state.kind === 'failed' && (
        <Alert tone="danger">
          {state.message}
          {onRetry !== undefined && (
            <button type="button" className={button('outline')} onClick={onRetry}>
              Retry layout
            </button>
          )}
        </Alert>
      )}
      {drawing !== undefined && (
        <ReactFlow
          {...FLOW_DEFAULTS}
          nodes={drawing.nodes}
          edges={drawing.edges}
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
          zoomOnScroll={!view.viewLocked}
          preventScrolling={!view.viewLocked}
          proOptions={{ hideAttribution: true }}
        >
          <DagGrid />
          {children}
          <RefitOnResize host={host} layoutKey={layoutKey} />
        </ReactFlow>
      )}
    </div>
  )
  return view.expanded ? (
    <FloatingWindow
      label={label}
      onClose={view.toggleExpanded}
      defaultWidth={1100}
      defaultHeight={720}
    >
      {canvas}
    </FloatingWindow>
  ) : (
    canvas
  )
}
