import { useEffect, useState } from 'react'
import { EvidenceTable } from '@/components/table/EvidenceTable'
import { Alert } from '@/components/ui/Alert'
import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Select } from '@/components/ui/Select'
import { field, fieldLabel } from '@/components/ui/recipes'
import { previewTimeColumnInWorker } from '@/data/client'
import type { CalendarReport as Report, CalendarRequest } from '@/domain/calendar'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { TimeInterpretation } from '@/domain/timeInterpretation'
import type { Frequency } from '@/domain/preprocessing'

type State = { kind: 'loading' } | { kind: 'unavailable'; detail: string } | { kind: 'ready'; report: Report }
const SOURCE_TYPE: TimeInterpretation = { kind: 'source-type' }
const WEEKDAYS = ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'] as const
const ALIGNMENTS = {
  daily: [{ value: 'daily', label: 'Daily' }],
  weekly: WEEKDAYS.map((day) => ({ value: day, label: day[0].toUpperCase() + day.slice(1) })),
  monthly: [{ value: 'month-start', label: 'Month start' }, { value: 'month-end', label: 'Month end' }],
  quarterly: [{ value: 'quarter-start', label: 'Quarter start (Jan, Apr, Jul, Oct)' }, { value: 'quarter-end', label: 'Quarter end (Mar, Jun, Sep, Dec)' }],
  yearly: [{ value: 'year-start', label: 'Year start (1 January)' }, { value: 'year-end', label: 'Year end (31 December)' }],
} satisfies Record<Frequency, readonly { value: CalendarRequest['schedule']; label: string }[]>

export function CalendarReport(props: {
  readonly file: File
  readonly profile: DatasetProfile
  readonly column: ColumnId
  readonly unitColumn?: ColumnId
  readonly interpretation?: TimeInterpretation
  readonly frequency: Frequency
}) {
  const [open, setOpen] = useState(false)
  return <details className="mt-3" onToggle={(event) => setOpen(event.currentTarget.open)} data-testid="calendar-report">
    <DisclosureSummary>Calendar coverage</DisclosureSummary>
    {open && <Coverage key={props.frequency} {...props} />}
  </details>
}

function Coverage({ file, profile, column, unitColumn, interpretation = SOURCE_TYPE, frequency }: Parameters<typeof CalendarReport>[0]) {
  const options = ALIGNMENTS[frequency]
  const [alignment, setAlignment] = useState<CalendarRequest['schedule']>(options[0].value)
  const [state, setState] = useState<State>({ kind: 'loading' })
  const schedule = frequency === 'daily' ? 'daily' : alignment
  useEffect(() => {
    let current = true
    setState({ kind: 'loading' })
    void previewTimeColumnInWorker(file, profile, column, interpretation, { schedule, unitColumn }).then((result) => {
      if (!current) return
      if (!result.ok) { setState({ kind: 'unavailable', detail: result.error.kind === 'materialization-failed' ? result.error.detail : 'The calendar could not be inspected. Check the selected time and unit columns.' }); return }
      setState(result.value.calendar === undefined
        ? { kind: 'unavailable', detail: 'Numeric time indices do not identify calendar dates. Choose a date interpretation to inspect calendar coverage.' }
        : { kind: 'ready', report: result.value.calendar })
    })
    return () => { current = false }
  }, [file, profile, column, unitColumn, interpretation, schedule])

  return <div className="mt-3 space-y-3">
    {frequency !== 'daily' && <label className="block sm:max-w-xs">
      <span className={fieldLabel}>Calendar alignment</span>
      <Select aria-label="Calendar alignment" className={field('text', 'mt-1')} value={alignment} onChange={(event) => {
        const selected = options.find((option) => option.value === event.target.value)
        if (selected !== undefined) setAlignment(selected.value)
      }}>
        {options.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
      </Select>
    </label>}
    <p className="m-0 text-body text-muted">Checks between the first and last observation{unitColumn === undefined ? '' : ' of each unit'} in UTC. No rows are added or values filled.</p>
    {state.kind === 'loading' && <p role="status" className="m-0 text-body text-muted">Checking calendar coverage…</p>}
    {state.kind === 'unavailable' && <Alert tone="info">{state.detail}</Alert>}
    {state.kind === 'ready' && <>
      <Alert tone={state.report.issues.length > 0 || state.report.missing > 0 ? 'warn' : 'ok'}>
        {state.report.issues.length > 0
          ? `${state.report.units - state.report.issues.length} of ${state.report.units} series checked. Resolve the date issues below to check the remaining series.`
          : state.report.missing === 0 ? 'No interior calendar gaps found.' : `${state.report.missing.toLocaleString()} missing calendar ${state.report.missing === 1 ? 'period' : 'periods'} across ${state.report.ranges.toLocaleString()} ${state.report.ranges === 1 ? 'gap' : 'gaps'}.`}
      </Alert>
      {state.report.gaps.length > 0 && <EvidenceTable title="Missing periods" rows={state.report.gaps} columns={[
        ...(unitColumn === undefined ? [] : [{ id: 'unit', header: 'Unit', value: (row: Report['gaps'][number]) => row.unit }]),
        { id: 'dates', header: 'Missing dates', value: (row) => row.first === row.last ? row.first : `${row.first} to ${row.last}` },
        { id: 'count', header: 'Periods', value: (row) => row.count, align: 'right' },
      ]} rowKey={(_, index) => String(index)} noun="gap" empty="No gaps." frame="none" />}
      {state.report.ranges > state.report.gaps.length && <p className="m-0 text-body text-muted">Showing the first 200 gaps. The total includes all gaps.</p>}
      {state.report.issues.length > 0 && <EvidenceTable title="Calendar issues" rows={state.report.issues} columns={[
        { id: 'unit', header: 'Series', value: (row) => row.unit },
        { id: 'detail', header: 'Issue', value: (row) => row.detail },
      ]} rowKey={(_, index) => String(index)} noun="issue" empty="No issues." frame="none" />}
    </>}
  </div>
}
