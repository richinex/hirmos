import { brand, isNonEmpty, type Brand, type NonEmptyArray } from './dop'
import { columnId, type ColumnId, type ColumnSummary, type PreviewCell } from './dataset'

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
