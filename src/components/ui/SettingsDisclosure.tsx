import type { ReactNode } from 'react'
import { Icon } from '@/components/Icon'
import { cn } from '@/lib/utils'
import { settingsStack, well } from './recipes'

export interface SettingsSummaryItem {
  readonly icon: string
  readonly text: string
}

export function SettingsDisclosure({
  title,
  items,
  children,
  defaultOpen = false,
  className,
}: {
  readonly title: string
  readonly items: readonly SettingsSummaryItem[]
  readonly children: ReactNode
  readonly defaultOpen?: boolean
  readonly className?: string
}) {
  return (
    <details className={cn(well('group min-w-0 max-w-4xl'), className)} open={defaultOpen}>
      <summary className="flex cursor-pointer list-none items-center justify-between gap-4 px-4 py-3 [&::-webkit-details-marker]:hidden">
        <span className="min-w-0">
          <span className="block text-body font-medium text-ink">{title}</span>
          {/* The values while closed; once open the fields show them, so the summary steps aside. */}
          <span className="mt-1 flex flex-wrap gap-x-4 gap-y-1 text-body text-muted group-open:hidden">
            {items.map((item) => (
              <span
                key={item.icon + item.text}
                className="inline-flex items-center gap-1.5 whitespace-nowrap"
              >
                <Icon name={item.icon} size={16} className="text-faint" />
                <span>{item.text}</span>
              </span>
            ))}
          </span>
        </span>
        <span className="inline-flex shrink-0 items-center gap-1 text-body font-medium text-link">
          Edit
          <Icon
            name="expand_more"
            size={16}
            className="transition-transform group-open:rotate-180"
          />
        </span>
      </summary>
      <div className={cn(settingsStack, 'px-4 pb-4')}>{children}</div>
    </details>
  )
}
