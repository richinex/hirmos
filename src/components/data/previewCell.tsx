import type { ReactNode } from 'react'
import type { PreviewCell } from '@/domain/dataset'
import { assertNever } from '@/domain/dop'
import { formatAbsent, storedDigits } from '@/lib/format/number'

/**
 * How a raw data preview shows a cell. The rule follows what pandas, DuckDB's shell, Polars, R's tibble
 * and a default spreadsheet cell do: a whole number is its digits, with no thousands grouping, so a year
 * or an id reads as written; a float takes one decimal count per column, from the values on screen;
 * a wide integer keeps every digit; a missing value is a muted marker, not a blank; a date or time is
 * ISO 8601. Grouped, rounded figures belong to counts in prose, which go through formatCount.
 */

const decimalsOf = (value: number): number => {
  if (Number.isInteger(value)) return 0
  const text = String(value)
  const exponent = text.indexOf('e-')
  if (exponent >= 0) return Math.min(6, Number(text.slice(exponent + 2)) + 1)
  const point = text.indexOf('.')
  return point < 0 ? 0 : Math.min(6, text.length - point - 1)
}

export const columnDecimals = (rows: readonly (readonly PreviewCell[] | null)[], columnIndex: number): number => {
  let decimals = 0
  for (const row of rows) {
    const cell = row?.[columnIndex]
    if (cell?.kind === 'number') decimals = Math.max(decimals, decimalsOf(cell.value))
  }
  return Math.min(decimals, 4)
}

export const previewCellText = (cell: PreviewCell, decimals: number): string => {
  switch (cell.kind) {
    case 'null': return formatAbsent('unavailable', 'missing value').text
    case 'number': return storedDigits(Number.isFinite(cell.value) ? cell.value.toFixed(decimals) : String(cell.value))
    case 'integer': return storedDigits(cell.value)
    case 'boolean': return cell.value ? 'true' : 'false'
    case 'temporal': return cell.value.replace('T', ' ').replace(/\.000Z$/, '').replace(/Z$/, '')
    case 'text': return cell.value
    default: return assertNever(cell)
  }
}

export function PreviewCellText({ cell, decimals }: { readonly cell: PreviewCell; readonly decimals: number }): ReactNode {
  if (cell.kind === 'null') {
    const absent = formatAbsent('unavailable', 'missing value')
    return <span className="text-muted" title="Missing value"><span className="sr-only">{absent.srText}</span><span aria-hidden>{absent.text}</span></span>
  }
  return previewCellText(cell, decimals)
}

export const isNumericCell = (cell: PreviewCell): boolean => cell.kind === 'number' || cell.kind === 'integer'
