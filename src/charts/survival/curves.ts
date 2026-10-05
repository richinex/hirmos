import type { EChartsCoreOption } from 'echarts/core'
import {
  axisLabelStyle,
  axisNameStyle,
  baseOption,
  categoryAxis,
  escapeHtml,
  gridAuto,
  legend,
  rangeSelection,
  tooltip,
  valueAxis,
  type ReferenceMark,
} from '@/charts/grammar'
import { seriesColour, type ChartTheme } from '@/charts/theme'
import { formatStatistic } from '@/lib/format/number'

export interface SurvivalSeries {
  readonly name: string
  readonly points: readonly (readonly [number, number])[]
}

export interface SurvivalSeriesWithCensoring extends SurvivalSeries {
  readonly censorTimes: readonly number[]
}

const percent = (value: number): string => `${(value * 100).toFixed(1)}%`

const timeAxis = (theme: ChartTheme, name: string, max?: number) => ({
  type: 'value' as const,
  min: 0,
  ...(max === undefined ? {} : { max }),
  name,
  nameLocation: 'middle' as const,
  nameGap: 25,
  nameTextStyle: axisNameStyle(theme),
  axisLine: { lineStyle: { color: theme.hair } },
  axisTick: { show: false },
  axisLabel: axisLabelStyle(theme),
  splitLine: { show: false },
})

const probabilityAxis = (theme: ChartTheme, name: string) => ({
  ...valueAxis(theme, name),
  min: 0,
  max: 1,
  axisLabel: {
    ...axisLabelStyle(theme),
    formatter: (value: number) => `${Math.round(value * 100)}%`,
  },
})

/** A vertical rule with its name above the plot, in the muted tone of a reference. */
const markLines = (marks: readonly ReferenceMark[], theme: ChartTheme) => ({
  markLine: {
    silent: true,
    symbol: 'none',
    lineStyle: { color: theme.muted, type: 'dashed', width: 1 },
    label: {
      color: theme.muted,
      fontFamily: theme.font,
      fontSize: theme.labelSize,
      position: 'insideEndTop',
      formatter: (raw: unknown) => String(Reflect.get(raw as object, 'name') ?? ''),
    },
    data: marks.map((mark) => ({ name: mark.name, xAxis: mark.value })),
  },
})

/**
 * Event-free probability over follow-up time: one fitted profile, or two observed groups as steps.
 * A single curve is filled to the axis so the area reads as time spent event-free; the fitted median
 * and any other reference times are dashed rules with their names.
 */
export function survivalCurvesOption(
  series: readonly SurvivalSeries[],
  timeLabel: string,
  theme: ChartTheme,
  {
    marks = [],
    fill = series.length === 1,
    stepped = series.length > 1,
  }: {
    readonly marks?: readonly ReferenceMark[]
    readonly fill?: boolean
    readonly stepped?: boolean
  } = {},
): EChartsCoreOption {
  return {
    ...baseOption(
      theme,
      `${series.map((item) => item.name).join(' and ')} event-free probability over ${timeLabel}${marks.length === 0 ? '' : `, marked at ${marks.map((mark) => `${mark.name} ${formatStatistic('raw', mark.value).text}`).join(' and ')}`}.`,
    ),
    // The slider sits under the axis name; a legend for two groups goes above the plot.
    grid: gridAuto({ top: stepped ? 30 : 16, bottom: 64 }),
    ...(stepped
      ? {
          legend: {
            ...legend(
              theme,
              series.map((item) => item.name),
            ),
            bottom: 'auto',
            top: 0,
          },
        }
      : {}),
    ...rangeSelection(theme),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const value = Reflect.get(first, 'value')
        const time = Array.isArray(value) ? Number(value[0]) : Number.NaN
        const lines = entries.flatMap((entry) => {
          if (entry === null || typeof entry !== 'object') return []
          const point = Reflect.get(entry, 'value')
          const probability = Array.isArray(point) ? Number(point[1]) : Number.NaN
          return Number.isFinite(probability)
            ? [
                `${escapeHtml(String(Reflect.get(entry, 'seriesName') ?? ''))} <strong>${percent(probability)}</strong>`,
              ]
            : []
        })
        return `${escapeHtml(timeLabel)} ${formatStatistic('raw', time).text}<br/>${lines.join('<br/>')}`
      },
    },
    xAxis: timeAxis(theme, timeLabel),
    yAxis: probabilityAxis(theme, 'event-free probability'),
    series: series.map((item, index) => {
      const colour = seriesColour(theme, index)
      return {
        type: 'line',
        name: item.name,
        data: item.points.map((point) => [...point]),
        step: stepped ? 'end' : false,
        showSymbol: false,
        lineStyle: { color: colour, width: 1.6 },
        itemStyle: { color: colour },
        ...(fill && index === 0 ? { areaStyle: { color: colour, opacity: 0.16 } } : {}),
        ...(index === 0 ? markLines(marks, theme) : {}),
        z: stepped ? 2 + index : 2,
      }
    }),
  }
}

const stepValueAt = (points: readonly (readonly [number, number])[], time: number): number => {
  let value = 1
  for (const point of points) {
    if (point[0] > time) break
    value = point[1]
  }
  return value
}

/** Kaplan–Meier curves with the censoring marks used by survival::plot.survfit. */
export function observedSurvivalOption(
  series: readonly SurvivalSeriesWithCensoring[],
  timeLabel: string,
  theme: ChartTheme,
  marks: readonly ReferenceMark[],
): EChartsCoreOption {
  const colours = theme.categorical
  return {
    ...survivalCurvesOption(series, timeLabel, theme, { marks, fill: false }),
    series: series.flatMap((item, index) => {
      const colour = colours[index % colours.length]
      return [
        {
          type: 'line',
          name: item.name,
          data: item.points.map((point) => [...point]),
          step: 'end',
          showSymbol: false,
          lineStyle: { color: colour, width: 1.6 },
          itemStyle: { color: colour },
          ...(index === 0 ? markLines(marks, theme) : {}),
          z: 2 + index,
        },
        {
          type: 'scatter',
          name: `${item.name} censored`,
          data: item.censorTimes.map((time) => [time, stepValueAt(item.points, time)]),
          symbol: 'rect',
          symbolSize: [1.5, 9],
          silent: true,
          tooltip: { show: false },
          itemStyle: { color: colour },
          z: 4,
        },
      ]
    }),
  }
}

/** Two survival curves filled to zero through the chosen horizon: each filled area is restricted mean event-free time. */
export function restrictedMeanOption(
  series: readonly SurvivalSeries[],
  horizon: number,
  theme: ChartTheme,
): EChartsCoreOption {
  const colours = theme.categorical
  const throughHorizon = (
    points: readonly (readonly [number, number])[],
  ): readonly (readonly [number, number])[] => [
    [0, 1],
    ...points.filter((point) => point[0] > 0 && point[0] < horizon),
    [horizon, stepValueAt(points, horizon)],
  ]
  return {
    ...survivalCurvesOption(series, 'follow-up time', theme, {
      marks: [{ name: 'comparison horizon', value: horizon }],
      fill: false,
    }),
    aria: {
      enabled: true,
      label: {
        enabled: false,
        description: `Area under each event-free probability curve through follow-up time ${formatStatistic('raw', horizon).text}; this area is restricted mean event-free time.`,
      },
    },
    series: series.map((item, index) => ({
      type: 'line',
      name: item.name,
      data: throughHorizon(item.points).map((point) => [...point]),
      step: 'end',
      showSymbol: false,
      lineStyle: { color: colours[index % colours.length], width: 1.6 },
      itemStyle: { color: colours[index % colours.length] },
      areaStyle: { color: colours[index % colours.length], opacity: index === 0 ? 0.12 : 0.18 },
      ...(index === 0 ? markLines([{ name: 'comparison horizon', value: horizon }], theme) : {}),
      z: 2 + index,
    })),
  }
}

/** A two-group accumulated-risk or smoothed-risk chart returned by ComparisonSurv. */
export function comparisonMeasureOption(
  series: readonly SurvivalSeries[],
  yLabel: 'cumulative hazard' | 'smoothed hazard',
  theme: ChartTheme,
  stepped: boolean,
): EChartsCoreOption {
  const colours = theme.categorical
  return {
    ...baseOption(theme, `${yLabel} by group over follow-up time.`),
    grid: gridAuto({ top: 30, bottom: 64 }),
    legend: {
      ...legend(
        theme,
        series.map((item) => item.name),
      ),
      bottom: 'auto',
      top: 0,
    },
    ...rangeSelection(theme),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
    },
    xAxis: timeAxis(theme, 'follow-up time'),
    yAxis: { ...valueAxis(theme, yLabel), min: 0 },
    series: series.map((item, index) => ({
      type: 'line',
      name: item.name,
      data: item.points.map((point) => [...point]),
      step: stepped ? 'end' : false,
      showSymbol: false,
      lineStyle: { color: colours[index % colours.length], width: 1.6 },
      itemStyle: { color: colours[index % colours.length] },
    })),
  }
}

/** The fitted hazard over follow-up time: how event risk changes with age, which the family's shape decides. */
export function hazardCurveOption(
  times: readonly number[],
  hazard: readonly (number | null)[],
  timeLabel: string,
  theme: ChartTheme,
): EChartsCoreOption {
  return {
    ...baseOption(
      theme,
      `Fitted hazard over ${timeLabel}: the instantaneous event rate among those still event-free.`,
    ),
    // Follows the event-free chart's window: the drag and the wheel stay, the slider is the other chart's.
    grid: gridAuto({ top: 16, bottom: 30 }),
    ...rangeSelection(theme, 0, { slider: false }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const first = (Array.isArray(raw) ? raw : [raw])[0]
        if (first === null || typeof first !== 'object') return ''
        const value = Reflect.get(first, 'value')
        if (!Array.isArray(value)) return ''
        return `${escapeHtml(timeLabel)} ${formatStatistic('raw', Number(value[0])).text}<br/>hazard <strong>${formatStatistic('raw', Number(value[1])).text}</strong>`
      },
    },
    xAxis: timeAxis(theme, timeLabel),
    yAxis: { ...valueAxis(theme, 'hazard'), min: 0 },
    series: [
      {
        type: 'line',
        name: 'hazard',
        data: times.map((time, index) => [time, hazard[index] ?? Number.NaN]),
        showSymbol: false,
        lineStyle: { color: seriesColour(theme, 0), width: 1.4 },
        itemStyle: { color: seriesColour(theme, 0) },
        areaStyle: { color: seriesColour(theme, 0), opacity: 0.12 },
      },
    ],
  }
}

/** Probabilities of occupying each destination state, conditional on one selected starting state. */
export function stateOccupancyOption(
  times: readonly number[],
  states: readonly number[],
  probabilities: readonly (readonly number[])[],
  initial: number,
  theme: ChartTheme,
): EChartsCoreOption {
  const source = Math.max(0, states.indexOf(initial))
  const colours = theme.categorical
  return {
    ...baseOption(
      theme,
      `Probability of each state over time when follow-up starts in state ${states[source]}.`,
    ),
    grid: gridAuto({ top: 30, bottom: 64 }),
    legend: {
      ...legend(
        theme,
        states.map((state) => `state ${state}`),
      ),
      bottom: 'auto',
      top: 0,
    },
    ...rangeSelection(theme),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries[0]
        if (first === null || typeof first !== 'object') return ''
        const value = Reflect.get(first, 'value')
        const time = Array.isArray(value) ? Number(value[0]) : Number.NaN
        const lines = entries.flatMap((entry) => {
          if (entry === null || typeof entry !== 'object') return []
          const point = Reflect.get(entry, 'value')
          const probability = Array.isArray(point) ? Number(point[1]) : Number.NaN
          return Number.isFinite(probability)
            ? [
                `${escapeHtml(String(Reflect.get(entry, 'seriesName') ?? ''))} <strong>${percent(probability)}</strong>`,
              ]
            : []
        })
        return `follow-up time ${formatStatistic('raw', time).text}<br/>${lines.join('<br/>')}`
      },
    },
    xAxis: timeAxis(theme, 'follow-up time'),
    yAxis: probabilityAxis(theme, 'state probability'),
    series: states.map((state, destination) => ({
      type: 'line',
      name: `state ${state}`,
      data: times.map((time, timeIndex) => [
        time,
        probabilities[timeIndex]?.[source * states.length + destination] ?? Number.NaN,
      ]),
      showSymbol: false,
      lineStyle: { width: 1.6, color: colours[destination % colours.length] },
      itemStyle: { color: colours[destination % colours.length] },
      stack: 'state probability',
      areaStyle: { color: colours[destination % colours.length], opacity: 0.16 },
    })),
  }
}

/** The allowed state changes in the fitted multi-state model. This is a transition structure, not a causal graph. */
export function transitionMapOption(
  states: readonly number[],
  transitions: readonly (readonly [number, number])[],
  theme: ChartTheme,
): EChartsCoreOption {
  return {
    ...baseOption(
      theme,
      `Transition structure with ${states.length} states and ${transitions.length} permitted directed transitions.`,
    ),
    tooltip: tooltip(theme),
    series: [
      {
        type: 'graph',
        layout: 'circular',
        roam: true,
        symbolSize: 42,
        edgeSymbol: ['none', 'arrow'],
        edgeSymbolSize: [0, 8],
        label: { show: true, color: theme.ink, fontFamily: theme.font, fontSize: theme.labelSize },
        lineStyle: { color: theme.muted, width: 1.4, curveness: 0.12 },
        itemStyle: { color: theme.panel, borderColor: theme.ink, borderWidth: 1.5 },
        data: states.map((state, index) => ({ id: String(index), name: `state ${state}` })),
        links: transitions.map(([from, to]) => ({ source: String(from), target: String(to) })),
      },
    ],
  }
}

/** One complete state-to-state transition-probability matrix at a named follow-up time. */
export function transitionMatrixOption(
  states: readonly number[],
  matrix: readonly number[],
  time: number,
  theme: ChartTheme,
): EChartsCoreOption {
  const labels = states.map((state) => `state ${state}`)
  return {
    ...baseOption(
      theme,
      `Transition probabilities at follow-up time ${formatStatistic('raw', time).text}; rows are starting states and columns are destination states.`,
    ),
    grid: gridAuto({ left: 56, right: 16, top: 12, bottom: 50 }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const value = Reflect.get(raw, 'value')
        if (!Array.isArray(value)) return ''
        const destination = states[Number(value[0])]
        const source = states[Number(value[1])]
        return `state ${source} to state ${destination}<br/><strong>${percent(Number(value[2]))}</strong>`
      },
    },
    xAxis: categoryAxis(theme, labels, 'destination state'),
    yAxis: { ...categoryAxis(theme, labels, 'starting state'), inverse: true },
    visualMap: {
      min: 0,
      max: 1,
      show: false,
      inRange: { color: [theme.panel, theme.hair, theme.signal] },
    },
    series: [
      {
        type: 'heatmap',
        data: states.flatMap((_, source) =>
          states.map((__, destination) => [
            destination,
            source,
            matrix[source * states.length + destination] ?? Number.NaN,
          ]),
        ),
        label: {
          show: true,
          color: theme.ink,
          fontFamily: theme.font,
          fontSize: theme.labelSize,
          formatter: (raw: unknown) => {
            if (raw === null || typeof raw !== 'object') return ''
            const value = Reflect.get(raw, 'value')
            return Array.isArray(value) ? percent(Number(value[2])) : ''
          },
        },
        itemStyle: { borderColor: theme.panel, borderWidth: 2 },
      },
    ],
  }
}
