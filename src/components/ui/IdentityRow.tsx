import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

/** Identity first; related facts stay together when the row wraps. */
export function IdentityRow({
  name,
  children,
  className,
}: {
  readonly name: ReactNode
  readonly children: ReactNode
  readonly className?: string
}) {
  return (
    <div className={cn('flex min-w-0 flex-wrap items-baseline gap-x-5 gap-y-1', className)}>
      <div className="min-w-0 flex-[1_1_6rem] [overflow-wrap:anywhere]">{name}</div>
      <div className="flex max-w-full flex-wrap items-baseline gap-x-4 gap-y-1 text-label text-faint">
        {children}
      </div>
    </div>
  )
}
