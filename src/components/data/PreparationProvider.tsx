import { createContext, useContext, useLayoutEffect, useState, type ReactNode } from 'react'
import { createStore, useStore } from 'zustand'
import { useWorkflow } from '@/components/WorkflowProvider'
import type { PipelineResume, SqlResume } from '@/domain/workflow'
import type { NonEmptyArray } from '@/domain/dop'
import type { SqlPreparationInput } from '@/domain/sqlPreparation'
import type { PipelineController } from './pipeline/pipelineSession'
import type { SqlController } from './sqlSession'

type Entry =
  | { readonly kind: 'none' }
  | { readonly kind: 'pipeline'; readonly resume: PipelineResume | null; readonly controller: PipelineController }
  | { readonly kind: 'sql'; readonly inputs: NonEmptyArray<SqlPreparationInput>; readonly resume: SqlResume | null; readonly controller: SqlController }

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
    selectSql: (inputs: NonEmptyArray<SqlPreparationInput>, resume: SqlResume | null, create: () => SqlController) => {
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

function ProjectPreparation({ children }: { readonly children: ReactNode }) {
  const [session] = useState(createPreparationSession)
  const discard = useWorkflow(state => state.workflow.kind === 'awaiting-data')
  useLayoutEffect(() => { session.activate(); return session.dispose }, [session])
  useLayoutEffect(() => { if (discard) session.clear() }, [session, discard])
  return <Context.Provider value={session}>{children}</Context.Provider>
}

export function PreparationProvider({ children }: { readonly children: ReactNode }) {
  const project = useWorkflow(state => 'project' in state.workflow ? state.workflow.project.id : '')
  return <ProjectPreparation key={project}>{children}</ProjectPreparation>
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

export function useSqlSession(inputs: NonEmptyArray<SqlPreparationInput>, resume: SqlResume | null, create: () => SqlController) {
  const session = usePreparationSession()
  const entry = useStore(session.store, state => state.entry)
  useLayoutEffect(() => { session.selectSql(inputs, resume, create) }, [session, inputs, resume, create])
  return entry.kind === 'sql' && entry.inputs === inputs && entry.resume === resume ? entry.controller : null
}
