import { useEffect, useState } from 'react'

/**
 * The geometry half of a floating tool window: clamp, persist, restore.
 *
 * Insets are parameters rather than constants because a window that sits below the app header needs a
 * different top margin from one that floats free.
 *
 * Geometry only: each floating surface keeps its own chrome.
 */
export interface Rect { x: number; y: number; width: number; height: number }

export interface RectInsets {
  /** Minimum distance from the top — the inspector sits below the app header. Default 8. */
  top?: number
  /** Horizontal breathing room on each side. Default 8. */
  side?: number
  /** Space kept clear at the bottom when clamping height. Default 32. */
  bottom?: number
}

/** Keep a stored or default rect inside the CURRENT viewport: a rect saved on a wide desktop would
 * otherwise place the window entirely off-screen on a phone. */
export function clampRect(r: Rect, insets: RectInsets = {}): Rect {
  const top = insets.top ?? 8
  const side = insets.side ?? 8
  const bottom = insets.bottom ?? 32
  const width = Math.min(r.width, window.innerWidth - side * 2)
  const height = Math.min(r.height, window.innerHeight - top - bottom)
  return {
    width,
    height,
    x: Math.min(Math.max(side, r.x), Math.max(side, window.innerWidth - width - side)),
    y: Math.min(Math.max(top, r.y), Math.max(top, window.innerHeight - height - side)),
  }
}

function load(key: string): Rect | null {
  try {
    const raw = localStorage.getItem(key)
    if (!raw) return null
    const j: unknown = JSON.parse(raw)
    if (!j || typeof j !== 'object') return null
    const o = j as Record<string, unknown>
    const num = (v: unknown) => (typeof v === 'number' && Number.isFinite(v) ? v : null)
    const x = num(o.x), y = num(o.y), width = num(o.width), height = num(o.height)
    // Parsed, not cast: a half-written rect would otherwise place the window at NaN and vanish it.
    return x !== null && y !== null && width !== null && height !== null ? { x, y, width, height } : null
  } catch { return null }
}

function save(key: string, r: Rect): void {
  try { localStorage.setItem(key, JSON.stringify(r)) } catch { /* quota */ }
}

/**
 * A draggable/resizable window's rect, clamped on open and persisted on change.
 * `storageKey` null means "do not persist" (the compile flow's expanded canvas, which is transient).
 */
export function useFloatingRect(storageKey: string | null, initial: () => Rect, insets?: RectInsets) {
  const [rect, setRect] = useState<Rect>(() => clampRect(storageKey ? load(storageKey) ?? initial() : initial(), insets))
  const top = insets?.top ?? 8
  const side = insets?.side ?? 8
  const bottom = insets?.bottom ?? 32
  useEffect(() => {
    const resize = () => setRect((current) => {
      const next = clampRect(current, { top, side, bottom })
      return next.x === current.x && next.y === current.y && next.width === current.width && next.height === current.height ? current : next
    })
    window.addEventListener('resize', resize)
    return () => window.removeEventListener('resize', resize)
  }, [top, side, bottom])
  const update = (r: Rect) => {
    setRect(r)
    if (storageKey) save(storageKey, r)
  }
  return {
    rect,
    position: { x: rect.x, y: rect.y },
    size: { width: rect.width, height: rect.height },
    /** react-rnd onDragStop */
    onDragStop: (x: number, y: number) => update({ ...rect, x, y }),
    /** react-rnd onResizeStop */
    onResizeStop: (width: number, height: number, x: number, y: number) => update({ x, y, width, height }),
  }
}
