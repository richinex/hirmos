import { useId, useMemo, useState, type ReactNode } from 'react'
import { flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type ColumnDef, type SortingState } from '@tanstack/react-table'
import { cellPadding, countLine, DensityToggle, FilterField, SortHeader, TableShell, useTableDensity } from '@/components/table/primitives'
import { num, table as tableCn, td, tr } from '@/components/ui/recipes'
import { cn } from '@/lib/utils'

/**
 * A sortable evidence table over plain rows: the same shell, header cells, density switch and count
 * line as the Data Studio tables, so every figure a run reports can be ordered and searched the same
 * way. Columns declare a raw value for sorting and searching and, separately, how it prints.
 */

export type EvidenceValue = string | number

export interface EvidenceColumn<Row> {
  readonly id: string
  readonly header: string
  readonly align?: 'left' | 'right'
  readonly value: (row: Row) => EvidenceValue
  readonly format?: (value: EvidenceValue, row: Row) => ReactNode
  /** Mono for marks and identifiers; figures already print in tabular numerals. */
  readonly mono?: boolean
}

export function EvidenceTable<Row>({ title, rows, columns, rowKey, noun, empty, maxHeight = 'max-h-96' }: {
  readonly title: string
  readonly rows: readonly Row[]
  readonly columns: readonly EvidenceColumn<Row>[]
  readonly rowKey: (row: Row, index: number) => string
  /** Singular noun for the count line: "cell", "weight", "edge". */
  readonly noun: string
  /** What to say when the run reported nothing. */
  readonly empty: string
  readonly maxHeight?: string
}) {
  const titleId = useId()
  const [sorting, setSorting] = useState<SortingState>([])
  const [query, setQuery] = useState('')
  const [density, setDensity] = useTableDensity()
  const textColumns = useMemo(() => columns.filter((column) => column.align !== 'right'), [columns])
  const visible = useMemo(() => {
    const needle = query.trim().toLowerCase()
    if (needle.length === 0) return rows
    return rows.filter((row) => textColumns.some((column) => String(column.value(row)).toLowerCase().includes(needle)))
  }, [query, rows, textColumns])
  const definitions = useMemo<ColumnDef<Row, EvidenceValue>[]>(
    () => columns.map((column) => ({
      id: column.id,
      header: column.header,
      accessorFn: column.value,
      cell: (info) => (column.format ? column.format(info.getValue(), info.row.original) : String(info.getValue())),
      sortingFn: column.align === 'right' ? 'basic' : 'alphanumeric',
      sortUndefined: 'last',
      meta: { align: column.align ?? 'left', mono: column.mono ?? false },
    })),
    [columns],
  )
  const table = useReactTable({
    data: visible as Row[],
    columns: definitions,
    state: { sorting },
    onSortingChange: setSorting,
    getRowId: (row, index) => rowKey(row, index),
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })
  const sortText = sorting[0] === undefined ? undefined : `sorted by ${columns.find((column) => column.id === sorting[0]?.id)?.header.toLowerCase() ?? sorting[0].id} ${sorting[0].desc ? 'descending' : 'ascending'}`
  const padding = cellPadding(density)
  return (
    <TableShell
      title={title}
      titleId={titleId}
      maxHeight={maxHeight}
      toolbar={(
        <>
          {textColumns.length > 0 && rows.length > 1 && <FilterField value={query} onChange={setQuery} placeholder="Search variables" label={`Search ${title.toLowerCase()}`} className="w-44" />}
          <DensityToggle density={density} onChange={setDensity} />
        </>
      )}
      count={countLine(visible.length, rows.length, noun, sortText)}
    >
      <table className={cn(tableCn, 'tabular-nums')}>
        <thead>
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
                  onToggle={() => header.column.toggleSorting()}
                  align={(header.column.columnDef.meta as { readonly align: 'left' | 'right' }).align}
                >
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </SortHeader>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {rows.length === 0 && <tr><td colSpan={columns.length} className="px-3.5 py-6 text-center text-body text-faint">{empty}</td></tr>}
          {rows.length > 0 && visible.length === 0 && <tr><td colSpan={columns.length} className="px-3.5 py-6 text-center text-body text-faint">No row matches the search.</td></tr>}
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id} className={tr('static')}>
              {row.getVisibleCells().map((cell) => {
                const meta = cell.column.columnDef.meta as { readonly align: 'left' | 'right'; readonly mono: boolean }
                return (
                  <td key={cell.id} className={td(cn(padding, meta.align === 'right' ? num('whitespace-nowrap text-right text-muted') : 'text-ink', meta.mono && 'font-mono text-muted'))}>
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                )
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </TableShell>
  )
}
