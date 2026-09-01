import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface PosteriorDensityView {
  readonly outcome: string
  readonly treatment: string
  readonly histogramStart: number
  readonly histogramBinWidth: number
  readonly histogramCounts: readonly number[]
  readonly mean: number
  readonly hdiLower: number
  readonly hdiUpper: number
}

/** The pooled posterior draws of the effect as a density histogram, the 94% highest-density band, and lines at no effect and the posterior mean. */
export function posteriorDensityOption(view: PosteriorDensityView, theme: ChartTheme): EChartsCoreOption {
  const total = view.histogramCounts.reduce((sum, count) => sum + count, 0) || 1
  const bars = view.histogramCounts.map((count, index) => [
    view.histogramStart + (index + 0.5) * view.histogramBinWidth,
    count / (total * view.histogramBinWidth),
  ])
  const description = `Posterior density of the effect of ${view.treatment} on ${view.outcome}: mean ${formatStatistic('raw', view.mean).text}, 94% highest-density interval ${formatStatistic('raw', view.hdiLower).text} to ${formatStatistic('raw', view.hdiUpper).text}.`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 46, top: 28 }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const value = raw !== null && typeof raw === 'object' ? Reflect.get(raw, 'value') : null
        if (!Array.isArray(value)) return ''
        return `effect ${formatStatistic('raw', Number(value[0])).text}<br/>density <strong>${formatStatistic('raw', Number(value[1])).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      name: `effect on ${view.outcome}`,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: axisLabelStyle(theme),
    },
    yAxis: { ...valueAxis(theme, 'density'), scale: false },
    series: [
      {
        type: 'bar',
        name: 'posterior',
        data: bars,
        barWidth: '96%',
        itemStyle: { color: theme.info, opacity: 0.55 },
        markArea: {
          silent: true,
          itemStyle: { color: theme.info, opacity: 0.14 },
          label: { show: true, position: 'insideBottomLeft', color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, formatter: '94% HDI' },
          data: [[{ xAxis: view.hdiLower }, { xAxis: view.hdiUpper }]],
        },
        markLine: {
          silent: true,
          symbol: 'none',
          label: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, formatter: (entry: { name?: string }) => entry.name ?? '' },
          data: [
            { name: 'no effect', xAxis: 0, lineStyle: { color: theme.muted, type: 'dashed', width: 1 } },
            { name: `mean ${formatStatistic('raw', view.mean).text}`, xAxis: view.mean, lineStyle: { color: theme.signal, type: 'solid', width: 1.5 } },
          ],
        },
      },
    ],
  }
}
