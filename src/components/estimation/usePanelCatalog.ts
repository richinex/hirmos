import { useEffect, useState } from 'react'
import {
  predictorSyntheticCatalog,
  type PredictorSyntheticCatalog,
} from '@/domain/predictorSyntheticControl'
import { describePanelDataProblem, type PanelKeyMatrix } from '@/domain/panel'
import type { DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
type Job =
  | { readonly kind: 'loading' }
  | { readonly kind: 'failed'; readonly detail: string }
  | {
      readonly kind: 'ready'
      readonly catalog: PredictorSyntheticCatalog
      readonly panelPeriods: PanelKeyMatrix['periods']
    }
export function usePanelCatalog(
  source: SelectedSource,
  profile: DatasetProfile,
  prepared: PreparedDatasetArtifact,
): Job {
  const [job, setJob] = useState<Job>({ kind: 'loading' })
  const unit = prepared.kind === 'prepared-panel' ? prepared.sampling.unitColumn : null
  const time = prepared.kind === 'prepared-panel' ? prepared.sampling.timeColumn : null
  useEffect(() => {
    let cancelled = false
    if (unit === null || time === null) {
      setJob({ kind: 'failed', detail: 'Prepare a long panel with unit and period keys.' })
      return
    }
    setJob({ kind: 'loading' })
    void (async () => {
      const { materializePanelKeysInWorker } = await import('@/data/client')
      const keys = await materializePanelKeysInWorker(source.file, profile, unit, time)
      if (cancelled) return
      setJob(
        keys.ok
          ? {
              kind: 'ready',
              catalog: predictorSyntheticCatalog(keys.value),
              panelPeriods: keys.value.periods,
            }
          : { kind: 'failed', detail: describePanelDataProblem(keys.error) },
      )
    })()
    return () => {
      cancelled = true
    }
  }, [source.file, profile, unit, time, prepared.id])
  return job
}
