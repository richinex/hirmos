import type { EChartsCoreOption } from 'echarts/core'
import type { TimeEffectPoint } from '@/domain/estimation'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, legend, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface ImpactPathView {
  readonly outcome: string
  readonly points: readonly TimeEffectPoint[]
  readonly stepLabel: string
}

/** The post-intervention window: the outcome as a hairline, its counterfactual as a dashed line inside its band. */
export function impactPathOption(view: ImpactPathView, theme: ChartTheme): EChartsCoreOption {
  const first = view.points[0]?.step ?? 1
  const last = view.points.at(-1)?.step ?? first
  const description = `${view.outcome} against its counterfactual over ${formatCount(view.points.length).text} post-intervention ${view.stepLabel}s, ${view.stepLabel} ${first} to ${last}.`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 46 }),
    legend: legend(theme, ['counterfactual', 'observed']),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const firstEntry = entries[0]
        if (firstEntry === null || typeof firstEntry !== 'object') return ''
        const index = Number(Reflect.get(firstEntry, 'dataIndex'))
        const point = view.points[index]
        if (point === undefined) return ''
        return `${view.stepLabel} ${point.step}<br/>observed <strong>${formatStatistic('raw', point.actual).text}</strong><br/>counterfactual ${formatStatistic('raw', point.counterfactual).text} [${formatStatistic('raw', point.lower).text}, ${formatStatistic('raw', point.upper).text}]<br/>effect <strong>${formatStatistic('raw', point.effect).text}</strong>`
      },
    },
    xAxis: {
      type: 'value',
      min: first,
      max: Math.max(last, first + 1),
      minInterval: 1,
      name: view.stepLabel,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: axisLabelStyle(theme),
      splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme), scale: true },
    series: [
      {
        type: 'line',
        name: 'lower',
        data: view.points.map((point) => [point.step, point.lower]),
        lineStyle: { opacity: 0 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'band',
        data: view.points.map((point) => [point.step, point.upper - point.lower]),
        lineStyle: { opacity: 0 },
        areaStyle: { color: theme.bone, opacity: 0.3 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'counterfactual',
        data: view.points.map((point) => [point.step, point.counterfactual]),
        lineStyle: { color: theme.muted, width: 1.2, type: 'dashed' },
        symbol: 'none',
      },
      {
        type: 'line',
        name: 'observed',
        data: view.points.map((point) => [point.step, point.actual]),
        lineStyle: { color: theme.ink, width: 1.4 },
        symbol: 'circle',
        symbolSize: 3,
        itemStyle: { color: theme.ink },
      },
    ],
  }
}
