import { z } from 'zod'

const uint32 = z.number().int().min(0).max(0xffffffff)
export const gcmRandomStateSchema = z
  .object({
    keys: z.array(uint32).length(624),
    position: z.number().int().min(0).max(624),
    normal: z.number().finite().nullable(),
  })
  .strict()
export const gcmRandomSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('seed'), seed: uint32 }).strict(),
  z.object({ kind: z.literal('resume'), state: gcmRandomStateSchema }).strict(),
])
