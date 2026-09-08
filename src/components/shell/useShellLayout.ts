import { useMemo, useSyncExternalStore } from 'react'
import type { LayoutStorage } from 'react-resizable-panels'
import { DEFAULT_SHELL_LAYOUT, parseShellLayout, type ShellLayout, type TableDensity } from '@/domain/shellLayout'

const KEY = 'hirmos.shell.v1'

const listeners = new Set<() => void>()
let layout: ShellLayout | null = null

const read = (): ShellLayout => {
  if (layout === null) {
    let raw: string | null = null
    try { raw = localStorage.getItem(KEY) } catch { /* private mode */ }
    layout = parseShellLayout(raw)
  }
  return layout
}

const write = (next: ShellLayout) => {
  layout = next
  try { localStorage.setItem(KEY, JSON.stringify(next)) } catch { /* quota or private mode */ }
  for (const notify of listeners) notify()
}

const subscribe = (notify: () => void): (() => void) => {
  listeners.add(notify)
  return () => { listeners.delete(notify) }
}

export interface ShellLayoutHandle {
  readonly layout: ShellLayout
  /** Storage adapter for `useDefaultLayout`, so one store owns every pane. */
  readonly storage: LayoutStorage
  readonly tableDensity: TableDensity
  readonly setTableDensity: (density: TableDensity) => void
}

export function useShellLayout(): ShellLayoutHandle {
  const current = useSyncExternalStore(subscribe, read, () => DEFAULT_SHELL_LAYOUT)
  const storage = useMemo<LayoutStorage>(() => ({
    getItem: (key) => read().panes[key] ?? null,
    setItem: (key, value) => { write({ ...read(), panes: { ...read().panes, [key]: value } }) },
  }), [])
  return {
    layout: current,
    storage,
    tableDensity: current.tableDensity,
    setTableDensity: (density) => write({ ...read(), tableDensity: density }),
  }
}
