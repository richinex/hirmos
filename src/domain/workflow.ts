import { assertNever, brand, err, ok, type Brand, type NonEmptyArray, type Result } from './dop'
import type { DatasetProfile, DatasetProfileProblem, SourcePersistence } from './dataset'
import type { DagDocument } from './dag'
import { EMPTY_ROOT_CAUSE, type RootCauseRun, type RootCauseWorkspace, type RootCauseCheckRecord } from './rootCauseAnalysis'
import type { GcmEffectsRun } from './gcmEffects'
import type { GcmInfluenceRun } from './gcmInfluence'
import { bindRootCauseRecord, appendRootCauseRecord } from './rootCauseRecords'
import { selectedRootCauseGraph, type RootCauseSelection } from './rootCause'
import type { DagCheckArtifact } from './dagValidation'
import type { DiscoveryRunArtifact } from './discovery'
import { deleteDiscoveryRun, type DeletableDiscoveryRun } from './discoveryLifecycle'
import type { GrangerEvidenceArtifact } from './granger'
import type { CountSeriesModelArtifact } from './countSeries'
import type { InterventionQueryArtifact } from './intervention'
import type { EstimationRunArtifact, EstimationRunId } from './estimation'
import type { SensitivityRunArtifact, SensitivityRunId } from './sensitivity'
import type { CounterfactualRunArtifact, CounterfactualRunId } from './counterfactual'
import type { SurvivalRunArtifact, SurvivalRunId } from './survival'
import { timeSeriesRunMatches, type TimeSeriesRun, type TimeSeriesRunId } from './timeSeries'
import { EMPTY_STUDY_DRAFT, type IdentificationArtifact, type StudyDesignDraft, type StudySpecification } from './study'
import type { PreparedDatasetArtifact, StationarityEvidenceArtifact } from './preprocessing'
import type { PersistedProject } from './persistence'
import type { ProjectOrigin } from './projectOrigin'
import type { SourceRecipe, SqlPreparationInput, SqlViewName } from './sqlPreparation'
import type { PipelineGraph } from './pipeline'

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
  /** The SQL step of a reopened project could not be run again on the files chosen. */
  | { readonly kind: 'replay-failed'; readonly detail: string }

export interface SelectedSource {
  readonly file: File
  readonly name: string
  readonly bytes: number
  readonly mediaType: string
  readonly lastModified: number
  readonly format: 'csv' | 'tsv' | 'parquet'
  readonly recipe: SourceRecipe
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
      readonly origin: ProjectOrigin
      readonly problem: SourceSelectionProblem | null
      /** A saved project waiting for its source file; the artifacts return once the file's fingerprint matches. */
      readonly restore: PersistedProject | null
    }
  | {
      readonly kind: 'sql-inputs-chosen'
      readonly project: Project
      readonly origin: ProjectOrigin
      /** The files the SQL console exposes as tables; a view becomes the source only once it materialises. */
      readonly inputs: readonly SqlPreparationInput[]
      /** The recorded statement and output view to start from, when the console is reopened on a source it made. */
      readonly resume: SqlResume | null
    }
  | {
      /** The pipeline canvas is open; its input blocks take their files on the canvas, and the output block becomes the source only once it materialises. */
      readonly kind: 'pipeline-opened'
      readonly project: Project
      readonly origin: ProjectOrigin
      /** The graph and files to start from, when the canvas is reopened on a source it made. */
      readonly resume: PipelineResume | null
    }
  | {
      /** An editor is to be reopened on a derived source, but its input files are no longer in memory and must be chosen again. */
      readonly kind: 'awaiting-editor-files'
      readonly project: Project
      readonly origin: ProjectOrigin
      readonly recipe: DerivedRecipe
      readonly problem: string | null
    }
  | { readonly kind: 'source-selected'; readonly project: Project; readonly origin: ProjectOrigin; readonly source: SelectedSource }
  | {
      readonly kind: 'profiling'
      readonly project: Project
      readonly origin: ProjectOrigin
      readonly source: SelectedSource
      readonly request: ImportRequestId
    }
  | {
      readonly kind: 'import-failed'
      readonly project: Project
      readonly origin: ProjectOrigin
      readonly source: SelectedSource
      readonly problem: DatasetProfileProblem
    }
  | {
      readonly kind: 'profiled'
      readonly project: Project
      readonly origin: ProjectOrigin
      readonly source: SelectedSource
      readonly profile: DatasetProfile
      readonly prepared: PreparedDatasetArtifact | null
      readonly stationarity: StationarityEvidenceArtifact | null
      readonly grangerEvidence: readonly GrangerEvidenceArtifact[]
      readonly countSeriesModels: readonly CountSeriesModelArtifact[]
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
      /** Standalone time-to-event analyses; these do not depend on a causal study or estimate. */
      readonly survivalRuns: readonly SurvivalRunArtifact[]
      readonly timeSeriesRuns: readonly TimeSeriesRun[]
      readonly rootCause: RootCauseWorkspace
    }

export type DerivedRecipe = Exclude<SourceRecipe, { readonly kind: 'uploaded-file' }>
export interface SqlResume { readonly statement: string; readonly outputView: SqlViewName }
export interface PipelineResume { readonly graph: PipelineGraph; readonly inputs: readonly SqlPreparationInput[] }

export type WorkflowEvent =
  | { readonly type: 'source-replaced'; readonly previous: SelectedSource; readonly source: SelectedSource }
  | { readonly type: 'gcm-effects-created'; readonly run: GcmEffectsRun }
  | { readonly type: 'gcm-effects-deleted'; readonly id: string }
  | { readonly type: 'gcm-influence-created'; readonly run: GcmInfluenceRun }
  | { readonly type: 'gcm-influence-deleted'; readonly id: string }
  | { readonly type: 'root-cause-selected'; readonly selection: RootCauseSelection }
  | { readonly type: 'root-cause-run-created'; readonly run: RootCauseRun }
  | { readonly type: 'root-cause-checks-created'; readonly record: RootCauseCheckRecord }
  | { readonly type: 'root-cause-run-deleted'; readonly id: string }
  | { readonly type: 'project-name-changed'; readonly value: string }
  | { readonly type: 'project-submitted' }
  | { readonly type: 'file-selected'; readonly file: File }
  | { readonly type: 'sql-inputs-chosen'; readonly inputs: readonly SqlPreparationInput[] }
  | { readonly type: 'pipeline-opened' }
  /** Reopen the editor that made the source; with the files when they are still in memory, otherwise the files are asked for first. Everything made from the source is dropped. */
  | { readonly type: 'editor-reopened'; readonly recipe: DerivedRecipe; readonly inputs: readonly SqlPreparationInput[] | null }
  | { readonly type: 'editor-files-refused'; readonly detail: string }
  /** A derived source is ready: the SQL view or the pipeline's output block has been written to a file. */
  | { readonly type: 'sql-source-created'; readonly source: SelectedSource }
  | { readonly type: 'profile-requested'; readonly request: ImportRequestId }
  | { readonly type: 'profile-succeeded'; readonly request: ImportRequestId; readonly profile: DatasetProfile }
  | { readonly type: 'profile-failed'; readonly request: ImportRequestId; readonly problem: DatasetProfileProblem }
  | { readonly type: 'prepared-dataset-created'; readonly artifact: PreparedDatasetArtifact }
  | { readonly type: 'stationarity-evidence-created'; readonly evidence: StationarityEvidenceArtifact }
  | { readonly type: 'stationarity-evidence-cleared' }
  | { readonly type: 'granger-evidence-created'; readonly evidence: GrangerEvidenceArtifact }
  | { readonly type: 'count-series-model-created'; readonly artifact: CountSeriesModelArtifact }
  | { readonly type: 'discovery-run-created'; readonly artifact: DiscoveryRunArtifact }
  | { readonly type: 'discovery-run-deletion-committed'; readonly deletion: DeletableDiscoveryRun }
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
  | { readonly type: 'survival-run-created'; readonly run: SurvivalRunArtifact }
  | { readonly type: 'survival-run-deleted'; readonly run: SurvivalRunId }
  | { readonly type: 'time-series-run-created'; readonly run: TimeSeriesRun }
  | { readonly type: 'time-series-run-deleted'; readonly run: TimeSeriesRunId }
  | { readonly type: 'count-series-model-deleted'; readonly run: CountSeriesModelArtifact['id'] }
  | { readonly type: 'source-cleared' }
  | { readonly type: 'project-reopened'; readonly snapshot: PersistedProject }
  | { readonly type: 'project-restored'; readonly file: File }
  | { readonly type: 'restore-rejected'; readonly problem: SourceSelectionProblem }
  | { readonly type: 'source-persistence-changed'; readonly persistence: SourcePersistence }
  | { readonly type: 'project-closed' }

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
    recipe: { kind: 'uploaded-file' },
  })
}

export function selectDerivedSource(
  file: File,
  recipe: Exclude<SourceRecipe, { readonly kind: 'uploaded-file' }>,
): Result<SelectedSource, SourceSelectionProblem> {
  const selected = selectSource(file)
  return selected.ok ? ok({ ...selected.value, recipe }) : selected
}

/** The SQL workspace's name for the same step. */
export const selectSqlDerivedSource = selectDerivedSource

/** The editor that made a derived source, open on it again; without the files in memory, the state that asks for them. */
const reopenEditor = (project: Project, origin: ProjectOrigin, event: Extract<WorkflowEvent, { readonly type: 'editor-reopened' }>): Workflow => {
  if (event.inputs === null) return { kind: 'awaiting-editor-files', project, origin, recipe: event.recipe, problem: null }
  switch (event.recipe.kind) {
    case 'sql-derived': return { kind: 'sql-inputs-chosen', project, origin, inputs: event.inputs, resume: { statement: event.recipe.statement, outputView: event.recipe.outputView } }
    case 'pipeline-derived': return { kind: 'pipeline-opened', project, origin, resume: { graph: event.recipe.graph, inputs: event.inputs } }
    default: return assertNever(event.recipe)
  }
}

export function stepWorkflow(state: Workflow, event: WorkflowEvent): Workflow {
  // Closing returns to the list whatever the project's stage; the caller saves first.
  if (event.type === 'project-closed') return INITIAL_WORKFLOW
  switch (state.kind) {
    case 'awaiting-project': {
      if (event.type === 'project-name-changed') {
        return { ...state, nameDraft: event.value, problem: null }
      }
      if (event.type === 'project-reopened') {
        return { kind: 'awaiting-data', project: event.snapshot.project, origin: event.snapshot.origin, problem: null, restore: event.snapshot.profile === null ? null : event.snapshot }
      }
      if (event.type !== 'project-submitted') return state
      if (event.type !== 'project-submitted') return state
      const parsed = projectName(state.nameDraft)
      return parsed.ok
        ? { kind: 'awaiting-data', project: newProject(parsed.value), origin: { kind: 'user' }, problem: null, restore: null }
        : { ...state, problem: parsed.error }
    }
    case 'awaiting-data': {
      if (event.type === 'restore-rejected') return { ...state, problem: event.problem }
      if (event.type === 'project-restored' && state.restore !== null && state.restore.profile !== null) {
        const parsed = selectSource(event.file)
        if (!parsed.ok) return { ...state, problem: parsed.error }
        const snapshot = state.restore
        const source = snapshot.source === null
          ? parsed.value
          : { ...parsed.value, recipe: snapshot.source.recipe }
        return {
          kind: 'profiled',
          project: snapshot.project,
          origin: snapshot.origin,
          source,
          profile: snapshot.profile ?? (() => { throw new Error('unreachable') })(),
          prepared: snapshot.prepared,
          stationarity: snapshot.stationarity,
          grangerEvidence: snapshot.grangerEvidence,
          countSeriesModels: snapshot.countSeriesModels,
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
          survivalRuns: snapshot.survivalRuns,
          timeSeriesRuns: snapshot.timeSeriesRuns,
          rootCause: snapshot.rootCause,
        }
      }
      if (event.type === 'sql-inputs-chosen' && state.restore === null) {
        return { kind: 'sql-inputs-chosen', project: state.project, origin: state.origin, inputs: event.inputs, resume: null }
      }
      if (event.type === 'pipeline-opened' && state.restore === null) {
        return { kind: 'pipeline-opened', project: state.project, origin: state.origin, resume: null }
      }
      if (event.type !== 'file-selected' || state.restore !== null) return state
      const parsed = selectSource(event.file)
      return parsed.ok
        ? { kind: 'source-selected', project: state.project, origin: state.origin, source: parsed.value }
        : { ...state, problem: parsed.error }
    }
    case 'sql-inputs-chosen':
    case 'pipeline-opened':
      if (event.type === 'sql-source-created') {
        return { kind: 'source-selected', project: state.project, origin: state.origin, source: event.source }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, origin: state.origin, problem: null, restore: null }
      return state
    case 'awaiting-editor-files':
      if (event.type === 'editor-reopened') return reopenEditor(state.project, state.origin, event)
      if (event.type === 'editor-files-refused') return { ...state, problem: event.detail }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, origin: state.origin, problem: null, restore: null }
      return state
    case 'source-selected':
      if (event.type === 'editor-reopened') return reopenEditor(state.project, state.origin, event)
      if (event.type === 'profile-requested') {
        return { kind: 'profiling', project: state.project, origin: state.origin, source: state.source, request: event.request }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, origin: state.origin, problem: null, restore: null }
      return state
    case 'profiling':
      if (event.type === 'profile-succeeded' && event.request === state.request) {
        return {
          kind: 'profiled',
          project: state.project,
          origin: state.origin,
          source: state.source,
          profile: event.profile,
          prepared: null,
          stationarity: null,
          grangerEvidence: [],
          countSeriesModels: [],
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
          survivalRuns: [],
          timeSeriesRuns: [],
          rootCause: EMPTY_ROOT_CAUSE,
        }
      }
      if (event.type === 'profile-failed' && event.request === state.request) {
        return { kind: 'import-failed', project: state.project, origin: state.origin, source: state.source, problem: event.problem }
      }
      return state
    case 'import-failed':
      if (event.type === 'profile-requested') {
        return { kind: 'profiling', project: state.project, origin: state.origin, source: state.source, request: event.request }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, origin: state.origin, problem: null, restore: null }
      return state
    case 'profiled':
      if (event.type === 'source-replaced') return event.previous === state.source
        ? { kind: 'source-selected', project: state.project, origin: state.origin, source: event.source }
        : state
      if (event.type === 'editor-reopened') return reopenEditor(state.project, state.origin, event)
      if (event.type === 'source-persistence-changed') {
        return { ...state, profile: { ...state.profile, source: { ...state.profile.source, persistence: event.persistence } } }
      }
      if (event.type === 'source-cleared') return { kind: 'awaiting-data', project: state.project, origin: state.origin, problem: null, restore: null }
      if (event.type === 'prepared-dataset-created') {
        return { ...state, prepared: event.artifact, stationarity: null, grangerEvidence: [], countSeriesModels: [], discoveryRuns: [], dagDocuments: [], dagChecks: [], interventionQueries: [], studyDraft: EMPTY_STUDY_DRAFT, studies: [], identifications: [], estimationRuns: [], sensitivityRuns: [], counterfactualRuns: [], survivalRuns: [], timeSeriesRuns: [], rootCause: EMPTY_ROOT_CAUSE }
      }
      if (event.type === 'root-cause-selected') {
        if (state.prepared === null || !selectedRootCauseGraph(event.selection, state.dagDocuments, state.prepared).ok) return state
        return { ...state, rootCause: { ...state.rootCause, selection: event.selection } }
      }
      switch (event.type) {
        case 'root-cause-run-created':
        case 'gcm-effects-created':
        case 'gcm-influence-created':
        case 'root-cause-checks-created': {
          const candidate = rootCauseRecord(event)
          const accepted = bindRootCauseRecord(candidate, state.dagDocuments, state.prepared)
          return accepted.ok ? { ...state, rootCause: appendRootCauseRecord(state.rootCause, accepted.value) } : state
        }
      }
      if (event.type === 'gcm-effects-deleted') {
        return { ...state, rootCause: { ...state.rootCause, effects: state.rootCause.effects.filter((run) => run.id !== event.id) } }
      }
      if (event.type === 'gcm-influence-deleted') {
        return { ...state, rootCause: { ...state.rootCause, influences: state.rootCause.influences.filter((run) => run.id !== event.id) } }
      }
      if (event.type === 'root-cause-run-deleted') {
        return { ...state, rootCause: { ...state.rootCause, runs: state.rootCause.runs.filter((run) => run.id !== event.id) } }
      }
      if (event.type === 'stationarity-evidence-cleared') return { ...state, stationarity: null }
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
      if (event.type === 'count-series-model-created'
        && state.prepared?.kind === 'prepared-time-series'
        && event.artifact.preparedDataset === state.prepared.id) {
        return { ...state, countSeriesModels: [...state.countSeriesModels, event.artifact] }
      }
      if (event.type === 'discovery-run-created'
        && state.prepared !== null
        && event.artifact.preparedDataset === state.prepared.id) {
        return { ...state, discoveryRuns: [...state.discoveryRuns, event.artifact] }
      }
      if (event.type === 'discovery-run-deletion-committed') {
        return {
          ...state,
          discoveryRuns: deleteDiscoveryRun(state.discoveryRuns, event.deletion),
        }
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
      if (event.type === 'survival-run-created'
        && state.prepared !== null
        && event.run.preparedDataset === state.prepared.id) {
        return { ...state, survivalRuns: [...state.survivalRuns, event.run] }
      }
      if (event.type === 'survival-run-deleted') {
        return { ...state, survivalRuns: state.survivalRuns.filter((run) => run.id !== event.run) }
      }
      if (event.type === 'time-series-run-created'
        && state.prepared?.kind === 'prepared-time-series'
        && timeSeriesRunMatches(event.run, state.prepared)) {
        return { ...state, timeSeriesRuns: [...state.timeSeriesRuns, event.run] }
      }
      if (event.type === 'time-series-run-deleted') {
        return { ...state, timeSeriesRuns: state.timeSeriesRuns.filter((run) => run.id !== event.run) }
      }
      if (event.type === 'count-series-model-deleted') {
        return { ...state, countSeriesModels: state.countSeriesModels.filter((run) => run.id !== event.run) }
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
    case 'replay-failed': return `The SQL step could not be run again: ${problem.detail}`
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
type RootCauseRecordEvent = Extract<WorkflowEvent, { readonly type: 'root-cause-run-created' | 'gcm-effects-created' | 'gcm-influence-created' | 'root-cause-checks-created' }>

function rootCauseRecord(event: RootCauseRecordEvent) {
  switch (event.type) {
    case 'root-cause-run-created': return { kind: 'run', record: event.run } as const
    case 'gcm-effects-created': return { kind: 'effects', record: event.run } as const
    case 'gcm-influence-created': return { kind: 'influence', record: event.run } as const
    case 'root-cause-checks-created': return { kind: 'checks', record: event.record } as const
    default: return assertNever(event)
  }
}
