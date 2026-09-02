import { useMemo } from 'react'
import { EChart } from '@/charts/EChart'
import { seriesOverviewOption } from '@/charts/data/seriesOverview'
import { useChartTheme } from '@/charts/theme'
import { caption, num } from '@/components/ui/recipes'
import type { PhysicalColumnProfile } from '@/domain/dataset'
import { formatCount } from '@/lib/format/number'
import type { ColumnDescription } from './useColumnProfile'

/** The bottom panel for the Data Studio: the selected numeric column in row order. */
export function ColumnSeriesPane({ column, description, stepLabel }: {
  readonly column: PhysicalColumnProfile | null
  readonly description: ColumnDescription
  /** What one row means: "row" for independent observations, "observation" when the rows are ordered in time. */
  readonly stepLabel: string
}) {
  const theme = useChartTheme()
  const series = description.kind === 'ready' && column !== null && description.column === column.id ? description.series : null
  const option = useMemo(
    () => (series === null || column === null ? null : seriesOverviewOption({ name: column.name, values: series.values, stepLabel }, theme)),
    [column, series, stepLabel, theme],
  )
  if (column === null) return <p className="m-0 px-3 py-2 text-body text-faint">Select a numeric column to draw it in row order.</p>
  if (description.kind === 'loading' || description.kind === 'idle') return <div aria-hidden className="skeleton m-3 h-40 rounded-lg" />
  if (description.kind === 'failed') return <p className="m-0 px-3 py-2 text-body text-faint">The column profile was refused, so there is no series to draw.</p>
  if (option === null || series === null) {
    return <p className="m-0 px-3 py-2 text-body text-faint">{column.name} is not numeric, so it has no series view. Its most frequent values are in the column profile.</p>
  }
  return (
    <div className="flex h-full flex-col px-3 py-2">
      <div className="flex items-baseline justify-between gap-3">
        <p className={caption('m-0')}>{column.name} by {stepLabel}</p>
        <p className={num('m-0 text-label text-faint')}>{formatCount(series.values.length, { noun: `${stepLabel}s` }).text}{series.missingCells > 0 ? ` · ${formatCount(series.missingCells).text} missing` : ''}</p>
      </div>
      <EChart option={option} label={`${column.name} in ${stepLabel} order`} className="min-h-[160px] flex-1" testId="column-series" />
    </div>
  )
}
