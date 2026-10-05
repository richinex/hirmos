import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

/**
 * A term and value list: two columns when the panel container is at least md wide, stacked below
 * it. Values may wrap anywhere, so an id or a long name never widens the panel.
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
      <dt className="text-label text-muted @max-md/panel:mt-2 @max-md/panel:first:mt-0">{term}</dt>
      <dd className="m-0 min-w-0 text-ink [overflow-wrap:anywhere]">{children}</dd>
    </>
  )
}
