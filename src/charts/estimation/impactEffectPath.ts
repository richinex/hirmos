import type { EChartsCoreOption } from 'echarts/core'
import type { ImpactEffectPoint } from '@/domain/estimation'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, baseOption, gridAuto, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface ImpactEffectView {
  /** 'Pointwise effect' or 'Cumulative effect'. */
  readonly label: string
  readonly points: readonly ImpactEffectPoint[]
  readonly stepLabel: string
  readonly evaluatedFrom: number
  readonly evaluatedTo: number
}

/** The difference between the outcome and its estimated no-intervention path, across the fitted
 * window and the evaluated one. Before the intervention it should sit near zero; the departure
 * after it is the effect being claimed. */
export function impactEffectPathOption(view: ImpactEffectView, theme: ChartTheme): EChartsCoreOption {
  const first = view.points[0]?.step ?? view.evaluatedFrom
  const last = view.points.at(-1)?.step ?? view.evaluatedFrom
  const banded = view.points.some((point) => point.band.kind === 'interval')
  const bound = (point: ImpactEffectPoint, edge: 'lower' | 'upper') =>
    (point.band.kind === 'interval' ? point.band[edge] : null)
  const description = `${view.label} by ${view.stepLabel}, ${view.stepLabel} ${first} to ${last}, against a zero line. The intervention begins at ${view.stepLabel} ${view.evaluatedFrom}.${banded ? ' The shaded range is a 95% interval.' : ' No interval is drawn for this route.'}`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ top: 20, bottom: 45 }),
    tooltip: {
      ...tooltip(theme, 'axis'),
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const head = entries[0]
        if (head === null || typeof head !== 'object') return ''
        const point = view.points[Number(Reflect.get(head, 'dataIndex'))]
        if (point === undefined) return ''
        const range = point.band.kind === 'interval'
          ? ` [${formatStatistic('raw', point.band.lower).text}, ${formatStatistic('raw', point.band.upper).text}]`
          : ''
        const window = point.step < view.evaluatedFrom ? ' (fitted window)' : ''
        return `${view.stepLabel} ${point.step}${window}<br/>${view.label.toLowerCase()} <strong>${formatStatistic('raw', point.effect).text}</strong>${range}`
      },
    },
    xAxis: {
      type: 'value',
      min: first,
      max: Math.max(last, first + 1),
      minInterval: 1,
      name: `${view.stepLabel}, intervention from ${view.stepLabel} ${view.evaluatedFrom}`,
      nameLocation: 'middle',
      nameGap: 24,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      axisLabel: axisLabelStyle(theme),
      splitLine: { show: false },
    },
    yAxis: { ...valueAxis(theme, view.label), scale: true },
    series: [
      {
        type: 'line',
        name: 'lower',
        data: view.points.map((point) => [point.step, bound(point, 'lower')]),
        lineStyle: { opacity: 0 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: 'band',
        data: view.points.map((point) => {
          const lower = bound(point, 'lower')
          const upper = bound(point, 'upper')
          return [point.step, lower === null || upper === null ? null : upper - lower]
        }),
        lineStyle: { opacity: 0 },
        areaStyle: { color: theme.info, opacity: 0.12 },
        symbol: 'none',
        stack: 'band',
        stackStrategy: 'all',
        silent: true,
      },
      {
        type: 'line',
        name: view.label,
        data: view.points.map((point) => [point.step, point.effect]),
        lineStyle: { color: theme.signal, width: 1.8 },
        symbol: 'none',
        // Zero is the reference the whole panel is read against; the intervention is ruled off
        // so the fitted window reads as the run-up to it, as in the top panel.
        markLine: {
          silent: true,
          symbol: 'none',
          label: { show: false },
          data: [{ yAxis: 0, lineStyle: { color: theme.muted, type: 'solid', width: 1 } },
            { xAxis: view.evaluatedFrom, lineStyle: { color: theme.faint, type: 'dashed', width: 1 } }],
        },
        markArea: {
          silent: true,
          itemStyle: { color: theme.well, opacity: 0.6 },
          data: [[{ xAxis: view.evaluatedFrom }, { xAxis: view.evaluatedTo }]],
        },
      },
    ],
  }
}
