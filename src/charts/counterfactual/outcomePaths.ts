import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, legend, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface OutcomePathsView {
  readonly outcome: string
  readonly treatment: string
  readonly interventions: readonly [number, number]
  readonly factual: readonly number[]
  readonly low: readonly number[]
  readonly high: readonly number[]
  readonly stepLabel: string
}

/** Every row's observed outcome beside what the model says it would have been under each intervention. */
export function outcomePathsOption(view: OutcomePathsView, theme: ChartTheme): EChartsCoreOption {
  const description = `${view.outcome} observed and under ${view.treatment} set to ${view.interventions[0]} and ${view.interventions[1]}, over ${formatCount(view.factual.length).text} ${view.stepLabel}s.`
  const steps = view.factual.map((_, index) => index + 1)
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 46 }),
    legend: legend(theme),
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
        return `${view.stepLabel} ${index + 1}<br/>observed <strong>${formatStatistic('raw', factual).text}</strong><br/>${view.treatment} = ${view.interventions[0]}: ${formatStatistic('raw', low).text}<br/>${view.treatment} = ${view.interventions[1]}: ${formatStatistic('raw', high).text}<br/>difference <strong>${formatStatistic('raw', high - low).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      min: 1,
      max: Math.max(steps.length, 2),
      minInterval: 1,
      name: view.stepLabel,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize },
    },
    yAxis: { ...valueAxis(theme), scale: true },
    series: [
      { type: 'line', name: `${view.treatment} = ${view.interventions[0]}`, data: view.low.map((value, index) => [index + 1, value]), lineStyle: { color: theme.muted, width: 1.2, type: 'dashed' }, symbol: 'none' },
      { type: 'line', name: `${view.treatment} = ${view.interventions[1]}`, data: view.high.map((value, index) => [index + 1, value]), lineStyle: { color: theme.bone, width: 1.2, type: 'dashed' }, symbol: 'none' },
      { type: 'line', name: 'observed', data: view.factual.map((value, index) => [index + 1, value]), lineStyle: { color: theme.ink, width: 1.4 }, symbol: 'circle', symbolSize: 3, itemStyle: { color: theme.ink } },
    ],
  }
}
