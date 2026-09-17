import type { ComponentPropsWithoutRef } from 'react'
import { Icon } from '@/components/Icon'
import { cn } from '@/lib/utils'

/** Keep native disclosure behavior while using the shared show/hide icons. */
export function DisclosureSummary({ children, className, icon, ...props }: ComponentPropsWithoutRef<'summary'> & { readonly icon?: string }) {
  return <summary {...props} className={cn('disclosure-summary', className)}>
    <span className="disclosure-show"><Icon name={icon ?? 'visibility'} size={16} /></span>
    <span className="disclosure-hide"><Icon name={icon ?? 'visibility_off'} size={16} /></span>
    <span className="min-w-0">{children}</span>
  </summary>
}
