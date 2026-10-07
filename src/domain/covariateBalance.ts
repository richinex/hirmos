import { z } from 'zod'

export const covariateBalanceSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('notRecorded') }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string() }).strict(),
  z
    .object({
      kind: z.literal('available'),
      reference: z.enum(['pooled', 'treated']),
      labels: z.array(z.string()).default([]),
      rows: z.array(
        z
          .object({
            column: z.number().int().nonnegative(),
            before: z.number().finite(),
            after: z.number().finite(),
            denominator: z.number().finite().nonnegative(),
            usedFullSampleSpread: z.boolean(),
          })
          .strict(),
      ),
    })
    .strict(),
])
export type CovariateBalance = z.infer<typeof covariateBalanceSchema>

export const rawBalanceSchema = z
  .object({
    kind: z.literal('rawBalance'),
    rows: z.array(
      z
        .object({
          column: z.number().int().nonnegative(),
          difference: z.number().finite(),
          denominator: z.number().finite().nonnegative(),
          usedFullSampleSpread: z.boolean(),
        })
        .strict(),
    ),
  })
  .strict()
export type RawBalance = z.infer<typeof rawBalanceSchema>
