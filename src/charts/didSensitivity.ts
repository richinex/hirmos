import { contours } from 'd3-contour'
import { ticks } from 'd3-array'
import type { DidSensitivityEvidence } from '@/domain/didSensitivity'
import type { ChartTheme } from './theme'
import { baseOption, categoryAxis, gridAuto, legend, tooltip, valueAxis } from './grammar'
import { formatSetPercent, formatStatistic, percentAsSet } from '@/lib/format/number'

const number = (v: number) => formatStatistic('raw', v).text
const shareName = ([y, d]: readonly [number, number]) =>
  `${formatSetPercent(y).text} / ${formatSetPercent(d).text}`
export function didBoundsOption(e: DidSensitivityEvidence, theme: ChartTheme) {
  const rows = [e.baseline, ...e.scenarios]
  const names = ['0% / 0%', ...e.request.scenarios.map(shareName)]
  return {
    ...baseOption(theme, 'DiD omitted-variable sensitivity bounds'),
    grid: gridAuto({ top: 50, bottom: 58 }),
    legend: { ...legend(theme, ['Confidence bounds', 'Effect bounds']), top: 0, bottom: 'auto' },
    tooltip: tooltip(theme, 'item'),
    // Every scenario keeps its name: four labels of about 55px fit a phone's plot, and a hidden one leaves a bar unnamed.
    xAxis: (() => {
      const axis = categoryAxis(theme, names, 'Outcome share / Riesz share')
      return { ...axis, axisLabel: { ...axis.axisLabel, interval: 0, hideOverlap: false } }
    })(),
    yAxis: valueAxis(theme, 'ATT'),
    series: [
      {
        name: 'Confidence bounds',
        type: 'line' as const,
        symbol: 'circle',
        symbolSize: 5,
        connectNulls: false,
        data: rows.flatMap((r, i) => [[i, r.interval[0]], [i, r.interval[1]], null]),
        lineStyle: { color: theme.info, width: 2 },
        itemStyle: { color: theme.info },
      },
      {
        name: 'Effect bounds',
        type: 'line' as const,
        symbol: 'circle',
        symbolSize: 7,
        connectNulls: false,
        data: rows.flatMap((r, i) => [[i, r.effect[0]], [i, r.effect[1]], null]),
        lineStyle: { color: theme.signal, width: 4 },
        itemStyle: { color: theme.signal },
        markLine: {
          silent: true,
          symbol: 'none',
          label: {
            show: true,
            position: 'insideEndTop' as const,
            formatter: 'Null',
            color: theme.muted,
          },
          lineStyle: { color: theme.muted, type: 'dashed' as const },
          data: [{ yAxis: e.request.null }],
        },
      },
    ],
  }
}

// D3 samples are at i+0.5. Convert back to the requested shares, including nonuniform axes.
export function contourShare(position: number, axis: readonly number[]): number {
  const offset = Math.max(0, Math.min(axis.length - 1, position - 0.5))
  const i = Math.min(axis.length - 2, Math.floor(offset))
  return axis[i]! + (offset - i) * (axis[i + 1]! - axis[i]!)
}
export function didContourOption(
  e: DidSensitivityEvidence,
  theme: ChartTheme,
  quantity: 'effect' | 'interval',
) {
  const side = e.estimate >= e.request.null ? 0 : 1
  const values = e.grid.flatMap((row) => row.map((b) => b[quantity][side]))
  const low = Math.min(...values),
    high = Math.max(...values)
  // Round levels, plus the null, so each line reads as a value a person would pick.
  const thresholds = Array.from(new Set([e.request.null, ...ticks(low, high, 6)]))
    .filter((v) => v > low && v < high)
    .sort((a, b) => a - b)
  const xs = e.request.rieszShares,
    ys = e.request.outcomeShares
  const shapes = contours().size([xs.length, ys.length]).thresholds(thresholds)(values)
  // Levels are values, not categories: one neutral line each, its value written at its end, and the null in signal.
  const levels = shapes.map((shape) => {
    const atNull = shape.value === e.request.null
    const color = atNull ? theme.signal : theme.muted
    const data = shape.coordinates.flatMap((polygon) =>
      polygon.flatMap((ring) => [
        // Do not draw the artificial boundary around D3's sampled rectangle as an isoline.
        ...ring.map(([x, y]) =>
          x! < 0.5 || y! < 0.5 || x! > xs.length - 0.5 || y! > ys.length - 0.5
            ? null
            : [contourShare(x!, xs) * 100, contourShare(y!, ys) * 100],
        ),
        null,
      ]),
    )
    while (data.at(-1) === null) data.pop()
    return {
      name: number(shape.value),
      type: 'line' as const,
      symbol: 'none',
      connectNulls: false,
      data,
      lineStyle: {
        color,
        width: atNull ? 2.5 : 1.25,
        type: atNull ? ('dashed' as const) : ('solid' as const),
      },
      itemStyle: { color },
      endLabel: {
        show: true,
        distance: 4,
        offset: [0, -6] as [number, number],
        formatter: number(shape.value),
        textBorderColor: theme.panel,
        textBorderWidth: 3,
        color: atNull ? theme.ink : theme.muted,
        fontSize: theme.labelSize,
        fontWeight: atNull ? 700 : 400,
      },
    }
  })
  const points = {
    name: 'Scenarios',
    type: 'scatter' as const,
    symbolSize: 8,
    itemStyle: { color: theme.panel, borderColor: theme.ink, borderWidth: 1.5 },
    // A halo in the panel colour keeps each name legible where a contour runs under it.
    label: {
      show: true,
      position: 'right' as const,
      formatter: '{b}',
      color: theme.ink,
      fontSize: theme.labelSize,
      textBorderColor: theme.panel,
      textBorderWidth: 3,
    },
    data: [
      // The origin sits on both axes, so its name goes above the x axis rather than along it.
      { name: 'No confounding', value: [0, 0], label: { position: [10, -16] as [number, number] } },
      ...e.request.scenarios.map(([y, d]) => ({
        name: shareName([y, d]),
        value: [percentAsSet(d), percentAsSet(y)],
      })),
    ],
  }
  const series = [...levels, points]
  return {
    ...baseOption(theme, 'DiD sensitivity contour grid'),
    grid: gridAuto({ top: 44, bottom: 48, right: 40 }),
    tooltip: tooltip(theme, 'item'),
    xAxis: { ...valueAxis(theme, 'Riesz share (%)'), min: xs[0]! * 100, max: xs.at(-1)! * 100 },
    yAxis: { ...valueAxis(theme, 'Outcome share (%)'), min: ys[0]! * 100, max: ys.at(-1)! * 100 },
    series,
  }
}
