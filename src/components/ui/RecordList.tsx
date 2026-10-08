import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

/**
 * A term and value list: two columns when the panel container is at least md wide, stacked below
 * it. Stacked, each term is set in ink with space above it, so it reads as the heading of its group.
 * Values may wrap anywhere, so an id or a long name never widens the panel.
 */
export function RecordList({
  className,
  children,
}: {
  readonly className?: string
  readonly children: ReactNode
}) {
  return (
    <dl
      className={cn(
        'm-0 grid grid-cols-1 gap-y-1 @md/panel:grid-cols-[minmax(8rem,auto)_1fr] @md/panel:gap-x-4 @md/panel:gap-y-1.5',
        className,
      )}
    >
      {children}
    </dl>
  )
}

export function RecordRow({
  term,
  children,
}: {
  readonly term: string
  readonly children: ReactNode
}) {
  return (
    <>
      <dt className="mt-3.5 text-label font-medium text-ink first:mt-0 @md/panel:mt-0 @md/panel:font-normal @md/panel:text-muted">
        {term}
      </dt>
      <dd className="m-0 min-w-0 text-ink [overflow-wrap:anywhere]">{children}</dd>
    </>
  )
}
