import { lazy, Suspense, useCallback, useEffect, useMemo, useReducer, useRef, useState, type FormEvent } from 'react'
import { Icon } from '@/components/Icon'
import { HirmosMark } from '@/components/HirmosMark'
import { AppShell } from '@/components/shell/AppShell'
import { ChapterBoundary } from '@/components/shell/ChapterBoundary'
import { ChapterSkeleton } from '@/components/shell/ChapterSkeleton'
import { ChapterNav, type ChapterEntry, type ChapterStatus } from '@/components/shell/ChapterNav'
import { useIsMobile } from '@/lib/useMediaQuery'
import { useShellLayout } from '@/components/shell/useShellLayout'
import { button, chromeAction, field, iconControl, label, literal, num } from '@/components/ui/recipes'
import { useTheme, type ThemeChoice } from '@/components/ui/useTheme'
import { formatTimestamp } from '@/lib/format/date'
import { DataStudio } from '@/components/data/DataStudio'
import { PreprocessingPanel } from '@/components/data/PreprocessingPanel'
import { DiscoveryPanel } from '@/components/discovery/DiscoveryPanel'
import { chapterPath, CHAPTER_IDS, describeRouteProblem, isCanonicalLocation, type ChapterId } from '@/domain/navigation'
import type { ChapterActivity, RunActivity } from '@/domain/activity'
import { describeSnapshotProblem, snapshotWorkflow, type PersistedProject, type SavedProjectHeader } from '@/domain/persistence'
import { EXAMPLE_BUNDLE_URL, EXAMPLE_PROJECT_ID, EXAMPLE_PROJECT_NAME, EXAMPLE_SOURCE_NAME } from '@/domain/example'
import { deleteProject, listProjects, loadProject, saveProject } from '@/data/projectStore'
import { lastStorageFailure, subscribeStorageHealth, type StorageFailure } from '@/data/storageHealth'
import { cacheSource, readCachedSource, removeCachedSource, sourceCacheAvailable } from '@/data/sourceCache'
import { buildBundle, bundleFileName, describeBundleProblem, parseBundle, serialiseBundle, type BundleData, type ProjectBundle } from '@/domain/bundle'
import { decodeSourceFile, downloadText, encodeSourceFile } from '@/data/bundleFiles'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { InternalLink } from '@/components/ui/InternalLink'
import { navigate, replace, useRoute } from '@/lib/router'
import {
  datasetProfileProblemDetail,
  describeDatasetProfileProblem,
  describeProjectNameProblem,
  describeSourceSelectionProblem,
  INITIAL_WORKFLOW,
  newImportRequestId,
  stepWorkflow,
  type SelectedSource,
} from '@/domain/workflow'
import { identificationAllowsEstimation } from '@/domain/study'

const loadDagWorkspace = () => import('@/components/dag/DagWorkspace')
const loadStudyDesignPanel = () => import('@/components/study/StudyDesignPanel')
const loadEstimationPanel = () => import('@/components/estimation/EstimationPanel')
const loadSensitivityPanel = () => import('@/components/sensitivity/SensitivityPanel')
const loadCounterfactualPanel = () => import('@/components/counterfactual/CounterfactualPanel')
const loadResultsPanel = () => import('@/components/results/ResultsPanel')

/** One loader per lazy chapter, shared with the nav prefetch so a hover warms the chunk `lazy` will ask for. */
const PANEL_LOADERS: Partial<Record<ChapterId, () => Promise<unknown>>> = {
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

const StudyDesignPanel = lazy(async () => ({ default: (await loadStudyDesignPanel()).StudyDesignPanel }))

const EstimationPanel = lazy(async () => ({ default: (await loadEstimationPanel()).EstimationPanel }))

const SensitivityPanel = lazy(async () => ({ default: (await loadSensitivityPanel()).SensitivityPanel }))

const CounterfactualPanel = lazy(async () => ({ default: (await loadCounterfactualPanel()).CounterfactualPanel }))

const ResultsPanel = lazy(async () => ({ default: (await loadResultsPanel()).ResultsPanel }))

type Chapter = Omit<ChapterEntry, 'status'>

const CHAPTERS: readonly Chapter[] = [
  { id: 'projects', name: 'Projects', shortName: 'Projects', icon: 'folder_open' },
  { id: 'data', name: 'Data studio', shortName: 'Data', icon: 'table_view' },
  { id: 'discovery', name: 'Discovery lab', shortName: 'Discovery', icon: 'schema' },
  { id: 'dag', name: 'DAG workspace', shortName: 'DAG', icon: 'conversion_path' },
  { id: 'study', name: 'Study design', shortName: 'Study', icon: 'experiment' },
  { id: 'estimation', name: 'Estimation', shortName: 'Estimate', icon: 'query_stats' },
  { id: 'sensitivity', name: 'Sensitivity', shortName: 'Sensitivity', icon: 'fact_check' },
  { id: 'counterfactual', name: 'Counterfactuals', shortName: 'What if', icon: 'alt_route' },
  { id: 'results', name: 'Results', shortName: 'Results', icon: 'monitoring' },
]

/** The icon previews the next stop in the theme cycle, so the button reads as "switch to". */
const THEME_ICON: Record<ThemeChoice, string> = {
  dark: 'dark_mode',
  operational: 'grid_view',
  light: 'light_mode',
  sketchbook: 'palette',
  'sketchbook-white': 'contrast',
  system: 'brightness_auto',
}

const formatBytes = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function SourceSummary({ source }: { readonly source: SelectedSource }) {
  return (
    <div className="lift rounded-xl border border-hair bg-panel p-4">
      <div className="flex items-start gap-3">
        <span className="grid h-9 w-9 shrink-0 place-items-center rounded-lg border border-hair bg-well text-muted">
          <Icon name="table" size={18} />
        </span>
        <div className="min-w-0 flex-1">
          <p className="m-0 truncate text-title font-medium text-ink">{source.name}</p>
          <p className={num('mb-0 mt-1 text-body text-faint')}>
            {source.format} · {formatBytes(source.bytes)}
          </p>
        </div>
      </div>
    </div>
  )
}

function App() {
  const [workflow, dispatch] = useReducer(stepWorkflow, INITIAL_WORKFLOW)
  const fileInput = useRef<HTMLInputElement>(null)
  const { location, route } = useRoute()
  const shell = useShellLayout()
  const theme = useTheme()
  const profiled = workflow.kind === 'profiled' ? workflow : null

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && !event.altKey && !event.shiftKey && event.key.toLowerCase() === 'b') {
        event.preventDefault()
        shell.toggleNav()
      }
    }
    window.addEventListener('keydown', onKeyDown)
    return () => window.removeEventListener('keydown', onKeyDown)
  }, [shell])

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
  const [activity, setActivity] = useState<ChapterActivity>({})
  const reportActivity = useMemo(() => Object.fromEntries(CHAPTER_IDS.map((chapter) => [chapter, (run: RunActivity | null) => setActivity((current) => {
    if (run === null) { if (!(chapter in current)) return current; const { [chapter]: _ended, ...rest } = current; return rest }
    return { ...current, [chapter]: run }
  })])) as Record<ChapterId, (run: RunActivity | null) => void>, [])
  const [saved, setSaved] = useState<readonly SavedProjectHeader[]>([])
  /** The example's row: its saved copy when there is one, otherwise the shipped bundle as it would be listed. */
  const exampleEntry: SavedProjectHeader = saved.find((entry) => entry.id === EXAMPLE_PROJECT_ID)
    ?? { id: EXAMPLE_PROJECT_ID, name: EXAMPLE_PROJECT_NAME, savedAt: '', sourceName: EXAMPLE_SOURCE_NAME, cachedSource: null, estimationRuns: 0 }
  const [storageFailure, setStorageFailure] = useState<StorageFailure | null>(() => lastStorageFailure())
  const [reopenProblem, setReopenProblem] = useState<string | null>(null)
  const refreshSaved = useCallback(() => { void listProjects().then(setSaved) }, [])
  useEffect(() => { refreshSaved() }, [refreshSaved])
  useEffect(() => subscribeStorageHealth(setStorageFailure), [])
  // Every durable change is written after a short quiet period; a failed write surfaces in the header, never silently.
  useEffect(() => {
    const snapshot = snapshotWorkflow(workflow, new Date().toISOString())
    if (snapshot === null) return
    const handle = window.setTimeout(() => { void saveProject(snapshot).then((result) => { if (result.ok) refreshSaved() }) }, 400)
    return () => window.clearTimeout(handle)
  }, [workflow, refreshSaved])
  const reopenProject = async (id: SavedProjectHeader['id']) => {
    setReopenProblem(null)
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
    const parsed = parseBundle(await file.text())
    if (!parsed.ok) { setImportProblem(describeBundleProblem(parsed.error)); return }
    await adoptBundle(parsed.value)
  }
  /**
   * The shipped example. Its saved copy is reopened with the data file from the bundle, so it never
   * asks for a file; a reset, or no saved copy, takes the bundle's own state.
   */
  const openExample = async (reset = false) => {
    setImportProblem(null)
    let bundle: ProjectBundle
    try {
      const response = await fetch(EXAMPLE_BUNDLE_URL)
      // A missing file comes back as the app shell in development, so the content type is the reliable check.
      if (!response.ok || !(response.headers.get('content-type') ?? '').includes('json')) { setImportProblem('The example bundle is not part of this build.'); return }
      const parsed = parseBundle(await response.text())
      if (!parsed.ok) { setImportProblem(describeBundleProblem(parsed.error)); return }
      bundle = parsed.value
    } catch (cause: unknown) {
      setImportProblem(`The example could not be loaded: ${cause instanceof Error ? cause.message : String(cause)}`)
      return
    }
    const existing = reset ? null : await loadProject(EXAMPLE_PROJECT_ID)
    if (existing !== null && existing.ok) await openWithBundleData(existing.value, bundle.data)
    else await adoptBundle(bundle)
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
    if (chapter === 'discovery') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'dag') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'study') return validatedDag
    if (chapter === 'estimation') return identifiedStudy
    if (chapter === 'sensitivity') return workflow.kind === 'profiled' && workflow.estimationRuns.length > 0
    if (chapter === 'counterfactual') return workflow.kind === 'profiled' && workflow.estimationRuns.length > 0
    if (chapter === 'results') return workflow.kind === 'profiled' && workflow.estimationRuns.length > 0
    return false
  }

  const chapterStatus = (chapter: ChapterId): ChapterStatus => {
    const prepared = workflow.kind === 'profiled' && workflow.prepared !== null
    switch (chapter) {
      case 'projects': return project === null ? 'not-started' : 'done'
      case 'data': return project === null ? 'locked' : prepared ? 'done' : 'in-progress'
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
      case 'results': return workflow.kind !== 'profiled' || workflow.estimationRuns.length === 0 ? 'locked' : 'done'
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
  const navigateToChapter = useCallback((chapter: ChapterId) => {
    const request = ++navigationRequest.current
    const commit = () => {
      if (navigationRequest.current === request) navigate(chapterPath(chapter))
    }
    const load = PANEL_LOADERS[chapter]
    if (load === undefined) { commit(); return }
    void load().then(commit, commit)
  }, [])

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
    document.title = `${activeName} · Hirmos`
    if (!route.ok) return
    const canonical = chapterPath(activeChapter)
    if (route.value.kind === 'chapter' && route.value.chapter !== activeChapter) replace(canonical)
    else if (route.value.kind === 'chapter' && !isCanonicalLocation(location.pathname, location.search, route.value)) replace(canonical)
  }, [activeChapter, activeName, location.pathname, location.search, route])

  const phone = useIsMobile()
  const [phoneNavOpen, setPhoneNavOpen] = useState(false)
  const navOpen = phone ? phoneNavOpen : !shell.navCollapsed

  const header = (
    <>
      <div className="flex min-w-0 items-center gap-3.5">
        <button
          type="button"
          className={iconControl('quiet', 'text-muted')}
          aria-label={navOpen ? 'Collapse chapter list' : 'Expand chapter list'}
          aria-expanded={navOpen}
          title={`${navOpen ? 'Collapse' : 'Expand'} chapter list (⌘B)`}
          onClick={() => { if (phone) setPhoneNavOpen((open) => !open); else shell.toggleNav() }}
        >
          <Icon name={navOpen ? 'left_panel_close' : 'left_panel_open'} size={16} />
        </button>
        <h1 className="m-0">
          <InternalLink href="/" className="flex items-center gap-2 text-body font-medium uppercase tracking-[0.1em] text-ink transition-opacity hover:opacity-70" title="Hirmos home">
            <HirmosMark className="text-signal" />
            hirmos
          </InternalLink>
        </h1>
        {project !== null && (
          <>
            <div className="h-[15px] w-px shrink-0 bg-hair" />
            <span className="truncate text-body text-muted">{project.name}</span>
          </>
        )}
      </div>
      <div className="flex min-w-0 items-center gap-1.5">
        {storageFailure !== null && (
          <span role="status" className={chromeAction('quiet', 'gap-1.5 text-warn')} title={`The browser refused the last save: ${storageFailure.reason}`}>
            <Icon name="cloud_off" size={14} />
            <span className="normal-case tracking-normal">Not saved</span>
          </span>
        )}
        {running !== null && (
          <button
            type="button"
            className={chromeAction('quiet', 'relative min-w-0 gap-2 pr-3 text-muted')}
            aria-label={`${running.label} running in ${CHAPTERS.find((chapter) => chapter.id === running.chapter)?.name ?? running.chapter}; open it`}
            onClick={() => navigateToChapter(running.chapter)}
          >
            <span aria-hidden className="pulse-live h-1.5 w-1.5 shrink-0 rounded-full bg-signal" />
            <span className="truncate normal-case tracking-normal">{running.label}</span>
            <span aria-hidden className="bar-live absolute inset-x-2 bottom-[3px] h-[2px] rounded-full bg-line">
              <span className="bar-live__fill block rounded-full bg-signal" style={{ width: `${Math.round((running.progress ?? 0) * 100)}%` }} />
            </span>
          </button>
        )}
        <button
          type="button"
          onClick={theme.cycle}
          aria-label="Change theme"
          title={`Theme: ${theme.choice}. Switch to ${theme.next}`}
          className={iconControl('quiet', 'text-muted')}
        >
          <Icon name={THEME_ICON[theme.next]} size={16} />
        </button>
      </div>
    </>
  )

  const fullBleed = profiled !== null && ['data', 'discovery', 'dag', 'study', 'estimation', 'sensitivity', 'counterfactual', 'results'].includes(activeChapter)

  return (
    <AppShell
      skipTarget="stage"
      mode={fullBleed ? 'full' : 'reading'}
      header={header}
      nav={<ChapterNav chapters={chapters} active={activeChapter} collapsed={shell.navCollapsed} onNavigate={navigateToChapter} onPrefetch={prefetchChapter} phoneOpen={phoneNavOpen} onPhoneOpen={() => setPhoneNavOpen(true)} onPhoneClose={() => setPhoneNavOpen(false)} />}
      stage={(
        <>
            {!route.ok && (
              <p role="alert" className="mb-4 rounded-lg border border-hair bg-well px-3 py-2 text-body text-muted">
                {describeRouteProblem(route.error)}; showing {activeName}.
              </p>
            )}
            {workflow.kind === 'awaiting-project' && (
              <section className="rise my-auto max-w-xl" aria-labelledby="new-analysis-title">
                <span className={label('text-faint')}>01 · Projects</span>
                <h2 id="new-analysis-title" className="mb-6 mt-3 text-heading text-ink">Create an analysis</h2>
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
                  <button type="submit" className={button('signal')}>Create project</button>
                </form>
                <section className="mt-8" aria-labelledby="projects-title">
                  <h3 id="projects-title" className="mb-2 text-title font-medium text-ink">Projects</h3>
                  <p className="mb-3 mt-0 max-w-[65ch] text-body text-faint">Kept in this browser. Reopening asks for the data file again and checks it is the same file; the example brings its own.</p>
                  {reopenProblem !== null && <p role="alert" className="mb-3 text-body text-danger">{reopenProblem}</p>}
                  {importProblem !== null && <p role="alert" className="mb-3 text-body text-danger">{importProblem}</p>}
                  <ul className="m-0 list-none divide-y divide-hair rounded-lg border border-hair p-0" aria-label="Projects">
                    {[exampleEntry, ...saved.filter((entry) => entry.id !== EXAMPLE_PROJECT_ID)].map((entry) => {
                      const example = entry.id === EXAMPLE_PROJECT_ID
                      const stored = saved.some((other) => other.id === entry.id)
                      return (
                        <li key={entry.id} className="flex items-center gap-3 px-3 py-2 transition-colors hover:bg-well">
                          <div className="min-w-0 flex-1">
                            <span className="flex items-center gap-2 text-body text-ink"><span className="truncate">{entry.name}</span>{example && <span className={label('shrink-0 text-faint')}>Example</span>}</span>
                            <span className={num('block truncate text-label text-faint')}>
                              {stored
                                ? `${entry.sourceName ?? 'no data yet'}${entry.cachedSource !== null ? ' · cached' : ''} · ${entry.estimationRuns} ${entry.estimationRuns === 1 ? 'estimate' : 'estimates'} · saved ${formatTimestamp(entry.savedAt)}`
                                : `${entry.sourceName} · every chapter filled in`}
                            </span>
                          </div>
                          {/* Every row renders all four action slots, hidden where a control does not apply, so the columns align down the list. */}
                          <button type="button" className={button('outline')} onClick={() => void (example ? openExample() : reopenProject(entry.id))}>Open</button>
                          {example && stored
                            ? <button type="button" className={iconControl('quiet')} aria-label="Reset the example" title="Put the example back as shipped, discarding changes to this copy" onClick={() => void openExample(true)}><Icon name="restart_alt" size={14} /></button>
                            : <span aria-hidden className={iconControl('quiet', 'invisible')} />}
                          {stored
                            ? <button type="button" className={iconControl('quiet')} aria-label={`Export ${entry.name}`} title="Export this project as a bundle, without the source file" onClick={() => void exportSaved(entry.id)}><Icon name="download" size={14} /></button>
                            : <span aria-hidden className={iconControl('quiet', 'invisible')} />}
                          {stored
                            ? <button type="button" className={iconControl('danger')} aria-label={`Delete ${entry.name}`} title="Delete this saved project" onClick={() => void removeProject(entry)}><Icon name="delete" size={14} /></button>
                            : <span aria-hidden className={iconControl('danger', 'invisible')} />}
                        </li>
                      )
                    })}
                  </ul>
                </section>
                <section className="mt-8" aria-labelledby="import-bundle-title">
                  <h3 id="import-bundle-title" className="mb-2 text-title font-medium text-ink">Open an exported project</h3>
                  <p className="mb-3 mt-0 max-w-[65ch] text-body text-faint">A <span className="font-mono">.hirmos.json</span> file from Export project. If it was exported without its data file, you choose the file after opening.</p>
                  <input ref={bundleInput} type="file" accept=".json,application/json" className="sr-only" aria-label="Exported project file" onChange={(event) => { void importBundle(event.target.files?.[0]); event.target.value = '' }} />
                  <button type="button" className={button('outline')} onClick={() => bundleInput.current?.click()}>Choose a file</button>
                </section>
              </section>
            )}

            {workflow.kind === 'awaiting-data' && (
              <section className="rise my-auto max-w-2xl" aria-labelledby="load-data-title">
                <span className={label('text-signal')}>02 · Data studio</span>
                <h2 id="load-data-title" className="mb-3 mt-3 text-heading text-ink">{workflow.restore === null ? 'Choose a data file' : 'Choose the data file again'}</h2>
                <p className="mb-6 text-body text-faint">
                  {workflow.restore === null
                    ? 'CSV, TSV, or Parquet'
                    : `${workflow.project.name} was built from ${workflow.restore.source?.name ?? 'a file'}${workflow.restore.source === null ? '' : ` · ${formatBytes(workflow.restore.source.bytes)}`}. The file is not stored; its SHA-256 is checked before the recorded work returns.`}
                </p>
                <input
                  ref={fileInput}
                  type="file"
                  accept=".csv,.tsv,.parquet,text/csv,text/tab-separated-values,application/vnd.apache.parquet"
                  className="sr-only"
                  onChange={(event) => chooseFile(event.target.files?.[0])}
                />
                <button type="button" className={button('signal', 'inline-flex items-center gap-2')} onClick={() => fileInput.current?.click()}>
                  <Icon name="upload_file" size={16} />
                  Choose data file
                </button>
                {workflow.problem && <p role="alert" className="mt-3 text-body text-danger">{describeSourceSelectionProblem(workflow.problem)}</p>}
              </section>
            )}

            {workflow.kind === 'source-selected' && (
              <section className="rise my-auto max-w-2xl" aria-labelledby="selected-source-title">
                <span className={label('text-signal')}>02 · Data studio</span>
                <h2 id="selected-source-title" className="mb-3 mt-3 text-heading text-ink">Source selected</h2>
                <SourceSummary source={workflow.source} />
                <div className="mt-4 flex gap-2">
                  <button type="button" className={button('signal')} onClick={() => void inspectSource()}>
                    Inspect data
                  </button>
                  <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'source-cleared' })}>
                    Choose another file
                  </button>
                </div>
              </section>
            )}

            {workflow.kind === 'profiling' && (
              <section className="rise my-auto max-w-2xl" aria-labelledby="profiling-title">
                <span className={label('text-signal')}>02 · Data studio</span>
                <h2 id="profiling-title" className="mb-3 mt-3 flex items-center gap-2 text-heading text-ink">
                  <Icon name="progress_activity" size={18} className="animate-spin [animation-duration:0.9s] text-[var(--color-info)]" />
                  Inspecting data
                </h2>
                <SourceSummary source={workflow.source} />
              </section>
            )}

            {workflow.kind === 'import-failed' && (
              <section className="rise my-auto max-w-2xl" aria-labelledby="import-failed-title">
                <span className={label('text-danger')}>Import refused</span>
                <h2 id="import-failed-title" className="mb-3 mt-3 text-heading text-ink">{describeDatasetProfileProblem(workflow.problem)}</h2>
                <SourceSummary source={workflow.source} />
                {datasetProfileProblemDetail(workflow.problem) && (
                  <details className="mt-3 rounded-lg border border-hair bg-well px-3 py-2 text-body text-muted">
                    <summary>Technical detail</summary>
                    <p className={literal('mb-0 mt-2 break-words text-faint')}>{datasetProfileProblemDetail(workflow.problem)}</p>
                  </details>
                )}
                <div className="mt-4 flex gap-2">
                  <button type="button" className={button('signal')} onClick={() => void inspectSource()}>Try again</button>
                  <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'source-cleared' })}>
                    Choose another file
                  </button>
                </div>
              </section>
            )}

            {workflow.kind === 'profiled' && (
              <>
                {activeChapter === 'data' && (
                  <DataStudio source={workflow.source} profile={workflow.profile} prepared={workflow.prepared}>
                    <PreprocessingPanel
                      key={workflow.profile.id}
                      source={workflow.source}
                      profile={workflow.profile}
                      onPrepared={(artifact) => dispatch({ type: 'prepared-dataset-created', artifact })}
                      onStationarityEvidence={(evidence) => dispatch({ type: 'stationarity-evidence-created', evidence })}
                      stationarity={workflow.stationarity}
                      preparedVersion={workflow.prepared}
                      grangerEvidence={workflow.grangerEvidence}
                      onGrangerEvidence={(evidence) => dispatch({ type: 'granger-evidence-created', evidence })}
                      countSeriesModels={workflow.countSeriesModels}
                      onCountSeriesModel={(artifact) => dispatch({ type: 'count-series-model-created', artifact })}
                    />
                  {workflow.prepared !== null && (
                    <section className="rounded-xl border border-edge bg-panel p-4" aria-labelledby="prepared-next-title">
                      <span className={label('text-faint')}>Continue</span>
                      <h3 id="prepared-next-title" className="mb-1 mt-1 text-title font-medium text-ink">Build a DAG or run discovery</h3>
                      <p className="mb-3 mt-0 text-body text-faint">Proceed directly to a DAG specified from substantive knowledge and the study design, or run discovery methods to obtain candidate empirical relations.</p>
                      <div className="flex flex-wrap gap-2">
                        <button type="button" className={button('signal')} onClick={() => navigateToChapter('dag')}>Build a DAG</button>
                        <button type="button" className={button('quiet')} onClick={() => navigateToChapter('discovery')}>Run discovery</button>
                      </div>
                    </section>
                  )}
                  <details className="group rounded-md border border-line bg-panel">
                    <summary className={label('flex cursor-pointer list-none items-center justify-between px-2.5 py-1.5 text-muted transition-colors marker:content-none hover:text-ink')}>
                      <span>Source file · storage and export</span>
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
                      <p className="mb-0 mt-1 max-w-[65ch] text-body text-faint">{workflow.profile.source.persistence.kind === 'cached-locally' ? 'Reopening this project reads the file from the browser store.' : 'Reopening this project asks for the file again.'}</p>
                      {cacheProblem !== null && <p role="alert" className="mb-0 mt-1 text-body text-danger">{cacheProblem}</p>}
                    </div>
                  </details>
                  </DataStudio>
                )}
                {activeChapter === 'discovery' && workflow.prepared !== null && (
                  <DiscoveryPanel
                    key={workflow.prepared.id}
                    onActivity={reportActivity.discovery}
                    source={workflow.source}
                    profile={workflow.profile}
                    prepared={workflow.prepared}
                    stationarity={workflow.stationarity}
                    runs={workflow.discoveryRuns}
                    onRun={(artifact) => dispatch({ type: 'discovery-run-created', artifact })}
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
                      studyDraft={workflow.studyDraft}
                      onStudyDraftChanged={(draft) => dispatch({ type: 'study-draft-changed', draft })}
                    />
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
                    />
                  </Suspense>
                  </ChapterBoundary>
                )}
              </>
            )}
        </>
      )}
    />
  )
}

export default App
