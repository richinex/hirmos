import { createStore } from 'zustand/vanilla'

export type Action = 'analysis' | 'checks'
export interface Progress {
  readonly completed: number
  readonly total: number
}
export type Job =
  | { readonly kind: 'idle' }
  | {
      readonly kind: 'running'
      readonly id: string
      readonly action: Action
      readonly stage: string
      readonly progress: Progress | null
    }
  | { readonly kind: 'failed'; readonly action: Action; readonly detail: string }
  | { readonly kind: 'cancelled'; readonly action: Action }

export const IDLE: Job = { kind: 'idle' }
interface Jobs {
  readonly jobs: Readonly<Record<string, Job>>
}

/** One project session owns these jobs; views only subscribe to them. */
export function createJobs(stop: () => void) {
  const store = createStore<Jobs>(() => ({ jobs: {} }))
  const active = () =>
    Object.keys(store.getState().jobs).find((key) => store.getState().jobs[key]?.kind === 'running')
  let available = true
  const current = (key: string, id: string) => {
    const job = store.getState().jobs[key]
    return available && job?.kind === 'running' && job.id === id
  }
  // Progress arrives once per resample: show the first at once, then at most one more a frame.
  const held = new Map<string, Job>()
  let frame: number | null = null
  const publish = () => {
    frame = null
    if (held.size === 0) return
    const reports = [...held]
    held.clear()
    store.setState((state) => {
      let jobs = state.jobs
      for (const [key, job] of reports) {
        // Dropped when the run finished, failed or was cancelled while held.
        if (jobs[key]?.kind !== 'running') continue
        jobs = { ...jobs, [key]: job }
      }
      return { jobs }
    })
  }
  const set = (key: string, job: Job) => {
    held.delete(key)
    store.setState((state) => ({ jobs: { ...state.jobs, [key]: job } }))
  }
  const hold = (key: string, job: Job) => {
    if (frame !== null) {
      held.set(key, job)
      return
    }
    frame = requestAnimationFrame(publish)
    set(key, job)
  }
  const cancel = (key: string) => {
    const job = store.getState().jobs[key]
    if (job?.kind !== 'running') return
    set(key, { kind: 'cancelled', action: job.action })
    stop()
  }
  return {
    store,
    current,
    start(key: string, action: Action, stage: string): string | null {
      if (!available || active() !== undefined) return null
      const id = crypto.randomUUID()
      set(key, { kind: 'running', id, action, stage, progress: null })
      return id
    },
    progress(key: string, id: string, stage: string, progress: Progress | null = null) {
      const job = store.getState().jobs[key]
      if (!current(key, id) || job?.kind !== 'running') return
      hold(key, { ...job, stage, progress })
    },
    finish(key: string, id: string) {
      if (current(key, id)) set(key, IDLE)
    },
    fail(key: string, id: string, detail: string) {
      const job = store.getState().jobs[key]
      if (!current(key, id) || job?.kind !== 'running') return
      set(key, { kind: 'failed', action: job.action, detail })
    },
    cancel,
    activate() {
      held.clear()
      if (!available) store.setState({ jobs: {} })
      available = true
    },
    dispose() {
      available = false
      const key = active()
      if (key !== undefined) cancel(key)
    },
  }
}
