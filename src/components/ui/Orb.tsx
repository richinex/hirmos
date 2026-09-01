import { useSyncExternalStore } from 'react'
import { ThinkingOrb, type OrbState } from 'thinking-orbs'
import { cn } from '@/lib/utils'

const subscribeTheme = (notify: () => void): (() => void) => {
  const observer = new MutationObserver(notify)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] })
  return () => observer.disconnect()
}

const isDarkTheme = (): boolean => {
  const theme = document.documentElement.dataset.theme
  return theme === 'dark' || theme === 'operational'
}

export function Orb({ state, className, 'aria-label': ariaLabel }: {
  readonly state: OrbState
  readonly className?: string
  readonly 'aria-label': string
}) {
  const dark = useSyncExternalStore(subscribeTheme, isDarkTheme, () => true)
  return <ThinkingOrb state={state} size={20} theme={dark ? 'dark' : 'light'} aria-label={ariaLabel} className={cn('shrink-0', className)} />
}
