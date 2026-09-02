import { z } from 'zod'
import type { CounterfactualRunArtifact } from './counterfactual'
import type { DagDocument } from './dag'
import type { DagCheckArtifact } from './dagValidation'
import { parseDatasetProfile, type DatasetProfile } from './dataset'
import type { DiscoveryRunArtifact } from './discovery'
import type { GrangerEvidenceArtifact } from './granger'
import type { CountSeriesModelArtifact } from './countSeries'
import type { InterventionQueryArtifact } from './intervention'
import { err, ok, type Result } from './dop'
import type { EstimationRunArtifact } from './estimation'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from './preprocessing'
import type { SensitivityRunArtifact } from './sensitivity'
import { EMPTY_STUDY_DRAFT, type IdentificationArtifact, type StudyDesignDraft, type StudySpecification } from './study'
import type { Project, SelectedSource, Workflow } from './workflow'

/**
 * What a project keeps between sessions: the manifest, the recorded artifacts and a description of the
 * source file, never its rows (DESIGN.md §5.6). A reopened project asks for the file again and checks
 * its fingerprint against the stored profile before any artifact is trusted with it.
 */

export interface SourceDescriptor {
  readonly name: string
  readonly bytes: number
  readonly mediaType: string
  readonly lastModified: number
  readonly format: SelectedSource['format']
}

export interface PersistedProject {
  readonly kind: 'hirmos-project'
  readonly version: 1
  readonly savedAt: string
  readonly project: Project
  readonly source: SourceDescriptor | null
  readonly profile: DatasetProfile | null
  readonly prepared: PreparedDatasetArtifact | null
  readonly stationarity: StationarityEvidenceArtifact | null
  readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
  readonly countSeriesModels: readonly CountSeriesModelArtifact[]
  readonly discoveryRuns: readonly DiscoveryRunArtifact[]
  readonly dagDocuments: readonly DagDocument[]
  readonly dagChecks: readonly DagCheckArtifact[]
  readonly interventionQueries: readonly InterventionQueryArtifact[]
  readonly studyDraft: StudyDesignDraft
  readonly studies: readonly StudySpecification[]
  readonly identifications: readonly IdentificationArtifact[]
  readonly estimationRuns: readonly EstimationRunArtifact[]
  readonly sensitivityRuns: readonly SensitivityRunArtifact[]
  readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
}

/** The lines a project list shows without opening the record. */
export interface SavedProjectHeader {
  readonly id: Project['id']
  readonly name: string
  readonly savedAt: string
  readonly sourceName: string | null
  /** The fingerprint of a source cached in this browser, so reopening can skip the file prompt and deleting can clear the cache. */
  readonly cachedSource: DatasetProfile['source']['fingerprint'] | null
  readonly estimationRuns: number
}

export const headerOf = (snapshot: PersistedProject): SavedProjectHeader => ({
  id: snapshot.project.id,
  name: snapshot.project.name,
  savedAt: snapshot.savedAt,
  sourceName: snapshot.source?.name ?? null,
  cachedSource: snapshot.profile !== null && snapshot.profile.source.persistence.kind === 'cached-locally' ? snapshot.profile.source.fingerprint : null,
  estimationRuns: snapshot.estimationRuns.length,
})

const describeSource = (source: SelectedSource): SourceDescriptor => ({
  name: source.name,
  bytes: source.bytes,
  mediaType: source.mediaType,
  lastModified: source.lastModified,
  format: source.format,
})

/** The record to write for the workflow as it stands, or null when nothing durable exists yet or a restore is pending. */
export function snapshotWorkflow(workflow: Workflow, savedAt: string): PersistedProject | null {
  switch (workflow.kind) {
    case 'awaiting-project': return null
    case 'awaiting-data':
      if (workflow.restore !== null) return null
      return {
        kind: 'hirmos-project', version: 1, savedAt, project: workflow.project, source: null, profile: null, prepared: null, stationarity: null,
        grangerEvidence: [], countSeriesModels: [], discoveryRuns: [], dagDocuments: [], dagChecks: [], interventionQueries: [], studyDraft: EMPTY_STUDY_DRAFT, studies: [], identifications: [], estimationRuns: [], sensitivityRuns: [], counterfactualRuns: [],
      }
    case 'source-selected':
    case 'profiling':
    case 'import-failed':
      return null
    case 'profiled':
      return {
        kind: 'hirmos-project',
        version: 1,
        savedAt,
        project: workflow.project,
        source: describeSource(workflow.source),
        profile: workflow.profile,
        prepared: workflow.prepared,
        stationarity: workflow.stationarity,
        grangerEvidence: workflow.grangerEvidence,
        countSeriesModels: workflow.countSeriesModels,
        discoveryRuns: workflow.discoveryRuns,
        dagDocuments: workflow.dagDocuments,
        dagChecks: workflow.dagChecks,
        interventionQueries: workflow.interventionQueries,
        studyDraft: workflow.studyDraft,
        studies: workflow.studies,
        identifications: workflow.identifications,
        estimationRuns: workflow.estimationRuns,
        sensitivityRuns: workflow.sensitivityRuns,
        counterfactualRuns: workflow.counterfactualRuns,
      }
    default: return null
  }
}

const F64 = '$f64'

/** JSON with typed arrays tagged, so a record round-trips through a string store without losing its numeric columns. */
export const serialiseSnapshot = (snapshot: PersistedProject): string =>
  JSON.stringify(snapshot, (_key, value: unknown) => (value instanceof Float64Array ? { [F64]: Array.from(value) } : value))

const revive = (_key: string, value: unknown): unknown => {
  if (typeof value === 'object' && value !== null && F64 in value) {
    const numbers = (value as Record<string, unknown>)[F64]
    return Array.isArray(numbers) ? Float64Array.from(numbers as number[]) : value
  }
  return value
}

export type SnapshotProblem =
  | { readonly kind: 'not-json'; readonly detail: string }
  | { readonly kind: 'invalid-snapshot'; readonly detail: string }
  | { readonly kind: 'unsupported-version'; readonly version: number }

const artifact = z.object({ id: z.string().min(1) }).passthrough()
const seriesTransformSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('levels') }).strict(),
  z.object({ kind: z.literal('difference'), order: z.literal(1) }).strict(),
  z.object({ kind: z.literal('linear-detrend') }).strict(),
])

const envelopeSchema = z.object({
  kind: z.literal('hirmos-project'),
  version: z.number().int(),
  savedAt: z.string().min(1),
  project: z.object({ id: z.string().min(1), name: z.string().min(1), createdAt: z.string().min(1) }),
  source: z.object({ name: z.string(), bytes: z.number().int().nonnegative(), mediaType: z.string(), lastModified: z.number(), format: z.enum(['csv', 'tsv', 'parquet']) }).nullable(),
  profile: z.unknown().nullable(),
  prepared: artifact.nullable(),
  stationarity: artifact.nullable(),
  grangerEvidence: z.array(artifact).default([]),
  countSeriesModels: z.array(artifact).default([]),
  discoveryRuns: z.array(artifact),
  dagDocuments: z.array(artifact),
  dagChecks: z.array(artifact).default([]),
  interventionQueries: z.array(artifact).default([]),
  studyDraft: z.object({}).passthrough(),
  studies: z.array(artifact),
  identifications: z.array(artifact),
  estimationRuns: z.array(artifact),
  sensitivityRuns: z.array(artifact),
  counterfactualRuns: z.array(artifact),
})
type ParsedEnvelope = z.output<typeof envelopeSchema>

/**
 * Reads a stored record back. The envelope, the project and the dataset profile are parsed in full;
 * artifact bodies are checked for their identity and otherwise trusted as the shapes this build wrote,
 * since the record only ever comes from this application's own store.
 */
export function parseSnapshot(raw: string): Result<PersistedProject, SnapshotProblem> {
  let value: unknown
  try { value = JSON.parse(raw, revive) } catch (cause) { return err({ kind: 'not-json', detail: cause instanceof Error ? cause.message : String(cause) }) }
  return parseSnapshotValue(value)
}

/** Tagged-JSON helpers shared with the export bundle, so a bundle and a stored record read the same way. */
export const taggedJsonReplacer = (_key: string, value: unknown): unknown => (value instanceof Float64Array ? { [F64]: Array.from(value) } : value)
export const taggedJsonReviver = revive

/** Add preparation fields introduced while the version-1 envelope remained stable. */
const upgradePreparedTransformRecord = (value: ParsedEnvelope['prepared']): Result<ParsedEnvelope['prepared'], SnapshotProblem> => {
  if (value === null || Reflect.get(value, 'kind') !== 'prepared-time-series') return ok(value)
  const columns = Reflect.get(value, 'columns')
  if (!Array.isArray(columns) || !columns.every((column) => typeof column === 'string')) {
    return err({ kind: 'invalid-snapshot', detail: 'prepared time series: columns are missing' })
  }
  return ok({
    ...value,
    seriesTransforms: Array.isArray(Reflect.get(value, 'seriesTransforms'))
      ? Reflect.get(value, 'seriesTransforms')
      : columns.map((column) => ({ column, transform: { kind: 'levels' } })),
    resampling: Reflect.get(value, 'resampling') ?? { kind: 'none' },
  })
}

/** Rename the version-1 stationarity display field; no numerical evidence is recomputed. */
const upgradeStationarityTransformRecord = (value: ParsedEnvelope['stationarity']): Result<ParsedEnvelope['stationarity'], SnapshotProblem> => {
  if (value === null || Reflect.get(value, 'kind') !== 'stationarity-evidence' || Reflect.get(value, 'diagnosticTransform') !== undefined) return ok(value)
  const legacy = seriesTransformSchema.safeParse(Reflect.get(value, 'transform'))
  if (!legacy.success) return err({ kind: 'invalid-snapshot', detail: 'stationarity evidence: diagnostic transform is missing' })
  const { transform: _legacyTransform, ...rest } = value
  return ok({ ...rest, diagnosticTransform: legacy.data })
}

/**
 * Upgrade estimation records written before applied adjustment and synthetic inference became
 * tagged evidence. The numerical result is retained; inference that was never computed is recorded
 * explicitly rather than fabricated during project loading.
 */
const upgradeEstimationRunRecord = (value: Record<string, unknown>): Record<string, unknown> => {
  const estimate = Reflect.get(value, 'estimate')
  let upgradedEstimate = estimate
  if (typeof estimate === 'object' && estimate !== null && Reflect.get(estimate, 'adjustment') === undefined) {
    const legacy = Reflect.get(estimate, 'adjustmentSet')
    if (Array.isArray(legacy)) {
      const { adjustmentSet: _legacyAdjustment, ...rest } = estimate as Record<string, unknown>
      upgradedEstimate = {
        ...rest,
        adjustment: legacy.length === 0
          ? { kind: 'none' }
          : { kind: 'contemporaneous', variables: legacy },
      }
    }
  }

  const configuration = Reflect.get(value, 'configuration')
  const evidence = Reflect.get(value, 'evidence')
  if (Reflect.get(value, 'kind') === 'synthetic-control-run'
    && typeof configuration === 'object' && configuration !== null
    && typeof evidence === 'object' && evidence !== null) {
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: {
        crossFitFolds: 3,
        alpha: 0.05,
        ...configuration,
      },
      evidence: {
        crossFit: { kind: 'unavailable', reason: 'This saved run predates cross-fitted inference; run the estimator again to compute it.' },
        donorPlacebo: { kind: 'unavailable', reason: 'This saved run predates donor-placebo inference; run the estimator again to compute it.' },
        conformalBand: { kind: 'unavailable', reason: 'This saved run predates prediction bands; run the estimator again to compute them.' },
        gaussianBand: { kind: 'unavailable', reason: 'This saved run predates prediction bands; run the estimator again to compute them.' },
        ...evidence,
      },
    }
  }
  if (Reflect.get(value, 'kind') === 'panel-intervention-run'
    && typeof configuration === 'object' && configuration !== null
    && typeof evidence === 'object' && evidence !== null) {
    return {
      ...value,
      estimate: upgradedEstimate,
      configuration: {
        placeboReplications: 100,
        seed: 0,
        ...configuration,
      },
      evidence: {
        syntheticControlPlacebo: { kind: 'unavailable', reason: 'This saved run predates panel placebo inference; run the estimator again to compute it.' },
        syntheticDidPlacebo: { kind: 'unavailable', reason: 'This saved run predates panel placebo inference; run the estimator again to compute it.' },
        syntheticControlInTime: { kind: 'unavailable', reason: 'This saved run predates the in-time placebo; run the estimator again to compute it.' },
        syntheticDidInTime: { kind: 'unavailable', reason: 'This saved run predates the in-time placebo; run the estimator again to compute it.' },
        ...evidence,
      },
    }
  }
  return upgradedEstimate === estimate ? value : { ...value, estimate: upgradedEstimate }
}

export function parseSnapshotValue(value: unknown): Result<PersistedProject, SnapshotProblem> {
  const parsed = envelopeSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-snapshot', detail: z.prettifyError(parsed.error) })
  if (parsed.data.version !== 1) return err({ kind: 'unsupported-version', version: parsed.data.version })
  const prepared = upgradePreparedTransformRecord(parsed.data.prepared)
  if (!prepared.ok) return prepared
  const stationarity = upgradeStationarityTransformRecord(parsed.data.stationarity)
  if (!stationarity.ok) return stationarity
  let profile: DatasetProfile | null = null
  if (parsed.data.profile !== null) {
    const profileParsed = parseDatasetProfile(parsed.data.profile)
    if (!profileParsed.ok) return err({ kind: 'invalid-snapshot', detail: `profile: ${profileParsed.error.kind}` })
    profile = profileParsed.value
  }
  // Granger moved from the Discovery Lab to the data diagnostics; a run recorded there by an earlier build has no reader now.
  const discoveryRuns = parsed.data.discoveryRuns.filter((run) => Reflect.get(run, 'kind') !== 'granger-ssr-f-run')
  const storedDraft = parsed.data.studyDraft as Partial<StudyDesignDraft>
  const studyDraft: StudyDesignDraft = { ...EMPTY_STUDY_DRAFT, ...storedDraft }
  const estimationRuns = parsed.data.estimationRuns.map((run) => upgradeEstimationRunRecord(run))
  return ok({
    ...(parsed.data as unknown as PersistedProject),
    version: 1,
    profile,
    prepared: prepared.value as PreparedDatasetArtifact | null,
    stationarity: stationarity.value as StationarityEvidenceArtifact | null,
    studyDraft,
    discoveryRuns: discoveryRuns as unknown as PersistedProject['discoveryRuns'],
    estimationRuns: estimationRuns as unknown as PersistedProject['estimationRuns'],
  })
}

export function describeSnapshotProblem(problem: SnapshotProblem): string {
  switch (problem.kind) {
    case 'not-json': return `The saved project could not be read: ${problem.detail}`
    case 'invalid-snapshot': return `The saved project is damaged: ${problem.detail}`
    case 'unsupported-version': return `The saved project was written by a newer version (${problem.version}).`
    default: { const exhaustive: never = problem; return exhaustive }
  }
}
