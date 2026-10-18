import { useMemo } from 'react'
import { Icon } from '@/components/Icon'
import { CHAPTER_SECTIONS, type ChapterId } from '@/domain/navigation'
import { fontFor, textWidth, useTextMetricsVersion } from '@/lib/textMetrics'
import { useElementWidth } from '@/lib/useElementWidth'
import { cn } from '@/lib/utils'
import { BusyBar, statusGlyph, type ChapterEntry, type ChapterStatus } from './ChapterNav'

const RESTING_TONE: Record<ChapterStatus, string> = {
  'not-started': '',
  locked: 'text-tab-ink',
  'in-progress': 'text-info',
  done: 'text-ok',
  refused: 'text-danger',
}

const ORDER: readonly ChapterId[] = CHAPTER_SECTIONS.flatMap((section) => section.chapters)

/** Horizontal room around the words, in px, matching the tab classes below. */
const ROOM = { resting: 24, open: 16 + 40 + 18 + 6, glyph: 6 + 12, gap: 4 } as const

/** The width the tabs take with every name shown: the open tab in the heading face, the rest in the body face. */
function namedWidth(chapters: readonly ChapterEntry[], active: ChapterId): number {
  const body = fontFor('body')
  const headingFace = getComputedStyle(document.documentElement).getPropertyValue('--font-heading').trim()
  const heading = body.replace(/px .+$/, `px ${headingFace}`)
  let width = ROOM.gap * Math.max(0, chapters.length - 1)
  for (const chapter of chapters) {
    const isActive = chapter.id === active
    width +=
      (isActive ? ROOM.open : ROOM.resting) +
      textWidth(chapter.shortName, isActive ? heading : body) +
      (statusGlyph(chapter.status) === null ? 0 : ROOM.glyph)
  }
  return width
}

/**
 * The chapters as folder tabs along the top of the desktop shell. The open chapter's tab takes the
 * folder's colour and joins the folder below it; the rest sit behind on the stage. Each tab keeps the
 * chapter's full name as its accessible name, so it answers to the same queries as the phone rail.
 */
export function ChapterTabs({
  chapters,
  active,
  onNavigate,
  onPrefetch,
}: {
  readonly chapters: readonly ChapterEntry[]
  readonly active: ChapterId
  readonly onNavigate: (chapter: ChapterId) => void
  readonly onPrefetch: (chapter: ChapterId) => void
}) {
  const entries = new Map(chapters.map((chapter) => [chapter.id, chapter]))
  const [row, width] = useElementWidth<HTMLElement>()
  const version = useTextMetricsVersion()
  const needed = useMemo(() => {
    void version
    return namedWidth(chapters, active)
  }, [chapters, active, version])
  const compact = width > 0 && needed > width

  return (
    <nav
      ref={row}
      aria-label="Workspace sections"
      data-compact={compact || undefined}
      className="chapter-tabs hidden min-w-0 flex-1 self-stretch md:flex"
    >
      <ol className="m-0 flex min-w-0 list-none items-end gap-1 p-0">
        {ORDER.map((id) => {
          const chapter = entries.get(id)
          if (chapter === undefined) return null
          const isActive = chapter.id === active
          const locked = chapter.status === 'locked'
          const glyph = statusGlyph(chapter.status)

          return (
            <li key={chapter.id} className={isActive ? 'flex shrink-0' : 'flex min-w-0'}>
              <button
                type="button"
                aria-current={isActive ? 'page' : undefined}
                aria-disabled={locked || undefined}
                aria-label={glyph === null ? chapter.name : `${chapter.name}, ${glyph.text}`}
                title={chapter.name}
                onClick={locked ? undefined : () => onNavigate(chapter.id)}
                onPointerEnter={locked ? undefined : () => onPrefetch(chapter.id)}
                onFocus={locked ? undefined : () => onPrefetch(chapter.id)}
                className={cn(
                  'chapter-tab relative flex min-w-0 items-center gap-1.5 whitespace-nowrap rounded-t-[14px] transition-colors duration-(--motion-fast) focus-visible:outline-offset-[-4px]',
                  isActive
                    ? 'mx-2 h-[38px] bg-rail px-5 font-heading text-body text-rail-ink'
                    : 'h-8 bg-tab px-3 text-body text-tab-ink',
                  !isActive && (locked ? 'cursor-not-allowed opacity-70' : 'hover:bg-tab-hover'),
                )}
              >
                <Icon name={chapter.icon} size={18} fill={isActive} className="chapter-tab-icon shrink-0" />
                <span className="chapter-tab-name truncate">{chapter.shortName}</span>
                {glyph === null ? null : (
                  <Icon
                    name={glyph.icon}
                    size={12}
                    className={cn('shrink-0', isActive ? glyph.tone : RESTING_TONE[chapter.status])}
                  />
                )}
                <BusyBar chapter={chapter.id} className="inset-x-3 bottom-1" />
              </button>
            </li>
          )
        })}
      </ol>
    </nav>
  )
}
