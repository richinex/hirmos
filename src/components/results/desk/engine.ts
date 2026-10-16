import type { DeskState, DeskViewName } from '@/domain/resultsDesk'
import {
  blurDesk,
  deskStore,
  fillPlacements,
  focusWindow,
  placeWindows,
  setRatio,
  setSlotOrder,
} from './deskStore'
import {
  dockTile,
  fit,
  inset,
  MARGIN,
  MIN_HEIGHT,
  MIN_WIDTH,
  overlaps,
  tidy,
  tile,
  workspaceOf,
  type Rect,
  type Seam,
  type Split,
} from './layout'

export const TITLE_HEIGHT = 40

const STEP = 0.004
const STIFFNESS = 650
const DAMPING = 45
const MAX_STEPS = 300
const FIELDS = ['centerX', 'centerY', 'width', 'height', 'scale'] as const
type Field = (typeof FIELDS)[number]

export interface DeskWindowElements {
  readonly frame: HTMLElement
  readonly content: HTMLElement
  readonly titlebar: HTMLElement
  readonly grip: HTMLElement
  readonly badge: HTMLElement
  /** The window's mark: drags it between slots and moves it with the arrow keys. */
  readonly handle: HTMLElement
}

/** The tiled views' split trees, built from the records on the desk. */
export type DeskTrees = Partial<Record<'compare' | 'evidence', Split | null>>

interface Spring {
  value: number
  velocity: number
}

interface Slots {
  readonly keys: readonly string[]
  readonly rects: readonly Rect[]
}

interface Live {
  elements: DeskWindowElements | null
  tile: HTMLElement | null
  springs: Record<Field, Spring>
  view: DeskViewName
  shown: boolean
  fresh: boolean
  override: Rect | null
  sizingUntil: number
  timer: number
}

export function createDeskEngine(surface: HTMLElement, trees: () => DeskTrees) {
  const live = new Map<string, Live>()
  const ratioOverride = new Map<string, number>()
  const seamElements: HTMLElement[] = []
  let frame = 0
  let clock = 0
  let lastView: DeskViewName = deskStore.getState().desk.view
  let lastVisible = new Map(
    deskStore.getState().desk.windows.map((window) => [window.key, window.visible]),
  )
  let pendingPlacements: Map<string, Rect> | null = null
  let started = false
  const layouts = new Map<DeskViewName, ReturnType<typeof tile>>()
  /** Where windows are drawn while a window is carried between slots, before the swap is stored. */
  const preview = new Map<string, Rect>()

  const calmQuery = window.matchMedia('(prefers-reduced-motion: reduce)')
  const reduced = () => calmQuery.matches
  const size = () => ({ width: surface.clientWidth, height: surface.clientHeight })
  const compact = () => surface.clientWidth < 620
  const state = (): DeskState => deskStore.getState().desk

  const entry = (key: string): Live => {
    let found = live.get(key)
    if (found === undefined) {
      found = {
        elements: null,
        tile: null,
        springs: Object.fromEntries(
          FIELDS.map((field) => [field, { value: 0, velocity: 0 }]),
        ) as Record<Field, Spring>,
        view: state().view,
        shown: false,
        fresh: true,
        override: null,
        sizingUntil: 0,
        timer: 0,
      }
      live.set(key, found)
    }
    return found
  }

  const tiles = (view: DeskViewName) => {
    const cached = layouts.get(view)
    if (cached !== undefined) return cached
    const computed = layoutOf(view)
    layouts.set(view, computed)
    return computed
  }

  const layoutOf = (view: DeskViewName) => {
    const space = inset(workspaceOf(size()), MARGIN)
    if (view === 'focus') {
      const focused = state().focused ?? state().windows.at(-1)?.key ?? null
      return { placed: new Map(focused === null ? [] : [[focused, space]]), seams: [] as Seam[] }
    }
    const tree = view === 'desk' ? null : (trees()[view] ?? null)
    if (tree === null) return { placed: new Map<string, Rect>(), seams: [] as Seam[] }
    return tile(tree, space, compact(), (id, ratio) => ratioOverride.get(id) ?? ratio)
  }

  const deskRect = (key: string): Rect => {
    const window = state().windows.find((candidate) => candidate.key === key)
    const space = workspaceOf(size())
    if (window?.maximized) return inset(space, MARGIN)
    const runtime = live.get(key)
    if (runtime?.override) return fit(runtime.override, space)
    if (window?.placed) return fit(window.placed, space)
    const missing = state()
      .windows.filter((candidate) => candidate.placed === null)
      .map((candidate) => candidate.key)
    const guess = tidy(
      state()
        .windows.filter((candidate) => candidate.visible)
        .map((candidate) => candidate.key),
      space,
    )
    const rect =
      guess.get(key) ?? fit({ left: MARGIN, top: MARGIN, width: 480, height: 360 }, space)
    if (missing.includes(key) && window?.visible) {
      pendingPlacements ??= new Map()
      pendingPlacements.set(key, rect)
    }
    return rect
  }

  const shownIn = (key: string, view: DeskViewName): boolean =>
    view === 'desk'
      ? (state().windows.find((window) => window.key === key)?.visible ?? false)
      : tiles(view).placed.has(key)

  const visibleRect = (key: string, view: DeskViewName): Rect =>
    preview.get(key) ??
    (view === 'desk' ? deskRect(key) : (tiles(view).placed.get(key) ?? deskRect(key)))

  // A tiled view, or a desk whose open windows do not overlap, holds its windows in slots read in
  // reading order. A carried window is inserted at the slot its centre enters and the windows between
  // shift along, as a Motion reorder group does along one axis. Overlapping windows have no slots.
  const slotsNow = (): Slots | null => {
    const view = state().view
    let entries: (readonly [string, Rect])[]
    if (view === 'compare' || view === 'evidence') entries = [...tiles(view).placed]
    else if (view === 'desk') {
      const shown = state().windows.filter((window) => window.visible && !window.maximized)
      if (shown.length < 2) return null
      entries = shown.map((window) => [window.key, deskRect(window.key)] as const)
      const boxes = entries.map(([, rect]) => rect)
      if (boxes.some((a, i) => boxes.slice(i + 1).some((b) => overlaps(a, b)))) return null
    } else return null
    const sorted = [...entries].sort(([, a], [, b]) =>
      Math.abs(a.top - b.top) > 1 ? a.top - b.top : a.left - b.left,
    )
    return { keys: sorted.map(([key]) => key), rects: sorted.map(([, rect]) => rect) }
  }

  const inserted = (keys: readonly string[], key: string, index: number) => {
    const rest = keys.filter((other) => other !== key)
    rest.splice(index, 0, key)
    return rest
  }

  /** Every window except the carried one drawn at its slot in `order`. */
  const showOrder = (slots: Slots, order: readonly string[], carried: string | null) => {
    order.forEach((other, i) => {
      if (other === carried) return
      const rect = slots.rects[i]!
      const was = visibleRect(other, state().view)
      if (Math.abs(was.width - rect.width) > 1 || Math.abs(was.height - rect.height) > 1)
        markSizing(other)
      preview.set(other, rect)
    })
    schedule()
  }

  const commitOrder = (slots: Slots, order: readonly string[]) => {
    const view = state().view
    showOrder(slots, order, null)
    if (view === 'compare' || view === 'evidence') {
      // The tree is rebuilt from the stored order on the next render; the preview holds until then.
      const byRect = new Map(slots.rects.map((rect, i) => [rect, order[i]!] as const))
      setSlotOrder(
        view,
        [...tiles(view).placed.values()].map((rect) => byRect.get(rect)!),
      )
    } else {
      placeWindows(new Map(order.map((key, i) => [key, slots.rects[i]!] as const)))
      preview.clear()
    }
  }

  const target = (key: string, runtime: Live): Record<Field, number> => {
    const rect = visibleRect(key, runtime.view)
    const index = state().windows.findIndex((window) => window.key === key)
    const dock = dockTile(Math.max(0, index), state().windows.length, size())
    return shownIn(key, runtime.view)
      ? {
          centerX: rect.left + rect.width / 2,
          centerY: rect.top + rect.height / 2,
          width: rect.width,
          height: rect.height,
          scale: 1,
        }
      : {
          centerX: dock.left + dock.width / 2,
          centerY: dock.top + dock.height / 2,
          width: rect.width,
          height: rect.height,
          scale: 0,
        }
  }

  const settle = (key: string) => {
    const runtime = entry(key)
    const goal = target(key, runtime)
    for (const field of FIELDS) {
      runtime.springs[field].value = goal[field]
      runtime.springs[field].velocity = 0
    }
  }

  const markSizing = (key: string, hold = 700) => {
    entry(key).sizingUntil = performance.now() + hold
  }

  const rise = (runtime: Live) => {
    if (reduced() || runtime.elements === null) return
    ;[...runtime.elements.content.children].forEach((child, index) => {
      child.animate(
        [
          { opacity: 0, transform: 'translateY(6px)' },
          { opacity: 1, transform: 'none' },
        ],
        {
          duration: 320,
          delay: 80 + index * 45,
          easing: 'ease-out',
          fill: 'backwards',
        },
      )
    })
  }

  const schedule = () => {
    if (frame === 0) {
      clock = performance.now()
      frame = requestAnimationFrame(tick)
    }
  }

  // A new view: windows that leave fold into the dock at once, and the rest travel a moment later,
  // one after another, so each window can be followed to its place.
  const onStore = () => {
    layouts.clear()
    const desk = state()
    const calm = reduced()
    if (desk.view !== lastView) {
      const coming = desk.view
      let order = 0
      for (const window of desk.windows) {
        const runtime = entry(window.key)
        clearTimeout(runtime.timer)
        runtime.timer = 0
        const leaving = !shownIn(window.key, coming)
        if (leaving || calm) {
          runtime.view = coming
          continue
        }
        const delay = 70 + order++ * 45
        runtime.timer = setTimeout(() => {
          const wasShown = runtime.shown
          runtime.timer = 0
          runtime.view = coming
          if (!wasShown) rise(runtime)
          markSizing(window.key)
          schedule()
        }, delay) as unknown as number
      }
      lastView = coming
    } else {
      for (const window of desk.windows) {
        const runtime = entry(window.key)
        if (runtime.view !== desk.view && runtime.timer === 0) runtime.view = desk.view
        if (window.visible && lastVisible.get(window.key) === false) rise(runtime)
      }
    }
    lastVisible = new Map(desk.windows.map((window) => [window.key, window.visible]))
    for (const key of [...live.keys()])
      if (!desk.windows.some((window) => window.key === key) && live.get(key)!.elements === null)
        live.delete(key)
    placeSeams()
    schedule()
  }

  function tick(now: number) {
    frame = 0
    const steps = Math.floor((now - clock) / (STEP * 1000))
    clock += steps * STEP * 1000
    const land = reduced() || steps > MAX_STEPS
    let moving = false
    for (const window of state().windows) {
      const runtime = entry(window.key)
      if (runtime.elements === null) continue
      const goal = target(window.key, runtime)
      if (runtime.fresh) {
        const index = state().windows.findIndex((candidate) => candidate.key === window.key)
        const dock = dockTile(index, state().windows.length, size())
        const from =
          started && goal.scale === 1
            ? {
                centerX: dock.left + dock.width / 2,
                centerY: dock.top + dock.height / 2,
                width: goal.width,
                height: goal.height,
                scale: 0,
              }
            : goal
        for (const field of FIELDS) runtime.springs[field].value = from[field]
        runtime.fresh = false
        if (from !== goal) {
          rise(runtime)
          moving = true
        }
      }
      for (const field of FIELDS) {
        const spring = runtime.springs[field]
        if (land) {
          spring.value = goal[field]
          spring.velocity = 0
          continue
        }
        for (let i = 0; i < steps; i++) {
          spring.velocity +=
            (-STIFFNESS * (spring.value - goal[field]) - DAMPING * spring.velocity) * STEP
          spring.value += spring.velocity * STEP
        }
        const threshold = field === 'scale' ? 2 ** -16 : 0.01
        if (
          Math.abs(spring.value - goal[field]) <= threshold &&
          Math.abs(spring.velocity) <= threshold
        ) {
          spring.value = goal[field]
          spring.velocity = 0
        } else moving = true
      }
      if (runtime.sizingUntil > now) moving = true
    }
    draw(now)
    started = true
    if (pendingPlacements !== null) {
      const placements = pendingPlacements
      pendingPlacements = null
      queueMicrotask(() => fillPlacements(placements))
    }
    if (moving) frame = requestAnimationFrame(tick)
  }

  // The frame is drawn at the animated size, position and scale, and clips its content, which takes the
  // destination size at once, so the content lays out once per destination instead of at every frame.
  function draw(now: number) {
    const desk = state()
    const order = [...desk.windows].sort((a, b) => a.rank - b.rank).map((window) => window.key)
    desk.windows.forEach((window, index) => {
      const runtime = entry(window.key)
      const tileRect = dockTile(index, desk.windows.length, size())
      const shown = shownIn(window.key, runtime.view)
      const focused = shown && desk.focused === window.key
      if (runtime.tile !== null) {
        const style = runtime.tile.style
        style.left = `${tileRect.left}px`
        style.top = `${tileRect.top}px`
        style.width = `${tileRect.width}px`
        style.height = `${tileRect.height}px`
        style.setProperty('--open', shown ? '1' : '0')
        style.setProperty('--focused', focused ? '1' : '0')
        runtime.tile.dataset.state = shown ? (focused ? 'front' : 'open') : 'docked'
      }
      if (runtime.elements === null) return
      const { frame: node, content, badge } = runtime.elements
      const s = runtime.springs
      const width = Math.max(0, s.width.value)
      const height = Math.max(0, s.height.value)
      const scale = Math.min(1, Math.max(0, s.scale.value))
      const landed = !shown && scale === 0
      node.style.width = `${width}px`
      node.style.height = `${height}px`
      node.style.transform = `translate3d(${s.centerX.value - width / 2}px, ${s.centerY.value - height / 2}px, 0) scale(${scale})`
      node.style.opacity = landed ? '0' : '1'
      node.style.visibility = landed ? 'hidden' : 'visible'
      node.style.zIndex = String(10 + order.indexOf(window.key))
      node.style.setProperty('--focused', focused ? '1' : '0')
      node.style.setProperty('--tiled', runtime.view === 'desk' && !window.maximized ? '0' : '1')
      node.toggleAttribute('data-sizing', runtime.sizingUntil > now)
      node.inert = !shown
      runtime.shown = shown
      const goal = visibleRect(window.key, runtime.view)
      content.style.width = `${Math.max(0, goal.width - 2)}px`
      content.style.height = `${Math.max(0, goal.height - TITLE_HEIGHT - 2)}px`
      if (runtime.sizingUntil > now)
        badge.textContent = `${Math.round(width)} × ${Math.round(height)}`
    })
  }

  // Seams sit in the gaps of a tiled view; dragging one moves its split, and the windows on both sides
  // follow at once. The ratio is stored when the seam is let go.
  function placeSeams() {
    const view = state().view
    const seams = view === 'compare' || view === 'evidence' ? tiles(view).seams : []
    while (seamElements.length < seams.length) {
      const element = document.createElement('div')
      element.className = 'desk-seam'
      element.setAttribute('role', 'separator')
      element.setAttribute('aria-label', 'Resize the windows on either side')
      element.tabIndex = 0
      surface.appendChild(element)
      const index = seamElements.length
      let dragging = false
      const seamAt = () => tiles(state().view).seams[index]
      const resizeAround = () => {
        for (const key of tiles(state().view).placed.keys()) {
          settle(key)
          markSizing(key)
        }
        placeSeams()
        schedule()
      }
      element.addEventListener('pointerdown', (event) => {
        event.stopPropagation()
        dragging = true
        element.setPointerCapture(event.pointerId)
        element.setAttribute('data-active', '')
      })
      element.addEventListener('pointermove', (event) => {
        const seam = seamAt()
        if (!dragging || seam === undefined) return
        const box = surface.getBoundingClientRect()
        const at =
          seam.dir === 'row'
            ? (event.clientX - box.left - seam.outer.left) / seam.outer.width
            : (event.clientY - box.top - seam.outer.top) / seam.outer.height
        ratioOverride.set(seam.id, Math.min(0.85, Math.max(0.15, at)))
        layouts.clear()
        resizeAround()
      })
      const end = () => {
        const seam = seamAt()
        dragging = false
        element.removeAttribute('data-active')
        if (seam !== undefined && ratioOverride.has(seam.id)) {
          setRatio(seam.id, ratioOverride.get(seam.id)!)
          ratioOverride.delete(seam.id)
        }
      }
      element.addEventListener('pointerup', end)
      element.addEventListener('pointercancel', end)
      element.addEventListener('keydown', (event) => {
        const seam = seamAt()
        if (seam === undefined) return
        const back = seam.dir === 'row' ? 'ArrowLeft' : 'ArrowUp'
        const forth = seam.dir === 'row' ? 'ArrowRight' : 'ArrowDown'
        if (event.key !== back && event.key !== forth) return
        event.preventDefault()
        const current = ratioOverride.get(seam.id) ?? state().ratios[seam.id] ?? 0.5
        setRatio(seam.id, current + (event.key === forth ? 1 : -1) * (event.shiftKey ? 0.1 : 0.02))
        resizeAround()
      })
      seamElements.push(element)
    }
    seamElements.forEach((element, index) => {
      const seam = seams[index]
      element.hidden = seam === undefined
      if (seam === undefined) return
      element.dataset.dir = seam.dir
      element.setAttribute('aria-orientation', seam.dir === 'row' ? 'vertical' : 'horizontal')
      element.style.left = `${seam.rect.left}px`
      element.style.top = `${seam.rect.top}px`
      element.style.width = `${seam.rect.width}px`
      element.style.height = `${seam.rect.height}px`
    })
  }

  // Leaving a tiled view for the free desk keeps every window where it is drawn. Focus only zooms one
  // window, so leaving it places that window and leaves the rest of the desk as it was.
  const tearOff = () => {
    const view = state().view
    if (view === 'desk') return
    const placed = tiles(view).placed
    for (const runtime of live.values()) {
      clearTimeout(runtime.timer)
      runtime.timer = 0
      runtime.view = 'desk'
    }
    lastView = 'desk'
    placeWindows(placed, { hideOthers: view !== 'focus' })
  }

  const onSurfaceDown = (event: PointerEvent) => {
    if (event.target === surface) blurDesk()
  }
  surface.addEventListener('pointerdown', onSurfaceDown)
  const resize = new ResizeObserver(() => {
    layouts.clear()
    placeSeams()
    schedule()
  })
  resize.observe(surface)
  const unsubscribe = deskStore.subscribe(onStore)

  return {
    register(key: string, elements: DeskWindowElements) {
      const runtime = entry(key)
      runtime.elements = elements
      runtime.fresh = true
      const onDown = () => focusWindow(key)
      elements.frame.addEventListener('pointerdown', onDown)
      elements.frame.addEventListener('focusin', onDown)

      let carry: {
        x: number
        y: number
        started: boolean
        start: Rect | null
        slots: Slots | null
        order: string[]
        follow: Rect | null
      } | null = null
      const end = () => {
        delete elements.titlebar.dataset.dragging
        delete elements.frame.dataset.carried
        document.removeEventListener('keydown', onEscape, true)
        carry = null
      }
      // Escape puts every window back in the slot it held when the carry began.
      const onEscape = (event: KeyboardEvent) => {
        if (event.key !== 'Escape' || carry?.slots == null) return
        event.preventDefault()
        event.stopPropagation()
        preview.clear()
        schedule()
        end()
      }
      const onTitleDown = (event: PointerEvent) => {
        const button = (event.target as Element).closest('button')
        if (event.button !== 0 || (button !== null && button !== elements.handle)) return
        const window = state().windows.find((candidate) => candidate.key === key)
        if (state().view === 'desk' && window?.maximized) return
        carry = {
          x: event.clientX,
          y: event.clientY,
          started: false,
          start: null,
          slots: null,
          order: [],
          follow: null,
        }
        elements.titlebar.setPointerCapture(event.pointerId)
      }
      const onTitleMove = (event: PointerEvent) => {
        if (carry === null) return
        const dx = event.clientX - carry.x
        const dy = event.clientY - carry.y
        if (!carry.started) {
          if (Math.hypot(dx, dy) < 4) return
          const start = visibleRect(key, runtime.view)
          const slots = slotsNow()
          if (slots === null) tearOff()
          else {
            elements.frame.dataset.carried = ''
            document.addEventListener('keydown', onEscape, true)
          }
          carry = { ...carry, started: true, start, slots, order: slots ? [...slots.keys] : [] }
          elements.titlebar.dataset.dragging = ''
        }
        const follow = { ...carry.start!, left: carry.start!.left + dx, top: carry.start!.top + dy }
        carry.follow = follow
        if (carry.slots === null) {
          runtime.override = follow
          settle(key)
          schedule()
          return
        }
        preview.set(key, follow)
        settle(key)
        const cx = follow.left + follow.width / 2
        const cy = follow.top + follow.height / 2
        const index = carry.slots.rects.findIndex(
          (r) => cx >= r.left && cx <= r.left + r.width && cy >= r.top && cy <= r.top + r.height,
        )
        if (index >= 0 && carry.order[index] !== key) {
          carry.order = inserted(carry.order, key, index)
          showOrder(carry.slots, carry.order, key)
        }
        schedule()
      }
      const onTitleUp = () => {
        if (carry?.started && carry.slots === null && runtime.override !== null) {
          const rect = fit(runtime.override, workspaceOf(size()))
          runtime.override = null
          placeWindows(new Map([[key, rect]]))
        } else if (carry?.started && carry.slots !== null) {
          const moved = carry.order.some((other, i) => other !== carry!.slots!.keys[i])
          if (moved) commitOrder(carry.slots, carry.order)
          else {
            preview.clear()
            schedule()
          }
        }
        end()
      }
      // Left and right move one place in reading order; up and down move to the nearest slot in the
      // row above or below. The windows between shift along, as with a drag.
      const onHandleKey = (event: KeyboardEvent) => {
        if (event.altKey || event.ctrlKey || event.metaKey) return
        if (!['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(event.key)) return
        const slots = slotsNow()
        const from = slots?.keys.indexOf(key) ?? -1
        if (slots === null || from < 0) return
        let to = event.key === 'ArrowLeft' ? from - 1 : event.key === 'ArrowRight' ? from + 1 : -1
        if (event.key === 'ArrowUp' || event.key === 'ArrowDown') {
          const own = slots.rects[from]!
          const middle = (r: Rect) => [r.left + r.width / 2, r.top + r.height / 2] as const
          const [ox, oy] = middle(own)
          let best = Infinity
          slots.rects.forEach((r, i) => {
            const [x, y] = middle(r)
            const rows = event.key === 'ArrowUp' ? oy - y : y - oy
            if (i === from || rows <= 1) return
            const distance = rows * 4 + Math.abs(x - ox)
            if (distance < best) {
              best = distance
              to = i
            }
          })
        }
        if (to < 0 || to >= slots.keys.length) return
        event.preventDefault()
        event.stopPropagation()
        commitOrder(slots, inserted(slots.keys, key, to))
      }
      elements.handle.addEventListener('keydown', onHandleKey)
      elements.titlebar.addEventListener('pointerdown', onTitleDown)
      elements.titlebar.addEventListener('pointermove', onTitleMove)
      elements.titlebar.addEventListener('pointerup', onTitleUp)
      elements.titlebar.addEventListener('pointercancel', onTitleUp)

      let resizing: { x: number; y: number; rect: Rect } | null = null
      const onGripDown = (event: PointerEvent) => {
        event.stopPropagation()
        tearOff()
        focusWindow(key)
        resizing = { x: event.clientX, y: event.clientY, rect: deskRect(key) }
        elements.grip.setPointerCapture(event.pointerId)
      }
      const onGripMove = (event: PointerEvent) => {
        if (resizing === null) return
        runtime.override = {
          ...resizing.rect,
          width: Math.max(MIN_WIDTH, resizing.rect.width + event.clientX - resizing.x),
          height: Math.max(MIN_HEIGHT, resizing.rect.height + event.clientY - resizing.y),
        }
        markSizing(key)
        settle(key)
        schedule()
      }
      const onGripUp = () => {
        if (resizing !== null && runtime.override !== null) {
          const rect = fit(runtime.override, workspaceOf(size()))
          runtime.override = null
          placeWindows(new Map([[key, rect]]))
        }
        resizing = null
      }
      elements.grip.addEventListener('pointerdown', onGripDown)
      elements.grip.addEventListener('pointermove', onGripMove)
      elements.grip.addEventListener('pointerup', onGripUp)
      elements.grip.addEventListener('pointercancel', onGripUp)
      schedule()
      return () => {
        elements.frame.removeEventListener('pointerdown', onDown)
        elements.frame.removeEventListener('focusin', onDown)
        elements.handle.removeEventListener('keydown', onHandleKey)
        document.removeEventListener('keydown', onEscape, true)
        elements.titlebar.removeEventListener('pointerdown', onTitleDown)
        elements.titlebar.removeEventListener('pointermove', onTitleMove)
        elements.titlebar.removeEventListener('pointerup', onTitleUp)
        elements.titlebar.removeEventListener('pointercancel', onTitleUp)
        elements.grip.removeEventListener('pointerdown', onGripDown)
        elements.grip.removeEventListener('pointermove', onGripMove)
        elements.grip.removeEventListener('pointerup', onGripUp)
        elements.grip.removeEventListener('pointercancel', onGripUp)
        runtime.elements = null
      }
    },
    registerTile(key: string, element: HTMLElement) {
      entry(key).tile = element
      schedule()
      return () => {
        const runtime = live.get(key)
        if (runtime) runtime.tile = null
      }
    },
    /** Leaves a tiled view for the free desk with every window where it is drawn. */
    freeze: () => tearOff(),
    seamIds(): string[] {
      const view = state().view
      return view === 'compare' || view === 'evidence'
        ? tiles(view).seams.map((seam) => seam.id)
        : []
    },
    workspace: () => workspaceOf(size()),
    refresh() {
      layouts.clear()
      preview.clear()
      placeSeams()
      schedule()
    },
    dispose() {
      cancelAnimationFrame(frame)
      unsubscribe()
      resize.disconnect()
      surface.removeEventListener('pointerdown', onSurfaceDown)
      for (const element of seamElements) element.remove()
      for (const runtime of live.values()) clearTimeout(runtime.timer)
    },
  }
}

export type DeskEngine = ReturnType<typeof createDeskEngine>
