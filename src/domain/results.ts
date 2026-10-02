import type { CounterfactualRunArtifact } from './counterfactual'
import type { DagDocument } from './dag'
import type { DatasetProfile } from './dataset'
import { adjustmentLabels, type EstimationRunArtifact } from './estimation'
import { describeSeriesTransform, type PreparedDatasetArtifact, type StationarityEvidenceArtifact } from './preprocessing'
import {sensitivityEstimationRun, type SensitivityRunArtifact} from './sensitivity'
import type { IdentificationArtifact, StudySpecification } from './study'
import type { SelectedSource } from './workflow'

/**
 * A result manifest is one estimate with everything it rests on, assembled from recorded artifacts and
 * nothing else: the source, the prepared version and its recipe, the graph revision, the study, the
 * identification, the estimator run with its eligibility, and the probes and counterfactuals that used
 * it. It is what the Results chapter shows and what an export writes (DESIGN.md §11 and §17).
 */

export interface ResultManifest {
  readonly kind: 'result-manifest'
  readonly version: 1
  readonly application: { readonly name: 'hirmos'; readonly build: string }
  readonly exportedAt: string
  readonly source: {
    readonly name: string
    readonly bytes: number
    readonly mediaType: string
    readonly format: SelectedSource['format']
    readonly lastModified: number
    readonly profile: DatasetProfile['id']
    /** Raw rows are never included; the manifest names the file and its fingerprint only. */
    readonly availability: 'not-included'
  }
  readonly schema: readonly { readonly id: string; readonly name: string; readonly type: string; readonly missing: number }[]
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly dag: {
    readonly id: DagDocument['id']
    readonly name: DagDocument['name']
    readonly revision: DagDocument['current']['id']
    readonly origin: DagDocument['origin']
    readonly graph: DagDocument['current']['graph']
    readonly validation: DagDocument['current']['validation']
  } | null
  readonly study: StudySpecification | null
  readonly identification: IdentificationArtifact | null
  readonly estimation: EstimationRunArtifact
  readonly sensitivity: readonly SensitivityRunArtifact[]
  readonly counterfactuals: readonly CounterfactualRunArtifact[]
  /** Unresolved requirements carried by the estimator run, verbatim. */
  readonly warnings: readonly string[]
}

export interface ResultInputs {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly documents: readonly DagDocument[]
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
}

declare const __APP_VERSION__: string

export function buildResultManifest(inputs: ResultInputs, run: EstimationRunArtifact, exportedAt: string): ResultManifest {
  const study = inputs.studies.find((candidate) => candidate.id === run.study) ?? null
  const identification = inputs.identifications.find((candidate) => candidate.id === run.identification) ?? null
  const document = study === null ? null : inputs.documents.find((candidate) => candidate.id === study.dagDocument) ?? null
  const warnings = run.eligibility.kind === 'caution' ? run.eligibility.unresolved.map((evaluation) => `${evaluation.caveat.requirement} ${evaluation.missingEvidence}`) : []
  return {
    kind: 'result-manifest',
    version: 1,
    application: { name: 'hirmos', build: typeof __APP_VERSION__ === 'string' ? __APP_VERSION__ : 'dev' },
    exportedAt,
    source: {
      name: inputs.source.name,
      bytes: inputs.source.bytes,
      mediaType: inputs.source.mediaType,
      format: inputs.source.format,
      lastModified: inputs.source.lastModified,
      profile: inputs.profile.id,
      availability: 'not-included',
    },
    schema: inputs.profile.columns.map((column) => ({ id: String(column.id), name: column.name, type: column.duckdbType, missing: column.nullCount })),
    prepared: inputs.prepared,
    stationarity: inputs.stationarity,
    dag: document === null ? null : {
      id: document.id,
      name: document.name,
      revision: document.current.id,
      origin: document.origin,
      graph: document.current.graph,
      validation: document.current.validation,
    },
    study,
    identification,
    estimation: run,
    sensitivity: inputs.sensitivityRuns.filter((probe) => sensitivityEstimationRun(probe) === run.id),
    counterfactuals: study === null ? [] : inputs.counterfactualRuns.filter((counterfactual) => counterfactual.study === study.id),
    warnings,
  }
}

/** Stable JSON: keys in declaration order, two-space indent, typed arrays as plain arrays. */
export const manifestJson = (manifest: ResultManifest): string =>
  JSON.stringify(manifest, (_key, value: unknown) => (ArrayBuffer.isView(value) ? Array.from(value as Float64Array) : value), 2)

export const manifestFileName = (manifest: ResultManifest): string =>
  `hirmos-result-${manifest.estimation.id.slice(0, 8)}-${manifest.exportedAt.slice(0, 10)}.json`

export interface ResultDifference {
  readonly field: string
  readonly left: string
  readonly right: string
}

const describeConfiguration = (run: EstimationRunArtifact): string =>
  Object.entries(run.configuration)
    .filter(([key]) => key !== 'kind')
    .map(([key, value]) => `${key} ${JSON.stringify(value)}`)
    .join(', ')

const describePreparedTransforms = (prepared: PreparedDatasetArtifact): string => prepared.kind === 'prepared-time-series'
  ? prepared.seriesTransforms.map((record) => `${record.column}:${describeSeriesTransform(record.transform)}`).join('|')
  : 'not applicable'

/** The fields that differ between two runs, so a comparison highlights what changed and nothing else. */
export function compareResults(left: ResultManifest, right: ResultManifest): readonly ResultDifference[] {
  const differences: ResultDifference[] = []
  const add = (field: string, a: string, b: string) => { if (a !== b) differences.push({ field, left: a, right: b }) }
  add('Prepared version', String(left.prepared.id), String(right.prepared.id))
  add('Missingness resolution', left.prepared.resolution.kind, right.prepared.resolution.kind)
  add('Seasonal adjustment', left.prepared.seasonalAdjustment.kind, right.prepared.seasonalAdjustment.kind)
  add('Series transformations', describePreparedTransforms(left.prepared), describePreparedTransforms(right.prepared))
  add('Graph', left.dag?.name ?? 'none', right.dag?.name ?? 'none')
  add('Graph revision', String(left.dag?.revision ?? 'none'), String(right.dag?.revision ?? 'none'))
  add('Arrows', String(left.dag?.graph.edges.length ?? 0), String(right.dag?.graph.edges.length ?? 0))
  add('Treatment', left.study?.treatment.name ?? 'none', right.study?.treatment.name ?? 'none')
  add('Outcome', left.study?.outcome.name ?? 'none', right.study?.outcome.name ?? 'none')
  add('Target', left.study?.estimand.kind ?? 'none', right.study?.estimand.kind ?? 'none')
  add('Assignment', left.study?.assignment.kind ?? 'none', right.study?.assignment.kind ?? 'none')
  const adjustment = (run: EstimationRunArtifact): string => run.estimate.adjustment.kind === 'structural-parent-model'
    ? `Wright parent model: ${run.estimate.adjustment.coefficients} coefficients, ${run.estimate.adjustment.paths} paths`
    : adjustmentLabels(run.estimate.adjustment).join(', ') || 'none'
  add('Adjustment strategy', adjustment(left.estimation), adjustment(right.estimation))
  add('Estimator', left.estimation.configuration.kind, right.estimation.configuration.kind)
  add('Configuration', describeConfiguration(left.estimation), describeConfiguration(right.estimation))
  add('Eligibility', left.estimation.eligibility.kind, right.estimation.eligibility.kind)
  add('Sensitivity probes', String(left.sensitivity.length), String(right.sensitivity.length))
  add('Counterfactual runs', String(left.counterfactuals.length), String(right.counterfactuals.length))
  return differences
}
