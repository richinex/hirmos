import { z } from 'zod'
import { err, ok, type Result } from './dop'

/**
 * How a file's columns are read. A Parquet file stores its column types. A delimited file has none,
 * so DuckDB detects each column's type from its values unless the reader declares one. The
 * declarable types are the ones DuckDB chooses between when it detects a type.
 */
export const DECLARED_TYPES = {
  text: 'VARCHAR',
  integer: 'BIGINT',
  decimal: 'DOUBLE',
  'yes-no': 'BOOLEAN',
  date: 'DATE',
  time: 'TIME',
  timestamp: 'TIMESTAMP',
} as const

export type DeclaredType = keyof typeof DECLARED_TYPES

export const DECLARED_TYPE_NAMES: readonly DeclaredType[] = Object.keys(
  DECLARED_TYPES,
) as DeclaredType[]

export const declaredTypeLabel = (type: DeclaredType): string => {
  switch (type) {
    case 'text':
      return 'Text'
    case 'integer':
      return 'Whole number'
    case 'decimal':
      return 'Decimal number'
    case 'yes-no':
      return 'Yes or no'
    case 'date':
      return 'Date'
    case 'time':
      return 'Time'
    case 'timestamp':
      return 'Date and time'
  }
}

/** Declared types by column name. A column absent here is read as DuckDB detects it. */
export type ColumnDeclarations = Readonly<Record<string, DeclaredType>>

export const NO_DECLARATIONS: ColumnDeclarations = Object.freeze({})

export type DelimitedFormat = 'csv' | 'tsv'

export type FileReading =
  | { readonly format: DelimitedFormat; readonly declared: ColumnDeclarations }
  | { readonly format: 'parquet' }

/** Just the reading of anything that carries one: a source, a profile's source, an input. */
export const fileReading = (value: FileReading): FileReading =>
  value.format === 'parquet'
    ? { format: value.format }
    : { format: value.format, declared: value.declared }

/** The declarations a reading applies; a Parquet file has none. */
export const declarationsOf = (reading: FileReading): ColumnDeclarations =>
  reading.format === 'parquet' ? NO_DECLARATIONS : reading.declared

/** What one column is read as. */
export type ColumnReading =
  { readonly kind: 'detected' } | { readonly kind: 'declared'; readonly type: DeclaredType }

export const columnReading = (reading: FileReading, column: string): ColumnReading => {
  if (reading.format === 'parquet') return { kind: 'detected' }
  const type = Object.hasOwn(reading.declared, column) ? reading.declared[column] : undefined
  return type === undefined ? { kind: 'detected' } : { kind: 'declared', type }
}

/** Declares one column's type, or with `null` returns it to detection. Other columns are unchanged. */
export const declareColumn = (
  declared: ColumnDeclarations,
  column: string,
  type: DeclaredType | null,
): ColumnDeclarations => {
  const { [column]: _previous, ...others } = declared
  return type === null ? others : { ...others, [column]: type }
}

/** The DuckDB type each declared column is read as, in column-name order so equal declarations give equal text. */
export const duckDbColumnTypes = (
  declared: ColumnDeclarations,
): readonly (readonly [string, string])[] =>
  Object.keys(declared)
    .sort()
    .map((column) => [column, DECLARED_TYPES[declared[column]!]] as const)

/** A stable text for a set of declarations: empty when none are declared. */
export const declarationsKey = (declared: ColumnDeclarations): string =>
  duckDbColumnTypes(declared)
    .map(([column, type]) => `${column}=${type}`)
    .join(';')

export const declaredTypeSchema = z.enum(DECLARED_TYPE_NAMES as [DeclaredType, ...DeclaredType[]])

export const columnDeclarationsSchema = z.record(z.string().min(1), declaredTypeSchema)

export type FileReadingProblem = { readonly kind: 'declarations-on-parquet' }

/** A stored format and declarations, read back. Records written before declarations existed have none. */
export const fileReadingOf = (
  format: DelimitedFormat | 'parquet',
  declared: ColumnDeclarations | undefined,
): Result<FileReading, FileReadingProblem> => {
  if (format === 'parquet') {
    return declared === undefined || Object.keys(declared).length === 0
      ? ok({ format })
      : err({ kind: 'declarations-on-parquet' })
  }
  return ok({ format, declared: declared ?? NO_DECLARATIONS })
}
