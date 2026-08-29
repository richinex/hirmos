import type { ChapterId } from './navigation'

/** A run in flight, reported by the chapter that owns it so the rail's busy bar and the header chip can show it. */
export interface RunActivity {
  readonly label: string
  /** 0 to 1 when the run reports progress, null while it cannot. */
  readonly progress: number | null
}

export type ChapterActivity = Partial<Record<ChapterId, RunActivity>>
