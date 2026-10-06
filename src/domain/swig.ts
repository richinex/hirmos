import { z } from 'zod'
import {
  swigVariable as variable,
  swigEquationSchema as equation,
  swigDifferenceSchema,
} from './swigMechanisms'
import { didDesignSchema, didAssessmentSchema } from './swigDid'
import {
  projectSwigGraph,
  swigProjectionSchema,
  type SwigProjection,
  type SwigGraphProblem,
} from './swigProjection'
export type { SwigGraphProblem } from './swigProjection'
import { brand } from './dop'
import type { DagDocument } from './dag'

const derivedNode = z.number().int().nonnegative().max(543)

export const swigSpecificationSchema = z
  .object({
    roles: z
      .array(z.enum(['endogenous', 'exogenous']))
      .min(1)
      .max(256),
    edges: z.array(z.tuple([variable, variable])).max(65_536),
    interventions: z
      .array(z.tuple([variable, z.number().finite()]))
      .min(1)
      .max(256),
    construction: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('swig') }).strict(),
      z
        .object({
          kind: z.literal('differences'),
          pairs: z.array(swigDifferenceSchema).min(1).max(32),
        })
        .strict(),
      z.object({ kind: z.literal('did'), design: didDesignSchema }).strict(),
      z.object({ kind: z.literal('difference'), earlier: equation, later: equation }).strict(),
    ]),
    query: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('graph') }).strict(),
      z
        .object({
          kind: z.literal('separation'),
          left: derivedNode,
          right: derivedNode,
          given: z.array(derivedNode).max(544),
        })
        .strict(),
    ]),
  })
  .strict()

export type SwigSpecification = z.infer<typeof swigSpecificationSchema>

export const swigEvidenceSchema = z
  .object({
    nodes: z
      .array(
        z.discriminatedUnion('kind', [
          z
            .object({
              kind: z.literal('random'),
              original: variable,
              role: z.enum(['endogenous', 'exogenous']),
              interventions: z.array(variable).max(256),
            })
            .strict(),
          z
            .object({ kind: z.literal('fixed'), original: variable, value: z.number().finite() })
            .strict(),
          z
            .object({
              kind: z.literal('difference'),
              earlier: variable,
              later: variable,
              cancelled: z.array(z.string()),
            })
            .strict(),
        ]),
      )
      .min(1)
      .max(544),
    edges: z.array(z.tuple([derivedNode, derivedNode])),
    conclusion: z.discriminatedUnion('kind', [
      z.object({ kind: z.literal('notRequested') }).strict(),
      z.object({ kind: z.literal('did'), assessment: didAssessmentSchema }).strict(),
      z
        .object({
          kind: z.literal('separated'),
          left: derivedNode,
          right: derivedNode,
          given: z.array(derivedNode),
        })
        .strict(),
      z
        .object({
          kind: z.literal('connected'),
          left: derivedNode,
          right: derivedNode,
          given: z.array(derivedNode),
        })
        .strict(),
    ]),
  })
  .strict()

export type SwigEvidence = z.infer<typeof swigEvidenceSchema>

/** Names and evidence belong to the recorded revision, never the currently open DAG. */
export const swigAnalysisSchema = z
  .object({
    kind: z.literal('swig-analysis'),
    projection: swigProjectionSchema.default({ kind: 'explicit' }),
    id: z
      .string()
      .uuid()
      .transform((value) => brand<string, 'SwigAnalysisId'>(value)),
    dagDocument: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'DagDocumentId'>(value)),
    dagRevision: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'DagRevisionId'>(value)),
    preparedDataset: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'PreparedDatasetVersionId'>(value)),
    createdAt: z.string().datetime(),
    names: z.array(z.string().min(1)).min(1).max(256),
    rationale: z.string().trim(),
    specification: swigSpecificationSchema,
    result: swigEvidenceSchema,
  })
  .strict()
  .superRefine((record, context) => {
    if (record.specification.construction.kind !== 'did' && record.rationale === '')
      context.addIssue({
        code: 'custom',
        path: ['rationale'],
        message: 'Record the assumptions supporting this intervention graph.',
      })
    const n = record.names.length,
      { specification: spec, result } = record
    const source = (index: number) => index < n
    const derived = (index: number) => index < result.nodes.length
    let valid =
      n === spec.roles.length &&
      spec.edges.every((edge) => edge.every(source)) &&
      spec.interventions.every(([index]) => source(index))
    const construction = spec.construction
    if (construction.kind === 'did') {
      const d = construction.design
      valid &&=
        d.panelRoles.length === n &&
        d.measured.every(source) &&
        d.selected.every(source) &&
        d.alpha.arguments.every(source) &&
        d.panelRoles.every((r) => r.kind !== 'disturbance' || source(r.of))
      if (result.conclusion.kind === 'did') {
        const decision = result.conclusion.assessment.decision
        if (decision.kind === 'supportedSubjectToOverlap')
          valid &&= decision.required.every(source) && decision.available.every(source)
        if (decision.kind === 'notEstablished')
          valid &&=
            decision.potentialSet.every(source) &&
            [...decision.treatedBlockers, ...decision.comparisonBlockers].every(
              (b) => source(b.variable) && (b.kind === 'unmeasured' || source(b.treatment)),
            )
      }
    }
    const pairs = swigDifferences(construction)
    for (const pair of pairs)
      valid &&= [pair.earlier, pair.later].every(
        (equation) =>
          source(equation.outcome) && equation.terms.every((term) => term.arguments.every(source)),
      )
    valid &&=
      result.edges.every((edge) => edge.every(derived)) &&
      result.nodes.every((node) => {
        switch (node.kind) {
          case 'random':
            return (
              source(node.original) &&
              node.interventions.every((index) =>
                spec.interventions.some(([original]) => index === original),
              )
            )
          case 'fixed':
            return spec.interventions.some(
              ([original, value]) => node.original === original && node.value === value,
            )
          case 'difference':
            return pairs.some(
              (pair) => node.earlier === pair.earlier.outcome && node.later === pair.later.outcome,
            )
        }
      })
    valid &&= result.nodes.length === n + spec.interventions.length + pairs.length
    if (spec.query.kind === 'graph')
      valid &&= result.conclusion.kind === (construction.kind === 'did' ? 'did' : 'notRequested')
    else {
      const query = spec.query,
        conclusion = result.conclusion
      valid &&=
        [query.left, query.right, ...query.given].every(derived) &&
        conclusion.kind !== 'notRequested' &&
        conclusion.kind !== 'did' &&
        query.left === conclusion.left &&
        query.right === conclusion.right &&
        JSON.stringify(query.given) === JSON.stringify(conclusion.given)
    }
    if (!valid)
      context.addIssue({
        code: 'custom',
        message: 'The SWIG record does not match its source variables or query.',
      })
  })
export type SwigAnalysis = z.output<typeof swigAnalysisSchema>

export function swigNodeName(record: SwigAnalysis, index: number): string {
  const node = record.result.nodes[index]
  if (node === undefined) throw new Error('Unknown derived graph node')
  const name = (original: number) => {
    const value = record.names[original]
    if (value === undefined) throw new Error('Unknown source graph variable')
    return value
  }
  const randomName = (original: number): string => {
    const random = record.result.nodes.find(
      (candidate) => candidate.kind === 'random' && candidate.original === original,
    )
    if (random === undefined || random.kind !== 'random')
      throw new Error('Missing random outcome node')
    const settings = random.interventions.map((original) => {
      const intervention = record.specification.interventions.find(([id]) => id === original)
      if (intervention === undefined) throw new Error('Missing recorded intervention')
      return `${name(original)} = ${intervention[1]}`
    })
    return settings.length === 0 ? name(original) : `${name(original)} (${settings.join(', ')})`
  }
  switch (node.kind) {
    case 'fixed':
      return `${name(node.original)} = ${node.value}`
    case 'random':
      return randomName(node.original)
    case 'difference':
      return `${randomName(node.later)} − ${randomName(node.earlier)}`
  }
}

export function swigDifferences(
  construction: SwigSpecification['construction'],
): z.infer<typeof swigDifferenceSchema>[] {
  switch (construction.kind) {
    case 'swig':
      return []
    case 'difference':
      return [{ earlier: construction.earlier, later: construction.later }]
    case 'differences':
      return construction.pairs
    case 'did': {
      const equations = [...construction.design.untreated].sort((a, b) => {
        const left = construction.design.panelRoles[a.outcome],
          right = construction.design.panelRoles[b.outcome]
        return (
          (left?.kind === 'outcome' ? left.period : -1) -
          (right?.kind === 'outcome' ? right.period : -1)
        )
      })
      return equations.slice(1).map((later, i) => ({ earlier: equations[i]!, later }))
    }
  }
}
export function readySwigGraph(
  document: DagDocument,
  projection: SwigProjection = { kind: 'explicit' },
) {
  return projectSwigGraph(document, projection)
}

export function describeSwigGraphProblem(problem: SwigGraphProblem): string {
  switch (problem.kind) {
    case 'invalid-periods':
      return 'Choose an end period after the first period, with at most 33 periods in total.'
    case 'invalid-invariance':
      return 'A time-invariant variable cannot have a time-varying parent or a lagged incoming arrow.'
    case 'invalid-graph':
      return 'Resolve the structural issues in this DAG before constructing an intervention graph.'
    case 'time-expansion-required':
      return 'This graph uses lagged arrows. SWIG analysis requires an explicit node for each variable and period; lagged arrows cannot be treated as same-period arrows.'
    case 'too-many-variables':
      return 'SWIG analysis supports up to 256 explicit variables, including disturbances.'
  }
}

/** A constructed graph and the separation checks run on it, oldest first. */
export interface SwigGraphEntry {
  readonly graph: SwigAnalysis
  readonly checks: readonly SwigAnalysis[]
}

/** Records share a graph when revision, data, projection and specification match, apart from the query. */
function swigGraphKey(record: SwigAnalysis): string {
  const { query: _query, ...graph } = record.specification
  return JSON.stringify([
    record.dagRevision,
    record.preparedDataset,
    record.projection,
    record.names,
    graph,
  ])
}

/** Groups records by graph, in the order each graph was first saved. */
export function groupSwigAnalyses(records: readonly SwigAnalysis[]): SwigGraphEntry[] {
  const groups = new Map<string, SwigAnalysis[]>()
  for (const record of records) {
    const key = swigGraphKey(record)
    const group = groups.get(key)
    if (group === undefined) groups.set(key, [record])
    else group.push(record)
  }
  return [...groups.values()].map((group) => ({
    graph: group.find((record) => record.specification.query.kind === 'graph') ?? group[0]!,
    checks: group.filter((record) => record.specification.query.kind === 'separation'),
  }))
}

/** States a separation check's conclusion, naming nodes as the derived graph does. */
export function describeSeparation(record: SwigAnalysis): string {
  const conclusion = record.result.conclusion
  if (conclusion.kind !== 'separated' && conclusion.kind !== 'connected')
    throw new Error('The record is not a separation check')
  const relation = conclusion.kind === 'separated' ? 'd-separated' : 'd-connected'
  const condition =
    conclusion.given.length === 0
      ? 'without conditioning'
      : `given ${conclusion.given.map((index) => swigNodeName(record, index)).join(', ')}`
  return `${swigNodeName(record, conclusion.left)} and ${swigNodeName(record, conclusion.right)} are ${relation} ${condition}.`
}

/** A separation check in the notation of the DAG graph checks: X ⊥ Y | Z. */
export function separationNotation(record: SwigAnalysis): string {
  const conclusion = record.result.conclusion
  if (conclusion.kind !== 'separated' && conclusion.kind !== 'connected')
    throw new Error('The record is not a separation check')
  const name = (index: number) => swigNodeName(record, index)
  const given = conclusion.given.length === 0 ? '' : ` | ${conclusion.given.map(name).join(', ')}`
  return `${name(conclusion.left)} ⊥ ${name(conclusion.right)}${given}`
}

/** Identifies a separation query regardless of the order of its two nodes or its conditioning set. */
export function separationKey(query: SwigSpecification['query']): string | null {
  if (query.kind !== 'separation') return null
  return JSON.stringify([
    Math.min(query.left, query.right),
    Math.max(query.left, query.right),
    [...query.given].sort((a, b) => a - b),
  ])
}

/** Checks newest first, keeping only the latest of any repeated query. */
export function distinctSeparationChecks(checks: readonly SwigAnalysis[]): SwigAnalysis[] {
  const seen = new Set<string>()
  return [...checks].reverse().filter((check) => {
    const key = separationKey(check.specification.query)
    if (key === null || seen.has(key)) return false
    seen.add(key)
    return true
  })
}

const didVerdicts = {
  baselineIdentity: 'baseline period',
  supportedSubjectToOverlap: 'supported subject to overlap',
  notEstablished: 'not established',
} as const

/** The name a saved graph goes by wherever it is listed: its construction or verdict, and its checks. */
export function swigGraphTitle(entry: SwigGraphEntry): string {
  const conclusion = entry.graph.result.conclusion
  const kind =
    conclusion.kind === 'did'
      ? `DiD adjustment, ${didVerdicts[conclusion.assessment.decision.kind]}`
      : entry.graph.specification.construction.kind === 'swig'
        ? 'SWIG'
        : 'Δ-SWIG'
  const count = distinctSeparationChecks(entry.checks).length
  return count === 0 ? kind : `${kind}, ${count} separation ${count === 1 ? 'check' : 'checks'}`
}
