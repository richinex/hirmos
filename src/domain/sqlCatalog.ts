import { assertNever, brand, isNonEmpty, type Brand, type NonEmptyArray, type Result } from './dop'
import { columnId, type ColumnId, type ColumnSummary, type PreviewCell } from './dataset'
import type { SqlPreparationProblem } from '@/data/sqlPreparation'

export type SqlRelationName = Brand<string, 'SqlRelationName'>

/** Where a relation in the SQL console comes from: a chosen file, or a statement the reader ran. */
export type SqlRelationKind = 'input' | 'table' | 'view'

export interface SqlColumn {
  readonly id: ColumnId
  readonly name: string
  readonly type: string
}

export interface SqlRelation {
  readonly name: SqlRelationName
  readonly kind: SqlRelationKind
  readonly columns: NonEmptyArray<SqlColumn>
}

/** One row of DuckDB's column catalog, in catalog order. */
export interface SqlCatalogRow {
  readonly relation: string
  readonly view: boolean
  readonly column: string
  readonly type: string
}

/** The relations the console can read, grouped from the column catalog in the order DuckDB lists them. */
export function sqlCatalog(
  rows: readonly SqlCatalogRow[],
  inputs: ReadonlySet<string>,
): readonly SqlRelation[] {
  const grouped = new Map<string, { readonly view: boolean; readonly columns: SqlColumn[] }>()
  for (const row of rows) {
    const entry = grouped.get(row.relation) ?? { view: row.view, columns: [] }
    entry.columns.push({
      id: columnId(entry.columns.length, row.column),
      name: row.column,
      type: row.type,
    })
    grouped.set(row.relation, entry)
  }
  return [...grouped].flatMap(([name, entry]) =>
    isNonEmpty(entry.columns)
      ? [
          {
            name: brand<string, 'SqlRelationName'>(name),
            kind: inputs.has(name) ? 'input' : entry.view ? 'view' : 'table',
            columns: entry.columns,
          },
        ]
      : [],
  )
}

export const sameRelation = (left: SqlRelation, right: SqlRelation): boolean =>
  left.name === right.name &&
  left.kind === right.kind &&
  left.columns.length === right.columns.length &&
  left.columns.every(
    (column, index) =>
      column.name === right.columns[index]?.name && column.type === right.columns[index]?.type,
  )

export interface SqlRelationRows {
  readonly rowCount: number
  readonly nullCounts: ReadonlyMap<ColumnId, number>
  readonly rows: readonly (readonly PreviewCell[])[]
}

export type SqlRelationSummary = ReadonlyMap<ColumnId, ColumnSummary>

export type SqlCatalogState =
  | { readonly kind: 'checking' }
  | { readonly kind: 'listed'; readonly relations: readonly SqlRelation[] }
  | { readonly kind: 'failed'; readonly problem: SqlPreparationProblem }

export type SqlSummaryState =
  | { readonly kind: 'idle' }
  | { readonly kind: 'summarizing' }
  | { readonly kind: 'summarized'; readonly byColumn: SqlRelationSummary }
  | { readonly kind: 'failed'; readonly problem: SqlPreparationProblem }

/** What the preview shows: nothing, a relation being read, its rows, or why they could not be read. */
export type SqlInspection =
  | { readonly kind: 'none' }
  | { readonly kind: 'reading'; readonly relation: SqlRelation }
  | {
      readonly kind: 'read'
      readonly relation: SqlRelation
      readonly data: SqlRelationRows
      readonly summary: SqlSummaryState
    }
  | {
      readonly kind: 'failed'
      readonly relation: SqlRelation
      readonly problem: SqlPreparationProblem
    }

/** What a fresh catalog listing asks of the preview. */
export type CatalogEffect =
  | { readonly kind: 'keep' }
  | { readonly kind: 'clear' }
  | { readonly kind: 'read'; readonly relation: SqlRelation; readonly inPlace: boolean }

/**
 * The first listing opens the first relation. Later listings re-read the shown relation: in place
 * while its columns are unchanged, so the rows stay on screen; afresh when they changed; and clear
 * the preview when the relation is gone.
 */
export function catalogEffect(
  inspection: SqlInspection,
  relations: readonly SqlRelation[],
  firstListing: boolean,
): CatalogEffect {
  if (inspection.kind === 'none') {
    const [first] = relations
    return firstListing && first !== undefined
      ? { kind: 'read', relation: first, inPlace: false }
      : { kind: 'keep' }
  }
  const current = relations.find((relation) => relation.name === inspection.relation.name)
  if (current === undefined) return { kind: 'clear' }
  const unchanged = sameRelation(current, inspection.relation)
  switch (inspection.kind) {
    case 'reading':
      return unchanged ? { kind: 'keep' } : { kind: 'read', relation: current, inPlace: false }
    case 'read':
      return { kind: 'read', relation: current, inPlace: unchanged }
    case 'failed':
      return { kind: 'read', relation: current, inPlace: false }
    default:
      return assertNever(inspection)
  }
}

const sameCounts = (left: SqlRelationRows, right: SqlRelationRows): boolean =>
  left.rowCount === right.rowCount &&
  [...left.nullCounts].every(([column, count]) => right.nullCounts.get(column) === count)

/** The preview while a relation is being read: unchanged for a re-read in place, otherwise the reading state. */
export const inspecting = (
  previous: SqlInspection,
  relation: SqlRelation,
  inPlace: boolean,
): SqlInspection => (inPlace && previous.kind === 'read' ? previous : { kind: 'reading', relation })

/** The preview once the rows arrive. A finished summary survives a re-read that changed no count. */
export function inspected(
  previous: SqlInspection,
  relation: SqlRelation,
  inPlace: boolean,
  read: Result<SqlRelationRows, SqlPreparationProblem>,
): SqlInspection {
  if (!read.ok) return { kind: 'failed', relation, problem: read.error }
  const summary: SqlSummaryState =
    inPlace &&
    previous.kind === 'read' &&
    previous.summary.kind === 'summarized' &&
    sameCounts(previous.data, read.value)
      ? previous.summary
      : { kind: 'idle' }
  return { kind: 'read', relation, data: read.value, summary }
}
