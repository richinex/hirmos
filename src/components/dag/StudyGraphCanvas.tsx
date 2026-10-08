import { useEffect, useMemo, useRef, useState, type ReactNode } from 'react'
import { Panel } from '@xyflow/react'
import {
  CanvasControls,
  canvasModel,
  studyLayout,
  useCanvasView,
  useDrawingStyle,
  visibleDagGraph,
  type VariableView,
} from './DagCanvas'
import { ReadOnlyCanvas, type ReadOnlyCanvasState } from './ReadOnlyCanvas'
import { dagCardSize } from './dagCardSize'
import { FloatingFigure } from '@/charts/FloatingFigure'
import { useTextMetricsVersion } from '@/lib/textMetrics'
import { analyseDagCausalFlow, type DagCausalFlow } from '@/domain/dagFlow'
import type { DagDocument, DagNodeId, EditableDag } from '@/domain/dag'

type Model = ReturnType<typeof canvasModel>

export function StudyGraphCanvas({
  document,
  treatment,
  outcome,
}: {
  readonly document: DagDocument
  readonly treatment: DagNodeId | null
  readonly outcome: DagNodeId | null
}) {
  const [variableView, setVariableView] = useState<VariableView>('connected')
  const graph = document.current.graph
  const visible = useMemo(() => visibleDagGraph(graph, variableView), [graph, variableView])
  const flow = useMemo(
    () =>
      treatment === null || outcome === null || treatment === outcome
        ? null
        : analyseDagCausalFlow(graph, treatment, outcome),
    [graph, treatment, outcome],
  )
  const label = `${document.name} graph`
  const shared = {
    document,
    graph: visible,
    flow,
    label,
    layoutKey: `${document.id}\u0000${treatment ?? ''}\u0000${outcome ?? ''}\u0000${variableView}`,
  }
  return (
    <FloatingFigure
      label={label}
      defaultHeight={720}
      figure={
        <StudyGraph
          {...shared}
          className="min-h-0 flex-1"
          frame="none"
          variables={{
            view: variableView,
            disconnected: graph.nodes.length - visibleDagGraph(graph, 'connected').nodes.length,
            onToggle: () => setVariableView((current) => (current === 'all' ? 'connected' : 'all')),
          }}
        />
      }
    >
      {(openButton) => <StudyGraph {...shared} className="h-80" overlay={openButton} />}
    </FloatingFigure>
  )
}

function StudyGraph({
  document,
  graph,
  flow,
  label,
  layoutKey,
  className,
  frame,
  variables,
  overlay,
}: {
  readonly document: DagDocument
  readonly graph: EditableDag
  readonly flow: DagCausalFlow | null
  readonly label: string
  readonly layoutKey: string
  readonly className: string
  readonly frame?: 'well' | 'none'
  readonly variables?: Parameters<typeof CanvasControls>[0]['variables']
  readonly overlay?: ReactNode
}) {
  const hostRef = useRef<HTMLDivElement>(null)
  const canvasView = useCanvasView(hostRef)
  const { orientation } = canvasView
  const [drawing] = useDrawingStyle()
  const metrics = useTextMetricsVersion()
  const [attempt, setAttempt] = useState(0)
  const previousKey = useRef(layoutKey)
  const layoutInput = JSON.stringify([
    layoutKey,
    document.current.graph,
    graph,
    flow?.treatment,
    flow?.outcome,
    orientation,
    drawing,
    metrics,
  ])
  const [state, setState] = useState<ReadOnlyCanvasState<Model['edges'][number]>>({
    kind: 'loading',
  })
  useEffect(() => {
    let active = true
    const controller = new AbortController()
    const preserve = previousKey.current === layoutKey
    previousKey.current = layoutKey
    setState((state) => ({
      kind: 'loading',
      previous: preserve ? (state.kind === 'ready' ? state : state.previous) : undefined,
    }))
    const size = dagCardSize(document.current.graph.nodes.map((node) => node.name))
    void studyLayout(graph, flow, orientation, size, controller.signal).then((layout) => {
      if (!active) return
      if (!layout.ok) {
        setState((state) => {
          const previous = state.kind === 'ready' ? state : state.previous
          return {
            kind: 'failed',
            previous,
            message:
              previous === undefined
                ? 'The graph could not be laid out.'
                : 'The new layout failed. The previous drawing is shown.',
          }
        })
        return
      }
      const model = canvasModel(document, null, flow, null, layout.value, false, drawing)
      setState({ kind: 'ready', nodes: model.nodes, edges: model.edges })
    })
    return () => {
      active = false
      controller.abort()
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- layoutInput records graph, target, view and font inputs by value
  }, [layoutInput, attempt])
  return (
    <ReadOnlyCanvas
      host={hostRef}
      view={canvasView}
      state={state}
      onRetry={() => setAttempt((attempt) => attempt + 1)}
      label={label}
      loading="Laying out the graph…"
      layoutKey={`${layoutKey}\u0000${orientation}`}
      className={className}
      frame={frame}
    >
      {variables === undefined ? (
        overlay !== null &&
        overlay !== undefined && <Panel position="bottom-right">{overlay}</Panel>
      ) : (
        <CanvasControls view={canvasView} expand={false} variables={variables} />
      )}
    </ReadOnlyCanvas>
  )
}
