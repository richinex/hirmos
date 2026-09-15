import { z } from 'zod'

const finite = z.number().finite()
const seed = z.number().int().min(0).max(0xffffffff)
const interval = z.tuple([finite, finite]).refine(([lower, upper]) => lower <= upper, 'The lower bound cannot exceed the upper bound.')
const method = z.enum(['percentile', 'pivot', 'normal'])

export const tLearnerUncertaintySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('bootstrap'), samples: z.number().int().min(2).max(1000),
    seed, level: finite.gt(0).lt(1), method,
  }).strict(),
])
export type TLearnerUncertainty = z.infer<typeof tLearnerUncertaintySchema>

export const tLearnerUncertaintyEvidenceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('none') }).strict(),
  z.object({
    kind: z.literal('bootstrap'), samples: z.number().int().min(2).max(1000),
    seed, level: finite.gt(0).lt(1), method,
    intervals: z.array(interval).min(1), standardErrors: z.array(finite.nonnegative()).min(1),
    average: z.object({ interval, standardErrorBound: finite.nonnegative() }).strict(),
  }).strict(),
])

export function matchesTLearnerUncertainty(settings: TLearnerUncertainty, evidence: z.infer<typeof tLearnerUncertaintyEvidenceSchema>): boolean {
  if (settings.kind === 'none') return evidence.kind === 'none'
  return evidence.kind === 'bootstrap' && settings.samples === evidence.samples
    && settings.seed === evidence.seed && settings.level === evidence.level && settings.method === evidence.method
}
