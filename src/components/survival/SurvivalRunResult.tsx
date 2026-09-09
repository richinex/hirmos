import { useState, type ReactNode } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { comparisonMeasureOption, hazardCurveOption, observedSurvivalOption, restrictedMeanOption, stateOccupancyOption, survivalCurvesOption, transitionMapOption, transitionMatrixOption } from '@/charts/survival/curves'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { Icon } from '@/components/Icon'
import { EvidenceTable, type EvidenceColumn, type EvidenceValue } from '@/components/table/EvidenceTable'
import { MetricTile } from '@/components/ui/figures'
import { SegmentedControl, type SegmentOption } from '@/components/ui/SegmentedControl'
import { caption, label, num, prose, well } from '@/components/ui/recipes'
import { assertNever } from '@/domain/dop'
import type { ComparisonSurvivalEvidence, ConversionDifference, ConversionRate, MultiStateSurvivalEvidence, NonparametricSurvivalEvidence, ParametricSurvivalFamily, SurvivalRunArtifact } from '@/domain/survival'
import { formatCount, formatEstimate, formatP, formatPercent, formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'

export const survivalFamilyLabel = (family: ParametricSurvivalFamily): string => ({
  exponential: 'Exponential',
  weibull: 'Weibull AFT',
  weibullPh: 'Weibull PH',
  logNormal: 'Log-normal',
  gamma: 'Gamma',
  gompertz: 'Gompertz',
  logLogistic: 'Log-logistic',
  generalizedGamma: 'Generalized gamma',
  generalizedF: 'Generalized F',
})[family]

export const survivalRunLabel = (run: SurvivalRunArtifact): string => {
  switch (run.kind) {
    case 'right-censored-survival-run': return 'Parametric'
    case 'nonparametric-survival-run': return 'Kaplan–Meier and Nelson–Aalen'
    case 'start-stop-survival-run': return 'Start–stop'
    case 'two-group-survival-run': return 'Two-group comparison'
    case 'multi-state-survival-run': return 'Multi-state'
    default: return assertNever(run)
  }
}

/** The method and the headline figure of a run, for the ledger row. */
export const survivalRunSummary = (run: SurvivalRunArtifact): { readonly method: string; readonly figure: string } => {
  switch (run.kind) {
    case 'right-censored-survival-run':
    case 'start-stop-survival-run': return { method: `${survivalRunLabel(run)} · ${survivalFamilyLabel(run.evidence.family)}`, figure: `median ${formatStatistic('raw', run.evidence.median).text}` }
    case 'nonparametric-survival-run': return { method: survivalRunLabel(run), figure: `event-free ${formatPercent(run.evidence.survival.at(-1) ?? Number.NaN).text}` }
    case 'two-group-survival-run': return { method: survivalRunLabel(run), figure: `event-free time difference ${formatStatistic('raw', run.evidence.restrictedMeanDifference).text}` }
    case 'multi-state-survival-run': return { method: `${survivalRunLabel(run)} · ${survivalFamilyLabel(run.evidence.family)}`, figure: `${formatCount(run.evidence.states.length).text} states` }
    default: return assertNever(run)
  }
}

/** The baseline parameters in the order flexsurv reports them, and how a covariate's exp(coefficient) reads for the family. */
const familyShape = (family: ParametricSurvivalFamily): { readonly baseline: readonly string[]; readonly ratio: string } => {
  switch (family) {
    case 'exponential': return { baseline: ['rate'], ratio: 'hazard ratio' }
    case 'weibull': return { baseline: ['shape', 'scale'], ratio: 'time ratio' }
    case 'weibullPh': return { baseline: ['shape', 'scale'], ratio: 'hazard ratio' }
    case 'logNormal': return { baseline: ['mean of log time', 'SD of log time'], ratio: 'time ratio' }
    case 'gamma': return { baseline: ['shape', 'rate'], ratio: 'rate ratio' }
    case 'gompertz': return { baseline: ['shape', 'rate'], ratio: 'hazard ratio' }
    case 'logLogistic': return { baseline: ['shape', 'scale'], ratio: 'time ratio' }
    case 'generalizedGamma': return { baseline: ['location', 'scale', 'shape Q'], ratio: 'time ratio' }
    case 'generalizedF': return { baseline: ['location', 'scale', 'shape Q', 'shape P'], ratio: 'time ratio' }
    default: return assertNever(family)
  }
}

const asNumber = (value: EvidenceValue): number => typeof value === 'number' ? value : Number(value)
const statistic = (value: number): string => formatStatistic('raw', value).text
const figureColumn = <Row,>(id: string, header: string, value: (row: Row) => number, print: (value: number) => string = statistic): EvidenceColumn<Row> =>
  ({ id, header, align: 'right', value, format: (value) => print(asNumber(value)) })
const interval = (bounds: readonly [number, number] | null): string => bounds === null ? '—' : `${statistic(bounds[0])} to ${statistic(bounds[1])}`

interface ParameterRow {
  readonly key: string
  readonly parameter: string
  readonly role: string
  readonly estimate: number
  readonly interval: readonly [number, number] | null
  readonly ratio: number | null
  readonly profile: number | null
}

interface NonparametricRow {
  readonly time: number
  readonly survival: number
  readonly survivalInterval: readonly [number, number]
  readonly cumulativeHazard: number
  readonly hazardInterval: readonly [number, number]
  readonly hazardIncrement: number
}

const parameterColumns = (ratioLabel: string): readonly EvidenceColumn<ParameterRow>[] => [
  { id: 'parameter', header: 'Parameter', value: (row) => row.parameter },
  { id: 'role', header: 'Role', value: (row) => row.role },
  figureColumn<ParameterRow>('estimate', 'Estimate', (row) => row.estimate),
  { id: 'interval', header: '95% interval', align: 'right', value: (row) => interval(row.interval) },
  { id: 'ratio', header: ratioLabel, align: 'right', value: (row) => row.ratio === null ? '' : row.ratio, format: (value) => value === '' ? '—' : statistic(asNumber(value)) },
  { id: 'profile', header: 'Drawn at', align: 'right', value: (row) => row.profile === null ? '' : row.profile, format: (value) => value === '' ? '—' : statistic(asNumber(value)) },
]

function Interpretation({ bottomLine, uncertainty, mustBeTrue }: { readonly bottomLine: ReactNode; readonly uncertainty: ReactNode; readonly mustBeTrue: ReactNode }) {
  return (
    <section className={well('mt-4 px-3 py-3')} aria-label="Interpretation">
      <h4 className="m-0 text-label font-medium text-faint">What this result means</h4>
      <h5 className="mb-0 mt-3 text-label font-medium text-bone">Bottom line</h5>
      <p className={prose('mb-0 mt-1 text-ink')}>{bottomLine}</p>
      <h5 className="mb-0 mt-3 border-t border-hair pt-3 text-label font-medium text-bone">Uncertainty</h5>
      <p className={prose('mb-0 mt-1 text-muted')}>{uncertainty}</p>
      <h5 className="mb-0 mt-3 border-t border-hair pt-3 text-label font-medium text-bone">What must be true</h5>
      <p className={prose('mb-0 mt-1 text-faint')}>{mustBeTrue}</p>
    </section>
  )
}

type ComparisonChart = 'survival' | 'eventFreeTime' | 'cumulativeHazard' | 'smoothedHazard'

const COMPARISON_CHARTS: readonly SegmentOption<ComparisonChart>[] = [
  { value: 'survival', label: 'Survival' },
  { value: 'eventFreeTime', label: 'Event-free time' },
  { value: 'cumulativeHazard', label: 'Cumulative hazard' },
  { value: 'smoothedHazard', label: 'Smoothed hazard' },
]

const atRiskAt = (points: readonly (readonly [number, number])[], time: number): number =>
  points.find(([observedTime]) => observedTime >= time)?.[1] ?? 0

type RecordedComparisonDiagnostics = Extract<ComparisonSurvivalEvidence['diagnostics'], { readonly kind: 'recorded' }>
type RecordedObservedConversion = Extract<ComparisonSurvivalEvidence['observedConversion'], { readonly kind: 'recorded' }>['result']
type RecordedFixedTimeConversion = Extract<ComparisonSurvivalEvidence['fixedTimeConversion'], { readonly kind: 'recorded' }>['result']

interface ConversionRow {
  readonly key: string
  readonly measure: string
  readonly groupZero: ConversionRateFigure
  readonly groupOne: ConversionRateFigure
  readonly difference: ConversionDifferenceFigure
  readonly pValue: number
}

type ConversionRateFigure = { readonly kind: 'rate'; readonly value: number }
type ConversionDifferenceFigure = { readonly kind: 'difference'; readonly value: number }
type ConversionFigure = ConversionRateFigure | ConversionDifferenceFigure

const conversionRate = (value: ConversionRate): ConversionRateFigure => ({ kind: 'rate', value })
const conversionDifference = (value: ConversionDifference): ConversionDifferenceFigure => ({ kind: 'difference', value })

const formatConversion = (figure: ConversionFigure) => {
  switch (figure.kind) {
    case 'rate': return formatPercent(figure.value, { precision: 1 })
    case 'difference': return formatEstimate(figure.value, { kind: 'probabilityDifference' })
    default: return assertNever(figure)
  }
}

const fixedPointScaleLabel = (scale: RecordedFixedTimeConversion['scaleTests'][number]['scale']): string => {
  switch (scale) {
    case 'naive': return 'Untransformed'
    case 'log': return 'Log'
    case 'complementaryLogLog': return 'Complementary log-log'
    case 'arcsineSquareRoot': return 'Arcsine square root'
    case 'logit': return 'Logit'
    default: return assertNever(scale)
  }
}

const recordedObservedConversion = (evidence: ComparisonSurvivalEvidence): RecordedObservedConversion | null => {
  switch (evidence.observedConversion.kind) {
    case 'recorded': return evidence.observedConversion.result
    case 'notRecorded':
    case 'unavailable': return null
    default: return assertNever(evidence.observedConversion)
  }
}

const recordedFixedTimeConversion = (evidence: ComparisonSurvivalEvidence): RecordedFixedTimeConversion | null => {
  switch (evidence.fixedTimeConversion.kind) {
    case 'recorded': return evidence.fixedTimeConversion.result
    case 'notRecorded':
    case 'unavailable': return null
    default: return assertNever(evidence.fixedTimeConversion)
  }
}

const petoPValue = (evidence: ComparisonSurvivalEvidence): number | null => {
  switch (evidence.petoPeto.kind) {
    case 'recorded': return evidence.petoPeto.result.pValue
    case 'notRecorded':
    case 'unavailable': return null
    default: return assertNever(evidence.petoPeto)
  }
}

function AtRiskTable({ diagnostics, truncationTime }: { readonly diagnostics: RecordedComparisonDiagnostics; readonly truncationTime: number }) {
  const times = Array.from({ length: 5 }, (_, index) => truncationTime * index / 4)
  return (
    <div className="mt-2 overflow-x-auto">
      <table className="w-full min-w-[420px] border-collapse text-label text-muted" aria-label="Number at risk">
        <thead>
          <tr className="border-b border-hair">
            <th className="px-2 py-1 text-left font-medium">Number at risk</th>
            {times.map((time) => <th key={time} className="px-2 py-1 text-right font-normal tabular-nums">{statistic(time)}</th>)}
          </tr>
        </thead>
        <tbody>
          <tr>
            <th className="px-2 py-1 text-left font-medium text-ink">Group 0</th>
            {times.map((time) => <td key={time} className="px-2 py-1 text-right tabular-nums">{formatCount(atRiskAt(diagnostics.groupZero.atRisk, time)).text}</td>)}
          </tr>
          <tr>
            <th className="px-2 py-1 text-left font-medium text-signal">Group 1</th>
            {times.map((time) => <td key={time} className="px-2 py-1 text-right tabular-nums">{formatCount(atRiskAt(diagnostics.groupOne.atRisk, time)).text}</td>)}
          </tr>
        </tbody>
      </table>
    </div>
  )
}

function ComparisonCharts({ evidence }: { readonly evidence: ComparisonSurvivalEvidence }) {
  const theme = useChartTheme()
  const [chart, setChart] = useState<ComparisonChart>('survival')
  const diagnostics = evidence.diagnostics
  if (diagnostics.kind === 'notRecorded') {
    return <ExpandableChart className="mt-3 h-[260px]" label="Observed event-free probability by group" testId="survival-groups" option={survivalCurvesOption([{ name: 'group 0', points: evidence.groupZeroCurve }, { name: 'group 1', points: evidence.groupOneCurve }], 'follow-up time', theme, { marks: [{ name: 'compared through', value: evidence.truncationTime }] })} />
  }

  const groupZero = { name: 'group 0', points: evidence.groupZeroCurve }
  const groupOne = { name: 'group 1', points: evidence.groupOneCurve }
  const curves = [groupZero, groupOne]
  const marks = [
    { name: 'compared through', value: evidence.truncationTime },
    ...diagnostics.crossingTimes.filter((time) => time > 0 && time <= evidence.truncationTime).slice(0, 3).map((time) => ({ name: 'curves cross', value: time })),
  ]
  const view = (() => {
    switch (chart) {
      case 'survival': return {
        label: 'Observed event-free probability by group',
        testId: 'survival-groups',
        option: observedSurvivalOption([
          { ...groupZero, censorTimes: diagnostics.groupZero.censorTimes },
          { ...groupOne, censorTimes: diagnostics.groupOne.censorTimes },
        ], 'follow-up time', theme, marks),
      }
      case 'eventFreeTime': return {
        label: 'Restricted mean event-free time by group',
        testId: 'restricted-mean-groups',
        option: restrictedMeanOption(curves, evidence.truncationTime, theme),
      }
      case 'cumulativeHazard': return {
        label: 'Cumulative hazard by group',
        testId: 'cumulative-hazard-groups',
        option: comparisonMeasureOption([
          { name: 'group 0', points: diagnostics.groupZero.cumulativeHazard },
          { name: 'group 1', points: diagnostics.groupOne.cumulativeHazard },
        ], 'cumulative hazard', theme, true),
      }
      case 'smoothedHazard': return {
        label: 'Smoothed hazard by group',
        testId: 'smoothed-hazard-groups',
        option: comparisonMeasureOption([
          { name: 'group 0', points: diagnostics.groupZero.smoothedHazard },
          { name: 'group 1', points: diagnostics.groupOne.smoothedHazard },
        ], 'smoothed hazard', theme, false),
      }
      default: return assertNever(chart)
    }
  })()

  return (
    <div className="mt-3">
      <SegmentedControl value={chart} onChange={setChart} options={COMPARISON_CHARTS} ariaLabel="Survival comparison chart" size="sm" wrap />
      <ExpandableChart className="mt-3 h-[280px]" label={view.label} testId={view.testId} option={view.option} />
      {chart === 'survival' ? <AtRiskTable diagnostics={diagnostics} truncationTime={evidence.truncationTime} /> : null}
      {chart === 'eventFreeTime' ? <p className={caption('mb-0 mt-2')}>Through time {statistic(evidence.truncationTime)}, group 0 accumulated {statistic(diagnostics.groupZero.restrictedMean)} and group 1 accumulated {statistic(diagnostics.groupOne.restrictedMean)} units of event-free time.</p> : null}
    </div>
  )
}

type MultiStateChart = 'occupancy' | 'transitions' | 'matrix'

const MULTI_STATE_CHARTS: readonly SegmentOption<MultiStateChart>[] = [
  { value: 'occupancy', label: 'Occupancy' },
  { value: 'transitions', label: 'Transitions' },
  { value: 'matrix', label: 'Probability matrix' },
]

function MultiStateCharts({ evidence, initial }: { readonly evidence: MultiStateSurvivalEvidence; readonly initial: number }) {
  const theme = useChartTheme()
  const [chart, setChart] = useState<MultiStateChart>('occupancy')
  const lastTime = evidence.predictionTimes.at(-1) ?? Number.NaN
  const lastMatrix = evidence.probabilities.at(-1) ?? []
  const view = (() => {
    switch (chart) {
      case 'occupancy': return { label: 'Estimated state occupancy', testId: 'state-occupancy', option: stateOccupancyOption(evidence.predictionTimes, evidence.states, evidence.probabilities, initial, theme) }
      case 'transitions': return { label: 'Permitted state transitions', testId: 'state-transitions', option: transitionMapOption(evidence.states, evidence.transitions, theme) }
      case 'matrix': return { label: `Transition probabilities at follow-up time ${statistic(lastTime)}`, testId: 'transition-probability-matrix', option: transitionMatrixOption(evidence.states, lastMatrix, lastTime, theme) }
      default: return assertNever(chart)
    }
  })()
  return (
    <div className="mt-3">
      <SegmentedControl value={chart} onChange={setChart} options={MULTI_STATE_CHARTS} ariaLabel="Multi-state chart" size="sm" fill />
      <ExpandableChart className="mt-3 h-[280px]" label={view.label} testId={view.testId} option={view.option} />
    </div>
  )
}

function Tiles({ children }: { readonly children: ReactNode }) {
  return <div className="mt-4 grid gap-px overflow-hidden rounded-lg border border-hair bg-hair sm:grid-cols-3">{children}</div>
}

/**
 * One recorded survival run as a card: the method and its headline figures, what they mean, the
 * fitted parameters, and the curves. `current` marks the newest run with the emphasised border, as
 * the other chapters' result cards do; the ledger shows the same card collapsed.
 */
export function SurvivalRunResult({ run, current = true, open = true, onDelete }: {
  readonly run: SurvivalRunArtifact
  readonly current?: boolean
  readonly open?: boolean
  readonly onDelete?: () => void
}) {
  const theme = useChartTheme()
  // The event-free chart carries the slider; the hazard chart follows its window.
  const [window, setWindow] = useState<VisibleWindow | null>(null)
  const summary = survivalRunSummary(run)
  const heading = (() => {
    switch (run.kind) {
      case 'right-censored-survival-run':
        return { title: survivalFamilyLabel(run.evidence.family), meta: `${formatCount(run.evidence.observations).text} observations · ${formatCount(run.evidence.events).text} events` }
      case 'start-stop-survival-run':
        return { title: survivalFamilyLabel(run.evidence.family), meta: `${formatCount(run.evidence.observations).text} intervals · ${formatCount(run.evidence.events).text} events` }
      case 'nonparametric-survival-run': return { title: 'Kaplan–Meier and Nelson–Aalen', meta: `${formatCount(run.evidence.observations).text} observations · ${formatCount(run.evidence.events).text} events` }
      case 'two-group-survival-run': return { title: 'Group 1 compared with group 0', meta: `${formatCount(run.evidence.observations).text} rows · compared through time ${statistic(run.evidence.truncationTime)}` }
      case 'multi-state-survival-run': return { title: `${survivalFamilyLabel(run.evidence.family)} transition model`, meta: `${formatCount(run.evidence.observations).text} transition rows · ${formatCount(run.evidence.states.length).text} states` }
      default: return assertNever(run)
    }
  })()
  const method = (() => {
    switch (run.kind) {
      case 'right-censored-survival-run': return 'Parametric survival'
      case 'nonparametric-survival-run': return 'Nonparametric survival'
      case 'start-stop-survival-run': return 'Start–stop survival'
      case 'two-group-survival-run': return 'Two-group survival comparison'
      case 'multi-state-survival-run': return 'Multi-state survival'
      default: return assertNever(run)
    }
  })()

  const body = (() => {
    switch (run.kind) {
      case 'nonparametric-survival-run': {
        const evidence: NonparametricSurvivalEvidence = run.evidence
        const lastTime = evidence.predictionTimes.at(-1) ?? Number.NaN
        const lastSurvival = evidence.survival.at(-1) ?? Number.NaN
        const rows: NonparametricRow[] = evidence.predictionTimes.map((time, index) => ({
          time,
          survival: evidence.survival[index] ?? Number.NaN,
          survivalInterval: [evidence.survivalLower[index] ?? Number.NaN, evidence.survivalUpper[index] ?? Number.NaN],
          cumulativeHazard: evidence.cumulativeHazard[index] ?? Number.NaN,
          hazardInterval: [evidence.cumulativeHazardLower[index] ?? Number.NaN, evidence.cumulativeHazardUpper[index] ?? Number.NaN],
          hazardIncrement: evidence.hazardIncrement[index] ?? Number.NaN,
        }))
        return <>
          <Tiles>
            <MetricTile frame="cell" size="compact" label={`Event-free at ${statistic(lastTime)}`} value={formatPercent(lastSurvival)} />
            <MetricTile frame="cell" size="compact" label="Events" value={formatCount(evidence.events)} context={`${formatCount(evidence.observations).text} observations`} />
            <MetricTile frame="cell" size="compact" label="Cumulative hazard" value={formatStatistic('raw', evidence.cumulativeHazard.at(-1) ?? Number.NaN)} context={`through ${statistic(lastTime)}`} />
          </Tiles>
          <Interpretation
            bottomLine={<>By follow-up time {statistic(lastTime)}, the Kaplan–Meier estimated survival function is {formatPercent(lastSurvival).text}. The Nelson–Aalen curve gives the estimated cumulative hazard over the same follow-up.</>}
            uncertainty={<>The table reports pointwise 95% confidence intervals for the estimated survival function and cumulative hazard.</>}
            mustBeTrue={<>An event indicator of 0 means the observation was right-censored at its recorded duration. These estimates describe the observed durations and events; they do not estimate a causal effect.</>}
          />
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>Observed event-free probability</p>
            <ExpandableChart className="h-[260px]" label="Kaplan–Meier event-free probability" testId="kaplan-meier-curve" option={survivalCurvesOption([{ name: 'Kaplan–Meier', points: evidence.predictionTimes.map((time, index) => [time, evidence.survival[index] ?? Number.NaN] as const) }], 'follow-up time', theme)} />
          </div>
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>Estimated cumulative hazard</p>
            <ExpandableChart className="h-[220px]" label="Nelson–Aalen cumulative hazard" testId="nelson-aalen-curve" option={comparisonMeasureOption([{ name: 'Nelson–Aalen', points: evidence.predictionTimes.map((time, index) => [time, evidence.cumulativeHazard[index] ?? Number.NaN] as const) }], 'cumulative hazard', theme, true)} />
          </div>
          <div className="mt-4">
            <EvidenceTable<NonparametricRow>
              frame="none"
              title="Curve values and period increments"
              rows={rows}
              rowKey={(row) => String(row.time)}
              noun="time"
              empty="The run reported no curve values."
              exportName="nonparametric-survival-curves"
              columns={[
                figureColumn<NonparametricRow>('time', 'Time', (row) => row.time),
                { id: 'survival', header: 'Event-free', align: 'right', value: (row) => formatPercent(row.survival).text },
                { id: 'survival-interval', header: '95% interval', align: 'right', value: (row) => `${formatPercent(row.survivalInterval[0]).text} to ${formatPercent(row.survivalInterval[1]).text}` },
                figureColumn<NonparametricRow>('cumulative-hazard', 'Cumulative hazard', (row) => row.cumulativeHazard),
                { id: 'hazard-interval', header: '95% hazard interval', align: 'right', value: (row) => `${statistic(row.hazardInterval[0])} to ${statistic(row.hazardInterval[1])}` },
                figureColumn<NonparametricRow>('hazard-increment', 'Nelson–Aalen increment', (row) => row.hazardIncrement),
              ]}
            />
            <p className={caption('mb-0 mt-2')}>The Nelson–Aalen increment is the increase in estimated cumulative hazard since the previous reported time. It is not an event probability.</p>
          </div>
        </>
      }
      case 'right-censored-survival-run':
      case 'start-stop-survival-run': {
        const evidence = run.evidence
        const shape = familyShape(evidence.family)
        const last = evidence.survival.at(-1) ?? Number.NaN
        const lastTime = evidence.predictionTimes.at(-1) ?? Number.NaN
        const covariates = run.configuration.covariates
        const rows: ParameterRow[] = [
          ...shape.baseline.map((name, index): ParameterRow => ({ key: `baseline-${index}`, parameter: name, role: 'baseline', estimate: evidence.naturalBaseline[index] ?? Number.NaN, interval: evidence.parameterIntervals[index] ?? null, ratio: null, profile: null })),
          ...covariates.map((column, index): ParameterRow => {
            const coefficient = evidence.coefficients[index] ?? Number.NaN
            return { key: `covariate-${column.id}`, parameter: column.name, role: 'covariate', estimate: coefficient, interval: evidence.parameterIntervals[shape.baseline.length + index] ?? null, ratio: Math.exp(coefficient), profile: evidence.profile[index] ?? null }
          }),
        ]
        const profileNote = covariates.length === 0
          ? 'The curve is the fitted distribution itself; there are no covariates.'
          : `The curve is drawn for a row at the average of each covariate (${covariates.map((column, index) => `${column.name} ${statistic(evidence.profile[index] ?? Number.NaN)}`).join(', ')}).`
        return <>
          <Tiles>
            <MetricTile frame="cell" size="compact" label={`Event-free at ${statistic(lastTime)}`} value={formatPercent(last)} />
            <MetricTile frame="cell" size="compact" label="Median time" value={formatStatistic('raw', evidence.median)} context={evidence.mean === null ? undefined : `mean ${statistic(evidence.mean)}`} />
            <MetricTile frame="cell" size="compact" label="AIC" value={formatStatistic('raw', evidence.aic)} context={`log likelihood ${statistic(evidence.logLikelihood)}`} />
          </Tiles>
          <Interpretation
            bottomLine={<>At follow-up time {statistic(lastTime)}, the model estimates that {formatPercent(last).text} of rows like the drawn profile remain event-free. The estimated median time to the event is {statistic(evidence.median)}. {profileNote}</>}
            uncertainty={<>The curve is a fitted point estimate. The 95% intervals in the table describe uncertainty in the fitted parameters; this result does not yet draw a band around the curve.</>}
            mustBeTrue={<>The chosen distribution and covariate form must describe how event risk changes over follow-up. Censoring must not depend on an unrecorded reason that also predicts the event. Covariate coefficients are associations unless a separate causal design justifies an effect interpretation.</>}
          />
          <div className="mt-4">
            <EvidenceTable<ParameterRow>
              frame="none"
              title="Fitted parameters"
              rows={rows}
              rowKey={(row) => row.key}
              noun="parameter"
              empty="The fit reported no parameter."
              exportName="survival-parameters"
              columns={parameterColumns(shape.ratio)}
            />
            <p className={caption('mb-0 mt-2')}>Baseline parameters are on their natural scale. A covariate's {shape.ratio} is exp(estimate) per unit of the covariate; the interval is for the estimate. Drawn at is the value the curve uses.</p>
          </div>
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>Event-free probability</p>
            <ExpandableChart className="h-[260px]" label="Fitted event-free probability" testId="survival-curve" window={window} onWindow={setWindow} option={survivalCurvesOption([{ name: 'fitted profile', points: evidence.predictionTimes.map((time, index) => [time, evidence.survival[index] ?? Number.NaN] as const) }], 'follow-up time', theme, { marks: Number.isFinite(evidence.median) && evidence.median <= lastTime ? [{ name: 'median', value: evidence.median }] : [] })} />
          </div>
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>Hazard over follow-up</p>
            <ExpandableChart className="h-[220px]" label="Fitted hazard" testId="hazard-curve" window={window} onWindow={setWindow} option={hazardCurveOption(evidence.predictionTimes, evidence.hazard, 'follow-up time', theme)} />
          </div>
        </>
      }
      case 'two-group-survival-run': {
        const evidence = run.evidence
        const direction = evidence.restrictedMeanDifference < 0 ? 'less' : 'more'
        const observed = recordedObservedConversion(evidence)
        const fixedTime = recordedFixedTimeConversion(evidence)
        const peto = petoPValue(evidence)
        const tests = [
          { key: 'two-stage', test: 'Two-stage', reads: 'overall test designed for crossing survival curves', p: evidence.twoStagePValue },
          { key: 'log-rank', test: 'Log-rank', reads: 'standard log-rank test', p: evidence.logRankPValue },
          { key: 'gehan', test: 'Gehan–Wilcoxon', reads: 'weighted by the pooled number at risk', p: evidence.gehanWilcoxonPValue },
          ...(peto === null ? [] : [{ key: 'peto-peto', test: 'Peto–Peto modified Gehan–Wilcoxon', reads: 'weighted by the pooled survival estimate', p: peto }]),
          { key: 'tarone', test: 'Tarone–Ware', reads: 'weighted by the square root of the pooled number at risk', p: evidence.taroneWarePValue },
          { key: 'weighted-km', test: 'Weighted Kaplan–Meier', reads: 'weighted Kaplan–Meier statistic', p: evidence.weightedKaplanMeierPValue },
          { key: 'absolute', test: 'Absolute difference', reads: 'integrated absolute difference between the survival curves', p: evidence.absoluteDifferencePValue },
          { key: 'squared', test: 'Squared difference', reads: 'integrated squared difference between the survival curves', p: evidence.squaredDifferencePValue },
          { key: 'ph', test: 'Proportional hazards', reads: 'test of the proportional-hazards assumption', p: evidence.proportionalHazardsPValue },
        ]
        const conversions: ConversionRow[] = [
          ...(observed === null ? [] : [{ key: 'observed', measure: 'Observed conversion; follow-up time ignored', groupZero: conversionRate(observed.groupZeroRate), groupOne: conversionRate(observed.groupOneRate), difference: conversionDifference(observed.difference), pValue: observed.pValue }]),
          ...(fixedTime === null ? [] : [{ key: 'fixed-time', measure: `Conversion by time ${statistic(fixedTime.time)}; Kaplan–Meier`, groupZero: conversionRate(fixedTime.groupZeroRate), groupOne: conversionRate(fixedTime.groupOneRate), difference: conversionDifference(fixedTime.difference), pValue: fixedTime.pValue }]),
        ]
        return <>
          {fixedTime === null ? <>
            <Tiles>
              <MetricTile frame="cell" size="compact" label="Event-free time difference" value={formatStatistic('raw', evidence.restrictedMeanDifference)} context={`through time ${statistic(evidence.truncationTime)}`} />
              <MetricTile frame="cell" size="compact" label="95% interval" value={formatStatistic('raw', evidence.restrictedMeanInterval[0])} context={`to ${statistic(evidence.restrictedMeanInterval[1])}`} />
              <MetricTile frame="cell" size="compact" label="Overall test p" value={formatP(evidence.twoStagePValue, { withLabel: false })} context="two-stage" />
            </Tiles>
            <Interpretation
              bottomLine={<>Up to follow-up time {statistic(evidence.truncationTime)}, group 1 accumulated {statistic(Math.abs(evidence.restrictedMeanDifference))} {direction} event-free time than group 0 on average.</>}
              uncertainty={<>The 95% interval runs from {statistic(evidence.restrictedMeanInterval[0])} to {statistic(evidence.restrictedMeanInterval[1])}. The overall test also compares the full curves and remains useful when they cross.</>}
              mustBeTrue={<>Censoring must be comparable between groups, and each row must represent an independent observation. This is a group comparison, not a causal effect, unless group assignment and the study design support that interpretation.</>}
            />
          </> : <>
            <Tiles>
              <MetricTile frame="cell" size="compact" label="Group 0 conversion" value={formatConversion(conversionRate(fixedTime.groupZeroRate))} context={`by time ${statistic(fixedTime.time)}`} />
              <MetricTile frame="cell" size="compact" label="Group 1 conversion" value={formatConversion(conversionRate(fixedTime.groupOneRate))} context={`by time ${statistic(fixedTime.time)}`} />
              <MetricTile frame="cell" size="compact" label="Difference" value={formatConversion(conversionDifference(fixedTime.difference))} context="group 1 minus group 0" />
            </Tiles>
            <Interpretation
              bottomLine={<>By follow-up time {statistic(fixedTime.time)}, estimated conversion was {formatConversion(conversionRate(fixedTime.groupZeroRate)).text} in group 0 and {formatConversion(conversionRate(fixedTime.groupOneRate)).text} in group 1, a difference of {formatConversion(conversionDifference(fixedTime.difference)).text}.</>}
              uncertainty={<>The 95% interval for the conversion difference runs from {formatConversion(conversionDifference(fixedTime.interval[0])).text} to {formatConversion(conversionDifference(fixedTime.interval[1])).text}. The data are compatible with no difference when this interval includes 0.</>}
              mustBeTrue={<>People who have not converted by the end of follow-up must be represented as censored, and censoring must be comparable between groups. This is a group comparison, not a causal effect, unless assignment and the study design support that interpretation.</>}
            />
          </>}
          {conversions.length > 0 && <div className="mt-4">
            <EvidenceTable<ConversionRow>
              frame="none"
              title="Conversion comparisons"
              rows={conversions}
              rowKey={(row) => row.key}
              noun="comparison"
              empty="The run reported no conversion comparison."
              exportName="survival-conversion-comparisons"
              columns={[
                { id: 'measure', header: 'Measure', value: (row) => row.measure },
                { id: 'group-zero', header: 'Group 0', align: 'right', value: (row) => formatConversion(row.groupZero).text },
                { id: 'group-one', header: 'Group 1', align: 'right', value: (row) => formatConversion(row.groupOne).text },
                { id: 'difference', header: 'Difference', align: 'right', value: (row) => formatConversion(row.difference).text },
                figureColumn<ConversionRow>('p', 'p', (row) => row.pValue, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
          </div>}
          {fixedTime !== null && <div className="mt-4">
            <EvidenceTable<RecordedFixedTimeConversion['scaleTests'][number]>
              frame="none"
              title="Fixed-time interval calculations"
              rows={fixedTime.scaleTests}
              rowKey={(row) => row.scale}
              noun="calculation"
              empty="The run reported no fixed-time calculation."
              exportName="survival-fixed-time-calculations"
              columns={[
                { id: 'scale', header: 'Scale', value: (row) => fixedPointScaleLabel(row.scale) },
                { id: 'group-zero', header: 'Group 0 interval', align: 'right', value: (row) => `${formatPercent(row.groupZeroInterval[0], { precision: 1 }).text} to ${formatPercent(row.groupZeroInterval[1], { precision: 1 }).text}` },
                { id: 'group-one', header: 'Group 1 interval', align: 'right', value: (row) => `${formatPercent(row.groupOneInterval[0], { precision: 1 }).text} to ${formatPercent(row.groupOneInterval[1], { precision: 1 }).text}` },
                figureColumn<RecordedFixedTimeConversion['scaleTests'][number]>('p', 'p', (row) => row.pValue, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
          </div>}
          <div className="mt-4">
            <EvidenceTable<typeof tests[number]>
              frame="none"
              title="Comparison tests"
              rows={tests}
              rowKey={(row) => row.key}
              noun="test"
              empty="The run reported no test."
              exportName="survival-comparison-tests"
              columns={[
                { id: 'test', header: 'Test', value: (row) => row.test },
                { id: 'reads', header: 'What it weighs', value: (row) => row.reads },
                figureColumn<typeof tests[number]>('p', 'p', (row) => row.p, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
          </div>
          <ComparisonCharts evidence={evidence} />
        </>
      }
      case 'multi-state-survival-run': {
        const evidence = run.evidence
        const initial = evidence.states[0] ?? 0
        const final = evidence.probabilities.at(-1) ?? []
        const row = final.slice(0, evidence.states.length)
        const destination = row.reduce((best, value, index) => value > (row[best] ?? -1) ? index : best, 0)
        const preparation = (() => {
          switch (evidence.preparation.kind) {
            case 'preparedRows': return null
            case 'longitudinalStates': return `Hirmos converted ${formatCount(evidence.preparation.sourceRows).text} exact state observations into ${formatCount(evidence.preparation.transitionRows).text} transition-risk rows.`
            case 'wideEvents': return `Hirmos converted ${formatCount(evidence.preparation.sourceRows).text} subject records into ${formatCount(evidence.preparation.transitionRows).text} transition-risk rows.`
            default: return assertNever(evidence.preparation)
          }
        })()
        return <>
          <Tiles>
            <MetricTile frame="cell" size="compact" label="States" value={formatCount(evidence.states.length)} />
            <MetricTile frame="cell" size="compact" label="Transitions" value={formatCount(evidence.transitions.length)} />
            <MetricTile frame="cell" size="compact" label="Most likely final state" value={formatStatistic('raw', evidence.states[destination] ?? Number.NaN)} context={formatPercent(row[destination] ?? Number.NaN).text} />
          </Tiles>
          <Interpretation
            bottomLine={<>For observations starting in state {initial}, the chart estimates the chance of occupying each state as follow-up continues.</>}
            uncertainty={<>These state probabilities are fitted point estimates; this result does not yet include simulation intervals.</>}
            mustBeTrue={<>Every permitted move must be represented by the observed transition rows. Transition hazards must follow the chosen proportional-hazards family, and future movement must depend on the state history in the way specified by this clock-forward model.</>}
          />
          {preparation !== null && <div className="mt-3 rounded-lg border border-hair bg-well px-3 py-2.5 text-body text-muted"><p className="m-0">{preparation}</p>{evidence.preparation.kind !== 'preparedRows' && evidence.preparation.notices.map((notice) => <p key={notice} className="mb-0 mt-1">{notice}</p>)}</div>}
          <MultiStateCharts evidence={evidence} initial={initial} />
        </>
      }
      default: return assertNever(run)
    }
  })()

  return (
    <article aria-labelledby={`survival-run-${run.id}`}>
      <details className={`group rounded-xl border bg-panel ${current ? 'border-edge' : 'border-hair'}`} open={open}>
        <summary className="flex cursor-pointer list-none items-start gap-3 rounded-xl py-4 pl-4 pr-4 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
          <Icon name="expand_more" size={16} className="mt-1 shrink-0 text-faint transition-transform duration-(--motion-fast) group-open:rotate-180" />
          <div className="min-w-0 flex-1">
            <span className={label(current ? 'text-signal' : 'text-faint')}>{method}</span>
            <h3 id={`survival-run-${run.id}`} className="mb-1 mt-1 text-title font-medium text-ink">{heading.title}</h3>
            <p className="m-0 text-body text-faint">{heading.meta} · {summary.figure} · <span className={num()}>{formatTime(run.createdAt)}</span></p>
          </div>
          {onDelete !== undefined && (
            <button type="button" className="grid h-8 w-8 shrink-0 place-items-center rounded-lg border border-transparent text-faint transition-colors hover:bg-well hover:text-danger" aria-label={`Delete ${summary.method} run`} title="Delete this run" onClick={(event) => { event.preventDefault(); event.stopPropagation(); onDelete() }}>
              <Icon name="delete" size={16} />
            </button>
          )}
        </summary>
        <div className="px-4 pb-4">{body}</div>
      </details>
    </article>
  )
}
