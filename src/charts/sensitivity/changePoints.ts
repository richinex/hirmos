import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, responsive, tooltip, valueAxis, rangeSelection } from '../grammar'
import { variableColour, type ChartTheme } from '../theme'

export interface ChangePointsChartView {
  readonly name: string
  readonly values: readonly (number | null)[]
  /** One-based row numbers where a new segment starts. */
  readonly changePoints: readonly number[]
  readonly stepLabel: string
  /**
   * Offer the zoom slider. A figure the reader scrubs earns the control; a thumbnail read at a
   * glance does not, and on a short strip the slider costs a quarter of the plot.
   */
  readonly zoom?: boolean
}

/** The series as a hairline with a dashed rule at every PELT segment boundary. */
export function changePointsOption(view: ChangePointsChartView, theme: ChartTheme): EChartsCoreOption {
  const description = `${view.name} across ${formatCount(view.values.length).text} ${view.stepLabel}s with ${formatCount(view.changePoints.length).text} change point${view.changePoints.length === 1 ? '' : 's'}${view.changePoints.length > 0 ? ` at ${view.changePoints.join(', ')}` : ''}.`
  return responsive({
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 6 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const first = Array.isArray(raw) ? raw[0] : raw
        if (first === null || typeof first !== 'object') return ''
        const index = Number(Reflect.get(first, 'dataIndex'))
        return `${view.stepLabel} ${index + 1}<br/><strong>${formatStatistic('raw', view.values[index] ?? Number.NaN).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      min: 1,
      max: Math.max(2, view.values.length),
      minInterval: 1,
      name: view.stepLabel,
      nameLocation: 'middle',
      nameGap: 22,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true, alignMinLabel: 'left', alignMaxLabel: 'right' },
      splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme), scale: true },
    ...(view.zoom === false ? {} : { ...rangeSelection(theme) }),
    series: [{
      type: 'line',
      name: view.name,
      data: view.values.map((value, index) => [index + 1, value]),
      // Thousands of points into a short strip: downsample to the pixels available. `minmax` emits the
      // real minimum and maximum of each frame in their original order, so a spike is never smoothed
      // away or dropped by a heuristic — which matters when the outliers are what the plot is for.
      sampling: 'minmax',
      symbol: 'none',
      connectNulls: false,
      lineStyle: { color: variableColour(theme, view.name), width: 1.2 },
      itemStyle: { color: variableColour(theme, view.name) },
      markLine: {
        symbol: 'none',
        silent: true,
        label: { show: true, position: 'insideEndTop', color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, formatter: (params: { readonly value?: unknown }) => String(params.value ?? '') },
        lineStyle: { color: theme.signal, type: 'dashed', width: 1 },
        data: view.changePoints.map((point, index) => ({ xAxis: point, value: point, label: { position: index % 2 === 0 ? 'insideEndTop' : 'insideEndBottom' } })),
      },
    }],
  }, {
    wide: { xAxis: { nameGap: 16 } },
    narrow: { xAxis: { name: '', nameGap: 0 } },
  })
}
