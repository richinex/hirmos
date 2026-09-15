import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { TLearnerUncertainty } from './TLearnerUncertainty'
import { TLearnerIntervals } from './TLearnerIntervals'
import { Orb } from '@/components/ui/Orb'
import { EmptyState } from '@/components/ui/EmptyState'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { LagListField } from '@/components/ui/LagListField'
import { useEffect, useMemo, useReducer, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { ExpandableChart } from '@/charts/ExpandableChart'
import { histogramOption } from '@/charts/data/histogram'
import { counterfactualCurvesOption } from '@/charts/estimation/counterfactualCurves'
import { impactPathOption } from '@/charts/estimation/impactPath'
import { posteriorDensityOption } from '@/charts/estimation/posteriorDensity'
import { runComparisonOption, type RunComparisonRow } from '@/charts/estimation/runComparison'
import { useChartTheme } from '@/charts/theme'
import { EligibilityView } from '@/components/EligibilityView'
import { MethodCaveats } from '@/components/MethodCaveats'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { Formula } from '@/components/ui/Formula'
import { RunFold } from '@/components/ui/RunFold'
import { RunMeta } from '@/components/ui/RunMeta'
import { FigureParts, MetricGrid, MetricTile } from '@/components/ui/figures'
import { EstimateHeadline, headlineFigure, scaleOf } from '@/components/results/EstimateHeadline'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { RadioList } from '@/components/ui/RadioList'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { ParameterLabel } from '@/components/ui/ParameterLabel'
import { button, chapterIntro, field, fieldHint, fieldLabel, label, literal, num, panel, prose, sectionTitle, well } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'
import type { DagDocument } from '@/domain/dag'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, mapNonEmpty, type NonEmptyArray } from '@/domain/dop'
import {
  additive,
  adjustmentLabels,
  boundsReading,
  causalEstimateFrom,
  contemporaneousAdjustmentVariables,
  defaultConfiguration,
  defaultEstimatorFor,
  describeCovariance,
  describeDiscreteStatePreparations,
  describeEstimator,
  describeInstrumentalVariableRoute, dmlNuisanceInputs,
  ESTIMATOR_GROUPS,
  ESTIMATOR_IDS,
  evaluateEstimatorEligibility,
  headlineValue,
  intervalTypeOf,
  interventionStartFromTreatment,
  methodIdOf,
  newEstimationRunId,
  stationaryMarksOf,
  summariseRowEffects,
  tLearnerInputs,
  type CausalEffectsAdjustment,
  type CausalEffectsAdjustmentProblem,
  type CausalEstimate,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
  type EstimatorGroupId,
  type EstimatorId,
  type TotalEffectEstimator,
} from '@/domain/estimation'
import { ESTIMATION_METHODS, methodDefinition, type MethodEligibility } from '@/domain/methods'
import { describeSeriesTransform, frequencyUnit, seriesTransformFor, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import {
  assessPanelInterventionLayout,
  type PanelInterventionLayout,
  type PanelInterventionPreflight,
  type PanelLongMatrix,
} from '@/domain/panel'
import { describeIdentificationStrategy, estimableIdentification, estimandSentence, identifiedInstruments, type IdentificationArtifact, type IdentificationId, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatEstimate, formatInterval, formatP, formatPercent, formatStatistic, formatWords, type Formatted } from '@/lib/format/number'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { lowerFirst } from '@/lib/text'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretEstimationResult, resultHeadline, resultSampleLine, resultScaleLine } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { describeAnalysisWorkerProblem, type AnalysisProgress, type DmlGroupsRequest } from '@/workers/analysisProtocol'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'

const PSS_CASES: Record<'c' | 'ct', readonly (2 | 3 | 4 | 5)[]> = { c: [2, 3], ct: [4, 5] }
const TRACE_LEVELS: readonly (90 | 95 | 99)[] = [90, 95, 99]
const VECM_TERMS = [['n', 'None'], ['co', 'Constant outside'], ['ci', 'Constant inside'], ['coli', 'Constant and trend']] as const

const ESTIMATOR_GROUP_LABELS: Readonly<Record<EstimatorGroupId, string>> = {
  'adjusted-outcome': 'Adjustment',
  'identified-functional': 'Identified',
  'graph-adjusted-temporal': 'Temporal graph',
  'dynamic-time-series': 'Count intervention',
  'intervention-comparison': 'Interventions',
}

const estimatorGroupFor = (estimator: EstimatorId) =>
  ESTIMATOR_GROUPS.find((group) => group.estimators.includes(estimator)) ?? ESTIMATOR_GROUPS[0]

const adjustmentFromEstimator = (estimator: TotalEffectEstimator): CausalEffectsAdjustment => estimator.kind === 'wrightParents'
  ? { kind: 'optimal' }
  : estimator.adjustment

const totalEffectEstimatorFromKind = (kind: TotalEffectEstimator['kind'], current: TotalEffectEstimator): TotalEffectEstimator => {
  const adjustment = adjustmentFromEstimator(current)
  switch (kind) {
    case 'linear': return { kind: 'linear', adjustment }
    case 'knn': return { kind: 'knn', k: 15, adjustment }
    case 'wrightParents': return { kind: 'wrightParents' }
    default: return assertNever(kind)
  }
}

const adjustmentStrategyLabel = (adjustment: CausalEffectsAdjustment): string => {
  switch (adjustment.kind) {
    case 'optimal': return 'Complete O-set'
    case 'minimizedOptimal': return 'Minimized O-set'
    case 'collidersMinimizedOptimal': return 'Collider-minimized O-set'
    case 'explicit': return 'User-supplied set'
    default: return assertNever(adjustment)
  }
}

type Job =
  | { readonly kind: 'idle' }
  | { readonly kind: 'running'; readonly progress: AnalysisProgress | null }
  | { readonly kind: 'failed'; readonly detail: string }

type ExplicitAdjustmentMemberDraft =
  | { readonly kind: 'closed' }
  | { readonly kind: 'editing'; readonly variable: number | null; readonly lag: number }

const CLOSED_ADJUSTMENT_DRAFT: ExplicitAdjustmentMemberDraft = { kind: 'closed' }

const timeIndexedNodeLabel = (
  node: readonly [number, number],
  names: readonly string[],
): string => {
  const variable = names[node[0]] ?? `Variable ${node[0] + 1}`
  return node[1] === 0 ? `${variable} (t)` : `${variable} (t−${Math.abs(node[1])})`
}

const describeAdjustmentProblems = (
  problems: readonly CausalEffectsAdjustmentProblem[],
  names: readonly string[],
): string => problems.map((problem) => {
  switch (problem.kind) {
    case 'queryTreatment': return `${timeIndexedNodeLabel(problem.node, names)} is the treatment node in this query.`
    case 'queryOutcome': return `${timeIndexedNodeLabel(problem.node, names)} is the outcome node in this query.`
    case 'laterTreatmentOccurrence': return `${timeIndexedNodeLabel(problem.node, names)} is a later occurrence of the treatment variable than the intervention node.`
    case 'forbiddenNode': return `${timeIndexedNodeLabel(problem.node, names)} is excluded by the time-indexed adjustment criterion.`
    case 'openNonCausalPath': return 'The complete set leaves at least one non-causal treatment–outcome path open.'
    default: return assertNever(problem)
  }
}).join(' ')

interface PanelBinding {
  readonly prepared: PreparedDatasetArtifact['id']
  readonly unit: ColumnId
  readonly time: ColumnId
  readonly outcome: ColumnId
  readonly treatment: ColumnId
}

interface StudyDataBinding {
  readonly prepared: PreparedDatasetArtifact['id']
  readonly dagRevision: StudySpecification['dagRevision']
  readonly treatment: ColumnId
  readonly outcome: ColumnId
}

type StudyDataPreflightJob =
  | { readonly kind: 'not-required' }
  | { readonly kind: 'loading'; readonly binding: StudyDataBinding }
  | { readonly kind: 'ready'; readonly binding: StudyDataBinding; readonly treatmentIsBinary: boolean; readonly outcomeIsCount: boolean; readonly observedGraphIsBinary: boolean }
  | { readonly kind: 'failed'; readonly binding: StudyDataBinding; readonly detail: string }

type PanelPreflightJob =
  | { readonly kind: 'not-required' }
  | { readonly kind: 'loading'; readonly binding: PanelBinding }
  | { readonly kind: 'ready'; readonly binding: PanelBinding; readonly matrix: PanelLongMatrix; readonly layout: PanelInterventionLayout }
  | { readonly kind: 'refused'; readonly binding: PanelBinding; readonly problem: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem'] }

/** The identification the panel works from, with the estimator that fits it and fresh defaults for every estimator. */
interface EstimationSelection {
  readonly identification: IdentificationId | null
  readonly estimator: EstimatorId
  readonly configurations: Readonly<Record<EstimatorId, EstimatorConfiguration>>
}

const estimationSelection = (identification: IdentificationArtifact | null, studies: readonly StudySpecification[], prepared: PreparedDatasetArtifact): EstimationSelection => {
  const study = studies.find((candidate) => candidate.id === identification?.study) ?? null
  return {
    identification: identification?.id ?? null,
    estimator: defaultEstimatorFor(identification?.result ?? null, prepared, study),
    configurations: Object.fromEntries(ESTIMATOR_IDS.map((estimator) => [estimator, defaultConfiguration(estimator, prepared, study)])) as Record<EstimatorId, EstimatorConfiguration>,
  }
}

interface State extends EstimationSelection {
  readonly job: Job
  readonly panelPreflight: PanelPreflightJob
  readonly studyDataPreflight: StudyDataPreflightJob
}

type Event =
  | { readonly type: 'identification-chosen'; readonly selection: EstimationSelection }
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
  | { readonly type: 'study-data-preflight-not-required' }
  | { readonly type: 'study-data-preflight-started'; readonly binding: StudyDataBinding }
  | { readonly type: 'study-data-preflight-succeeded'; readonly binding: StudyDataBinding; readonly treatmentIsBinary: boolean; readonly outcomeIsCount: boolean; readonly observedGraphIsBinary: boolean }
  | { readonly type: 'study-data-preflight-failed'; readonly binding: StudyDataBinding; readonly detail: string }

const step = (state: State, event: Event): State => {
  switch (event.type) {
    case 'identification-chosen': return { ...state, ...event.selection, job: { kind: 'idle' } }
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
    case 'study-data-preflight-not-required': return { ...state, studyDataPreflight: { kind: 'not-required' } }
    case 'study-data-preflight-started': return { ...state, studyDataPreflight: { kind: 'loading', binding: event.binding } }
    case 'study-data-preflight-succeeded': return { ...state, studyDataPreflight: { kind: 'ready', binding: event.binding, treatmentIsBinary: event.treatmentIsBinary, outcomeIsCount: event.outcomeIsCount, observedGraphIsBinary: event.observedGraphIsBinary } }
    case 'study-data-preflight-failed': return { ...state, studyDataPreflight: { kind: 'failed', binding: event.binding, detail: event.detail } }
    default: return assertNever(event)
  }
}


const intervalText = (estimate: CausalEstimate): string => {
  if (estimate.interval.kind === 'none') return 'none'
  const figure = formatInterval(headlineValue(estimate.effect), estimate.interval.lower, estimate.interval.upper, intervalTypeOf(estimate.interval), scaleOf(estimate))
  return `[${figure.bounds.lower}, ${figure.bounds.upper}]`
}


const samePanelBinding = (left: PanelBinding, right: PanelBinding): boolean =>
  left.prepared === right.prepared
  && left.unit === right.unit
  && left.time === right.time
  && left.outcome === right.outcome
  && left.treatment === right.treatment

const sameStudyDataBinding = (left: StudyDataBinding, right: StudyDataBinding): boolean =>
  left.prepared === right.prepared
  && left.dagRevision === right.dagRevision
  && left.treatment === right.treatment
  && left.outcome === right.outcome

const eligibilityHint = (eligibility: MethodEligibility): string => {
  switch (eligibility.kind) {
    case 'eligible': return 'Available: pre-run checks completed.'
    case 'caution': return 'Review: runnable, with conditions to assess.'
    case 'refused': return `Unavailable: ${eligibility.violations[0]?.evidence ?? 'a known requirement is not met.'}`
    default: return assertNever(eligibility)
  }
}

const panelWeight = (values: readonly number[], index: number): string => formatStatistic('score', values[index] ?? Number.NaN).text

function SyntheticControlEvidenceDetails({ run }: { readonly run: Extract<EstimationRunArtifact, { readonly kind: 'synthetic-control-run' }> }) {
  const { evidence } = run
  const donorNames = run.columns.slice(2).map((column) => column.name)
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <summary className="cursor-pointer text-ink">Synthetic-control inference</summary>
      <div className="mt-3 grid gap-4">
        {evidence.crossFit.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table className="w-full border-collapse text-body" aria-label="Cross-fitted synthetic-control folds">
              <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Fold</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Held-out rows</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Bias</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Effect</th></tr></thead>
              <tbody>{evidence.crossFit.folds.map((fold, index) => <tr key={index}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{index + 1}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{fold.heldOut[0] + 1}–{(fold.heldOut.at(-1) ?? fold.heldOut[0]) + 1}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', fold.bias).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', fold.att).text}</td></tr>)}</tbody>
            </table>
          </div>
        ) : <p className="m-0 text-body text-muted">Cross-fitted inference unavailable: {evidence.crossFit.reason}</p>}
        {evidence.donorPlacebo.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table className="w-full border-collapse text-body" aria-label="Donor-placebo MSPE ratios">
              <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Series</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Pre MSPE</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Post MSPE</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Post/pre ratio</th></tr></thead>
              <tbody>
                <tr><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-medium">Treated</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', evidence.donorPlacebo.treatedPreMspe).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', evidence.donorPlacebo.treatedPostMspe).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{evidence.donorPlacebo.treatedMspeRatio === null ? '∞' : formatStatistic('raw', evidence.donorPlacebo.treatedMspeRatio).text}</td></tr>
                {evidence.donorPlacebo.placebos.map((placebo) => <tr key={placebo.donor}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{donorNames[placebo.donor] ?? `Donor ${placebo.donor + 1}`}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', placebo.preMspe).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', placebo.postMspe).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{placebo.mspeRatio === null ? '∞' : formatStatistic('raw', placebo.mspeRatio).text}</td></tr>)}
              </tbody>
            </table>
          </div>
        ) : <p className="m-0 text-body text-muted">Donor-placebo inference unavailable: {evidence.donorPlacebo.reason}</p>}
        <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body">
          <dt>Conformal band</dt><dd className="m-0">{evidence.conformalBand.kind === 'available' ? `half-width ${formatStatistic('raw', evidence.conformalBand.halfWidth).text}` : evidence.conformalBand.reason}</dd>
          <dt>Gaussian band</dt><dd className="m-0">{evidence.gaussianBand.kind === 'available' ? `half-width ${formatStatistic('raw', evidence.gaussianBand.halfWidth).text}` : evidence.gaussianBand.reason}</dd>
        </dl>
      </div>
    </details>
  )
}

type PanelPeriodDisplay =
  | { readonly kind: 'source-labels'; readonly labels: readonly string[] }
  | { readonly kind: 'dense-codes'; readonly labels: readonly string[] }

function panelPeriodDisplay(run: Extract<EstimationRunArtifact, { readonly kind: 'panel-intervention-run' }>): PanelPeriodDisplay {
  const recorded: unknown = Reflect.get(run, 'timeLabels')
  if (Array.isArray(recorded)
    && recorded.length === run.evidence.times.length
    && recorded.every((label) => typeof label === 'string' && label.length > 0)) {
    return { kind: 'source-labels', labels: recorded }
  }
  return { kind: 'dense-codes', labels: run.evidence.times.map(String) }
}

function PanelEvidenceDetails({ run }: { readonly run: Extract<EstimationRunArtifact, { readonly kind: 'panel-intervention-run' }> }) {
  const { evidence } = run
  const controls = evidence.units.slice(0, evidence.controlUnits)
  const periods = panelPeriodDisplay(run)
  const preLabels = periods.labels.slice(0, evidence.nPre)
  const postLabels = periods.labels.slice(evidence.nPre)
  const estimates = [
    ['Difference-in-differences', evidence.did, null, null],
    ['Synthetic control', evidence.syntheticControl, evidence.syntheticControlPlacebo, evidence.syntheticControlInTime],
    ['Synthetic difference-in-differences', evidence.syntheticDid, evidence.syntheticDidPlacebo, evidence.syntheticDidInTime],
  ] as const
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <summary className="cursor-pointer text-ink">Panel weights and period effects</summary>
      <div className="mt-3 grid gap-4">
        {periods.kind === 'dense-codes' && <Alert tone="info" live={false}><p className="m-0">This saved run does not contain source period labels. The period tables therefore show zero-based dense codes.</p></Alert>}
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel estimator comparison">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Estimator</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Estimate</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Placebo SE</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">In-time placebo</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Noise level</th></tr></thead>
            <tbody>{estimates.map(([name, estimate, placebo, inTime]) => <tr key={name}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{name}{name.startsWith('Synthetic difference') ? ' · primary' : ''}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.estimate).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{placebo === null ? '—' : placebo.kind === 'available' ? formatStatistic('raw', placebo.standardError).text : 'Unavailable'}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{inTime === null ? '—' : inTime.kind === 'available' ? formatStatistic('raw', inTime.estimate).text : 'Unavailable'}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.noiseLevel).text}</td></tr>)}</tbody>
          </table>
        </div>
        {[evidence.syntheticControlPlacebo, evidence.syntheticDidPlacebo, evidence.syntheticControlInTime, evidence.syntheticDidInTime].some((item) => item.kind === 'unavailable') ? (
          <ul className="m-0 grid gap-1 pl-5 text-body text-muted">
            {evidence.syntheticControlPlacebo.kind === 'unavailable' ? <li>SC placebo SE: {evidence.syntheticControlPlacebo.reason}</li> : null}
            {evidence.syntheticDidPlacebo.kind === 'unavailable' ? <li>SDID placebo SE: {evidence.syntheticDidPlacebo.reason}</li> : null}
            {evidence.syntheticControlInTime.kind === 'unavailable' ? <li>SC in-time placebo: {evidence.syntheticControlInTime.reason}</li> : null}
            {evidence.syntheticDidInTime.kind === 'unavailable' ? <li>SDID in-time placebo: {evidence.syntheticDidInTime.reason}</li> : null}
          </ul>
        ) : null}
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel unit weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Control unit</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{controls.map((unit, index) => <tr key={unit}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{unit}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.omega, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-body" aria-label="Panel time weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Pre-period</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{preLabels.map((period, index) => <tr key={`${period}-${index}`}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{period}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.lambda, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
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
    <details className={well('mt-3 px-3 py-2 text-body')}>
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
      case 'frontdoor-two-stage-run': {
        const { evidence } = run
        const mediator = run.columns[evidence.mediator]?.name ?? 'Mediator'
        return [
          { label: 'Mediator', value: formatWords(mediator), context: `${formatStatistic('raw', evidence.controlValue).text} → ${formatStatistic('raw', evidence.treatmentValue).text} treatment contrast` },
          { label: 'Treatment → mediator', value: formatStatistic('raw', evidence.firstStageEffect), context: `${formatCount(evidence.firstStageParams.length).text} first-stage parameters` },
          { label: 'Mediator → outcome', value: formatStatistic('raw', evidence.secondStageEffect), context: `${formatCount(evidence.secondStageParams.length).text} second-stage parameters` },
        ]
      }
      case 'instrumental-variable-run': {
        const { evidence } = run
        const instruments = evidence.instruments.map((index) => run.columns[index]?.name ?? String(index)).join(', ')
        return [
          { label: 'Estimator route', value: formatWords(describeInstrumentalVariableRoute(evidence.route)), context: `${formatCount(evidence.instruments.length, { noun: 'instrument' }).text} · ${formatCount(evidence.params.length, { noun: 'coefficient' }).text}` },
          { label: 'Instruments', value: formatWords(instruments), context: `${formatCount(evidence.observations).text} rows` },
          { label: 'Bootstrap SE', value: evidence.standardError === null ? formatWords('none') : formatStatistic('raw', evidence.standardError), context: evidence.uncertainty.kind === 'bootstrap' ? `${formatCount(evidence.uncertainty.simulations).text} resamples · seed ${evidence.uncertainty.seed}` : 'no bootstrap requested' },
        ]
      }
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
          { label: 'Convergence', value: formatWords(evidence.converged ? 'converged' : 'not converged'), context: `${formatCount(evidence.iterations).text} iterations` },
        ]
      }
      case 'negative-binomial-ingarch-run': {
        const { evidence } = run
        return [
          { label: 'Average path difference', value: formatStatistic('raw', evidence.averageEffect), context: `${formatCount(evidence.horizon).text} forecast periods · cumulative ${formatStatistic('raw', evidence.cumulativeEffect).text}` },
          { label: 'Overdispersion', value: formatStatistic('raw', evidence.dispersion), context: `negative-binomial size ${formatStatistic('raw', evidence.size).text}` },
          { label: 'Recursion', value: formatWords(`${evidence.link === 'identity' ? 'additive' : 'multiplicative'} · count lag ${evidence.pastObservationLags.join(', ')} · mean lag ${evidence.pastMeanLags.join(', ')}`), context: `${formatCount(evidence.iterations).text} optimizer iterations · ${evidence.functionEvaluations}/${evidence.gradientEvaluations} function/gradient evaluations` },
        ]
      }
      case 'ardl-run': {
        const { evidence } = run
        const reading = boundsReading(evidence)
        const [lower, upper] = evidence.boundsCritical[1] ?? [Number.NaN, Number.NaN]
        return [
          { label: 'Bounds test', value: formatWords(reading === 'level-relation' ? 'level relation' : reading === 'no-level-relation' ? 'no level relation' : 'inconclusive'), context: `F ${formatStatistic('raw', evidence.boundsStatistic).text} against 5% bounds ${formatStatistic('raw', lower).text} to ${formatStatistic('raw', upper).text}` },
          { label: 'Bounds p', value: formatP(evidence.boundsPUpper, { withLabel: false }), context: `I(1) bound · I(0) bound p ${formatP(evidence.boundsPLower, { withLabel: false }).text}` },
          { label: 'Lag orders', value: formatWords(`ARDL(${evidence.arLag}, ${evidence.dlLag})`), context: `AIC over ${formatCount(evidence.grid.length).text} candidates · ${evidence.trend === 'ct' ? 'constant and trend' : 'constant'} · case ${evidence.case}` },
        ]
      }
      case 'vecm-run': {
        const { evidence } = run
        return [
          { label: 'Cointegration rank', value: formatCount(evidence.rank), context: `Johansen trace at ${['90', '95', '99'][evidence.significance] ?? ''}% · ${evidence.kArDiff} lagged differences` },
          { label: 'Adjustment p', value: formatWords(evidence.pvaluesAlpha.map((row) => formatP(row[0] ?? Number.NaN, { withLabel: false }).text).join(' · ')), context: `alpha per equation · terms “${evidence.deterministic}”` },
          evidence.chow === null
            ? { label: 'Chow break', value: formatWords('not requested'), context: 'set a split after a row to test stability' }
            : { label: 'Chow break', value: formatP(evidence.chow[1], { withLabel: false }), context: `F ${formatStatistic('raw', evidence.chow[0]).text} · split after row ${run.configuration.breakIndex}` },
        ]
      }
      case 'synthetic-control-run': {
        const { evidence } = run
        const donorNames = run.columns.slice(2).map((variable) => variable.name)
        return [
          {
            label: 'Donor weights',
            value: formatWords(evidence.weights
              .map((weight, index) => ({ name: donorNames[index] ?? String(index), weight }))
              .filter((donor) => donor.weight >= 0.0005)
              .sort((first, second) => second.weight - first.weight)
              .map((donor) => `${donor.name} ${formatStatistic('score', donor.weight).text}`)
              .join(' · ')),
            context: `${formatCount(evidence.weights.filter((weight) => weight >= 0.0005).length).text} of ${evidence.weights.length} donors carry weight · sum to one`,
          },
          { label: 'Pre-period loss', value: formatStatistic('raw', evidence.loss), context: `${formatCount(evidence.nPre).text} pre rows · ${formatCount(evidence.iterations).text} active-set steps` },
          { label: 'Average post gap', value: formatStatistic('raw', evidence.att), context: `over ${formatCount(evidence.nPost).text} post rows` },
          evidence.crossFit.kind === 'available'
            ? { label: 'Bias-corrected estimate', value: formatStatistic('raw', evidence.crossFit.att), context: `pre-period blocks left out in turn · SE ${formatStatistic('raw', evidence.crossFit.standardError).text} · ${formatInterval(evidence.crossFit.att, evidence.crossFit.confidenceInterval[0], evidence.crossFit.confidenceInterval[1], { kind: 'confidence', level: 0.95 }, additive).text} · p ${formatP(evidence.crossFit.pValue, { withLabel: false }).text}` }
            : { label: 'Bias-corrected estimate', value: formatWords('unavailable'), context: evidence.crossFit.reason },
          evidence.donorPlacebo.kind === 'available'
            ? { label: 'Donor placebo rank', value: formatP(evidence.donorPlacebo.pValue, { withLabel: false }), context: `compared with ${formatCount(evidence.donorPlacebo.nValidPlacebos).text} donors treated in turn; smaller means fewer donors looked as unusual` }
            : { label: 'Donor placebo rank', value: formatWords('unavailable'), context: evidence.donorPlacebo.reason },
          evidence.conformalBand.kind === 'available'
            ? { label: 'Conformal band', value: formatStatistic('raw', evidence.conformalBand.halfWidth), context: `${Math.round((1 - evidence.conformalBand.alpha) * 100)}% fixed-weight prediction half-width` }
            : { label: 'Conformal band', value: formatWords('unavailable'), context: evidence.conformalBand.reason },
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
          { label: 'Method comparison', value: formatWords(`DID ${formatStatistic('raw', evidence.did.estimate).text} · synthetic control ${formatStatistic('raw', evidence.syntheticControl.estimate).text} · synthetic DID ${formatStatistic('raw', evidence.syntheticDid.estimate).text}`), context: `synthetic DID chosen as the main result before fitting · ${formatCount(evidence.nPost).text} post periods` },
          { label: 'Panel layout', value: formatWords(`${evidence.treatedUnits} treated · ${evidence.controlUnits} comparison`), context: `${evidence.units.length} units × ${evidence.times.length} periods` },
          { label: 'Most influential comparison units', value: formatWords(topWeights || 'none'), context: 'largest synthetic-DID unit weights' },
        ]
      }
      case 't-learner-run': {
        const { evidence } = run
        // The bound estimate carries the effects as a non-empty list; the raw evidence only promises a list.
        const summary = run.estimate.effect.kind === 'perRow' ? summariseRowEffects(run.estimate.effect.effects) : null
        return [
          { label: 'Arms', value: formatWords(`${formatCount(evidence.controlRows).text} control · ${formatCount(evidence.treatedRows).text} treated`), context: 'one outcome forest each' },
          { label: 'Row effects', value: formatWords(summary === null ? 'none' : `${formatStatistic('raw', summary.minimum).text} to ${formatStatistic('raw', summary.maximum).text}`), context: summary === null ? '' : `median ${formatStatistic('raw', summary.median).text} · ${formatPercent(summary.positiveShare, { precision: 0 }).text} above zero` },
          { label: 'Forests', value: formatWords(`${formatCount(evidence.trees).text} trees`), context: `minimum leaf ${evidence.minLeaf} · learner seed ${evidence.seed}` },
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
      case 'bayesian-gaussian-run': {
        const { evidence } = run
        return [
          { label: 'Divergences', value: formatCount(evidence.divergences), context: `${formatCount(evidence.chains).text} chains · acceptance ${formatStatistic('score', evidence.acceptanceRate).text} · step ${formatStatistic('raw', evidence.stepSize).text}` },
          { label: 'P(effect > 0)', value: formatStatistic('score', evidence.probabilityPositive), context: `posterior mean ${formatStatistic('raw', evidence.effectMean).text} · sd ${formatStatistic('raw', evidence.effectSd).text} · median ${formatStatistic('raw', evidence.effectMedian).text}` },
          { label: 'Residual sd', value: formatStatistic('raw', evidence.sigmaMean), context: `${formatCount(evidence.warmup).text} warmup · ${formatCount(evidence.samples).text} draws per chain · seed ${evidence.seed}` },
        ]
      }
      case 'discrete-bn-run': {
        const { evidence } = run
        return [
          { label: 'Treatment states', value: formatWords(`${evidence.treatmentStates[0]} → ${evidence.treatmentStates[1]}`), context: `state budget ${evidence.bins} · counts ${evidence.stateCounts.join('/')}` },
          { label: 'State preparation', value: formatWords(`${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'observedStates').length} observed · ${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'quantiles').length} quantile`), context: describeDiscreteStatePreparations(evidence.statePreparations) },
          { label: 'Expected outcome', value: formatWords(`${formatStatistic('raw', evidence.expectations[0]).text} → ${formatStatistic('raw', evidence.expectations[1]).text}`), context: 'under do(low) and do(high)' },
          ...(evidence.parentsAdjusted.join(', ') === adjustmentLabels(run.estimate.adjustment).join(', ') ? [] : [
  { label: 'Adjustment set', value: formatWords(evidence.parentsAdjusted.length === 0 ? 'none' : evidence.parentsAdjusted.join(', ')), context: evidence.minimalAdjustmentSet === null ? 'no minimal adjustment set' : `minimal set ${evidence.minimalAdjustmentSet.length === 0 ? 'empty' : evidence.minimalAdjustmentSet.join(', ')}` },
          ]),
        ]
      }
      case 'binary-ett-run': {
        const { evidence } = run
        return [
          { label: 'E[Y(1) | X=1]', value: formatStatistic('raw', evidence.treatedPotentialOutcomeMean), context: 'treated potential-outcome mean among treated rows' },
          { label: 'E[Y(0) | X=1]', value: formatStatistic('raw', evidence.untreatedPotentialOutcomeMean), context: 'untreated potential-outcome mean among treated rows' },
          { label: 'ETT', value: formatStatistic('raw', evidence.effectOnTreated), context: `${formatCount(evidence.observations).text} rows · plug-in estimate` },
        ]
      }
      case 'double-ml-run': {
        const { evidence } = run
        return [
          { label: 'Model', value: formatWords(evidence.model === 'plr' ? 'partially linear' : evidence.att ? 'interactive · effect on the treated' : 'interactive · average effect'), context: evidence.treatBinary ? 'binary treatment' : 'continuous treatment' },
          { label: 'Standard error', value: formatStatistic('raw', evidence.standardError), context: 'sandwich, cross-fitted' },
          { label: 'Fold seed', value: formatCount(evidence.seed), context: '5 folds · 200 trees · learner seed 7' },
        ]
      }
      case 'causal-effects-run': {
        const { evidence } = run
        const nodeName = (node: readonly [number, number]) => `${run.columns[node[0]]?.name ?? node[0]}${node[1] === 0 ? '' : ` (t−${Math.abs(node[1])})`}`
        const fitTiles = (() => {
          switch (evidence.fit.kind) {
            case 'unfitted': return []
            case 'invalidAdjustment': return []
            case 'adjustedLinear': return [{ label: 'Adjustment set', value: formatWords(evidence.fit.adjustmentSet.length === 0 ? 'None' : evidence.fit.adjustmentSet.map(nodeName).join(', ')), context: `${adjustmentStrategyLabel(evidence.fit.selection)} · linear · τ max ${evidence.tauMax}` }]
            case 'adjustedKnn': return [{ label: 'Adjustment set', value: formatWords(evidence.fit.adjustmentSet.length === 0 ? 'None' : evidence.fit.adjustmentSet.map(nodeName).join(', ')), context: `${adjustmentStrategyLabel(evidence.fit.selection)} · ${evidence.fit.k}-neighbour · τ max ${evidence.tauMax}` }]
            case 'wrightParents': return [
              { label: 'Direct effect', value: formatStatistic('raw', evidence.fit.directEffect), context: 'sum of direct path contrasts' },
              { label: 'Indirect effect', value: formatStatistic('raw', evidence.fit.indirectEffect), context: `${evidence.fit.paths.length} directed paths · ${evidence.fit.coefficients.length} parent coefficients` },
            ]
            default: return assertNever(evidence.fit)
          }
        })()
        return [
          ...fitTiles,
          { label: 'Predictions', value: formatWords(evidence.predictions.map((value) => formatStatistic('raw', value).text).join(' → ')), context: `at ${evidence.interventions[0]} and ${evidence.interventions[1]}` },
          { label: 'Fitted rows', value: formatCount(evidence.fittedObservations), context: evidence.mediators.length === 0 ? 'no mediators' : `${evidence.mediators.length} mediator nodes` },
          ...(evidence.uncertainty.kind === 'bootstrap' ? [{ label: 'Bootstrap', value: formatCount(evidence.uncertainty.samples), context: `${Math.round(evidence.uncertainty.confidenceLevel * 100)}% percentile interval · block ${evidence.uncertainty.resolvedBlockLength} · seed ${evidence.uncertainty.seed}` }] : []),
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
  const adjustmentValue = formatWords(run.kind === 'panel-intervention-run'
    ? 'Unit and time weights'
    : run.kind === 'frontdoor-two-stage-run'
    ? `stage 1: ${run.evidence.firstStageAdjustment.length === 0 ? 'none' : run.evidence.firstStageAdjustment.map((index) => run.columns[index]?.name ?? index).join(', ')} · stage 2: ${run.evidence.secondStageAdjustment.length === 0 ? 'none' : run.evidence.secondStageAdjustment.map((index) => run.columns[index]?.name ?? index).join(', ')}`
    : run.kind === 'instrumental-variable-run'
      ? 'None; the estimator uses no covariates'
    : run.estimate.adjustment.kind === 'structural-parent-model'
      ? `${run.estimate.adjustment.coefficients} parent coefficients · ${run.estimate.adjustment.paths} directed paths`
      : adjustmentLabels(run.estimate.adjustment).length === 0 ? 'None' : adjustmentLabels(run.estimate.adjustment).join(', '))

  // A method that reports the set it actually fitted names the same members as the recorded estimate
  // whenever it accepted them, so the two tiles would print one value twice and leave the grid's last
  // row part-filled. Keep the one tile and let it carry how the set was chosen.
  const restated = tiles.find((tile) => tile.label === 'Adjustment set' && tile.value.text === adjustmentValue.text)

  return (
    <MetricGrid className="mt-4" label="Diagnostics">
      <MetricTile
        label={run.kind === 'panel-intervention-run' ? 'Comparison design' : run.kind === 'frontdoor-two-stage-run' ? 'Stage adjustments' : run.kind === 'instrumental-variable-run' ? 'Covariates' : run.estimate.adjustment.kind === 'structural-parent-model' ? 'Structural model' : 'Adjustment set'}
        size="compact"
        frame="cell"
        value={adjustmentValue}
        context={run.kind === 'panel-intervention-run' ? 'This design does not use a DAG adjustment set.' : restated?.context}
      />
      {tiles.filter((tile) => tile !== restated).map((tile) => <MetricTile key={tile.label} label={tile.label} size="compact" frame="cell" value={tile.value} context={tile.context} />)}
    </MetricGrid>
  )
}

function ResultCard({ run, study, current, stepLabel, onDelete, others = [] }: { readonly others?: readonly EstimationRunArtifact[]; readonly run: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly stepLabel: string; readonly onDelete?: () => void }) {
  const theme = useChartTheme()
  const estimate = run.estimate
  const adjustmentVariables = useMemo(() => contemporaneousAdjustmentVariables(estimate.adjustment) ?? [], [estimate.adjustment])
  const sentence = resultHeadline(run, study)
  const scaleLine = resultScaleLine(run, study, stepLabel)
  const sampleLine = resultSampleLine(run)
  // Other path-valued runs of the same question can be drawn as a ghost behind this one.
  const [ghostId, setGhostId] = useState('')
  const ghosts = others.filter((other) => other.id !== run.id && other.estimate.effect.kind === 'path')
  const ghostRun = ghosts.find((other) => String(other.id) === ghostId) ?? null
  const chart = useMemo(() => (estimate.effect.kind === 'path'
    ? impactPathOption({
      outcome: study.outcome.name,
      points: estimate.effect.values,
      stepLabel,
      ghost: ghostRun !== null && ghostRun.estimate.effect.kind === 'path' ? { name: `${describeEstimator(ghostRun.configuration.kind)} · ${formatTime(ghostRun.createdAt)}`, points: ghostRun.estimate.effect.values } : undefined,
    }, theme)
    : null), [estimate.effect, stepLabel, study.outcome.name, theme, ghostRun])
  // Runs recorded before the histogram was added carry no draws, so they keep the summary alone.
  const posterior = useMemo(() => (run.kind === 'bayesian-gaussian-run' && (run.evidence.histogramCounts ?? []).length > 0
    ? posteriorDensityOption({
      outcome: study.outcome.name,
      treatment: study.treatment.name,
      histogramStart: run.evidence.histogramStart,
      histogramBinWidth: run.evidence.histogramBinWidth,
      histogramCounts: run.evidence.histogramCounts,
      mean: run.evidence.effectMean,
      hdiLower: run.evidence.hdiLower,
      hdiUpper: run.evidence.hdiUpper,
    }, theme)
    : null), [run, study.outcome.name, study.treatment.name, theme])
  const [chosenCurve, setChosenCurve] = useState(0)
  const curves = run.kind === 'bayesian-gaussian-run' ? run.evidence.curves ?? [] : []
  const curveIndex = Math.min(chosenCurve, Math.max(0, curves.length - 1))
  const curveChart = useMemo(() => {
    const curve = curves[curveIndex]
    return curve === undefined
      ? null
      : counterfactualCurvesOption({
        outcome: study.outcome.name,
        treatment: study.treatment.name,
        covariate: adjustmentVariables[curveIndex]?.name ?? `covariate ${curveIndex + 1}`,
        curve,
      }, theme)
  }, [curves, curveIndex, adjustmentVariables, study.outcome.name, study.treatment.name, theme])
  const stamp = <RunMeta>{[describeEstimator(run.configuration.kind), ...(run.kind === 'backdoor-linear-run' ? [describeCovariance(run.configuration.covariance)] : []), formatTime(run.createdAt)]}</RunMeta>
  // A grouped effect draws each group's interval on the shared axis, the whole-population average last.
  const groupChart = useMemo(() => (estimate.effect.kind === 'byGroup'
    ? runComparisonOption([
      ...estimate.effect.groups.map((group): RunComparisonRow => ({ label: group.label, estimate: group.value, lower: group.interval.lower, upper: group.interval.upper, current: false })),
      { label: 'All rows', estimate: estimate.effect.overall, lower: estimate.interval.kind === 'none' ? null : estimate.interval.lower, upper: estimate.interval.kind === 'none' ? null : estimate.interval.upper, current },
    ], theme)
    : null), [estimate, current, theme])
  // Per-row effects are drawn as their distribution, with the average marked, since a thousand points have no order to plot.
  const rowChart = useMemo(() => {
    if (estimate.effect.kind !== 'perRow') return null
    const summary = summariseRowEffects(estimate.effect.effects)
    return histogramOption({ name: `effect of ${study.treatment.name} on ${study.outcome.name}`, bins: summary.bins, nullCount: 0, marks: [{ name: 'average', value: estimate.effect.overall }, { name: 'median', value: summary.median }] }, theme)
  }, [estimate, study.treatment.name, study.outcome.name, theme])
  const body = (
    <>
      <div className="mt-3">
        <EstimateHeadline estimate={estimate} sentence={sentence} scaleLine={scaleLine} sampleLine={sampleLine} stepLabel={stepLabel} accent={current} testId="effect-estimate" />
      </div>
      {groupChart !== null && estimate.effect.kind === 'byGroup' && (
        <div className="mt-3">
          <ExpandableChart option={groupChart} label={`Effect of ${study.treatment.name} on ${study.outcome.name} by group of ${estimate.effect.modifier}`} className="h-[220px]" testId="group-effects" />
        </div>
      )}
      {rowChart !== null && (
        <div className="mt-3">
          <ExpandableChart option={rowChart} label={`Distribution of the per-row effect of ${study.treatment.name} on ${study.outcome.name}`} className="h-[220px]" testId="row-effects" />
        </div>
      )}
      {run.kind === 't-learner-run' && <TLearnerIntervals evidence={run.evidence} />}
      {chart !== null && (
        <div className="mt-3">
          {ghosts.length > 0 && (
            <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
              <p className={label('m-0 text-muted')}>Compare with</p>
              <Select aria-label="Compare with" className={field('text', 'w-64')} value={ghostId} onChange={(event) => setGhostId(event.target.value)}>
                <option value="">No other run</option>
                {ghosts.map((other) => <option key={other.id} value={String(other.id)}>{describeEstimator(other.configuration.kind)} · {formatTime(other.createdAt)}</option>)}
              </Select>
            </div>
          )}
          <ExpandableChart option={chart} label={`Observed ${study.outcome.name.replaceAll('_', ' ')} and its estimated no-intervention path`} className="h-[260px]" testId="impact-path" />
        </div>
      )}
      {posterior !== null && (
        <div className="mt-3">
          <ExpandableChart option={posterior} label={`Posterior density of the effect of ${study.treatment.name} on ${study.outcome.name}`} className="h-[220px]" testId="posterior-density" />
        </div>
      )}
      {curveChart !== null && (
        <div className="mt-3">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <p className={label('m-0 text-muted')}>Expected outcome across a covariate</p>
            <Select aria-label="Curve covariate" className={field('text', 'w-40')} value={curveIndex} onChange={(event) => setChosenCurve(Number(event.target.value))}>
              {curves.map((_, index) => <option key={index} value={index}>{adjustmentVariables[index]?.name ?? `covariate ${index + 1}`}</option>)}
            </Select>
          </div>
          <div className="mt-2"><ExpandableChart option={curveChart} label={`Expected ${study.outcome.name} across the chosen covariate under both interventions`} className="h-[240px]" testId="counterfactual-curves" /></div>
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
      <ResultInterpretation interpretation={interpretEstimationResult(run, study, stepLabel)} className="mt-3" />
      <Diagnostics run={run} />
      {run.kind === 'synthetic-control-run' ? <SyntheticControlEvidenceDetails run={run} /> : null}
      {run.kind === 'panel-intervention-run' ? <PanelEvidenceDetails run={run} /> : null}
      <RunRecord run={run} />
    </>
  )
  if (!current) {
    // History rows fold to one line in the runs drawer; only the current estimate keeps the stage.
    return (
      <RunFold title={sentence} figure={headlineFigure(estimate).text} stamp={stamp} onDelete={onDelete} deleteLabel="Delete this run">
        {body}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl border border-edge bg-panel p-4" aria-label={`${sentence} estimate`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal')}>Current estimate</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {body}
    </article>
  )
}

export function EstimationPanel({ source, profile, prepared, stationarity, documents, studies, identifications, runs, sensitivityRuns, onRun, onDeleteRun, onOpenStudy, onActivity }: {
  readonly onActivity?: (activity: RunActivity | null) => void
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly runs: readonly EstimationRunArtifact[]
  /** The recorded probes, so deleting a run can say which of them go with it. */
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly onRun: (run: EstimationRunArtifact) => void
  readonly onDeleteRun: (run: EstimationRunArtifact['id']) => void
  readonly onOpenStudy: () => void
}) {
  const identified = identifications.filter((identification) => estimableIdentification(identification.result))
  const chartTheme = useChartTheme()
  const [pendingDelete, setPendingDelete] = useState<EstimationRunArtifact | null>(null)
  const [adjustmentDraft, setAdjustmentDraft] = useState<ExplicitAdjustmentMemberDraft>(CLOSED_ADJUSTMENT_DRAFT)
  const [state, dispatch] = useReducer(step, null, (): State => {
    // The controls open as the latest recorded run set them, so a reopened project shows the analysis it holds.
    const latest = runs.filter((run) => ESTIMATOR_GROUPS.some((group) => group.estimators.includes(run.configuration.kind))).at(-1) ?? null
    const recorded = latest === null ? null : identified.find((candidate) => candidate.id === latest.identification) ?? null
    const selection = estimationSelection(recorded ?? identified.at(-1) ?? null, studies, prepared)
    return {
      ...selection,
      ...(latest !== null && recorded !== null ? { estimator: latest.configuration.kind, configurations: { ...selection.configurations, [latest.configuration.kind]: latest.configuration } } : {}),
      job: { kind: 'idle' },
      panelPreflight: { kind: 'not-required' },
      studyDataPreflight: { kind: 'not-required' },
    }
  })
  const [visibleEstimatorGroup, setVisibleEstimatorGroup] = useState<EstimatorGroupId>(() => estimatorGroupFor(state.estimator).id)
  useEffect(() => setVisibleEstimatorGroup(estimatorGroupFor(state.estimator).id), [state.estimator])
  const visibleGroup = ESTIMATOR_GROUPS.find((group) => group.id === visibleEstimatorGroup) ?? ESTIMATOR_GROUPS[0]
  const selectedEstimatorIsVisible = visibleGroup.estimators.includes(state.estimator)
  const identification = identified.find((candidate) => candidate.id === state.identification) ?? null
  const study = identification === null ? null : studies.find((candidate) => candidate.id === identification.study) ?? null
  const studyScale = prepared.kind === 'prepared-time-series' && study !== null
    ? [study.treatment, study.outcome].map((variable) => `${variable.name}: ${describeSeriesTransform(seriesTransformFor(prepared.seriesTransforms, variable.column))}`).join(' · ')
    : null
  const document = study === null ? null : documents.find((candidate) => candidate.id === study.dagDocument) ?? null
  const configuration = state.configurations[state.estimator]
  const method = methodDefinition(methodIdOf(state.estimator))
  const stepLabel = prepared.kind === 'prepared-time-series' ? frequencyUnit(prepared.sampling.frequency) : prepared.kind === 'prepared-panel' ? 'panel row' : 'row'
  const panelBinding = useMemo<PanelBinding | null>(() => prepared.kind === 'prepared-panel' && study !== null
    ? { prepared: prepared.id, unit: prepared.sampling.unitColumn, time: prepared.sampling.timeColumn, outcome: study.outcome.column, treatment: study.treatment.column }
    : null, [prepared, study])
  const studyDataBinding = useMemo<StudyDataBinding | null>(() => study === null
    ? null
    : { prepared: prepared.id, dagRevision: study.dagRevision, treatment: study.treatment.column, outcome: study.outcome.column }, [prepared.id, study])
  const studyDataFacts = useMemo(() => {
    if (studyDataBinding === null) return { treatmentIsBinary: null, outcomeIsCount: null, observedGraphIsBinary: null } as const
    const job = state.studyDataPreflight
    if (job.kind !== 'ready' || !sameStudyDataBinding(job.binding, studyDataBinding)) return { treatmentIsBinary: null, outcomeIsCount: null, observedGraphIsBinary: null } as const
    return { treatmentIsBinary: job.treatmentIsBinary, outcomeIsCount: job.outcomeIsCount, observedGraphIsBinary: job.observedGraphIsBinary } as const
  }, [state.studyDataPreflight, studyDataBinding])
  const studyDataPending = studyDataBinding !== null
    && (state.studyDataPreflight.kind === 'not-required'
      || !('binding' in state.studyDataPreflight)
      || !sameStudyDataBinding(state.studyDataPreflight.binding, studyDataBinding)
      || state.studyDataPreflight.kind === 'loading')
  const studyDataError = studyDataBinding !== null
    && state.studyDataPreflight.kind === 'failed'
    && sameStudyDataBinding(state.studyDataPreflight.binding, studyDataBinding)
    ? state.studyDataPreflight.detail
    : null
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

  useEffect(() => {
    if (studyDataBinding === null || study === null) {
      dispatch({ type: 'study-data-preflight-not-required' })
      return
    }
    let cancelled = false
    dispatch({ type: 'study-data-preflight-started', binding: studyDataBinding })
    void (async () => {
      const { materialisePrepared, describePreparedMaterialisationProblem } = await import('@/data/prepared')
      const observedColumns = study.graph.nodes.flatMap((node) => node.column === null ? [] : [node.column]) as unknown as NonEmptyArray<ColumnId>
      const matrix = await materialisePrepared(source, profile, prepared, observedColumns)
      if (cancelled) return
      if (!matrix.ok) {
        dispatch({ type: 'study-data-preflight-failed', binding: studyDataBinding, detail: describePreparedMaterialisationProblem(matrix.error) })
        return
      }
      const treatmentPosition = observedColumns.findIndex((column) => column === study.treatment.column)
      const outcomePosition = observedColumns.findIndex((column) => column === study.outcome.column)
      const treatment = matrix.value.values.subarray(treatmentPosition * matrix.value.rowCount, (treatmentPosition + 1) * matrix.value.rowCount)
      const outcome = matrix.value.values.subarray(outcomePosition * matrix.value.rowCount, (outcomePosition + 1) * matrix.value.rowCount)
      dispatch({
        type: 'study-data-preflight-succeeded',
        binding: studyDataBinding,
        treatmentIsBinary: treatment.every((value) => value === 0 || value === 1),
        outcomeIsCount: outcome.every((value) => Number.isInteger(value) && value >= 0),
        observedGraphIsBinary: matrix.value.values.every((value) => value === 0 || value === 1),
      })
    })()
    return () => { cancelled = true }
  }, [prepared, profile, source, study, studyDataBinding])

  const eligibilityByEstimator = useMemo<ReadonlyMap<EstimatorId, MethodEligibility>>(() => {
    const evaluations = new Map<EstimatorId, MethodEligibility>()
    if (identification === null) return evaluations
    for (const estimator of ESTIMATOR_GROUPS.flatMap((group) => group.estimators)) {
      const definition = methodDefinition(methodIdOf(estimator))
      if (!definition.ok) continue
      evaluations.set(estimator, evaluateEstimatorEligibility(definition.value, {
        identification: identification.result,
        prepared,
        stationarity,
        configuration: state.configurations[estimator],
        outcomeIsCount: studyDataFacts.outcomeIsCount,
        treatmentIsBinary: studyDataFacts.treatmentIsBinary,
        observedGraphIsBinary: studyDataFacts.observedGraphIsBinary,
        document,
        study,
        panelPreflight,
      }))
    }
    return evaluations
  }, [document, identification, panelPreflight, prepared, state.configurations, stationarity, study, studyDataFacts])
  const eligibility = eligibilityByEstimator.get(state.estimator) ?? null
  const configure = (next: EstimatorConfiguration) => dispatch({ type: 'configured', configuration: next })
  const adjustmentDraftOpen = configuration.kind === 'causal-effects-total'
    && configuration.estimator.kind !== 'wrightParents'
    && configuration.estimator.adjustment.kind === 'explicit'
    && adjustmentDraft.kind === 'editing'

  useEffect(() => {
    const explicitAdjustmentActive = configuration.kind === 'causal-effects-total'
      && configuration.estimator.kind !== 'wrightParents'
      && configuration.estimator.adjustment.kind === 'explicit'
    if (!explicitAdjustmentActive && adjustmentDraft.kind === 'editing') {
      setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
    }
  }, [adjustmentDraft.kind, configuration])

  useRunActivity(onActivity, state.job.kind === 'running' ? { label: describeEstimator(state.estimator), progress: state.job.progress === null ? null : state.job.progress.completed / Math.max(1, state.job.progress.total) } : null)
  const execute = async () => {
    if (identification === null || study === null || eligibility === null || eligibility.kind === 'refused' || state.job.kind === 'running' || adjustmentDraftOpen || !method.ok) return
    if (!estimableIdentification(identification.result)) return
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
      if (configuration.kind === 'frontdoor-two-stage') {
        if (identification.result.kind !== 'graphically-identified' || identification.result.frontdoor.kind !== 'identified' || identification.result.frontdoor.mediators.length !== 1) {
          dispatch({ type: 'run-failed', detail: 'This identification record does not contain one observed front-door mediator.' })
          return
        }
        const mediator = identification.result.frontdoor.mediators[0]
        const columns: NonEmptyArray<StudyVariable> = [study.treatment, mediator, study.outcome]
        const matrix = await materialise(columns)
        const evidence = await analysis.runFrontdoorTwoStage(matrix.values, matrix.rowCount, columns.length, {
          treatment: 0,
          mediator: 1,
          outcome: 2,
          firstStageAdjustment: [],
          secondStageAdjustment: [0],
          controlValue: configuration.interventions[0],
          treatmentValue: configuration.interventions[1],
          uncertainty: {
            kind: 'bootstrap',
            simulations: configuration.simulations,
            sampleSizeFraction: configuration.sampleSizeFraction,
            confidenceLevel: configuration.level,
            seed: configuration.seed,
          },
        }, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
        const run = { kind: 'frontdoor-two-stage-run', configuration, evidence: evidence.value } as const
        const estimate = causalEstimateFrom(study, identification, run)
        finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The front-door identification record is no longer available.')
        return
      }
      if (configuration.kind === 'instrumental-variable') {
        const instruments = identifiedInstruments(identification.result)
        if (instruments === null) {
          dispatch({ type: 'run-failed', detail: 'This identification record names no observed instrument.' })
          return
        }
        const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...instruments]
        const matrix = await materialise(columns)
        const evidence = await analysis.runInstrumentalVariable(matrix.values, matrix.rowCount, columns.length, {
          treatment: 0,
          outcome: 1,
          instruments: instruments.map((_, index) => 2 + index),
          uncertainty: {
            kind: 'bootstrap',
            simulations: configuration.simulations,
            sampleSizeFraction: configuration.sampleSizeFraction,
            confidenceLevel: configuration.level,
            seed: configuration.seed,
          },
        }, (progress) => dispatch({ type: 'run-progressed', progress }))
        if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
        const run = { kind: 'instrumental-variable-run', configuration, evidence: evidence.value } as const
        const estimate = causalEstimateFrom(study, identification, run)
        finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The instrument identification record is no longer available.')
        return
      }
      if (configuration.kind === 'binary-ett-idc-star') {
        if (identification.result.kind !== 'counterfactually-identified') {
          dispatch({ type: 'run-failed', detail: 'This identification record does not contain binary ETT expressions from IDC*.' })
          return
        }
        const observed = study.graph.nodes.flatMap((node, nodeIndex) => node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name, nodeIndex }])
        if (observed.length < 2) { dispatch({ type: 'run-failed', detail: 'Binary ETT needs measured treatment and outcome nodes.' }); return }
        const columns = observed.map(({ node, column, name }) => ({ node, column, name })) as unknown as NonEmptyArray<StudyVariable>
        const matrix = await materialise(columns)
        const treatment = study.graph.nodes.findIndex((node) => node.node === study.treatment.node)
        const outcome = study.graph.nodes.findIndex((node) => node.node === study.outcome.node)
        const evidence = await analysis.runBinaryEtt(matrix.values, matrix.rowCount, columns.length, {
          observedNodes: observed.map((node) => node.nodeIndex),
          names: study.graph.nodes.map((node) => node.name),
          edges: study.graph.edges,
          treatment,
          outcome,
          unobserved: study.graph.nodes.flatMap((node, index) => node.column === null ? [index] : []),
        })
        if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
        const run = { kind: 'binary-ett-run', configuration, evidence: evidence.value } as const
        const estimate = causalEstimateFrom(study, identification, run)
        finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The IDC* identification record is no longer available.')
        return
      }
      if (identification.result.kind !== 'identified') {
        dispatch({ type: 'run-failed', detail: 'This estimator requires a back-door identification record.' })
        return
      }
      switch (configuration.kind) {
        case 'backdoor-linear-regression': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const evidence = await analysis.runBackdoorLinear(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: identification.result.adjustment.variables.map((_, index) => index + 2), hacMaxLags: null, level: configuration.level })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'backdoor-linear-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind) as typeof run extends never ? never : 'backdoor-linear-regression' extends string ? ReturnType<typeof methodIdOf> & typeof method.value.id : never, columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'dml-plr':
        case 'dml-irm': {
          const nuisance = dmlNuisanceInputs(identification.result.adjustment.variables, study.estimand)
          const modifier = study.estimand.kind === 'conditional-average-treatment-effect' ? study.estimand.modifier : null
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...nuisance]
          const matrix = await materialise(columns)
          if (configuration.kind === 'dml-irm' && columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} is not binary; the interactive model needs a 0/1 treatment.` }); return }
          const groups: DmlGroupsRequest = study.estimand.kind !== 'conditional-average-treatment-effect'
            ? { kind: 'none' }
            : study.estimand.grouping.kind === 'levels'
              ? { kind: 'levels', column: columns.findIndex((variable) => variable.column === modifier?.column) }
              : { kind: 'quantiles', column: columns.findIndex((variable) => variable.column === modifier?.column), bins: study.estimand.grouping.bins }
          const evidence = await analysis.runDoubleMl(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: nuisance.map((_, index) => index + 2), model: configuration.kind === 'dml-plr' ? 'plr' : 'irm', att: configuration.kind === 'dml-irm' && configuration.att, seed: configuration.seed, groups })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'double-ml-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 't-learner': {
          const inputs = tLearnerInputs(identification.result.adjustment.variables, study.estimand)
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...inputs]
          const matrix = await materialise(columns)
          if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} is not binary; the T-learner fits one outcome model per arm and needs a 0/1 treatment.` }); return }
          const evidence = await analysis.runTLearner(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: inputs.map((_, index) => index + 2), seed: configuration.seed, uncertainty: configuration.uncertainty })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 't-learner-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'ardl-pss': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome]
          const matrix = await materialise(columns)
          const evidence = await analysis.runArdlPss(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, maxLag: configuration.maxLag, trend: configuration.trend, case: configuration.case })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'ardl-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'vecm': {
          const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const evidence = await analysis.runVecm(matrix.values, matrix.rowCount, columns.length, { endogenous: columns.map((_, index) => index), maxLags: configuration.maxLags, deterministic: configuration.deterministic, significance: configuration.significance === 90 ? 0 : configuration.significance === 95 ? 1 : 2, breakIndex: configuration.breakIndex })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'vecm-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact,
            evidence.value.rank === 0 ? 'The Johansen trace test did not identify a cointegrating relation under this specification, so no long-run coefficient is reported.' : `The trace test found ${evidence.value.rank} cointegrating relations; a single long-run coefficient is read only at rank one.`)
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
          const evidence = await analysis.runSyntheticControl(matrix.values, matrix.rowCount, columns.length, { treated: 1, donors: donorVariables.map((_, index) => index + 2), nPre, crossFitFolds: configuration.crossFitFolds, alpha: configuration.alpha })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
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
            matrix.periodCodes,
            { placeboReplications: configuration.placeboReplications, seed: configuration.seed },
            (progress) => dispatch({ type: 'run-progressed', progress }),
          )
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment]
          const run = { kind: 'panel-intervention-run', configuration, evidence: evidence.value, timeLabels: mapNonEmpty(state.panelPreflight.layout.periods, (period) => period.label) } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The panel intervention run produced no estimate.')
          return
        }
        case 'negbin-nuts': {
          const confounder = identification.result.adjustment.variables[0]
          if (confounder === undefined || identification.result.adjustment.variables.length !== 1) { dispatch({ type: 'run-failed', detail: 'This model takes exactly one confounder.' }); return }
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, confounder]
          const matrix = await materialise(columns)
          const outcome = columnAt(matrix.values, matrix.rowCount, 1)
          if (outcome.some((value) => value < 0 || !Number.isInteger(value))) { dispatch({ type: 'run-failed', detail: `${study.outcome.name} is not a count: it holds negative or fractional values.` }); return }
          const evidence = await analysis.runNegbinNuts(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, confounder: 2, warmup: configuration.warmup, samples: configuration.samples, seed: configuration.seed })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'negbin-nuts-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'bayesian-gaussian': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `${study.treatment.name} is not binary; the Gaussian model needs a 0/1 treatment.` }); return }
          const evidence = await analysis.runBayesianGaussian(matrix.values, matrix.rowCount, columns.length, { treatment: 0, outcome: 1, adjustment: identification.result.adjustment.variables.map((_, index) => index + 2), warmup: configuration.warmup, samples: configuration.samples, seed: configuration.seed })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'bayesian-gaussian-run', configuration, evidence: evidence.value } as const
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
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
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
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'count-glm-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'negative-binomial-ingarch': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const outcome = columnAt(matrix.values, matrix.rowCount, 1)
          if (outcome.some((value) => value < 0 || !Number.isInteger(value))) { dispatch({ type: 'run-failed', detail: `${study.outcome.name} is not a count: it holds negative or fractional values.` }); return }
          const regressors = [0, ...identification.result.adjustment.variables.map((_, index) => index + 2)]
          const baselineRegressors = regressors.map((column) => matrix.values[column * matrix.rowCount + matrix.rowCount - 1] ?? 0)
          const evidence = await analysis.runNegativeBinomialIngarch(matrix.values, matrix.rowCount, columns.length, {
            outcome: 1,
            link: configuration.link,
            regressors,
            pastObservationLags: configuration.pastObservationLags,
            pastMeanLags: configuration.pastMeanLags,
            externalRegressors: regressors.map(() => false),
            horizon: configuration.horizon,
            baselineRegressors,
            interventionRegressor: 0,
            controlValue: configuration.controlValue,
            treatmentValue: configuration.treatmentValue,
            schedule: configuration.schedule,
          }, (progress) => dispatch({ type: 'run-progressed', progress }))
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'negative-binomial-ingarch-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The INGARCH run produced no forecast path.')
          return
        }
        case 'causal-effects-total': {
          if (document === null) { dispatch({ type: 'run-failed', detail: 'The study’s DAG document is missing.' }); return }
          const nodes = document.current.graph.nodes
          const graphVariables = nodes.map((node): StudyVariable | null => node.kind === 'observed' ? { node: node.id, column: node.column, name: node.name } : null)
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
            uncertainty: configuration.uncertainty,
          }, (progress) => dispatch({ type: 'run-progressed', progress }))
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          if (evidence.value.fit.kind === 'invalidAdjustment') {
            dispatch({
              type: 'run-failed',
              detail: describeAdjustmentProblems(evidence.value.fit.problems, nodes.map((node) => node.name)),
            })
            return
          }
          const run = { kind: 'causal-effects-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, { ...run, graphVariables })
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
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
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
  const temporalAdjustmentCandidates = document === null || study === null
    ? []
    : document.current.graph.nodes.flatMap((node, index) => node.kind === 'observed'
      ? [{ index, name: node.name }]
      : [])
  const temporalAdjustmentMaxLag = document === null || configuration.kind !== 'causal-effects-total'
    ? 0
    : stationaryMarksOf(document).statLag + configuration.treatmentLag

  const controls = ((): React.ReactNode => {
    switch (configuration.kind) {
      case 'frontdoor-two-stage':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Control value" help={ESTIMATION_PARAMETER_HELP.frontdoor.controlValue} /><input type="number" step="any" aria-label="Front-door control value" className={field('text', 'mt-1')} value={configuration.interventions[0]} onChange={(event) => configure({ ...configuration, interventions: [Number(event.target.value) || 0, configuration.interventions[1]] })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Treatment value" help={ESTIMATION_PARAMETER_HELP.frontdoor.treatmentValue} /><input type="number" step="any" aria-label="Front-door treatment value" className={field('text', 'mt-1')} value={configuration.interventions[1]} onChange={(event) => configure({ ...configuration, interventions: [configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap resamples" help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapResamples} /><input type="number" min={20} max={5000} aria-label="Front-door bootstrap resamples" className={field('text', 'mt-1')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapSeed} /><input type="number" min={0} aria-label="Front-door bootstrap seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className={prose('m-0 text-faint @md/panel:col-span-2 @4xl/panel:col-span-4')}>The first regression estimates treatment → mediator. The second estimates mediator → outcome while adjusting for treatment. Their product gives the linear front-door contrast; the interval uses a seeded row bootstrap.</p>
          </div>
        )
      case 'instrumental-variable':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap resamples" help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapResamples} /><input type="number" min={20} max={5000} aria-label="Instrumental-variable bootstrap resamples" className={field('text', 'mt-1')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapSeed} /><input type="number" min={0} aria-label="Instrumental-variable bootstrap seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className={prose('m-0 text-faint @md/panel:col-span-2 @4xl/panel:col-span-4')}>The estimate is the ratio of the instrument’s effect on the outcome to its effect on the treatment: the Wald estimator for one binary instrument, a covariance ratio for one continuous instrument, and two-stage least squares otherwise. The effect is reported for the treatment set to 1 rather than 0; the interval uses a seeded row bootstrap.</p>
          </div>
        )
      case 'backdoor-linear-regression':
        return (
          <div>
            <ParameterLabel className={fieldLabel} label="Interval" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.interval} />
            <SegmentedControl className="mt-1" ariaLabel="Interval covariance" value={configuration.covariance} onChange={(covariance) => configure({ ...configuration, covariance })} options={[{ value: 'hac', label: 'Newey–West HAC' }, { value: 'classical', label: 'Classical' }]} />
            <p className={cn(fieldHint, 'max-w-[65ch]')}>95% confidence level. Heteroskedasticity and autocorrelation consistent (HAC) covariance uses a Bartlett kernel and a bandwidth of floor(4 (n/100)^(2/9)) lags. The classical interval assumes independent errors.</p>
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
            <label className="block"><ParameterLabel className={fieldLabel} label="Fold seed" help={ESTIMATION_PARAMETER_HELP.dml.foldSeed} /><input type="number" min={0} aria-label="Fold seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className={prose('m-0 self-end text-faint @md/panel:col-span-2')}>Five shuffled folds, 200 random-forest trees, minimum leaf 5, learner seed 7. The Sensitivity chapter repeats this fit at the same seed before its refuters.</p>
          </div>
        )
      case 't-learner':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Learner seed" help={ESTIMATION_PARAMETER_HELP.tLearner.learnerSeed} /><input type="number" min={0} aria-label="Learner seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <TLearnerUncertainty value={configuration.uncertainty} onChange={(uncertainty) => configure({ ...configuration, uncertainty })} />
            <p className={prose('m-0 self-end text-faint @md/panel:col-span-2')}>One random forest per treatment arm, with 200 trees and a minimum leaf size of 5. Bootstrap intervals refit both forests on resampled rows and take longer to calculate.</p>
          </div>
        )
      case 'ardl-pss':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Maximum lag" help={ESTIMATION_PARAMETER_HELP.ardl.maximumLag} /><input type="number" min={1} max={24} aria-label="Maximum lag" className={field('text', 'mt-1')} value={configuration.maxLag} onChange={(event) => configure({ ...configuration, maxLag: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
            <div>
              <ParameterLabel className={fieldLabel} label="Deterministic terms" help={ESTIMATION_PARAMETER_HELP.ardl.deterministicTerms} />
              <SegmentedControl className="mt-1" fill ariaLabel="Deterministic terms" value={configuration.trend} onChange={(trend) => configure({ ...configuration, trend, case: trend === 'c' ? 3 : 4 })} options={[{ value: 'c', label: 'Constant' }, { value: 'ct', label: 'Constant and trend' }]} />
            </div>
            <div>
              <ParameterLabel className={fieldLabel} label="PSS case" help={ESTIMATION_PARAMETER_HELP.ardl.pssCase} />
              <SegmentedControl className="mt-1" fill ariaLabel="PSS case" value={String(configuration.case)} onChange={(chosen) => { const candidate = PSS_CASES[configuration.trend].find((item) => String(item) === chosen); if (candidate !== undefined) configure({ ...configuration, case: candidate }) }} options={PSS_CASES[configuration.trend].map((candidate) => ({ value: String(candidate), label: `Case ${candidate}` }))} />
            </div>
            <p className={prose('m-0 self-end text-faint')}>AIC lag search, error-correction fit, delta-method interval on the long-run effect, bounds test at the recorded case.</p>
          </div>
        )
      case 'vecm':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Maximum lags" help={ESTIMATION_PARAMETER_HELP.vecm.maximumLags} /><input type="number" min={1} max={24} aria-label="Maximum lags" className={field('text', 'mt-1')} value={configuration.maxLags} onChange={(event) => configure({ ...configuration, maxLags: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
            <div>
              <ParameterLabel className={fieldLabel} label="Deterministic terms" help={ESTIMATION_PARAMETER_HELP.vecm.deterministicTerms} />
              <Select className={field('text', 'mt-1')} aria-label="VECM deterministic terms" value={configuration.deterministic} onChange={(event) => { const term = VECM_TERMS.find(([value]) => value === event.target.value); if (term !== undefined) configure({ ...configuration, deterministic: term[0] }) }}>
                {VECM_TERMS.map(([value, name]) => <option key={value} value={value}>{name}</option>)}
              </Select>
            </div>
            <div>
              <ParameterLabel className={fieldLabel} label="Trace significance" help={ESTIMATION_PARAMETER_HELP.vecm.traceSignificance} />
              <SegmentedControl className="mt-1" fill ariaLabel="Trace significance" value={String(configuration.significance)} onChange={(chosen) => { const level = TRACE_LEVELS.find((item) => String(item) === chosen); if (level !== undefined) configure({ ...configuration, significance: level }) }} options={TRACE_LEVELS.map((level) => ({ value: String(level), label: `${level}%` }))} />
            </div>
            <label className="block"><ParameterLabel className={fieldLabel} label="Chow split after row" help={ESTIMATION_PARAMETER_HELP.vecm.chowBreakRow} /><input type="number" min={4} max={prepared.observations - 4} aria-label="Chow split after row" placeholder="none" className={field('text', 'mt-1')} value={configuration.breakIndex ?? ''} onChange={(event) => configure({ ...configuration, breakIndex: event.target.value === '' ? null : Math.max(4, Math.min(prepared.observations - 4, Math.floor(Number(event.target.value) || 4))) })} /></label>
          </div>
        )
      case 'synthetic-control':
        return (
          <div className="grid gap-3">
            <div>
              <ParameterLabel className={fieldLabel} label="Intervention start" help={ESTIMATION_PARAMETER_HELP.syntheticControl.interventionStart} />
              <div className="mt-1 flex flex-wrap items-center gap-2">
                <RadioList legend="Synthetic intervention start" legendHidden value={configuration.start.kind} onChange={(kind) => configure({ ...configuration, start: kind === 'from-treatment' ? { kind: 'from-treatment' } : { kind: 'row', row: Math.max(3, Math.floor(prepared.observations / 2)) } })} options={[{ value: 'from-treatment', label: `Where ${study?.treatment.name ?? 'the treatment'} turns on` }, { value: 'row', label: 'At a row' }]} />
                {configuration.start.kind === 'row' && (
                  <label className="text-body text-ink">First post-intervention row<input type="number" min={3} max={prepared.observations} aria-label="First post-intervention row" className={field('text', 'ml-2 w-28')} value={configuration.start.row} onChange={(event) => configure({ ...configuration, start: { kind: 'row', row: Math.max(3, Math.min(prepared.observations, Number(event.target.value) || 3)) } })} /></label>
                )}
              </div>
            </div>
            <div>
              <div className="flex flex-wrap items-center justify-between gap-2">
                <ParameterLabel className={fieldLabel} label="Donor series" help={ESTIMATION_PARAMETER_HELP.syntheticControl.donorSeries} />
                {controlCandidates.length > 0 && (
                  <div className="flex items-center gap-2">
                    <button type="button" className={button('quiet')} onClick={() => configure({ ...configuration, donors: controlCandidates.map((column) => column.id) })}>Select all</button>
                    <button type="button" className={button('quiet')} onClick={() => configure({ ...configuration, donors: [] })}>Clear</button>
                  </div>
                )}
              </div>
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
            <div className="grid gap-3 @md/panel:grid-cols-2">
              <label className="block"><ParameterLabel className={fieldLabel} label="Cross-fit folds" help={ESTIMATION_PARAMETER_HELP.syntheticControl.crossFitFolds} /><input type="number" min={2} max={20} aria-label="Cross-fit folds" className={field('text', 'mt-1')} value={configuration.crossFitFolds} onChange={(event) => configure({ ...configuration, crossFitFolds: Math.max(2, Math.min(20, Math.floor(Number(event.target.value) || 2))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Inference alpha" help={ESTIMATION_PARAMETER_HELP.syntheticControl.inferenceAlpha} /><input type="number" min={0.001} max={0.5} step={0.01} aria-label="Synthetic-control inference alpha" className={field('text', 'mt-1')} value={configuration.alpha} onChange={(event) => configure({ ...configuration, alpha: Math.max(0.001, Math.min(0.5, Number(event.target.value) || 0.05)) })} /></label>
            </div>
          </div>
        )
      case 'panel-intervention':
        return (
          <div className="grid gap-2">
            <p className={prose('m-0 text-faint')}>
              Hirmos uses synthetic difference-in-differences as the main result and shows conventional difference-in-differences and synthetic control beside it. The main method is fixed before the estimates appear.
            </p>
            <div className="grid gap-3 @md/panel:grid-cols-2">
              <label className="block"><ParameterLabel className={fieldLabel} label="Placebo replications" help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboReplications} /><input type="number" min={2} max={2000} aria-label="Panel placebo replications" className={field('text', 'mt-1')} value={configuration.placeboReplications} onChange={(event) => configure({ ...configuration, placeboReplications: Math.max(2, Math.min(2000, Math.floor(Number(event.target.value) || 2))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Placebo seed" help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboSeed} /><input type="number" min={0} max={0xffff_ffff} aria-label="Panel placebo seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.min(0xffff_ffff, Math.floor(Number(event.target.value) || 0))) })} /></label>
            </div>
            {panelPreflight.kind === 'pending' && <p className="m-0 text-body text-muted">Checking treatment timing, treated and control units, pre/post periods, and control pre-period variation…</p>}
            {panelPreflight.kind === 'ready' && <p className="m-0 text-body text-muted">Ready: {panelPreflight.layout.treated.length} treated and {panelPreflight.layout.controls.length} control units · {panelPreflight.layout.prePeriods} pre- and {panelPreflight.layout.postPeriods} post-periods · adoption at {panelPreflight.layout.adoption.label}.</p>}
          </div>
        )
      case 'negbin-nuts':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Warmup" help={ESTIMATION_PARAMETER_HELP.nuts.warmup} /><input type="number" min={10} max={5000} aria-label="Warmup" className={field('text', 'mt-1')} value={configuration.warmup} onChange={(event) => configure({ ...configuration, warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Draws" help={ESTIMATION_PARAMETER_HELP.nuts.draws} /><input type="number" min={10} max={5000} aria-label="Draws" className={field('text', 'mt-1')} value={configuration.samples} onChange={(event) => configure({ ...configuration, samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Seed" help={ESTIMATION_PARAMETER_HELP.nuts.seed} /><input type="number" min={0} aria-label="Sampler seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className="m-0 self-end text-body text-faint">Gamma-Poisson likelihood, standardised treatment and confounder, NUTS with step-size and diagonal mass adaptation at target acceptance 0.8.</p>
          </div>
        )
      case 'bayesian-gaussian':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="Warmup" help={ESTIMATION_PARAMETER_HELP.nuts.warmup} /><input type="number" min={10} max={5000} aria-label="Warmup" className={field('text', 'mt-1')} value={configuration.warmup} onChange={(event) => configure({ ...configuration, warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Draws per chain" help={ESTIMATION_PARAMETER_HELP.nuts.draws} /><input type="number" min={10} max={5000} aria-label="Draws per chain" className={field('text', 'mt-1')} value={configuration.samples} onChange={(event) => configure({ ...configuration, samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Seed" help={ESTIMATION_PARAMETER_HELP.nuts.seed} /><input type="number" min={0} aria-label="Sampler seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className="m-0 self-end text-body text-faint">Normal(0, 10) intercepts, Normal(0, 1) slopes, half-normal(10) residual scale; non-binary adjustment columns are standardised; three NUTS chains with step-size and diagonal mass adaptation at target acceptance 0.8.</p>
          </div>
        )
      case 'discrete-bn-query':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <label className="block"><ParameterLabel className={fieldLabel} label="State budget" help={ESTIMATION_PARAMETER_HELP.discreteBn.stateBudget} /><input type="number" min={2} max={10} aria-label="State budget" className={field('text', 'mt-1')} value={configuration.bins} onChange={(event) => configure({ ...configuration, bins: Math.max(2, Math.min(10, Math.floor(Number(event.target.value) || 2))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Equivalent sample size" help={ESTIMATION_PARAMETER_HELP.discreteBn.equivalentSampleSize} /><input type="number" min={0.1} step="any" aria-label="Equivalent sample size" className={field('text', 'mt-1')} value={configuration.equivalentSampleSize} onChange={(event) => configure({ ...configuration, equivalentSampleSize: Math.max(0.1, Number(event.target.value) || 0.1) })} /></label>
            <p className={prose('m-0 self-end text-faint @md/panel:col-span-2')}>Observed binary and ordinal states are preserved when they fit the budget; higher-cardinality values are divided at quantiles. The BDeu prior smooths the conditional tables, and the effect contrasts the lowest and highest treatment states.</p>
          </div>
        )
      case 'binary-ett-idc-star':
        return <p className="m-0 text-body text-faint">The run evaluates the two recorded IDC* expressions against the empirical binary joint distribution. It applies no discretisation and reports no sampling interval.</p>
      case 'poisson-glm':
      case 'negative-binomial-p':
        return <p className="m-0 text-body text-faint">Log link on the expected count of {study?.outcome.name ?? 'the outcome'}; exponentiating the treatment coefficient gives an expected-count ratio with a 95% normal interval.</p>
      case 'negative-binomial-ingarch':
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <div><ParameterLabel className={fieldLabel} label="Mean link" help={ESTIMATION_PARAMETER_HELP.ingarch.meanLink} /><SegmentedControl className="mt-1" ariaLabel="INGARCH mean link" value={configuration.link} onChange={(link) => configure({ ...configuration, link })} options={[{ value: 'identity', label: 'Additive' }, { value: 'log', label: 'Multiplicative' }]} /></div>
            <LagListField label="Past count lags" help={ESTIMATION_PARAMETER_HELP.ingarch.pastCountLags} lags={configuration.pastObservationLags} onChange={(pastObservationLags) => configure({ ...configuration, pastObservationLags })} />
            <LagListField label="Past mean lags" help={ESTIMATION_PARAMETER_HELP.ingarch.pastMeanLags} lags={configuration.pastMeanLags} onChange={(pastMeanLags) => configure({ ...configuration, pastMeanLags })} />
            <label className="block"><ParameterLabel className={fieldLabel} label="Forecast periods" help={ESTIMATION_PARAMETER_HELP.ingarch.forecastPeriods} /><input type="number" min={1} max={240} className={field('text', 'mt-1')} value={configuration.horizon} onChange={(event) => configure({ ...configuration, horizon: Math.max(1, Math.min(240, Math.floor(Number(event.target.value) || 1))) })} /></label>
            <div><ParameterLabel className={fieldLabel} label="Treatment schedule" help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentSchedule} /><SegmentedControl className="mt-1" ariaLabel="INGARCH treatment schedule" value={configuration.schedule.kind} onChange={(kind) => configure({ ...configuration, schedule: kind === 'decaying' ? { kind: 'decaying', delta: 0.6 } : { kind } })} options={[{ value: 'point', label: 'One period' }, { value: 'persistent', label: 'Persistent' }, { value: 'decaying', label: 'Decaying' }]} /></div>
            <label className="block"><ParameterLabel className={fieldLabel} label="Control value" help={ESTIMATION_PARAMETER_HELP.ingarch.controlValue} /><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.controlValue} onChange={(event) => configure({ ...configuration, controlValue: Number(event.target.value) || 0 })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Treatment value" help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentValue} /><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.treatmentValue} onChange={(event) => configure({ ...configuration, treatmentValue: Number(event.target.value) || 0 })} /></label>
            {configuration.schedule.kind === 'decaying' && <label className="block"><ParameterLabel className={fieldLabel} label="Decay δ" help={ESTIMATION_PARAMETER_HELP.ingarch.decay} /><input type="number" min={0} max={1} step={0.05} className={field('text', 'mt-1')} value={configuration.schedule.delta} onChange={(event) => configure({ ...configuration, schedule: { kind: 'decaying', delta: Math.max(0, Math.min(1, Number(event.target.value) || 0)) } })} /></label>}
            <p className={prose('m-0 text-faint @md/panel:col-span-2 @4xl/panel:col-span-4')}>The additive link expresses effects in expected counts and requires non-negative regressors. The multiplicative link expresses effects on the log expected count and permits signed regressors. Both use the treatment and identified same-period adjustment variables with the selected count and mean lags; no sampling interval is reported.</p>
          </div>
        )
      case 'causal-effects-total': {
        const bootstrap = configuration.uncertainty.kind === 'bootstrap' ? configuration.uncertainty : null
        const adjustedEstimator = configuration.estimator.kind === 'wrightParents' ? null : configuration.estimator
        const explicitAdjustment = adjustedEstimator?.adjustment.kind === 'explicit' ? adjustedEstimator.adjustment : null
        const setAdjustment = (adjustment: CausalEffectsAdjustment) => {
          if (adjustedEstimator === null) return
          configure({ ...configuration, estimator: { ...adjustedEstimator, adjustment } })
        }
        const setNeighbours = (k: number) => {
          if (adjustedEstimator?.kind !== 'knn') return
          configure({ ...configuration, estimator: { ...adjustedEstimator, k } })
        }
        return (
          <div className="grid gap-3 @md/panel:grid-cols-2 @4xl/panel:grid-cols-4">
            <div>
              <ParameterLabel className={fieldLabel} label="Effect model" help={ESTIMATION_PARAMETER_HELP.causalEffects.effectModel} />
              <SegmentedControl className="mt-1" ariaLabel="CausalEffects model" value={configuration.estimator.kind} onChange={(kind) => configure({ ...configuration, estimator: totalEffectEstimatorFromKind(kind, configuration.estimator) })} options={[{ value: 'linear', label: 'Adjusted linear' }, { value: 'knn', label: 'Adjusted k-NN' }, { value: 'wrightParents', label: 'Wright paths' }]} />
            </div>
            {configuration.estimator.kind === 'knn' && (
              <label className="block"><ParameterLabel className={fieldLabel} label="Neighbours k" help={ESTIMATION_PARAMETER_HELP.causalEffects.neighbours} /><input type="number" min={1} max={100} className={field('text', 'mt-1')} value={configuration.estimator.k} onChange={(event) => setNeighbours(Math.max(1, Math.min(100, Number(event.target.value) || 1)))} /></label>
            )}
            {configuration.estimator.kind === 'wrightParents' && (
              <p className={prose('m-0 self-end text-faint @md/panel:col-span-2')}>Fits each node on its time-indexed parents, then sums products of coefficients along directed treatment-to-outcome paths. The result separates direct and indirect path contributions.</p>
            )}
            {adjustedEstimator !== null && (
              <div className="@md/panel:col-span-2">
                <label className="block">
                  <ParameterLabel className={fieldLabel} label="Time-indexed adjustment set" help={ESTIMATION_PARAMETER_HELP.causalEffects.adjustmentSet} />
                  <Select className={field('text', 'mt-1')} value={adjustedEstimator.adjustment.kind} onChange={(event) => {
                    setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
                    switch (event.target.value) {
                      case 'optimal': setAdjustment({ kind: 'optimal' }); break
                      case 'minimizedOptimal': setAdjustment({ kind: 'minimizedOptimal' }); break
                      case 'collidersMinimizedOptimal': setAdjustment({ kind: 'collidersMinimizedOptimal' }); break
                      case 'explicit': setAdjustment({ kind: 'explicit', nodes: [] }); break
                    }
                  }}>
                    <option value="optimal">Complete O-set</option>
                    <option value="collidersMinimizedOptimal">Collider-minimized O-set</option>
                    <option value="minimizedOptimal">Minimized O-set</option>
                    <option value="explicit">User-supplied set</option>
                  </Select>
                </label>
                <p className={cn(fieldHint, 'mt-1 max-w-[72ch]')}>The generated choices are computed from the stationary graph. A user-supplied set is checked against every open non-causal treatment–outcome path before fitting.</p>
              </div>
            )}
            {explicitAdjustment !== null && (
              <div className="grid gap-2 @md/panel:col-span-2 @4xl/panel:col-span-4">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <ParameterLabel className={fieldLabel} label="Adjustment members" help={ESTIMATION_PARAMETER_HELP.causalEffects.adjustmentMembers} />
                  <button type="button" className={button('quiet')} disabled={temporalAdjustmentCandidates.length === 0 || adjustmentDraft.kind === 'editing'} onClick={() => setAdjustmentDraft({ kind: 'editing', variable: null, lag: 0 })}>Add member</button>
                </div>
                {explicitAdjustment.nodes.length === 0 && <p className="m-0 text-body text-faint">The empty set will be tested. It is valid only when the graph has no open non-causal treatment–outcome path.</p>}
                {adjustmentDraft.kind === 'editing' && (
                  <div className={well('grid grid-cols-[minmax(0,1fr)_8rem_auto_auto] items-end gap-2 p-2')}>
                    <label className="block">
                      <ParameterLabel className={fieldLabel} label="Variable" help={ESTIMATION_PARAMETER_HELP.causalEffects.variable} />
                      <Select className={field('text', 'mt-1')} value={adjustmentDraft.variable ?? ''} onChange={(event) => setAdjustmentDraft({ ...adjustmentDraft, variable: event.target.value === '' ? null : Number(event.target.value) })}>
                        <option value="">Choose a variable</option>
                        {temporalAdjustmentCandidates.map((candidate) => <option key={candidate.index} value={candidate.index}>{candidate.name}</option>)}
                      </Select>
                    </label>
                    <label className="block"><ParameterLabel className={fieldLabel} label="Lag" help={ESTIMATION_PARAMETER_HELP.causalEffects.lag} /><input type="number" min={0} max={temporalAdjustmentMaxLag} className={field('text', 'mt-1')} value={adjustmentDraft.lag} onChange={(event) => setAdjustmentDraft({ ...adjustmentDraft, lag: Math.max(0, Math.min(temporalAdjustmentMaxLag, Math.floor(Number(event.target.value) || 0))) })} /></label>
                    <button type="button" className={button('quiet')} disabled={adjustmentDraft.variable === null || explicitAdjustment.nodes.some((node) => node[0] === adjustmentDraft.variable && node[1] === -adjustmentDraft.lag)} onClick={() => {
                      if (adjustmentDraft.variable === null) return
                      setAdjustment({ kind: 'explicit', nodes: [...explicitAdjustment.nodes, [adjustmentDraft.variable, -adjustmentDraft.lag]] })
                      setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)
                    }}>Add</button>
                    <button type="button" className={button('quiet')} onClick={() => setAdjustmentDraft(CLOSED_ADJUSTMENT_DRAFT)}>Cancel</button>
                  </div>
                )}
                {explicitAdjustment.nodes.map((node, index) => (
                  <div key={`${index}-${node[0]}-${node[1]}`} className="grid grid-cols-[minmax(0,1fr)_8rem_auto] items-end gap-2">
                    <label className="block">
                      <ParameterLabel className={fieldLabel} label="Variable" help={ESTIMATION_PARAMETER_HELP.causalEffects.variable} />
                      <Select className={field('text', 'mt-1')} value={node[0]} onChange={(event) => setAdjustment({ kind: 'explicit', nodes: explicitAdjustment.nodes.map((item, position) => position === index ? [Number(event.target.value), item[1]] : item) })}>
                        {temporalAdjustmentCandidates.map((candidate) => <option key={candidate.index} value={candidate.index}>{candidate.name}</option>)}
                      </Select>
                    </label>
                    <label className="block"><ParameterLabel className={fieldLabel} label="Lag" help={ESTIMATION_PARAMETER_HELP.causalEffects.lag} /><input type="number" min={0} max={temporalAdjustmentMaxLag} className={field('text', 'mt-1')} value={-node[1]} onChange={(event) => setAdjustment({ kind: 'explicit', nodes: explicitAdjustment.nodes.map((item, position) => position === index ? [item[0], -Math.max(0, Math.min(temporalAdjustmentMaxLag, Math.floor(Number(event.target.value) || 0)))] : item) })} /></label>
                    <button type="button" className={button('quiet')} onClick={() => setAdjustment({ kind: 'explicit', nodes: explicitAdjustment.nodes.filter((_, position) => position !== index) })}>Remove</button>
                  </div>
                ))}
              </div>
            )}
            <label className="block"><ParameterLabel className={fieldLabel} label="Treatment lag" help={ESTIMATION_PARAMETER_HELP.causalEffects.treatmentLag} /><input type="number" min={0} max={20} className={field('text', 'mt-1')} value={configuration.treatmentLag} onChange={(event) => configure({ ...configuration, treatmentLag: Math.max(0, Math.min(20, Number(event.target.value) || 0)) })} /></label>
            <div className="grid grid-cols-2 gap-2">
              <label className="block"><ParameterLabel className={fieldLabel} label="From value" help={ESTIMATION_PARAMETER_HELP.causalEffects.fromValue} /><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.interventions[0]} onChange={(event) => configure({ ...configuration, interventions: [Number(event.target.value) || 0, configuration.interventions[1]] })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="To value" help={ESTIMATION_PARAMETER_HELP.causalEffects.toValue} /><input type="number" step="any" className={field('text', 'mt-1')} value={configuration.interventions[1]} onChange={(event) => configure({ ...configuration, interventions: [configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
            </div>
            <div>
              <ParameterLabel className={fieldLabel} label="Sampling uncertainty" help={ESTIMATION_PARAMETER_HELP.causalEffects.uncertainty} />
              <SegmentedControl className="mt-1" ariaLabel="CausalEffects sampling uncertainty" value={configuration.uncertainty.kind} onChange={(kind) => configure({ ...configuration, uncertainty: kind === 'none' ? { kind: 'none' } : { kind: 'bootstrap', samples: 100, blockLength: { kind: 'fixed', length: 1 }, confidenceLevel: 0.9, seed: 4 } })} options={[{ value: 'bootstrap', label: 'Block bootstrap' }, { value: 'none', label: 'Point estimate' }]} />
            </div>
            {bootstrap !== null && (
              <>
                <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap samples" help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSamples} /><input type="number" min={20} max={5000} aria-label="CausalEffects bootstrap samples" className={field('text', 'mt-1')} value={bootstrap.samples} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, samples: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) } })} /></label>
                <div>
                  <ParameterLabel className={fieldLabel} label="Block length" help={ESTIMATION_PARAMETER_HELP.causalEffects.blockLength} />
                  <SegmentedControl className="mt-1" ariaLabel="CausalEffects block length policy" value={bootstrap.blockLength.kind} onChange={(kind) => configure({ ...configuration, uncertainty: { ...bootstrap, blockLength: kind === 'fixed' ? { kind: 'fixed', length: 1 } : { kind: 'cubeRoot' } } })} options={[{ value: 'fixed', label: 'Fixed' }, { value: 'cubeRoot', label: 'Cube root' }]} />
                </div>
                {bootstrap.blockLength.kind === 'fixed' && <label className="block"><ParameterLabel className={fieldLabel} label="Observations per block" help={ESTIMATION_PARAMETER_HELP.causalEffects.observationsPerBlock} /><input type="number" min={1} aria-label="CausalEffects observations per block" className={field('text', 'mt-1')} value={bootstrap.blockLength.length} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, blockLength: { kind: 'fixed', length: Math.max(1, Math.floor(Number(event.target.value) || 1)) } } })} /></label>}
                <label className="block"><ParameterLabel className={fieldLabel} label="Confidence level" help={ESTIMATION_PARAMETER_HELP.causalEffects.confidenceLevel} /><input type="number" min={50} max={99.9} step={0.1} aria-label="CausalEffects confidence level" className={field('text', 'mt-1')} value={bootstrap.confidenceLevel * 100} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, confidenceLevel: Math.max(0.5, Math.min(0.999, (Number(event.target.value) || 90) / 100)) } })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSeed} /><input type="number" min={0} aria-label="CausalEffects bootstrap seed" className={field('text', 'mt-1')} value={bootstrap.seed} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) } })} /></label>
                <p className={prose('m-0 text-faint @md/panel:col-span-2 @4xl/panel:col-span-4')}>Contiguous blocks preserve the lag alignment used by the fitted graph. Choose a block length that represents the series’ dependence; the cube-root option takes the cube root of the row count.</p>
              </>
            )}
          </div>
        )
      }
      case 'causal-impact':
        return (
          <div className="grid gap-3">
            <div>
              <ParameterLabel className={fieldLabel} label="Intervention start" help={ESTIMATION_PARAMETER_HELP.causalImpact.interventionStart} />
              <div className="mt-1 flex flex-wrap items-center gap-2">
                <RadioList legend="Intervention start" legendHidden value={configuration.start.kind} onChange={(kind) => configure({ ...configuration, start: kind === 'from-treatment' ? { kind: 'from-treatment' } : { kind: 'row', row: Math.max(9, Math.floor(prepared.observations / 2)) } })} options={[{ value: 'from-treatment', label: `Where ${study?.treatment.name ?? 'the treatment'} turns on` }, { value: 'row', label: 'At a row' }]} />
                {configuration.start.kind === 'row' && (
                  <label className="text-body text-ink">First post-intervention row<input type="number" min={9} max={prepared.observations} aria-label="First post-intervention row" className={field('text', 'ml-2 w-28')} value={configuration.start.row} onChange={(event) => configure({ ...configuration, start: { kind: 'row', row: Math.max(9, Math.min(prepared.observations, Number(event.target.value) || 9)) } })} /></label>
                )}
              </div>
            </div>
            <div>
              <ParameterLabel className={fieldLabel} label="Control series" help={ESTIMATION_PARAMETER_HELP.causalImpact.controlSeries} />
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
        <ChapterHeading id="estimation-title" className="mb-2">Estimation</ChapterHeading>
        <p className={chapterIntro}>Identification determines how the causal question can be expressed using observed data. Estimation applies a statistical method to that expression. In this chapter, choose a compatible estimator and examine the effect estimate, its uncertainty, and the method-specific diagnostics.</p>
      </div>

      <section className={panel('p-(--panel-space)')} aria-labelledby="estimation-setup-title">
        <h3 id="estimation-setup-title" className={cn(sectionTitle, 'mb-3 mt-0')}>{method.ok ? method.value.name : 'Estimator'}</h3>
        {identified.length === 0 ? (
          <Alert tone="info" live={false}>
            <p className="m-0"><button type="button" className="underline" onClick={onOpenStudy}>Open Study design</button> and identify a study first.</p>
          </Alert>
        ) : (
          <>
            <div className="grid grid-cols-1 gap-4">
              <label className="block">
                <span className={fieldLabel}>Identified study</span>
                <Select className={field('text', 'mt-1')} value={state.identification ?? ''} onChange={(event) => dispatch({ type: 'identification-chosen', selection: estimationSelection(identified.find((candidate) => candidate.id === event.target.value) ?? null, studies, prepared) })}>
                {identified.map((candidate) => {
                  const bound = studies.find((item) => item.id === candidate.study)
                  return <option key={candidate.id} value={candidate.id}>{bound === undefined ? candidate.id : `${estimandSentence(bound)} · ${bound.dagName}`}</option>
                })}
              </Select>
                {identification !== null && identification.result.kind === 'identified' && <span className={cn(fieldHint, 'block max-w-[65ch]')}>Adjustment set: {identification.result.adjustment.variables.length === 0 ? 'none' : identification.result.adjustment.variables.map((variable) => variable.name).join(', ')} · {formatCount(study?.population.observations ?? 0).text} rows</span>}
                {identification !== null && identification.result.kind === 'graphically-identified' && identification.result.frontdoor.kind === 'identified' && <span className={cn(fieldHint, 'block max-w-[65ch]')}>Front-door mediator: {identification.result.frontdoor.mediators.map((variable) => variable.name).join(', ')} · {formatCount(study?.population.observations ?? 0).text} rows</span>}
                {identification !== null && identifiedInstruments(identification.result) !== null && <span className={cn(fieldHint, 'block max-w-[65ch]')}>Instruments: {(identifiedInstruments(identification.result) ?? []).map((variable) => variable.name).join(', ')} · {formatCount(study?.population.observations ?? 0).text} rows</span>}
              </label>
              <div>
                <SegmentedControl
                  variant="line"
                  size="sm"
                  ariaLabel="Estimator family"
                  value={visibleEstimatorGroup}
                  onChange={setVisibleEstimatorGroup}
                  options={ESTIMATOR_GROUPS.map((group) => ({ value: group.id, label: ESTIMATOR_GROUP_LABELS[group.id], title: group.name }))}
                />
                <div className="mb-2 mt-3">
                  <h3 className="m-0 text-body font-medium text-ink">{visibleGroup.name}</h3>
                  <p className="mb-0 mt-0.5 max-w-[65ch] text-label text-faint">{visibleGroup.description}</p>
                </div>
                <RadioList frame="none"
                  columns={2}
                  legend={visibleGroup.name}
                  legendHidden
                  value={state.estimator}
                  onChange={(estimator) => dispatch({ type: 'estimator-chosen', estimator })}
                  options={visibleGroup.estimators.flatMap((id) => {
                    const definition = methodDefinition(methodIdOf(id))
                    const candidateEligibility = eligibilityByEstimator.get(id)
                    return definition.ok && candidateEligibility !== undefined
                      ? [{ value: id, label: definition.value.name, hint: eligibilityHint(candidateEligibility), disabled: candidateEligibility.kind === 'refused', title: candidateEligibility.kind === 'refused' ? `${definition.value.name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}` : undefined }]
                      : []
                  })}
                />
                {!selectedEstimatorIsVisible && <p className={cn(fieldHint, 'mt-3')}>Choose a method from this family to configure it.</p>}
                {selectedEstimatorIsVisible && method.ok && <p className={cn(fieldHint, 'mt-3 max-w-[65ch]')}>{method.value.summary}</p>}
                {selectedEstimatorIsVisible && method.ok && method.value.summaryTex !== undefined && <div className="formula max-w-[65ch] text-body"><Formula {...method.value.summaryTex} /></div>}
              </div>
              {selectedEstimatorIsVisible && <div>{controls}</div>}
            </div>
            {selectedEstimatorIsVisible && eligibility !== null && <EligibilityView eligibility={eligibility} subject="this study" />}
            {studyDataError !== null && <Alert tone="danger" className="mt-3"><p className="m-0">The treatment and outcome columns could not be checked: {studyDataError}</p></Alert>}
            {state.job.kind === 'failed' && <Alert tone="danger" className="mt-3"><p className="m-0">The estimate could not run: {state.job.detail}</p></Alert>}
            <div className="mt-4 flex items-center gap-3">
              <button type="button" className={button('signal')} disabled={!selectedEstimatorIsVisible || identification === null || eligibility === null || eligibility.kind === 'refused' || studyDataPending || studyDataError !== null || adjustmentDraftOpen || (configuration.kind === 'panel-intervention' && panelPreflight.kind !== 'ready')} aria-busy={state.job.kind === 'running'} onClick={state.job.kind === 'running' ? undefined : () => void execute()}>
                {!selectedEstimatorIsVisible ? 'Choose a method' : studyDataPending ? 'Checking treatment and outcome…' : configuration.kind === 'panel-intervention' && panelPreflight.kind === 'pending' ? 'Checking panel…' : `Run ${lowerFirst(describeEstimator(state.estimator))}`}
              </button>
              {state.job.kind === 'running' && <Orb state="solving" aria-label="Estimator running" />}
            </div>
          </>
        )}
      </section>

      {latestRun !== null && (
        <section aria-labelledby="estimation-results-title" className="grid grid-cols-1 gap-4">
          <div>
            <h2 id="estimation-results-title" className={cn(sectionTitle, 'm-0')}>Estimates</h2>
          </div>
          {(() => {
            const bound = studies.find((candidate) => candidate.id === latestRun.study)
            return bound === undefined ? null : <ResultCard run={latestRun} study={bound} current stepLabel={stepLabel} others={runs} />
          })()}
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
        {study !== null && identification !== null && estimableIdentification(identification.result) && (
          <>
            <dl className="m-0 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-body" aria-label="Study binding">
              <dt className="text-faint">Treatment</dt><dd className="m-0 text-ink">{study.treatment.name}</dd>
              <dt className="text-faint">Outcome</dt><dd className="m-0 text-ink">{study.outcome.name}</dd>
              <dt className="text-faint">Graph</dt><dd className="m-0 text-ink">{study.dagName} · <span className={literal()}>{study.dagRevision.slice(0, 8)}</span></dd>
              <dt className="text-faint">Strategy</dt><dd className="m-0 text-ink">{describeIdentificationStrategy(identification.result)}</dd>
              <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}>{prepared.kind === 'prepared-time-series' ? 'Time series' : prepared.kind === 'prepared-panel' ? 'Panel' : 'Independent'} · {formatCount(prepared.observations).text}</dd>
              {studyScale !== null && <><dt className="text-faint">Analysis scale</dt><dd className="m-0 text-ink">{studyScale}</dd></>}
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

  // Runs for the selected study whose additive estimates share one axis; the newest carries the accent.
  const comparison = useMemo(() => {
    const comparable = runs.filter((run) => run.study === identification?.study && run.estimate.effect.kind === 'additive')
    if (comparable.length < 2) return null
    const rows: RunComparisonRow[] = comparable.map((run, index) => ({
      label: `${describeEstimator(run.configuration.kind)}${run.kind === 'backdoor-linear-run' ? ` · ${describeCovariance(run.configuration.covariance)}` : ''} · ${formatTime(run.createdAt)}`,
      estimate: run.estimate.effect.kind === 'additive' ? run.estimate.effect.value : 0,
      lower: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.lower,
      upper: run.estimate.interval.kind === 'none' ? null : run.estimate.interval.upper,
      current: index === comparable.length - 1,
    }))
    return { option: runComparisonOption(rows, chartTheme), height: rows.length * 30 + 56 }
  }, [runs, identification, chartTheme])

  const ledger = (
    <>
      {comparison !== null && (
        <div className="border-b border-hair px-3 pb-1 pt-2">
          <EChart option={comparison.option} label="Recorded estimates compared on one axis" className="w-full" style={{ height: comparison.height }} testId="run-comparison" />
        </div>
      )}
      <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Estimation runs">
        {runs.length === 0 && <li className="px-3 py-2 text-faint">Choose an estimator and run it.</li>}
        {[...runs].reverse().map((run) => {
          const bound = studies.find((candidate) => candidate.id === run.study)
          return bound === undefined ? null : (
            <ResultCard key={run.id} run={run} study={bound} current={false} stepLabel={stepLabel} onDelete={() => setPendingDelete(run)} />
          )
        })}
      </ul>
    </>
  )
  const dependentProbes = pendingDelete === null ? 0 : sensitivityRuns.filter((probe) => probe.estimationRun === pendingDelete.id).length

  const deleteDialog = (
    <ConfirmDialog
      open={pendingDelete !== null}
      title="Delete this estimate?"
      danger
      confirmLabel="Delete run"
      message={pendingDelete === null ? '' : `Removes the ${describeEstimator(pendingDelete.configuration.kind)} run${dependentProbes > 0 ? ` and the ${dependentProbes} sensitivity ${dependentProbes === 1 ? 'probe' : 'probes'} recorded against it` : ''}. Recorded results cannot be restored.`}
      onConfirm={() => { if (pendingDelete !== null) onDeleteRun(pendingDelete.id) }}
      onClose={() => setPendingDelete(null)}
    />
  )

  return (
    <>
    <WorkbenchLayout
      id="estimation"
      stage={stage}
      inspector={{ title: 'Study and method requirements', body: inspector }}
      bottom={{ title: `Runs (${runs.length})`, body: <>{ledger}{deleteDialog}</>, defaultSize: comparison === null ? 150 : 150 + comparison.height }}
    />
    </>
  )
}
