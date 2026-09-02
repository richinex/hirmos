import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, responsive, tooltip, valueAxis, zoomPair } from '../grammar'
import type { ChartTheme } from '../theme'

export interface SeriesOverviewView {
  readonly name: string
  /** One value per row in row order; NaN marks a missing cell and breaks the line. */
  readonly values: Float64Array
  /** Row order is the x axis; the label names what one step means. */
  readonly stepLabel: string
}

/** One column in row order: a hairline `bone` line, LTTB-sampled beyond a few thousand points, with a slider zoom. */
export function seriesOverviewOption(view: SeriesOverviewView, theme: ChartTheme): EChartsCoreOption {
  const observed = view.values.length
  let min = Number.POSITIVE_INFINITY
  let max = Number.NEGATIVE_INFINITY
  let missing = 0
  for (const value of view.values) {
    if (Number.isNaN(value)) { missing += 1; continue }
    if (value < min) min = value
    if (value > max) max = value
  }
  const description = `${view.name} across ${formatCount(observed).text} ${view.stepLabel}s, ranging from ${formatStatistic('raw', min).text} to ${formatStatistic('raw', max).text}.${missing > 0 ? ` ${formatCount(missing).text} missing cells break the line.` : ''}`
  const data = Array.from(view.values, (value, index) => [index + 1, Number.isNaN(value) ? null : value])
  const base = {
    ...baseOption(theme, description),
    // The slider sits under the axis name; the grid reserves nothing for it.
    grid: gridAuto({ bottom: 64 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const first = Array.isArray(raw) ? raw[0] : raw
        if (first === null || typeof first !== 'object') return ''
        const point = Reflect.get(first, 'data')
        if (!Array.isArray(point)) return ''
        const [step, value] = point
        return `${view.stepLabel} ${String(step)}<br/><strong>${value === null ? 'missing' : formatStatistic('raw', Number(value)).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      min: 1,
      max: observed,
      name: view.stepLabel,
      nameLocation: 'middle',
      nameGap: 22,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: axisLabelStyle(theme),
      splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme), scale: true },
    dataZoom: zoomPair(theme),
    series: [{
      type: 'line',
      name: view.name,
      data,
      showSymbol: false,
      connectNulls: false,
      sampling: 'minmax',
      lineStyle: { color: theme.bone, width: 1.2 },
      itemStyle: { color: theme.bone },
      emphasis: { lineStyle: { width: 1.2 } },
    }],
  }
  return responsive(base, { narrow: { grid: { bottom: 8 }, dataZoom: [{}, { show: false }] } })
}
