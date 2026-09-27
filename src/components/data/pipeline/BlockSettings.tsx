import { Metadata } from '@/components/ui/Metadata'
import { lazy, Suspense, useEffect, useRef } from 'react'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { Orb } from '@/components/ui/Orb'
import { Select } from '@/components/ui/Select'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
import { button, caption, field, fieldLabel, iconControl, literal, num } from '@/components/ui/recipes'
import { blockSql, calendarShareName, calendarWindowDays, conditionWithTest, measureWithFunction, type AggregateFunction, type AggregateMeasure, type CalendarWindow, type JoinKind, type PipelineBlock, type PipelineBlockId, type PipelineNode, type RowCondition, type RowTest } from '@/domain/pipeline'
import type { PreviewColumn } from '@/data/pipeline'
import { CALENDAR_TIME_INTERPRETATIONS } from '@/domain/timeInterpretation'
import { usePythonSession } from './PythonProvider'
import { formatDuration } from '@/lib/format/number'
import { usePythonRun, usePythonRuntime } from './usePythonRun'
import { assertNever } from '@/domain/dop'

/** CodeMirror and the Python mode load with the first script block, not with the canvas. */
const PythonEditor = lazy(() => import('@/components/ui/PythonEditor').then((module) => ({ default: module.PythonEditor })))
export const preloadPythonEditor = (): void => { void import('@/components/ui/PythonEditor') }
/** The calendar and its date library load with the first custom window, not with the canvas. */
const CalendarWindowPicker = lazy(() => import('@/components/ui/CalendarWindowPicker').then((module) => ({ default: module.CalendarWindowPicker })))

const ROW_TEST_OPTIONS = (
  <>
    <option value="eq">equals</option><option value="ne">does not equal</option><option value="lt">is less than</option><option value="le">is at most</option><option value="gt">is greater than</option><option value="ge">is at least</option><option value="contains">contains</option><option value="starts-with">starts with</option><option value="is-null">is missing</option><option value="not-null">is present</option>
  </>
)
const AGGREGATE_OPTIONS = (
  <>
    <option value="count">count rows</option><option value="count-distinct">count distinct</option><option value="sum">sum</option><option value="mean">mean</option><option value="min">minimum</option><option value="max">maximum</option><option value="first">first</option>
  </>
)
const JOIN_OPTIONS = (
  <>
    <option value="inner">Inner: rows matched on both sides</option><option value="left">Left: every row of input 1</option><option value="right">Right: every row of input 2</option><option value="full">Full: every row of either</option><option value="cross">Cross: every pair</option>
  </>
)
const SORT_OPTIONS = <><option value="ascending">ascending</option><option value="descending">descending</option></>

/** Input columns are in port order. */
export function BlockSettings({ node, inputColumns, inputNames, onChange }: {
  readonly node: PipelineNode
  readonly inputColumns: readonly (readonly PreviewColumn[])[]
  readonly inputNames: readonly string[]
  readonly onChange: (block: PipelineBlock) => void
}) {
  const block = node.block
  const firstInputColumns = inputColumns[0] ?? []
  const rowsOf = (name: string): number => { const value = (block as unknown as Record<string, unknown>)[name]; return Array.isArray(value) ? value.length : 0 }
  const conditionsKeys = useRowKeys(`${node.id}:conditions`, rowsOf('conditions'))
  const renamesKeys = useRowKeys(`${node.id}:renames`, rowsOf('renames'))
  const columnsKeys = useRowKeys(`${node.id}:columns`, rowsOf('columns'))
  const keysKeys = useRowKeys(`${node.id}:keys`, rowsOf('keys'))
  const measuresKeys = useRowKeys(`${node.id}:measures`, rowsOf('measures'))
  const sortKeys = useRowKeys(`${node.id}:sort`, rowsOf('sort'))
  switch (block.kind) {
    case 'input': return null
    case 'output': return null
    case 'filter-rows': return (
      <div className="space-y-3">
        <Rows label="Keep rows where" onAdd={() => onChange({ ...block, conditions: [...block.conditions, { column: firstInputColumns[0]?.name ?? '', test: 'eq', value: '' }] })} addLabel="Add a condition">
          {block.conditions.map((condition, index) => {
            const replace = (next: RowCondition) => onChange({ ...block, conditions: block.conditions.map((c, i) => i === index ? next : c) })
            return (
              <div key={conditionsKeys.at(index)} className="grid grid-cols-[1fr_auto] gap-1.5">
                <div className="grid grid-cols-2 gap-1.5">
                  <ColumnPick label={`Condition ${index + 1} column`} columns={firstInputColumns} value={condition.column} onChange={(column) => replace({ ...condition, column })} />
                  <Select aria-label={`Condition ${index + 1} test`} className={field('text')} value={condition.test} onChange={(event) => replace(conditionWithTest(condition, event.target.value as RowTest))}>
                    {ROW_TEST_OPTIONS}
                  </Select>
                  {'value' in condition && <input aria-label={`Condition ${index + 1} value`} className={field('mono', 'col-span-2')} value={String(condition.value)} onChange={(event) => replace({ ...condition, value: numberOrText(event.target.value) })} placeholder="value" />}
                </div>
                <RemoveRow label={`Remove condition ${index + 1}`} index={index} onRemove={() => { conditionsKeys.removed(index); onChange({ ...block, conditions: block.conditions.filter((_, i) => i !== index) }) }} />
              </div>
            )
          })}
        </Rows>
        {block.conditions.length > 1 && (
          <SegmentedControl size="sm" ariaLabel="Conditions combine" value={block.match} onChange={(match) => onChange({ ...block, match })} options={[{ value: 'all', label: 'All must hold' }, { value: 'any', label: 'Any may hold' }]} />
        )}
      </div>
    )
    case 'select-columns': return (
      <div className="space-y-3">
        <SegmentedControl size="sm" ariaLabel="Column selection" value={block.mode} onChange={(mode) => onChange({ ...block, mode })} options={[{ value: 'keep', label: 'Keep these' }, { value: 'drop', label: 'Drop these' }]} />
        <div className="grid gap-1" role="group" aria-label="Columns">
          {firstInputColumns.map((column) => {
            const checked = block.columns.includes(column.name)
            return (
              <label key={column.name} className="flex items-center gap-2 text-body text-ink">
                <input type="checkbox" checked={checked} onChange={() => onChange({ ...block, columns: checked ? block.columns.filter((c) => c !== column.name) : [...block.columns, column.name] })} />
                <span className={literal('truncate')}>{column.name}</span>
                <span className="ml-auto text-micro text-faint">{column.type}</span>
              </label>
            )
          })}
          {firstInputColumns.length === 0 && <p className={caption('m-0')}>Wire an input in to see its columns.</p>}
        </div>
        <Rows label="Rename" onAdd={() => onChange({ ...block, renames: [...block.renames, { from: firstInputColumns[0]?.name ?? '', to: '' }] })} addLabel="Add a rename">
          {block.renames.map((rename, index) => (
            <div key={renamesKeys.at(index)} className="grid grid-cols-[1fr_1fr_auto] gap-1.5">
              <ColumnPick label={`Rename ${index + 1} from`} columns={firstInputColumns} value={rename.from} onChange={(from) => onChange({ ...block, renames: block.renames.map((r, i) => i === index ? { ...r, from } : r) })} />
              <input aria-label={`Rename ${index + 1} to`} className={field('mono')} value={rename.to} placeholder="new name" onChange={(event) => onChange({ ...block, renames: block.renames.map((r, i) => i === index ? { ...r, to: event.target.value } : r) })} />
              <RemoveRow label={`Remove rename ${index + 1}`} index={index} onRemove={() => { renamesKeys.removed(index); onChange({ ...block, renames: block.renames.filter((_, i) => i !== index) }) }} />
            </div>
          ))}
        </Rows>
      </div>
    )
    case 'derive-columns': return (
      <Rows label="New columns" onAdd={() => onChange({ ...block, columns: [...block.columns, { name: '', expression: '' }] })} addLabel="Add a column">
        {block.columns.map((column, index) => (
          <div key={columnsKeys.at(index)} className="grid grid-cols-[1fr_auto] gap-1.5">
            <div className="grid gap-1.5">
              <input aria-label={`Derived column ${index + 1} name`} className={field('mono')} value={column.name} placeholder="name" onChange={(event) => onChange({ ...block, columns: block.columns.map((c, i) => i === index ? { ...c, name: event.target.value } : c) })} />
              <input aria-label={`Derived column ${index + 1} expression`} className={field('mono')} value={column.expression} placeholder="expression, for example population / 1000" onChange={(event) => onChange({ ...block, columns: block.columns.map((c, i) => i === index ? { ...c, expression: event.target.value } : c) })} />
            </div>
            <RemoveRow label={`Remove derived column ${index + 1}`} index={index} onRemove={() => { columnsKeys.removed(index); onChange({ ...block, columns: block.columns.filter((_, i) => i !== index) }) }} />
          </div>
        ))}
        <p className={caption('m-0')}>An expression in DuckDB's SQL over the input's columns: {firstInputColumns.length === 0 ? 'wire an input in to see them' : firstInputColumns.map((column) => column.name).join(', ')}.</p>
      </Rows>
    )
    case 'calendar-events': {
      const days = calendarWindowDays(block.window)
      const setWindow = (window: CalendarWindow) => onChange({ ...block, window, name: block.name === calendarShareName(block.window) ? calendarShareName(window) : block.name })
      return (
        <div className="space-y-3">
          <div className="grid gap-1.5 @md/card:grid-cols-2">
            <label className="block"><span className={fieldLabel}>Date column</span>
              <ColumnPick label="Date column" columns={firstInputColumns} value={block.column} onChange={(column) => onChange({ ...block, column })} />
            </label>
            <label className="block"><span className={fieldLabel}>Time interpretation</span>
              <Select aria-label="Time interpretation" className={field('text')} value={block.interpretation.kind === 'date-format' ? block.interpretation.format : block.interpretation.kind} onChange={(event) => { const choice = CALENDAR_TIME_INTERPRETATIONS.find((entry) => entry.value === event.target.value); if (choice) onChange({ ...block, interpretation: choice.interpretation }) }}>
                {CALENDAR_TIME_INTERPRETATIONS.map((entry) => <option key={entry.value} value={entry.value}>{entry.label}</option>)}
              </Select>
            </label>
          </div>
          <div className="space-y-1">
            <span className={fieldLabel}>Each row covers</span>
            <SegmentedControl size="sm" ariaLabel="Days each row covers" value={block.span} onChange={(span) => onChange({ ...block, span })} options={[{ value: 'day', label: 'One day' }, { value: 'week', label: 'One week' }, { value: 'month', label: 'One month' }]} />
            <p className={caption('m-0')}>Days are counted from the row's date.</p>
          </div>
          <div className="space-y-1">
            <span className={fieldLabel}>Window</span>
            <SegmentedControl size="sm" ariaLabel="Calendar window" value={block.window.kind} onChange={(kind) => setWindow(kind === 'year-end' ? { kind } : { kind, from: days.from, to: days.to })} options={[{ value: 'year-end', label: 'Year-end shutdown' }, { value: 'custom', label: 'Custom window' }]} />
            {block.window.kind === 'year-end'
              ? <p className={caption('m-0')}>24 December to 2 January, every year in the series.</p>
              : (
                <div className="space-y-1">
                  <Suspense fallback={<div className={field('text', 'h-[30px]')} aria-busy />}>
                    <CalendarWindowPicker from={days.from} to={days.to} onChange={(window) => setWindow({ kind: 'custom', ...window })} />
                  </Suspense>
                  <p className={caption('m-0')}>Repeats every year.</p>
                </div>
              )}
          </div>
          <label className="block"><span className={fieldLabel}>Column name</span>
            <input aria-label="Calendar column name" className={field('mono', 'mt-1')} value={block.name} placeholder={calendarShareName(block.window)} onChange={(event) => onChange({ ...block, name: event.target.value })} />
          </label>
          <p className={caption('m-0')}>Adds a column to mark rows that fall within a specified calendar window, such as a year-end shutdown. The value is the proportion of days covered by the row that fall within the window, so a week partly overlapping the window is assigned a fraction. Declare the window before fitting, as calendar events can influence activity and outcomes in ways a trend term may not capture.</p>
        </div>
      )
    }
    case 'join': {
      const left = inputColumns[0] ?? []
      const right = inputColumns[1] ?? []
      return (
        <div className="space-y-3">
          <label className="block"><span className={fieldLabel}>Kind</span>
            <Select aria-label="Join kind" className={field('text', 'mt-1')} value={block.how} onChange={(event) => onChange({ ...block, how: event.target.value as JoinKind })}>
              {JOIN_OPTIONS}
            </Select>
          </label>
          {block.how !== 'cross' && (
            <Rows label="Match on" onAdd={() => onChange({ ...block, keys: [...block.keys, { left: left[0]?.name ?? '', right: right[0]?.name ?? '' }] })} addLabel="Add a key">
              {block.keys.map((key, index) => (
                <div key={keysKeys.at(index)} className="grid grid-cols-[1fr_1fr_auto] gap-1.5">
                  <ColumnPick label={`Key ${index + 1} in ${inputNames[0] ?? 'input 1'}`} columns={left} value={key.left} onChange={(value) => onChange({ ...block, keys: block.keys.map((k, i) => i === index ? { ...k, left: value } : k) })} />
                  <ColumnPick label={`Key ${index + 1} in ${inputNames[1] ?? 'input 2'}`} columns={right} value={key.right} onChange={(value) => onChange({ ...block, keys: block.keys.map((k, i) => i === index ? { ...k, right: value } : k) })} />
                  <RemoveRow label={`Remove key ${index + 1}`} index={index} onRemove={() => { keysKeys.removed(index); onChange({ ...block, keys: block.keys.filter((_, i) => i !== index) }) }} />
                </div>
              ))}
            </Rows>
          )}
          <p className={caption('m-0')}>Input 1 corresponds to the top-left port; input 2 to the top-right.</p>
        </div>
      )
    }
    case 'union': return (
      <div className="space-y-3">
        <SegmentedControl size="sm" ariaLabel="Union matches columns" value={block.by} onChange={(by) => onChange({ ...block, by })} options={[{ value: 'name', label: 'By column name' }, { value: 'position', label: 'By position' }]} />
        <label className="flex items-center gap-2 text-body text-ink"><input type="checkbox" checked={block.distinct} onChange={(event) => onChange({ ...block, distinct: event.target.checked })} />Drop duplicate rows</label>
        <p className={caption('m-0')}>Connect as many inputs as needed. A new port appears below each arrow.</p>
      </div>
    )
    case 'aggregate': return (
      <div className="space-y-3">
        <div className="grid gap-1" role="group" aria-label="Group by">
          <span className={fieldLabel}>Group by</span>
          {firstInputColumns.map((column) => {
            const checked = block.groupBy.includes(column.name)
            return (
              <label key={column.name} className="flex items-center gap-2 text-body text-ink">
                <input type="checkbox" checked={checked} onChange={() => onChange({ ...block, groupBy: checked ? block.groupBy.filter((c) => c !== column.name) : [...block.groupBy, column.name] })} />
                <span className={literal('truncate')}>{column.name}</span>
              </label>
            )
          })}
        </div>
        <Rows label="Measures" onAdd={() => onChange({ ...block, measures: [...block.measures, { function: 'count', as: 'n' }] })} addLabel="Add a measure">
          {block.measures.map((measure, index) => {
            const replace = (next: AggregateMeasure) => onChange({ ...block, measures: block.measures.map((m, i) => i === index ? next : m) })
            return (
              <div key={measuresKeys.at(index)} className="grid grid-cols-[1fr_auto] gap-1.5">
                <div className="grid grid-cols-2 gap-1.5">
                  <Select aria-label={`Measure ${index + 1} function`} className={field('text')} value={measure.function} onChange={(event) => replace(measureWithFunction(measure, event.target.value as AggregateFunction, firstInputColumns[0]?.name ?? ''))}>
                    {AGGREGATE_OPTIONS}
                  </Select>
                  {measure.function === 'count'
                    ? <span className="self-center text-body text-faint">of rows</span>
                    : <ColumnPick label={`Measure ${index + 1} column`} columns={firstInputColumns} value={measure.column} onChange={(column) => replace({ ...measure, column })} />}
                  <input aria-label={`Measure ${index + 1} name`} className={field('mono', 'col-span-2')} value={measure.as} placeholder="result column" onChange={(event) => replace({ ...measure, as: event.target.value })} />
                </div>
                <RemoveRow label={`Remove measure ${index + 1}`} index={index} onRemove={() => { measuresKeys.removed(index); onChange({ ...block, measures: block.measures.filter((_, i) => i !== index) }) }} />
              </div>
            )
          })}
        </Rows>
      </div>
    )
    case 'sort-limit': return (
      <div className="space-y-3">
        <Rows label="Sort by" onAdd={() => onChange({ ...block, sort: [...block.sort, { column: firstInputColumns[0]?.name ?? '', direction: 'ascending' }] })} addLabel="Add a sort column">
          {block.sort.map((entry, index) => (
            <div key={sortKeys.at(index)} className="grid grid-cols-[1fr_1fr_auto] gap-1.5">
              <ColumnPick label={`Sort ${index + 1} column`} columns={firstInputColumns} value={entry.column} onChange={(column) => onChange({ ...block, sort: block.sort.map((s, i) => i === index ? { ...s, column } : s) })} />
              <Select aria-label={`Sort ${index + 1} direction`} className={field('text')} value={entry.direction} onChange={(event) => onChange({ ...block, sort: block.sort.map((s, i) => i === index ? { ...s, direction: event.target.value as 'ascending' | 'descending' } : s) })}>
                {SORT_OPTIONS}
              </Select>
              <RemoveRow label={`Remove sort ${index + 1}`} index={index} onRemove={() => { sortKeys.removed(index); onChange({ ...block, sort: block.sort.filter((_, i) => i !== index) }) }} />
            </div>
          ))}
        </Rows>
        <label className="block"><span className={fieldLabel}>Keep the first</span>
          <input type="number" min={0} className={field('text', 'mt-1 w-32')} value={block.limit ?? ''} placeholder="all rows" onChange={(event) => onChange({ ...block, limit: event.target.value === '' ? null : Math.max(0, Math.floor(Number(event.target.value) || 0)) })} />
        </label>
      </div>
    )
    case 'script': return (
      <div className="space-y-2">
        <Suspense fallback={<div className="min-h-[14rem] rounded-md border border-control bg-well" aria-busy />}>
          <PythonEditor label="Python script" value={block.code} onChange={(code) => onChange({ ...block, code })} />
        </Suspense>
        <p className={caption('m-0')}>Create a DataFrame without an input, or read connected DataFrames from <code>inputs</code> in port order. Assign the result to <code>prepared</code>.</p>
        <PythonRuntimeLine step={node.id} />
      </div>
    )
    default: return assertNever(block)
  }
}

/** The runtime's state under the editor, and the run in progress with a way to stop it; opening a script block starts the download, so the first run does not wait for it. */
function PythonRuntimeLine({ step }: { readonly step: PipelineBlockId }) {
  const runtime = usePythonSession()
  const state = usePythonRuntime()
  const running = usePythonRun(step)
  useEffect(() => { runtime.warm() }, [runtime])
  if (running !== null) {
    return (
      <div className="flex items-center gap-3" data-testid="python-runtime" aria-busy>
        <Orb state="working" aria-label="Script running" />
        <span className={num('text-label text-muted')}><Metadata><span>Running</span><span>{formatDuration(running.elapsedMs).text}</span></Metadata></span>
        <button type="button" className={button('quiet', undefined, 'sm')} onClick={runtime.cancel}>Cancel run</button>
      </div>
    )
  }
  switch (state.kind) {
    case 'idle': return null
    case 'loading': return <p className="m-0 flex items-center gap-2 text-label text-muted" data-testid="python-runtime" aria-busy><Orb state="connecting" aria-label="Python runtime loading" />{state.detail}</p>
    case 'ready': return <p className="m-0 text-micro text-faint" data-testid="python-runtime"><Metadata><span>Python {state.python}</span><span>pandas</span><span>numpy</span></Metadata></p>
    case 'failed': return <Alert tone="danger" testId="python-runtime"><p className="m-0">Python could not load: {state.detail}</p></Alert>
    default: return assertNever(state)
  }
}

/** Null for a block that does not compile to SQL. */
export function blockSqlText(node: PipelineNode, inputViews: readonly string[]): string | null {
  if (node.block.kind === 'input' || node.block.kind === 'script') return null
  const sql = blockSql(node, inputViews)
  return sql.ok ? sql.value : null
}

const numberOrText = (value: string): string | number => {
  const trimmed = value.trim()
  return trimmed !== '' && Number.isFinite(Number(trimmed)) ? Number(trimmed) : value
}

function ColumnPick({ label, columns, value, onChange }: { readonly label: string; readonly columns: readonly PreviewColumn[]; readonly value: string; readonly onChange: (value: string) => void }) {
  const known = columns.some((column) => column.name === value)
  return (
    <Select aria-label={label} className={field('text')} value={value} onChange={(event) => onChange(event.target.value)}>
      {!known && <option value={value}>{value === '' ? 'Choose a column' : `${value} (missing)`}</option>}
      {columns.map((column) => <option key={column.name} value={column.name}>{column.name}</option>)}
    </Select>
  )
}

/**
 * A stable key per row of a list that is stored without ids. The ids live here, never in the recipe:
 * one is made when a row appears and dropped when its row is removed, so React keeps each row's
 * element with its row across insertions and removals instead of reusing them by position.
 */
function useRowKeys(list: string, count: number): { readonly at: (index: number) => string; readonly removed: (index: number) => void } {
  const lists = useRef(new Map<string, string[]>())
  const keys = lists.current.get(list) ?? []
  while (keys.length < count) keys.push(crypto.randomUUID())
  if (keys.length > count) keys.length = count
  lists.current.set(list, keys)
  return {
    at: (index) => keys[index] ?? String(index),
    removed: (index) => { keys.splice(index, 1) },
  }
}

/** After a row goes, focus lands on the row that took its place, or on the add button when it was the last. */
const focusAfterRemoval = (from: HTMLElement, index: number): void => {
  const rows = from.closest<HTMLElement>('[data-rows]')
  if (rows === null) return
  requestAnimationFrame(() => {
    const removers = rows.querySelectorAll<HTMLElement>('[data-remove-row]')
    const next = removers[index] ?? removers[removers.length - 1] ?? rows.querySelector<HTMLElement>('[data-add-row]')
    next?.focus()
  })
}

function Rows({ label, onAdd, addLabel, children }: { readonly label: string; readonly onAdd: () => void; readonly addLabel: string; readonly children: React.ReactNode }) {
  return (
    <div className="space-y-2" data-rows>
      <span className={fieldLabel}>{label}</span>
      {children}
      <button type="button" className={button('quiet', undefined, 'sm')} data-add-row onClick={onAdd}><Icon name="add" size={14} /> {addLabel}</button>
    </div>
  )
}

function RemoveRow({ label, index, onRemove }: { readonly label: string; readonly index: number; readonly onRemove: () => void }) {
  return (
    <button
      type="button"
      className={iconControl('quiet', 'h-7 w-7 self-start')}
      aria-label={label}
      title={label}
      data-remove-row
      onClick={(event) => { onRemove(); focusAfterRemoval(event.currentTarget, index) }}
    >
      <Icon name="delete" size={14} />
    </button>
  )
}
