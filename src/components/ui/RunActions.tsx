import type { ReactNode } from 'react'
import type { OrbState } from 'thinking-orbs'
import { Orb } from './Orb'
import { button } from './recipes'
import { cn } from '@/lib/utils'

/**
 * The row under every run button: the button, then the orb and "Cancel run". The orb and cancel keep their
 * place while idle, invisible and out of the tab order, so starting or finishing a run never moves the row
 * or anything below it, at any label length or screen width. On a narrow screen the slot wraps under a long
 * label and stays reserved there.
 */
export function RunActions({
  running,
  onCancel,
  orb = 'solving',
  orbLabel,
  className,
  testId,
  children,
}: {
  readonly running: boolean
  readonly onCancel: () => void
  readonly orb?: OrbState
  /** What the orb announces while the run is in flight, such as "Estimator running". */
  readonly orbLabel: string
  readonly className?: string
  readonly testId?: string
  /** The run button, and any control that belongs beside it. */
  readonly children: ReactNode
}) {
  return (
    <div
      className={cn('flex min-w-0 flex-wrap items-center gap-3', className)}
      data-testid={testId}
    >
      {children}
      <span
        className={cn('inline-flex shrink-0 items-center gap-3', !running && 'invisible')}
        aria-hidden={running ? undefined : true}
      >
        <span className="inline-flex h-5 w-5 items-center">
          {running && <Orb state={orb} aria-label={orbLabel} />}
        </span>
        <button type="button" className={button('quiet')} disabled={!running} onClick={onCancel}>
          Cancel run
        </button>
      </span>
    </div>
  )
}
