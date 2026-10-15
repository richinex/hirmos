import { useStore } from 'zustand'
import { createStore } from 'zustand/vanilla'
import { readDesk, writeDesk } from '@/components/shell/useShellLayout'
import { parseRoute } from '@/domain/navigation'
import {
  DESK_CAPACITY,
  deskKey,
  parseDeskKey,
  parseDeskState,
  type DeskState,
  type DeskViewName,
  type DeskWindowRef,
  type DeskWindowState,
} from '@/domain/resultsDesk'
import type { Rect } from './layout'

interface DeskStore {
  readonly project: string | null
  readonly desk: DeskState
  /** The window that last left the desk to keep it within capacity, for the status line. */
  readonly dropped: string | null
}

export const emptyDesk = (): DeskState => ({
  version: 1,
  windows: [{ key: 'runs', placed: null, visible: true, maximized: false, rank: 0 }],
  view: 'desk',
  ratios: {},
  focused: 'runs',
  pair: { selected: null, compareWith: null },
})

export const deskStore = createStore<DeskStore>(() => ({
  project: null,
  desk: emptyDesk(),
  dropped: null,
}))

/** The project the address bar names, the same source the shell reopens it from. */
const openProject = (): string | null => {
  const route = parseRoute(window.location.pathname, window.location.search)
  return route.ok && route.value.kind === 'chapter' ? route.value.project : null
}

const nextRank = (windows: readonly DeskWindowState[]) =>
  windows.reduce((most, window) => Math.max(most, window.rank), -1) + 1

/** Adds or brings back a window, then keeps the desk within capacity without dropping any key in `keep`. */
function withWindow(
  desk: DeskState,
  ref: DeskWindowRef,
  keep: ReadonlySet<string> = new Set(),
): { desk: DeskState; dropped: string | null } {
  const key = deskKey(ref)
  const rank = nextRank(desk.windows)
  const present = desk.windows.some((window) => window.key === key)
  let windows = present
    ? desk.windows.map((window) =>
        window.key === key ? { ...window, visible: true, rank } : window,
      )
    : [...desk.windows, { key, placed: null, visible: true, maximized: false, rank }]
  let dropped: string | null = null
  if (windows.length > DESK_CAPACITY) {
    const candidates = windows
      .filter((window) => window.key !== key && window.key !== 'runs' && !keep.has(window.key))
      .sort((a, b) => Number(a.visible) - Number(b.visible) || a.rank - b.rank)
    dropped = candidates[0]?.key ?? null
    windows = windows.filter((window) => window.key !== dropped)
  }
  return { desk: { ...desk, windows, focused: key }, dropped }
}

const update = (change: (desk: DeskState) => DeskState) =>
  deskStore.setState((state) => ({ desk: change(state.desk) }))

const save = () => {
  const { project, desk } = deskStore.getState()
  if (project !== null) writeDesk(project, desk)
}
deskStore.subscribe((state, previous) => {
  if (state.desk !== previous.desk) save()
})

/**
 * Loads a project's desk, or starts it from `seed` when none is saved, then drops windows whose
 * records no longer exist.
 */
export function loadDesk(
  project: string,
  seed: () => DeskState,
  exists: (ref: DeskWindowRef) => boolean,
): void {
  const saved = parseDeskState(readDesk(project))
  const desk = saved ?? seed()
  const windows = desk.windows.filter((window) => {
    const ref = parseDeskKey(window.key)
    return ref !== null && exists(ref)
  })
  const keys = new Set(windows.map((window) => window.key))
  const withRuns = keys.has('runs')
    ? windows
    : [
        { key: 'runs', placed: null, visible: true, maximized: false, rank: nextRank(windows) },
        ...windows,
      ]
  deskStore.setState({
    project,
    dropped: null,
    desk: {
      ...desk,
      windows: withRuns,
      focused: desk.focused !== null && keys.has(desk.focused) ? desk.focused : 'runs',
    },
  })
}

export function openWindow(ref: DeskWindowRef): void {
  deskStore.setState((state) => {
    const next = withWindow({ ...state.desk, view: 'desk' }, ref)
    return { desk: next.desk, dropped: next.dropped }
  })
}

/** From another chapter: the result goes on the open project's desk, saved or loaded. */
export function putOnDesk(ref: DeskWindowRef): void {
  const current = openProject()
  if (current === null) return
  if (deskStore.getState().project === current) {
    openWindow(ref)
    return
  }
  const saved = parseDeskState(readDesk(current)) ?? emptyDesk()
  writeDesk(current, withWindow({ ...saved, view: 'desk' }, ref).desk)
}

export const closeWindow = (key: string): void =>
  deskStore.setState((state) => ({
    dropped: null,
    desk: closed(state.desk, key),
  }))

const closed = (desk: DeskState, key: string): DeskState => ({
  ...desk,
  view: 'desk',
  windows: key === 'runs' ? desk.windows : desk.windows.filter((window) => window.key !== key),
  focused:
    desk.focused === key
      ? topVisible(desk.windows.filter((window) => window.key !== key))
      : desk.focused,
})

const topVisible = (windows: readonly DeskWindowState[]): string | null =>
  [...windows].filter((window) => window.visible).sort((a, b) => b.rank - a.rank)[0]?.key ?? null

export const minimizeWindow = (key: string): void =>
  update((desk) => {
    const windows = desk.windows.map((window) =>
      window.key === key ? { ...window, visible: false, maximized: false } : window,
    )
    return {
      ...desk,
      view: 'desk',
      windows,
      focused: desk.focused === key ? topVisible(windows) : desk.focused,
    }
  })

export const focusWindow = (key: string): void =>
  update((desk) =>
    desk.focused === key &&
    desk.windows.find((window) => window.key === key)?.rank === nextRank(desk.windows) - 1
      ? desk
      : {
          ...desk,
          focused: key,
          windows: desk.windows.map((window) =>
            window.key === key ? { ...window, rank: nextRank(desk.windows) } : window,
          ),
        },
  )

export const blurDesk = (): void =>
  update((desk) => (desk.focused === null ? desk : { ...desk, focused: null }))

export const toggleMaximize = (key: string): void =>
  update((desk) => ({
    ...desk,
    focused: key,
    windows: desk.windows.map((window) =>
      window.key === key
        ? { ...window, maximized: !window.maximized, rank: nextRank(desk.windows) }
        : window,
    ),
  }))

/** Stores rectangles as the user placed them and shows those windows on the free desk; `hideOthers` docks the rest. */
export const placeWindows = (
  rects: ReadonlyMap<string, Rect>,
  options: { readonly hideOthers?: boolean } = {},
): void =>
  update((desk) => ({
    ...desk,
    view: 'desk',
    windows: desk.windows.map((window) => {
      const rect = rects.get(window.key)
      if (rect === undefined) return options.hideOthers ? { ...window, visible: false } : window
      return { ...window, placed: rect, maximized: false, visible: true }
    }),
  }))

export const fillPlacements = (rects: ReadonlyMap<string, Rect>): void =>
  update((desk) => ({
    ...desk,
    windows: desk.windows.map((window) => {
      const rect = rects.get(window.key)
      return window.placed === null && rect !== undefined ? { ...window, placed: rect } : window
    }),
  }))

export const setView = (view: DeskViewName, open: readonly DeskWindowRef[] = []): void =>
  deskStore.setState((state) => {
    let desk = state.desk
    let dropped: string | null = null
    const needed = new Set(open.map(deskKey))
    for (const ref of open) {
      const key = deskKey(ref)
      if (!desk.windows.some((window) => window.key === key)) {
        const next = withWindow(desk, ref, needed)
        desk = next.desk
        dropped = next.dropped ?? dropped
        desk = {
          ...desk,
          windows: desk.windows.map((window) =>
            window.key === key ? { ...window, visible: false } : window,
          ),
        }
      }
    }
    return { desk: { ...desk, view, focused: state.desk.focused }, dropped }
  })

export const setRatio = (id: string, ratio: number): void =>
  update((desk) => ({
    ...desk,
    ratios: { ...desk.ratios, [id]: Math.min(0.85, Math.max(0.15, ratio)) },
  }))

export const resetRatios = (ids: readonly string[]): void =>
  update((desk) => ({
    ...desk,
    ratios: Object.fromEntries(Object.entries(desk.ratios).filter(([id]) => !ids.includes(id))),
  }))

export const setPair = (pair: DeskState['pair']): void => update((desk) => ({ ...desk, pair }))

export function useDesk<T>(select: (state: DeskStore) => T): T {
  return useStore(deskStore, select)
}
