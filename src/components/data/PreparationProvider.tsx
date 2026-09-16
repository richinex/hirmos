import { createContext, useContext, useLayoutEffect, useState, type ReactNode } from 'react'
import { createStore, useStore } from 'zustand'
import { useWorkflow } from '@/components/WorkflowProvider'
import type { PipelineResume } from '@/domain/workflow'
import type { PipelineController } from './pipeline/pipelineSession'

export function createPreparationSession() {
  const store = createStore<{ readonly pipeline: { readonly resume: PipelineResume | null; readonly controller: PipelineController } | null }>(() => ({ pipeline: null }))
  const clear = () => { store.getState().pipeline?.controller.dispose(); store.setState({ pipeline: null }) }
  return {
    store, clear,
    selectPipeline: (resume: PipelineResume | null, create: () => PipelineController) => {
      const current = store.getState().pipeline
      if (current !== null && current.resume === resume) { current.controller.open(); return }
      clear()
      const controller = create()
      controller.open()
      store.setState({ pipeline: { resume, controller } })
    },
    activate: () => store.getState().pipeline?.controller.open(),
    dispose: () => store.getState().pipeline?.controller.dispose(),
  }
}
const Context = createContext<ReturnType<typeof createPreparationSession> | null>(null)

function ProjectPreparation({ children }: { readonly children: ReactNode }) {
  const [session] = useState(createPreparationSession)
  const discard = useWorkflow(state => state.workflow.kind === 'awaiting-data' || state.workflow.kind === 'sql-inputs-chosen')
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
  const entry = useStore(session.store, state => state.pipeline)
  useLayoutEffect(() => { session.selectPipeline(resume, create) }, [session, resume, create])
  return entry?.resume === resume ? entry.controller : null
}
