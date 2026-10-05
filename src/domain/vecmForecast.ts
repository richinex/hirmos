import { z } from 'zod'
const row = z.array(z.number().finite()).min(2)
export const vecmForecastSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRequested') }).strict(),
  z.object({ kind: z.literal('notFitted') }).strict(),
  z
    .object({
      kind: z.literal('recorded'),
      confidence: z.number().gt(0).lt(1),
      mean: z.array(row).min(1).max(200),
      lower: z.array(row),
      upper: z.array(row),
      covariance: z.array(z.array(row)),
    })
    .strict()
    .superRefine((v, ctx) => {
      const n = v.mean[0]!.length,
        h = v.mean.length
      if (
        v.lower.length !== h ||
        v.upper.length !== h ||
        v.covariance.length !== h ||
        [...v.mean, ...v.lower, ...v.upper].some((r) => r.length !== n) ||
        v.covariance.some((m) => m.length !== n || m.some((r) => r.length !== n))
      ) {
        ctx.addIssue({
          code: 'custom',
          message:
            'VECM forecast dimensions must agree across periods, series and covariance matrices.',
        })
        return
      }
      for (let t = 0; t < h; t++)
        for (let j = 0; j < n; j++) {
          if (v.lower[t]![j]! > v.mean[t]![j]! || v.upper[t]![j]! < v.mean[t]![j]!)
            ctx.addIssue({
              code: 'custom',
              message: 'Forecast intervals must contain their estimates.',
            })
          if (v.covariance[t]![j]![j]! < 0)
            ctx.addIssue({ code: 'custom', message: 'Forecast variances cannot be negative.' })
        }
    }),
])
