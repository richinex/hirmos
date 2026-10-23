import { z } from 'zod'
import type { DatasetProfile, PhysicalColumnProfile } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import {
  csvDialectSchema,
  duckDbColumnTypes,
  NOT_RECORDED,
  type CsvDialect,
  type DialectRecord,
  type FileReading,
} from '@/domain/fileReading'

const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`

/** How a relation reads a delimited file: with the dialect and types detected once, or by detecting them over the whole file. */
export type RelationLayout =
  | {
      readonly kind: 'known'
      readonly dialect: CsvDialect
      readonly columns: readonly Pick<PhysicalColumnProfile, 'name' | 'duckdbType'>[]
    }
  | { readonly kind: 'detect' }

export const DETECT: RelationLayout = { kind: 'detect' }

/** DuckDB's `{'column': 'TYPE', ...}` struct for column types, in the order given. */
const typesStruct = (types: Iterable<readonly [string, string]>): string =>
  `{${[...types].map(([column, type]) => `${sqlString(column)}: ${sqlString(type)}`).join(', ')}}`

/** The types a reading declares, with `textColumn` read as text, as DuckDB's `types` option or nothing. */
export const declaredTypesOption = (reading: FileReading, textColumn?: string): string => {
  if (reading.format === 'parquet') return ''
  const types = new Map(duckDbColumnTypes(reading.declared))
  if (textColumn !== undefined) types.set(textColumn, 'VARCHAR')
  return types.size === 0 ? '' : `, types = ${typesStruct(types)}`
}

/**
 * The table a registered file holds, read as its reading declares. `textColumn` is read as text
 * whatever its declaration, for a time column whose text is parsed by an interpretation. A known
 * layout reads the file with that dialect and those types instead of detecting them again.
 */
export const fileRelation = (
  reading: FileReading,
  path: string,
  textColumn?: string,
  layout: RelationLayout = DETECT,
): string => {
  if (reading.format === 'parquet') return `read_parquet(${sqlString(path)})`
  switch (layout.kind) {
    case 'known': {
      const { dialect, columns } = layout
      const types = columns.map(
        ({ name, duckdbType }) => [name, name === textColumn ? 'VARCHAR' : duckdbType] as const,
      )
      return `read_csv(${sqlString(path)}, auto_detect = false, header = true, delim = ${sqlString(dialect.delimiter)}, quote = ${sqlString(dialect.quote)}, escape = ${sqlString(dialect.escape)}, new_line = ${sqlString(dialect.newLine)}, comment = ${sqlString(dialect.comment)}, skip = ${dialect.skipRows}, columns = ${typesStruct(types)})`
    }
    case 'detect':
      // A partial sample can infer integers and silently round decimals appearing later.
      return `read_csv_auto(${sqlString(path)}, header = true, sample_size = -1${declaredTypesOption(reading, textColumn)})`
    default:
      return assertNever(layout)
  }
}

/** The layout a profiled file is read with: the dialect its profile recorded when it has one. */
export const profileLayout = (profile: DatasetProfile): RelationLayout =>
  profile.source.format === 'parquet' || profile.source.dialect.kind === 'not-recorded'
    ? DETECT
    : { kind: 'known', dialect: profile.source.dialect.dialect, columns: profile.columns }

/** The sniffer writes "(empty)" for a quote, escape or comment the file does not have. */
const sniffedOptional = z.string().transform((value) => (value === '(empty)' ? '' : value))

const sniffRowSchema = z.object({
  Delimiter: z.string(),
  Quote: sniffedOptional,
  Escape: sniffedOptional,
  NewLineDelimiter: z.string(),
  Comment: sniffedOptional,
  SkipRows: z.number(),
  Columns: z.array(z.object({ name: z.string(), type: z.string().min(1) })).min(1),
})

/** The layout `sniff_csv` reported for a whole file, or detection when its row cannot be read as one. */
export const layoutFromSniff = (row: unknown): RelationLayout => {
  const sniffed = sniffRowSchema.safeParse(row)
  if (!sniffed.success) return DETECT
  const dialect = csvDialectSchema.safeParse({
    delimiter: sniffed.data.Delimiter,
    quote: sniffed.data.Quote,
    escape: sniffed.data.Escape,
    newLine: sniffed.data.NewLineDelimiter,
    comment: sniffed.data.Comment,
    skipRows: sniffed.data.SkipRows,
  })
  if (!dialect.success) return DETECT
  return {
    kind: 'known',
    dialect: dialect.data,
    columns: sniffed.data.Columns.map(({ name, type }) => ({ name, duckdbType: type })),
  }
}

/** What a profile records of a layout: the dialect when one was detected. */
export const dialectRecord = (layout: RelationLayout): DialectRecord =>
  layout.kind === 'known' ? { kind: 'recorded', dialect: layout.dialect } : NOT_RECORDED
