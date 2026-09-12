import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisNameStyle, baseOption, gridAuto, tooltip } from '../grammar'
import { variableColour, type ChartTheme } from '../theme'

export interface PairwiseScatterView {
  readonly xName: string
  readonly yName: string
  readonly x: readonly number[]
  readonly y: readonly number[]
  readonly correlation: number
}

const MAX_POINTS = 2_000

/** Evenly spaced source indices retain the full row range without a random or order-dependent sample. */
const displayedPoints = (view: PairwiseScatterView): readonly (readonly [number, number])[] => {
  const length = Math.min(view.x.length, view.y.length)
  const count = Math.min(length, MAX_POINTS)
  if (count === length) return Array.from({ length }, (_, index) => [view.x[index] ?? 0, view.y[index] ?? 0] as const)
  return Array.from({ length: count }, (_, index) => {
    const source = Math.round(index * (length - 1) / (count - 1))
    return [view.x[source] ?? 0, view.y[source] ?? 0] as const
  })
}

export function pairwiseScatterOption(view: PairwiseScatterView, theme: ChartTheme): EChartsCoreOption {
  const points = displayedPoints(view)
  return {
    ...baseOption(theme, `${view.yName} against ${view.xName}; Pearson correlation ${formatStatistic('score', view.correlation).text}; ${points.length} of ${Math.min(view.x.length, view.y.length)} observations shown.`),
    grid: gridAuto(),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const value = raw !== null && typeof raw === 'object' ? Reflect.get(raw, 'value') : null
        return Array.isArray(value)
          ? `${view.xName} <strong>${formatStatistic('raw', Number(value[0])).text}</strong><br/>${view.yName} <strong>${formatStatistic('raw', Number(value[1])).text}</strong>`
          : ''
      },
    },
    xAxis: {
      type: 'value',
      name: view.xName,
      nameLocation: 'middle',
      nameGap: 28,
      nameTextStyle: axisNameStyle(theme),
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true },
      splitLine: { lineStyle: { color: theme.hair } },
    },
    yAxis: {
      type: 'value',
      name: view.yName,
      nameLocation: 'middle',
      nameGap: 44,
      nameTextStyle: axisNameStyle(theme),
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true },
      splitLine: { lineStyle: { color: theme.hair } },
    },
    series: [{
      type: 'scatter',
      data: points,
      symbolSize: 5,
      itemStyle: { color: variableColour(theme, view.yName), opacity: 0.62 },
      large: points.length >= 1_000,
      largeThreshold: 1_000,
    }],
  }
}
