import { useEffect, useState } from 'react'
import { useWorkflow } from '@/components/WorkflowProvider'
import {
  loadPreparationDraft,
  savePreparationDraft,
  removePreparationDraft,
  type PreparationDraft,
} from '@/data/preparationDraftStore'
import { reportStorageFailure } from '@/data/storageHealth'
import { button } from '@/components/ui/recipes'
const inspected = new WeakSet<object>()

export function usePreparationDraft(
  owner: object,
  kind: PreparationDraft['kind'],
  basis: string,
  current: PreparationDraft | null,
  restore: (draft: PreparationDraft) => void,
) {
  const project = useWorkflow((state) =>
    'project' in state.workflow ? state.workflow.project.id : '',
  )
  const [recovery, setRecovery] = useState<{ key: string; draft: PreparationDraft | null } | null>(
    null,
  )
  const key = JSON.stringify([project, kind, basis])
  const failed = (error: unknown) =>
    reportStorageFailure({
      store: 'hirmos-preparation-drafts',
      reason: String(error),
      at: new Date().toISOString(),
    })
  useEffect(() => {
    let active = true
    if (inspected.has(owner)) {
      setRecovery({ key, draft: null })
      return
    }
    void loadPreparationDraft(project, kind, basis)
      .then((draft) => {
        if (active) {
          if (draft === null) inspected.add(owner)
          setRecovery({ key, draft })
        }
      })
      .catch(failed)
    return () => {
      active = false
    }
  }, [project, kind, basis, key, owner])
  useEffect(() => {
    if (recovery?.key !== key || recovery.draft !== null || current === null) return
    void savePreparationDraft(project, kind, basis, current).catch(failed)
  }, [project, kind, basis, current, recovery, key])
  if (recovery?.key !== key || recovery.draft === null) return null
  return (
    <div role="status" className="m-3 flex flex-wrap items-center gap-3 text-label">
      <span>
        A saved {kind === 'sql' ? 'SQL' : 'pipeline'} draft is available. Restoring it does not
        replace your dataset.
      </span>
      <button
        type="button"
        className={button('outline')}
        onClick={() => {
          restore(recovery.draft!)
          inspected.add(owner)
          setRecovery({ key, draft: null })
        }}
      >
        Restore draft
      </button>
      <button
        type="button"
        className={button('quiet')}
        onClick={() => {
          inspected.add(owner)
          void removePreparationDraft(project, kind, basis).catch(failed)
          setRecovery({ key, draft: null })
        }}
      >
        Keep current draft
      </button>
    </div>
  )
}
