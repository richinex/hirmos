import { EmptyState } from '@/components/ui/EmptyState'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { useEffect, useMemo, useReducer } from 'react'
import { EChart } from '@/charts/EChart'
import { impactPathOption } from '@/charts/estimation/impactPath'
import { useChartTheme } from '@/charts/theme'
import { EligibilityView } from '@/components/EligibilityView'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { IntervalFigure, MetricTile } from '@/components/ui/figures'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, field, fieldHint, fieldLabel, label, literal, num } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { DagDocument } from '@/domain/dag'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  additive,
  boundsReading,
  causalEstimateFrom,
  defaultConfiguration,
  describeCovariance,
  describeEstimator,
  ESTIMATOR_IDS,
  evaluateEstimatorEligibility,
  interventionStartFromTreatment,
  methodIdOf,
  newEstimationRunId,
  stationaryMarksOf,
  type CausalEstimate,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
  type EstimatorId,
} from '@/domain/estimation'
import { ESTIMATION_METHODS, methodDefinition, type MethodEligibility } from '@/domain/methods'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from '@/domain/preprocessing'
import {
  assessPanelInterventionLayout,
  type PanelInterventionLayout,
  type PanelInterventionPreflight,
  type PanelLongMatrix,
} from '@/domain/panel'
import { estimandSentence, type IdentificationArtifact, type IdentificationId, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatEstimate, formatInterval, formatP, formatStatistic, type Formatted } from '@/lib/format/number'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { lowerFirst } from '@/lib/text'
import { useRunActivity } from '@/lib/useRunActivity'
import type { RunActivity } from '@/domain/activity'
import type { AnalysisProgress } from '@/workers/analysisProtocol'

const PSS_CASES: Record<'c' | 'ct', readonly (2 | 3 | 4 | 5)[]> = { c: [2, 3], ct: [4, 5] }
const TRACE_LEVELS: readonly (90 | 95 | 99)[] = [90, 95, 99]
const VECM_TERMS = [['n', 'None'], ['co', 'Constant outside'], ['ci', 'Constant inside'], ['coli', 'Constant and trend']] as const

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly progress: AnalysisProgress | null }
  | { readonly kind: 'failed'; readonly detail: string }

interface PanelBinding {
  readonly prepared: PreparedDatasetArtifact['id']
  readonly unit: ColumnId
  readonly time: ColumnId
  readonly outcome: ColumnId
  readonly treatment: ColumnId
}

type PanelPreflightJob =
  | { readonly kind: 'not-required' }
  | { readonly kind: 'loading'; readonly binding: PanelBinding }
  | { readonly kind: 'ready'; readonly binding: PanelBinding; readonly matrix: PanelLongMatrix; readonly layout: PanelInterventionLayout }
  | { readonly kind: 'refused'; readonly binding: PanelBinding; readonly problem: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem'] }

interface State {
  readonly identification: IdentificationId | null
  readonly estimator: EstimatorId
  readonly configurations: Readonly<Record<EstimatorId, EstimatorConfiguration>>
  readonly job: Job
  readonly panelPreflight: PanelPreflightJob
}

type Event =
  | { readonly type: 'identification-chosen'; readonly identification: IdentificationId | null; readonly configurations: Readonly<Record<EstimatorId, EstimatorConfiguration>> }
  | { readonly type: 'estimator-chosen'; readonly estimator: EstimatorId }
  | { readonly type: 'configured'; readonly configuration: EstimatorConfiguration }
  | { readonly type: 'run-started' }
  | { readonly type: 'run-progressed'; readonly progress: AnalysisProgress }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }
  | { readonly type: 'panel-preflight-not-required' }
  | { readonly type: 'panel-preflight-started'; readonly binding: PanelBinding }
  | { readonly type: 'panel-preflight-succeeded'; readonly binding: PanelBinding; readonly matrix: PanelLongMatrix; readonly layout: PanelInterventionLayout }
  | { readonly type: 'panel-preflight-refused'; readonly binding: PanelBinding; readonly problem: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem'] }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'identification-chosen': return { ...state, identification: event.identification, configurations: event.configurations, job: { kind: 'idle' } }
    case 'estimator-chosen': return { ...state, estimator: event.estimator, job: { kind: 'idle' } }
    case 'configured': return { ...state, configurations: { ...state.configurations, [event.configuration.kind]: event.configuration }, job: { kind: 'idle' } }
    case 'run-started': return { ...state, job: { kind: 'running', progress: null } }
    case 'run-progressed': return state.job.kind === 'running' ? { ...state, job: { kind: 'running', progress: event.progress } } : state
    case 'run-failed': return { ...state, job: { kind: 'failed', detail: event.detail } }
    case 'run-finished': return { ...state, job: { kind: 'idle' } }
    case 'panel-preflight-not-required': return { ...state, panelPreflight: { kind: 'not-required' } }
    case 'panel-preflight-started': return { ...state, panelPreflight: { kind: 'loading', binding: event.binding } }
    case 'panel-preflight-succeeded': return { ...state, panelPreflight: { kind: 'ready', binding: event.binding, matrix: event.matrix, layout: event.layout } }
    case 'panel-preflight-refused': return { ...state, panelPreflight: { kind: 'refused', binding: event.binding, problem: event.problem } }
    default: return assertNever(event)
  }
}

const IRR = { kind: 'ratio', label: 'IRR' } as const

const scaleOf = (estimate: CausalEstimate) => (estimate.effect.kind === 'incidenceRateRatio' ? IRR : additive)

/** The headline figure of any estimate, for ledgers and comparisons. */
const headline = (estimate: CausalEstimate): Formatted => {
  switch (estimate.effect.kind) {
    case 'additive': return formatEstimate(estimate.effect.value, additive)
    case 'incidenceRateRatio': return formatEstimate(estimate.effect.value, IRR)
    case 'path': return formatEstimate(estimate.effect.aggregate.cumulative, additive)
    default: return assertNever(estimate.effect)
  }
}

const intervalText = (estimate: CausalEstimate): string => {
  if (estimate.interval.kind !== 'confidence') return 'none'
  const value = estimate.effect.kind === 'path' ? estimate.effect.aggregate.cumulative : estimate.effect.value
  const figure = formatInterval(value, estimate.interval.lower, estimate.interval.upper, { kind: 'confidence', level: estimate.interval.level }, scaleOf(estimate))
  return `[${figure.bounds.lower}, ${figure.bounds.upper}]`
}

const text = (value: string): Formatted => ({ text: value, parts: [{ kind: 'digits', text: value }], exact: '', srText: value })

const samePanelBinding = (left: PanelBinding, right: PanelBinding): boolean =>
  left.prepared === right.prepared
  && left.unit === right.unit
  && left.time === right.time
  && left.outcome === right.outcome
  && left.treatment === right.treatment

const eligibilityLabel = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'available'
    case 'caution': return 'review'
    case 'refused': return 'unavailable'
    default: return assertNever(eligibility)
  }
}

const eligibilityTone = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'text-ok'
    case 'caution': return 'text-warn'
    case 'refused': return 'text-danger'
    default: return assertNever(eligibility)
  }
}

function EstimatorOptionLabel({ name, eligibility }: { readonly name: string; readonly eligibility: MethodEligibility }) {
  return <span>{name} <span className={cn('ml-1 text-label', eligibilityTone(eligibility))}>· {eligibilityLabel(eligibility)}</span></span>
}

const panelWeight = (values: readonly number[], index: number): string => formatStatistic('score', values[index] ?? Number.NaN).text

function PanelEvidenceDetails({ run }: { readonly run: Extract<EstimationRunArtifact, { readonly kind: 'panel-intervention-run' }> }) {
  const { evidence } = run
  const controls = evidence.units.slice(0, evidence.controlUnits)
  // A run saved before period labels were recorded falls back to its numeric time codes.
  const timeLabels: readonly string[] = Array.isArray(run.timeLabels) ? run.timeLabels : evidence.times.map(String)
  const preLabels = timeLabels.slice(0, evidence.nPre)
  const postLabels = timeLabels.slice(evidence.nPre)
  const estimates = [
    ['Difference-in-differences', evidence.did],
    ['Synthetic control', evidence.syntheticControl],
    ['Synthetic difference-in-differences', evidence.syntheticDid],
  ] as const
  return (
    <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
      <summary className="cursor-pointer text-ink">Panel weights and period effects</summary>
      <div className="mt-3 grid gap-4">
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel estimator comparison">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Estimator</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Estimate</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Noise level</th></tr></thead>
            <tbody>{estimates.map(([name, estimate]) => <tr key={name}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{name}{name.startsWith('Synthetic difference') ? ' · primary' : ''}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.estimate).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.noiseLevel).text}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel unit weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Control unit</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{controls.map((unit, index) => <tr key={unit}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{unit}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.omega, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel time weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Pre-period</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{preLabels.map((period, index) => <tr key={`${period}-${index}`}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{period}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.lambda, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel post-period effects">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Post-period</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{postLabels.map((period, index) => <tr key={`${period}-${index}`}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{period}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', evidence.did.effectCurve[index] ?? Number.NaN).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', evidence.syntheticControl.effectCurve[index] ?? Number.NaN).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', evidence.syntheticDid.effectCurve[index] ?? Number.NaN).text}</td></tr>)}</tbody>
          </table>
        </div>
      </div>
    </details>
  )
}

function RunRecord({ run }: { readonly run: EstimationRunArtifact }) {
  return (
    <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body">
      <summary className="cursor-pointer text-ink">Run details</summary>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Study</dt><dd className={literal('m-0 break-all')}>{run.study}</dd>
        <dt>Identification</dt><dd className={literal('m-0 break-all')}>{run.identification}</dd>
        <dt>Prepared dataset</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Columns</dt><dd className="m-0">{run.columns.map((column) => column.name).join(', ')}</dd>
        <dt>Configuration</dt><dd className={literal('m-0 break-all')}>{JSON.stringify(run.configuration)}</dd>
        <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        <dt>Method</dt><dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </details>
  )
}

function Diagnostics({ run }: { readonly run: EstimationRunArtifact }) {
  const tiles = ((): readonly { readonly label: string; readonly value: Formatted; readonly context?: string }[] => {
    switch (run.kind) {
      case 'backdoor-linear-run': {
        const { evidence } = run
        return [
          { label: 'R squared', value: formatStatistic('score', evidence.rSquared), context: `${formatCount(evidence.parameters).text} parameters` },
          { label: 'Residual SD', value: formatStatistic('sd', evidence.residualSd), context: `${formatCount(evidence.degreesOfFreedom).text} degrees of freedom · Durbin–Watson ${formatStatistic('raw', evidence.durbinWatson).text}` },
          { label: 'Newey–West bandwidth', value: formatCount(evidence.hacMaxLags, { noun: 'lag' }), context: `heteroskedasticity and autocorrelation consistent p ${formatP(evidence.hacPValue, { withLabel: false }).text}` },
        ]
      }
      case 'count-glm-run': {
        const { evidence } = run
        return [
          { label: 'Coefficient', value: formatStatistic('raw', evidence.coefficient), context: `SE ${formatStatistic('raw', evidence.standardError).text} · p ${formatP(evidence.pValue, { withLabel: false }).text}` },
          evidence.family === 'poisson'
            ? { label: 'Deviance', value: formatStatistic('raw', evidence.deviance ?? Number.NaN), context: `${formatCount(evidence.degreesOfFreedom).text} degrees of freedom` }
            : { label: 'Dispersion alpha', value: formatStatistic('raw', evidence.alpha ?? Number.NaN), context: `log likelihood ${formatStatistic('raw', evidence.logLikelihood ?? Number.NaN).text}` },
          { label: 'Convergence', value: text(evidence.converged ? 'converged' : 'not converged'), context: `${formatCount(evidence.iterations).text} iterations` },
        ]
      }
      case 'ardl-run': {
        const { evidence } = run
        const reading = boundsReading(evidence)
        const [lower, upper] = evidence.boundsCritical[1] ?? [Number.NaN, Number.NaN]
        return [
          { label: 'Bounds test', value: text(reading === 'level-relation' ? 'level relation' : reading === 'no-level-relation' ? 'no level relation' : 'inconclusive'), context: `F ${formatStatistic('raw', evidence.boundsStatistic).text} against 5% bounds ${formatStatistic('raw', lower).text} to ${formatStatistic('raw', upper).text}` },
          { label: 'Bounds p', value: formatP(evidence.boundsPUpper, { withLabel: false }), context: `I(1) bound · I(0) bound p ${formatP(evidence.boundsPLower, { withLabel: false }).text}` },
          { label: 'Lag orders', value: text(`ARDL(${evidence.arLag}, ${evidence.dlLag})`), context: `AIC over ${formatCount(evidence.grid.length).text} candidates · ${evidence.trend === 'ct' ? 'constant and trend' : 'constant'} · case ${evidence.case}` },
        ]
      }
      case 'vecm-run': {
        const { evidence } = run
        return [
          { label: 'Cointegration rank', value: formatCount(evidence.rank), context: `Johansen trace at ${['90', '95', '99'][evidence.significance] ?? ''}% · ${evidence.kArDiff} lagged differences` },
          { label: 'Adjustment p', value: text(evidence.pvaluesAlpha.map((row) => formatP(row[0] ?? Number.NaN, { withLabel: false }).text).join(' · ')), context: `alpha per equation · terms “${evidence.deterministic}”` },
          evidence.chow === null
            ? { label: 'Chow break', value: text('not requested'), context: 'set a break row to test stability' }
            : { label: 'Chow break', value: formatP(evidence.chow[1], { withLabel: false }), context: `F ${formatStatistic('raw', evidence.chow[0]).text} at the chosen row` },
        ]
      }
      case 'synthetic-control-run': {
        const { evidence } = run
        const donorNames = run.columns.slice(2).map((variable) => variable.name)
        return [
          { label: 'Donor weights', value: text(evidence.weights.map((weight, index) => `${donorNames[index] ?? index} ${formatStatistic('score', weight).text}`).join(' · ')), context: `${formatCount(evidence.weights.length).text} donors · sum to one` },
          { label: 'Pre-period loss', value: formatStatistic('raw', evidence.loss), context: `${formatCount(evidence.nPre).text} pre rows · ${formatCount(evidence.iterations).text} active-set steps` },
          { label: 'Average post gap', value: formatStatistic('raw', evidence.att), context: `over ${formatCount(evidence.nPost).text} post rows` },
        ]
      }
      case 'panel-intervention-run': {
        const { evidence } = run
        const topWeights = evidence.syntheticDid.omega
          .map((weight, index) => ({ weight, unit: evidence.units[index] ?? `control ${index + 1}` }))
          .sort((left, right) => right.weight - left.weight)
          .slice(0, 3)
          .map(({ weight, unit }) => `${unit} ${formatStatistic('score', weight).text}`)
          .join(' · ')
        return [
          { label: 'Estimator comparison', value: text(`DID ${formatStatistic('raw', evidence.did.estimate).text} · SC ${formatStatistic('raw', evidence.syntheticControl.estimate).text} · SDID ${formatStatistic('raw', evidence.syntheticDid.estimate).text}`), context: `${formatCount(evidence.nPost).text} post periods` },
          { label: 'Panel layout', value: text(`${evidence.treatedUnits} treated · ${evidence.controlUnits} controls`), context: `${evidence.units.length} units × ${evidence.times.length} periods` },
          { label: 'Largest SDID weights', value: text(topWeights || 'none'), context: `noise level ${formatStatistic('raw', evidence.syntheticDid.noiseLevel).text}` },
        ]
      }
      case 'negbin-nuts-run': {
        const { evidence } = run
        return [
          { label: 'Divergences', value: formatCount(evidence.divergences), context: `acceptance ${formatStatistic('score', evidence.acceptanceRate).text} · step ${formatStatistic('raw', evidence.stepSize).text}` },
          { label: 'Rate ratio per SD', value: formatStatistic('raw', Math.exp(evidence.betaTreatmentMean)), context: `per standard deviation of ${run.columns[0]?.name ?? 'the treatment'} · beta ${formatStatistic('raw', evidence.betaTreatmentMean).text}, posterior sd ${formatStatistic('raw', evidence.betaTreatmentSd).text}` },
          { label: 'Dispersion r', value: formatStatistic('raw', evidence.dispersionMean), context: `${formatCount(evidence.warmup).text} warmup · ${formatCount(evidence.samples).text} draws · seed ${evidence.seed}` },
        ]
      }
      case 'discrete-bn-run': {
        const { evidence } = run
        return [
          { label: 'Treatment bins', value: text(`${evidence.treatmentStates[0]} → ${evidence.treatmentStates[1]}`), context: `${evidence.bins} quantile bins · states ${evidence.stateCounts.join('/')}` },
          { label: 'Expected outcome', value: text(`${formatStatistic('raw', evidence.expectations[0]).text} → ${formatStatistic('raw', evidence.expectations[1]).text}`), context: 'under do(low) and do(high)' },
          ...(evidence.parentsAdjusted.join(', ') === run.estimate.adjustmentSet.map((variable) => variable.name).join(', ') ? [] : [
  { label: 'Adjustment set', value: text(evidence.parentsAdjusted.length === 0 ? 'none' : evidence.parentsAdjusted.join(', ')), context: evidence.minimalAdjustmentSet === null ? 'no minimal adjustment set' : `minimal set ${evidence.minimalAdjustmentSet.length === 0 ? 'empty' : evidence.minimalAdjustmentSet.join(', ')}` },
          ]),
        ]
      }
      case 'double-ml-run': {
        const { evidence } = run
        return [
          { label: 'Model', value: text(evidence.model === 'plr' ? 'partially linear' : evidence.att ? 'interactive · effect on the treated' : 'interactive · average effect'), context: evidence.treatBinary ? 'binary treatment' : 'continuous treatment' },
          { label: 'Standard error', value: formatStatistic('raw', evidence.standardError), context: 'sandwich, cross-fitted' },
          { label: 'Fold seed', value: formatCount(evidence.seed), context: '5 folds · 200 trees · learner seed 7' },
        ]
      }
      case 'causal-effects-run': {
        const { evidence } = run
        const nodeName = (node: readonly [number, number]) => `${run.columns[node[0]]?.name ?? node[0]}${node[1] === 0 ? '' : ` (t${node[1]})`}`
        return [
          { label: 'Adjustment set', value: text(evidence.adjustmentSet.length === 0 ? 'None' : evidence.adjustmentSet.map(nodeName).join(', ')), context: `τ max ${evidence.tauMax}` },
          { label: 'Predictions', value: text(evidence.predictions.map((value) => formatStatistic('raw', value).text).join(' → ')), context: `at ${evidence.interventions[0]} and ${evidence.interventions[1]}` },
          { label: 'Fitted rows', value: formatCount(evidence.fittedObservations), context: evidence.mediators.length === 0 ? 'no mediators' : `${evidence.mediators.length} mediator nodes` },
        ]
      }
      case 'causal-impact-run': {
        const { evidence } = run
        return [
          { label: 'Cumulative effect', value: formatStatistic('raw', evidence.cumulative), context: `over ${formatCount(evidence.nPost).text} post rows` },
          { label: 'Average effect', value: formatStatistic('raw', evidence.average), context: `pre window ${formatCount(evidence.nPre).text} rows` },
          { label: 'Log likelihood', value: formatStatistic('raw', evidence.logLikelihood), context: `${formatCount(evidence.controls.length).text} controls` },
        ]
      }
      default: return assertNever(run)
    }
  })()
  return (
    <div className="mt-4 grid gap-2 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4" aria-label="Diagnostics">
      <MetricTile label="Adjustment set" size="compact" value={text(run.estimate.adjustmentSet.length === 0 ? 'None' : run.estimate.adjustmentSet.map((variable) => variable.name).join(', '))} />
      {tiles.map((tile) => <MetricTile key={tile.label} label={tile.label} size="compact" value={tile.value} context={tile.context} />)}
    </div>
  )
}

function ResultCard({ run, study, current, stepLabel }: { readonly run: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly stepLabel: string }) {
  const theme = useChartTheme()
  const estimate = run.estimate
  const sentence = estimandSentence(study)
  const scaleLine = estimate.effect.kind === 'incidenceRateRatio'
    ? `incidence rate ratio · multiplicative change in ${study.outcome.name} per unit of ${study.treatment.name}`
    : estimate.effect.kind === 'path'
      ? `additive · ${study.outcome.name} minus its counterfactual per ${stepLabel}`
      : `additive · units of ${study.outcome.name} per unit of ${study.treatment.name}`
  const chart = useMemo(() => (estimate.effect.kind === 'path'
    ? impactPathOption({ outcome: study.outcome.name, points: estimate.effect.values, stepLabel }, theme)
    : null), [estimate.effect, stepLabel, study.outcome.name, theme])
  const stamp = `${describeEstimator(run.configuration.kind)}${run.kind === 'backdoor-linear-run' ? ` · ${describeCovariance(run.configuration.covariance)}` : ''} · ${formatTime(run.createdAt)}`
  const body = (
    <>
      <div className="mt-3">
        {estimate.interval.kind === 'confidence' && estimate.effect.kind !== 'path' ? (
          <IntervalFigure
            sentence={sentence}
            estimate={estimate.effect.value}
            lower={estimate.interval.lower}
            upper={estimate.interval.upper}
            type={{ kind: 'confidence', level: estimate.interval.level }}
            scale={scaleOf(estimate)}
            standardError={estimate.standardError ?? undefined}
            observations={estimate.sample.observations}
            scaleLine={scaleLine}
            accent={current}
            testId="effect-estimate"
          />
        ) : (
          <figure className="m-0" data-testid="effect-estimate">
            <figcaption className="text-title text-ink">{sentence}</figcaption>
            <p className={num(`mb-0 mt-1 text-metric font-semibold ${current ? 'text-signal' : 'text-ink'}`)}>{headline(estimate).text}</p>
            <p className={num('mb-0 mt-1 text-body text-bone')}>
              {estimate.effect.kind === 'path' ? `cumulative over ${formatCount(estimate.effect.values.length).text} ${stepLabel}s · average ${formatStatistic('raw', estimate.effect.aggregate.average).text} per ${stepLabel} · ` : ''}
              no interval · n = {formatCount(estimate.sample.observations).text}
            </p>
            <p className="mb-0 mt-1 text-body text-muted">{estimate.interval.kind === 'none' ? estimate.interval.reason : ''}</p>
            <p className={label('mb-0 mt-2 text-muted')}>{scaleLine}</p>
          </figure>
        )}
      </div>
      {chart !== null && (
        <div className="mt-3">
          <EChart option={chart} label={`${study.outcome.name} against its counterfactual after the intervention`} className="h-[260px]" testId="impact-path" />
        </div>
      )}
      {run.kind === 'backdoor-linear-run' && (
        <p className="mb-0 mt-3 text-body text-muted">
          {estimate.interval.kind === 'confidence' && (estimate.interval.lower > 0 || estimate.interval.upper < 0) ? 'The interval excludes zero.' : 'The interval includes zero: the data do not rule out no effect.'}{' '}
          For comparison, the {run.configuration.covariance === 'hac' ? 'classical' : 'HAC'} interval is <span className={num('text-bone')}>{intervalText({ ...estimate, interval: { kind: 'confidence', level: run.evidence.level, lower: run.configuration.covariance === 'hac' ? run.evidence.interval[0] : run.evidence.hacInterval[0], upper: run.configuration.covariance === 'hac' ? run.evidence.interval[1] : run.evidence.hacInterval[1] } })}</span>.
        </p>
      )}
      {run.kind === 'count-glm-run' && !run.evidence.converged && (
        <Alert tone="warn" live={false} className="mt-3"><p className="m-0">The optimiser did not converge; treat the estimate and its interval as provisional.</p></Alert>
      )}
      <Diagnostics run={run} />
      {run.kind === 'panel-intervention-run' && <PanelEvidenceDetails run={run} />}
      <RunRecord run={run} />
    </>
  )
  if (!current) {
    // Earlier runs fold to one line, so the current estimate keeps the page; the ledger below lists every run.
    return (
      <article className="rounded-xl border border-hair bg-panel" aria-label={`${sentence} estimate`}>
        <details className="group">
          <summary className="flex cursor-pointer list-none flex-wrap items-baseline gap-x-3 gap-y-1 rounded-xl p-4 transition-colors hover:bg-well [&::-webkit-details-marker]:hidden">
            <Icon name="expand_more" size={14} className="shrink-0 self-center text-faint transition-transform duration-150 group-open:rotate-180" />
            <span className={label('text-faint')}>Earlier run</span>
            <span className="min-w-0 text-body text-ink">{sentence}</span>
            <span className={num('text-body font-medium text-ink')}>{headline(estimate).text}</span>
            <span className={num('ml-auto text-micro text-faint')}>{stamp}</span>
          </summary>
          <div className="px-4 pb-4">{body}</div>
        </details>
      </article>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${sentence} estimate`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label(current ? 'text-signal' : 'text-faint')}>{current ? 'Current estimate' : 'Earlier run'}</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {body}
    </article>
  )
}

export function EstimationPanel({ source, profile, prepared, stationarity, documents, studies, identifications, runs, onRun, onOpenStudy, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly runs: readonly EstimationRunArtifact[]
  readonly onRun: (run: EstimationRunArtifact) => void
  readonly onOpenStudy: () => void
}) {
  const identified = identifications.filter((identification) => identification.result.kind === 'identified')
  const latestStudy = studies.find((candidate) => candidate.id === identified.at(-1)?.study) ?? null
  const [state, dispatch] = useReducer(step, null, (): State => ({
    identification: identified.at(-1)?.id ?? null,
    estimator: prepared.kind === 'prepared-panel' ? 'panel-intervention' : 'backdoor-linear-regression',
    configurations: Object.fromEntries(ESTIMATOR_IDS.map((estimator) => [estimator, defaultConfiguration(estimator, prepared, latestStudy)])) as Record<EstimatorId, EstimatorConfiguration>,
    job: { kind: 'idle' },
    panelPreflight: { kind: 'not-required' },
  }))
  const identification = identified.find((candidate) => candidate.id === state.identification) ?? null
  const study = identification === null ? null : studies.find((candidate) => candidate.id === identification.study) ?? null
  const document = study === null ? null : documents.find((candidate) => candidate.id === study.dagDocument) ?? null
  const configuration = state.configurations[state.estimator]
  const method = methodDefinition(methodIdOf(state.estimator))
  const stepLabel = prepared.kind === 'prepared-time-series' ? 'observation' : prepared.kind === 'prepared-panel' ? 'Panel' : 'row'
  const panelBinding = useMemo<PanelBinding | null>(() => prepared.kind === 'prepared-panel' && study !== null
    ? { prepared: prepared.id, unit: prepared.sampling.unitColumn, time: prepared.sampling.timeColumn, outcome: study.outcome.column, treatment: study.treatment.column }
    : null, [prepared, study])
  const panelPreflight = useMemo<PanelInterventionPreflight>(() => {
    if (panelBinding === null) return { kind: 'not-applicable' }
    const job = state.panelPreflight
    if (job.kind === 'not-required' || !samePanelBinding(job.binding, panelBinding)) return { kind: 'pending' }
    switch (job.kind) {
      case 'loading': return { kind: 'pending' }
      case 'ready': return { kind: 'ready', layout: job.layout }
      case 'refused': return { kind: 'refused', problem: job.problem }
      default: return assertNever(job)
    }
  }, [panelBinding, state.panelPreflight])

  useEffect(() => {
    if (panelBinding === null || prepared.kind !== 'prepared-panel') {
      dispatch({ type: 'panel-preflight-not-required' })
      return
    }
    let cancelled = false
    dispatch({ type: 'panel-preflight-started', binding: panelBinding })
    void (async () => {
      const { materializePanelInWorker } = await import('@/data/client')
      const matrix = await materializePanelInWorker(source.file, profile, {
        unit: panelBinding.unit,
        time: panelBinding.time,
        outcome: panelBinding.outcome,
        treatment: panelBinding.treatment,
      })
      if (cancelled) return
      if (!matrix.ok) {
        dispatch({ type: 'panel-preflight-refused', binding: panelBinding, problem: { kind: 'panel-data', problem: matrix.error } })
        return
      }
      const layout = assessPanelInterventionLayout(matrix.value)
      if (!layout.ok) {
        dispatch({ type: 'panel-preflight-refused', binding: panelBinding, problem: { kind: 'panel-layout', problem: layout.error } })
        return
      }
      dispatch({ type: 'panel-preflight-succeeded', binding: panelBinding, matrix: matrix.value, layout: layout.value })
    })()
    return () => { cancelled = true }
  }, [panelBinding, prepared.kind, profile, source.file])

  const eligibilityByEstimator = useMemo<ReadonlyMap<EstimatorId, MethodEligibility>>(() => {
    const evaluations = new Map<EstimatorId, MethodEligibility>()
    if (identification === null) return evaluations
    for (const estimator of ESTIMATOR_IDS) {
      const definition = methodDefinition(methodIdOf(estimator))
      if (!definition.ok) continue
      evaluations.set(estimator, evaluateEstimatorEligibility(definition.value, {
        identification: identification.result,
        prepared,
        stationarity,
        configuration: state.configurations[estimator],
        outcomeIsCount: null,
        treatmentIsBinary: null,
        document,
        study,
        panelPreflight,
      }))
    }
    return evaluations
  }, [document, identification, panelPreflight, prepared, state.configurations, stationarity, study])
  const eligibility = eligibilityByEstimator.get(state.estimator) ?? null
  const configure = (next: EstimatorConfiguration) => dispatch({ type: 'configured', configuration: next })

  useRunActivity(onActivity, state.job.kind === 'running' ? { label: describeEstimator(state.estimator), progress: state.job.progress === null ? null : state.job.progress.completed / Math.max(1, state.job.progress.total) } : null)
  const execute = async () => {
    if (identification === null || study === null || eligibility === null || eligibility.kind === 'refused' || state.job.kind === 'running' || !method.ok) return
    if (identification.result.kind !== 'identified') return
    dispatch({ type: 'run-started' })
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      const materialise = async (columns: NonEmptyArray<StudyVariable>) => {
        const matrix = await materialisePrepared(source, profile, prepared, columns.map((column) => column.column) as unknown as NonEmptyArray<ColumnId>)
        if (!matrix.ok) throw new Error(describePreparedMaterialisationProblem(matrix.error))
        return matrix.value
      }
      const columnAt = (values: Float64Array, rows: number, index: number): number[] => Array.from(values.subarray(index * rows, (index + 1) * rows))
      const identity = { id: newEstimationRunId(), study: study.id, identification: identification.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), eligibility } as const
      const finish = (run: EstimationRunArtifact | null, detail: string) => {
        if (run === null) { dispatch({ type: 'run-failed', detail }); return }
        onRun(run)
        dispatch({ type: 'run-finished' })
      }
      switch (configuration.kind) {
        case 'backdoor-linear-regression': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const evidence = await analysis.runBackdoorLinear(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: identification.result.adjustment.variables.map((_, index) => index + 2), hacMaxLags: null, level: configuration.level })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'backdoor-linear-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind) as typeof run extends never ? never : 'backdoor-linear-regression' extends string ? ReturnType<typeof methodIdOf> & typeof method.value.id : never, columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'dml-plr':
        case 'dml-irm': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          if (configuration.kind === 'dml-irm' && columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} is not binary; the interactive model needs a 0/1 treatment.` }); return }
          const evidence = await analysis.runDoubleMl(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: identification.result.adjustment.variables.map((_, index) => index + 2), model: configuration.kind === 'dml-plr' ? 'plr' : 'irm', att: configuration.kind === 'dml-irm' && configuration.att, seed: configuration.seed })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'double-ml-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'ardl-pss': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome]
          const matrix = await materialise(columns)
          const evidence = await analysis.runArdlPss(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, maxLag: configuration.maxLag, trend: configuration.trend, case: configuration.case })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'ardl-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'vecm': {
          const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const evidence = await analysis.runVecm(matrix.values, matrix.rowCount, columns.length, { endogenous: columns.map((_, index) => index), maxLags: configuration.maxLags, deterministic: configuration.deterministic, significance: configuration.significance === 90 ? 0 : configuration.significance === 95 ? 1 : 2, breakIndex: configuration.breakIndex })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'vecm-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact,
            evidence.value.rank === 0 ? 'The Johansen trace test found no cointegrating relation, so no long-run effect exists to report.' : `The trace test found ${evidence.value.rank} cointegrating relations; a single long-run effect is read only at rank one.`)
          return
        }
        case 'synthetic-control': {
          const donors = prepared.columns.filter((column) => configuration.donors.includes(column) && column !== study.outcome.column && column !== study.treatment.column)
          const donorVariables: StudyVariable[] = donors.map((column) => ({ node: study.outcome.node, column, name: profile.columns.find((candidate) => candidate.id === column)?.name ?? String(column) }))
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...donorVariables]
          const matrix = await materialise(columns)
          let nPre: number
          if (configuration.start.kind === 'row') nPre = configuration.start.row - 1
          else {
            const start = interventionStartFromTreatment(columnAt(matrix.values, matrix.rowCount, 0))
            if (!start.ok) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} does not switch on once: ${start.error.detail} Give the intervention row instead.` }); return }
            nPre = start.value
          }
          const evidence = await analysis.runSyntheticControl(matrix.values, matrix.rowCount, columns.length, { treated: 1, donors: donorVariables.map((_, index) => index + 2), nPre })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'synthetic-control-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The synthetic control run produced no post-intervention rows.')
          return
        }
        case 'panel-intervention': {
          if (prepared.kind !== 'prepared-panel') { dispatch({ type: 'run-failed', detail: 'Prepare a balanced long panel before running the panel intervention estimator.' }); return }
          if (panelBinding === null || state.panelPreflight.kind !== 'ready' || !samePanelBinding(state.panelPreflight.binding, panelBinding)) {
            dispatch({ type: 'run-failed', detail: 'The panel treatment-layout check has not completed for this study.' })
            return
          }
          const matrix = state.panelPreflight.matrix
          const evidence = await analysis.runPanelIntervention(
            matrix.values.slice(),
            matrix.rowCount,
            matrix.units,
            matrix.times,
            (progress) => dispatch({ type: 'run-progressed', progress }),
          )
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment]
          const run = { kind: 'panel-intervention-run', configuration, evidence: evidence.value, timeLabels: state.panelPreflight.layout.periodLabels } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The panel intervention run produced no estimate.')
          return
        }
        case 'negbin-nuts': {
          const confounder = identification.result.adjustment.variables[0]
          if (confounder === undefined || identification.result.adjustment.variables.length !== 1) { dispatch({ type: 'run-failed', detail: 'The ported model takes exactly one confounder.' }); return }
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, confounder]
          const matrix = await materialise(columns)
          const outcome = columnAt(matrix.values, matrix.rowCount, 1)
          if (outcome.some((value) => value < 0 || !Number.isInteger(value))) { dispatch({ type: 'run-failed', detail: `${study.outcome.name} is not a count: it holds negative or fractional values.` }); return }
          const evidence = await analysis.runNegbinNuts(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, confounder: 2, warmup: configuration.warmup, samples: configuration.samples, seed: configuration.seed })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'negbin-nuts-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'discrete-bn-query': {
          const measured = study.graph.nodes.flatMap((node) => (node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name }]))
          if (measured.length !== study.graph.nodes.length) { dispatch({ type: 'run-failed', detail: 'Every DAG node must be measured for the network.' }); return }
          const columns = measured as unknown as NonEmptyArray<StudyVariable>
          const matrix = await materialise(columns)
          const position = (node: StudyVariable['node']) => measured.findIndex((candidate) => candidate.node === node)
          const evidence = await analysis.runDiscreteBnQuery(matrix.values, matrix.rowCount, columns.length, {
            nodes: measured.map((_, index) => index),
            names: measured.map((variable) => variable.name),
            edges: study.graph.edges.map(([cause, effect]) => [position(study.graph.nodes[cause]?.node ?? study.treatment.node), position(study.graph.nodes[effect]?.node ?? study.outcome.node)] as const),
            treatment: position(study.treatment.node),
            outcome: position(study.outcome.node),
            bins: configuration.bins,
            equivalentSampleSize: configuration.equivalentSampleSize,
          })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'discrete-bn-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'poisson-glm':
        case 'negative-binomial-p': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const outcome = columnAt(matrix.values, matrix.rowCount, 1)
          if (outcome.some((value) => value < 0 || !Number.isInteger(value))) { dispatch({ type: 'run-failed', detail: `${study.outcome.name} is not a count: it holds negative or fractional values.` }); return }
          const evidence = await analysis.runCountGlm(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: identification.result.adjustment.variables.map((_, index) => index + 2), family: configuration.kind === 'poisson-glm' ? 'poisson' : 'negativeBinomial' })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'count-glm-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'causal-effects-total': {
          if (document === null) { dispatch({ type: 'run-failed', detail: 'The study’s DAG document is missing.' }); return }
          const nodes = document.current.graph.nodes
          const observed = nodes.flatMap((node) => (node.kind === 'observed' ? [{ node: node.id, column: node.column, name: node.name }] : []))
          if (observed.length === 0) { dispatch({ type: 'run-failed', detail: 'The DAG has no observed variables.' }); return }
          const matrix = await materialise(observed as unknown as NonEmptyArray<StudyVariable>)
          const rows = matrix.rowCount
          const values = new Float64Array(rows * nodes.length)
          nodes.forEach((node, position) => {
            if (node.kind !== 'observed') return
            const source = observed.findIndex((candidate) => candidate.node === node.id)
            values.set(matrix.values.subarray(source * rows, (source + 1) * rows), position * rows)
          })
          const marks = stationaryMarksOf(document)
          const treatmentIndex = nodes.findIndex((node) => node.id === study.treatment.node)
          const outcomeIndex = nodes.findIndex((node) => node.id === study.outcome.node)
          const lags = marks.statLag + configuration.treatmentLag
          const evidence = await analysis.runCausalEffectsTotal(values, rows, nodes.length, {
            statLag: marks.statLag,
            graph: marks.marks,
            x: [[treatmentIndex, -configuration.treatmentLag]],
            y: [[outcomeIndex, 0]],
            hidden: marks.hidden.flatMap((variable) => Array.from({ length: lags + 1 }, (_, lag) => [variable, -lag] as const)),
            estimator: configuration.estimator,
            interventions: configuration.interventions,
          })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'causal-effects-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          const columns = nodes.map((node) => (node.kind === 'observed' ? { node: node.id, column: node.column, name: node.name } : { node: node.id, column: study.treatment.column, name: `${node.name} (unmeasured)` })) as unknown as NonEmptyArray<StudyVariable>
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact,
            evidence.value.noCausalPath ? 'The time-series graph has no causal path from the treatment to the outcome.' : 'The effect is not identifiable by adjustment in the projected time-series graph.')
          return
        }
        case 'causal-impact': {
          const controls = prepared.columns.filter((column) => configuration.controls.includes(column) && column !== study.outcome.column && column !== study.treatment.column)
          const controlVariables: StudyVariable[] = controls.map((column) => ({ node: study.outcome.node, column, name: profile.columns.find((candidate) => candidate.id === column)?.name ?? String(column) }))
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...controlVariables]
          const matrix = await materialise(columns)
          let nPre: number
          if (configuration.start.kind === 'row') nPre = configuration.start.row - 1
          else {
            const start = interventionStartFromTreatment(columnAt(matrix.values, matrix.rowCount, 0))
            if (!start.ok) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} does not switch on once: ${start.error.detail} Give the intervention row instead.` }); return }
            nPre = start.value
          }
          const evidence = await analysis.runCausalImpact(matrix.values, matrix.rowCount, columns.length, { outcome: 1, controls: controlVariables.map((_, index) => index + 2), nPre, maxIter: configuration.maxIter })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: evidence.error.detail }); return }
          const run = { kind: 'causal-impact-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The impact run produced no post-intervention rows.')
          return
        }
        default:
          return assertNever(configuration)
      }
    } catch (cause: unknown) {
      dispatch({ type: 'run-failed', detail: cause instanceof Error ? cause.message : String(cause) })
    }
  }

  const latestRun = runs.at(-1) ?? null
  const controlCandidates = study === null ? [] : profile.columns.filter((column) => prepared.columns.includes(column.id) && column.id !== study.outcome.column && column.id !== study.treatment.column)

  const controls = ((): React.ReactNode => {
    switch (configuration.kind) {
      case 'backdoor-linear-regression':
        return (
          <div>
            <span className={fieldLabel}>Interval</span>
            <SegmentedControl className="mt-1" ariaLabel="Interval covariance" value={configuration.covariance} onChange={(covariance) => configure({ ...configuration, covariance })} options={[{ value: 'hac', label: 'Newey–West HAC' }, { value: 'classical', label: 'Classical' }]} />
            <p className={cn(fieldHint, 'max-w-[65ch]')}>95% confidence level. Heteroskedasticity and autocorrelation consistent (HAC) covariance uses a Bartlett kernel and the statsmodels default bandwidth. The classical interval assumes independent errors.</p>
          </div>
        )
      case 'dml-plr':
      case 'dml-irm':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            {configuration.kind === 'dml-irm' && (
              <div>
                <span className={fieldLabel}>Study target</span>
                <p className="mb-0 mt-1 text-body text-ink">{configuration.att ? 'Effect on the treated (ATT)' : 'Average treatment effect (ATE)'}</p>
                <p className={cn(fieldHint, 'mt-1')}>Change the target in Study design, not in the estimator.</p>
              </div>
            )}
            <label className="block"><span className={fieldLabel}>Fold seed</span><input type="number" min={0} aria-label="Fold seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className="m-0 max-w-[65ch] self-end text-body text-faint @md/panel:col-span-2">Five shuffled folds, 200 random-forest trees, minimum leaf 5, learner seed 7. The Sensitivity chapter repeats this fit at the same seed before its refuters.</p>
          </div>
        )
      case 'ardl-pss':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><span className={fieldLabel}>Maximum lag</span><input type="number" min={1} max={24} aria-label="Maximum lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => configure({ ...configuration, maxLag: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
            <div>
              <span className={fieldLabel}>Deterministic terms</span>
              <SegmentedControl className="mt-1" ariaLabel="Deterministic terms" value={configuration.trend} onChange={(trend) => configure({ ...configuration, trend, case: trend === 'c' ? 3 : 4 })} options={[{ value: 'c', label: 'Constant' }, { value: 'ct', label: 'Constant and trend' }]} />
            </div>
            <div>
              <span className={fieldLabel}>PSS case</span>
              <SegmentedControl className="mt-1" fill ariaLabel="PSS case" value={String(configuration.case)} onChange={(chosen) => { const candidate = PSS_CASES[configuration.trend].find((item) => String(item) === chosen); if (candidate !== undefined) configure({ ...configuration, case: candidate }) }} options={PSS_CASES[configuration.trend].map((candidate) => ({ value: String(candidate), label: `Case ${candidate}` }))} />
            </div>
            <p className="m-0 max-w-[65ch] self-end text-body text-faint">AIC lag search, error-correction fit, delta-method interval on the long-run effect, bounds test at the recorded case.</p>
          </div>
        )
      case 'vecm':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><span className={fieldLabel}>Maximum lags</span><input type="number" min={1} max={24} aria-label="Maximum lags" className={field('text', 'mt-1')} value={configuration.maxLags} onChange={(event) => configure({ ...configuration, maxLags: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
            <div>
              <span className={fieldLabel}>Deterministic terms</span>
              <Select className={field('text', 'mt-1')} aria-label="VECM deterministic terms" value={configuration.deterministic} onChange={(event) => { const term = VECM_TERMS.find(([value]) => value === event.target.value); if (term !== undefined) configure({ ...configuration, deterministic: term[0] }) }}>
                {VECM_TERMS.map(([value, name]) => <option key={value} value={value}>{name}</option>)}
              </Select>
            </div>
            <div>
              <span className={fieldLabel}>Trace significance</span>
              <SegmentedControl className="mt-1" fill ariaLabel="Trace significance" value={String(configuration.significance)} onChange={(chosen) => { const level = TRACE_LEVELS.find((item) => String(item) === chosen); if (level !== undefined) configure({ ...configuration, significance: level }) }} options={TRACE_LEVELS.map((level) => ({ value: String(level), label: `${level}%` }))} />
            </div>
            <label className="block"><span className={fieldLabel}>Chow break row</span><input type="number" min={4} max={prepared.observations - 4} aria-label="Chow break row" placeholder="none" className={field('text', 'mt-1')} value={configuration.breakIndex ?? ''} onChange={(event) => configure({ ...configuration, breakIndex: event.target.value === '' ? null : Math.max(4, Math.min(prepared.observations - 4, Math.floor(Number(event.target.value) || 4))) })} /></label>
          </div>
        )
      case 'synthetic-control':
        return (
          <div className="grid gap-3">
            <div>
              <span className={fieldLabel}>Intervention start</span>
              <div className="mt-1 flex flex-wrap items-center gap-2">
                <SegmentedControl ariaLabel="Synthetic intervention start" value={configuration.start.kind} onChange={(kind) => configure({ ...configuration, start: kind === 'from-treatment' ? { kind: 'from-treatment' } : { kind: 'row', row: Math.max(3, Math.floor(prepared.observations / 2)) } })} options={[{ value: 'from-treatment', label: `Where ${study?.treatment.name ?? 'the treatment'} turns on` }, { value: 'row', label: 'At a row' }]} />
                {configuration.start.kind === 'row' && (
                  <label className="text-body text-ink">First post-intervention row<input type="number" min={3} max={prepared.observations} aria-label="First post-intervention row" className={field('text', 'ml-2 w-28')} value={configuration.start.row} onChange={(event) => configure({ ...configuration, start: { kind: 'row', row: Math.max(3, Math.min(prepared.observations, Number(event.target.value) || 3)) } })} /></label>
                )}
              </div>
            </div>
            <div>
              <span className={fieldLabel}>Donor series</span>
              <div className="mt-1 flex flex-wrap gap-2" role="group" aria-label="Donor series">
                {controlCandidates.length === 0 && <span className="text-body text-faint">No other prepared columns to use as donors.</span>}
                {controlCandidates.map((column) => (
                  <label key={column.id} className="flex items-center gap-1.5 text-body text-ink">
                    <input type="checkbox" checked={configuration.donors.includes(column.id)} onChange={(event) => configure({ ...configuration, donors: event.target.checked ? [...configuration.donors, column.id] : configuration.donors.filter((candidate) => candidate !== column.id) })} />
                    {column.name}
                  </label>
                ))}
              </div>
            </div>
          </div>
        )
      case 'panel-intervention':
        return (
          <div className="grid gap-2">
            <p className="m-0 max-w-[72ch] text-body text-faint">
              The primary estimator is predeclared as synthetic difference-in-differences. Conventional DID and synthetic control are reported as required comparisons; the primary result cannot be changed after viewing the estimates.
            </p>
            {panelPreflight.kind === 'pending' && <p className="m-0 text-body text-muted">Checking treatment timing, treated and control units, pre/post periods, and control pre-period variation…</p>}
            {panelPreflight.kind === 'ready' && <p className="m-0 text-body text-muted">Ready: {panelPreflight.layout.treated.length} treated and {panelPreflight.layout.controls.length} control units · {panelPreflight.layout.prePeriods} pre- and {panelPreflight.layout.postPeriods} post-periods · adoption at {panelPreflight.layout.adoptionLabel}.</p>}
          </div>
        )
      case 'negbin-nuts':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><span className={fieldLabel}>Warmup</span><input type="number" min={10} max={5000} aria-label="Warmup" className={field('text', 'mt-1')} value={configuration.warmup} onChange={(event) => configure({ ...configuration, warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><span className={fieldLabel}>Draws</span><input type="number" min={10} max={5000} aria-label="Draws" className={field('text', 'mt-1')} value={configuration.samples} onChange={(event) => configure({ ...configuration, samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><span className={fieldLabel}>Seed</span><input type="number" min={0} aria-label="Sampler seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className="m-0 self-end text-body text-faint">Gamma-Poisson likelihood, standardised treatment and confounder, NUTS with step-size and diagonal mass adaptation at target acceptance 0.8.</p>
          </div>
        )
      case 'discrete-bn-query':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><span className={fieldLabel}>Quantile bins</span><input type="number" min={2} max={10} aria-label="Quantile bins" className={field('text', 'mt-1')} value={configuration.bins} onChange={(event) => configure({ ...configuration, bins: Math.max(2, Math.min(10, Math.floor(Number(event.target.value) || 2))) })} /></label>
            <label className="block"><span className={fieldLabel}>Equivalent sample size</span><input type="number" min={0.1} step="any" aria-label="Equivalent sample size" className={field('text', 'mt-1')} value={configuration.equivalentSampleSize} onChange={(event) => configure({ ...configuration, equivalentSampleSize: Math.max(0.1, Number(event.target.value) || 0.1) })} /></label>
            <p className="m-0 max-w-[65ch] self-end text-body text-faint @md/panel:col-span-2">Every DAG node is cut into quantile bins; the BDeu prior smooths the conditional tables; the effect contrasts the highest and lowest treatment bins.</p>
          </div>
        )
      case 'poisson-glm':
      case 'negative-binomial-p':
        return <p className="m-0 text-body text-faint">Log link on the expected count of {study?.outcome.name ?? 'the outcome'}; the treatment coefficient exponentiates to an incidence rate ratio with a 95% normal interval.</p>
      case 'causal-effects-total':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <div>
              <span className={fieldLabel}>First-stage estimator</span>
              <SegmentedControl className="mt-1" ariaLabel="First-stage estimator" value={configuration.estimator.kind} onChange={(kind) => configure({ ...configuration, estimator: kind === 'linear' ? { kind: 'linear' } : { kind: 'knn', k: 15 } })} options={[{ value: 'linear', label: 'Linear' }, { value: 'knn', label: 'k-nearest neighbours' }]} />
            </div>
            {configuration.estimator.kind === 'knn' && (
              <label className="block"><span className={fieldLabel}>Neighbours k</span><input type="number" min={1} max={100} className={field('text', 'mt-1')} value={configuration.estimator.k} onChange={(event) => configure({ ...configuration, estimator: { kind: 'knn', k: Math.max(1, Math.min(100, Number(event.target.value) || 1)) } })} /></label>
            )}
            <label className="block"><span className={fieldLabel}>Treatment lag</span><input type="number" min={0} max={20} className={field('text', 'mt-1')} value={configuration.treatmentLag} onChange={(event) => configure({ ...configuration, treatmentLag: Math.max(0, Math.min(20, Number(event.target.value) || 0)) })} /></label>
            <div className="grid grid-cols-2 gap-2">
              <label className="block"><span className={fieldLabel}>From value</span><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.interventions[0]} onChange={(event) => configure({ ...configuration, interventions: [Number(event.target.value) || 0, configuration.interventions[1]] })} /></label>
              <label className="block"><span className={fieldLabel}>To value</span><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.interventions[1]} onChange={(event) => configure({ ...configuration, interventions: [configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
            </div>
          </div>
        )
      case 'causal-impact':
        return (
          <div className="grid gap-3">
            <div>
              <span className={fieldLabel}>Intervention start</span>
              <div className="mt-1 flex flex-wrap items-center gap-2">
                <SegmentedControl ariaLabel="Intervention start" value={configuration.start.kind} onChange={(kind) => configure({ ...configuration, start: kind === 'from-treatment' ? { kind: 'from-treatment' } : { kind: 'row', row: Math.max(9, Math.floor(prepared.observations / 2)) } })} options={[{ value: 'from-treatment', label: `Where ${study?.treatment.name ?? 'the treatment'} turns on` }, { value: 'row', label: 'At a row' }]} />
                {configuration.start.kind === 'row' && (
                  <label className="text-body text-ink">First post-intervention row<input type="number" min={9} max={prepared.observations} aria-label="First post-intervention row" className={field('text', 'ml-2 w-28')} value={configuration.start.row} onChange={(event) => configure({ ...configuration, start: { kind: 'row', row: Math.max(9, Math.min(prepared.observations, Number(event.target.value) || 9)) } })} /></label>
                )}
              </div>
            </div>
            <div>
              <span className={fieldLabel}>Control series</span>
              <div className="mt-1 flex flex-wrap gap-2" role="group" aria-label="Control series">
                {controlCandidates.length === 0 && <span className="text-body text-faint">No other prepared columns to use as controls.</span>}
                {controlCandidates.map((column) => (
                  <label key={column.id} className="flex items-center gap-1.5 text-body text-ink">
                    <input type="checkbox" checked={configuration.controls.includes(column.id)} onChange={(event) => configure({ ...configuration, controls: event.target.checked ? [...configuration.controls, column.id] : configuration.controls.filter((candidate) => candidate !== column.id) })} />
                    {column.name}
                  </label>
                ))}
              </div>
            </div>
          </div>
        )
      default:
        return assertNever(configuration)
    }
  })()

  const stage = (
    <section aria-labelledby="estimation-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-signal')}>06 · Estimation</span>
        <h2 id="estimation-title" className="mb-2 mt-2 text-heading text-ink">Estimate the specified causal effect</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">Choose an estimator that meets the identified study's design, sampling and variable requirements. Review the method requirements before interpreting the estimate.</p>
      </div>

      <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="estimation-setup-title">
        <h3 id="estimation-setup-title" className="mb-3 mt-0 text-title font-medium text-ink">{method.ok ? method.value.name : 'Estimator'}</h3>
        {identified.length === 0 ? (
          <Alert tone="info" live={false}>
            <p className="m-0"><button type="button" className="underline" onClick={onOpenStudy}>Open Study design</button> and identify a study first.</p>
          </Alert>
        ) : (
          <>
            <div className="grid gap-4">
              <label className="block">
                <span className={fieldLabel}>Identified study</span>
                <Select className={field('text', 'mt-1')} value={state.identification ?? ''} onChange={(event) => { const chosen = event.target.value === '' ? null : (event.target.value as IdentificationId); const chosenStudy = studies.find((candidate) => candidate.id === identifications.find((identification) => identification.id === chosen)?.study) ?? null; dispatch({ type: 'identification-chosen', identification: chosen, configurations: Object.fromEntries(ESTIMATOR_IDS.map((estimator) => [estimator, defaultConfiguration(estimator, prepared, chosenStudy)])) as Record<EstimatorId, EstimatorConfiguration> }) }}>
                {identified.map((candidate) => {
                  const bound = studies.find((item) => item.id === candidate.study)
                  return <option key={candidate.id} value={candidate.id}>{bound === undefined ? candidate.id : `${estimandSentence(bound)} · ${bound.dagName}`}</option>
                })}
              </Select>
                {identification !== null && identification.result.kind === 'identified' && (
                  <span className={cn(fieldHint, 'block max-w-[65ch]')}>Adjustment set: {identification.result.adjustment.variables.length === 0 ? 'none' : identification.result.adjustment.variables.map((variable) => variable.name).join(', ')} · {formatCount(study?.population.observations ?? 0).text} rows</span>
                )}
              </label>
              <div>
                <span className={fieldLabel}>Estimator</span>
                <SegmentedControl
                  className="mt-1"
                  ariaLabel="Estimator"
                  value={state.estimator}
                  onChange={(estimator) => dispatch({ type: 'estimator-chosen', estimator })}
                  options={ESTIMATOR_IDS.flatMap((id) => {
                    const definition = methodDefinition(methodIdOf(id))
                    const candidateEligibility = eligibilityByEstimator.get(id)
                    return definition.ok && candidateEligibility !== undefined
                      ? [{ value: id, label: <EstimatorOptionLabel name={definition.value.name} eligibility={candidateEligibility} />, disabled: candidateEligibility.kind === 'refused', title: `${definition.value.name}: ${eligibilityLabel(candidateEligibility)}` }]
                      : []
                  })}
                />
                {method.ok && <p className={cn(fieldHint, 'max-w-[65ch]')}>{method.value.summary}</p>}
              </div>
              <div>{controls}</div>
            </div>
            {eligibility !== null && <EligibilityView eligibility={eligibility} subject="this study" />}
            {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The estimate could not run: {state.job.detail}</p></Alert>}
            <button type="button" className={button('signal', 'mt-4')} disabled={identification === null || eligibility === null || eligibility.kind === 'refused' || state.job.kind === 'running' || (configuration.kind === 'panel-intervention' && panelPreflight.kind !== 'ready')} onClick={() => void execute()}>
              {state.job.kind === 'running' ? 'Estimating…' : configuration.kind === 'panel-intervention' && panelPreflight.kind === 'pending' ? 'Checking panel…' : `Run ${lowerFirst(describeEstimator(state.estimator))}`}
            </button>
          </>
        )}
      </section>

      {runs.length > 0 && (
        <section aria-labelledby="estimation-results-title" className="grid gap-4">
          <div>
            <span className={label('text-faint')}>Recorded results</span>
            <h2 id="estimation-results-title" className="mb-0 mt-1 text-title font-medium text-ink">Estimates</h2>
          </div>
          {[...runs].reverse().map((run) => {
            const bound = studies.find((candidate) => candidate.id === run.study)
            return bound === undefined ? null : <ResultCard key={run.id} run={run} study={bound} current={latestRun?.id === run.id} stepLabel={stepLabel} />
          })}
        </section>
      )}
      {runs.length === 0 && identified.length > 0 && (
        <EmptyState>Choose an estimator and run it against the identified study.</EmptyState>
      )}
    </section>
  )

  const inspector = (
    <div className="space-y-4">
      <section aria-labelledby="estimation-study-title">
        <h3 id="estimation-study-title" className="mb-2 mt-1 text-body font-medium text-ink">{study === null ? 'No study chosen' : estimandSentence(study)}</h3>
        {study !== null && identification !== null && identification.result.kind === 'identified' && (
          <>
            <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body" aria-label="Study binding">
              <dt className="text-faint">Treatment</dt><dd className="m-0 text-ink">{study.treatment.name}</dd>
              <dt className="text-faint">Outcome</dt><dd className="m-0 text-ink">{study.outcome.name}</dd>
              <dt className="text-faint">Graph</dt><dd className="m-0 text-ink">{study.dagName} · <span className={literal()}>{study.dagRevision.slice(0, 8)}</span></dd>
              <dt className="text-faint">Strategy</dt><dd className="m-0 text-ink">Back-door adjustment</dd>
              <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{prepared.kind === 'prepared-time-series' ? 'Time series' : prepared.kind === 'prepared-panel' ? 'Panel' : 'Independent'} · {formatCount(prepared.observations).text}</dd>
            </dl>
          </>
        )}
      </section>
      <MethodCaveats
        methods={method.ok ? [method.value] : ESTIMATION_METHODS}
        eligibility={eligibility}
        identification={identification?.result ?? null}
      />
    </div>
  )

  const ledger = (
    <table className="w-full border-collapse text-body" aria-label="Estimation runs">
      <thead>
        <tr className="text-left">
          <th scope="col" className={label('px-3 py-1.5 font-normal text-muted')}>Estimand</th>
          <th scope="col" className={label('px-3 py-1.5 font-normal text-muted')}>Estimator</th>
          <th scope="col" className={label('px-3 py-1.5 text-right font-normal text-muted')}>Estimate</th>
          <th scope="col" className={label('px-3 py-1.5 text-right font-normal text-muted')}>95% confidence interval</th>
          <th scope="col" className={label('px-3 py-1.5 font-normal text-muted')}>Created</th>
        </tr>
      </thead>
      <tbody>
        {runs.length === 0 && <tr><td colSpan={5} className="px-3 py-2 text-faint">Choose an estimator and run it.</td></tr>}
        {[...runs].reverse().map((run) => {
          const bound = studies.find((candidate) => candidate.id === run.study)
          return (
            <tr key={run.id} className="border-t border-hair">
              <td className="px-3 py-1.5 text-ink">{bound === undefined ? run.study : estimandSentence(bound)}</td>
              <td className="px-3 py-1.5 text-muted">{describeEstimator(run.configuration.kind)}{run.kind === 'backdoor-linear-run' ? ` · ${describeCovariance(run.configuration.covariance)}` : ''}</td>
              <td className={num('whitespace-nowrap px-3 py-1.5 text-right text-ink')}>{headline(run.estimate).text}</td>
              <td className={num('whitespace-nowrap px-3 py-1.5 text-right text-bone')}>{intervalText(run.estimate)}</td>
              <td className={num('whitespace-nowrap px-3 py-1.5 text-faint')}>{formatTime(run.createdAt)}</td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )

  return (
    <WorkbenchLayout
      id="estimation"
      stage={stage}
      inspector={{ title: 'Study and method requirements', body: inspector }}
      bottom={{ title: `Runs · ${runs.length}`, body: ledger, defaultSize: 150 }}
    />
  )
}
