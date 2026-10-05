import { useSyncExternalStore } from 'react'

/**
 * The live theme, read off `<html data-theme>` rather than passed in.
 *
 * One MutationObserver serves the whole app; useSyncExternalStore reads the attribute at render, so there is
 * no redundant second render on mount, and the observer lives only while something subscribes.
 */
const subscribers = new Set<() => void>()
let observer: MutationObserver | null = null

function subscribe(notify: () => void): () => void {
  subscribers.add(notify)
  if (!observer) {
    observer = new MutationObserver(() => {
      for (const fn of subscribers) fn()
    })
    observer.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme'],
    })
  }
  return () => {
    subscribers.delete(notify)
    if (subscribers.size === 0) {
      observer?.disconnect()
      observer = null
    }
  }
}

const getSnapshot = (): string => document.documentElement.dataset.theme ?? 'light'
const getServerSnapshot = (): string => 'light'

export function useDocTheme(): string {
  return useSyncExternalStore(subscribe, getSnapshot, getServerSnapshot)
}
