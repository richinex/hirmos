import { RunDetails } from '@/components/ui/RunDetails'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { useWorkflow } from '@/components/WorkflowProvider'
import { initialEstimationDraft, estimationSelection, samePanelBinding, sameStudyDataBinding, type EstimationEvent, type PanelBinding, type StudyDataBinding } from '@/domain/estimationDraft'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { SelectionActions } from '@/components/ui/SelectionActions'
import { Metadata } from '@/components/ui/Metadata'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { TLearnerUncertainty } from './TLearnerUncertainty'
import { CausalForestControls } from './CausalForestControls'
import { CausalForestAnalysisControls, CausalForestSamplingControls } from './CausalForestAnalysisControls'
import { forestAnalysisColumns, type ForestAnalysis } from '@/domain/causalForestAnalysis'
import { CausalForestResults } from './CausalForestResults'
import { causalForestInputs, causalForestTarget, parseCausalForestEvidence, type ForestFeature } from '@/domain/causalForest'
import { CausalImpactInference } from './CausalImpactInference'
import { SharpRdResult } from './SharpRdResult'
import { BayesianImpactResult } from './BayesianImpactResult'
import { ImpactEffectPanel } from './ImpactEffectPanel'
import { StaggeredDidControls } from './StaggeredDidControls'
import { StaggeredDidResult } from './StaggeredDidResult'
import { defaultStaggeredSpecification, staggeredInput } from '@/domain/staggeredDid'
import { AdjustedDidControls } from './AdjustedDidControls'
import { AdjustedDidResult } from './AdjustedDidResult'
import { describePanelDataProblem } from '@/domain/panel'
import { TLearnerIntervals } from './TLearnerIntervals'
import { Orb } from '@/components/ui/Orb'
import { EmptyState } from '@/components/ui/EmptyState'
import { Icon } from '@/components/Icon'
import { Select } from '@/components/ui/Select'
import { LagListField } from '@/components/ui/LagListField'
import { memo, useEffect, useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { ExpandableChart } from '@/charts/ExpandableChart'
import type { VisibleWindow } from '@/charts/window'
import { runBoostedGridSearch } from '@/analysis/boostedSearch'
import { BoostedSearchFields } from './BoostedSearchFields'
import { BootstrapIntervalFields } from './BootstrapIntervalFields'
import type { BoostedTreatmentModel, PropensityTreatmentModel } from '@/workers/analysisProtocol'
import { adjustedRegressionPlan } from '@/analysis/adjustedRegressionPlan'
import { histogramOption } from '@/charts/data/histogram'
import { counterfactualCurvesOption } from '@/charts/estimation/counterfactualCurves'
import { impactPathOption } from '@/charts/estimation/impactPath'
import { posteriorDensityOption } from '@/charts/estimation/posteriorDensity'
import { propensityDistributionOption } from '@/charts/estimation/propensityOverlap'
import { propensityWeightOption } from '@/charts/estimation/propensityWeights'
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
import { actionGap, button, chapterIntro, field, fieldHint, fieldLabel, fieldRow, label, literal, num, panel, prose, sectionTitle, stepsStack, well } from '@/components/ui/recipes'
import { SettingsStep } from '@/components/ui/SettingsStep'
import { cn } from '@/lib/utils'
import type { DagDocument } from '@/domain/dag'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { assertNever, err, isNonEmpty, mapNonEmpty, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import { describeDesignExpansionProblem, designLayouts, expandDesign, expandsDesign } from '@/domain/designMatrix'
import { ColumnChecklist } from '@/components/ui/ColumnChecklist'
import { DEFAULT_ARMA_ITERATIONS, MAX_ARMA_ORDER } from '@/domain/interruptedSeries'
import { ArmaUncertaintyAlert } from './ArmaUncertaintyAlert'
import {
  additive,
  adjustmentLabels,
  armaReading,
  boundsReading,
  causalEstimateFrom,
  contemporaneousAdjustmentVariables,
  type PropensityUncertainty,
  type LinearErrors,
  type FixedEffects,
  describeCovariance,
  describeFixedEffects,
  selectFixedEffects,
  describeDiscreteStatePreparations,
  describeEstimator,
  describeInstrumentalVariableRoute, dmlNuisanceInputs,
  boostedCommand,
  boostedGridCommand,
  boostedCandidateCount,
  DEFAULT_BOOSTED_SEARCH,
  effectiveSampleSize,
  weightSpread,
  type ArmChoice,
  type BoostedTreatmentModelChoice,
  ESTIMATOR_GROUPS,
  evaluateEstimatorEligibility,
  headlineValue,
  intervalTypeOf,
  interventionStartFromTreatment,
  evaluatedEnd,
  preInterventionPoints,
  methodIdOf,
  newEstimationRunId,
  stationaryMarksOf,
  summariseRowEffects,
  tLearnerInputs,
  tLearnerRowEffects,
  type CausalEffectsAdjustment,
  type CausalEffectsAdjustmentProblem,
  type CausalEstimate,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
  type EstimatorGroupId,
  type EstimatorId,
  type TotalEffectEstimator,
  type TreatmentModelEvidence,
} from '@/domain/estimation'
import { ESTIMATION_METHODS, methodDefinition, type MethodEligibility } from '@/domain/methods'
import { describeSeriesTransform, frequencyUnit, seriesTransformFor, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import {
  assessPanelInterventionLayout,
  type PanelInterventionPreflight,
} from '@/domain/panel'
import { describeIdentificationStrategy, estimableIdentification, estimandSentence, identifiedInstruments, type IdentificationArtifact, type IdentificationId, type StudySpecification, type StudyVariable } from '@/domain/study'
import type { SelectedSource } from '@/domain/workflow'
import { namesFigure, namesInProse } from '@/lib/format/names'
import { formatCount, formatEstimate, formatInterval, formatP, formatPercent, formatStatistic, formatWords, type Formatted } from '@/lib/format/number'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { lowerFirst } from '@/lib/text'
import { useRunActivity } from '@/lib/useRunActivity'
import { interpretEstimationResult, resultHeadline, resultSampleLine, resultScaleLine } from '@/domain/resultInterpretation'
import type { RunActivity } from '@/domain/activity'
import { describeAnalysisWorkerProblem, type AnalysisProgress, type AnalysisWorkerProblem, type DmlGroupsRequest } from '@/workers/analysisProtocol'
import { ESTIMATION_PARAMETER_HELP } from '@/domain/parameterHelp'

const PSS_CASES: Record<'c' | 'ct', readonly (2 | 3 | 4 | 5)[]> = { c: [2, 3], ct: [4, 5] }
const TRACE_LEVELS: readonly (90 | 95 | 99)[] = [90, 95, 99]
const VECM_TERMS = [['n', 'None'], ['co', 'Constant outside'], ['ci', 'Constant inside'], ['coli', 'Constant and trend']] as const

/** An adjustment set as a figure: named when short, counted when long. */
const adjustmentFigure = (names: readonly string[], none: string): Formatted =>
  formatWords(names.length === 0 ? none : namesFigure(names, 'variables').value)

const ESTIMATOR_GROUP_LABELS: Readonly<Record<EstimatorGroupId, string>> = {
  'adjusted-outcome': 'Adjustment',
  'propensity-score': 'Propensity score',
  'identified-functional': 'Identified',
  'graph-adjusted-temporal': 'Temporal graph',
  'dynamic-time-series': 'Count intervention',
  'intervention-comparison': 'Interventions',
}

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

type RunEvent =
  | { readonly type: 'run-progressed'; readonly progress: AnalysisProgress }
  | { readonly type: 'run-failed'; readonly detail: string }
  | { readonly type: 'run-finished' }

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
    case 'openNonCausalPath': return 'The complete set leaves open at least one path between treatment and outcome that is not causal.'
    default: return assertNever(problem)
  }
}).join(' ')

const intervalText = (estimate: CausalEstimate): string => {
  if (estimate.interval.kind === 'none') return 'none'
  const figure = formatInterval(headlineValue(estimate.effect), estimate.interval.lower, estimate.interval.upper, intervalTypeOf(estimate.interval), scaleOf(estimate))
  return `[${figure.bounds.lower}, ${figure.bounds.upper}]`
}


/** The HAC bandwidth rule the hint refers to, typeset like every other formula in the workbench. */
/** The HC1 small-sample factor the hint refers to; G counts the absorbed fixed effects when there are any. */
const hc1Factor = (absorbsEffects: boolean) => absorbsEffects
  ? { tex: 'c = \\dfrac{n}{n - G - k}', plain: 'c = n / (n − G − k)' }
  : { tex: 'c = \\dfrac{n}{n - k}', plain: 'c = n / (n − k)' }

const HAC_BANDWIDTH = { tex: 'L = \\left\\lfloor 4\\left(\\dfrac{n}{100}\\right)^{2/9} \\right\\rfloor', plain: 'L = floor(4 (n/100)^(2/9))' } as const

/** What the chosen error treatment assumes and how its interval is built, for the hint under the control. */
const describeErrorTreatment = (errors: LinearErrors, fixedEffects: FixedEffects): string => {
  const effects = fixedEffects.kind === 'none' ? null : fixedEffects.kind === 'unit-and-time' ? `${fixedEffects.name} and ${fixedEffects.timeName} effects` : `${fixedEffects.name} effects`
  switch (errors.kind) {
    case 'classical': return effects !== null
      ? `The interval uses a 95% confidence level. The classical interval assumes errors are independent and share a common variance; its degrees of freedom count the ${effects} as parameters.`
      : 'The interval uses a 95% confidence level. The classical interval assumes errors are independent and share a common variance.'
    case 'hc1': return effects !== null
      ? `The interval uses a 95% confidence level. The robust (HC1) interval allows the error variance to differ between rows and scales each squared residual by the factor c, where n is the number of rows and G counts the ${effects}:`
      : 'The interval uses a 95% confidence level. The robust (HC1) interval allows the error variance to differ between rows and scales each squared residual by the factor c, where n is the number of rows:'
    case 'cluster': return `The interval uses a 95% confidence level. The clustered interval allows errors to correlate within each value of ${errors.name} and treats the clusters as independent. A finite-sample correction accounts for the cluster count and fitted parameters. With two-way effects, both effect dimensions count towards that correction. With one-way effects, effects nested within clusters do not.`
    case 'hac': return 'The interval uses a 95% confidence level. Heteroskedasticity- and autocorrelation-consistent (HAC) covariance uses a Bartlett kernel and a bandwidth of L lags, where n is the number of rows:'
    case 'arma': return 'The interval uses a 95% confidence level.'
    default: return assertNever(errors)
  }
}

/** What the fixed-effects choice does to the regression, for the hint under the control. */
const describeFixedEffectsChoice = (fixedEffects: FixedEffects): string => {
  switch (fixedEffects.kind) {
    case 'none': return 'Every row is used as observed. Choose fixed effects when the same unit appears in several rows.'
    case 'time': return `Period effects account for additive differences shared by rows with the same ${fixedEffects.name}. They do not account for persistent differences between units.`
    case 'unit': return `Each variable is replaced by its deviation from the mean of its ${fixedEffects.name}. The run lists adjustment variables absorbed by these effects.`
    case 'unit-and-time': return `The regression removes additive ${fixedEffects.name} and ${fixedEffects.timeName} effects before fitting the remaining variation. The run lists adjustment variables absorbed by those effects. Unbalanced panels are also supported.`
    default: return assertNever(fixedEffects)
  }
}

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
      <DisclosureSummary className="cursor-pointer text-ink">Synthetic-control inference</DisclosureSummary>
      <div className="mt-3 grid gap-4">
        {evidence.crossFit.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table className="w-full border-collapse text-table" aria-label="Cross-fitted synthetic-control folds">
              <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Fold</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Held-out rows</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Bias</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Effect</th></tr></thead>
              <tbody>{evidence.crossFit.folds.map((fold, index) => <tr key={index}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{index + 1}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{fold.heldOut[0] + 1}–{(fold.heldOut.at(-1) ?? fold.heldOut[0]) + 1}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', fold.bias).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', fold.att).text}</td></tr>)}</tbody>
            </table>
          </div>
        ) : <p className="m-0 text-body text-muted">Cross-fitted inference unavailable: {evidence.crossFit.reason}</p>}
        {evidence.donorPlacebo.kind === 'available' ? (
          <div className="figure-strip overflow-x-auto">
            <table className="w-full border-collapse text-table" aria-label="Donor-placebo MSPE ratios">
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
  if (evidence.kind === 'staggeredDid') return <StaggeredDidResult evidence={evidence} labels={panelPeriodDisplay(run).labels} />
  const controls = evidence.units.slice(0, evidence.controlUnits)
  const periods = panelPeriodDisplay(run)
  const preLabels = periods.labels.slice(0, evidence.nPre)
  const postLabels = periods.labels.slice(evidence.nPre)
  if (evidence.kind === 'panelAdjusted') return <AdjustedDidResult evidence={evidence} labels={periods.labels} covariates={run.columns.slice(2).map(column => column.name)} />
  if (evidence.kind === 'panelDid') return <EvidenceTable
    title="Post-period differences" frame="none" rows={evidence.did.effectCurve.map((value, index) => ({ key: String(index), period: postLabels[index] ?? String(index), value }))}
    rowKey={(row) => row.key} noun="period" empty="No post-period differences." exportName="did-period-effects"
    columns={[{ id: 'period', header: 'Period', value: (row) => row.period }, { id: 'effect', header: 'Difference', value: (row) => formatStatistic('raw', row.value).text }]} />
  const estimates = [
    ['Difference-in-differences', evidence.did, null, null],
    ['Synthetic control', evidence.syntheticControl, evidence.syntheticControlPlacebo, evidence.syntheticControlInTime],
    ['Synthetic difference-in-differences', evidence.syntheticDid, evidence.syntheticDidPlacebo, evidence.syntheticDidInTime],
  ] as const
  return (
    <details className={well('mt-3 px-3 py-2 text-body')}>
      <DisclosureSummary className="cursor-pointer text-ink">Panel weights and period effects</DisclosureSummary>
      <div className="mt-3 grid gap-4">
        {periods.kind === 'dense-codes' && <Alert tone="info" live={false}><p className="m-0">This saved run does not contain source period labels. The period tables therefore show zero-based dense codes.</p></Alert>}
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-table" aria-label="Panel estimator comparison">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Estimator</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Estimate</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Placebo SE</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">In-time placebo</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">Noise level</th></tr></thead>
            <tbody>{estimates.map(([name, estimate, placebo, inTime]) => <tr key={name}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{name}{name.startsWith('Synthetic difference') ? <Metadata><span></span><span>primary</span></Metadata> : ''}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.estimate).text}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{placebo === null ? '—' : placebo.kind === 'available' ? formatStatistic('raw', placebo.standardError).text : 'Unavailable'}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{inTime === null ? '—' : inTime.kind === 'available' ? formatStatistic('raw', inTime.estimate).text : 'Unavailable'}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{formatStatistic('raw', estimate.noiseLevel).text}</td></tr>)}</tbody>
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
          <table className="w-full border-collapse text-table" aria-label="Panel unit weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Control unit</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{controls.map((unit, index) => <tr key={unit}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{unit}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.omega, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.omega, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-table" aria-label="Panel time weights">
            <thead><tr className="text-left"><th className="border-b border-hair px-2 py-1.5 font-medium">Pre-period</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">DID</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SC</th><th className="border-b border-hair px-2 py-1.5 text-right font-medium">SDID</th></tr></thead>
            <tbody>{preLabels.map((period, index) => <tr key={`${period}-${index}`}><th scope="row" className="border-b border-hair px-2 py-1.5 text-left font-normal">{period}</th><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.did.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticControl.lambda, index)}</td><td className={num('border-b border-hair px-2 py-1.5 text-right')}>{panelWeight(evidence.syntheticDid.lambda, index)}</td></tr>)}</tbody>
          </table>
        </div>
        <div className="figure-strip overflow-x-auto">
          <table className="w-full border-collapse text-table" aria-label="Panel post-period effects">
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
    <>
      <dl className="mb-0 mt-2 grid grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-micro text-faint">
        <dt>Run</dt><dd className={literal('m-0 break-all')}>{run.id}</dd>
        <dt>Study</dt><dd className={literal('m-0 break-all')}>{run.study}</dd>
        <dt>Identification</dt><dd className={literal('m-0 break-all')}>{run.identification}</dd>
        <dt>Prepared dataset</dt><dd className={literal('m-0 break-all')}>{run.preparedDataset}</dd>
        <dt>Columns</dt><dd className="m-0">{run.columns.map((column) => column.name).join(', ')}</dd>
        <dt>Configuration</dt><dd className={literal('m-0 break-all')}>{JSON.stringify(run.configuration)}</dd>
        {run.kind === 'causal-forest-run' && <><dt>Fitted forest settings</dt><dd className={literal('m-0 break-all')}>{JSON.stringify(run.evidence.fitted)}</dd></>}
        <dt>Created</dt><dd className={literal('m-0')}>{formatTimestamp(run.createdAt)}</dd>
        <dt>Method</dt><dd className={literal('m-0')}>{run.method}</dd>
      </dl>
    </>
  )
}

const treatmentModelTiles = (model: TreatmentModelEvidence): readonly { readonly label: string; readonly value: Formatted; readonly context?: React.ReactNode }[] =>
  model.kind === 'logistic'
    ? [{ label: 'Treatment model', value: formatWords(model.converged ? 'Converged' : 'Stopped early'), context: `${formatCount(model.parameters).text} parameters` }]
    : [
      {
        label: 'Treatment model',
        value: formatWords(model.scoring.kind === 'crossFitted' ? 'Boosted, cross-fitted' : 'Boosted trees'),
        context: <Metadata><span>learning rate {model.learningRate}</span><span>depth {model.maxDepth}</span><span>{formatCount(model.nEstimators).text} trees</span><span>{formatCount(model.candidates).text} candidate{model.candidates === 1 ? '' : 's'}</span></Metadata>,
      },
      ...boostedScoringTiles(model.scoring),
    ]

const boostedScoringTiles = (scoring: Extract<TreatmentModelEvidence, { kind: 'boosted' }>['scoring']): readonly { readonly label: string; readonly value: Formatted; readonly context?: React.ReactNode }[] => {
  switch (scoring.kind) {
    case 'oneModel': return [
      { label: 'Cross-validated AUC', value: formatStatistic('score', scoring.validationAuc), context: 'the chosen candidate, averaged over the search folds' },
      { label: 'AUC on the fitted rows', value: formatStatistic('score', scoring.fittedAuc), context: 'the model scoring the rows it was fitted on' },
    ]
    case 'crossFitted': return [
      { label: 'Cross-fitted AUC', value: formatStatistic('score', scoring.auc), context: 'each half predicted from the other half; hyperparameters selected on the full sample first' },
    ]
    default: return assertNever(scoring)
  }
}

function Diagnostics({ run }: { readonly run: EstimationRunArtifact }) {
  const tiles = ((): readonly { readonly label: string; readonly value: Formatted; readonly context?: React.ReactNode }[] => {
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
          { label: 'Estimator route', value: formatWords(describeInstrumentalVariableRoute(evidence.route)), context: <Metadata><span>{formatCount(evidence.instruments.length, { noun: 'instrument' }).text}</span><span>{formatCount(evidence.params.length, { noun: 'coefficient' }).text}</span></Metadata> },
          { label: 'Instruments', value: formatWords(instruments), context: `${formatCount(evidence.observations).text} rows` },
          { label: 'Bootstrap SE', value: evidence.standardError === null ? formatWords('none') : formatStatistic('raw', evidence.standardError), context: evidence.uncertainty.kind === 'bootstrap' ? <Metadata><span>{formatCount(evidence.uncertainty.simulations).text} resamples</span><span>seed {evidence.uncertainty.seed}</span></Metadata> : 'no bootstrap requested' },
        ]
      }
      case 'backdoor-linear-run': {
        const { evidence } = run
        const absorbedNames = evidence.fixedEffects.kind === 'none' ? [] : evidence.fixedEffects.absorbed.map((column) => run.columns[column]?.name ?? String(column))
        const counted = (n: number, singular: string, plural: string) => formatCount(n, { noun: n === 1 ? singular : plural })
        return [
          evidence.fixedEffects.kind !== 'none'
            ? { label: 'R squared, within', value: formatStatistic('score', evidence.rSquared), context: <Metadata><span>{counted(evidence.parameters, 'regressor', 'regressors').text}</span><span>{evidence.fixedEffects.kind === 'time' ? counted(evidence.fixedEffects.periods, 'period', 'periods').text : counted(evidence.fixedEffects.units, 'unit', 'units').text}</span>{evidence.fixedEffects.kind === 'unitAndTime' && <span>{counted(evidence.fixedEffects.periods, 'period', 'periods').text}</span>}</Metadata> }
            : { label: 'R squared', value: formatStatistic('score', evidence.rSquared), context: `${formatCount(evidence.parameters).text} parameters` },
          { label: 'Residual SD', value: formatStatistic('sd', evidence.residualSd), context: <Metadata><span>{formatCount(evidence.degreesOfFreedom).text} degrees of freedom</span><span>Durbin–Watson {formatStatistic('raw', evidence.durbinWatson).text}</span></Metadata> },
          evidence.fixedEffects.kind !== 'none'
            ? { label: 'Absorbed by the fixed effects', value: absorbedNames.length === 0 ? formatWords('none') : formatCount(absorbedNames.length), context: absorbedNames.length === 0 ? 'No adjustment columns were removed during absorption.' : absorbedNames.join(', ') }
            : { label: 'Newey–West bandwidth', value: formatCount(evidence.hacMaxLags, { noun: 'lag' }), context: `heteroskedasticity and autocorrelation consistent p ${formatP(evidence.hacPValue, { withLabel: false }).text}` },
          ...(evidence.errorModel.kind === 'hc1' ? [
            { label: 'Robust SE (HC1)', value: formatStatistic('raw', evidence.errorModel.standardError), context: `p ${formatP(evidence.errorModel.pValue, { withLabel: false }).text}` },
          ] : []),
          ...(evidence.errorModel.kind === 'cluster' ? [
            { label: 'Clustered SE', value: formatStatistic('raw', evidence.errorModel.standardError), context: <Metadata><span>{formatCount(evidence.errorModel.clusters, { noun: 'clusters' }).text}</span><span>p {formatP(evidence.errorModel.pValue, { withLabel: false }).text}</span></Metadata> },
          ] : []),
          ...(evidence.errorModel.kind === 'arma' ? [
            { label: `ARMA(${evidence.errorModel.errors.p}, ${evidence.errorModel.errors.q}) errors`, value: formatWords([...evidence.errorModel.errors.ar, ...evidence.errorModel.errors.ma].map((term) => `${term.name} ${formatStatistic('raw', term.coefficient).text}`).join(', ')), context: <Metadata><span>innovation variance {formatStatistic('raw', evidence.errorModel.errors.sigma2).text}</span><span>AIC {formatStatistic('raw', evidence.errorModel.errors.aic).text}</span><span>{evidence.errorModel.errors.converged ? `converged in ${evidence.errorModel.errors.iterations} iterations` : `stopped at ${evidence.errorModel.errors.iterations} iterations`}</span></Metadata> },
          ] : []),
        ]
      }
      case 'count-glm-run': {
        const { evidence } = run
        return [
          { label: 'Coefficient', value: formatStatistic('raw', evidence.coefficient), context: <Metadata><span>SE {formatStatistic('raw', evidence.standardError).text}</span><span>p {formatP(evidence.pValue, { withLabel: false }).text}</span></Metadata> },
          evidence.family === 'poisson'
            ? { label: 'Deviance', value: formatStatistic('raw', evidence.deviance ?? Number.NaN), context: `${formatCount(evidence.degreesOfFreedom).text} degrees of freedom` }
            : { label: 'Dispersion alpha', value: formatStatistic('raw', evidence.alpha ?? Number.NaN), context: `log likelihood ${formatStatistic('raw', evidence.logLikelihood ?? Number.NaN).text}` },
          { label: 'Convergence', value: formatWords(evidence.converged ? 'converged' : 'not converged'), context: `${formatCount(evidence.iterations).text} iterations` },
        ]
      }
      case 'negative-binomial-ingarch-run': {
        const { evidence } = run
        return [
          { label: 'Average path difference', value: formatStatistic('raw', evidence.averageEffect), context: <Metadata><span>{formatCount(evidence.horizon).text} forecast periods</span><span>cumulative {formatStatistic('raw', evidence.cumulativeEffect).text}</span></Metadata> },
          { label: 'Overdispersion', value: formatStatistic('raw', evidence.dispersion), context: `negative-binomial size ${formatStatistic('raw', evidence.size).text}` },
          { label: 'Recursion', value: formatWords(`${evidence.link === 'identity' ? 'additive' : 'multiplicative'}, count lag ${evidence.pastObservationLags.join(', ')}, mean lag ${evidence.pastMeanLags.join(', ')}`), context: <Metadata><span>{formatCount(evidence.iterations).text} optimizer iterations</span><span>{evidence.functionEvaluations}/{evidence.gradientEvaluations} function/gradient evaluations</span></Metadata> },
        ]
      }
      case 'ardl-run': {
        const { evidence } = run
        const reading = boundsReading(evidence)
        const [lower, upper] = evidence.boundsCritical[1] ?? [Number.NaN, Number.NaN]
        return [
          { label: 'Bounds test', value: formatWords(reading === 'level-relation' ? 'level relation' : reading === 'no-level-relation' ? 'no level relation' : 'inconclusive'), context: `F ${formatStatistic('raw', evidence.boundsStatistic).text} against 5% bounds ${formatStatistic('raw', lower).text} to ${formatStatistic('raw', upper).text}` },
          { label: 'Bounds p', value: formatP(evidence.boundsPUpper, { withLabel: false }), context: <Metadata><span>I(1) bound</span><span>I(0) bound p {formatP(evidence.boundsPLower, { withLabel: false }).text}</span></Metadata> },
          { label: 'Lag orders', value: formatWords(`ARDL(${evidence.arLag}, ${evidence.dlLag})`), context: <Metadata><span>AIC over {formatCount(evidence.grid.length).text} candidates</span><span>{evidence.trend === 'ct' ? 'constant and trend' : 'constant'}</span><span>case {evidence.case}</span></Metadata> },
        ]
      }
      case 'vecm-run': {
        const { evidence } = run
        return [
          { label: 'Cointegration rank', value: formatCount(evidence.rank), context: <Metadata><span>Johansen trace at {['90', '95', '99'][evidence.significance] ?? ''}%</span><span>{evidence.kArDiff} lagged differences</span></Metadata> },
          { label: 'Adjustment p', value: formatWords(evidence.pvaluesAlpha.map((row) => formatP(row[0] ?? Number.NaN, { withLabel: false }).text).join('; ')), context: <Metadata><span>alpha per equation</span><span>terms “{evidence.deterministic}”</span></Metadata> },
          evidence.chow === null
            ? { label: 'Chow break', value: formatWords('not requested'), context: 'set a split after a row to test stability' }
            : { label: 'Chow break', value: formatP(evidence.chow[1], { withLabel: false }), context: <Metadata><span>F {formatStatistic('raw', evidence.chow[0]).text}</span><span>split after row {run.configuration.breakIndex}</span></Metadata> },
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
              .join('; ')),
            context: <Metadata><span>{formatCount(evidence.weights.filter((weight) => weight >= 0.0005).length).text} of {evidence.weights.length} donors carry weight</span><span>sum to one</span></Metadata>,
          },
          { label: 'Pre-period loss', value: formatStatistic('raw', evidence.loss), context: <Metadata><span>{formatCount(evidence.nPre).text} pre rows</span><span>{formatCount(evidence.iterations).text} active-set steps</span></Metadata> },
          { label: 'Average post gap', value: formatStatistic('raw', evidence.att), context: `over ${formatCount(evidence.nPost).text} post rows` },
          evidence.crossFit.kind === 'available'
            ? { label: 'Bias-corrected estimate', value: formatStatistic('raw', evidence.crossFit.att), context: <Metadata><span>pre-period blocks left out in turn</span><span>SE {formatStatistic('raw', evidence.crossFit.standardError).text}</span><span>{formatInterval(evidence.crossFit.att, evidence.crossFit.confidenceInterval[0], evidence.crossFit.confidenceInterval[1], { kind: 'confidence', level: 0.95 }, additive).text}</span><span>p {formatP(evidence.crossFit.pValue, { withLabel: false }).text}</span></Metadata> }
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
        if (evidence.kind === 'staggeredDid') return [
          {label:'Cohorts',value:formatCount(evidence.cohorts.keys.length),context:'Distinct first-treatment periods'},
          {label:'Retained units',value:formatCount(evidence.units.length),context:String(evidence.times.length)+' periods'},
          {label:'Overall ATT',value:formatWords('Dynamic aggregation'),context:'Equal average of supported nonnegative event-time effects'},
        ]
        if (evidence.kind === 'panelAdjusted') return [
          { label: 'Method', value: formatWords(evidence.specification.kind === 'regression' ? 'Regression DiD' : 'Doubly robust DiD'), context: 'Average effect on the treated group' },
          { label: 'Panel layout', value: formatWords(`${evidence.treatedUnits} treated, ${evidence.controlUnits} comparison`), context: 'One before and one after period' },
        ]
        if (evidence.kind === 'panelDid') return [
          { label: 'Method', value: formatWords('Conventional difference-in-differences'), context: 'Change in treated outcomes minus change in comparison outcomes' },
          { label: 'Panel layout', value: formatWords(`${evidence.treatedUnits} treated, ${evidence.controlUnits} comparison`), context: `${evidence.nPre} pre- and ${evidence.nPost} post-periods` },
        ]
        const topWeights = evidence.syntheticDid.omega
          .map((weight, index) => ({ weight, unit: evidence.units[index] ?? `control ${index + 1}` }))
          .sort((left, right) => right.weight - left.weight)
          .slice(0, 3)
        const [heaviest, ...nextWeights] = topWeights
        return [
          { label: 'Synthetic DID', value: formatStatistic('raw', evidence.syntheticDid.estimate), context: <Metadata><span>chosen as the main result before fitting</span><span>DID {formatStatistic('raw', evidence.did.estimate).text}</span><span>synthetic control {formatStatistic('raw', evidence.syntheticControl.estimate).text}</span><span>{formatCount(evidence.nPost).text} post periods</span></Metadata> },
          { label: 'Panel layout', value: formatWords(`${evidence.treatedUnits} treated, ${evidence.controlUnits} comparison`), context: `${evidence.units.length} units × ${evidence.times.length} periods` },
          heaviest === undefined
            ? { label: 'Largest comparison weight', value: formatWords('none'), context: 'no synthetic-DID unit weights' }
            : { label: 'Largest comparison weight', value: formatStatistic('score', heaviest.weight), context: <Metadata><span>unit {heaviest.unit}</span>{nextWeights.map(({ unit, weight }) => <span key={unit}>{unit} {formatStatistic('score', weight).text}</span>)}</Metadata> },
        ]
      }
      case 'causal-forest-run': return []
      case 't-learner-run': {
        const { evidence } = run
        const rows = tLearnerRowEffects(evidence)
        const summary = rows === null ? null : summariseRowEffects(rows)
        const rowEffects = { label: 'Row effects', value: formatWords(summary === null ? 'none' : `${formatStatistic('raw', summary.minimum).text} to ${formatStatistic('raw', summary.maximum).text}`), context: summary === null ? '' : <Metadata><span>median {formatStatistic('raw', summary.median).text}</span><span>{formatPercent(summary.positiveShare, { precision: 0 }).text} above zero</span></Metadata> }
        switch (evidence.kind) {
          case 'tLearner': return [
            { label: 'Arms', value: formatWords(`${formatCount(evidence.controlRows).text} control, ${formatCount(evidence.treatedRows).text} treated`), context: 'one outcome forest each' },
            rowEffects,
            { label: 'Forests', value: formatWords(`${formatCount(evidence.trees).text} trees`), context: <Metadata><span>minimum leaf {evidence.minLeaf}</span><span>learner seed {evidence.seed}</span></Metadata> },
          ]
          case 'crossFittedTLearner': return [
            { label: 'Arms', value: formatWords(`${formatCount(evidence.controlRows).text} control, ${formatCount(evidence.treatedRows).text} treated`), context: 'two halves of equal size, stratified on the outcome' },
            rowEffects,
            ...([['Treated model', evidence.selected.treated], ['Control model', evidence.selected.control]] as const).map(([label, choice]) => ({
              label,
              value: formatWords(`learning rate ${choice.learningRate}, depth ${choice.maxDepth}, ${formatCount(choice.nEstimators).text} trees`),
              context: <Metadata><span>best AUC {formatStatistic('score', choice.validationAuc).text}</span><span>{formatCount(evidence.candidates).text} candidates, {evidence.splits}-fold</span></Metadata>,
            })),
          ]
          default: return assertNever(evidence)
        }
      }
      case 'negbin-nuts-run': {
        const { evidence } = run
        return [
          { label: 'Divergences', value: formatCount(evidence.divergences), context: <Metadata><span>acceptance {formatStatistic('score', evidence.acceptanceRate).text}</span><span>step {formatStatistic('raw', evidence.stepSize).text}</span></Metadata> },
          { label: 'Rate ratio per SD', value: formatStatistic('raw', Math.exp(evidence.betaTreatmentMean)), context: <Metadata><span>per standard deviation of {run.columns[0]?.name ?? 'the treatment'}</span><span>beta {formatStatistic('raw', evidence.betaTreatmentMean).text}, posterior sd {formatStatistic('raw', evidence.betaTreatmentSd).text}</span></Metadata> },
          { label: 'Dispersion r', value: formatStatistic('raw', evidence.dispersionMean), context: <Metadata><span>{formatCount(evidence.warmup).text} warmup</span><span>{formatCount(evidence.samples).text} draws</span><span>seed {evidence.seed}</span></Metadata> },
        ]
      }
      case 'bayesian-gaussian-run': {
        const { evidence } = run
        return [
          { label: 'Divergences', value: formatCount(evidence.divergences), context: <Metadata><span>{formatCount(evidence.chains).text} chains</span><span>acceptance {formatStatistic('score', evidence.acceptanceRate).text}</span><span>step {formatStatistic('raw', evidence.stepSize).text}</span></Metadata> },
          { label: 'P(effect > 0)', value: formatStatistic('score', evidence.probabilityPositive), context: <Metadata><span>posterior mean {formatStatistic('raw', evidence.effectMean).text}</span><span>sd {formatStatistic('raw', evidence.effectSd).text}</span><span>median {formatStatistic('raw', evidence.effectMedian).text}</span></Metadata> },
          { label: 'Residual sd', value: formatStatistic('raw', evidence.sigmaMean), context: <Metadata><span>{formatCount(evidence.warmup).text} warmup</span><span>{formatCount(evidence.samples).text} draws per chain</span><span>seed {evidence.seed}</span></Metadata> },
        ]
      }
      case 'discrete-bn-run': {
        const { evidence } = run
        return [
          { label: 'Treatment states', value: formatWords(`${evidence.treatmentStates[0]} → ${evidence.treatmentStates[1]}`), context: <Metadata><span>state budget {evidence.bins}</span><span>counts {evidence.stateCounts.join('/')}</span></Metadata> },
          { label: 'State preparation', value: formatWords(`${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'observedStates').length} observed, ${evidence.statePreparations.filter((entry) => entry.strategy.kind === 'quantiles').length} quantile`), context: describeDiscreteStatePreparations(evidence.statePreparations) },
          { label: 'Expected outcome', value: formatWords(`${formatStatistic('raw', evidence.expectations[0]).text} → ${formatStatistic('raw', evidence.expectations[1]).text}`), context: 'under do(low) and do(high)' },
          ...(evidence.parentsAdjusted.join(', ') === adjustmentLabels(run.estimate.adjustment).join(', ') ? [] : [
  { label: 'Adjustment set', value: adjustmentFigure(evidence.parentsAdjusted, 'none'), context: evidence.minimalAdjustmentSet === null ? 'no minimal adjustment set' : `minimal set ${evidence.minimalAdjustmentSet.length === 0 ? 'empty' : namesInProse(evidence.minimalAdjustmentSet, (count) => `of ${count} variables`)}` },
          ]),
        ]
      }
      case 'binary-ett-run': {
        const { evidence } = run
        return [
          { label: 'E[Y(1) | X=1]', value: formatStatistic('raw', evidence.treatedPotentialOutcomeMean), context: 'treated potential-outcome mean among treated rows' },
          { label: 'E[Y(0) | X=1]', value: formatStatistic('raw', evidence.untreatedPotentialOutcomeMean), context: 'untreated potential-outcome mean among treated rows' },
          { label: 'ETT', value: formatStatistic('raw', evidence.effectOnTreated), context: <Metadata><span>{formatCount(evidence.observations).text} rows</span><span>plug-in estimate</span></Metadata> },
        ]
      }
      case 'double-ml-run': {
        const { evidence } = run
        return [
          { label: 'Model', value: formatWords(evidence.model === 'plr' ? 'partially linear' : evidence.att ? 'interactive (effect on the treated)' : 'interactive (average effect)'), context: evidence.treatBinary ? 'binary treatment' : 'continuous treatment' },
          { label: 'Standard error', value: formatStatistic('raw', evidence.standardError), context: 'sandwich, cross-fitted' },
          { label: 'Fold seed', value: formatCount(evidence.seed), context: <Metadata><span>5 folds</span><span>200 trees</span><span>learner seed 7</span></Metadata> },
        ]
      }
      case 'causal-effects-run': {
        const { evidence } = run
        const nodeName = (node: readonly [number, number]) => `${run.columns[node[0]]?.name ?? node[0]}${node[1] === 0 ? '' : ` (t−${Math.abs(node[1])})`}`
        const fitTiles = (() => {
          switch (evidence.fit.kind) {
            case 'unfitted': return []
            case 'invalidAdjustment': return []
            case 'adjustedLinear': return [{ label: 'Adjustment set', value: adjustmentFigure(evidence.fit.adjustmentSet.map(nodeName), 'None'), context: <Metadata><span>{adjustmentStrategyLabel(evidence.fit.selection)}</span><span>linear</span><span>τ max {evidence.tauMax}</span></Metadata> }]
            case 'adjustedKnn': return [{ label: 'Adjustment set', value: adjustmentFigure(evidence.fit.adjustmentSet.map(nodeName), 'None'), context: <Metadata><span>{adjustmentStrategyLabel(evidence.fit.selection)}</span><span>{evidence.fit.k}-neighbour</span><span>τ max {evidence.tauMax}</span></Metadata> }]
            case 'wrightParents': return [
              { label: 'Direct effect', value: formatStatistic('raw', evidence.fit.directEffect), context: 'sum of direct path contrasts' },
              { label: 'Indirect effect', value: formatStatistic('raw', evidence.fit.indirectEffect), context: <Metadata><span>{evidence.fit.paths.length} directed paths</span><span>{evidence.fit.coefficients.length} parent coefficients</span></Metadata> },
            ]
            default: return assertNever(evidence.fit)
          }
        })()
        return [
          ...fitTiles,
          { label: 'Predictions', value: formatWords(evidence.predictions.map((value) => formatStatistic('raw', value).text).join(' → ')), context: `at ${evidence.interventions[0]} and ${evidence.interventions[1]}` },
          { label: 'Fitted rows', value: formatCount(evidence.fittedObservations), context: evidence.mediators.length === 0 ? 'no mediators' : `${evidence.mediators.length} mediator nodes` },
          ...(evidence.uncertainty.kind === 'bootstrap' ? [{ label: 'Bootstrap', value: formatCount(evidence.uncertainty.samples), context: <Metadata><span>{Math.round(evidence.uncertainty.confidenceLevel * 100)}% percentile interval</span><span>block {evidence.uncertainty.resolvedBlockLength}</span><span>seed {evidence.uncertainty.seed}</span></Metadata> }] : []),
        ]
      }
      case 'sharp-rd-run': return [
        { label: 'Estimation bandwidth', value: formatStatistic('raw', run.evidence.bandwidth), context: 'mserd, triangular kernel' },
        { label: 'Bias bandwidth', value: formatStatistic('raw', run.evidence.biasBandwidth), context: 'local quadratic bias correction' },
        { label: 'Local sample', value: formatCount(run.evidence.effectiveObservations[0] + run.evidence.effectiveObservations[1]), context: `${run.evidence.effectiveObservations[0]} below; ${run.evidence.effectiveObservations[1]} at or above cutoff` },
      ]
      case 'causal-impact-run': {
        const { evidence } = run
        return [
          { label: 'Cumulative effect', value: formatStatistic('raw', evidence.cumulative), context: `over ${formatCount(evidence.nPost).text} post rows` },
          { label: 'Average effect', value: formatStatistic('raw', evidence.average), context: `pre window ${formatCount(evidence.nPre).text} rows` },
          ...(evidence.kind !== 'causalImpact'
            ? [{ label: 'Posterior draws', value: formatCount(evidence.draws), context: `${evidence.warmup} warmup iterations` }]
            : [{ label: 'Log likelihood', value: formatStatistic('raw', evidence.logLikelihood), context: `${formatCount(evidence.controls.length).text} controls` }]),
        ]
      }
      case 'propensity-weighting-run': {
        const { evidence } = run
        const sampleSize = effectiveSampleSize(evidence.weights, evidence.treated)
        return [
          { label: 'Treated mean', value: formatStatistic('raw', evidence.treatedMean), context: `${formatCount(evidence.treatedRows).text} treated rows` },
          { label: 'Control mean', value: formatStatistic('raw', evidence.controlMean), context: `${formatCount(evidence.controlRows).text} control rows` },
          { label: 'Weight sums', value: formatWords(`${evidence.treatedWeightSum.toFixed(0)} treated, ${evidence.controlWeightSum.toFixed(0)} control`), context: 'the total weight each arm carries' },
          { label: 'Effective sample size', value: formatWords(`${sampleSize.treated.toFixed(0)} treated, ${sampleSize.control.toFixed(0)} control`), context: 'lower values relative to each arm’s row count indicate more uneven weights' },
          ...treatmentModelTiles(evidence.treatmentModel),
        ]
      }
      case 'propensity-matching-run': {
        const { evidence } = run
        return [
          { label: 'Paired rows', value: formatCount(evidence.matches.length), context: 'one nearest neighbour from the other arm for every row' },
          { label: 'Arms', value: formatWords(`${evidence.treatedRows} treated, ${evidence.controlRows} control`), context: 'pairs are averaged over every row' },
          ...treatmentModelTiles(evidence.treatmentModel),
        ]
      }
      case 'doubly-robust-run': {
        const { evidence } = run
        return [
          { label: 'Treated term', value: formatStatistic('raw', evidence.treatedTerm), context: 'weighted residual plus the fitted outcome under treatment' },
          { label: 'Control term', value: formatStatistic('raw', evidence.controlTerm), context: 'the same under no treatment' },
          { label: 'Treatment model', value: formatWords(evidence.converged ? 'Converged' : 'Stopped early'), context: `${formatCount(evidence.parameters).text} parameters` },
        ]
      }
      case 'continuous-gps-run': {
        const { evidence } = run
        const rows = formatCount(evidence.observations).text
        const spreadTiles = evidence.weights === null
          ? [{ label: 'Effective sample size', value: formatWords('Not recorded'), context: 'this run was saved before weights were recorded; run it again to see them' }]
          : (() => {
              const spread = weightSpread(evidence.weights, 10)
              return [
                { label: 'Effective sample size', value: formatCount(Math.round(spread.effectiveSampleSize)), context: `of ${rows} rows; the closer to the row count, the less a few weights dominate` },
                { label: 'Largest weight', value: formatStatistic('raw', spread.largestWeight), context: `the ${spread.largestShare.count} largest weights carry ${formatPercent(spread.largestShare.share).text} of the total` },
              ]
            })()
        return [
          { label: 'Standard error', value: formatStatistic('raw', evidence.standardError), context: 'from the weighted regression, with the weights taken as fixed' },
          ...spreadTiles,
          { label: 'Residual scale', value: formatStatistic('raw', evidence.residualScale), context: 'the spread of the treatment around its fitted value' },
        ]
      }
      default: return assertNever(run)
    }
  })()
  // The design-specific tiles say what stands in for an adjustment set; otherwise the set itself, counted when
  // long with its first names beneath. The identification record lists every one.
  const adjustmentTile = ((): { readonly value: Formatted; readonly preview: string | null } => {
    if (run.kind === 'panel-intervention-run') return { value: formatWords(run.evidence.kind === 'staggeredDid' ? 'Adoption-cohort comparisons' : 'Unit and time weights'), preview: null }
    if (run.kind === 'frontdoor-two-stage-run') {
      const stage = (columns: readonly number[]) => columns.length === 0 ? 'none' : columns.map((index) => run.columns[index]?.name ?? index).join(', ')
      return { value: formatWords(`stage 1: ${stage(run.evidence.firstStageAdjustment)}, stage 2: ${stage(run.evidence.secondStageAdjustment)}`), preview: null }
    }
    if (run.kind === 'instrumental-variable-run') return { value: formatWords('None; the estimator uses no covariates'), preview: null }
    if (run.estimate.adjustment.kind === 'structural-parent-model') return { value: formatWords(`${run.estimate.adjustment.coefficients} parent coefficients, ${run.estimate.adjustment.paths} directed paths`), preview: null }
    const names = adjustmentLabels(run.estimate.adjustment)
    return { value: adjustmentFigure(names, 'None'), preview: names.length === 0 ? null : namesFigure(names, 'variables').preview }
  })()
  const adjustmentValue = adjustmentTile.value

  // A method that reports the set it actually fitted names the same members as the recorded estimate
  // whenever it accepted them, so the two tiles would print one value twice and leave the grid's last
  // row part-filled. Keep the one tile and let it carry how the set was chosen.
  const restated = tiles.find((tile) => tile.label === 'Adjustment set' && tile.value.text === adjustmentValue.text)
  const declaredCategorical = run.columns
    .filter((column) => run.encodings[column.column]?.kind === 'categorical')
    .map((column) => column.name)

  return (
    <MetricGrid className="mt-4" label="Diagnostics">
      <MetricTile
        label={run.kind === 'panel-intervention-run' ? 'Comparison design' : run.kind === 'frontdoor-two-stage-run' ? 'Stage adjustments' : run.kind === 'instrumental-variable-run' ? 'Covariates' : run.estimate.adjustment.kind === 'structural-parent-model' ? 'Structural model' : 'Adjustment set'}
        size="compact"
        frame="cell"
        value={adjustmentValue}
        context={run.kind === 'panel-intervention-run'
          ? 'This design does not use a DAG adjustment set.'
          : declaredCategorical.length > 0
            ? `Categorical: ${namesInProse(declaredCategorical, (count) => `${count} variables`)}. One column per level.`
            : adjustmentTile.preview ?? restated?.context}
      />
      {tiles.filter((tile) => tile !== restated).map((tile) => <MetricTile key={tile.label} label={tile.label} size="compact" frame="cell" value={tile.value} context={tile.context} />)}
    </MetricGrid>
  )
}

/** Memoised: editing the form must not redraw the result, its diagnostics or its charts. */
const ResultCard = memo(function ResultCard({ run, study, current, stepLabel, onDelete, others = [] }: { readonly others?: readonly EstimationRunArtifact[]; readonly run: EstimationRunArtifact; readonly study: StudySpecification; readonly current: boolean; readonly stepLabel: string; readonly onDelete?: (run: EstimationRunArtifact) => void }) {
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
      before: run.kind === 'causal-impact-run' ? preInterventionPoints(run.evidence.preInterventionPath) : [],
      points: estimate.effect.values,
      stepLabel,
      ghost: ghostRun !== null && ghostRun.estimate.effect.kind === 'path' ? { name: `${describeEstimator(ghostRun.configuration.kind)}, ${formatTime(ghostRun.createdAt)}`, points: ghostRun.estimate.effect.values } : undefined,
    }, theme)
    : null), [estimate.effect, run, stepLabel, study.outcome.name, theme, ghostRun])
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
  const overlap = useMemo(() => {
    const scored = run.kind === 'propensity-weighting-run' || run.kind === 'propensity-matching-run' || run.kind === 'doubly-robust-run'
    if (!scored) return null
    return propensityDistributionOption({
      treatment: study.treatment.name,
      propensity: run.evidence.propensity,
      treated: run.evidence.treated,
      weights: run.kind === 'propensity-weighting-run' ? run.evidence.weights : undefined,
    }, theme)
  }, [run, study.treatment.name, theme])
  const overlapPanels = run.kind === 'propensity-weighting-run' ? 2 : 1
  const weightScatter = useMemo(() => run.kind !== 'propensity-weighting-run'
    ? null
    : propensityWeightOption({
      treatment: study.treatment.name,
      outcomeName: study.outcome.name,
      propensity: run.evidence.propensity,
      treated: run.evidence.treated,
      weights: run.evidence.weights,
      outcome: run.evidence.outcome,
    }, theme), [run, study.outcome.name, study.treatment.name, theme])
  const [scoreWindow, setScoreWindow] = useState<VisibleWindow | null>(null)
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
  const stamp = <RunMeta>{[describeEstimator(run.configuration.kind), ...(run.kind === 'backdoor-linear-run' ? [describeFixedEffects(run.configuration.fixedEffects), describeCovariance(run.configuration.errors)].filter((part): part is string => part !== null) : []), formatTime(run.createdAt)]}</RunMeta>
  // A grouped effect draws each group's interval on the shared axis, the whole-population average last.
  const groupChart = useMemo(() => (estimate.effect.kind === 'byGroup'
    ? runComparisonOption([
      ...estimate.effect.groups.map((group): RunComparisonRow => ({ label: group.label, estimate: group.value, lower: group.interval.lower, upper: group.interval.upper, current: false })),
      { label: 'All rows', estimate: estimate.effect.overall, lower: estimate.interval.kind === 'none' ? null : estimate.interval.lower, upper: estimate.interval.kind === 'none' ? null : estimate.interval.upper, current },
    ], theme)
    : null), [estimate, current, theme])
  // Per-row effects are drawn as their distribution, with the average marked, since a thousand points have no order to plot.
  const rowChart = useMemo(() => {
    const rows = run.kind === 't-learner-run' ? tLearnerRowEffects(run.evidence) : null
    if (run.kind !== 't-learner-run' || rows === null) return null
    const summary = summariseRowEffects(rows)
    return histogramOption({ name: `effect of ${study.treatment.name} on ${study.outcome.name}`, bins: summary.bins, nullCount: 0, marks: [{ name: 'average', value: run.evidence.average }, { name: 'median', value: summary.median }] }, theme)
  }, [run, study.treatment.name, study.outcome.name, theme])
  const body = (
    <>
      <div className="mt-3">
        {run.kind === 'causal-forest-run'
          ? <CausalForestResults evidence={run.evidence} outcome={study.outcome.name} columns={run.columns} />
          : <EstimateHeadline estimate={estimate} sentence={sentence} scaleLine={scaleLine} sampleLine={sampleLine} stepLabel={stepLabel} accent={current} testId="effect-estimate" />}
      </div>
      {groupChart !== null && estimate.effect.kind === 'byGroup' && (
        <div className="mt-3">
          <ExpandableChart option={groupChart} label={`Effect of ${study.treatment.name} on ${study.outcome.name} by group of ${estimate.effect.modifier}`} className="h-[220px]" testId="group-effects" />
        </div>
      )}
      {overlap !== null && (
        <div className="mt-3">
          <ExpandableChart option={overlap} window={scoreWindow} onWindow={setScoreWindow} label={`Fitted propensity score for ${study.treatment.name} by arm`} className={overlapPanels === 1 ? 'h-[240px]' : 'h-[480px]'} testId="propensity-overlap" />
        </div>
      )}
      {weightScatter !== null && (
        <div className="mt-3">
          <ExpandableChart option={weightScatter} window={scoreWindow} onWindow={setScoreWindow} label={`${study.outcome.name} against the fitted propensity score, each row at its weight`} className="h-[300px]" testId="propensity-weights" />
        </div>
      )}
      {rowChart !== null && (
        <div className="mt-3">
          <ExpandableChart option={rowChart} label={`Distribution of the per-row effect of ${study.treatment.name} on ${study.outcome.name}`} className="h-[220px]" testId="row-effects" />
        </div>
      )}
      {run.kind === 't-learner-run' && run.evidence.kind === 'tLearner' && <TLearnerIntervals evidence={run.evidence} />}
      {chart !== null && (
        <div className="mt-3">
          {ghosts.length > 0 && (
            <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
              <p className={label('m-0 text-muted')}>Compare with</p>
              <Select aria-label="Compare with" className={field('text', 'w-64')} value={ghostId} onChange={(event) => setGhostId(event.target.value)}>
                <option value="">No other run</option>
                {ghosts.map((other) => <option key={other.id} value={String(other.id)}>{describeEstimator(other.configuration.kind)}, {formatTime(other.createdAt)}</option>)}
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
          {run.configuration.errors.kind === 'arma'
            ? <>For comparison, least squares gives <span className={num('text-bone')}>{formatStatistic('raw', run.evidence.estimate).text}</span> with a Newey–West interval of <span className={num('text-bone')}>{intervalText({ ...estimate, effect: { kind: 'additive', value: run.evidence.estimate, unit: '' }, interval: { kind: 'confidence', level: run.evidence.level, lower: run.evidence.hacInterval[0], upper: run.evidence.hacInterval[1] } })}</span>.</>
            : run.configuration.errors.kind === 'classical'
              ? (run.configuration.fixedEffects.kind !== 'none'
                ? null
                : <>For comparison, the HAC interval is <span className={num('text-bone')}>{intervalText({ ...estimate, interval: { kind: 'confidence', level: run.evidence.level, lower: run.evidence.hacInterval[0], upper: run.evidence.hacInterval[1] } })}</span>.</>)
              : <>For comparison, the classical interval is <span className={num('text-bone')}>{intervalText({ ...estimate, interval: { kind: 'confidence', level: run.evidence.level, lower: run.evidence.interval[0], upper: run.evidence.interval[1] } })}</span>.</>}
        </p>
      )}
      {run.kind === 'backdoor-linear-run' && armaReading(run)?.errors.converged === false && (
        <Alert tone="warn" live={false} className="mt-3"><p className="m-0">The ARMA fit stopped without converging; review the specification and optimiser settings before reading the estimate.</p></Alert>
      )}
      {run.kind === 'backdoor-linear-run' && <ArmaUncertaintyAlert evidence={armaReading(run)?.errors} />}
      {run.kind === 'count-glm-run' && !run.evidence.converged && (
        <Alert tone="warn" live={false} className="mt-3"><p className="m-0">The optimiser did not converge; treat the estimate and its interval as provisional.</p></Alert>
      )}
      <ResultInterpretation interpretation={interpretEstimationResult(run, study, stepLabel)} className="mt-3" />
      <Diagnostics run={run} />
      {run.kind === 'causal-impact-run' && <ImpactEffectPanel evidence={run.evidence} stepLabel={stepLabel} />}
      {run.kind === 'causal-impact-run' && run.evidence.kind !== 'causalImpact' && <BayesianImpactResult evidence={run.evidence} columnNames={run.columns.map(column => column.name)} />}
      {run.kind === 'sharp-rd-run' && <SharpRdResult evidence={run.evidence} running={run.columns[0].name} outcome={study.outcome.name} />}
      {run.kind === 'synthetic-control-run' ? <SyntheticControlEvidenceDetails run={run} /> : null}
      {run.kind === 'panel-intervention-run' ? <PanelEvidenceDetails run={run} /> : null}
    </>
  )
  if (!current) {
    // History rows fold to one line in the runs drawer; only the current estimate keeps the stage.
    return (
      <RunFold title={sentence} figure={run.kind === 'causal-forest-run' && estimate.effect.kind === 'perRow' ? `${estimate.effect.effects.length} conditional predictions` : headlineFigure(estimate).text} stamp={stamp} onDelete={onDelete === undefined ? undefined : () => onDelete(run)} deleteLabel="Delete this run">
        <RunDetails label="Estimation run details"><RunRecord run={run} /></RunDetails>
        {body}
      </RunFold>
    )
  }
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label={`${sentence} estimate`}>
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <span className={label('text-signal-text')}>Current estimate</span>
        <span className={num('text-micro text-faint')}>{stamp}</span>
      </div>
      {body}
    </article>
  )
})

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
  const empty = useMemo(() => initialEstimationDraft(prepared, studies, [], []), [prepared, studies])
  const state = useWorkflow(store => store.estimationDraft?.prepared === prepared.id ? store.estimationDraft.draft : empty)
  const change = useWorkflow(store => store.changeEstimation)
  const dispatch = (event: EstimationEvent) => change(prepared.id, event)
  const visibleEstimatorGroup = state.group
  const setVisibleEstimatorGroup = (group: EstimatorGroupId) => dispatch({ type: 'group-chosen', group })
  const visibleGroup = ESTIMATOR_GROUPS.find((group) => group.id === visibleEstimatorGroup) ?? ESTIMATOR_GROUPS[0]
  const selectedEstimatorIsVisible = visibleGroup.estimators.includes(state.estimator)
  const identification = identified.find((candidate) => candidate.id === state.identification) ?? null
  const study = identification === null ? null : studies.find((candidate) => candidate.id === identification.study) ?? null
  const studyScale = prepared.kind === 'prepared-time-series' && study !== null
    ? [study.treatment, study.outcome].map((variable) => `${variable.name}: ${describeSeriesTransform(seriesTransformFor(prepared.seriesTransforms, variable.column))}`).join('; ')
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

  const session = useJob('estimation')
  const { job } = session
  useRunActivity(onActivity, job.kind === 'running' ? { label: describeEstimator(state.estimator), progress: job.progress === null ? null : job.progress.completed / Math.max(1, job.progress.total) } : null)
  const execute = async () => {
    if (identification === null || study === null || eligibility === null || eligibility.kind === 'refused' || adjustmentDraftOpen || !method.ok) return
    if (!estimableIdentification(identification.result)) return
    const current = session.start('analysis', describeEstimator(state.estimator))
    if (current === null) return
    const dispatch = (event: RunEvent) => {
      switch (event.type) {
        case 'run-progressed': session.progress(current, describeEstimator(state.estimator), event.progress); return
        case 'run-failed': session.fail(current, event.detail); return
        case 'run-finished': session.finish(current); return
        default: assertNever(event)
      }
    }
    try {
      const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] = await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
      if (!session.current(current)) return
      const materialise = async (columns: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>>) => {
        const matrix = await materialisePrepared(source, profile, prepared, columns.map((column) => column.column) as unknown as NonEmptyArray<ColumnId>)
        if (!session.current(current)) throw new DOMException('The estimation run was cancelled.', 'AbortError')
        if (!matrix.ok) throw new Error(describePreparedMaterialisationProblem(matrix.error))
        return matrix.value
      }
      const columnAt = (values: Float64Array, rows: number, index: number): number[] => Array.from(values.subarray(index * rows, (index + 1) * rows))
      const identity = { id: newEstimationRunId(), study: study.id, identification: identification.id, preparedDataset: prepared.id, createdAt: new Date().toISOString(), eligibility, encodings: state.encodings } as const
      const finish = (run: EstimationRunArtifact | null, detail: string) => {
        if (!session.current(current)) return
        if (run === null) { dispatch({ type: 'run-failed', detail }); return }
        onRun(run)
        dispatch({ type: 'run-finished' })
      }
      if (configuration.kind === 'sharp-rd') {
        if (study.estimand.kind !== 'local-cutoff-effect' || identification.result.kind !== 'cutoff-design') {
          dispatch({ type: 'run-failed', detail: 'Record the cutoff-local study before fitting sharp RD.' }); return
        }
        const columns: NonEmptyArray<StudyVariable> = [study.estimand.running, study.outcome, study.treatment]
        const matrix = await materialise(columns)
        const evidence = await analysis.runSharpRd(matrix.values, matrix.rowCount, study.estimand.cutoff)
        if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
        const run = { kind: 'sharp-rd-run', configuration, evidence: evidence.value } as const
        const estimate = causalEstimateFrom(study, identification, run)
        finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate }, 'The RD result no longer matches the recorded cutoff-local study.')
        return
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
          // Grouping columns ride along after the design as numeric columns; they are not part of the fit.
          type Grouping = Pick<StudyVariable, 'column' | 'name'>
          const effects = configuration.fixedEffects
          const unit: Grouping | null = effects.kind === 'none' ? null : { column: effects.column, name: effects.name }
          const time: Grouping | null = effects.kind === 'unit-and-time' ? { column: effects.timeColumn, name: effects.timeName } : null
          const cluster: Grouping | null = configuration.errors.kind === 'cluster' ? { column: configuration.errors.column, name: configuration.errors.name } : null
          for (const [role, grouping] of [['unit', unit], ['time', time], ['cluster', cluster]] as const) {
            if (grouping !== null && columns.some((column) => column.column === grouping.column)) throw new Error(`The ${role} column must be outside the design: not the treatment, the outcome or an adjustment variable.`)
          }
          if (unit !== null && time !== null && time.column === unit.column) throw new Error('The unit and time columns must differ.')
          const groupings = [...new Map([unit, time, cluster].filter((grouping): grouping is Grouping => grouping !== null).map((grouping) => [grouping.column, grouping])).values()]
          // A panel's own keys are read beside the design; any other grouping column is materialised with it.
          const panel = prepared.kind === 'prepared-panel' ? prepared : null
          const isPanelKey = (grouping: Grouping) => panel !== null && (grouping.column === panel.sampling.unitColumn || grouping.column === panel.sampling.timeColumn)
          const extras = groupings.filter((grouping) => !isPanelKey(grouping))
          const materialized: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>> = [...columns, ...extras]
          const matrix = await materialise(materialized)
          const layouts = [...designLayouts(columns, 2, configuration.kind, state.encodings), ...extras.map(() => ({ kind: 'numeric' } as const))]
          const design = expandDesign(matrix.values, matrix.rowCount, materialized.length, layouts)
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const adjustment = design.value.expanded.slice(2, 2 + identification.result.adjustment.variables.length).flat()
          let values = design.value.values
          let columnCount = design.value.columnCount
          const columnOf = new Map<ColumnId, number>()
          for (const [at, grouping] of extras.entries()) {
            const index = design.value.expanded[columns.length + at]?.[0]
            if (index === undefined) { dispatch({ type: 'run-failed', detail: 'A selected grouping column is missing from the prepared regression matrix.' }); return }
            columnOf.set(grouping.column, index)
          }
          if (panel !== null && groupings.some(isPanelKey)) {
            const { materializePanelKeysInWorker } = await import('@/data/client')
            const keys = await materializePanelKeysInWorker(source.file, profile, panel.sampling.unitColumn, panel.sampling.timeColumn)
            if (!session.current(current)) return
            if (!keys.ok) { dispatch({ type: 'run-failed', detail: describePanelDataProblem(keys.error) }); return }
            if (keys.value.sourceFingerprint !== profile.source.fingerprint || keys.value.rowCount !== matrix.rowCount) { dispatch({ type: 'run-failed', detail: 'The panel keys no longer match the prepared rows. Recreate the prepared panel version.' }); return }
            const append = (codes: readonly number[]): number => {
              const coded = new Float64Array(values.length + matrix.rowCount)
              coded.set(values)
              codes.forEach((code, row) => { coded[columnCount * matrix.rowCount + row] = code })
              values = coded
              columnCount += 1
              return columnCount - 1
            }
            if (groupings.some((grouping) => grouping.column === panel.sampling.unitColumn)) {
              const labels = [...new Set(keys.value.units)].sort()
              columnOf.set(panel.sampling.unitColumn, append(keys.value.units.map((label: string) => labels.indexOf(label))))
            }
            if (groupings.some((grouping) => grouping.column === panel.sampling.timeColumn)) columnOf.set(panel.sampling.timeColumn, append(keys.value.periodCodes))
          }
          const plan = adjustedRegressionPlan(configuration, columnOf)
          if (!plan.ok) { dispatch({ type: 'run-failed', detail: 'A selected fixed-effect or cluster column could not be materialised. Review the grouping columns.' }); return }
          const evidence = await analysis.runBackdoorLinear(values, matrix.rowCount, columnCount, { treatment: 0, outcome: 1, adjustment, hacMaxLags: null, level: configuration.level, ...plan.value })
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
          if (configuration.kind === 'dml-irm' && columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `The interactive model cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.` }); return }
          const groups: DmlGroupsRequest = study.estimand.kind !== 'conditional-average-treatment-effect'
            ? { kind: 'none' }
            : study.estimand.grouping.kind === 'levels'
              ? { kind: 'levels', column: columns.findIndex((variable) => variable.column === modifier?.column) }
              : { kind: 'quantiles', column: columns.findIndex((variable) => variable.column === modifier?.column), bins: study.estimand.grouping.bins }
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const evidence = await analysis.runDoubleMl(design.value.values, matrix.rowCount, design.value.columnCount, { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2).flat(), model: configuration.kind === 'dml-plr' ? 'plr' : 'irm', att: configuration.kind === 'dml-irm' && configuration.att, seed: configuration.seed, groups })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'double-ml-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
          return
        }
        case 'causal-forest': {
          const target = causalForestTarget(study.estimand)
          if (target === null) { dispatch({ type: 'run-failed', detail: 'The recorded target is not supported by causal forest.' }); return }
          const inputs = causalForestInputs(identification.result.adjustment.variables, study.estimand)
          const auxiliary = configuration.analysis === undefined ? [] : forestAnalysisColumns(configuration.analysis)
          const required = [study.treatment.column, study.outcome.column, ...inputs.map(c => c.column)]
          const extra: Pick<StudyVariable, 'column' | 'name'>[] = []
          for (const id of auxiliary) {
            if (id === study.treatment.column || id === study.outcome.column) throw new Error('Use separate columns for forest sampling, projection and priority scores, not the treatment or outcome.')
            if (required.includes(id as ColumnId)) continue
            const column = profile.columns.find(c => c.id === id && prepared.columns.includes(c.id))
            if (column === undefined) throw new Error('An auxiliary forest column is not in the prepared dataset.')
            extra.push({ column: column.id, name: column.name })
          }
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...inputs]
          const materialized: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>> = [...columns, ...extra]
          const matrix = await materialise(materialized)
          const layouts = [...designLayouts(columns, 2, configuration.kind, state.encodings), ...extra.map(() => ({ kind: 'numeric' } as const))]
          const design = expandDesign(matrix.values, matrix.rowCount, materialized.length, layouts)
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map(column => column.name)) }); return }
          const evidence = await analysis.runCausalForest(design.value.values, matrix.rowCount, design.value.columnCount,
            { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2, 2 + inputs.length).flat(), target, configuration,
              columnNames: materialized.flatMap((column, index) => design.value.expanded[index]!.map(() => column.column)) })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const features = inputs.flatMap<ForestFeature>((column, index) => {
            const levels = design.value.levels[index + 2]!
            return levels.length === 0 ? [{ kind: 'numeric', column: column.column, name: column.name } as const]
              : levels.map(level => ({ kind: 'indicator', column: column.column, name: column.name, level } as const))
          })
          const labelled = parseCausalForestEvidence({ ...evidence.value, features })
          if (!labelled.ok) { dispatch({ type: 'run-failed', detail: 'The encoded covariate labels do not match the forest result.' }); return }
          const run = { kind: 'causal-forest-run', configuration, evidence: labelled.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact,
            'The forest could not report the recorded target for every required observation. Review the sample and forest settings.')
          return
        }
        case 't-learner': {
          const inputs = tLearnerInputs(identification.result.adjustment.variables, study.estimand)
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...inputs]
          const matrix = await materialise(columns)
          if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `The T-learner fits one outcome model per arm, so it cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.` }); return }
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const shared = { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2).flat() }
          const model = configuration.model
          if (model.kind === 'boosted-cross-fitted' && columnAt(matrix.values, matrix.rowCount, 1).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `${study.outcome.name} is not 0 or 1; the boosted learner fits a classifier per arm.` }); return }
          const evidence = await (async () => {
            switch (model.kind) {
              case 'forest': return analysis.runTLearner(design.value.values, matrix.rowCount, design.value.columnCount, { ...shared, seed: model.seed, uncertainty: model.uncertainty })
              case 'boosted-cross-fitted': {
                // Each arm's grid is searched across a worker pool on that arm's rows, which is
                // the search the kernel would run itself; the kernel then fits the two choices.
                const command = boostedGridCommand(model.grid)
                const chooseArm = async (arm: 0 | 1): Promise<Result<ArmChoice, AnalysisWorkerProblem>> => {
                  const rows = columnAt(design.value.values, matrix.rowCount, 0).flatMap((value, row) => (value === arm ? [row] : []))
                  const armValues = new Float64Array(rows.length * design.value.columnCount)
                  for (let column = 0; column < design.value.columnCount; column += 1) {
                    rows.forEach((row, at) => { armValues[column * rows.length + at] = design.value.values[column * matrix.rowCount + row]! })
                  }
                  // The pooled search predicts its `treatment` column, so the outcome takes that place.
                  const searched = await runBoostedGridSearch(armValues, rows.length, design.value.columnCount,
                    { treatment: 1, outcome: 0, adjustment: shared.adjustment }, command)
                  if (!searched.ok) return searched
                  const { learningRate, maxDepth, nEstimators, meanScore } = searched.value.best
                  return ok({ learningRate, maxDepth, nEstimators, validationAuc: meanScore })
                }
                const treated = await chooseArm(1)
                if (!treated.ok) return treated
                if (!session.current(current)) return err({ kind: 'analysis-cancelled', detail: 'The analysis run was cancelled.' } as const)
                const control = await chooseArm(0)
                if (!control.ok) return control
                if (!session.current(current)) return err({ kind: 'analysis-cancelled', detail: 'The analysis run was cancelled.' } as const)
                return analysis.runCrossFittedTLearner(design.value.values, matrix.rowCount, design.value.columnCount,
                  { ...shared, ...command, selection: { kind: 'chosen', treated: treated.value, control: control.value } })
              }
              default: return assertNever(model)
            }
          })()
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
          if (configuration.primary === 'staggered') {
            if(panelBinding===null) {dispatch({type:'run-failed',detail:'Select panel unit, time, outcome and treatment columns.'});return}
            const { materializePanelInWorker } = await import('@/data/client')
            const materialized=await materializePanelInWorker(source.file,profile,{unit:panelBinding.unit,time:panelBinding.time,outcome:panelBinding.outcome,treatment:panelBinding.treatment,covariates:configuration.covariates,clusterColumn:configuration.clustering?.kind==='column'?configuration.clustering.column:undefined})
            if(!session.current(current)) return
            if(!materialized.ok){dispatch({type:'run-failed',detail:describePanelDataProblem(materialized.error)});return}
            const input=staggeredInput(materialized.value,configuration.specification,configuration.clustering)
            if(!input.ok){dispatch({type:'run-failed',detail:input.error});return}
            const evidence=await analysis.runStaggeredDid(input.value.values,input.value.model)
            if(!evidence.ok){dispatch({type:'run-failed',detail:describeAnalysisWorkerProblem(evidence.error)});return}
            const columns:NonEmptyArray<StudyVariable>=[study.outcome,study.treatment,...configuration.covariates.map(column=>({column,node:study.outcome.node,name:profile.columns.find(c=>c.id===column)?.name??String(column)}))]
            const labels=evidence.value.times.map(time=>materialized.value.periods.find(p=>p.code===time)?.label??String(time))
            if(!isNonEmpty(labels)){dispatch({type:'run-failed',detail:'No retained panel periods were returned.'});return}
            const run={kind:'panel-intervention-run',configuration,evidence:evidence.value,timeLabels:labels} as const
            const estimate=causalEstimateFrom(study,identification,run)
            finish(estimate===null?null:{...identity,...run,method:methodIdOf(configuration.kind),columns,estimate},'The staggered DiD result does not match the treated-group study target.')
            return
          }
          if (panelBinding === null || state.panelPreflight.kind !== 'ready' || !samePanelBinding(state.panelPreflight.binding, panelBinding)) {
            dispatch({ type: 'run-failed', detail: 'The panel treatment-layout check has not completed for this study.' })
            return
          }
          if (configuration.primary === 'adjusted') {
            const { materializePanelInWorker } = await import('@/data/client')
            const materialized = await materializePanelInWorker(source.file, profile, { unit: panelBinding.unit, time: panelBinding.time, outcome: panelBinding.outcome, treatment: panelBinding.treatment, covariates: configuration.covariates })
            if (!session.current(current)) return
            if (!materialized.ok) { dispatch({ type: 'run-failed', detail: describePanelDataProblem(materialized.error) }); return }
            const matrix = materialized.value
            const evidence = await analysis.runAdjustedDid(matrix.values, matrix.rowCount, 2+configuration.covariates.length, matrix.units, matrix.periodCodes, configuration.specification)
            if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
            const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment, ...configuration.covariates.map(column => ({ column, node: study.outcome.node, name: profile.columns.find(c => c.id === column)?.name ?? String(column) }))]
            const run = { kind: 'panel-intervention-run', configuration, evidence: evidence.value, timeLabels: mapNonEmpty(state.panelPreflight.layout.periods, period => period.label) } as const
            const estimate = causalEstimateFrom(study, identification, run)
            finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate }, 'The adjusted panel result does not match its study target.')
            return
          }
          const matrix = state.panelPreflight.matrix
          const evidence = await analysis.runPanelIntervention(
            matrix.values.slice(),
            matrix.rowCount,
            matrix.units,
            matrix.periodCodes,
            { placeboReplications: configuration.placeboReplications, seed: configuration.seed, primary: configuration.primary },
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
          if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) { dispatch({ type: 'run-failed', detail: `The Gaussian model cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.` }); return }
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const evidence = await analysis.runBayesianGaussian(design.value.values, matrix.rowCount, design.value.columnCount, { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2).flat(), warmup: configuration.warmup, samples: configuration.samples, seed: configuration.seed })
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
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const evidence = await analysis.runCountGlm(design.value.values, matrix.rowCount, design.value.columnCount, { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2).flat(), family: configuration.kind === 'poisson-glm' ? 'poisson' : 'negativeBinomial' })
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
          const postEnd = evaluatedEnd(configuration.window, matrix.rowCount)
          if (postEnd <= nPre) { dispatch({ type: 'run-failed', detail: `The evaluated window ends at row ${postEnd}, which is not after the intervention row. Choose a later row.` }); return }
          const design = { outcome: 1, controls: controlVariables.map((_, index) => index + 2), nPre, postEnd }
          const evidence = configuration.inference === undefined
            ? await analysis.runCausalImpact(matrix.values, matrix.rowCount, columns.length, { ...design, maxIter: configuration.maxIter })
            : configuration.inference.kind === 'structural'
            ? await analysis.runStructuralCausalImpact(matrix.values, matrix.rowCount, columns.length, {
                ...design, draws:configuration.inference.draws, warmup:configuration.inference.warmup,
                seed:configuration.inference.seed, model:configuration.inference.model,
              })
            : await analysis.runBayesianCausalImpact(matrix.values, matrix.rowCount, columns.length, {
                ...design, draws: configuration.inference.draws, warmup: configuration.inference.warmup,
                seed: configuration.inference.seed, priorLevelSd: configuration.inference.priorLevelSd,
              })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'causal-impact-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The impact run produced no post-intervention rows.')
          return
        }
        case 'propensity-weighting':
        case 'propensity-matching':
        case 'doubly-robust': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const shared = { treatment: 0, outcome: 1, adjustment: design.value.expanded.slice(2).flat() }
          const logistic = configuration.model === 'lbfgsb'
            ? { kind: 'lbfgsb', maxIter: configuration.maxIter } as const
            : { kind: 'newton' } as const
          const drawn = (uncertainty: PropensityUncertainty) => uncertainty.kind === 'none'
            ? null
            : { rounds: uncertainty.rounds, seed: uncertainty.seed, level: uncertainty.level }
          const treatmentModel = (picked: { readonly grid: BoostedTreatmentModelChoice; readonly searched: number } | null): PropensityTreatmentModel =>
            picked === null
              ? { kind: 'logistic', model: logistic }
              : { kind: 'boosted', model: boostedCommand(picked.grid, picked.searched) }
          const record = (run: Parameters<typeof causalEstimateFrom>[2]) => {
            const estimate = causalEstimateFrom(study, identification, run)
            finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact,
              'The identification is no longer identified.')
          }
          // A boosted model searches the grid across a worker pool, then fits the one candidate
          // it chose, so the long part of the run is not on a single worker. Doubly robust has no
          // boosted arm, so there is no grid to search and nothing to choose.
          type ChosenGrid = { readonly grid: BoostedTreatmentModelChoice; readonly searched: number }
          const search = async (): Promise<Result<ChosenGrid | null, AnalysisWorkerProblem>> => {
            if (configuration.kind === 'doubly-robust' || configuration.model !== 'boosted') return ok(null)
            const declared = configuration.boosted
            const outcome = await runBoostedGridSearch(design.value.values.slice(), matrix.rowCount,
              design.value.columnCount, shared, boostedCommand(declared))
            if (!outcome.ok) return outcome
            return ok({
              searched: declared.learningRate.length * declared.maxDepth.length * declared.nEstimators.length,
              grid: {
                ...declared,
                learningRate: [outcome.value.best.learningRate],
                maxDepth: [outcome.value.best.maxDepth],
                nEstimators: [outcome.value.best.nEstimators],
              },
            })
          }
          const searched = await search()
          if (!session.current(current)) return
          if (!searched.ok) {
            dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(searched.error) })
            return
          }
          const chosen = searched.value
          if (configuration.kind === 'propensity-weighting') {
            const fit = chosen === null
              ? { kind: 'logistic', model: logistic, bootstrap: drawn(configuration.uncertainty) } as const
              : { kind: 'boosted', model: boostedCommand(chosen.grid, chosen.searched) } as const
            const evidence = await analysis.runPropensityWeighting(design.value.values, matrix.rowCount, design.value.columnCount,
              { ...shared, scale: configuration.scale, fit })
            if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
            record({ kind: 'propensity-weighting-run', configuration, evidence: evidence.value })
            return
          }
          if (configuration.kind === 'propensity-matching') {
            const evidence = await analysis.runPropensityMatching(design.value.values, matrix.rowCount, design.value.columnCount,
              { ...shared, model: treatmentModel(chosen) })
            if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
            record({ kind: 'propensity-matching-run', configuration, evidence: evidence.value })
            return
          }
          const evidence = await analysis.runDoublyRobust(design.value.values, matrix.rowCount, design.value.columnCount,
            { ...shared, model: logistic, bootstrap: drawn(configuration.uncertainty) })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          record({ kind: 'doubly-robust-run', configuration, evidence: evidence.value })
          return
        }
        case 'continuous-gps': {
          const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...identification.result.adjustment.variables]
          const matrix = await materialise(columns)
          const design = expandDesign(matrix.values, matrix.rowCount, columns.length, designLayouts(columns, 2, configuration.kind, state.encodings))
          if (!design.ok) { dispatch({ type: 'run-failed', detail: describeDesignExpansionProblem(design.error, columns.map((column) => column.name)) }); return }
          const evidence = await analysis.runContinuousGps(design.value.values, matrix.rowCount, design.value.columnCount, {
            treatment: 0, outcome: 1,
            adjustment: design.value.expanded.slice(2).flat(),
            scale: configuration.scale,
            bootstrap: configuration.uncertainty.kind === 'none' ? null
              : { rounds: configuration.uncertainty.rounds, seed: configuration.uncertainty.seed, level: configuration.uncertainty.level },
          })
          if (!evidence.ok) { dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) }); return }
          const run = { kind: 'continuous-gps-run', configuration, evidence: evidence.value } as const
          const estimate = causalEstimateFrom(study, identification, run)
          finish(estimate === null ? null : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate } as EstimationRunArtifact, 'The identification is no longer identified.')
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
  // A unit or cluster column identifies groups of rows; it cannot also be a regressor. A panel's own unit key comes first.
  const designColumns = new Set(identification !== null && 'adjustment' in identification.result ? identification.result.adjustment.variables.map((variable) => variable.column) : [])
  const panelUnit = prepared.kind === 'prepared-panel' ? profile.columns.find((column) => column.id === prepared.sampling.unitColumn) ?? null : null
  const groupingCandidates = [...(panelUnit === null ? [] : [panelUnit]), ...controlCandidates.filter((column) => !designColumns.has(column.id) && column.id !== panelUnit?.id)]
  const panelTime = prepared.kind === 'prepared-panel' ? profile.columns.find((column) => column.id === prepared.sampling.timeColumn) ?? null : null
  const timeCandidates = [...(panelTime === null ? [] : [panelTime]), ...groupingCandidates.filter((column) => column.id !== panelTime?.id)]
  const temporalAdjustmentCandidates = document === null || study === null
    ? []
    : document.current.graph.nodes.flatMap((node, index) => node.kind === 'observed'
      ? [{ index, name: node.name }]
      : [])
  const temporalAdjustmentMaxLag = document === null || configuration.kind !== 'causal-effects-total'
    ? 0
    : stationaryMarksOf(document).statLag + configuration.treatmentLag

  const encodingVariables = identification?.result.kind !== 'identified' ? []
    : state.estimator === 'causal-forest' ? causalForestInputs(identification.result.adjustment.variables, study?.estimand ?? null)
    : state.estimator === 't-learner' ? tLearnerInputs(identification.result.adjustment.variables, study?.estimand ?? null)
    : identification.result.adjustment.variables
  const categoricalChecklist = selectedEstimatorIsVisible && expandsDesign(state.estimator)
    && encodingVariables.length > 0
    ? (
      <ColumnChecklist
                  title="Categorical covariates"
                  help="Tick the covariates whose values are categories rather than quantities. A ticked covariate becomes one column per level instead of one numeric column."
                  columns={encodingVariables.map((variable) => ({ id: variable.column, name: variable.name }))}
                  selected={encodingVariables
                    .filter((variable) => state.encodings[variable.column]?.kind === 'categorical')
                    .map((variable) => variable.column)}
                  onChange={(selected) => {
                    for (const variable of encodingVariables) {
                      const encoding = selected.includes(variable.column) ? 'categorical' as const : 'numeric' as const
                      if ((state.encodings[variable.column]?.kind ?? 'numeric') !== encoding) {
                        dispatch({ type: 'encoding-declared', column: variable.column, encoding: { kind: encoding } })
                      }
                    }
                  }} />
    )
    : null

  const covariatesStep = (number: number) => categoricalChecklist === null
    ? null
    : <SettingsStep number={number} title="Covariates">{categoricalChecklist}</SettingsStep>

  const controls = ((): React.ReactNode => {
    switch (configuration.kind) {
      case 'sharp-rd': return <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>{study?.estimand.kind === 'local-cutoff-effect' ? `Treatment is 1 where ${study.estimand.running.name} is at least ${study.estimand.cutoff}, and 0 below. ` : 'Choose a cutoff-local study. '}Local-linear fits use a triangular kernel, automatic mserd bandwidth and nearest-neighbor variance with three neighbors. The headline uses robust bias-corrected inference.</p>
      case 'frontdoor-two-stage':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Contrast">
              <div className={fieldRow.two}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Control value" help={ESTIMATION_PARAMETER_HELP.frontdoor.controlValue} /><input type="number" step="any" aria-label="Front-door control value" className={field('text', 'mt-1 w-full')} value={configuration.interventions[0]} onChange={(event) => configure({ ...configuration, interventions: [Number(event.target.value) || 0, configuration.interventions[1]] })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Treatment value" help={ESTIMATION_PARAMETER_HELP.frontdoor.treatmentValue} /><input type="number" step="any" aria-label="Front-door treatment value" className={field('text', 'mt-1 w-full')} value={configuration.interventions[1]} onChange={(event) => configure({ ...configuration, interventions: [configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
              </div>
            </SettingsStep>
            <SettingsStep number={2} title="Report uncertainty">
              <div className={fieldRow.two}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap resamples" help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapResamples} /><input type="number" min={20} max={5000} aria-label="Front-door bootstrap resamples" className={field('text', 'mt-1 w-full')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.frontdoor.bootstrapSeed} /><input type="number" min={0} aria-label="Front-door bootstrap seed" className={field('text', 'mt-1 w-full')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
              </div>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The first regression estimates treatment → mediator. The second estimates mediator → outcome while adjusting for treatment. Their product gives the linear front-door contrast; the interval uses a seeded row bootstrap.</p>
            </SettingsStep>
          </div>
        )
      case 'instrumental-variable':
        return (
          <SettingsStep title="Report uncertainty">
            <div className={fieldRow.two}>
              <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap resamples" help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapResamples} /><input type="number" min={20} max={5000} aria-label="Instrumental-variable bootstrap resamples" className={field('text', 'mt-1 w-full')} value={configuration.simulations} onChange={(event) => configure({ ...configuration, simulations: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.instrumentalVariable.bootstrapSeed} /><input type="number" min={0} aria-label="Instrumental-variable bootstrap seed" className={field('text', 'mt-1 w-full')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The estimate is the ratio of the instrument’s effect on the outcome to its effect on the treatment: the Wald estimator for one binary instrument, a covariance ratio for one continuous instrument, and two-stage least squares otherwise. The effect is reported for the treatment set to 1 rather than 0; the interval uses a seeded row bootstrap.</p>
          </SettingsStep>
        )
      case 'backdoor-linear-regression':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Fixed effects">
            <div>
            <ParameterLabel className={fieldLabel} label="Fixed effects" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.fixedEffects} />
            <SegmentedControl className="mt-1" ariaLabel="Fixed effects" value={configuration.fixedEffects.kind} onChange={(kind) => {
              const current = configuration.fixedEffects
              const unit = kind === 'time' ? panelTime ?? timeCandidates[0] ?? null : current.kind === 'none' || current.kind === 'time' ? groupingCandidates[0] ?? null : { id: current.column, name: current.name }
              const time = current.kind === 'unit-and-time' ? { id: current.timeColumn, name: current.timeName } : timeCandidates.find((candidate) => candidate.id !== unit?.id) ?? null
              const selection = selectFixedEffects(kind, unit, time)
              if (!selection.ok) return
              const fixedEffects = selection.value
              // Series error processes are not supported by the within fit. Clustering is independent.
              const errors: LinearErrors = fixedEffects.kind !== 'none'
                ? (configuration.errors.kind === 'hac' || configuration.errors.kind === 'arma' ? { kind: 'classical' } : configuration.errors)
                : configuration.errors
              configure({ ...configuration, fixedEffects, errors })
            }} options={[
              { value: 'none', label: 'None' },
              { value: 'unit', label: 'By unit', ariaLabel: 'By unit', disabled: groupingCandidates.length === 0, title: groupingCandidates.length === 0 ? 'Requires a grouping column outside the regression design.' : undefined },
              { value: 'time', label: 'By time', ariaLabel: 'By time', disabled: timeCandidates.length === 0, title: timeCandidates.length === 0 ? 'Requires a period column outside the regression design.' : undefined },
              { value: 'unit-and-time', label: 'By unit and time', ariaLabel: 'By unit and time', disabled: !groupingCandidates.some(unit => timeCandidates.some(time => time.id !== unit.id)), title: !groupingCandidates.some(unit => timeCandidates.some(time => time.id !== unit.id)) ? 'Requires distinct unit and period columns outside the regression design.' : undefined },
            ]} />
            <p className={cn(fieldHint, 'max-w-[65ch]')}>{describeFixedEffectsChoice(configuration.fixedEffects)}</p>
            {configuration.fixedEffects.kind !== 'none' && <div className="mt-3">
              <ParameterLabel className={fieldLabel} label={configuration.fixedEffects.kind === 'time' ? 'Time column' : 'Unit column'} help={configuration.fixedEffects.kind === 'time' ? ESTIMATION_PARAMETER_HELP.adjustedRegression.timeColumn : ESTIMATION_PARAMETER_HELP.adjustedRegression.unitColumn} />
              {(configuration.fixedEffects.kind === 'time' ? timeCandidates : groupingCandidates).length === 0
                ? <p className={cn(fieldHint, 'max-w-[65ch]')}>No prepared column is outside the design. Prepare the dataset again with the unit column selected, and leave that column out of the graph.</p>
                : <Select aria-label={configuration.fixedEffects.kind === 'time' ? 'Time column' : 'Unit column'} className={field('text', 'mt-1 w-full max-w-xs')} value={configuration.fixedEffects.column} onChange={(event) => {
                  const column = (configuration.fixedEffects.kind === 'time' ? timeCandidates : groupingCandidates).find((candidate) => candidate.id === event.target.value)
                  if (column === undefined) return
                  if (configuration.fixedEffects.kind === 'unit-and-time' && column.id === configuration.fixedEffects.timeColumn) return
                  const fixedEffects: FixedEffects = configuration.fixedEffects.kind === 'unit-and-time'
                    ? { ...configuration.fixedEffects, column: column.id, name: column.name }
                    : { kind: configuration.fixedEffects.kind === 'time' ? 'time' : 'unit', column: column.id, name: column.name }
                  configure({ ...configuration, fixedEffects })
                }}>
                  {(configuration.fixedEffects.kind === 'time' ? timeCandidates : groupingCandidates).filter(candidate => configuration.fixedEffects.kind !== 'unit-and-time' || candidate.id !== configuration.fixedEffects.timeColumn).map((candidate) => <option key={candidate.id} value={candidate.id}>{candidate.name}</option>)}
                </Select>}
            </div>}
            {configuration.fixedEffects.kind === 'unit-and-time' && <div className="mt-3">
              <ParameterLabel className={fieldLabel} label="Time column" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.timeColumn} />
              {timeCandidates.filter((candidate) => configuration.fixedEffects.kind === 'unit-and-time' && candidate.id !== configuration.fixedEffects.column).length === 0
                ? <p className={cn(fieldHint, 'max-w-[65ch]')}>No prepared column is outside the design besides the unit column. Prepare the dataset again with the time column selected, and leave that column out of the graph.</p>
                : <Select aria-label="Time column" className={field('text', 'mt-1 w-full max-w-xs')} value={configuration.fixedEffects.timeColumn} onChange={(event) => {
                  const column = timeCandidates.find((candidate) => candidate.id === event.target.value)
                  if (column === undefined || configuration.fixedEffects.kind !== 'unit-and-time') return
                  configure({ ...configuration, fixedEffects: { ...configuration.fixedEffects, timeColumn: column.id, timeName: column.name } })
                }}>
                  {timeCandidates.filter((candidate) => configuration.fixedEffects.kind === 'unit-and-time' && candidate.id !== configuration.fixedEffects.column).map((candidate) => <option key={candidate.id} value={candidate.id}>{candidate.name}</option>)}
                </Select>}
            </div>}
            </div>
            </SettingsStep>
            <SettingsStep number={categoricalChecklist === null ? undefined : 3} title="Report uncertainty">
            <div>
            <ParameterLabel className={fieldLabel} label="Errors" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.interval} />
            <SegmentedControl className="mt-1" ariaLabel="Error treatment" value={configuration.errors.kind} onChange={(kind) => {
              const unit = configuration.fixedEffects.kind === 'none' ? null : configuration.fixedEffects
              const candidate = panelUnit ?? timeCandidates[0]
              const clusterBy = configuration.errors.kind === 'cluster' ? configuration.errors : candidate === undefined ? unit : { column: candidate.id, name: candidate.name }
              if (kind === 'cluster') {
                if (clusterBy === null) return
                configure({ ...configuration, errors: { kind, column: clusterBy.column, name: clusterBy.name } })
              } else configure({ ...configuration, errors: kind === 'arma' ? { kind, p: 1, q: 0, maxIter: DEFAULT_ARMA_ITERATIONS } : { kind } })
            }} options={configuration.fixedEffects.kind !== 'none'
              ? [{ value: 'classical', label: 'Classical' }, { value: 'hc1', label: 'Robust (HC1)' }, { value: 'cluster', label: 'Clustered', disabled: timeCandidates.length === 0, title: timeCandidates.length === 0 ? 'Requires a cluster column outside the regression design.' : undefined }]
              : [{ value: 'classical', label: 'Classical' }, { value: 'hc1', label: 'Robust (HC1)' }, { value: 'cluster', label: 'Clustered', disabled: timeCandidates.length === 0, title: timeCandidates.length === 0 ? 'Requires a cluster column outside the regression design.' : undefined }, { value: 'hac', label: 'Newey–West HAC' }, { value: 'arma', label: 'ARMA errors' }]} />
            <p className={cn(fieldHint, 'max-w-[65ch]')}>{describeErrorTreatment(configuration.errors, configuration.fixedEffects)}</p>
            {configuration.errors.kind === 'hc1' && <div className="formula mt-1 max-w-[65ch] text-body"><Formula {...hc1Factor(configuration.fixedEffects.kind !== 'none')} /></div>}
            {configuration.errors.kind === 'hac' && <div className="formula mt-1 max-w-[65ch] text-body"><Formula {...HAC_BANDWIDTH} /></div>}
            {configuration.errors.kind === 'cluster' && <div className="mt-3">
              <ParameterLabel className={fieldLabel} label="Cluster column" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.clusterColumn} />
              {timeCandidates.length === 0
                ? <p className={cn(fieldHint, 'max-w-[65ch]')}>No prepared column is outside the design. Prepare the dataset again with the cluster column selected, and leave that column out of the graph.</p>
                : <Select aria-label="Cluster column" className={field('text', 'mt-1 w-full max-w-xs')} value={configuration.errors.column} onChange={(event) => { const column = timeCandidates.find((candidate) => candidate.id === event.target.value); if (column !== undefined) configure({ ...configuration, errors: { kind: 'cluster', column: column.id, name: column.name } }) }}>
                  {timeCandidates.map((candidate) => <option key={candidate.id} value={candidate.id}>{candidate.name}</option>)}
                </Select>}
            </div>}
            {configuration.errors.kind === 'arma' && <div className="mt-3 grid gap-3 @md/panel:grid-cols-3">
              <label className="block"><ParameterLabel className={fieldLabel} label="Autoregressive order" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.autoregressiveOrder} />
                <input aria-label="Autoregressive order" type="number" min={0} max={MAX_ARMA_ORDER} className={field('text', 'mt-1')} value={configuration.errors.p} onChange={(event) => configure({ ...configuration, errors: { kind: 'arma', p: Number(event.target.value), q: configuration.errors.kind === 'arma' ? configuration.errors.q : 0, maxIter: configuration.errors.kind === 'arma' ? configuration.errors.maxIter : DEFAULT_ARMA_ITERATIONS } })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Moving-average order" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.movingAverageOrder} />
                <input aria-label="Moving-average order" type="number" min={0} max={MAX_ARMA_ORDER} className={field('text', 'mt-1')} value={configuration.errors.q} onChange={(event) => configure({ ...configuration, errors: { kind: 'arma', p: configuration.errors.kind === 'arma' ? configuration.errors.p : 1, q: Number(event.target.value), maxIter: configuration.errors.kind === 'arma' ? configuration.errors.maxIter : DEFAULT_ARMA_ITERATIONS } })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Optimiser iterations" help={ESTIMATION_PARAMETER_HELP.adjustedRegression.armaIterations} />
                <input aria-label="Optimiser iterations" type="number" min={1} className={field('text', 'mt-1')} value={configuration.errors.maxIter} onChange={(event) => configure({ ...configuration, errors: { kind: 'arma', p: configuration.errors.kind === 'arma' ? configuration.errors.p : 1, q: configuration.errors.kind === 'arma' ? configuration.errors.q : 0, maxIter: Number(event.target.value) } })} /></label>
            </div>}
            </div>
            </SettingsStep>
          </div>
        )
      case 'dml-plr':
      case 'dml-irm':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Cross-fitting">
            {configuration.kind === 'dml-irm' && (
              <div>
                <span className={fieldLabel}>Study target</span>
                <p className="mb-0 mt-1 text-body text-ink">{configuration.att ? 'Effect on the treated (ATT)' : 'Average treatment effect (ATE)'}</p>
                <p className={cn(fieldHint, 'mt-1')}>Change the target in Study design, not in the estimator.</p>
              </div>
            )}
            <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Fold seed" help={ESTIMATION_PARAMETER_HELP.dml.foldSeed} /><input type="number" min={0} aria-label="Fold seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Five shuffled folds, 200 random-forest trees, minimum leaf 5, learner seed 7. The Sensitivity section repeats this fit at the same seed before its refuters.</p>
            </SettingsStep>
          </div>
        )
      case 'propensity-weighting':
      case 'propensity-matching':
      case 'doubly-robust': {
        const uncertainty = configuration.kind === 'propensity-matching' || configuration.model === 'boosted' ? null : configuration.uncertainty
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Fit the propensity score">
              <div>
                <ParameterLabel className={fieldLabel} label="Treatment model" help={ESTIMATION_PARAMETER_HELP.propensity.treatmentModel} />
                <SegmentedControl className="mt-1 max-w-md" fill ariaLabel="Treatment model" value={configuration.model}
                  options={configuration.kind === 'doubly-robust'
                    ? [{ value: 'newton', label: 'Newton' }, { value: 'lbfgsb', label: 'L-BFGS-B' }]
                    : [{ value: 'newton', label: 'Newton' }, { value: 'lbfgsb', label: 'L-BFGS-B' }, { value: 'boosted', label: 'Boosted' }]}
                  onChange={(model) => configure(configuration.kind === 'doubly-robust'
                    ? { ...configuration, model: model === 'newton' ? 'newton' : 'lbfgsb' }
                    : { ...configuration, model: model === 'newton' || model === 'lbfgsb' || model === 'boosted' ? model : 'newton' })} />
              </div>
              {configuration.model === 'lbfgsb' && (
                <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Iteration limit" help={ESTIMATION_PARAMETER_HELP.propensity.maxIter} /><input type="number" min={1} max={100000} aria-label="Iteration limit" className={field('text', 'mt-1 w-full')} value={configuration.maxIter} onChange={(event) => configure({ ...configuration, maxIter: Math.max(1, Math.min(100000, Math.floor(Number(event.target.value) || 1))) })} /></label>
              )}
              {configuration.kind !== 'doubly-robust' && configuration.model === 'boosted' && (
                <BoostedSearchFields grid={configuration.boosted} seedHelp={ESTIMATION_PARAMETER_HELP.propensity.treeSeed}
                  onChange={(grid) => configure({ ...configuration, boosted: { ...configuration.boosted, ...grid } })}
                  scoring={{
                    summary: { icon: 'target', text: configuration.boosted.scoring === 'cross-fitted' ? 'cross-fitted' : 'one model' },
                    control: <div>
                      <ParameterLabel className={fieldLabel} label="Scoring" help={ESTIMATION_PARAMETER_HELP.propensity.crossFitted} />
                      <SegmentedControl className="mt-1" fill ariaLabel="Scoring" value={configuration.boosted.scoring}
                        options={[{ value: 'one-model', label: 'One model' }, { value: 'cross-fitted', label: 'Cross-fitted' }]}
                        onChange={(scoring) => configure({ ...configuration, boosted: { ...configuration.boosted, scoring } })} />
                    </div>,
                  }} />
              )}
              {categoricalChecklist}
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>
                {`The score is fitted on the identified adjustment set as supplied. Declare categorical covariates before the run, and read the fitted scores with the result: a score at zero or one leaves a row with no counterpart in the other arm.${configuration.model === 'boosted' ? ` The search scores ${boostedCandidateCount(configuration.boosted)} candidates by held-out ROC AUC across ${configuration.boosted.splits} folds, then refits the one it chose, and reports no bootstrap interval.` : ''}`}
              </p>
            </SettingsStep>
            {configuration.kind === 'propensity-weighting' && (
              <SettingsStep number={2} title="Weight the sample">
                <div>
                  <ParameterLabel className={fieldLabel} label="Weights" help={ESTIMATION_PARAMETER_HELP.propensity.scale} />
                  <SegmentedControl className="mt-1 max-w-md" fill ariaLabel="Weights" value={configuration.scale}
                    options={[{ value: 'inverseProbability', label: 'Inverse probability' }, { value: 'stabilized', label: 'Stabilized' }]}
                    onChange={(scale) => configure({ ...configuration, scale: scale === 'stabilized' ? 'stabilized' : 'inverseProbability' })} />
                </div>
              </SettingsStep>
            )}
            {uncertainty !== null && configuration.kind !== 'propensity-matching' && (
              <SettingsStep number={configuration.kind === 'propensity-weighting' ? 3 : 2} title="Report uncertainty">
                <BootstrapIntervalFields value={uncertainty} help={ESTIMATION_PARAMETER_HELP.propensity.uncertainty} onChange={(next) => configure({ ...configuration, uncertainty: next })} />
              </SettingsStep>
            )}
          </div>
        )
      }
      case 'continuous-gps': {
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? 1 : 2} title="Weight the sample">
              <div>
                <ParameterLabel className={fieldLabel} label="Weights" help={ESTIMATION_PARAMETER_HELP.propensity.gpsScale} />
                <SegmentedControl className="mt-1 max-w-md" fill ariaLabel="Weights" value={configuration.scale}
                  options={[{ value: 'inverseDensity', label: 'Inverse density' }, { value: 'stabilized', label: 'Stabilized' }]}
                  onChange={(scale) => configure({ ...configuration, scale: scale === 'stabilized' ? 'stabilized' : 'inverseDensity' })} />
              </div>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The treatment is regressed on the adjustment set and taken as normal around its fitted value with constant variance. The weighted model fits one slope, so the result is a single treatment response rather than a curve.</p>
            </SettingsStep>
            <SettingsStep number={categoricalChecklist === null ? 2 : 3} title="Report uncertainty">
              <BootstrapIntervalFields value={configuration.uncertainty} help={ESTIMATION_PARAMETER_HELP.propensity.uncertainty} onChange={(uncertainty) => configure({ ...configuration, uncertainty })} />
            </SettingsStep>
          </div>
        )
      }
      case 'causal-forest': {
        const analyse = (analysis: ForestAnalysis) => configure({ ...configuration, analysis: { ...analysis,
          labels: forestAnalysisColumns(analysis).flatMap(id => { const column = profile.columns.find(c => c.id === id); return column === undefined ? [] : [{ column: column.id, name: column.name }] }),
        } })
        const numbered = (number: number) => categoricalChecklist === null ? undefined : number
        return <div className="grid grid-cols-1 gap-8 @6xl/panel:grid-cols-2 @6xl/panel:items-start @6xl/panel:gap-x-16">
          {covariatesStep(1)}
          <SettingsStep number={numbered(2)} title="Fit the causal forest" className="@6xl/panel:row-span-2">
            <CausalForestControls configuration={configuration} onChange={configure} />
            <CausalForestSamplingControls value={configuration.analysis} columns={controlCandidates} onChange={analyse} />
          </SettingsStep>
          <SettingsStep number={numbered(3)} title="Summarize effect heterogeneity">
            <CausalForestAnalysisControls value={configuration.analysis} columns={controlCandidates} onChange={analyse} />
          </SettingsStep>
        </div>
      }
      case 't-learner': {
        const model = configuration.model
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Fit the outcome models">
              <div>
                <ParameterLabel className={fieldLabel} label="Outcome model" help={ESTIMATION_PARAMETER_HELP.tLearner.outcomeModel} />
                <SegmentedControl className="mt-1 max-w-md" fill ariaLabel="Outcome model" value={model.kind}
                  options={[{ value: 'forest', label: 'Random forest' }, { value: 'boosted-cross-fitted', label: 'Boosted, cross-fitted' }]}
                  onChange={(kind) => configure({ ...configuration, model: kind === 'boosted-cross-fitted'
                    ? { kind, grid: DEFAULT_BOOSTED_SEARCH }
                    : { kind: 'forest', seed: 7, uncertainty: { kind: 'none' } } })} />
              </div>
              {(() => {
                switch (model.kind) {
                  case 'forest': return <>
                    <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Learner seed" help={ESTIMATION_PARAMETER_HELP.tLearner.learnerSeed} /><input type="number" min={0} aria-label="Learner seed" className={field('text', 'mt-1')} value={model.seed} onChange={(event) => configure({ ...configuration, model: { ...model, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) } })} /></label>
                    <TLearnerUncertainty value={model.uncertainty} onChange={(uncertainty) => configure({ ...configuration, model: { ...model, uncertainty } })} />
                    <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>One random forest per treatment arm, with 200 trees and a minimum leaf size of 5. Bootstrap intervals refit both forests on resampled rows and take longer to calculate.</p>
                  </>
                  case 'boosted-cross-fitted': return <>
                    <BoostedSearchFields grid={model.grid} seedHelp={ESTIMATION_PARAMETER_HELP.tLearner.boostedSeed} onChange={(grid) => configure({ ...configuration, model: { ...model, grid } })} />
                    <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>{`Grid search compares ${boostedCandidateCount(model.grid)} candidates on all rows within each treatment arm before splitting the sample into two halves, stratified by outcome. Each half is predicted by models fitted on the other half. Hyperparameter selection is not repeated within the training halves. The outcome must be 0 or 1; no interval is calculated.`}</p>
                  </>
                  default: return assertNever(model)
                }
              })()}
            </SettingsStep>
          </div>
        )
      }
      case 'ardl-pss':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Fit the error-correction model">
              <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Maximum lag" help={ESTIMATION_PARAMETER_HELP.ardl.maximumLag} /><input type="number" min={1} max={24} aria-label="Maximum lag" className={field('text', 'mt-1 w-full')} value={configuration.maxLag} onChange={(event) => configure({ ...configuration, maxLag: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
              <div>
                <ParameterLabel className={fieldLabel} label="Deterministic terms" help={ESTIMATION_PARAMETER_HELP.ardl.deterministicTerms} />
                <SegmentedControl className="mt-1" fill ariaLabel="Deterministic terms" value={configuration.trend} onChange={(trend) => configure({ ...configuration, trend, case: trend === 'c' ? 3 : 4 })} options={[{ value: 'c', label: 'Constant' }, { value: 'ct', label: 'Constant and trend' }]} />
              </div>
            </SettingsStep>
            <SettingsStep number={2} title="Bounds test">
              <div>
                <ParameterLabel className={fieldLabel} label="PSS case" help={ESTIMATION_PARAMETER_HELP.ardl.pssCase} />
                <SegmentedControl className="mt-1" fill ariaLabel="PSS case" value={String(configuration.case)} onChange={(chosen) => { const candidate = PSS_CASES[configuration.trend].find((item) => String(item) === chosen); if (candidate !== undefined) configure({ ...configuration, case: candidate }) }} options={PSS_CASES[configuration.trend].map((candidate) => ({ value: String(candidate), label: `Case ${candidate}` }))} />
              </div>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>AIC lag search, error-correction fit, delta-method interval on the long-run effect, bounds test at the recorded case.</p>
            </SettingsStep>
          </div>
        )
      case 'vecm':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Fit the model">
              <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Maximum lags" help={ESTIMATION_PARAMETER_HELP.vecm.maximumLags} /><input type="number" min={1} max={24} aria-label="Maximum lags" className={field('text', 'mt-1 w-full')} value={configuration.maxLags} onChange={(event) => configure({ ...configuration, maxLags: Math.max(1, Math.min(24, Number(event.target.value) || 1)) })} /></label>
              <div>
                <ParameterLabel className={fieldLabel} label="Deterministic terms" help={ESTIMATION_PARAMETER_HELP.vecm.deterministicTerms} />
                <Select className={field('text', 'mt-1')} aria-label="VECM deterministic terms" value={configuration.deterministic} onChange={(event) => { const term = VECM_TERMS.find(([value]) => value === event.target.value); if (term !== undefined) configure({ ...configuration, deterministic: term[0] }) }}>
                  {VECM_TERMS.map(([value, name]) => <option key={value} value={value}>{name}</option>)}
                </Select>
              </div>
            </SettingsStep>
            <SettingsStep number={2} title="Test for cointegration">
              <div>
                <ParameterLabel className={fieldLabel} label="Trace significance" help={ESTIMATION_PARAMETER_HELP.vecm.traceSignificance} />
                <SegmentedControl className="mt-1" fill ariaLabel="Trace significance" value={String(configuration.significance)} onChange={(chosen) => { const level = TRACE_LEVELS.find((item) => String(item) === chosen); if (level !== undefined) configure({ ...configuration, significance: level }) }} options={TRACE_LEVELS.map((level) => ({ value: String(level), label: `${level}%` }))} />
              </div>
            </SettingsStep>
            <SettingsStep number={3} title="Test for a break">
              <label className="block max-w-xs"><ParameterLabel className={fieldLabel} label="Chow split after row" help={ESTIMATION_PARAMETER_HELP.vecm.chowBreakRow} /><input type="number" min={4} max={prepared.observations - 4} aria-label="Chow split after row" placeholder="none" className={field('text', 'mt-1 w-full')} value={configuration.breakIndex ?? ''} onChange={(event) => configure({ ...configuration, breakIndex: event.target.value === '' ? null : Math.max(4, Math.min(prepared.observations - 4, Math.floor(Number(event.target.value) || 4))) })} /></label>
            </SettingsStep>
          </div>
        )
      case 'synthetic-control':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Mark the intervention">
              <div>
                <ParameterLabel className={fieldLabel} label="Intervention start" help={ESTIMATION_PARAMETER_HELP.syntheticControl.interventionStart} />
                <div className="mt-1 flex flex-wrap items-center gap-2">
                  <RadioList legend="Synthetic intervention start" legendHidden value={configuration.start.kind} onChange={(kind) => configure({ ...configuration, start: kind === 'from-treatment' ? { kind: 'from-treatment' } : { kind: 'row', row: Math.max(3, Math.floor(prepared.observations / 2)) } })} options={[{ value: 'from-treatment', label: `Where ${study?.treatment.name ?? 'the treatment'} turns on` }, { value: 'row', label: 'At a row' }]} />
                  {configuration.start.kind === 'row' && (
                    <label className="text-body text-ink">First post-intervention row<input type="number" min={3} max={prepared.observations} aria-label="First post-intervention row" className={field('text', 'ml-2 w-28')} value={configuration.start.row} onChange={(event) => configure({ ...configuration, start: { kind: 'row', row: Math.max(3, Math.min(prepared.observations, Number(event.target.value) || 3)) } })} /></label>
                  )}
                </div>
              </div>
            </SettingsStep>
            <SettingsStep number={2} title="Choose donor series">
              <div className="max-w-3xl">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <ParameterLabel className={fieldLabel} label="Donor series" help={ESTIMATION_PARAMETER_HELP.syntheticControl.donorSeries} />
                  {controlCandidates.length > 0 && (
                    <SelectionActions selectLabel="Select all donor series" clearLabel="Clear selected donor series"
                      onSelectAll={() => configure({ ...configuration, donors: controlCandidates.map((column) => column.id) })}
                      onClear={() => configure({ ...configuration, donors: [] })} />
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
            </SettingsStep>
            <SettingsStep number={3} title="Report uncertainty">
              <div className={fieldRow.two}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Cross-fit folds" help={ESTIMATION_PARAMETER_HELP.syntheticControl.crossFitFolds} /><input type="number" min={2} max={20} aria-label="Cross-fit folds" className={field('text', 'mt-1 w-full')} value={configuration.crossFitFolds} onChange={(event) => configure({ ...configuration, crossFitFolds: Math.max(2, Math.min(20, Math.floor(Number(event.target.value) || 2))) })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Inference alpha" help={ESTIMATION_PARAMETER_HELP.syntheticControl.inferenceAlpha} /><input type="number" min={0.001} max={0.5} step={0.01} aria-label="Synthetic-control inference alpha" className={field('text', 'mt-1 w-full')} value={configuration.alpha} onChange={(event) => configure({ ...configuration, alpha: Math.max(0.001, Math.min(0.5, Number(event.target.value) || 0.05)) })} /></label>
              </div>
            </SettingsStep>
          </div>
        )
      case 'panel-intervention':
        return (
          <div className={stepsStack}>
            <SettingsStep number={configuration.primary === 'did' ? undefined : 1} title="Choose the method">
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Choose the method before fitting. Conventional DiD can use one period before and one after adoption.</p>
              <SegmentedControl wrap className="justify-self-start" ariaLabel="Panel method" value={configuration.primary === 'adjusted' ? configuration.specification.kind : configuration.primary ?? 'syntheticDid'} onChange={(primary) => configure(primary === 'staggered' ? {kind:'panel-intervention',primary:'staggered',covariates:[],specification:defaultStaggeredSpecification,clustering:{kind:'unit'}} : primary === 'regression' || primary === 'doublyRobust'
                ? { kind: 'panel-intervention', primary: 'adjusted', covariates: [], specification: primary === 'regression' ? { kind: 'regression' } : { kind: 'doublyRobust', folds: 2, seed: 1234, trimming: 0.01, normalization: 'in-sample' } }
                : { kind: 'panel-intervention', primary, placeboReplications: 100, seed: 0 })} options={[{ value: 'did', label: 'Conventional' }, { value: 'regression', label: 'Regression' }, { value: 'doublyRobust', label: 'Doubly robust' }, { value: 'syntheticDid', label: 'Synthetic' }, {value:'staggered',label:'Staggered adoption'}]} />
            </SettingsStep>
            {configuration.primary === 'staggered' && <SettingsStep number={2} title="Compare cohorts">
              <StaggeredDidControls configuration={configuration} candidates={controlCandidates.filter(c=>prepared.kind!=='prepared-panel'||(c.id!==prepared.panel.unitColumn&&c.id!==prepared.panel.timeColumn))} clusterCandidates={profile.columns.filter(c=>c.id!==study?.treatment.column&&c.id!==study?.outcome.column&&(prepared.kind!=='prepared-panel'||(c.id!==prepared.panel.unitColumn&&c.id!==prepared.panel.timeColumn)))} onChange={configure} />
            </SettingsStep>}
            {configuration.primary === 'adjusted' && <SettingsStep number={2} title="Adjust for covariates">
              <AdjustedDidControls configuration={configuration} candidates={controlCandidates.filter(c => prepared.kind !== 'prepared-panel' || (c.id !== prepared.panel.unitColumn && c.id !== prepared.panel.timeColumn))} onChange={configure} />
            </SettingsStep>}
            {configuration.primary !== 'did' && configuration.primary !== 'adjusted' && configuration.primary !== 'staggered' && <SettingsStep number={2} title="Report uncertainty">
              <div className={fieldRow.two}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Placebo replications" help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboReplications} /><input type="number" min={2} max={2000} aria-label="Panel placebo replications" className={field('text', 'mt-1 w-full')} value={configuration.placeboReplications} onChange={(event) => configure({ ...configuration, placeboReplications: Math.max(2, Math.min(2000, Math.floor(Number(event.target.value) || 2))) })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Placebo seed" help={ESTIMATION_PARAMETER_HELP.panelIntervention.placeboSeed} /><input type="number" min={0} max={0xffff_ffff} aria-label="Panel placebo seed" className={field('text', 'mt-1 w-full')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.min(0xffff_ffff, Math.floor(Number(event.target.value) || 0))) })} /></label>
              </div>
            </SettingsStep>}
          </div>
        )
      case 'negbin-nuts':
        return (
          <SettingsStep title="Sampler">
            <div className={fieldRow.three}>
              <label className="block"><ParameterLabel className={fieldLabel} label="Warmup" help={ESTIMATION_PARAMETER_HELP.nuts.warmup} /><input type="number" min={10} max={5000} aria-label="Warmup" className={field('text', 'mt-1 w-full')} value={configuration.warmup} onChange={(event) => configure({ ...configuration, warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Draws" help={ESTIMATION_PARAMETER_HELP.nuts.draws} /><input type="number" min={10} max={5000} aria-label="Draws" className={field('text', 'mt-1 w-full')} value={configuration.samples} onChange={(event) => configure({ ...configuration, samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Seed" help={ESTIMATION_PARAMETER_HELP.nuts.seed} /><input type="number" min={0} aria-label="Sampler seed" className={field('text', 'mt-1 w-full')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Gamma-Poisson likelihood, standardised treatment and confounder, NUTS with step-size and diagonal mass adaptation at target acceptance 0.8.</p>
          </SettingsStep>
        )
      case 'bayesian-gaussian':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <SettingsStep number={categoricalChecklist === null ? undefined : 2} title="Sampler">
            <div className={fieldRow.three}>
            <label className="block"><ParameterLabel className={fieldLabel} label="Warmup" help={ESTIMATION_PARAMETER_HELP.nuts.warmup} /><input type="number" min={10} max={5000} aria-label="Warmup" className={field('text', 'mt-1')} value={configuration.warmup} onChange={(event) => configure({ ...configuration, warmup: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Draws per chain" help={ESTIMATION_PARAMETER_HELP.nuts.draws} /><input type="number" min={10} max={5000} aria-label="Draws per chain" className={field('text', 'mt-1')} value={configuration.samples} onChange={(event) => configure({ ...configuration, samples: Math.max(10, Math.min(5000, Math.floor(Number(event.target.value) || 10))) })} /></label>
            <label className="block"><ParameterLabel className={fieldLabel} label="Seed" help={ESTIMATION_PARAMETER_HELP.nuts.seed} /><input type="number" min={0} aria-label="Sampler seed" className={field('text', 'mt-1')} value={configuration.seed} onChange={(event) => configure({ ...configuration, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) })} /></label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Normal(0, 10) intercepts, Normal(0, 1) slopes, half-normal(10) residual scale; non-binary adjustment columns are standardised; three NUTS chains with step-size and diagonal mass adaptation at target acceptance 0.8.</p>
            </SettingsStep>
          </div>
        )
      case 'discrete-bn-query':
        return (
          <SettingsStep title="Discretise and smooth">
            <div className={fieldRow.two}>
              <label className="block"><ParameterLabel className={fieldLabel} label="State budget" help={ESTIMATION_PARAMETER_HELP.discreteBn.stateBudget} /><input type="number" min={2} max={10} aria-label="State budget" className={field('text', 'mt-1 w-full')} value={configuration.bins} onChange={(event) => configure({ ...configuration, bins: Math.max(2, Math.min(10, Math.floor(Number(event.target.value) || 2))) })} /></label>
              <label className="block"><ParameterLabel className={fieldLabel} label="Equivalent sample size" help={ESTIMATION_PARAMETER_HELP.discreteBn.equivalentSampleSize} /><input type="number" min={0.1} step="any" aria-label="Equivalent sample size" className={field('text', 'mt-1 w-full')} value={configuration.equivalentSampleSize} onChange={(event) => configure({ ...configuration, equivalentSampleSize: Math.max(0.1, Number(event.target.value) || 0.1) })} /></label>
            </div>
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Observed binary and ordinal states are preserved when they fit the budget; higher-cardinality values are divided at quantiles. The BDeu prior smooths the conditional tables, and the effect contrasts the lowest and highest treatment states.</p>
          </SettingsStep>
        )
      case 'binary-ett-idc-star':
        return <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The run evaluates the two recorded IDC* expressions against the empirical binary joint distribution. It applies no discretisation and reports no sampling interval.</p>
      case 'poisson-glm':
      case 'negative-binomial-p':
        return (
          <div className={stepsStack}>
            {covariatesStep(1)}
            <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Log link on the expected count of {study?.outcome.name ?? 'the outcome'}; exponentiating the treatment coefficient gives an expected-count ratio with a 95% normal interval.</p>
          </div>
        )
      case 'negative-binomial-ingarch':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Fit the count model">
              <div><ParameterLabel className={fieldLabel} label="Mean link" help={ESTIMATION_PARAMETER_HELP.ingarch.meanLink} /><SegmentedControl className="mt-1" ariaLabel="INGARCH mean link" value={configuration.link} onChange={(link) => configure({ ...configuration, link })} options={[{ value: 'identity', label: 'Additive' }, { value: 'log', label: 'Multiplicative' }]} /></div>
              <div className={fieldRow.two}>
                <LagListField label="Past count lags" help={ESTIMATION_PARAMETER_HELP.ingarch.pastCountLags} lags={configuration.pastObservationLags} onChange={(pastObservationLags) => configure({ ...configuration, pastObservationLags })} />
                <LagListField label="Past mean lags" help={ESTIMATION_PARAMETER_HELP.ingarch.pastMeanLags} lags={configuration.pastMeanLags} onChange={(pastMeanLags) => configure({ ...configuration, pastMeanLags })} />
              </div>
              <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The additive link expresses effects in expected counts and requires non-negative regressors. The multiplicative link expresses effects on the log expected count and permits signed regressors. Both use the treatment and identified same-period adjustment variables with the selected count and mean lags; no sampling interval is reported.</p>
            </SettingsStep>
            <SettingsStep number={2} title="Forecast the contrast">
              <div><ParameterLabel className={fieldLabel} label="Treatment schedule" help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentSchedule} /><SegmentedControl className="mt-1" ariaLabel="INGARCH treatment schedule" value={configuration.schedule.kind} onChange={(kind) => configure({ ...configuration, schedule: kind === 'decaying' ? { kind: 'decaying', delta: 0.6 } : { kind } })} options={[{ value: 'point', label: 'One period' }, { value: 'persistent', label: 'Persistent' }, { value: 'decaying', label: 'Decaying' }]} /></div>
              <div className={fieldRow.three}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Forecast periods" help={ESTIMATION_PARAMETER_HELP.ingarch.forecastPeriods} /><input type="number" min={1} max={240} className={field('text', 'mt-1 w-full')} value={configuration.horizon} onChange={(event) => configure({ ...configuration, horizon: Math.max(1, Math.min(240, Math.floor(Number(event.target.value) || 1))) })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Control value" help={ESTIMATION_PARAMETER_HELP.ingarch.controlValue} /><input type="number" step="any" className={field('text', 'mt-1 w-full')} value={configuration.controlValue} onChange={(event) => configure({ ...configuration, controlValue: Number(event.target.value) || 0 })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="Treatment value" help={ESTIMATION_PARAMETER_HELP.ingarch.treatmentValue} /><input type="number" step="any" className={field('text', 'mt-1 w-full')} value={configuration.treatmentValue} onChange={(event) => configure({ ...configuration, treatmentValue: Number(event.target.value) || 0 })} /></label>
                {configuration.schedule.kind === 'decaying' && <label className="block"><ParameterLabel className={fieldLabel} label="Decay δ" help={ESTIMATION_PARAMETER_HELP.ingarch.decay} /><input type="number" min={0} max={1} step={0.05} className={field('text', 'mt-1 w-full')} value={configuration.schedule.delta} onChange={(event) => configure({ ...configuration, schedule: { kind: 'decaying', delta: Math.max(0, Math.min(1, Number(event.target.value) || 0)) } })} /></label>}
              </div>
            </SettingsStep>
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
        const later = adjustedEstimator === null ? 2 : 3
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Choose the effect model">
              <div>
                <ParameterLabel className={fieldLabel} label="Effect model" help={ESTIMATION_PARAMETER_HELP.causalEffects.effectModel} />
                <SegmentedControl className="mt-1" ariaLabel="CausalEffects model" value={configuration.estimator.kind} onChange={(kind) => configure({ ...configuration, estimator: totalEffectEstimatorFromKind(kind, configuration.estimator) })} options={[{ value: 'linear', label: 'Adjusted linear' }, { value: 'knn', label: 'Adjusted k-NN' }, { value: 'wrightParents', label: 'Wright paths' }]} />
              </div>
              {configuration.estimator.kind === 'knn' && (
                <label className="block"><ParameterLabel className={fieldLabel} label="Neighbours k" help={ESTIMATION_PARAMETER_HELP.causalEffects.neighbours} /><input type="number" min={1} max={100} className={field('text', 'mt-1 w-full')} value={configuration.estimator.k} onChange={(event) => setNeighbours(Math.max(1, Math.min(100, Number(event.target.value) || 1)))} /></label>
              )}
              {configuration.estimator.kind === 'wrightParents' && (
                <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Fits each node on its time-indexed parents, then sums products of coefficients along directed treatment-to-outcome paths. The result separates direct and indirect path contributions.</p>
              )}
            </SettingsStep>
            {adjustedEstimator !== null && (
              <SettingsStep number={2} title="Choose the adjustment set">
                <div className="grid gap-2">
                  <label className="block max-w-md">
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
                  <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>The generated choices are computed from the stationary graph. A user-supplied set is checked against every open path between treatment and outcome that is not causal before fitting.</p>
                </div>
                {explicitAdjustment !== null && (
                  <div className="grid max-w-3xl gap-2">
                    <div className="flex flex-wrap items-center justify-between gap-2">
                      <ParameterLabel className={fieldLabel} label="Adjustment members" help={ESTIMATION_PARAMETER_HELP.causalEffects.adjustmentMembers} />
                      <button type="button" className={button('quiet')} disabled={temporalAdjustmentCandidates.length === 0 || adjustmentDraft.kind === 'editing'} onClick={() => setAdjustmentDraft({ kind: 'editing', variable: null, lag: 0 })}>Add member</button>
                    </div>
                    {explicitAdjustment.nodes.length === 0 && <p className="m-0 text-body text-faint">The empty set will be tested. It is valid only when the graph has no open path between treatment and outcome that is not causal.</p>}
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
              </SettingsStep>
            )}
            <SettingsStep number={later} title="Contrast">
              <div className={fieldRow.three}>
                <label className="block"><ParameterLabel className={fieldLabel} label="Treatment lag" help={ESTIMATION_PARAMETER_HELP.causalEffects.treatmentLag} /><input type="number" min={0} max={20} className={field('text', 'mt-1 w-full')} value={configuration.treatmentLag} onChange={(event) => configure({ ...configuration, treatmentLag: Math.max(0, Math.min(20, Number(event.target.value) || 0)) })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="From value" help={ESTIMATION_PARAMETER_HELP.causalEffects.fromValue} /><input type="number" step="any" className={field('text', 'mt-1 w-full')} value={configuration.interventions[0]} onChange={(event) => configure({ ...configuration, interventions: [Number(event.target.value) || 0, configuration.interventions[1]] })} /></label>
                <label className="block"><ParameterLabel className={fieldLabel} label="To value" help={ESTIMATION_PARAMETER_HELP.causalEffects.toValue} /><input type="number" step="any" className={field('text', 'mt-1 w-full')} value={configuration.interventions[1]} onChange={(event) => configure({ ...configuration, interventions: [configuration.interventions[0], Number(event.target.value) || 0] })} /></label>
              </div>
            </SettingsStep>
            <SettingsStep number={later + 1} title="Report uncertainty">
              <div>
                <ParameterLabel className={fieldLabel} label="Sampling uncertainty" help={ESTIMATION_PARAMETER_HELP.causalEffects.uncertainty} />
                <SegmentedControl className="mt-1" ariaLabel="CausalEffects sampling uncertainty" value={configuration.uncertainty.kind} onChange={(kind) => configure({ ...configuration, uncertainty: kind === 'none' ? { kind: 'none' } : { kind: 'bootstrap', samples: 100, blockLength: { kind: 'fixed', length: 1 }, confidenceLevel: 0.9, seed: 4 } })} options={[{ value: 'bootstrap', label: 'Block bootstrap' }, { value: 'none', label: 'Point estimate' }]} />
              </div>
              {bootstrap !== null && (
                <>
                  <div className={fieldRow.three}>
                    <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap samples" help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSamples} /><input type="number" min={20} max={5000} aria-label="CausalEffects bootstrap samples" className={field('text', 'mt-1 w-full')} value={bootstrap.samples} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, samples: Math.max(20, Math.min(5000, Math.floor(Number(event.target.value) || 20))) } })} /></label>
                    <div>
                      <ParameterLabel className={fieldLabel} label="Block length" help={ESTIMATION_PARAMETER_HELP.causalEffects.blockLength} />
                      <SegmentedControl className="mt-1" ariaLabel="CausalEffects block length policy" value={bootstrap.blockLength.kind} onChange={(kind) => configure({ ...configuration, uncertainty: { ...bootstrap, blockLength: kind === 'fixed' ? { kind: 'fixed', length: 1 } : { kind: 'cubeRoot' } } })} options={[{ value: 'fixed', label: 'Fixed' }, { value: 'cubeRoot', label: 'Cube root' }]} />
                    </div>
                    {bootstrap.blockLength.kind === 'fixed' && <label className="block"><ParameterLabel className={fieldLabel} label="Observations per block" help={ESTIMATION_PARAMETER_HELP.causalEffects.observationsPerBlock} /><input type="number" min={1} aria-label="CausalEffects observations per block" className={field('text', 'mt-1 w-full')} value={bootstrap.blockLength.length} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, blockLength: { kind: 'fixed', length: Math.max(1, Math.floor(Number(event.target.value) || 1)) } } })} /></label>}
                    <label className="block"><ParameterLabel className={fieldLabel} label="Confidence level" help={ESTIMATION_PARAMETER_HELP.causalEffects.confidenceLevel} /><input type="number" min={50} max={99.9} step={0.1} aria-label="CausalEffects confidence level" className={field('text', 'mt-1 w-full')} value={bootstrap.confidenceLevel * 100} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, confidenceLevel: Math.max(0.5, Math.min(0.999, (Number(event.target.value) || 90) / 100)) } })} /></label>
                    <label className="block"><ParameterLabel className={fieldLabel} label="Bootstrap seed" help={ESTIMATION_PARAMETER_HELP.causalEffects.bootstrapSeed} /><input type="number" min={0} aria-label="CausalEffects bootstrap seed" className={field('text', 'mt-1 w-full')} value={bootstrap.seed} onChange={(event) => configure({ ...configuration, uncertainty: { ...bootstrap, seed: Math.max(0, Math.floor(Number(event.target.value) || 0)) } })} /></label>
                  </div>
                  <p className={cn(fieldHint, 'm-0 max-w-[65ch]')}>Contiguous blocks preserve the lag alignment used by the fitted graph. Choose a block length that represents the series’ dependence; the cube-root option takes the cube root of the row count.</p>
                </>
              )}
            </SettingsStep>
          </div>
        )
      }
      case 'causal-impact':
        return (
          <div className={stepsStack}>
            <SettingsStep number={1} title="Choose the model">
              <CausalImpactInference configuration={configuration} onChange={configure} />
            </SettingsStep>
            <SettingsStep number={2} title="Mark the intervention">
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
                <ParameterLabel className={fieldLabel} label="Evaluated window" help={ESTIMATION_PARAMETER_HELP.causalImpact.evaluationWindow} />
                <div className="mt-1 flex flex-wrap items-center gap-2">
                  <RadioList legend="Evaluated window" legendHidden value={configuration.window.kind} onChange={(kind) => configure({ ...configuration, window: kind === 'through-last-row' ? { kind: 'through-last-row' } : { kind: 'to-row', row: prepared.observations } })} options={[{ value: 'through-last-row', label: 'Through the last row' }, { value: 'to-row', label: 'To a row' }]} />
                  {configuration.window.kind === 'to-row' && (
                    <label className="text-body text-ink">Last evaluated row<input type="number" min={1} max={prepared.observations} aria-label="Last evaluated row" className={field('text', 'ml-2 w-28')} value={configuration.window.row} onChange={(event) => configure({ ...configuration, window: { kind: 'to-row', row: Math.max(1, Math.min(prepared.observations, Number(event.target.value) || 1)) } })} /></label>
                  )}
                </div>
              </div>
            </SettingsStep>
            <SettingsStep number={3} title="Choose control series">
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
            </SettingsStep>
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
        <p className={chapterIntro}>Identification determines how to express the causal question using observed data. Estimation applies a statistical method to that expression. In this section, you choose a compatible estimator and examine the effect estimate, its uncertainty, and the method-specific diagnostics.</p>
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
                  return <option key={candidate.id} value={candidate.id}>{bound === undefined ? candidate.id : `${estimandSentence(bound)}, ${bound.dagName}`}</option>
                })}
              </Select>
                {identification !== null && identification.result.kind === 'identified' && <span className={cn(fieldHint, 'block max-w-[65ch]')}><Metadata><span>Adjustment set: {identification.result.adjustment.variables.length === 0 ? 'none' : identification.result.adjustment.variables.map((variable) => variable.name).join(', ')}</span><span>{formatCount(study?.population.observations ?? 0).text} rows</span></Metadata></span>}
                {identification !== null && identification.result.kind === 'graphically-identified' && identification.result.frontdoor.kind === 'identified' && <span className={cn(fieldHint, 'block max-w-[65ch]')}><Metadata><span>Front-door mediator: {identification.result.frontdoor.mediators.map((variable) => variable.name).join(', ')}</span><span>{formatCount(study?.population.observations ?? 0).text} rows</span></Metadata></span>}
                {identification !== null && identifiedInstruments(identification.result) !== null && <span className={cn(fieldHint, 'block max-w-[65ch]')}><Metadata><span>Instruments: {(identifiedInstruments(identification.result) ?? []).map((variable) => variable.name).join(', ')}</span><span>{formatCount(study?.population.observations ?? 0).text} rows</span></Metadata></span>}
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
                      ? [{ value: id, label: definition.value.name, hint: id === 'panel-intervention' && candidateEligibility.kind === 'refused' ? 'Review the selected panel method and its requirements below.' : eligibilityHint(candidateEligibility), disabled: candidateEligibility.kind === 'refused' && id !== 'panel-intervention', title: candidateEligibility.kind === 'refused' ? `${definition.value.name}: ${candidateEligibility.violations[0]?.evidence ?? 'a requirement is not met'}` : undefined }]
                      : []
                  })}
                />
                {!selectedEstimatorIsVisible && <p className={cn(fieldHint, 'mt-3')}>Choose a method from this family to configure it.</p>}
                {selectedEstimatorIsVisible && method.ok && <p className={cn(fieldHint, 'mt-3 max-w-[65ch]')}>{method.value.summary}</p>}
                {selectedEstimatorIsVisible && method.ok && method.value.summaryTex !== undefined && <div className="formula max-w-[65ch] text-body"><Formula {...method.value.summaryTex} /></div>}
              </div>
              {selectedEstimatorIsVisible && <div className="mt-8">{controls}</div>}
            </div>
            <div className={cn(actionGap, 'grid gap-3')}>
            {selectedEstimatorIsVisible && eligibility !== null && <EligibilityView eligibility={eligibility} subject="this study" />}
            {studyDataError !== null && <Alert tone="danger"><p className="m-0">The treatment and outcome columns could not be checked: {studyDataError}</p></Alert>}
            <JobNotice job={job} />
            <div className="flex items-center gap-3">
              <button type="button" className={button('signal')} disabled={job.kind === 'running' || session.blocked || !selectedEstimatorIsVisible || identification === null || eligibility === null || eligibility.kind === 'refused' || studyDataPending || studyDataError !== null || adjustmentDraftOpen || (configuration.kind === 'panel-intervention' && configuration.primary !== 'staggered' && panelPreflight.kind !== 'ready')} aria-busy={job.kind === 'running'} onClick={() => void execute()}>
                {!selectedEstimatorIsVisible ? 'Choose a method' : studyDataPending ? 'Checking treatment and outcome…' : configuration.kind === 'panel-intervention' && configuration.primary !== 'staggered' && panelPreflight.kind === 'pending' ? 'Checking panel…' : `Run ${lowerFirst(describeEstimator(state.estimator))}`}
              </button>
              {job.kind === 'running' && <><Orb state="solving" aria-label="Estimator running" /><button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button></>}
            </div>
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
              <dt className="text-faint">Graph</dt><dd className="m-0 text-ink"><Metadata><span>{study.dagName}</span><span><span className={literal()}>{study.dagRevision.slice(0, 8)}</span></span></Metadata></dd>
              <dt className="text-faint">Strategy</dt><dd className="m-0 text-ink">{describeIdentificationStrategy(identification.result)}</dd>
              <dt className="text-faint">Rows</dt><dd className={num('m-0 text-ink')}><Metadata><span>{prepared.kind === 'prepared-time-series' ? 'Time series' : prepared.kind === 'prepared-panel' ? 'Panel' : 'Cross-section'}</span><span>{formatCount(prepared.observations).text}</span></Metadata></dd>
              {studyScale !== null && <><dt className="text-faint">Analysis scale</dt><dd className="m-0 text-ink">{studyScale}</dd></>}
            </dl>
          </>
        )}
      </section>
      {configuration.kind === 'panel-intervention' && configuration.primary !== 'staggered' && panelPreflight.kind === 'pending' && <p className="m-0 text-body text-muted" role="status">Checking panel structure and treatment timing…</p>}
      {configuration.kind === 'panel-intervention' && configuration.primary !== 'staggered' && panelPreflight.kind === 'ready' && <section aria-label="Checked panel structure" className="space-y-2">
        <p className="m-0 flex items-center gap-1.5 text-body text-ok"><Icon name="check_circle" size={16} />Panel structure checked</p>
        <MetricGrid label="Panel structure">
          <MetricTile size="compact" frame="cell" label="Treated / control units" value={formatWords(`${formatCount(panelPreflight.layout.treated.length).text} / ${formatCount(panelPreflight.layout.controls.length).text}`)} />
          <MetricTile size="compact" frame="cell" label="Pre / post periods" value={formatWords(`${formatCount(panelPreflight.layout.prePeriods).text} / ${formatCount(panelPreflight.layout.postPeriods).text}`)} />
          <MetricTile size="compact" frame="cell" label="Adoption period" value={formatWords(panelPreflight.layout.adoption.label)} />
        </MetricGrid>
      </section>}
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
      label: `${describeEstimator(run.configuration.kind)}${run.kind === 'backdoor-linear-run' ? `${[describeFixedEffects(run.configuration.fixedEffects), describeCovariance(run.configuration.errors)].filter((part) => part !== null).map((part) => `, ${part}`).join('')}` : ''}, ${formatTime(run.createdAt)}`,
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
            <ResultCard key={run.id} run={run} study={bound} current={false} stepLabel={stepLabel} onDelete={setPendingDelete} />
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
      inspector={{ trigger: { label: 'Requirements', icon: 'contract' }, title: 'Study and method requirements', body: inspector }}
      bottom={{ trigger: { label: 'History', icon: 'history' }, title: `Runs (${runs.length})`, body: <>{ledger}{deleteDialog}</>, defaultSize: comparison === null ? 150 : 150 + comparison.height }}
    />
    </>
  )
}
