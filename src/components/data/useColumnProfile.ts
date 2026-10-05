import { useEffect, useRef, useState } from 'react'
import type {
  ColumnId,
  ColumnProfile,
  ColumnProfileProblem,
  DatasetProfile,
} from '@/domain/dataset'
import { isNumericDuckDbType } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'

export type ColumnSeries = {
  readonly column: ColumnId
  /** Row order; NaN marks a missing cell. */
  readonly values: Float64Array
  readonly missingCells: number
}

export type ColumnDescription =
  | { readonly kind: 'idle' }
  | { readonly kind: 'loading'; readonly column: ColumnId }
  | {
      readonly kind: 'ready'
      readonly column: ColumnId
      readonly profile: ColumnProfile
      readonly series: ColumnSeries | null
    }
  | { readonly kind: 'failed'; readonly column: ColumnId; readonly problem: ColumnProfileProblem }

const seriesFromMatrix = (
  matrix: {
    readonly values: Float64Array
    readonly validity: Uint8Array
    readonly rowCount: number
    readonly missingCells: number
  },
  column: ColumnId,
): ColumnSeries => {
  const values = new Float64Array(matrix.rowCount)
  for (let row = 0; row < matrix.rowCount; row += 1) {
    const valid = (matrix.validity[row >> 3] >> (row & 7)) & 1
    values[row] = valid === 1 ? matrix.values[row] : Number.NaN
  }
  return { column, values, missingCells: matrix.missingCells }
}

/**
 * Describe the selected column through the data worker, once per column per source. Results are kept
 * for the life of the profile so switching back to a column is instant.
 */
export function useColumnProfile(
  source: SelectedSource | null,
  profile: DatasetProfile | null,
  column: ColumnId | null,
): ColumnDescription {
  const cache = useRef(
    new Map<ColumnId, Extract<ColumnDescription, { readonly kind: 'ready' | 'failed' }>>(),
  )
  const [state, setState] = useState<ColumnDescription>({ kind: 'idle' })

  const profileId = profile?.id ?? null
  useEffect(() => {
    cache.current = new Map()
  }, [profileId])

  useEffect(() => {
    if (column === null || profile === null || source === null) {
      setState({ kind: 'idle' })
      return
    }
    const cached = cache.current.get(column)
    if (cached !== undefined) {
      setState(cached)
      return
    }
    let cancelled = false
    setState({ kind: 'loading', column })
    const physical = profile.columns.find((candidate) => candidate.id === column)
    const numeric = physical !== undefined && isNumericDuckDbType(physical.duckdbType)
    void (async () => {
      const client = await import('@/data/client')
      const [described, matrix] = await Promise.all([
        client.profileColumnInWorker(source.file, profile, column),
        numeric
          ? client.materializeNumericColumnsInWorker(source.file, profile, [column])
          : Promise.resolve(null),
      ])
      if (cancelled) return
      const next: Extract<ColumnDescription, { readonly kind: 'ready' | 'failed' }> = described.ok
        ? {
            kind: 'ready',
            column,
            profile: described.value,
            series: matrix !== null && matrix.ok ? seriesFromMatrix(matrix.value, column) : null,
          }
        : { kind: 'failed', column, problem: described.error }
      cache.current.set(column, next)
      setState(next)
    })()
    return () => {
      cancelled = true
    }
  }, [column, profile, source])

  return state
}
