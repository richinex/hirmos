import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { baseOption, categoryAxis, gridAuto, legend, tooltip, valueAxis } from '../grammar'
import { seriesColour, type ChartTheme } from '../theme'

export interface InterventionBarsView {
  readonly set: string
  readonly read: string
  /** Outcome states in order, with P(state | do(low)) and P(state | do(high)). */
  readonly states: readonly string[]
  readonly low: readonly number[]
  readonly high: readonly number[]
}

/** The interventional distribution of the read variable under the two settings, side by side: the visible difference between do and see. */
export function interventionBarsOption(view: InterventionBarsView, theme: ChartTheme): EChartsCoreOption {
  const description = `${view.read} under ${view.set} set low and set high: ${view.states.length} bins; the largest probabilities are ${formatStatistic('score', Math.max(...view.low)).text} and ${formatStatistic('score', Math.max(...view.high)).text}.`
  const lowName = `${view.set} set low`
  const highName = `${view.set} set high`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ top: 30, bottom: 8 }),
    // Above the plot: the axis name owns the bottom edge.
    legend: { ...legend(theme, [lowName, highName]), bottom: 'auto', top: 0 },
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const index = Number(Reflect.get(first, 'dataIndex'))
        return `${view.read} bin ${view.states[index] ?? ''}<br/>${lowName}: ${formatStatistic('score', view.low[index] ?? 0).text}<br/>${highName}: ${formatStatistic('score', view.high[index] ?? 0).text}`
      },
    },
    xAxis: categoryAxis(theme, view.states.map((state) => `bin ${state}`), `${view.read} bin`),
    yAxis: { ...valueAxis(theme, 'probability'), min: 0, max: 1 },
    series: [
      { type: 'bar', name: lowName, data: [...view.low], barMaxWidth: 28, itemStyle: { color: seriesColour(theme, 0) } },
      { type: 'bar', name: highName, data: [...view.high], barMaxWidth: 28, itemStyle: { color: seriesColour(theme, 1) } },
    ],
  }
}
