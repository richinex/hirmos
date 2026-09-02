import type { EChartsCoreOption } from 'echarts/core'
import { baseOption, gridAuto, legend, stepAxis, tooltip, valueAxis, zoomPair } from '../grammar'
import type { ChartTheme } from '../theme'

export interface RegimeMembershipView {
  readonly memberships: readonly (readonly number[])[]
}

export function regimeMembershipOption(view: RegimeMembershipView, theme: ChartTheme): EChartsCoreOption {
  const observations = view.memberships[0]?.length ?? 0
  return {
    ...baseOption(theme, `RPCMCI regime membership across ${observations} observations and ${view.memberships.length} inferred regimes.`),
    // The legend sits above the plot: at the foot it prints over the axis name, and a regime run is
    // read against the observation index, so that name has to stay. The grid clears the zoom slider
    // below it as well, which occupies the bottom 24px of the container.
    grid: gridAuto({ top: 30, bottom: 64 }),
    legend: { ...legend(theme, view.memberships.map((_, regime) => `Regime ${regime + 1}`)), bottom: 'auto', top: 0 },
    tooltip: tooltip(theme, 'axis'),
    dataZoom: zoomPair(theme),
    xAxis: stepAxis(theme, 'Observation'),
    yAxis: { ...valueAxis(theme, 'Membership'), min: 0, max: 1 },
    series: view.memberships.map((memberships, regime) => ({
      type: 'line',
      name: `Regime ${regime + 1}`,
      step: 'end',
      sampling: 'minmax',
      showSymbol: false,
      symbol: 'none',
      lineStyle: { width: 1.5, color: theme.categorical[regime % theme.categorical.length] },
      itemStyle: { color: theme.categorical[regime % theme.categorical.length] },
      data: memberships.map((membership, observation) => [observation + 1, membership]),
    })),
  }
}

export interface AnnealingObjectiveView {
  readonly best: readonly number[]
}

export function annealingObjectiveOption(view: AnnealingObjectiveView, theme: ChartTheme): EChartsCoreOption {
  return {
    ...baseOption(theme, `Best RPCMCI annealing objective over ${view.best.length} completed iterations.`),
    grid: gridAuto(),
    tooltip: tooltip(theme, 'axis'),
    xAxis: stepAxis(theme, 'Iteration'),
    yAxis: valueAxis(theme, 'Objective'),
    series: [{
      type: 'line',
      name: 'Best objective',
      showSymbol: view.best.length < 20,
      symbolSize: 5,
      lineStyle: { width: 1.5, color: theme.signal },
      itemStyle: { color: theme.signal },
      data: view.best.map((objective, iteration) => [iteration + 1, objective]),
    }],
  }
}
