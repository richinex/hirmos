import { z } from 'zod'

const node = z.number().int().nonnegative()
export const adjustmentValidationSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('valid') }).strict(),
  z.object({ kind: z.literal('endpoints'), nodes: z.array(node).min(1) }).strict(),
  z.object({ kind: z.literal('unobserved'), nodes: z.array(node).min(1) }).strict(),
  z.object({ kind: z.literal('forbiddenDescendants'), nodes: z.array(node).min(1) }).strict(),
  z.object({ kind: z.literal('openNoncausalPath') }).strict(),
])
export type AdjustmentValidation = z.infer<typeof adjustmentValidationSchema>
export const adjustmentValidationDesignSchema = z
  .object({
    nodes: z.number().int().min(2).max(256),
    edges: z.array(z.tuple([node, node])),
    treatment: node,
    outcome: node,
    unobserved: z.array(node),
    sets: z.array(z.array(node)).max(256),
  })
  .strict()
export type AdjustmentValidationDesign = z.infer<typeof adjustmentValidationDesignSchema>
