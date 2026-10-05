import { useMemo } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption, gridAuto, tooltip, valueAxis } from '@/charts/grammar'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import type { SharpRdEvidence } from '@/domain/sharpRd'
import { formatStatistic } from '@/lib/format/number'

export function SharpRdResult({
  evidence,
  running,
  outcome,
}: {
  readonly evidence: SharpRdEvidence
  readonly running: string
  readonly outcome: string
}) {
  const theme = useChartTheme()
  const option = useMemo(() => {
    const { cutoff, bandwidth, leftCoefficients: left, rightCoefficients: right } = evidence
    const support = evidence.points.reduce(
      ([lo, hi], [x]) => [Math.min(lo, x), Math.max(hi, x)] as const,
      [Infinity, -Infinity] as readonly [number, number],
    )
    const line = (coefficients: readonly number[], start: number, end: number) =>
      [start, end].map((x) => [x, coefficients[0]! + coefficients[1]! * (x - cutoff)])
    return {
      ...baseOption(
        theme,
        `${outcome} against ${running}; separate local-linear fits within the estimation bandwidth around ${cutoff}.`,
      ),
      grid: gridAuto({ top: 20, bottom: 55 }),
      tooltip: tooltip(theme, 'item'),
      xAxis: { ...valueAxis(theme, running), scale: true },
      yAxis: { ...valueAxis(theme, outcome), scale: true },
      series: [
        {
          id: 'observations',
          name: 'Observed',
          type: 'scatter',
          data: evidence.points,
          symbolSize: 4,
          itemStyle: { color: theme.muted, opacity: 0.4 },
          markLine: {
            silent: true,
            symbol: 'none',
            data: [{ xAxis: cutoff, name: 'Cutoff' }],
            label: {
              formatter: `Cutoff ${formatStatistic('raw', cutoff).text}`,
              color: theme.muted,
            },
            lineStyle: { color: theme.muted, type: 'dashed' },
          },
        },
        {
          id: 'left-fit',
          name: 'Local fit below cutoff',
          type: 'line',
          data: line(left, Math.max(cutoff - bandwidth, support[0]), cutoff),
          symbol: 'none',
          lineStyle: { color: theme.ink, width: 2 },
        },
        {
          id: 'right-fit',
          name: 'Local fit at or above cutoff',
          type: 'line',
          data: line(right, cutoff, Math.min(cutoff + bandwidth, support[1])),
          symbol: 'none',
          lineStyle: { color: theme.signal, width: 2 },
        },
      ],
    }
  }, [evidence, running, outcome, theme])
  const rows = [
    { name: 'Conventional', ...evidence.conventional },
    { name: 'Bias-corrected', ...evidence.biasCorrected },
    { name: 'Robust bias-corrected', ...evidence.robust },
  ]
  type Row = (typeof rows)[number]
  return (
    <div className="mt-3 grid gap-3">
      <ExpandableChart
        option={option}
        label={`Sharp RD: ${outcome} at the ${running} cutoff`}
        className="h-[280px]"
        testId="sharp-rd-plot"
      />
      <EvidenceTable
        title="RD estimates"
        help="The headline uses the robust bias-corrected estimate and interval. The plotted lines are the conventional local-linear fits, not bias-corrected curves. All intervals are 95%."
        rows={rows}
        rowKey={(row) => row.name}
        noun="estimate"
        empty="No RD estimates."
        columns={[
          { id: 'method', header: 'Inference', value: (row) => row.name },
          figureColumn<Row>('estimate', 'Estimate', (row) => row.value),
          figureColumn<Row>('se', 'Standard error', (row) => row.standardError),
          figureColumn<Row>('lower', 'Lower', (row) => row.interval[0]),
          figureColumn<Row>('upper', 'Upper', (row) => row.interval[1]),
        ]}
      />
    </div>
  )
}
