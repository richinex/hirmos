import type { EChartsCoreOption } from 'echarts/core'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { baseOption, categoryAxis, gridAuto, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface HistogramView {
  readonly name: string
  /** Bin edges, `counts.length + 1` long. */
  readonly edges: readonly number[]
  readonly counts: readonly number[]
  readonly nullCount: number
}

const edgeLabel = (value: number): string => formatStatistic('raw', value).text

/** Distribution of one numeric column: bars on `bone`, one bin per bar, counts in the tooltip. */
export function histogramOption(view: HistogramView, theme: ChartTheme): EChartsCoreOption {
  const labels = view.counts.map((_, index) => edgeLabel(view.edges[index]))
  const total = view.counts.reduce((sum, count) => sum + count, 0)
  const peak = Math.max(0, ...view.counts)
  const description = `Histogram of ${view.name}: ${formatCount(total).text} values in ${view.counts.length} bins from ${edgeLabel(view.edges[0])} to ${edgeLabel(view.edges[view.edges.length - 1])}; the largest bin holds ${formatCount(peak).text} values.${view.nullCount > 0 ? ` ${formatCount(view.nullCount).text} null cells are not drawn.` : ''}`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 6 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'shadow', shadowStyle: { color: theme.hair, opacity: 0.4 } },
      formatter: (raw: unknown) => {
        const first = Array.isArray(raw) ? raw[0] : raw
        if (first === null || typeof first !== 'object') return ''
        const index = Reflect.get(first, 'dataIndex')
        if (typeof index !== 'number') return ''
        const lower = view.edges[index]
        const upper = view.edges[index + 1]
        return `${edgeLabel(lower)} to ${edgeLabel(upper)}<br/><strong>${formatCount(view.counts[index]).text}</strong> values`
      },
    },
    xAxis: categoryAxis(theme, labels),
    yAxis: { ...valueAxis(theme), min: 0 },
    series: [{
      type: 'bar',
      name: view.name,
      data: [...view.counts],
      barCategoryGap: '8%',
      itemStyle: { color: theme.bone, borderRadius: [2, 2, 0, 0] },
      emphasis: { itemStyle: { color: theme.ink } },
    }],
  }
}
