import { z } from 'zod'
import { assertNever, err, ok, type Result } from './dop'
import type { DenseReadyMissingness } from './preprocessing'

/**
 * The executed missingness resolution: what the data-preparation core did to the nullable matrix,
 * cell by cell, and what it refused. Absence, deliberate exclusion, and imputation stay distinct.
 */

export type ImputationMethod = 'linearInterior' | 'forwardFill' | 'structuralZero'

export type MissingnessResolutionCommand =
  | { readonly kind: 'completeInterval' }
  | { readonly kind: 'imputation'; readonly method: ImputationMethod; readonly maxGap: number; readonly confirmation: string | null }

export const STRUCTURAL_ZERO_CONFIRMATION = 'The user confirmed that a missing cell records a true zero.'

/** The façade command for a recorded policy, or null when the policy needs no execution. */
export function resolutionCommandFor(missingness: DenseReadyMissingness): MissingnessResolutionCommand | null {
  switch (missingness.kind) {
    case 'not-present': return null
    case 'complete-interval': return { kind: 'completeInterval' }
    case 'imputation': return {
      kind: 'imputation',
      method: missingness.method,
      maxGap: missingness.maxGap,
      confirmation: missingness.method === 'structuralZero' && missingness.confirmedStructuralZero ? STRUCTURAL_ZERO_CONFIRMATION : null,
    }
    default: return assertNever(missingness)
  }
}

export const missingnessResolvedEvidenceSchema = z.object({
  kind: z.literal('missingnessResolved'),
  rows: z.number().int().positive(),
  columns: z.number().int().positive(),
  resolution: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('completeInterval') }).strict(),
    z.object({ kind: z.literal('imputation'), method: z.enum(['linearInterior', 'forwardFill', 'structuralZero']), maxGap: z.number().int().nonnegative(), confirmation: z.string().nullable() }).strict(),
  ]),
  outcome: z.discriminatedUnion('kind', [
    z.object({
      kind: z.literal('completed'),
      values: z.array(z.number().finite()),
      windowStart: z.number().int().nonnegative(),
      windowEnd: z.number().int().positive(),
      imputedCells: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    }).strict(),
    z.object({
      kind: z.literal('refused'),
      reasons: z.array(z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('noCompleteInterval') }).strict(),
        z.object({ kind: z.literal('unresolvedCells'), count: z.number().int().positive() }).strict(),
      ])).min(1),
      imputedCells: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
      unresolvedRuns: z.array(z.object({ column: z.number().int().nonnegative(), start: z.number().int().nonnegative(), end: z.number().int().positive() }).strict()),
      remainingMissing: z.number().int().positive(),
    }).strict(),
  ]),
}).strict()

export type MissingnessResolvedEvidence = z.infer<typeof missingnessResolvedEvidenceSchema>

export type MissingnessEvidenceProblem = { readonly kind: 'invalid-missingness-evidence'; readonly detail: string }

export function parseMissingnessResolvedEvidence(value: unknown): Result<MissingnessResolvedEvidence, MissingnessEvidenceProblem> {
  const parsed = missingnessResolvedEvidenceSchema.safeParse(value)
  if (!parsed.success) return err({ kind: 'invalid-missingness-evidence', detail: z.prettifyError(parsed.error) })
  const { rows, columns, outcome } = parsed.data
  for (const [row, column] of outcome.imputedCells) {
    if (row >= rows || column >= columns) return err({ kind: 'invalid-missingness-evidence', detail: 'An imputed cell lies outside the source matrix.' })
  }
  if (outcome.kind === 'completed') {
    if (outcome.windowEnd <= outcome.windowStart || outcome.windowEnd > rows) {
      return err({ kind: 'invalid-missingness-evidence', detail: 'The retained window is empty, reversed, or outside the source rows.' })
    }
    if (outcome.values.length !== (outcome.windowEnd - outcome.windowStart) * columns) {
      return err({ kind: 'invalid-missingness-evidence', detail: 'The resolved values do not fill the retained window.' })
    }
  } else {
    for (const run of outcome.unresolvedRuns) {
      if (run.column >= columns || run.start >= run.end || run.end > rows) {
        return err({ kind: 'invalid-missingness-evidence', detail: 'An unresolved run lies outside the source matrix.' })
      }
    }
    const reported = outcome.reasons.reduce((sum, reason) => reason.kind === 'unresolvedCells' ? sum + reason.count : sum, 0)
    if (reported > 0 && reported !== outcome.remainingMissing) {
      return err({ kind: 'invalid-missingness-evidence', detail: 'The unresolved-cell reason disagrees with the remaining-missing count.' })
    }
  }
  return ok(parsed.data)
}

export type MissingnessRefusal = Extract<MissingnessResolvedEvidence['outcome'], { readonly kind: 'refused' }>['reasons'][number]

export function describeMissingnessRefusal(refusal: MissingnessRefusal): string {
  switch (refusal.kind) {
    case 'noCompleteInterval': return 'No row has every selected column observed.'
    case 'unresolvedCells': return `${refusal.count} cell${refusal.count === 1 ? '' : 's'} stay missing because a gap exceeds the limit or touches the edge of the series.`
    default: return assertNever(refusal)
  }
}

/** What the prepared version records about its executed resolution. */
export type MissingnessResolutionRecord =
  | { readonly kind: 'none' }
  | { readonly kind: 'imputed'; readonly method: ImputationMethod; readonly maxGap: number; readonly cells: number }
  | { readonly kind: 'window'; readonly start: number; readonly endExclusive: number; readonly sourceRows: number }

export function describeImputationMethod(method: ImputationMethod): string {
  switch (method) {
    case 'linearInterior': return 'linear interpolation inside the series'
    case 'forwardFill': return 'carry the last value forward'
    case 'structuralZero': return 'structural zero'
    default: return assertNever(method)
  }
}

export function describeResolutionRecord(record: MissingnessResolutionRecord): string | null {
  switch (record.kind) {
    case 'none': return null
    case 'imputed': return `${record.cells} cell${record.cells === 1 ? '' : 's'} imputed by ${describeImputationMethod(record.method)}${record.method === 'structuralZero' ? '' : `, gaps up to ${record.maxGap}`}`
    case 'window': return `rows ${record.start + 1} to ${record.endExclusive} kept as the longest complete interval (${record.start} before and ${record.sourceRows - record.endExclusive} after dropped)`
    default: return assertNever(record)
  }
}
