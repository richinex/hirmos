import type { EChartsCoreOption } from 'echarts/core'
import { axisLabelStyle, axisNameStyle, baseOption, escapeHtml, gridAuto, tooltip } from '@/charts/grammar'
import type { ChartTheme } from '@/charts/theme'
import { formatP, formatStatistic } from '@/lib/format/number'

export interface RatioEstimate {
  readonly label: string
  readonly ratio: number
  readonly interval: readonly [number, number]
  readonly pValue: number
}

/** The height a forest plot needs to keep every row legible, for the host to reserve. */
export const ratioForestHeight = (rows: number): number => Math.max(160, 26 * rows + 64)

const ROUND_RATIOS = [0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1, 1.1, 1.2, 1.25, 1.5, 2, 2.5, 3, 4, 5, 6, 8, 10, 15, 20]

/** Round ratios inside the axis range, thinned to at most seven with 1 always kept, so the labels read as ratios rather than as powers. */
const ratioTicks = (low: number, high: number): number[] => {
  let ticks = ROUND_RATIOS.filter((value) => Math.log(value) >= low && Math.log(value) <= high)
  while (ticks.length > 7) {
    const parity = Math.max(0, ticks.indexOf(1)) % 2
    ticks = ticks.filter((_, index) => index % 2 === parity)
  }
  return ticks.map((value) => Math.log(value))
}

/**
 * Every covariate's ratio with its interval on one axis, largest at the top: the forest plot of a
 * fitted model. The axis is the log of the ratio, labelled in ratios, so a halving and a doubling sit
 * the same distance from the rule at 1, where the covariate leaves the event rate or the time unchanged.
 */
export function ratioForestOption(
  rows: readonly RatioEstimate[],
  ratioName: 'hazard ratio' | 'time ratio',
  confidence: number,
  theme: ChartTheme,
): EChartsCoreOption {
  // ECharts draws category index 0 at the bottom, so ascending order puts the largest ratio on top.
  const ordered = [...rows].sort((a, b) => a.ratio - b.ratio)
  const logs = ordered.flatMap((row) => [Math.log(row.interval[0]), Math.log(row.interval[1])])
  const low = Math.min(0, ...logs)
  const high = Math.max(0, ...logs)
  const pad = Math.max(0.02, (high - low) * 0.06)
  const ticks = ratioTicks(low - pad, high + pad)
  const excluding = ordered.filter((row) => row.interval[0] > 1 || row.interval[1] < 1).length
  const ratio = (value: number): string => formatStatistic('raw', value).text
  return {
    ...baseOption(theme, `${ratioName} of ${ordered.length} covariates with ${confidence}% intervals on a log scale; ${excluding} intervals exclude 1.`),
    grid: gridAuto({ top: 12, bottom: 34 }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const index = raw !== null && typeof raw === 'object' ? Number(Reflect.get(raw, 'dataIndex')) : Number.NaN
        const row = ordered[index]
        if (row === undefined) return ''
        return `${escapeHtml(row.label)}<br/>${ratioName} <strong>${ratio(row.ratio)}</strong><br/>${confidence}% interval ${ratio(row.interval[0])} to ${ratio(row.interval[1])}<br/>${formatP(row.pValue).text}`
      },
    },
    xAxis: {
      type: 'value',
      min: low - pad,
      max: high + pad,
      name: `${ratioName}, log scale`,
      nameLocation: 'middle',
      nameGap: 22,
      nameTextStyle: axisNameStyle(theme),
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: true, lineStyle: { color: theme.hair }, customValues: ticks },
      axisLabel: { ...axisLabelStyle(theme), customValues: ticks, formatter: (value: number) => String(Number(Math.exp(value).toPrecision(3))) },
      // The rules would follow the axis's own even steps in log space, not the round ratios the labels name.
      splitLine: { show: false },
    },
    yAxis: {
      type: 'category',
      data: ordered.map((row) => row.label),
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, width: 200, overflow: 'truncate' },
    },
    series: [
      ...ordered.map((row, index) => ({
        type: 'line' as const,
        name: `${row.label} interval`,
        data: [[Math.log(row.interval[0]), index], [Math.log(row.interval[1]), index]],
        lineStyle: { color: theme.muted, width: 1.6 },
        symbol: 'none',
        silent: true,
        tooltip: { show: false },
        z: 2,
      })),
      {
        type: 'scatter',
        name: ratioName,
        data: ordered.map((row, index) => [Math.log(row.ratio), index]),
        symbol: 'rect',
        symbolSize: 7,
        itemStyle: { color: theme.ink },
        z: 3,
        markLine: {
          silent: true,
          symbol: 'none',
          label: { show: false },
          data: [{ xAxis: 0, lineStyle: { color: theme.muted, type: 'dashed', width: 1 } }],
        },
      },
    ],
  }
}
