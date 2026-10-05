import type { EChartsCoreOption } from 'echarts/core'
import type { TimeEffectPoint } from '@/domain/estimation'
import { formatCount, formatStatistic } from '@/lib/format/number'
import {
  axisLabelStyle,
  baseOption,
  gridAuto,
  legend,
  rangeSelection,
  tooltip,
  valueAxis,
} from '../grammar'
import type { ChartTheme } from '../theme'

export interface ImpactPathView {
  readonly outcome: string
  /** The fitted window, where the two paths should agree. Empty when the run did not report it. */
  readonly before: readonly TimeEffectPoint[]
  readonly points: readonly TimeEffectPoint[]
  readonly stepLabel: string
  /** Another run of the same question, drawn faintly so its estimated no-intervention path can be compared. */
  readonly ghost?: { readonly name: string; readonly points: readonly TimeEffectPoint[] }
}

/** The outcome as a hairline against its estimated no-intervention path, dashed inside its band,
 * across the fitted window and the evaluated one. The evaluated window is shaded, so the rows the
 * fit was judged on stay visible beside the rows the effect is read from. */
export function impactPathOption(view: ImpactPathView, theme: ChartTheme): EChartsCoreOption {
  const drawn = [...view.before, ...view.points]
  const evaluatedFrom = view.points[0]?.step ?? 1
  const first = drawn[0]?.step ?? evaluatedFrom
  const last = drawn.at(-1)?.step ?? evaluatedFrom
  const ghost = view.ghost ?? null
  const ghostAt = new Map(ghost?.points.map((point) => [point.step, point]) ?? [])
  const fitted =
    view.before.length === 0
      ? ''
      : ` The ${formatCount(view.before.length).text} ${view.stepLabel}s before it are the fitted window, where the two paths should agree.`
  const description = `${view.outcome} against its estimated no-intervention path over ${formatCount(view.points.length).text} post-intervention ${view.stepLabel}s, ${view.stepLabel} ${evaluatedFrom} to ${last}.${fitted}${ghost === null ? '' : ` The estimated no-intervention path from ${ghost.name} is drawn faintly for comparison.`}`
  return {
    ...baseOption(theme, description),
    // The slider sits under the axis name; the legend goes above the plot.
    grid: gridAuto({ top: 30, bottom: 64 }),
    legend: {
      ...legend(
        theme,
        ghost === null
          ? ['estimated no intervention', 'observed']
          : ['estimated no intervention', 'observed', ghost.name],
      ),
      bottom: 'auto',
      top: 0,
    },
    ...rangeSelection(theme),
    tooltip: {
      ...tooltip(theme, 'axis'),
      axisPointer: { type: 'line', lineStyle: { color: theme.muted, type: 'dashed' } },
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const firstEntry = entries[0]
        if (firstEntry === null || typeof firstEntry !== 'object') return ''
        const index = Number(Reflect.get(firstEntry, 'dataIndex'))
        const point = drawn[index]
        if (point === undefined) return ''
        const other = ghostAt.get(point.step)
        const comparison =
          ghost === null || other === undefined
            ? ''
            : `<br/>${ghost.name}: no-intervention estimate ${formatStatistic('raw', other.counterfactual).text}, difference ${formatStatistic('raw', other.effect).text} (${formatStatistic('raw', point.effect - other.effect).text} apart)`
        const window = point.step < evaluatedFrom ? ' (fitted window)' : ''
        return `${view.stepLabel} ${point.step}${window}<br/>observed <strong>${formatStatistic('raw', point.actual).text}</strong><br/>estimated no intervention ${formatStatistic('raw', point.counterfactual).text} [${formatStatistic('raw', point.lower).text}, ${formatStatistic('raw', point.upper).text}]<br/>difference <strong>${formatStatistic('raw', point.effect).text}</strong>${comparison}`
      },
    },
    xAxis: {
      type: 'value',
      min: first,
      max: Math.max(last, first + 1),
      minInterval: 1,
      name: `${view.stepLabel}, intervention from ${view.stepLabel} ${evaluatedFrom}`,
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
      // The ghost is the other run's no-intervention path: faint and dashed behind this run.
      ...(ghost === null
        ? []
        : [
            {
              type: 'line',
              name: ghost.name,
              data: ghost.points.map((point) => [point.step, point.counterfactual]),
              lineStyle: { color: theme.faint, width: 1, type: 'dotted' },
              itemStyle: { color: theme.faint },
              symbol: 'none',
              z: 1,
            },
          ]),
      {
        type: 'line',
        name: 'lower',
        data: drawn.map((point) => [point.step, point.lower]),
        lineStyle: { opacity: 0 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'band',
        data: drawn.map((point) => [point.step, point.upper - point.lower]),
        lineStyle: { opacity: 0 },
        areaStyle: { color: theme.info, opacity: 0.12 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'estimated no intervention',
        data: drawn.map((point) => [point.step, point.counterfactual]),
        itemStyle: { color: theme.info },
        lineStyle: { color: theme.info, width: 1.4, type: 'dashed' },
        symbol: 'none',
        // The evaluated window is shaded, as the interrupted-series chart shades its post period,
        // and the intervention itself is ruled off so the fitted window reads as the run-up to it.
        markArea:
          view.points.length === 0 || view.before.length === 0
            ? undefined
            : {
                silent: true,
                itemStyle: { color: theme.well, opacity: 0.6 },
                data: [
                  [{ xAxis: evaluatedFrom }, { xAxis: view.points.at(-1)?.step ?? evaluatedFrom }],
                ],
              },
        markLine:
          view.before.length === 0
            ? undefined
            : {
                silent: true,
                symbol: 'none',
                label: { show: false },
                lineStyle: { color: theme.faint, type: 'dashed', width: 1 },
                data: [{ xAxis: evaluatedFrom }],
              },
      },
      {
        type: 'line',
        name: 'observed',
        data: drawn.map((point) => [point.step, point.actual]),
        lineStyle: { color: theme.ink, width: 1.4 },
        symbol: 'circle',
        symbolSize: 3,
        itemStyle: { color: theme.ink },
      },
    ],
  }
}
