import { isNumericDuckDbType } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'

/** The four families a DuckDB type falls into, each with the glyph the schema shows beside a column. */
export type ColumnKind = 'numeric' | 'text' | 'temporal' | 'boolean'

export const kindOf = (duckdbType: string): ColumnKind => {
  if (isNumericDuckDbType(duckdbType)) return 'numeric'
  if (duckdbType === 'BOOLEAN') return 'boolean'
  if (/^(DATE|TIME|TIMESTAMP|INTERVAL)/.test(duckdbType)) return 'temporal'
  return 'text'
}

export const kindGlyph = (kind: ColumnKind): string => {
  switch (kind) {
    case 'numeric':
      return '#'
    case 'text':
      return 'Aa'
    case 'temporal':
      return '⏱'
    case 'boolean':
      return '◐'
    default:
      return assertNever(kind)
  }
}

export const kindText = (kind: ColumnKind): string => {
  switch (kind) {
    case 'numeric':
      return 'Numeric'
    case 'text':
      return 'Text'
    case 'temporal':
      return 'Temporal'
    case 'boolean':
      return 'Boolean'
    default:
      return assertNever(kind)
  }
}

export const KINDS: readonly ColumnKind[] = ['numeric', 'text', 'temporal', 'boolean']
