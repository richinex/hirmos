import { useEffect } from 'react'
import type { RunActivity } from '@/domain/activity'

/** Reports a panel's run to the shell while it is in flight. JobsProvider clears it when the run stops, so it survives the panel unmounting. */
export function useRunActivity(
  report: ((activity: RunActivity | null) => void) | undefined,
  activity: RunActivity | null,
): void {
  const label = activity?.label ?? null
  const progress = activity?.progress ?? null
  useEffect(() => {
    report?.(label === null ? null : { label, progress })
  }, [report, label, progress])
}
