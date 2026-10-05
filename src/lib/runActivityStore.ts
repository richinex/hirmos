import { useStore } from 'zustand'
import { createStore } from 'zustand/vanilla'
import type { ChapterActivity, RunActivity } from '@/domain/activity'
import { CHAPTER_IDS, type ChapterId } from '@/domain/navigation'

interface RunActivityState {
  readonly activity: ChapterActivity
}

/** Runs in flight, kept outside the workbench tree so only the rail's bar and the header chip redraw. */
const store = createStore<RunActivityState>(() => ({ activity: {} }))

// Progress arrives once per resample: show the first at once, then at most one more a frame.
const held = new Map<ChapterId, RunActivity | null>()
let frame: number | null = null

function flush(): void {
  if (held.size === 0) return
  const current = store.getState().activity
  let next = current
  for (const [chapter, run] of held) {
    if (run === null) {
      if (!(chapter in next)) continue
      const { [chapter]: _ended, ...rest } = next
      next = rest
      continue
    }
    next = { ...next, [chapter]: run }
  }
  held.clear()
  if (next !== current) store.setState({ activity: next })
}

function reportRunActivity(chapter: ChapterId, run: RunActivity | null): void {
  held.set(chapter, run)
  if (frame !== null) return
  frame = requestAnimationFrame(() => {
    frame = null
    flush()
  })
  flush()
}

/** One reporter per chapter, built once, so a panel's `onActivity` prop keeps its identity. */
export const REPORT_ACTIVITY: Record<ChapterId, (run: RunActivity | null) => void> =
  Object.fromEntries(
    CHAPTER_IDS.map((chapter) => [
      chapter,
      (run: RunActivity | null) => reportRunActivity(chapter, run),
    ]),
  ) as Record<ChapterId, (run: RunActivity | null) => void>

/** The chapter of the first run in flight, or null when nothing is running. */
export function useRunningChapter(): ChapterId | null {
  return useStore(
    store,
    (state) => CHAPTER_IDS.find((chapter) => state.activity[chapter] !== undefined) ?? null,
  )
}

/** The run a chapter is reporting, or null when it is not running. */
export function useChapterRun(chapter: ChapterId | null): RunActivity | null {
  return useStore(store, (state) => (chapter === null ? null : (state.activity[chapter] ?? null)))
}

/** How far a chapter's run has got, 0 to 1, or null when it is not running or cannot say. */
export function useChapterBusy(chapter: ChapterId): number | null {
  return useStore(store, (state) => {
    const run = state.activity[chapter]
    return run === undefined ? null : (run.progress ?? 0)
  })
}
