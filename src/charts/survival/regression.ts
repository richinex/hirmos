import type { EChartsCoreOption } from 'echarts/core'
import { baseOption, gridAuto, rangeSelection, tooltip, valueAxis } from '@/charts/grammar'
import { seriesColour, type ChartTheme } from '@/charts/theme'

/** plot.aareg: cumulative coefficients, with separate pointwise bounds and a signed axis. */
export function aalenCurveOption(
  name: string,
  points: readonly (readonly [number, number, number, number])[],
  theme: ChartTheme,
  term = 0,
): EChartsCoreOption {
  const colour = seriesColour(theme, term)
  return {
    ...baseOption(
      theme,
      `Cumulative coefficient for ${name}, with approximate pointwise 95% bounds.`,
    ),
    grid: gridAuto({ bottom: 64 }),
    ...rangeSelection(theme),
    tooltip: tooltip(theme, 'axis'),
    xAxis: {
      ...valueAxis(theme, 'follow-up time'),
      min: 0,
      nameGap: 25,
      splitLine: { show: false },
    },
    yAxis: valueAxis(theme, 'cumulative coefficient'),
    series: [
      {
        type: 'line',
        data: points.map((p) => [p[0], p[2]]),
        stack: 'bounds',
        stackStrategy: 'all',
        step: 'end',
        symbol: 'none',
        silent: true,
        tooltip: { show: false },
        lineStyle: { opacity: 0 },
        areaStyle: { opacity: 0 },
      },
      {
        type: 'line',
        data: points.map((p) => [p[0], p[3] - p[2]]),
        stack: 'bounds',
        stackStrategy: 'all',
        step: 'end',
        symbol: 'none',
        silent: true,
        tooltip: { show: false },
        lineStyle: { opacity: 0 },
        areaStyle: { color: colour, opacity: 0.16 },
      },
      ...([1, 2, 3] as const).map((index) => ({
        type: 'line',
        name: { 1: name, 2: 'lower bound', 3: 'upper bound' }[index],
        data: points.map((point) => [point[0], point[index]]),
        step: 'end',
        showSymbol: false,
        itemStyle: { color: colour },
        lineStyle: {
          color: colour,
          type: index === 1 ? 'solid' : 'dashed',
          width: index === 1 ? 1.6 : 1,
        },
        ...(index === 1
          ? {
              markLine: {
                silent: true,
                symbol: 'none',
                label: { show: false },
                data: [{ yAxis: 0 }],
                lineStyle: { color: theme.hair },
              },
            }
          : {}),
      })),
    ],
  }
}

export function importanceOption(
  rows: readonly { readonly name: string; readonly value: number }[],
  theme: ChartTheme,
): EChartsCoreOption {
  const ordered = rows
    .map((row, index) => ({ ...row, colour: seriesColour(theme, index) }))
    .sort((a, b) => a.value - b.value)
  return {
    ...baseOption(
      theme,
      'Permutation importance: change in out-of-bag concordance when a covariate is shuffled.',
    ),
    grid: gridAuto({ bottom: 36 }),
    tooltip: tooltip(theme),
    xAxis: valueAxis(theme, 'permutation importance'),
    yAxis: {
      type: 'category',
      data: ordered.map((r) => r.name),
      axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { show: false },
      axisTick: { show: false },
    },
    series: [
      {
        type: 'bar',
        data: ordered.map((r) => ({ value: r.value, itemStyle: { color: r.colour } })),
        barMaxWidth: 14,
      },
    ],
  }
}
