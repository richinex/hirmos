import { useId, useMemo, useState, type ReactNode } from 'react'
import { createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type SortingState } from '@tanstack/react-table'
import { Icon } from '@/components/Icon'
import { CategoryBar, cellPadding, countLine, DensityToggle, FacetPills, FilterField, MiniHistogram, SortHeader, TableShell, useTableDensity, type Facet } from '@/components/table/primitives'
import { literal, num, table as tableCn, td, tr } from '@/components/ui/recipes'
import { isNumericDuckDbType, type ColumnId, type ColumnSummary, type DatasetProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { formatCount, formatPercent, formatStatistic } from '@/lib/format/number'
import { cn } from '@/lib/utils'
import type { DatasetSummaryState } from './useDatasetSummary'

type ColumnKind = 'numeric' | 'text' | 'temporal' | 'boolean'

interface SchemaRow {
  readonly id: ColumnId
  readonly name: string
  readonly duckdbType: string
  readonly kind: ColumnKind
  readonly nullCount: number
  readonly nullShare: number
  readonly summary: ColumnSummary | null
}

const kindOf = (duckdbType: string): ColumnKind => {
  if (isNumericDuckDbType(duckdbType)) return 'numeric'
  if (duckdbType === 'BOOLEAN') return 'boolean'
  if (/^(DATE|TIME|TIMESTAMP|INTERVAL)/.test(duckdbType)) return 'temporal'
  return 'text'
}

const kindGlyph = (kind: ColumnKind): string => {
  switch (kind) {
    case 'numeric': return '#'
    case 'text': return 'Aa'
    case 'temporal': return '⏱'
    case 'boolean': return '◐'
    default: return assertNever(kind)
  }
}

const kindText = (kind: ColumnKind): string => {
  switch (kind) {
    case 'numeric': return 'Numeric'
    case 'text': return 'Text'
    case 'temporal': return 'Temporal'
    case 'boolean': return 'Boolean'
    default: return assertNever(kind)
  }
}

const KINDS: readonly ColumnKind[] = ['numeric', 'text', 'temporal', 'boolean']

/** The name with the search match marked, so a reader sees why the row survived the filter. */
function Highlighted({ text, query }: { readonly text: string; readonly query: string }): ReactNode {
  if (query.length === 0) return text
  const at = text.toLowerCase().indexOf(query.toLowerCase())
  if (at < 0) return text
  return <>{text.slice(0, at)}<mark className="rounded-sm bg-well text-ink">{text.slice(at, at + query.length)}</mark>{text.slice(at + query.length)}</>
}

const trimNumber = (raw: string): string => {
  const value = Number(raw)
  if (!Number.isFinite(value)) return raw
  return Number.isInteger(value) ? formatCount(value).text : formatStatistic('raw', value).text
}

const helper = createColumnHelper<SchemaRow>()

export function SchemaTable({ profile, summary, selectedColumn, onSelectColumn }: {
  readonly profile: DatasetProfile
  readonly summary: DatasetSummaryState
  readonly selectedColumn: ColumnId | null
  readonly onSelectColumn: (column: ColumnId) => void
}) {
  const titleId = useId()
  const [density, setDensity] = useTableDensity()
  const [search, setSearch] = useState('')
  const [kinds, setKinds] = useState<ReadonlySet<ColumnKind>>(new Set())
  const [sorting, setSorting] = useState<SortingState>([])

  const rows = useMemo<readonly SchemaRow[]>(() => profile.columns.map((column) => ({
    id: column.id,
    name: column.name,
    duckdbType: column.duckdbType,
    kind: kindOf(column.duckdbType),
    nullCount: column.nullCount,
    nullShare: profile.rowCount === 0 ? 0 : column.nullCount / profile.rowCount,
    summary: summary.kind === 'ready' ? summary.byColumn.get(column.id) ?? null : null,
  })), [profile, summary])

  const query = search.trim()
  const hasTypeChoice = new Set(rows.map((row) => row.kind)).size > 1
  const visible = useMemo(() => rows.filter((row) =>
    (!hasTypeChoice || kinds.size === 0 || kinds.has(row.kind)) && (query.length === 0 || row.name.toLowerCase().includes(query.toLowerCase()))), [hasTypeChoice, kinds, query, rows])

  const facets: readonly Facet[] = KINDS.map((kind) => ({
    id: kind,
    text: kindText(kind),
    count: rows.filter((row) => row.kind === kind && (query.length === 0 || row.name.toLowerCase().includes(query.toLowerCase()))).length,
    active: kinds.has(kind),
  })).filter((facet) => facet.count > 0 || facet.active)

  const columns = useMemo(() => [
    helper.accessor('name', {
      header: 'Column',
      cell: (context) => {
        const row = context.row.original
        const selected = row.id === selectedColumn
        return (
          <span className="flex items-center gap-2">
            <span aria-hidden className={literal('w-4 shrink-0 text-center text-micro text-faint')} title={kindText(row.kind)}>{kindGlyph(row.kind)}</span>
            <button
              type="button"
              aria-pressed={selected}
              onClick={(event) => { event.stopPropagation(); onSelectColumn(row.id) }}
              className="block min-w-0 flex-1 truncate text-left text-body text-ink"
              title={`Profile ${row.name}`}
            >
              <Highlighted text={row.name} query={query} />
            </button>
          </span>
        )
      },
    }),
    helper.accessor('duckdbType', {
      header: 'Type',
      cell: (context) => <span className={literal('text-faint')}>{context.getValue()}</span>,
    }),
    helper.accessor('nullCount', {
      header: 'Missing',
      meta: { align: 'right' },
      cell: (context) => <span className={context.getValue() > 0 ? 'text-ink' : 'text-faint'}>{formatCount(context.getValue()).text}</span>,
    }),
    helper.accessor('nullShare', {
      header: '%',
      meta: { align: 'right' },
      cell: (context) => {
        const row = context.row.original
        const percent = formatPercent(row.nullShare)
        return (
          <span className="relative block min-w-12" title={`${formatCount(row.nullCount).text} of ${formatCount(profile.rowCount).text} rows missing`}>
            {row.nullShare > 0 && <span aria-hidden className="absolute inset-y-1 right-0 rounded-sm bg-bone/40" style={{ width: `${Math.max(2, Math.round(row.nullShare * 100))}%` }} />}
            <span className={cn('relative', row.nullShare > 0 ? 'text-ink' : 'text-faint')}>{percent.text}</span>
          </span>
        )
      },
    }),
    helper.accessor((row) => row.summary?.distinctCount ?? -1, {
      id: 'distinct',
      header: 'Distinct',
      meta: { align: 'right' },
      cell: (context) => {
        const row = context.row.original
        if (row.summary === null) return <span className="text-faint">…</span>
        const identifier = row.summary.distinctCount >= profile.rowCount && profile.rowCount > 1
        return <span className={identifier ? 'text-muted' : 'text-ink'} title={identifier ? 'Every row is distinct: an identifier' : undefined}>{formatCount(row.summary.distinctCount, { compact: true }).text}{identifier && <span aria-hidden className="ml-1 text-faint">=</span>}</span>
      },
    }),
    helper.accessor((row) => row.summary?.min ?? '', {
      id: 'range',
      header: 'Range',
      enableSorting: false,
      cell: (context) => {
        const row = context.row.original
        if (row.summary === null) return <span className="text-faint">…</span>
        if (row.summary.min === null || row.summary.max === null) return <span className="text-faint">—</span>
        const numeric = row.kind === 'numeric'
        return <span className={cn('whitespace-nowrap', numeric ? 'text-ink' : 'text-muted')} title={`${row.summary.min} to ${row.summary.max}`}>{numeric ? trimNumber(row.summary.min) : row.summary.min.slice(0, 12)} <span className="text-faint">…</span> {numeric ? trimNumber(row.summary.max) : row.summary.max.slice(0, 12)}</span>
      },
    }),
    helper.display({
      id: 'distribution',
      header: 'Distribution',
      cell: (context) => {
        const row = context.row.original
        if (row.summary === null) return null
        if (row.summary.histogram !== null) return <MiniHistogram bins={row.summary.histogram} />
        if (row.summary.categories !== null) return <CategoryBar categories={row.summary.categories} total={profile.rowCount - row.nullCount} />
        return <span className="text-faint">—</span>
      },
    }),
  ], [onSelectColumn, profile.rowCount, query, selectedColumn])

  const table = useReactTable({
    data: visible,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })

  const sortText = sorting[0] === undefined ? undefined : `sorted by ${sorting[0].id === 'nullShare' ? 'missing share' : sorting[0].id === 'nullCount' ? 'missing' : sorting[0].id} ${sorting[0].desc ? 'descending' : 'ascending'}`
  const padding = cellPadding(density)

  return (
    <TableShell
      title="Physical schema"
      titleId={titleId}
      toolbar={(
        <>
          <div className="flex w-full min-w-0 items-center gap-2 sm:w-auto sm:flex-1">
            <FilterField value={search} onChange={setSearch} placeholder="Find a column" label="Search columns" className="min-w-0 flex-1 sm:w-40" />
            <DensityToggle density={density} onChange={setDensity} />
          </div>
          {hasTypeChoice && <FacetPills facets={facets} label="Column types" onToggle={(id) => setKinds((current) => { const next = new Set(current); if (next.has(id as ColumnKind)) next.delete(id as ColumnKind); else next.add(id as ColumnKind); return next })} />}
        </>
      )}
      count={countLine(visible.length, rows.length, 'column', sortText)}
      foot={summary.kind === 'failed'
        ? <p role="status" className="m-0 flex items-center gap-1.5 border-t border-hair px-3.5 py-1.5 text-label text-faint"><Icon name="info" size={12} /> Column summaries could not be computed for this file.</p>
        : undefined}
    >
      <table className={tableCn}>
        <thead>
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
                  canSort={header.column.getCanSort()}
                  onToggle={() => header.column.toggleSorting()}
                  align={(header.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined)?.align ?? 'left'}
                >
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </SortHeader>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.length === 0 && (
            <tr><td colSpan={columns.length} className="px-3.5 py-6 text-center text-body text-faint">No column matches. Loosen the search or the type filter.</td></tr>
          )}
          {table.getRowModel().rows.map((row) => {
            const selected = row.original.id === selectedColumn
            return (
              <tr key={row.id} className={tr(selected ? 'selected' : 'action')} onClick={() => onSelectColumn(row.original.id)}>
                {row.getVisibleCells().map((cell) => {
                  const align = (cell.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined)?.align
                  return (
                    <td key={cell.id} className={td(cn(padding, align === 'right' && num('whitespace-nowrap text-right'), cell.column.id === 'name' && 'max-w-56'))}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </td>
                  )
                })}
              </tr>
            )
          })}
        </tbody>
      </table>
    </TableShell>
  )
}
