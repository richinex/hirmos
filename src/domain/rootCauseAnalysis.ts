import { z } from 'zod'
import { rootCauseSelectionSchema } from './rootCause'

const index = z.number().int().nonnegative()
const count = z.number().int().positive()
const finite = z.number().finite()
const uint32 = z.number().int().min(0).max(0xffffffff)
const randomState = z.object({ keys: z.array(uint32).length(624), position: index.max(624), normal: finite.nullable() }).strict()
const random = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('seed'), seed: uint32 }).strict(),
  z.object({ kind: z.literal('resume'), state: randomState }).strict(),
])
const execution = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('independentJobs') }).strict(),
  z.object({ kind: z.literal('recordedBatches'), repetitions: z.array(z.array(z.array(z.array(z.boolean()).min(1)).min(1)).min(1)).min(1) }).strict(),
])
const query = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('anomaly'), samples: count }).strict(),
  z.object({ kind: z.literal('change'), rows: count, samples: count, execution }).strict(),
  z.object({ kind: z.literal('intervention'), rows: count, order: z.array(index).min(1), shifts: z.array(z.object({ node: index, amount: finite }).strict()).min(1) }).strict(),
])

export const rootCauseRequestSchema = z.object({
  names: z.array(z.string().min(1)).min(1), edges: z.array(z.tuple([index, index])),
  rows: count, target: index, repetitions: count, upperQuantile: finite.min(0.5).max(1),
  fraction: finite.gt(0).max(1), random, query,
}).strict().superRefine((request, context) => {
  const fail = (message: string) => context.addIssue({ code: 'custom', message })
  const columns = request.names.length
  if (new Set(request.names).size !== columns) fail('Graph variable names must be distinct.')
  if (request.target >= columns || request.edges.some(([a, b]) => a >= columns || b >= columns)) fail('Graph positions must refer to named variables.')
  if (Math.floor(request.rows * request.fraction) === 0) fail('The fitting fraction leaves no observations.')
  switch (request.query.kind) {
    case 'anomaly': break
    case 'change':
      if (Math.floor(request.query.rows * request.fraction) === 0) fail('The comparison fitting fraction leaves no observations.')
      if (request.query.execution.kind === 'recordedBatches' && request.query.execution.repetitions.length !== request.repetitions) fail('Supply one batch plan per repetition.')
      break
    case 'intervention':
      if (request.query.order.length !== columns || new Set(request.query.order).size !== columns || request.query.order.some((node) => node >= columns)) fail('The summary order must contain every graph variable once.')
      if (request.query.shifts.some((shift) => shift.node >= columns)) fail('Each shift must refer to a graph variable.')
      if (new Set(request.query.shifts.map((shift) => shift.node)).size !== request.query.shifts.length) fail('Specify each shifted variable once.')
      break
  }
})
export type RootCauseRequest = z.infer<typeof rootCauseRequestSchema>

const summary = z.object({
  estimates: z.array(finite).min(1), bounds: z.array(z.tuple([finite, finite])).min(1),
  quantiles: z.tuple([finite.min(0).max(0.5), finite.min(0.5).max(1)]),
  replicates: z.array(z.array(finite).min(1)).min(1), optimizerStatus: z.union([z.literal(0), z.literal(1), z.literal(2)]),
}).strict().superRefine((value, context) => {
  const width = value.estimates.length
  if (value.bounds.length !== width || value.replicates.some((row) => row.length !== width)) context.addIssue({ code: 'custom', message: 'Estimates, bounds and replicate columns must agree.' })
  if (value.bounds.some(([lower, upper]) => lower > upper)) context.addIssue({ code: 'custom', message: 'The lower bound cannot exceed the upper bound.' })
  if (Math.abs(value.quantiles[0] + value.quantiles[1] - 1) > 1e-12) context.addIssue({ code: 'custom', message: 'The reported percentiles must be complementary.' })
})

export const rootCauseEvidenceSchema = z.object({
  outcome: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('anomaly'), nodes: z.array(index).min(1), summary }).strict(),
    z.object({ kind: z.literal('change'), nodes: z.array(index).min(1), summary }).strict(),
    z.object({ kind: z.literal('intervention'), target: index, nodes: z.array(index).min(1), observedMean: finite, summary }).strict(),
  ]),
  random: randomState,
}).strict().superRefine(({ outcome }, context) => {
  const width = outcome.nodes.length
  if (outcome.summary.estimates.length !== width) context.addIssue({ code: 'custom', message: 'The estimates must match the reported variables.' })
  if (new Set(outcome.nodes).size !== width) context.addIssue({ code: 'custom', message: 'Reported variables must be distinct.' })
  if (outcome.kind === 'intervention' && !outcome.nodes.includes(outcome.target)) context.addIssue({ code: 'custom', message: 'The intervention summary must include its target.' })
})
export const rootCauseResponseSchema = z.object({ kind: z.literal('rootCause'), evidence: rootCauseEvidenceSchema }).strict()
export type RootCauseEvidence = z.infer<typeof rootCauseEvidenceSchema>

export const rootCauseRunSchema = z.object({
  id: z.string().min(1), createdAt: z.string().datetime(), graph: rootCauseSelectionSchema,
  model: rootCauseRequestSchema, evidence: rootCauseEvidenceSchema,
  comparison: z.object({ name: z.string().min(1), fingerprint: z.string().min(1), rows: count }).strict(),
  observation: z.array(z.number().finite()).optional(),
}).strict().superRefine((run, context) => {
  if (run.observation !== undefined && (run.model.query.kind !== 'anomaly' || run.observation.length !== run.model.names.length)) context.addIssue({ code: 'custom', message: 'Entered observations must contain one value per graph variable for an unusual-observation analysis.' })
  if (run.model.query.kind !== run.evidence.outcome.kind) context.addIssue({ code: 'custom', message: 'The result does not match the requested analysis.' })
  const outcome = run.evidence.outcome
  if (outcome.kind === 'intervention' && outcome.target !== run.model.target) context.addIssue({ code: 'custom', message: 'The result target differs from the requested target.' })
  const expectedRows = run.model.query.kind === 'anomaly' ? 1 : run.model.query.rows
  if (run.comparison.rows !== expectedRows) context.addIssue({ code: 'custom', message: 'The recorded comparison row count differs from the request.' })
  if (outcome.summary.quantiles[1] !== run.model.upperQuantile) context.addIssue({ code: 'custom', message: 'The result percentiles differ from the requested percentiles.' })
  const nodes = outcome.nodes
  if (run.model.query.kind === 'intervention' && JSON.stringify(nodes) !== JSON.stringify(run.model.query.order)) context.addIssue({ code: 'custom', message: 'Intervention summaries must retain the requested variable order.' })
  if (nodes.some((node) => node >= run.model.names.length)) context.addIssue({ code: 'custom', message: 'The result refers to an unknown variable.' })
  if (run.evidence.outcome.summary.replicates.length !== run.model.repetitions) context.addIssue({ code: 'custom', message: 'The result has a different repetition count.' })
})
export type RootCauseRun = z.infer<typeof rootCauseRunSchema>
export const rootCauseCheckRequestSchema = z.object({ names: z.array(z.string().min(1)).min(1), edges: z.array(z.tuple([index, index])), rows: count.min(5), seed: uint32, scope: z.enum(['full', 'fitted']).optional() }).strict()
const probability = finite.min(0).max(1)
const graphCheck = z.object({
  order: z.array(index).min(1),
  implications: z.array(z.object({ x: index, y: index, given: z.array(index), p: probability }).strict()),
  lmcViolations: index, tpaViolations: index,
}).strict().superRefine((value, context) => {
  if (value.lmcViolations > value.implications.length || value.tpaViolations > value.implications.length) context.addIssue({ code: 'custom', message: 'Violation counts cannot exceed the number of tested implications.' })
})
const fullRootCauseChecksSchema = z.object({
  mechanisms: z.array(z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('root'), node: index, klDivergence: finite }).strict(),
    z.object({ kind: z.literal('conditional'), node: index, crps: finite, mse: finite.nonnegative(), nmse: finite.nonnegative(), r2: finite }).strict(),
  ])).min(1),
  invertibility: z.array(z.object({ node: index, pValue: probability, rejected: z.boolean() }).strict()),
  overallDivergence: finite, given: graphCheck, permutations: z.array(graphCheck).min(1),
  pValueLmc: probability, pValueTpa: probability,
  verdict: z.enum(['noImplications', 'notInformative', 'retained', 'rejected']), random: randomState,
}).strict()
export type FullRootCauseChecks = z.infer<typeof fullRootCauseChecksSchema>
const fittedModelChecksSchema = fullRootCauseChecksSchema.pick({ mechanisms: true, invertibility: true, overallDivergence: true, random: true }).extend({ scope: z.literal('fitted') }).strict()
export const rootCauseChecksSchema = z.union([fullRootCauseChecksSchema, fittedModelChecksSchema])
export const rootCauseChecksResponseSchema = z.object({ kind: z.literal('rootCauseChecks'), evidence: rootCauseChecksSchema }).strict()
export const rootCauseCheckRecordSchema = z.object({
  id: z.string().min(1), createdAt: z.string().datetime(), graph: rootCauseSelectionSchema,
  model: rootCauseCheckRequestSchema, evidence: rootCauseChecksSchema,
}).strict().superRefine((value, context) => {
  if ((value.model.scope === 'fitted') !== ('scope' in value.evidence)) context.addIssue({ code: 'custom', message: 'The model-check result must match the requested scope.' })
  const nodes = value.evidence.mechanisms.map((entry) => entry.node)
  if (nodes.length !== value.model.names.length || new Set(nodes).size !== nodes.length || nodes.some((node) => node >= nodes.length)) context.addIssue({ code: 'custom', message: 'Model checks must report each graph variable once.' })
})
export type RootCauseCheckRequest = z.infer<typeof rootCauseCheckRequestSchema>
export type RootCauseChecks = z.infer<typeof rootCauseChecksSchema>
export type RootCauseCheckRecord = z.infer<typeof rootCauseCheckRecordSchema>
export const rootCauseWorkspaceSchema = z.object({ selection: rootCauseSelectionSchema.nullable(), runs: z.array(rootCauseRunSchema), checks: z.array(rootCauseCheckRecordSchema).default([]) }).strict()
export type RootCauseWorkspace = z.infer<typeof rootCauseWorkspaceSchema>
export const EMPTY_ROOT_CAUSE: RootCauseWorkspace = { selection: null, runs: [], checks: [] }
