import { sampleMembershipSchema } from './sampleMembership'
import { surrogateHorizonsSchema, noSurrogateHorizons } from './surrogateHorizons'
import type { SurrogatePathRequest } from './surrogatePath'
import {
  surrogateChecksSchema,
  noSurrogateChecks,
  type SurrogateDiagnosticRequest,
} from './surrogateDiagnostics'
import { z } from 'zod'
import { brand, err, ok, type NonEmptyArray, type Result } from './dop'
import { type ColumnId, type DatasetProfile, type NullableNumericMatrix } from './dataset'
import { surrogateRequestSchema, type SurrogateRequest } from './surrogate'

const column = z
  .string()
  .min(1)
  .transform((v) => brand<string, 'ColumnId'>(v))
/** Roles refer to source columns, before any complete-case filtering or imputation. */
export const surrogateSelectionSchema = z
  .object({
    sample: sampleMembershipSchema,
    treatment: column,
    outcome: column,
    surrogates: z.tuple([column]).rest(column).readonly(),
    adjustment: z
      .discriminatedUnion('kind', [
        z.object({ kind: z.literal('none') }).strict(),
        z
          .object({
            kind: z.literal('baseline'),
            columns: z.tuple([column]).rest(column).readonly(),
          })
          .strict(),
      ])
      .readonly(),
    checks: surrogateChecksSchema.default(noSurrogateChecks),
    horizons: surrogateHorizonsSchema.default(noSurrogateHorizons),
    estimator: surrogateRequestSchema.unwrap().shape.estimator,
    uncertainty: surrogateRequestSchema.unwrap().shape.uncertainty,
  })
  .strict()
  .superRefine((s, ctx) => {
    const columns = surrogateRoleColumns(s)
    if (new Set(columns).size !== columns.length)
      ctx.addIssue({
        code: 'custom',
        message:
          'Choose distinct columns for sample membership, treatment, outcome, surrogates and baseline covariates.',
      })
    if (s.horizons.windows.kind === 'groups') {
      const grouped = s.horizons.windows.groups.flatMap((g) => g.columns)
      if (
        grouped.length !== s.surrogates.length ||
        grouped.some((id) => !s.surrogates.includes(id))
      )
        ctx.addIssue({
          code: 'custom',
          message: 'Assign every selected surrogate to exactly one horizon group.',
        })
    }
    if (
      s.horizons.observed.kind === 'periods' &&
      s.horizons.observed.periods.some(
        (p) =>
          p.column === s.sample.column ||
          p.column === s.treatment ||
          (s.adjustment.kind === 'baseline' && s.adjustment.columns.includes(p.column)),
      )
    )
      ctx.addIssue({
        code: 'custom',
        message:
          'Observed outcome periods cannot be sample membership, treatment or baseline covariates.',
      })
  })
  .readonly()
export type SurrogateSelection = z.infer<typeof surrogateSelectionSchema>

function surrogateRoleColumns(s: SurrogateSelection): NonEmptyArray<ColumnId> {
  return [
    s.sample.column,
    s.treatment,
    s.outcome,
    ...s.surrogates,
    ...(s.adjustment.kind === 'baseline' ? s.adjustment.columns : []),
  ]
}

export function surrogateSelectionColumns(s: SurrogateSelection): NonEmptyArray<ColumnId> {
  const [first, ...rest] = surrogateRoleColumns(s)
  const extra =
    s.horizons.observed.kind === 'none' ? [] : s.horizons.observed.periods.map((p) => p.column)
  return [first, ...new Set([...rest, ...extra].filter((id) => id !== first))]
}

export type SurrogateSampleProblem =
  | { readonly kind: 'invalid-selection'; readonly detail: string }
  | { readonly kind: 'source-mismatch' }
  | { readonly kind: 'missing-column'; readonly column: ColumnId }
  | {
      readonly kind: 'missing-value'
      readonly row: number
      readonly column: ColumnId
      readonly name: string
      readonly sample: 'membership' | 'experimental' | 'observational'
    }
  | { readonly kind: 'invalid-request'; readonly detail: string }

export interface SurrogateSamples {
  readonly observedPath:
    { readonly kind: 'none' } | Extract<SurrogatePathRequest, { kind: 'observedOutcomes' }>
  readonly validationOutcome:
    { readonly kind: 'none' } | { readonly kind: 'observed'; readonly values: readonly number[] }
  readonly request: SurrogateRequest
  /** Zero-based source rows. Excluded rows have a known, unselected membership value. */
  readonly rows: {
    readonly experimental: readonly number[]
    readonly observational: readonly number[]
    readonly excluded: readonly number[]
  }
}

/** One materialization keeps every role aligned. Unneeded responses may be missing. */
export function selectSurrogateSamples(
  matrix: NullableNumericMatrix,
  selection: SurrogateSelection,
  profile: DatasetProfile,
): Result<SurrogateSamples, SurrogateSampleProblem> {
  const parsed = surrogateSelectionSchema.safeParse(selection)
  if (!parsed.success)
    return err({
      kind: 'invalid-selection',
      detail: parsed.error.issues.map((i) => i.message).join(' '),
    })
  const s = parsed.data
  if (
    matrix.sourceFingerprint !== profile.source.fingerprint ||
    matrix.rowCount !== profile.rowCount
  )
    return err({ kind: 'source-mismatch' })
  const positions = new Map(matrix.columns.map((c, i) => [c.id, i]))
  for (const id of surrogateSelectionColumns(s)) {
    if (!positions.has(id) || !profile.columns.some((c) => c.id === id))
      return err({ kind: 'missing-column', column: id })
  }
  const read = (
    id: ColumnId,
    row: number,
    sample: 'membership' | 'experimental' | 'observational',
  ): Result<number, SurrogateSampleProblem> => {
    const position = positions.get(id)!
    const index = position * matrix.rowCount + row
    if (
      (matrix.validity[index >> 3] & (1 << (index & 7))) === 0 ||
      !Number.isFinite(matrix.values[index])
    )
      return err({
        kind: 'missing-value',
        row,
        column: id,
        name: matrix.columns[position]!.name,
        sample,
      })
    return ok(matrix.values[index]!)
  }
  const readColumns = (
    ids: readonly ColumnId[],
    row: number,
    sample: 'experimental' | 'observational',
  ): Result<number[], SurrogateSampleProblem> => {
    const values: number[] = []
    for (const id of ids) {
      const value = read(id, row, sample)
      if (!value.ok) return value
      values.push(value.value)
    }
    return ok(values)
  }
  const rows = {
    experimental: [] as number[],
    observational: [] as number[],
    excluded: [] as number[],
  }
  const experimentalOutcome: number[] = []
  const periodOutcomes: number[][] = []
  const experimental = { surrogates: [] as number[][], treatment: [] as number[] }
  const observational = { surrogates: [] as number[][], outcome: [] as number[] }
  const baseline = { experimental: [] as number[][], observational: [] as number[][] }
  for (let row = 0; row < matrix.rowCount; row++) {
    const membership = read(s.sample.column, row, 'membership')
    if (!membership.ok) return membership
    const sample =
      membership.value === (s.sample.kind === 'numeric' ? s.sample.experimental : 1)
        ? 'experimental'
        : membership.value === (s.sample.kind === 'numeric' ? s.sample.observational : 0)
          ? 'observational'
          : null
    if (sample === null) {
      rows.excluded.push(row)
      continue
    }
    const predictors = readColumns(s.surrogates, row, sample)
    if (!predictors.ok) return predictors
    const response = read(sample === 'experimental' ? s.treatment : s.outcome, row, sample)
    if (!response.ok) return response
    if (s.adjustment.kind === 'baseline') {
      const values = readColumns(s.adjustment.columns, row, sample)
      if (!values.ok) return values
      baseline[sample].push(values.value)
    }
    if (sample === 'experimental' && s.checks.validation === 'observedOutcome') {
      const outcome = read(s.outcome, row, sample)
      if (!outcome.ok) return outcome
      experimentalOutcome.push(outcome.value)
    }
    if (sample === 'experimental' && s.horizons.observed.kind === 'periods') {
      const values = readColumns(
        s.horizons.observed.periods.map((p) => p.column),
        row,
        sample,
      )
      if (!values.ok) return values
      periodOutcomes.push(values.value)
    }
    rows[sample].push(row)
    if (sample === 'experimental') {
      experimental.surrogates.push(predictors.value)
      experimental.treatment.push(response.value)
    } else {
      observational.surrogates.push(predictors.value)
      observational.outcome.push(response.value)
    }
  }
  if (rows.experimental.length === 0 || rows.observational.length === 0)
    return err({
      kind: 'invalid-request',
      detail:
        'Both samples need observations. Check that the membership values exactly match values in the source.',
    })
  const request = surrogateRequestSchema.safeParse({
    experimental,
    observational,
    adjustment: s.adjustment.kind === 'none' ? { kind: 'none' } : { kind: 'baseline', ...baseline },
    estimator: s.estimator,
    uncertainty: s.uncertainty,
  })
  return request.success
    ? ok({
        request: request.data,
        rows,
        observedPath:
          s.horizons.observed.kind === 'none'
            ? { kind: 'none' }
            : {
                kind: 'observedOutcomes',
                uncertainty: s.uncertainty,
                outcomes: periodOutcomes,
                treatment: request.data.experimental.treatment,
                labels: s.horizons.observed.periods.map((p) => p.label),
              },
        validationOutcome:
          s.checks.validation === 'none'
            ? { kind: 'none' }
            : { kind: 'observed', values: experimentalOutcome },
      })
    : err({ kind: 'invalid-request', detail: request.error.issues.map((i) => i.message).join(' ') })
}

export function surrogateDiagnosticsFor(
  samples: SurrogateSamples,
  selection: SurrogateSelection,
): readonly SurrogateDiagnosticRequest[] {
  const { experimental, observational, adjustment } = samples.request
  const common = { experimental, observational, adjustment }
  const requests: SurrogateDiagnosticRequest[] = []
  if (samples.validationOutcome.kind === 'observed')
    requests.push({
      ...common,
      analysis: { kind: 'validation', experimentalOutcome: samples.validationOutcome.values },
    })
  if (selection.checks.biasBounds.kind !== 'none')
    requests.push({
      ...common,
      analysis: { kind: 'biasBounds', restriction: selection.checks.biasBounds },
    })
  return requests
}
