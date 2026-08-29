import type { ReactNode } from 'react'
import { cn } from '@/lib/utils'

/** A region with nothing in it yet: one dashed outline and one sentence saying how to fill it. Not a refusal; nothing went wrong. */
export function EmptyState({ children, className }: { readonly children: ReactNode; readonly className?: string }) {
  return <p className={cn('rounded-xl border border-dashed border-line bg-panel p-4 text-body text-faint', className)}>{children}</p>
}
