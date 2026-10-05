import { z } from 'zod'
import { surrogateRequestSchema } from './surrogate'

const finite = z.number().finite()
const count = z.number().int().safe().positive()
export const surrogateBiasRestrictionSchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('binaryWithoutSurrogacy') }).strict(),
    z.object({ kind: z.literal('binaryWithoutComparability') }).strict(),
    z.object({ kind: z.literal('boundedDirectEffect'), maximum: finite.nonnegative() }).strict(),
    z
      .object({ kind: z.literal('boundedSampleDifference'), maximum: finite.nonnegative() })
      .strict(),
  ])
  .readonly()
export type SurrogateBiasRestriction = z.infer<typeof surrogateBiasRestrictionSchema>
export const surrogateChecksSchema = z
  .object({
    validation: z.enum(['none', 'observedOutcome']),
    biasBounds: z.union([
      z.object({ kind: z.literal('none') }).strict(),
      surrogateBiasRestrictionSchema,
    ]),
  })
  .strict()
  .readonly()
export const noSurrogateChecks = { validation: 'none', biasBounds: { kind: 'none' } } as const

const samples = surrogateRequestSchema.unwrap().shape
export const surrogateDiagnosticRequestSchema = z
  .object({
    experimental: samples.experimental,
    observational: samples.observational,
    adjustment: samples.adjustment,
    analysis: z
      .discriminatedUnion('kind', [
        z
          .object({
            kind: z.literal('validation'),
            experimentalOutcome: z.array(finite).min(1).readonly(),
          })
          .strict(),
        z
          .object({ kind: z.literal('biasBounds'), restriction: surrogateBiasRestrictionSchema })
          .strict(),
      ])
      .readonly(),
  })
  .strict()
  .superRefine((r, ctx) => {
    const sampleCheck = surrogateRequestSchema.safeParse({
      experimental: r.experimental,
      observational: r.observational,
      adjustment: r.adjustment,
      estimator: 'index',
      uncertainty: { kind: 'none' },
    })
    if (!sampleCheck.success)
      for (const issue of sampleCheck.error.issues)
        ctx.addIssue({ code: 'custom', message: issue.message })
    if (
      r.analysis.kind === 'validation' &&
      r.analysis.experimentalOutcome.length !== r.experimental.treatment.length
    )
      ctx.addIssue({
        code: 'custom',
        message:
          'Validation requires the observed long-term outcome for every experimental observation.',
      })
  })
  .readonly()
export type SurrogateDiagnosticRequest = z.infer<typeof surrogateDiagnosticRequestSchema>
// Untagged historical coefficients retain their reported inference; they are not reclassified without data.
const coefficient = z
  .union([
    z
      .object({
        kind: z.literal('estimated').default('estimated'),
        estimate: finite,
        standardError: finite.positive(),
        tStatistic: finite,
        residualDegreesOfFreedom: count,
      })
      .strict(),
    z
      .object({ kind: z.literal('perfectFit'), estimate: finite, residualDegreesOfFreedom: count })
      .strict(),
  ])
  .readonly()
export const surrogateDiagnosticEvidenceSchema = z
  .object({
    experimentalRows: count,
    observationalRows: count,
    surrogateColumns: count,
    baselineColumns: z.number().int().safe().nonnegative(),
    analysis: z
      .discriminatedUnion('kind', [
        z
          .object({
            kind: z.literal('validation'),
            surrogacy: coefficient,
            comparability: coefficient,
          })
          .strict(),
        z
          .object({
            kind: z.literal('biasBounds'),
            restriction: surrogateBiasRestrictionSchema,
            lower: finite,
            upper: finite,
          })
          .strict(),
      ])
      .readonly(),
  })
  .strict()
  .superRefine((e, ctx) => {
    if (e.analysis.kind === 'biasBounds' && e.analysis.lower > e.analysis.upper)
      ctx.addIssue({ code: 'custom', message: 'Bias bounds must be ordered.' })
    if (e.analysis.kind === 'validation') {
      const predictors = 2 + e.surrogateColumns + e.baselineColumns
      if (
        e.analysis.surrogacy.residualDegreesOfFreedom !== e.experimentalRows - predictors ||
        e.analysis.comparability.residualDegreesOfFreedom !==
          e.experimentalRows + e.observationalRows - predictors
      )
        ctx.addIssue({
          code: 'custom',
          message: 'Validation degrees of freedom must match the selected regression designs.',
        })
    }
  })
  .readonly()
export type SurrogateDiagnosticEvidence = z.infer<typeof surrogateDiagnosticEvidenceSchema>

export function sameSurrogateRestriction(
  a: SurrogateBiasRestriction,
  b: SurrogateBiasRestriction,
): boolean {
  if (a.kind !== b.kind) return false
  return !('maximum' in a) || ('maximum' in b && a.maximum === b.maximum)
}
export function surrogateDiagnosticMatches(
  e: SurrogateDiagnosticEvidence,
  r: SurrogateDiagnosticRequest,
): boolean {
  if (
    e.experimentalRows !== r.experimental.treatment.length ||
    e.observationalRows !== r.observational.outcome.length ||
    e.surrogateColumns !== r.experimental.surrogates[0]?.length ||
    e.baselineColumns !== (r.adjustment.kind === 'none' ? 0 : r.adjustment.experimental[0]?.length)
  )
    return false
  return r.analysis.kind === 'validation'
    ? e.analysis.kind === 'validation'
    : e.analysis.kind === 'biasBounds' &&
        sameSurrogateRestriction(e.analysis.restriction, r.analysis.restriction)
}
