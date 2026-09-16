import type { EChartsCoreOption } from 'echarts/core'
import type { GcmInfluenceRun } from '@/domain/gcmInfluence'
import { layoutDirectedGraph } from '@/components/dag/dagCanvasModel'
import { baseOption, escapeHtml, tooltip } from './grammar'
import { contributionOption } from './rootCause'
import type { ChartTheme } from './theme'
import { formatStatistic } from '@/lib/format/number'

/** The notebook divides absolute contributions by their absolute sum. Keep signed values separately. */
export function absoluteShares(values: readonly number[]): readonly number[] | null {
  const sum = values.reduce((total, value) => total + Math.abs(value), 0)
  return sum > 0 && Number.isFinite(sum) ? values.map(value => 100 * Math.abs(value) / sum) : null
}

export function influenceBars(run: GcmInfluenceRun, scale: 'signed' | 'absoluteShare', theme: ChartTheme): EChartsCoreOption {
  const { outcome } = run.evidence
  const shares = scale === 'absoluteShare' ? absoluteShares(outcome.values) : null
  const values = shares ?? outcome.values
  const measure = shares !== null ? 'Share of absolute contributions (%)' : outcome.kind === 'intrinsic' ? 'Contribution to target variance' : 'Incoming-arrow strength'
  return contributionOption(outcome.nodes.map((node, index) => ({ name: run.model.names[node]!, estimate: values[index]!, interval: null })), measure, theme)
}

export function influenceGraph(run: GcmInfluenceRun, theme: ChartTheme): EChartsCoreOption {
  const { names, edges, target } = run.model
  const { outcome } = run.evidence
  const strengths = new Map(outcome.kind === 'arrows' ? outcome.nodes.map((node, i) => [node, outcome.values[i]!] as const) : [])
  const max = Math.max(0, ...Array.from(strengths.values(), Math.abs))
  const positions = layoutDirectedGraph(names.map((_, i) => ({ id: String(i) })), edges.map(([cause, effect]) => ({ cause: String(cause), effect: String(effect) })))
  const number = (value: number) => formatStatistic('raw', value).text
  return {
    ...baseOption(theme, `The recorded causal graph. Target: ${names[target]}. Coloured incoming arrows have measured strengths; other relationships have not been measured in this run.`),
    tooltip: { ...tooltip(theme), formatter: (raw: unknown) => {
      if (raw === null || typeof raw !== 'object') return ''
      const data = Reflect.get(raw, 'data')
      if (data === null || typeof data !== 'object') return ''
      const name = escapeHtml(String(Reflect.get(data, 'name') ?? ''))
      const value = Reflect.get(data, 'value')
      return typeof value === 'number' ? `${name}<br/>Strength: ${number(value)}` : name
    } },
    series: [{ type: 'graph', layout: 'none', roam: true, symbolSize: 28,
      edgeSymbol: ['none', 'arrow'], edgeSymbolSize: [0, 9],
      label: { show: true, position: 'bottom', color: theme.ink, fontFamily: theme.font, fontSize: theme.labelSize },
      data: positions.map((position, i) => ({ ...position, name: names[i], itemStyle: { color: i === target ? theme.signal : theme.panel, borderColor: theme.ink, borderWidth: 1.5 } })),
      links: edges.map(([from, to]) => {
        const value = to === target ? strengths.get(from) : undefined
        return { source: String(from), target: String(to), name: `${names[from]} → ${names[to]}`, value,
          lineStyle: { color: value === undefined ? theme.muted : theme.signal, type: value === undefined ? 'dashed' : 'solid', opacity: value === undefined ? 0.45 : 1, width: value === undefined || max === 0 ? 1 : 1 + 5 * Math.abs(value) / max } }
      }),
    }],
  }
}
