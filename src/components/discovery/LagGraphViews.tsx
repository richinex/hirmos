import { useMemo, useState } from 'react'
import { assertNever } from '@/domain/dop'
import { useElementWidth } from '@/lib/useElementWidth'
import { EChart } from '@/charts/EChart'
import { lagGridOption, lagGridSize, summaryGraphOption } from '@/charts/discovery/lagGraphs'
import { useChartTheme } from '@/charts/theme'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { summarizeLagGraph, type LagGraph, type LagGraphSemantics, type LagGraphWarning } from '@/domain/lagGraph'
import { literal, well } from '@/components/ui/recipes'

interface MarkMeaning { readonly mark: string; readonly meaning: string }

/**
 * What each link mark says about the pair it joins, in the words of the reference implementation:
 * the `run_lpcmci` docstring (tigramite/lpcmci.py, "Graph of causal relations") for the PAG marks,
 * the `run_pcmciplus` docstring (tigramite/pcmci.py, link_assumptions) for the stationary lag graph.
 * The table's Mark column prints the same three-character strings. The docstring adds "and there is
 * a link between them" to every mark; the drawn link says that, so the legend leaves it out.
 */
const markMeanings = (semantics: LagGraphSemantics): readonly MarkMeaning[] => {
  switch (semantics) {
    case 'pag': return [
      { mark: '-->', meaning: 'the source is an ancestor of the target' },
      { mark: '<->', meaning: 'neither is an ancestor of the other' },
      { mark: 'o->', meaning: 'the target is not an ancestor of the source; the circle end is undetermined' },
      { mark: 'o-o', meaning: 'contemporaneous; neither end is oriented' },
      { mark: 'x', meaning: 'ambiguous endpoint information' },
    ]
    case 'stationary-lag-graph':
    case 'nonstationary-lag-graph':
    case 'regime-specific-lag-graph': return [
      { mark: '-->', meaning: 'directed; lagged links always point forward in time' },
      { mark: 'o-o', meaning: 'contemporaneous; neither end is oriented' },
      { mark: 'x-x', meaning: 'contemporaneous; orientation rules conflict' },
    ]
    case 'weighted-directed-evidence':
    case 'lagged-information':
    case 'neural-lagged-granger':
    case 'temporal-dag': return []
    default: return assertNever(semantics)
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
      case '>': return <path d={`M${x - 6},3.5 L${x},7 L${x - 6},10.5 Z`} fill="currentColor" />
      case '<': return <path d={`M${x + 6},3.5 L${x},7 L${x + 6},10.5 Z`} fill="currentColor" />
      case 'o': return <circle cx={x} cy={7} r={3} fill="none" stroke="currentColor" strokeWidth={1.3} />
      case 'x': return <path d={`M${x - 3},4 L${x + 3},10 M${x + 3},4 L${x - 3},10`} stroke="currentColor" strokeWidth={1.4} strokeLinecap="round" />
      default: return null
    }
  }
  return (
    <svg viewBox="0 0 46 14" width={46} height={14} aria-hidden focusable="false" className="shrink-0 self-center">
      <line x1={7} y1={7} x2={39} y2={7} stroke="currentColor" strokeWidth={1.4} strokeLinecap="round" />
      {end('left', left)}
      {end('right', right)}
    </svg>
  )
}

export function LagGraphViews({ graph, warnings = [], label, highlighted = [], initial = 'summary', compact = false }: {
  readonly graph: LagGraph
  readonly warnings?: readonly LagGraphWarning[]
  /** Accessible name for the rendered chart, for example "PCMCI+ evidence graph". */
  readonly label: string
  /** Variable ids to outline in the summary graph, for example the selected candidate's endpoints. */
  readonly highlighted?: readonly string[]
  readonly initial?: 'summary' | 'lag-grid'
  readonly compact?: boolean
}) {
  const theme = useChartTheme()
  const [view, setView] = useState<'summary' | 'lag-grid'>(initial)
  const summary = useMemo(() => summarizeLagGraph(graph), [graph])
  const [host, hostWidth] = useElementWidth<HTMLDivElement>()
  const summaryMetrics = useMemo(
    () => (compact
      ? { width: Math.max(200, Math.min(hostWidth, 340)), height: 260, nodeSize: 40 }
      : { width: Math.max(240, Math.min(hostWidth, 560)), height: Math.min(380, 200 + 36 * graph.variables.length), nodeSize: 46 }),
    [compact, graph.variables.length, hostWidth],
  )
  const option = useMemo(
    () => (view === 'summary'
      ? summaryGraphOption(summary, theme, summaryMetrics, highlighted)
      : lagGridOption(graph, theme)),
    [graph, highlighted, summary, summaryMetrics, theme, view],
  )
  const gridSize = lagGridSize(graph)
  const height = view === 'summary' ? summaryMetrics.height : Math.min(gridSize.height, 520)
  const meanings = markMeanings(graph.semantics)
  return (
    <div className={well('p-2')}>
      <div className="mb-1 flex flex-wrap items-center justify-between gap-2 px-1">
        {graph.tauMax > 0
          ? <SegmentedControl size="sm" frame="none" ariaLabel="Structure view" value={view} onChange={setView} options={[{ value: 'summary', label: 'Summary' }, { value: 'lag-grid', label: 'Lag grid' }]} />
          : <span className="text-micro text-faint">Directed structure</span>}
        <span className="text-micro text-faint">{graph.links.length} {graph.semantics === 'temporal-dag' ? 'arrow' : 'link'}{graph.links.length === 1 ? '' : 's'}{graph.tauMax > 0 ? ` · τ max ${graph.tauMax}` : ' · same-period'}</span>
      </div>
      <div ref={host} className={view === 'lag-grid' ? 'figure-strip overflow-x-auto' : 'flex justify-center'}>
        {view === 'summary'
          ? (hostWidth > 0 && <EChart key="summary" option={option} label={label} className="block" style={{ width: summaryMetrics.width, height }} testId="summary-graph" />)
          : <EChart key="lag-grid" option={option} label={label} className="block" style={{ width: gridSize.width, height, minWidth: gridSize.width }} testId="lag-grid" />}
      </div>
      {meanings.length > 0 && (
        <dl className="mb-0 mt-2 grid grid-cols-[auto_auto_minmax(0,1fr)] items-center gap-x-3 gap-y-1 px-1 text-label text-faint" aria-label="Link mark legend">
          {[...meanings, { mark: '<--', meaning: 'mirrored: the same with the ends swapped' }].map((entry) => (
            <div key={entry.mark} className="contents">
              <dt className="text-muted"><MarkSample mark={entry.mark} /></dt>
              {/* The table's Mark column prints these same strings, so the notation stays beside the drawing. */}
              <dd className={literal('m-0 text-muted')}>{entry.mark}</dd>
              <dd className="m-0">{entry.meaning}</dd>
            </div>
          ))}
        </dl>
      )}
      {warnings.length > 0 && (
        <p className="mb-0 mt-2 px-1 text-label text-warn" role="status">
          Unsupported mark{warnings.length === 1 ? '' : 's'} · {warnings.map((warning) => warning.mark || '(empty)').join(', ')} · drawn unresolved
        </p>
      )}
    </div>
  )
}
