import { useMemo, useState } from 'react'
import { EChart } from '@/charts/EChart'
import { histogramOption } from '@/charts/data/histogram'
import { useChartTheme } from '@/charts/theme'
import { Alert } from '@/components/ui/Alert'
import { FigureParts } from '@/components/ui/figures'
import { IdentityRow } from '@/components/ui/IdentityRow'
import { figureGrid, label, literal, num, sectionTitle } from '@/components/ui/recipes'
import { ShareBar } from '@/components/table/primitives'
import type { ColumnProfile, ColumnProfileProblem, DatasetProfile, PhysicalColumnProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { formatAbsent, formatCount, formatPercent, formatStatistic, formatWords, type Formatted } from '@/lib/format/number'
import type { ColumnDescription } from './useColumnProfile'

const describeProblem = (problem: ColumnProfileProblem): string => {
  switch (problem.kind) {
    case 'column-not-found': return 'Choose another column. The selected column is no longer part of this dataset.'
    case 'source-changed': return 'The file changed after profiling. Choose it again to refresh the profile.'
    case 'column-profile-failed':
    case 'worker-protocol-failed':
    case 'worker-unavailable': return problem.detail
    default: return assertNever(problem)
  }
}

function Stat({ name, value, tone = 'ink' }: { readonly name: string; readonly value: Formatted; readonly tone?: 'ink' | 'muted' | 'warn' }) {
  return (
    <div className="bg-panel px-3 py-2">
      <dt className={label('text-faint')}>{name}</dt>
      <dd className={num(`m-0 mt-0.5 text-body ${tone === 'warn' ? 'text-warn' : tone === 'muted' ? 'text-muted' : 'text-ink'}`)} title={value.exact || value.srText}><FigureParts value={value} /></dd>
    </div>
  )
}

/** A compact definition list on the shared numeric surface. */
function StatList({ children }: { readonly children: React.ReactNode }) {
  return <dl className={figureGrid('m-0 grid-cols-2 @max-[300px]/inspector:grid-cols-1')}>{children}</dl>
}

function NumericSummary({ column, profile, rowCount }: { readonly column: PhysicalColumnProfile; readonly profile: Extract<ColumnProfile, { readonly kind: 'numeric-column-profile' }>; readonly rowCount: number }) {
  const theme = useChartTheme()
  // The rules mark the mean and the median the list beside the chart prints, so the shape and the figures read together.
  const option = useMemo(() => histogramOption({
    name: column.name,
    bins: profile.histogram,
    nullCount: profile.nullCount,
    marks: [{ name: 'mean', value: profile.mean }, { name: 'median', value: profile.quartiles.median }],
  }, theme), [column.name, profile, theme])
  const missing = formatPercent(profile.nullCount / rowCount, { numerator: profile.nullCount, denominator: rowCount })
  return (
    <>
      <StatList>
        <Stat name="Values" value={formatCount(profile.count)} />
        <Stat name="Missing" value={missing} tone={profile.nullCount > 0 ? 'warn' : 'muted'} />
        <Stat name="Distinct" value={formatCount(profile.distinctCount)} />
        <Stat name="Zeros" value={formatCount(profile.zeroCount)} tone="muted" />
        <Stat name="Mean" value={formatStatistic('mean', profile.mean)} />
        <Stat name="Std. deviation" value={profile.standardDeviation === null ? formatAbsent('unavailable') : formatStatistic('sd', profile.standardDeviation)} />
        <Stat name="Minimum" value={formatStatistic('raw', profile.min)} />
        <Stat name="Maximum" value={formatStatistic('raw', profile.max)} />
        <Stat name="Lower quartile" value={formatStatistic('raw', profile.quartiles.lower)} tone="muted" />
        <Stat name="Median" value={formatStatistic('raw', profile.quartiles.median)} />
        <Stat name="Upper quartile" value={formatStatistic('raw', profile.quartiles.upper)} tone="muted" />
        <Stat name="Bins" value={formatCount(profile.histogram.counts.length)} tone="muted" />
      </StatList>
      <div className="mt-3">
        <h4 className={`${sectionTitle} mb-1 mt-0`}>Distribution</h4>
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
        <Stat name="Values" value={formatCount(profile.count)} />
        <Stat name="Missing" value={missing} tone={profile.nullCount > 0 ? 'warn' : 'muted'} />
        <Stat name="Distinct" value={formatCount(profile.distinctCount)} />
        <Stat name="Shown" value={formatWords(`top ${profile.top.length}`)} tone="muted" />
      </StatList>
      <div className="mt-3">
        <h4 className={`${sectionTitle} mb-2 mt-0`}>Most frequent values</h4>
        <ol className="m-0 list-none space-y-1.5 p-0" aria-label="Most frequent values">
          {profile.top.map((entry) => (
            <li key={entry.value} className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-x-3 gap-y-0.5 text-body">
              <span className="truncate text-ink" title={entry.value}>{entry.value}</span>
              <span className={num('text-muted')}>{formatCount(entry.count).text}</span>
              <ShareBar share={entry.count / peak} className="col-span-2" />
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
      <div className={figureGrid('grid-cols-2')}>
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
  const [displayed, setDisplayed] = useState<{ readonly column: PhysicalColumnProfile; readonly profile: ColumnProfile } | null>(null)
  const current = description.kind !== 'idle' && description.column === column?.id
  if (current && description.kind === 'ready' && column !== null && displayed?.profile !== description.profile) {
    setDisplayed({ column, profile: description.profile })
  }
  if (column === null) {
    return <p className="m-0 text-body text-faint">Select a column to inspect its values, missing values and distribution.</p>
  }
  const failed = current && description.kind === 'failed'
  const shown = failed ? column : displayed?.column ?? column
  return (
    <div className="@container/inspector" aria-busy={!current || description.kind === 'loading'}>
      <div className="mb-3">
        <IdentityRow name={<h3 className="m-0 text-title font-medium text-ink">{shown.name}</h3>}>
          <span className={literal()}>{shown.duckdbType}</span>
          <span>{shown.nullable ? 'Nullable' : 'Not null'}</span>
        </IdentityRow>
      </div>
      {failed ? <Alert tone="danger" title="Column profile refused">{describeProblem(description.problem)}</Alert>
        : displayed === null ? <Skeleton />
        : displayed.profile.kind === 'numeric-column-profile'
          ? <NumericSummary column={displayed.column} profile={displayed.profile} rowCount={profile.rowCount} />
          : <CategoricalSummary profile={displayed.profile} rowCount={profile.rowCount} />}
    </div>
  )
}
