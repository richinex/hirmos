import { assertNever, brand, err, ok, type Brand, type Result } from './dop'
import type { DatasetProfile, DatasetProfileProblem, SourcePersistence } from './dataset'
import type { DagDocument } from './dag'
import type { DagCheckArtifact } from './dagValidation'
import type { DiscoveryRunArtifact } from './discovery'
import type { GrangerEvidenceArtifact } from './granger'
import type { InterventionQueryArtifact } from './intervention'
import type { EstimationRunArtifact, EstimationRunId } from './estimation'
import type { SensitivityRunArtifact, SensitivityRunId } from './sensitivity'
import type { CounterfactualRunArtifact, CounterfactualRunId } from './counterfactual'
import { EMPTY_STUDY_DRAFT, type IdentificationArtifact, type StudyDesignDraft, type StudySpecification } from './study'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from './preprocessing'
import type { PersistedProject } from './persistence'

export type ProjectId = Brand<string, 'ProjectId'>
export type ProjectName = Brand<string, 'ProjectName'>
export type ImportRequestId = Brand<string, 'ImportRequestId'>

export interface Project {
  readonly id: ProjectId
  readonly name: ProjectName
  readonly createdAt: string
}

export type ProjectNameProblem =
  | { readonly kind: 'empty' }
  | { readonly kind: 'too-long'; readonly maximum: number }

export type SourceSelectionProblem =
  | { readonly kind: 'empty-file' }
  | { readonly kind: 'unsupported-format'; readonly extension: string }
  /** The file chosen for a reopened project is not the one its artifacts were computed from. */
  | { readonly kind: 'source-mismatch'; readonly expected: string }
  | { readonly kind: 'fingerprint-failed'; readonly detail: string }

export interface SelectedSource {
  readonly file: File
  readonly name: string
  readonly bytes: number
  readonly mediaType: string
  readonly lastModified: number
  readonly format: 'csv' | 'tsv' | 'parquet'
}

export type Workflow =
  | {
      readonly kind: 'awaiting-project'
      readonly nameDraft: string
      readonly problem: ProjectNameProblem | null
    }
  | {
      readonly kind: 'awaiting-data'
      readonly project: Project
      readonly problem: SourceSelectionProblem | null
      /** A saved project waiting for its source file; the artifacts return once the file's fingerprint matches. */
      readonly restore: PersistedProject | null
    }
  | { readonly kind: 'source-selected'; readonly project: Project; readonly source: SelectedSource }
  | {
      readonly kind: 'profiling'
      readonly project: Project
      readonly source: SelectedSource
      readonly request: ImportRequestId
    }
  | {
      readonly kind: 'import-failed'
      readonly project: Project
      readonly source: SelectedSource
      readonly problem: DatasetProfileProblem
    }
  | {
      readonly kind: 'profiled'
      readonly project: Project
      readonly source: SelectedSource
      readonly profile: DatasetProfile
      readonly prepared: PreparedDatasetArtifact | null
      readonly stationarity: StationarityEvidenceArtifact | null
      readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
      readonly discoveryRuns: readonly DiscoveryRunArtifact[]
      readonly dagDocuments: readonly DagDocument[]
      readonly dagChecks: readonly DagCheckArtifact[]
      /** Do-queries asked of the graphs above; each records the revision it was asked of. */
      readonly interventionQueries: readonly InterventionQueryArtifact[]
      /** The treatment and outcome being bound, shared by the DAG Workspace and Study Design. */
      readonly studyDraft: StudyDesignDraft
      readonly studies: readonly StudySpecification[]
      readonly identifications: readonly IdentificationArtifact[]
      readonly estimationRuns: readonly EstimationRunArtifact[]
      readonly sensitivityRuns: readonly SensitivityRunArtifact[]
      readonly counterfactualRuns: readonly CounterfactualRunArtifact[]
    }

export type WorkflowEvent =
  | { readonly type: 'project-name-changed'; readonly value: string }
  | { readonly type: 'project-submitted' }
  | { readonly type: 'file-selected'; readonly file: File }
  | { readonly type: 'profile-requested'; readonly request: ImportRequestId }
  | { readonly type: 'profile-succeeded'; readonly request: ImportRequestId; readonly profile: DatasetProfile }
  | { readonly type: 'profile-failed'; readonly request: ImportRequestId; readonly problem: DatasetProfileProblem }
  | { readonly type: 'prepared-dataset-created'; readonly artifact: PreparedDatasetArtifact }
  | { readonly type: 'stationarity-evidence-created'; readonly evidence: StationarityEvidenceArtifact }
  | { readonly type: 'granger-evidence-created'; readonly evidence: GrangerEvidenceArtifact }
  | { readonly type: 'discovery-run-created'; readonly artifact: DiscoveryRunArtifact }
  | { readonly type: 'dag-document-created'; readonly document: DagDocument }
  | { readonly type: 'dag-document-revised'; readonly document: DagDocument }
  | { readonly type: 'dag-check-created'; readonly check: DagCheckArtifact }
  | { readonly type: 'intervention-query-created'; readonly query: InterventionQueryArtifact }
  | { readonly type: 'study-draft-changed'; readonly draft: StudyDesignDraft }
  | { readonly type: 'study-identified'; readonly study: StudySpecification; readonly identification: IdentificationArtifact }
  | { readonly type: 'estimation-run-created'; readonly run: EstimationRunArtifact }
  | { readonly type: 'sensitivity-run-created'; readonly run: SensitivityRunArtifact }
  | { readonly type: 'counterfactual-run-created'; readonly run: CounterfactualRunArtifact }
  | { readonly type: 'estimation-run-deleted'; readonly run: EstimationRunId }
  | { readonly type: 'sensitivity-run-deleted'; readonly run: SensitivityRunId }
  | { readonly type: 'counterfactual-run-deleted'; readonly run: CounterfactualRunId }
  | { readonly type: 'source-cleared' }
  | { readonly type: 'project-reopened'; readonly snapshot: PersistedProject }
  | { readonly type: 'project-restored'; readonly file: File }
  | { readonly type: 'restore-rejected'; readonly problem: SourceSelectionProblem }
  | { readonly type: 'source-persistence-changed'; readonly persistence: SourcePersistence }

export const INITIAL_WORKFLOW: Workflow = {
  kind: 'awaiting-project',
  nameDraft: '',
  problem: null,
}

const MAX_PROJECT_NAME = 80

export function projectName(raw: string): Result<ProjectName, ProjectNameProblem> {
  const value = raw.trim()
  if (value.length === 0) return err({ kind: 'empty' })
  if (value.length > MAX_PROJECT_NAME) return err({ kind: 'too-long', maximum: MAX_PROJECT_NAME })
  return ok(brand<string, 'ProjectName'>(value))
}

export function newProject(name: ProjectName): Project {
  return {
    id: brand<string, 'ProjectId'>(crypto.randomUUID()),
    name,
    createdAt: new Date().toISOString(),
  }
}

export const newImportRequestId = (): ImportRequestId =>
  brand<string, 'ImportRequestId'>(crypto.randomUUID())

export const importRequestId = (value: string): Result<ImportRequestId, { readonly kind: 'invalid-import-request-id' }> =>
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(value)
    ? ok(brand<string, 'ImportRequestId'>(value))
    : err({ kind: 'invalid-import-request-id' })

const extensionOf = (name: string): string => name.includes('.')
  ? name.slice(name.lastIndexOf('.') + 1).toLowerCase()
  : ''

export function selectSource(file: File): Result<SelectedSource, SourceSelectionProblem> {
  if (file.size === 0) return err({ kind: 'empty-file' })
  const extension = extensionOf(file.name)
  const format = extension === 'csv'
    ? 'csv'
    : extension === 'tsv'
      ? 'tsv'
      : extension === 'parquet'
        ? 'parquet'
        : null
  if (format === null) return err({ kind: 'unsupported-format', extension })
  return ok({
    file,
    name: file.name,
    bytes: file.size,
    mediaType: file.type,
    lastModified: file.lastModified,
    format,
  })
}

export function stepWorkflow(state: Workflow, event: WorkflowEvent): Workflow {
  switch (state.kind) {
    case 'awaiting-project': {
      if (event.type === 'project-name-changed') {
        return { ...state, nameDraft: event.value, problem: null }
      }
      if (event.type === 'project-reopened') {
        return { kind: 'awaiting-data', project: event.snapshot.project, problem: null, restore: event.snapshot.profile === null ? null : event.snapshot }
      }
      if (event.type !== 'project-submitted') return state
      if (event.type !== 'project-submitted') return state
      const parsed = projectName(state.nameDraft)
      return parsed.ok
        ? { kind: 'awaiting-data', project: newProject(parsed.value), problem: null, restore: null }
        : { ...state, problem: parsed.error }
    }
    case 'awaiting-data': {
      if (event.type === 'restore-rejected') return { ...state, problem: event.problem }
      if (event.type === 'project-restored' && state.restore !== null && state.restore.profile !== null) {
        const parsed = selectSource(event.file)
        if (!parsed.ok) return { ...state, problem: parsed.error }
        const snapshot = state.restore
        return {
          kind: 'profiled',
          project: snapshot.project,
          source: parsed.value,
          profile: snapshot.profile ?? (() => { throw new Error('unreachable') })(),
          prepared: snapshot.prepared,
          stationarity: snapshot.stationarity,
          grangerEvidence: snapshot.grangerEvidence,
          discoveryRuns: snapshot.discoveryRuns,
          dagDocuments: snapshot.dagDocuments,
          dagChecks: snapshot.dagChecks,
          interventionQueries: snapshot.interventionQueries,
          studyDraft: snapshot.studyDraft,
          studies: snapshot.studies,
          identifications: snapshot.identifications,
          estimationRuns: snapshot.estimationRuns,
          sensitivityRuns: snapshot.sensitivityRuns,
          counterfactualRuns: snapshot.counterfactualRuns,
        }
      }
      if (event.type !== 'file-selected' || state.restore !== null) return state
      const parsed = selectSource(event.file)
      return parsed.ok
        ? { kind: 'source-selected', project: state.project, source: parsed.value }
        : { ...state, problem: parsed.error }
    }
    case 'source-selected':
      if (event.type === 'profile-requested') {
        return { kind: 'profiling', project: state.project, source: state.source, request: event.request }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null, restore: null }
      return state
    case 'profiling':
      if (event.type === 'profile-succeeded' && event.request === state.request) {
        return {
          kind: 'profiled',
          project: state.project,
          source: state.source,
          profile: event.profile,
          prepared: null,
          stationarity: null,
          grangerEvidence: [],
          discoveryRuns: [],
          dagDocuments: [],
          dagChecks: [],
          interventionQueries: [],
          studyDraft: EMPTY_STUDY_DRAFT,
          studies: [],
          identifications: [],
          estimationRuns: [],
          sensitivityRuns: [],
          counterfactualRuns: [],
        }
      }
      if (event.type === 'profile-failed' && event.request === state.request) {
        return { kind: 'import-failed', project: state.project, source: state.source, problem: event.problem }
      }
      return state
    case 'import-failed':
      if (event.type === 'profile-requested') {
        return { kind: 'profiling', project: state.project, source: state.source, request: event.request }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null, restore: null }
      return state
    case 'profiled':
      if (event.type === 'source-persistence-changed') {
        return { ...state, profile: { ...state.profile, source: { ...state.profile.source, persistence: event.persistence } } }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null, restore: null }
      if (event.type === 'prepared-dataset-created') {
        return { ...state, prepared: event.artifact, stationarity: null, grangerEvidence: [], discoveryRuns: [], dagDocuments: [], dagChecks: [], interventionQueries: [], studyDraft: EMPTY_STUDY_DRAFT, studies: [], identifications: [], estimationRuns: [], sensitivityRuns: [], counterfactualRuns: [] }
      }
      if (event.type === 'stationarity-evidence-created'
        && state.prepared !== null
        && event.evidence.preparedDataset === state.prepared.id) {
        return { ...state, stationarity: event.evidence }
      }
      if (event.type === 'granger-evidence-created'
        && state.prepared !== null
        && event.evidence.preparedDataset === state.prepared.id) {
        return { ...state, grangerEvidence: [...state.grangerEvidence, event.evidence] }
      }
      if (event.type === 'discovery-run-created'
        && state.prepared !== null
        && event.artifact.preparedDataset === state.prepared.id) {
        return { ...state, discoveryRuns: [...state.discoveryRuns, event.artifact] }
      }
      if (event.type === 'dag-document-created'
        && state.prepared !== null
        && event.document.preparedDataset === state.prepared.id
        && !state.dagDocuments.some((document) => document.id === event.document.id)) {
        return { ...state, dagDocuments: [...state.dagDocuments, event.document] }
      }
      if (event.type === 'dag-document-revised'
        && state.prepared !== null
        && event.document.preparedDataset === state.prepared.id
        && state.dagDocuments.some((document) => document.id === event.document.id)) {
        return {
          ...state,
          dagDocuments: state.dagDocuments.map((document) =>
            document.id === event.document.id ? event.document : document),
        }
      }
      if (event.type === 'dag-check-created'
        && state.prepared !== null
        && event.check.preparedDataset === state.prepared.id
        && state.dagDocuments.some((document) => document.id === event.check.dagDocument)) {
        return { ...state, dagChecks: [...state.dagChecks, event.check] }
      }
      if (event.type === 'intervention-query-created'
        && state.prepared !== null
        && event.query.preparedDataset === state.prepared.id
        && state.dagDocuments.some((document) => document.id === event.query.dagDocument)) {
        return { ...state, interventionQueries: [...state.interventionQueries, event.query] }
      }
      if (event.type === 'study-draft-changed') return { ...state, studyDraft: event.draft }
      if (event.type === 'study-identified'
        && state.prepared !== null
        && event.study.preparedDataset === state.prepared.id
        && event.identification.study === event.study.id
        && !state.studies.some((study) => study.id === event.study.id)) {
        return {
          ...state,
          studies: [...state.studies, event.study],
          identifications: [...state.identifications, event.identification],
        }
      }
      if (event.type === 'estimation-run-created'
        && state.prepared !== null
        && event.run.preparedDataset === state.prepared.id
        && state.identifications.some((identification) => identification.id === event.run.identification)) {
        return { ...state, estimationRuns: [...state.estimationRuns, event.run] }
      }
      if (event.type === 'sensitivity-run-created'
        && state.prepared !== null
        && event.run.preparedDataset === state.prepared.id
        && state.estimationRuns.some((run) => run.id === event.run.estimationRun)) {
        return { ...state, sensitivityRuns: [...state.sensitivityRuns, event.run] }
      }
      if (event.type === 'counterfactual-run-created'
        && state.prepared !== null
        && event.run.preparedDataset === state.prepared.id
        && state.identifications.some((identification) => identification.id === event.run.identification)) {
        return { ...state, counterfactualRuns: [...state.counterfactualRuns, event.run] }
      }
      if (event.type === 'estimation-run-deleted') {
        // The probes recorded against a run describe nothing once it is gone, so they go with it.
        return {
          ...state,
          estimationRuns: state.estimationRuns.filter((run) => run.id !== event.run),
          sensitivityRuns: state.sensitivityRuns.filter((run) => run.estimationRun !== event.run),
        }
      }
      if (event.type === 'sensitivity-run-deleted') {
        return { ...state, sensitivityRuns: state.sensitivityRuns.filter((run) => run.id !== event.run) }
      }
      if (event.type === 'counterfactual-run-deleted') {
        return { ...state, counterfactualRuns: state.counterfactualRuns.filter((run) => run.id !== event.run) }
      }
      return state
    default:
      return assertNever(state)
  }
}

export function describeProjectNameProblem(problem: ProjectNameProblem): string {
  switch (problem.kind) {
    case 'empty': return 'Give the analysis a name.'
    case 'too-long': return `Keep the name to ${problem.maximum} characters or fewer.`
    default: return assertNever(problem)
  }
}

export function describeSourceSelectionProblem(problem: SourceSelectionProblem): string {
  switch (problem.kind) {
    case 'empty-file': return 'The selected file is empty.'
    case 'source-mismatch': return `This is not the file the project was built from. Choose ${problem.expected}, unchanged, or start a new project.`
    case 'fingerprint-failed': return `The file could not be checked: ${problem.detail}`
    case 'unsupported-format':
      return problem.extension
        ? `.${problem.extension} is not supported yet. Choose CSV, TSV, or Parquet.`
        : 'Choose a CSV, TSV, or Parquet file.'
    default: return assertNever(problem)
  }
}

export function describeDatasetProfileProblem(problem: DatasetProfileProblem): string {
  switch (problem.kind) {
    case 'fingerprint-failed': return 'The source fingerprint could not be calculated.'
    case 'engine-unavailable': return 'The local data engine could not start.'
    case 'registration-failed': return 'The browser could not give the data engine access to this file.'
    case 'parse-failed': return 'DuckDB could not read this file.'
    case 'cleanup-failed': return 'The local data engine could not release this source cleanly.'
    case 'empty-dataset': return 'The file has headers but no data rows.'
    case 'no-columns': return 'No columns were detected.'
    case 'unsafe-row-count': return 'The row count is outside the browser-safe integer range.'
    case 'worker-unavailable': return 'The local data worker could not start.'
    case 'worker-protocol-failed': return 'The local data worker returned an invalid response.'
    default: return assertNever(problem)
  }
}

export function datasetProfileProblemDetail(problem: DatasetProfileProblem): string | null {
  switch (problem.kind) {
    case 'fingerprint-failed': return problem.detail
    case 'engine-unavailable': return problem.detail
    case 'registration-failed': return problem.detail
    case 'parse-failed': return problem.detail
    case 'cleanup-failed': return problem.detail
    case 'empty-dataset': return null
    case 'no-columns': return null
    case 'unsafe-row-count': return problem.value
    case 'worker-unavailable': return problem.detail
    case 'worker-protocol-failed': return problem.detail
    default: return assertNever(problem)
  }
}
