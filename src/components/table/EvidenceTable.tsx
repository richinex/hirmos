import { useCallback, useEffect, useId, useMemo, useRef, useState, type ReactNode } from 'react'
import { flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type ColumnDef, type SortingState } from '@tanstack/react-table'
import { useVirtualizer } from '@tanstack/react-virtual'
import { cellPadding, countLine, DensityToggle, FilterField, ROW_HEIGHT, SortHeader, TableShell, useTableDensity } from '@/components/table/primitives'
import { button, num, table as tableCn, td, tr } from '@/components/ui/recipes'
import { toCsv } from '@/lib/csv'
import { cn } from '@/lib/utils'

/**
 * A sortable evidence table over plain rows: the same shell, header cells, density switch and count
 * line as the Data Studio tables, so every figure a run reports can be ordered and searched the same
 * way. Columns declare a raw value for sorting and searching and, separately, how it prints.
 *
 * Above `VIRTUALISE_ABOVE` rows the body renders a window instead of every row. A lag graph reports
 * one cell per ordered pair per lag, so a 32-variable run at maximum lag reaches 21,504 rows; below
 * the threshold the plain body keeps native table semantics for find-in-page and screen readers.
 */

/** Render every row up to this count; above it, render a scrolled window. */
const VIRTUALISE_ABOVE = 200

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

type CellMeta = { readonly align: 'left' | 'right'; readonly mono: boolean }

export function EvidenceTable<Row>({ title, rows, columns, rowKey, noun, empty, filters, total, exportName, maxHeight = 'max-h-96', frame = 'panel' }: {
  readonly title: string
  readonly rows: readonly Row[]
  readonly columns: readonly EvidenceColumn<Row>[]
  readonly rowKey: (row: Row, index: number) => string
  /** Singular noun for the count line: "cell", "weight", "edge". */
  readonly noun: string
  /** What to say when the run reported nothing. */
  readonly empty: string
  /** Domain controls for narrowing the rows, shown in the toolbar before the search field. */
  readonly filters?: ReactNode
  /** Rows before any caller-side narrowing, so the count line reports what the filters removed. */
  readonly total?: number
  /** File stem for the CSV download; omit to withhold the export. */
  readonly exportName?: string
  readonly maxHeight?: string
  /** `none` where the table already sits inside a card of the same fill. */
  readonly frame?: 'panel' | 'none'
}) {
  const titleId = useId()
  const scrollRef = useRef<HTMLDivElement | null>(null)
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
      meta: { align: column.align ?? 'left', mono: column.mono ?? false } satisfies CellMeta,
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
  const modelRows = table.getRowModel().rows
  const virtualised = modelRows.length > VIRTUALISE_ABOVE
  const rowHeight = ROW_HEIGHT[density]
  const virtualizer = useVirtualizer({
    count: virtualised ? modelRows.length : 0,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => rowHeight,
    overscan: 12,
  })
  useEffect(() => { if (virtualised) virtualizer.measure() }, [rowHeight, virtualised, virtualizer])

  // Recycling replaces the focused cell's element, so remember which cell held focus by its
  // row and column and restore it after the window repaints. The frame retry covers the paint
  // that has not happened yet when the effect runs.
  const focusToken = useRef<string | null>(null)
  const rememberFocus = useCallback((token: string) => { focusToken.current = token }, [])
  useEffect(() => {
    const token = focusToken.current
    if (!virtualised || token === null) return undefined
    const scroller = scrollRef.current
    if (scroller === null) return undefined
    if (scroller.querySelector(`[data-cell="${CSS.escape(token)}"]`) === document.activeElement) return undefined
    const frame = requestAnimationFrame(() => {
      const target = scroller.querySelector<HTMLElement>(`[data-cell="${CSS.escape(token)}"]`)
      if (target !== null) target.focus({ preventScroll: true })
    })
    return () => cancelAnimationFrame(frame)
  })

  const download = useCallback(() => {
    if (exportName === undefined) return
    const body = modelRows.map((row) => columns.map((column) => column.value(row.original)))
    const blob = new Blob([toCsv(columns.map((column) => column.header), body)], { type: 'text/csv;charset=utf-8' })
    const url = URL.createObjectURL(blob)
    const anchor = document.createElement('a')
    anchor.href = url
    anchor.download = `${exportName}.csv`
    anchor.click()
    URL.revokeObjectURL(url)
  }, [columns, exportName, modelRows])

  const sortText = sorting[0] === undefined ? undefined : `sorted by ${columns.find((column) => column.id === sorting[0]?.id)?.header.toLowerCase() ?? sorting[0].id} ${sorting[0].desc ? 'descending' : 'ascending'}`
  const padding = cellPadding(density)
  const items = virtualizer.getVirtualItems()
  const leadHeight = items[0]?.start ?? 0
  const tailHeight = virtualizer.getTotalSize() - (items.at(-1)?.end ?? 0)

  const renderRow = (row: (typeof modelRows)[number], rowIndex: number) => (
    <tr key={row.id} className={tr('static')} style={virtualised ? { height: rowHeight } : undefined} aria-rowindex={virtualised ? rowIndex + 2 : undefined}>
      {row.getVisibleCells().map((cell) => {
        const meta = cell.column.columnDef.meta as CellMeta
        const token = `${row.id}:${cell.column.id}`
        return (
          <td
            key={cell.id}
            data-cell={token}
            tabIndex={virtualised ? -1 : undefined}
            onFocus={virtualised ? () => rememberFocus(token) : undefined}
            className={td(cn(padding, meta.align === 'right' ? num('whitespace-nowrap text-right text-muted') : 'text-ink', meta.mono && 'font-mono text-muted'))}
          >
            {flexRender(cell.column.columnDef.cell, cell.getContext())}
          </td>
        )
      })}
    </tr>
  )

  return (
    <TableShell
      title={title}
      titleId={titleId}
      frame={frame}
      maxHeight={maxHeight}
      scrollRef={scrollRef}
      toolbar={(
        <>
          {filters}
          {textColumns.length > 0 && rows.length > 1 && <FilterField value={query} onChange={setQuery} placeholder="Search variables" label={`Search ${title.toLowerCase()}`} className="w-44" />}
          <DensityToggle density={density} onChange={setDensity} />
          {exportName !== undefined && modelRows.length > 0 && (
            <button type="button" className={button('quiet')} onClick={download}>Export CSV</button>
          )}
        </>
      )}
      count={countLine(visible.length, total ?? rows.length, noun, sortText)}
    >
      <table className={cn(tableCn, 'tabular-nums')} aria-rowcount={virtualised ? modelRows.length + 1 : undefined}>
        <thead>
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
                  onToggle={() => header.column.toggleSorting()}
                  align={(header.column.columnDef.meta as CellMeta).align}
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
          {!virtualised && modelRows.map((row, index) => renderRow(row, index))}
          {virtualised && leadHeight > 0 && (
            <tr role="presentation" aria-hidden><td role="presentation" colSpan={columns.length} style={{ height: leadHeight, padding: 0, border: 0 }} /></tr>
          )}
          {virtualised && items.map((item) => {
            const row = modelRows[item.index]
            return row === undefined ? null : renderRow(row, item.index)
          })}
          {virtualised && tailHeight > 0 && (
            <tr role="presentation" aria-hidden><td role="presentation" colSpan={columns.length} style={{ height: tailHeight, padding: 0, border: 0 }} /></tr>
          )}
        </tbody>
      </table>
    </TableShell>
  )
}
