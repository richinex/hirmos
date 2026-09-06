import { useId, useMemo, useState } from 'react'
import { createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable, type SortingState } from '@tanstack/react-table'
import { cellPadding, countLine, FacetPills, FilterField, SortHeader, TableShell, useTableDensity } from '@/components/table/primitives'
import { num, table as tableCn, td, tdText, tr } from '@/components/ui/recipes'
import type { DagDocument, DagEdgeId, DirectedDagEdge, EdgeSupport, EdgeTiming } from '@/domain/dag'
import { assertNever } from '@/domain/dop'
import { useOpenPane } from '@/components/shell/WorkbenchLayout'
import { cn } from '@/lib/utils'

interface LedgerRow {
  readonly id: DagEdgeId
  readonly arrow: string
  readonly cause: string
  readonly effect: string
  readonly timing: string
  readonly lag: number
  readonly support: EdgeSupport['kind']
  readonly rationale: string
  readonly evidence: number
}

const timingText = (timing: EdgeTiming): string => {
  switch (timing.kind) {
    case 'contemporaneous': return 't'
    case 'lagged': return `t−${timing.lag}`
    default: return assertNever(timing)
  }
}

const rationaleOf = (support: EdgeSupport): string => {
  switch (support.kind) {
    case 'user-assumption':
    case 'experimental-design': return support.rationale
    case 'unstated': return ''
    default: return assertNever(support)
  }
}

const helper = createColumnHelper<LedgerRow>()

/** Every arrow in the current revision as one hairline row; a click selects it on the canvas. */
export function EdgeLedgerTable({ document, selectedEdge, onSelectEdge }: {
  readonly document: DagDocument
  readonly selectedEdge: DagEdgeId | null
  readonly onSelectEdge: (edge: DagEdgeId) => void
}) {
  const openPane = useOpenPane()
  const titleId = useId()
  const [density] = useTableDensity()
  const [search, setSearch] = useState('')
  const [onlyUnstated, setOnlyUnstated] = useState(false)
  const [sorting, setSorting] = useState<SortingState>([])
  const nameOf = (id: DirectedDagEdge['cause']): string => document.current.graph.nodes.find((node) => node.id === id)?.name ?? String(id)

  const rows = useMemo<readonly LedgerRow[]>(() => document.current.graph.edges.map((edge) => ({
    id: edge.id,
    arrow: `${nameOf(edge.cause)} → ${nameOf(edge.effect)}`,
    cause: nameOf(edge.cause),
    effect: nameOf(edge.effect),
    timing: timingText(edge.timing),
    lag: edge.timing.kind === 'lagged' ? edge.timing.lag : 0,
    support: edge.support.kind,
    rationale: rationaleOf(edge.support),
    evidence: edge.evidence.length,
  })), [document]) // eslint-disable-line react-hooks/exhaustive-deps

  const query = search.trim().toLowerCase()
  const visible = useMemo(() => rows.filter((row) => (!onlyUnstated || row.support === 'unstated') && (query.length === 0 || row.arrow.toLowerCase().includes(query) || row.rationale.toLowerCase().includes(query))), [onlyUnstated, query, rows])
  const unstated = rows.filter((row) => row.support === 'unstated').length

  const columns = useMemo(() => [
    helper.accessor('arrow', { header: 'Arrow', cell: (context) => <span className="text-ink">{context.getValue()}</span> }),
    helper.accessor('lag', { header: 'Timing', meta: { align: 'right' }, cell: (context) => <span className="text-muted">{context.row.original.timing}</span> }),
    helper.accessor('support', { header: 'Support', cell: (context) => <span className={context.getValue() === 'unstated' ? 'text-warn' : 'text-faint'}>{context.getValue()}</span> }),
    helper.accessor('rationale', { header: 'Rationale', cell: (context) => context.getValue().length === 0 ? <span className="text-warn">Rationale required</span> : <span className="text-muted">{context.getValue()}</span> }),
    helper.accessor('evidence', { header: 'Evidence', meta: { align: 'right' }, cell: (context) => <span className={context.getValue() === 0 ? 'text-faint' : 'text-ink'}>{context.getValue()}</span> }),
  ], [])

  const table = useReactTable({
    data: visible,
    columns,
    state: { sorting },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
  })
  const padding = cellPadding(density)

  return (
    <TableShell
      title="Arrows"
      titleId={titleId}
      frame="none"
      maxHeight="max-h-none"
      toolbar={(
        <>
          <FilterField value={search} onChange={setSearch} placeholder="Find an arrow" label="Search arrows" className="w-40" />
          <FacetPills label="Arrow state" facets={[{ id: 'unstated', text: 'Needs rationale', count: unstated, active: onlyUnstated }]} onToggle={() => setOnlyUnstated((current) => !current)} />
        </>
      )}
      count={countLine(visible.length, rows.length, 'arrow', sorting[0] === undefined ? undefined : `sorted by ${sorting[0].id} ${sorting[0].desc ? 'descending' : 'ascending'}`)}
    >
      <table className={tableCn} aria-label="Arrows">
        <thead>
          {table.getHeaderGroups().map((group) => (
            <tr key={group.id}>
              {group.headers.map((header) => (
                <SortHeader
                  key={header.id}
                  sorted={header.column.getIsSorted()}
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
          {rows.length === 0 && <tr><td colSpan={columns.length} className="px-3.5 py-4 text-body text-faint">Add an arrow to the DAG.</td></tr>}
          {rows.length > 0 && visible.length === 0 && <tr><td colSpan={columns.length} className="px-3.5 py-4 text-body text-faint">No arrow matches.</td></tr>}
          {table.getRowModel().rows.map((row) => {
            const selected = row.original.id === selectedEdge
            return (
              <tr key={row.id} className={tr(selected ? 'selected' : 'action')} onClick={() => { onSelectEdge(row.original.id); openPane('inspector') }}>
                {row.getVisibleCells().map((cell) => {
                  const align = (cell.column.columnDef.meta as { readonly align?: 'left' | 'right' } | undefined)?.align
                  return (
                    <td key={cell.id} className={cell.column.id === 'rationale' ? tdText(cn(padding, 'max-w-[560px]')) : td(cn(padding, align === 'right' && num('whitespace-nowrap text-right')))}>
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
