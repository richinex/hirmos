import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, legend, stepAxis, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface RegretCurve {
  readonly name: string
  readonly mean: readonly number[]
  readonly low: readonly number[]
  readonly high: readonly number[]
}

export interface RegretView {
  readonly steps: readonly number[]
  readonly seeds: number
  readonly curves: readonly [RegretCurve, RegretCurve]
}

/** Cumulative regret of two learners against the best randomised estimate, with the range across seeds as a band. */
export function regretOption(view: RegretView, theme: ChartTheme): EChartsCoreOption {
  const colours = [theme.muted, theme.signal] as const
  const last = view.steps.length - 1
  const band = (curve: RegretCurve, i: number) => [
    {
      type: 'line',
      name: `${curve.name} low`,
      data: curve.low.map((value, k) => [view.steps[k], value]),
      lineStyle: { opacity: 0 },
      symbol: 'none',
      stack: `regret-band-${i}`,
      stackStrategy: 'all',
      silent: true,
    },
    {
      type: 'line',
      name: `${curve.name} range`,
      data: curve.high.map((value, k) => [view.steps[k], value - (curve.low[k] ?? value)]),
      lineStyle: { opacity: 0 },
      areaStyle: { color: colours[i], opacity: 0.14 },
      symbol: 'none',
      stack: `regret-band-${i}`,
      stackStrategy: 'all',
      silent: true,
    },
    {
      type: 'line',
      name: curve.name,
      data: curve.mean.map((value, k) => [view.steps[k], value]),
      lineStyle: { color: colours[i], width: 1.5 },
      itemStyle: { color: colours[i] },
      symbol: 'none',
    },
  ]
  return {
    ...baseOption(
      theme,
      `Cumulative regret over ${formatCount(view.steps[last] ?? 0).text} steps, averaged over ${formatCount(view.seeds).text} seeds: ${view.curves
        .map(
          (curve) =>
            `${curve.name} ${formatStatistic('mean', curve.mean[last] ?? Number.NaN).text}`,
        )
        .join(', ')}.`,
    ),
    grid: gridAuto({ top: 30, bottom: 28 }),
    legend: {
      ...legend(
        theme,
        view.curves.map((curve) => curve.name),
      ),
      bottom: 'auto',
      top: 0,
    },
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const k = Number(Reflect.get(first, 'dataIndex'))
        return [
          `Step ${formatCount(view.steps[k] ?? 0).text}`,
          ...view.curves.map(
            (curve) =>
              `${curve.name}: <strong>${formatStatistic('mean', curve.mean[k] ?? Number.NaN).text}</strong>`,
          ),
        ].join('<br/>')
      },
    },
    xAxis: { ...stepAxis(theme, 'Step'), min: 0, max: view.steps[last] ?? 1 },
    yAxis: valueAxis(theme, 'Cumulative regret'),
    series: view.curves.flatMap(band),
  }
}
