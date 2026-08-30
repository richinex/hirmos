import { Icon } from '@/components/Icon'
import { Sheet } from '@/components/ui/Sheet'
import { label } from '@/components/ui/recipes'
import type { ChapterId } from '@/domain/navigation'
import { assertNever } from '@/domain/dop'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'

export type ChapterStatus = 'locked' | 'not-started' | 'in-progress' | 'done' | 'refused'

export interface ChapterEntry {
  readonly id: ChapterId
  readonly name: string
  readonly shortName: string
  readonly icon: string
  readonly status: ChapterStatus
  /** 0 to 1 while a run is in flight for this chapter, null otherwise; drawn as the rail's progress bar. */
  readonly busy?: number | null
}

const statusGlyph = (status: ChapterStatus): { readonly icon: string; readonly tone: string; readonly text: string } | null => {
  switch (status) {
    case 'not-started': return null
    case 'locked': return { icon: 'lock', tone: 'text-dim', text: 'not yet available' }
    case 'in-progress': return { icon: 'progress_activity', tone: 'text-[var(--color-info)]', text: 'in progress' }
    case 'done': return { icon: 'check', tone: 'text-ok', text: 'done' }
    case 'refused': return { icon: 'block', tone: 'text-danger', text: 'refused' }
    default: return assertNever(status)
  }
}

const number = (index: number): string => String(index + 1).padStart(2, '0')

/** The same bar the phone tab carries, at rail width: under the icon, because anything alongside crowds the glyph. */
function BusyBar({ fraction, className }: { readonly fraction: number; readonly className?: string }) {
  return (
    <span aria-hidden className={cn('bar-live absolute h-[2px] rounded-full bg-line', className)}>
      <span
        className="bar-live__fill block origin-left rounded-full bg-signal transition-transform duration-500 ease-[cubic-bezier(0.2,0.7,0.2,1)]"
        style={{ transform: `scaleX(${fraction})` }}
      />
    </span>
  )
}

interface ChapterNavProps {
  readonly chapters: readonly ChapterEntry[]
  readonly active: ChapterId
  readonly collapsed: boolean
  readonly onNavigate: (chapter: ChapterId) => void
  /** Phone only: whether the full list is open as a sheet; session state, not the persisted desktop preference. */
  readonly sheetOpen: boolean
  readonly onSheetClose: () => void
}

/**
 * The chapter list: 208px with numbers and names, or the 48px icon rail. On a phone there is no
 * rail, so the stage has the full width; the header button opens the list as a left-edge sheet,
 * the only place a focus trap belongs (docs/design-suggestions/sidebars-and-panels.md).
 * Gated chapters stay in the tab order.
 */
export function ChapterNav({ chapters, active, collapsed, onNavigate, sheetOpen, onSheetClose }: ChapterNavProps) {
  const phone = useIsMobile()
  return (
    <>
    <Sheet side="left" open={phone && sheetOpen} onClose={onSheetClose} title="Chapters">
      <ol className="m-0 flex list-none flex-col gap-1 p-0" aria-label="Chapter list">
        {chapters.map((chapter, index) => {
          const isActive = chapter.id === active
          const locked = chapter.status === 'locked'
          const glyph = statusGlyph(chapter.status)
          return (
            <li key={chapter.id}>
              <button
                type="button"
                aria-current={isActive ? 'page' : undefined}
                aria-disabled={locked || undefined}
                onClick={() => { if (!locked) { onNavigate(chapter.id); onSheetClose() } }}
                className={cn(
                  'flex w-full items-center gap-3 rounded-lg border px-3 py-2.5 text-left text-body',
                  isActive ? 'border-edge bg-raised text-ink' : 'border-transparent text-faint',
                  locked && 'cursor-not-allowed text-dim',
                )}
              >
                <Icon name={chapter.icon} size={18} fill={isActive} />
                <span className="min-w-0 flex-1 truncate"><span className="tabular-nums text-faint">{number(index)}</span> · {chapter.name}</span>
                {glyph && <Icon name={glyph.icon} size={14} className={glyph.tone} />}
                {glyph && <span className="sr-only">, {glyph.text}</span>}
              </button>
            </li>
          )
        })}
      </ol>
    </Sheet>
    {!phone && <nav
      aria-label="Workspace chapters"
      data-chapter-nav
      className={cn(
        'flex shrink-0 flex-col gap-1 overflow-hidden border-r border-line bg-panel py-2 transition-[width] duration-200 ease-[cubic-bezier(0.2,0.7,0.2,1)] motion-reduce:transition-none',
        collapsed ? 'w-12 items-center' : 'w-52 px-2',
      )}
    >
      <ol className="m-0 flex list-none flex-col gap-1 p-0">
        {chapters.map((chapter, index) => {
          const isActive = chapter.id === active
          const locked = chapter.status === 'locked'
          const glyph = statusGlyph(chapter.status)
          const busy = chapter.busy ?? null
          const name = `${number(index)} · ${chapter.name}`
          return (
            <li key={chapter.id}>
              <button
                type="button"
                aria-current={isActive ? 'page' : undefined}
                aria-disabled={locked || undefined}
                aria-label={collapsed ? chapter.name : undefined}
                title={collapsed ? (busy === null ? name : `${name}, ${Math.round(busy * 100)}% done`) : undefined}
                onClick={() => { if (!locked) onNavigate(chapter.id) }}
                className={cn(
                  'relative flex items-center gap-2 rounded-lg border text-left text-body transition-colors duration-150',
                  collapsed ? 'grid h-9 w-9 place-items-center pointer-coarse:h-10 pointer-coarse:w-10' : 'w-full px-2.5 py-2',
                  isActive ? 'border-edge bg-raised text-ink' : 'border-transparent text-faint hover:text-ink',
                  locked && 'cursor-not-allowed text-dim hover:text-dim',
                )}
              >
                <Icon name={chapter.icon} size={collapsed ? 19 : 16} fill={isActive} className={busy !== null ? 'text-signal' : undefined} />
                {!collapsed && (
                  <span className="min-w-0 flex-1 truncate">
                    <span className="tabular-nums text-faint">{number(index)}</span> · {chapter.name}
                  </span>
                )}
                {glyph && (
                  <Icon name={glyph.icon} size={12} className={cn(glyph.tone, collapsed && 'absolute right-0.5 top-0.5')} />
                )}
                {glyph && <span className="sr-only">, {glyph.text}</span>}
                {busy !== null && <BusyBar fraction={busy} className="bottom-[3px] left-[5px] right-[5px]" />}
              </button>
            </li>
          )
        })}
      </ol>
    </nav>}
    </>
  )
}
