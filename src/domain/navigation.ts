import { z } from 'zod'
import { err, ok, type Result } from './dop'

export type ChapterId = 'projects' | 'data' | 'discovery' | 'dag' | 'study' | 'estimation' | 'robustness' | 'results'

export type ChapterSelection =
  | { readonly kind: 'default' }
  | { readonly kind: 'selected'; readonly chapter: ChapterId }

export type ChapterSelectionProblem = {
  readonly kind: 'invalid-chapter-query'
  readonly value: string
}

const chapterSchema = z.enum([
  'projects',
  'data',
  'discovery',
  'dag',
  'study',
  'estimation',
  'robustness',
  'results',
])

export function parseChapterSelection(search: string): Result<ChapterSelection, ChapterSelectionProblem> {
  const value = new URLSearchParams(search).get('chapter')
  if (value === null) return ok({ kind: 'default' })
  const parsed = chapterSchema.safeParse(value)
  return parsed.success
    ? ok({ kind: 'selected', chapter: parsed.data })
    : err({ kind: 'invalid-chapter-query', value })
}
