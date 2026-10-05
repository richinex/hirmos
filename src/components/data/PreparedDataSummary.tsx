import type { DatasetProfile } from '@/domain/dataset'
import { Icon } from '@/components/Icon'
import { assertNever } from '@/domain/dop'
import { describeSeriesTransform, type PreparedDatasetArtifact } from '@/domain/preprocessing'
import { describeResampling } from '@/domain/resampling'
import { describeSeasonalAdjustment } from '@/domain/seasonal'
import { table, th, td, tr } from '@/components/ui/recipes'
import { formatCount } from '@/lib/format/number'

function structure(prepared: PreparedDatasetArtifact): string {
  switch (prepared.kind) {
    case 'prepared-time-series':
      return 'Time series'
    case 'prepared-panel':
      return 'Panel'
    case 'prepared-cross-section':
      return 'Cross-section'
    default:
      return assertNever(prepared)
  }
}

export function PreparedDataSummary({
  prepared,
  profile,
}: {
  readonly prepared: PreparedDatasetArtifact
  readonly profile: DatasetProfile
}) {
  const name = (id: DatasetProfile['columns'][number]['id']) =>
    profile.columns.find((column) => column.id === id)?.name ?? String(id)
  const details = [
    prepared.kind === 'prepared-time-series' ? describeResampling(prepared.resampling, name) : null,
    describeSeasonalAdjustment(prepared.seasonalAdjustment, name),
    prepared.kind === 'prepared-panel'
      ? `${formatCount(prepared.panel.units).text} units × ${formatCount(prepared.panel.periods).text} ${prepared.sampling.frequency} periods, keyed by ${name(prepared.panel.unitColumn)} and ${name(prepared.panel.timeColumn)}`
      : null,
  ].filter((detail): detail is string => detail !== null)
  const transforms =
    prepared.kind === 'prepared-time-series'
      ? prepared.seriesTransforms.filter((record) => record.transform.kind !== 'levels')
      : []
  return (
    <section aria-label="Prepared data" className="mb-5">
      <h3 className="m-0 flex items-center gap-2 text-body font-medium text-ink">
        <Icon name="table_view" size={16} />
        Prepared data
      </h3>
      <p role="status" className="mb-0 mt-1 text-body text-muted">
        {structure(prepared)},{' '}
        <span className="tabular-nums">{formatCount(prepared.observations).text}</span> rows
      </p>
      {(details.length > 0 || transforms.length > 0) && (
        <details className="group mt-2">
          <summary className="flex cursor-pointer list-none items-center gap-1.5 text-body text-muted [&::-webkit-details-marker]:hidden">
            <Icon
              name="chevron_right"
              size={16}
              className="transition-transform group-open:rotate-90"
            />
            Preparation details
          </summary>
          <div className="mt-2 space-y-2">
            {details.map((detail, index) => (
              <p key={index} className="m-0 text-label text-faint">
                {detail}
              </p>
            ))}
            {transforms.length > 0 && (
              <div className="overflow-x-auto">
                <table className={table} aria-label="Prepared variable transformations">
                  <thead>
                    <tr>
                      <th className={th()}>Variable</th>
                      <th className={th()}>Transformation</th>
                    </tr>
                  </thead>
                  <tbody>
                    {transforms.map((record) => (
                      <tr key={record.column} className={tr()}>
                        <td className={td('whitespace-normal break-words text-ink')}>
                          {name(record.column)}
                        </td>
                        <td className={td('whitespace-normal text-muted')}>
                          {describeSeriesTransform(record.transform)}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </details>
      )}
    </section>
  )
}
