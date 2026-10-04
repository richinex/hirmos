import { useState, type ReactNode } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { preparedColumnRole, type PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { ColumnProfilePane } from './ColumnProfilePane'
import { ColumnSeriesPane } from './ColumnSeriesPane'
import { DataProfileView } from './DataProfileView'
import { useColumnProfile } from './useColumnProfile'
import { PreparedDataSummary } from './PreparedDataSummary'
import { ConfirmDialog } from '@/components/ui/ConfirmDialog'
import { declaredTypeLabel, type DeclaredType } from '@/domain/fileReading'

/** The Data Studio chapter: schema and preview on the stage, the selected column described beside and below. */
type Declaration = { readonly column: string; readonly type: DeclaredType | null }

export function DataStudio({ source, profile, prepared, onEditSource, onDeclare, children }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact | null
  /** Reopens the editor that made the source; null for an uploaded file. */
  readonly onEditSource: (() => void) | null
  /** Reads the file again with one column declared, which profiles it from the start. */
  readonly onDeclare: (column: string, type: DeclaredType | null) => void
  /** The preparation cards and the continue card, rendered under the tables. */
  readonly children?: ReactNode
}) {
  const [chosen, setChosen] = useState<ColumnId | null>(null)
  // A prepared dataset and every run built on it are cleared by a new reading, so that waits for a yes.
  const [pending, setPending] = useState<Declaration | null>(null)
  const declare = (column: string, type: DeclaredType | null) => prepared === null ? onDeclare(column, type) : setPending({ column, type })
  const selected = profile.columns.find((column) => column.id === chosen) ?? profile.columns[0]
  const description = useColumnProfile(source, profile, selected.id)
  const role = preparedColumnRole(prepared, selected.id)
  const stepLabel = prepared?.kind === 'prepared-time-series' ? 'observation' : prepared?.kind === 'prepared-panel' ? 'panel row' : 'row'
  return (
    <WorkbenchLayout
      id="data"
      stage={(
        <>
          <DataProfileView source={source} profile={profile} selectedColumn={selected.id} onSelectColumn={setChosen} onEditSource={onEditSource} onDeclare={declare}>
            {children}
          </DataProfileView>
          <ConfirmDialog
            open={pending !== null}
            title="Read the file again?"
            message={pending === null ? '' : `Reading ${pending.column} ${pending.type === null ? 'as detected' : `as ${declaredTypeLabel(pending.type).toLowerCase()}`} profiles the file again and clears the prepared dataset and its analysis runs. The file is unchanged.`}
            confirmLabel="Read again"
            danger
            onClose={() => setPending(null)}
            onConfirm={() => { if (pending !== null) onDeclare(pending.column, pending.type); setPending(null) }}
          />
        </>
      )}
      inspector={{ trigger: { label: 'Profile', icon: 'query_stats' }, title: 'Column profile', body: <>{prepared !== null && <PreparedDataSummary prepared={prepared} profile={profile} />}<ColumnProfilePane key={profile.id} profile={profile} column={selected} description={description} role={role} /></> }}
      bottom={{ trigger: { label: 'Series', icon: 'show_chart' }, title: 'Series', body: <ColumnSeriesPane key={profile.id} column={selected} description={description} stepLabel={stepLabel} role={role} /> }}
    />
  )
}
