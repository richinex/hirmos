import { useReducer, useRef, useSyncExternalStore, type FormEvent } from 'react'
import { Icon } from '@/components/Icon'
import { button, field, label, literal, num } from '@/components/ui/recipes'
import { DataProfileView } from '@/components/data/DataProfileView'
import { PreprocessingPanel } from '@/components/data/PreprocessingPanel'
import { DiscoveryPanel } from '@/components/discovery/DiscoveryPanel'
import { DagWorkspace } from '@/components/dag/DagWorkspace'
import { parseChapterSelection, type ChapterId } from '@/domain/navigation'
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

interface Chapter {
  readonly id: ChapterId
  readonly name: string
  readonly shortName: string
  readonly icon: string
}

const CHAPTERS: readonly Chapter[] = [
  { id: 'projects', name: 'Projects', shortName: 'Projects', icon: 'folder_open' },
  { id: 'data', name: 'Data Studio', shortName: 'Data', icon: 'table_view' },
  { id: 'discovery', name: 'Discovery Lab', shortName: 'Discovery', icon: 'schema' },
  { id: 'dag', name: 'DAG Workspace', shortName: 'DAG', icon: 'conversion_path' },
  { id: 'study', name: 'Study Design', shortName: 'Study', icon: 'experiment' },
  { id: 'estimation', name: 'Estimation', shortName: 'Estimate', icon: 'query_stats' },
  { id: 'robustness', name: 'Robustness', shortName: 'Robust', icon: 'fact_check' },
  { id: 'results', name: 'Results', shortName: 'Results', icon: 'monitoring' },
]

const ROUTE_EVENT = 'hirmos-route-change'

const subscribeToSearch = (notify: () => void): (() => void) => {
  window.addEventListener('popstate', notify)
  window.addEventListener(ROUTE_EVENT, notify)
  return () => {
    window.removeEventListener('popstate', notify)
    window.removeEventListener(ROUTE_EVENT, notify)
  }
}

const currentSearch = (): string => window.location.search
const serverSearch = (): string => ''

const navigateToChapter = (chapter: ChapterId) => {
  const url = new URL(window.location.href)
  url.searchParams.set('chapter', chapter)
  window.history.pushState(null, '', url)
  window.dispatchEvent(new Event(ROUTE_EVENT))
}

const formatBytes = (bytes: number): string => {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function SourceSummary({ source }: { readonly source: SelectedSource }) {
  return (
    <div className="lift rounded-xl border border-line bg-panel p-4">
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
  const chapterSelection = parseChapterSelection(useSyncExternalStore(subscribeToSearch, currentSearch, serverSearch))

  const createProject = (event: FormEvent) => {
    event.preventDefault()
    dispatch({ type: 'project-submitted' })
  }

  const chooseFile = (file: File | undefined) => {
    if (!file) return
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

  const chapterIsAvailable = (chapter: ChapterId): boolean => {
    if (chapter === 'projects') return workflow.kind === 'awaiting-project'
    if (chapter === 'data') return project !== null
    if (chapter === 'discovery') return workflow.kind === 'profiled' && workflow.prepared !== null
    if (chapter === 'dag') return workflow.kind === 'profiled' && workflow.prepared !== null
    return false
  }

  const defaultChapter: ChapterId = workflow.kind === 'awaiting-project' ? 'projects' : 'data'
  const requestedChapter = chapterSelection.ok && chapterSelection.value.kind === 'selected'
    ? chapterSelection.value.chapter
    : defaultChapter
  const activeChapter = chapterIsAvailable(requestedChapter) ? requestedChapter : defaultChapter

  return (
    <div className="flex h-full min-h-0 flex-col bg-stage text-ink">
      <header className="relative flex h-12 shrink-0 items-center justify-between gap-4 border-b border-line bg-panel px-3.5">
        <div className="flex min-w-0 items-baseline gap-3">
          <h1 className="m-0 text-title font-semibold tracking-[-0.03em]">Hirmos</h1>
          <span className={label('hidden text-faint sm:inline')}>causal inference workbench</span>
        </div>
        {project && <span className="truncate text-body text-muted">{project.name}</span>}
      </header>

      <div className="flex min-h-0 flex-1">
        <nav aria-label="Workspace chapters" className="hidden w-44 shrink-0 flex-col border-r border-line bg-panel px-2 py-3 md:flex">
          <ol className="space-y-1">
            {CHAPTERS.map((chapter) => {
              const active = chapter.id === activeChapter
              const available = chapterIsAvailable(chapter.id)
              return (
                <li key={chapter.id}>
                  <button
                    type="button"
                    disabled={!available}
                    aria-current={active ? 'page' : undefined}
                    onClick={() => navigateToChapter(chapter.id)}
                    className={`flex w-full items-center gap-2 rounded-lg border px-2.5 py-2 text-left text-body transition-colors ${
                      active ? 'border-edge bg-raised text-ink' : 'border-transparent text-muted disabled:text-dim'
                    }`}
                  >
                    <Icon name={chapter.icon} size={16} fill={active} />
                    <span>{chapter.name}</span>
                  </button>
                </li>
              )
            })}
          </ol>
        </nav>

        <main className="panel-scroll min-w-0 flex-1 overflow-y-auto">
          <div className="mx-auto flex min-h-full w-full max-w-5xl flex-col px-5 py-8 sm:px-8 lg:px-12">
            {!chapterSelection.ok && (
              <p role="alert" className="mb-4 rounded-lg border border-hair bg-well px-3 py-2 text-body text-muted">
                Unknown chapter “{chapterSelection.error.value}” in the URL; showing {defaultChapter === 'projects' ? 'Projects' : 'Data Studio'}.
              </p>
            )}
            {workflow.kind === 'awaiting-project' && (
              <section className="rise my-auto max-w-xl" aria-labelledby="new-analysis-title">
                <span className={label('text-signal')}>01 · Projects</span>
                <h2 id="new-analysis-title" className="mb-6 mt-3 text-heading text-ink">Create an analysis</h2>
                <form onSubmit={createProject} className="max-w-md space-y-3">
                  <label className="block">
                    <span className="mb-1.5 block text-body font-medium text-ink">Project name</span>
                    <input
                      autoFocus
                      value={workflow.nameDraft}
                      onChange={(event) => dispatch({ type: 'project-name-changed', value: event.target.value })}
                      className={field('text')}
                      placeholder="e.g. Seat-belt law and road fatalities"
                    />
                  </label>
                  {workflow.problem && <p role="alert" className="text-body text-danger">{describeProjectNameProblem(workflow.problem)}</p>}
                  <button type="submit" className={button('signal')}>Create project</button>
                </form>
              </section>
            )}

            {workflow.kind === 'awaiting-data' && (
              <section className="rise my-auto max-w-2xl" aria-labelledby="load-data-title">
                <span className={label('text-signal')}>02 · Data studio</span>
                <h2 id="load-data-title" className="mb-3 mt-3 text-heading text-ink">Choose a data file</h2>
                <p className="mb-6 text-body text-faint">CSV, TSV, or Parquet</p>
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
                <div hidden={activeChapter !== 'data'}>
                  <DataProfileView profile={workflow.profile}>
                    <PreprocessingPanel
                      key={workflow.profile.id}
                      source={workflow.source}
                      profile={workflow.profile}
                      onPrepared={(artifact) => dispatch({ type: 'prepared-dataset-created', artifact })}
                      onStationarityEvidence={(evidence) => dispatch({ type: 'stationarity-evidence-created', evidence })}
                    />
                  </DataProfileView>
                  <button type="button" className={button('quiet', 'mt-4')} onClick={() => dispatch({ type: 'source-cleared' })}>
                    Choose another file
                  </button>
                  {workflow.prepared !== null && (
                    <section className="mt-4 rounded-xl border border-edge bg-panel p-4" aria-labelledby="prepared-next-title">
                      <span className={label('text-faint')}>Continue</span>
                      <h3 id="prepared-next-title" className="mb-1 mt-1 text-title font-medium text-ink">Choose the next reasoning step</h3>
                      <p className="mb-3 mt-0 text-body text-faint">A prepared dataset can go directly to a user-authored DAG. Discovery is optional evidence, not a prerequisite.</p>
                      <div className="flex flex-wrap gap-2">
                        <button type="button" className={button('signal')} onClick={() => navigateToChapter('dag')}>Build a DAG</button>
                        <button type="button" className={button('quiet')} onClick={() => navigateToChapter('discovery')}>Explore discovery evidence</button>
                      </div>
                    </section>
                  )}
                </div>
                {activeChapter === 'discovery' && workflow.prepared !== null && (
                  <DiscoveryPanel
                    source={workflow.source}
                    profile={workflow.profile}
                    prepared={workflow.prepared}
                    stationarity={workflow.stationarity}
                    runs={workflow.discoveryRuns}
                    onRun={(artifact) => dispatch({ type: 'discovery-run-created', artifact })}
                  />
                )}
                {activeChapter === 'dag' && workflow.prepared !== null && (
                  <DagWorkspace
                    key={workflow.prepared.id}
                    profile={workflow.profile}
                    prepared={workflow.prepared}
                    discoveryRuns={workflow.discoveryRuns}
                    documents={workflow.dagDocuments}
                    onDocumentCreated={(document) => dispatch({ type: 'dag-document-created', document })}
                    onDocumentRevised={(document) => dispatch({ type: 'dag-document-revised', document })}
                  />
                )}
              </>
            )}

          </div>
        </main>
      </div>

      <nav aria-label="Workspace chapters" className="flex shrink-0 items-stretch overflow-x-auto border-t border-line bg-panel pb-[env(safe-area-inset-bottom)] md:hidden">
        {CHAPTERS.slice(0, 5).map((chapter) => {
          const active = chapter.id === activeChapter
          return (
            <button
              key={chapter.id}
              type="button"
              disabled={!chapterIsAvailable(chapter.id)}
              aria-current={active ? 'page' : undefined}
              onClick={() => navigateToChapter(chapter.id)}
              className="flex min-w-[68px] flex-1 flex-col items-center gap-1 px-2 py-2 text-micro text-muted disabled:text-dim aria-[current=page]:text-signal"
            >
              <Icon name={chapter.icon} size={18} fill={active} />
              <span>{chapter.shortName}</span>
            </button>
          )
        })}
      </nav>
    </div>
  )
}

export default App
