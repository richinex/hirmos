import { useMemo } from 'react'
import type { FeWeightDiagnostic } from '@/domain/feWeights'
import { assertNever } from '@/domain/dop'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme, type ChartTheme } from '@/charts/theme'
import { baseOption, gridAuto, rangeSelection, valueAxis, tooltip } from '@/charts/grammar'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import { sectionTitle, fieldHint } from '@/components/ui/recipes'
import { formatStatistic, formatCount } from '@/lib/format/number'

type Available = Extract<FeWeightDiagnostic, { kind: 'available' }>

function weightPlots(evidence: Available, theme: ChartTheme) {
  const groups = evidence.groups
  const views = [
    {
      id: 'density',
      title: 'Within-group treatment variance',
      x: 'Within-group variance',
      y: 'Density',
      data: evidence.density,
    },
    {
      id: 'ranked',
      title: 'Ranked relative weights',
      x: 'Share of groups, highest weight first',
      y: 'Relative weight',
      data: groups.map((g) => [g.rankShare, g.relativeWeight]),
      reference: 1,
    },
    {
      id: 'cumulative',
      title: 'Cumulative identifying weight',
      x: 'Share of groups, highest weight first',
      y: 'Cumulative weight',
      data: [[0, 0], ...groups.map((g) => [g.rankShare, g.cumulativeWeight])],
    },
    {
      id: 'dropout',
      title: 'Leaving out high-weight groups',
      x: 'Original identifying weight removed',
      y: 'Treatment coefficient',
      data: evidence.dropout.map((p) => [
        p.originalWeightRemoved,
        p.fit.kind === 'estimated' ? p.fit.coefficient : null,
      ]),
    },
  ]
  return views.map((view) => ({
    ...view,
    option: {
      ...baseOption(theme, view.title),
      grid: gridAuto({ top: 28, bottom: 64 }),
      ...rangeSelection(theme),
      tooltip: tooltip(theme, 'axis'),
      xAxis: { ...valueAxis(theme, view.x), nameGap: 25, splitLine: { show: false } },
      yAxis: { ...valueAxis(theme, view.y), scale: view.id === 'dropout' },
      series: [
        {
          type: 'line',
          connectNulls: false,
          showSymbol: view.id === 'dropout',
          symbolSize: 7,
          data: view.data,
          lineStyle: { color: theme.signal, width: 2 },
          itemStyle: { color: theme.signal },
          ...(view.reference === undefined
            ? {}
            : {
                markLine: {
                  silent: true,
                  symbol: 'none',
                  lineStyle: { color: theme.muted, type: 'dashed' },
                  label: {
                    formatter: 'Equal weight',
                    position: 'insideEndTop',
                    color: theme.muted,
                    fontFamily: theme.font,
                  },
                  data: [{ yAxis: view.reference }],
                },
              }),
        },
      ],
    },
  }))
}

type Props = { readonly evidence: FeWeightDiagnostic; readonly grouping: string }

function AvailableDiagnostic({ evidence, grouping }: Props & { readonly evidence: Available }) {
  const theme = useChartTheme()
  const plots = useMemo(() => weightPlots(evidence, theme), [evidence, theme])
  const metrics = [
    ['Gini of group variances', evidence.giniVariance],
    ['Maximum group weight', evidence.maximumWeight],
    ['Effective groups', evidence.effectiveGroups],
    ['Top-decile weight share', evidence.topDecileShare],
  ] as const
  const labels = useMemo(() => new Map(evidence.groups.map((g) => [g.group, g.label])), [evidence])
  return (
    <section className="mt-4 space-y-4" aria-label="Fixed-effects weight diagnostics">
      <h3 className={sectionTitle}>Fixed-effects weight diagnostics</h3>
      <p className={fieldHint}>
        These weights show how much each group contributes to the treatment coefficient.
        Concentrated weights do not establish collider bias.
      </p>
      <dl className="grid grid-cols-2 gap-4 xl:grid-cols-4">
        {metrics.map(([label, value]) => (
          <div key={label}>
            <dt className={fieldHint}>{label}</dt>
            <dd className="text-subtitle tabular-nums text-ink">
              {formatStatistic('raw', value).text}
            </dd>
          </div>
        ))}
      </dl>
      <div className="grid grid-cols-1 gap-4 xl:grid-cols-2">
        {plots.map((plot) => (
          <figure key={plot.id}>
            <figcaption className="mb-2 text-body font-medium text-ink">{plot.title}</figcaption>
            <ExpandableChart
              option={plot.option}
              label={plot.title}
              testId={`fe-weights-${plot.id}`}
              className="h-[280px]"
            />
          </figure>
        ))}
      </div>
      <EvidenceTable
        title="Group contributions"
        help="Variance uses the group size as its divisor. Each weight is the group’s treatment sum of squares divided by the total. Relative weight compares it with equal weighting across groups. The Gini summarises group variances, not weights. The top decile contains the highest-weight tenth of groups, rounded up."
        rows={evidence.groups}
        rowKey={(g) => String(g.group)}
        noun="group"
        empty="No groups were recorded."
        columns={[
          { id: 'label', header: grouping, value: (g) => g.label },
          {
            id: 'observations',
            header: 'Rows',
            value: (g) => g.observations,
            format: (_, g) => formatCount(g.observations).text,
          },
          figureColumn<Available['groups'][number]>('variance', 'Variance', (g) => g.variance),
          figureColumn<Available['groups'][number]>('weight', 'Weight', (g) => g.weight),
          figureColumn<Available['groups'][number]>(
            'relative',
            'Relative weight',
            (g) => g.relativeWeight,
          ),
          figureColumn<Available['groups'][number]>(
            'cumulative',
            'Cumulative weight',
            (g) => g.cumulativeWeight,
          ),
        ]}
      />
      <EvidenceTable
        title="High-weight group leave-out"
        help="Each fit removes groups from the original weight ranking and retains at least two groups. Tied weights are ordered by group value. Coefficients have no uncertainty intervals. Removing groups changes the estimation sample; a stable coefficient does not establish causal validity."
        rows={evidence.dropout}
        rowKey={(p) => String(p.removedGroups.length)}
        noun="fit"
        empty="No leave-out fits were recorded."
        columns={[
          { id: 'removed', header: 'Groups removed', value: (p) => p.removedGroups.length },
          {
            id: 'labels',
            header: 'Removed group values',
            value: (p) =>
              p.removedGroups.length === 0
                ? 'None'
                : p.removedGroups.map((id) => labels.get(id)).join(', '),
          },
          {
            id: 'rows',
            header: 'Rows retained',
            value: (p) => p.remainingObservations,
            format: (_, p) => formatCount(p.remainingObservations).text,
          },
          figureColumn<Available['dropout'][number]>(
            'weight',
            'Weight removed',
            (p) => p.originalWeightRemoved,
          ),
          {
            id: 'coefficient',
            header: 'Coefficient',
            value: (p) => (p.fit.kind === 'estimated' ? p.fit.coefficient : p.fit.reason),
            format: (_, p) =>
              p.fit.kind === 'estimated'
                ? formatStatistic('raw', p.fit.coefficient).text
                : p.fit.reason,
          },
        ]}
      />
      <p className={fieldHint}>
        {formatCount(evidence.observations).text} fitted rows across{' '}
        {formatCount(evidence.groups.length).text} groups. The density is a smoothed view of the
        group variances.
      </p>
    </section>
  )
}

export function FeWeightDiagnosticResult({ evidence, grouping }: Props) {
  switch (evidence.kind) {
    case 'notApplicable':
      return null
    case 'notRecorded':
      return (
        <Alert tone="info" className="mt-3">
          Weight diagnostics were not recorded for this run. Run the regression again to add them.
        </Alert>
      )
    case 'unavailable':
      return (
        <Alert tone="info" className="mt-3">
          {evidence.reason}
        </Alert>
      )
    case 'available':
      return <AvailableDiagnostic evidence={evidence} grouping={grouping} />
    default:
      return assertNever(evidence)
  }
}
