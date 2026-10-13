import { createContext, useContext, useState, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { createWorkflowStore } from '@/domain/workflowStore'
import { AssistantToolsProvider } from '@/assistant/applicationTools'

const Context = createContext<ReturnType<typeof createWorkflowStore> | null>(null)
type State = ReturnType<ReturnType<typeof createWorkflowStore>['getState']>

export function WorkflowProvider({ children }: { readonly children: ReactNode }) {
  const [store] = useState(() => createWorkflowStore())
  return (
    <Context.Provider value={store}>
      <AssistantToolsProvider>{children}</AssistantToolsProvider>
    </Context.Provider>
  )
}

export function useWorkflow<T>(selector: (state: State) => T): T {
  const store = useWorkflowStore()
  return useStore(store, selector)
}

export function useWorkflowStore() {
  const store = useContext(Context)
  if (store === null) throw new Error('WorkflowProvider is required for the workbench.')
  return store
}
