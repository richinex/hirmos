import { z } from 'zod'
import {
  causalForestConfigurationSchema,
  causalForestEvidenceSchema,
  causalForestTargetSchema,
} from './causalForest'

const finite = z.number().finite()
const index = z.number().int().nonnegative()
const options = z
  .object({
    trees: index.positive(),
    group_size: index.positive(),
    sample_fraction: finite.gt(0).lt(1),
    seed: index.max(0xffffffff),
    seed_mode: z.enum(['Indexed', 'Legacy']),
    batches: index.positive(),
    tree: z
      .object({
        mtry: index.positive(),
        min_node_size: index.positive(),
        honesty: z.union([
          z.literal('Disabled'),
          z
            .object({
              Enabled: z.object({ fraction: finite.gt(0).lt(1), prune: z.boolean() }).strict(),
            })
            .strict(),
        ]),
        alpha: finite,
        imbalance_penalty: finite,
      })
      .strict(),
  })
  .strict()
export const forestTuningBatchSchema = z
  .object({
    stage: index,
    offset: index,
    columns: z.array(z.array(finite)).min(1),
    outcome: index,
    treatment: index.nullable(),
    weight: index.nullable(),
    labels: z.array(index).min(2),
    perCluster: index.positive(),
    stabilize: z.boolean(),
    options: z.array(options).min(1),
  })
  .strict()
export type ForestTuningBatch = z.infer<typeof forestTuningBatchSchema>
export const forestTuningRequestSchema = z
  .object({
    rows: index.positive(),
    columns: index.positive(),
    treatment: index,
    outcome: index,
    adjustment: z.array(index).min(1),
    target: causalForestTargetSchema,
    configuration: causalForestConfigurationSchema,
    columnNames: z.array(z.string()),
    scores: z.array(z.array(finite.nullable())),
  })
  .strict()
export const forestTuningCommandSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('plan'),
      id: index,
      values: z.instanceof(Float64Array),
      request: forestTuningRequestSchema,
    })
    .strict(),
  z.object({ kind: z.literal('prepare'), id: index, batch: forestTuningBatchSchema }).strict(),
  z.object({ kind: z.literal('score'), id: index, index }).strict(),
])
export type ForestTuningCommand = z.infer<typeof forestTuningCommandSchema>
export const forestTuningReplySchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('pending'), batch: forestTuningBatchSchema }).strict(),
  z.object({ kind: z.literal('complete'), evidence: causalForestEvidenceSchema }).strict(),
])
export const forestTuningEventSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('planned'), id: index, result: forestTuningReplySchema }).strict(),
  z.object({ kind: z.literal('prepared'), id: index }).strict(),
  z.object({ kind: z.literal('scored'), id: index, score: finite.nullable() }).strict(),
  z.object({ kind: z.literal('failed'), id: index, detail: z.string() }).strict(),
])
export type ForestTuningEvent = z.infer<typeof forestTuningEventSchema>
