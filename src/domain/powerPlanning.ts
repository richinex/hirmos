import { z } from 'zod'
const finite = z.number().finite()
const probability = finite.gt(0).lt(1)
const size = finite.min(2).max(1e7)
export const powerDesignSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('independentT'), ratio: finite.positive() }).strict(),
  z.object({ kind: z.literal('independentNormal'), ratio: finite.positive() }).strict(),
  z.object({ kind: z.literal('oneSampleT') }).strict(),
  z.object({ kind: z.literal('pairedT') }).strict(),
])
export const powerTargetSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('power'), n: size, effect: finite }).strict(),
  z.object({ kind: z.literal('minimumEffect'), n: size, target: probability }).strict(),
  z.object({ kind: z.literal('sampleSize'), effect: finite, target: probability }).strict(),
])
export const powerScenarioSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('independent'), treated: size.int(), controls: size.int() }).strict(),
  z
    .object({
      kind: z.literal('equalClusters'),
      treated: size.int(),
      controls: size.int(),
      members: finite.int().min(1),
      icc: finite.min(0).max(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('twoPeriodDid'),
      treated: size.int(),
      controls: size.int(),
      correlation: finite.min(-1).max(1),
      violation: finite,
    })
    .strict(),
])
export const powerRequestSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('analytical'),
      design: powerDesignSchema,
      alternative: z.enum(['twoSided', 'larger', 'smaller']),
      alpha: probability,
      target: powerTargetSchema,
    })
    .strict(),
  z
    .object({
      kind: z.literal('simulation'),
      scenario: powerScenarioSchema,
      effect: finite,
      sd: finite.positive(),
      replications: finite.int().min(1).max(10000),
      seed: finite.int().min(0).max(4294967295),
      alpha: probability,
    })
    .strict(),
])
export const powerEvidenceSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('analytical'),
      effect: finite,
      n: size,
      secondGroup: size.nullable(),
      power: finite.min(0).max(1),
      continuousN: size.nullable(),
      curve: z.array(z.object({ n: size, power: finite.min(0).max(1) }).strict()).length(41),
    })
    .strict(),
  z
    .object({
      kind: z.literal('simulation'),
      replications: finite.int().positive(),
      rejected: finite.int().nonnegative(),
      rejectionRate: finite.min(0).max(1),
      mcse: finite.nonnegative(),
      coverage: finite.min(0).max(1),
      bias: finite,
      rmse: finite.nonnegative(),
    })
    .strict(),
])
export const powerRecordSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('analytical'),
      specification: powerRequestSchema.options[0],
      result: powerEvidenceSchema.options[0],
    })
    .strict(),
  z
    .object({
      kind: z.literal('simulation'),
      specification: powerRequestSchema.options[1],
      result: powerEvidenceSchema.options[1],
    })
    .strict(),
])
export type PowerRequest = z.infer<typeof powerRequestSchema>
export type PowerEvidence = z.infer<typeof powerEvidenceSchema>
export type PowerRecord = z.infer<typeof powerRecordSchema>
export function powerRecord(
  specification: PowerRequest,
  result: PowerEvidence,
): PowerRecord | null {
  const parsed = powerRecordSchema.safeParse({ kind: specification.kind, specification, result })
  return parsed.success ? parsed.data : null
}
export const DEFAULT_POWER: PowerRequest = {
  kind: 'analytical',
  design: { kind: 'independentT', ratio: 1 },
  alternative: 'twoSided',
  alpha: 0.05,
  target: { kind: 'minimumEffect', n: 100, target: 0.8 },
}
