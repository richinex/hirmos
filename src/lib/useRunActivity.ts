import { useEffect } from 'react'
import type { RunActivity } from '@/domain/activity'

/** Reports a panel's run to the shell while it is in flight and clears it when the run ends or the panel unmounts. */
export function useRunActivity(report: ((activity: RunActivity | null) => void) | undefined, activity: RunActivity | null): void {
  const label = activity?.label ?? null
  const progress = activity?.progress ?? null
  useEffect(() => {
    report?.(label === null ? null : { label, progress })
  }, [report, label, progress])
  useEffect(() => () => report?.(null), [report])
}
