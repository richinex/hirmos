import { Icon } from '@/components/Icon'
import { HirmosMark } from '@/components/HirmosMark'
import { InternalLink } from '@/components/ui/InternalLink'
import { CHAPTER_SECTIONS, type ChapterId } from '@/domain/navigation'
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

/** The same bar the phone tab carries, under the icon: anything alongside crowds the glyph. */
function BusyBar({ fraction, className }: { readonly fraction: number; readonly className?: string }) {
  return (
    <span aria-hidden className={cn('bar-live absolute h-[2px] rounded-full bg-rail-dim', className)}>
      <span
        className="bar-live__fill block origin-left rounded-full bg-rail-signal transition-transform duration-500 ease-[cubic-bezier(0.2,0.7,0.2,1)]"
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

/** Desktop navigation occupies its own column; mobile navigation overlays the workspace. */
export function ChapterNav({ chapters, active, open, onOpen, onClose, onNavigate, onPrefetch, project, onExport }: ChapterNavProps) {
  const phone = useIsMobile()
  const column = useRef<HTMLDivElement>(null)
  const slide = usePhoneSlide(column, onOpen, onClose)
  const drag = phone ? slide.drag : null
  const visible = open || drag !== null
  const panelHandlers = phone ? slide.handlers('panel') : {}

  useEffect(() => phone && visible ? pushLayer(NAV_LAYER) : undefined, [phone, visible])
  useEffect(() => {
    if (!open) return
    const escape = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && (!phone || escapeFor(NAV_LAYER, event))) onClose()
    }
    window.addEventListener('keydown', escape)
    return () => window.removeEventListener('keydown', escape)
  }, [open, phone, onClose])

  const navigate = (id: ChapterId) => { onNavigate(id); if (phone) onClose() }
  const entries = new Map(chapters.map(chapter => [chapter.id, chapter]))
  return <>
    {phone && !visible && <div aria-hidden className="absolute inset-y-0 left-0 z-(--z-overlay) w-3 touch-none" {...slide.handlers('edge')} />}
    {phone && visible && <div aria-hidden className="dashboard-scrim" onClick={onClose} {...panelHandlers} />}
    <aside ref={column} className="dashboard-sidebar" data-open={visible} inert={phone && !visible ? true : undefined}
      style={drag === null ? undefined : { transform: 'translateX(' + drag.offset + 'px)', transition: 'none' }}
      onClickCapture={event => { slide.clickGuard(event) }} {...panelHandlers}>
      <div className="dashboard-brand">
        <InternalLink href="/" className="flex min-w-0 items-center gap-3 text-ink no-underline" title="Hirmos home">
          <span className="dashboard-brand-mark"><HirmosMark size={26} /></span>
          <span className="dashboard-nav-label text-title font-semibold tracking-tight">hirmos</span>
        </InternalLink>
        {phone && <button type="button" className="dashboard-nav-close" aria-label="Close chapter list" onClick={onClose}><Icon name="close" size={20} /></button>}
      </div>
      <nav aria-label="Workspace chapters" className="dashboard-chapters">
        {CHAPTER_SECTIONS.map(section => <section key={section.title} className="dashboard-nav-group">
          <h2 className="dashboard-nav-label dashboard-nav-heading">{section.title}</h2>
          <ol aria-label={section.title}>
            {section.chapters.map(id => {
              const chapter = entries.get(id)
              if (chapter === undefined) return null
              const locked = chapter.status === 'locked'
              const glyph = statusGlyph(chapter.status)
              const busy = chapter.busy ?? null
              return <li key={id}>
                <button type="button" className="dashboard-nav-item" aria-current={active === id ? 'page' : undefined}
                  aria-disabled={locked || undefined} aria-label={glyph === null ? chapter.name : chapter.name + ', ' + glyph.text}
                  title={open ? undefined : chapter.name}
                  onClick={() => { if (!locked) navigate(id) }}
                  onPointerEnter={locked ? undefined : () => onPrefetch(id)} onFocus={locked ? undefined : () => onPrefetch(id)}>
                  <Icon name={chapter.icon} size={20} fill={active === id} />
                  <span className="dashboard-nav-label min-w-0 flex-1 truncate">{chapter.name}</span>
                  {glyph && <Icon name={glyph.icon} size={13} className="dashboard-nav-status" />}
                  {busy !== null && <BusyBar fraction={busy} className="inset-x-3 bottom-1" />}
                </button>
              </li>
            })}
          </ol>
        </section>)}
      </nav>
      {project !== null && <div className="dashboard-project">
        <button type="button" className="dashboard-project-link" aria-label={project.name + '; open Projects'} title={project.name} onClick={() => navigate('projects')}>
          <Icon name="folder" size={20} />
          <span className="dashboard-nav-label min-w-0 text-left"><span className="block truncate font-medium">{project.name}</span><span className="block truncate text-label text-faint">{project.detail}</span></span>
        </button>
        {onExport !== null && <button type="button" className="dashboard-nav-label dashboard-nav-close" aria-label="Export project" title="Export project" onClick={onExport}><Icon name="download" size={18} /></button>}
      </div>}
    </aside>
  </>
}
