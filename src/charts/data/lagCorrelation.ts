import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, axisNameStyle, baseOption, tooltip, valueAxis, rangeSelection } from '../grammar'
import type { ChartTheme } from '../theme'

export interface LagCorrelationView {
  readonly name: string
  readonly acf: readonly number[]
  readonly acfLimits: readonly number[]
  readonly pacf: readonly number[]
  readonly pacfLimits: readonly number[]
}

/** ACF and plot_pacf(method="ywm") with statsmodels' 95% reference bands. */
export function lagCorrelationOption(view: LagCorrelationView, theme: ChartTheme): EChartsCoreOption {
  // statsmodels draws lag 0, where the correlation is 1 by definition and the kernel reports a
  // zero-width band. Keeping it makes the axis line up with plot_acf for parity reading.
  const lags = Array.from({ length: view.acf.length }, (_, index) => index)
  const panels = [
    ['Autocorrelation', view.acf, view.acfLimits],
    ['Partial autocorrelation', view.pacf, view.pacfLimits],
  ] as const
  const grids = [{ left: 62, right: 18, top: '5%', height: '36%' }, { left: 62, right: 18, top: '56%', height: '36%' }]
  const xAxes = panels.map((_, index) => ({
    type: 'category' as const,
    gridIndex: index,
    data: lags.map(String),
    name: 'lag',
    nameLocation: 'middle' as const,
    nameGap: 25,
    nameTextStyle: axisNameStyle(theme),
    axisLine: { lineStyle: { color: theme.hair } },
    axisTick: { show: false },
    axisLabel: axisLabelStyle(theme),
  }))
  const yAxes = panels.map(([name], index) => ({ ...valueAxis(theme, name), gridIndex: index, min: -1, max: 1, nameGap: 46 }))
  // The band is an invisible negative floor with the full width stacked on it. ECharts refuses to
  // stack a positive value onto a negative one under its default `samesign` strategy, which would
  // draw the band from zero to twice the limit instead of around zero.
  const series = panels.flatMap(([name, values, limits], panel) => {
    const lower = limits.map((limit) => -limit)
    const width = limits.map((limit) => 2 * limit)
    return [
      { type: 'line', name: `${name} lower`, xAxisIndex: panel, yAxisIndex: panel, data: lower, stack: `band-${panel}`, stackStrategy: 'all', symbol: 'none', silent: true, lineStyle: { opacity: 0 }, areaStyle: { opacity: 0 } },
      { type: 'line', name: `${name} 95% band`, xAxisIndex: panel, yAxisIndex: panel, data: width, stack: `band-${panel}`, stackStrategy: 'all', symbol: 'none', silent: true, lineStyle: { opacity: 0 }, areaStyle: { color: theme.info, opacity: 0.14 } },
      { type: 'bar', name, xAxisIndex: panel, yAxisIndex: panel, data: [...values], barMaxWidth: 3, itemStyle: { color: theme.ink } },
    ]
  })
  return {
    ...baseOption(theme, `${view.name} autocorrelation and partial autocorrelation from lag 0 through lag ${lags.length - 1}. Shaded regions are approximate 95 percent reference bands. Values outside a band indicate temporal dependence at that lag under the plot's assumptions; they do not establish a causal relation.`),
    grid: grids,
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const rows = Array.isArray(raw) ? raw : []
        const value = rows.find((row) => row !== null && typeof row === 'object' && !String(Reflect.get(row, 'seriesName')).includes('band') && !String(Reflect.get(row, 'seriesName')).includes('lower'))
        if (value === undefined || value === null || typeof value !== 'object') return ''
        return `lag ${Number(Reflect.get(value, 'dataIndex'))}<br/>${String(Reflect.get(value, 'seriesName'))}: <strong>${formatStatistic('score', Number(Reflect.get(value, 'value'))).text}</strong>`
      },
    },
    ...rangeSelection(theme, [0, 1]),
    xAxis: xAxes,
    yAxis: yAxes,
    series,
  }
}
