import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { memo, useState } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { contributionOption, rootCauseBars, rootCauseOption } from '@/charts/rootCause'
import { Select } from '@/components/ui/Select'
import { field, fieldLabel } from '@/components/ui/recipes'
import { useChartTheme } from '@/charts/theme'
import type { RootCauseRun } from '@/domain/rootCauseAnalysis'
import { formatStatistic } from '@/lib/format/number'
import { downloadText } from '@/data/bundleFiles'
import { button, resultSurface, resultTitle, fieldHint } from '@/components/ui/recipes'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'

const number = (value: number) => formatStatistic('raw', value).text
const percentile = (value: number) => String(Number((100 * value).toPrecision(12)))

interface Contribution {
  readonly node: number
  readonly name: string
  readonly estimate: number
  readonly lower: number
  readonly upper: number
}

export const RootCauseRunResult = memo(function RootCauseRunResult({
  run,
}: {
  readonly run: RootCauseRun
}) {
  const theme = useChartTheme()
  const [meanNode, setMeanNode] = useState(run.model.target)
  const { outcome } = run.evidence
  const { summary } = outcome
  const nodes = outcome.nodes
  const title =
    outcome.kind === 'anomaly'
      ? 'Anomaly attribution scores'
      : outcome.kind === 'change'
        ? 'Contributions to the mean change'
        : 'Predicted outcome after the shifts'
  const rows = nodes.map((node, index): Contribution => ({
    node,
    name: run.model.names[node],
    estimate: summary.estimates[index],
    lower: summary.bounds[index][0],
    upper: summary.bounds[index][1],
  }))
  const columns: readonly EvidenceColumn<Contribution>[] = [
    { id: 'variable', header: 'Variable', value: (row) => row.name },
    {
      id: 'estimate',
      header: 'Estimate',
      align: 'right',
      value: (row) => row.estimate,
      format: (_, row) => number(row.estimate),
    },
    {
      id: 'lower',
      header: `${percentile(summary.quantiles[0])}th percentile`,
      align: 'right',
      value: (row) => row.lower,
      format: (_, row) => number(row.lower),
    },
    {
      id: 'upper',
      header: `${percentile(summary.quantiles[1])}th percentile`,
      align: 'right',
      value: (row) => row.upper,
      format: (_, row) => number(row.upper),
    },
  ]
  return (
    <section aria-label="Root-cause result" className={resultSurface()}>
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0">
          <h3 className={`${resultTitle} m-0`}>{title}</h3>
          <p className="m-0 mt-1 break-words text-body text-muted">
            Target:{' '}
            <span className="font-medium text-ink">{run.model.names[run.model.target]}</span>
          </p>
        </div>
        <button
          type="button"
          className={button('quiet')}
          onClick={() => downloadText(`root-cause-${run.id}.json`, JSON.stringify(run, null, 2))}
        >
          Export analysis record
        </button>
      </div>
      <ExpandableChart
        label={title}
        testId="root-cause-bars"
        style={{ height: Math.max(230, rootCauseBars(run).length * 32 + 80) }}
        option={rootCauseOption(run, theme)}
      />
      {outcome.kind === 'change' && outcome.means.kind === 'recorded' && (
        <details>
          <DisclosureSummary className="cursor-pointer text-body font-medium">
            Compare observed means
          </DisclosureSummary>
          <div className="mt-3 space-y-3">
            <label className="block">
              <span className={fieldLabel}>Variable</span>
              <Select
                aria-label="Observed mean variable"
                className={field('text', 'mt-1')}
                value={String(meanNode)}
                onChange={(event) => setMeanNode(Number(event.target.value))}
              >
                {outcome.means.values.map((entry) => (
                  <option key={entry.node} value={entry.node}>
                    {run.model.names[entry.node]}
                  </option>
                ))}
              </Select>
            </label>
            <ExpandableChart
              label="Observed baseline and comparison means"
              testId="root-cause-means"
              style={{ height: 260 }}
              option={contributionOption(
                outcome.means.values
                  .filter((entry) => entry.node === meanNode)
                  .flatMap((entry) => [
                    { name: 'Baseline', estimate: entry.baseline, interval: null },
                    { name: 'Comparison', estimate: entry.comparison, interval: null },
                  ]),
                `Mean ${run.model.names[meanNode]}`,
                theme,
                'Observed mean',
              )}
            />
            <p className={fieldHint}>These are observed averages, not attributed causal effects.</p>
          </div>
        </details>
      )}
      {outcome.kind === 'intervention' && (
        <p className={`${fieldHint} max-w-[65ch]`}>
          The observed mean is {number(outcome.observedMean)}. The estimated mean after the
          specified shifts is {number(summary.estimates[outcome.nodes.indexOf(outcome.target)])}.
        </p>
      )}
      <EvidenceTable
        title="Variable estimates"
        rows={rows}
        columns={columns}
        rowKey={(row) => String(row.node)}
        noun="variable"
        empty="No variable estimates."
        frame="none"
        exportName={`root-cause-estimates-${run.id}`}
      />
      {summary.optimizerStatus !== 0 && (
        <p role="status" className="text-body text-warn">
          The numerical summary did not report convergence. Review the replicate estimates before
          interpreting its centre.
        </p>
      )}
    </section>
  )
})
