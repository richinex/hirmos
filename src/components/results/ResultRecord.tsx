import { EstimateHeadline } from '@/components/results/EstimateHeadline'
import { Metadata } from '@/components/ui/Metadata'
import { ResultInterpretation } from '@/components/ui/ResultInterpretation'
import { Formula } from '@/components/ui/Formula'
import { button, chip, label, literal, panel, statusText } from '@/components/ui/recipes'
import { RecordList, RecordRow } from '@/components/ui/RecordList'
import { MetricGrid, MetricTile } from '@/components/ui/figures'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { describeDagBasis, describeDagValidation } from '@/domain/dag'
import {
  adjustmentLabels,
  configurationSettings,
  describeEstimator,
  describeRun,
  type AppliedAdjustment,
  type EstimationRunArtifact,
} from '@/domain/estimation'
import { describeSeriesTransform } from '@/domain/preprocessing'
import { manifestFileName, manifestJson, type ResultManifest } from '@/domain/results'
import {
  describeAssignmentKind,
  describeEstimand,
  describeStudyDesignCategory,
  estimandSentence,
  identifiedExpression,
  identifiedExpressionTex,
  studyDesignCategory,
  type IdentificationArtifact,
  type StudySpecification,
  type StudyVariable,
} from '@/domain/study'
import { assertNever } from '@/domain/dop'
import { describeStationarityAssessment } from '@/domain/stationarityAssessment'
import { formatCount, formatP, formatStatistic, formatWords } from '@/lib/format/number'
import { formatTimestamp } from '@/lib/format/date'
import {
  interpretEstimationResult,
  resultHeadline,
  resultSampleLine,
  resultScaleLine,
} from '@/domain/resultInterpretation'

const Row = RecordRow

/** A set of variables the reader counts; empty sets read as a word. */
const names = (variables: readonly StudyVariable[]): React.ReactNode =>
  variables.length === 0
    ? 'none'
    : variables.map((variable) => (
        <span key={variable.node} className={chip('mr-1')}>
          {variable.name}
        </span>
      ))

const appliedAdjustment = (adjustment: AppliedAdjustment): React.ReactNode => {
  switch (adjustment.kind) {
    case 'none':
      return 'none'
    case 'contemporaneous':
    case 'time-indexed':
      return adjustmentLabels(adjustment).map((text) => (
        <span key={text} className={chip('mr-1')}>
          {text}
        </span>
      ))
    case 'structural-parent-model':
      return `${adjustment.coefficients} parent coefficients across ${adjustment.paths} directed paths`
    default:
      return assertNever(adjustment)
  }
}

const identificationMethod = (identification: IdentificationArtifact): string => {
  switch (identification.result.kind) {
    case 'cutoff-design':
      return 'sharp RD continuity assumptions'
    case 'identified':
      return 'back-door adjustment'
    case 'graphically-identified':
      return 'general ID expression'
    case 'counterfactually-identified':
      return 'IDC* counterfactual expressions'
    case 'instrument-identified':
      return 'instrumental variables'
    case 'backdoor-not-identified':
      return 'not identified from the observational distribution'
    default:
      return assertNever(identification.result)
  }
}

/** An id the reader may need to match: mono, quiet, and short, with the whole value on hover. */
const shortId = (id: string) => (
  <span className={literal('text-muted')} title={id}>
    {id.slice(0, 8)}
  </span>
)

function Section({
  title,
  children,
}: {
  readonly title: string
  readonly children: React.ReactNode
}) {
  return (
    <section aria-label={title}>
      <h3 className="mb-2 mt-0 text-body font-medium text-ink">{title}</h3>
      <RecordList className="text-body">{children}</RecordList>
    </section>
  )
}

export interface BiteCheck {
  readonly study: StudySpecification
  readonly run: EstimationRunArtifact | null
}

/** The latest estimate for each study that checks this study's bite, beside its own estimate. */
function BiteChecks({
  study,
  checks,
  stepLabel,
}: {
  readonly stepLabel: string
  readonly study: StudySpecification
  readonly checks: readonly BiteCheck[]
}) {
  const title = `Bite of ${study.treatment.name}`
  return (
    <section className={panel('p-(--panel-space)')} aria-label={title}>
      <h3 className="m-0 flex items-center gap-1.5 text-body font-medium text-ink">
        {title}
        <ParameterHelp
          label={title}
          help={`These estimates assess measured intermediate outcomes. They provide evidence about proposed mechanisms, but do not establish or rule out an effect on ${study.outcome.name}. Each card shows the latest saved estimate for its study.`}
        />
      </h3>
      <MetricGrid className="mt-3">
        {checks.map(({ study: check, run }) =>
          run === null ? (
            <MetricTile
              key={check.id}
              label={`Effect on ${check.outcome.name}`}
              value={formatWords('Not estimated')}
              context="Run an estimator for this study in Estimation."
            />
          ) : (
            <div key={check.id}>
              <h4 className="m-0 mb-2 text-body font-medium">Effect on {check.outcome.name}</h4>
              <EstimateHeadline
                accent={false}
                testId={`bite-estimate-${check.id}`}
                estimate={run.estimate}
                sentence={resultHeadline(run, check)}
                scaleLine={resultScaleLine(run, check, stepLabel)}
                sampleLine={resultSampleLine(run)}
                stepLabel={stepLabel}
              />
            </div>
          ),
        )}
      </MetricGrid>
    </section>
  )
}

export interface ManifestParts {
  readonly manifest: ResultManifest
  readonly stepLabel: string
  readonly bite: readonly BiteCheck[]
  readonly checked: StudySpecification | null
}

const downloadManifest = (manifest: ResultManifest) => {
  const blob = new Blob([manifestJson(manifest)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = manifestFileName(manifest)
  anchor.click()
  URL.revokeObjectURL(url)
}

/** The estimate with its uncertainty and reading, and the manifest export. */
export function ResultSummary({
  manifest,
  stepLabel,
}: Pick<ManifestParts, 'manifest' | 'stepLabel'>) {
  const { estimation: run, study } = manifest
  const estimate = run.estimate
  const outcomeName = study?.outcome.name ?? 'outcome'
  const treatmentName = study?.treatment.name ?? 'treatment'
  return (
    <article className="rounded-xl bg-panel lift p-4" aria-label="Result">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <div>
          <span className={label('text-signal-text')}>Estimate and uncertainty</span>
          <p className="mb-0 mt-1 text-body text-muted">
            <Metadata>
              <span>{describeRun(run.configuration)}</span>
              <span>
                {run.eligibility.kind === 'eligible'
                  ? 'All pre-run checks completed'
                  : `${run.eligibility.unresolved.length} pre-run requirements to review`}
              </span>
            </Metadata>
          </p>
        </div>
        <button
          type="button"
          className={button('outline')}
          onClick={() => downloadManifest(manifest)}
        >
          Export manifest
        </button>
      </div>
      <div className="mt-3">
        <EstimateHeadline
          estimate={estimate}
          sentence={study === null ? run.method : resultHeadline(run, study)}
          scaleLine={
            study === null
              ? estimate.effect.kind === 'expectedCountRatio'
                ? `Ratio of expected ${outcomeName} counts per unit increase in ${treatmentName}.`
                : `Difference in ${outcomeName} per unit increase in ${treatmentName}.`
              : resultScaleLine(run, study, stepLabel)
          }
          sampleLine={resultSampleLine(run)}
          stepLabel={stepLabel}
          accent
          testId="result-figure"
        />
      </div>
      {study !== null && (
        <ResultInterpretation
          interpretation={interpretEstimationResult(run, study, stepLabel)}
          className="mt-3"
        />
      )}
    </article>
  )
}

/** Requirements still to review, bite checks, and the analysis record behind an estimate. */
export function ResultRecord({ manifest, stepLabel, bite, checked }: ManifestParts) {
  const { estimation: run, study } = manifest
  const estimate = run.estimate
  return (
    <div className="grid gap-4">
      {study !== null && bite.length > 0 && (
        <BiteChecks study={study} checks={bite} stepLabel={stepLabel} />
      )}

      {manifest.warnings.length > 0 && (
        <section className={panel('p-(--panel-space)')} aria-label="Unresolved requirements">
          <h3 className="mb-2 mt-0 text-warn text-label font-medium">Requirements to review</h3>
          <ul className="review-list m-0 space-y-2 p-0 text-body text-muted">
            {manifest.warnings.map((warning) => (
              <li key={warning}>{warning}</li>
            ))}
          </ul>
        </section>
      )}

      <div className={panel('space-y-6 p-4')} aria-label="Analysis record">
        <Section title="Estimator">
          <Row term="Method">{describeEstimator(run.configuration.kind)}</Row>
          {configurationSettings(run.configuration).map((setting) => (
            <Row key={setting.key} term={setting.label}>
              {setting.value}
            </Row>
          ))}
          <Row term="Pre-run eligibility">
            {run.eligibility.kind === 'eligible'
              ? 'all checks completed'
              : `${run.eligibility.unresolved.length} requirements to review`}
          </Row>
          <Row term="Rows">{formatCount(estimate.sample.observations).text}</Row>
          <Row term="Run">
            <Metadata>
              {shortId(run.id)}
              <time>{formatTimestamp(run.createdAt)}</time>
            </Metadata>
          </Row>
        </Section>

        {study !== null && (
          <Section title="Study">
            <Row term="Target quantity">{describeEstimand(study)}</Row>
            <Row term="Treatment">{study.treatment.name}</Row>
            <Row term="Outcome">{study.outcome.name}</Row>
            {study.biteOf !== undefined && (
              <Row term="Bite check for">
                {checked === null ? 'a deleted study' : estimandSentence(checked)}
              </Row>
            )}
            <Row term="Assignment">{describeAssignmentKind(study.assignment.kind)}</Row>
            <Row term="Design category">
              {describeStudyDesignCategory(studyDesignCategory(study))}
            </Row>
            <Row term="Adjustment used by estimator">{appliedAdjustment(estimate.adjustment)}</Row>
            <Row term="Identification">
              {manifest.identification === null
                ? 'not recorded'
                : identificationMethod(manifest.identification)}
            </Row>
            {manifest.identification?.result.kind === 'identified' && (
              <>
                <Row term="Canonical adjustment set">
                  {names(manifest.identification.result.canonicalAdjustmentSet)}
                </Row>
                <Row term="Minimal valid sets">
                  {manifest.identification.result.minimalAdjustmentSets.sets.map((set, index) => (
                    <span key={index}>
                      {index > 0 && <span className="text-faint"> or </span>}
                      {names(set)}
                    </span>
                  ))}
                  {manifest.identification.result.minimalAdjustmentSets.kind === 'truncated' ? (
                    <span className="text-faint">
                      <Metadata className="ml-3">
                        <span>result limit reached</span>
                      </Metadata>
                    </span>
                  ) : null}
                </Row>
                <Row term="Identified expression">
                  <Formula
                    tex={identifiedExpressionTex(
                      study,
                      manifest.identification.result.adjustment.variables,
                    )}
                    plain={identifiedExpression(
                      study,
                      manifest.identification.result.adjustment.variables,
                    )}
                  />
                </Row>
              </>
            )}
          </Section>
        )}

        {manifest.dag !== null && (
          <Section title="Graph">
            <Row term="Name">{manifest.dag.name}</Row>
            <Row term="Revision">
              <Metadata>
                {shortId(String(manifest.dag.revision))}
                <span>{describeDagValidation(manifest.dag.validation)}</span>
              </Metadata>
            </Row>
            <Row term="Origin">
              {manifest.dag.origin.kind === 'user-authored'
                ? describeDagBasis(manifest.dag.origin.basis)
                : 'substantive review of discovery results'}
            </Row>
            <Row term="Arrows">
              {formatCount(manifest.dag.graph.edges.length).text} among{' '}
              {formatCount(manifest.dag.graph.nodes.length).text} nodes
            </Row>
          </Section>
        )}

        <Section title="Data">
          <Row term="Source">
            <Metadata>
              <span>{manifest.source.name}</span>
              <span>{formatCount(manifest.source.bytes).text} bytes</span>
              <span>Rows not included</span>
            </Metadata>
          </Row>
          <Row term="Prepared dataset">
            <Metadata>
              {shortId(String(manifest.prepared.id))}
              <span>
                {manifest.prepared.kind === 'prepared-time-series'
                  ? `${manifest.prepared.sampling.frequency} series`
                  : manifest.prepared.kind === 'prepared-panel'
                    ? `Panel of ${manifest.prepared.panel.units} units × ${manifest.prepared.panel.periods} periods`
                    : 'Independent rows'}
              </span>
              <span>{formatCount(manifest.prepared.observations).text} rows</span>
            </Metadata>
          </Row>
          <Row term="Missing data">
            {manifest.prepared.resolution.kind === 'none'
              ? 'none'
              : manifest.prepared.resolution.kind === 'window'
                ? `rows ${manifest.prepared.resolution.start + 1} to ${manifest.prepared.resolution.endExclusive}`
                : manifest.prepared.resolution.kind === 'lag-aware-exclusion'
                  ? `${manifest.prepared.resolution.cells} cells excluded during lagged sample construction`
                  : `${manifest.prepared.resolution.method}, ${manifest.prepared.resolution.cells} cells`}
          </Row>
          <Row term="Seasonal adjustment">
            {manifest.prepared.seasonalAdjustment.kind === 'none'
              ? 'none'
              : `seasonal-trend decomposition using loess (STL), period ${manifest.prepared.seasonalAdjustment.period}`}
          </Row>
          <Row term="Series transformations">
            {manifest.prepared.kind !== 'prepared-time-series'
              ? 'not applicable'
              : manifest.prepared.seriesTransforms.map((record) => (
                  <span className="block" key={String(record.column)}>
                    {manifest.schema.find((column) => column.id === String(record.column))?.name ??
                      record.column}
                    : {describeSeriesTransform(record.transform)}
                  </span>
                ))}
          </Row>
          {manifest.stationarity !== null && (
            <Row term="Stationarity">
              {manifest.stationarity.variables.map((variable) => {
                const assessment = describeStationarityAssessment(variable.assessment)
                return (
                  <span className="block" key={String(variable.column)}>
                    {manifest.schema.find((column) => column.id === String(variable.column))
                      ?.name ?? variable.column}
                    <span className={statusText[assessment.tone]}> {assessment.verdict}</span>
                  </span>
                )
              })}
            </Row>
          )}
        </Section>

        <Section title="Sensitivity and counterfactuals">
          <Row term="Sensitivity">
            {manifest.sensitivity.length === 0
              ? 'none recorded'
              : manifest.sensitivity
                  .map((probe) =>
                    probe.kind === 'pretest-coverage-run'
                      ? 'pre-test coverage under a specified linear-trend scenario'
                      : probe.kind === 'did-sensitivity-run'
                        ? `DiD omitted-variable sensitivity, ${probe.evidence.scenarios.length} scenarios`
                        : probe.kind === 'honest-did-run'
                          ? `parallel-trends sensitivity, ${probe.evidence.request.configuration.bounds.length} restriction bounds`
                          : probe.kind === 'linear-refutation-run'
                            ? `movement-only perturbations, no refuter p-values (placebo ${formatStatistic('raw', probe.evidence.placeboEffect).text}; subset ${formatStatistic('raw', probe.evidence.subsetEffect).text})`
                            : probe.kind === 'dml-refutation-run'
                              ? `DML mean-shift tests (placebo p ${formatP(probe.evidence.placebo.pValue, { withLabel: false }).text}; random covariate p ${formatP(probe.evidence.randomCommonCause.pValue, { withLabel: false }).text})`
                              : `simulated-confounder movement grid ${probe.evidence.kappaT.length}×${probe.evidence.kappaY.length}, no p-value`,
                  )
                  .join('; ')}
          </Row>
          <Row term="Counterfactuals">
            {manifest.counterfactuals.length === 0
              ? 'none recorded'
              : manifest.counterfactuals
                  .map(
                    (counterfactual) =>
                      `model-implied ${counterfactual.evidence.interventions[0]} → ${counterfactual.evidence.interventions[1]} contrast: ${formatStatistic('raw', counterfactual.evidence.averageEffect).text}`,
                  )
                  .join('; ')}
          </Row>
        </Section>
      </div>
    </div>
  )
}

export function Manifest(props: ManifestParts) {
  return (
    <div className="grid gap-4">
      <ResultSummary manifest={props.manifest} stepLabel={props.stepLabel} />
      <ResultRecord {...props} />
    </div>
  )
}
