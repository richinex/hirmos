import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { baseOption, escapeHtml, gridAuto, responsive, tooltip } from '../grammar'
import type { ChartTheme } from '../theme'

export interface MatrixHeatmapView {
  readonly title: string
  /** Row labels: the sources, drawn top to bottom. */
  readonly sources: readonly string[]
  /** Column labels: the targets. */
  readonly targets: readonly string[]
  /** `values[source][target]`; null cells are drawn empty. */
  readonly values: readonly (readonly (number | null)[])[]
  /** 'signed' uses a diverging ramp centred on zero; 'magnitude' a single-hue ramp from zero. */
  readonly scale: 'signed' | 'magnitude'
  /** Name of the quantity in a cell, for the tooltip and the description. */
  readonly quantity: string
  /** Symmetric matrices compare an unordered pair and must not be narrated as a directed edge. */
  readonly relation?: 'directed' | 'symmetric'
  /** Text drawn in a cell instead of its number, for example a link mark. */
  readonly cellText?: (source: number, target: number) => string | null
  /** Cells whose magnitude reaches this are outlined, so a threshold the reader set is visible where it bites. */
  readonly threshold?: number
}

/**
 * A source-by-target matrix as a heatmap. Signed values run `info → panel → signal` (negative, zero,
 * positive); magnitudes run `panel → signal`. Neither ramp is a category colour.
 */
export function matrixHeatmapOption(view: MatrixHeatmapView, theme: ChartTheme): EChartsCoreOption {
  const cells: (readonly [number, number, number | null])[] = []
  let peak = 0
  let filled = 0
  view.values.forEach((row, source) => row.forEach((value, target) => {
    cells.push([target, view.sources.length - 1 - source, value])
    if (value !== null) {
      filled += 1
      peak = Math.max(peak, Math.abs(value))
    }
  }))
  const magnitude = Math.max(peak, 1e-9)
  // On a symmetric matrix the diagonal is a variable against itself, which no threshold is about.
  const reaches = (x: number, y: number, value: number | null): boolean =>
    view.threshold !== undefined && value !== null && Math.abs(value) >= view.threshold && !(view.relation === 'symmetric' && view.sources.length - 1 - y === x)
  const flagged = cells.filter(([x, y, value]) => reaches(x, y, value)).length
  const thresholdNote = view.threshold === undefined ? '' : ` ${flagged} cells reach the threshold of ${formatStatistic('score', view.threshold).text} and are outlined.`
  const description = view.relation === 'symmetric'
    ? `${view.title}: ${view.sources.length} by ${view.targets.length}, ${filled} cells with a ${view.quantity}; the largest magnitude is ${formatStatistic('score', peak).text}.${thresholdNote}`
    : `${view.title}: ${view.sources.length} sources by ${view.targets.length} targets, ${filled} cells with a ${view.quantity}; the largest magnitude is ${formatStatistic('score', peak).text}.${thresholdNote}`
  const showLabels = view.sources.length * view.targets.length <= 64
  // Neither axis is named. The row and column labels already carry the variables, an axis name at the
  // grid's end would sit under the button that lifts the figure into a floating window, and on a
  // symmetric matrix 'source' and 'target' would assert a direction the quantity does not have.
  // The ramp sits under the grid, which reserves nothing for it: bar thickness, its labels and the offset.
  const rampReserve = 20 + theme.labelSize + 14
  const base = {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: rampReserve }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const value = Reflect.get(raw, 'value')
        if (!Array.isArray(value)) return ''
        const target = Number(value[0])
        const source = view.sources.length - 1 - Number(value[1])
        const cell = value[2]
        const text = cell === null || cell === undefined ? 'no relation' : `${view.quantity} <strong>${formatStatistic('score', Number(cell)).text}</strong>`
        const mark = view.cellText?.(source, target)
        const pair = view.relation === 'symmetric'
          ? `${escapeHtml(view.sources[source] ?? '')} × ${escapeHtml(view.targets[target] ?? '')}`
          : `${escapeHtml(view.sources[source] ?? '')} → ${escapeHtml(view.targets[target] ?? '')}`
        return `${pair}<br/>${text}${mark ? `<br/>${escapeHtml(mark)}` : ''}`
      },
    },
    xAxis: {
      type: 'category',
      data: [...view.targets],
      position: 'top',
      axisLine: { show: false },
      axisTick: { show: false },
      // Every label while they fit; past six columns the auto interval thins them instead of forcing an overlap.
      axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true, interval: view.targets.length <= 6 ? 0 : 'auto', rotate: 0 },
      splitArea: { show: false },
    },
    yAxis: {
      type: 'category',
      data: [...view.sources].reverse(),
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
    },
    visualMap: {
      type: 'continuous',
      min: view.scale === 'signed' ? -magnitude : 0,
      max: magnitude,
      dimension: 2,
      orient: 'horizontal',
      left: 'center',
      bottom: 4,
      calculable: false,
      itemHeight: 120,
      text: view.scale === 'signed' ? ['positive', 'negative'] : ['strong', 'none'],
      textStyle: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize },
      inRange: { color: view.scale === 'signed' ? [theme.info, theme.panel, theme.signal] : [theme.panel, theme.signal] },
      borderColor: theme.hair,
    },
    series: [{
      type: 'heatmap',
      name: view.quantity,
      data: cells.map(([x, y, value]) => (value === null
        ? { value: [x, y, null], itemStyle: { color: theme.line } }
        : reaches(x, y, value) ? { value: [x, y, value], itemStyle: { borderColor: theme.ink, borderWidth: 2 } } : [x, y, value])),
      label: {
        show: showLabels,
        color: theme.ink,
        fontFamily: theme.font,
        fontSize: theme.labelSize,
        formatter: (raw: unknown) => {
          if (raw === null || typeof raw !== 'object') return ''
          const value = Reflect.get(raw, 'value')
          if (!Array.isArray(value)) return ''
          const target = Number(value[0])
          const source = view.sources.length - 1 - Number(value[1])
          const mark = view.cellText?.(source, target)
          if (mark !== undefined && mark !== null) return mark
          return typeof value[2] === 'number' ? formatStatistic('score', value[2]).text : ''
        },
      },
      itemStyle: { borderColor: theme.hair, borderWidth: 1 },
      emphasis: { itemStyle: { borderColor: theme.ink, borderWidth: 2 } },
      labelLayout: { hideOverlap: true },
    }],
  }
  return responsive(base, {
    wide: {
      xAxis: { axisLabel: { rotate: 45 } },
      visualMap: { itemHeight: 80 },
      series: [{ label: { show: showLabels && view.sources.length * view.targets.length <= 36 } }],
    },
    narrow: {
      grid: { right: 60, bottom: 8 },
      xAxis: { axisLabel: { rotate: 90, width: 60, overflow: 'truncate' } },
      visualMap: { orient: 'vertical', left: 'auto', right: 0, top: 'middle', bottom: 'auto', itemHeight: 100 },
      series: [{ label: { show: false } }],
    },
  })
}
