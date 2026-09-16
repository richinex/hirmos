import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { baseOption, tooltip } from '../grammar'
import { variableColour, type ChartTheme } from '../theme'

export interface RunComparisonRow {
  readonly label: string
  readonly estimate: number
  readonly lower: number | null
  readonly upper: number | null
  /** The study's current accepted answer takes the signal colour. */
  readonly current: boolean
}

/** Every comparable run on one axis: its interval as a horizontal line, its estimate as a dot, and the zero reference. */
export function runComparisonOption(rows: readonly RunComparisonRow[], theme: ChartTheme): EChartsCoreOption {
  const labels = rows.map((row) => row.label)
  const colour = (row: RunComparisonRow) => (row.current ? theme.signal : variableColour(theme, row.label))
  const description = `Estimates compared: ${rows.map((row) => `${row.label} ${formatStatistic('raw', row.estimate).text}`).join('; ')}.`
  return {
    ...baseOption(theme, description),
    grid: { left: 8, right: 16, top: 10, bottom: 28, containLabel: true },
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const index = raw !== null && typeof raw === 'object' ? Number(Reflect.get(raw, 'dataIndex')) : Number.NaN
        const row = rows[index]
        if (row === undefined) return ''
        const interval = row.lower === null || row.upper === null ? 'no interval' : `[${formatStatistic('raw', row.lower).text}, ${formatStatistic('raw', row.upper).text}]`
        return `${row.label}<br/>estimate <strong>${formatStatistic('raw', row.estimate).text}</strong><br/>${interval}`
      },
    },
    xAxis: {
      type: 'value',
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true, formatter: (value: number) => formatStatistic('raw', value).text },
      splitNumber: 4,
      splitLine: { lineStyle: { color: theme.hair } },
    },
    yAxis: {
      type: 'category',
      data: labels,
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, width: 240, overflow: 'truncate' },
    },
    series: [
      ...rows.map((row, index) => ({
        type: 'line' as const,
        name: `${row.label} interval`,
        data: row.lower === null || row.upper === null ? [] : [[row.lower, index], [row.upper, index]],
        lineStyle: { color: colour(row), width: 2 },
        symbol: 'none',
        silent: true,
        tooltip: { show: false },
      })),
      {
        type: 'scatter',
        name: 'estimate',
        data: rows.map((row, index) => ({ value: [row.estimate, index], itemStyle: { color: colour(row) } })),
        symbolSize: 9,
        markLine: {
          silent: true,
          symbol: 'none',
          label: { show: false },
          data: [{ xAxis: 0, lineStyle: { color: theme.muted, type: 'dashed', width: 1 } }],
        },
      },
    ],
  }
}
