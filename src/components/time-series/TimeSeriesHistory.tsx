import { useState } from 'react'
import { Icon } from '@/components/Icon'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { iconControl } from '@/components/ui/recipes'
import { formatTime } from '@/lib/format/date'
import type { CountSeriesModelArtifact } from '@/domain/countSeries'
import { timeSeriesRunLabel, type TimeSeriesRun } from '@/domain/timeSeries'
import { CountSeriesRecord } from './CountSeriesCard'
import { TimeSeriesRunResult } from './TimeSeriesRunResult'

type Entry = CountSeriesModelArtifact | TimeSeriesRun

export function TimeSeriesHistory<T extends Entry>({ entries, onDelete }: {
  readonly entries: readonly T[]
  readonly onDelete: (entry: T) => void
}) {
  const [pending, setPending] = useState<T | null>(null)
  return <>
    <ul className="m-0 list-none divide-y divide-hair p-0 text-body" aria-label="Time-series runs">
      {entries.length === 0 && <li className="px-3 py-2 text-faint">No runs yet.</li>}
      {[...entries].reverse().map((entry) => <li key={entry.id} className="flex items-start gap-2 px-3 py-2">
        <details className="min-w-0 flex-1">
          <summary className="cursor-pointer text-body text-ink"><span className="[overflow-wrap:anywhere]">{entry.kind === 'count-series-model' ? `Count model · ${entry.outcome.name}` : timeSeriesRunLabel(entry)}</span><span className="ml-3 whitespace-nowrap text-label tabular-nums text-faint">{formatTime(entry.createdAt)}</span></summary>
          <div className="mt-3">{entry.kind === 'count-series-model' ? <ul className="m-0 list-none p-0"><CountSeriesRecord artifact={entry} open /></ul> : <TimeSeriesRunResult run={entry} />}</div>
        </details>
        <button type="button" className={iconControl('quiet', 'shrink-0')} aria-label={`Delete ${entry.kind === 'count-series-model' ? 'count-model run' : timeSeriesRunLabel(entry)}`} onClick={() => setPending(entry)}><Icon name="delete" size={16} /></button>
      </li>)}
    </ul>
    <ConfirmDialog open={pending !== null} title="Delete time-series run?" message="This removes the saved result. The prepared data and other runs are unchanged." confirmLabel="Delete run" danger onClose={() => setPending(null)} onConfirm={() => { if (pending !== null) onDelete(pending); setPending(null) }} />
  </>
}
