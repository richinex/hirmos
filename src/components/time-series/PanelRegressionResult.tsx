import { useMemo } from 'react'
import type { TimeSeriesRun } from '@/domain/timeSeries'
import { assertNever } from '@/domain/dop'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { baseOption, gridAuto, tooltip, valueAxis } from '@/charts/grammar'
import { useChartTheme } from '@/charts/theme'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { formatStatistic, formatWords, formatP } from '@/lib/format/number'
import { resultSurface, resultTitle } from '@/components/ui/recipes'
import type { BaconEvidence } from '@/domain/panelRegression'
import { periodLabel } from '@/domain/periodLabels'

const number = (v: number | string) => (typeof v === 'number' ? formatStatistic('raw', v).text : v)
const probability = (v: number | string) =>
  typeof v === 'number' ? formatP(v, { withLabel: false }).text : v
function comparison(
  c: Extract<
    BaconEvidence['decomposition'],
    { kind: 'unadjusted' }
  >['components'][number]['comparison'],
  labels: ReadonlyMap<number, string>,
): string {
  const label = (code: number) => periodLabel(code, labels)
  switch (c.kind) {
    case 'treatedVsNever':
      return `Adoption ${label(c.adoption)}: treated vs never treated`
    case 'earlierVsLater':
      return `${label(c.earlier)} vs ${label(c.later)}: earlier vs later treated`
    case 'laterVsEarlier':
      return `${label(c.later)} vs ${label(c.earlier)}: later vs earlier treated`
    case 'laterVsAlways':
      return `Adoption ${label(c.adoption)}: later vs always treated`
    case 'bothTreated':
      return `Adoption ${label(c.earlier)} and ${label(c.later)}: both treated`
    default:
      return assertNever(c)
  }
}
export function BaconResult({ run }: { readonly run: Extract<TimeSeriesRun, { kind: 'bacon' }> }) {
  const e = run.evidence,
    theme = useChartTheme()
  const labels = useMemo(
    () => new Map(run.periods.map((label, code) => [code, label])),
    [run.periods],
  )
  const rows = useMemo(
    () =>
      e.decomposition.kind === 'unadjusted'
        ? e.decomposition.components.map((c, index) => ({
            index,
            label: comparison(c.comparison, labels),
            estimate: c.estimate,
            weight: c.weight,
          }))
        : [
            {
              index: 0,
              label: 'Within timing groups',
              estimate: e.decomposition.withinEstimate,
              weight: e.decomposition.withinWeight,
            },
            ...e.decomposition.between.map((c, i) => ({
              index: i + 1,
              label: comparison(c.comparison, labels),
              estimate: c.estimate,
              weight: c.weight,
            })),
          ],
    [e, labels],
  )
  const option = useMemo(
    () => ({
      ...baseOption(theme, 'Goodman–Bacon comparison estimates and weights.'),
      grid: gridAuto({ top: 24, bottom: 30 }),
      tooltip: tooltip(theme, 'item'),
      xAxis: valueAxis(theme, 'Comparison weight'),
      yAxis: valueAxis(theme, 'Comparison estimate'),
      series: [
        {
          id: 'comparisons',
          name: 'Comparison',
          type: 'scatter',
          symbolSize: 8,
          data: rows.map((r) => ({ name: r.label, value: [r.weight, r.estimate] })),
          itemStyle: { color: theme.signal },
          markLine: {
            silent: true,
            symbol: 'none',
            label: { formatter: () => `TWFE ${number(e.twfe)}`, position: 'insideEndTop' },
            data: [{ yAxis: e.twfe, name: 'TWFE coefficient' }],
          },
        },
      ],
    }),
    [e, rows, theme],
  )
  return (
    <section className={resultSurface()} aria-label="Bacon decomposition result">
      <h3 className={resultTitle}>Goodman–Bacon decomposition for {run.outcome.name}</h3>
      <MetricGrid label="Decomposition summary">
        <MetricTile label="TWFE coefficient" value={formatWords(number(e.twfe))} />
        <MetricTile
          label="Reconstructed coefficient"
          value={formatWords(number(e.reconstructed))}
        />
      </MetricGrid>
      <p className="text-body text-muted">
        This decomposes the two-way fixed-effects coefficient. It is not a separate estimate of the
        effect among treated units. Comparisons that use already-treated units as controls can
        complicate interpretation when effects vary over time or across cohorts.
      </p>
      <p className="text-label text-faint">
        No confidence interval is calculated for this decomposition.
      </p>
      <ExpandableChart label="Bacon comparison weights" option={option} className="h-[300px]" />
      <EvidenceTable
        title="Weighted comparisons"
        rows={rows}
        rowKey={(r) => String(r.index)}
        noun="comparison"
        empty="No comparisons"
        columns={[
          { id: 'label', header: 'Comparison', value: (r) => r.label },
          {
            id: 'estimate',
            header: 'Estimate',
            value: (r) => r.estimate,
            format: number,
            align: 'right',
          },
          {
            id: 'weight',
            header: 'Weight',
            value: (r) => r.weight,
            format: number,
            align: 'right',
          },
        ]}
      />
    </section>
  )
}
export function PanelRegressionResult({
  run,
}: {
  readonly run: Extract<TimeSeriesRun, { kind: 'panel-regression' }>
}) {
  const e = run.evidence,
    s = e.request.specification,
    theme = useChartTheme()
  const points = useMemo(
    () =>
      e.events.flatMap((event) => {
        const term = e.terms.find((t) => t.index === event.index)
        return term === undefined ? [] : [{ ...term, ...event }]
      }),
    [e],
  )
  const option = useMemo(
    () => ({
      ...baseOption(
        theme,
        'Pooled regression event coefficients and pointwise Student-t intervals.',
      ),
      grid: gridAuto({ top: 24, bottom: 30 }),
      tooltip: tooltip(theme, 'item'),
      xAxis: { ...valueAxis(theme, 'Periods since adoption'), minInterval: 1 },
      yAxis: valueAxis(theme, 'Regression coefficient'),
      series: [
        {
          id: 'intervals',
          name: 'Pointwise interval',
          type: 'line',
          symbol: 'none',
          connectNulls: false,
          data: points.flatMap((p) => [[p.period, p.lower], [p.period, p.upper], null]),
          lineStyle: { color: theme.signal, width: 1.5 },
        },
        {
          id: 'estimates',
          name: 'Coefficient',
          type: 'scatter',
          symbolSize: 7,
          data: points.map((p) => [p.period, p.estimate]),
          itemStyle: { color: theme.signal },
          markLine: { silent: true, symbol: 'none', label: { show: false }, data: [{ yAxis: 0 }] },
        },
        ...(s.kind === 'eventStudy'
          ? [
              {
                id: 'reference',
                name: 'Omitted reference',
                type: 'scatter',
                symbol: 'emptyCircle',
                data: [[s.window.reference, 0]],
                itemStyle: { color: theme.muted },
              },
            ]
          : []),
      ],
    }),
    [points, s, theme],
  )
  return (
    <section className={resultSurface()} aria-label="Panel regression result">
      <h3 className={resultTitle}>
        {s.kind === 'eventStudy' ? 'Regression event study' : 'Interaction regression'} for{' '}
        {run.outcome.name}
      </h3>
      <MetricGrid label="Regression summary">
        <MetricTile label="Observations" value={formatWords(String(e.observations))} />
        <MetricTile label="Clusters" value={formatWords(String(e.clusters))} />
        <MetricTile label="Retained coefficients" value={formatWords(String(e.terms.length))} />
      </MetricGrid>
      <p className="text-body text-muted">
        {s.kind === 'eventStudy'
          ? 'Unit and period fixed effects with one-way clustered covariance and a finite-sample correction. These pooled regression coefficients are not cohort-specific ATT estimates.'
          : 'Hierarchical interactions with CR2 clustered covariance and coefficient-specific Satterthwaite degrees of freedom. Lower-order terms are included with each interaction.'}{' '}
        Intervals use a {number(e.request.confidence * 100)}% confidence level and are pointwise,
        not simultaneous.
      </p>
      {s.kind === 'eventStudy' && (
        <>
          <p className="text-body text-muted">
            The omitted event period is {s.window.reference}.{' '}
            {s.window.tails === 'bin'
              ? 'The endpoint indicators include observations beyond the event window.'
              : 'Observations outside the event window share the omitted category.'}
          </p>
          <ExpandableChart label="Regression event study" option={option} className="h-[300px]" />
          <p className="text-body text-muted">
            {e.leadTest.kind === 'recorded'
              ? `Joint test of zero lead coefficients: F = ${number(e.leadTest.statistic)}, p ${formatP(e.leadTest.pValue, { withLabel: false }).text}. This compares leads with the recorded omitted category; non-rejection does not establish parallel trends.`
              : e.leadTest.reason}
            {s.window.reference >= 0
              ? ' The reference is not a pre-treatment period, so this is not a pre-treatment-normalized trend test.'
              : ''}
          </p>
          <EvidenceTable
            title="Event-time support"
            rows={e.events}
            rowKey={(r) => String(r.index)}
            noun="period"
            empty="No event periods"
            columns={[
              { id: 'period', header: 'Event period', value: (r) => r.period },
              { id: 'support', header: 'Observations', value: (r) => r.support },
              {
                id: 'retained',
                header: 'Coefficient',
                value: (r) => (e.terms.some((t) => t.index === r.index) ? 'Estimated' : 'Absorbed'),
              },
            ]}
          />
        </>
      )}
      {s.kind === 'interactions' && (
        <p className="text-body text-muted">
          A triple interaction measures a difference between differences in the fitted model. A
          causal interpretation requires the corresponding parallel-trends assumption for the
          untreated differences and a justified comparison group.
        </p>
      )}
      <EvidenceTable
        title="Regression coefficients"
        rows={e.terms}
        rowKey={(r) => String(r.index)}
        noun="coefficient"
        empty="No coefficients"
        exportName="panel-regression-coefficients"
        columns={[
          { id: 'term', header: 'Term', value: (r) => r.name },
          {
            id: 'estimate',
            header: 'Estimate',
            value: (r) => r.estimate,
            format: number,
            align: 'right',
          },
          {
            id: 'se',
            header: 'Standard error',
            value: (r) => r.standardError,
            format: number,
            align: 'right',
          },
          {
            id: 'df',
            header: 'Degrees of freedom',
            value: (r) => r.degreesOfFreedom,
            format: number,
            align: 'right',
          },
          { id: 'lower', header: 'Lower', value: (r) => r.lower, format: number, align: 'right' },
          { id: 'upper', header: 'Upper', value: (r) => r.upper, format: number, align: 'right' },
          {
            id: 'p',
            header: 'p-value',
            value: (r) => r.pValue,
            format: probability,
            align: 'right',
          },
        ]}
      />
      {e.omitted.length > 0 && (
        <p className="text-body text-muted">
          Not estimable in this specification: {e.omitted.join(', ')}. These coefficients were
          omitted, not estimated as zero.
        </p>
      )}
    </section>
  )
}
