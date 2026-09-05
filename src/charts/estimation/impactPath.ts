import type { EChartsCoreOption } from 'echarts/core'
import type { TimeEffectPoint } from '@/domain/estimation'
import { formatCount, formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, legend, rangeSelection, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface ImpactPathView {
  readonly outcome: string
  readonly points: readonly TimeEffectPoint[]
  readonly stepLabel: string
  /** Another run of the same question, drawn as a faint ghost so its counterfactual can be read against this one. */
  readonly ghost?: { readonly name: string; readonly points: readonly TimeEffectPoint[] }
}

/** The post-intervention window: the outcome as a hairline, its counterfactual as a dashed line inside its band. */
export function impactPathOption(view: ImpactPathView, theme: ChartTheme): EChartsCoreOption {
  const first = view.points[0]?.step ?? 1
  const last = view.points.at(-1)?.step ?? first
  const ghost = view.ghost ?? null
  const ghostAt = new Map(ghost?.points.map((point) => [point.step, point]) ?? [])
  const description = `${view.outcome} against its counterfactual over ${formatCount(view.points.length).text} post-intervention ${view.stepLabel}s, ${view.stepLabel} ${first} to ${last}.${ghost === null ? '' : ` The counterfactual of ${ghost.name} is drawn as a ghost for comparison.`}`
  return {
    ...baseOption(theme, description),
    // The slider sits under the axis name; the legend goes above the plot.
    grid: gridAuto({ top: 30, bottom: 64 }),
    legend: { ...legend(theme, ghost === null ? ['counterfactual', 'observed'] : ['counterfactual', 'observed', ghost.name]), bottom: 'auto', top: 0 },
    ...rangeSelection(theme),
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
        const other = ghostAt.get(point.step)
        const comparison = ghost === null || other === undefined ? '' : `<br/>${ghost.name}: counterfactual ${formatStatistic('raw', other.counterfactual).text}, effect ${formatStatistic('raw', other.effect).text} (${formatStatistic('raw', point.effect - other.effect).text} apart)`
        return `${view.stepLabel} ${point.step}<br/>observed <strong>${formatStatistic('raw', point.actual).text}</strong><br/>counterfactual ${formatStatistic('raw', point.counterfactual).text} [${formatStatistic('raw', point.lower).text}, ${formatStatistic('raw', point.upper).text}]<br/>effect <strong>${formatStatistic('raw', point.effect).text}</strong>${comparison}`
      },
    },
    xAxis: {
      type: 'value',
      min: first,
      max: Math.max(last, first + 1),
      minInterval: 1,
      // The window opens at the intervention, so the axis says so rather than a rule on its own left edge.
      name: `${view.stepLabel} · intervention from ${view.stepLabel} ${first}`,
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
      // The ghost is the other run's counterfactual: faint and dashed, behind everything this run draws.
      ...(ghost === null ? [] : [{
        type: 'line',
        name: ghost.name,
        data: ghost.points.map((point) => [point.step, point.counterfactual]),
        lineStyle: { color: theme.faint, width: 1, type: 'dotted' },
        itemStyle: { color: theme.faint },
        symbol: 'none',
        z: 1,
      }]),
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
