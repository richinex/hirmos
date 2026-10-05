import { useSyncExternalStore } from 'react'

const subscribe =
  (query: string) =>
  (notify: () => void): (() => void) => {
    const list = window.matchMedia(query)
    list.addEventListener('change', notify)
    return () => list.removeEventListener('change', notify)
  }

/** Live match state for one media query; the JS twin of a Tailwind breakpoint, for layout branching. */
export function useMediaQuery(query: string): boolean {
  return useSyncExternalStore(
    subscribe(query),
    () => window.matchMedia(query).matches,
    () => false,
  )
}

/** True below the md breakpoint: the phone layout. */
export const useIsMobile = (): boolean => useMediaQuery('(width < 48rem)')

/** True at the xl breakpoint and above, where the chapter nav opens by default. */
export const useIsWide = (): boolean => useMediaQuery('(width >= 80rem)')
