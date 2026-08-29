import type { ReactNode } from 'react'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { isNumericDuckDbType } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'
import { label, literal, num } from '@/components/ui/recipes'
import { formatCount } from '@/lib/format/number'
import { PreviewTable } from './PreviewTable'
import { SchemaTable } from './SchemaTable'
import { useDatasetSummary } from './useDatasetSummary'

/**
 * The Data Studio stage: the physical schema as a sortable, filterable table, the preview as a window
 * onto the whole file, and the preparation cards passed as children. Selecting a column in either
 * table drives the column profile in the inspector.
 */
export function DataProfileView({ source, profile, selectedColumn, onSelectColumn, children }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly selectedColumn: ColumnId | null
  readonly onSelectColumn: (column: ColumnId) => void
  readonly children?: ReactNode
}) {
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType)).length
  const summary = useDatasetSummary(source, profile)
  return (
    <section className="rise @container/studio flex w-full flex-col gap-5" aria-labelledby="data-profile-title">
      <header className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <span className={label('text-signal')}>02 · Data studio</span>
          <h2 id="data-profile-title" className="mb-0 mt-2 text-heading text-ink">Data profile</h2>
          <p className="mb-0 mt-1 text-body text-muted">{profile.source.fileName}</p>
        </div>
        <dl aria-label="Dataset size" className="m-0 grid grid-cols-3 gap-px overflow-hidden rounded-lg border border-hair bg-hair">
          <div className="bg-panel px-3 py-2">
            <dt className={label('text-faint')}>Rows</dt>
            <dd className={num('m-0 mt-1 text-title text-ink')}>{formatCount(profile.rowCount).text}</dd>
          </div>
          <div className="bg-panel px-3 py-2">
            <dt className={label('text-faint')}>Columns</dt>
            <dd className={num('m-0 mt-1 text-title text-ink')}>{profile.columns.length}</dd>
          </div>
          <div className="bg-panel px-3 py-2">
            <dt className={label('text-faint')}>Numeric</dt>
            <dd className={num('m-0 mt-1 text-title text-ink')}>{numericColumns}</dd>
          </div>
        </dl>
      </header>

      <div className="grid gap-3 @5xl/studio:grid-cols-[minmax(420px,0.9fr)_minmax(0,1.4fr)]">
        <SchemaTable profile={profile} summary={summary} selectedColumn={selectedColumn} onSelectColumn={onSelectColumn} />
        <PreviewTable source={source} profile={profile} summary={summary} selectedColumn={selectedColumn} onSelectColumn={onSelectColumn} />
      </div>

      {children}

      <p className={label('m-0 truncate text-faint')} title={profile.source.fingerprint}>
        SHA-256 <span className={literal('normal-case tracking-normal')}>{profile.source.fingerprint.slice(0, 12)}</span> · DuckDB {profile.parser.engineVersion}
      </p>
    </section>
  )
}
