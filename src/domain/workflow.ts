import { assertNever, brand, err, ok, type Brand, type Result } from './dop'
import type { DatasetProfile, DatasetProfileProblem } from './dataset'
import type { DiscoveryRunArtifact } from './discovery'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from './preprocessing'

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
      readonly discoveryRuns: readonly DiscoveryRunArtifact[]
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
  | { readonly type: 'discovery-run-created'; readonly artifact: DiscoveryRunArtifact }
  | { readonly type: 'source-cleared' }

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
      if (event.type !== 'project-submitted') return state
      const parsed = projectName(state.nameDraft)
      return parsed.ok
        ? { kind: 'awaiting-data', project: newProject(parsed.value), problem: null }
        : { ...state, problem: parsed.error }
    }
    case 'awaiting-data': {
      if (event.type !== 'file-selected') return state
      const parsed = selectSource(event.file)
      return parsed.ok
        ? { kind: 'source-selected', project: state.project, source: parsed.value }
        : { ...state, problem: parsed.error }
    }
    case 'source-selected':
      if (event.type === 'profile-requested') {
        return { kind: 'profiling', project: state.project, source: state.source, request: event.request }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null }
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
          discoveryRuns: [],
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
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null }
      return state
    case 'profiled':
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, problem: null }
      if (event.type === 'prepared-dataset-created') {
        return { ...state, prepared: event.artifact, stationarity: null, discoveryRuns: [] }
      }
      if (event.type === 'stationarity-evidence-created'
        && state.prepared !== null
        && event.evidence.preparedDataset === state.prepared.id) {
        return { ...state, stationarity: event.evidence }
      }
      if (event.type === 'discovery-run-created'
        && state.prepared !== null
        && event.artifact.preparedDataset === state.prepared.id) {
        return { ...state, discoveryRuns: [...state.discoveryRuns, event.artifact] }
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
