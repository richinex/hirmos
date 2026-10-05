import { useEffect, useState } from 'react'
import type {
  ColumnId,
  ColumnSummary,
  DatasetProfile,
  DatasetSummaryProblem,
} from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'

export type DatasetSummaryState =
  | { readonly kind: 'loading' }
  | { readonly kind: 'ready'; readonly byColumn: ReadonlyMap<ColumnId, ColumnSummary> }
  | { readonly kind: 'failed'; readonly problem: DatasetSummaryProblem }

/** The per-column summaries behind the schema table, fetched once per profile after the first paint. */
export function useDatasetSummary(
  source: SelectedSource,
  profile: DatasetProfile,
): DatasetSummaryState {
  const [state, setState] = useState<DatasetSummaryState>({ kind: 'loading' })
  useEffect(() => {
    let cancelled = false
    setState({ kind: 'loading' })
    void (async () => {
      const client = await import('@/data/client')
      const result = await client.summarizeColumnsInWorker(source.file, profile)
      if (cancelled) return
      setState(
        result.ok
          ? {
              kind: 'ready',
              byColumn: new Map(result.value.columns.map((column) => [column.column, column])),
            }
          : { kind: 'failed', problem: result.error },
      )
    })()
    return () => {
      cancelled = true
    }
  }, [profile, source])
  return state
}
