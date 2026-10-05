import type { ReactNode } from 'react'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import { isNumericDuckDbType } from '@/domain/dataset'
import type { SelectedSource } from '@/domain/workflow'
import type { DeclaredType } from '@/domain/fileReading'
import { FigureParts } from '@/components/ui/figures'
import { Metadata } from '@/components/ui/Metadata'
import { Icon } from '@/components/Icon'
import { iconControl, literal, num } from '@/components/ui/recipes'
import { formatCount } from '@/lib/format/number'
import { PreviewTable } from './PreviewTable'
import { SchemaTable } from './SchemaTable'
import { useDatasetSummary } from './useDatasetSummary'

/**
 * The Data Studio stage: the physical schema as a sortable, filterable table, the preview as a window
 * onto the whole file, and the preparation cards passed as children. Selecting a column in either
 * table drives the column profile in the inspector.
 */
export function DataProfileView({
  source,
  profile,
  selectedColumn,
  onSelectColumn,
  onEditSource,
  onDeclare,
  children,
}: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly selectedColumn: ColumnId | null
  readonly onSelectColumn: (column: ColumnId) => void
  readonly onEditSource: (() => void) | null
  readonly onDeclare?: (column: string, type: DeclaredType | null) => void
  readonly children?: ReactNode
}) {
  const numericColumns = profile.columns.filter((column) =>
    isNumericDuckDbType(column.duckdbType),
  ).length
  const summary = useDatasetSummary(source, profile)
  const sizeFigures = [
    { name: 'Rows', figure: formatCount(profile.rowCount) },
    { name: 'Columns', figure: formatCount(profile.columns.length) },
    { name: 'Numeric', figure: formatCount(numericColumns) },
  ]
  return (
    <section
      className="rise @container/studio flex w-full flex-col gap-5"
      aria-labelledby="data-profile-title"
    >
      <header className="flex min-w-0 flex-wrap items-center justify-between gap-x-6 gap-y-3">
        <div className="flex min-w-0 max-w-full items-center gap-2">
          <Icon name="description" size={22} className="text-muted" />
          <h2
            id="data-profile-title"
            className="m-0 min-w-0 text-heading text-ink [overflow-wrap:anywhere]"
          >
            {profile.source.fileName}
          </h2>
          {onEditSource !== null && (
            <button
              type="button"
              className={iconControl('quiet', 'h-7 w-7')}
              aria-label="Edit data"
              title="Edit data"
              onClick={onEditSource}
            >
              <Icon name="edit" size={15} />
            </button>
          )}
        </div>
        <dl
          aria-label="Dataset size"
          className="m-0 flex flex-wrap items-center gap-x-5 gap-y-2 text-label"
        >
          {sizeFigures.map(({ name, figure }) => (
            <div key={name} className="flex items-baseline gap-1">
              <dt className="text-muted">{name.toLowerCase()}</dt>
              <dd className={num('order-first m-0 text-ink')} title={figure.exact}>
                <FigureParts value={figure} />
              </dd>
            </div>
          ))}
        </dl>
      </header>

      <div className="grid gap-3 @5xl/studio:grid-cols-[minmax(420px,0.9fr)_minmax(0,1.4fr)]">
        <SchemaTable
          profile={profile}
          summary={summary}
          selectedColumn={selectedColumn}
          onSelectColumn={onSelectColumn}
          onDeclare={onDeclare}
        />
        <PreviewTable
          source={source}
          profile={profile}
          summary={summary}
          selectedColumn={selectedColumn}
          onSelectColumn={onSelectColumn}
        />
      </div>

      {children}

      <p className="m-0 text-label text-faint" title={profile.source.fingerprint}>
        <Metadata>
          <span>
            SHA-256 <span className={literal()}>{profile.source.fingerprint.slice(0, 12)}</span>
          </span>
          <span>DuckDB {profile.parser.engineVersion}</span>
        </Metadata>
      </p>
    </section>
  )
}
