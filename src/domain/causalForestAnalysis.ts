import { z } from 'zod'

const column = z.string().min(1)
const finite = z.number().finite()
const weighting = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('uniform') }).strict(),
  z.object({ kind: z.literal('column'), column }).strict(),
])
export const forestAnalysisSchema = z.object({
  sampling: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('independent'), weighting }).strict(),
    z.object({ kind: z.literal('clustered'), column, equalize: z.literal(false), weighting }).strict(),
    z.object({ kind: z.literal('equal-clusters'), column }).strict(),
  ]),
  averageMethod: z.enum(['aipw', 'tmle']),
  projection: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('linear'), columns: z.array(column).min(1), overlap: z.boolean(), covariance: z.enum(['hc0', 'hc1', 'hc2', 'hc3']) }).strict(),
  ]),
  ranking: z.discriminatedUnion('kind', [
    z.object({ kind: z.literal('none') }).strict(),
    z.object({ kind: z.literal('external'), columns: z.array(column).min(1).max(2), rationale: z.string().trim().min(1),
      target: z.enum(['autoc', 'qini']), quantiles: z.array(finite.gt(0).max(1)).min(1),
      replications: z.number().int().min(2), seed: z.number().int().min(0).max(0xffffffff),
    }).strict(),
  ]),
  labels: z.array(z.object({ column, name: z.string().min(1) }).strict()).optional(),
  moderation: z.object({ between: z.array(column), within: z.array(z.object({ column, threshold: finite }).strict()) }).strict().optional(),
}).strict().superRefine((value, context) => {
  if (value.moderation !== undefined && (value.sampling.kind !== 'equal-clusters' || value.moderation.between.length + value.moderation.within.length === 0))
    context.addIssue({ code: 'custom', message: 'Cluster-score comparisons require equal cluster weights and at least one selected comparison.' })
  if (value.moderation !== undefined && (new Set(value.moderation.between).size !== value.moderation.between.length || new Set(value.moderation.within.map(v => v.column)).size !== value.moderation.within.length))
    context.addIssue({ code: 'custom', message: 'Select distinct moderation columns within each comparison type.' })
  if (value.averageMethod === 'tmle' && (value.sampling.kind !== 'independent' || value.sampling.weighting.kind !== 'uniform'))
    context.addIssue({ code: 'custom', message: 'TMLE requires independent, unweighted observations.' })
  for (const analysis of [value.projection, value.ranking]) if ('columns' in analysis && new Set(analysis.columns).size !== analysis.columns.length)
    context.addIssue({ code: 'custom', message: 'Select distinct analysis columns.' })
  if (value.ranking.kind === 'external' && (value.ranking.quantiles.at(-1) !== 1 || value.ranking.quantiles.some((q, i, all) => i > 0 && q <= all[i - 1]!)))
    context.addIssue({ code: 'custom', message: 'RATE fractions must increase and end at 1.' })
})
export type ForestAnalysis = z.infer<typeof forestAnalysisSchema>
export const DEFAULT_FOREST_ANALYSIS: ForestAnalysis = {
  sampling: { kind: 'independent', weighting: { kind: 'uniform' } }, averageMethod: 'aipw',
  projection: { kind: 'none' }, ranking: { kind: 'none' },
}
export function forestAnalysisColumns(spec: ForestAnalysis): readonly string[] {
  const sample = spec.sampling
  return [...new Set([
    ...(sample.kind === 'independent' ? [] : [sample.column]),
    ...('weighting' in sample && sample.weighting.kind === 'column' ? [sample.weighting.column] : []),
    ...(spec.projection.kind === 'linear' ? spec.projection.columns : []),
    ...(spec.ranking.kind === 'external' ? spec.ranking.columns : []),
    ...(spec.moderation?.between ?? []), ...(spec.moderation?.within.map(v => v.column) ?? []),
  ])]
}
const reading = <T extends z.ZodType>(schema: T) => z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('not-requested') }).strict(),
  z.object({ kind: z.literal('unavailable'), reason: z.string().min(1) }).strict(),
  z.object({ kind: z.literal('estimated'), result: schema }).strict(),
])
const linear = z.object({ estimates: z.array(finite), standardErrors: z.array(finite.nonnegative()), statistics: z.array(finite), pValues: z.array(finite.min(0).max(1)), degreesOfFreedom: z.number().int().positive() }).strict()
const rule = z.object({ estimate: finite, standardError: finite.nonnegative(), toc: z.array(finite), tocStandardError: z.array(finite.nonnegative()) }).strict()
const moderationTest = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('contrast'), estimate: finite, standardError: finite.positive(), statistic: finite, degreesOfFreedom: finite.positive(), pValue: finite.min(0).max(1), lower: finite, upper: finite }).strict(),
  z.object({ kind: z.literal('omnibus'), statistic: finite.nonnegative(), degreesOfFreedom: finite.positive(), residualDegreesOfFreedom: finite.positive(), pValue: finite.min(0).max(1) }).strict(),
])
export const forestAnalysisEvidenceSchema = z.object({
  specification: forestAnalysisSchema,
  projection: reading(linear),
  ranking: reading(z.object({ rules: z.array(rule), difference: rule.nullable() }).strict()),
  moderation: z.array(z.object({ column, method: z.string().min(1), result: reading(moderationTest) }).strict()).optional(),
}).strict().superRefine((value, context) => {
  const { specification: spec, projection, ranking } = value
  const fail = (message: string) => context.addIssue({ code: 'custom', message })
  if ((spec.moderation === undefined) !== (value.moderation === undefined)) fail('Moderation evidence does not match its specification.')
  if (spec.moderation !== undefined && value.moderation !== undefined) {
    const expected = [...spec.moderation.between.flatMap(column => [column, column]), ...spec.moderation.within.map(v => v.column)]
    if (JSON.stringify(value.moderation.map(v => v.column)) !== JSON.stringify(expected)) fail('Moderation results do not match the selected comparisons.')
  }
  if ((spec.projection.kind === 'none') !== (projection.kind === 'not-requested')) fail('Projection evidence does not match its specification.')
  if ((spec.ranking.kind === 'none') !== (ranking.kind === 'not-requested')) fail('Ranking evidence does not match its specification.')
  if (projection.kind === 'estimated' && spec.projection.kind === 'linear') {
    const r = projection.result, n = spec.projection.columns.length + 1
    if ([r.estimates, r.standardErrors, r.statistics, r.pValues].some(a => a.length !== n)) fail('Projection terms do not match the selected covariates.')
  }
  if (ranking.kind === 'estimated' && spec.ranking.kind === 'external') {
    if (ranking.result.rules.length !== spec.ranking.columns.length || (ranking.result.difference !== null) !== (spec.ranking.columns.length === 2)) fail('Ranking results do not match the selected rules.')
    const rules = [...ranking.result.rules, ...(ranking.result.difference === null ? [] : [ranking.result.difference])]
    const count = spec.ranking.quantiles.length
    if (rules.some(r => r.toc.length !== count || r.tocStandardError.length !== count)) fail('The targeting curves do not match the requested fractions.')
  }
})
