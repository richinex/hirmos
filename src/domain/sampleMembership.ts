import { z } from 'zod'
import { brand } from './dop'

const column = z
  .string()
  .min(1)
  .transform((v) => brand<string, 'ColumnId'>(v))
const values = z
  .tuple([z.string().min(1)])
  .rest(z.string().min(1))
  .readonly()
/** Explicit, disjoint category sets. Missing membership is not an unselected category. */
export const categoricalMembershipSchema = z
  .object({
    kind: z.literal('categories'),
    column,
    experimental: values,
    observational: values,
  })
  .strict()
  .superRefine((s, ctx) => {
    const all = [...s.experimental, ...s.observational]
    if (new Set(all).size !== all.length)
      ctx.addIssue({
        code: 'custom',
        message: 'Select distinct category values, with no value in both samples.',
      })
  })
  .readonly()
export type CategoricalMembership = z.infer<typeof categoricalMembershipSchema>
// Legacy numeric records retain their values and acquire an explicit tag.
export const sampleMembershipSchema = z
  .union([
    z
      .object({
        kind: z.literal('numeric').default('numeric'),
        column,
        experimental: z.number().finite(),
        observational: z.number().finite(),
      })
      .strict()
      .refine(
        (s) => s.experimental !== s.observational,
        'Experimental and observational samples must have different membership values.',
      ),
    categoricalMembershipSchema,
  ])
  .readonly()
