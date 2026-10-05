import type { EChartsCoreOption } from 'echarts/core'
import type { HistogramBins } from '@/domain/dataset'
import { formatCount, formatStatistic } from '@/lib/format/number'
import {
  baseOption,
  categoryAxis,
  gridAuto,
  tooltip,
  valueAxis,
  type ReferenceMark,
} from '../grammar'
import { variableColour, type ChartTheme } from '../theme'

export interface HistogramView {
  readonly name: string
  /** The same bins the schema table's micro-histogram draws, so the two never disagree. */
  readonly bins: HistogramBins
  readonly nullCount: number
  /** Values drawn as rules across the bars, for instance the mean and the median printed beside the chart. */
  readonly marks?: readonly ReferenceMark[]
}

const edgeLabel = (value: number): string => formatStatistic('raw', value).text

/**
 * Where a value falls on the category axis: bins are categories, so a value maps to a fractional bin
 * index, with the bin's centre at the whole number.
 */
const binPosition = (bins: HistogramBins, value: number): number => {
  const low = bins.edges[0]
  const high = bins.edges[bins.edges.length - 1]
  if (high <= low) return 0
  return ((value - low) / (high - low)) * bins.counts.length - 0.5
}

/** Distribution of one numeric column: one bin per bar, counts in the tooltip. */
export function histogramOption(view: HistogramView, theme: ChartTheme): EChartsCoreOption {
  const { edges, counts } = view.bins
  const labels = counts.map((_, index) => edgeLabel(edges[index]))
  const total = counts.reduce((sum, count) => sum + count, 0)
  const peak = Math.max(0, ...counts)
  const marks = view.marks ?? []
  const description = `Histogram of ${view.name}: ${formatCount(total).text} values in ${counts.length} bins from ${edgeLabel(edges[0])} to ${edgeLabel(edges[edges.length - 1])}; the largest bin holds ${formatCount(peak).text} values.${marks.length > 0 ? ` Rules mark the ${marks.map((mark) => `${mark.name} at ${edgeLabel(mark.value)}`).join(' and the ')}.` : ''}${view.nullCount > 0 ? ` ${formatCount(view.nullCount).text} null cells are not drawn.` : ''}`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ top: marks.length > 0 ? 22 : 8, bottom: 6 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'shadow', shadowStyle: { color: theme.hair, opacity: 0.4 } },
      formatter: (raw: unknown) => {
        const first = Array.isArray(raw) ? raw[0] : raw
        if (first === null || typeof first !== 'object') return ''
        const index = Reflect.get(first, 'dataIndex')
        if (typeof index !== 'number') return ''
        return `${edgeLabel(edges[index])} to ${edgeLabel(edges[index + 1])}<br/><strong>${formatCount(counts[index]).text}</strong> values`
      },
    },
    xAxis: categoryAxis(theme, labels),
    yAxis: { ...valueAxis(theme), min: 0 },
    series: [
      {
        type: 'bar',
        name: view.name,
        data: [...counts],
        barCategoryGap: '8%',
        itemStyle: { color: variableColour(theme, view.name), borderRadius: [2, 2, 0, 0] },
        emphasis: {
          itemStyle: {
            color: variableColour(theme, view.name),
            borderColor: theme.ink,
            borderWidth: 1,
          },
        },
        markLine:
          marks.length === 0
            ? undefined
            : {
                silent: true,
                symbol: 'none',
                lineStyle: { color: theme.muted, type: 'dashed', width: 1 },
                label: {
                  color: theme.muted,
                  fontFamily: theme.font,
                  fontSize: theme.labelSize,
                  formatter: (params: { readonly name?: string }) => params.name ?? '',
                },
                // Two marks often fall a bin apart, so their labels take turns on either side of their rules.
                data: marks.map((mark, index) => ({
                  name: mark.name,
                  xAxis: binPosition(view.bins, mark.value),
                  label: { position: index % 2 === 0 ? 'insideEndTop' : 'insideEndBottom' },
                })),
              },
      },
    ],
  }
}
