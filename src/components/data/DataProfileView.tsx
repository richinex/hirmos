import type { ReactNode } from 'react'
import { assertNever } from '@/domain/dop'
import type { DatasetProfile, PreviewCell } from '@/domain/dataset'
import { label, literal, num } from '@/components/ui/recipes'

const cellText = (cell: PreviewCell): string => {
  switch (cell.kind) {
    case 'null': return 'null'
    case 'number': return String(cell.value)
    case 'integer': return cell.value
    case 'boolean': return cell.value ? 'true' : 'false'
    case 'temporal': return cell.value
    case 'text': return cell.value
    default: return assertNever(cell)
  }
}

export function DataProfileView({
  profile,
  children,
}: {
  readonly profile: DatasetProfile
  readonly children?: ReactNode
}) {
  return (
    <section className="rise w-full" aria-labelledby="data-profile-title">
      <span className={label('text-signal')}>02 · Data studio</span>
      <div className="mb-5 mt-3 flex flex-wrap items-end justify-between gap-3">
        <div>
          <h2 id="data-profile-title" className="m-0 text-heading text-ink">Data profile</h2>
          <p className="mb-0 mt-1 text-body text-muted">{profile.source.fileName}</p>
        </div>
        <dl className="m-0 flex gap-6">
          <div>
            <dt className={label('text-faint')}>Rows</dt>
            <dd className={num('m-0 mt-1 text-title font-medium text-ink')}>{profile.rowCount.toLocaleString()}</dd>
          </div>
          <div>
            <dt className={label('text-faint')}>Columns</dt>
            <dd className={num('m-0 mt-1 text-title font-medium text-ink')}>{profile.columns.length}</dd>
          </div>
        </dl>
      </div>

      {children}

      <div className={`${children ? 'mt-8 border-t border-line pt-7' : ''} grid gap-4 xl:grid-cols-[minmax(260px,0.7fr)_minmax(0,1.3fr)]`}>
        <section className="overflow-hidden rounded-xl border border-line bg-panel" aria-labelledby="physical-schema-title">
          <h3 id="physical-schema-title" className={label('m-0 border-b border-hair px-3 py-2.5 text-muted')}>Physical schema</h3>
          <div className="figure-strip panel-scroll max-h-[420px] overflow-auto">
            <table className="w-full border-collapse text-left text-body">
              <thead className="sticky top-0 bg-panel text-faint">
                <tr>
                  <th className="border-b border-hair px-3 py-2 font-normal">Column</th>
                  <th className="border-b border-hair px-3 py-2 font-normal">Type</th>
                  <th className="border-b border-hair px-3 py-2 text-right font-normal">Nulls</th>
                </tr>
              </thead>
              <tbody>
                {profile.columns.map((column) => (
                  <tr key={column.id} className="border-b border-hair last:border-0">
                    <td className="max-w-48 truncate px-3 py-2 text-ink" title={column.name}>{column.name}</td>
                    <td className={literal('px-3 py-2 text-faint')}>{column.duckdbType}</td>
                    <td className={num('px-3 py-2 text-right text-faint')}>{column.nullCount}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>

        <section className="min-w-0 overflow-hidden rounded-xl border border-line bg-panel" aria-labelledby="source-preview-title">
          <h3 id="source-preview-title" className={label('m-0 border-b border-hair px-3 py-2.5 text-muted')}>Preview</h3>
          <div className="panel-scroll max-h-[420px] overflow-auto">
            <table className="min-w-full border-collapse text-left text-body">
              <thead className="sticky top-0 bg-panel text-faint">
                <tr>
                  {profile.columns.map((column) => (
                    <th key={column.id} className="whitespace-nowrap border-b border-r border-hair px-3 py-2 font-normal last:border-r-0">{column.name}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {profile.preview.map((row, rowIndex) => (
                  <tr key={rowIndex} className="border-b border-hair last:border-0">
                    {row.map((cell, columnIndex) => (
                      <td
                        key={profile.columns[columnIndex]?.id ?? columnIndex}
                        className={`${cell.kind === 'null' ? 'italic text-faint' : 'text-ink'} whitespace-nowrap border-r border-hair px-3 py-2 last:border-r-0`}
                      >
                        {cellText(cell)}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <p className={literal('mb-0 mt-3 truncate text-micro text-faint')} title={profile.source.fingerprint}>
        SHA-256 {profile.source.fingerprint.slice(0, 12)} · DuckDB {profile.parser.engineVersion}
      </p>
    </section>
  )
}
