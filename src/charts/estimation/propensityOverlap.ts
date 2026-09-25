import type { EChartsCoreOption } from 'echarts/core'
import { formatStatistic } from '@/lib/format/number'
import { axisLabelStyle, axisNameStyle, baseOption, escapeHtml, legend, rangeSelection, tooltip, valueAxis } from '../grammar'
import type { ChartTheme } from '../theme'

export interface PropensityDistributionView {
  readonly treatment: string
  /** P(treated | covariates) in row order. */
  readonly propensity: readonly number[]
  /** Which arm each row is in, paired with the scores. */
  readonly treated: readonly boolean[]
  /** Row weights, when the estimator computed them; the second panel is drawn only with these. */
  readonly weights?: readonly number[]
}

const BINS = 30

interface Arm { readonly label: string; readonly rows: readonly number[] }

/** Shares within the arm, so a small arm stays readable beside a large one. */
const shares = (
  arm: Arm,
  propensity: readonly number[],
  weights: readonly number[] | null,
  low: number,
  width: number,
): readonly number[] => {
  const bins = new Array<number>(BINS).fill(0)
  let total = 0
  for (const row of arm.rows) {
    const bin = Math.min(BINS - 1, Math.max(0, Math.floor((propensity[row]! - low) / width)))
    const weight = weights === null ? 1 : weights[row]!
    bins[bin]! += weight
    total += weight
  }
  return total === 0 ? bins : bins.map((value) => value / total)
}

/**
 * The fitted score by arm, and the same after weighting when weights were computed. Overlap is
 * read from the pair: weighting that works brings the two arms together.
 */
export function propensityDistributionOption(
  view: PropensityDistributionView,
  theme: ChartTheme,
): EChartsCoreOption {
  const control: number[] = []
  const treated: number[] = []
  for (const [row, isTreated] of view.treated.entries()) (isTreated ? treated : control).push(row)
  const arms: readonly Arm[] = [
    { label: 'Not treated', rows: control },
    { label: 'Treated', rows: treated },
  ]

  let low = Infinity
  let high = -Infinity
  for (const score of view.propensity) { low = Math.min(low, score); high = Math.max(high, score) }
  if (low === high) { low -= 0.5; high += 0.5 }
  const width = (high - low) / BINS
  const centres = Array.from({ length: BINS }, (_, bin) => low + (bin + 0.5) * width)

  const weighted = view.weights ?? null
  const panels = weighted === null
    ? [{ title: 'Propensity distribution', weights: null as readonly number[] | null }]
    : [
      { title: 'Propensity distribution', weights: null as readonly number[] | null },
      { title: 'Weighted propensity distribution', weights: weighted },
    ]
  const bars = panels.map((panel) => arms.map((arm) => shares(arm, view.propensity, panel.weights, low, width)))
  const tallest = Math.max(...bars.flat(2))
  const colour = [theme.info, theme.signal]
  const edge = view.propensity.filter((score) => score < 0.05 || score > 0.95).length
  const description = `Fitted propensity score by arm: ${treated.length} treated rows, ${control.length} not treated`
    + `${edge === 0 ? '' : `, ${edge} outside 0.05 to 0.95 where the arms have little in common`}.`
    + (weighted === null ? '' : ' The lower panel repeats it with each row at its weight; weighting that works brings the arms together.')

  const grids = panels.length === 1
    ? [{ left: 66, right: 18, top: 34, bottom: 62 }]
    : [{ left: 66, right: 18, top: 34, height: '34%' }, { left: 66, right: 18, top: '58%', bottom: 62 }]
  const selection = rangeSelection(theme, panels.map((_, index) => index))
  return {
    ...baseOption(theme, description),
    animation: false,
    legend: { ...legend(theme, arms.map((arm) => arm.label)), top: 0, bottom: undefined },
    grid: grids,
    ...selection,
    dataZoom: selection.dataZoom.map((entry) => entry.type === 'slider' ? { ...entry, bottom: 8 } : entry),
    xAxis: panels.map((panel, index) => ({
      type: 'value' as const,
      gridIndex: index,
      min: low,
      max: high,
      name: panel.title,
      nameLocation: 'middle' as const,
      nameGap: 26,
      nameTextStyle: axisNameStyle(theme),
      axisLine: { lineStyle: { color: theme.hair } },
      axisTick: { show: false },
      splitLine: { show: false },
      axisLabel: { ...axisLabelStyle(theme), formatter: (value: number) => formatStatistic('raw', value).text },
    })),
    yAxis: panels.map((_, index) => ({
      ...valueAxis(theme, 'share of the arm'),
      gridIndex: index,
      min: 0,
      max: tallest,
      nameGap: 48,
      splitLine: { lineStyle: { color: theme.hair, type: 'dotted' as const } },
      axisLabel: { ...axisLabelStyle(theme), formatter: (value: number) => formatStatistic('raw', value).text },
    })),
    tooltip: {
      ...tooltip(theme),
      formatter: (raw: unknown) => {
        const entries = Array.isArray(raw) ? raw : [raw]
        const first = entries.find((entry) => entry !== null && typeof entry === 'object')
        const centre = first === undefined ? null : Reflect.get(first, 'value')
        if (!Array.isArray(centre)) return ''
        const lines = entries.map((entry) => {
          const value: unknown = Reflect.get(entry, 'value')
          const name = String(Reflect.get(entry, 'seriesName') ?? '')
          return Array.isArray(value)
            ? `${escapeHtml(name)} <strong>${formatStatistic('raw', Number(value[1]) * 100).text}%</strong>`
            : ''
        })
        return `score ${formatStatistic('raw', Number(centre[0])).text}<br/>${lines.filter(Boolean).join('<br/>')}`
      },
    },
    series: panels.flatMap((_, index) => arms.map((arm, armIndex) => ({
      type: 'bar' as const,
      name: arm.label,
      xAxisIndex: index,
      yAxisIndex: index,
      barCategoryGap: 0,
      barGap: '-100%',
      itemStyle: { color: colour[armIndex], opacity: 0.55 },
      data: bars[index]![armIndex]!.map((share, bin) => [centres[bin], share]),
    }))),
  }
}
