import { useEffect, useState } from 'react'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { TimeInterpretation, TimePreview as Preview } from '@/domain/timeInterpretation'
import { previewTimeColumnInWorker } from '@/data/client'

type State = { readonly kind: 'loading' } | { readonly kind: 'failed'; readonly detail: string } | { readonly kind: 'ready'; readonly preview: Preview }
const SOURCE_TYPE: TimeInterpretation = { kind: 'source-type' }

export function TimePreview({ file, profile, column, interpretation = SOURCE_TYPE }: {
  readonly file: File
  readonly profile: DatasetProfile
  readonly column: ColumnId
  readonly interpretation?: TimeInterpretation
}) {
  const [state, setState] = useState<State>({ kind: 'loading' })
  useEffect(() => {
    let current = true
    setState({ kind: 'loading' })
    void previewTimeColumnInWorker(file, profile, column, interpretation).then((result) => {
      if (!current) return
      setState(result.ok ? { kind: 'ready', preview: result.value } : { kind: 'failed', detail: 'The time preview could not be read. Check the selected column and format.' })
    })
    return () => { current = false }
  }, [file, profile, column, interpretation])

  if (state.kind === 'loading') return <p role="status" className="text-body text-muted">Reading time preview…</p>
  if (state.kind === 'failed') return <Alert tone="danger">{state.detail}</Alert>
  const columns: readonly EvidenceColumn<Preview['rows'][number]>[] = [
    { id: 'original', header: 'Source value', value: (row) => row.original ?? 'Missing' },
    { id: 'parsed', header: state.preview.kind === 'calendar' ? 'Interpreted time (UTC)' : 'Time index', value: (row) => {
      if (row.parsed === null) return row.original === null ? 'Missing' : 'Invalid'
      return state.preview.kind === 'ordinal' ? row.parsed : new Date(row.parsed).toISOString().replace('T', ' ').replace('.000Z', '')
    } },
  ]
  return <EvidenceTable title="Time preview" help="First 12 source rows. Preparation validates each time value and checks for duplicate time points." rows={state.preview.rows} columns={columns} rowKey={(_, index) => String(index)} noun="row" empty="No source rows." frame="none" />
}
