import { z } from 'zod'

const series = z.array(z.number().finite()).min(1)

export const ardlLongRunSchema = z
  .object({
    kind: z.literal('recorded'),
    observed: series,
    departures: series,
  })
  .strict()
  .superRefine((value, ctx) => {
    if (value.observed.length !== value.departures.length)
      ctx.addIssue({
        code: 'custom',
        message: 'The observed series and long-run departures must cover the same rows.',
      })
    if (value.observed.some((observed, row) => !Number.isFinite(observed - value.departures[row]!)))
      ctx.addIssue({ code: 'custom', message: 'The estimated long-run level must be finite.' })
  })

export const vecmLongRunSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notFitted') }).strict(),
  z
    .object({
      kind: z.literal('recorded'),
      startRow: z.number().int().nonnegative(),
      departures: z.array(series).min(1),
    })
    .strict(),
])

export const plotTimeSchema = z
  .object({
    kind: z.enum(['calendar', 'ordinal']),
    values: series,
  })
  .strict()
  .superRefine((axis, ctx) => {
    if (axis.values.some((value, i) => i > 0 && value <= axis.values[i - 1]!))
      ctx.addIssue({
        code: 'custom',
        message: 'Chart dates must be distinct and in increasing order.',
      })
  })
export type PlotTime = z.infer<typeof plotTimeSchema>
