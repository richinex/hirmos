import { z } from 'zod'
import type { InterventionQueryId } from './intervention'
import type { DagDocumentId, DagRevisionId } from './dag'
import type { PreparedDatasetVersionId } from './preprocessing'

const assignment = z
  .object({ variable: z.number().int().nonnegative(), state: z.number().int().nonnegative() })
  .strict()
export const networkQuerySchema = z
  .object({
    rows: z.number().int().positive(),
    columns: z.number().int().positive(),
    names: z.array(z.string().trim().min(1)).min(1),
    edges: z.array(z.tuple([z.number().int().nonnegative(), z.number().int().nonnegative()])),
    outcomes: z.array(z.number().int().nonnegative()).min(1),
    interventions: z.array(assignment),
    observations: z.array(assignment),
    bins: z.number().int().min(2).max(10),
    equivalentSampleSize: z.number().finite().nonnegative(),
  })
  .strict()
  .superRefine((q, ctx) => {
    const roles = [
      ...q.outcomes,
      ...q.interventions.map((a) => a.variable),
      ...q.observations.map((a) => a.variable),
    ]
    if (
      q.names.length !== q.columns ||
      new Set(q.names).size !== q.columns ||
      new Set(roles).size !== roles.length ||
      roles.some((i) => i >= q.columns)
    )
      ctx.addIssue({
        code: 'custom',
        message: 'Choose distinct variables for outcomes, interventions and observations.',
      })
  })
export type NetworkQuery = z.infer<typeof networkQuerySchema>
export const networkQueryEvidenceSchema = z
  .object({
    kind: z.literal('networkQuery'),
    observations: z.number().int().positive(),
    states: z.array(z.array(z.tuple([z.string(), z.number().finite()])).min(1)).min(1),
    distribution: z
      .array(z.tuple([z.array(z.string()).min(1), z.number().finite().min(0).max(1)]))
      .min(1),
  })
  .strict()
export type NetworkQueryEvidence = z.infer<typeof networkQueryEvidenceSchema>
export const networkQueryArtifactSchema = z
  .object({
    kind: z.literal('network-query'),
    id: z.string().min(1),
    dagDocument: z.string().min(1),
    dagRevision: z.string().min(1),
    preparedDataset: z.string().min(1),
    createdAt: z.string().min(1),
    specification: networkQuerySchema,
    result: networkQueryEvidenceSchema,
  })
  .strict()
  .superRefine((a, ctx) => {
    if (
      a.result.states.length !== a.specification.columns ||
      a.result.observations !== a.specification.rows ||
      a.result.distribution.some(([s]) => s.length !== a.specification.outcomes.length) ||
      Math.abs(a.result.distribution.reduce((n, [, p]) => n + p, 0) - 1) > 1e-8
    )
      ctx.addIssue({ code: 'custom', message: 'The saved distribution does not match its query.' })
  })
export interface NetworkQueryArtifact {
  readonly kind: 'network-query'
  readonly id: InterventionQueryId
  readonly dagDocument: DagDocumentId
  readonly dagRevision: DagRevisionId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly specification: NetworkQuery
  readonly result: NetworkQueryEvidence
}
export function describeNetworkQuery(q: NetworkQuery): string {
  const values = (items: NetworkQuery['interventions']) =>
    items.map((a) => `${q.names[a.variable]} state ${a.state}`).join(', ')
  const conditions = [
    ...(q.interventions.length ? [`do(${values(q.interventions)})`] : []),
    ...(q.observations.length ? [values(q.observations)] : []),
  ]
  return `P(${q.outcomes.map((i) => q.names[i]).join(', ')}${conditions.length ? ` | ${conditions.join('; ')}` : ''})`
}
