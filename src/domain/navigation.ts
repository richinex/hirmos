import { z } from 'zod'
import { err, ok, type Result } from './dop'

export const CHAPTER_IDS = [
  'projects',
  'data',
  'discovery',
  'dag',
  'study',
  'estimation',
  'sensitivity',
  'counterfactual',
  'results',
] as const

export type ChapterId = (typeof CHAPTER_IDS)[number]

const chapterSchema = z.enum(CHAPTER_IDS)

/** Every screen the shell can show. The public root is the landing page; workbench routes live under `/app`. */
export type Route =
  | { readonly kind: 'default' }
  | { readonly kind: 'chapter'; readonly chapter: ChapterId }

export type RouteProblem =
  | { readonly kind: 'unknown-path'; readonly path: string }
  | { readonly kind: 'invalid-chapter-query'; readonly value: string }

export const chapterPath = (chapter: ChapterId): string => `/app/${chapter}`

export const routePath = (route: Route): string => route.kind === 'default' ? '/app' : chapterPath(route.chapter)

/**
 * Parse a workbench location. The path form (`/app/dag`) is canonical; `/app?chapter=dag`
 * still parses so early Hirmos links keep working, and the shell rewrites it to the path.
 */
export function parseRoute(pathname: string, search: string): Result<Route, RouteProblem> {
  const segments = pathname.split('/').filter((segment) => segment.length > 0)
  if (segments.at(0) !== 'app' || segments.length > 2) return err({ kind: 'unknown-path', path: pathname })
  const segment = segments.at(1)
  if (segment !== undefined) {
    const parsed = chapterSchema.safeParse(segment)
    return parsed.success
      ? ok({ kind: 'chapter', chapter: parsed.data })
      : err({ kind: 'unknown-path', path: pathname })
  }
  const query = new URLSearchParams(search).get('chapter')
  if (query === null) return ok({ kind: 'default' })
  const parsed = chapterSchema.safeParse(query)
  return parsed.success
    ? ok({ kind: 'chapter', chapter: parsed.data })
    : err({ kind: 'invalid-chapter-query', value: query })
}

/** True when the location already has the canonical form for its route. */
export const isCanonicalLocation = (pathname: string, search: string, route: Route): boolean =>
  pathname === routePath(route) && new URLSearchParams(search).get('chapter') === null

export function describeRouteProblem(problem: RouteProblem): string {
  switch (problem.kind) {
    case 'unknown-path': return `There is no page at “${problem.path}”`
    case 'invalid-chapter-query': return `Unknown chapter “${problem.value}” in the URL`
    default: {
      const exhaustive: never = problem
      return exhaustive
    }
  }
}
