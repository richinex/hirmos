import { z } from 'zod'
import { honestConfigurationSchema } from './honestDid'

export type HonestSettings = z.infer<typeof honestSettingsSchema>

export const honestSettingsSchema = z
  .object({
    sourceRun: z.string().min(1),
    configuration: honestConfigurationSchema,
    target: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('first') }).strict(),
      z.object({ kind: z.literal('average') }).strict(),
      z
        .object({ kind: z.literal('specified'), weights: z.array(z.number().finite()).min(1) })
        .strict(),
    ]),
  })
  .strict()
