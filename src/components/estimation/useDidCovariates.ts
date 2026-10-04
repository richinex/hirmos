import { useEffect, useState } from 'react'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { PanelBinding } from '@/domain/estimationDraft'
import { assessPanelInterventionLayout, type PanelInterventionLayout, type PanelLongMatrix } from '@/domain/panel'

type Inspection =
  | { readonly kind: 'pending' }
  | { readonly kind: 'unavailable' }
  | { readonly kind: 'ready'; readonly matrix: PanelLongMatrix; readonly layout: PanelInterventionLayout }

/** One keyed read for the candidate list. Selected covariates are checked again on the run's own matrix. */
export function useDidCovariates(file: File, profile: DatasetProfile, binding: PanelBinding | null, candidates: readonly { readonly id: ColumnId }[]): Inspection {
  const [record, setRecord] = useState<{ readonly file: File; readonly profile: DatasetProfile; readonly binding: PanelBinding; readonly candidates: typeof candidates; readonly inspection: Inspection } | null>(null)
  useEffect(() => {
    if (binding === null) return
    let cancelled = false
    void (async () => {
      const { materializePanelInWorker } = await import('@/data/client')
      const result = await materializePanelInWorker(file, profile, { ...binding, covariates: candidates.map(column => column.id) })
      if (cancelled) return
      const layout = result.ok ? assessPanelInterventionLayout(result.value) : null
      const inspection: Inspection = result.ok && layout?.ok ? { kind: 'ready', matrix: result.value, layout: layout.value } : { kind: 'unavailable' }
      setRecord({ file, profile, binding, candidates, inspection })
    })()
    return () => { cancelled = true }
  }, [file, profile, binding, candidates])
  return record?.file === file && record.profile === profile && record.binding === binding && record.candidates === candidates ? record.inspection : { kind: 'pending' }
}
