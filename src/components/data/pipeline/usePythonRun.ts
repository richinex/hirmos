import { useEffect, useState } from 'react'
import { useStore } from 'zustand'
import type { ActivePythonRun, PythonRuntimeState } from '@/data/pythonRuntime'
import { usePythonSession } from './PythonProvider'
import type { PipelineBlockId } from '@/domain/pipeline'

export const usePythonRuntime = (): PythonRuntimeState =>
  useStore(usePythonSession().store, (state) => state.runtime)

/** The script run in progress for this block, if any, with its elapsed time ticking once a second. */
export function usePythonRun(
  step: PipelineBlockId,
): { readonly run: ActivePythonRun; readonly elapsedMs: number } | null {
  const run = useStore(usePythonSession().store, (state) =>
    state.active?.step === step ? state.active : null,
  )
  const [now, setNow] = useState(() => Date.now())
  useEffect(() => {
    if (run === null) return
    setNow(Date.now())
    const handle = window.setInterval(() => setNow(Date.now()), 1000)
    return () => window.clearInterval(handle)
  }, [run])
  return run === null ? null : { run, elapsedMs: Math.max(0, now - run.startedAt) }
}
