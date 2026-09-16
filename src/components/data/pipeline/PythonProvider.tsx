import { createContext, useContext, type ReactNode } from 'react'
import type { createPythonRuntime } from '@/data/pythonRuntime'

const Context = createContext<ReturnType<typeof createPythonRuntime> | null>(null)

export function PythonProvider({ children, runtime }: { readonly children: ReactNode; readonly runtime: ReturnType<typeof createPythonRuntime> }) {
  return <Context.Provider value={runtime}>{children}</Context.Provider>
}

export function usePythonSession() {
  const runtime = useContext(Context)
  if (runtime === null) throw new Error('PythonProvider is required for script blocks.')
  return runtime
}
