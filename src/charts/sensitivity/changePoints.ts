import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, responsive, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface ChangePointsChartView {
  readonly name: string
  readonly values: readonly number[]
  /** One-based row numbers where a new segment starts. */
  readonly changePoints: readonly number[]
  readonly stepLabel: string
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
    },
    yAxis: { ...valueAxis(theme), scale: true },
    series: [{
      type: 'line',
      name: view.name,
      data: view.values.map((value, index) => [index + 1, value]),
      symbol: 'none',
      lineStyle: { color: theme.ink, width: 1.2 },
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
