import type { EChartsCoreOption } from 'echarts/core'
import type { PlotTime } from '@/domain/longRun'
import {
  axisLabelStyle,
  baseOption,
  categoryAxis,
  gridAuto,
  legend,
  rangeSelection,
  tooltip,
  valueAxis,
} from '../grammar'
import type { ChartTheme } from '../theme'

/**
 * The figures of Lopez Bernal, Cummins and Gasparrini (2017): the series with the period after
 * the event shaded, the fitted curve and the counterfactual dashed; the same with the seasonal
 * terms held at one phase, the deseasonalised trend; the residuals over time; and the ACF and
 * PACF of the residuals with their limits.
 */

const timeAxis = (theme: ChartTheme, axis: PlotTime) => ({
  type: axis.kind === 'calendar' ? ('time' as const) : ('value' as const),
  min: axis.values[0],
  max: axis.values[axis.values.length - 1],
  axisLabel: axisLabelStyle(theme),
  axisTick: { show: false },
  axisLine: { lineStyle: { color: theme.hair } },
  splitLine: { show: false },
})

export function interruptedFitOption(
  view: {
    readonly title: string
    readonly axis: PlotTime
    readonly observed: readonly number[]
    readonly fitted: readonly number[]
    readonly counterfactual: readonly number[]
    readonly fittedName: string
    readonly interventionRow: number
    readonly outcomeName: string
  },
  theme: ChartTheme,
): EChartsCoreOption {
  const times = view.axis.values
  const at = (values: readonly number[]) => values.map((value, row) => [times[row], value])
  return {
    ...baseOption(theme, view.title),
    grid: gridAuto({ top: 42, bottom: 78 }),
    ...rangeSelection(theme, 0, { slider: true }),
    legend: {
      ...legend(theme, [view.outcomeName, view.fittedName, 'Without the event']),
      bottom: 'auto',
      top: 0,
    },
    tooltip: tooltip(theme, 'axis'),
    xAxis: timeAxis(theme, view.axis),
    yAxis: { ...valueAxis(theme, ''), scale: true },
    series: [
      {
        name: view.outcomeName,
        type: 'scatter',
        symbolSize: 5,
        data: at(view.observed),
        itemStyle: { color: theme.muted },
        // The paper shades the post-intervention period grey.
        markArea: {
          silent: true,
          itemStyle: { color: theme.well, opacity: 0.6 },
          data: [[{ xAxis: times[view.interventionRow] }, { xAxis: times[times.length - 1] }]],
        },
      },
      {
        name: view.fittedName,
        type: 'line',
        symbol: 'none',
        data: at(view.fitted),
        itemStyle: { color: theme.signal },
        lineStyle: { color: theme.signal, width: 1.5 },
      },
      {
        name: 'Without the event',
        type: 'line',
        symbol: 'none',
        data: at(view.counterfactual),
        itemStyle: { color: theme.info },
        lineStyle: { color: theme.info, width: 1.5, type: 'dashed' },
      },
    ],
  }
}

export function interruptedResidualOption(
  view: {
    readonly axis: PlotTime
    readonly residuals: readonly number[]
    readonly interventionRow: number
    readonly kind: 'deviance' | 'ordinary' | 'standardised'
  },
  theme: ChartTheme,
): EChartsCoreOption {
  const times = view.axis.values
  const title = `${view.kind === 'deviance' ? 'Deviance residuals' : view.kind === 'standardised' ? 'Standardised residuals' : 'Residuals'} over time`
  return {
    ...baseOption(theme, title),
    grid: gridAuto({ top: 24, bottom: 30 }),
    ...rangeSelection(theme, 0, { slider: false }),
    tooltip: tooltip(theme, 'axis'),
    xAxis: timeAxis(theme, view.axis),
    yAxis: { ...valueAxis(theme, ''), scale: true },
    series: [
      {
        name: title,
        type: 'scatter',
        symbolSize: 5,
        data: view.residuals.map((value, row) => [times[row], value]),
        itemStyle: { color: theme.muted },
        markLine: {
          silent: true,
          symbol: 'none',
          label: { show: false },
          lineStyle: { color: theme.faint, type: 'dashed', width: 1 },
          data: [{ yAxis: 0 }],
        },
        markArea: {
          silent: true,
          itemStyle: { color: theme.well, opacity: 0.6 },
          data: [[{ xAxis: times[view.interventionRow] }, { xAxis: times[times.length - 1] }]],
        },
      },
    ],
  }
}

/** `plot_acf` or `plot_pacf`: bars per lag with the 95% band, lag zero left out. */
export function interruptedCorrelationOption(
  view: {
    readonly title: string
    readonly correlations: readonly { readonly value: number; readonly limit: number }[]
  },
  theme: ChartTheme,
): EChartsCoreOption {
  const values = view.correlations.map((c) => c.value)
  const limits = view.correlations.map((c) => c.limit)
  const lags = values.slice(1).map((_, i) => String(i + 1))
  return {
    ...baseOption(theme, view.title),
    grid: gridAuto({ top: 24, bottom: 30 }),
    tooltip: tooltip(theme, 'axis'),
    xAxis: categoryAxis(theme, lags, 'Lag'),
    yAxis: { ...valueAxis(theme, ''), min: -1, max: 1 },
    series: [
      // The band between −limit and +limit: a transparent base at −limit, then twice the limit stacked on it and filled.
      {
        name: 'Lower limit',
        type: 'line',
        symbol: 'none',
        data: limits.slice(1).map((v) => -v),
        stack: 'band',
        lineStyle: { color: theme.info, width: 1, type: 'dashed' },
        itemStyle: { color: theme.info },
        silent: true,
      },
      {
        name: 'Upper limit',
        type: 'line',
        symbol: 'none',
        data: limits.slice(1).map((v) => 2 * v),
        stack: 'band',
        lineStyle: { color: theme.info, width: 1, type: 'dashed' },
        itemStyle: { color: theme.info },
        areaStyle: { color: theme.info, opacity: 0.08 },
        silent: true,
      },
      {
        name: view.title,
        type: 'bar',
        barWidth: 6,
        data: values.slice(1),
        itemStyle: { color: theme.signal },
      },
    ],
  }
}
