import { err, ok, type Result } from './dop'
import type { ProjectedSwigGraph } from './swigProjection'
import type { SwigDidDesign, SwigPanelRole } from './swigDid'
import { swigSpecificationSchema, type SwigSpecification } from './swig'
/** A disturbance's variable is its single child in the graph, so the draft does not record it. */
export type SwigRoleDraft =
  | { readonly kind: 'unassigned' }
  | { readonly kind: 'confounder' }
  | { readonly kind: 'disturbance' }
  | { readonly kind: 'covariate' | 'treatment' | 'outcome'; readonly period: number }
export type SwigRoleKind = Exclude<SwigRoleDraft['kind'], 'unassigned'>

/** The period a node starts with: its expansion period, or the earliest period its role allows. */
export function defaultSwigPeriod(
  graph: ProjectedSwigGraph,
  index: number,
  kind: 'covariate' | 'treatment' | 'outcome',
): number {
  const period = graph.origins[index]?.period ?? null
  const earliest = kind === 'treatment' ? 1 : 0
  return period === null ? earliest : Math.max(earliest, period)
}

/** Gives `kind` to exactly the nodes in `chosen`; nodes that leave the role become unassigned. */
export function assignSwigRole(
  graph: ProjectedSwigGraph,
  roles: readonly SwigRoleDraft[],
  kind: SwigRoleKind,
  chosen: readonly number[],
): readonly SwigRoleDraft[] {
  return roles.map((role, index) => {
    if (!chosen.includes(index)) return role.kind === kind ? { kind: 'unassigned' } : role
    if (role.kind === kind) return role
    return kind === 'confounder' || kind === 'disturbance'
      ? { kind }
      : { kind, period: defaultSwigPeriod(graph, index, kind) }
  })
}
/**
 * The rule holds only under these conditions, which no graph can show. The form states them, and running
 * the assessment records them as assumed with the analysis.
 */
const STATED_ASSUMPTIONS: SwigDidDesign['assumptions'] = {
  absorbingBinaryTreatment: true,
  consistency: true,
  independentDisturbances: true,
}

/** The untreated outcomes are declared as alpha + g_t. No common mechanism is inferred from arrows alone. */
export function prepareDidSwig(
  graph: ProjectedSwigGraph,
  roles: readonly SwigRoleDraft[],
  options: {
    readonly adoption: number
    readonly outcome: number
    readonly comparison: SwigDidDesign['comparison']
    readonly selected: readonly number[]
  },
): Result<SwigSpecification, string> {
  if (roles.length !== graph.names.length) return err('Assign a DiD role to every node.')
  const panelRoles: SwigPanelRole[] = []
  for (const [index, role] of roles.entries()) {
    if (role.kind === 'unassigned') return err('Assign a DiD role to every node.')
    if (role.kind !== 'disturbance') {
      panelRoles.push(role)
      continue
    }
    const children = graph.edges.filter(([cause]) => cause === index)
    if (children.length !== 1)
      return err(
        `The disturbance ${graph.names[index]} must have exactly one arrow, into the variable it disturbs.`,
      )
    panelRoles.push({ kind: 'disturbance', of: children[0]![1] })
  }
  const outcomes = panelRoles
    .flatMap((r, i) => (r.kind === 'outcome' ? [{ index: i, period: r.period }] : []))
    .sort((a, b) => a.period - b.period)
  const treatments = panelRoles.flatMap((r, i) => (r.kind === 'treatment' ? [i] : []))
  if (outcomes.length < 2 || outcomes.length > 33)
    return err('Choose between two and 33 outcome periods, beginning at period 0.')
  const confounders = panelRoles.flatMap((r, i) => (r.kind === 'confounder' ? [i] : []))
  const commonCovariates = panelRoles.flatMap((r, i) =>
    r.kind === 'covariate' &&
    outcomes.every((y) => graph.edges.some(([a, b]) => a === i && b === y.index))
      ? [i]
      : [],
  )
  const alpha = {
    identity: 'alpha',
    arguments: [...confounders, ...commonCovariates].sort((a, b) => a - b),
  }
  const untreated = outcomes.map((y) => ({
    outcome: y.index,
    terms: [
      alpha,
      {
        identity: `g_${y.period}`,
        arguments: graph.edges
          .filter(([a, b]) => b === y.index && !treatments.includes(a) && !confounders.includes(a))
          .map(([a]) => a)
          .sort((a, b) => a - b),
      },
    ],
  }))
  const specification = {
    roles: panelRoles.map((r) => (r.kind === 'disturbance' ? 'exogenous' : 'endogenous')),
    edges: graph.edges,
    interventions: treatments.map((i) => [i, 0]),
    query: { kind: 'graph' },
    construction: {
      kind: 'did',
      design: {
        panelRoles,
        untreated,
        alpha,
        assumptions: STATED_ASSUMPTIONS,
        adoption: options.adoption,
        outcome: options.outcome,
        comparison: options.comparison,
        measured: [...graph.measured],
        selected: [...options.selected],
      },
    },
  }
  const parsed = swigSpecificationSchema.safeParse(specification)
  return parsed.success
    ? ok(parsed.data)
    : err('Choose valid whole-number periods and a treatment node for each period after baseline.')
}

export interface SwigDidDraft {
  readonly roles: readonly SwigRoleDraft[]
  readonly adoption: number
  readonly outcome: number
  readonly comparison: SwigDidDesign['comparison']
  readonly selected: readonly string[]
  readonly rationale: string
}
export function initialSwigDidDraft(count: number): SwigDidDraft {
  return {
    roles: Array.from({ length: count }, () => ({ kind: 'unassigned' })),
    adoption: 1,
    outcome: 1,
    comparison: { kind: 'neverTreated' },
    selected: [],
    rationale: '',
  }
}
