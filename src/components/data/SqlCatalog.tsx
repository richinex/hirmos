import { memo, useId, useMemo, useState } from 'react'
import {
  createColumnHelper,
  flexRender,
  getCoreRowModel,
  getSortedRowModel,
  useReactTable,
  type SortingState,
} from '@tanstack/react-table'
import { Icon } from '@/components/Icon'
import { Alert } from '@/components/ui/Alert'
import { EvidenceTable, type EvidenceColumn } from '@/components/table/EvidenceTable'
import {
  CategoryBar,
  DensityToggle,
  FilterField,
  MiniHistogram,
  SortHeader,
  TableShell,
  cellPadding,
  countLine,
  useTableDensity,
} from '@/components/table/primitives'
import {
  caption,
  chip,
  fieldHint,
  iconControl,
  literal,
  num,
  table as tableCn,
  td,
  tr,
} from '@/components/ui/recipes'
import { describeSqlPreparationProblem } from '@/data/sqlPreparation'
import { isNumericDuckDbType, type ColumnId, type ColumnSummary } from '@/domain/dataset'
import type { SqlPreparationInput } from '@/domain/sqlPreparation'
import type { SqlRelation, SqlRelationRows } from '@/domain/sqlCatalog'
import { formatCount, formatPercent, formatStored, storedDigits } from '@/lib/format/number'
import { cn } from '@/lib/utils'
import { columnDecimals, PreviewCellText, sortableValue } from './previewCell'
import { TypeGlyph } from './ReadAs'
import type { Catalog, Inspection } from './sqlSession'

function SqlTableItem({
  relation,
  fileName,
  selected,
  onInspect,
}: {
  readonly relation: SqlRelation
  readonly fileName: string | null
  readonly selected: boolean
  readonly onInspect: (relation: SqlRelation) => void
}) {
  const [open, setOpen] = useState(false)
  const columnsId = useId()
  const count = formatCount(relation.columns.length, {
    noun: relation.columns.length === 1 ? 'column' : 'columns',
  })
  return (
    <li className="min-w-0">
      <div className={cn('flex min-w-0 items-start gap-1 rounded-md', selected && 'bg-raised')}>
        <button
          type="button"
          aria-expanded={open}
          aria-controls={columnsId}
          aria-label={`Columns of ${relation.name}`}
          className={iconControl('quiet', 'mt-0.5 size-6 shrink-0')}
          onClick={() => setOpen((current) => !current)}
        >
          <Icon name={open ? 'expand_more' : 'chevron_right'} size={16} />
        </button>
        <button
          type="button"
          aria-pressed={selected}
          className="flex min-w-0 flex-1 items-start gap-2 py-1 pr-1 text-left"
          onClick={() => onInspect(relation)}
        >
          <span className="min-w-0 flex-1">
            <code className={literal('block truncate text-body text-ink')}>{relation.name}</code>
            {fileName !== null && (
              <span className={num('block truncate text-label text-faint')}>{fileName}</span>
            )}
          </span>
          <span className={chip('shrink-0 text-faint')}>{relation.kind}</span>
          <span className={num('w-6 shrink-0 text-right text-label text-faint')} title={count.text}>
            <span aria-hidden>{relation.columns.length}</span>
            <span className="sr-only">{count.text}</span>
          </span>
        </button>
      </div>
      {open && (
        <ul
          id={columnsId}
          aria-label={`Columns of ${relation.name}`}
          className="mb-1 ml-7 mt-0.5 list-none space-y-0.5 p-0"
        >
          {relation.columns.map((column) => (
            <li key={column.id} className="flex min-w-0 items-baseline justify-between gap-3">
              <code className={literal('min-w-0 truncate text-label text-muted')}>
                {column.name}
              </code>
              <span className={literal('shrink-0 text-micro text-faint')}>{column.type}</span>
            </li>
          ))}
        </ul>
      )}
    </li>
  )
}

/** The console's input files, tables and views, each with its columns, read from DuckDB's catalog. */
export const SqlTables = memo(function SqlTables({
  catalog,
  inputs,
  selected,
  onInspect,
}: {
  readonly catalog: Catalog
  readonly inputs: readonly SqlPreparationInput[]
  readonly selected: string | null
  readonly onInspect: (relation: SqlRelation) => void
}) {
  const files = useMemo(
    () =>
      new Map(
        inputs
          .filter((input) => input.format !== 'duckdb-export-file')
          .map((input) => [input.alias as string, input.fileName]),
      ),
    [inputs],
  )
  return (
    <div>
      <p className="m-0 text-body font-medium text-ink">Tables</p>
      {catalog.kind === 'checking' && <p className={cn(fieldHint, 'mt-2')}>Listing the tables.</p>}
      {catalog.kind === 'failed' && (
        <p role="alert" className="mt-2 text-body text-danger">
          {describeSqlPreparationProblem(catalog.problem)}
        </p>
      )}
      {catalog.kind === 'listed' && catalog.relations.length === 0 && (
        <p className={cn(fieldHint, 'mt-2')}>Create a table or view to see it here.</p>
      )}
      {catalog.kind === 'listed' && catalog.relations.length > 0 && (
        <ul aria-label="Tables" className="mb-0 mt-2 list-none space-y-0.5 p-0">
          {catalog.relations.map((relation) => (
            <SqlTableItem
              key={relation.name}
              relation={relation}
              fileName={relation.kind === 'input' ? (files.get(relation.name) ?? null) : null}
              selected={relation.name === selected}
              onInspect={onInspect}
            />
          ))}
        </ul>
      )}
    </div>
  )
})

interface PreviewRow {
  readonly index: number
  readonly cells: SqlRelationRows['rows'][number]
}

function SqlRowsTable({
  relation,
  data,
}: {
  readonly relation: SqlRelation
  readonly data: SqlRelationRows
}) {
  const decimals = useMemo(
    () => relation.columns.map((_, index) => columnDecimals(data.rows, index)),
    [data.rows, relation.columns],
  )
  const columns = useMemo<EvidenceColumn<PreviewRow>[]>(
    () => [
      { id: '#', header: '#', align: 'right', value: (row) => row.index + 1 },
      ...relation.columns.map((column, columnIndex): EvidenceColumn<PreviewRow> => ({
        id: column.id,
        header: column.name,
        detail: column.type,
        align: isNumericDuckDbType(column.type) ? 'right' : 'left',
        mono: false,
        value: (row) => sortableValue(row.cells[columnIndex]),
        format: (_value, row) => {
          const cell = row.cells[columnIndex]
          return cell === undefined ? null : (
            <PreviewCellText cell={cell} decimals={decimals[columnIndex] ?? 0} />
          )
        },
      })),
    ],
    [decimals, relation.columns],
  )
  const rows = useMemo<PreviewRow[]>(
    () => data.rows.map((cells, index) => ({ index, cells })),
    [data.rows],
  )
  return (
    <EvidenceTable
      appearance="data"
      title={relation.name}
      rows={rows}
      columns={columns}
      rowKey={(row) => String(row.index)}
      noun="row"
      empty="The table has no rows."
      total={data.rowCount}
      exportName={`${relation.name}-preview`}
      maxHeight="max-h-none"
      frame="none"
    />
  )
}

type Summary = Extract<Inspection, { readonly kind: 'read' }>['summary']

interface SchemaRow {
  readonly id: ColumnId
  readonly name: string
  readonly type: string
  readonly numeric: boolean
  readonly nullCount: number
  readonly nullShare: number
  readonly summary: ColumnSummary | null
}

const trimNumber = (raw: string): string => {
  if (/^-?\d+$/.test(raw)) return storedDigits(raw)
  const value = Number(raw)
  return Number.isFinite(value) ? formatStored(value).text : raw
}

const schemaHelper = createColumnHelper<SchemaRow>()

function SqlSchemaTable({
  relation,
  data,
  summary,
}: {
  readonly relation: SqlRelation
  readonly data: SqlRelationRows
  readonly summary: Summary
}) {
  const titleId = useId()
  const [density, setDensity] = useTableDensity()
  const [search, setSearch] = useState('')
  const [sorting, setSorting] = useState<SortingState>([])
  const rowCount = data.rowCount

  const rows = useMemo<readonly SchemaRow[]>(
    () =>
      relation.columns.map((column) => {
        const nullCount = data.nullCounts.get(column.id) ?? 0
        return {
          id: column.id,
          name: column.name,
          type: column.type,
          numeric: isNumericDuckDbType(column.type),
          nullCount,
          nullShare: rowCount === 0 ? 0 : nullCount / rowCount,
          summary: summary.kind === 'summarized' ? (summary.byColumn.get(column.id) ?? null) : null,
        }
      }),
    [data.nullCounts, relation.columns, rowCount, summary],
  )
  const query = search.trim().toLowerCase()
  const visible = useMemo(
    () =>
      query.length === 0 ? rows : rows.filter((row) => row.name.toLowerCase().includes(query)),
    [query, rows],
  )
  const pending = summary.kind === 'idle' || summary.kind === 'summarizing'

  const columns = useMemo(
    () => [
      schemaHelper.accessor('name', {
        header: 'Column',
        cell: (context) => (
          <span className="flex min-w-0 items-center gap-2">
            <TypeGlyph type={context.row.original.type} />
            <span className="block min-w-0 flex-1 truncate text-table text-ink">
              {context.getValue()}
            </span>
          </span>
        ),
      }),
      schemaHelper.accessor('type', {
        header: 'Type',
        cell: (context) => (
          <span className="whitespace-nowrap">
            <span className={literal('text-faint')}>{context.getValue()}</span>
          </span>
        ),
      }),
      schemaHelper.accessor('nullCount', {
        header: 'Missing',
        meta: { align: 'right' },
        cell: (context) => (
          <span className={context.getValue() > 0 ? 'text-ink' : 'text-faint'}>
            {formatCount(context.getValue()).text}
          </span>
        ),
      }),
      schemaHelper.accessor('nullShare', {
        header: '%',
        meta: { align: 'right' },
        cell: (context) => {
          const row = context.row.original
          return (
            <span
              className="relative block min-w-12"
              title={`${formatCount(row.nullCount).text} of ${formatCount(rowCount).text} rows missing`}
            >
              {row.nullShare > 0 && (
                <span
                  aria-hidden
                  className="absolute inset-y-1 right-0 rounded-sm bg-bone/40"
                  style={{ width: `${Math.max(2, Math.round(row.nullShare * 100))}%` }}
                />
              )}
              <span className={cn('relative', row.nullShare > 0 ? 'text-ink' : 'text-faint')}>
                {formatPercent(row.nullShare).text}
              </span>
            </span>
          )
        },
      }),
      schemaHelper.accessor((row) => row.summary?.distinctCount ?? -1, {
        id: 'distinct',
        header: 'Distinct',
        meta: { align: 'right' },
        cell: (context) => {
          const row = context.row.original
          if (row.summary === null) return <span className="text-faint">{pending ? '…' : '—'}</span>
          const identifier = row.summary.distinctCount >= rowCount && rowCount > 1
          return (
            <span
              className={identifier ? 'text-muted' : 'text-ink'}
              title={identifier ? 'Every row is distinct: an identifier' : undefined}
            >
              {formatCount(row.summary.distinctCount, { compact: true }).text}
              {identifier && (
                <span aria-hidden className="ml-1 text-faint">
                  =
                </span>
              )}
            </span>
          )
        },
      }),
      schemaHelper.accessor((row) => row.summary?.min ?? '', {
        id: 'range',
        header: 'Range',
        enableSorting: false,
        cell: (context) => {
          const row = context.row.original
          if (row.summary === null) return <span className="text-faint">{pending ? '…' : '—'}</span>
          if (row.summary.min === null || row.summary.max === null)
            return <span className="text-faint">—</span>
          return (
            <span
              className={cn('whitespace-nowrap', row.numeric ? 'text-ink' : 'text-muted')}
              title={`${row.summary.min} to ${row.summary.max}`}
            >
              {row.numeric ? trimNumber(row.summary.min) : row.summary.min.slice(0, 12)}{' '}
              <span className="text-faint">…</span>{' '}
              {row.numeric ? trimNumber(row.summary.max) : row.summary.max.slice(0, 12)}
            </span>
          )
        },
      }),
      schemaHelper.display({
        id: 'distribution',
        header: 'Distribution',
        cell: (context) => {
          const row = context.row.original
          if (row.summary === null) return null
          if (row.summary.histogram !== null) return <MiniHistogram bins={row.summary.histogram} />
          if (row.summary.categories !== null)
            return (
              <CategoryBar categories={row.summary.categories} total={rowCount - row.nullCount} />
            )
          return <span className="text-faint">—</span>
        },
      }),
    ],
    [pending, rowCount],
  )

  const table = useReactTable({
    data: visible as SchemaRow[],
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })
  const padding = cellPadding(density)

  return (
    <TableShell
      title={relation.name}
      titleId={titleId}
      frame="none"
      maxHeight="max-h-none"
      toolbar={
        <div className="flex w-full min-w-0 items-center gap-2 @lg/table:w-auto @lg/table:flex-1">
          <span role="status" className="shrink-0 whitespace-nowrap text-label text-faint">
            {visible.length === rows.length ? '' : countLine(visible.length, rows.length, 'column')}
          </span>
          <FilterField
            value={search}
            onChange={setSearch}
            placeholder="Find a column"
            label="Search columns"
            className="min-w-0 flex-1 @lg/table:w-40"
          />
          <DensityToggle density={density} onChange={setDensity} />
        </div>
      }
      foot={
        summary.kind === 'failed' ? (
          <div className="border-t border-hair px-3.5 py-1.5">
            <Alert tone="danger">{describeSqlPreparationProblem(summary.problem)}</Alert>
          </div>
        ) : undefined
      }
    >
      <table className={tableCn} aria-busy={pending}>
        <thead>
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
                  canSort={header.column.getCanSort()}
                  onToggle={() => header.column.toggleSorting()}
                  align={
                    (
                      header.column.columnDef.meta as
                        { readonly align?: 'left' | 'right' } | undefined
                    )?.align ?? 'left'
                  }
                >
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </SortHeader>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.length === 0 && (
            <tr>
              <td colSpan={columns.length} className="px-3.5 py-6 text-center text-body text-faint">
                No columns match the criteria.
              </td>
            </tr>
          )}
          {table.getRowModel().rows.map((row) => (
            <tr key={row.id} className={tr('static')}>
              {row.getVisibleCells().map((cell) => {
                const align = (
                  cell.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined
                )?.align
                return (
                  <td
                    key={cell.id}
                    className={td(
                      cn(
                        padding,
                        align === 'right' && num('whitespace-nowrap text-right'),
                        cell.column.id === 'name' && 'max-w-56',
                      ),
                    )}
                  >
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

export type PreviewView = 'rows' | 'schema'

/** The selected relation's first rows, or its columns with their summaries. */
export function SqlPreview({
  inspection,
  view,
}: {
  readonly inspection: Inspection
  readonly view: PreviewView
}) {
  return (
    <div
      className="flex h-full min-h-0 flex-col"
      data-testid="sql-preview"
      aria-busy={inspection.kind === 'reading'}
    >
      {inspection.kind === 'none' && (
        <p className={caption('m-3')}>Select a table to see its rows.</p>
      )}
      {inspection.kind === 'failed' && (
        <div className="m-3">
          <Alert tone="danger">{describeSqlPreparationProblem(inspection.problem)}</Alert>
        </div>
      )}
      {inspection.kind === 'read' && (
        <div className="min-h-0 flex-1 overflow-auto p-3">
          {view === 'rows' ? (
            <SqlRowsTable relation={inspection.relation} data={inspection.data} />
          ) : (
            <SqlSchemaTable
              relation={inspection.relation}
              data={inspection.data}
              summary={inspection.summary}
            />
          )}
        </div>
      )}
    </div>
  )
}
