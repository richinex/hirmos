import { memo, useMemo } from 'react'
import type { EChartsCoreOption } from 'echarts/core'
import type { EstimationRunArtifact } from '@/domain/estimation'
import { EvidenceTable, figureColumn } from '@/components/table/EvidenceTable'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { useChartTheme } from '@/charts/theme'
import { axisLabelStyle, baseOption, gridAuto, legend, tooltip, valueAxis } from '@/charts/grammar'
import { fieldHint } from '@/components/ui/recipes'

type Run = Extract<EstimationRunArtifact, { readonly kind: 'predictor-synthetic-control-run' }>
export const PredictorSyntheticResult = memo(function PredictorSyntheticResult({
  run,
  outcome,
}: {
  readonly run: Run
  readonly outcome: string
}) {
  const theme = useChartTheme()
  const { evidence, catalog, configuration } = run
  const treated =
    catalog.units.find((unit) => unit.code === evidence.treatedUnit)?.label ??
    String(evidence.treatedUnit)
  const periods = useMemo(
    () =>
      evidence.plotPeriods.map(
        (code) => catalog.periods.find((period) => period.code === code)?.label ?? String(code),
      ),
    [evidence.plotPeriods, catalog.periods],
  )
  const event = catalog.periods.find(
    (period) => period.code === configuration.interventionPeriod,
  )?.label
  const charts = useMemo(() => {
    const common: EChartsCoreOption = {
      ...baseOption(
        theme,
        `Observed and synthetic ${outcome} for the treated unit, with the intervention period marked.`,
      ),
      grid: gridAuto({ top: 38, bottom: 36 }),
      tooltip: tooltip(theme, 'axis'),
      xAxis: {
        type: 'category',
        data: periods,
        boundaryGap: false,
        axisLabel: axisLabelStyle(theme),
      },
      yAxis: valueAxis(theme),
    }
    const markLine =
      event === undefined
        ? undefined
        : {
            silent: true,
            symbol: 'none',
            lineStyle: { color: theme.muted, type: 'dashed' },
            label: { show: false },
            data: [{ xAxis: event }],
          }
    return [
      {
        ...common,
        legend: { ...legend(theme, ['Observed', 'Synthetic']), top: 0, bottom: 'auto' },
        series: [
          {
            name: 'Observed',
            type: 'line',
            data: evidence.observed,
            showSymbol: false,
            lineStyle: { color: theme.ink, width: 1.5 },
            itemStyle: { color: theme.ink },
            markLine,
          },
          {
            name: 'Synthetic',
            type: 'line',
            data: evidence.synthetic,
            connectNulls: false,
            showSymbol: false,
            lineStyle: { color: theme.info, width: 1.5, type: 'dashed' },
            itemStyle: { color: theme.info },
          },
        ],
      },
      {
        ...common,
        series: [
          {
            name: 'Observed minus synthetic',
            type: 'line',
            data: evidence.gaps,
            connectNulls: false,
            showSymbol: false,
            lineStyle: { color: theme.info, width: 1.5 },
            itemStyle: { color: theme.info },
            markLine: {
              ...markLine,
              silent: true,
              symbol: 'none',
              label: { show: false },
              lineStyle: { color: theme.muted, type: 'dashed' },
              data: [...(event === undefined ? [] : [{ xAxis: event }]), { yAxis: 0 }],
            },
          },
        ],
      },
    ] satisfies EChartsCoreOption[]
  }, [evidence, theme, outcome, event, periods])
  const donors = evidence.donors.map((code, index) => ({
    name: catalog.units.find((unit) => unit.code === code)?.label ?? String(code),
    weight: evidence.donorWeights[index]!,
  }))
  const balances = evidence.balance.map((row, index) => {
    const ordinary = configuration.predictors[index]
    if (ordinary !== undefined)
      return {
        ...row,
        label: run.columns.find((c) => c.column === ordinary)?.name ?? row.predictor,
      }
    const special = configuration.special[index - configuration.predictors.length]
    if (special === undefined) return { ...row, label: row.predictor }
    const name = run.columns.find((c) => c.column === special.column)?.name ?? row.predictor
    const labels = special.periods.map(
      (code) => catalog.periods.find((p) => p.code === code)?.label ?? String(code),
    )
    return { ...row, label: `${name} (${special.summary}, ${labels.join(', ')})` }
  })
  const attempts = evidence.attempts.map((attempt, index) => ({
    ...attempt,
    index,
    status:
      attempt.kind === 'failed'
        ? attempt.detail
        : attempt.convergenceCode === 0
          ? 'Converged'
          : attempt.convergenceCode === 1
            ? 'Evaluation limit reached'
            : `Convergence code ${attempt.convergenceCode}`,
  }))
  const selection = evidence.selection
  const selected =
    selection.kind === 'single-predictor'
      ? 'The single predictor has weight one.'
      : selection.kind === 'supplied-weights'
        ? 'The specified predictor weights were normalized before fitting donor weights.'
        : `Selected fit: ${selection.start === 'equal' ? 'equal' : 'regression-based'} starting weights, ${selection.method === 'nelder-mead' ? 'Nelder-Mead' : 'BFGS'}, ${selection.evaluations} objective evaluations. ${selection.convergenceCode === 0 ? 'Converged.' : selection.convergenceCode === 1 ? 'Evaluation limit reached; convergence was not established.' : `Convergence code ${selection.convergenceCode}.`}`
  return (
    <div className="grid min-w-0 gap-6" data-testid="predictor-synthetic-results">
      <p className={fieldHint}>
        Treated unit: {treated}. The synthetic control is a weighted combination of the selected
        donor units. Predictor weights determine the balance criterion; donor weights determine the
        synthetic outcome.
      </p>
      <p className={fieldHint}>{selected}</p>
      <div>
        <h4 className="text-body font-medium text-ink">Observed and synthetic outcomes</h4>
        <ExpandableChart
          option={charts[0]!}
          label={`Observed and synthetic ${outcome} for ${treated}`}
          className="h-[260px]"
          testId="predictor-synthetic-path"
        />
      </div>
      <div>
        <h4 className="text-body font-medium text-ink">Observed minus synthetic</h4>
        <ExpandableChart
          option={charts[1]!}
          label={`Observed minus synthetic ${outcome} for ${treated}`}
          className="h-[220px]"
          testId="predictor-synthetic-gap"
        />
        <p className={fieldHint}>
          The vertical line marks the intervention period. Pre-intervention gaps describe fit;
          post-intervention gaps estimate the treatment effect under the synthetic-control
          assumptions. No uncertainty interval is reported.
        </p>
      </div>
      <EvidenceTable
        title="Predictor balance"
        rows={balances}
        noun="predictor"
        empty="No predictor summaries."
        rowKey={(_, i) => String(i)}
        columns={[
          { id: 'predictor', header: 'Predictor summary', value: (row) => row.label },
          figureColumn<(typeof balances)[number]>('treated', 'Treated', (row) => row.treated),
          figureColumn<(typeof balances)[number]>('synthetic', 'Synthetic', (row) => row.synthetic),
          figureColumn<(typeof balances)[number]>(
            'donorMean',
            'Donor mean',
            (row) => row.donorMean,
          ),
          figureColumn<(typeof balances)[number]>(
            'weight',
            'Predictor weight',
            (row) => row.weight,
          ),
        ]}
      />
      <EvidenceTable
        title="Donor weights"
        rows={donors}
        noun="donor"
        empty="No donor weights."
        rowKey={(row) => row.name}
        columns={[
          { id: 'unit', header: 'Donor unit', value: (row) => row.name },
          figureColumn<(typeof donors)[number]>('weight', 'Weight', (row) => row.weight),
        ]}
      />
      <EvidenceTable
        title="Optimization attempts"
        help="The selected fit minimizes the pre-intervention outcome loss among the completed attempts. Reaching the evaluation limit is not convergence."
        rows={attempts}
        noun="attempt"
        empty="No optimization attempts were requested."
        rowKey={(row) => String(row.index)}
        columns={[
          {
            id: 'start',
            header: 'Starting weights',
            value: (row) => (row.start === 'equal' ? 'Equal' : 'Regression-based'),
          },
          {
            id: 'method',
            header: 'Method',
            value: (row) => (row.method === 'nelder-mead' ? 'Nelder-Mead' : 'BFGS'),
          },
          { id: 'status', header: 'Status', value: (row) => row.status },
          figureColumn<(typeof attempts)[number]>(
            'evaluations',
            'Evaluations',
            (row) => row.evaluations,
          ),
          {
            id: 'loss',
            header: 'Outcome MSPE',
            align: 'right',
            value: (row) => (row.kind === 'completed' ? row.outcomeMspe : 'Not available'),
          },
        ]}
      />
      {evidence.preparationNotes.length > 0 && (
        <details>
          <summary className="text-body text-ink">Predictor preparation notes</summary>
          <ul className="text-body text-faint">
            {evidence.preparationNotes.map((note, i) => (
              <li key={i}>{note}</li>
            ))}
          </ul>
        </details>
      )}
    </div>
  )
})
