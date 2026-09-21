import { createContext, useContext, useLayoutEffect, useState, type ReactNode } from 'react'
import { createStore, useStore } from 'zustand'
import { useWorkflow } from '@/components/WorkflowProvider'
import type { PipelineResume, SqlResume } from '@/domain/workflow'
import type { SqlPreparationInput } from '@/domain/sqlPreparation'
import type { PipelineController } from './pipeline/pipelineSession'
import type { SqlController } from './sqlSession'

type Entry =
  | { readonly kind: 'none' }
  | { readonly kind: 'pipeline'; readonly resume: PipelineResume | null; readonly controller: PipelineController }
  | { readonly kind: 'sql'; readonly inputs: readonly SqlPreparationInput[]; readonly resume: SqlResume | null; readonly controller: SqlController }

export function createPreparationSession() {
  const store = createStore<{ readonly entry: Entry }>(() => ({ entry: { kind: 'none' } }))
  const dispose = () => { const { entry } = store.getState(); if (entry.kind !== 'none') entry.controller.dispose() }
  const clear = () => { dispose(); store.setState({ entry: { kind: 'none' } }) }
  return {
    store, clear,
    selectPipeline: (resume: PipelineResume | null, create: () => PipelineController) => {
      const current = store.getState().entry
      if (current.kind === 'pipeline' && current.resume === resume) { current.controller.open(); return }
      clear()
      const controller = create()
      controller.open()
      store.setState({ entry: { kind: 'pipeline', resume, controller } })
    },
    selectSql: (inputs: readonly SqlPreparationInput[], resume: SqlResume | null, create: () => SqlController) => {
      const current = store.getState().entry
      if (current.kind === 'sql' && current.inputs === inputs && current.resume === resume) { current.controller.open(); return }
      clear()
      const controller = create()
      controller.open()
      store.setState({ entry: { kind: 'sql', inputs, resume, controller } })
    },
    activate: () => { const { entry } = store.getState(); if (entry.kind !== 'none') entry.controller.open() },
    dispose,
  }
}
const Context = createContext<ReturnType<typeof createPreparationSession> | null>(null)

type ProjectSession = { readonly project: string; readonly session: ReturnType<typeof createPreparationSession> }

/**
 * One preparation session per project. The project id and its session are held as one value, so a
 * session can never belong to another project; when the project changes the pair is replaced during
 * render, the old session is disposed by the effect's cleanup, and the children stay mounted, so state
 * held above the chapters (a notice, the open rail) survives opening a project.
 */
export function PreparationProvider({ children }: { readonly children: ReactNode }) {
  const project = useWorkflow(state => 'project' in state.workflow ? state.workflow.project.id : '')
  const [current, setCurrent] = useState<ProjectSession>(() => ({ project, session: createPreparationSession() }))
  // A state change during render makes React render again with the new pair before anything commits.
  if (current.project !== project) setCurrent({ project, session: createPreparationSession() })
  const session = current.session
  const discard = useWorkflow(state => state.workflow.kind === 'awaiting-data')
  useLayoutEffect(() => { session.activate(); return session.dispose }, [session])
  useLayoutEffect(() => { if (discard) session.clear() }, [session, discard])
  return <Context.Provider value={session}>{children}</Context.Provider>
}

export function usePreparationSession() {
  const session = useContext(Context)
  if (session === null) throw new Error('PreparationProvider is required for data preparation.')
  return session
}

export function usePipelineSession(resume: PipelineResume | null, create: () => PipelineController) {
  const session = usePreparationSession()
  const entry = useStore(session.store, state => state.entry)
  useLayoutEffect(() => { session.selectPipeline(resume, create) }, [session, resume, create])
  return entry.kind === 'pipeline' && entry.resume === resume ? entry.controller : null
}

export function useSqlSession(inputs: readonly SqlPreparationInput[], resume: SqlResume | null, create: () => SqlController) {
  const session = usePreparationSession()
  const entry = useStore(session.store, state => state.entry)
  useLayoutEffect(() => { session.selectSql(inputs, resume, create) }, [session, inputs, resume, create])
  return entry.kind === 'sql' && entry.inputs === inputs && entry.resume === resume ? entry.controller : null
}
