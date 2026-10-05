import type { ReactNode } from 'react'
import { DisclosureSummary } from './DisclosureSummary'

/** Technical metadata stays with the history entry that owns it. */
export function RunDetails({
  label,
  children,
}: {
  readonly label: string
  readonly children: ReactNode
}) {
  return (
    <details className="mt-3 text-body" data-testid="history-run-details">
      <DisclosureSummary icon="receipt_long" className="text-ink">
        {label}
      </DisclosureSummary>
      <div className="mt-3">{children}</div>
    </details>
  )
}
