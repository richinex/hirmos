import { useCallback, useMemo, useState, type DragEvent } from 'react'
import {
  Background,
  BackgroundVariant,
  Handle,
  Position,
  ReactFlow,
  ReactFlowProvider,
  useReactFlow,
  type Connection,
  type Edge,
  type EdgeChange,
  type FinalConnectionState,
  type ReactFlowInstance,
  type Node,
  type NodeChange,
  type NodeProps,
} from '@xyflow/react'
import '@xyflow/react/dist/style.css'
import { Icon } from '@/components/Icon'
import { FlowControls, flowControl } from '@/components/flow/FlowControls'
import { useIsMobile } from '@/lib/useMediaQuery'
import { blockLabel, type PipelineBlockId, type PipelineEdge, type PipelineGraph, type PipelineNode } from '@/domain/pipeline'
import type { BlockOutcome } from '@/data/pipeline'
import { formatCount, formatDuration } from '@/lib/format/number'
import { Orb } from '@/components/ui/Orb'
import { usePythonRun } from './usePythonRun'
import { cn } from '@/lib/utils'
import { BLOCK_DRAG_TYPE, blockIcon, type PaletteKind } from './blockIcons'
import { CARD_WIDTH, connectionRefusal, indexGraph, inputPorts, type ConnectionRefusal } from './pipelineWorkspaceModel'

interface CardData extends Record<string, unknown> {
  readonly node: PipelineNode
  readonly ports: number
  readonly outcome: BlockOutcome | undefined
  readonly summary: string
}
type CanvasNode = Node<CardData, 'block'>
type CanvasEdge = Edge<{ readonly edge: PipelineEdge }>

const PORT_STYLE: React.CSSProperties = { width: 10, height: 10, background: 'var(--color-panel)', border: '1.5px solid var(--color-edge)' }

function BlockCard({ data, selected }: NodeProps<CanvasNode>) {
  const { node, ports, outcome } = data
  const running = usePythonRun(node.id)
  const failed = outcome?.kind === 'failed'
  const skipped = outcome?.kind === 'skipped'
  const foot = running !== null
    ? <span className="flex items-center gap-1.5"><Orb state="working" aria-label="Script running" className="scale-75" />running · {formatDuration(running.elapsedMs).text}</span>
    : outcome === undefined
      ? 'not run'
      : outcome.kind === 'ran'
        ? `${formatCount(outcome.rowCount).text} rows · ${outcome.columns.length} ${outcome.columns.length === 1 ? 'column' : 'columns'}`
        : outcome.kind === 'failed' ? 'failed' : outcome.kind === 'waiting' ? 'waiting' : 'not run'
  return (
    <div
      className={cn('rounded-lg border bg-panel text-left', selected ? 'border-signal' : failed ? 'border-danger' : 'border-hair', skipped && 'opacity-50')}
      style={{ width: CARD_WIDTH, boxShadow: selected ? '0 0 0 2px var(--color-panel), 0 0 0 3px var(--color-signal)' : undefined }}
      data-testid={`block-${node.id}`}
    >
      {Array.from({ length: ports }, (_, port) => (
        <Handle
          key={port}
          id={`in-${port}`}
          type="target"
          position={Position.Top}
          style={{ ...PORT_STYLE, left: `${((port + 1) / (ports + 1)) * 100}%` }}
          title={ports > 1 ? `Input ${port + 1}` : 'Input'}
        />
      ))}
      <div className="flex items-center gap-2 px-2.5 pt-2 pb-1">
        <span className={cn('grid h-5 w-5 shrink-0 place-items-center rounded', node.block.kind === 'input' || node.block.kind === 'output' ? 'bg-ink text-panel' : 'bg-raised text-muted')}><Icon name={blockIcon(node.block.kind)} size={14} /></span>
        <span className="truncate text-body font-medium text-ink">{blockLabel(node.block.kind)}</span>
      </div>
      <p className="m-0 truncate px-2.5 pb-2 font-mono text-[11px] text-faint" title={data.summary}>{data.summary}</p>
      <div className={cn('flex justify-between border-t border-line px-2.5 py-1 text-[10px]', failed ? 'text-danger' : 'text-faint')}>
        <span className="tabular-nums">{foot}</span>
      </div>
      {node.block.kind !== 'output' && (
        <Handle id="out" type="source" position={Position.Bottom} style={PORT_STYLE} title="Output" />
      )}
    </div>
  )
}

const NODE_TYPES = { block: BlockCard }
const edgeId = (edge: PipelineEdge): string => `${edge.from}->${edge.to}:${edge.port}`
const portOf = (handle: string | null | undefined): number => Number((handle ?? 'in-0').replace('in-', '')) || 0

export function PipelineCanvas(props: PipelineCanvasProps) {
  return (
    <div className="absolute inset-0" data-testid="pipeline-canvas">
      <ReactFlowProvider>
        <Flow {...props} />
      </ReactFlowProvider>
    </div>
  )
}

interface PipelineCanvasProps {
  readonly graph: PipelineGraph
  readonly outcomes: ReadonlyMap<PipelineBlockId, BlockOutcome>
  readonly selected: PipelineBlockId | null
  readonly summaries: ReadonlyMap<PipelineBlockId, string>
  readonly onSelect: (id: PipelineBlockId | null) => void
  readonly onMove: (id: PipelineBlockId, position: { readonly x: number; readonly y: number }) => void
  readonly onConnect: (edge: PipelineEdge) => void
  readonly onRemoveEdge: (edge: PipelineEdge) => void
  readonly onRemoveBlock: (id: PipelineBlockId) => void
  readonly onRefused: (refusal: ConnectionRefusal | null) => void
  readonly onDropBlock: (kind: PaletteKind, position: { readonly x: number; readonly y: number }) => void
  readonly onTidy: () => void
}

const FIT_VIEW = { padding: 0.2, maxZoom: 1 } as const
const EDGE_STYLE = { stroke: 'var(--color-edge)', strokeWidth: 1.5 } as const
const CONNECTION_LINE_STYLE = { stroke: 'var(--color-signal)', strokeWidth: 2 } as const
const DEFAULT_EDGE_OPTIONS = { type: 'smoothstep' } as const
const PRO_OPTIONS = { hideAttribution: true } as const
const BACKGROUND = <Background variant={BackgroundVariant.Dots} color="var(--color-edge)" gap={18} size={1} />
const fitAfterLayout = (instance: ReactFlowInstance<CanvasNode, CanvasEdge>) => { void instance.fitView(FIT_VIEW) }

const toEdge = (connection: Connection): PipelineEdge | null => connection.source && connection.target
  ? { from: connection.source as PipelineBlockId, to: connection.target as PipelineBlockId, port: portOf(connection.targetHandle) }
  : null

type CardPosition = { readonly x: number; readonly y: number }

function Flow({ graph, outcomes, selected, summaries, onSelect, onMove, onConnect, onRemoveEdge, onRemoveBlock, onRefused, onDropBlock, onTidy }: PipelineCanvasProps) {
  const { screenToFlowPosition, fitView } = useReactFlow()
  const index = useMemo(() => indexGraph(graph), [graph])
  // A finger or the pointer always pans, as on the DAG editor. On a phone the canvas sits in a page that
  // scrolls, so the wheel and pinch stay with the page until the view is unlocked; on a desktop the canvas
  // fills the stage and the wheel zooms it.
  const isMobile = useIsMobile()
  const [lockOverride, setLockOverride] = useState<boolean | null>(null)
  const viewLocked = lockOverride ?? isMobile
  // A card being dragged moves here, not in the graph, so the workspace and its DuckDB run see one change when the drag ends.
  const [dragging, setDragging] = useState<ReadonlyMap<PipelineBlockId, CardPosition>>(() => new Map())

  const dropBlock = useCallback((event: DragEvent<HTMLDivElement>) => {
    const kind = event.dataTransfer.getData(BLOCK_DRAG_TYPE)
    if (kind === '') return
    event.preventDefault()
    const at = screenToFlowPosition({ x: event.clientX, y: event.clientY })
    onDropBlock(kind as PaletteKind, { x: at.x - CARD_WIDTH / 2, y: at.y - 20 })
  }, [onDropBlock, screenToFlowPosition])
  const allowDrop = useCallback((event: DragEvent<HTMLDivElement>) => {
    if (!event.dataTransfer.types.includes(BLOCK_DRAG_TYPE)) return
    event.preventDefault()
    event.dataTransfer.dropEffect = 'copy'
  }, [])

  const nodes = useMemo<CanvasNode[]>(() => graph.nodes.map((node) => ({
    id: node.id,
    type: 'block',
    position: dragging.get(node.id) ?? node.position,
    selected: node.id === selected,
    deletable: node.block.kind !== 'input' && node.block.kind !== 'output',
    data: { node, ports: inputPorts(index, node), outcome: outcomes.get(node.id), summary: summaries.get(node.id) ?? '' },
  })), [dragging, graph.nodes, index, outcomes, selected, summaries])
  const edges = useMemo<CanvasEdge[]>(() => graph.edges.map((edge) => ({
    id: edgeId(edge),
    source: edge.from,
    target: edge.to,
    sourceHandle: 'out',
    targetHandle: `in-${edge.port}`,
    data: { edge },
    style: EDGE_STYLE,
  })), [graph.edges])

  const applyNodeChanges = useCallback((changes: NodeChange<CanvasNode>[]) => {
    for (const change of changes) {
      if (change.type === 'position' && change.position !== undefined) {
        const id = change.id as PipelineBlockId
        if (change.dragging) setDragging((current) => new Map(current).set(id, change.position!))
        else { onMove(id, change.position); setDragging((current) => { if (!current.has(id)) return current; const next = new Map(current); next.delete(id); return next }) }
      } else if (change.type === 'select' && change.selected) onSelect(change.id as PipelineBlockId)
      else if (change.type === 'remove') onRemoveBlock(change.id as PipelineBlockId)
    }
  }, [onMove, onRemoveBlock, onSelect])
  const applyEdgeChanges = useCallback((changes: EdgeChange<CanvasEdge>[]) => {
    for (const change of changes) {
      if (change.type !== 'remove') continue
      const edge = graph.edges.find((candidate) => edgeId(candidate) === change.id)
      if (edge !== undefined) onRemoveEdge(edge)
    }
  }, [graph.edges, onRemoveEdge])
  const isConnectionValid = useCallback((connection: Connection | CanvasEdge): boolean => {
    const edge = toEdge({ source: connection.source, target: connection.target, sourceHandle: connection.sourceHandle ?? null, targetHandle: connection.targetHandle ?? null })
    return edge !== null && connectionRefusal(index, edge) === null
  }, [index])
  const connect = useCallback((connection: Connection) => {
    const edge = toEdge(connection)
    if (edge === null) return
    const refusal = connectionRefusal(index, edge)
    if (refusal !== null) { onRefused(refusal); return }
    onRefused(null)
    onConnect(edge)
  }, [index, onConnect, onRefused])
  // A drop on a handle that was refused says why; a drop on empty canvas says nothing.
  const connectEnd = useCallback((_event: MouseEvent | TouchEvent, state: FinalConnectionState) => {
    if (state.isValid !== false || state.toHandle === null || state.toHandle.type !== 'target' || state.fromNode === null) return
    const edge: PipelineEdge = { from: state.fromNode.id as PipelineBlockId, to: state.toHandle.nodeId as PipelineBlockId, port: portOf(state.toHandle.id) }
    const refusal = connectionRefusal(index, edge)
    if (refusal !== null) onRefused(refusal)
  }, [index, onRefused])
  const clearSelection = useCallback(() => onSelect(null), [onSelect])
  const tidy = useCallback(() => { onTidy(); window.setTimeout(() => void fitView({ ...FIT_VIEW, duration: 220 }), 30) }, [fitView, onTidy])

  return (
    <div className="h-full w-full" onDragOver={allowDrop} onDrop={dropBlock}>
        <ReactFlow<CanvasNode, CanvasEdge>
          nodes={nodes}
          edges={edges}
          nodeTypes={NODE_TYPES}
          onNodesChange={applyNodeChanges}
          onEdgesChange={applyEdgeChanges}
          onConnect={connect}
          isValidConnection={isConnectionValid}
          onConnectEnd={connectEnd}
          onPaneClick={clearSelection}
          connectionLineStyle={CONNECTION_LINE_STYLE}
          defaultEdgeOptions={DEFAULT_EDGE_OPTIONS}
          onInit={fitAfterLayout}
          zoomOnScroll={!viewLocked}
          preventScrolling={!viewLocked}
          minZoom={0.3}
          maxZoom={1.6}
          nodesConnectable
          proOptions={PRO_OPTIONS}
        >
          {BACKGROUND}
          <FlowControls fit={FIT_VIEW} fitLabel="Fit the pipeline">
            <button
              type="button"
              className={flowControl}
              title="Tidy: lay the blocks out again top to bottom in the order the arrows run, and fit the view. Dragged positions are replaced; nothing is rewired."
              aria-label="Tidy pipeline"
              onClick={tidy}
            >
              <Icon name="auto_awesome_mosaic" size={14} />
            </button>
            {isMobile && (
              <button
                type="button"
                className={flowControl}
                title={viewLocked ? 'Scrolling and pinching stay with the page. Tap to let them zoom the canvas.' : 'Scrolling and pinching zoom the canvas. Tap to give them back to the page.'}
                aria-label="Lock the view"
                aria-pressed={viewLocked}
                onClick={() => setLockOverride(!viewLocked)}
              >
                <Icon name={viewLocked ? 'lock' : 'lock_open'} size={14} />
              </button>
            )}
          </FlowControls>
        </ReactFlow>
    </div>
  )
}
