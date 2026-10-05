import { useMemo } from 'react'
import type { PanelBinding, PanelPreflightJob } from '@/domain/estimationDraft'
import { samePanelBinding } from '@/domain/estimationDraft'
import { describePanelDataProblem, type PanelLongMatrix } from '@/domain/panel'
import { sunAbrahamCatalog, type SunAbrahamCatalog } from '@/domain/sunAbraham'

export type SunAbrahamPanelJob =
  | { readonly kind: 'loading' }
  | { readonly kind: 'failed'; readonly detail: string }
  | {
      readonly kind: 'ready'
      readonly matrix: PanelLongMatrix
      readonly catalog: SunAbrahamCatalog
    }

/** Reuse the panel preflight's keyed data even when simultaneous DiD is unavailable. */
export function useSunAbrahamPanel(
  job: PanelPreflightJob,
  binding: PanelBinding | null,
): SunAbrahamPanelJob {
  return useMemo(() => {
    if (
      binding === null ||
      job.kind === 'not-required' ||
      !samePanelBinding(job.binding, binding) ||
      job.kind === 'loading'
    )
      return { kind: 'loading' }
    if (job.kind === 'refused')
      return {
        kind: 'failed',
        detail:
          job.problem.kind === 'panel-data'
            ? describePanelDataProblem(job.problem.problem)
            : 'The panel comparison could not be read.',
      }
    const catalog = sunAbrahamCatalog(job.matrix)
    return catalog.ok
      ? { kind: 'ready', matrix: job.matrix, catalog: catalog.value }
      : { kind: 'failed', detail: catalog.error }
  }, [job, binding])
}
