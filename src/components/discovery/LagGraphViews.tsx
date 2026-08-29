import { useMemo, useState } from 'react'
import { assertNever } from '@/domain/dop'
import { useElementWidth } from '@/lib/useElementWidth'
import { EChart } from '@/charts/EChart'
import { lagGridOption, lagGridSize, summaryGraphOption } from '@/charts/discovery/lagGraphs'
import { useChartTheme } from '@/charts/theme'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { summarizeLagGraph, type LagGraph, type LagGraphSemantics } from '@/domain/lagGraph'
import { literal } from '@/components/ui/recipes'

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
    ]
    case 'stationary-lag-graph': return [
      { mark: '-->', meaning: 'directed; lagged links always point forward in time' },
      { mark: 'o-o', meaning: 'contemporaneous; neither end is oriented' },
    ]
    case 'weighted-directed-evidence':
    case 'lagged-information':
    case 'temporal-dag': return []
    default: return assertNever(semantics)
  }
}

export function LagGraphViews({ graph, label, highlighted = [], initial = 'summary', compact = false }: {
  readonly graph: LagGraph
  /** Accessible name for the container, for example "PCMCI+ evidence graph". */
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
    <div className="rounded-lg border border-hair bg-well p-2" aria-label={label}>
      <div className="mb-1 flex flex-wrap items-center justify-between gap-2 px-1">
        <SegmentedControl size="sm" className="bg-panel" ariaLabel="Structure view" value={view} onChange={setView} options={[{ value: 'summary', label: 'Summary' }, { value: 'lag-grid', label: 'Lag grid' }]} />
        <span className="text-micro text-faint">{graph.links.length} link{graph.links.length === 1 ? '' : 's'} · τ max {graph.tauMax}</span>
      </div>
      <div ref={host} className={view === 'lag-grid' ? 'panel-scroll overflow-x-auto' : 'flex justify-center'}>
        {view === 'summary'
          ? (hostWidth > 0 && <EChart key="summary" option={option} label={label} className="block" style={{ width: summaryMetrics.width, height }} testId="summary-graph" />)
          : <EChart key="lag-grid" option={option} label={label} className="block" style={{ width: gridSize.width, height, minWidth: gridSize.width }} testId="lag-grid" />}
      </div>
      {meanings.length > 0 && (
        <dl className="mb-0 mt-2 grid grid-cols-[auto_minmax(0,1fr)] gap-x-3 gap-y-0.5 px-1 text-label text-faint" aria-label="Link mark legend">
          {meanings.map((entry) => (
            <div key={entry.mark} className="contents">
              <dt className={literal('text-muted')}>{entry.mark}</dt>
              <dd className="m-0">{entry.meaning}</dd>
            </div>
          ))}
          <dt className={literal('text-muted')}>{'<--'}</dt>
          <dd className="m-0">mirrored: the same with the ends swapped</dd>
        </dl>
      )}
    </div>
  )
}
