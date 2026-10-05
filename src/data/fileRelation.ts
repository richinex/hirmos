import { duckDbColumnTypes, type FileReading } from '@/domain/fileReading'

const sqlString = (value: string): string => `'${value.replaceAll("'", "''")}'`

/**
 * The table a registered file holds, read as its reading declares. `textColumn` is read as text
 * whatever its declaration, for a time column whose text is parsed by an interpretation.
 */
export const fileRelation = (reading: FileReading, path: string, textColumn?: string): string => {
  if (reading.format === 'parquet') return `read_parquet(${sqlString(path)})`
  const types = new Map(duckDbColumnTypes(reading.declared))
  if (textColumn !== undefined) types.set(textColumn, 'VARCHAR')
  const declared =
    types.size === 0
      ? ''
      : `, types = {${[...types].map(([column, type]) => `${sqlString(column)}: '${type}'`).join(', ')}}`
  return `read_csv_auto(${sqlString(path)}, header = true, sample_size = 20480${declared})`
}
