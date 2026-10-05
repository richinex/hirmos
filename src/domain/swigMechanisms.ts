import { z } from 'zod'
export const swigVariable = z.number().int().nonnegative().max(255)
export const swigTermSchema = z
  .object({
    identity: z.string().trim().min(1).max(256),
    arguments: z.array(swigVariable).max(256),
  })
  .strict()
export const swigEquationSchema = z
  .object({ outcome: swigVariable, terms: z.array(swigTermSchema).max(256) })
  .strict()
export const swigDifferenceSchema = z
  .object({ earlier: swigEquationSchema, later: swigEquationSchema })
  .strict()
