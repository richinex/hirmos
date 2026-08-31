import { useSyncExternalStore } from 'react'
import { ThinkingOrb, type OrbState } from 'thinking-orbs'
import { cn } from '@/lib/utils'

const subscribeTheme = (notify: () => void): (() => void) => {
  const observer = new MutationObserver(notify)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] })
  return () => observer.disconnect()
}

const isDarkTheme = (): boolean => document.documentElement.dataset.theme === 'dark'

/**
 * The one way to mount an orb: inline size, theme pinned from `<html data-theme>`. The package's
 * `auto` only recognises `data-theme="dark|light"`, so the sketchbook themes would fall through to
 * the OS preference; every theme except dark paints a light ground, so all of them take dark ink.
 * An orb marks work running right now — callers unmount it when the run settles rather than pausing.
 */
export function Orb({ state, className, 'aria-label': ariaLabel }: {
  readonly state: OrbState
  readonly className?: string
  readonly 'aria-label': string
}) {
  const dark = useSyncExternalStore(subscribeTheme, isDarkTheme, () => true)
  return <ThinkingOrb state={state} size={20} theme={dark ? 'dark' : 'light'} aria-label={ariaLabel} className={cn('shrink-0', className)} />
}
