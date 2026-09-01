import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, legend, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export function countSeriesFitOption(view: {
  readonly name: string
  readonly observed: readonly number[]
  readonly fitted: readonly number[]
  readonly strongestReferencePoint: number
}, theme: ChartTheme): EChartsCoreOption {
  return {
    ...baseOption(theme, `${view.name}: observed counts and fitted INGARCH conditional means across ${view.observed.length} reference points.`),
    grid: gridAuto({ bottom: 44 }),
    legend: legend(theme, ['observed', 'fitted mean']),
    tooltip: { ...tooltip(theme, 'axis'), axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } } },
    xAxis: { type: 'value', min: 1, max: Math.max(2, view.observed.length), minInterval: 1, name: 'reference point', nameLocation: 'middle', nameGap: 22, nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize }, axisLine: { lineStyle: { color: theme.hair } }, axisTick: { show: false }, axisLabel: axisLabelStyle(theme) },
    yAxis: { ...valueAxis(theme, 'count'), scale: true },
    series: [
      { type: 'line', name: 'observed', data: view.observed.map((value, index) => [index + 1, value]), symbol: 'none', lineStyle: { color: theme.ink, width: 1.1 }, markLine: { silent: true, symbol: 'none', label: { color: theme.muted, formatter: 'strongest scan date' }, lineStyle: { color: theme.signal, type: 'dashed', width: 1 }, data: [{ xAxis: view.strongestReferencePoint + 1 }] } },
      { type: 'line', name: 'fitted mean', data: view.fitted.map((value, index) => [index + 1, value]), symbol: 'none', lineStyle: { color: theme.info, width: 1.4, type: 'dashed' } },
    ],
  }
}

export function interventionScoreOption(view: {
  readonly candidates: readonly { readonly referencePoint: number; readonly scoreStatistic: number }[]
}, theme: ChartTheme): EChartsCoreOption {
  const strongest = view.candidates.reduce((best, candidate) => candidate.scoreStatistic > best.scoreStatistic ? candidate : best)
  return {
    ...baseOption(theme, `Unknown-date intervention score scan across ${view.candidates.length} candidate reference points; the maximum is at ${strongest.referencePoint + 1}.`),
    grid: gridAuto({ bottom: 28 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const entry = Array.isArray(raw) ? raw[0] : raw
        if (entry === null || typeof entry !== 'object') return ''
        const data = Reflect.get(entry, 'data')
        if (!Array.isArray(data)) return ''
        return `reference point ${Number(data[0])}<br/>score statistic <strong>${formatStatistic('raw', Number(data[1])).text}</strong>`
      },
    },
    xAxis: { type: 'value', minInterval: 1, name: 'candidate reference point', nameLocation: 'middle', nameGap: 22, nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize }, axisLine: { lineStyle: { color: theme.hair } }, axisTick: { show: false }, axisLabel: axisLabelStyle(theme) },
    yAxis: { ...valueAxis(theme, 'score statistic'), scale: true },
    series: [{ type: 'line', name: 'score statistic', data: view.candidates.map((candidate) => [candidate.referencePoint + 1, candidate.scoreStatistic]), symbol: 'circle', symbolSize: 3, lineStyle: { color: theme.ink, width: 1.2 }, itemStyle: { color: theme.ink }, markPoint: { symbolSize: 28, label: { color: theme.panel, fontFamily: theme.mono, fontSize: theme.labelSize, formatter: String(strongest.referencePoint + 1) }, itemStyle: { color: theme.signal }, data: [{ coord: [strongest.referencePoint + 1, strongest.scoreStatistic] }] } }],
  }
}
