import type { ReactNode } from 'react'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { isNumericDuckDbType } from '@/domain/dataset'
import { chapterLabel } from '@/domain/navigation'
import type { SelectedSource } from '@/domain/workflow'
import { FigureParts } from '@/components/ui/figures'
import { Icon } from '@/components/Icon'
import { figureGrid, iconControl, label, literal, num } from '@/components/ui/recipes'
import { formatCount } from '@/lib/format/number'
import { PreviewTable } from './PreviewTable'
import { SchemaTable } from './SchemaTable'
import { useDatasetSummary } from './useDatasetSummary'

/**
 * The Data Studio stage: the physical schema as a sortable, filterable table, the preview as a window
 * onto the whole file, and the preparation cards passed as children. Selecting a column in either
 * table drives the column profile in the inspector.
 */
export function DataProfileView({ source, profile, selectedColumn, onSelectColumn, onEditSource, children }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly selectedColumn: ColumnId | null
  readonly onSelectColumn: (column: ColumnId) => void
  readonly onEditSource: (() => void) | null
  readonly children?: ReactNode
}) {
  const numericColumns = profile.columns.filter((column) => isNumericDuckDbType(column.duckdbType)).length
  const summary = useDatasetSummary(source, profile)
  const sizeFigures = [
    { name: 'Rows', figure: formatCount(profile.rowCount) },
    { name: 'Columns', figure: formatCount(profile.columns.length) },
    { name: 'Numeric', figure: formatCount(numericColumns) },
  ]
  return (
    <section className="rise @container/studio flex w-full flex-col gap-5" aria-labelledby="data-profile-title">
      <header className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <span className={label('text-faint')}>{chapterLabel('data')}</span>
          <h2 id="data-profile-title" className="mb-0 mt-2 text-heading text-ink">Data profile</h2>
          <p className="mb-0 mt-1 flex items-center gap-1 text-body text-muted">
            <span>{profile.source.fileName}</span>
            {source.recipe.kind !== 'uploaded-file' && onEditSource !== null && (
              <button type="button" className={iconControl('quiet', 'h-7 w-7')} aria-label={source.recipe.kind === 'sql-derived' ? 'Edit SQL' : 'Edit pipeline'} title={source.recipe.kind === 'sql-derived' ? 'Edit the SQL that made this source' : 'Edit the pipeline that made this source'} onClick={onEditSource}>
                <Icon name="edit" size={15} />
              </button>
            )}
          </p>
        </div>
        <dl aria-label="Dataset size" className={figureGrid('m-0 w-full grid-cols-3 @2xl/studio:w-auto')}>
          {sizeFigures.map(({ name, figure }) => (
            <div key={name} className="bg-panel px-3 py-2">
              <dt className={label('text-faint')}>{name}</dt>
              <dd className={num('m-0 mt-1 text-title leading-none tracking-tight text-ink')} title={figure.exact}><FigureParts value={figure} /></dd>
            </div>
          ))}
        </dl>
      </header>

      <div className="grid gap-3 @5xl/studio:grid-cols-[minmax(420px,0.9fr)_minmax(0,1.4fr)]">
        <SchemaTable profile={profile} summary={summary} selectedColumn={selectedColumn} onSelectColumn={onSelectColumn} />
        <PreviewTable source={source} profile={profile} summary={summary} selectedColumn={selectedColumn} onSelectColumn={onSelectColumn} />
      </div>

      {children}

      <p className="m-0 truncate text-label text-faint" title={profile.source.fingerprint}>
        SHA-256 <span className={literal()}>{profile.source.fingerprint.slice(0, 12)}</span> · DuckDB {profile.parser.engineVersion}
      </p>
    </section>
  )
}
