import { Metadata } from '@/components/ui/Metadata'
import { LagGridCanvas } from './LagGridCanvas'
import { useEffect, useMemo, useState, type ReactNode } from 'react'
import type { EChartsCoreOption, EChartsType } from 'echarts/core'
import { assertNever } from '@/domain/dop'
import { useElementSize } from '@/lib/useElementWidth'
import { EChart } from '@/charts/EChart'
import { FloatingFigure } from '@/charts/FloatingFigure'
import { useChartExport } from '@/charts/useChartExport'
import {
  lagGridMetrics,
  lagGridOption,
  lagGridSize,
  summaryGraphOption,
} from '@/charts/discovery/lagGraphs'
import { useChartTheme } from '@/charts/theme'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import {
  summarizeLagGraph,
  type LagGraph,
  type LagGraphSemantics,
  type LagGraphWarning,
} from '@/domain/lagGraph'
import { well } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'

interface MarkMeaning {
  readonly mark: string
  readonly meaning: string
}

/**
 * What each link mark says about the pair it joins, in the words of the reference implementation:
 * the `run_lpcmci` docstring (tigramite/lpcmci.py, "Graph of causal relations") for the PAG marks,
 * the `run_pcmciplus` docstring (tigramite/pcmci.py, link_assumptions) for the stationary lag graph.
 * The table's Mark column prints the same three-character strings. The docstring adds "and there is
 * a link between them" to every mark; the drawn link says that, so the legend leaves it out.
 */
const markMeanings = (semantics: LagGraphSemantics): readonly MarkMeaning[] => {
  switch (semantics) {
    case 'cpdag':
      return [
        { mark: '-->', meaning: 'the direction is compelled within the fitted equivalence class' },
        { mark: '---', meaning: 'the variables are adjacent; the direction is not determined' },
      ]
    case 'pag':
      return [
        { mark: '-->', meaning: 'the source is an ancestor of the target' },
        { mark: '<->', meaning: 'neither is an ancestor of the other' },
        {
          mark: 'o->',
          meaning: 'the target is not an ancestor of the source; the circle end is undetermined',
        },
        { mark: 'o-o', meaning: 'contemporaneous; neither end is oriented' },
        { mark: 'x', meaning: 'ambiguous endpoint information' },
      ]
    case 'stationary-lag-graph':
    case 'joint-stationary-lag-graph':
    case 'nonstationary-lag-graph':
    case 'regime-specific-lag-graph':
      return [
        { mark: '-->', meaning: 'directed; lagged links always point forward in time' },
        { mark: 'o-o', meaning: 'contemporaneous; neither end is oriented' },
        { mark: 'x-x', meaning: 'contemporaneous; orientation rules conflict' },
      ]
    case 'weighted-directed-evidence':
    case 'lagged-information':
    case 'neural-lagged-granger':
      return []
    default:
      return assertNever(semantics)
  }
}

/**
 * One drawn sample of a link mark, so the legend is a key to the picture rather than a gloss on it.
 *
 * The endpoints repeat what the chart draws for each `LagEndpoint`: nothing for a tail, a filled head
 * for an arrow, an open circle, and the cross for a conflicting end. The sample stays in the text
 * colour on purpose — in the graph a link's colour carries its strength and sign, so colouring the
 * key would say that the mark itself has a colour.
 */
function MarkSample({ mark }: { readonly mark: string }) {
  const [left, right] = mark.length === 3 ? [mark[0], mark[2]] : ['', mark[0]]
  const end = (side: 'left' | 'right', glyph: string) => {
    const x = side === 'left' ? 7 : 39
    switch (glyph) {
      case '>':
        return <path d={`M${x - 6},3.5 L${x},7 L${x - 6},10.5 Z`} fill="currentColor" />
      case '<':
        return <path d={`M${x + 6},3.5 L${x},7 L${x + 6},10.5 Z`} fill="currentColor" />
      case 'o':
        return <circle cx={x} cy={7} r={3} fill="none" stroke="currentColor" strokeWidth={1.3} />
      case 'x':
        return (
          <path
            d={`M${x - 3},4 L${x + 3},10 M${x + 3},4 L${x - 3},10`}
            stroke="currentColor"
            strokeWidth={1.4}
            strokeLinecap="round"
          />
        )
      default:
        return null
    }
  }
  return (
    <svg
      viewBox="0 0 46 14"
      width={46}
      height={14}
      aria-hidden
      focusable="false"
      className="shrink-0 self-center"
    >
      <line
        x1={7}
        y1={7}
        x2={39}
        y2={7}
        stroke="currentColor"
        strokeWidth={1.4}
        strokeLinecap="round"
      />
      {end('left', left)}
      {end('right', right)}
    </svg>
  )
}

type LagGraphView = 'summary' | 'lag-grid'

/**
 * A lag graph as a summary graph or a lag grid, in place and in a floating window. The view is one
 * state shared by both, so lifting the figure shows the view being read and switching it in the
 * window switches it in the panel.
 */
export function LagGraphViews({
  graph,
  warnings = [],
  label,
  highlighted = [],
  initial = 'summary',
  compact = false,
}: {
  readonly graph: LagGraph
  readonly warnings?: readonly LagGraphWarning[]
  /** Accessible name for the rendered chart, for example "PCMCI+ evidence graph". */
  readonly label: string
  /** Variable ids to outline in the summary graph, for example the selected candidate's endpoints. */
  readonly highlighted?: readonly string[]
  readonly initial?: LagGraphView
  readonly compact?: boolean
}) {
  const [view, setView] = useState<LagGraphView>(initial)
  const exporter = useChartExport(label, 'lag-graph')
  const figure = { graph, warnings, label, highlighted, view, onView: setView }
  return (
    <FloatingFigure
      label={label}
      defaultHeight={720}
      actions={exporter.buttons}
      notice={exporter.problem}
      figure={<LagGraphFigure {...figure} fill onChart={exporter.register} />}
    >
      {(openButton) => (
        <LagGraphFigure {...figure} compact={compact} openButton={openButton} testIds />
      )}
    </FloatingFigure>
  )
}

function LagGraphFigure({
  graph,
  warnings,
  label,
  highlighted,
  view,
  onView,
  compact = false,
  fill = false,
  openButton = null,
  testIds = false,
  onChart,
}: {
  readonly graph: LagGraph
  readonly warnings: readonly LagGraphWarning[]
  readonly label: string
  readonly highlighted: readonly string[]
  readonly view: LagGraphView
  readonly onView: (view: LagGraphView) => void
  readonly compact?: boolean
  /** Fill the host, as in the floating window, instead of resting at the panel size. */
  readonly fill?: boolean
  readonly openButton?: ReactNode
  readonly testIds?: boolean
  /** The drawn chart and its option, for export. */
  readonly onChart?: (chart: EChartsType, option: EChartsCoreOption) => void
}) {
  const theme = useChartTheme()
  const summary = useMemo(() => summarizeLagGraph(graph), [graph])
  const [host, hostWidth, hostHeight] = useElementSize<HTMLDivElement>()
  const [chart, setChart] = useState<EChartsType | null>(null)
  const summaryMetrics = useMemo(() => {
    if (fill)
      return {
        width: Math.max(240, hostWidth),
        height: Math.max(240, hostHeight),
        nodeSize: Math.min(64, Math.max(46, Math.floor(Math.min(hostWidth, hostHeight) / 9))),
      }
    return compact
      ? { width: Math.max(200, Math.min(hostWidth, 340)), height: 260, nodeSize: 40 }
      : {
          width: Math.max(240, Math.min(hostWidth, 560)),
          height: Math.min(380, 200 + 36 * graph.variables.length),
          nodeSize: 46,
        }
  }, [compact, fill, graph.variables.length, hostHeight, hostWidth])
  const gridMetrics = useMemo(() => lagGridMetrics(graph), [graph])
  const option = useMemo(
    () =>
      view === 'summary'
        ? summaryGraphOption(summary, theme, summaryMetrics, highlighted)
        : lagGridOption(graph, theme, gridMetrics),
    [graph, gridMetrics, highlighted, summary, summaryMetrics, theme, view],
  )
  useEffect(() => {
    if (chart !== null) onChart?.(chart, option)
  }, [chart, onChart, option])
  const gridSize = lagGridSize(graph, gridMetrics)
  const height = view === 'summary' ? summaryMetrics.height : gridSize.height
  const meanings = markMeanings(graph.semantics)
  return (
    <div className={cn(well('p-2'), fill && 'flex min-h-0 flex-1 flex-col')}>
      <div className="mb-1 flex flex-wrap items-center justify-between gap-2 px-1">
        {graph.tauMax > 0 ? (
          <SegmentedControl
            variant="line"
            size="sm"
            ariaLabel="Structure view"
            value={view}
            onChange={onView}
            options={[
              { value: 'summary', label: 'Summary' },
              { value: 'lag-grid', label: 'Lag grid' },
            ]}
          />
        ) : (
          <span className="text-micro text-faint">Directed structure</span>
        )}
        <span className="flex items-center gap-2 text-micro text-faint">
          {graph.links.length} link
          {graph.links.length === 1 ? '' : 's'}
          {graph.tauMax > 0 ? (
            <Metadata>
              <span></span>
              <span>τ max {graph.tauMax}</span>
            </Metadata>
          ) : (
            <Metadata>
              <span></span>
              <span>same-period</span>
            </Metadata>
          )}
          {openButton}
        </span>
      </div>
      <div
        ref={host}
        data-testid="lag-graph-viewport"
        className={cn(
          view === 'lag-grid' ? 'relative overflow-hidden' : 'flex justify-center',
          view === 'lag-grid' && !fill && 'h-[520px]',
          fill && 'min-h-0 flex-1',
        )}
      >
        {view === 'summary' ? (
          hostWidth > 0 && (
            <EChart
              key="summary"
              option={option}
              label={label}
              className="block"
              style={{ width: summaryMetrics.width, height }}
              testId={testIds ? 'summary-graph' : undefined}
              onReady={setChart}
            />
          )
        ) : (
          <LagGridCanvas
            option={option}
            label={label}
            width={gridSize.width}
            height={height}
            testId={testIds ? 'lag-grid' : undefined}
            onReady={setChart}
          />
        )}
      </div>
      {meanings.length > 0 && (
        <dl
          className="mb-0 mt-2 grid grid-cols-[auto_minmax(0,1fr)] items-center gap-x-3 gap-y-1 px-1 text-label text-faint"
          aria-label="Link mark legend"
        >
          {[...meanings, { mark: '<--', meaning: 'mirrored: the same with the ends swapped' }].map(
            (entry) => (
              <div key={entry.mark} className="contents">
                {/* The drawing is the key. Printing the notation beside it repeats the same mark twice. */}
                <dt className="text-muted" aria-label={entry.mark}>
                  <MarkSample mark={entry.mark} />
                </dt>
                <dd className="m-0">{entry.meaning}</dd>
              </div>
            ),
          )}
        </dl>
      )}
      {warnings.length > 0 && (
        <p className="mb-0 mt-2 px-1 text-label text-warn" role="status">
          <Metadata>
            <span>Unsupported mark{warnings.length === 1 ? '' : 's'}</span>
            <span>{warnings.map((warning) => warning.mark || '(empty)').join(', ')}</span>
            <span>drawn unresolved</span>
          </Metadata>
        </p>
      )}
    </div>
  )
}
