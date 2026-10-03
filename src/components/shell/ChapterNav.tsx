import { Icon } from '@/components/Icon'
import { HirmosMark } from '@/components/HirmosMark'
import { InternalLink } from '@/components/ui/InternalLink'
import { CHAPTER_SECTIONS, type ChapterId } from '@/domain/navigation'
import { assertNever } from '@/domain/dop'
import { useEffect, useRef, useState, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent, type RefObject } from 'react'
import { escapeFor, pushLayer } from '@/lib/dismissal'
import { useChapterBusy } from '@/lib/runActivityStore'
import { useIsMobile } from '@/lib/useMediaQuery'
import { cn } from '@/lib/utils'

export type ChapterStatus = 'locked' | 'not-started' | 'in-progress' | 'done' | 'refused'

export interface ChapterEntry {
  readonly id: ChapterId
  readonly name: string
  readonly shortName: string
  readonly icon: string
  readonly status: ChapterStatus
}

/** What the pill at the foot of the rail says about the open project. */
export interface RailProject {
  readonly name: string
  /** The data file's name, or what is still missing. */
  readonly detail: string
}

const statusGlyph = (status: ChapterStatus): { readonly icon: string; readonly tone: string; readonly text: string } | null => {
  switch (status) {
    case 'not-started': return null
    case 'locked': return { icon: 'lock', tone: 'text-rail-dim', text: 'not yet available' }
    case 'in-progress': return { icon: 'progress_activity', tone: 'text-rail-info', text: 'in progress' }
    case 'done': return { icon: 'check', tone: 'text-rail-ok', text: 'done' }
    case 'refused': return { icon: 'block', tone: 'text-rail-danger', text: 'refused' }
    default: return assertNever(status)
  }
}

/** The rail's progress bar. It subscribes itself, so progress redraws only this bar. */
function BusyBar({ chapter, className }: { readonly chapter: ChapterId; readonly className?: string }) {
  const fraction = useChapterBusy(chapter)
  if (fraction === null) return null
  return (
    <span aria-hidden className={cn('bar-live absolute h-[2px] rounded-full bg-rail-dim', className)}>
      <span
        className="bar-live__fill block origin-left rounded-full bg-rail-signal transition-transform duration-(--motion-progress)"
        style={{ transform: `scaleX(${fraction})` }}
      />
    </span>
  )
}

/** A drag counts once the finger has moved this far sideways; anything less is a tap or a vertical scroll. */
const SLIDE_START = 20
/** A drag past this snaps to the state it is heading for; short of it, the list springs back. */
const SLIDE_SNAP = 70

/** 'edge' opens the closed rail from the left edge of the stage; 'panel' closes the open one from the rail or the scrim. */
type SlideOrigin = 'edge' | 'panel'

interface SlideGesture {
  readonly id: number
  readonly startX: number
  readonly from: SlideOrigin
  /** The rail's width when the gesture began: the distance between closed and open. */
  readonly span: number
  moved: boolean
}

interface SlideState {
  /** Current translateX of the rail, from -span (closed) to 0 (open). */
  readonly offset: number
  readonly span: number
}

/**
 * Finger-following slide for the phone rail. It tracks the pointer between fully closed and fully
 * open, then snaps on release. The surfaces set touch-action, so the browser keeps vertical
 * scrolling and a vertical gesture cancels the pointer instead of fighting it.
 */
function usePhoneSlide(column: RefObject<HTMLElement | null>, onOpen: () => void, onClose: () => void) {
  const [drag, setDrag] = useState<SlideState | null>(null)
  const gesture = useRef<SlideGesture | null>(null)
  /** Clicks are swallowed until this time, so the click the browser sends after a drag does not count as a tap. */
  const swallowClicksUntil = useRef(0)

  const move = (event: ReactPointerEvent<HTMLElement>) => {
    const current = gesture.current
    if (current === null || event.pointerId !== current.id) return
    const dx = event.clientX - current.startX
    if (!current.moved) {
      if (Math.abs(dx) < SLIDE_START) return
      current.moved = true
      event.currentTarget.setPointerCapture(event.pointerId)
    }
    const resting = current.from === 'edge' ? -current.span : 0
    setDrag({ offset: Math.min(0, Math.max(-current.span, resting + dx)), span: current.span })
  }
  const end = (event: ReactPointerEvent<HTMLElement>) => {
    const current = gesture.current
    if (current === null || event.pointerId !== current.id) return
    gesture.current = null
    setDrag(null)
    if (!current.moved) return
    swallowClicksUntil.current = performance.now() + 300
    const dx = event.clientX - current.startX
    const opens = current.from === 'edge' ? dx > SLIDE_SNAP : -dx <= SLIDE_SNAP
    if (opens) onOpen()
    else onClose()
  }
  const cancel = () => { gesture.current = null; setDrag(null) }
  const handlers = (from: SlideOrigin) => ({
    onPointerDown: (event: ReactPointerEvent<HTMLElement>) => {
      const span = column.current?.offsetWidth ?? 0
      if (gesture.current !== null || span === 0) return
      gesture.current = { id: event.pointerId, startX: event.clientX, from, span, moved: false }
      // The edge strip is narrower than the start distance and has nothing to tap, so it captures at once;
      // the rail and scrim capture only once this is a drag, so a plain tap still reaches the button under it.
      if (from === 'edge') event.currentTarget.setPointerCapture(event.pointerId)
    },
    onPointerMove: move,
    onPointerUp: end,
    onPointerCancel: cancel,
  })
  /** A drag that ends on a button or the scrim must not also count as a tap on it. */
  const clickGuard = (event: ReactMouseEvent<HTMLElement>): boolean => {
    if (performance.now() > swallowClicksUntil.current) return false
    event.preventDefault()
    event.stopPropagation()
    return true
  }
  return { drag, handlers, clickGuard }
}

interface ChapterNavProps {
  readonly chapters: readonly ChapterEntry[]
  readonly active: ChapterId
  /** Whether the lobe with the names is out over the stage; on a phone, whether the rail is slid in at all. */
  readonly open: boolean
  readonly onOpen: () => void
  readonly onClose: () => void
  readonly onNavigate: (chapter: ChapterId) => void
  /** Warms a chapter's code chunk on hover or focus, ahead of the click. */
  readonly onPrefetch: (chapter: ChapterId) => void
  readonly project: RailProject | null
  /** Exports the open project as a bundle; null until there is one to export. */
  readonly onExport: (() => void) | null
}

/** One rail at a time, so the layer stack needs no per-instance id. */
const NAV_LAYER = 'chapter-rail'

/** The rail's width, which the column reserves; the lobe and the pill grow past it over the stage. */
const RAIL = 'w-14'
const LOBE_OPEN = 'w-[232px]'
const CORNER = 'rounded-r-[26px]'

/**
 * The chapter rail: a solid shape rising from the left edge under the round toggle, the mark at its
 * head, the chapters as icons in a lobe, and the project in a pill at its foot. Opening it sends
 * the lobe and the pill out over the stage with the names, the numbers and the status glyphs; the
 * stage does not move. It closes on a choice, on Escape, and on a click anywhere else. When the shell
 * container is narrower than md the whole rail leaves the flow and slides in over a scrim; a swipe from
 * the left edge opens it and a drag closes it. Gated chapters stay in the tab order.
 */
export function ChapterNav({ chapters, active, open, onOpen, onClose, onNavigate, onPrefetch, project, onExport }: ChapterNavProps) {
  const phone = useIsMobile()
  const column = useRef<HTMLDivElement>(null)
  const slide = usePhoneSlide(column, onOpen, onClose)
  const drag = phone ? slide.drag : null
  const slidIn = phone && (open || drag !== null)
  const panelHandlers = phone ? slide.handlers('panel') : {}
  // On a phone the rail is either away or out with its names, so the lobe is open whenever the rail shows.
  const lobeOpen = phone ? slidIn : open
  const layered = phone ? slidIn : open
  useEffect(() => (layered ? pushLayer(NAV_LAYER) : undefined), [layered])
  useEffect(() => {
    if (!layered) return
    const escape = (event: KeyboardEvent) => { if (event.key === 'Escape' && escapeFor(NAV_LAYER, event)) onClose() }
    window.addEventListener('keydown', escape)
    return () => window.removeEventListener('keydown', escape)
  }, [layered, onClose])
  useEffect(() => {
    if (phone || !open) return
    // The lobe lies over the stage, so a press anywhere off the rail puts it away; the toggle handles its own press.
    const away = (event: PointerEvent) => {
      const target = event.target
      if (!(target instanceof Node)) return
      if (column.current?.contains(target) || (target instanceof Element && target.closest('[data-rail-toggle]') !== null)) return
      onClose()
    }
    document.addEventListener('pointerdown', away)
    return () => document.removeEventListener('pointerdown', away)
  }, [onClose, open, phone])

  const entries = new Map(chapters.map((chapter) => [chapter.id, chapter]))
  return (
    <>
    {phone && !open && <div aria-hidden className="absolute inset-y-0 left-0 z-(--z-overlay) w-5 touch-none" {...slide.handlers('edge')} />}
    {slidIn && (
      <div
        aria-hidden
        onClick={(event) => { if (!slide.clickGuard(event)) onClose() }}
        style={drag === null ? undefined : { opacity: 1 + drag.offset / drag.span, transition: 'none' }}
        className="absolute inset-0 z-(--z-overlay) touch-none bg-black/40 backdrop-blur-sm transition-opacity duration-(--motion-base) starting:opacity-0 motion-reduce:transition-none"
        {...panelHandlers}
      />
    )}
    <div
      ref={column}
      inert={phone && !slidIn ? true : undefined}
      style={drag === null ? undefined : { translate: `${drag.offset}px 0`, transition: 'none' }}
      onClickCapture={(event) => { slide.clickGuard(event) }}
      className={cn(
        'dashboard-sidebar relative z-(--z-overlay) w-[76px] shrink-0 transition-[translate] duration-(--motion-base) motion-reduce:transition-none',
        '@max-md/shell:absolute @max-md/shell:inset-y-0 @max-md/shell:left-0 @max-md/shell:w-[264px] @max-md/shell:touch-pan-y @max-md/shell:will-change-transform',
        open ? '@max-md/shell:translate-x-0' : '@max-md/shell:-translate-x-full',
      )}
      {...panelHandlers}
    >
      {/* The rail itself: the shape the lobe and the pill grow out of. */}
      <div aria-hidden className={cn('absolute inset-y-0 left-0 top-3 bottom-4 bg-rail text-rail-ink', RAIL, CORNER)} />
      <h1 className="absolute left-0 top-7 m-0 grid w-14 place-items-center">
        <InternalLink href="/" className="grid h-8 w-8 place-items-center rounded-full text-rail-signal no-underline transition-opacity hover:opacity-70" title="Hirmos home">
          <HirmosMark size={24} />
          <span className="sr-only">Hirmos</span>
        </InternalLink>
      </h1>

      <nav
        aria-label="Workspace sections"
        className={cn(
          'absolute left-0 top-[84px] flex max-h-[calc(100%-84px-72px)] flex-col overflow-hidden bg-rail text-rail-ink [--scroll-thumb:var(--color-rail-faint)] transition-[width] duration-(--motion-base) motion-reduce:transition-none',
          CORNER,
          lobeOpen ? LOBE_OPEN : RAIL,
        )}
      >
        {/* The list scrolls inside a box inset by the corner radius, so the scrollbar stays on the straight edge. */}
        <div className="my-[26px] min-h-0 overflow-y-auto overflow-x-hidden overscroll-contain">
        {CHAPTER_SECTIONS.map((section) => (
          <div key={section.title} className="not-first:mt-2">
            <div aria-hidden className={cn('h-[18px] overflow-hidden whitespace-nowrap pl-[18px] text-micro leading-[18px] text-rail-faint transition-opacity duration-(--motion-base)', lobeOpen ? 'opacity-100' : 'opacity-0')}>{section.title}</div>
            <ol className="m-0 list-none p-0" aria-label={section.title}>
              {section.chapters.map((id) => {
                const chapter = entries.get(id)
                if (chapter === undefined) return null
                const isActive = chapter.id === active
                const locked = chapter.status === 'locked'
                const glyph = statusGlyph(chapter.status)
                const name = chapter.name
                return (
                  <li key={chapter.id}>
                    <button
                      type="button"
                      aria-current={isActive ? 'page' : undefined}
                      aria-disabled={locked || undefined}
                      aria-label={glyph === null ? name : `${name}, ${glyph.text}`}
                      title={lobeOpen ? undefined : name}
                      onClick={() => { if (!locked) { onNavigate(chapter.id); onClose() } }}
                      onPointerEnter={locked ? undefined : () => onPrefetch(chapter.id)}
                      onFocus={locked ? undefined : () => onPrefetch(chapter.id)}
                      className={cn(
                        'relative flex h-11 w-full items-center whitespace-nowrap pl-[18px] text-left text-body text-rail-faint transition-colors duration-(--motion-fast)',
                        isActive && 'text-rail-ink after:absolute after:inset-y-1 after:right-0 after:w-1 after:rounded-l-[3px] after:bg-rail-signal',
                        locked ? 'cursor-not-allowed text-rail-dim' : 'hover:text-rail-ink',
                      )}
                    >
                      <Icon name={chapter.icon} size={20} fill={isActive} className="shrink-0" />
                      <span
                        aria-hidden
                        className={cn(
                          'flex min-w-0 items-baseline gap-2 overflow-hidden transition-[width,margin,opacity] duration-(--motion-base) motion-reduce:transition-none',
                          lobeOpen ? 'ml-[18px] w-[150px] opacity-100' : 'ml-0 w-0 opacity-0',
                        )}
                      >
                        <span className="truncate">{chapter.name}</span>
                      </span>
                      {glyph && (
                        <Icon
                          name={glyph.icon}
                          size={lobeOpen ? 12 : 10}
                          className={cn(glyph.tone, 'absolute transition-[top,right,left] duration-(--motion-base)', lobeOpen ? 'right-[22px] top-4' : 'left-[34px] top-2')}
                        />
                      )}
                      <BusyBar chapter={chapter.id} className={cn('bottom-[5px] left-[18px]', lobeOpen ? 'right-[22px]' : 'w-5')} />
                    </button>
                  </li>
                )
              })}
            </ol>
          </div>
        ))}
        </div>
      </nav>

      {/* The open project, in the pill at the foot: its name and data file out with the lobe, and the export. */}
      {project !== null && (
      <div
        className={cn(
          'absolute bottom-3 left-2 flex h-10 items-center overflow-hidden rounded-full bg-rail text-rail-ink transition-[width] duration-(--motion-base) motion-reduce:transition-none',
          lobeOpen ? 'w-[248px]' : 'w-10',
        )}
      >
        <button
          type="button"
          className="dashboard-project-link grid h-10 w-10 shrink-0 place-items-center rounded-full text-rail-signal transition-opacity hover:opacity-70"
          aria-label={`${project.name}; open Projects`}
          title={project.name}
          onClick={() => { onNavigate('projects'); onClose() }}
        >
          <Icon name="folder" size={20} fill />
        </button>
        <span aria-hidden className={cn('min-w-0 flex-1 whitespace-nowrap pr-2 transition-opacity duration-(--motion-base)', lobeOpen ? 'opacity-100' : 'opacity-0')}>
          <span className="block truncate text-body font-medium">{project.name}</span>
          <span className="block truncate text-micro text-rail-faint">{project.detail}</span>
        </span>
        {onExport !== null && (
          <button
            type="button"
            className={cn('mr-1 grid h-9 w-9 shrink-0 place-items-center rounded-full text-rail-signal transition-opacity hover:opacity-70', lobeOpen ? 'opacity-100' : 'opacity-0')}
            aria-label="Export project"
            title="Export this project as a bundle"
            tabIndex={lobeOpen ? undefined : -1}
            onClick={() => { onExport(); onClose() }}
          >
            <Icon name="download" size={18} />
          </button>
        )}
      </div>
      )}
    </div>
    </>
  )
}
