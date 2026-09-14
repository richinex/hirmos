import { ExpandableChart } from '@/charts/ExpandableChart'
import { survivalCurvesOption, comparisonMeasureOption } from '@/charts/survival/curves'
import { importanceOption } from '@/charts/survival/regression'
import { useChartTheme } from '@/charts/theme'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { caption } from '@/components/ui/recipes'
import { AalenCoefficients } from './AalenCoefficients'
import type { SurvivalRunArtifact } from '@/domain/survival'
import { formatCount, formatP, formatPercent, formatStatistic } from '@/lib/format/number'
import { SurvivalInterpretation } from './SurvivalInterpretation'

const statistic = (value: number) => formatStatistic('raw', value).text

export function AalenResult({ run }: { readonly run: Extract<SurvivalRunArtifact, { kind: 'aalen-run' }> }) {
  const { evidence, configuration } = run
  const names = ['Intercept', ...configuration.covariates.map((c) => c.name)]
  const rows = evidence.coefficients.map(([slope, coefficient, standardError, z, p], i) => ({ name: names[i]!, slope, coefficient, standardError, z, p }))
  return <>
    <MetricGrid>
      <MetricTile frame="cell" size="compact" label="Last fitted time" value={formatStatistic('raw', evidence.lastTime)} context={configuration.duration.name} />
      <MetricTile frame="cell" size="compact" label="Events used" value={formatCount(evidence.fittedEvents)} context={`of ${evidence.events} observed events`} />
      <MetricTile frame="cell" size="compact" label="Overall association" value={formatP(evidence.pValue)} context={`chi-squared ${statistic(evidence.chisq)} on ${evidence.degreesOfFreedom} df`} />
    </MetricGrid>
    <SurvivalInterpretation run={run}
      bottomLine={<>The coefficient curves show how each covariate’s association with the event rate changes during follow-up. An upward slope indicates an increased event rate; a downward slope indicates a decreased rate, with the other covariates held fixed. The fit extends through time {statistic(evidence.lastTime)}.</>}
      uncertainty={<>The dashed curves give approximate 95% confidence bounds at each time. They do not provide 95% coverage for the entire curve at once. Estimates become less precise as fewer observations remain at risk.</>}
      mustBeTrue={<>Covariates must contribute additively to the event rate. Censoring must be independent of the event after accounting for the covariates. These are associations unless the study design supports a causal interpretation.</>}
    />
    <div className="mt-4"><EvidenceTable title="Additive coefficient summary" rows={rows} rowKey={(r) => r.name} noun="term" empty="No coefficient estimates." exportName="aalen-coefficients" frame="none" columns={[
      { id: 'name', header: 'Term', value: (r) => r.name },
      ...(['slope', 'coefficient', 'standardError', 'z', 'p'] as const).map((key) => ({ id: key, header: { slope: 'Slope', coefficient: 'Weighted coefficient', standardError: 'Standard error', z: 'z', p: 'p' }[key], align: 'right' as const, value: (r: typeof rows[number]) => r[key], format: (value: unknown) => key === 'p' ? formatP(Number(value), { withLabel: false }).text : statistic(Number(value)) })),
    ]} /></div>
    <p className={caption('mt-2')}>The table summarises the coefficient increments using Aalen’s test weights, as in survival::summary.aareg. Its slope and weighted coefficient are different summaries. The chart shows the cumulative coefficient itself.</p>
    <AalenCoefficients names={names} curves={evidence.curves} />
  </>
}

export function ForestResult({ run }: { readonly run: Extract<SurvivalRunArtifact, { kind: 'survival-forest-run' }> }) {
  const theme = useChartTheme()
  const { evidence, configuration } = run
  const last = evidence.predictionTimes.at(-1)!
  const rows = configuration.covariates.map((c, i) => ({ name: c.name, profile: evidence.profile[i]!, importance: evidence.importance[i]! }))
  const importance = rows.flatMap((r) => r.importance.kind === 'recorded' ? [{ name: r.name, value: r.importance.result }] : [])
  const survival = { name: `Prepared row ${evidence.predictionRow + 1}`, points: evidence.predictionTimes.map((t, i) => [t, evidence.survival[i]!] as const) }
  const hazard = { name: survival.name, points: evidence.predictionTimes.map((t, i) => [t, evidence.cumulativeHazard[i]!] as const) }
  return <>
    <MetricGrid>
      <MetricTile frame="cell" size="compact" label="Trees" value={formatCount(evidence.trees)} context={`seed ${configuration.settings.seed}`} />
      <MetricTile frame="cell" size="compact" label="Out-of-bag concordance" value={formatStatistic('raw', evidence.concordance.kind === 'recorded' ? evidence.concordance.result : Number.NaN)} context={evidence.concordance.kind === 'recorded' ? 'ranking of comparable observations' : 'unavailable'} />
      <MetricTile frame="cell" size="compact" label={`Event-free at ${statistic(last)}`} value={formatPercent(evidence.survival.at(-1)!)} context={`prepared row ${evidence.predictionRow + 1}`} />
    </MetricGrid>
    <SurvivalInterpretation run={run}
      bottomLine={<>For the covariate values in prepared row {evidence.predictionRow + 1}, the forest estimates an event-free probability of {formatPercent(evidence.survival.at(-1)!).text} at time {statistic(last)}. The curve shows this prediction across follow-up; the table identifies the covariate values used.</>}
      uncertainty={<>No confidence interval is calculated for this prediction. Out-of-bag concordance checks whether observations with earlier events receive higher predicted risk, using trees that did not train on those observations. It measures ranking accuracy rather than uncertainty in the curve.</>}
      mustBeTrue={<>Censoring must be independent of the event conditional on the covariates. Predictions for other populations require separate validation. Covariate importance measures predictive contribution and does not establish a causal effect.</>}
    />
    <div className="mt-4"><EvidenceTable title="Prediction profile and importance" rows={rows} rowKey={(r) => r.name} noun="covariate" empty="No covariates." exportName="survival-forest-importance" frame="none" columns={[
      { id: 'name', header: 'Covariate', value: (r) => r.name },
      { id: 'profile', header: 'Profile value', value: (r) => r.profile, align: 'right', format: (v) => statistic(Number(v)) },
      { id: 'importance', header: 'Permutation importance', value: (r) => r.importance.kind === 'recorded' ? r.importance.result : 'Unavailable', format: (value) => typeof value === 'number' ? statistic(value) : String(value), align: 'right' },
    ]} /></div>
    <p className={caption('mt-2')}>Larger positive importance means that shuffling the covariate reduced out-of-bag concordance more. Negative values are possible. They mean shuffling improved the score in this fitted forest.</p>
    <ExpandableChart className="mt-4" style={{ height: Math.max(180, rows.length * 30 + 70) }} label="Permutation importance" testId="survival-forest-importance" option={importanceOption(importance, theme)} />
    <ExpandableChart className="mt-4 h-[280px]" label="Forest event-free probability" testId="survival-forest-survival" option={survivalCurvesOption([survival], 'follow-up time', theme, { stepped: true })} />
    <ExpandableChart className="mt-4 h-[280px]" label="Forest cumulative hazard" testId="survival-forest-hazard" option={comparisonMeasureOption([hazard], 'cumulative hazard', theme, true)} />
    <p className={caption('mt-2')}>The forest averages the trees’ cumulative hazards and converts that average to survival probability. The plotted profile uses all fitted trees; the concordance and permutation importance use out-of-bag observations.</p>
  </>
}
