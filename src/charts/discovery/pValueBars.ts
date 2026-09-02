import type { EChartsCoreOption } from 'echarts/core'
import { formatP, formatStatistic } from '@/lib/format/number'
import { baseOption, categoryAxis, gridAuto, hatch, tooltip } from '../grammar'
import type { ChartTheme } from '../theme'

export interface PValueBarsView {
  readonly title: string
  readonly categories: readonly string[]
  readonly pValues: readonly number[]
  readonly statistics?: readonly number[]
  readonly statisticName?: string
  /** The threshold drawn as a reference line, in p-value units. */
  readonly alpha: number
}

/** p-values per category as bars on a log scale with the α reference line; bars below α are drawn in `signal`. */
export function pValueBarsOption(view: PValueBarsView, theme: ChartTheme): EChartsCoreOption {
  const floor = 1e-6
  const shown = view.pValues.map((p) => Math.max(p, floor))
  const below = view.pValues.filter((p) => p < view.alpha).length
  const description = `${view.title}: ${view.categories.length} tests; ${below} fall below α ${view.alpha}. Smallest p-value ${formatP(Math.min(...view.pValues)).text}.`
  return {
    ...baseOption(theme, description),
    grid: gridAuto({ bottom: 6 }),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        if (raw === null || typeof raw !== 'object') return ''
        const index = Reflect.get(raw, 'dataIndex')
        if (typeof index !== 'number') return ''
        const statistic = view.statistics?.[index]
        return `${view.categories[index]}<br/>${formatP(view.pValues[index]).text}${statistic === undefined ? '' : `<br/>${view.statisticName ?? 'statistic'} ${formatStatistic('score', statistic).text}`}`
      },
    },
    xAxis: categoryAxis(theme, view.categories),
    yAxis: {
      type: 'log',
      min: floor,
      max: 1,
      inverse: true,
      name: 'p-value',
      nameLocation: 'middle',
      nameGap: 42,
      nameTextStyle: { color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize },
      axisLine: { show: false },
      axisTick: { show: false },
      axisLabel: { color: theme.faint, fontFamily: theme.font, fontSize: theme.labelSize, hideOverlap: true, formatter: (value: number) => (value >= 0.001 ? String(value) : value.toExponential(0)) },
      splitLine: { lineStyle: { color: theme.hair } },
    },
    series: [{
      type: 'bar',
      name: 'p-value',
      barMaxWidth: 36,
      barMinWidth: 3,
      data: shown.map((p, index) => (view.pValues[index] < view.alpha
        ? { value: p, itemStyle: { color: theme.signal, decal: hatch(theme) } }
        : { value: p, itemStyle: { color: theme.bone } })),
      markLine: {
        silent: true,
        symbol: 'none',
        lineStyle: { color: theme.muted, type: 'dashed', width: 1 },
        label: { formatter: `α ${view.alpha}`, color: theme.muted, fontFamily: theme.font, fontSize: theme.labelSize, position: 'insideStartTop' },
        data: [{ yAxis: view.alpha }],
      },
    }],
  }
}
