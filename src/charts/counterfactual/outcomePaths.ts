import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import {
  axisLabelStyle,
  baseOption,
  gridAuto,
  legend,
  tooltip,
  valueAxis,
  rangeSelection,
} from '../grammar'
import { seriesColour, type ChartTheme } from '../theme'

export interface OutcomePathsView {
  readonly outcome: string
  readonly treatment: string
  readonly interventions: readonly [number, number]
  readonly factual: readonly number[]
  readonly low: readonly number[]
  readonly high: readonly number[]
  readonly stepLabel: string
  /** One-based label of the first plotted point; defaults to one for row-wise counterfactuals. */
  readonly firstStep?: number
}

/** Every row's observed outcome beside what the model says it would have been under each intervention. */
export function outcomePathsOption(view: OutcomePathsView, theme: ChartTheme): EChartsCoreOption {
  const description = `${view.outcome} observed and under ${view.treatment} set to ${view.interventions[0]} and ${view.interventions[1]}, over ${formatCount(view.factual.length).text} ${view.stepLabel}s.`
  const firstStep = view.firstStep ?? 1
  const steps = view.factual.map((_, index) => firstStep + index)
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ top: 30, bottom: 40 }),
    legend: { ...legend(theme), bottom: 'auto', top: 0 },
    ...rangeSelection(theme),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const index = Number(Reflect.get(first, 'dataIndex'))
        const factual = view.factual[index]
        const low = view.low[index]
        const high = view.high[index]
        if (factual === undefined || low === undefined || high === undefined) return ''
        return `${view.stepLabel} ${firstStep + index}<br/>observed <strong>${formatStatistic('raw', factual).text}</strong><br/>${view.treatment} = ${view.interventions[0]}: ${formatStatistic('raw', low).text}<br/>${view.treatment} = ${view.interventions[1]}: ${formatStatistic('raw', high).text}<br/>difference <strong>${formatStatistic('raw', high - low).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      min: firstStep,
      max: Math.max(firstStep + steps.length - 1, firstStep + 1),
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
        name: `${view.treatment} = ${view.interventions[0]}`,
        data: view.low.map((value, index) => [firstStep + index, value]),
        lineStyle: { color: seriesColour(theme, 0), width: 1.2, type: 'dashed' },
        itemStyle: { color: seriesColour(theme, 0) },
        symbol: 'none',
      },
      {
        type: 'line',
        name: `${view.treatment} = ${view.interventions[1]}`,
        data: view.high.map((value, index) => [firstStep + index, value]),
        lineStyle: { color: seriesColour(theme, 1), width: 1.2, type: 'dashed' },
        itemStyle: { color: seriesColour(theme, 1) },
        symbol: 'none',
      },
      {
        type: 'line',
        name: 'observed',
        data: view.factual.map((value, index) => [firstStep + index, value]),
        lineStyle: { color: theme.ink, width: 1.4 },
        symbol: 'circle',
        symbolSize: 3,
        itemStyle: { color: theme.ink },
      },
    ],
  }
}
