import { useMemo } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { baseOption, categoryAxis, gridAuto, tooltip, valueAxis } from '@/charts/grammar'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import type { AdjustedDidEvidence } from '@/domain/adjustedDid'

export function AdjustedDidConvergenceWarning({
  evidence,
}: {
  readonly evidence: AdjustedDidEvidence
}) {
  return evidence.inference.kind === 'crossFitted' &&
    evidence.inference.optimizerStatus.some(
      (status) => status === 'iteration-limit' || status === 'line-search-failed',
    ) ? (
    <Alert tone="warn" live={false}>
      At least one propensity fit did not converge. Review the covariates and treatment overlap
      before interpreting this result.
    </Alert>
  ) : null
}

export function AdjustedDidResult({
  evidence,
  labels,
  covariates,
}: {
  readonly evidence: AdjustedDidEvidence
  readonly labels: readonly string[]
  readonly covariates: readonly string[]
}) {
  const theme = useChartTheme()
  const option = useMemo(
    () => ({
      ...baseOption(
        theme,
        'Observed comparison and treated group means before and after adoption. These are unadjusted descriptive means, not fitted counterfactuals.',
      ),
      grid: gridAuto({ top: 20, bottom: 25 }),
      tooltip: tooltip(theme, 'axis'),
      xAxis: categoryAxis(theme, labels),
      yAxis: valueAxis(theme, 'Observed group mean'),
      series: ['Comparison', 'Treated'].map((name, index) => ({
        id: name,
        name,
        type: 'line',
        data: evidence.groupMeans[index],
        symbol: 'circle',
        symbolSize: 6,
        lineStyle: { color: index === 0 ? theme.ink : theme.signal },
        itemStyle: { color: index === 0 ? theme.ink : theme.signal },
      })),
    }),
    [evidence, labels, theme],
  )
  const rows = ['Comparison', 'Treated'].map((name, index) => ({
    name,
    before: evidence.groupMeans[index]![0],
    after: evidence.groupMeans[index]![1],
  }))
  type Row = (typeof rows)[number]
  const inference = evidence.inference
  const coefficients =
    inference.kind === 'independentErrors'
      ? [
          'Intercept',
          'Treated group',
          'Post period',
          'Treated group × post period',
          ...covariates,
        ].map((name, index) => ({
          name,
          value: inference.coefficients[index]!,
          se: inference.standardErrors[index]!,
          lower: inference.intervals[index]![0],
          upper: inference.intervals[index]![1],
        }))
      : []
  type Coefficient = (typeof coefficients)[number]
  return (
    <div className="mt-3 grid gap-3">
      <ExpandableChart
        option={option}
        label="Observed DiD group means"
        className="h-[250px]"
        testId="did-group-means"
      />
      <EvidenceTable
        title="Observed group means"
        help="Unadjusted means. Their difference-in-differences need not equal a covariate-adjusted or doubly robust estimate."
        rows={rows}
        rowKey={(row) => row.name}
        noun="group"
        empty="No group means."
        columns={[
          { id: 'group', header: 'Group', value: (row) => row.name },
          figureColumn<Row>('before', 'Before', (row) => row.before),
          figureColumn<Row>('after', 'After', (row) => row.after),
          figureColumn<Row>('change', 'Change', (row) => row.after - row.before),
        ]}
      />
      {evidence.inference.kind === 'independentErrors' ? (
        <>
          <EvidenceTable
            title="Regression coefficients"
            help="The treated-group by post-period interaction is the DiD estimate. The other coefficients describe the fitted regression, not separate causal effects."
            rows={coefficients}
            rowKey={(row) => row.name}
            noun="coefficient"
            empty="No coefficients."
            columns={[
              { id: 'term', header: 'Term', value: (row) => row.name },
              figureColumn<Coefficient>('estimate', 'Estimate', (row) => row.value),
              figureColumn<Coefficient>('se', 'Standard error', (row) => row.se),
              figureColumn<Coefficient>('lower', 'Lower 95%', (row) => row.lower),
              figureColumn<Coefficient>('upper', 'Upper 95%', (row) => row.upper),
            ]}
          />
          <Alert tone="info" live={false}>
            Classical Student-t inference assumes independent, homoskedastic errors. The interval is
            not clustered by unit.
          </Alert>
        </>
      ) : (
        <>
          {evidence.inference.propensityFit !== undefined && (
            <p className="m-0 text-body text-muted">
              Baseline covariates are centred and scaled within each training fold for the
              propensity regression. The same transformation is applied to that fold’s held-out
              units. Every propensity fit must converge before an estimate is reported.
            </p>
          )}
          <EvidenceTable
            title="Propensity optimizer"
            rows={evidence.inference.optimizerStatus.map((status, index) => ({
              fold: index + 1,
              status,
            }))}
            rowKey={(row) => String(row.fold)}
            noun="fold"
            empty="No fold diagnostics."
            columns={[
              { id: 'fold', header: 'Fold', value: (row) => String(row.fold) },
              {
                id: 'status',
                header: 'Termination',
                value: (row) =>
                  ({
                    'projected-gradient': 'Projected-gradient tolerance',
                    'function-tolerance': 'Function tolerance',
                    'iteration-limit': 'Iteration limit reached',
                    'line-search-failed': 'Line search failed',
                  })[row.status],
              },
            ]}
          />
        </>
      )}
    </div>
  )
}
