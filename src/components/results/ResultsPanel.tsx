import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { useMemo, useState } from 'react'
import { TimeSeriesRunResult } from '@/components/time-series/TimeSeriesRunResult'
import {HonestDidResult} from '@/components/sensitivity/HonestDidPanel'
import { CountSeriesRecord } from '@/components/time-series/CountSeriesCard'
import { isRegressionDesignRun, type TimeSeriesRun } from '@/domain/timeSeries'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { EstimateHeadline } from '@/components/results/EstimateHeadline'
import { Metadata } from '@/components/ui/Metadata'
import { SurvivalRunResult, survivalRunLabel } from '@/components/survival/SurvivalRunResult'
import { RootCauseRunResult } from '@/components/root-cause/RootCauseRunResult'
import { GcmEffectResult } from '@/components/root-cause/GcmEffectsPanel'
import type { GcmEffectsRun } from '@/domain/gcmEffects'
import type { GcmInfluenceRun } from '@/domain/gcmInfluence'
import { GcmInfluenceResult } from '@/components/root-cause/GcmInfluencePanel'
import type { RootCauseRun } from '@/domain/rootCauseAnalysis'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { Formula } from '@/components/ui/Formula'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, chapterIntro, chip, label, literal, num, panel, statusText, table, td, th, tr, well } from '@/components/ui/recipes'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { Select } from '@/components/ui/Select'
import type { CounterfactualRunArtifact } from '@/domain/counterfactual'
import { describeDagBasis, describeDagValidation, type DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import { adjustmentLabels, describeEstimator, headlineValue, intervalTypeOf, type AppliedAdjustment, type EstimationRunArtifact, type EstimationRunId } from '@/domain/estimation'
import { describeSeriesTransform, frequencyUnit, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import { buildResultManifest, compareResults, manifestFileName, manifestJson, type ResultManifest } from '@/domain/results'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import type { SurvivalRunArtifact, SurvivalRunId } from '@/domain/survival'
import { describeAssignmentKind, describeEstimand, describeStudyDesignCategory, estimandSentence, identifiedExpression, identifiedExpressionTex, studyDesignCategory, type IdentificationArtifact, type StudySpecification, type StudyVariable } from '@/domain/study'
import { assertNever } from '@/domain/dop'
import { describeStationarityAssessment } from '@/domain/stationarityAssessment'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatP, formatStatistic } from '@/lib/format/number'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { interpretEstimationResult, resultHeadline, resultSampleLine, resultScaleLine } from '@/domain/resultInterpretation'

const Row = RecordRow

/** A set of variables the reader counts; empty sets read as a word. */
const names = (variables: readonly StudyVariable[]): React.ReactNode =>
  variables.length === 0 ? 'none' : variables.map((variable) => <span key={variable.node} className={chip('mr-1')}>{variable.name}</span>)

const appliedAdjustment = (adjustment: AppliedAdjustment): React.ReactNode => {
  switch (adjustment.kind) {
    case 'none': return 'none'
    case 'contemporaneous':
    case 'time-indexed': return adjustmentLabels(adjustment).map((text) => <span key={text} className={chip('mr-1')}>{text}</span>)
    case 'structural-parent-model': return `${adjustment.coefficients} parent coefficients across ${adjustment.paths} directed paths`
    default: return assertNever(adjustment)
  }
}

const identificationMethod = (identification: IdentificationArtifact): string => {
  switch (identification.result.kind) {
    case 'cutoff-design': return 'sharp RD continuity assumptions'
    case 'identified': return 'back-door adjustment'
    case 'graphically-identified': return 'general ID expression'
    case 'counterfactually-identified': return 'IDC* counterfactual expressions'
    case 'instrument-identified': return 'instrumental variables'
    case 'backdoor-not-identified': return 'not identified from the observational distribution'
    default: return assertNever(identification.result)
  }
}

/** An id the reader may need to match: mono, quiet, and short, with the whole value on hover. */
const shortId = (id: string) => <span className={literal('text-muted')} title={id}>{id.slice(0, 8)}</span>

function Section({ title, children }: { readonly title: string; readonly children: React.ReactNode }) {
  return (
    <section className="border-t border-hair pt-3 first:border-0 first:pt-0" aria-label={title}>
      <h3 className="mb-2 mt-0 text-body font-medium text-ink">{title}</h3>
      <RecordList className="text-body">{children}</RecordList>
    </section>
  )
}

function Manifest({ manifest, stepLabel }: { readonly manifest: ResultManifest; readonly stepLabel: string }) {
  const { estimation: run, study } = manifest
  const estimate = run.estimate
  const download = () => {
    const blob = new Blob([manifestJson(manifest)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const anchor = document.createElement('a')
    anchor.href = url
    anchor.download = manifestFileName(manifest)
    anchor.click()
    URL.revokeObjectURL(url)
  }
  const outcomeName = study?.outcome.name ?? 'outcome'
  const treatmentName = study?.treatment.name ?? 'treatment'
  return (
    <div className="grid gap-4">
      <article className="rounded-xl bg-panel lift p-4" aria-label="Result">
        <div className="flex flex-wrap items-baseline justify-between gap-2">
          <div>
            <span className={label('text-signal-text')}>Estimate and uncertainty</span>
            <p className="mb-0 mt-1 text-body text-muted"><Metadata><span>{describeEstimator(run.configuration.kind)}</span><span>{run.eligibility.kind === 'eligible' ? 'All pre-run checks completed' : `${run.eligibility.unresolved.length} pre-run requirements to review`}</span></Metadata></p>
          </div>
          <button type="button" className={button('outline')} onClick={download}>Export the manifest</button>
        </div>
        <div className="mt-3">
          <EstimateHeadline estimate={estimate} sentence={study === null ? run.method : resultHeadline(run, study)} scaleLine={study === null ? (estimate.effect.kind === 'expectedCountRatio' ? `Ratio of expected ${outcomeName} counts per unit increase in ${treatmentName}.` : `Difference in ${outcomeName} per unit increase in ${treatmentName}.`) : resultScaleLine(run, study, stepLabel)} sampleLine={resultSampleLine(run)} stepLabel={stepLabel} accent testId="result-figure" />
        </div>
        {study !== null && <ResultInterpretation interpretation={interpretEstimationResult(run, study, stepLabel)} className="mt-3" />}
      </article>

      {manifest.warnings.length > 0 && (
        <section className={panel('p-(--panel-space)')} aria-label="Unresolved requirements">
          <h3 className="mb-2 mt-0 text-warn text-label font-medium">Requirements to review</h3>
          <ul className="m-0 space-y-1 pl-4 text-body text-muted">
            {manifest.warnings.map((warning) => <li key={warning}>{warning}</li>)}
          </ul>
        </section>
      )}

      <div className={panel('space-y-3 p-4')} aria-label="Analysis record">
      <Section title="Estimator">
        <Row term="Method">{describeEstimator(run.configuration.kind)}</Row>
        <Row term="Configuration"><dl className="m-0 grid grid-cols-[minmax(0,1fr)_minmax(0,2fr)] gap-x-3 gap-y-1">{Object.entries(run.configuration).filter(([key]) => key !== 'kind').map(([key, value]) => <div key={key} className="contents"><dt className="text-muted [overflow-wrap:anywhere]">{key}</dt><dd className={literal('m-0 text-ink [overflow-wrap:anywhere]')}>{JSON.stringify(value)}</dd></div>)}</dl></Row>
        <Row term="Pre-run eligibility">{run.eligibility.kind === 'eligible' ? 'all checks completed' : `${run.eligibility.unresolved.length} requirements to review`}</Row>
        <Row term="Rows">{formatCount(estimate.sample.observations).text}</Row>
        <Row term="Run"><Metadata>{shortId(run.id)}<time>{formatTimestamp(run.createdAt)}</time></Metadata></Row>
      </Section>

      {study !== null && (
        <Section title="Study">
          <Row term="Target quantity">{describeEstimand(study)}</Row>
          <Row term="Treatment">{study.treatment.name}</Row>
          <Row term="Outcome">{study.outcome.name}</Row>
          <Row term="Assignment">{describeAssignmentKind(study.assignment.kind)}</Row>
          <Row term="Design category">{describeStudyDesignCategory(studyDesignCategory(study))}</Row>
          <Row term="Adjustment used by estimator">{appliedAdjustment(estimate.adjustment)}</Row>
          <Row term="Identification">{manifest.identification === null ? 'not recorded' : identificationMethod(manifest.identification)}</Row>
          {manifest.identification?.result.kind === 'identified' && (
            <>
              <Row term="Canonical adjustment set">{names(manifest.identification.result.canonicalAdjustmentSet)}</Row>
              <Row term="Minimal valid sets">{manifest.identification.result.minimalAdjustmentSets.sets.map((set, index) => <span key={index}>{index > 0 && <span className="text-faint"> or </span>}{names(set)}</span>)}{manifest.identification.result.minimalAdjustmentSets.kind === 'truncated' ? <span className="text-faint"><Metadata className="ml-3"><span>result limit reached</span></Metadata></span> : null}</Row>
              <Row term="Identified expression"><Formula tex={identifiedExpressionTex(study, manifest.identification.result.adjustment.variables)} plain={identifiedExpression(study, manifest.identification.result.adjustment.variables)} /></Row>
            </>
          )}
        </Section>
      )}

      {manifest.dag !== null && (
        <Section title="Graph">
          <Row term="Name">{manifest.dag.name}</Row>
          <Row term="Revision"><Metadata>{shortId(String(manifest.dag.revision))}<span>{describeDagValidation(manifest.dag.validation)}</span></Metadata></Row>
          <Row term="Origin">{manifest.dag.origin.kind === 'user-authored' ? describeDagBasis(manifest.dag.origin.basis) : 'substantive review of discovery results'}</Row>
          <Row term="Arrows">{formatCount(manifest.dag.graph.edges.length).text} among {formatCount(manifest.dag.graph.nodes.length).text} nodes</Row>
        </Section>
      )}

      <Section title="Data">
        <Row term="Source"><Metadata><span>{manifest.source.name}</span><span>{formatCount(manifest.source.bytes).text} bytes</span><span>Rows not included</span></Metadata></Row>
        <Row term="Prepared dataset"><Metadata>{shortId(String(manifest.prepared.id))}<span>{manifest.prepared.kind === 'prepared-time-series' ? `${manifest.prepared.sampling.frequency} series` : manifest.prepared.kind === 'prepared-panel' ? `Panel of ${manifest.prepared.panel.units} units × ${manifest.prepared.panel.periods} periods` : 'Independent rows'}</span><span>{formatCount(manifest.prepared.observations).text} rows</span></Metadata></Row>
        <Row term="Missing data">{manifest.prepared.resolution.kind === 'none'
          ? 'none'
          : manifest.prepared.resolution.kind === 'window'
            ? `rows ${manifest.prepared.resolution.start + 1} to ${manifest.prepared.resolution.endExclusive}`
            : manifest.prepared.resolution.kind === 'lag-aware-exclusion'
              ? `${manifest.prepared.resolution.cells} cells excluded during lagged sample construction`
              : `${manifest.prepared.resolution.method}, ${manifest.prepared.resolution.cells} cells`}</Row>
        <Row term="Seasonal adjustment">{manifest.prepared.seasonalAdjustment.kind === 'none' ? 'none' : `seasonal-trend decomposition using loess (STL), period ${manifest.prepared.seasonalAdjustment.period}`}</Row>
        <Row term="Series transformations">{manifest.prepared.kind !== 'prepared-time-series' ? 'not applicable' : manifest.prepared.seriesTransforms.map((record) => <span className="block" key={String(record.column)}>{manifest.schema.find((column) => column.id === String(record.column))?.name ?? record.column}: {describeSeriesTransform(record.transform)}</span>)}</Row>
        {manifest.stationarity !== null && (
          <Row term="Stationarity">
            {manifest.stationarity.variables.map((variable) => {
              const assessment = describeStationarityAssessment(variable.assessment)
              return (
                <span className="block" key={String(variable.column)}>
                  {manifest.schema.find((column) => column.id === String(variable.column))?.name ?? variable.column}
                  <span className={statusText[assessment.tone]}> {assessment.verdict}</span>
                </span>
              )
            })}
          </Row>
        )}
      </Section>

      <Section title="Sensitivity and counterfactuals">
        <Row term="Sensitivity">{manifest.sensitivity.length === 0 ? 'none recorded' : manifest.sensitivity.map((probe) => probe.kind === 'honest-did-run' ? `parallel-trends sensitivity, ${probe.evidence.request.configuration.bounds.length} restriction bounds` : probe.kind === 'linear-refutation-run' ? `movement-only perturbations, no refuter p-values (placebo ${formatStatistic('raw', probe.evidence.placeboEffect).text}; subset ${formatStatistic('raw', probe.evidence.subsetEffect).text})` : probe.kind === 'dml-refutation-run' ? `DML mean-shift tests (placebo p ${formatP(probe.evidence.placebo.pValue, { withLabel: false }).text}; random covariate p ${formatP(probe.evidence.randomCommonCause.pValue, { withLabel: false }).text})` : `simulated-confounder movement grid ${probe.evidence.kappaT.length}×${probe.evidence.kappaY.length}, no p-value`).join('; ')}</Row>
        <Row term="Counterfactuals">{manifest.counterfactuals.length === 0 ? 'none recorded' : manifest.counterfactuals.map((counterfactual) => `model-implied ${counterfactual.evidence.interventions[0]} → ${counterfactual.evidence.interventions[1]} contrast: ${formatStatistic('raw', counterfactual.evidence.averageEffect).text}`).join('; ')}</Row>
      </Section>
      </div>
    </div>
  )
}

type ResultView =
  | { readonly kind: 'root-cause' }
  | { readonly kind: 'time-series' }
  | { readonly kind: 'regression-designs' }
  | { readonly kind: 'estimation'; readonly selected: EstimationRunId | null; readonly compareWith: EstimationRunId | null }
  | { readonly kind: 'survival'; readonly selected: SurvivalRunId | null }

const initialResultView = (
  estimationRuns: readonly EstimationRunArtifact[],
  survivalRuns: readonly SurvivalRunArtifact[],
  hasTimeSeries: boolean,
  hasRootCause: boolean,
  hasDesigns: boolean,
): ResultView => estimationRuns.length > 0
  ? { kind: 'estimation', selected: estimationRuns.at(-1)?.id ?? null, compareWith: null }
  : survivalRuns.length > 0 ? { kind: 'survival', selected: survivalRuns.at(-1)?.id ?? null }
  : hasDesigns ? { kind: 'regression-designs' } : hasTimeSeries ? { kind: 'time-series' }
  : hasRootCause ? { kind: 'root-cause' } : { kind: 'survival', selected: null }

const availableResultView = (
  view: ResultView,
  estimationRuns: readonly EstimationRunArtifact[],
  survivalRuns: readonly SurvivalRunArtifact[],
  hasTimeSeries: boolean,
  hasRootCause: boolean,
  hasDesigns: boolean,
): ResultView => {
  const available = initialResultView(estimationRuns, survivalRuns, hasTimeSeries, hasRootCause, hasDesigns)
  switch (view.kind) {
    case 'root-cause': return hasRootCause ? view : available
    case 'time-series': return hasTimeSeries ? view : available
    case 'regression-designs': return hasDesigns ? view : available
    case 'estimation': {
      if (estimationRuns.length === 0) return available
      const selected = estimationRuns.some(run => run.id === view.selected)
        ? view.selected : estimationRuns.at(-1)?.id ?? null
      const compareWith = estimationRuns.some(run => run.id === view.compareWith && run.id !== selected)
        ? view.compareWith : null
      return { kind: view.kind, selected, compareWith }
    }
    case 'survival': return survivalRuns.length === 0 ? available : {
      kind: view.kind,
      selected: survivalRuns.some(run => run.id === view.selected)
        ? view.selected : survivalRuns.at(-1)?.id ?? null,
    }
    default: return assertNever(view)
  }
}

const resultIntroduction = (view: ResultView): string => {
  switch (view.kind) {
    case 'regression-designs': return 'Review regression event-time coefficients, interaction contrasts and the comparisons underlying a two-way fixed-effects coefficient. Their causal interpretation depends on the study design and identifying assumptions.'
    case 'root-cause': return 'Review the attributed changes, uncertainty and assumed causal model. Anomaly scores describe how unusual an observation is; contributions to a mean change use the target variable’s units.'
    case 'time-series': return 'Review count-model scans and long-run relationships fitted to the prepared series. These results do not by themselves estimate the effect of an intervention.'
    case 'estimation': return 'A causal result must be interpreted with its causal question, identification strategy, estimate, uncertainty, diagnostics, and assumptions. In this section, examine those parts together, compare runs when the data or analysis choices differ, and export the analysis record.'
    case 'survival': return 'Review the event definition, fitted curve or group comparison, uncertainty, and assumptions together. Survival results describe time until an event or movement between states; they are not causal effects unless a separate study design supports that interpretation.'
    default: return assertNever(view)
  }
}

export function ResultsPanel({ source, profile, prepared, stationarity, documents, studies, identifications, estimationRuns, sensitivityRuns, counterfactualRuns, survivalRuns, timeSeriesRuns, countSeriesModels, rootCauseRuns, gcmEffects, gcmInfluences }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
  readonly survivalRuns: readonly SurvivalRunArtifact[]
  readonly timeSeriesRuns: readonly TimeSeriesRun[]
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly rootCauseRuns: readonly RootCauseRun[]
  readonly gcmEffects: readonly GcmEffectsRun[]
  readonly gcmInfluences: readonly GcmInfluenceRun[]
}) {
  const designRuns = timeSeriesRuns.filter(isRegressionDesignRun)
  const temporalRuns = timeSeriesRuns.filter(run => !isRegressionDesignRun(run))
  const hasDesigns = designRuns.length > 0
  const hasTimeSeries = temporalRuns.length + countSeriesModels.length > 0
  const hasRootCause = rootCauseRuns.length + gcmEffects.length + gcmInfluences.length > 0
  const [view, setView] = useState<ResultView>(() => initialResultView(estimationRuns, survivalRuns, hasTimeSeries, hasRootCause, hasDesigns))
  const activeView = availableResultView(view, estimationRuns, survivalRuns, hasTimeSeries, hasRootCause, hasDesigns)
  const stepLabel = prepared.kind === 'prepared-time-series' ? frequencyUnit(prepared.sampling.frequency) : prepared.kind === 'prepared-panel' ? 'panel row' : 'row'
  const inputs = useMemo(() => ({ source, profile, prepared, stationarity, documents, studies, identifications, sensitivityRuns, counterfactualRuns }), [counterfactualRuns, documents, identifications, prepared, profile, sensitivityRuns, source, stationarity, studies])
  const studyOf = (candidate: EstimationRunArtifact) => studies.find((study) => study.id === candidate.study)

  const selectFamily = (kind: ResultView['kind']) => {
    switch (kind) {
      case 'root-cause': setView({ kind }); return
      case 'time-series': setView({ kind }); return
      case 'regression-designs': setView({ kind }); return
      case 'estimation': setView({ kind, selected: estimationRuns.at(-1)?.id ?? null, compareWith: null }); return
      case 'survival': setView({ kind, selected: survivalRuns.at(-1)?.id ?? null }); return
      default: return assertNever(kind)
    }
  }

  const body = (() => {
    switch (activeView.kind) {
      case 'root-cause': return <div className="space-y-6">
        {[...rootCauseRuns].reverse().map((run) => <article key={run.id}><div className="mb-2 flex flex-wrap items-baseline justify-between gap-2 text-label text-muted"><span>{run.comparison.name}</span><time>{formatTime(run.createdAt)}</time></div><RootCauseRunResult run={run} /></article>)}
        {[...gcmEffects].reverse().map(run => <GcmEffectResult key={run.id} run={run} />)}
        {[...gcmInfluences].reverse().map(run => <GcmInfluenceResult key={run.id} run={run} />)}
      </div>
      case 'time-series': return <><div className="space-y-4">{[...temporalRuns].reverse().map((run) => <TimeSeriesRunResult key={run.id} run={run} />)}</div><ul className="list-none space-y-4 p-0">{[...countSeriesModels].reverse().map((artifact) => <CountSeriesRecord key={artifact.id} artifact={artifact} open />)}</ul></>
      case 'regression-designs': return <div className="space-y-4">{[...designRuns].reverse().map(run => <div key={run.id} className="space-y-4"><TimeSeriesRunResult run={run}/>{sensitivityRuns.map(probe=>probe.kind==='honest-did-run'&&probe.source.kind==='regressionDesign'&&probe.source.run===run.id?<HonestDidResult key={probe.id} run={probe}/>:null)}</div>)}</div>
      case 'estimation': {
        const run = estimationRuns.find((candidate) => candidate.id === activeView.selected) ?? null
        const other = estimationRuns.find((candidate) => candidate.id === activeView.compareWith) ?? null
        const manifest = run === null ? null : buildResultManifest(inputs, run, new Date().toISOString())
        const otherManifest = other === null ? null : buildResultManifest(inputs, other, new Date().toISOString())
        const differences = manifest !== null && otherManifest !== null ? compareResults(manifest, otherManifest) : []
        return <>
          <div className="grid grid-cols-1 gap-3 @lg/panel:grid-cols-2">
            <label className="block"><span className="block text-body font-medium text-ink">Estimate</span><Select className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink" value={activeView.selected ?? ''} onChange={(event) => setView({ ...activeView, selected: event.target.value === '' ? null : event.target.value as EstimationRunId })}><option value="" disabled>Choose a run</option>{[...estimationRuns].reverse().map((candidate) => <option key={candidate.id} value={candidate.id}>{(studyOf(candidate) === undefined ? candidate.method : estimandSentence(studyOf(candidate) as StudySpecification))}, {describeEstimator(candidate.configuration.kind)}, {formatTime(candidate.createdAt)}</option>)}</Select></label>
            <label className="block"><span className="block text-body font-medium text-ink">Compare with</span><Select className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink" value={activeView.compareWith ?? ''} onChange={(event) => setView({ ...activeView, compareWith: event.target.value === '' ? null : event.target.value as EstimationRunId })}><option value="">None</option>{[...estimationRuns].reverse().filter((candidate) => candidate.id !== activeView.selected).map((candidate) => <option key={candidate.id} value={candidate.id}>{(studyOf(candidate) === undefined ? candidate.method : estimandSentence(studyOf(candidate) as StudySpecification))}, {describeEstimator(candidate.configuration.kind)}, {formatTime(candidate.createdAt)}</option>)}</Select></label>
          </div>
          {otherManifest !== null && manifest !== null && <section className={panel('p-(--panel-space)')} aria-label="Differences"><h3 className="mb-2 mt-0 text-faint text-label font-medium"><Metadata><span>Differences</span><span>{differences.length}</span></Metadata></h3>{differences.length === 0 ? <p className="m-0 text-body text-muted">The two runs share every recorded field.</p> : <div className="figure-strip overflow-x-auto"><table className={table}><thead><tr><th className={th()}>Field</th><th className={th()}>Selected</th><th className={th()}>Compared</th></tr></thead><tbody>{differences.map((difference) => <tr key={difference.field} className={tr()}><td className={td('text-ink')}>{difference.field}</td><td className={td('whitespace-normal text-muted')}>{difference.left}</td><td className={td('whitespace-normal text-muted')}>{difference.right}</td></tr>)}</tbody></table></div>}</section>}
          {manifest !== null && <Manifest manifest={manifest} stepLabel={stepLabel} />}
          {manifest?.sensitivity.map(probe=>probe.kind==='honest-did-run'?<HonestDidResult key={probe.id} run={probe}/>:null)}
        </>
      }
      case 'survival': {
        const run = survivalRuns.find((candidate) => candidate.id === activeView.selected) ?? null
        return <>
          <label className="block max-w-xl"><span className="block text-body font-medium text-ink">Survival run</span><Select className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink" value={activeView.selected ?? ''} onChange={(event) => {
            const selected = survivalRuns.find((candidate) => candidate.id === event.target.value)?.id ?? null
            setView({ ...activeView, selected })
          }}><option value="" disabled>Choose a run</option>{[...survivalRuns].reverse().map((candidate) => <option key={candidate.id} value={candidate.id}>{survivalRunLabel(candidate)}, {formatTime(candidate.createdAt)}</option>)}</Select></label>
          {run === null ? <div className={well('px-4 py-6 text-center text-body text-faint')}>Choose a survival run to review.</div> : <SurvivalRunResult run={run} />}
        </>
      }
      default: return assertNever(activeView)
    }
  })()

  const bottom = (() => {
    switch (activeView.kind) {
      case 'root-cause': return undefined
      case 'time-series': return undefined
      case 'regression-designs': return undefined
      case 'estimation': return {
        title: `Estimates (${estimationRuns.length})`,
        body: <div className="figure-strip overflow-x-auto"><table className={table} aria-label="Estimates"><thead><tr><th className={th()}>Target</th><th className={th()}>Estimator</th><th className={th('text-right')}>Estimate</th><th className={th()}>Graph</th><th className={th()}>Created</th></tr></thead><tbody>{[...estimationRuns].reverse().map((candidate) => { const study = studyOf(candidate); const effect = candidate.estimate.effect; return <tr key={candidate.id} className={tr(candidate.id === activeView.selected ? 'selected' : 'action')} onClick={() => setView({ ...activeView, selected: candidate.id })}><td className={td('text-ink')}>{study === undefined ? candidate.method : estimandSentence(study)}</td><td className={td('text-muted')}>{describeEstimator(candidate.configuration.kind)}</td><td className={td(num('whitespace-nowrap text-right text-ink'))}>{formatStatistic('raw', effect.kind === 'path' ? effect.aggregate.average : headlineValue(effect)).text}</td><td className={td('text-muted')}>{study?.dagName ?? '—'}</td><td className={td(num('whitespace-nowrap text-muted'))}>{formatTime(candidate.createdAt)}</td></tr> })}</tbody></table></div>,
        defaultSize: 180,
      }
      case 'survival': return {
        title: `Survival runs (${survivalRuns.length})`,
        body: <div className="figure-strip overflow-x-auto"><table className={table} aria-label="Survival runs"><thead><tr><th className={th()}>Analysis</th><th className={th()}>Records</th><th className={th()}>Created</th></tr></thead><tbody>{[...survivalRuns].reverse().map((candidate) => <tr key={candidate.id} className={tr(candidate.id === activeView.selected ? 'selected' : 'action')} onClick={() => setView({ ...activeView, selected: candidate.id })}><td className={td('text-ink')}>{survivalRunLabel(candidate)}</td><td className={td(num('text-muted'))}>{formatCount(candidate.evidence.observations).text}</td><td className={td(num('whitespace-nowrap text-muted'))}>{formatTime(candidate.createdAt)}</td></tr>)}</tbody></table></div>,
        defaultSize: 180,
      }
      default: return assertNever(activeView)
    }
  })()

  const families: { value: ResultView['kind']; label: string }[] = [
    ...(hasRootCause ? [{ value: 'root-cause' as const, label: 'Causal model analysis' }] : []),
    ...(estimationRuns.length > 0 ? [{ value: 'estimation' as const, label: 'Causal estimates' }] : []),
    ...(hasDesigns ? [{ value: 'regression-designs' as const, label: 'Regression designs' }] : []),
    ...(hasTimeSeries ? [{ value: 'time-series' as const, label: 'Time series' }] : []),
    ...(survivalRuns.length > 0 ? [{ value: 'survival' as const, label: 'Survival' }] : []),
  ]
  const familyControl = families.length > 1
    ? <SegmentedControl variant="line" size="sm" ariaLabel="Result family" value={activeView.kind} onChange={selectFamily} options={families} />
    : null

  return <WorkbenchLayout id="results" stage={<section aria-labelledby="results-title" className="@container/panel flex flex-col gap-5"><div><ChapterHeading id="results-title" className="mb-2">Results</ChapterHeading><p className={chapterIntro}>{resultIntroduction(activeView)}</p></div>{familyControl}{body}</section>} bottom={bottom} />
}
