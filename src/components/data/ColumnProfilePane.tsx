import { useMemo } from 'react'
import { EChart } from '@/charts/EChart'
import { histogramOption } from '@/charts/data/histogram'
import { useChartTheme } from '@/charts/theme'
import { Alert } from '@/components/ui/Alert'
import { label, literal, num } from '@/components/ui/recipes'
import type { ColumnProfile, ColumnProfileProblem, DatasetProfile, PhysicalColumnProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { formatCount, formatPercent, formatStatistic } from '@/lib/format/number'
import type { ColumnDescription } from './useColumnProfile'

const describeProblem = (problem: ColumnProfileProblem): string => {
  switch (problem.kind) {
    case 'column-not-found': return 'Choose another column. The selected column is no longer part of this dataset.'
    case 'source-changed': return 'The file changed after it was profiled. Choose it again to refresh the profile.'
    case 'column-profile-failed':
    case 'worker-protocol-failed':
    case 'worker-unavailable': return problem.detail
    default: return assertNever(problem)
  }
}

function Stat({ name, value, tone = 'ink' }: { readonly name: string; readonly value: string; readonly tone?: 'ink' | 'muted' | 'warn' }) {
  return (
    <div className="bg-panel px-3 py-2.5">
      <dt className={label('text-faint')}>{name}</dt>
      <dd className={num(`m-0 mt-1 text-title ${tone === 'warn' ? 'text-warn' : tone === 'muted' ? 'text-muted' : 'text-ink'}`)}>{value}</dd>
    </div>
  )
}

/** The hairline stat list: cells separated by the `hair` colour showing through a 1px gap. */
function StatList({ children }: { readonly children: React.ReactNode }) {
  return <dl className="m-0 grid grid-cols-2 gap-px overflow-hidden rounded-lg border border-hair bg-hair @max-[300px]/inspector:grid-cols-1">{children}</dl>
}

function NumericSummary({ column, profile, rowCount }: { readonly column: PhysicalColumnProfile; readonly profile: Extract<ColumnProfile, { readonly kind: 'numeric-column-profile' }>; readonly rowCount: number }) {
  const theme = useChartTheme()
  const option = useMemo(() => histogramOption({ name: column.name, edges: profile.histogram.edges, counts: profile.histogram.counts, nullCount: profile.nullCount }, theme), [column.name, profile, theme])
  const missing = formatPercent(profile.nullCount / rowCount, { numerator: profile.nullCount, denominator: rowCount })
  return (
    <>
      <StatList>
        <Stat name="Values" value={formatCount(profile.count).text} />
        <Stat name="Missing" value={missing.text} tone={profile.nullCount > 0 ? 'warn' : 'muted'} />
        <Stat name="Distinct" value={formatCount(profile.distinctCount).text} />
        <Stat name="Zeros" value={formatCount(profile.zeroCount).text} tone="muted" />
        <Stat name="Mean" value={formatStatistic('mean', profile.mean).text} />
        <Stat name="Std. deviation" value={profile.standardDeviation === null ? '—' : formatStatistic('sd', profile.standardDeviation).text} />
        <Stat name="Minimum" value={formatStatistic('raw', profile.min).text} />
        <Stat name="Maximum" value={formatStatistic('raw', profile.max).text} />
        <Stat name="Lower quartile" value={formatStatistic('raw', profile.quartiles.lower).text} tone="muted" />
        <Stat name="Median" value={formatStatistic('raw', profile.quartiles.median).text} />
        <Stat name="Upper quartile" value={formatStatistic('raw', profile.quartiles.upper).text} tone="muted" />
        <Stat name="Bins" value={formatCount(profile.histogram.counts.length).text} tone="muted" />
      </StatList>
      <div className="mt-3">
        <p className={label('mb-1 text-faint')}>Distribution</p>
        <EChart option={option} label={`Histogram of ${column.name}`} className="h-[clamp(160px,28cqb,220px)]" testId="column-histogram" />
      </div>
    </>
  )
}

function CategoricalSummary({ profile, rowCount }: { readonly profile: Extract<ColumnProfile, { readonly kind: 'categorical-column-profile' }>; readonly rowCount: number }) {
  const missing = formatPercent(profile.nullCount / rowCount, { numerator: profile.nullCount, denominator: rowCount })
  const peak = Math.max(1, ...profile.top.map((entry) => entry.count))
  return (
    <>
      <StatList>
        <Stat name="Values" value={formatCount(profile.count).text} />
        <Stat name="Missing" value={missing.text} tone={profile.nullCount > 0 ? 'warn' : 'muted'} />
        <Stat name="Distinct" value={formatCount(profile.distinctCount).text} />
        <Stat name="Shown" value={`top ${profile.top.length}`} tone="muted" />
      </StatList>
      <div className="mt-3">
        <p className={label('mb-2 text-faint')}>Most frequent values</p>
        <ol className="m-0 list-none space-y-1.5 p-0" aria-label="Most frequent values">
          {profile.top.map((entry) => (
            <li key={entry.value} className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-x-3 gap-y-0.5 text-body">
              <span className="truncate text-ink" title={entry.value}>{entry.value}</span>
              <span className={num('text-muted')}>{formatCount(entry.count).text}</span>
              <span aria-hidden className="col-span-2 h-1 rounded-full bg-hair"><span className="block h-full rounded-full bg-bone" style={{ width: `${(entry.count / peak) * 100}%` }} /></span>
            </li>
          ))}
        </ol>
      </div>
    </>
  )
}

function Skeleton() {
  return (
    <div aria-hidden className="space-y-3">
      <div className="grid grid-cols-2 gap-px overflow-hidden rounded-lg border border-hair bg-hair">
        {Array.from({ length: 6 }, (_, index) => <div key={index} className="bg-panel px-3 py-2.5"><div className="skeleton h-2.5 w-14 rounded" /><div className="skeleton mt-2 h-4 w-20 rounded" /></div>)}
      </div>
      <div className="skeleton h-40 rounded-lg" />
    </div>
  )
}

/** The inspector body for the Data Studio: what one column contains, with its distribution. */
export function ColumnProfilePane({ profile, column, description }: {
  readonly profile: DatasetProfile
  readonly column: PhysicalColumnProfile | null
  readonly description: ColumnDescription
}) {
  if (column === null) {
    return <p className="m-0 text-body text-faint">Select a column to inspect its values, missing values and distribution.</p>
  }
  return (
    <div className="@container/inspector">
      <div className="mb-3">
        <p className={label('m-0 text-faint')}>Column</p>
        <h3 className="mb-0 mt-1 truncate text-title font-medium text-ink" title={column.name}>{column.name}</h3>
        <p className={literal('mb-0 mt-1 text-micro text-faint')}>{column.duckdbType}{column.nullable ? '' : ' · not null'}</p>
      </div>
      {description.kind === 'loading' && <Skeleton />}
      {description.kind === 'idle' && <Skeleton />}
      {description.kind === 'failed' && description.column === column.id && (
        <Alert tone="danger" title="Column profile refused">{describeProblem(description.problem)}</Alert>
      )}
      {description.kind === 'ready' && description.column === column.id && (
        description.profile.kind === 'numeric-column-profile'
          ? <NumericSummary column={column} profile={description.profile} rowCount={profile.rowCount} />
          : <CategoricalSummary profile={description.profile} rowCount={profile.rowCount} />
      )}
    </div>
  )
}
