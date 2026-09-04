import { Icon } from '@/components/Icon'
import type { ChapterId } from '@/domain/navigation'
import { assertNever } from '@/domain/dop'
import { useEffect, useRef, useState, type MouseEvent as ReactMouseEvent, type PointerEvent as ReactPointerEvent, type RefObject } from 'react'
import { escapeFor, pushLayer } from '@/lib/dismissal'
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

/** A drag counts once the finger has moved this far sideways; anything less is a tap or a vertical scroll. */
const SLIDE_START = 20
/** A drag past this snaps to the state it is heading for; short of it, the list springs back. */
const SLIDE_SNAP = 70

/** 'edge' opens the closed list from the left edge of the stage; 'panel' closes the open one from the list or the scrim. */
type SlideOrigin = 'edge' | 'panel'

interface SlideGesture {
  readonly id: number
  readonly startX: number
  readonly from: SlideOrigin
  /** The list's width when the gesture began: the distance between closed and open. */
  readonly span: number
  moved: boolean
}

interface SlideState {
  /** Current translateX of the list, from -span (closed) to 0 (open). */
  readonly offset: number
  readonly span: number
}

/**
 * Finger-following slide for the phone list. The list tracks the pointer between fully closed and
 * fully open, then snaps on release. The surfaces set touch-action, so the browser keeps vertical
 * scrolling and a vertical gesture cancels the pointer instead of fighting it.
 */
function usePhoneSlide(list: RefObject<HTMLElement | null>, onOpen: () => void, onClose: () => void) {
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
      const span = list.current?.offsetWidth ?? 0
      if (gesture.current !== null || span === 0) return
      gesture.current = { id: event.pointerId, startX: event.clientX, from, span, moved: false }
      // The edge strip is narrower than the start distance and has nothing to tap, so it captures at once;
      // the list and scrim capture only once this is a drag, so a plain tap still reaches the button under it.
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
  readonly collapsed: boolean
  readonly onNavigate: (chapter: ChapterId) => void
  /** Warms a chapter's code chunk on hover or focus, ahead of the click. */
  readonly onPrefetch: (chapter: ChapterId) => void
  /** Phone only: whether the list is slid in over the stage; session state, not the persisted desktop preference. */
  readonly phoneOpen: boolean
  readonly onPhoneOpen: () => void
  readonly onPhoneClose: () => void
}

/**
 * The chapter list: 208px with numbers and names, or the 48px icon rail. When the shell container
 * is narrower than md the list leaves the flow and slides in under the header, over the stage, so
 * the header and its toggle stay put and the stage keeps the full width; a swipe from the left
 * edge opens it and a drag closes it. Gated chapters stay in the tab order.
 */
/** One phone nav at a time, so the layer stack needs no per-instance id. */
const PHONE_NAV_LAYER = 'chapter-nav-phone'

export function ChapterNav({ chapters, active, collapsed, onNavigate, onPrefetch, phoneOpen, onPhoneOpen, onPhoneClose }: ChapterNavProps) {
  const phone = useIsMobile()
  const rail = collapsed && !phone
  const list = useRef<HTMLElement>(null)
  const slide = usePhoneSlide(list, onPhoneOpen, onPhoneClose)
  const drag = phone ? slide.drag : null
  const slidIn = phone && (phoneOpen || drag !== null)
  const panelHandlers = phone ? slide.handlers('panel') : {}
  useEffect(() => (slidIn ? pushLayer(PHONE_NAV_LAYER) : undefined), [slidIn])
  useEffect(() => {
    if (!slidIn) return
    const escape = (event: KeyboardEvent) => { if (event.key === 'Escape' && escapeFor(PHONE_NAV_LAYER, event)) onPhoneClose() }
    window.addEventListener('keydown', escape)
    return () => window.removeEventListener('keydown', escape)
  }, [onPhoneClose, slidIn])
  return (
    <>
    {phone && !phoneOpen && <div aria-hidden className="absolute inset-y-0 left-0 z-(--z-overlay) w-5 touch-none" {...slide.handlers('edge')} />}
    {slidIn && (
      <div
        aria-hidden
        onClick={(event) => { if (!slide.clickGuard(event)) onPhoneClose() }}
        style={drag === null ? undefined : { opacity: 1 + drag.offset / drag.span, transition: 'none' }}
        className="absolute inset-0 z-(--z-overlay) touch-none bg-black/40 backdrop-blur-sm transition-opacity duration-200 starting:opacity-0 motion-reduce:transition-none"
        {...panelHandlers}
      />
    )}
    <nav
      ref={list}
      aria-label="Workspace chapters"
      inert={phone && !slidIn ? true : undefined}
      style={drag === null ? undefined : { translate: `${drag.offset}px 0`, transition: 'none' }}
      onClickCapture={(event) => { slide.clickGuard(event) }}
      className={cn(
        'flex shrink-0 flex-col gap-1 overflow-hidden border-r border-line bg-panel py-2 transition-[width,translate] duration-200 ease-[cubic-bezier(0.2,0.7,0.2,1)] motion-reduce:transition-none',
        rail ? 'w-12 items-center' : 'w-60 px-2',
        '@max-md/shell:absolute @max-md/shell:inset-y-0 @max-md/shell:left-0 @max-md/shell:z-(--z-overlay) @max-md/shell:touch-pan-y @max-md/shell:will-change-transform',
        phoneOpen ? '@max-md/shell:translate-x-0' : '@max-md/shell:-translate-x-full',
      )}
      {...panelHandlers}
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
                aria-label={rail ? chapter.name : undefined}
                title={rail ? (busy === null ? name : `${name}, ${Math.round(busy * 100)}% done`) : undefined}
                onClick={() => { if (!locked) { onNavigate(chapter.id); onPhoneClose() } }}
                onPointerEnter={locked ? undefined : () => onPrefetch(chapter.id)}
                onFocus={locked ? undefined : () => onPrefetch(chapter.id)}
                className={cn(
                  'relative flex items-center gap-2 rounded-lg border text-left text-body transition-colors duration-150',
                  rail ? 'grid h-9 w-9 place-items-center pointer-coarse:h-10 pointer-coarse:w-10' : 'w-full px-2.5 py-2',
                  isActive ? 'border-edge bg-raised text-ink' : 'border-transparent text-faint hover:text-ink',
                  locked && 'cursor-not-allowed text-dim hover:text-dim',
                )}
              >
                <Icon name={chapter.icon} size={rail ? 19 : 16} fill={isActive} className={busy !== null ? 'text-signal' : undefined} />
                {!rail && (
                  <span className="min-w-0 flex-1 truncate">
                    <span className="tabular-nums text-faint">{number(index)}</span> · {chapter.name}
                  </span>
                )}
                {glyph && (
                  <Icon name={glyph.icon} size={12} className={cn(glyph.tone, rail && 'absolute right-0.5 top-0.5')} />
                )}
                {glyph && <span className="sr-only">, {glyph.text}</span>}
                {busy !== null && <BusyBar fraction={busy} className="bottom-[3px] left-[5px] right-[5px]" />}
              </button>
            </li>
          )
        })}
      </ol>
    </nav>
    </>
  )
}
