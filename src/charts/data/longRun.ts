import type { EChartsCoreOption } from 'echarts/core'
import type { PlotTime } from '@/domain/longRun'
import { axisLabelStyle, baseOption, gridAuto, legend, rangeSelection, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export function longRunOption(view: {
  readonly title: string
  readonly axis: PlotTime
  readonly startRow: number
  readonly series: readonly { readonly name: string; readonly values: readonly number[] }[]
  readonly slider: boolean
  readonly zero: boolean
}, theme: ChartTheme): EChartsCoreOption {
  const times = view.axis.values
  return {
    ...baseOption(theme, view.title),
    useUTC: true,
    grid: gridAuto({ top: 42, bottom: view.slider ? 78 : 30 }),
    ...rangeSelection(theme, 0, { slider: view.slider }),
    legend: { ...legend(theme, view.series.map((s) => s.name)), bottom: 'auto', top: 0 },
    tooltip: tooltip(theme, 'axis'),
    xAxis: {
      type: view.axis.kind === 'calendar' ? 'time' : 'value',
      min: times[0], max: times[times.length - 1],
      axisLabel: axisLabelStyle(theme), axisTick: { show: false },
      axisLine: { lineStyle: { color: theme.hair } }, splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme, ''), scale: true },
    series: view.series.map((series, i) => ({
      name: series.name, type: 'line', symbol: 'none',
      data: series.values.map((value, row) => [times[row + view.startRow], value]),
      itemStyle: { color: i === 0 ? theme.info : theme.signal },
      lineStyle: { color: i === 0 ? theme.info : theme.signal, width: 1.5, type: i === 0 ? 'solid' : 'dashed' },
      ...(view.zero && i === 0 ? { markLine: { silent: true, symbol: 'none', label: { show: false }, lineStyle: { color: theme.muted, type: 'dashed', width: 1 }, data: [{ yAxis: 0 }] } } : {}),
    })),
  }
}
