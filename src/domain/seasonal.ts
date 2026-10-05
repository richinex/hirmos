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
  | {
      readonly kind: 'stl'
      readonly period: number
      readonly robust: boolean
      readonly columns: NonEmptyArray<ColumnId>
    }

/** The seasonal period statsmodels' STL needs, from the declared sampling; yearly rows have none. */
export const seasonalPeriodOf = (frequency: Frequency): number | null => {
  switch (frequency) {
    case 'daily':
      return 7
    case 'weekly':
      return 52
    case 'monthly':
      return 12
    case 'quarterly':
      return 4
    case 'yearly':
      return null
    default:
      return assertNever(frequency)
  }
}

const stlComponentSchema = z
  .object({
    column: z.number().int().nonnegative(),
    observed: z.array(z.number().finite()),
    trend: z.array(z.number().finite()),
    seasonal: z.array(z.number().finite()),
    remainder: z.array(z.number().finite()),
    robustWeights: z.array(z.number().min(0).max(1)),
    trendStrength: z.number().min(0).max(1),
    seasonalStrengthBefore: z.number().min(0).max(1),
    seasonalStrengthAfter: z.number().min(0).max(1),
  })
  .strict()

export const seasonalAdjustedEvidenceSchema = z
  .object({
    kind: z.literal('seasonalAdjusted'),
    rows: z.number().int().positive(),
    columns: z.number().int().positive(),
    period: z.number().int().min(2),
    values: z.array(z.number().finite()),
    adjusted: z.tuple([stlComponentSchema]).rest(stlComponentSchema),
  })
  .strict()

export type SeasonalAdjustedEvidence = z.infer<typeof seasonalAdjustedEvidenceSchema>

export type SeasonalAdjustedEvidenceProblem = {
  readonly kind: 'invalid-seasonal-adjusted-evidence'
  readonly detail: string
}

export function parseSeasonalAdjustedEvidence(
  value: unknown,
): Result<SeasonalAdjustedEvidence, SeasonalAdjustedEvidenceProblem> {
  const parsed = seasonalAdjustedEvidenceSchema.safeParse(value)
  if (!parsed.success)
    return err({
      kind: 'invalid-seasonal-adjusted-evidence',
      detail: z.prettifyError(parsed.error),
    })
  const { adjusted, columns, rows, values } = parsed.data
  if (values.length !== rows * columns)
    return err({
      kind: 'invalid-seasonal-adjusted-evidence',
      detail: 'The adjusted matrix does not match rows by columns.',
    })
  if (adjusted.some(({ column }) => column >= columns))
    return err({
      kind: 'invalid-seasonal-adjusted-evidence',
      detail: 'An STL component names a column outside the adjusted matrix.',
    })
  if (new Set(adjusted.map(({ column }) => column)).size !== adjusted.length)
    return err({
      kind: 'invalid-seasonal-adjusted-evidence',
      detail: 'An adjusted column appears more than once.',
    })
  if (
    adjusted.some(({ observed, trend, seasonal, remainder, robustWeights }) =>
      [observed, trend, seasonal, remainder, robustWeights].some(
        (component) => component.length !== rows,
      ),
    )
  ) {
    return err({
      kind: 'invalid-seasonal-adjusted-evidence',
      detail: 'Each STL component must contain one value per adjusted row.',
    })
  }
  for (const component of adjusted) {
    for (let row = 0; row < rows; row += 1) {
      const scale = 1 + Math.abs(component.observed[row])
      const reconstruction =
        component.trend[row] + component.seasonal[row] + component.remainder[row]
      if (Math.abs(component.observed[row] - reconstruction) > 1e-10 * scale)
        return err({
          kind: 'invalid-seasonal-adjusted-evidence',
          detail: 'The STL components do not reconstruct the observed series.',
        })
      if (
        Math.abs(
          values[component.column * rows + row] -
            (component.observed[row] - component.seasonal[row]),
        ) >
        1e-10 * scale
      )
        return err({
          kind: 'invalid-seasonal-adjusted-evidence',
          detail:
            'The adjusted matrix does not equal the observed series minus its seasonal component.',
        })
    }
  }
  return ok(parsed.data)
}

export function describeSeasonalAdjustment(
  record: SeasonalAdjustmentRecord,
  name: (column: ColumnId) => string,
): string | null {
  switch (record.kind) {
    case 'none':
      return null
    case 'stl':
      return `STL seasonal adjustment, period ${record.period}${record.robust ? ', robust' : ''}, on ${record.columns.map(name).join(', ')}`
    default:
      return assertNever(record)
  }
}
