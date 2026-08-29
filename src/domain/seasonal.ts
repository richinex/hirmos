import { z } from 'zod'
import type { ColumnId } from './dataset'
import { assertNever, err, ok, type NonEmptyArray, type Result } from './dop'
import type { Frequency } from './preprocessing'

/**
 * STL seasonal adjustment as a recorded preparation step: the prepared version says which columns
 * lost their seasonal component and at what period, and every chapter materialises the same series.
 */

export type SeasonalAdjustmentRecord =
  | { readonly kind: 'none' }
  | { readonly kind: 'stl'; readonly period: number; readonly robust: boolean; readonly columns: NonEmptyArray<ColumnId> }

/** The seasonal period statsmodels' STL needs, from the declared sampling; yearly rows have none. */
export const seasonalPeriodOf = (frequency: Frequency): number | null => {
  switch (frequency) {
    case 'daily': return 7
    case 'weekly': return 52
    case 'monthly': return 12
    case 'quarterly': return 4
    case 'yearly': return null
    default: return assertNever(frequency)
  }
}

export const seasonalAdjustedEvidenceSchema = z.object({
  kind: z.literal('seasonalAdjusted'),
  rows: z.number().int().positive(),
  columns: z.number().int().positive(),
  period: z.number().int().min(2),
  values: z.array(z.number().finite()),
  adjusted: z.array(z.object({
    column: z.number().int().nonnegative(),
    seasonalStrengthBefore: z.number().finite(),
    seasonalStrengthAfter: z.number().finite(),
  }).strict()),
}).strict()

export type SeasonalAdjustedEvidence = z.infer<typeof seasonalAdjustedEvidenceSchema>

export type SeasonalAdjustedEvidenceProblem = { readonly kind: 'invalid-seasonal-adjusted-evidence'; readonly detail: string }

export function parseSeasonalAdjustedEvidence(value: unknown): Result<SeasonalAdjustedEvidence, SeasonalAdjustedEvidenceProblem> {
  const parsed = seasonalAdjustedEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-seasonal-adjusted-evidence', detail: z.prettifyError(parsed.error) })
  if (parsed.data.values.length !== parsed.data.rows * parsed.data.columns) return err({ kind: 'invalid-seasonal-adjusted-evidence', detail: 'The adjusted matrix does not match rows by columns.' })
  return ok(parsed.data)
}

export function describeSeasonalAdjustment(record: SeasonalAdjustmentRecord, name: (column: ColumnId) => string): string | null {
  switch (record.kind) {
    case 'none': return null
    case 'stl': return `STL seasonal adjustment, period ${record.period}${record.robust ? ', robust' : ''}, on ${record.columns.map(name).join(', ')}`
    default: return assertNever(record)
  }
}
