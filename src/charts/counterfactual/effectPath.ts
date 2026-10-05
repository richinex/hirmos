import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import {
  axisLabelStyle,
  baseOption,
  gridAuto,
  tooltip,
  valueAxis,
  rangeSelection,
} from '../grammar'
import type { ChartTheme } from '../theme'

export interface CounterfactualEffectPathView {
  readonly outcome: string
  readonly effects: readonly number[]
  readonly lower: readonly number[]
  readonly upper: readonly number[]
  readonly confidenceLevel: number
  readonly stepLabel: string
  readonly firstStep: number
}

/** High-minus-low dynamic contrast with an equal-tail pointwise block-bootstrap band. */
export function counterfactualEffectPathOption(
  view: CounterfactualEffectPathView,
  theme: ChartTheme,
): EChartsCoreOption {
  const lastStep = view.firstStep + view.effects.length - 1
  const level = Math.round(view.confidenceLevel * 100)
  return {
    ...baseOption(
      theme,
      `${view.outcome} high-minus-low counterfactual contrast and its ${level}% pointwise block-bootstrap interval over ${formatCount(view.effects.length).text} ${view.stepLabel}s.`,
    ),
    // No slider of its own: the outcome chart above carries it and this chart follows its window.
    grid: gridAuto({ bottom: 28 }),
    ...rangeSelection(theme, 0, { slider: false }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const index = Number(Reflect.get(first, 'dataIndex'))
        const effect = view.effects[index]
        const lower = view.lower[index]
        const upper = view.upper[index]
        if (effect === undefined || lower === undefined || upper === undefined) return ''
        return `${view.stepLabel} ${view.firstStep + index}<br/>contrast <strong>${formatStatistic('raw', effect).text}</strong><br/>${level}% pointwise CI [${formatStatistic('raw', lower).text}, ${formatStatistic('raw', upper).text}]`
      },
    },
    xAxis: {
      type: 'value',
      min: view.firstStep,
      max: Math.max(lastStep, view.firstStep + 1),
      minInterval: 1,
      name: view.stepLabel,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: axisLabelStyle(theme),
      splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme), scale: true },
    series: [
      {
        type: 'line',
        name: 'lower',
        data: view.lower.map((value, index) => [view.firstStep + index, value]),
        lineStyle: { opacity: 0 },
        symbol: 'none',
        stack: 'counterfactual-effect-band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: `${level}% pointwise CI`,
        data: view.upper.map((value, index) => [
          view.firstStep + index,
          value - (view.lower[index] ?? value),
        ]),
        lineStyle: { opacity: 0 },
        areaStyle: { color: theme.signal, opacity: 0.16 },
        symbol: 'none',
        stack: 'counterfactual-effect-band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'contrast',
        data: view.effects.map((value, index) => [view.firstStep + index, value]),
        lineStyle: { color: theme.signal, width: 1.5 },
        symbol: 'circle',
        symbolSize: 3,
        itemStyle: { color: theme.signal },
      },
    ],
  }
}
