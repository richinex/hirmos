import { useWorkflowStore } from '@/components/WorkflowProvider'
import type { useJob } from '@/analysis/JobsProvider'
import type { Workflow } from '@/domain/workflow'
import { useAnalysisRunTool } from './applicationTools'
import type { RunInspection } from './analysisRun'

interface SavedRun {
  readonly id: string
}

export function useRecordedAnalysis<R extends SavedRun>(
  options: ({ readonly prepared: string } | { readonly profile: string }) & {
    readonly key: string
    readonly binding: string
    readonly inspection: RunInspection
    readonly session: ReturnType<typeof useJob>
    readonly unavailable?: string | null
    readonly execute: () => Promise<void>
    readonly records: (workflow: Extract<Workflow, { kind: 'profiled' }>) => readonly R[]
    readonly summarise: (run: R) => string
  },
) {
  const store = useWorkflowStore()
  const read = () => {
    const workflow = store.getState().workflow
    return workflow.kind === 'profiled' &&
      ('prepared' in options
        ? workflow.prepared?.id === options.prepared
        : workflow.profile.id === options.profile)
      ? options.records(workflow)
      : []
  }
  useAnalysisRunTool(options.key, options.binding, options.inspection, async (signal) => {
    if (signal.aborted) return { kind: 'cancelled' }
    if (options.unavailable) return { kind: 'refused', reason: options.unavailable }
    if (!options.session.canStart())
      return { kind: 'refused', reason: 'Another analysis is running.' }
    const previous = new Set(read().map((run) => run.id))
    const pending = options.execute()
    const started = options.session.read()
    const cancel = () => {
      if (started.kind === 'running' && options.session.current(started.id))
        options.session.cancel()
    }
    signal.addEventListener('abort', cancel, { once: true })
    try {
      await pending
      if (signal.aborted) return { kind: 'cancelled' }
      const run = read().find((run) => !previous.has(run.id))
      if (run)
        return {
          kind: 'completed',
          runId: run.id,
          method: options.inspection.method,
          summary: options.summarise(run),
        }
      const job = options.session.read()
      if (job.kind === 'cancelled') return { kind: 'cancelled' }
      return job.kind === 'failed'
        ? { kind: 'failed', reason: job.detail }
        : {
            kind: 'refused',
            reason:
              'No result was saved. Review the requirements and inputs in the analysis panel.',
          }
    } finally {
      signal.removeEventListener('abort', cancel)
    }
  })
}

export function runReceipt(
  run: SavedRun & { readonly kind: string; readonly createdAt: string },
): string {
  return JSON.stringify({
    kind: run.kind,
    createdAt: run.createdAt,
    note: 'This is a saved-run receipt, not a numerical interpretation. Inspect the result in the application.',
  })
}
