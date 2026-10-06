import { canvasMotion } from '@/lib/motion'
import { FloatingWindow } from '@/components/ui/FloatingWindow'
import { lazy, Suspense, useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { createStore } from 'zustand/vanilla'
import { useStore as useAppStore } from 'zustand'
import {
  Background,
  BackgroundVariant,
  BaseEdge,
  type ConnectionLineComponentProps,
  EdgeLabelRenderer,
  Handle,
  MarkerType,
  Position,
  ReactFlow,
  useConnection,
  useInternalNode,
  useNodesInitialized,
  useStore,
  useStoreApi,
  useNodesState,
  useReactFlow,
  useUpdateNodeInternals,
  useViewport,
  type Connection,
  type Edge,
  type EdgeProps,
  type FinalConnectionState,
  type InternalNode,
  type Node,
  type NodeProps,
} from '@xyflow/react'
import '@xyflow/react/dist/style.css'
import { XYHandle, isMouseEvent } from '@xyflow/system'
import { Icon } from '@/components/Icon'
import { Tooltip } from '@/components/ui/Tooltip'
import { iconControl, label, literal } from '@/components/ui/recipes'
import {
  describeDagEditProblem,
  inspectDagEdgeAddition,
  inspectDagEdgeReplacement,
  nameOfDagNode,
  type DagDocument,
  type DagEditProblem,
  type DagEdgeId,
  type DagNodeId,
  type EditableDag,
  type DirectedDagEdge,
} from '@/domain/dag'
import type { DiscoveryCandidate } from '@/domain/dagEvidence'
import { affectedDagEdges } from '@/domain/dagValidation'
import { assertNever, err, isNonEmpty, ok, type Result } from '@/domain/dop'
import { DEFAULT_CARD_SIZE, type DagCardSize, type DagLayoutOrientation } from './dagCanvasModel'
import {
  extendHeldPositions,
  layoutDag,
  movedRoute,
  separateCards,
  type DagLayout,
  type DagRoute,
  type LayoutPoint,
  type LayoutProblem,
} from './elkLayout'
import { dagCardSize } from './dagCardSize'
import { useTextMetricsVersion } from '@/lib/textMetrics'
import { roleWord, type DagCausalFlow } from '@/domain/dagFlow'
import type { InterventionOverlay } from '@/domain/intervention'
import { FlowControls, flowControl } from '@/components/flow/FlowControls'
import { useMediaQuery } from '@/lib/useMediaQuery'
import { dagPointerTarget, type ScreenTargetBox } from './dagPointerTarget'
import { placeRouteLabels, routeLabelText, type RouteLabel } from './routeLabels'

const SketchStroke = lazy(() => import('./SketchStroke'))
export type DrawingStyle = 'clean' | 'sketch'
export type VariableView = 'all' | 'connected'

export function visibleDagGraph(graph: EditableDag, view: VariableView): EditableDag {
  if (view === 'all') return graph
  const connected = new Set(graph.edges.flatMap((edge) => [edge.cause, edge.effect]))
  const nodes = graph.nodes.filter((node) => connected.has(node.id))
  return isNonEmpty(nodes) ? { ...graph, nodes } : graph
}

interface DagNodeData extends Record<string, unknown> {
  readonly name: string
  readonly kind: 'observed' | 'latent' | 'derived'
  /** The variable's place relative to the bound treatment and outcome; null when nothing is bound. */
  readonly role: string | null
  readonly evidenceRole: 'none' | 'source' | 'target' | 'both'
  /** During a drag, whether the arrow in hand may land on this card. */
  readonly droppable: boolean
  /** The node's part in the intervention being framed: set by do(), read as the answer, or neither. */
  readonly intervention: 'set' | 'read' | null
  /** The size every card in this graph is drawn at, fitted to the longest name (dagCardSize.ts). */
  readonly size: DagCardSize
  readonly drawing: DrawingStyle
}

export type CanvasNode = Node<DagNodeData, 'dagVariable'>

interface DagEdgeData extends Record<string, unknown> {
  readonly route: DagRoute
  /** Severed by the intervention: an arrow into the set node. */
  readonly cut: boolean
  readonly drawing: DrawingStyle
  readonly labelPlacement: RouteLabel | null
}

export type DerivedCanvasEdge = Edge<DagEdgeData, 'dagEdge'>
type CanvasEdge = Edge<DagEdgeData & { readonly edge: DirectedDagEdge }, 'dagEdge'>

const REST_TARGET_STYLE: React.CSSProperties = {
  opacity: 0,
  pointerEvents: 'none',
  width: 4,
  minWidth: 0,
  height: 4,
  minHeight: 0,
  border: 'none',
  background: 'transparent',
}

/**
 * A handle that is the whole card. Every arrow starts from one and, while an arrow is in hand, lands
 * on one, so there is nothing small to aim at; `touch-action: none` keeps a finger from scrolling the
 * page instead. The one that starts arrows stays mounted throughout, because every edge is anchored
 * on it and React Flow cannot draw an edge whose handle has gone.
 */
const FULL_CARD_STYLE: React.CSSProperties = {
  position: 'absolute',
  inset: 0,
  transform: 'none',
  width: '100%',
  minWidth: 0,
  height: '100%',
  minHeight: 0,
  borderRadius: 'inherit',
  border: 'none',
  background: 'transparent',
  opacity: 0,
  pointerEvents: 'all',
  touchAction: 'none',
}
const IDLE_SOURCE_STYLE: React.CSSProperties = { ...FULL_CARD_STYLE, cursor: 'crosshair' }
const HIDDEN_SOURCE_STYLE: React.CSSProperties = { ...FULL_CARD_STYLE, pointerEvents: 'none' }

/** The selector a card is dragged by: its name, since the rest of the card draws arrows. */
const CARD_GRIP = 'dag-card-grip'

export function DagVariableCard({
  id,
  data,
  selected,
  isConnectable,
  draggable,
}: NodeProps<CanvasNode>) {
  const connection = useConnection()
  const updateNodeInternals = useUpdateNodeInternals()
  const isTarget = connection.inProgress && connection.fromNode.id !== id
  const refused = isTarget && !data.droppable
  // An arrow lifted at its cause end travels from the effect's target handle, so it must land on a source handle.
  const dropType =
    connection.inProgress && connection.fromHandle.type === 'target' ? 'source' : 'target'
  const dropping = isTarget && !refused
  // The card's handles change shape with the gesture, and React Flow caches their bounds.
  useEffect(() => {
    updateNodeInternals(id)
  }, [connection.inProgress, dropping, dropType, id, updateNodeInternals])
  const evidenceColor =
    data.evidenceRole === 'source'
      ? 'var(--color-info)'
      : data.evidenceRole === 'target'
        ? 'var(--color-warn)'
        : data.evidenceRole === 'both'
          ? 'var(--color-signal)'
          : null
  return (
    <div
      className="group relative flex flex-col justify-center rounded-lg border bg-panel px-3 text-center transition-shadow"
      style={{
        width: data.size.width,
        height: data.size.height,
        borderColor:
          data.drawing === 'sketch'
            ? 'transparent'
            : refused
              ? 'var(--color-danger)'
              : isTarget
                ? 'var(--color-signal)'
                : data.intervention === 'set'
                  ? 'var(--color-signal)'
                  : data.intervention === 'read'
                    ? 'var(--color-info)'
                    : (evidenceColor ??
                      (data.kind === 'latent' ? 'var(--color-faint)' : 'var(--color-muted)')),
        borderStyle: data.kind === 'latent' ? 'dashed' : 'solid',
        boxShadow:
          selected || (isTarget && !refused) || evidenceColor !== null
            ? `0 0 0 2px var(--color-panel), 0 0 0 3px ${evidenceColor ?? 'var(--color-signal)'}`
            : undefined,
        opacity: refused ? 0.4 : undefined,
      }}
    >
      {data.drawing === 'sketch' && (
        <svg
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 overflow-visible"
          width={data.size.width}
          height={data.size.height}
          style={{
            color: refused
              ? 'var(--color-danger)'
              : data.intervention === 'set'
                ? 'var(--color-signal)'
                : data.intervention === 'read'
                  ? 'var(--color-info)'
                  : (evidenceColor ??
                    (data.kind === 'latent' ? 'var(--color-faint)' : 'var(--color-muted)')),
          }}
        >
          <Suspense
            fallback={
              <rect
                x={0}
                y={0}
                width={data.size.width}
                height={data.size.height}
                rx={8}
                fill="none"
                stroke="currentColor"
              />
            }
          >
            <SketchStroke
              id={`card:${id}`}
              path={`M 8 0 H ${data.size.width - 8} Q ${data.size.width} 0 ${data.size.width} 8 V ${data.size.height - 8} Q ${data.size.width} ${data.size.height} ${data.size.width - 8} ${data.size.height} H 8 Q 0 ${data.size.height} 0 ${data.size.height - 8} V 8 Q 0 0 8 0 Z`}
              style={{
                strokeWidth: 1,
                strokeDasharray: data.kind === 'latent' ? '3 3' : undefined,
              }}
            />
          </Suspense>
        </svg>
      )}
      <Handle
        id="card"
        type="source"
        position={Position.Right}
        isConnectableEnd={false}
        isConnectable={isConnectable}
        style={!isConnectable || connection.inProgress ? HIDDEN_SOURCE_STYLE : IDLE_SOURCE_STYLE}
        className="nodrag nopan"
        title={isConnectable ? `Drag to draw an arrow from ${data.name}` : undefined}
      />
      <span
        className={`${CARD_GRIP} relative z-10 line-clamp-3 ${draggable ? 'cursor-grab active:cursor-grabbing' : ''} whitespace-normal text-body font-medium text-ink [overflow-wrap:break-word]`}
        title={draggable ? `${data.name}, drag to move` : data.name}
      >
        {data.name}
      </span>
      <span
        className={label(
          `mt-0.5 truncate ${data.intervention === 'set' ? 'text-signal-text' : data.intervention === 'read' ? 'text-[var(--color-info)]' : 'text-faint'}`,
        )}
        title={data.role ?? undefined}
      >
        {data.role ?? (data.kind === 'latent' ? 'Unmeasured' : 'Observed')}
      </span>
      <Handle
        type="target"
        position={Position.Left}
        isConnectableStart={false}
        isConnectable={isConnectable && !refused}
        style={dropping && dropType === 'target' ? FULL_CARD_STYLE : REST_TARGET_STYLE}
      />
      {dropping && dropType === 'source' && (
        <Handle
          id="drop"
          type="source"
          position={Position.Right}
          isConnectableStart={false}
          style={FULL_CARD_STYLE}
          className="nodrag nopan"
        />
      )}
    </div>
  )
}

const NODE_TYPES = { dagVariable: DagVariableCard }

/** The size a card was given, for the moment before React Flow has measured it. */
const cardSizeOf = (node: InternalNode): DagCardSize =>
  (node.data as { readonly size?: DagCardSize }).size ?? DEFAULT_CARD_SIZE
const cardWidth = (node: InternalNode): number => node.measured.width ?? cardSizeOf(node).width
const cardHeight = (node: InternalNode): number => node.measured.height ?? cardSizeOf(node).height

const centreOf = (node: InternalNode) => ({
  x: node.internals.positionAbsolute.x + cardWidth(node) / 2,
  y: node.internals.positionAbsolute.y + cardHeight(node) / 2,
})

/** An arrow ends this far outside the card border, so the head never touches the card. */
const ANCHOR_CLEAR = 5.5
/** One fit for first paint and the buttons: a small graph never zooms past 100%. */
const FIT_VIEW = { padding: 0.18, maxZoom: 1 } as const
/** Above this many cards the role placement is skipped and ELK's layered layout stays: the role constraints grow with the graph and the drawing stops reading as a book diagram. */
const ROLE_LAYOUT_LIMIT = 60
/** The point just above the card's top edge, where a self-loop leaves and re-enters. */
const topPoint = (node: InternalNode) => {
  const centre = centreOf(node)
  return { x: centre.x, y: centre.y - cardHeight(node) / 2 - ANCHOR_CLEAR }
}
/**
 * Where an arrow toward a point leaves the card: on the border, along the line from the card's centre,
 * and the dot's clearance beyond it. Continuous, so the end slides along the border as the other end
 * moves instead of jumping between the four dots. The edge, the reconnect knobs and the connection
 * preview all read this one function, so they cannot disagree.
 */
const anchorPoint = (node: InternalNode, toward: { readonly x: number; readonly y: number }) => {
  const centre = centreOf(node)
  const halfWidth = cardWidth(node) / 2
  const halfHeight = cardHeight(node) / 2
  const dx = toward.x - centre.x
  const dy = toward.y - centre.y
  const length = Math.hypot(dx, dy)
  if (length === 0) return topPoint(node)
  const reach = Math.min(
    halfWidth / Math.max(Math.abs(dx), 1e-6),
    halfHeight / Math.max(Math.abs(dy), 1e-6),
  )
  return {
    x: centre.x + dx * reach + (dx / length) * ANCHOR_CLEAR,
    y: centre.y + dy * reach + (dy / length) * ANCHOR_CLEAR,
  }
}

const routePoints = (route: DagRoute, source: InternalNode, target: InternalNode) =>
  movedRoute(route, source.internals.positionAbsolute, target.internals.positionAbsolute)

/**
 * The preview ends where the committed arrow will: at the card's border facing the origin once a card
 * is under the pointer, else at the pointer, with a dot that turns to the verdict as soon as there is one.
 */
function DagConnectionLine({
  fromNode,
  toNode,
  toX,
  toY,
  connectionStatus,
  connectionLineStyle,
}: ConnectionLineComponentProps<CanvasNode>) {
  const pointer = { x: toX, y: toY }
  const from = anchorPoint(fromNode, toNode === null ? pointer : centreOf(toNode))
  const to = toNode === null ? pointer : anchorPoint(toNode, centreOf(fromNode))
  const tone =
    connectionStatus === 'valid'
      ? 'var(--color-ok)'
      : connectionStatus === 'invalid'
        ? 'var(--color-danger)'
        : 'var(--color-signal)'
  return (
    <g>
      <path
        d={`M ${from.x} ${from.y} L ${to.x} ${to.y}`}
        fill="none"
        style={{ ...connectionLineStyle, stroke: tone }}
      />
      <circle cx={to.x} cy={to.y} r={4} fill="var(--color-panel)" stroke={tone} strokeWidth={2} />
    </g>
  )
}

export function DagEdgePath({
  id,
  source,
  target,
  data,
  style,
  markerEnd,
  label: edgeLabel,
}: EdgeProps<DerivedCanvasEdge>) {
  const sourceNode = useInternalNode(source)
  const targetNode = useInternalNode(target)
  if (!sourceNode || !targetNode || data === undefined) return null
  const [first, ...rest] = routePoints(data.route, sourceNode, targetNode)
  const path = `M ${first.x} ${first.y} ${rest.map((p) => `L ${p.x} ${p.y}`).join(' ')}`
  const labelCentre = data.labelPlacement?.centre ?? first
  const sourcePosition = sourceNode.internals.positionAbsolute
  const targetPosition = targetNode.internals.positionAbsolute
  const labelAt = {
    x:
      labelCentre.x +
      (sourcePosition.x -
        data.route.sourcePosition.x +
        targetPosition.x -
        data.route.targetPosition.x) /
        2,
    y:
      labelCentre.y +
      (sourcePosition.y -
        data.route.sourcePosition.y +
        targetPosition.y -
        data.route.targetPosition.y) /
        2,
  }
  return (
    <>
      <BaseEdge
        id={id}
        path={path}
        style={data.drawing === 'sketch' ? { ...style, stroke: 'transparent' } : style}
        markerEnd={markerEnd as string | undefined}
      />
      {data.drawing === 'sketch' && (
        <Suspense fallback={<path d={path} fill="none" style={style} />}>
          <SketchStroke id={`edge:${id}`} path={path} style={{ ...style, color: style?.stroke }} />
        </Suspense>
      )}
      {typeof edgeLabel === 'string' && edgeLabel.length > 0 && (
        <EdgeLabelRenderer>
          <span
            className={literal(
              'pointer-events-none absolute rounded border border-hair bg-panel px-1.5 py-0.5 text-micro text-faint',
            )}
            data-route-label={data.labelPlacement?.kind}
            style={{ transform: `translate(-50%, -50%) translate(${labelAt.x}px, ${labelAt.y}px)` }}
          >
            {edgeLabel}
          </span>
        </EdgeLabelRenderer>
      )}
    </>
  )
}

const EDGE_TYPES = { dagEdge: DagEdgePath }

type Endpoint = 'cause' | 'effect'

/**
 * The selected arrow's toolbar and its two lift knobs. A knob hands the pointer to React Flow's own
 * connection engine with the far end of the arrow as the fixed handle, so lifting an end is the same
 * gesture as drawing a new arrow: same preview, same drop zones, same validity, same snap. React Flow's
 * stock anchors are mouse-only, which is why the knobs are ours and listen to touch as well.
 */
function EdgeActionBar({
  edge,
  coarse,
  onReverse,
  onConfound,
  onRemove,
  onLift,
  onLanded,
  isValidLanding,
}: {
  readonly edge: CanvasEdge
  readonly coarse: boolean
  readonly onReverse: () => void
  readonly onConfound: () => void
  readonly onRemove: () => void
  readonly onLift: (endpoint: Endpoint) => void
  readonly onLanded: (endpoint: Endpoint, connection: Connection) => void
  readonly isValidLanding: (endpoint: Endpoint, candidate: Connection | Edge) => boolean
}) {
  const store = useStoreApi()
  const sourceNode = useInternalNode(edge.source)
  const targetNode = useInternalNode(edge.target)
  if (!sourceNode || !targetNode || edge.data === undefined) return null
  const points = routePoints(edge.data.route, sourceNode, targetNode)
  const from = points[0]
  const to = points[points.length - 1] ?? from
  const vertical = Math.abs(to.y - from.y) > Math.abs(to.x - from.x)
  const lift =
    (endpoint: Endpoint) =>
    (event: React.MouseEvent<HTMLButtonElement> | React.TouchEvent<HTMLButtonElement>) => {
      const native = event.nativeEvent
      if (isMouseEvent(native) && native.button !== 0) return
      event.stopPropagation()
      const fixed =
        endpoint === 'effect'
          ? { nodeId: edge.source, type: 'source' as const }
          : { nodeId: edge.target, type: 'target' as const }
      const state = store.getState()
      onLift(endpoint)
      XYHandle.onPointerDown(native, {
        autoPanOnConnect: state.autoPanOnConnect,
        connectionMode: state.connectionMode,
        connectionRadius: state.connectionRadius,
        domNode: state.domNode,
        nodeLookup: state.nodeLookup,
        lib: state.lib,
        flowId: state.rfId,
        handleId: null,
        nodeId: fixed.nodeId,
        isTarget: fixed.type === 'target',
        edgeUpdaterType: fixed.type,
        updateConnection: state.updateConnection,
        panBy: state.panBy,
        cancelConnection: state.cancelConnection,
        isValidConnection: (candidate) => isValidLanding(endpoint, candidate),
        onConnect: (candidate) => onLanded(endpoint, candidate),
        onConnectStart: state.onConnectStart,
        onConnectEnd: state.onConnectEnd,
        getTransform: () => store.getState().transform,
        getFromHandle: () => store.getState().connection.fromHandle,
        dragThreshold: state.connectionDragThreshold,
        handleDomNode: event.currentTarget,
      })
    }
  const knob = `nodrag nopan pointer-events-auto absolute h-6 w-6 cursor-grab lift rounded-full border-2 border-signal bg-panel active:cursor-grabbing ${coarse ? "before:absolute before:-inset-2.5 before:content-['']" : ''}`
  return (
    <EdgeLabelRenderer>
      <>
        <div
          className="nodrag nopan pointer-events-auto absolute flex overflow-hidden float rounded-lg border border-hair bg-panel"
          style={{
            transform: `${vertical ? 'translate(24px, -50%)' : 'translate(-50%, -140%)'} translate(${(from.x + to.x) / 2}px, ${(from.y + to.y) / 2}px)`,
            zIndex: 10,
          }}
        >
          <button
            type="button"
            className={iconControl('quiet', 'rounded-none border-0')}
            title="Reverse arrow"
            aria-label="Reverse selected arrow"
            onClick={onReverse}
          >
            <Icon name="swap_horiz" size={14} />
          </button>
          <button
            type="button"
            className={iconControl('quiet', 'rounded-none border-0 border-l border-hair')}
            title="Replace with an unmeasured common cause"
            aria-label="Confound selected arrow"
            onClick={onConfound}
          >
            <Icon name="call_split" size={14} />
          </button>
          <button
            type="button"
            className={iconControl('danger', 'rounded-none border-0 border-l border-hair')}
            title="Remove arrow"
            aria-label="Remove selected arrow"
            onClick={onRemove}
          >
            <Icon name="delete" size={14} />
          </button>
        </div>
        <button
          type="button"
          aria-label="Reconnect cause endpoint"
          title="Drag to reconnect the cause"
          className={knob}
          style={{
            transform: `translate(-50%, -50%) translate(${from.x}px, ${from.y}px)`,
            zIndex: 11,
            touchAction: 'none',
          }}
          onMouseDown={lift('cause')}
          onTouchStart={lift('cause')}
        />
        <button
          type="button"
          aria-label="Reconnect effect endpoint"
          title="Drag to reconnect the effect"
          className={knob}
          style={{
            transform: `translate(-50%, -50%) translate(${to.x}px, ${to.y}px)`,
            zIndex: 11,
            touchAction: 'none',
          }}
          onMouseDown={lift('effect')}
          onTouchStart={lift('effect')}
        />
      </>
    </EdgeLabelRenderer>
  )
}

/** Grid pitch in flow units; the dots double their spacing when zoomed out so the grid never turns to moiré. */
const GRID_PITCH = 16
const MIN_SCREEN_PITCH = 12
export function DagGrid() {
  const { zoom } = useViewport()
  let gap = GRID_PITCH
  while (gap * zoom < MIN_SCREEN_PITCH) gap *= 2
  return (
    <Background variant={BackgroundVariant.Dots} color="var(--color-edge)" gap={gap} size={1} />
  )
}

/** Refits the view when the canvas box changes size, so a pane resize or a taller stage never leaves the graph cut off. */
export function RefitOnResize({
  host,
  layoutKey,
}: {
  readonly host: React.RefObject<HTMLDivElement | null>
  readonly layoutKey: string
}) {
  const { fitView } = useReactFlow<CanvasNode, CanvasEdge>()
  const initialized = useNodesInitialized()
  const nodeCount = useStore((store) => store.nodes.length)
  useEffect(() => {
    if (!initialized) return
    const timer = window.setTimeout(() => {
      void fitView({ ...FIT_VIEW, ...canvasMotion('zoom') })
    }, 60)
    return () => window.clearTimeout(timer)
  }, [fitView, initialized, layoutKey, nodeCount])
  useEffect(() => {
    const element = host.current
    if (element === null) return
    let last = { width: element.clientWidth, height: element.clientHeight }
    let timer = 0
    const observer = new ResizeObserver((entries) => {
      const box = entries[0]?.contentRect
      if (
        box === undefined ||
        (Math.abs(box.width - last.width) < 2 && Math.abs(box.height - last.height) < 2)
      )
        return
      last = { width: box.width, height: box.height }
      window.clearTimeout(timer)
      timer = window.setTimeout(() => {
        void fitView({ ...FIT_VIEW, ...canvasMotion('zoom') })
      }, 80)
    })
    observer.observe(element)
    return () => {
      observer.disconnect()
      window.clearTimeout(timer)
    }
  }, [fitView, host])
  return null
}

/** The drawing style is a reader preference shared by every graph canvas, not part of a project. */
const drawingStore = createStore<{ readonly style: DrawingStyle }>(() => ({ style: 'sketch' }))
const toggleDrawingStyle = () =>
  drawingStore.setState(({ style }) => ({ style: style === 'clean' ? 'sketch' : 'clean' }))

export function useDrawingStyle(): readonly [DrawingStyle, () => void] {
  return [useAppStore(drawingStore, (state) => state.style), toggleDrawingStyle]
}

/** How a graph canvas is viewed: in a floating window or not, laid out across or down, wheel locked or not. */
export interface CanvasView {
  readonly expanded: boolean
  readonly orientation: DagLayoutOrientation
  readonly viewLocked: boolean
  readonly toggleExpanded: () => void
  readonly toggleOrientation: () => void
  readonly toggleLock: () => void
}

/**
 * View state for one graph canvas. A narrow canvas runs the layout down the page instead of across it,
 * unless the reader has turned it. The canvas sits inside a scrolling stage, so a wheel over it is
 * ambiguous; locked is the safer default: the wheel scrolls the page, ⌘ or Ctrl with the wheel still
 * zooms, and the buttons always work. Esc returns an expanded canvas to the page.
 */
export function useCanvasView(
  host: React.RefObject<HTMLDivElement | null>,
  onOrientationChange?: () => void,
): CanvasView {
  const [expanded, setExpanded] = useState(false)
  const [fitted, setFitted] = useState<DagLayoutOrientation>('across')
  const [chosen, setChosen] = useState<DagLayoutOrientation | null>(null)
  const [viewLocked, setViewLocked] = useState(true)
  const orientation = chosen ?? fitted
  useEffect(() => {
    const element = host.current
    if (element === null) return
    const observer = new ResizeObserver(([entry]) => {
      setFitted(entry.contentRect.width < 600 ? 'down' : 'across')
    })
    observer.observe(element)
    return () => observer.disconnect()
  }, [host, expanded])
  return {
    expanded,
    orientation,
    viewLocked,
    toggleExpanded: () => setExpanded((open) => !open),
    toggleOrientation: () => {
      onOrientationChange?.()
      setChosen(orientation === 'across' ? 'down' : 'across')
    },
    toggleLock: () => setViewLocked((locked) => !locked),
  }
}

/**
 * The toolbar shared by the DAG editor and derived graphs. Tidy, arrow labels and disconnected
 * variables act on a drawable DAG, so a read-only graph leaves them out and gets a fit button instead.
 */
export function CanvasControls({
  view,
  tidy,
  labels,
  variables,
  expand = true,
}: {
  readonly view: CanvasView
  readonly expand?: boolean
  readonly tidy?: () => void
  readonly labels?: { readonly shown: boolean; readonly onToggle: () => void }
  readonly variables?: {
    readonly view: VariableView
    readonly disconnected: number
    readonly onToggle: () => void
  }
}) {
  const { fitView } = useReactFlow()
  const [drawing, toggleDrawing] = useDrawingStyle()
  const { orientation, viewLocked } = view
  const control = flowControl
  return (
    // Tidy fits the view, so the editor's strip carries no separate fit button.
    <FlowControls
      fit={tidy === undefined ? FIT_VIEW : undefined}
      fitLabel={tidy === undefined ? 'Fit the graph' : undefined}
    >
      <button
        type="button"
        className={control}
        title={
          orientation === 'across'
            ? 'Lay out the graph down the page'
            : 'Lay out the graph across the page'
        }
        aria-label={orientation === 'across' ? 'Lay out down' : 'Lay out across'}
        onClick={view.toggleOrientation}
      >
        <Icon
          name={orientation === 'across' ? 'rotate_90_degrees_cw' : 'rotate_90_degrees_ccw'}
          size={14}
        />
      </button>
      {variables !== undefined && (
        <button
          type="button"
          className={control}
          disabled={variables.disconnected === 0}
          title={`${variables.view === 'all' ? 'Hide disconnected variables' : `Show disconnected variables (${variables.disconnected})`}. Hidden variables remain in the DAG.`}
          aria-label={
            variables.view === 'all'
              ? 'Hide disconnected variables'
              : `Show disconnected variables (${variables.disconnected})`
          }
          aria-pressed={variables.view === 'connected'}
          onClick={variables.onToggle}
        >
          <Icon name={variables.view === 'all' ? 'visibility' : 'visibility_off'} size={16} />
        </button>
      )}
      <button
        type="button"
        className={control}
        title={drawing === 'clean' ? 'Use hand-drawn style' : 'Use clean drawing'}
        aria-label={drawing === 'clean' ? 'Use hand-drawn style' : 'Use clean drawing'}
        aria-pressed={drawing === 'sketch'}
        onClick={toggleDrawing}
      >
        <Icon name="draw" size={16} fill={drawing === 'sketch'} />
      </button>
      {tidy !== undefined && (
        <button
          type="button"
          className={control}
          title="Tidy: Lay out the variables again from left to right in causal order and fit the view. Any positions you dragged will be replaced, but the connections remain unchanged."
          aria-label="Tidy graph"
          onClick={() => {
            tidy()
            window.setTimeout(() => void fitView({ ...FIT_VIEW, ...canvasMotion('fit') }), 30)
          }}
        >
          <Icon name="auto_awesome_mosaic" size={14} />
        </button>
      )}
      {labels !== undefined && (
        <button
          type="button"
          className={control}
          title={
            labels.shown
              ? 'Hide the label on each arrow'
              : 'Show the label on each arrow: its lag, and whether it still needs a rationale'
          }
          aria-label={labels.shown ? 'Hide arrow labels' : 'Show arrow labels'}
          aria-pressed={labels.shown}
          onClick={labels.onToggle}
        >
          <Icon name={labels.shown ? 'label' : 'label_off'} size={14} />
        </button>
      )}
      <button
        type="button"
        className={control}
        title={
          viewLocked
            ? 'The scroll wheel scrolls the page. Click to enable graph zooming, or hold ⌘ or Ctrl while scrolling.'
            : 'The scroll wheel zooms the graph. Click to restore page scrolling.'
        }
        aria-label={viewLocked ? 'Let the wheel zoom' : 'Lock the view'}
        aria-pressed={viewLocked}
        onClick={view.toggleLock}
      >
        <Icon name={viewLocked ? 'lock' : 'lock_open'} size={14} />
      </button>
      {expand && !view.expanded && <ExpandControl view={view} />}
    </FlowControls>
  )
}

function ExpandControl({ view }: { readonly view: CanvasView }) {
  return (
    <button
      type="button"
      className={flowControl}
      title="Open the graph in a floating window"
      aria-label="Expand graph"
      onClick={view.toggleExpanded}
    >
      <Icon name="open_in_full" size={14} />
    </button>
  )
}

const evidenceColumns = (candidate: DiscoveryCandidate | null) =>
  candidate === null ? null : { source: candidate.source.column, target: candidate.target.column }

const dagNodeFromCanvas = (document: DagDocument, raw: string | null): DagNodeId | null =>
  raw === null ? null : (document.current.graph.nodes.find((node) => node.id === raw)?.id ?? null)

type ConnectionNotice =
  | { readonly kind: 'edge-added' }
  | { readonly kind: 'edge-reversed' }
  | { readonly kind: 'edge-reconnected' }
  | { readonly kind: 'edge-refused'; readonly problem: DagEditProblem }

/** The arrow in hand: a new one from a card, or an existing one lifted at one end. */
type Gesture =
  | { readonly kind: 'connect'; readonly from: DagNodeId }
  | { readonly kind: 'reconnect'; readonly edge: DagEdgeId; readonly endpoint: Endpoint }

const describeConnectionNotice = (notice: ConnectionNotice): string => {
  switch (notice.kind) {
    case 'edge-added':
      return 'An arrow has been added in a new graph revision. Record its rationale in the selected-edge panel.'
    case 'edge-reversed':
      return 'An arrow has been reversed in a new graph revision. Provide a rationale for the reversed relation.'
    case 'edge-reconnected':
      return 'An endpoint has been moved in a new graph revision. Provide a rationale for the reconnected relation.'
    case 'edge-refused':
      return describeDagEditProblem(notice.problem)
    default:
      return assertNever(notice)
  }
}

/** One canvas at a time, so the layer stack needs no per-instance id. */

const IDLE_HINT =
  'Draw an arrow by dragging from one card to another. Move a card by dragging its name. Select an arrow to reverse or remove it. To reconnect an arrow, drag either endpoint to another card.'

type LayoutState =
  | { readonly kind: 'pending'; readonly previous: DagLayout | null }
  | { readonly kind: 'ready'; readonly value: DagLayout }
  | {
      readonly kind: 'failed'
      readonly problem: LayoutProblem
      readonly previous: DagLayout | null
    }

const retainedLayout = (state: LayoutState): DagLayout | null => {
  switch (state.kind) {
    case 'ready':
      return state.value
    case 'pending':
    case 'failed':
      return state.previous
    default:
      return assertNever(state)
  }
}

async function placeAndRoute(
  graph: EditableDag,
  automatic: DagLayout,
  placement:
    | { readonly kind: 'role'; readonly flow: DagCausalFlow }
    | { readonly kind: 'held'; readonly held: ReadonlyMap<string, LayoutPoint> },
  orientation: DagLayoutOrientation,
  size: DagCardSize,
): Promise<Result<DagLayout, LayoutProblem>> {
  try {
    const { routeFixedDag } = await import('./fixedRouting')
    const byRoles =
      placement.kind === 'role'
        ? (await import('./roleLayout')).placeByRole(graph, placement.flow, orientation, size)
        : null
    const positions =
      byRoles === null
        ? await extendHeldPositions(graph, placement.kind === 'held' ? placement.held : new Map(), size)
        : byRoles.ok
          ? await separateCards(byRoles.value, size)
          : byRoles
    return positions.ok ? await routeFixedDag(graph, positions.value, size, automatic) : positions
  } catch (cause) {
    return err({ kind: 'engine', message: cause instanceof Error ? cause.message : String(cause) })
  }
}

export async function studyLayout(
  graph: EditableDag,
  flow: DagCausalFlow | null,
  orientation: DagLayoutOrientation,
  size: DagCardSize,
): Promise<Result<DagLayout, LayoutProblem>> {
  const automatic = await layoutDag(graph, orientation, size)
  if (!automatic.ok || flow === null || graph.nodes.length > ROLE_LAYOUT_LIMIT) return automatic
  const placed = await placeAndRoute(graph, automatic.value, { kind: 'role', flow }, orientation, size)
  return placed.ok ? placed : automatic
}

export const canvasModel = (
  document: DagDocument,
  candidate: DiscoveryCandidate | null,
  flow: DagCausalFlow | null,
  intervention: InterventionOverlay | null,
  layout: DagLayout | null,
  labelsShown: boolean,
  drawing: DrawingStyle,
): {
  readonly nodes: CanvasNode[]
  readonly edges: CanvasEdge[]
  /** The size every card is drawn at; a change relays the whole drawing, since kept positions were fitted to the old size. */
  readonly size: DagCardSize
} => {
  const size = dagCardSize(document.current.graph.nodes.map((node) => node.name))
  const placements = layout?.nodes
  const labelPlacements =
    layout === null
      ? new Map<DagEdgeId, RouteLabel>()
      : placeRouteLabels(document.current.graph, layout, size)
  const highlighted = evidenceColumns(candidate)
  const validation = document.current.validation
  const problemEdges = new Set(
    validation.structure.kind === 'invalid'
      ? validation.structure.issues.flatMap(affectedDagEdges)
      : [],
  )
  const latentNodes = new Set(
    document.current.graph.nodes.filter((node) => node.kind === 'latent').map((node) => node.id),
  )
  const edges: CanvasEdge[] = document.current.graph.edges.flatMap((edge): CanvasEdge[] => {
    const route = layout?.routes.get(edge.id)
    if (route === undefined) return [] // A new arrow waits for the engine, not an invented route.
    const invalid = problemEdges.has(edge.id)
    const unstated = edge.support.kind === 'unstated'
    const edgeFlow = flow?.edges.get(edge.id) ?? null
    // Graph surgery: an arrow into the set node is severed, so it is drawn faint and broken.
    const cut = intervention !== null && edge.effect === intervention.set
    // With a binding, the flow overlay distinguishes causal paths, open biasing paths, and other edges.
    const stroke = cut
      ? 'var(--color-faint)'
      : invalid
        ? 'var(--color-danger)'
        : edgeFlow !== null
          ? edgeFlow.biasing
            ? 'var(--color-danger)'
            : edgeFlow.causal
              ? 'var(--color-ok)'
              : 'var(--color-muted)'
          : unstated
            ? 'var(--color-warn)'
            : edge.evidence.length > 0
              ? 'var(--color-info)'
              : 'var(--color-muted)'
    const flowWords =
      edgeFlow === null
        ? ''
        : edgeFlow.biasing
          ? '; lies on an open biasing path'
          : edgeFlow.causal
            ? '; lies on a directed causal path'
            : '; not on an active treatment–outcome path'
    const edgeLabel = routeLabelText(edge)
    return [
      {
        id: edge.id,
        type: 'dagEdge',
        source: edge.cause,
        target: edge.effect,
        data: { edge, route, cut, drawing, labelPlacement: labelPlacements.get(edge.id) ?? null },
        label: cut ? 'cut by do()' : labelsShown ? edgeLabel : '',
        // User-space units keep the head 6px long whatever the stroke width, so selection does not swell it.
        markerEnd: {
          type: MarkerType.ArrowClosed,
          color: stroke,
          markerUnits: 'userSpaceOnUse',
          width: 24,
          height: 24,
          strokeWidth: 1,
        },
        style: {
          stroke,
          strokeWidth: 1.8,
          ...(cut
            ? { strokeDasharray: '2 5', opacity: 0.7 }
            : edge.timing.kind === 'lagged'
              ? { strokeDasharray: '7 4' }
              : latentNodes.has(edge.cause)
                ? { strokeDasharray: '3 3' }
                : {}),
        },
        ariaLabel: `${nameOfDagNode(document, edge.cause)} causes ${nameOfDagNode(document, edge.effect)}${edge.timing.kind === 'lagged' ? ` at lag ${edge.timing.lag}` : ' contemporaneously'}${unstated ? '; rationale not yet recorded' : ''}${flowWords}${cut ? '; cut by the intervention' : ''}`,
      },
    ]
  })
  return {
    size,
    nodes: document.current.graph.nodes.flatMap((node): CanvasNode[] => {
      const placed = placements?.get(node.id)
      if (placed === undefined) return []
      const source = node.kind === 'observed' && highlighted?.source === node.column
      const target = node.kind === 'observed' && highlighted?.target === node.column
      const evidenceRole: DagNodeData['evidenceRole'] =
        source && target ? 'both' : source ? 'source' : target ? 'target' : 'none'
      return [
        {
          id: node.id,
          type: 'dagVariable',
          // The engine and the card share known dimensions, including immediately after portal remount.
          initialWidth: size.width,
          initialHeight: size.height,
          position: { x: placed.x, y: placed.y },
          data: {
            name: node.name,
            kind: node.kind,
            role:
              intervention !== null && node.id === intervention.set
                ? `do(${node.name})`
                : intervention !== null && node.id === intervention.read
                  ? 'read'
                  : flow === null
                    ? null
                    : roleWord(flow.roles.get(node.id) ?? { kind: 'unrelated' }),
            evidenceRole,
            droppable: true,
            intervention:
              intervention === null
                ? null
                : node.id === intervention.set
                  ? 'set'
                  : node.id === intervention.read
                    ? 'read'
                    : null,
            size,
            drawing,
          },
          ariaLabel: `${node.kind === 'latent' ? 'Unmeasured' : 'Observed'} variable: ${node.name}`,
        },
      ]
    }),
    edges,
  }
}

export function DagCanvas({
  document,
  selectedEvidence,
  selectedEdge,
  onConnectionDrawn,
  onEdgeReconnected,
  onEdgeConfounded,
  onEdgeRemoved,
  onEdgeSelected,
  flow,
  intervention = null,
}: {
  readonly document: DagDocument
  readonly selectedEvidence: DiscoveryCandidate | null
  /** The bound treatment and outcome's flow overlay; null until both are chosen. */
  readonly flow: DagCausalFlow | null
  /** The intervention being framed on the Intervene tab; the canvas draws the surgery while it is open. */
  readonly intervention?: InterventionOverlay | null
  readonly selectedEdge: DagEdgeId | null
  /** Applies the drawn arrow as a revision; returns the refusal when the domain rejects it. */
  readonly onConnectionDrawn: (cause: DagNodeId, effect: DagNodeId) => DagEditProblem | null
  readonly onEdgeReconnected: (
    edge: DagEdgeId,
    cause: DagNodeId,
    effect: DagNodeId,
  ) => DagEditProblem | null
  readonly onEdgeConfounded: (edge: DagEdgeId) => void
  readonly onEdgeRemoved: (edge: DagEdgeId) => void
  readonly onEdgeSelected: (edge: DagEdgeId | null) => void
}) {
  const hostRef = useRef<HTMLDivElement>(null)
  // Cards someone has dragged keep their place; every other card follows the layout as the graph changes.
  const placedByHand = useRef<Set<string>>(new Set())
  const view = useCanvasView(hostRef, () => placedByHand.current.clear())
  const { expanded, orientation, viewLocked } = view
  // The card size is measured from the names, so the model re-runs once the document's fonts have loaded.
  const metricsVersion = useTextMetricsVersion()
  const [labelsShown, setLabelsShown] = useState(false)
  const [drawing] = useDrawingStyle()
  const [variableView, setVariableView] = useState<VariableView>('all')
  const connectedIds = useMemo(
    () => new Set(document.current.graph.edges.flatMap((edge) => [edge.cause, edge.effect])),
    [document.current.graph.edges],
  )
  const disconnectedCount = document.current.graph.nodes.filter(
    (node) => !connectedIds.has(node.id),
  ).length
  const visibleGraph = useMemo(
    () => visibleDagGraph(document.current.graph, variableView),
    [document.current.graph, variableView],
  )
  const [layoutState, setLayoutState] = useState<LayoutState>({ kind: 'pending', previous: null })
  const [layoutAttempt, setLayoutAttempt] = useState(0)
  const size = useMemo(
    () => dagCardSize(document.current.graph.nodes.map((node) => node.name)),
    [document.current.graph.nodes, metricsVersion],
  )
  // A chosen treatment and outcome give every card a role, and the roles decide where cards go.
  const study = flow === null ? '' : `${flow.treatment}\u0000${flow.outcome}`
  // Only geometry changes trigger layout. Selecting evidence, a query or labels does not move cards.
  const layoutKey = JSON.stringify([
    document.id,
    visibleGraph.nodes.map((n) => n.id),
    visibleGraph.edges.map((e) => [e.id, e.cause, e.effect, routeLabelText(e)]),
    orientation,
    size.width,
    size.height,
    study,
  ])
  useEffect(() => {
    let current = true
    setLayoutState((state) => ({ kind: 'pending', previous: retainedLayout(state) }))
    const preserve =
      previousBinding.current ===
        `${document.id}\u0000${orientation}\u0000${size.width}x${size.height}\u0000${study}` &&
      placedByHand.current.size > 0
    const held = new Map(latestNodes.current.map((node) => [node.id, node.position]))
    const byRole = flow !== null && !preserve && visibleGraph.nodes.length <= ROLE_LAYOUT_LIMIT
    void layoutDag(visibleGraph, orientation, size).then(async (result) => {
      // ELK still runs first: its self-loop shapes are kept when another step places the cards.
      if (result.ok && (preserve || byRole)) {
        const automatic = result.value
        const placed = await placeAndRoute(
          visibleGraph,
          automatic,
          byRole && flow !== null ? { kind: 'role', flow } : { kind: 'held', held },
          orientation,
          size,
        )
        // A role placement that fails leaves ELK's drawing in place rather than an empty canvas.
        result = placed.ok || !byRole ? placed : ok(automatic)
      }
      if (!current) return
      setLayoutState((state) =>
        result.ok
          ? { kind: 'ready', value: result.value }
          : { kind: 'failed', problem: result.error, previous: retainedLayout(state) },
      )
    })
    return () => {
      current = false
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- layoutKey contains the engine's entire geometry input
  }, [layoutKey, layoutAttempt])
  const layout = retainedLayout(layoutState)
  const model = useMemo(
    () => canvasModel(document, selectedEvidence, flow, intervention, layout, labelsShown, drawing),
    [document, flow, intervention, layout, selectedEvidence, labelsShown, drawing],
  )
  const [nodes, setNodes, onNodesChange] = useNodesState<CanvasNode>(model.nodes)
  // The gesture is read in React Flow's callbacks, which fire from listeners bound at pointer-down, so a ref carries it as well as state.
  const [gesture, setGesture] = useState<Gesture | null>(null)
  const gestureRef = useRef<Gesture | null>(null)
  const beginGesture = (next: Gesture | null) => {
    gestureRef.current = next
    setGesture(next)
  }
  const [connectionNotice, setConnectionNotice] = useState<ConnectionNotice | null>(null)
  // A refusal describes one gesture; once the document moves on (an arrow added through the form, an undo) it is stale.
  useEffect(() => {
    setConnectionNotice((notice) =>
      notice !== null && notice.kind === 'edge-refused' ? null : notice,
    )
  }, [document])
  const selectedCanvasEdge = model.edges.find((edge) => edge.id === selectedEdge) ?? null

  const latestNodes = useRef(nodes)
  latestNodes.current = nodes
  const reducedMotion = useMediaQuery('(prefers-reduced-motion: reduce)')
  const motion = useRef<number | null>(null)
  const stopMotion = useCallback(() => {
    if (motion.current !== null) cancelAnimationFrame(motion.current)
    motion.current = null
  }, [])
  // A finished rearrangement refits the view, so cards that moved out of sight come back into it.
  const [rearrangements, setRearrangements] = useState(0)
  /**
   * Moves the cards to their new places. React Flow draws arrows from the positions in its store, so
   * the positions themselves are stepped frame by frame; a CSS transition would leave the arrows behind.
   */
  const arrange = useCallback(
    (targets: readonly CanvasNode[]) => {
      stopMotion()
      const starts = new Map(latestNodes.current.map((node) => [node.id, node.position]))
      const moved = targets.some((node) => {
        const from = starts.get(node.id)
        return (
          from !== undefined &&
          (Math.abs(from.x - node.position.x) > 0.5 || Math.abs(from.y - node.position.y) > 0.5)
        )
      })

      const { duration } = canvasMotion('arrange')
      if (!moved || reducedMotion || duration === 0) {
        setNodes([...targets])
        if (moved) setRearrangements((count) => count + 1)
        return
      }

      const began = performance.now()
      const step = (now: number) => {
        const t = Math.min(1, (now - began) / duration)
        const eased = 1 - Math.pow(1 - t, 3)
        setNodes(
          targets.map((node) => {
            const from = starts.get(node.id) ?? node.position
            return {
              ...node,
              position: {
                x: from.x + (node.position.x - from.x) * eased,
                y: from.y + (node.position.y - from.y) * eased,
              },
            }
          }),
        )
        if (t < 1) {
          motion.current = requestAnimationFrame(step)
          return
        }
        motion.current = null
        setRearrangements((count) => count + 1)
      }
      motion.current = requestAnimationFrame(step)
    },
    [reducedMotion, setNodes, stopMotion],
  )
  useEffect(() => stopMotion, [stopMotion])

  // Orientation, card-size and study changes reset manual placement: each one lays the whole drawing out again.
  const bindingKey = `${document.id}\u0000${orientation}\u0000${model.size.width}x${model.size.height}\u0000${study}`
  const previousBinding = useRef(bindingKey)
  useEffect(() => {
    if (previousBinding.current !== bindingKey) {
      previousBinding.current = bindingKey
      placedByHand.current.clear()
    }

    const positions = new Map(latestNodes.current.map((node) => [node.id, node.position]))
    const kept = (id: string) => (placedByHand.current.size > 0 ? positions.get(id) : undefined)
    arrange(
      model.nodes.map((next) => {
        const held = kept(next.id)
        return held === undefined ? next : { ...next, position: held }
      }),
    )
  }, [arrange, bindingKey, model.nodes, model.size])

  const tidy = () => {
    placedByHand.current.clear()
    setLayoutAttempt((attempt) => attempt + 1)
    arrange(model.nodes)
  }
  /** Notes every card the pointer moves; a drag also stops any rearrangement still in motion. */
  const nodesChanged = useCallback(
    (changes: Parameters<typeof onNodesChange>[0]) => {
      for (const change of changes) {
        if (change.type === 'position' && change.dragging === true) {
          stopMotion()
          placedByHand.current.add(change.id)
        }
      }
      onNodesChange(changes)
      if (changes.some((change) => change.type === 'position' && change.dragging === false))
        setLayoutAttempt((attempt) => attempt + 1)
    },
    [onNodesChange, stopMotion],
  )
  // A finger is imprecise: connections snap from further away and a tap on one dot then another also connects.
  const coarse = useMediaQuery('(pointer: coarse)')
  const additionAllowed = useCallback(
    (cause: DagNodeId | null, effect: DagNodeId | null): boolean =>
      inspectDagEdgeAddition(document, cause, effect).ok,
    [document],
  )
  const connect = (connection: Connection) => {
    const cause = dagNodeFromCanvas(document, connection.source)
    const effect = dagNodeFromCanvas(document, connection.target)
    if (cause !== null && effect !== null) addArrow(cause, effect)
  }
  const addArrow = (cause: DagNodeId, effect: DagNodeId) => {
    const problem = onConnectionDrawn(cause, effect)
    setConnectionNotice(
      problem === null ? { kind: 'edge-added' } : { kind: 'edge-refused', problem },
    )
  }
  const replacementFor = useCallback(
    (edgeId: DagEdgeId, endpoint: Endpoint, node: DagNodeId) => {
      const edge = document.current.graph.edges.find((candidate) => candidate.id === edgeId)
      if (edge === undefined) return null
      const cause = endpoint === 'cause' ? node : edge.cause
      const effect = endpoint === 'effect' ? node : edge.effect
      return {
        edge,
        cause,
        effect,
        inspected: inspectDagEdgeReplacement(document, edgeId, cause, effect, edge.timing),
      }
    },
    [document],
  )
  /** Moves one end of an arrow to a card; landing it back where it was is not a revision. */
  const moveEnd = (edgeId: DagEdgeId, endpoint: Endpoint, node: DagNodeId) => {
    const replacement = replacementFor(edgeId, endpoint, node)
    if (replacement === null) return
    if (!replacement.inspected.ok) {
      setConnectionNotice({ kind: 'edge-refused', problem: replacement.inspected.error })
      return
    }
    if (
      replacement.cause === replacement.edge.cause &&
      replacement.effect === replacement.edge.effect
    ) {
      setConnectionNotice(null)
      return
    }
    const problem = onEdgeReconnected(edgeId, replacement.cause, replacement.effect)
    setConnectionNotice(
      problem === null ? { kind: 'edge-reconnected' } : { kind: 'edge-refused', problem },
    )
  }
  /** The card a landed connection names for the lifted end: the far end is the fixed one. */
  const landedNode = (endpoint: Endpoint, candidate: Connection | Edge): DagNodeId | null =>
    dagNodeFromCanvas(document, endpoint === 'effect' ? candidate.target : candidate.source)
  /** Whether the arrow in hand may land on a card. */
  const landable = (active: Gesture, node: DagNodeId): boolean => {
    switch (active.kind) {
      case 'connect':
        return additionAllowed(active.from, node)
      case 'reconnect':
        return replacementFor(active.edge, active.endpoint, node)?.inspected.ok === true
      default:
        return assertNever(active)
    }
  }
  /** Lands the arrow in hand on a card, or reports why it may not. */
  const land = (active: Gesture, node: DagNodeId) => {
    switch (active.kind) {
      case 'connect':
        addArrow(active.from, node)
        return
      case 'reconnect':
        moveEnd(active.edge, active.endpoint, node)
        return
      default:
        return assertNever(active)
    }
  }
  /**
   * A drop that React Flow did not resolve still lands on the nearest card within reach; a drop far
   * from every card is a change of mind and says nothing.
   */
  const connectionEnded = (event: MouseEvent | TouchEvent, connection: FinalConnectionState) => {
    const active = gestureRef.current
    beginGesture(null)
    if (active === null || connection.isValid === true) return
    const point = 'changedTouches' in event ? event.changedTouches[0] : event
    if (point === undefined) {
      setConnectionNotice(null)
      return
    }
    // Letting go on the card the arrow came from is a change of mind, not an attempt at a self-loop.
    const origin = active.kind === 'connect' ? active.from : null
    const boxes = cardBoxes((node) => landable(active, node)).filter((box) => box.node !== origin)
    const target = dagPointerTarget({ x: point.clientX, y: point.clientY }, boxes, coarse ? 56 : 28)
    switch (target.kind) {
      case 'eligible-node':
        land(active, target.node)
        return
      case 'refused-node':
        land(active, target.node)
        return
      case 'none':
        setConnectionNotice(null)
        return
      default:
        return assertNever(target)
    }
  }
  const reverse = (edge: CanvasEdge) => {
    const causalEdge = edge.data?.edge
    if (causalEdge === undefined) return
    const problem = onEdgeReconnected(causalEdge.id, causalEdge.effect, causalEdge.cause)
    setConnectionNotice(
      problem === null ? { kind: 'edge-reversed' } : { kind: 'edge-refused', problem },
    )
  }
  /** The graph's variables by id; cards are matched to them on every frame of a rearrangement. */
  const graphNodes = useMemo(
    () =>
      new Map<string, DagNodeId>(document.current.graph.nodes.map((node) => [node.id, node.id])),
    [document.current.graph.nodes],
  )
  /** Every card's screen box, marked with whether the gesture in hand may land on it. */
  const cardBoxes = useCallback(
    (eligible: (node: DagNodeId) => boolean): ScreenTargetBox[] => {
      const host = hostRef.current
      if (host === null) return []
      const boxes: ScreenTargetBox[] = []
      for (const element of host.querySelectorAll<HTMLElement>('.react-flow__node')) {
        const node =
          element.dataset.id === undefined ? undefined : graphNodes.get(element.dataset.id)
        if (node === undefined) continue
        const rect = element.getBoundingClientRect()
        boxes.push({
          node,
          left: rect.left,
          top: rect.top,
          right: rect.right,
          bottom: rect.bottom,
          eligible: eligible(node),
        })
      }
      return boxes
    },
    [graphNodes],
  )

  const displayedNodes = nodes.map((node): CanvasNode => {
    const domainNode = graphNodes.get(node.id)
    if (domainNode === undefined) return node
    // The card body draws arrows, so the name is what moves it.
    const dragged = { ...node, dragHandle: `.${CARD_GRIP}` }
    if (gesture !== null)
      return { ...dragged, data: { ...node.data, droppable: landable(gesture, domainNode) } }
    return dragged
  })

  const keyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement
    if (
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement ||
      target instanceof HTMLSelectElement ||
      target.isContentEditable
    )
      return
    if (selectedCanvasEdge !== null && event.key.toLowerCase() === 'r') {
      reverse(selectedCanvasEdge)
      event.preventDefault()
    } else if (selectedCanvasEdge !== null && event.key.toLowerCase() === 'u') {
      const edge = selectedCanvasEdge.data?.edge
      if (edge !== undefined) onEdgeConfounded(edge.id)
      event.preventDefault()
    } else if (
      selectedCanvasEdge !== null &&
      (event.key === 'Delete' || event.key === 'Backspace')
    ) {
      const edge = selectedCanvasEdge.data?.edge
      if (edge === undefined) return
      onEdgeRemoved(edge.id)
      event.preventDefault()
    } else if (event.key === 'Escape') {
      onEdgeSelected(null)
    }
  }
  const refusedNotice = connectionNotice?.kind === 'edge-refused'
  const canvas = (
    <div
      ref={hostRef}
      className={
        expanded
          ? 'relative flex min-h-0 flex-1 flex-col overflow-hidden'
          : 'relative flex min-h-[22rem] flex-1 flex-col overflow-hidden rounded-xl border border-edge bg-well @max-md/panel:min-h-[26rem]'
      }
      aria-label="Causal DAG editor"
      onKeyDown={keyDown}
    >
      <div className="relative min-h-0 flex-1">
        <div className="absolute inset-0">
          <ReactFlow<CanvasNode, CanvasEdge>
            nodes={displayedNodes}
            edges={model.edges.map((edge) =>
              edge.id === selectedEdge
                ? { ...edge, selected: true, style: { ...edge.style, strokeWidth: 2.8 } }
                : selectedEdge === null
                  ? { ...edge, selected: false }
                  : { ...edge, selected: false, style: { ...edge.style, opacity: 0.3 } },
            )}
            nodeTypes={NODE_TYPES}
            edgeTypes={EDGE_TYPES}
            onNodesChange={nodesChanged}
            onConnect={connect}
            onConnectStart={(_event, params) => {
              setConnectionNotice(null)
              // A lifted arrow end registers its gesture before the engine starts; a new arrow registers here.
              if (gestureRef.current?.kind === 'reconnect') return
              const from = dagNodeFromCanvas(document, params.nodeId)
              if (from !== null) beginGesture({ kind: 'connect', from })
            }}
            onConnectEnd={connectionEnded}
            isValidConnection={(connection) =>
              additionAllowed(
                dagNodeFromCanvas(document, connection.source),
                dagNodeFromCanvas(document, connection.target),
              )
            }
            onEdgeClick={(_, edge) => {
              const id = edge.data?.edge.id
              if (id !== undefined) onEdgeSelected(id)
            }}
            onEdgeDoubleClick={(_, edge) => {
              const canvasEdge = model.edges.find((candidate) => candidate.id === edge.id)
              if (canvasEdge !== undefined) reverse(canvasEdge)
            }}
            onPaneClick={() => onEdgeSelected(null)}
            connectionLineComponent={DagConnectionLine}
            connectionLineStyle={{ stroke: 'var(--color-signal)', strokeWidth: 2 }}
            fitView
            fitViewOptions={FIT_VIEW}
            minZoom={0.3}
            maxZoom={2}
            deleteKeyCode={null}
            nodesConnectable
            edgesReconnectable={false}
            connectionRadius={coarse ? 64 : 28}
            connectOnClick={false}
            connectionDragThreshold={coarse ? 8 : 1}
            nodeDragThreshold={coarse ? 8 : 1}
            // Raised SVG edges intercept the HTML reconnect grips drawn over them.
            // Selection is shown by stroke width and colour, not by changing the hit-test layer.
            elevateEdgesOnSelect={false}
            zoomOnDoubleClick={false}
            zoomOnScroll={!viewLocked}
            preventScrolling={!viewLocked}
            proOptions={{ hideAttribution: true }}
          >
            <DagGrid />
            {selectedCanvasEdge !== null && (
              <EdgeActionBar
                edge={selectedCanvasEdge}
                coarse={coarse}
                onLift={(endpoint) => {
                  const edge = selectedCanvasEdge.data?.edge
                  if (edge === undefined) return
                  setConnectionNotice(null)
                  beginGesture({ kind: 'reconnect', edge: edge.id, endpoint })
                }}
                onLanded={(endpoint, connection) => {
                  const edge = selectedCanvasEdge.data?.edge
                  const node = landedNode(endpoint, connection)
                  if (edge !== undefined && node !== null) moveEnd(edge.id, endpoint, node)
                }}
                isValidLanding={(endpoint, candidate) => {
                  const edge = selectedCanvasEdge.data?.edge
                  const node = landedNode(endpoint, candidate)
                  return (
                    edge !== undefined &&
                    node !== null &&
                    replacementFor(edge.id, endpoint, node)?.inspected.ok === true
                  )
                }}
                onReverse={() => reverse(selectedCanvasEdge)}
                onConfound={() => {
                  const edge = selectedCanvasEdge.data?.edge
                  if (edge !== undefined) onEdgeConfounded(edge.id)
                }}
                onRemove={() => {
                  const edge = selectedCanvasEdge.data?.edge
                  if (edge !== undefined) onEdgeRemoved(edge.id)
                }}
              />
            )}
            <CanvasControls
              view={view}
              tidy={tidy}
              labels={{ shown: labelsShown, onToggle: () => setLabelsShown((shown) => !shown) }}
              variables={{
                view: variableView,
                disconnected: disconnectedCount,
                onToggle: () => {
                  placedByHand.current.clear()
                  setVariableView((current) => (current === 'all' ? 'connected' : 'all'))
                  setRearrangements((count) => count + 1)
                },
              }}
            />
            <RefitOnResize host={hostRef} layoutKey={`${bindingKey}\u0000${rearrangements}`} />
          </ReactFlow>
        </div>
      </div>
      <div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 border-t border-hair bg-panel px-3 py-1.5">
        <div className="flex min-w-0 flex-1 basis-[16rem] items-center gap-2">
          <Tooltip text={IDLE_HINT}>
            <button
              type="button"
              className="grid h-6 w-6 shrink-0 place-items-center rounded-md text-faint transition-colors hover:text-ink"
              aria-label="How to draw and edit arrows"
            >
              <Icon name="help" size={15} />
            </button>
          </Tooltip>
          <p
            role="status"
            className={`m-0 min-w-0 flex-1 text-label ${refusedNotice ? 'text-warn' : 'text-ink'}`}
          >
            {layoutState.kind === 'failed'
              ? 'The graph could not be laid out. Try Tidy again.'
              : connectionNotice === null
                ? ''
                : describeConnectionNotice(connectionNotice)}
          </p>
        </div>
        {flow !== null && (
          <p
            className="m-0 flex flex-wrap items-center gap-x-3 gap-y-1 text-label text-faint"
            aria-label="Arrow legend"
          >
            <span className="flex items-center gap-1.5">
              <span aria-hidden className="h-[2px] w-4 rounded bg-ok" />
              directed causal path
            </span>
            <span className="flex items-center gap-1.5">
              <span aria-hidden className="h-[2px] w-4 rounded bg-danger" />
              open biasing path
            </span>
            <span className="flex items-center gap-1.5">
              <span aria-hidden className="h-[2px] w-4 rounded bg-muted" />
              other relation
            </span>
          </p>
        )}
        {intervention !== null && (
          <p
            className="m-0 flex flex-wrap items-center gap-x-3 gap-y-1 text-label text-faint"
            aria-label="Intervention legend"
          >
            <span className="flex items-center gap-1.5">
              <span
                aria-hidden
                className="h-[2px] w-4 rounded border-t border-dashed border-faint"
              />
              cut by do()
            </span>
            <span className="flex items-center gap-1.5">
              <span aria-hidden className="h-3 w-3 rounded-sm border-2 border-signal" />
              set
            </span>
            <span className="flex items-center gap-1.5">
              <span
                aria-hidden
                className="h-3 w-3 rounded-sm border-2 border-[var(--color-info)]"
              />
              read
            </span>
          </p>
        )}
      </div>
    </div>
  )
  return expanded ? (
    <FloatingWindow
      label={document.name}
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
