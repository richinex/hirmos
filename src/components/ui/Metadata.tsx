import { Children, type ReactNode } from 'react'
import { cn } from '@/lib/utils'

/** Short, independent facts wrap as units without decorative punctuation. */
export function Metadata({ children, className }: { readonly children: ReactNode; readonly className?: string }) {
  return <span className={cn('inline-flex max-w-full flex-wrap items-baseline gap-x-3 gap-y-1', className)}>
    {Children.toArray(children).map((item, index) => <span key={index} className="min-w-0 [overflow-wrap:anywhere]">{item}<span className="sr-only"> </span></span>)}
  </span>
}
