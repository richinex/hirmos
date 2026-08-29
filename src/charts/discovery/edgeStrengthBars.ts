import type { EChartsCoreOption } from 'echarts/core'
import { formatP, formatStatistic } from '@/lib/format/number'
import { baseOption, gridAuto, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface EdgeStrengthBarsView {
  readonly title: string
  readonly edges: readonly {
    readonly name: string
    readonly strength: number
    readonly pValue?: number
  }[]
  readonly quantity: string
}

/** Selected relations ranked by strength, strongest at the top; horizontal so long names stay readable. */
export function edgeStrengthBarsOption(view: EdgeStrengthBarsView, theme: ChartTheme): EChartsCoreOption {
  const ranked = [...view.edges].sort((left, right) => left.strength - right.strength)
  const description = `${view.title}: ${view.edges.length} relations ranked by ${view.quantity}; the strongest is ${ranked.at(-1)?.name ?? 'none'} at ${formatStatistic('score', ranked.at(-1)?.strength ?? 0).text}.`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ left: 4, top: 8, bottom: 6 }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const index = Reflect.get(raw, 'dataIndex')
        if (typeof index !== 'number') return ''
        const edge = ranked[index]
        return `${edge.name}<br/>${view.quantity} <strong>${formatStatistic('score', edge.strength).text}</strong>${edge.pValue === undefined ? '' : `<br/>${formatP(edge.pValue).text}`}`
      },
    },
    xAxis: { ...valueAxis(theme, view.quantity), scale: false, min: 0 },
    yAxis: {
      type: 'category',
      data: ranked.map((edge) => edge.name),
      axisLine: { show: false },
      axisTick: { show: false },
      // The bars carry their own names, so the axis reserves no gutter.
      axisLabel: { show: false },
    },
    series: [{
      type: 'bar',
      name: view.quantity,
      barMaxWidth: 20,
      barMinWidth: 2,
      label: { show: true, position: 'insideLeft', distance: 6, color: theme.panel, fontFamily: theme.font, fontSize: theme.labelSize, formatter: (raw: unknown) => { const index = raw !== null && typeof raw === 'object' ? Reflect.get(raw, 'dataIndex') : undefined; return typeof index === 'number' ? ranked[index]?.name ?? '' : '' } },
      data: ranked.map((edge) => ({ value: edge.strength, itemStyle: { color: theme.bone, borderRadius: [0, 2, 2, 0] } })),
      emphasis: { itemStyle: { color: theme.ink } },
    }],
  }
}
