import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { seriesOverviewOption } from '@/charts/data/seriesOverview'
import { useChartTheme } from '@/charts/theme'
import { summariseWindow, type VisibleWindow } from '@/charts/window'
import { FigureParts } from '@/components/ui/figures'
import { caption, figureGrid, label, num } from '@/components/ui/recipes'
import { isNumericDuckDbType, type PhysicalColumnProfile } from '@/domain/dataset'
import { formatAbsent, formatCount, formatStatistic, type Formatted } from '@/lib/format/number'
import type { ColumnDescription, ColumnSeries } from './useColumnProfile'

type DisplayedSeries = { readonly column: PhysicalColumnProfile; readonly series: ColumnSeries }

function Cell({ name, value }: { readonly name: string; readonly value: Formatted }) {
  return (
    <div className="bg-panel px-2.5 py-1">
      <dt className={label('text-faint')}>{name}</dt>
      <dd className={num('m-0 mt-0.5 text-body leading-none text-ink')} title={value.exact || value.srText}><FigureParts value={value} /></dd>
    </div>
  )
}

const statistic = (value: number): Formatted => (Number.isNaN(value) ? formatAbsent('unavailable') : formatStatistic('raw', value))

/** The bottom panel for the Data Studio: the selected numeric column in row order, with figures that follow the zoom. */
export function ColumnSeriesPane({ column, description, stepLabel }: {
  readonly column: PhysicalColumnProfile | null
  readonly description: ColumnDescription
  /** What one row means: "row" for independent observations, "observation" when the rows are ordered in time. */
  readonly stepLabel: string
}) {
  const theme = useChartTheme()
  const [displayed, setDisplayed] = useState<DisplayedSeries | null>(null)
  const [zoom, setZoom] = useState<{ readonly series: ColumnSeries; readonly window: VisibleWindow | null } | null>(null)
  const current = description.kind !== 'idle' && description.column === column?.id
  const ready = current && description.kind === 'ready' ? description.series : null
  // Retain the last labelled plot while the worker describes the next column.
  // Adjust before children render, so labels and values change together.
  if (ready !== null && column !== null && displayed?.series !== ready) {
    setDisplayed({ column, series: ready })
  }
  const series = displayed?.series ?? null
  const shown = displayed?.column ?? null
  const window = zoom?.series === series ? zoom?.window ?? null : null
  const pending = !current || description.kind === 'loading'
  const option = useMemo(
    () => (series === null || shown === null ? null : seriesOverviewOption({ name: shown.name, values: series.values, stepLabel }, theme)),
    [shown, series, stepLabel, theme],
  )
  const summary = useMemo(() => (series === null ? null : summariseWindow(series.values, window)), [series, window])
  if (column === null) return <p className="m-0 px-3 py-2 text-body text-faint">Select a numeric column to draw it in row order.</p>
  if (current && description.kind === 'failed') return <p className="m-0 px-3 py-2 text-body text-faint">The column could not be profiled, so its series cannot be displayed.</p>
  if (!isNumericDuckDbType(column.duckdbType)) {
    return <p className="m-0 px-3 py-2 text-body text-faint">{column.name} is not numeric, so it has no series view. Its most frequent values are in the column profile.</p>
  }
  if (current && description.kind === 'ready' && ready === null) return <p className="m-0 px-3 py-2 text-body text-faint">The numeric series could not be loaded.</p>
  if (option === null || series === null || summary === null || shown === null) return <div aria-hidden className="skeleton m-3 h-40 rounded-lg" />
  return (
    <div className="flex h-full flex-col px-3 py-2" aria-busy={pending}>
      <div className="flex flex-wrap items-baseline justify-between gap-x-3 gap-y-1">
        <p className={caption('m-0')}>{shown.name} by {stepLabel}<span className="sr-only">{pending ? `; loading ${column.name}` : ''}</span></p>
        <p className={num('m-0 flex flex-wrap gap-x-4 text-label text-faint')}><span>{formatCount(series.values.length, { noun: `${stepLabel}s` }).text}</span>{series.missingCells > 0 && <span>{formatCount(series.missingCells).text} missing</span>}</p>
      </div>
      {/* The figures describe what the chart shows: narrow the chart and they narrow with it. */}
      <dl aria-label={`Summary of ${shown.name}`} className={figureGrid('my-1.5 shrink-0 grid-cols-4')}>
        <Cell name="Shown" value={formatCount(summary.observed, { noun: `${stepLabel}s` })} />
        <Cell name="Minimum" value={statistic(summary.min)} />
        <Cell name="Mean" value={statistic(summary.mean)} />
        <Cell name="Maximum" value={statistic(summary.max)} />
      </dl>
      <EChart option={option} label={`${shown.name} in ${stepLabel} order`} className="min-h-[120px] flex-1" testId="column-series" window={window} onWindow={(window) => setZoom({ series, window })} />
    </div>
  )
}
