import { z } from 'zod'
import { assertNever } from './dop'

export const structuralSeasonalitySchema = z
  .discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z
      .object({
        kind: z.literal('seasonal'),
        seasons: z.number().int().min(2),
        duration: z.number().int().positive(),
      })
      .strict(),
    z
      .object({
        kind: z.literal('harmonic'),
        period: z.number().finite().positive(),
        pairs: z.number().int().positive(),
      })
      .strict(),
  ])
  .refine(
    (value) => value.kind !== 'harmonic' || 2 * value.pairs < value.period,
    'Harmonic pairs must be below half the period.',
  )

export const structuralModelSchema = z
  .object({
    version: z.literal('gaussian-components-v1'),
    trend: z.enum(['level', 'linear', 'semilocal']),
    seasonality: structuralSeasonalitySchema,
  })
  .strict()
export type StructuralModel = z.infer<typeof structuralModelSchema>
export const structuralImpactSettingsSchema = z
  .object({
    kind: z.literal('structural'),
    model: structuralModelSchema,
    draws: z.number().int().min(2),
    warmup: z.number().int().nonnegative(),
    seed: z.number().int().min(0).max(0xffffffff),
  })
  .strict()
export type StructuralImpactSettings = z.infer<typeof structuralImpactSettingsSchema>

export function sameStructuralModel(a: StructuralModel, b: StructuralModel): boolean {
  if (a.version !== b.version || a.trend !== b.trend) return false
  const x = a.seasonality
  const y = b.seasonality
  switch (x.kind) {
    case 'none':
      return y.kind === 'none'
    case 'seasonal':
      return y.kind === 'seasonal' && x.seasons === y.seasons && x.duration === y.duration
    case 'harmonic':
      return y.kind === 'harmonic' && x.period === y.period && x.pairs === y.pairs
    default:
      return assertNever(x)
  }
}

export const structuralContributionSchema = z
  .object({
    component: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('trend') }).strict(),
      z.object({ kind: z.literal('seasonal') }).strict(),
      z.object({ kind: z.literal('predictor'), column: z.number().int().nonnegative() }).strict(),
    ]),
    mean: z.array(z.number().finite()),
    lower: z.array(z.number().finite()),
    upper: z.array(z.number().finite()),
  })
  .strict()
