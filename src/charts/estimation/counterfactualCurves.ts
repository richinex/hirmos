import type { EChartsCoreOption } from 'echarts/core'
import type { BayesianGaussianCurve } from '@/domain/estimation'
import { formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, legend, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface CounterfactualCurvesView {
  readonly outcome: string
  readonly treatment: string
  readonly covariate: string
  readonly curve: BayesianGaussianCurve
}

/** Posterior expected outcome across one covariate under do(0) and do(1): a median line inside its 94% band for each arm. */
export function counterfactualCurvesOption(view: CounterfactualCurvesView, theme: ChartTheme): EChartsCoreOption {
  const { curve } = view
  const names = [`do(${view.treatment} = 0)`, `do(${view.treatment} = 1)`]
  const arm = (name: string, colour: string, lower: readonly number[], median: readonly number[], upper: readonly number[]) => [
    {
      type: 'line',
      name: `${name} lower`,
      data: curve.grid.map((x, index) => [x, lower[index]]),
      lineStyle: { opacity: 0 },
      symbol: 'none',
      stack: name,
      stackStrategy: 'all',
      silent: true,
    },
    {
      type: 'line',
      name: `${name} band`,
      data: curve.grid.map((x, index) => [x, (upper[index] ?? 0) - (lower[index] ?? 0)]),
      lineStyle: { opacity: 0 },
      symbol: 'none',
      stack: name,
      stackStrategy: 'all',
      areaStyle: { color: colour, opacity: 0.16 },
      silent: true,
    },
    {
      type: 'line',
      name,
      data: curve.grid.map((x, index) => [x, median[index]]),
      symbol: 'none',
      lineStyle: { color: colour, width: 2 },
      itemStyle: { color: colour },
    },
  ]
  const description = `Expected ${view.outcome} across ${view.covariate} under both interventions on ${view.treatment}, with 94% posterior bands.`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 70, top: 20 }),
    legend: legend(theme, names),
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const entries = (Array.isArray(raw) ? raw : [raw]).filter((entry): entry is { seriesName?: string; value?: unknown } =>
          entry !== null && typeof entry === 'object' && names.includes(String(Reflect.get(entry, 'seriesName'))))
        if (entries.length === 0) return ''
        const first = Array.isArray(entries[0]?.value) ? Number(entries[0]?.value[0]) : Number.NaN
        const lines = entries.map((entry) => `${String(entry.seriesName)} <strong>${formatStatistic('raw', Array.isArray(entry.value) ? Number(entry.value[1]) : Number.NaN).text}</strong>`)
        return `${view.covariate} ${formatStatistic('raw', first).text}<br/>${lines.join('<br/>')}`
      },
    },
    xAxis: {
      type: 'value',
      name: curve.standardised ? `${view.covariate} · standardised` : view.covariate,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize },
      scale: true,
    },
    yAxis: { ...valueAxis(theme, `expected ${view.outcome}`), scale: true },
    series: [
      ...arm(names[0] ?? 'do(0)', theme.categorical[0] ?? theme.info, curve.controlLower, curve.controlMedian, curve.controlUpper),
      ...arm(names[1] ?? 'do(1)', theme.categorical[1] ?? theme.signal, curve.treatedLower, curve.treatedMedian, curve.treatedUpper),
    ],
  }
}
