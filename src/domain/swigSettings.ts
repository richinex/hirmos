import { z } from 'zod'
import { err, ok, type Result } from './dop'
import { prepareDidSwig } from './swigDidDraft'
import { didDesignSchema } from './swigDid'
import { swigSpecificationSchema, type SwigSpecification } from './swig'
import type { ProjectedSwigGraph } from './swigProjection'
import type { ConfigurationSetting } from './estimation'

const node = z.string().regex(/^(0|[1-9]\d*)$/)
const equation = z
  .object({
    outcome: node,
    terms: z
      .array(
        z
          .object({ key: z.string(), identity: z.string().trim().min(1), arguments: z.array(node) })
          .strict(),
      )
      .max(256),
  })
  .strict()
export const swigSettingsSchema = z.discriminatedUnion('purpose', [
  z
    .object({
      purpose: z.literal('graph'),
      interventions: z
        .array(
          z.object({ key: z.string(), variable: node, value: z.string().trim().min(1) }).strict(),
        )
        .min(1)
        .max(256),
      exogenous: z.array(node),
      construction: z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('swig') }).strict(),
        z.object({ kind: z.literal('difference'), earlier: equation, later: equation }).strict(),
      ]),
      additionalPairs: z
        .array(z.object({ key: z.string(), earlier: equation, later: equation }).strict())
        .max(31),
      rationale: z.string().trim().min(1),
    })
    .strict(),
  z
    .object({
      purpose: z.literal('did'),
      draft: z
        .object({
          roles: z.array(
            z.discriminatedUnion('kind', [
              z.object({ kind: z.enum(['unassigned', 'confounder', 'disturbance']) }).strict(),
              z
                .object({
                  kind: z.enum(['covariate', 'treatment', 'outcome']),
                  period: z.number().int().min(0).max(32),
                })
                .strict(),
            ]),
          ),
          adoption: z.number().int().min(1).max(32),
          outcome: didDesignSchema.shape.outcome,
          comparison: didDesignSchema.shape.comparison,
          selected: z.array(node),
          rationale: z.string(),
        })
        .strict(),
    })
    .strict(),
])
export type SwigSettings = z.infer<typeof swigSettingsSchema>

export function prepareSwigSettings(
  raw: unknown,
  graph: ProjectedSwigGraph,
): Result<SwigSpecification, string> {
  const parsed = swigSettingsSchema.safeParse(raw)
  if (!parsed.success)
    return err(
      'Choose the variables and periods, enter finite intervention values, and name each additive term. Record the assumptions for a graph construction.',
    )
  const settings = parsed.data
  const exists = (id: string) => Number(id) < graph.names.length
  if (settings.purpose === 'did') {
    if (settings.draft.selected.some((id) => !exists(id)))
      return err('Choose controls from the current graph.')
    return prepareDidSwig(graph, settings.draft.roles, {
      ...settings.draft,
      selected: settings.draft.selected.map(Number),
    })
  }
  if (
    settings.exogenous.some((id) => !exists(id)) ||
    settings.interventions.some(
      (row) => !exists(row.variable) || !Number.isFinite(Number(row.value)),
    )
  )
    return err(
      'Choose intervention and disturbance variables from the current graph and enter finite values.',
    )
  if (
    new Set(settings.interventions.map((row) => row.variable)).size !==
    settings.interventions.length
  )
    return err('Each variable can have only one intervention value.')
  let construction: SwigSpecification['construction'] = { kind: 'swig' }
  if (settings.construction.kind === 'difference') {
    const pairs = [settings.construction, ...settings.additionalPairs]
    if (
      pairs.some(
        (pair) =>
          pair.earlier.outcome === pair.later.outcome ||
          [pair.earlier, pair.later].some(
            (e) => !exists(e.outcome) || e.terms.some((t) => t.arguments.some((id) => !exists(id))),
          ),
      )
    )
      return err('Choose distinct outcomes and valid term arguments for each difference.')
    const convert = (e: z.infer<typeof equation>) => ({
      outcome: Number(e.outcome),
      terms: e.terms.map((t) => ({
        identity: t.identity,
        arguments: t.arguments.map(Number).sort((a, b) => a - b),
      })),
    })
    const converted = pairs.map((p) => ({ earlier: convert(p.earlier), later: convert(p.later) }))
    construction =
      converted.length === 1
        ? { kind: 'difference', ...converted[0]! }
        : { kind: 'differences', pairs: converted }
  }
  const specification = swigSpecificationSchema.safeParse({
    roles: graph.names.map((_, i) =>
      settings.exogenous.includes(String(i)) ? 'exogenous' : 'endogenous',
    ),
    edges: graph.edges,
    interventions: settings.interventions.map((row) => [Number(row.variable), Number(row.value)]),
    construction,
    query: { kind: 'graph' },
  })
  return specification.success
    ? ok(specification.data)
    : err('Review the intervention graph specification.')
}

export function swigSettingsDescription(
  value: SwigSettings,
  names: readonly string[],
): readonly ConfigurationSetting[] {
  const name = (id: string) => names[Number(id)] ?? 'Unknown variable'
  if (value.purpose === 'did')
    return [
      { key: 'purpose', label: 'Analysis', value: 'DiD adjustment' },
      ...value.draft.roles.map((r, i) => ({
        key: `role-${i}`,
        label: names[i] ?? 'Unknown variable',
        value:
          {
            unassigned: 'Not assigned',
            confounder: 'Common cause',
            disturbance: 'Exogenous disturbance',
            covariate: 'Covariate',
            treatment: 'Treatment',
            outcome: 'Outcome',
          }[r.kind] + ('period' in r ? `, period ${r.period}` : ''),
      })),
      { key: 'adoption', label: 'Adoption period', value: String(value.draft.adoption) },
      { key: 'outcome', label: 'Outcome period', value: String(value.draft.outcome) },
      {
        key: 'comparison',
        label: 'Comparison',
        value:
          value.draft.comparison.kind === 'neverTreated'
            ? 'Never treated'
            : `Not treated through period ${value.draft.comparison.through}`,
      },
      {
        key: 'controls',
        label: 'Controls',
        value: value.draft.selected.map(name).join(', ') || 'None',
      },
      { key: 'rationale', label: 'Assumptions', value: value.draft.rationale || 'Not recorded' },
    ]
  const pairs =
    value.construction.kind === 'difference' ? [value.construction, ...value.additionalPairs] : []
  return [
    {
      key: 'purpose',
      label: 'Construction',
      value: pairs.length ? 'Difference graph' : 'Intervention graph',
    },
    ...value.interventions.map((r, i) => ({
      key: `intervention-${i}`,
      label: 'Intervention',
      value: `${name(r.variable)} = ${r.value}`,
    })),
    {
      key: 'exogenous',
      label: 'Exogenous disturbances',
      value: value.exogenous.map(name).join(', ') || 'None',
    },
    ...pairs.flatMap((p, i) =>
      [p.earlier, p.later].map((e, j) => ({
        key: `equation-${i}-${j}`,
        label: name(e.outcome),
        value:
          e.terms.map((t) => `${t.identity}(${t.arguments.map(name).join(', ')})`).join(' + ') ||
          'No additive terms',
      })),
    ),
    { key: 'rationale', label: 'Assumptions', value: value.rationale },
  ]
}
