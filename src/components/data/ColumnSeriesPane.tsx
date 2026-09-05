import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { seriesOverviewOption } from '@/charts/data/seriesOverview'
import { useChartTheme } from '@/charts/theme'
import { summariseWindow, type VisibleWindow } from '@/charts/window'
import { FigureParts } from '@/components/ui/figures'
import { caption, figureGrid, label, num } from '@/components/ui/recipes'
import type { PhysicalColumnProfile } from '@/domain/dataset'
import { formatAbsent, formatCount, formatStatistic, type Formatted } from '@/lib/format/number'
import type { ColumnDescription } from './useColumnProfile'

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
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const series = description.kind === 'ready' && column !== null && description.column === column.id ? description.series : null
  const option = useMemo(
    () => (series === null || column === null ? null : seriesOverviewOption({ name: column.name, values: series.values, stepLabel }, theme)),
    [column, series, stepLabel, theme],
  )
  const summary = useMemo(() => (series === null ? null : summariseWindow(series.values, window)), [series, window])
  if (column === null) return <p className="m-0 px-3 py-2 text-body text-faint">Select a numeric column to draw it in row order.</p>
  if (description.kind === 'loading' || description.kind === 'idle') return <div aria-hidden className="skeleton m-3 h-40 rounded-lg" />
  if (description.kind === 'failed') return <p className="m-0 px-3 py-2 text-body text-faint">The column profile was refused, so there is no series to draw.</p>
  if (option === null || series === null || summary === null) {
    return <p className="m-0 px-3 py-2 text-body text-faint">{column.name} is not numeric, so it has no series view. Its most frequent values are in the column profile.</p>
  }
  const scope = window === null ? `all ${stepLabel}s` : `${stepLabel}s ${formatCount(Math.ceil(window.start)).text} to ${formatCount(Math.floor(window.end)).text}`
  return (
    <div className="flex h-full flex-col px-3 py-2">
      <div className="flex flex-wrap items-baseline justify-between gap-x-3 gap-y-1">
        <p className={caption('m-0')}>{column.name} by {stepLabel}</p>
        <p className={num('m-0 text-label text-faint')}>{formatCount(series.values.length, { noun: `${stepLabel}s` }).text}{series.missingCells > 0 ? ` · ${formatCount(series.missingCells).text} missing` : ''}</p>
      </div>
      {/* The figures describe what the chart shows: narrow the chart and they narrow with it. */}
      <dl aria-label={`Summary of `} className={figureGrid('my-1.5 shrink-0 grid-cols-4')}>
        <Cell name="Shown" value={formatCount(summary.observed, { noun: `${stepLabel}s` })} />
        <Cell name="Minimum" value={statistic(summary.min)} />
        <Cell name="Mean" value={statistic(summary.mean)} />
        <Cell name="Maximum" value={statistic(summary.max)} />
      </dl>
      <EChart option={option} label={`${column.name} in ${stepLabel} order`} className="min-h-[120px] flex-1" testId="column-series" onWindow={setWindow} />
    </div>
  )
}
