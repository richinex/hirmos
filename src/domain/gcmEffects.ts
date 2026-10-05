import { z } from 'zod'
import { rootCauseSelectionSchema } from './rootCause'

const index = z.number().int().nonnegative()
const count = z.number().int().positive()
const finite = z.number().finite()
const seed = z.number().int().min(0).max(0xffffffff)
const groupScope = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('all') }).strict(),
  z.object({ kind: z.literal('lowerBound'), minimum: finite }).strict(),
])
export const gcmMechanismSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('empirical') }).strict(),
  z.object({ kind: z.literal('regression') }).strict(),
  z.object({ kind: z.literal('classifier'), classes: count.min(2).max(7) }).strict(),
])
export type GcmMechanism = z.infer<typeof gcmMechanismSchema>
export const gcmEffectsRequestSchema = z
  .object({
    names: z.array(z.string().min(1)).min(2).max(64),
    edges: z.array(z.tuple([index, index])),
    rows: count.min(2),
    treatment: index,
    outcome: index,
    mechanisms: z.array(gcmMechanismSchema).min(2),
    trees: count.max(1000),
    minLeaf: count,
    fitSeed: seed,
    simulationSeed: seed,
    repetitions: count.max(1000),
    upperQuantile: finite.gt(0.5).lt(1),
    grouping: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z
        .object({
          kind: z.literal('quantiles'),
          column: index,
          groups: count.min(2).max(10),
          scope: groupScope,
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((value, context) => {
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    const n = value.names.length
    if (new Set(value.names).size !== n || value.mechanisms.length !== n)
      fail('Each graph variable needs a distinct name and one model.')
    if (value.treatment >= n || value.outcome >= n || value.treatment === value.outcome)
      fail('Choose different treatment and outcome variables.')
    if (value.edges.some(([a, b]) => a >= n || b >= n || a === b))
      fail('Relationships must refer to different graph variables.')
    value.mechanisms.forEach((model, node) => {
      const hasParents = value.edges.some(([, child]) => child === node)
      if ((model.kind === 'empirical') === hasParents)
        fail(
          'Variables without parents use their observed distribution; other variables require an outcome model.',
        )
    })
    if (value.grouping.kind === 'quantiles') {
      const column = value.grouping.column
      if (
        column >= n ||
        column === value.treatment ||
        column === value.outcome ||
        value.edges.some(([, child]) => child === column)
      )
        fail('Choose a grouping variable without parents, distinct from treatment and outcome.')
    }
  })
export type GcmEffectsRequest = z.infer<typeof gcmEffectsRequestSchema>
export const gcmEffectsEvidenceSchema = z
  .object({
    estimates: z.array(finite).min(1),
    bounds: z.array(z.tuple([finite, finite])).min(1),
    quantiles: z.tuple([finite.gt(0).lt(0.5), finite.gt(0.5).lt(1)]),
    replicates: z.array(z.array(finite).min(1)).min(1),
    optimizer: z
      .object({
        status: z.enum(['converged', 'iterationLimit', 'precisionLoss']),
        iterations: index,
      })
      .strict(),
    grouping: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('none') }).strict(),
      z
        .object({
          kind: z.literal('quantiles'),
          column: index,
          cutpoints: z.array(finite).min(1),
          counts: z.array(count).min(2),
          scope: groupScope,
          excluded: index,
        })
        .strict(),
    ]),
  })
  .strict()
  .superRefine((value, context) => {
    const width = value.estimates.length
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    if (value.bounds.length !== width || value.replicates.some((row) => row.length !== width))
      fail('Every estimate needs matching bounds and simulation values.')
    if (value.bounds.some(([low, high]) => low > high)) fail('Bounds must be ordered.')
    if (Math.abs(value.quantiles[0] + value.quantiles[1] - 1) > 1e-12)
      fail('Percentiles must be complementary.')
    if (value.grouping.kind === 'none' && width !== 1)
      fail('An ungrouped analysis reports one average.')
    if (value.grouping.kind === 'quantiles') {
      const { counts, cutpoints } = value.grouping
      if (counts.length + 1 !== width || cutpoints.length + 1 !== counts.length)
        fail('Group boundaries and estimates must agree.')
      if (cutpoints.some((point, i) => i > 0 && point <= cutpoints[i - 1]!))
        fail('Group boundaries must increase.')
    }
  })
export type GcmEffectsEvidence = z.infer<typeof gcmEffectsEvidenceSchema>
export const gcmEffectsResponseSchema = z
  .object({ kind: z.literal('gcmEffects'), evidence: gcmEffectsEvidenceSchema })
  .strict()
export const gcmEffectsRunSchema = z
  .object({
    id: z.string().min(1),
    createdAt: z.string().datetime(),
    graph: rootCauseSelectionSchema,
    model: gcmEffectsRequestSchema,
    evidence: gcmEffectsEvidenceSchema,
  })
  .strict()
  .superRefine(({ model, evidence }, context) => {
    const fail = (message: string) => context.addIssue({ code: 'custom', message })
    if (
      model.repetitions !== evidence.replicates.length ||
      model.upperQuantile !== evidence.quantiles[1]
    )
      fail('The result must match the requested simulations and percentiles.')
    if (model.grouping.kind !== evidence.grouping.kind)
      fail('The grouping does not match the request.')
    if (
      model.grouping.kind === 'quantiles' &&
      evidence.grouping.kind === 'quantiles' &&
      (model.grouping.column !== evidence.grouping.column ||
        model.grouping.groups !== evidence.grouping.counts.length ||
        evidence.grouping.counts.reduce((sum, n) => sum + n, evidence.grouping.excluded) !==
          model.rows ||
        JSON.stringify(model.grouping.scope) !== JSON.stringify(evidence.grouping.scope))
    )
      fail('The groups must match the requested column, scope and population.')
  })
export type GcmEffectsRun = z.infer<typeof gcmEffectsRunSchema>
