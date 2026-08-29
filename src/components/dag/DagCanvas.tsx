import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import {
  Background,
  BackgroundVariant,
  BaseEdge,
  type ConnectionLineComponentProps,
  EdgeLabelRenderer,
  Handle,
  MarkerType,
  Panel,
  Position,
  ReactFlow,
  useConnection,
  useInternalNode,
  useStore,
  useNodesState,
  useReactFlow,
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
import { Icon } from '@/components/Icon'
import { iconControl, label, literal } from '@/components/ui/recipes'
import {
  describeDagEditProblem,
  inspectDagEdgeAddition,
  inspectDagEdgeReplacement,
  type DagDocument,
  type DagEditProblem,
  type DagEdgeId,
  type DagNodeId,
  type DirectedDagEdge,
} from '@/domain/dag'
import type { DiscoveryCandidate } from '@/domain/dagEvidence'
import { affectedDagEdges } from '@/domain/dagValidation'
import { assertNever } from '@/domain/dop'
import { layoutDagForCanvas } from './dagCanvasModel'
import { roleWord, type DagCausalFlow } from '@/domain/dagFlow'
import type { InterventionOverlay } from '@/domain/intervention'
import { dagPointerTarget, type DagPointerTarget, type ScreenTargetBox } from './dagPointerTarget'

interface DagNodeData extends Record<string, unknown> {
  readonly name: string
  readonly kind: 'observed' | 'latent'
  /** The variable's place relative to the bound treatment and outcome; null when nothing is bound. */
  readonly role: string | null
  readonly evidenceRole: 'none' | 'source' | 'target' | 'both'
  readonly droppable: boolean
  readonly reconnectTarget: 'none' | 'eligible' | 'snapped' | 'refused'
  /** The node's part in the intervention being framed: set by do(), read as the answer, or neither. */
  readonly intervention: 'set' | 'read' | null
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

const SIDE_HANDLES = [
  { id: 'top', position: Position.Top },
  { id: 'right', position: Position.Right },
  { id: 'bottom', position: Position.Bottom },
  { id: 'left', position: Position.Left },
] as const

const DOT_STYLE: React.CSSProperties = {
  width: 24,
  height: 24,
  border: 'none',
  background: 'radial-gradient(circle, var(--color-muted) 0 4px, var(--color-panel) 4.5px 6px, transparent 6.5px)',
}

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

const FULL_TARGET_STYLE: React.CSSProperties = {
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
}

function DagVariableCard({ id, data, selected }: NodeProps<CanvasNode>) {
  const connection = useConnection()
  const isTarget = connection.inProgress && connection.fromNode.id !== id
  const refused = isTarget && !data.droppable
  const reconnecting = data.reconnectTarget !== 'none'
  const reconnectRefused = data.reconnectTarget === 'refused'
  const reconnectSnapped = data.reconnectTarget === 'snapped'
  const evidenceColor = data.evidenceRole === 'source'
    ? 'var(--color-info)'
    : data.evidenceRole === 'target'
      ? 'var(--color-warn)'
      : data.evidenceRole === 'both'
        ? 'var(--color-signal)'
        : null
  return (
    <div
      className="group relative flex h-[58px] w-[164px] flex-col justify-center rounded-lg border bg-panel px-3 text-center transition-shadow"
      style={{
        borderColor: refused || reconnectRefused
          ? 'var(--color-danger)'
          : isTarget || reconnectSnapped
            ? 'var(--color-signal)'
            : data.reconnectTarget === 'eligible'
              ? 'var(--color-info)'
              : data.intervention === 'set'
                ? 'var(--color-signal)'
                : data.intervention === 'read'
                  ? 'var(--color-info)'
                  : evidenceColor ?? (data.kind === 'latent' ? 'var(--color-faint)' : 'var(--color-muted)'),
        borderStyle: data.kind === 'latent' ? 'dashed' : 'solid',
        boxShadow: selected || (isTarget && !refused) || reconnectSnapped || evidenceColor !== null
          ? `0 0 0 2px var(--color-panel), 0 0 0 3px ${evidenceColor ?? 'var(--color-signal)'}`
          : undefined,
        opacity: refused || reconnectRefused ? 0.4 : reconnecting && data.reconnectTarget !== 'snapped' ? 0.72 : undefined,
      }}
    >
      <span className="truncate text-body font-medium text-ink" title={data.name}>{data.name}</span>
      <span className={label(`mt-0.5 truncate ${data.intervention === 'set' ? 'normal-case tracking-normal text-signal' : data.intervention === 'read' ? 'text-[var(--color-info)]' : 'text-faint'}`)} title={data.role ?? undefined}>{data.role ?? (data.kind === 'latent' ? 'Unmeasured' : 'Observed')}</span>
      {!connection.inProgress && SIDE_HANDLES.map((side) => (
        <Handle
          key={side.id}
          id={side.id}
          type="source"
          position={side.position}
          style={DOT_STYLE}
          className="opacity-70 transition-opacity group-hover:opacity-100"
          title={`Draw a causal relation from ${data.name}`}
        />
      ))}
      <Handle
        type="target"
        position={Position.Left}
        isConnectableStart={false}
        isConnectable={!refused}
        style={isTarget && !refused ? FULL_TARGET_STYLE : REST_TARGET_STYLE}
      />
    </div>
  )
}

const NODE_TYPES = { dagVariable: DagVariableCard }

const centreOf = (node: InternalNode) => ({
  x: node.internals.positionAbsolute.x + (node.measured.width ?? 164) / 2,
  y: node.internals.positionAbsolute.y + (node.measured.height ?? 58) / 2,
})

/** Radius of the visible handle dot; an arrow ends just past its rim, so the head meets the dot rather than the card border. */
const DOT_RADIUS = 6
const ANCHOR_CLEAR = DOT_RADIUS - 0.5
/** Sideways spacing between arrows that join the same two variables. */
const PAIR_SPACING = 12
/** One fit for first paint and the buttons: a small graph never zooms past 100%. */
const FIT_VIEW = { padding: 0.18, maxZoom: 1 } as const
/** Labels sit nearer the cause than the midpoint: arrows converging on one effect then keep their labels apart. */
const LABEL_ALONG = 0.32
/** Perpendicular distance from the line to the label's centre, in flow units. */
const LABEL_OFFSET = 11
type HandleSide = (typeof SIDE_HANDLES)[number]['id']
/** The point just outside a side's handle dot. The edge, the reconnect knobs and the connection preview all read this one function, so they cannot disagree. */
const handlePoint = (node: InternalNode, side: HandleSide) => {
  const centre = centreOf(node)
  const width = node.measured.width ?? 164
  const height = node.measured.height ?? 58
  switch (side) {
    case 'right': return { x: centre.x + width / 2 + ANCHOR_CLEAR, y: centre.y }
    case 'left': return { x: centre.x - width / 2 - ANCHOR_CLEAR, y: centre.y }
    case 'bottom': return { x: centre.x, y: centre.y + height / 2 + ANCHOR_CLEAR }
    case 'top': return { x: centre.x, y: centre.y - height / 2 - ANCHOR_CLEAR }
    default: return assertNever(side)
  }
}
/** The side whose dot faces the other node: left or right unless the other node sits more above or below than beside. */
const sideToward = (node: InternalNode, toward: { readonly x: number; readonly y: number }): HandleSide => {
  const centre = centreOf(node)
  const dx = toward.x - centre.x
  const dy = toward.y - centre.y
  if (Math.abs(dx) >= Math.abs(dy)) return dx >= 0 ? 'right' : 'left'
  return dy > 0 ? 'bottom' : 'top'
}
const anchorPoint = (node: InternalNode, toward: { readonly x: number; readonly y: number }) => handlePoint(node, sideToward(node, toward))

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
    const halfWidth = (node.measured.width ?? 164) / 2 + OBSTACLE_PAD
    const halfHeight = (node.measured.height ?? 58) / 2 + OBSTACLE_PAD
    return { left: centre.x - halfWidth, right: centre.x + halfWidth, top: centre.y - halfHeight, bottom: centre.y + halfHeight }
  })
  for (let step = 1; step < 40; step += 1) {
    const point = quadraticAt(from, control, to, step / 40)
    if (boxes.some((box) => point.x > box.left && point.x < box.right && point.y > box.top && point.y < box.bottom)) return true
  }
  return false
}

/** The preview ends where the committed arrow will: at the dot facing the origin once a card is under the pointer, else at the pointer. */
function DagConnectionLine({ fromNode, toNode, toX, toY, connectionLineStyle }: ConnectionLineComponentProps<CanvasNode>) {
  const pointer = { x: toX, y: toY }
  const from = anchorPoint(fromNode, toNode === null ? pointer : centreOf(toNode))
  const to = toNode === null ? pointer : anchorPoint(toNode, centreOf(fromNode))
  return <path d={`M ${from.x} ${from.y} L ${to.x} ${to.y}`} fill="none" style={connectionLineStyle} />
}

function DagEdgePath({ id, source, target, data, style, markerEnd, label: edgeLabel }: EdgeProps<CanvasEdge>) {
  const sourceNode = useInternalNode(source)
  const targetNode = useInternalNode(target)
  const nodeLookup = useStore((state) => state.nodeLookup)
  if (!sourceNode || !targetNode) return null
  const geometry = ((): { readonly path: string; readonly labelAt: { readonly x: number; readonly y: number } } => {
    if (source === target) {
      // A lagged self-edge leaves and re-enters the top dot as a loop; its label sits above the loop.
      const top = handlePoint(sourceNode, 'top')
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

function EdgeActionBar({ edge, onReverse, onConfound, onRemove, onEndpointPointerDown }: {
  readonly edge: CanvasEdge
  readonly onReverse: () => void
  readonly onConfound: () => void
  readonly onRemove: () => void
  readonly onEndpointPointerDown: (endpoint: 'cause' | 'effect', event: React.PointerEvent<HTMLButtonElement>) => void
}) {
  const sourceNode = useInternalNode(edge.source)
  const targetNode = useInternalNode(edge.target)
  if (!sourceNode || !targetNode) return null
  const from = anchorPoint(sourceNode, centreOf(targetNode))
  const to = anchorPoint(targetNode, centreOf(sourceNode))
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
          className="nodrag nopan pointer-events-auto absolute h-6 w-6 cursor-grab lift rounded-full border-2 border-signal bg-panel active:cursor-grabbing"
          style={{ transform: `translate(-50%, -50%) translate(${from.x}px, ${from.y}px)`, zIndex: 11 }}
          onPointerDown={(event) => onEndpointPointerDown('cause', event)}
        />
        <button
          type="button"
          aria-label="Reconnect effect endpoint"
          title="Drag to reconnect the effect"
          className="nodrag nopan pointer-events-auto absolute h-6 w-6 cursor-grab lift rounded-full border-2 border-signal bg-panel active:cursor-grabbing"
          style={{ transform: `translate(-50%, -50%) translate(${to.x}px, ${to.y}px)`, zIndex: 11 }}
          onPointerDown={(event) => onEndpointPointerDown('effect', event)}
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
function RefitOnResize({ host }: { readonly host: React.RefObject<HTMLDivElement | null> }) {
  const { fitView } = useReactFlow<CanvasNode, CanvasEdge>()
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
  const { fitView, zoomIn, zoomOut } = useReactFlow<CanvasNode, CanvasEdge>()
  const control = iconControl('quiet', 'rounded-none border-0')
  return (
    <Panel position="bottom-right" className="!m-2">
      <div className="flex flex-col overflow-hidden rounded-lg border border-hair bg-panel/95 backdrop-blur">
        <button type="button" className={control} title="Zoom in" aria-label="Zoom in" onClick={() => void zoomIn({ duration: 160 })}><Icon name="add" size={14} /></button>
        <button type="button" className={control} title="Zoom out" aria-label="Zoom out" onClick={() => void zoomOut({ duration: 160 })}><Icon name="remove" size={14} /></button>
        <button type="button" className={control} title="Fit graph" aria-label="Fit graph" onClick={() => void fitView({ ...FIT_VIEW, duration: 220 })}><Icon name="fit_screen" size={14} /></button>
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
      </div>
    </Panel>
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
  | { readonly kind: 'missed-target' }

type ReconnectGesture =
  | { readonly kind: 'idle' }
  | {
      readonly kind: 'dragging'
      readonly edge: DagEdgeId
      readonly endpoint: 'cause' | 'effect'
      readonly pointerId: number
      readonly origin: { readonly x: number; readonly y: number }
      readonly pointer: { readonly x: number; readonly y: number }
      readonly target: DagPointerTarget
    }

const describeConnectionNotice = (notice: ConnectionNotice): string => {
  switch (notice.kind) {
    case 'edge-added': return 'Arrow added as a new revision. Record its rationale in the selected-edge panel.'
    case 'edge-reversed': return 'Arrow reversed as a new revision. The reversed relation needs its own rationale.'
    case 'edge-reconnected': return 'Endpoint moved as a new revision. The reconnected relation needs its own rationale.'
    case 'edge-refused': return describeDagEditProblem(notice.problem)
    case 'missed-target': return 'No variable received the connector. Drop anywhere on a highlighted variable card.'
    default: return assertNever(notice)
  }
}

const IDLE_HINT = 'Drag from a variable’s handle onto another variable to draw a causal arrow. Select an arrow to reverse, reconnect, or remove it.'

const canvasModel = (document: DagDocument, candidate: DiscoveryCandidate | null, flow: DagCausalFlow | null, intervention: InterventionOverlay | null): {
  readonly nodes: CanvasNode[]
  readonly edges: CanvasEdge[]
} => {
  const placements = new Map(layoutDagForCanvas(document.current.graph, flow === null ? null : { treatment: flow.treatment, outcome: flow.outcome }).map((placed) => [placed.id, placed]))
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
    const edgeLabel = [timingLabel, unstated ? 'needs rationale' : ''].filter((part) => part.length > 0).join(' · ')
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
      ariaLabel: `${edge.cause} causes ${edge.effect}${edge.timing.kind === 'lagged' ? ` at lag ${edge.timing.lag}` : ' contemporaneously'}${unstated ? '; rationale not yet recorded' : ''}${flowWords}${cut ? '; cut by the intervention' : ''}`,
    }
  })
  return {
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
          reconnectTarget: 'none',
          intervention: intervention === null ? null : node.id === intervention.set ? 'set' : node.id === intervention.read ? 'read' : null,
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
  const model = useMemo(() => canvasModel(document, selectedEvidence, flow, intervention), [document, flow, intervention, selectedEvidence])
  const [nodes, setNodes, onNodesChange] = useNodesState<CanvasNode>(model.nodes)
  const [connectingFrom, setConnectingFrom] = useState<DagNodeId | null>(null)
  const [connectionNotice, setConnectionNotice] = useState<ConnectionNotice | null>(null)
  // A refusal describes one gesture; once the document moves on (an arrow added through the form, an undo) it is stale.
  useEffect(() => {
    setConnectionNotice((notice) => (notice !== null && (notice.kind === 'missed-target' || notice.kind === 'edge-refused') ? null : notice))
  }, [document])
  const [reconnectGesture, setReconnectGesture] = useState<ReconnectGesture>({ kind: 'idle' })
  const selectedCanvasEdge = model.edges.find((edge) => edge.id === selectedEdge) ?? null

  // Dragged positions survive every revision; a newcomer takes its layout slot unless a card already
  // sits there, in which case it steps down until it finds clear ground.
  useEffect(() => {
    setNodes((current) => {
      const occupied: { x: number; y: number }[] = []
      const overlaps = (position: { x: number; y: number }) =>
        occupied.some((taken) => Math.abs(taken.x - position.x) < 164 + 12 && Math.abs(taken.y - position.y) < 58 + 12)
      return model.nodes.map((next) => {
        const existing = current.find((node) => node.id === next.id)
        let position = existing === undefined ? next.position : existing.position
        if (existing === undefined) {
          while (overlaps(position)) position = { x: position.x, y: position.y + 58 + 46 }
        }
        occupied.push(position)
        return { ...next, position }
      })
    })
  }, [model.nodes, setNodes])

  const tidy = () => {
    setNodes(model.nodes)
  }
  // Binding the study changes every card's role, so the drawing is laid out again around the new baseline.
  const bindingKey = flow === null ? '' : `${flow.treatment}\u0000${flow.outcome}`
  const previousBinding = useRef(bindingKey)
  useEffect(() => {
    if (previousBinding.current === bindingKey) return
    previousBinding.current = bindingKey
    setNodes(model.nodes)
  }, [bindingKey, model])
  // The canvas sits inside a scrolling stage, so a wheel over it is ambiguous. Locked is the safer
  // default: the wheel scrolls the page, ⌘ or Ctrl with the wheel still zooms, and the buttons always work.
  const [viewLocked, setViewLocked] = useState(true)
  const [expanded, setExpanded] = useState(false)
  useEffect(() => {
    if (!expanded) return
    const escape = (event: KeyboardEvent) => { if (event.key === 'Escape') setExpanded(false) }
    window.addEventListener('keydown', escape)
    return () => window.removeEventListener('keydown', escape)
  }, [expanded])
  const additionAllowed = useCallback((cause: DagNodeId | null, effect: DagNodeId | null): boolean =>
    inspectDagEdgeAddition(document, cause, effect).ok, [document])
  const connect = (connection: Connection) => {
    const cause = dagNodeFromCanvas(document, connection.source)
    const effect = dagNodeFromCanvas(document, connection.target)
    if (cause === null || effect === null) return
    const problem = onConnectionDrawn(cause, effect)
    setConnectionNotice(problem === null ? { kind: 'edge-added' } : { kind: 'edge-refused', problem })
  }
  /** A refused card exposes no connectable handle, so the drop point identifies it instead of xyflow's `toNode`. */
  const nodeUnderPointer = (event: MouseEvent | TouchEvent): DagNodeId | null => {
    const host = hostRef.current
    if (host === null) return null
    const point = 'changedTouches' in event ? event.changedTouches[0] : event
    if (point === undefined) return null
    for (const element of host.querySelectorAll<HTMLElement>('.react-flow__node')) {
      const rect = element.getBoundingClientRect()
      if (point.clientX >= rect.left && point.clientX <= rect.right && point.clientY >= rect.top && point.clientY <= rect.bottom) {
        return dagNodeFromCanvas(document, element.dataset.id ?? null)
      }
    }
    return null
  }
  const connectionEnded = (event: MouseEvent | TouchEvent, connection: FinalConnectionState) => {
    setConnectingFrom(null)
    if (connection.isValid === true) return
    const cause = dagNodeFromCanvas(document, connection.fromNode?.id ?? null)
    const effect = dagNodeFromCanvas(document, connection.toNode?.id ?? null) ?? nodeUnderPointer(event)
    if (cause === null || effect === null) {
      setConnectionNotice({ kind: 'missed-target' })
      return
    }
    const inspected = inspectDagEdgeAddition(document, cause, effect)
    setConnectionNotice(inspected.ok ? { kind: 'missed-target' } : { kind: 'edge-refused', problem: inspected.error })
  }
  const reverse = (edge: CanvasEdge) => {
    const causalEdge = edge.data?.edge
    if (causalEdge === undefined) return
    const problem = onEdgeReconnected(causalEdge.id, causalEdge.effect, causalEdge.cause)
    setConnectionNotice(problem === null ? { kind: 'edge-reversed' } : { kind: 'edge-refused', problem })
  }
  const replacementFor = useCallback((edgeId: DagEdgeId, endpoint: 'cause' | 'effect', node: DagNodeId) => {
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
  }, [document])

  const targetBoxes = useCallback((edgeId: DagEdgeId, endpoint: 'cause' | 'effect'): ScreenTargetBox[] => {
    const host = hostRef.current
    if (host === null) return []
    const boxes: ScreenTargetBox[] = []
    for (const element of host.querySelectorAll<HTMLElement>('.react-flow__node')) {
      const rawId = element.dataset.id
      const node = document.current.graph.nodes.find((candidate) => candidate.id === rawId)
      if (node === undefined) continue
      const rect = element.getBoundingClientRect()
      const replacement = replacementFor(edgeId, endpoint, node.id)
      boxes.push({
        node: node.id,
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        eligible: replacement?.inspected.ok === true,
      })
    }
    return boxes
  }, [document.current.graph.nodes, replacementFor])

  const beginEndpointReconnect = (edge: CanvasEdge, endpoint: 'cause' | 'effect', event: React.PointerEvent<HTMLButtonElement>) => {
    if (event.button !== 0) return
    const causalEdge = edge.data?.edge
    if (causalEdge === undefined) return
    const host = hostRef.current
    if (host === null) return
    event.preventDefault()
    event.stopPropagation()
    event.currentTarget.setPointerCapture(event.pointerId)
    const hostBox = host.getBoundingClientRect()
    const endpointBox = event.currentTarget.getBoundingClientRect()
    const origin = {
      x: endpointBox.left + endpointBox.width / 2 - hostBox.left,
      y: endpointBox.top + endpointBox.height / 2 - hostBox.top,
    }
    setConnectionNotice(null)
    setReconnectGesture({
      kind: 'dragging',
      edge: causalEdge.id,
      endpoint,
      pointerId: event.pointerId,
      origin,
      pointer: origin,
      target: { kind: 'none' },
    })
  }

  useEffect(() => {
    if (reconnectGesture.kind !== 'dragging') return
    const active = reconnectGesture
    const resolve = (event: PointerEvent) => dagPointerTarget(
      { x: event.clientX, y: event.clientY },
      targetBoxes(active.edge, active.endpoint),
      56,
    )
    const localPoint = (event: PointerEvent, target: DagPointerTarget) => {
      const hostBox = hostRef.current?.getBoundingClientRect()
      if (hostBox === undefined) return active.pointer
      const screen = target.kind === 'none' ? { x: event.clientX, y: event.clientY } : target.anchor
      return { x: screen.x - hostBox.left, y: screen.y - hostBox.top }
    }
    const move = (event: PointerEvent) => {
      if (event.pointerId !== active.pointerId) return
      const target = resolve(event)
      setReconnectGesture((current) => current.kind === 'dragging' && current.pointerId === event.pointerId
        ? { ...current, target, pointer: localPoint(event, target) }
        : current)
    }
    const finish = (event: PointerEvent) => {
      if (event.pointerId !== active.pointerId) return
      const target = resolve(event)
      if (target.kind === 'eligible-node') {
        const replacement = replacementFor(active.edge, active.endpoint, target.node)
        if (replacement !== null && replacement.inspected.ok) {
          const unchanged = replacement.cause === replacement.edge.cause && replacement.effect === replacement.edge.effect
          if (!unchanged) {
            const problem = onEdgeReconnected(active.edge, replacement.cause, replacement.effect)
            setConnectionNotice(problem === null ? { kind: 'edge-reconnected' } : { kind: 'edge-refused', problem })
          }
        }
      } else if (target.kind === 'refused-node') {
        const replacement = replacementFor(active.edge, active.endpoint, target.node)
        setConnectionNotice(replacement !== null && !replacement.inspected.ok
          ? { kind: 'edge-refused', problem: replacement.inspected.error }
          : { kind: 'missed-target' })
      } else {
        setConnectionNotice({ kind: 'missed-target' })
      }
      setReconnectGesture({ kind: 'idle' })
    }
    const cancel = (event: PointerEvent) => {
      if (event.pointerId === active.pointerId) setReconnectGesture({ kind: 'idle' })
    }
    window.addEventListener('pointermove', move)
    window.addEventListener('pointerup', finish)
    window.addEventListener('pointercancel', cancel)
    return () => {
      window.removeEventListener('pointermove', move)
      window.removeEventListener('pointerup', finish)
      window.removeEventListener('pointercancel', cancel)
    }
  }, [onEdgeReconnected, reconnectGesture.kind, replacementFor, targetBoxes])

  const reconnectEligibility = useMemo(() => {
    if (reconnectGesture.kind !== 'dragging') return new Map<DagNodeId, boolean>()
    return new Map(document.current.graph.nodes.map((node) => [
      node.id,
      replacementFor(reconnectGesture.edge, reconnectGesture.endpoint, node.id)?.inspected.ok === true,
    ]))
  }, [document.current.graph.nodes, reconnectGesture, replacementFor])

  const displayedNodes = nodes.map((node): CanvasNode => {
    const domainNode = document.current.graph.nodes.find((candidate) => candidate.id === node.id)
    if (connectingFrom !== null && domainNode !== undefined) {
      return { ...node, data: { ...node.data, droppable: additionAllowed(connectingFrom, domainNode.id) } }
    }
    if (reconnectGesture.kind !== 'dragging') return node
    const snapped = reconnectGesture.target.kind !== 'none' && reconnectGesture.target.node === node.id
    const eligible = domainNode !== undefined && reconnectEligibility.get(domainNode.id) === true
    return {
      ...node,
      data: {
        ...node.data,
        reconnectTarget: snapped ? (eligible ? 'snapped' : 'refused') : eligible ? 'eligible' : 'refused',
      },
    }
  })

  const reconnectLine = reconnectGesture.kind === 'dragging'
    ? {
        length: Math.hypot(reconnectGesture.pointer.x - reconnectGesture.origin.x, reconnectGesture.pointer.y - reconnectGesture.origin.y),
        angle: Math.atan2(reconnectGesture.pointer.y - reconnectGesture.origin.y, reconnectGesture.pointer.x - reconnectGesture.origin.x) * 180 / Math.PI,
      }
    : null
  const keyDown = (event: React.KeyboardEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement || target instanceof HTMLSelectElement || target.isContentEditable) return
    if (event.key === 'Escape' && reconnectGesture.kind === 'dragging') {
      setReconnectGesture({ kind: 'idle' })
      setConnectionNotice(null)
      event.preventDefault()
    } else if (selectedCanvasEdge !== null && event.key.toLowerCase() === 'r') {
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
  const refusedNotice = connectionNotice?.kind === 'missed-target' || connectionNotice?.kind === 'edge-refused'
  return (
    <div
      ref={hostRef}
      className={expanded
        ? 'fixed inset-3 z-(--z-dialog) flex flex-col overflow-hidden rounded-xl border border-edge bg-well float'
        : 'relative flex min-h-[16rem] flex-1 flex-col overflow-hidden rounded-xl border border-edge bg-well'}
      aria-label="Causal DAG editor"
      onKeyDown={keyDown}
    >
      {reconnectGesture.kind === 'dragging' && reconnectLine !== null && (
        <>
          <span
            aria-hidden
            className="pointer-events-none absolute z-(--z-canvas) h-0.5 origin-left bg-signal"
            style={{
              left: reconnectGesture.origin.x,
              top: reconnectGesture.origin.y,
              width: reconnectLine.length,
              transform: `rotate(${reconnectLine.angle}deg)`,
            }}
          />
          <span
            aria-hidden
            className="pointer-events-none absolute z-(--z-canvas) h-3 w-3 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-signal bg-panel"
            style={{ left: reconnectGesture.pointer.x, top: reconnectGesture.pointer.y }}
          />
        </>
      )}
      <div className="relative min-h-0 flex-1">
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
            setConnectingFrom(dagNodeFromCanvas(document, params.nodeId))
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
          connectionLineStyle={{ stroke: 'var(--color-signal)', strokeWidth: 1.6 }}
          fitView
          fitViewOptions={FIT_VIEW}
          minZoom={0.3}
          maxZoom={2}
          deleteKeyCode={null}
          nodesConnectable
          edgesReconnectable={false}
          connectionRadius={28}
          connectOnClick={false}
          zoomOnDoubleClick={false}
          zoomOnScroll={!viewLocked}
          preventScrolling={!viewLocked}
          proOptions={{ hideAttribution: true }}
        >
          <DagGrid />
          {selectedCanvasEdge !== null && (
            <EdgeActionBar
              edge={selectedCanvasEdge}
              onReverse={() => reverse(selectedCanvasEdge)}
              onConfound={() => {
                const edge = selectedCanvasEdge.data?.edge
                if (edge !== undefined) onEdgeConfounded(edge.id)
              }}
              onRemove={() => {
                const edge = selectedCanvasEdge.data?.edge
                if (edge !== undefined) onEdgeRemoved(edge.id)
              }}
              onEndpointPointerDown={(endpoint, event) => beginEndpointReconnect(selectedCanvasEdge, endpoint, event)}
            />
          )}
          <CanvasControls onTidy={tidy} viewLocked={viewLocked} onToggleLock={() => setViewLocked((locked) => !locked)} expanded={expanded} onToggleExpand={() => setExpanded((open) => !open)} />
          <RefitOnResize host={hostRef} />
        </ReactFlow>
      </div>
      <div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 border-t border-hair bg-panel px-3 py-1.5">
        <p
          role="status"
          className={`m-0 min-w-0 flex-1 text-label ${refusedNotice ? 'text-warn' : connectionNotice !== null ? 'text-ink' : 'text-faint'}`}
        >
          {connectionNotice === null ? IDLE_HINT : describeConnectionNotice(connectionNotice)}
        </p>
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
}
