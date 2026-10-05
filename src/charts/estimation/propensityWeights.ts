import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import {
  axisLabelStyle,
  axisNameStyle,
  baseOption,
  escapeHtml,
  gridAuto,
  legend,
  rangeSelection,
  tooltip,
  valueAxis,
} from '../grammar'
import type { ChartTheme } from '../theme'

export interface PropensityWeightView {
  readonly treatment: string
  readonly outcomeName: string
  readonly propensity: readonly number[]
  readonly treated: readonly boolean[]
  readonly weights: readonly number[]
  readonly outcome: readonly number[]
}

const MIN_SYMBOL = 3
const MAX_SYMBOL = 22
const STEPS = 6

export function propensityWeightOption(
  view: PropensityWeightView,
  theme: ChartTheme,
): EChartsCoreOption {
  let lightest = Infinity
  let heaviest = -Infinity
  for (const weight of view.weights) {
    lightest = Math.min(lightest, weight)
    heaviest = Math.max(heaviest, weight)
  }
  const span = heaviest - lightest
  const step = (weight: number) =>
    span === 0 ? 0 : Math.min(STEPS - 1, Math.floor(Math.sqrt((weight - lightest) / span) * STEPS))
  const sizeOf = (index: number) => MIN_SYMBOL + (MAX_SYMBOL - MIN_SYMBOL) * ((index + 0.5) / STEPS)

  const arms = [
    { label: 'Not treated', treated: false, colour: theme.info },
    { label: 'Treated', treated: true, colour: theme.signal },
  ] as const

  const heavy = view.weights.filter((weight) => weight > 10).length
  const description =
    `${view.outcomeName} against the fitted propensity score, each row drawn at its weight.` +
    ` Weights run from ${formatStatistic('raw', lightest).text} to ${formatStatistic('raw', heaviest).text}` +
    `${heavy === 0 ? '' : `, with ${heavy} rows above 10`}.`

  return {
    ...baseOption(theme, description),
    animation: false,
    legend: {
      ...legend(
        theme,
        arms.map((arm) => arm.label),
      ),
      top: 0,
      bottom: undefined,
    },
    grid: gridAuto({ top: 34, bottom: 44 }),
    ...rangeSelection(theme, 0, { slider: false }),
    dataZoom: rangeSelection(theme, 0, { slider: false }).dataZoom.map((entry) => ({
      ...entry,
      filterMode: 'filter' as const,
      throttle: 100,
    })),
    xAxis: {
      type: 'value' as const,
      scale: true,
      name: `P(${view.treatment} | covariates)`,
      nameLocation: 'middle' as const,
      nameGap: 28,
      nameTextStyle: axisNameStyle(theme),
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      splitLine: { show: false },
      axisLabel: {
        ...axisLabelStyle(theme),
        formatter: (value: number) => formatStatistic('raw', value).text,
      },
    },
    yAxis: {
      ...valueAxis(theme, view.outcomeName),
      nameGap: 48,
      splitLine: { lineStyle: { color: theme.hair, type: 'dotted' as const } },
      axisLabel: {
        ...axisLabelStyle(theme),
        formatter: (value: number) => formatStatistic('raw', value).text,
      },
    },
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const entry = Array.isArray(raw) ? raw[0] : raw
        const value: unknown =
          entry === null || typeof entry !== 'object' ? null : Reflect.get(entry, 'value')
        if (!Array.isArray(value)) return ''
        const name = String(Reflect.get(entry as object, 'seriesName') ?? '')
        return (
          `${escapeHtml(name)}<br/>score ${formatStatistic('raw', Number(value[0])).text}` +
          `<br/>${escapeHtml(view.outcomeName)} ${formatStatistic('raw', Number(value[1])).text}` +
          `<br/>weight <strong>${formatStatistic('raw', Number(value[2])).text}</strong>`
        )
      },
    },
    series: arms.flatMap((arm) => {
      const buckets: number[][][] = Array.from({ length: STEPS }, () => [])
      for (const [row, score] of view.propensity.entries()) {
        if (view.treated[row] !== arm.treated) continue
        const weight = view.weights[row]!
        buckets[step(weight)]!.push([score, view.outcome[row]!, weight])
      }
      return buckets.flatMap((data, index) =>
        data.length === 0
          ? []
          : [
              {
                type: 'scatter' as const,
                name: arm.label,
                progressive: 4000,
                progressiveThreshold: 4000,
                symbolSize: sizeOf(index),
                itemStyle: { color: arm.colour, opacity: 0.35 },
                data,
              },
            ],
      )
    }),
  }
}
