import { createPortal } from 'react-dom'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
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
  type DirectedDagEdge,
} from '@/domain/dag'
import type { DiscoveryCandidate } from '@/domain/dagEvidence'
import { affectedDagEdges } from '@/domain/dagValidation'
import { assertNever } from '@/domain/dop'
import { DEFAULT_CARD_SIZE, layoutDagForCanvas, type DagCardSize, type DagLayoutOrientation } from './dagCanvasModel'
import { dagCardSize } from './dagCardSize'
import { useTextMetricsVersion } from '@/lib/textMetrics'
import { roleWord, type DagCausalFlow } from '@/domain/dagFlow'
import type { InterventionOverlay } from '@/domain/intervention'
import { FlowControls, flowControl } from '@/components/flow/FlowControls'
import { useMediaQuery } from '@/lib/useMediaQuery'
import { dagPointerTarget, type ScreenTargetBox } from './dagPointerTarget'

interface DagNodeData extends Record<string, unknown> {
  readonly name: string
  readonly kind: 'observed' | 'latent'
  /** The variable's place relative to the bound treatment and outcome; null when nothing is bound. */
  readonly role: string | null
  readonly evidenceRole: 'none' | 'source' | 'target' | 'both'
  /** During a drag, whether the arrow in hand may land on this card. */
  readonly droppable: boolean
  /** The node's part in the intervention being framed: set by do(), read as the answer, or neither. */
  readonly intervention: 'set' | 'read' | null
  /** The size every card in this graph is drawn at, fitted to the longest name (dagCardSize.ts). */
  readonly size: DagCardSize
}

type CanvasNode = Node<DagNodeData, 'dagVariable'>

interface DagEdgeData extends Record<string, unknown> {
  readonly edge: DirectedDagEdge
  /** Sideways shift in flow units when another arrow joins the same two variables, so the pair does not overprint. */
  readonly offset: number
  /** Severed by the intervention: an arrow into the set node. */
  readonly cut: boolean
}

type CanvasEdge = Edge<DagEdgeData, 'dagEdge'>

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

function DagVariableCard({ id, data, selected }: NodeProps<CanvasNode>) {
  const connection = useConnection()
  const updateNodeInternals = useUpdateNodeInternals()
  const isTarget = connection.inProgress && connection.fromNode.id !== id
  const refused = isTarget && !data.droppable
  // An arrow lifted at its cause end travels from the effect's target handle, so it must land on a source handle.
  const dropType = connection.inProgress && connection.fromHandle.type === 'target' ? 'source' : 'target'
  const dropping = isTarget && !refused
  // The card's handles change shape with the gesture, and React Flow caches their bounds.
  useEffect(() => { updateNodeInternals(id) }, [connection.inProgress, dropping, dropType, id, updateNodeInternals])
  const evidenceColor = data.evidenceRole === 'source'
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
        borderColor: refused
          ? 'var(--color-danger)'
          : isTarget
            ? 'var(--color-signal)'
            : data.intervention === 'set'
              ? 'var(--color-signal)'
              : data.intervention === 'read'
                ? 'var(--color-info)'
                : evidenceColor ?? (data.kind === 'latent' ? 'var(--color-faint)' : 'var(--color-muted)'),
        borderStyle: data.kind === 'latent' ? 'dashed' : 'solid',
        boxShadow: selected || (isTarget && !refused) || evidenceColor !== null
          ? `0 0 0 2px var(--color-panel), 0 0 0 3px ${evidenceColor ?? 'var(--color-signal)'}`
          : undefined,
        opacity: refused ? 0.4 : undefined,
      }}
    >
      <Handle
        id="card"
        type="source"
        position={Position.Right}
        isConnectableEnd={false}
        style={connection.inProgress ? HIDDEN_SOURCE_STYLE : IDLE_SOURCE_STYLE}
        className="nodrag nopan"
        title={`Drag to draw an arrow from ${data.name}`}
      />
      <span className={`${CARD_GRIP} relative z-10 line-clamp-3 cursor-grab whitespace-normal text-body font-medium text-ink [overflow-wrap:break-word] active:cursor-grabbing`} title={`${data.name}, drag to move`}>{data.name}</span>
      <span className={label(`mt-0.5 truncate ${data.intervention === 'set' ? 'text-signal' : data.intervention === 'read' ? 'text-[var(--color-info)]' : 'text-faint'}`)} title={data.role ?? undefined}>{data.role ?? (data.kind === 'latent' ? 'Unmeasured' : 'Observed')}</span>
      <Handle
        type="target"
        position={Position.Left}
        isConnectableStart={false}
        isConnectable={!refused}
        style={dropping && dropType === 'target' ? FULL_CARD_STYLE : REST_TARGET_STYLE}
      />
      {dropping && dropType === 'source' && (
        <Handle id="drop" type="source" position={Position.Right} isConnectableStart={false} style={FULL_CARD_STYLE} className="nodrag nopan" />
      )}
    </div>
  )
}

const NODE_TYPES = { dagVariable: DagVariableCard }

/** The size a card was given, for the moment before React Flow has measured it. */
const cardSizeOf = (node: InternalNode): DagCardSize => (node.data as { readonly size?: DagCardSize }).size ?? DEFAULT_CARD_SIZE
const cardWidth = (node: InternalNode): number => node.measured.width ?? cardSizeOf(node).width
const cardHeight = (node: InternalNode): number => node.measured.height ?? cardSizeOf(node).height

const centreOf = (node: InternalNode) => ({
  x: node.internals.positionAbsolute.x + cardWidth(node) / 2,
  y: node.internals.positionAbsolute.y + cardHeight(node) / 2,
})

/** An arrow ends this far outside the card border, so the head never touches the card. */
const ANCHOR_CLEAR = 5.5
/** Sideways spacing between arrows that join the same two variables. */
const PAIR_SPACING = 12
/** One fit for first paint and the buttons: a small graph never zooms past 100%. */
const FIT_VIEW = { padding: 0.18, maxZoom: 1 } as const
/** Labels sit nearer the cause than the midpoint: arrows converging on one effect then keep their labels apart. */
const LABEL_ALONG = 0.32
/** Perpendicular distance from the line to the label's centre, in flow units. */
const LABEL_OFFSET = 11
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
  const reach = Math.min(halfWidth / Math.max(Math.abs(dx), 1e-6), halfHeight / Math.max(Math.abs(dy), 1e-6))
  return { x: centre.x + dx * reach + (dx / length) * ANCHOR_CLEAR, y: centre.y + dy * reach + (dy / length) * ANCHOR_CLEAR }
}

type Point = { readonly x: number; readonly y: number }
/** Control-point distance for an arrow that has to bow around a card: the curve's peak clears half a card plus its label. */
const BOW_CLEARANCE = 108
/** A card counts as crossed when the line comes within this many units of its border. */
const OBSTACLE_PAD = 10
const quadraticAt = (from: Point, control: Point, to: Point, t: number): Point => ({
  x: (1 - t) * (1 - t) * from.x + 2 * (1 - t) * t * control.x + t * t * to.x,
  y: (1 - t) * (1 - t) * from.y + 2 * (1 - t) * t * control.y + t * t * to.y,
})
/** Whether a curve (or a straight line, when `control` is the midpoint) passes over any of the cards, sampled along its length. */
const crossesAny = (from: Point, control: Point, to: Point, obstacles: readonly InternalNode[]): boolean => {
  const boxes = obstacles.map((node) => {
    const centre = centreOf(node)
    const halfWidth = cardWidth(node) / 2 + OBSTACLE_PAD
    const halfHeight = cardHeight(node) / 2 + OBSTACLE_PAD
    return { left: centre.x - halfWidth, right: centre.x + halfWidth, top: centre.y - halfHeight, bottom: centre.y + halfHeight }
  })
  for (let step = 1; step < 40; step += 1) {
    const point = quadraticAt(from, control, to, step / 40)
    if (boxes.some((box) => point.x > box.left && point.x < box.right && point.y > box.top && point.y < box.bottom)) return true
  }
  return false
}

/**
 * The preview ends where the committed arrow will: at the card's border facing the origin once a card
 * is under the pointer, else at the pointer, with a dot that turns to the verdict as soon as there is one.
 */
function DagConnectionLine({ fromNode, toNode, toX, toY, connectionStatus, connectionLineStyle }: ConnectionLineComponentProps<CanvasNode>) {
  const pointer = { x: toX, y: toY }
  const from = anchorPoint(fromNode, toNode === null ? pointer : centreOf(toNode))
  const to = toNode === null ? pointer : anchorPoint(toNode, centreOf(fromNode))
  const tone = connectionStatus === 'valid' ? 'var(--color-ok)' : connectionStatus === 'invalid' ? 'var(--color-danger)' : 'var(--color-signal)'
  return (
    <g>
      <path d={`M ${from.x} ${from.y} L ${to.x} ${to.y}`} fill="none" style={{ ...connectionLineStyle, stroke: tone }} />
      <circle cx={to.x} cy={to.y} r={4} fill="var(--color-panel)" stroke={tone} strokeWidth={2} />
    </g>
  )
}

function DagEdgePath({ id, source, target, data, style, markerEnd, label: edgeLabel }: EdgeProps<CanvasEdge>) {
  const sourceNode = useInternalNode(source)
  const targetNode = useInternalNode(target)
  const nodeLookup = useStore((state) => state.nodeLookup)
  if (!sourceNode || !targetNode) return null
  const geometry = ((): { readonly path: string; readonly labelAt: { readonly x: number; readonly y: number } } => {
    if (source === target) {
      // A lagged self-edge leaves and re-enters the top edge as a loop; its label sits above the loop.
      const top = topPoint(sourceNode)
      return {
        path: `M ${top.x - 5} ${top.y} C ${top.x - 52} ${top.y - 58}, ${top.x + 52} ${top.y - 58}, ${top.x + 5} ${top.y}`,
        labelAt: { x: top.x, y: top.y - 52 },
      }
    }
    const origin = anchorPoint(sourceNode, centreOf(targetNode))
    const end = anchorPoint(targetNode, centreOf(sourceNode))
    const length = Math.hypot(end.x - origin.x, end.y - origin.y) || 1
    const normal = { x: -(end.y - origin.y) / length, y: (end.x - origin.x) / length }
    const obstacles = [...nodeLookup.values()].filter((node) => node.id !== source && node.id !== target)
    const midpoint = { x: (origin.x + end.x) / 2, y: (origin.y + end.y) / 2 }
    if (crossesAny(origin, midpoint, end, obstacles)) {
      // The straight line would run over another card: bow around it, on the first side that is clear.
      const candidates = [1, -1].map((side) => {
        const control = { x: midpoint.x + normal.x * BOW_CLEARANCE * side, y: midpoint.y + normal.y * BOW_CLEARANCE * side }
        const from = anchorPoint(sourceNode, control)
        const to = anchorPoint(targetNode, control)
        return { side, control, from, to, clear: !crossesAny(from, control, to, obstacles) }
      })
      const bow = candidates.find((candidate) => candidate.clear) ?? candidates[0]
      const at = quadraticAt(bow.from, bow.control, bow.to, LABEL_ALONG)
      return {
        path: `M ${bow.from.x} ${bow.from.y} Q ${bow.control.x} ${bow.control.y} ${bow.to.x} ${bow.to.y}`,
        labelAt: { x: at.x + normal.x * bow.side * LABEL_OFFSET, y: at.y + normal.y * bow.side * LABEL_OFFSET },
      }
    }
    // Arrows between the same two variables straddle the dots on a shared normal, whichever way each one points.
    const shift = (data?.offset ?? 0) * (source < target ? 1 : -1)
    const from = { x: origin.x + normal.x * shift, y: origin.y + normal.y * shift }
    const to = { x: end.x + normal.x * shift, y: end.y + normal.y * shift }
    return {
      path: `M ${from.x} ${from.y} L ${to.x} ${to.y}`,
      // The label sits beside the line, offset along the unit normal, so it never masks the dash it describes.
      labelAt: { x: from.x + (to.x - from.x) * LABEL_ALONG - normal.x * LABEL_OFFSET, y: from.y + (to.y - from.y) * LABEL_ALONG - normal.y * LABEL_OFFSET },
    }
  })()
  const { path, labelAt } = geometry
  return (
    <>
      <BaseEdge id={id} path={path} style={style} markerEnd={markerEnd as string | undefined} />
      {typeof edgeLabel === 'string' && edgeLabel.length > 0 && (
        <EdgeLabelRenderer>
          <span
            className={literal('pointer-events-none absolute rounded border border-hair bg-panel px-1.5 py-0.5 text-micro text-faint')}
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
function EdgeActionBar({ edge, coarse, onReverse, onConfound, onRemove, onLift, onLanded, isValidLanding }: {
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
  if (!sourceNode || !targetNode) return null
  const from = anchorPoint(sourceNode, centreOf(targetNode))
  const to = anchorPoint(targetNode, centreOf(sourceNode))
  const lift = (endpoint: Endpoint) => (event: React.MouseEvent<HTMLButtonElement> | React.TouchEvent<HTMLButtonElement>) => {
    const native = event.nativeEvent
    if (isMouseEvent(native) && native.button !== 0) return
    event.stopPropagation()
    const fixed = endpoint === 'effect' ? { nodeId: edge.source, type: 'source' as const } : { nodeId: edge.target, type: 'target' as const }
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
          style={{ transform: `translate(-50%, -140%) translate(${(from.x + to.x) / 2}px, ${(from.y + to.y) / 2}px)`, zIndex: 10 }}
        >
          <button type="button" className={iconControl('quiet', 'rounded-none border-0')} title="Reverse arrow" aria-label="Reverse selected arrow" onClick={onReverse}><Icon name="swap_horiz" size={14} /></button>
          <button type="button" className={iconControl('quiet', 'rounded-none border-0 border-l border-hair')} title="Replace with an unmeasured common cause" aria-label="Confound selected arrow" onClick={onConfound}><Icon name="call_split" size={14} /></button>
          <button type="button" className={iconControl('danger', 'rounded-none border-0 border-l border-hair')} title="Remove arrow" aria-label="Remove selected arrow" onClick={onRemove}><Icon name="delete" size={14} /></button>
        </div>
        <button
          type="button"
          aria-label="Reconnect cause endpoint"
          title="Drag to reconnect the cause"
          className={knob}
          style={{ transform: `translate(-50%, -50%) translate(${from.x}px, ${from.y}px)`, zIndex: 11, touchAction: 'none' }}
          onMouseDown={lift('cause')}
          onTouchStart={lift('cause')}
        />
        <button
          type="button"
          aria-label="Reconnect effect endpoint"
          title="Drag to reconnect the effect"
          className={knob}
          style={{ transform: `translate(-50%, -50%) translate(${to.x}px, ${to.y}px)`, zIndex: 11, touchAction: 'none' }}
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
function DagGrid() {
  const { zoom } = useViewport()
  let gap = GRID_PITCH
  while (gap * zoom < MIN_SCREEN_PITCH) gap *= 2
  return <Background variant={BackgroundVariant.Dots} color="var(--color-edge)" gap={gap} size={1} />
}

/** Refits the view when the canvas box changes size, so a pane resize or a taller stage never leaves the graph cut off. */
function RefitOnResize({ host, layoutKey }: { readonly host: React.RefObject<HTMLDivElement | null>; readonly layoutKey: string }) {
  const { fitView } = useReactFlow<CanvasNode, CanvasEdge>()
  useEffect(() => {
    const timer = window.setTimeout(() => { void fitView({ ...FIT_VIEW, duration: 160 }) }, 60)
    return () => window.clearTimeout(timer)
  }, [fitView, layoutKey])
  useEffect(() => {
    const element = host.current
    if (element === null) return
    let last = { width: element.clientWidth, height: element.clientHeight }
    let timer = 0
    const observer = new ResizeObserver((entries) => {
      const box = entries[0]?.contentRect
      if (box === undefined || (Math.abs(box.width - last.width) < 2 && Math.abs(box.height - last.height) < 2)) return
      last = { width: box.width, height: box.height }
      window.clearTimeout(timer)
      timer = window.setTimeout(() => { void fitView({ ...FIT_VIEW, duration: 160 }) }, 80)
    })
    observer.observe(element)
    return () => { observer.disconnect(); window.clearTimeout(timer) }
  }, [fitView, host])
  return null
}

function CanvasControls({ onTidy, viewLocked, onToggleLock, expanded, onToggleExpand }: {
  readonly onTidy: () => void
  readonly viewLocked: boolean
  readonly onToggleLock: () => void
  readonly expanded: boolean
  readonly onToggleExpand: () => void
}) {
  const { fitView } = useReactFlow<CanvasNode, CanvasEdge>()
  const control = flowControl
  return (
    <FlowControls fit={FIT_VIEW} fitLabel="Fit graph">
        <button
          type="button"
          className={control}
          title="Tidy: lay the variables out again left to right in causal order and fit the view. Dragged positions are replaced; nothing is rewired."
          aria-label="Tidy graph"
          onClick={() => { onTidy(); window.setTimeout(() => void fitView({ ...FIT_VIEW, duration: 220 }), 30) }}
        >
          <Icon name="auto_awesome_mosaic" size={14} />
        </button>
        <button
          type="button"
          className={control}
          title={viewLocked
            ? 'Scroll wheel scrolls the page. Click to let it zoom the graph instead, or hold ⌘ or Ctrl.'
            : 'Scroll wheel zooms the graph. Click to give it back to the page.'}
          aria-label={viewLocked ? 'Let the wheel zoom' : 'Lock the view'}
          aria-pressed={viewLocked}
          onClick={onToggleLock}
        >
          <Icon name={viewLocked ? 'lock' : 'lock_open'} size={14} />
        </button>
        <button
          type="button"
          className={control}
          title={expanded ? 'Return the graph to the page (Esc)' : 'Expand the graph to the whole window'}
          aria-label={expanded ? 'Return graph to the page' : 'Expand graph'}
          aria-pressed={expanded}
          onClick={onToggleExpand}
        >
          <Icon name={expanded ? 'close_fullscreen' : 'open_in_full'} size={14} />
        </button>
    </FlowControls>
  )
}

const evidenceColumns = (candidate: DiscoveryCandidate | null) => candidate === null
  ? null
  : { source: candidate.source.column, target: candidate.target.column }

const dagNodeFromCanvas = (document: DagDocument, raw: string | null): DagNodeId | null =>
  raw === null ? null : document.current.graph.nodes.find((node) => node.id === raw)?.id ?? null

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
    case 'edge-added': return 'Arrow added as a new revision. Record its rationale in the selected-edge panel.'
    case 'edge-reversed': return 'Arrow reversed as a new revision. The reversed relation needs its own rationale.'
    case 'edge-reconnected': return 'Endpoint moved as a new revision. The reconnected relation needs its own rationale.'
    case 'edge-refused': return describeDagEditProblem(notice.problem)
    default: return assertNever(notice)
  }
}

/** One canvas at a time, so the layer stack needs no per-instance id. */
const EXPANDED_CANVAS_LAYER = 'dag-canvas-expanded'

const IDLE_HINT = 'Drag from a card onto another card to draw an arrow; drag a card by its name to move it. Select an arrow to reverse or remove it, or drag either of its ends to another card.'

const canvasModel = (document: DagDocument, candidate: DiscoveryCandidate | null, flow: DagCausalFlow | null, intervention: InterventionOverlay | null, orientation: DagLayoutOrientation): {
  readonly nodes: CanvasNode[]
  readonly edges: CanvasEdge[]
  /** The size every card is drawn at; a change relays the whole drawing, since kept positions were fitted to the old size. */
  readonly size: DagCardSize
} => {
  const size = dagCardSize(document.current.graph.nodes.map((node) => node.name))
  const placements = new Map(layoutDagForCanvas(document.current.graph, flow === null ? null : { treatment: flow.treatment, outcome: flow.outcome }, orientation, size).map((placed) => [placed.id, placed]))
  const highlighted = evidenceColumns(candidate)
  const validation = document.current.validation
  const problemEdges = new Set(validation.kind === 'invalid' ? validation.issues.flatMap(affectedDagEdges) : [])
  const latentNodes = new Set(document.current.graph.nodes.filter((node) => node.kind === 'latent').map((node) => node.id))
  const pairKey = (edge: DirectedDagEdge): string => [edge.cause, edge.effect].sort().join('\u0000')
  const pairSizes = new Map<string, number>()
  for (const edge of document.current.graph.edges) pairSizes.set(pairKey(edge), (pairSizes.get(pairKey(edge)) ?? 0) + 1)
  const pairSeen = new Map<string, number>()
  const edges: CanvasEdge[] = document.current.graph.edges.map((edge): CanvasEdge => {
    const key = pairKey(edge)
    const rank = pairSeen.get(key) ?? 0
    pairSeen.set(key, rank + 1)
    const offset = (rank - ((pairSizes.get(key) ?? 1) - 1) / 2) * PAIR_SPACING
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
        ? edgeFlow.biasing ? 'var(--color-danger)' : edgeFlow.causal ? 'var(--color-ok)' : 'var(--color-muted)'
        : unstated
          ? 'var(--color-warn)'
          : edge.evidence.length > 0 ? 'var(--color-info)' : 'var(--color-muted)'
    const flowWords = edgeFlow === null ? '' : edgeFlow.biasing ? '; lies on an open biasing path' : edgeFlow.causal ? '; lies on a directed causal path' : '; not on an active treatment–outcome path'
    const timingLabel = edge.timing.kind === 'lagged' ? `t−${edge.timing.lag}` : ''
    const edgeLabel = [timingLabel, unstated ? 'needs rationale' : ''].filter((part) => part.length > 0).join('; ')
    return {
      id: edge.id,
      type: 'dagEdge',
      source: edge.cause,
      target: edge.effect,
      data: { edge, offset, cut },
      label: cut ? 'cut by do()' : edgeLabel,
      // User-space units keep the head 6px long whatever the stroke width, so selection does not swell it.
      markerEnd: { type: MarkerType.ArrowClosed, color: stroke, markerUnits: 'userSpaceOnUse', width: 24, height: 24, strokeWidth: 1 },
      style: {
        stroke,
        strokeWidth: 1.8,
        ...(cut ? { strokeDasharray: '2 5', opacity: 0.7 } : edge.timing.kind === 'lagged' ? { strokeDasharray: '7 4' } : latentNodes.has(edge.cause) ? { strokeDasharray: '3 3' } : {}),
      },
      ariaLabel: `${nameOfDagNode(document, edge.cause)} causes ${nameOfDagNode(document, edge.effect)}${edge.timing.kind === 'lagged' ? ` at lag ${edge.timing.lag}` : ' contemporaneously'}${unstated ? '; rationale not yet recorded' : ''}${flowWords}${cut ? '; cut by the intervention' : ''}`,
    }
  })
  return {
    size,
    nodes: document.current.graph.nodes.map((node) => {
      const placed = placements.get(node.id) ?? { x: 34, y: 34 }
      const source = node.kind === 'observed' && highlighted?.source === node.column
      const target = node.kind === 'observed' && highlighted?.target === node.column
      const evidenceRole: DagNodeData['evidenceRole'] = source && target ? 'both' : source ? 'source' : target ? 'target' : 'none'
      return {
        id: node.id,
        type: 'dagVariable',
        position: { x: placed.x, y: placed.y },
        data: {
          name: node.name,
          kind: node.kind,
          role: intervention !== null && node.id === intervention.set
            ? `do(${node.name})`
            : intervention !== null && node.id === intervention.read
              ? 'read'
              : flow === null ? null : roleWord(flow.roles.get(node.id) ?? { kind: 'unrelated' }),
          evidenceRole,
          droppable: true,
          intervention: intervention === null ? null : node.id === intervention.set ? 'set' : node.id === intervention.read ? 'read' : null,
          size,
        },
        ariaLabel: `${node.kind === 'latent' ? 'Unmeasured' : 'Observed'} variable: ${node.name}`,
      }
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
  readonly onEdgeReconnected: (edge: DagEdgeId, cause: DagNodeId, effect: DagNodeId) => DagEditProblem | null
  readonly onEdgeConfounded: (edge: DagEdgeId) => void
  readonly onEdgeRemoved: (edge: DagEdgeId) => void
  readonly onEdgeSelected: (edge: DagEdgeId | null) => void
}) {
  const hostRef = useRef<HTMLDivElement>(null)
  // A canvas narrower than three rails runs the layout down the page instead of across it.
  const [orientation, setOrientation] = useState<DagLayoutOrientation>('across')
  useEffect(() => {
    const host = hostRef.current
    if (host === null) return
    const observer = new ResizeObserver(([entry]) => { setOrientation(entry.contentRect.width < 600 ? 'down' : 'across') })
    observer.observe(host)
    return () => observer.disconnect()
  }, [])
  // The card size is measured from the names, so the model re-runs once the document's fonts have loaded.
  const metricsVersion = useTextMetricsVersion()
  // eslint-disable-next-line react-hooks/exhaustive-deps -- metricsVersion invalidates the measurements the model is built on
  const model = useMemo(() => canvasModel(document, selectedEvidence, flow, intervention, orientation), [document, flow, intervention, orientation, selectedEvidence, metricsVersion])
  const [nodes, setNodes, onNodesChange] = useNodesState<CanvasNode>(model.nodes)
  // The gesture is read in React Flow's callbacks, which fire from listeners bound at pointer-down, so a ref carries it as well as state.
  const [gesture, setGesture] = useState<Gesture | null>(null)
  const gestureRef = useRef<Gesture | null>(null)
  const beginGesture = (next: Gesture | null) => { gestureRef.current = next; setGesture(next) }
  const [connectionNotice, setConnectionNotice] = useState<ConnectionNotice | null>(null)
  // A refusal describes one gesture; once the document moves on (an arrow added through the form, an undo) it is stale.
  useEffect(() => {
    setConnectionNotice((notice) => (notice !== null && notice.kind === 'edge-refused' ? null : notice))
  }, [document])
  const selectedCanvasEdge = model.edges.find((edge) => edge.id === selectedEdge) ?? null

  // Dragged positions survive every revision; a newcomer takes its layout slot unless a card already
  // sits there, in which case it steps down until it finds clear ground.
  useEffect(() => {
    setNodes((current) => {
      const occupied: { x: number; y: number }[] = []
      const overlaps = (position: { x: number; y: number }) =>
        occupied.some((taken) => Math.abs(taken.x - position.x) < model.size.width + 12 && Math.abs(taken.y - position.y) < model.size.height + 12)
      return model.nodes.map((next) => {
        const existing = current.find((node) => node.id === next.id)
        let position = existing === undefined ? next.position : existing.position
        if (existing === undefined) {
          while (overlaps(position)) position = { x: position.x, y: position.y + model.size.height + 46 }
        }
        occupied.push(position)
        return { ...next, position }
      })
    })
  }, [model.nodes, model.size, setNodes])

  const tidy = () => {
    setNodes(model.nodes)
  }
  // Binding the study changes every card's role, turning the layout changes every slot, and a new card size
  // (a longer name arriving) outgrows the positions the cards were placed at, so the drawing is laid out again.
  const bindingKey = `${flow === null ? '' : `${flow.treatment}\u0000${flow.outcome}`}\u0000${orientation}\u0000${model.size.width}x${model.size.height}`
  const previousBinding = useRef(bindingKey)
  useEffect(() => {
    if (previousBinding.current === bindingKey) return
    previousBinding.current = bindingKey
    setNodes(model.nodes)
  }, [bindingKey, model])
  // The canvas sits inside a scrolling stage, so a wheel over it is ambiguous. Locked is the safer
  // default: the wheel scrolls the page, ⌘ or Ctrl with the wheel still zooms, and the buttons always work.
  const [viewLocked, setViewLocked] = useState(true)
  // A finger is imprecise: connections snap from further away and a tap on one dot then another also connects.
  const coarse = useMediaQuery('(pointer: coarse)')
  const [expanded, setExpanded] = useState(false)
  useEffect(() => (expanded ? pushLayer(EXPANDED_CANVAS_LAYER) : undefined), [expanded])
  useEffect(() => {
    if (!expanded) return undefined
    const escape = (event: KeyboardEvent) => { if (event.key === 'Escape' && escapeFor(EXPANDED_CANVAS_LAYER, event)) setExpanded(false) }
    window.addEventListener('keydown', escape)
    return () => window.removeEventListener('keydown', escape)
  }, [expanded])
  const additionAllowed = useCallback((cause: DagNodeId | null, effect: DagNodeId | null): boolean =>
    inspectDagEdgeAddition(document, cause, effect).ok, [document])
  const connect = (connection: Connection) => {
    const cause = dagNodeFromCanvas(document, connection.source)
    const effect = dagNodeFromCanvas(document, connection.target)
    if (cause !== null && effect !== null) addArrow(cause, effect)
  }
  const addArrow = (cause: DagNodeId, effect: DagNodeId) => {
    const problem = onConnectionDrawn(cause, effect)
    setConnectionNotice(problem === null ? { kind: 'edge-added' } : { kind: 'edge-refused', problem })
  }
  const replacementFor = useCallback((edgeId: DagEdgeId, endpoint: Endpoint, node: DagNodeId) => {
    const edge = document.current.graph.edges.find((candidate) => candidate.id === edgeId)
    if (edge === undefined) return null
    const cause = endpoint === 'cause' ? node : edge.cause
    const effect = endpoint === 'effect' ? node : edge.effect
    return { edge, cause, effect, inspected: inspectDagEdgeReplacement(document, edgeId, cause, effect, edge.timing) }
  }, [document])
  /** Moves one end of an arrow to a card; landing it back where it was is not a revision. */
  const moveEnd = (edgeId: DagEdgeId, endpoint: Endpoint, node: DagNodeId) => {
    const replacement = replacementFor(edgeId, endpoint, node)
    if (replacement === null) return
    if (!replacement.inspected.ok) { setConnectionNotice({ kind: 'edge-refused', problem: replacement.inspected.error }); return }
    if (replacement.cause === replacement.edge.cause && replacement.effect === replacement.edge.effect) { setConnectionNotice(null); return }
    const problem = onEdgeReconnected(edgeId, replacement.cause, replacement.effect)
    setConnectionNotice(problem === null ? { kind: 'edge-reconnected' } : { kind: 'edge-refused', problem })
  }
  /** The card a landed connection names for the lifted end: the far end is the fixed one. */
  const landedNode = (endpoint: Endpoint, candidate: Connection | Edge): DagNodeId | null =>
    dagNodeFromCanvas(document, endpoint === 'effect' ? candidate.target : candidate.source)
  /** Whether the arrow in hand may land on a card. */
  const landable = (active: Gesture, node: DagNodeId): boolean => {
    switch (active.kind) {
      case 'connect': return additionAllowed(active.from, node)
      case 'reconnect': return replacementFor(active.edge, active.endpoint, node)?.inspected.ok === true
      default: return assertNever(active)
    }
  }
  /** Lands the arrow in hand on a card, or reports why it may not. */
  const land = (active: Gesture, node: DagNodeId) => {
    switch (active.kind) {
      case 'connect': addArrow(active.from, node); return
      case 'reconnect': moveEnd(active.edge, active.endpoint, node); return
      default: return assertNever(active)
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
    if (point === undefined) { setConnectionNotice(null); return }
    // Letting go on the card the arrow came from is a change of mind, not an attempt at a self-loop.
    const origin = active.kind === 'connect' ? active.from : null
    const boxes = cardBoxes((node) => landable(active, node)).filter((box) => box.node !== origin)
    const target = dagPointerTarget({ x: point.clientX, y: point.clientY }, boxes, coarse ? 56 : 28)
    switch (target.kind) {
      case 'eligible-node': land(active, target.node); return
      case 'refused-node': land(active, target.node); return
      case 'none': setConnectionNotice(null); return
      default: return assertNever(target)
    }
  }
  const reverse = (edge: CanvasEdge) => {
    const causalEdge = edge.data?.edge
    if (causalEdge === undefined) return
    const problem = onEdgeReconnected(causalEdge.id, causalEdge.effect, causalEdge.cause)
    setConnectionNotice(problem === null ? { kind: 'edge-reversed' } : { kind: 'edge-refused', problem })
  }
  /** Every card's screen box, marked with whether the gesture in hand may land on it. */
  const cardBoxes = useCallback((eligible: (node: DagNodeId) => boolean): ScreenTargetBox[] => {
    const host = hostRef.current
    if (host === null) return []
    const boxes: ScreenTargetBox[] = []
    for (const element of host.querySelectorAll<HTMLElement>('.react-flow__node')) {
      const rawId = element.dataset.id
      const node = document.current.graph.nodes.find((candidate) => candidate.id === rawId)
      if (node === undefined) continue
      const rect = element.getBoundingClientRect()
      boxes.push({ node: node.id, left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom, eligible: eligible(node.id) })
    }
    return boxes
  }, [document.current.graph.nodes])

  const displayedNodes = nodes.map((node): CanvasNode => {
    const domainNode = document.current.graph.nodes.find((candidate) => candidate.id === node.id)
    if (domainNode === undefined) return node
    // The card body draws arrows, so the name is what moves it.
    const dragged = { ...node, dragHandle: `.${CARD_GRIP}` }
    if (gesture !== null) return { ...dragged, data: { ...node.data, droppable: landable(gesture, domainNode.id) } }
    return dragged
  })

  const keyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement || target instanceof HTMLSelectElement || target.isContentEditable) return
    if (selectedCanvasEdge !== null && event.key.toLowerCase() === 'r') {
      reverse(selectedCanvasEdge)
      event.preventDefault()
    } else if (selectedCanvasEdge !== null && event.key.toLowerCase() === 'u') {
      const edge = selectedCanvasEdge.data?.edge
      if (edge !== undefined) onEdgeConfounded(edge.id)
      event.preventDefault()
    } else if (selectedCanvasEdge !== null && (event.key === 'Delete' || event.key === 'Backspace')) {
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
      className={expanded
        ? 'fixed inset-3 z-(--z-dialog) flex flex-col overflow-hidden rounded-xl border border-edge bg-well float'
        : 'relative flex min-h-[16rem] flex-1 flex-col overflow-hidden rounded-xl border border-edge bg-well @max-md/panel:min-h-[26rem]'}
      aria-label="Causal DAG editor"
      onKeyDown={keyDown}
    >
      <div className="relative min-h-0 flex-1">
        <div className="absolute inset-0">
        <ReactFlow<CanvasNode, CanvasEdge>
          nodes={displayedNodes}
          edges={model.edges.map((edge) => edge.id === selectedEdge
            ? { ...edge, selected: true, style: { ...edge.style, strokeWidth: 2.8 } }
            : selectedEdge === null
              ? { ...edge, selected: false }
              : { ...edge, selected: false, style: { ...edge.style, opacity: 0.3 } })}
          nodeTypes={NODE_TYPES}
          edgeTypes={EDGE_TYPES}
          onNodesChange={onNodesChange}
          onConnect={connect}
          onConnectStart={(_event, params) => {
            setConnectionNotice(null)
            // A lifted arrow end registers its gesture before the engine starts; a new arrow registers here.
            if (gestureRef.current?.kind === 'reconnect') return
            const from = dagNodeFromCanvas(document, params.nodeId)
            if (from !== null) beginGesture({ kind: 'connect', from })
          }}
          onConnectEnd={connectionEnded}
          isValidConnection={(connection) => additionAllowed(
            dagNodeFromCanvas(document, connection.source),
            dagNodeFromCanvas(document, connection.target),
          )}
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
          elevateEdgesOnSelect
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
                return edge !== undefined && node !== null && replacementFor(edge.id, endpoint, node)?.inspected.ok === true
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
          <CanvasControls onTidy={tidy} viewLocked={viewLocked} onToggleLock={() => setViewLocked((locked) => !locked)} expanded={expanded} onToggleExpand={() => setExpanded((open) => !open)} />
          <RefitOnResize host={hostRef} layoutKey={bindingKey} />
        </ReactFlow>
        </div>
      </div>
      <div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 border-t border-hair bg-panel px-3 py-1.5">
        <div className="flex min-w-0 flex-1 basis-[16rem] items-center gap-2">
          <Tooltip text={IDLE_HINT}>
            <button type="button" className="grid h-6 w-6 shrink-0 place-items-center rounded-md text-faint transition-colors hover:text-ink" aria-label="How to draw and edit arrows">
              <Icon name="help" size={15} />
            </button>
          </Tooltip>
          <p role="status" className={`m-0 min-w-0 flex-1 text-label ${refusedNotice ? 'text-warn' : 'text-ink'}`}>
            {connectionNotice === null ? '' : describeConnectionNotice(connectionNotice)}
          </p>
        </div>
        {flow !== null && (
          <p className="m-0 flex flex-wrap items-center gap-x-3 gap-y-1 text-label text-faint" aria-label="Arrow legend">
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-[2px] w-4 rounded bg-ok" />directed causal path</span>
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-[2px] w-4 rounded bg-danger" />open biasing path</span>
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-[2px] w-4 rounded bg-muted" />other relation</span>
          </p>
        )}
        {intervention !== null && (
          <p className="m-0 flex flex-wrap items-center gap-x-3 gap-y-1 text-label text-faint" aria-label="Intervention legend">
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-[2px] w-4 rounded border-t border-dashed border-faint" />cut by do()</span>
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-3 w-3 rounded-sm border-2 border-signal" />set</span>
            <span className="flex items-center gap-1.5"><span aria-hidden className="h-3 w-3 rounded-sm border-2 border-[var(--color-info)]" />read</span>
          </p>
        )}
      </div>
    </div>
  )
  // Expanded, the canvas is portalled to the body: the workbench pane declares `container-type: size`,
  // which makes it the containing block for fixed positioning, so in place the layer would be inset
  // from the pane and clipped by its neighbours instead of filling the window.
  // `document` here is the DAG document prop, so the global is named explicitly.
  return expanded ? createPortal(canvas, globalThis.document.body) : canvas
}
