import type { EChartsCoreOption } from 'echarts/core'
import { baseOption, escapeHtml, gridAuto, tooltip, valueAxis } from './grammar'
import { seriesColour, type ChartTheme } from './theme'
import type { RootCauseRun, FullRootCauseChecks } from '@/domain/rootCauseAnalysis'
import { formatStatistic } from '@/lib/format/number'

export function rootCauseBars(run: RootCauseRun) {
  const outcome = run.evidence.outcome
  const { summary } = outcome
  if (outcome.kind === 'intervention') return [
    { name: 'Observed mean', estimate: outcome.observedMean, interval: null },
    { name: 'Mean after shifts', estimate: summary.estimates[outcome.nodes.indexOf(outcome.target)]!, interval: summary.bounds[outcome.nodes.indexOf(outcome.target)]! },
  ]
  return outcome.nodes.map((node, index) => ({ name: run.model.names[node]!, estimate: summary.estimates[index]!, interval: summary.bounds[index]! }))
}

/** The notebook compares violation fractions across relabelled graphs, on shared bins. */
export function graphChecksOption(evidence: FullRootCauseChecks, theme: ChartTheme): EChartsCoreOption {
  const methods = [
    { name: 'Conditional independence', field: 'lmcViolations' as const },
    { name: 'Graph structure', field: 'tpaViolations' as const },
  ]
  const fraction = (entry: FullRootCauseChecks['given'], field: 'lmcViolations' | 'tpaViolations') => entry[field] / Math.max(1, entry.implications.length)
  const values = methods.map(({ field }) => evidence.permutations.map((entry) => fraction(entry, field)))
  const low = Math.min(...values.flat())
  const high = Math.max(...values.flat())
  const start = low === high ? low - 0.5 : low
  const end = low === high ? high + 0.5 : high
  const width = (end - start) / 10
  return {
    ...baseOption(theme, 'Violation fractions for relabelled graphs. Dashed lines mark the proposed graph.'),
    grid: gridAuto({ top: 50, bottom: 44 }), tooltip: tooltip(theme, 'axis'),
    legend: { top: 0, textStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize } },
    xAxis: { ...valueAxis(theme, 'Fraction of violations'), min: Math.min(0, start), max: Math.max(end, ...methods.map(({ field }) => fraction(evidence.given, field))), nameLocation: 'middle', nameGap: 28 },
    yAxis: { ...valueAxis(theme, 'Permutations'), min: 0, minInterval: 1 },
    series: methods.map(({ name, field }, index) => {
      const counts = Array<number>(10).fill(0)
      for (const value of values[index]!) counts[Math.min(9, Math.max(0, Math.floor((value - start) / width)))]! += 1
      const colour = seriesColour(theme, index)
      return { type: 'bar', name, data: counts.map((count, bin) => [start + (bin + 0.5) * width, count]), itemStyle: { color: colour, opacity: 0.65 },
        markLine: { silent: true, symbol: 'none', label: { show: false }, lineStyle: { color: colour, type: 'dashed' }, data: [{ xAxis: fraction(evidence.given, field) }] } }
    }),
  }
}

/** DoWhy v0.14 bar_plot omits both error arms when the centre is outside its bounds. */
export function rootCauseOption(run: RootCauseRun, theme: ChartTheme): EChartsCoreOption {
  const rows = rootCauseBars(run)
  const kind = run.evidence.outcome.kind
  const measure = kind === 'anomaly' ? 'Anomaly attribution score' : kind === 'change' ? 'Contribution to the mean change' : `Mean ${run.model.names[run.model.target]}`
  return contributionOption(rows, measure, theme)
}

export interface ContributionBar {
  readonly name: string
  readonly estimate: number
  readonly interval: readonly [number, number] | null
}

export function contributionOption(rows: readonly ContributionBar[], measure: string, theme: ChartTheme, valueLabel = 'Estimate'): EChartsCoreOption {
  const number = (value: number) => formatStatistic('raw', value).text
  return {
    ...baseOption(theme, `${measure}. Bars show estimates.${rows.some(row => row.interval !== null) ? ' Lines show refit percentile bounds.' : ''}`),
    grid: gridAuto({ bottom: 48 }),
    tooltip: { ...tooltip(theme), formatter: (raw: unknown) => {
      const index = raw !== null && typeof raw === 'object' ? Number(Reflect.get(raw, 'dataIndex')) : -1
      const row = rows[index]
      if (row === undefined) return ''
      return `${escapeHtml(row.name)}<br/>${escapeHtml(valueLabel)}: ${number(row.estimate)}${row.interval === null ? '' : `<br/>Percentile bounds: ${number(row.interval[0])} to ${number(row.interval[1])}`}`
    } },
    xAxis: { ...valueAxis(theme, measure), nameLocation: 'middle', nameGap: 28 },
    yAxis: { type: 'category', inverse: true, data: rows.map((row) => row.name), axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, width: 150, overflow: 'truncate' }, axisLine: { show: false }, axisTick: { show: false } },
    series: [
      { type: 'bar', data: rows.map((row, index) => ({ value: row.estimate, itemStyle: { color: seriesColour(theme, index) } })), barMaxWidth: 18,
        markLine: { silent: true, symbol: 'none', label: { show: false }, data: [{ xAxis: 0 }], lineStyle: { color: theme.muted, width: 1 } } },
      ...rows.flatMap((row, index) => {
        const bounds = row.interval
        if (bounds === null || row.estimate < bounds[0] || row.estimate > bounds[1]) return []
        return [{ type: 'line' as const, data: [[bounds[0], index], [bounds[1], index]], symbol: 'rect', symbolSize: [2, 10], lineStyle: { color: theme.ink, width: 1.5 }, itemStyle: { color: theme.ink }, silent: true, tooltip: { show: false }, z: 3 }]
      }),
    ],
  }
}
