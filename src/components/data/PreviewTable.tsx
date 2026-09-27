import { Select } from '@/components/ui/Select'
import { useEffect, useId, useMemo, useRef, useState } from 'react'
import { useVirtualizer } from '@tanstack/react-virtual'
import { Icon } from '@/components/Icon'
import { cellPadding, Chip, FilterField, HeaderMenu, ROW_HEIGHT, TableShell, useTableDensity, type MenuItem } from '@/components/table/primitives'
import { Alert } from '@/components/ui/Alert'
import { facet, button, field, label, num, table as tableCn, th } from '@/components/ui/recipes'
import { isNumericDuckDbType, type ColumnId, type ColumnSummary, type DatasetProfile, type PhysicalColumnProfile, type PreviewCell, type PreviewFilter, type PreviewSort } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import type { SelectedSource } from '@/domain/workflow'
import { formatCount } from '@/lib/format/number'
import { cn } from '@/lib/utils'
import { columnDecimals, isNumericCell, PreviewCellText } from './previewCell'
import type { DatasetSummaryState } from './useDatasetSummary'
import { usePreviewWindows } from './usePreviewWindows'
import { fontFor, textWidth } from '@/lib/textMetrics'

const isRightAligned = (cell: PreviewCell | undefined, column: PhysicalColumnProfile): boolean =>
  cell === undefined || cell.kind === 'null' ? isNumericDuckDbType(column.duckdbType) : isNumericCell(cell)

const describeFilter = (filter: PreviewFilter, name: string): string => {
  switch (filter.kind) {
    case 'range':
      if (filter.min !== null && filter.max !== null) return `${name} ${formatCount(filter.min).text} … ${formatCount(filter.max).text}`
      if (filter.min !== null) return `${name} ≥ ${formatCount(filter.min).text}`
      if (filter.max !== null) return `${name} ≤ ${formatCount(filter.max).text}`
      return `${name} any`
    case 'contains': return `${name} contains “${filter.text}”`
    case 'one-of': return `${name} in ${filter.values.length} value${filter.values.length === 1 ? '' : 's'}`
    case 'missing': return filter.missing ? `${name} is missing` : `${name} is present`
    default: return assertNever(filter)
  }
}

const filterKey = (filter: PreviewFilter): string => `${filter.column}:${filter.kind}`

interface EditorDraft {
  readonly column: PhysicalColumnProfile
  readonly summary: ColumnSummary | null
  readonly min: string
  readonly max: string
  readonly text: string
  readonly values: ReadonlySet<string>
  readonly missing: 'any' | 'only' | 'exclude'
}

const draftFor = (column: PhysicalColumnProfile, summary: ColumnSummary | null, existing: readonly PreviewFilter[]): EditorDraft => {
  const own = existing.filter((filter) => filter.column === column.id)
  const range = own.find((filter): filter is Extract<PreviewFilter, { kind: 'range' }> => filter.kind === 'range')
  const contains = own.find((filter): filter is Extract<PreviewFilter, { kind: 'contains' }> => filter.kind === 'contains')
  const oneOf = own.find((filter): filter is Extract<PreviewFilter, { kind: 'one-of' }> => filter.kind === 'one-of')
  const missing = own.find((filter): filter is Extract<PreviewFilter, { kind: 'missing' }> => filter.kind === 'missing')
  return {
    column,
    summary,
    min: range?.min === null || range?.min === undefined ? '' : String(range.min),
    max: range?.max === null || range?.max === undefined ? '' : String(range.max),
    text: contains?.text ?? '',
    values: new Set(oneOf?.values ?? []),
    missing: missing === undefined ? 'any' : missing.missing ? 'only' : 'exclude',
  }
}

const filtersFromDraft = (draft: EditorDraft): readonly PreviewFilter[] => {
  const filters: PreviewFilter[] = []
  const numeric = isNumericDuckDbType(draft.column.duckdbType)
  if (numeric) {
    const min = draft.min.trim() === '' ? null : Number(draft.min)
    const max = draft.max.trim() === '' ? null : Number(draft.max)
    if ((min !== null && Number.isFinite(min)) || (max !== null && Number.isFinite(max))) {
      filters.push({ column: draft.column.id, kind: 'range', min: min !== null && Number.isFinite(min) ? min : null, max: max !== null && Number.isFinite(max) ? max : null })
    }
  } else if (draft.summary?.categories !== null && draft.summary?.categories !== undefined) {
    if (draft.values.size > 0) filters.push({ column: draft.column.id, kind: 'one-of', values: [...draft.values] })
  } else if (draft.text.trim().length > 0) {
    filters.push({ column: draft.column.id, kind: 'contains', text: draft.text.trim() })
  }
  if (draft.missing !== 'any') filters.push({ column: draft.column.id, kind: 'missing', missing: draft.missing === 'only' })
  return filters
}

function FilterEditor({ draft, onChange, onApply, onCancel }: {
  readonly draft: EditorDraft
  readonly onChange: (draft: EditorDraft) => void
  readonly onApply: () => void
  readonly onCancel: () => void
}) {
  const numeric = isNumericDuckDbType(draft.column.duckdbType)
  const categories = draft.summary?.categories ?? null
  return (
    <form
      className="border-b border-hair bg-well px-3.5 py-2.5"
      aria-label={`Filter ${draft.column.name}`}
      onSubmit={(event) => { event.preventDefault(); onApply() }}
      onKeyDown={(event) => { if (event.key === 'Escape') { event.preventDefault(); onCancel() } }}
    >
      <div className="flex flex-wrap items-end gap-3">
        <span className={label('text-muted')}>Filter <span>{draft.column.name}</span></span>
        {numeric && (
          <>
            <label className="text-body text-ink">Min<input type="number" step="any" className={field('text', 'mt-1 w-32')} value={draft.min} placeholder={draft.summary?.min ?? ''} onChange={(event) => onChange({ ...draft, min: event.target.value })} /></label>
            <label className="text-body text-ink">Max<input type="number" step="any" className={field('text', 'mt-1 w-32')} value={draft.max} placeholder={draft.summary?.max ?? ''} onChange={(event) => onChange({ ...draft, max: event.target.value })} /></label>
          </>
        )}
        {!numeric && categories === null && (
          <label className="text-body text-ink">Contains<input type="text" className={field('text', 'mt-1 w-48')} value={draft.text} onChange={(event) => onChange({ ...draft, text: event.target.value })} /></label>
        )}
        <label className="text-body text-ink">Missing values<Select className={field('text', 'mt-1')} value={draft.missing} onChange={(event) => onChange({ ...draft, missing: event.target.value as EditorDraft['missing'] })}><option value="any">Keep</option><option value="exclude">Exclude</option><option value="only">Only missing</option></Select></label>
        <div className="flex gap-2">
          <button type="submit" className={button('signal')}>Apply filter</button>
          <button type="button" className={button('quiet')} onClick={onCancel}>Cancel</button>
        </div>
      </div>
      {!numeric && categories !== null && (
        <div className="mt-2 flex flex-wrap gap-1" role="group" aria-label={`Values of ${draft.column.name}`}>
          {categories.map((category) => {
            const active = draft.values.has(category.value)
            return (
              <button
                key={category.value}
                type="button"
                aria-pressed={active}
                className={facet(active, 'min-h-6 px-2 py-0')}
                onClick={() => onChange({ ...draft, values: active ? new Set([...draft.values].filter((value) => value !== category.value)) : new Set([...draft.values, category.value]) })}
              >
                {category.value} <span className={num('text-faint')}>{formatCount(category.count, { compact: true }).text}</span>
              </button>
            )
          })}
        </div>
      )}
    </form>
  )
}

export function PreviewTable({ source, profile, summary, selectedColumn, onSelectColumn }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly summary: DatasetSummaryState
  readonly selectedColumn: ColumnId | null
  readonly onSelectColumn: (column: ColumnId) => void
}) {
  const titleId = useId()
  const [density] = useTableDensity()
  const [sort, setSort] = useState<PreviewSort | null>(null)
  const [filters, setFilters] = useState<readonly PreviewFilter[]>([])
  const [search, setSearch] = useState('')
  const [hidden, setHidden] = useState<ReadonlySet<ColumnId>>(new Set())
  const [editing, setEditing] = useState<EditorDraft | null>(null)
  const [searchDraft, setSearchDraft] = useState('')
  const scrollRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const handle = window.setTimeout(() => setSearch(searchDraft.trim()), 250)
    return () => window.clearTimeout(handle)
  }, [searchDraft])

  const shape = useMemo(() => ({ sort, filters, search }), [filters, search, sort])
  const windows = usePreviewWindows(source, profile, shape)
  const total = windows.total ?? 0
  const rowHeight = ROW_HEIGHT[density]
  const virtualizer = useVirtualizer({ count: total, getScrollElement: () => scrollRef.current, estimateSize: () => rowHeight, overscan: 12 })
  const items = virtualizer.getVirtualItems()
  const first = items[0]?.index ?? 0
  const last = items.at(-1)?.index ?? 0
  useEffect(() => { if (total > 0) windows.ensure(first, last) }, [first, last, total, windows])
  useEffect(() => { virtualizer.measure() }, [rowHeight, virtualizer])

  const visibleColumns = profile.columns.filter((column) => !hidden.has(column.id))
  const summaryOf = (column: ColumnId): ColumnSummary | null => (summary.kind === 'ready' ? summary.byColumn.get(column) ?? null : null)
  const onScreen = items.map((item) => windows.rowAt(item.index)?.cells ?? null)
  const decimals = new Map(visibleColumns.map((column) => [column.id, columnDecimals(onScreen, profile.columns.indexOf(column))]))
  const padding = cellPadding(density)

  const replaceFilters = (column: ColumnId, next: readonly PreviewFilter[]) =>
    setFilters((current) => [...current.filter((filter) => filter.column !== column), ...next])

  const menuFor = (column: PhysicalColumnProfile): readonly MenuItem[] => [
    { id: 'asc', text: 'Sort ascending', icon: 'arrow_upward', onSelect: () => setSort({ column: column.id, direction: 'asc' }) },
    { id: 'desc', text: 'Sort descending', icon: 'arrow_downward', onSelect: () => setSort({ column: column.id, direction: 'desc' }) },
    { id: 'clear-sort', text: 'Clear sort', icon: 'sort', disabled: sort?.column !== column.id, onSelect: () => setSort(null) },
    { id: 'filter', text: 'Filter…', icon: 'filter_list', onSelect: () => setEditing(draftFor(column, summaryOf(column.id), filters)) },
    { id: 'profile', text: 'Profile column', icon: 'query_stats', onSelect: () => onSelectColumn(column.id) },
    { id: 'hide', text: 'Hide column', icon: 'visibility_off', disabled: visibleColumns.length <= 1, onSelect: () => setHidden((current) => new Set([...current, column.id])) },
  ]

  const nameOf = (id: ColumnId): string => profile.columns.find((column) => column.id === id)?.name ?? String(id)
  const shownFrom = total === 0 ? 0 : first + 1
  const shownTo = total === 0 ? 0 : Math.min(total, last + 1)
  const clauses = [
    sort === null ? null : `sorted by ${nameOf(sort.column)} ${sort.direction === 'asc' ? '↑' : '↓'}`,
    filters.length === 0 ? null : `${filters.length} filter${filters.length === 1 ? '' : 's'}`,
    search.length === 0 ? null : `search “${search}”`,
  ].filter((clause): clause is string => clause !== null)
  const count = windows.total === null
    ? 'Loading rows…'
    : `rows ${formatCount(shownFrom).text} to ${formatCount(shownTo).text} of ${formatCount(total).text}${total !== profile.rowCount ? ` (${formatCount(profile.rowCount).text} in the file)` : ''}${clauses.length > 0 ? `, ${clauses.join('; ')}` : ''}`

  // The column holds the grouped count in tabular figures, measured in the body face, inside 28px of padding.
  const rowNumberWidth = Math.max(48, Math.ceil(textWidth(formatCount(profile.rowCount).text, fontFor('body'))) + 28)

  return (
    <TableShell
      collapsible
      title="Preview"
      titleId={titleId}
      scrollRef={scrollRef}
      toolbar={(
        <>
          <div className="flex w-full min-w-0 items-center gap-2 sm:w-auto sm:flex-1">
          <FilterField value={searchDraft} onChange={setSearchDraft} placeholder="Search all columns" label="Search rows" className="min-w-0 flex-1 sm:w-44" />
          <details className="relative shrink-0">
            <summary aria-label={`Choose visible columns, ${visibleColumns.length} of ${profile.columns.length} shown`} title={`Choose visible columns, ${visibleColumns.length} of ${profile.columns.length} shown`} className={cn(facet(hidden.size > 0, 'flex min-h-8 min-w-8 cursor-pointer list-none items-center justify-center px-2 py-0 pointer-coarse:min-h-[3.125rem] pointer-coarse:min-w-[3.125rem]'))}>
              <Icon name="view_column" size={18} />
            </summary>
            <div className="float absolute right-0 top-full z-20 mt-1 max-h-72 w-56 overflow-y-auto rounded-lg border border-edge bg-panel p-2">
              <ul className="m-0 list-none space-y-1 p-0" aria-label="Shown columns">
                {profile.columns.map((column) => (
                  <li key={column.id}>
                    <label className="flex items-center gap-2 text-body text-ink">
                      <input
                        type="checkbox"
                        checked={!hidden.has(column.id)}
                        disabled={!hidden.has(column.id) && visibleColumns.length <= 1}
                        onChange={(event) => setHidden((current) => { const next = new Set(current); if (event.target.checked) next.delete(column.id); else next.add(column.id); return next })}
                      />
                      <span className="truncate">{column.name}</span>
                    </label>
                  </li>
                ))}
              </ul>
              {hidden.size > 0 && <button type="button" className={button('quiet', 'mt-2 w-full')} onClick={() => setHidden(new Set())}>Show all</button>}
            </div>
          </details>
          </div>
          {filters.map((filter) => (
            <Chip key={filterKey(filter)} removeLabel={`Remove filter ${describeFilter(filter, nameOf(filter.column))}`} onRemove={() => setFilters((current) => current.filter((candidate) => filterKey(candidate) !== filterKey(filter)))}>
              {describeFilter(filter, nameOf(filter.column))}
            </Chip>
          ))}
        </>
      )}
      count={count}
      lead={(
        <>
          {editing !== null && (
            <FilterEditor
              draft={editing}
              onChange={setEditing}
              onApply={() => { replaceFilters(editing.column.id, filtersFromDraft(editing)); setEditing(null) }}
              onCancel={() => setEditing(null)}
            />
          )}
          {windows.problem !== null && (
            <Alert tone="danger" className="m-3">
              <p className="m-0">The preview could not be read: {windows.problem.kind === 'source-changed' ? 'the file changed on disk.' : windows.problem.detail}</p>
            </Alert>
          )}
        </>
      )}
    >
      <table className={cn(tableCn, 'min-w-full table-fixed')} aria-rowcount={total + 1} aria-busy={windows.loading || undefined}>
        <colgroup>
          <col style={{ width: rowNumberWidth }} />
          {visibleColumns.map((column) => <col key={column.id} style={{ width: Math.max(104, Math.min(240, column.name.length * 8 + 56)) }} />)}
        </colgroup>
        <thead>
          <tr aria-rowindex={1}>
            <th scope="col" className={th('p-0 sticky left-0 z-20 border-r border-hair text-right')} aria-label="Row">
              <span className="block px-3.5 py-[7px]" aria-hidden>#</span>
            </th>
            {visibleColumns.map((column) => {
              const selected = column.id === selectedColumn
              const sorted = sort?.column === column.id ? sort.direction : null
              const numeric = isNumericDuckDbType(column.duckdbType)
              return (
                <th
                  key={column.id}
                  scope="col"
                  aria-label={column.name}
                  aria-sort={sorted === 'asc' ? 'ascending' : sorted === 'desc' ? 'descending' : undefined}
                  className={th(cn('group/th p-0 align-top', selected && 'text-ink'))}
                >
                  <div className={cn('flex items-center gap-1 px-2 py-[5px]', numeric && 'flex-row-reverse')}>
                    <button
                      type="button"
                      aria-pressed={selected}
                      onClick={() => onSelectColumn(column.id)}
                      title={`Profile ${column.name}`}
                      className={cn('min-w-0 flex-1 truncate text-label font-medium', numeric ? 'text-right' : 'text-left', selected ? 'text-ink' : 'hover:text-ink')}
                    >
                      {column.name}
                    </button>
                    {sorted !== null && <Icon name={sorted === 'asc' ? 'arrow_upward' : 'arrow_downward'} size={11} className="shrink-0 text-ink" />}
                    <HeaderMenu label={`Actions for ${column.name}`} items={menuFor(column)} />
                  </div>
                </th>
              )
            })}
          </tr>
        </thead>
        <tbody>
          {items.length > 0 && items[0].start > 0 && (
            <tr role="presentation" aria-hidden><td role="presentation" colSpan={visibleColumns.length + 1} style={{ height: items[0].start, padding: 0, border: 0 }} /></tr>
          )}
          {items.map((item) => {
            const row = windows.rowAt(item.index)
            return (
              <tr key={item.key} aria-rowindex={item.index + 2} className="border-b border-line" style={{ height: rowHeight }}>
                <th scope="row" className={num(cn('sticky left-0 z-10 border-r border-hair bg-panel px-3.5 text-right font-normal text-faint', padding))}>{row === null ? '' : formatCount(row.index).text}</th>
                {visibleColumns.map((column) => {
                  const columnIndex = profile.columns.indexOf(column)
                  const cell = row?.cells[columnIndex]
                  const right = isRightAligned(cell, column)
                  return (
                    <td
                      key={column.id}
                      className={cn('truncate whitespace-nowrap px-3.5', padding, right ? num('text-right') : 'text-left', cell?.kind === 'null' ? 'text-muted' : 'text-ink', column.id === selectedColumn && 'bg-raised')}
                      title={cell !== undefined && cell.kind !== 'null' ? String(cell.kind === 'boolean' ? cell.value : cell.value) : undefined}
                    >
                      {cell === undefined ? <span className="text-faint">…</span> : <PreviewCellText cell={cell} decimals={decimals.get(column.id) ?? 0} />}
                    </td>
                  )
                })}
              </tr>
            )
          })}
          {items.length > 0 && virtualizer.getTotalSize() - (items.at(-1)?.end ?? 0) > 0 && (
            <tr role="presentation" aria-hidden><td role="presentation" colSpan={visibleColumns.length + 1} style={{ height: virtualizer.getTotalSize() - (items.at(-1)?.end ?? 0), padding: 0, border: 0 }} /></tr>
          )}
          {windows.total === 0 && (
            <tr><td colSpan={visibleColumns.length + 1} className="px-3.5 py-6 text-center text-body text-faint">No rows match the criteria. Loosen a filter or broaden the search.</td></tr>
          )}
        </tbody>
      </table>
    </TableShell>
  )
}
