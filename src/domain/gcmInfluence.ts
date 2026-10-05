import { z } from 'zod'
import { rootCauseSelectionSchema } from './rootCause'
import { gcmRandomSchema, gcmRandomStateSchema } from './gcmRandom'
import { reachable } from './dagFlow'

const index = z.number().int().nonnegative()
const count = z.number().int().positive()
const finite = z.number().finite()
export const gcmInfluenceRequestSchema = z
  .object({
    names: z.array(z.string().min(1)).min(1),
    edges: z.array(z.tuple([index, index])),
    rows: count.min(5).max(200_000),
    target: index,
    random: gcmRandomSchema,
    query: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('intrinsic'),
          training: count.min(5).max(200_000),
          randomization: count,
          baseline: count,
        })
        .strict(),
      z
        .object({
          kind: z.literal('arrows'),
          conditional: count,
          maxRuns: count,
          tolerance: finite.nonnegative(),
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((request, context) => {
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    const n = request.names.length
    if (new Set(request.names).size !== n) fail('Graph variable names must be distinct.')
    if (request.target >= n || request.edges.some(([a, b]) => a >= n || b >= n || a === b))
      fail('Relationships and target must refer to graph variables.')
    if (
      request.query.kind === 'arrows' &&
      !request.edges.some(([, child]) => child === request.target)
    )
      fail('Choose a target with at least one parent.')
  })
export type GcmInfluenceRequest = z.infer<typeof gcmInfluenceRequestSchema>

const values = { nodes: z.array(index).min(1), values: z.array(finite).min(1) }
export const gcmInfluenceEvidenceSchema = z
  .object({
    outcome: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('intrinsic'), ...values }).strict(),
      z.object({ kind: z.literal('arrows'), ...values }).strict(),
    ]),
    mechanisms: z
      .array(
        z.discriminatedUnion('kind', [
          z.object({ kind: z.literal('empirical'), node: index }).strict(),
          z
            .object({
              kind: z.literal('additive'),
              node: index,
              predictor: z.enum(['linear', 'quadratic', 'boosted']),
              noise: z.enum(['continuous', 'discrete']),
            })
            .strict(),
        ]),
      )
      .min(1),
    random: gcmRandomStateSchema,
  })
  .strict()
  .superRefine(({ outcome, mechanisms }, context) => {
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    if (
      outcome.nodes.length !== outcome.values.length ||
      new Set(outcome.nodes).size !== outcome.nodes.length
    )
      fail('Each distinct variable needs one influence estimate.')
    if (new Set(mechanisms.map((model) => model.node)).size !== mechanisms.length)
      fail('Each variable needs one fitted model.')
  })
export type GcmInfluenceEvidence = z.infer<typeof gcmInfluenceEvidenceSchema>
export const gcmInfluenceResponseSchema = z
  .object({ kind: z.literal('gcmInfluence'), evidence: gcmInfluenceEvidenceSchema })
  .strict()
export const gcmInfluenceRunSchema = z
  .object({
    id: z.string().min(1),
    createdAt: z.string().datetime(),
    graph: rootCauseSelectionSchema,
    model: gcmInfluenceRequestSchema,
    evidence: gcmInfluenceEvidenceSchema,
  })
  .strict()
  .superRefine(({ model, evidence }, context) => {
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    if (model.query.kind !== evidence.outcome.kind)
      fail('The result must match the requested measure.')
    if (evidence.outcome.nodes.some((node) => node >= model.names.length))
      fail('The result refers to an unknown graph variable.')
    if (
      evidence.mechanisms.length !== model.names.length ||
      evidence.mechanisms.some((entry) => entry.node >= model.names.length)
    )
      fail('The fitted models must cover the graph variables.')
    if (model.query.kind === 'arrows') {
      const parents = new Set(
        model.edges.filter(([, child]) => child === model.target).map(([parent]) => parent),
      )
      if (
        parents.size !== evidence.outcome.nodes.length ||
        evidence.outcome.nodes.some((node) => !parents.has(node))
      )
        fail('Arrow strengths must refer to the target’s parents.')
    }
    if (model.query.kind === 'intrinsic') {
      const ancestors = reachable(
        model.edges.map(([cause, effect]) => ({ cause, effect })),
        model.target,
        'backward',
      )
      ancestors.add(model.target)
      if (
        ancestors.size !== evidence.outcome.nodes.length ||
        evidence.outcome.nodes.some((node) => !ancestors.has(node))
      )
        fail('Intrinsic contributions must cover the target and its ancestors.')
    }
  })
export type GcmInfluenceRun = z.infer<typeof gcmInfluenceRunSchema>
