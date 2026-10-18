import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, axisNameStyle, baseOption, legend, tooltip, zoomPair } from '../grammar'
import type { ChartTheme } from '../theme'

export interface PlanEstimateMark {
  readonly estimate: number
  /** The bootstrap interval, for compared plans only. */
  readonly lower: number | null
  readonly upper: number | null
}

export interface PlanColumn {
  readonly label: string
  /** The short name under the plan's column. */
  readonly axisLabel: string
  readonly value: PlanEstimateMark
  /** The plan's value minus the observed average on the same rows, with a paired interval. */
  readonly difference: PlanEstimateMark
  /** Natural bounds, for plans that always choose one action. */
  readonly bounds: { readonly lower: number; readonly upper: number } | null
  readonly randomised: number | null
}

/** Plans in view before the reader moves the slider. */
const WINDOW = 12

const rate = (value: number) => formatStatistic('mean', value).text

/** The value rounded up to its leading digit and the next, so the top tick reads as a plain number. */
const tidyCeiling = (value: number) => {
  const step = 10 ** (Math.floor(Math.log10(value)) - 1)
  return Math.ceil(value / step) * step
}
const signed = (value: number) => (value > 0 ? `+${rate(value)}` : rate(value))

const interval = (mark: PlanEstimateMark, show: (value: number) => string) =>
  mark.lower === null || mark.upper === null
    ? ', interval not evaluated'
    : ` [${show(mark.lower)}, ${show(mark.upper)}]`

/** A dashed horizontal reference across one panel, carried by that panel's estimate series. */
const reference = (theme: ChartTheme, value: number, label: string) => ({
  silent: true,
  // Marker lines round their value to two decimals by default, which would move a rate such as 0.0067 to 0.01.
  precision: 20,
  symbol: 'none',
  label: {
    show: true,
    position: 'insideEndTop',
    formatter: label,
    color: theme.faint,
    fontFamily: theme.font,
    fontSize: theme.labelSize,
  },
  data: [{ yAxis: value, lineStyle: { color: theme.muted, type: 'dashed', width: 1 } }],
})

const intervals = (
  columns: readonly PlanColumn[],
  pick: (column: PlanColumn) => PlanEstimateMark,
  axis: number,
  theme: ChartTheme,
  suffix: string,
) =>
  columns.map((column, index) => {
    const mark = pick(column)
    return {
      type: 'line' as const,
      name: `${column.label} ${suffix}`,
      xAxisIndex: axis,
      yAxisIndex: axis,
      data:
        mark.lower === null || mark.upper === null
          ? []
          : [
              [index, mark.lower],
              [index, mark.upper],
            ],
      lineStyle: { color: theme.info, width: 2 },
      symbol: 'none',
      silent: true,
      z: 3,
    }
  })

/**
 * Every plan as a column in two panels that share one slider. The upper panel shows the plan's weighted estimate
 * on its interval, its natural bounds as a pale bar cut at the panel's top, and its randomised estimate as a tick,
 * against the observed average. The lower panel shows the paired difference from the observed average, against zero.
 */
export function planComparisonOption(
  columns: readonly PlanColumn[],
  observed: number,
  outcome: string,
  theme: ChartTheme,
): EChartsCoreOption {
  const description = `${outcome} by plan: ${columns.map((column) => `${column.label} ${rate(column.value.estimate)}`).join('; ')}; observed average ${rate(observed)}.`
  const paged = columns.length > WINDOW
  const named = [
    'Weighted estimate',
    ...(columns.some((column) => column.randomised !== null) ? ['Randomised estimate'] : []),
    ...(columns.some((column) => column.bounds !== null) ? ['Natural bounds'] : []),
  ]
  const firstBounds = columns.findIndex((column) => column.bounds !== null)
  // The upper panel is scaled to the estimates; natural bounds that reach beyond it are cut at the top edge.
  const highest = Math.max(
    observed,
    ...columns.flatMap((column) => [
      column.value.upper ?? column.value.estimate,
      column.randomised ?? column.value.estimate,
    ]),
  )
  const labels = columns.map((column) => column.axisLabel)
  const category = (axis: number) => ({
    type: 'category' as const,
    gridIndex: axis,
    data: labels,
    axisLine: { lineStyle: { color: theme.hair } },
    axisTick: { show: false },
    axisLabel:
      axis === 0 ? { show: false } : { ...axisLabelStyle(theme), interval: 0, hideOverlap: true },
  })
  const value = (axis: number, name: string, show: (value: number) => string) => ({
    type: 'value' as const,
    gridIndex: axis,
    name,
    nameLocation: 'middle' as const,
    nameGap: 62,
    nameTextStyle: axisNameStyle(theme),
    axisLine: { show: false },
    axisTick: { show: false },
    axisLabel: { ...axisLabelStyle(theme), formatter: show },
    splitLine: { lineStyle: { color: theme.hair } },
    splitNumber: 3,
  })

  return {
    ...baseOption(theme, description),
    legend: { ...legend(theme, named), top: 0, bottom: undefined },
    grid: [
      { left: 84, right: 16, top: 32, height: '34%' },
      { left: 84, right: 16, top: '54%', bottom: paged ? 64 : 32 },
    ],
    ...(paged
      ? {
          dataZoom: zoomPair(theme, [0, 1]).map((zoom) => ({
            ...zoom,
            start: 0,
            end: (WINDOW / columns.length) * 100,
            minValueSpan: 3,
          })),
        }
      : {}),
    tooltip: {
      ...tooltip(theme),
      trigger: 'axis',
      axisPointer: { type: 'shadow', shadowStyle: { color: `${theme.hair}55` } },
      formatter: (raw: unknown) => {
        const first = Array.isArray(raw) ? raw[0] : raw
        const index =
          first !== null && typeof first === 'object'
            ? Number(Reflect.get(first, 'dataIndex'))
            : Number.NaN
        const column = columns[index]
        if (column === undefined) return ''
        return [
          column.label,
          `weighted estimate <strong>${rate(column.value.estimate)}</strong>${interval(column.value, rate)}`,
          `difference from observed ${signed(column.difference.estimate)}${interval(column.difference, signed)}`,
          ...(column.randomised === null ? [] : [`randomised estimate ${rate(column.randomised)}`]),
          ...(column.bounds === null
            ? []
            : [`natural bounds ${rate(column.bounds.lower)} to ${rate(column.bounds.upper)}`]),
        ].join('<br/>')
      },
    },
    axisPointer: { link: [{ xAxisIndex: 'all' }] },
    xAxis: [category(0), category(1)],
    yAxis: [
      {
        ...value(0, outcome, rate),
        min: 0,
        max: highest > 0 ? tidyCeiling(highest * 1.4) : undefined,
      },
      value(1, 'Difference', signed),
    ],
    series: [
      ...columns.map((column, index) => ({
        type: 'line' as const,
        name: index === firstBounds ? 'Natural bounds' : `${column.label} bounds`,
        data:
          column.bounds === null
            ? []
            : [
                [index, column.bounds.lower],
                [index, column.bounds.upper],
              ],
        lineStyle: { color: theme.line, width: 14, cap: 'butt' as const },
        itemStyle: { color: theme.line },
        symbol: 'none',
        silent: true,
        // Line series default to z 3 and scatter series to z 2; the bars must sit under every mark.
        z: 1,
      })),
      ...intervals(columns, (column) => column.value, 0, theme, 'interval'),
      ...intervals(columns, (column) => column.difference, 1, theme, 'paired interval'),
      {
        type: 'scatter',
        name: 'Weighted estimate',
        data: columns.map((column, index) => ({ value: [index, column.value.estimate] })),
        itemStyle: { color: theme.info },
        symbolSize: 9,
        z: 4,
        markLine: reference(theme, observed, 'Observed average'),
      },
      {
        type: 'scatter',
        name: 'Difference',
        xAxisIndex: 1,
        yAxisIndex: 1,
        data: columns.map((column, index) => ({ value: [index, column.difference.estimate] })),
        itemStyle: { color: theme.info },
        symbolSize: 9,
        z: 4,
        markLine: reference(theme, 0, 'No difference'),
      },
      {
        type: 'scatter',
        name: 'Randomised estimate',
        data: columns.flatMap((column, index) =>
          column.randomised === null ? [] : [{ value: [index, column.randomised] }],
        ),
        symbol: 'rect',
        symbolSize: [16, 3],
        itemStyle: { color: theme.signal },
        z: 5,
      },
    ],
  }
}
