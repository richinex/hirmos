import { useMemo } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { planComparisonOption } from '@/charts/decisions/planValues'
import { useChartTheme } from '@/charts/theme'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import {
  COMPARED_PLAN_LIMIT,
  type DecisionEvidence,
  type PlanEstimate,
  type PlanRow,
} from '@/domain/decisionEvidence'
import { formatCount, formatPercent, formatStatistic } from '@/lib/format/number'

const rate = (value: number) => formatStatistic('mean', value).text
const signed = (value: number) => (value > 0 ? `+${rate(value)}` : rate(value))

const interval = (estimate: PlanEstimate, show: (value: number) => string) =>
  estimate.interval === null
    ? 'Not evaluated'
    : `${show(estimate.interval.lower)} to ${show(estimate.interval.upper)}`

const PLANS_HELP = `Each plan is scored by weighting the logged outcomes by the recorded chances. Intervals are 95% percentile bootstraps, computed for up to ${COMPARED_PLAN_LIMIT} plans per run: equal chance, the copied rule, and every always plan when they fit. The difference is the plan’s value minus the observed average on the same rows, with a paired interval. The copied rule is scored only on the rows held out from copying.`

const WEIGHTS_HELP =
  'Support is the share of scored rows where the plan could have made the logged choice. The largest weight is the biggest ratio of the plan’s chance to the recorded chance. Effective rows are (Σw)² ÷ Σw²: the number of equally weighted rows that would give the same precision.'

const VALUE_COLUMNS: EvidenceColumn<PlanRow>[] = [
  { id: 'plan', header: 'Plan', value: (row) => row.label },
  {
    id: 'value',
    header: 'Weighted estimate',
    align: 'right',
    value: (row) => row.scored.value.value,
    format: (v) => rate(Number(v)),
  },
  {
    id: 'interval',
    header: '95% interval',
    align: 'right',
    value: (row) => row.scored.value.interval?.lower ?? -1,
    format: (_, row) => interval(row.scored.value, rate),
  },
  {
    id: 'difference',
    header: 'Difference from observed',
    align: 'right',
    value: (row) => row.scored.difference.value,
    format: (v) => signed(Number(v)),
  },
  {
    id: 'paired',
    header: 'Paired 95% interval',
    align: 'right',
    value: (row) => row.scored.difference.interval?.lower ?? -1,
    format: (_, row) => interval(row.scored.difference, signed),
  },
  {
    id: 'randomised',
    header: 'Randomised estimate',
    align: 'right',
    value: (row) => row.randomised?.mean ?? -1,
    format: (_, row) =>
      row.randomised === null
        ? 'Not evaluated'
        : `${rate(row.randomised.mean)} (${formatCount(row.randomised.rows).text} rows)`,
  },
]

const WEIGHT_COLUMNS: EvidenceColumn<PlanRow>[] = [
  { id: 'plan', header: 'Plan', value: (row) => row.label },
  {
    id: 'rows',
    header: 'Rows scored',
    align: 'right',
    value: (row) => row.scored.rows,
    format: (v) => formatCount(Number(v)).text,
  },
  {
    id: 'support',
    header: 'Support',
    align: 'right',
    value: (row) => row.scored.weights.support,
    format: (v) => formatPercent(Number(v)).text,
  },
  {
    id: 'weight',
    header: 'Largest weight',
    align: 'right',
    value: (row) => row.scored.weights.maxWeight,
    format: (v) => formatStatistic('raw', Number(v)).text,
  },
  {
    id: 'effective',
    header: 'Effective rows',
    align: 'right',
    value: (row) => row.scored.weights.effectiveRows,
    format: (v) => formatCount(Math.round(Number(v))).text,
  },
]

/** Compared plans first, in request order, then the rest. */
const ordered = (plans: readonly PlanRow[]) => [
  ...plans.filter((plan) => plan.compared),
  ...plans.filter((plan) => !plan.compared),
]

const axisLabel = (plan: PlanRow) => plan.label.replace(/^Always /, '')

/** The scored plans of one evidence run: values and paired differences per plan, then the plans in tables. */
export function PlanComparison({
  run,
  plans,
}: {
  readonly run: DecisionEvidence
  readonly plans: readonly PlanRow[]
}) {
  const theme = useChartTheme()
  const observed = run.scoring!.result.observed
  const option = useMemo(
    () =>
      planComparisonOption(
        ordered(plans).map((plan) => ({
          label: plan.label,
          axisLabel: axisLabel(plan),
          value: {
            estimate: plan.scored.value.value,
            lower: plan.scored.value.interval?.lower ?? null,
            upper: plan.scored.value.interval?.upper ?? null,
          },
          difference: {
            estimate: plan.scored.difference.value,
            lower: plan.scored.difference.interval?.lower ?? null,
            upper: plan.scored.difference.interval?.upper ?? null,
          },
          bounds: plan.bounds,
          randomised: plan.randomised?.mean ?? null,
        })),
        observed.value,
        run.outcome.name,
        theme,
      ),
    [plans, observed.value, run.outcome.name, theme],
  )

  return (
    <section aria-labelledby={`plans-${run.id}`} className="mt-4">
      <div className="flex items-center gap-1.5">
        <h4 id={`plans-${run.id}`} className="m-0 text-body font-medium text-ink">
          Compare plans
        </h4>
        <ParameterHelp label="Compare plans" help={PLANS_HELP} />
      </div>
      <p className="mb-0 mt-1 text-body text-muted">
        Observed average of the logged rows: {rate(observed.value)}
        {observed.interval === null
          ? '.'
          : `, 95% interval ${rate(observed.interval.lower)} to ${rate(observed.interval.upper)}.`}
      </p>
      <ExpandableChart
        option={option}
        label="Value of each plan and its difference from the observed average"
        className="mt-2 h-[480px]"
      />
      <div className="-mx-3.5 mt-4">
        <EvidenceTable
          title="Every scored plan"
          help={PLANS_HELP}
          frame="none"
          rows={plans}
          rowKey={(row) => String(row.index)}
          noun="plan"
          empty="No plans were scored."
          columns={VALUE_COLUMNS}
        />
      </div>
      <div className="-mx-3.5 mt-4">
        <EvidenceTable
          title="Weight checks"
          help={WEIGHTS_HELP}
          frame="none"
          rows={plans}
          rowKey={(row) => String(row.index)}
          noun="plan"
          empty="No plans were scored."
          columns={WEIGHT_COLUMNS}
        />
      </div>
    </section>
  )
}
