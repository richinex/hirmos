import { lazy, Suspense, useState } from 'react'
import { createStore } from 'zustand/vanilla'
import { useStore } from 'zustand'
import type { SelectedSource, SqlResume, PipelineResume } from '@/domain/workflow'
import type { NonEmptyArray } from '@/domain/dop'
import { assertNever, isNonEmpty, ok } from '@/domain/dop'
import { matchInputFiles, type SqlPreparationInput } from '@/domain/sourceInputs'
import { inputsFromMemory } from '@/data/inputFiles'
import { initialGraph } from './pipeline/pipelineWorkspaceModel'
import { Alert } from '@/components/ui/Alert'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { button } from '@/components/ui/recipes'
import { DataDropZone } from './DataDropZone'
import { Dialog } from '@/components/ui/Dialog'

const SqlShell = lazy(async () => ({ default: (await import('./SqlPreparationWorkspace')).SqlShell }))
const PipelineWorkspace = lazy(async () => ({ default: (await import('./pipeline/PipelineWorkspace')).PipelineWorkspace }))

type Editor =
  | { readonly kind: 'sql'; readonly inputs: NonEmptyArray<SqlPreparationInput>; readonly resume: SqlResume | null }
  | { readonly kind: 'pipeline'; readonly resume: PipelineResume }
type Stage =
  | { readonly kind: 'choose' }
  | { readonly kind: 'reading' }
  | { readonly kind: 'files'; readonly detail: string | null }
  | { readonly kind: 'failed'; readonly detail: string }
  | { readonly kind: 'editing'; readonly editor: Editor; readonly proposed: SelectedSource | null }

function editorFor(source: SelectedSource, inputs: NonEmptyArray<SqlPreparationInput>, route: 'sql' | 'pipeline'): Editor {
  const recipe = source.recipe
  switch (recipe.kind) {
    case 'uploaded-file': return route === 'sql' ? { kind: 'sql', inputs, resume: null }
      : { kind: 'pipeline', resume: { graph: initialGraph(inputs), inputs } }
    case 'sql-derived': return { kind: 'sql', inputs, resume: { statement: recipe.statement, outputView: recipe.outputView } }
    case 'pipeline-derived': return { kind: 'pipeline', resume: { graph: recipe.graph, inputs } }
    default: return assertNever(recipe)
  }
}

/** Editing owns a proposal, never the accepted source or its results. */
export function SourceEditor({ source, onView, onCancel, onAccept }: {
  readonly source: SelectedSource
  readonly onView: (kind: 'choosing' | 'editing') => void
  readonly onCancel: () => void
  readonly onAccept: (source: SelectedSource) => void
}) {
  const [store] = useState(() => createStore<{ readonly stage: Stage }>(() => ({ stage: { kind: 'choose' } })))
  const stage = useStore(store, state => state.stage)
  const setStage = (stage: Stage) => {
    store.setState({ stage })
    onView(stage.kind === 'editing' ? 'editing' : 'choosing')
  }

  const open = async (route: 'sql' | 'pipeline', files?: readonly File[]) => {
    setStage({ kind: 'reading' })
    try {
      const recipe = source.recipe
      const remembered = recipe.kind === 'uploaded-file' ? null : inputsFromMemory(recipe.inputs)
      if (recipe.kind !== 'uploaded-file' && files === undefined && remembered === null) {
        setStage({ kind: 'files', detail: null })
        return
      }
      const data = await import('@/data/sqlPreparation')
      const offered = remembered !== null && files === undefined && isNonEmpty(remembered)
        ? { ok: true as const, value: remembered }
        : await data.prepareSqlInputs(files ?? [source.file])
      if (!offered.ok) { setStage({ kind: 'failed', detail: data.describeSqlPreparationProblem(offered.error) }); return }
      const inputs = recipe.kind === 'uploaded-file' ? ok(offered.value) : matchInputFiles(recipe.inputs, offered.value)
      if (!inputs.ok) {
        setStage({ kind: 'files', detail: inputs.error })
        return
      }
      setStage({ kind: 'editing', editor: editorFor(source, inputs.value, route), proposed: null })
    } catch (error) {
      setStage({ kind: 'failed', detail: error instanceof Error ? error.message : String(error) })
    }
  }
  const propose = (proposed: SelectedSource | null) => store.setState(state => state.stage.kind === 'editing'
    ? { stage: { ...state.stage, proposed } } : state)

  if (stage.kind !== 'editing') return <Dialog open dismissible title="Edit data" onClose={onCancel}>
    <div className="flex flex-col gap-4">
    {stage.kind === 'choose' && <>
      <p className="m-0 text-body text-muted">Your prepared dataset and results stay unchanged until you accept a replacement.</p>
    </>}
    {stage.kind === 'reading' && <p role="status" className="m-0 text-body text-muted">Opening editor…</p>}
    {stage.kind === 'files' && <>
      {stage.detail !== null && <Alert tone="danger">{stage.detail}</Alert>}
      <DataDropZone multiple invitation="Choose the original input files." action="Choose input files" onFiles={files => void open(source.recipe.kind === 'sql-derived' ? 'sql' : 'pipeline', files)} />
    </>}
    {stage.kind === 'failed' && <><Alert tone="danger">{stage.detail}</Alert><button type="button" className={button('outline')} onClick={() => setStage({ kind: 'choose' })}>Try again</button></>}
    </div>
    <div className="mt-4 flex flex-wrap justify-center gap-2">
      {stage.kind === 'choose' && <>
        {source.recipe.kind === 'uploaded-file' ? <>
          <button type="button" className={button('outline', 'max-sm:flex-1')} onClick={() => void open('sql')}>Prepare with SQL</button>
          <button type="button" className={button('outline', 'max-sm:flex-1')} onClick={() => void open('pipeline')}>Build a pipeline</button>
        </> : <button type="button" className={button('signal')} onClick={() => void open(source.recipe.kind === 'sql-derived' ? 'sql' : 'pipeline')}>
          {source.recipe.kind === 'sql-derived' ? 'Edit SQL' : 'Edit pipeline'}
        </button>}
    </>}
    </div>
  </Dialog>

  return <>
      <Suspense fallback={<p role="status">Loading editor…</p>}>
        {stage.editor.kind === 'sql'
          ? <section className="rise w-full max-w-6xl" aria-label="Prepare with SQL"><SqlShell inputs={stage.editor.inputs} resume={stage.editor.resume} onPrepared={propose} onCancelEditing={onCancel} clearLabel="Back to editor choice" onCleared={() => setStage({ kind: 'choose' })} /></section>
          : <PipelineWorkspace resume={stage.editor.resume} onPrepared={propose} onCancelEditing={onCancel} />}
      </Suspense>
      {stage.proposed !== null && <ConfirmDialog open title="Replace dataset?" message="Accepting this result replaces the current source and clears its prepared dataset and analysis runs. The original input files are unchanged." confirmLabel="Replace dataset" danger onClose={() => propose(null)} onConfirm={() => { if (stage.proposed !== null) onAccept(stage.proposed) }} />}
  </>
}
