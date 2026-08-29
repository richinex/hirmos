import { useCallback, useEffect, useRef, useState } from 'react'
import { PREVIEW_WINDOW_LIMIT, type DatasetProfile, type PreviewFilter, type PreviewRow, type PreviewSort, type PreviewWindowProblem } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'

export interface PreviewShape {
  readonly sort: PreviewSort | null
  readonly filters: readonly PreviewFilter[]
  readonly search: string
}

export interface PreviewWindows {
  /** Rows matching the shape over the whole file; null until the first window answers. */
  readonly total: number | null
  readonly rowAt: (index: number) => PreviewRow | null
  /** Fetch every window overlapping [first, last]; already-fetched and in-flight windows are skipped. */
  readonly ensure: (first: number, last: number) => void
  readonly problem: PreviewWindowProblem | null
  readonly loading: boolean
}

const shapeKey = (shape: PreviewShape): string => JSON.stringify(shape)

/** Windows of the sorted, filtered file, keyed by offset; a new shape drops them all. */
export function usePreviewWindows(source: SelectedSource, profile: DatasetProfile, shape: PreviewShape): PreviewWindows {
  const key = shapeKey(shape)
  const [windows, setWindows] = useState<{ readonly key: string; readonly total: number | null; readonly rows: ReadonlyMap<number, readonly PreviewRow[]>; readonly problem: PreviewWindowProblem | null }>({ key, total: null, rows: new Map(), problem: null })
  const inFlight = useRef(new Set<string>())
  const [loading, setLoading] = useState(0)

  const fetchWindow = useCallback((offset: number, forKey: string, current: PreviewShape) => {
    const token = `${forKey}:${offset}`
    if (inFlight.current.has(token)) return
    inFlight.current.add(token)
    setLoading((count) => count + 1)
    void (async () => {
      const client = await import('@/data/client')
      const result = await client.previewWindowInWorker(source.file, profile, { offset, limit: PREVIEW_WINDOW_LIMIT, ...current })
      inFlight.current.delete(token)
      setLoading((count) => count - 1)
      setWindows((existing) => {
        if (existing.key !== forKey) return existing
        if (!result.ok) return { ...existing, problem: result.error }
        const rows = new Map(existing.rows)
        rows.set(offset, result.value.rows)
        return { key: forKey, total: result.value.total, rows, problem: null }
      })
    })()
  }, [profile, source])

  useEffect(() => {
    setWindows({ key, total: null, rows: new Map(), problem: null })
    fetchWindow(0, key, shape)
    // The shape is captured through its key so a new object with the same content does not refetch.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key, fetchWindow])

  const rowAt = useCallback((index: number): PreviewRow | null => {
    const offset = Math.floor(index / PREVIEW_WINDOW_LIMIT) * PREVIEW_WINDOW_LIMIT
    return windows.rows.get(offset)?.[index - offset] ?? null
  }, [windows.rows])

  const ensure = useCallback((first: number, last: number) => {
    const from = Math.max(0, Math.floor(first / PREVIEW_WINDOW_LIMIT) * PREVIEW_WINDOW_LIMIT)
    const to = Math.max(0, Math.floor(last / PREVIEW_WINDOW_LIMIT) * PREVIEW_WINDOW_LIMIT)
    for (let offset = from; offset <= to; offset += PREVIEW_WINDOW_LIMIT) {
      if (!windows.rows.has(offset)) fetchWindow(offset, key, shape)
    }
    // Same reasoning as above: the shape's identity is its key.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchWindow, key, windows.rows])

  return { total: windows.key === key ? windows.total : null, rowAt, ensure, problem: windows.problem, loading: loading > 0 }
}
