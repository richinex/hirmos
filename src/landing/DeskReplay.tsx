import { useLayoutEffect, useRef, useState, type CSSProperties } from 'react'
import { createTimeline, utils } from 'animejs'
import { Icon } from '@/components/Icon'
import { ReplayCursor } from './ReplayCursor'

/**
 * The NSW example's Results desk, replayed: the three saved windows open from the dock in the cascade
 * the desk gives them, the back window is dragged to the front, and Arrange lays them side by side. The
 * titles, numbers and requirements are the desk's own for run 133cff07. The stage is drawn at a fixed
 * size and scaled to its frame, so the windows keep their proportions at any width.
 */

const TILE = 36

type Rect = { readonly x: number; readonly y: number; readonly w: number; readonly h: number }
type Point = { readonly left: number; readonly top: number }

/** Where everything sits on one stage: its size, the three windows' cascade and arranged boxes, the dock, and the cursor's marks. */
type Layout = {
  readonly stage: { readonly width: number; readonly height: number }
  readonly cascade: readonly Rect[]
  readonly arranged: readonly Rect[]
  readonly dock: readonly Rect[]
  readonly dragged: Rect
  readonly cursorFrom: Point
  readonly arrangeAt: Point
}

const dock = (stage: Layout['stage']): readonly Rect[] =>
  [0, 1, 2].map((i) => ({
    x: stage.width / 2 - (3 * TILE + 20) / 2 + i * (TILE + 10),
    y: stage.height - 50,
    w: TILE,
    h: TILE,
  }))

const WIDE_STAGE = { width: 1100, height: 440 }
const WIDE: Layout = {
  stage: WIDE_STAGE,
  cascade: [0, 1, 2].map((i) => ({ x: 40 + i * 260, y: 70 + i * 26, w: 470, h: 270 })),
  arranged: [0, 1, 2].map((i) => ({ x: 16 + i * 361, y: 64, w: 345, h: 306 })),
  dock: dock(WIDE_STAGE),
  dragged: { x: 150, y: 96, w: 470, h: 270 },
  cursorFrom: { left: 980, top: 300 },
  arrangeAt: { left: 1046, top: 18 },
}

const NARROW_STAGE = { width: 400, height: 1010 }
const NARROW: Layout = {
  stage: NARROW_STAGE,
  cascade: [0, 1, 2].map((i) => ({ x: 12 + i * 10, y: 64 + i * 40, w: 356, h: 270 })),
  arranged: [0, 1, 2].map((i) => ({ x: 12, y: 64 + i * 292, w: 376, h: 280 })),
  dock: dock(NARROW_STAGE),
  dragged: { x: 24, y: 150, w: 356, h: 270 },
  cursorFrom: { left: 330, top: 420 },
  arrangeAt: { left: 362, top: 18 },
}

const NARROW_QUERY = '(max-width: 620px)'

const box = (r: Rect) => ({ left: r.x, top: r.y, width: r.w, height: r.h })
const VIEWS = ['Desk', 'Compare', 'Evidence', 'Focus'] as const

const WINDOWS = [
  { icon: 'receipt_long', title: 'Saved results' },
  { icon: 'query_stats', title: 'Average effect of treat on re78, Adjusted linear regression, 2:02pm' },
  { icon: 'assignment', title: 'Record: Average effect of treat on re78, Adjusted linear regression, 2:02pm' },
] as const

function Body({ index }: { readonly index: number }) {
  if (index === 0)
    return (
      <div className="dr-saved">
        <div>
          <b>Average effect of treat on re78</b>
          <span>Adjusted linear regression, 2:02pm</span>
        </div>
        <strong>1.22</strong>
        <span className="dr-chip">Open</span>
        <span className="dr-chip">Compare</span>
      </div>
    )
  if (index === 1)
    return (
      <div className="dr-estimate">
        <span className="dr-label">Estimate and uncertainty</span>
        <b>Effect of changing treat on re78</b>
        <strong>1.22</strong>
        <span>[−0.34, 2.78] 95% CI</span>
        <span>SE 0.79, n = 614</span>
      </div>
    )
  return (
    <div className="dr-record">
      <span className="dr-label">Requirements to review</span>
      <p>The outcome is linear and additive in treatment and covariates with one effect for every unit.</p>
      <p>Treated and untreated values occur across the range of the covariates (positivity).</p>
    </div>
  )
}

export function DeskReplay() {
  const rootRef = useRef<HTMLDivElement>(null)
  const [narrow, setNarrow] = useState(() => window.matchMedia(NARROW_QUERY).matches)
  const layout = narrow ? NARROW : WIDE

  useLayoutEffect(() => {
    const query = window.matchMedia(NARROW_QUERY)
    const change = () => setNarrow(query.matches)
    query.addEventListener('change', change)
    return () => query.removeEventListener('change', change)
  }, [])

  useLayoutEffect(() => {
    const root = rootRef.current
    if (root === null) return
    const stage = root.querySelector<HTMLElement>('.dr-stage')
    const pane = root.querySelector<HTMLElement>('.replay-pane')
    const cursor = root.querySelector<HTMLElement>('.replay-cursor')
    const arrange = root.querySelector<HTMLElement>('.dr-arrange')
    const windows = Array.from(root.querySelectorAll<HTMLElement>('.dr-window'))
    const contents = Array.from(root.querySelectorAll<HTMLElement>('.dr-body'))
    const tiles = Array.from(root.querySelectorAll<HTMLElement>('.dr-tile'))
    if (!stage || !pane || !cursor || !arrange || windows.length !== 3) return

    const fit = () => root.style.setProperty('--k', String(root.clientWidth / layout.stage.width))
    fit()
    const resize = new ResizeObserver(fit)
    resize.observe(root)
    if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return () => resize.disconnect()

    const { cascade, arranged, dock, dragged, cursorFrom, arrangeAt } = layout
    const light = (lit: number) => windows.forEach((w, i) => w.toggleAttribute('data-lit', i === lit))
    const reset = () => {
      windows.forEach((w, i) => utils.set(w, { ...box(dock[i]!), opacity: 0, zIndex: i + 1 }))
      utils.set(contents, { opacity: 0 })
      utils.set(tiles, { scale: 1 })
      utils.set(cursor, { opacity: 0, ...cursorFrom, scale: 1 })
      tiles.forEach((t) => t.removeAttribute('data-open'))
      arrange.removeAttribute('data-pressed')
      light(-1)
    }
    reset()

    const tl = createTimeline({ loop: true, defaults: { ease: 'out(3)' }, onLoop: reset })
    windows.forEach((w, i) => {
      const at = 300 + i * 600
      tl.add(tiles[i]!, { scale: [1, 1.18, 1], duration: 360 }, at)
      tl.call(() => { tiles[i]!.setAttribute('data-open', ''); light(i) }, at + 100)
      tl.add(w, { ...box(cascade[i]!), opacity: [0, 1], duration: 620, ease: 'out(4)' }, at + 100)
      tl.add(contents[i]!, { opacity: [0, 1], duration: 300 }, at + 520)
    })
    tl.add(cursor, { opacity: [0, 1], duration: 200 }, 2400)
    tl.add(cursor, { left: cascade[0]!.x + 190, top: cascade[0]!.y + 18, duration: 700, ease: 'inOut(3)' }, 2400)
    tl.call(() => { utils.set(windows[0]!, { zIndex: 4 }); light(0) }, 3150)
    tl.add(cursor, { scale: [1, 0.86], duration: 120 }, 3150)
    tl.add(windows[0]!, { left: dragged.x, top: dragged.y, duration: 700, ease: 'inOut(2)' }, 3300)
    tl.add(cursor, { left: dragged.x + 190, top: dragged.y + 18, duration: 700, ease: 'inOut(2)' }, 3300)
    tl.add(cursor, { scale: [0.86, 1], duration: 140 }, 4050)
    tl.add(cursor, { ...arrangeAt, duration: 750, ease: 'inOut(3)' }, 4300)
    tl.call(() => arrange.setAttribute('data-pressed', ''), 5080)
    tl.add(cursor, { scale: [1, 0.86, 1], duration: 260 }, 5080)
    tl.call(() => { arrange.removeAttribute('data-pressed'); light(-1) }, 5340)
    windows.forEach((w, i) => tl.add(w, { ...box(arranged[i]!), duration: 820, ease: 'inOut(3)' }, 5300 + i * 70))
    tl.add(cursor, { opacity: 0, duration: 300 }, 6300)
    tl.add(pane, { opacity: 0, duration: 420, ease: 'inOut(2)' }, '+=3600')
    tl.add(pane, { opacity: 1, duration: 0 }, '+=60')

    tl.pause()
    const io = new IntersectionObserver(([entry]) => (entry?.isIntersecting ? tl.play() : tl.pause()), { threshold: 0.3 })
    io.observe(root)
    return () => {
      io.disconnect()
      resize.disconnect()
      tl.revert()
    }
  }, [layout])

  return (
    <div
      ref={rootRef}
      className="replay dr"
      role="img"
      aria-label="The Results desk for the NSW example: the saved results, the estimate and its record opened as windows and arranged side by side"
      style={{ '--stage-width': `${layout.stage.width}px`, '--stage-height': `${layout.stage.height}px` } as CSSProperties}
    >
      <div className="replay-pane dr-frame" aria-hidden>
        <div className="dr-stage">
          <div className="dr-toolbar">
            <span className="dr-views">
              {VIEWS.map((view, i) => (
                <span key={view} className={i === 0 ? 'dr-view dr-view--on' : 'dr-view'}>
                  {view}
                </span>
              ))}
            </span>
            <span className="dr-arrange">
              <Icon name="auto_awesome_mosaic" size={18} />
              {narrow ? null : 'Arrange'}
            </span>
          </div>
          {WINDOWS.map((w, i) => (
            <div key={w.title} className="dr-window" style={{ ...box(layout.arranged[i]!), zIndex: i + 1 }}>
              <div className="dr-titlebar">
                <Icon name={w.icon} size={18} />
                <span className="dr-name">{w.title}</span>
              </div>
              <div className="dr-body">
                <Body index={i} />
              </div>
            </div>
          ))}
          {WINDOWS.map((w, i) => (
            <span key={w.icon} className="dr-tile" data-open="" style={box(layout.dock[i]!)}>
              <Icon name={w.icon} size={18} />
            </span>
          ))}
          <ReplayCursor />
        </div>
      </div>
    </div>
  )
}
