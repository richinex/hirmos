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

export const adjustmentRecommendationSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('available'),
      nodes: z.array(node),
      guarantee: z.enum(['established', 'notEstablished']),
    })
    .strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string() }).strict(),
])
export type AdjustmentRecommendation = z.infer<typeof adjustmentRecommendationSchema>
export const adjustmentGraphSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('identified'),
      canonicalSet: z.array(node),
      emptyValid: z.boolean(),
      recommendation: adjustmentRecommendationSchema,
    })
    .strict(),
  z.object({ kind: z.literal('notIdentified') }).strict(),
])
export const adjustmentResponseSchema = z
  .object({ checks: z.array(adjustmentValidationSchema), analysis: adjustmentGraphSchema })
  .strict()
export type AdjustmentResponse = z.infer<typeof adjustmentResponseSchema>
