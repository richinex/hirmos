import { useSyncExternalStore } from 'react'
import { parseRoute, type Route, type RouteProblem } from '@/domain/navigation'
import type { Result } from '@/domain/dop'

const ROUTE_EVENT = 'hirmos-route-change'

export interface Location {
  readonly pathname: string
  readonly search: string
}

const subscribe = (notify: () => void): (() => void) => {
  window.addEventListener('popstate', notify)
  window.addEventListener(ROUTE_EVENT, notify)
  return () => {
    window.removeEventListener('popstate', notify)
    window.removeEventListener(ROUTE_EVENT, notify)
  }
}

let snapshot: Location = { pathname: '/app', search: '' }
const readLocation = (): Location => {
  if (snapshot.pathname !== window.location.pathname || snapshot.search !== window.location.search) {
    snapshot = { pathname: window.location.pathname, search: window.location.search }
  }
  return snapshot
}
const serverLocation = (): Location => ({ pathname: '/app', search: '' })

/** The live location as a parsed route, with the raw location beside it for canonical-URL checks. */
export function useRoute(): { readonly location: Location; readonly route: Result<Route, RouteProblem> } {
  const location = useSyncExternalStore(subscribe, readLocation, serverLocation)
  return { location, route: parseRoute(location.pathname, location.search) }
}

const announce = () => window.dispatchEvent(new Event(ROUTE_EVENT))

/** Push a new history entry; the shell re-renders from the location store. */
export function navigate(path: string): void {
  if (window.location.pathname === path && window.location.search === '') return
  window.history.pushState(null, '', path)
  announce()
}

/** Rewrite the current entry without adding history; used for canonical redirects. */
export function replace(path: string): void {
  window.history.replaceState(null, '', path)
  announce()
}
