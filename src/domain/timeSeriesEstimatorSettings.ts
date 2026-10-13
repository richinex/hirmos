import { z } from 'zod'
import { ingarchInterventionScheduleSchema } from './estimation'

export const PSS_CASES: Readonly<Record<'c' | 'ct', readonly (2 | 3 | 4 | 5)[]>> = {
  c: [2, 3],
  ct: [4, 5],
}
export const seriesLagSchema = z.number().int().min(1).max(24)
export const ingarchHorizonSchema = z.number().int().min(1).max(240)
const lags = z
  .tuple([seriesLagSchema], seriesLagSchema)
  .refine(
    (values) => values.every((value, i) => i === 0 || value > values[i - 1]!),
    'Use distinct lags in increasing order.',
  )

export const timeSeriesEstimatorSettingsSchema = z.union([
  z
    .object({
      kind: z.literal('ardl-pss'),
      maxLag: seriesLagSchema,
      trend: z.enum(['c', 'ct']),
      case: z.union([z.literal(2), z.literal(3), z.literal(4), z.literal(5)]),
    })
    .strict()
    .refine(
      (value) => PSS_CASES[value.trend].includes(value.case),
      'The PSS case must match the deterministic terms.',
    ),
  z
    .object({
      kind: z.literal('vecm'),
      maxLags: seriesLagSchema,
      deterministic: z.enum(['n', 'co', 'ci', 'coli']),
      significance: z.union([z.literal(90), z.literal(95), z.literal(99)]),
      breakIndex: z.number().int().min(4).nullable(),
    })
    .strict(),
  z
    .object({
      kind: z.literal('negative-binomial-ingarch'),
      link: z.enum(['identity', 'log']),
      pastObservationLags: lags,
      pastMeanLags: lags,
      horizon: ingarchHorizonSchema,
      controlValue: z.number().finite(),
      treatmentValue: z.number().finite(),
      schedule: ingarchInterventionScheduleSchema,
    })
    .strict()
    .refine(
      (value) =>
        value.link !== 'identity' || (value.controlValue >= 0 && value.treatmentValue >= 0),
      'The additive link requires non-negative intervention values.',
    ),
])
