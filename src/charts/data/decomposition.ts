import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, axisNameStyle, baseOption, tooltip, valueAxis, rangeSelection } from '../grammar'
import type { ChartTheme } from '../theme'

export interface StlChartView {
  readonly name: string
  readonly time: readonly number[]
  readonly calendar: boolean
  readonly observed: readonly number[]
  readonly trend: readonly number[]
  readonly seasonal: readonly number[]
  readonly remainder: readonly number[]
}

const day = (timestamp: number): string => new Date(timestamp).toISOString().slice(0, 10)

/** Statsmodels-compatible additive STL as four aligned panels sharing one time axis. */
export function decompositionOption(view: StlChartView, theme: ChartTheme): EChartsCoreOption {
  const components = [
    ['Observed', view.observed, theme.ink],
    ['Trend', view.trend, theme.info],
    ['Seasonal', view.seasonal, theme.ok],
    ['Remainder', view.remainder, theme.signal],
  ] as const
  const grids = components.map((_, index) => ({ left: 70, right: 18, top: `${3 + index * 24}%`, height: '18%' }))
  const xAxes = components.map((_, index) => ({
    type: 'value' as const,
    gridIndex: index,
    min: 'dataMin',
    max: 'dataMax',
    axisLine: { lineStyle: { color: theme.hair } },
    axisTick: { show: false },
    axisLabel: { ...axisLabelStyle(theme), show: index === components.length - 1, formatter: view.calendar ? (value: number) => day(value) : undefined },
    splitLine: { show: false },
  }))
  const yAxes = components.map(([name], index) => ({
    ...valueAxis(theme, name),
    gridIndex: index,
    scale: true,
    nameGap: 48,
    nameTextStyle: axisNameStyle(theme),
  }))
  return {
    ...baseOption(theme, `${view.name} additive STL decomposition. Four aligned panels show the observed series, fitted trend, repeating seasonal component, and remainder. At each observation, observed equals trend plus seasonal plus remainder.`),
    grid: grids,
    axisPointer: { link: [{ xAxisIndex: 'all' }] },
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const rows = Array.isArray(raw) ? raw : []
        if (rows.length === 0) return ''
        const first = rows[0]
        const point = first !== null && typeof first === 'object' ? Reflect.get(first, 'value') : null
        const time = Array.isArray(point) ? Number(point[0]) : Number.NaN
        const heading = view.calendar ? day(time) : `observation ${view.time.indexOf(time) + 1}`
        const values = rows.flatMap((row) => {
          if (row === null || typeof row !== 'object') return []
          const value = Reflect.get(row, 'value')
          return Array.isArray(value) ? [`${String(Reflect.get(row, 'seriesName'))}: <strong>${formatStatistic('raw', Number(value[1])).text}</strong>`] : []
        })
        return `${heading}<br/>${values.join('<br/>')}`
      },
    },
    xAxis: xAxes,
    yAxis: yAxes,
    ...rangeSelection(theme, [0, 1, 2, 3]),
    series: components.map(([name, values, colour], index) => ({
      type: 'line',
      name,
      xAxisIndex: index,
      yAxisIndex: index,
      data: values.map((value, row) => [view.time[row], value]),
      symbol: 'none',
      lineStyle: { color: colour, width: 1.2 },
    })),
  }
}
