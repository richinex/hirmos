import { useState, type ReactNode } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import type { SelectedSource } from '@/domain/workflow'
import { ColumnProfilePane } from './ColumnProfilePane'
import { ColumnSeriesPane } from './ColumnSeriesPane'
import { DataProfileView } from './DataProfileView'
import { useColumnProfile } from './useColumnProfile'
import { PreparedDataSummary } from './PreparedDataSummary'

/** The Data Studio chapter: schema and preview on the stage, the selected column described beside and below. */
export function DataStudio({ source, profile, prepared, onEditSource, children }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact | null
  /** Reopens the editor that made the source; null for an uploaded file. */
  readonly onEditSource: (() => void) | null
  /** The preparation cards and the continue card, rendered under the tables. */
  readonly children?: ReactNode
}) {
  const [chosen, setChosen] = useState<ColumnId | null>(null)
  const selected = profile.columns.find((column) => column.id === chosen) ?? profile.columns[0]
  const description = useColumnProfile(source, profile, selected.id)
  const stepLabel = prepared?.kind === 'prepared-time-series' ? 'observation' : prepared?.kind === 'prepared-panel' ? 'panel row' : 'row'
  return (
    <WorkbenchLayout
      id="data"
      stage={(
        <DataProfileView source={source} profile={profile} selectedColumn={selected.id} onSelectColumn={setChosen} onEditSource={onEditSource}>
          {children}
        </DataProfileView>
      )}
      inspector={{ trigger: { label: 'Profile', icon: 'query_stats' }, title: 'Column profile', body: <>{prepared !== null && <PreparedDataSummary prepared={prepared} profile={profile} />}<ColumnProfilePane key={profile.id} profile={profile} column={selected} description={description} /></> }}
      bottom={{ trigger: { label: 'Series', icon: 'show_chart' }, title: 'Series', body: <ColumnSeriesPane key={profile.id} column={selected} description={description} stepLabel={stepLabel} /> }}
    />
  )
}
