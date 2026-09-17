import { Metadata } from '@/components/ui/Metadata'
import { Alert } from '@/components/ui/Alert'
import { useState, type ReactNode } from 'react'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { comparisonMeasureOption, hazardCurveOption, observedSurvivalOption, restrictedMeanOption, stateOccupancyOption, survivalCurvesOption, transitionMapOption, transitionMatrixOption } from '@/charts/survival/curves'
import { ratioForestHeight, ratioForestOption, type RatioEstimate } from '@/charts/survival/estimates'
import { useChartTheme } from '@/charts/theme'
import type { VisibleWindow } from '@/charts/window'
import { Icon } from '@/components/Icon'
import { EvidenceTable, type EvidenceColumn, type EvidenceValue } from '@/components/table/EvidenceTable'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { SegmentedControl, type SegmentOption } from '@/components/ui/SegmentedControl'
import { caption, iconControl, label, num } from '@/components/ui/recipes'
import { assertNever } from '@/domain/dop'
import type { ComparisonSurvivalEvidence, ConversionDifference, ConversionRate, CoxRegressionEvidence, MultiStateSurvivalEvidence, NonparametricSurvivalEvidence, ParametricSurvivalFamily, SurvivalRunArtifact } from '@/domain/survival'
import { formatCount, formatEstimate, formatP, formatPercent, formatStatistic } from '@/lib/format/number'
import { formatTime } from '@/lib/format/date'
import { SurvivalInterpretation as Interpretation } from './SurvivalInterpretation'
import { AalenResult, ForestResult } from './SurvivalRegressionResult'

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
    case 'cox-regression-run': return 'Cox regression'
    case 'aalen-run': return 'Aalen regression'
    case 'survival-forest-run': return 'Survival forest'
    case 'penalized-aft-run': return 'Penalised AFT'
    case 'two-group-survival-run': return 'Two-group comparison'
    case 'multi-state-survival-run': return 'Multi-state'
    default: return assertNever(run)
  }
}

export const penalizedAftFamilyLabel = (family: 'weibull' | 'logLogistic'): string => family === 'weibull' ? 'Weibull AFT' : 'Log-logistic AFT'

/** The method and the headline figure of a run, for the ledger row. */
export const survivalRunSummary = (run: SurvivalRunArtifact): { readonly method: string; readonly figure: string } => {
  switch (run.kind) {
    case 'right-censored-survival-run':
    case 'start-stop-survival-run': return { method: `${survivalRunLabel(run)}, ${survivalFamilyLabel(run.evidence.family)}`, figure: `median ${formatStatistic('raw', run.evidence.median).text}` }
    case 'cox-regression-run': return { method: survivalRunLabel(run), figure: `hazard ratio ${formatStatistic('raw', run.evidence.coefficients[0]?.hazardRatio ?? Number.NaN).text}` }
    case 'aalen-run': return { method: survivalRunLabel(run), figure: `fitted through ${statistic(run.evidence.lastTime)}` }
    case 'survival-forest-run': return { method: survivalRunLabel(run), figure: run.evidence.concordance.kind === 'recorded' ? `out-of-bag concordance ${statistic(run.evidence.concordance.result)}` : 'out-of-bag concordance unavailable' }
    case 'penalized-aft-run': return { method: `${survivalRunLabel(run)}, ${penalizedAftFamilyLabel(run.evidence.family)}`, figure: `time ratio ${formatStatistic('raw', run.evidence.coefficients[0]?.timeRatio ?? Number.NaN).text}` }
    case 'nonparametric-survival-run': return { method: survivalRunLabel(run), figure: `event-free ${formatPercent(run.evidence.survival.at(-1) ?? Number.NaN).text}` }
    case 'two-group-survival-run': return { method: survivalRunLabel(run), figure: `event-free time difference ${formatStatistic('raw', run.evidence.restrictedMeanDifference).text}` }
    case 'multi-state-survival-run': return { method: `${survivalRunLabel(run)}, ${survivalFamilyLabel(run.evidence.family)}`, figure: `${formatCount(run.evidence.transitions.length).text} transitions` }
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

interface CoxCoefficientRow {
  readonly key: string
  readonly covariate: string
  readonly coefficient: number
  readonly standardError: number
  readonly hazardRatio: number
  readonly hazardRatioInterval: readonly [number, number]
  readonly z: number
  readonly pValue: number
}

interface AftRow {
  readonly key: string
  readonly parameter: string
  readonly role: string
  readonly coefficient: number
  readonly standardError: number
  readonly timeRatio: number
  readonly timeRatioInterval: readonly [number, number]
  readonly z: number
  readonly pValue: number
}

interface CoxAssumptionRow {
  readonly key: string
  readonly covariate: string
  readonly transform: string
  readonly statistic: number
  readonly pValue: number
}

const parameterColumns = (ratioLabel: string): readonly EvidenceColumn<ParameterRow>[] => [
  { id: 'parameter', header: 'Parameter', value: (row) => row.parameter },
  { id: 'role', header: 'Role', value: (row) => row.role },
  figureColumn<ParameterRow>('estimate', 'Estimate', (row) => row.estimate),
  { id: 'interval', header: '95% interval', align: 'right', value: (row) => interval(row.interval) },
  { id: 'ratio', header: ratioLabel, align: 'right', value: (row) => row.ratio === null ? '' : row.ratio, format: (value) => value === '' ? '—' : statistic(asNumber(value)) },
  { id: 'profile', header: 'Drawn at', align: 'right', value: (row) => row.profile === null ? '' : row.profile, format: (value) => value === '' ? '—' : statistic(asNumber(value)) },
]

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
  const rows = [
    { group: 'Group 0', counts: times.map((time) => atRiskAt(diagnostics.groupZero.atRisk, time)) },
    { group: 'Group 1', counts: times.map((time) => atRiskAt(diagnostics.groupOne.atRisk, time)) },
  ]
  const columns: readonly EvidenceColumn<typeof rows[number]>[] = [
    { id: 'group', header: 'Group', value: (row) => row.group },
    ...times.map((time, index): EvidenceColumn<typeof rows[number]> => ({
      id: `time-${index}`, header: statistic(time), align: 'right',
      value: (row) => row.counts[index], format: (_, row) => formatCount(row.counts[index]).text,
    })),
  ]
  return (
    <div className="mt-2">
      <EvidenceTable title="Number at risk" rows={rows} columns={columns} rowKey={(row) => row.group} noun="group" empty="No risk counts." frame="none" exportName="survival-number-at-risk" />
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
      <SegmentedControl value={chart} onChange={setChart} options={COMPARISON_CHARTS} ariaLabel="Survival comparison chart" size="sm" variant="line" />
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
      <SegmentedControl value={chart} onChange={setChart} options={MULTI_STATE_CHARTS} ariaLabel="Multi-state chart" size="sm" variant="line" />
      <ExpandableChart className="mt-3 h-[280px]" label={view.label} testId={view.testId} option={view.option} />
    </div>
  )
}

/** The covariate ratios of one fit as a forest plot, under the table that lists them. */
function RatioForest({ rows, ratioName, confidence, testId }: { readonly rows: readonly RatioEstimate[]; readonly ratioName: 'hazard ratio' | 'time ratio'; readonly confidence: number; readonly testId: string }) {
  const theme = useChartTheme()
  if (rows.length === 0) return null
  const unchanged = ratioName === 'hazard ratio' ? 'the event rate' : 'the time to the event'
  return (
    <div className="mt-3">
      <p className={label('m-0 mb-2 text-muted')}>{ratioName === 'hazard ratio' ? 'Hazard ratios' : 'Time ratios'} with {confidence}% intervals</p>
      <ExpandableChart className="" style={{ height: ratioForestHeight(rows.length) }} label={`${ratioName === 'hazard ratio' ? 'Hazard' : 'Time'} ratio forest plot`} testId={testId} option={ratioForestOption(rows, ratioName, confidence, theme)} />
      <p className={caption('mb-0 mt-2')}>Each square is a covariate's {ratioName} and the line through it is the {confidence}% interval, on a log scale with the largest ratio at the top. The dashed rule at 1 is where the covariate leaves {unchanged} unchanged; an interval that crosses it is compatible with no change.</p>
    </div>
  )
}

function Tiles({ children }: { readonly children: ReactNode }) {
  return <MetricGrid className="mt-4" testId="survival-summary-cards">{children}</MetricGrid>
}

const coxStandardErrorLabel = (method: CoxRegressionEvidence['standardErrors']): string => {
  switch (method) {
    case 'modelBased': return 'Model-based'
    case 'robust': return 'Robust'
    case 'clustered': return 'Clustered'
    default: return assertNever(method)
  }
}

const coxTimeTransformLabel = (transform: Extract<CoxRegressionEvidence['proportionalHazardsTests'], { readonly kind: 'recorded' }>['transforms'][number]['transform']): string => {
  switch (transform) {
    case 'eventRank': return 'Event rank'
    case 'kaplanMeier': return 'Kaplan–Meier'
    case 'identity': return 'Time'
    case 'logTime': return 'Log time'
    default: return assertNever(transform)
  }
}

const coxBaselineSeries = (baseline: CoxRegressionEvidence['baseline']) => {
  switch (baseline.kind) {
    case 'shared': return [{ name: 'baseline', points: baseline.estimates.map(({ time, survival }) => [time, survival] as const) }]
    case 'stratified': return baseline.curves.map(({ stratum, estimates }) => ({ name: `stratum ${stratum}`, points: estimates.map(({ time, survival }) => [time, survival] as const) }))
    default: return assertNever(baseline)
  }
}

const hazardRatioIntervalPosition = (interval: readonly [number, number]): string => {
  if (interval[1] < 1) return 'is entirely below 1'
  if (interval[0] > 1) return 'is entirely above 1'
  return 'includes 1'
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
      case 'aalen-run': return { title: 'Aalen additive regression', meta: <Metadata><span>{formatCount(run.evidence.observations).text} observations</span><span>{formatCount(run.evidence.events).text} events</span></Metadata> }
      case 'survival-forest-run': return { title: 'Random survival forest', meta: <Metadata><span>{formatCount(run.evidence.observations).text} observations</span><span>{formatCount(run.evidence.trees).text} trees</span></Metadata> }
      case 'right-censored-survival-run':
        return { title: survivalFamilyLabel(run.evidence.family), meta: <Metadata><span>{formatCount(run.evidence.observations).text} observations</span><span>{formatCount(run.evidence.events).text} events</span></Metadata> }
      case 'start-stop-survival-run':
        return { title: survivalFamilyLabel(run.evidence.family), meta: <Metadata><span>{formatCount(run.evidence.observations).text} intervals</span><span>{formatCount(run.evidence.events).text} events</span></Metadata> }
      case 'penalized-aft-run':
        return { title: `${penalizedAftFamilyLabel(run.evidence.family)} with an L2 penalty`, meta: <Metadata><span>{formatCount(run.evidence.observations).text} observations</span><span>{formatCount(run.evidence.events).text} events</span><span>penalizer {statistic(run.evidence.penalizer)}</span></Metadata> }
      case 'cox-regression-run':
        return { title: 'Cox proportional-hazards model', meta: <Metadata><span>{formatCount(run.evidence.observations).text} {run.evidence.observation.kind === 'startStop' ? 'intervals' : 'observations'}</span><span>{formatCount(run.evidence.events).text} events</span></Metadata> }
      case 'nonparametric-survival-run': return { title: 'Kaplan–Meier and Nelson–Aalen', meta: <Metadata><span>{formatCount(run.evidence.observations).text} observations</span><span>{formatCount(run.evidence.events).text} events</span></Metadata> }
      case 'two-group-survival-run': return { title: 'Group 1 compared with group 0', meta: <Metadata><span>{formatCount(run.evidence.observations).text} rows</span><span>compared through time {statistic(run.evidence.truncationTime)}</span></Metadata> }
      case 'multi-state-survival-run': return { title: `${survivalFamilyLabel(run.evidence.family)} transition model`, meta: <Metadata><span>{formatCount(run.evidence.observations).text} transition rows</span><span>{formatCount(run.evidence.states.length).text} states</span></Metadata> }
      default: return assertNever(run)
    }
  })()
  const method = (() => {
    switch (run.kind) {
      case 'aalen-run': return 'Aalen additive regression'
      case 'survival-forest-run': return 'Random survival forest'
      case 'right-censored-survival-run': return 'Parametric survival'
      case 'nonparametric-survival-run': return 'Nonparametric survival'
      case 'start-stop-survival-run': return 'Start–stop survival'
      case 'cox-regression-run': return 'Cox regression'
      case 'penalized-aft-run': return 'Penalised accelerated failure time'
      case 'two-group-survival-run': return 'Two-group survival comparison'
      case 'multi-state-survival-run': return 'Multi-state survival'
      default: return assertNever(run)
    }
  })()

  const body = (() => {
    switch (run.kind) {
      case 'aalen-run': return <AalenResult run={run} />
      case 'survival-forest-run': return <ForestResult run={run} />
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
          <Interpretation run={run}
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
          <Interpretation run={run}
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
      case 'cox-regression-run': {
        const evidence = run.evidence
        const confidence = Math.round(run.configuration.confidenceLevel * 100)
        const coefficientRows: CoxCoefficientRow[] = run.configuration.covariates.map((covariate, index) => {
          const estimate = evidence.coefficients[index]!
          return {
            key: covariate.id,
            covariate: covariate.name,
            coefficient: estimate.coefficient,
            standardError: estimate.standardError,
            hazardRatio: estimate.hazardRatio,
            hazardRatioInterval: estimate.hazardRatioInterval,
            z: estimate.z,
            pValue: estimate.pValue,
          }
        })
        const first = coefficientRows[0]!
        const rateChange = Math.abs(first.hazardRatio - 1)
        const direction = first.hazardRatio < 1 ? 'lower' : 'higher'
        const intervalPosition = hazardRatioIntervalPosition(first.hazardRatioInterval)
        const assumptionRows: CoxAssumptionRow[] = evidence.proportionalHazardsTests.kind === 'recorded'
          ? evidence.proportionalHazardsTests.transforms.flatMap(({ transform, tests }) => tests.map((test, index) => ({
              key: `${transform}-${index}`,
              covariate: run.configuration.covariates[index]?.name ?? `Covariate ${index + 1}`,
              transform: coxTimeTransformLabel(transform),
              statistic: test.statistic,
              pValue: test.pValue,
            })))
          : []
        const frailty = evidence.frailty
        const frailtyGroup = run.configuration.observation.kind === 'right-censored' && run.configuration.observation.frailty.kind === 'gamma'
          ? run.configuration.observation.frailty.group.name
          : 'group'
        return <>
          {evidence.fitting.kind === 'clusteredBreslow' && <>
            <Metadata><span>Breslow ties</span><span>{formatCount(evidence.fitting.clusters).text} clusters</span><span>Clustered standard errors</span></Metadata>
            {evidence.fitting.convergence === 'iterationLimit' && <Alert tone="warn" title="Iteration limit reached">The fit did not converge. Do not rely on these estimates or intervals.</Alert>}
            {evidence.fitting.convergence === 'convergedDuringHalving' && <Alert tone="warn">The fit stopped during step halving. Check the stability of the estimates.</Alert>}
          </>}
          <Tiles>
            <MetricTile frame="cell" size="compact" label={`${first.covariate} hazard ratio`} value={formatStatistic('raw', first.hazardRatio)} context={`${formatPercent(rateChange, { precision: 1 }).text} ${direction} event rate`} />
            <MetricTile frame="cell" size="compact" label={`${confidence}% interval`} value={formatStatistic('raw', first.hazardRatioInterval[0])} context={`to ${statistic(first.hazardRatioInterval[1])}`} />
            <MetricTile frame="cell" size="compact" label="Concordance" value={formatStatistic('raw', evidence.concordance.kind === 'recorded' ? evidence.concordance.result : Number.NaN)} context={evidence.concordance.kind === 'recorded' ? coxStandardErrorLabel(evidence.standardErrors) : 'unavailable for this observation form'} />
          </Tiles>
          <Interpretation run={run}
            bottomLine={<>With the other selected covariates held fixed, a 1-unit higher {first.covariate} is associated with a {formatPercent(rateChange, { precision: 1 }).text} {direction} event rate at each follow-up time. Its estimated hazard ratio is {statistic(first.hazardRatio)}.</>}
            uncertainty={<>The {confidence}% interval for this hazard ratio is {statistic(first.hazardRatioInterval[0])} to {statistic(first.hazardRatioInterval[1])}. The interval {intervalPosition}; a hazard ratio of 1 means the fitted event rate does not change with the covariate.</>}
            mustBeTrue={<>{frailty.kind === 'gamma' ? <>Each {frailtyGroup} carries one unobserved multiplier on its hazard, drawn from a gamma distribution with mean 1, and the covariate effects are the same within every {frailtyGroup}. </> : null}The event-rate ratio must remain constant over follow-up, after accounting for the selected covariates{run.configuration.strata.kind === 'column' ? ' within each stratum' : ''}. Censoring must not depend on an unrecorded reason that also predicts the event. These coefficients describe associations unless a separate causal design supports an effect interpretation.</>}
          />
          {frailty.kind === 'gamma' && <div className="mt-4">
            <p className={label('m-0 mb-2 text-muted')}>Shared gamma frailty by {frailtyGroup}</p>
            <Tiles>
              <MetricTile frame="cell" size="compact" label="Variance of the frailty" value={formatStatistic('raw', frailty.theta)} context={`${formatCount(frailty.groups).text} groups`} />
              <MetricTile frame="cell" size="compact" label="Frailty term" value={formatStatistic('raw', frailty.termTest.statistic)} context={<Metadata><span>chi-squared on {statistic(frailty.termTest.df)} df</span><span>{formatP(frailty.termTest.pValue).text}</span></Metadata>} />
              <MetricTile frame="cell" size="compact" label="Effective degrees of freedom" value={formatStatistic('raw', frailty.degreesOfFreedom)} context={<Metadata><span>{formatCount(frailty.outerIterations).text} outer and {formatCount(frailty.innerIterations).text} Newton iterations</span><span>{frailty.ties === 'breslow' ? 'Breslow' : 'Efron'} ties</span></Metadata>} />
            </Tiles>
            <p className={caption('mb-0 mt-2')}>The variance is the theta of the gamma frailty, chosen by the "em" search of survival's frailty.gamma on the corrected likelihood. The frailty term's statistic is the sum of squared frailty coefficients over their variances, tested on the term's effective degrees of freedom (Therneau, Grambsch and Pankratz 2003). The coefficient standard errors are from the penalised information matrix; the likelihood-ratio test uses the effective degrees of freedom of the whole model.</p>
          </div>}
          <div className="mt-4">
            <EvidenceTable<CoxCoefficientRow>
              frame="none"
              title="Covariate estimates"
              rows={coefficientRows}
              rowKey={(row) => row.key}
              noun="covariate"
              empty="The fit reported no covariate estimate."
              exportName="cox-regression-coefficients"
              columns={[
                { id: 'covariate', header: 'Covariate', value: (row) => row.covariate },
                figureColumn<CoxCoefficientRow>('coefficient', 'Coefficient', (row) => row.coefficient),
                figureColumn<CoxCoefficientRow>('standard-error', 'Standard error', (row) => row.standardError),
                figureColumn<CoxCoefficientRow>('hazard-ratio', 'Hazard ratio', (row) => row.hazardRatio),
                { id: 'hazard-ratio-interval', header: `${confidence}% interval`, align: 'right', value: (row) => `${statistic(row.hazardRatioInterval[0])} to ${statistic(row.hazardRatioInterval[1])}` },
                figureColumn<CoxCoefficientRow>('z', 'z', (row) => row.z),
                figureColumn<CoxCoefficientRow>('p', 'p', (row) => row.pValue, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
            <p className={caption('mb-0 mt-2')}>A hazard ratio above 1 corresponds to a higher fitted event rate; a value below 1 corresponds to a lower fitted event rate. Each comparison is per 1-unit increase in that covariate, with the other selected covariates held fixed.</p>
          </div>
          <RatioForest rows={coefficientRows.map((row) => ({ label: row.covariate, ratio: row.hazardRatio, interval: row.hazardRatioInterval, pValue: row.pValue }))} ratioName="hazard ratio" confidence={confidence} testId="cox-forest" />
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>{evidence.baseline.kind === 'shared' ? 'Baseline event-free probability' : 'Baseline event-free probability by stratum'}</p>
            <ExpandableChart className="h-[260px]" label="Cox baseline event-free probability" testId="cox-baseline-survival" option={survivalCurvesOption(coxBaselineSeries(evidence.baseline), 'follow-up time', theme)} />
            <p className={caption('mb-0 mt-2')}>{evidence.fitting.kind === 'clusteredBreslow' ? 'The baseline curve uses the recorded centring values: zero for columns containing only −1, 0 or 1, and the mean for other columns.' : 'The baseline curves use the fitted covariate means.'} They are not unadjusted Kaplan–Meier curves.</p>
          </div>
          {assumptionRows.length > 0 && <div className="mt-4">
            <EvidenceTable<CoxAssumptionRow>
              frame="none"
              title="Proportional-hazards checks"
              rows={assumptionRows}
              rowKey={(row) => row.key}
              noun="check"
              empty="No proportional-hazards check was recorded."
              exportName="cox-proportional-hazards-checks"
              columns={[
                { id: 'covariate', header: 'Covariate', value: (row) => row.covariate },
                { id: 'transform', header: 'Time scale', value: (row) => row.transform },
                figureColumn<CoxAssumptionRow>('statistic', 'Statistic', (row) => row.statistic),
                figureColumn<CoxAssumptionRow>('p', 'p', (row) => row.pValue, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
          </div>}
        </>
      }
      case 'penalized-aft-run': {
        const evidence = run.evidence
        const confidence = Math.round(run.configuration.confidenceLevel * 100)
        const rows: AftRow[] = [
          ...run.configuration.covariates.map((covariate, index) => {
            const estimate = evidence.coefficients[index]!
            return { key: covariate.id, parameter: covariate.name, role: 'covariate', coefficient: estimate.coefficient, standardError: estimate.standardError, timeRatio: estimate.timeRatio, timeRatioInterval: estimate.timeRatioInterval, z: estimate.z, pValue: estimate.pValue }
          }),
          { key: 'intercept', parameter: 'Intercept', role: 'location', coefficient: evidence.intercept.coefficient, standardError: evidence.intercept.standardError, timeRatio: evidence.intercept.timeRatio, timeRatioInterval: evidence.intercept.timeRatioInterval, z: evidence.intercept.z, pValue: evidence.intercept.pValue },
          { key: 'ancillary', parameter: evidence.family === 'weibull' ? 'rho (shape)' : 'beta (shape)', role: 'ancillary', coefficient: evidence.ancillary.coefficient, standardError: evidence.ancillary.standardError, timeRatio: evidence.ancillary.timeRatio, timeRatioInterval: evidence.ancillary.timeRatioInterval, z: evidence.ancillary.z, pValue: evidence.ancillary.pValue },
        ]
        const first = rows[0]!
        const change = Math.abs(first.timeRatio - 1)
        const direction = first.timeRatio < 1 ? 'shorter' : 'longer'
        const lastTime = evidence.predictionTimes.at(-1) ?? Number.NaN
        const lastSurvival = evidence.survival.at(-1) ?? Number.NaN
        return <>
          <Tiles>
            <MetricTile frame="cell" size="compact" label={`${first.parameter} time ratio`} value={formatStatistic('raw', first.timeRatio)} context={`${formatPercent(change, { precision: 1 }).text} ${direction} time to the event`} />
            <MetricTile frame="cell" size="compact" label="AIC" value={formatStatistic('raw', evidence.aic)} context={<Metadata><span>BIC {statistic(evidence.bic)}</span><span>log likelihood {statistic(evidence.logLikelihood)}</span></Metadata>} />
            <MetricTile frame="cell" size="compact" label="Concordance" value={formatStatistic('raw', evidence.concordance.kind === 'recorded' ? evidence.concordance.result : Number.NaN)} context={evidence.concordance.kind === 'recorded' ? 'on the predicted medians' : 'unavailable'} />
          </Tiles>
          <Interpretation run={run}
            bottomLine={<>With the other covariates held fixed, a 1-unit higher {first.parameter} multiplies the expected time to the event by {statistic(first.timeRatio)}, a {formatPercent(change, { precision: 1 }).text} {direction} time. At the covariate means the fitted median time is {statistic(evidence.median)} and the event-free probability at {statistic(lastTime)} is {formatPercent(lastSurvival).text}.</>}
            uncertainty={<>The {confidence}% interval for this time ratio is {statistic(first.timeRatioInterval[0])} to {statistic(first.timeRatioInterval[1])}. These intervals are calculated for the penalised model. The penalty of {statistic(evidence.penalizer)} shrinks the coefficients toward zero, so the log likelihood, AIC and BIC are those of the penalised fit.</>}
            mustBeTrue={<>The log of the duration must follow the {penalizedAftFamilyLabel(evidence.family)} family, shifted by the covariates through the location parameter. Censoring must not depend on an unrecorded reason that also predicts the event. These time ratios describe associations unless a separate causal design supports an effect interpretation.</>}
          />
          <div className="mt-4">
            <EvidenceTable<AftRow>
              frame="none"
              title="Parameter estimates"
              rows={rows}
              rowKey={(row) => row.key}
              noun="parameter"
              empty="The fit reported no estimate."
              exportName="penalized-aft-estimates"
              columns={[
                { id: 'parameter', header: 'Parameter', value: (row) => row.parameter },
                { id: 'role', header: 'Role', value: (row) => row.role },
                figureColumn<AftRow>('coefficient', 'Coefficient', (row) => row.coefficient),
                figureColumn<AftRow>('standard-error', 'Standard error', (row) => row.standardError),
                figureColumn<AftRow>('time-ratio', 'Time ratio', (row) => row.timeRatio),
                { id: 'time-ratio-interval', header: `${confidence}% interval`, align: 'right', value: (row) => `${statistic(row.timeRatioInterval[0])} to ${statistic(row.timeRatioInterval[1])}` },
                figureColumn<AftRow>('z', 'z', (row) => row.z),
                figureColumn<AftRow>('p', 'p', (row) => row.pValue, (value) => formatP(value, { withLabel: false }).text),
              ]}
            />
            <p className={caption('mb-0 mt-2')}>A time ratio above 1 means a longer time to the event per 1-unit increase in the covariate; below 1, a shorter one. The intercept row is the location parameter at zero covariates and the shape row is the ancillary parameter, both on the log scale in the coefficient column.</p>
          </div>
          <RatioForest rows={rows.filter((row) => row.role === 'covariate').map((row) => ({ label: row.parameter, ratio: row.timeRatio, interval: row.timeRatioInterval, pValue: row.pValue }))} ratioName="time ratio" confidence={confidence} testId="aft-forest" />
          <div className="mt-3">
            <p className={label('m-0 mb-2 text-muted')}>Fitted event-free probability at the covariate means</p>
            <ExpandableChart className="h-[260px]" label="AFT event-free probability" testId="aft-survival" option={survivalCurvesOption([{ name: penalizedAftFamilyLabel(evidence.family), points: evidence.predictionTimes.map((time, index) => [time, evidence.survival[index] ?? Number.NaN] as const) }], 'follow-up time', theme)} />
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
            <Interpretation run={run}
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
            <Interpretation run={run}
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
            <MetricTile frame="cell" size="compact" label="Most likely final state" value={formatCount(evidence.states[destination] ?? Number.NaN)} context={formatPercent(row[destination] ?? Number.NaN).text} />
          </Tiles>
          <Interpretation run={run}
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
      <details className={`group ${current ? 'rounded-xl border border-edge bg-panel' : ''}`} open={open}>
        <summary className={`flex cursor-pointer list-none items-start gap-3 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden ${current ? 'rounded-xl p-4' : 'py-1.5'}`}>
          <Icon name="expand_more" size={16} className="mt-1 shrink-0 text-faint transition-transform duration-(--motion-fast) group-open:rotate-180" />
          {current ? <div className="min-w-0 flex-1">
            <span className={label(current ? 'text-signal' : 'text-faint')}>{method}</span>
            <h3 id={`survival-run-${run.id}`} className="mb-1 mt-1 text-heading font-medium text-ink text-balance">{heading.title}</h3>
            <p className="m-0 text-body text-faint"><Metadata><span>{heading.meta}</span><span>{summary.figure}</span><span><span className={num()}>{formatTime(run.createdAt)}</span></span></Metadata></p>
          </div> : <div className="flex min-w-0 flex-1 flex-wrap items-baseline gap-x-4 gap-y-1">
            <h3 id={`survival-run-${run.id}`} className={caption('m-0 text-ink')}>{survivalRunLabel(run)}</h3>
            <span className={num('text-label text-ink')}>{summary.figure}</span>
            <time dateTime={run.createdAt} className={num('ml-auto text-label text-faint')}>{formatTime(run.createdAt)}</time>
          </div>}
          {onDelete !== undefined && (
            <button type="button" className={iconControl('danger')} aria-label={`Delete ${summary.method} run`} title="Delete this run" onClick={(event) => { event.preventDefault(); event.stopPropagation(); onDelete() }}>
              <Icon name="delete" size={16} />
            </button>
          )}
        </summary>
        <div className="px-4 pb-4">{body}</div>
      </details>
    </article>
  )
}
