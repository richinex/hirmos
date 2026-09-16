import { createContext, useContext, useLayoutEffect, useState, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { cancelAnalysisRuns } from './client'
import { createJobs, IDLE, type Progress } from './jobs'
import type { PreparedDatasetVersionId } from '@/domain/preprocessing'
import type { DatasetProfileId } from '@/domain/dataset'

const Context = createContext<ReturnType<typeof createJobs> | null>(null)

export function JobsProvider({ children, prepared, profile = null }: { readonly children: ReactNode; readonly prepared: PreparedDatasetVersionId | null; readonly profile?: DatasetProfileId | null }) {
  const [jobs] = useState(() => createJobs(cancelAnalysisRuns))
  useLayoutEffect(() => {
    jobs.activate()
    return () => jobs.dispose()
  }, [jobs, prepared, profile])
  return <Context.Provider value={jobs}>{children}</Context.Provider>
}

export function useJob(key: string) {
  const jobs = useContext(Context)
  if (jobs === null) throw new Error('JobsProvider is required for analysis runs.')
  const job = useStore(jobs.store, state => state.jobs[key] ?? IDLE)
  const blocked = useStore(jobs.store, state => Object.entries(state.jobs).some(([name, job]) => name !== key && job.kind === 'running'))
  return {
    job, blocked,
    start: (action: 'analysis' | 'checks', stage: string) => jobs.start(key, action, stage),
    current: (id: string) => jobs.current(key, id),
    progress: (id: string, stage: string, progress: Progress | null = null) => jobs.progress(key, id, stage, progress),
    finish: (id: string) => jobs.finish(key, id),
    fail: (id: string, detail: string) => jobs.fail(key, id, detail),
    cancel: () => jobs.cancel(key),
  }
}
