import { useMemo } from 'react'
import type { CovariateBalance } from '@/domain/covariateBalance'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption, categoryAxis, valueAxis, legend } from '@/charts/grammar'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'

type Available = Extract<CovariateBalance, { kind: 'available' }>
function BalancePlot({
  evidence,
  augmented,
}: {
  readonly evidence: Available
  readonly augmented: boolean
}) {
  const theme = useChartTheme()
  const rows = useMemo(
    () => evidence.rows.map((row) => ({ ...row, name: evidence.labels[row.column]! })),
    [evidence],
  )
  // Worst imbalance before adjustment first, as cobalt's love.plot orders by the unadjusted value.
  const plotted = useMemo(
    () => [...rows].sort((a, b) => Math.abs(b.before) - Math.abs(a.before)),
    [rows],
  )
  const option = useMemo(
    () => ({
      ...baseOption(theme, 'Absolute standardised mean differences before and after adjustment'),
      legend: { ...legend(theme, ['Before', 'After']), top: 0, bottom: undefined },
      grid: { left: 24, right: 28, top: 42, bottom: 48, containLabel: true },
      xAxis: {
        ...valueAxis(theme, 'Absolute standardised mean difference'),
        min: 0,
        nameLocation: 'middle',
        nameGap: 30,
      },
      yAxis: {
        ...categoryAxis(
          theme,
          plotted.map((row) => row.name),
        ),
        inverse: true,
      },
      series: [
        {
          name: 'Change',
          type: 'line',
          silent: true,
          symbol: 'none',
          z: 1,
          lineStyle: { color: theme.hair, width: 2 },
          // Each covariate's before and after joined by a segment; '-' breaks the line between covariates.
          data: plotted.flatMap((row) => [
            [Math.abs(row.before), row.name],
            [Math.abs(row.after), row.name],
            '-',
          ]),
        },
        {
          name: 'Before',
          type: 'scatter',
          symbol: 'circle',
          symbolSize: 9,
          z: 2,
          itemStyle: { color: theme.info },
          data: plotted.map((row) => [Math.abs(row.before), row.name]),
          markLine: {
            silent: true,
            symbol: 'none',
            lineStyle: { color: theme.muted, type: 'dashed' },
            label: {
              formatter: '0.1 reference',
              position: 'start',
              color: theme.muted,
              fontFamily: theme.font,
              fontSize: theme.labelSize,
            },
            data: [{ xAxis: 0.1 }],
          },
        },
        {
          name: 'After',
          type: 'scatter',
          symbol: 'diamond',
          symbolSize: 10,
          z: 2,
          itemStyle: { color: theme.signal },
          data: plotted.map((row) => [Math.abs(row.after), row.name]),
        },
      ],
    }),
    [theme, plotted],
  )
  return (
    <section className="mt-4 space-y-3" aria-label="Covariate balance">
      <ExpandableChart
        option={option}
        label="Covariate balance before and after adjustment"
        testId="covariate-balance"
        style={{ height: Math.max(240, rows.length * 28 + 100) }}
      />
      <EvidenceTable
        title="Covariate balance"
        help={`The plot shows absolute standardised differences; the table retains their signs. The 0.1 line is a reference, not a test of causal identification. Before and after values use the original ${evidence.reference === 'treated' ? 'treated-group' : 'pooled'} standard deviation, except where the table records full-sample spread.${augmented ? ' After adjustment refers to propensity weighting, not the outcome regression.' : ''}`}
        rows={rows}
        rowKey={(row) => String(row.column)}
        noun="covariate"
        empty="No adjustment covariates were recorded."
        columns={[
          { id: 'name', header: 'Covariate', value: (row) => row.name },
          figureColumn<(typeof rows)[number]>('before', 'Before', (row) => row.before),
          figureColumn<(typeof rows)[number]>('after', 'After', (row) => row.after),
          {
            id: 'spread',
            header: 'Reference spread',
            value: (row) =>
              row.usedFullSampleSpread
                ? 'Full sample (degenerate reference arm)'
                : evidence.reference === 'treated'
                  ? 'Treated group'
                  : 'Pooled',
          },
        ]}
      />
    </section>
  )
}
export function CovariateBalanceResult({
  evidence,
  augmented,
}: {
  readonly evidence: CovariateBalance
  readonly augmented: boolean
}) {
  switch (evidence.kind) {
    case 'notRecorded':
      return (
        <Alert tone="info">
          Covariate balance was not recorded for this run. Run the estimator again to calculate it.
        </Alert>
      )
    case 'unavailable':
      return <Alert tone="warn">{evidence.reason}</Alert>
    case 'available':
      return evidence.labels.length !== evidence.rows.length ? (
        <Alert tone="warn">
          The recorded balance values do not have matching covariate labels.
        </Alert>
      ) : (
        <BalancePlot evidence={evidence} augmented={augmented} />
      )
  }
}
