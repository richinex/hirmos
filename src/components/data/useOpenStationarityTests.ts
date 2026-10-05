import { useWorkflow } from '@/components/WorkflowProvider'
import { chapterPath } from '@/domain/navigation'
import { navigate } from '@/lib/router'

/**
 * Takes the reader to the Data studio with its stationarity pane selected, where the missing tests
 * run. Null when the prepared dataset is not a time series, since only a series has the pane.
 */
export function useOpenStationarityTests(): (() => void) | null {
  const prepared = useWorkflow((state) =>
    state.workflow.kind === 'profiled' ? state.workflow.prepared : null,
  )
  const changeDiagnostic = useWorkflow((state) => state.changeDiagnostic)
  if (prepared === null || prepared.kind !== 'prepared-time-series') return null
  return () => {
    changeDiagnostic(prepared.id, { type: 'view', view: 'stationarity' })
    navigate(chapterPath('data'))
  }
}
