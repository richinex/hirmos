import { useWorkflow } from '@/components/WorkflowProvider'
import type { PreparedDatasetVersionId } from '@/domain/preprocessing'
import type { TimeSeriesDraft } from '@/domain/timeSeriesDraft'

export function useTimeSeriesDraft<T>(prepared: PreparedDatasetVersionId, select: (draft: TimeSeriesDraft) => T): T {
  return useWorkflow(state => {
    const draft = state.timeSeriesDraft
    if (draft?.prepared !== prepared) throw new Error('Time-series settings require the active prepared dataset.')
    return select(draft)
  })
}
