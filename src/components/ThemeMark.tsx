import { cn } from '@/lib/utils'
import type { ThemeChoice } from '@/components/ui/useTheme'

/** One ring whose fill carries the state: empty for light, half for system, solid for dark. */
export function ThemeMark({ choice, size = 16, className }: {
  readonly choice: ThemeChoice
  readonly size?: number
  readonly className?: string
}) {
  const radius = 6.4
  return (
    <svg aria-hidden viewBox="0 0 16 16" width={size} height={size}
      className={cn('shrink-0 select-none', className)} fill="none" stroke="currentColor">
      <circle cx="8" cy="8" r={radius} strokeWidth={1.2} />
      {choice === 'system' ? (
        <path d={`M8 ${8 - radius} A${radius} ${radius} 0 0 1 8 ${8 + radius} Z`} fill="currentColor" stroke="none" />
      ) : null}
      {choice === 'dark' ? <circle cx="8" cy="8" r={radius} fill="currentColor" stroke="none" /> : null}
    </svg>
  )
}
