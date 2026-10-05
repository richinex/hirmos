import type { EChartsCoreOption } from 'echarts/core'
import { causalForestPredictionLabel, type CausalForestEvidence } from '@/domain/causalForest'
import { formatStatistic } from '@/lib/format/number'
import {
  baseOption,
  escapeHtml,
  gridAuto,
  legend,
  rangeSelection,
  stepAxis,
  tooltip,
  valueAxis,
} from '../grammar'
import type { ChartTheme } from '../theme'

export function causalForestTocOption(
  evidence: CausalForestEvidence,
  theme: ChartTheme,
): EChartsCoreOption {
  const analysis = evidence.analysis
  if (analysis?.ranking.kind !== 'estimated' || analysis.specification.ranking.kind !== 'external')
    return {}
  const quantiles = analysis.specification.ranking.quantiles
  const rules = analysis.ranking.result.rules
  const priorities = analysis.specification.ranking.columns
  const names = rules.map(
    (_, index) =>
      analysis.specification.labels?.find((column) => column.column === priorities[index])?.name ??
      `Priority rule ${index + 1}`,
  )
  return {
    ...baseOption(
      theme,
      'Targeting operator characteristic: effect among the highest-priority fraction minus the average effect.',
    ),
    grid: gridAuto({ top: 32, bottom: 48 }),
    legend: legend(theme, names),
    tooltip: tooltip(theme),
    xAxis: { ...valueAxis(theme, 'Highest-priority fraction'), min: 0, max: 1 },
    yAxis: valueAxis(theme, 'Effect relative to the overall average'),
    series: rules.map((rule, index) => ({
      id: `priority-rule-${index}`,
      name: names[index],
      type: 'line',
      clip: true,
      data: rule.toc.map((estimate, at) => [quantiles[at], estimate]),
      itemStyle: { color: index === 0 ? theme.signal : theme.info },
      markLine: {
        silent: true,
        symbol: 'none',
        label: { show: false },
        data: [{ yAxis: 0 }],
        lineStyle: { color: theme.muted, type: 'dashed' },
      },
    })),
  }
}

/** Prepared-row identity is stable. Do not connect rows into a time path or sort
 * them into an apparent dose-response curve. Missing predictions stay missing. */
export function causalForestPredictionsOption(
  evidence: CausalForestEvidence,
  outcome: string,
  theme: ChartTheme,
): EChartsCoreOption {
  const label = causalForestPredictionLabel(evidence.target)
  const confidence = `${formatStatistic('raw', evidence.confidenceLevel * 100).text}% pointwise interval`
  const points: (readonly [number, number])[] = []
  const intervals: (readonly [number, number] | null)[] = []
  for (const [index, prediction] of evidence.predictions.entries()) {
    if (prediction.kind === 'unavailable') continue
    points.push([index + 1, prediction.estimate])
    if (prediction.uncertainty.kind === 'estimated') {
      const { lower, upper } = prediction.uncertainty.interval
      intervals.push([index + 1, lower], [index + 1, upper], null)
    }
  }
  const unavailable = evidence.observations - points.length
  return {
    ...baseOption(
      theme,
      `${label} for ${outcome} by prepared row. ${points.length} estimates; ${unavailable} unavailable predictions. Intervals are pointwise, not simultaneous. Row order does not represent time.`,
    ),
    grid: gridAuto({ top: 32, bottom: 64 }),
    legend: { ...legend(theme, [label, confidence]), top: 0, bottom: undefined },
    ...rangeSelection(theme),
    xAxis: { ...stepAxis(theme, 'Prepared row'), min: 0.5, max: evidence.observations + 0.5 },
    yAxis: valueAxis(theme, `${label} on ${outcome}`),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const point: unknown = Reflect.get(raw, 'value')
        if (!Array.isArray(point) || typeof point[0] !== 'number') return ''
        const row = point[0]
        const prediction = evidence.predictions[row - 1]
        if (prediction === undefined || prediction.kind === 'unavailable') return ''
        const uncertainty =
          prediction.uncertainty.kind === 'estimated'
            ? `${confidence}: ${formatStatistic('raw', prediction.uncertainty.interval.lower).text} to ${formatStatistic('raw', prediction.uncertainty.interval.upper).text}`
            : `Interval unavailable: ${escapeHtml(prediction.uncertainty.reason)}`
        return `Prepared row ${row}<br/>${escapeHtml(label)}: ${formatStatistic('raw', prediction.estimate).text}<br/>${uncertainty}`
      },
    },
    series: [
      {
        id: 'conditional-intervals',
        name: confidence,
        type: 'line',
        data: intervals,
        symbol: 'none',
        connectNulls: false,
        clip: true,
        silent: true,
        lineStyle: { color: theme.info, width: 1, opacity: 0.4 },
      },
      {
        id: 'conditional-estimates',
        name: label,
        type: 'scatter',
        data: points,
        symbolSize: 5,
        clip: true,
        itemStyle: { color: theme.signal },
        markLine: {
          silent: true,
          symbol: 'none',
          label: { show: false },
          data: [{ yAxis: 0 }],
          lineStyle: { color: theme.muted, type: 'dashed', width: 1 },
        },
      },
    ],
  }
}
