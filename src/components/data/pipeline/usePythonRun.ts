import { useEffect, useState, useSyncExternalStore } from 'react'
import { activePythonRun, pythonRuntimeState, subscribePythonRuntime, type ActivePythonRun, type PythonRuntimeState } from '@/data/pythonRuntime'
import type { PipelineBlockId } from '@/domain/pipeline'

export const usePythonRuntime = (): PythonRuntimeState => useSyncExternalStore(subscribePythonRuntime, pythonRuntimeState)

/** The script run in progress for this block, if any, with its elapsed time ticking once a second. */
export function usePythonRun(step: PipelineBlockId): { readonly run: ActivePythonRun; readonly elapsedMs: number } | null {
  const active = useSyncExternalStore(subscribePythonRuntime, activePythonRun)
  const run = active !== null && active.step === step ? active : null
  const [now, setNow] = useState(() => Date.now())
  useEffect(() => {
    if (run === null) return
    setNow(Date.now())
    const handle = window.setInterval(() => setNow(Date.now()), 1000)
    return () => window.clearInterval(handle)
  }, [run])
  return run === null ? null : { run, elapsedMs: Math.max(0, now - run.startedAt) }
}
