import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { ProjectId } from './workflow'

export const CHAPTER_IDS = [
  'projects',
  'data',
  'time-series',
  'discovery',
  'dag',
  'study',
  'estimation',
  'sensitivity',
  'counterfactual',
  'root-cause',
  'survival',
  'results',
] as const

export type ChapterId = (typeof CHAPTER_IDS)[number]

const CHAPTER_SECTION_TITLES = [
  'Set up',
  'Time series',
  'Model',
  'Estimate',
  'Causal models',
  'Time to event',
  'Report',
] as const
type ChapterSectionTitle = (typeof CHAPTER_SECTION_TITLES)[number]

export interface ChapterMetadata {
  readonly name: string
  readonly shortName: string
  readonly icon: string
  readonly section: ChapterSectionTitle
}

/** One catalog owns the text, icon, and section for every route in `CHAPTER_IDS`. */
export const CHAPTER_METADATA = {
  projects: { name: 'Projects', shortName: 'Projects', icon: 'folder_open', section: 'Set up' },
  data: { name: 'Data studio', shortName: 'Data', icon: 'table_view', section: 'Set up' },
  'time-series': {
    name: 'Time-series analysis',
    shortName: 'Time series',
    icon: 'timeline',
    section: 'Time series',
  },
  discovery: { name: 'Discovery lab', shortName: 'Discovery', icon: 'schema', section: 'Model' },
  dag: { name: 'DAG workspace', shortName: 'DAG', icon: 'conversion_path', section: 'Model' },
  study: { name: 'Study design', shortName: 'Study', icon: 'experiment', section: 'Model' },
  estimation: {
    name: 'Estimation',
    shortName: 'Estimate',
    icon: 'query_stats',
    section: 'Estimate',
  },
  sensitivity: {
    name: 'Sensitivity',
    shortName: 'Sensitivity',
    icon: 'fact_check',
    section: 'Estimate',
  },
  counterfactual: {
    name: 'Counterfactuals',
    shortName: 'What if',
    icon: 'alt_route',
    section: 'Estimate',
  },
  'root-cause': {
    name: 'Causal model analysis',
    shortName: 'Causal models',
    icon: 'root_cause',
    section: 'Causal models',
  },
  survival: {
    name: 'Survival analysis',
    shortName: 'Survival',
    icon: 'vital_signs',
    section: 'Time to event',
  },
  results: { name: 'Results', shortName: 'Results', icon: 'monitoring', section: 'Report' },
} as const satisfies Readonly<Record<ChapterId, ChapterMetadata>>

/** Rail groups are derived from the catalog, so a chapter cannot drift into a second ordering table. */
export const CHAPTER_SECTIONS: readonly {
  readonly title: ChapterSectionTitle
  readonly chapters: readonly ChapterId[]
}[] = CHAPTER_SECTION_TITLES.map((title) => ({
  title,
  chapters: CHAPTER_IDS.filter((chapter) => CHAPTER_METADATA[chapter].section === title),
}))

/** Headings and navigation use the same chapter name. */
export const chapterLabel = (chapter: ChapterId): string => CHAPTER_METADATA[chapter].name

const chapterSchema = z.enum(CHAPTER_IDS)

/**
 * Every screen the shell can show. The public root is the landing page; workbench routes live under `/app`.
 * A chapter inside an open project names that project, so a reload can reopen it.
 */
export type Route =
  | { readonly kind: 'default' }
  | { readonly kind: 'chapter'; readonly chapter: ChapterId; readonly project: ProjectId | null }

export type RouteProblem =
  | { readonly kind: 'unknown-path'; readonly path: string }
  | { readonly kind: 'invalid-chapter-query'; readonly value: string }

/** The project list never names a project; every other chapter does while one is open. */
export const chapterPath = (chapter: ChapterId, project: ProjectId | null = null): string =>
  project === null || chapter === 'projects'
    ? `/app/${chapter}`
    : `/app/projects/${encodeURIComponent(project)}/${chapter}`

export const routePath = (route: Route): string =>
  route.kind === 'default' ? '/app' : chapterPath(route.chapter, route.project)

/** Project ids are generated UUIDs or example slugs; anything else in that position is not a project. */
const projectSegment = z.string().regex(/^[A-Za-z0-9-]{1,64}$/u)

/**
 * Parse a workbench location. Inside a project the canonical form is `/app/projects/<id>/dag`; with no
 * project open it is `/app/dag`. `/app?chapter=dag` still parses so early Hirmos links keep working, and the
 * shell rewrites both older forms to the canonical path.
 */
export function parseRoute(pathname: string, search: string): Result<Route, RouteProblem> {
  const segments = pathname.split('/').filter((segment) => segment.length > 0)
  if (segments.at(0) !== 'app') return err({ kind: 'unknown-path', path: pathname })
  // `/app/projects/<id>/<chapter>`: a chapter of an open project. The bare id opens its data chapter.
  if (segments.at(1) === 'projects' && segments.length > 2) {
    const id = projectSegment.safeParse(decodeURIComponent(segments[2]!))
    const chapter = chapterSchema.safeParse(segments.at(3) ?? 'data')
    if (segments.length > 4 || !id.success || !chapter.success || chapter.data === 'projects')
      return err({ kind: 'unknown-path', path: pathname })
    return ok({
      kind: 'chapter',
      chapter: chapter.data,
      project: brand<string, 'ProjectId'>(id.data),
    })
  }
  if (segments.length > 2) return err({ kind: 'unknown-path', path: pathname })
  const segment = segments.at(1)
  if (segment !== undefined) {
    const parsed = chapterSchema.safeParse(segment)
    return parsed.success
      ? ok({ kind: 'chapter', chapter: parsed.data, project: null })
      : err({ kind: 'unknown-path', path: pathname })
  }
  const query = new URLSearchParams(search).get('chapter')
  if (query === null) return ok({ kind: 'default' })
  const parsed = chapterSchema.safeParse(query)
  return parsed.success
    ? ok({ kind: 'chapter', chapter: parsed.data, project: null })
    : err({ kind: 'invalid-chapter-query', value: query })
}

/** True when the location already has the canonical form for its route. */
export const isCanonicalLocation = (pathname: string, search: string, route: Route): boolean =>
  pathname === routePath(route) && new URLSearchParams(search).get('chapter') === null

export function describeRouteProblem(problem: RouteProblem): string {
  switch (problem.kind) {
    case 'unknown-path':
      return `There is no page at “${problem.path}”`
    case 'invalid-chapter-query':
      return `Unknown section “${problem.value}” in the URL`
    default: {
      const exhaustive: never = problem
      return exhaustive
    }
  }
}
