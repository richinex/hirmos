import { useMemo, useState } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { EstimateHeadline } from '@/components/results/EstimateHeadline'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { Formula } from '@/components/ui/Formula'
import { button, chip, label, literal, num, panel, statusText, table, td, th, tr } from '@/components/ui/recipes'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { Select } from '@/components/ui/Select'
import type { CounterfactualRunArtifact } from '@/domain/counterfactual'
import { describeDagBasis, describeDagValidation, type DagDocument } from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import { adjustmentLabels, describeEstimator, intervalTypeOf, type AppliedAdjustment, type EstimationRunArtifact, type EstimationRunId } from '@/domain/estimation'
import { describeSeriesTransform, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from '@/domain/preprocessing'
import { buildResultManifest, compareResults, manifestFileName, manifestJson, type ResultManifest } from '@/domain/results'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import { describeAssignmentKind, describeEstimand, describeStudyDesignCategory, estimandSentence, identifiedExpression, identifiedExpressionTex, studyDesignCategory, type IdentificationArtifact, type StudySpecification, type StudyVariable } from '@/domain/study'
import { assertNever } from '@/domain/dop'
import { describeStationarityAssessment } from '@/domain/stationarityAssessment'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount, formatP, formatStatistic } from '@/lib/format/number'
import { formatTime, formatTimestamp } from '@/lib/format/date'
import { interpretEstimationResult, resultScaleLine } from '@/domain/resultInterpretation'

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
    case 'identified': return 'back-door adjustment'
    case 'graphically-identified': return 'general ID expression'
    case 'counterfactually-identified': return 'IDC* counterfactual expressions'
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
      <article className="rounded-xl border border-edge bg-panel p-4" aria-label="Result">
        <div className="flex flex-wrap items-baseline justify-between gap-2">
          <div>
            <span className={label('text-signal')}>Estimate and uncertainty</span>
            <p className="mb-0 mt-1 text-body text-muted">{describeEstimator(run.configuration.kind)} · {run.eligibility.kind === 'eligible' ? 'all pre-run checks completed' : `${run.eligibility.unresolved.length} pre-run requirements to review`}</p>
          </div>
          <button type="button" className={button('outline')} onClick={download}>Export the manifest</button>
        </div>
        <div className="mt-3">
          <EstimateHeadline estimate={estimate} sentence={study === null ? run.method : estimandSentence(study)} scaleLine={study === null ? (estimate.effect.kind === 'incidenceRateRatio' ? `incidence rate ratio · ${outcomeName} per unit of ${treatmentName}` : `additive · ${outcomeName} per unit of ${treatmentName}`) : resultScaleLine(run, study, stepLabel)} stepLabel={stepLabel} accent testId="result-figure" />
        </div>
        {study !== null && <ResultInterpretation interpretation={interpretEstimationResult(run, study, stepLabel)} className="mt-3" />}
      </article>

      {manifest.warnings.length > 0 && (
        <section className={panel('p-4')} aria-label="Unresolved requirements">
          <h3 className="mb-2 mt-0 text-warn text-label font-medium">Requirements to review</h3>
          <ul className="m-0 space-y-1 pl-4 text-body text-muted">
            {manifest.warnings.map((warning) => <li key={warning}>{warning}</li>)}
          </ul>
        </section>
      )}

      <div className={panel('space-y-3 p-4')} aria-label="Analysis record">
      <Section title="Estimator">
        <Row term="Method">{describeEstimator(run.configuration.kind)}</Row>
        <Row term="Configuration"><span className={literal('text-muted')}>{Object.entries(run.configuration).filter(([key]) => key !== 'kind').map(([key, value]) => `${key} ${JSON.stringify(value)}`).join(' · ') || 'defaults'}</span></Row>
        <Row term="Pre-run eligibility">{run.eligibility.kind === 'eligible' ? 'all checks completed' : `${run.eligibility.unresolved.length} requirements to review`}</Row>
        <Row term="Rows">{formatCount(estimate.sample.observations).text}</Row>
        <Row term="Run">{shortId(run.id)} · {formatTimestamp(run.createdAt)}</Row>
      </Section>

      {study !== null && (
        <Section title="Study">
          <Row term="Target quantity">{describeEstimand(study)}</Row>
          <Row term="Treatment">{study.treatment.name}</Row>
          <Row term="Outcome">{study.outcome.name}</Row>
          <Row term="Design category">{describeAssignmentKind(study.assignment.kind)} · {describeStudyDesignCategory(studyDesignCategory(study))}</Row>
          <Row term="Adjustment used by estimator">{appliedAdjustment(estimate.adjustment)}</Row>
          <Row term="Identification">{manifest.identification === null ? 'not recorded' : identificationMethod(manifest.identification)}</Row>
          {manifest.identification?.result.kind === 'identified' && (
            <>
              <Row term="Canonical adjustment set">{names(manifest.identification.result.canonicalAdjustmentSet)}</Row>
              <Row term="Minimal valid sets">{manifest.identification.result.minimalAdjustmentSets.sets.map((set, index) => <span key={index}>{index > 0 && <span className="text-faint"> or </span>}{names(set)}</span>)}{manifest.identification.result.minimalAdjustmentSets.kind === 'truncated' ? <span className="text-faint"> · result limit reached</span> : null}</Row>
              <Row term="Identified expression"><Formula tex={identifiedExpressionTex(study, manifest.identification.result.adjustment.variables)} plain={identifiedExpression(study, manifest.identification.result.adjustment.variables)} /></Row>
            </>
          )}
        </Section>
      )}

      {manifest.dag !== null && (
        <Section title="Graph">
          <Row term="Name">{manifest.dag.name}</Row>
          <Row term="Revision">{shortId(String(manifest.dag.revision))} · {describeDagValidation(manifest.dag.validation)}</Row>
          <Row term="Origin">{manifest.dag.origin.kind === 'user-authored' ? describeDagBasis(manifest.dag.origin.basis) : 'substantive review of discovery results'}</Row>
          <Row term="Arrows">{formatCount(manifest.dag.graph.edges.length).text} among {formatCount(manifest.dag.graph.nodes.length).text} nodes</Row>
        </Section>
      )}

      <Section title="Data">
        <Row term="Source">{manifest.source.name} · {formatCount(manifest.source.bytes).text} bytes · rows not included</Row>
        <Row term="Prepared dataset">{shortId(String(manifest.prepared.id))} ·{manifest.prepared.kind === 'prepared-time-series' ? `${manifest.prepared.sampling.frequency} series` : manifest.prepared.kind === 'prepared-panel' ? `panel · ${manifest.prepared.panel.units} units × ${manifest.prepared.panel.periods} periods` : 'independent rows'} · {formatCount(manifest.prepared.observations).text} rows</Row>
        <Row term="Missing data">{manifest.prepared.resolution.kind === 'none'
          ? 'none'
          : manifest.prepared.resolution.kind === 'window'
            ? `rows ${manifest.prepared.resolution.start + 1} to ${manifest.prepared.resolution.endExclusive}`
            : manifest.prepared.resolution.kind === 'lag-aware-exclusion'
              ? `${manifest.prepared.resolution.cells} cells excluded during lagged sample construction`
              : `${manifest.prepared.resolution.method}, ${manifest.prepared.resolution.cells} cells`}</Row>
        <Row term="Seasonal adjustment">{manifest.prepared.seasonalAdjustment.kind === 'none' ? 'none' : `seasonal-trend decomposition using loess (STL), period ${manifest.prepared.seasonalAdjustment.period}`}</Row>
        <Row term="Series transformations">{manifest.prepared.kind !== 'prepared-time-series' ? 'not applicable' : manifest.prepared.seriesTransforms.map((record, index) => <span key={String(record.column)}>{index > 0 && <span className="text-faint"> · </span>}{manifest.schema.find((column) => column.id === String(record.column))?.name ?? record.column}: {describeSeriesTransform(record.transform)}</span>)}</Row>
        {manifest.stationarity !== null && (
          <Row term="Stationarity">
            {manifest.stationarity.variables.map((variable, index) => {
              const assessment = describeStationarityAssessment(variable.assessment)
              return (
                <span key={String(variable.column)}>
                  {index > 0 && <span className="text-faint"> · </span>}
                  {manifest.schema.find((column) => column.id === String(variable.column))?.name ?? variable.column}
                  <span className={statusText[assessment.tone]}> {assessment.verdict}</span>
                </span>
              )
            })}
          </Row>
        )}
      </Section>

      <Section title="Sensitivity and counterfactuals">
        <Row term="Sensitivity">{manifest.sensitivity.length === 0 ? 'none recorded' : manifest.sensitivity.map((probe) => probe.kind === 'linear-refutation-run' ? `movement-only perturbations, no refuter p-values (placebo ${formatStatistic('raw', probe.evidence.placeboEffect).text}; subset ${formatStatistic('raw', probe.evidence.subsetEffect).text})` : probe.kind === 'dml-refutation-run' ? `DML mean-shift tests (placebo p ${formatP(probe.evidence.placebo.pValue, { withLabel: false }).text}; random covariate p ${formatP(probe.evidence.randomCommonCause.pValue, { withLabel: false }).text})` : `simulated-confounder movement grid ${probe.evidence.kappaT.length}×${probe.evidence.kappaY.length}, no p-value`).join(' · ')}</Row>
        <Row term="Counterfactuals">{manifest.counterfactuals.length === 0 ? 'none recorded' : manifest.counterfactuals.map((counterfactual) => `model-implied ${counterfactual.evidence.interventions[0]} → ${counterfactual.evidence.interventions[1]} contrast: ${formatStatistic('raw', counterfactual.evidence.averageEffect).text}`).join(' · ')}</Row>
      </Section>
      </div>
    </div>
  )
}

export function ResultsPanel({ source, profile, prepared, stationarity, documents, studies, identifications, estimationRuns, sensitivityRuns, counterfactualRuns }: {
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
}) {
  const [selected, setSelected] = useState<EstimationRunId | null>(estimationRuns.at(-1)?.id ?? null)
  const [compareWith, setCompareWith] = useState<EstimationRunId | null>(null)
  const stepLabel = prepared.kind === 'prepared-time-series' ? 'observation' : 'row'
  const inputs = useMemo(() => ({ source, profile, prepared, stationarity, documents, studies, identifications, sensitivityRuns, counterfactualRuns }), [counterfactualRuns, documents, identifications, prepared, profile, sensitivityRuns, source, stationarity, studies])
  const run = estimationRuns.find((candidate) => candidate.id === selected) ?? null
  const other = estimationRuns.find((candidate) => candidate.id === compareWith) ?? null
  const manifest = useMemo(() => (run === null ? null : buildResultManifest(inputs, run, new Date().toISOString())), [inputs, run])
  const otherManifest = useMemo(() => (other === null ? null : buildResultManifest(inputs, other, new Date().toISOString())), [inputs, other])
  const differences = manifest !== null && otherManifest !== null ? compareResults(manifest, otherManifest) : []
  const studyOf = (candidate: EstimationRunArtifact) => studies.find((study) => study.id === candidate.study)

  const stage = (
    <section aria-labelledby="results-title" className="@container/panel flex flex-col gap-5">
      <div>
        <span className={label('text-faint')}>09 · Results</span>
        <h2 id="results-title" className="mb-2 mt-2 text-heading text-ink">Review the complete analysis</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">A causal result must be interpreted with its causal question, identification strategy, estimate, uncertainty, diagnostics, and assumptions. In this chapter, examine those parts together, compare runs when the data or analysis choices differ, and export the analysis record.</p>
      </div>
      <div className="grid grid-cols-1 gap-3 @lg/panel:grid-cols-2">
        <label className="block">
          <span className="block text-body font-medium text-ink">Estimate</span>
          <Select className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink" value={selected ?? ''} onChange={(event) => setSelected(event.target.value === '' ? null : (event.target.value as EstimationRunId))}>
            <option value="" disabled>Choose a run</option>
            {[...estimationRuns].reverse().map((candidate) => <option key={candidate.id} value={candidate.id}>{(studyOf(candidate) === undefined ? candidate.method : estimandSentence(studyOf(candidate) as StudySpecification))} · {describeEstimator(candidate.configuration.kind)} · {formatTime(candidate.createdAt)}</option>)}
          </Select>
        </label>
        <label className="block">
          <span className="block text-body font-medium text-ink">Compare with</span>
          <Select className="mt-1 w-full rounded-md border border-control bg-well px-2 py-1.5 text-body text-ink" value={compareWith ?? ''} onChange={(event) => setCompareWith(event.target.value === '' ? null : (event.target.value as EstimationRunId))}>
            <option value="">None</option>
            {[...estimationRuns].reverse().filter((candidate) => candidate.id !== selected).map((candidate) => <option key={candidate.id} value={candidate.id}>{(studyOf(candidate) === undefined ? candidate.method : estimandSentence(studyOf(candidate) as StudySpecification))} · {describeEstimator(candidate.configuration.kind)} · {formatTime(candidate.createdAt)}</option>)}
          </Select>
        </label>
      </div>
      {otherManifest !== null && manifest !== null && (
        <section className={panel('p-4')} aria-label="Differences">
          <h3 className="mb-2 mt-0 text-faint text-label font-medium">Differences · {differences.length}</h3>
          {differences.length === 0 ? <p className="m-0 text-body text-muted">The two runs share every recorded field.</p> : (
            <div className="figure-strip overflow-x-auto">
              <table className={table}>
                <thead><tr><th className={th()}>Field</th><th className={th()}>Selected</th><th className={th()}>Compared</th></tr></thead>
                <tbody>
                  {differences.map((difference) => (
                    <tr key={difference.field} className={tr()}>
                      <td className={td('text-ink')}>{difference.field}</td>
                      <td className={td('whitespace-normal text-muted')}>{difference.left}</td>
                      <td className={td('whitespace-normal text-muted')}>{difference.right}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>
      )}
      {manifest !== null && <Manifest manifest={manifest} stepLabel={stepLabel} />}
    </section>
  )

  const ledger = (
    <div className="figure-strip overflow-x-auto">
      <table className={table} aria-label="Estimates">
        <thead>
          <tr>
            <th className={th()}>Target</th>
            <th className={th()}>Estimator</th>
            <th className={th('text-right')}>Estimate</th>
            <th className={th()}>Graph</th>
            <th className={th()}>Created</th>
          </tr>
        </thead>
        <tbody>
          {[...estimationRuns].reverse().map((candidate) => {
            const study = studyOf(candidate)
            const effect = candidate.estimate.effect
            return (
              <tr key={candidate.id} className={tr(candidate.id === selected ? 'selected' : 'action')} onClick={() => setSelected(candidate.id)}>
                <td className={td('text-ink')}>{study === undefined ? candidate.method : estimandSentence(study)}</td>
                <td className={td('text-muted')}>{describeEstimator(candidate.configuration.kind)}</td>
                <td className={td(num('whitespace-nowrap text-right text-ink'))}>{effect.kind === 'path' ? formatStatistic('raw', effect.aggregate.average).text : formatStatistic('raw', effect.value).text}</td>
                <td className={td('text-muted')}>{study?.dagName ?? '—'}</td>
                <td className={td(num('whitespace-nowrap text-muted'))}>{formatTime(candidate.createdAt)}</td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )

  return (
    <WorkbenchLayout
      id="results"
      stage={stage}
      bottom={{ title: `Estimates · ${estimationRuns.length}`, body: ledger, defaultSize: 180 }}
    />
  )
}
