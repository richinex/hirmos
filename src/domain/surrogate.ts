import { z } from 'zod'

const finite = z.number().finite()
const count = z.number().int().safe().positive()
const seed = z.number().int().min(0).max(0xffffffff)
const repetitions = count.min(2)
const matrix = z.array(z.array(finite).min(1).readonly()).min(1)
  .refine(rows => rows.every(row => row.length === rows[0]!.length), 'Each sample must have the same columns on every row.').readonly()
const estimator = z.enum(['index', 'score', 'influenceFunction'])

export const surrogateRequestSchema = z.object({
  experimental: z.object({ surrogates: matrix, treatment: z.array(z.union([z.literal(0), z.literal(1)])).min(2).readonly() }).strict().readonly(),
  observational: z.object({ surrogates: matrix, outcome: z.array(finite).min(1).readonly() }).strict().readonly(),
  adjustment: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('baseline'), experimental: matrix, observational: matrix }).strict(),
  ]).readonly(),
  estimator,
  uncertainty: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('bootstrap'), repetitions, seed }).strict(),
  ]).readonly(),
}).strict().superRefine((r, ctx) => {
  const issue = (message: string) => ctx.addIssue({ code: 'custom', message })
  if (r.experimental.surrogates.length !== r.experimental.treatment.length || r.observational.surrogates.length !== r.observational.outcome.length)
    issue('Each sample must supply one response per predictor row.')
  if (!r.experimental.treatment.includes(0) || !r.experimental.treatment.includes(1))
    issue('The experimental sample must contain treated and control observations.')
  if (r.experimental.surrogates[0]?.length !== r.observational.surrogates[0]?.length)
    issue('Both samples must supply the same surrogate columns in the same order.')
  if (r.adjustment.kind === 'baseline' && (r.adjustment.experimental.length !== r.experimental.treatment.length ||
      r.adjustment.observational.length !== r.observational.outcome.length ||
      r.adjustment.experimental[0]?.length !== r.adjustment.observational[0]?.length))
    issue('Baseline covariates must match both sample sizes and have the same columns in each sample.')
}).readonly()
export type SurrogateRequest = z.infer<typeof surrogateRequestSchema>

export const surrogateEvidenceSchema = z.object({
  estimator, experimentalRows: count, observationalRows: count, surrogateColumns: count,
  baselineColumns: z.number().int().safe().nonnegative(), estimate: finite,
  uncertainty: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('bootstrapStandardError'), standardError: finite.nonnegative(), repetitions, seed }).strict(),
  ]).readonly(),
}).strict().readonly()
export type SurrogateEvidence = z.infer<typeof surrogateEvidenceSchema>

export function surrogateEvidenceMatchesRequest(e: SurrogateEvidence, r: SurrogateRequest): boolean {
  if (e.estimator !== r.estimator || e.experimentalRows !== r.experimental.treatment.length ||
      e.observationalRows !== r.observational.outcome.length || e.surrogateColumns !== r.experimental.surrogates[0]?.length ||
      e.baselineColumns !== (r.adjustment.kind === 'none' ? 0 : r.adjustment.experimental[0]?.length)) return false
  switch (r.uncertainty.kind) {
    case 'none': return e.uncertainty.kind === 'none'
    case 'bootstrap': return e.uncertainty.kind === 'bootstrapStandardError' &&
      e.uncertainty.repetitions === r.uncertainty.repetitions && e.uncertainty.seed === r.uncertainty.seed
  }
}
