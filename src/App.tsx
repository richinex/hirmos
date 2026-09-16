import { Metadata } from '@/components/ui/Metadata'
import { causalModelRunCount } from '@/domain/rootCauseAnalysis'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { lazy, Suspense, useCallback, useEffect, useMemo, useRef, useState, type FormEvent } from 'react'
import { JobsProvider } from '@/analysis/JobsProvider'
import { WorkflowProvider, useWorkflow } from '@/components/WorkflowProvider'
import { PreparationProvider } from '@/components/data/PreparationProvider'
import { Icon } from '@/components/Icon'
import { DataDropZone } from '@/components/data/DataDropZone'
import { AppShell } from '@/components/shell/AppShell'
import { ChapterBoundary } from '@/components/shell/ChapterBoundary'
import { ChapterSkeleton } from '@/components/shell/ChapterSkeleton'
import { ChapterNav, type ChapterEntry, type ChapterStatus } from '@/components/shell/ChapterNav'
import { useShellLayout } from '@/components/shell/useShellLayout'
import { button, chromeAction, field, fieldHint, iconControl, label, literal, num, panel, prose, sectionTitle, well } from '@/components/ui/recipes'
import { useTheme, THEME_LABELS, type ThemeChoice } from '@/components/ui/useTheme'
import { formatDay, formatTimestamp } from '@/lib/format/date'
import { formatBytes } from '@/lib/format/number'
import { DataStudio } from '@/components/data/DataStudio'
import { PreprocessingPanel } from '@/components/data/PreprocessingPanel'
import { DiscoveryPanel } from '@/components/discovery/DiscoveryPanel'
import { chapterPath, CHAPTER_IDS, CHAPTER_METADATA, describeRouteProblem, isCanonicalLocation, type ChapterId } from '@/domain/navigation'
import type { ChapterActivity, RunActivity } from '@/domain/activity'
import { describeSnapshotProblem, snapshotWorkflow, type PersistedProject, type SavedProjectHeader } from '@/domain/persistence'
import { assessExampleCopy, isShippedExampleId, SHIPPED_EXAMPLES, stampExampleRelease, type ShippedExample } from '@/domain/example'
import { ExampleLedger } from '@/components/projects/ExampleLedger'
import { OpenControl } from '@/components/projects/OpenControl'
import { deleteProject, listProjects, loadProject, saveProject, saveProjectIfChanged } from '@/data/projectStore'
import { lastStorageFailure, subscribeStorageHealth, type StorageFailure } from '@/data/storageHealth'
import { cacheSource, readCachedSource, removeCachedSource, sourceCacheAvailable } from '@/data/sourceCache'
import { buildBundle, bundleFileName, describeBundleProblem, parseBundle, serialiseBundle, type BundleData, type ProjectBundle } from '@/domain/bundle'
import { decodeSourceFile, downloadText, encodeSourceFile } from '@/data/bundleFiles'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { navigate, replace, useRoute } from '@/lib/router'
import { ChartExportProvider } from '@/charts/exportContext'
import {
  datasetProfileProblemDetail,
  describeDatasetProfileProblem,
  describeProjectNameProblem,
  describeSourceSelectionProblem,
  newImportRequestId,
  type DerivedRecipe,
  type SelectedSource,
} from '@/domain/workflow'
import { identificationAllowsEstimation } from '@/domain/study'
import {
  type DiscoveryEvent,
} from '@/domain/discovery'
import { assertNever, err, isNonEmpty, type Result } from '@/domain/dop'
import type { SourceRecipe, SqlPreparationInput } from '@/domain/sqlPreparation'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { cn } from '@/lib/utils'

const loadDagWorkspace = () => import('@/components/dag/DagWorkspace')
const loadSurvivalPanel = () => import('@/components/survival/SurvivalPanel')
const loadRootCausePanel = () => import('@/components/root-cause/RootCausePanel')
const loadTimeSeriesPanel = () => import('@/components/time-series/TimeSeriesPanel')
const loadStudyDesignPanel = () => import('@/components/study/StudyDesignPanel')
const loadEstimationPanel = () => import('@/components/estimation/EstimationPanel')
const loadSensitivityPanel = () => import('@/components/sensitivity/SensitivityPanel')
const loadCounterfactualPanel = () => import('@/components/counterfactual/CounterfactualPanel')
const loadResultsPanel = () => import('@/components/results/ResultsPanel')
const loadSqlPreparationWorkspace = () => import('@/components/data/SqlPreparationWorkspace')
const loadPipelineWorkspace = () => import('@/components/data/pipeline/PipelineWorkspace')

/** One loader per lazy chapter, shared with the nav prefetch so a hover warms the chunk `lazy` will ask for. */
const PANEL_LOADERS: Partial<Record<ChapterId, () => Promise<unknown>>> = {
  survival: loadSurvivalPanel,
  'root-cause': loadRootCausePanel,
  'time-series': loadTimeSeriesPanel,
  dag: loadDagWorkspace,
  study: loadStudyDesignPanel,
  estimation: loadEstimationPanel,
  sensitivity: loadSensitivityPanel,
  counterfactual: loadCounterfactualPanel,
  results: loadResultsPanel,
}

/** A failed prefetch is dropped silently; navigation retries the import for real. */
const prefetchChapter = (chapter: ChapterId): void => {
  const load = PANEL_LOADERS[chapter]
  if (load !== undefined) void load().catch(() => undefined)
}

const DagWorkspace = lazy(async () => ({ default: (await loadDagWorkspace()).DagWorkspace }))

const SurvivalPanel = lazy(async () => ({ default: (await loadSurvivalPanel()).SurvivalPanel }))
const RootCausePanel = lazy(async () => ({ default: (await loadRootCausePanel()).RootCausePanel }))
const TimeSeriesPanel = lazy(async () => ({ default: (await loadTimeSeriesPanel()).TimeSeriesPanel }))

const StudyDesignPanel = lazy(async () => ({ default: (await loadStudyDesignPanel()).StudyDesignPanel }))

const EstimationPanel = lazy(async () => ({ default: (await loadEstimationPanel()).EstimationPanel }))

const SensitivityPanel = lazy(async () => ({ default: (await loadSensitivityPanel()).SensitivityPanel }))

const CounterfactualPanel = lazy(async () => ({ default: (await loadCounterfactualPanel()).CounterfactualPanel }))

const ResultsPanel = lazy(async () => ({ default: (await loadResultsPanel()).ResultsPanel }))

const SqlShell = lazy(async () => ({ default: (await loadSqlPreparationWorkspace()).SqlShell }))
const PipelineWorkspace = lazy(async () => ({ default: (await loadPipelineWorkspace()).PipelineWorkspace }))

type SqlIntake =
  | { readonly kind: 'idle' }
  | { readonly kind: 'reading' }
  | { readonly kind: 'failed'; readonly detail: string }

type Chapter = Omit<ChapterEntry, 'status'>

const CHAPTERS: readonly Chapter[] = CHAPTER_IDS.map((id) => ({ id, ...CHAPTER_METADATA[id] }))

const THEME_ICON: Record<ThemeChoice, string> = {
  dark: 'dark_mode', light: 'wb_twilight', 'soft-dark': 'dark_mode', 'original-light': 'light_mode', system: 'brightness_auto',
}

function SourceSummary({ source }: { readonly source: SelectedSource }) {
  const sourceDetail = (() => {
    switch (source.recipe.kind) {
      case 'uploaded-file': return `${source.format}, ${formatBytes(source.bytes)}`
      case 'sql-derived': return `prepared with SQL, ${source.recipe.outputView}, ${source.recipe.inputs.length} ${source.recipe.inputs.length === 1 ? 'input' : 'inputs'}, ${formatBytes(source.bytes)}`
      case 'pipeline-derived': return `built with a pipeline, ${source.recipe.graph.nodes.length} blocks, ${source.recipe.inputs.length} ${source.recipe.inputs.length === 1 ? 'input' : 'inputs'}, ${formatBytes(source.bytes)}`
      default: return assertNever(source.recipe)
    }
  })()
  return (
    <div className={panel('lift p-4')}>
      <div className="flex items-start gap-3">
        <span className={well('grid h-9 w-9 shrink-0 place-items-center text-muted')}>
          <Icon name="table" size={18} />
        </span>
        <div className="min-w-0 flex-1">
          <p className="m-0 truncate text-title font-medium text-ink">{source.name}</p>
          <p className={num('mb-0 mt-1 text-body text-faint')}>
            {sourceDetail}
          </p>
        </div>
      </div>
    </div>
  )
}

function App() {
  const workflow = useWorkflow(state => state.workflow)
  const dispatch = useWorkflow(state => state.dispatch)
  const fileInput = useRef<HTMLInputElement>(null)
  const [dataEntryMode, setDataEntryMode] = useState<'file' | 'sql' | 'pipeline'>('file')
  const { location, route } = useRoute()
  const shell = useShellLayout()
  /** Whether the rail's lobe is out over the stage; on a phone, whether the rail is slid in. Session state, never saved. */
  const [navOpen, setNavOpen] = useState(false)
  const openNav = useCallback(() => setNavOpen(true), [])
  const closeNav = useCallback(() => setNavOpen(false), [])
  const theme = useTheme()
  const profiled = workflow.kind === 'profiled' ? workflow : null
  const currentPrepared = profiled?.prepared ?? null
  const discoverySession = useWorkflow(state => state.discovery)
  const dispatchDiscoverySession = useWorkflow(state => state.dispatchDiscovery)

  const reportDiscoveryEvent = useCallback((event: DiscoveryEvent) => {
    dispatchDiscoverySession({ type: 'discovery-event-received', event })
  }, [dispatchDiscoverySession])

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && !event.altKey && !event.shiftKey && event.key.toLowerCase() === 'b') {
        event.preventDefault()
        setNavOpen((open) => !open)
      }
    }
    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [])

  const createProject = (event: FormEvent) => {
    event.preventDefault()
    dispatch({ type: 'project-submitted' })
  }

  const chooseFile = (file: File | undefined) => {
    if (!file) return
    if (workflow.kind === 'awaiting-data' && workflow.restore !== null && workflow.restore.profile !== null) {
      const expected = workflow.restore.profile.source.fingerprint
      const expectedName = workflow.restore.source?.name ?? 'the original file'
      void import('@/data/fingerprint').then(({ fingerprintFile }) => fingerprintFile(file)).then((fingerprint) => {
        if (!fingerprint.ok) { dispatch({ type: 'restore-rejected', problem: { kind: 'fingerprint-failed', detail: fingerprint.error.detail } }); return }
        if (fingerprint.value !== expected) { dispatch({ type: 'restore-rejected', problem: { kind: 'source-mismatch', expected: expectedName } }); return }
        dispatch({ type: 'project-restored', file })
      })
      return
    }
    dispatch({ type: 'file-selected', file })
  }

  const replayRecipe = async (recipe: Exclude<SourceRecipe, { readonly kind: 'uploaded-file' }>, files: readonly File[]): Promise<Result<File, string>> => {
    if (recipe.kind === 'sql-derived') {
      const data = await import('@/data/sqlPreparation')
      const replayed = await data.replaySqlRecipe(recipe, files)
      return replayed.ok ? replayed : err(data.describeSqlPreparationProblem(replayed.error))
    }
    const [data, python] = await Promise.all([import('@/data/pipeline'), import('@/data/pythonRuntime')])
    const runtime = python.createPythonRuntime()
    try {
      const replayed = await data.replayPipelineRecipe(recipe, files, runtime.scripts)
      return replayed.ok ? replayed : err(data.describePipelineRunProblem(replayed.error, (id) => id))
    } finally { runtime.dispose() }
  }

  // The editor that made a derived source opens on it again, with its files when the page still holds them.
  const reopenEditor = async (recipe: DerivedRecipe) => {
    const [{ inputsFromMemory }] = await Promise.all([import('@/data/inputFiles'), recipe.kind === 'sql-derived' ? loadSqlPreparationWorkspace() : loadPipelineWorkspace()])
    const inputs = inputsFromMemory(recipe.inputs)
    dispatch({ type: 'editor-reopened', recipe, inputs: inputs !== null && isNonEmpty(inputs) ? inputs : null })
  }
  // The files chosen again for an editor: each must match a recorded fingerprint, and takes the alias the recipe used.
  const editorFilesChosen = async (files: readonly File[]) => {
    if (workflow.kind !== 'awaiting-editor-files') return
    const recipe = workflow.recipe
    setSqlIntake({ kind: 'reading' })
    const data = await import('@/data/sqlPreparation')
    const offered = await data.prepareSqlInputs(files)
    setSqlIntake({ kind: 'idle' })
    if (!offered.ok) { dispatch({ type: 'editor-files-refused', detail: data.describeSqlPreparationProblem(offered.error) }); return }
    const matched: SqlPreparationInput[] = []
    const missing: string[] = []
    for (const descriptor of recipe.inputs) {
      const input = offered.value.find((candidate) => candidate.fingerprint === descriptor.fingerprint)
      if (input === undefined) missing.push(descriptor.fileName)
      else matched.push({ ...input, alias: descriptor.alias })
    }
    if (missing.length > 0 || !isNonEmpty(matched)) { dispatch({ type: 'editor-files-refused', detail: `The files chosen do not include ${missing.join(', ')}, unchanged.` }); return }
    dispatch({ type: 'editor-reopened', recipe, inputs: matched })
  }
  const [editConfirm, setEditConfirm] = useState<DerivedRecipe | null>(null)

  // A source the SQL or pipeline step built is rebuilt from its input files; the result then passes the same fingerprint check.
  const restoreFromInputs = async (files: readonly File[]) => {
    if (workflow.kind !== 'awaiting-data' || workflow.restore === null) return
    const recipe = workflow.restore.source?.recipe
    if (recipe === undefined || recipe.kind === 'uploaded-file') return
    setSqlIntake({ kind: 'reading' })
    const replayed = await replayRecipe(recipe, files)
    setSqlIntake({ kind: 'idle' })
    if (!replayed.ok) { dispatch({ type: 'restore-rejected', problem: { kind: 'replay-failed', detail: replayed.error } }); return }
    chooseFile(replayed.value)
  }

  const inspectSource = async () => {
    if (workflow.kind !== 'source-selected' && workflow.kind !== 'import-failed') return
    const source = workflow.source
    const request = newImportRequestId()
    dispatch({ type: 'profile-requested', request })
    try {
      const { profileSourceInWorker } = await import('@/data/client')
      const result = await profileSourceInWorker(request, source.file)
      dispatch(result.ok
        ? { type: 'profile-succeeded', request, profile: result.value }
        : { type: 'profile-failed', request, problem: result.error })
    } catch (cause: unknown) {
      dispatch({
        type: 'profile-failed',
        request,
        problem: {
          kind: 'engine-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        },
      })
    }
  }

  const project = workflow.kind === 'awaiting-project' ? null : workflow.project
  useEffect(() => { setDataEntryMode('file') }, [project?.id])
  const [sqlIntake, setSqlIntake] = useState<SqlIntake>({ kind: 'idle' })
  // The engine and the console load while the files are read, so the SQL step renders with both already loaded.
  const chooseSqlInputs = async (files: readonly File[]) => {
    setSqlIntake({ kind: 'reading' })
    const [data] = await Promise.all([import('@/data/sqlPreparation'), loadSqlPreparationWorkspace()])
    const prepared = await data.prepareSqlInputs(files)
    if (!prepared.ok) { setSqlIntake({ kind: 'failed', detail: data.describeSqlPreparationProblem(prepared.error) }); return }
    setSqlIntake({ kind: 'idle' })
    dispatch({ type: 'sql-inputs-chosen', inputs: prepared.value })
  }
  // The canvas takes its files on its input cards, so opening it needs no file up front.
  const openPipelineEditor = async () => {
    await loadPipelineWorkspace()
    dispatch({ type: 'pipeline-opened' })
  }
  const [activity, setActivity] = useState<ChapterActivity>({})
  const reportActivity = useMemo(() => Object.fromEntries(CHAPTER_IDS.map((chapter) => [chapter, (run: RunActivity | null) => setActivity((current) => {
    if (run === null) { if (!(chapter in current)) return current; const { [chapter]: _ended, ...rest } = current; return rest }
    return { ...current, [chapter]: run }
  })])) as Record<ChapterId, (run: RunActivity | null) => void>, [])
  const discoveryDraft = discoverySession.kind === 'with-prepared-dataset'
    && currentPrepared !== null
    && discoverySession.preparedDataset === currentPrepared.id
    ? discoverySession.draft
    : null
  const [saved, setSaved] = useState<readonly SavedProjectHeader[]>([])
  /** The user's own projects; the shipped examples are listed by the ledger from the catalog instead. */
  const yours = saved.filter((entry) => !isShippedExampleId(entry.id))
  const [storageFailure, setStorageFailure] = useState<StorageFailure | null>(() => lastStorageFailure())
  const [reopenProblem, setReopenProblem] = useState<string | null>(null)
  const [exampleNotice, setExampleNotice] = useState<string | null>(null)
  const refreshSaved = useCallback(() => { void listProjects().then(setSaved) }, [])
  useEffect(() => { refreshSaved() }, [refreshSaved])
  useEffect(() => subscribeStorageHealth(setStorageFailure), [])
  // Every durable change is written after a short quiet period; opening an unchanged record is not a new save.
  useEffect(() => {
    const snapshot = snapshotWorkflow(workflow, new Date().toISOString())
    if (snapshot === null) return
    const handle = window.setTimeout(() => {
      void saveProjectIfChanged(snapshot).then((result) => {
        if (!result.ok) return
        switch (result.value.kind) {
          case 'saved': refreshSaved(); return
          case 'unchanged': return
          default: assertNever(result.value)
        }
      })
    }, 400)
    return () => window.clearTimeout(handle)
  }, [workflow, refreshSaved])
  const reopenProject = async (id: SavedProjectHeader['id']) => {
    setReopenProblem(null)
    setExampleNotice(null)
    const loaded = await loadProject(id)
    if (!loaded.ok) { setReopenProblem(loaded.error.kind === 'storage-unavailable' ? `The browser store could not be read: ${loaded.error.detail}` : loaded.error.kind === 'not-found' ? 'That project is no longer saved. Choose another project or import a bundle.' : describeSnapshotProblem(loaded.error)); return }
    const snapshot = loaded.value
    if (snapshot.profile !== null && snapshot.profile.source.persistence.kind === 'cached-locally') {
      const cached = await readCachedSource(snapshot.profile.source.fingerprint)
      if (cached !== null) {
        const { fingerprintFile } = await import('@/data/fingerprint')
        const fingerprint = await fingerprintFile(cached)
        if (fingerprint.ok && fingerprint.value === snapshot.profile.source.fingerprint) {
          dispatch({ type: 'project-reopened', snapshot })
          dispatch({ type: 'project-restored', file: cached })
          return
        }
      }
    }
    dispatch({ type: 'project-reopened', snapshot })
  }
  const removeProject = async (entry: SavedProjectHeader) => {
    const removed = await deleteProject(entry.id)
    if (!removed.ok) return
    if (entry.cachedSource !== null && !saved.some((other) => other.id !== entry.id && other.cachedSource === entry.cachedSource)) await removeCachedSource(entry.cachedSource)
    refreshSaved()
  }
  const [cacheProblem, setCacheProblem] = useState<string | null>(null)
  const [includeSource, setIncludeSource] = useState(false)
  const [importProblem, setImportProblem] = useState<string | null>(null)
  const bundleInput = useRef<HTMLInputElement>(null)
  const exportProject = async () => {
    const snapshot = snapshotWorkflow(workflow, new Date().toISOString())
    if (snapshot === null || workflow.kind !== 'profiled') return
    const data = includeSource ? await encodeSourceFile(workflow.source.file) : { kind: 'not-included' as const }
    const bundle = buildBundle(snapshot, data, new Date().toISOString())
    downloadText(bundleFileName(bundle), serialiseBundle(bundle))
  }
  const exportSaved = async (id: SavedProjectHeader['id']) => {
    const loaded = await loadProject(id)
    if (!loaded.ok) { setReopenProblem(loaded.error.kind === 'storage-unavailable' ? `The browser store could not be read: ${loaded.error.detail}` : loaded.error.kind === 'not-found' ? 'That project is no longer saved. Choose another project or import a bundle.' : describeSnapshotProblem(loaded.error)); return }
    const bundle = buildBundle(loaded.value, { kind: 'not-included' }, new Date().toISOString())
    downloadText(bundleFileName(bundle), serialiseBundle(bundle))
  }
  /** Opens a project with the data file a bundle carried, when that file is the one the project was built from. */
  const openWithBundleData = async (snapshot: PersistedProject, data: BundleData) => {
    dispatch({ type: 'project-reopened', snapshot })
    if (data.kind !== 'source-file' || snapshot.profile === null) return
    const source = decodeSourceFile(data)
    const { fingerprintFile } = await import('@/data/fingerprint')
    const fingerprint = await fingerprintFile(source)
    if (fingerprint.ok && fingerprint.value === snapshot.profile.source.fingerprint) dispatch({ type: 'project-restored', file: source })
    else dispatch({ type: 'restore-rejected', problem: { kind: 'source-mismatch', expected: snapshot.source?.name ?? 'the original file' } })
  }
  const adoptBundle = async (bundle: ProjectBundle) => {
    await saveProject(bundle.project)
    refreshSaved()
    await openWithBundleData(bundle.project, bundle.data)
  }
  const importBundle = async (file: File | undefined) => {
    if (!file) return
    setImportProblem(null)
    setExampleNotice(null)
    const parsed = parseBundle(await file.text())
    if (!parsed.ok) { setImportProblem(describeBundleProblem(parsed.error)); return }
    await adoptBundle(parsed.value)
  }
  /**
   * A shipped example. Edits to this release are retained, while a copy made from an older shipped
   * release is replaced so newly completed chapters are not hidden behind stale browser state.
   */
  const openExample = async (example: ShippedExample, reset = false) => {
    setImportProblem(null)
    setExampleNotice(null)
    let bundle: ProjectBundle
    try {
      const response = await fetch(example.bundleUrl)
      // A missing file comes back as the app shell in development, so the content type is the reliable check.
      if (!response.ok || !(response.headers.get('content-type') ?? '').includes('json')) { setImportProblem('The example bundle is not part of this build.'); return }
      const parsed = parseBundle(await response.text())
      if (!parsed.ok) { setImportProblem(describeBundleProblem(parsed.error)); return }
      const stamped = stampExampleRelease(parsed.value.project, parsed.value.exportedAt, example.id)
      if (!stamped.ok) { setImportProblem('The shipped example contains the wrong project.'); return }
      bundle = { ...parsed.value, project: stamped.value }
    } catch (cause: unknown) {
      setImportProblem(`The example could not be loaded: ${cause instanceof Error ? cause.message : String(cause)}`)
      return
    }
    const existing = reset ? null : await loadProject(example.id)
    if (existing !== null && existing.ok) {
      const assessment = assessExampleCopy(existing.value, bundle.exportedAt, example.id)
      switch (assessment.kind) {
        case 'current-release': await openWithBundleData(assessment.snapshot, bundle.data); return
        case 'replace-with-shipped':
          setExampleNotice('The example changed in this build, so your earlier copy was replaced.')
          break
        case 'invalid-copy': setImportProblem(assessment.detail); return
        default: assertNever(assessment)
      }
    }
    await adoptBundle(bundle)
  }
  const changeSourcePersistence = async (kind: 'ephemeral' | 'cached-locally') => {
    if (workflow.kind !== 'profiled') return
    setCacheProblem(null)
    const fingerprint = workflow.profile.source.fingerprint
    if (kind === 'cached-locally') {
      const cached = await cacheSource(fingerprint, workflow.source.file)
      if (!cached.ok) { setCacheProblem(cached.error.kind === 'cache-unavailable' ? 'Choose Not stored. This browser does not offer a private file store.' : `The browser refused to store the file: ${cached.error.detail}`); return }
    } else {
      await removeCachedSource(fingerprint)
    }
    dispatch({ type: 'source-persistence-changed', persistence: { kind } })
  }
  const running = useMemo(() => CHAPTER_IDS.flatMap((chapter) => { const run = activity[chapter]; return run === undefined ? [] : [{ chapter, ...run }] }).at(0) ?? null, [activity])

  const validatedDag = workflow.kind === 'profiled'
    && workflow.dagDocuments.some((document) => document.current.validation.kind === 'structurally-valid')
  const identifiedStudy = workflow.kind === 'profiled'
    && workflow.identifications.some((identification) => identificationAllowsEstimation(identification.result.kind))

  const chapterIsAvailable = (chapter: ChapterId): boolean => {
    if (chapter === 'projects') return workflow.kind === 'awaiting-project'
    if (chapter === 'data') return project !== null
    if (chapter === 'survival') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'root-cause') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'time-series') return workflow.kind === 'profiled' && workflow.prepared?.kind === 'prepared-time-series'
    if (chapter === 'discovery') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'dag') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'study') return validatedDag
    if (chapter === 'estimation') return identifiedStudy
    if (chapter === 'sensitivity') return workflow.kind === 'profiled' && workflow.estimationRuns.length > 0
    if (chapter === 'counterfactual') return workflow.kind === 'profiled' && workflow.estimationRuns.length > 0
    if (chapter === 'results') return workflow.kind === 'profiled' && (causalModelRunCount(workflow.rootCause) > 0 || workflow.estimationRuns.length > 0 || workflow.survivalRuns.length > 0 || workflow.timeSeriesRuns.length > 0 || workflow.countSeriesModels.length > 0)
    return false
  }

  const chapterStatus = (chapter: ChapterId): ChapterStatus => {
    const prepared = workflow.kind === 'profiled' && workflow.prepared !== null
    switch (chapter) {
      case 'projects': return project === null ? 'not-started' : 'done'
      case 'data': return project === null ? 'locked' : prepared ? 'done' : 'in-progress'
      case 'time-series': return workflow.kind !== 'profiled' || workflow.prepared?.kind !== 'prepared-time-series' ? 'locked' : workflow.timeSeriesRuns.length + workflow.countSeriesModels.length > 0 ? 'done' : 'not-started'
      case 'survival': return !prepared || workflow.kind !== 'profiled'
        ? 'locked'
        : workflow.survivalRuns.length > 0 ? 'done' : 'not-started'
      case 'root-cause': return !prepared || workflow.kind !== 'profiled' ? 'locked' : causalModelRunCount(workflow.rootCause) > 0 ? 'done' : workflow.rootCause.selection === null ? 'not-started' : 'in-progress'
      case 'discovery': return !prepared ? 'locked' : workflow.discoveryRuns.length > 0 ? 'done' : 'not-started'
      case 'dag': return !prepared
        ? 'locked'
        : workflow.dagDocuments.some((document) => document.current.validation.kind === 'structurally-valid')
          ? 'done'
          : workflow.dagDocuments.length > 0 ? 'in-progress' : 'not-started'
      case 'study': return !validatedDag || workflow.kind !== 'profiled'
        ? 'locked'
        : identifiedStudy ? 'done' : workflow.studies.length > 0 ? 'refused' : 'not-started'
      case 'estimation': return !identifiedStudy || workflow.kind !== 'profiled'
        ? 'locked'
        : workflow.estimationRuns.length > 0 ? 'done' : 'not-started'
      case 'sensitivity': return workflow.kind !== 'profiled' || workflow.estimationRuns.length === 0
        ? 'locked'
        : workflow.sensitivityRuns.length > 0 ? 'done' : 'not-started'
      case 'counterfactual': return workflow.kind !== 'profiled' || workflow.estimationRuns.length === 0
        ? 'locked'
        : workflow.counterfactualRuns.length > 0 ? 'done' : 'not-started'
      case 'results': return workflow.kind !== 'profiled' || (causalModelRunCount(workflow.rootCause) + workflow.estimationRuns.length + workflow.survivalRuns.length + workflow.timeSeriesRuns.length + workflow.countSeriesModels.length === 0) ? 'locked' : 'done'
      default: return chapter
    }
  }
  const chapters: readonly ChapterEntry[] = CHAPTERS.map((chapter) => ({ ...chapter, status: chapterStatus(chapter.id), busy: activity[chapter.id] === undefined ? null : activity[chapter.id]?.progress ?? 0 }))

  const defaultChapter: ChapterId = workflow.kind === 'awaiting-project' ? 'projects' : 'data'
  const requestedChapter = route.ok && route.value.kind === 'chapter' ? route.value.chapter : defaultChapter
  const activeChapter = chapterIsAvailable(requestedChapter) ? requestedChapter : defaultChapter
  const activeName = CHAPTERS.find((chapter) => chapter.id === activeChapter)?.name ?? 'Hirmos'

  // Keep the current chapter on screen while a cold code chunk is fetched. The request counter prevents
  // a slower first click from winning if the user chooses a different chapter before it has loaded.
  const navigationRequest = useRef(0)
  // Leaving for the list saves the open project first, so the debounced save cannot be lost.
  const closeProject = useCallback(async () => {
    const snapshot = snapshotWorkflow(workflow, new Date().toISOString())
    if (snapshot !== null) await saveProjectIfChanged(snapshot)
    dispatch({ type: 'project-closed' })
    refreshSaved()
    navigate(chapterPath('projects'))
  }, [workflow, refreshSaved])

  const navigateToChapter = useCallback((chapter: ChapterId) => {
    if (chapter === 'projects' && workflow.kind !== 'awaiting-project') { void closeProject(); return }
    const request = ++navigationRequest.current
    const commit = () => {
      if (navigationRequest.current === request) navigate(chapterPath(chapter))
    }
    const load = PANEL_LOADERS[chapter]
    if (load === undefined) { commit(); return }
    void load().then(commit, commit)
  }, [workflow.kind, closeProject])

  const warmableChapterKey = chapters
    .filter((chapter) => chapter.status !== 'locked' && PANEL_LOADERS[chapter.id] !== undefined)
    .map((chapter) => chapter.id)
    .join('|')
  const chartsAreReachable = workflow.kind === 'profiled' && workflow.prepared !== null

  // Warm every available lazy chapter and the shared chart registry during idle time. Availability can
  // expand as the study progresses, so the key changes only when another chapter becomes reachable.
  useEffect(() => {
    const warm = () => {
      for (const chapter of warmableChapterKey.split('|')) {
        if (chapter !== '') prefetchChapter(chapter as ChapterId)
      }
      if (chartsAreReachable) void import('@/charts/registry')
    }
    if (typeof window.requestIdleCallback !== 'function') {
      const handle = window.setTimeout(warm, 300)
      return () => window.clearTimeout(handle)
    }
    const handle = window.requestIdleCallback(warm)
    return () => window.cancelIdleCallback(handle)
  }, [chartsAreReachable, warmableChapterKey])

  // The URL always names the chapter on screen: a legacy `?chapter=` link and a gated chapter both
  // rewrite to the canonical path without adding history, and the tab title follows.
  useEffect(() => {
    document.title = `${activeName}, Hirmos`
    if (!route.ok) return
    const canonical = chapterPath(activeChapter)
    if (route.value.kind === 'chapter' && route.value.chapter !== activeChapter) replace(canonical)
    else if (route.value.kind === 'chapter' && !isCanonicalLocation(location.pathname, location.search, route.value)) replace(canonical)
  }, [activeChapter, activeName, location.pathname, location.search, route])

  const railProject = project === null ? null : { name: project.name, detail: workflow.kind === 'profiled' ? workflow.source.file.name : 'No data file yet' }

  const header = (
    <>
      <div className="flex min-w-0 items-center">
        {/* The round toggle sits in the corner above the rail: two bars that turn into a cross while the lobe is out. */}
        <button
          type="button"
          data-rail-toggle
          className={cn(
            'relative h-10 w-10 shrink-0 rounded-full bg-rail transition-opacity hover:opacity-85',
            'before:absolute before:left-3 before:h-[2px] before:w-4 before:rounded-full before:bg-rail-ink before:transition-[top,transform] before:duration-(--motion-base) before:content-[""]',
            'after:absolute after:left-3 after:h-[2px] after:w-4 after:rounded-full after:bg-rail-ink after:transition-[top,transform] after:duration-(--motion-base) after:content-[""]',
            navOpen ? 'before:top-[19px] before:rotate-[135deg] after:top-[19px] after:-rotate-[135deg]' : 'before:top-[15px] after:top-[23px]',
          )}
          aria-label={navOpen ? 'Collapse chapter list' : 'Expand chapter list'}
          aria-expanded={navOpen}
          title={`${navOpen ? 'Collapse' : 'Expand'} chapter list (⌘B)`}
          onClick={() => setNavOpen((open) => !open)}
        />
      </div>
      <div className="flex min-w-0 items-center gap-1.5">
        {storageFailure !== null && (
          <span role="status" className={chromeAction('quiet', 'gap-1.5 text-warn')} title={`The browser refused the last save: ${storageFailure.reason}`}>
            <Icon name="cloud_off" size={14} />
            <span>Not saved</span>
          </span>
        )}
        {running !== null && (
          <button
            type="button"
            className={chromeAction('quiet', 'relative min-w-0 gap-2 pr-3 text-muted')}
            aria-label={`${running.label} running in ${CHAPTERS.find((chapter) => chapter.id === running.chapter)?.name ?? running.chapter}; open it`}
            onClick={() => navigateToChapter(running.chapter)}
          >
            <span className="truncate">{running.label}</span>
            <span aria-hidden className="bar-live absolute inset-x-2 bottom-[3px] h-[2px] rounded-full bg-line">
              <span className="bar-live__fill block rounded-full bg-signal" style={{ width: `${Math.round((running.progress ?? 0) * 100)}%` }} />
            </span>
          </button>
        )}
        <button type="button" onClick={theme.cycle} aria-label="Change theme"
          title={`Theme: ${THEME_LABELS[theme.choice]}. Switch to ${THEME_LABELS[theme.next]}`}
          className={iconControl('quiet', 'text-muted')}>
          <Icon name={THEME_ICON[theme.next]} size={16} />
        </button>
      </div>
    </>
  )

  const editConfirmDialog = editConfirm !== null && (
    <ConfirmDialog
      open
      title={editConfirm.kind === 'sql-derived' ? 'Edit SQL' : 'Edit pipeline'}
      message="Editing replaces the prepared dataset and removes the current analysis."
      confirmLabel="Edit and remove"
      danger
      onConfirm={() => { const recipe = editConfirm; setEditConfirm(null); void reopenEditor(recipe) }}
      onClose={() => setEditConfirm(null)}
    />
  )
  const fullBleed = workflow.kind === 'pipeline-opened' || profiled !== null && ['data', 'time-series', 'survival', 'root-cause', 'discovery', 'dag', 'study', 'estimation', 'sensitivity', 'counterfactual', 'results'].includes(activeChapter)

  // Every chart export names the project it came from.
  const exportContext = useMemo(() => ({ project: project?.name ?? null }), [project])
  return (
    <ChartExportProvider.Provider value={exportContext}>
      <JobsProvider key={project?.id ?? ''} prepared={currentPrepared?.id ?? null} profile={profiled?.profile.id ?? null}>
      {editConfirmDialog}
      <AppShell
        skipTarget="stage"
        mode={fullBleed ? 'full' : 'reading'}
        header={header}
        nav={<ChapterNav chapters={chapters} active={activeChapter} open={navOpen} onOpen={openNav} onClose={closeNav} onNavigate={navigateToChapter} onPrefetch={prefetchChapter} project={railProject} onExport={workflow.kind === 'profiled' ? () => void exportProject() : null} />}
        stage={(
          <>
              {!route.ok && (
                <p role="alert" className={well('mb-4 px-3 py-2 text-body text-muted')}>
                  {describeRouteProblem(route.error)}; showing {activeName}.
                </p>
              )}
              {exampleNotice !== null && activeChapter === 'data' && (
                <p role="status" className={well('mb-4 px-3 py-2 text-body text-muted')}>{exampleNotice}</p>
              )}
              {workflow.kind === 'awaiting-project' && (
                <section className="rise my-auto w-full max-w-6xl" aria-labelledby="new-analysis-title">
                  <ChapterHeading id="new-analysis-title" className="mb-6">Create an analysis</ChapterHeading>
                  <form onSubmit={createProject} className="max-w-md space-y-3">
                    <label className="block">
                      <span className="mb-1.5 block text-body font-medium text-ink">Project name</span>
                      <input
                        autoFocus
                        value={workflow.nameDraft}
                        onChange={(event) => dispatch({ type: 'project-name-changed', value: event.target.value })}
                        className={field('text')}
                        placeholder="For example: minimum wage and employment"
                      />
                    </label>
                    {workflow.problem && <p role="alert" className="text-body text-danger">{describeProjectNameProblem(workflow.problem)}</p>}
                    {/* The two ways to get a project, side by side: make one, or open one exported earlier.
                        On a narrow stage the two share the row and the same height, so no dead space sits to the
                        right and a label that wraps does not leave its neighbour shorter. */}
                    <div className="flex flex-wrap items-center gap-2 @max-md/panel:items-stretch @max-md/panel:*:flex-1">
                      <button type="submit" className={button('signal')}>Create project</button>
                      <input ref={bundleInput} type="file" accept=".json,application/json" className="sr-only" aria-label="Exported project file" onChange={(event) => { void importBundle(event.target.files?.[0]); event.target.value = '' }} />
                      <button type="button" className={button('outline')} title="A .hirmos.json file from Export project. If it was exported without its data file, you choose the file after opening." onClick={() => bundleInput.current?.click()}>Open an exported file</button>
                    </div>
                  </form>
                  <section className="mt-8" aria-labelledby="projects-title">
                    <h3 id="projects-title" className={cn(sectionTitle, 'mb-2')}>Your projects</h3>
                    {reopenProblem !== null && <p role="alert" className="mb-3 text-body text-danger">{reopenProblem}</p>}
                    {importProblem !== null && <p role="alert" className="mb-3 text-body text-danger">{importProblem}</p>}
                    {yours.length === 0
                      ? <p className="m-0 text-body text-faint">None yet. Create one above, open an exported file, or start from an example below.</p>
                      : (
                        <ul className="m-0 list-none divide-y divide-hair rounded-lg border border-hair p-0" aria-label="Projects">
                          {yours.map((entry) => (
                            <li key={entry.id} className="flex items-center gap-3 px-3 py-2 transition-colors hover:bg-well">
                              <div className="min-w-0 flex-1">
                                <span className="block truncate text-body text-ink">{entry.name}</span>
                                <span className={num('block truncate text-label text-faint')}>
                                  {<Metadata><span>{entry.sourceName ?? 'no data yet'}{entry.cachedSource !== null ? ', cached' : ''}</span><span>{entry.estimationRuns} {entry.estimationRuns === 1 ? 'estimate' : 'estimates'}</span><span>saved {formatTimestamp(entry.savedAt)}</span></Metadata>}
                                </span>
                              </div>
                              <OpenControl name={entry.name} onOpen={() => void reopenProject(entry.id)} />
                              <button type="button" className={iconControl('quiet')} aria-label={`Export ${entry.name}`} title="Export this project as a bundle, without the source file" onClick={() => void exportSaved(entry.id)}><Icon name="download" size={14} /></button>
                              <button type="button" className={iconControl('danger')} aria-label={`Delete ${entry.name}`} title="Delete this saved project" onClick={() => void removeProject(entry)}><Icon name="delete" size={14} /></button>
                            </li>
                          ))}
                        </ul>
                      )}
                  </section>
                  <section className="mt-8" aria-labelledby="examples-title">
                    <h3 id="examples-title" className={cn(sectionTitle, 'mb-2')}>Examples</h3>
                    <p className={prose('mb-3 mt-0 text-faint')}>Below is a list of sample walkthroughs. Feel free to edit and use the reset button to return the sample to its original state.</p>
                    <ExampleLedger
                      examples={SHIPPED_EXAMPLES}
                      saved={saved}
                      formatSaved={formatDay}
                      onOpen={(example) => void openExample(example)}
                      onReset={(example) => void openExample(example, true)}
                      onExport={(id) => void exportSaved(id)}
                      onDelete={(entry) => void removeProject(entry)}
                    />
                  </section>
                </section>
              )}
  
              {workflow.kind === 'awaiting-data' && (
                <section className="rise my-auto w-full max-w-6xl" aria-labelledby="load-data-title">
                  <div className="grid gap-6 lg:grid-cols-[20rem_minmax(0,1fr)] lg:gap-8">
                    <div className="min-w-0">
                      <ChapterHeading id="load-data-title" className="mb-3">{workflow.restore === null ? 'Choose data' : 'Choose the data file again'}</ChapterHeading>
                      {workflow.restore === null ? (
                        <>
                          <SegmentedControl
                            variant="line"
                            ariaLabel="Data input method"
                            value={dataEntryMode}
                            onChange={setDataEntryMode}
                            options={[{ value: 'file', label: 'Upload a file' }, { value: 'sql', label: 'Prepare with SQL' }, { value: 'pipeline', label: 'Build a pipeline' }]}
                          />
                          <p className={cn(fieldHint, 'mt-3 min-h-[3lh]')}>
                            {dataEntryMode === 'file'
                              ? 'Choose one CSV, TSV or Parquet file. It becomes the source as it is.'
                              : dataEntryMode === 'sql'
                                ? 'Choose one or more CSV, TSV or Parquet files. Each file becomes a table in a SQL console, and created views can be selected for further analysis.'
                                : 'Open a canvas of blocks. Give each Input file card a file, wire blocks together to filter, join, derive and aggregate, and use the result as the source.'}
                          </p>
                        </>
                      ) : workflow.restore.source !== null && workflow.restore.source.recipe.kind !== 'uploaded-file' ? (
                        <p className={cn(fieldHint, 'mt-0')}>
                          {`${workflow.project.name} was built from ${workflow.restore.source.name}, which the ${workflow.restore.source.recipe.kind === 'sql-derived' ? 'SQL step' : 'pipeline'} created from ${workflow.restore.source.recipe.inputs.map((input) => `${input.fileName} (${formatBytes(input.bytes)})`).join(' and ')}. Choose those files again, unchanged. The recorded ${workflow.restore.source.recipe.kind === 'sql-derived' ? 'statement runs' : 'blocks run'} on them and the result is checked against the recorded fingerprint before the work returns.`}
                        </p>
                      ) : (
                        <p className={cn(fieldHint, 'mt-0')}>
                          {`${workflow.project.name} was built from ${workflow.restore.source?.name ?? 'a file'}${workflow.restore.source === null ? '' : `, ${formatBytes(workflow.restore.source.bytes)}`}. The file is not stored; its SHA-256 is checked before the recorded work returns.`}
                        </p>
                      )}
                      {workflow.problem && <p role="alert" className="mt-3 text-body text-danger">{describeSourceSelectionProblem(workflow.problem)}</p>}
                      {sqlIntake.kind === 'failed' && <p role="alert" className="mt-3 text-body text-danger">{sqlIntake.detail}</p>}
                    </div>
                    {workflow.restore?.source !== null && workflow.restore?.source !== undefined && workflow.restore.source.recipe.kind !== 'uploaded-file' ? (
                      <DataDropZone
                        multiple
                        invitation="Drop the input files here."
                        consequence={workflow.restore.source.recipe.kind === 'sql-derived' ? 'The SQL step runs again on them.' : 'The pipeline runs again on them.'}
                        action="Choose input files"
                        busy={sqlIntake.kind === 'reading'}
                        onFiles={(files) => void restoreFromInputs(files)}
                      />
                    ) : dataEntryMode === 'file' || workflow.restore !== null ? (
                      <DataDropZone
                        invitation="Drop a CSV, TSV or Parquet file here."
                        action="Choose data file"
                        onFiles={([file]) => chooseFile(file)}
                      />
                    ) : dataEntryMode === 'sql' ? (
                      <DataDropZone
                        multiple
                        invitation="Drop one or more CSV, TSV or Parquet files here."
                        consequence="The console loads each file as a table with the file's name."
                        action="Choose input files"
                        busy={sqlIntake.kind === 'reading'}
                        onFiles={(files) => void chooseSqlInputs(files)}
                        onIntent={() => { void loadSqlPreparationWorkspace() }}
                      />
                    ) : (
                      <div className="flex min-h-[18rem] flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-line bg-well px-6 py-8 text-center">
                        <Icon name="account_tree" size={28} className="text-faint" aria-hidden />
                        <p className="m-0 text-body text-ink">Build the source from blocks on a canvas.</p>
                        <p className="m-0 max-w-[40ch] font-serif text-body text-faint text-pretty">Each Input file card takes one CSV, TSV or Parquet file. Wire the cards into filters, joins, derived columns, aggregates or a Python script, and use the last block as the source.</p>
                        <button type="button" className={button('signal', 'mt-1 inline-flex items-center gap-2')} onMouseEnter={() => { void loadPipelineWorkspace() }} onFocus={() => { void loadPipelineWorkspace() }} onClick={() => void openPipelineEditor()}>Open the editor</button>
                      </div>
                    )}
                  </div>
                </section>
              )}

              {workflow.kind === 'pipeline-opened' && (
                <Suspense fallback={<ChapterSkeleton label="Loading the pipeline canvas…" />}>
                  <PipelineWorkspace resume={workflow.resume} onPrepared={(source) => dispatch({ type: 'sql-source-created', source })} />
                </Suspense>
              )}

              {workflow.kind === 'sql-inputs-chosen' && (
                <section className="rise w-full max-w-6xl" aria-label="Prepare with SQL">
                  <Suspense fallback={<ChapterSkeleton label="Loading SQL preparation…" />}>
                    <SqlShell
                      inputs={workflow.inputs}
                      resume={workflow.resume}
                      onPrepared={(source) => dispatch({ type: 'sql-source-created', source })}
                      onCleared={() => dispatch({ type: 'source-cleared' })}
                    />
                  </Suspense>
                </section>
              )}

              {workflow.kind === 'source-selected' && (
                <section className="rise my-auto max-w-2xl" aria-labelledby="selected-source-title">
                  <ChapterHeading id="selected-source-title" className="mb-3">Source selected</ChapterHeading>
                  <SourceSummary source={workflow.source} />
                  <div className="mt-4 flex gap-2">
                    <button type="button" className={button('signal')} onClick={() => void inspectSource()}>
                      Inspect data
                    </button>
                    {workflow.source.recipe.kind !== 'uploaded-file' && (
                      <button type="button" className={button('outline')} onClick={() => { const recipe = workflow.source.recipe; if (recipe.kind !== 'uploaded-file') void reopenEditor(recipe) }}>
                        {workflow.source.recipe.kind === 'sql-derived' ? 'Edit SQL' : 'Edit pipeline'}
                      </button>
                    )}
                    <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'source-cleared' })}>
                      Choose another file
                    </button>
                  </div>
                </section>
              )}

              {workflow.kind === 'awaiting-editor-files' && (
                <section className="rise my-auto w-full max-w-6xl" aria-labelledby="editor-files-title">
                  <div className="grid gap-6 lg:grid-cols-[20rem_minmax(0,1fr)] lg:gap-8">
                    <div className="min-w-0">
                      <ChapterHeading id="editor-files-title" className="mb-3">Choose the input files again</ChapterHeading>
                      <p className={cn(fieldHint, 'mt-0')}>
                        {`The page no longer holds the files the ${workflow.recipe.kind === 'sql-derived' ? 'SQL' : 'pipeline'} was built from: ${workflow.recipe.inputs.map((input) => `${input.fileName} (${formatBytes(input.bytes)})`).join(' and ')}. Choose them again, unchanged, and the editor opens where it left off.`}
                      </p>
                      {workflow.problem !== null && <p role="alert" className="mt-3 text-body text-danger">{workflow.problem}</p>}
                      <button type="button" className={button('quiet', 'mt-4')} onClick={() => dispatch({ type: 'source-cleared' })}>Choose other data instead</button>
                    </div>
                    <DataDropZone
                      multiple
                      invitation="Drop the input files here."
                      consequence="The editor opens with its blocks and arrows as they were."
                      action="Choose input files"
                      busy={sqlIntake.kind === 'reading'}
                      onFiles={(files) => void editorFilesChosen(files)}
                    />
                  </div>
                </section>
              )}
  
              {workflow.kind === 'profiling' && (
                <section className="rise my-auto max-w-2xl" aria-labelledby="profiling-title">
                  <ChapterHeading id="profiling-title" className="mb-3 flex items-center gap-2">
                    <Icon name="progress_activity" size={18} className="animate-spin [animation-duration:0.9s] text-[var(--color-info)]" />
                    Inspecting data
                  </ChapterHeading>
                  <SourceSummary source={workflow.source} />
                </section>
              )}
  
              {workflow.kind === 'import-failed' && (
                <section className="rise my-auto max-w-2xl" aria-labelledby="import-failed-title">
                  <span className={label('text-danger')}>Import refused</span>
                  <h2 id="import-failed-title" className="mb-3 mt-3 text-heading text-ink">{describeDatasetProfileProblem(workflow.problem)}</h2>
                  <SourceSummary source={workflow.source} />
                  {datasetProfileProblemDetail(workflow.problem) && (
                    <details className={well('mt-3 px-3 py-2 text-body text-muted')}>
                      <summary>Technical detail</summary>
                      <p className={literal('mb-0 mt-2 break-words text-faint')}>{datasetProfileProblemDetail(workflow.problem)}</p>
                    </details>
                  )}
                  <div className="mt-4 flex gap-2">
                    <button type="button" className={button('signal')} onClick={() => void inspectSource()}>Try again</button>
                    {workflow.source.recipe.kind !== 'uploaded-file' && (
                      <button type="button" className={button('outline')} onClick={() => { const recipe = workflow.source.recipe; if (recipe.kind !== 'uploaded-file') void reopenEditor(recipe) }}>
                        {workflow.source.recipe.kind === 'sql-derived' ? 'Edit SQL' : 'Edit pipeline'}
                      </button>
                    )}
                    <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'source-cleared' })}>
                      Choose another file
                    </button>
                  </div>
                </section>
              )}
  
              {workflow.kind === 'profiled' && (
                <>
                  {activeChapter === 'data' && (
                    <DataStudio source={workflow.source} profile={workflow.profile} prepared={workflow.prepared} onEditSource={workflow.source.recipe.kind === 'uploaded-file' ? null : () => { const recipe = workflow.source.recipe; if (recipe.kind !== 'uploaded-file') setEditConfirm(recipe) }}>
                      <PreprocessingPanel
                        key={workflow.profile.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        onPrepared={(artifact) => dispatch({ type: 'prepared-dataset-created', artifact })}
                        onStationarityEvidence={(evidence) => dispatch({ type: 'stationarity-evidence-created', evidence })}
                        onClearStationarityEvidence={() => dispatch({ type: 'stationarity-evidence-cleared' })}
                        stationarity={workflow.stationarity}
                        preparedVersion={workflow.prepared}
                        grangerEvidence={workflow.grangerEvidence}
                        onGrangerEvidence={(evidence) => dispatch({ type: 'granger-evidence-created', evidence })}
                      />
                    {workflow.prepared !== null && (
                      <section className="rounded-xl border border-edge bg-panel p-4" aria-labelledby="prepared-next-title">
                        <span className={label('text-faint')}>Continue</span>
                        <h3 id="prepared-next-title" className={cn(sectionTitle, 'mb-1 mt-1')}>Build a DAG or run discovery</h3>
                        <p className="mb-3 mt-0 text-body text-faint">Proceed directly to a DAG specified from substantive knowledge and the study design, or run discovery methods to obtain candidate empirical relations.</p>
                        <div className="flex flex-wrap gap-2">
                          <button type="button" className={button('signal')} onClick={() => navigateToChapter('dag')}>Build a DAG</button>
                          <button type="button" className={button('quiet')} onClick={() => navigateToChapter('discovery')}>Run discovery</button>
                        </div>
                      </section>
                    )}
                    <details className="group rounded-md border border-line bg-panel">
                      <summary className="flex cursor-pointer list-none items-center justify-between px-2.5 py-1.5 text-label text-muted transition-colors marker:content-none hover:text-ink">
                        <span>Source file storage and export</span>
                        <Icon name="expand_more" size={14} className="shrink-0 transition-transform group-open:rotate-180" />
                      </summary>
                      <div className="border-t border-hair px-2.5 py-3">
                        <div className="flex flex-wrap items-center gap-3">
                          <SegmentedControl
                            ariaLabel="Source file storage"
                            value={workflow.profile.source.persistence.kind}
                            onChange={(kind) => void changeSourcePersistence(kind)}
                            options={[{ value: 'ephemeral', label: 'Not stored' }, { value: 'cached-locally', label: 'Cached locally', disabled: !sourceCacheAvailable(), title: sourceCacheAvailable() ? undefined : 'This browser does not offer a private file store.' }]}
                          />
                          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'source-cleared' })}>
                            Choose another file
                          </button>
                          <span className="h-[15px] w-px bg-hair" aria-hidden />
                          <button type="button" className={button('outline')} onClick={() => void exportProject()}>Export project</button>
                          <label className="flex items-center gap-1.5 text-body text-ink">
                            <input type="checkbox" checked={includeSource} onChange={(event) => setIncludeSource(event.target.checked)} />
                            Include the source file ({formatBytes(workflow.source.bytes)})
                          </label>
                        </div>
                        <p className={prose('mb-0 mt-1 text-faint')}>{workflow.profile.source.persistence.kind === 'cached-locally' ? 'Reopening this project reads the file from the browser store.' : 'Reopening this project asks for the file again.'}</p>
                        {cacheProblem !== null && <p role="alert" className="mb-0 mt-1 text-body text-danger">{cacheProblem}</p>}
                      </div>
                    </details>
                    </DataStudio>
                  )}
                  {activeChapter === 'discovery' && workflow.prepared !== null && discoveryDraft !== null && (
                    <DiscoveryPanel
                      key={workflow.prepared.id}
                      source={workflow.source}
                      profile={workflow.profile}
                      prepared={workflow.prepared}
                      stationarity={workflow.stationarity}
                      runs={workflow.discoveryRuns}
                      documents={workflow.dagDocuments}
                      draft={discoveryDraft}
                      onEvent={reportDiscoveryEvent}
                      onActivity={reportActivity.discovery}
                      onRun={(artifact) => dispatch({ type: 'discovery-run-created', artifact })}
                      onDeleteRun={(deletion) => dispatch({ type: 'discovery-run-deletion-committed', deletion })}
                    />
                  )}
                  {activeChapter === 'dag' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading DAG editor…" />}>
                      <DagWorkspace
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        discoveryRuns={workflow.discoveryRuns}
                        documents={workflow.dagDocuments}
                        checks={workflow.dagChecks}
                        interventionQueries={workflow.interventionQueries}
                        onInterventionQuery={(query) => dispatch({ type: 'intervention-query-created', query })}
                        onDocumentCreated={(document) => dispatch({ type: 'dag-document-created', document })}
                        onDocumentRevised={(document) => dispatch({ type: 'dag-document-revised', document })}
                        onCheck={(check) => dispatch({ type: 'dag-check-created', check })}
                        onUseForStudy={() => navigateToChapter('study')}
                        onUseForRootCause={(selection) => { dispatch({ type: 'root-cause-selected', selection }); navigateToChapter('root-cause') }}
                        studyDraft={workflow.studyDraft}
                        onStudyDraftChanged={(draft) => dispatch({ type: 'study-draft-changed', draft })}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'root-cause' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                      <Suspense fallback={<ChapterSkeleton label="Loading causal model analysis…" />}>
                        <RootCausePanel key={`${workflow.prepared.id}:${workflow.rootCause.selection?.dagRevision ?? ''}`} source={workflow.source} profile={workflow.profile} prepared={workflow.prepared}
                          documents={workflow.dagDocuments} workspace={workflow.rootCause} onGraph={() => navigateToChapter('dag')}
                          onRun={(run) => dispatch({ type: 'root-cause-run-created', run })} onDelete={(id) => dispatch({ type: 'root-cause-run-deleted', id })}
                          onEffects={(run) => dispatch({ type: 'gcm-effects-created', run })} onDeleteEffects={(id) => dispatch({ type: 'gcm-effects-deleted', id })}
                          onInfluence={(run) => dispatch({ type: 'gcm-influence-created', run })} onDeleteInfluence={(id) => dispatch({ type: 'gcm-influence-deleted', id })}
                          onChecks={(record) => dispatch({ type: 'root-cause-checks-created', record })} />
                      </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'study' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading study design…" />}>
                      <StudyDesignPanel
                      onActivity={reportActivity.study}
                        key={workflow.prepared.id}
                        prepared={workflow.prepared}
                        documents={workflow.dagDocuments}
                        draft={workflow.studyDraft}
                        onDraftChanged={(draft) => dispatch({ type: 'study-draft-changed', draft })}
                        studies={workflow.studies}
                        identifications={workflow.identifications}
                        onIdentified={(study, identification) => dispatch({ type: 'study-identified', study, identification })}
                        onContinue={() => navigateToChapter('estimation')}
                        onOpenDag={() => navigateToChapter('dag')}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'estimation' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading estimation…" />}>
                      <EstimationPanel
                      onActivity={reportActivity.estimation}
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        stationarity={workflow.stationarity}
                        documents={workflow.dagDocuments}
                        studies={workflow.studies}
                        identifications={workflow.identifications}
                        runs={workflow.estimationRuns}
                        sensitivityRuns={workflow.sensitivityRuns}
                        onRun={(run) => dispatch({ type: 'estimation-run-created', run })}
                        onDeleteRun={(run) => dispatch({ type: 'estimation-run-deleted', run })}
                        onOpenStudy={() => navigateToChapter('study')}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'sensitivity' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading sensitivity…" />}>
                      <SensitivityPanel
                      onActivity={reportActivity.sensitivity}
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        studies={workflow.studies}
                        estimationRuns={workflow.estimationRuns}
                        runs={workflow.sensitivityRuns}
                        onRun={(run) => dispatch({ type: 'sensitivity-run-created', run })}
                        onDeleteRun={(run) => dispatch({ type: 'sensitivity-run-deleted', run })}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'counterfactual' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading counterfactuals…" />}>
                      <CounterfactualPanel
                      onActivity={reportActivity.counterfactual}
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        documents={workflow.dagDocuments}
                        studies={workflow.studies}
                        identifications={workflow.identifications}
                        runs={workflow.counterfactualRuns}
                        onRun={(run) => dispatch({ type: 'counterfactual-run-created', run })}
                        onDeleteRun={(run) => dispatch({ type: 'counterfactual-run-deleted', run })}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'time-series' && workflow.prepared?.kind === 'prepared-time-series' && (
                    <ChapterBoundary chapter="Time-series analysis">
                    <Suspense fallback={<ChapterSkeleton label="Loading time-series analysis…" />}>
                      <TimeSeriesPanel key={workflow.prepared.id} source={workflow.source} profile={workflow.profile} prepared={workflow.prepared}
                        runs={workflow.timeSeriesRuns} counts={workflow.countSeriesModels}
                        onRun={(run) => dispatch({ type: 'time-series-run-created', run })}
                        onDeleteRun={(run) => dispatch({ type: 'time-series-run-deleted', run })}
                        onCount={(artifact) => dispatch({ type: 'count-series-model-created', artifact })}
                        onDeleteCount={(run) => dispatch({ type: 'count-series-model-deleted', run })}
                        onActivity={reportActivity['time-series']} />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'survival' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading survival analysis…" />}>
                      <SurvivalPanel
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        runs={workflow.survivalRuns}
                        onRun={(run) => dispatch({ type: 'survival-run-created', run })}
                        onDeleteRun={(run) => dispatch({ type: 'survival-run-deleted', run })}
                        onActivity={reportActivity.survival}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                  {activeChapter === 'results' && workflow.prepared !== null && (
                    <ChapterBoundary key={activeChapter} chapter={activeName}>
                    <Suspense fallback={<ChapterSkeleton label="Loading results…" />}>
                      <ResultsPanel
                        key={workflow.prepared.id}
                        source={workflow.source}
                        profile={workflow.profile}
                        prepared={workflow.prepared}
                        stationarity={workflow.stationarity}
                        documents={workflow.dagDocuments}
                        studies={workflow.studies}
                        identifications={workflow.identifications}
                        estimationRuns={workflow.estimationRuns}
                        sensitivityRuns={workflow.sensitivityRuns}
                        counterfactualRuns={workflow.counterfactualRuns}
                        survivalRuns={workflow.survivalRuns}
                        timeSeriesRuns={workflow.timeSeriesRuns}
                        countSeriesModels={workflow.countSeriesModels}
                        rootCauseRuns={workflow.rootCause.runs}
                        gcmEffects={workflow.rootCause.effects}
                        gcmInfluences={workflow.rootCause.influences}
                      />
                    </Suspense>
                    </ChapterBoundary>
                  )}
                </>
              )}
          </>
        )}
      />
      </JobsProvider>
    </ChartExportProvider.Provider>
  )
}

export default function Workbench() {
  return <WorkflowProvider><PreparationProvider><App /></PreparationProvider></WorkflowProvider>
}
