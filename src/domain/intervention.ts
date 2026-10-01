import { z } from 'zod'
import type { NetworkQueryArtifact } from './networkQuery'
import type { ConditionalGaussianArtifact } from './conditionalGaussianQuery'
import { assertNever, brand, err, ok, type Brand, type Result } from './dop'
import type { DagDocument, DagDocumentId, DagNode, DagNodeId, DagRevisionId } from './dag'
import { discreteStatePreparationSchema, type DiscreteBnEvidence } from './estimation'
import type { PreparedDatasetVersionId } from './preprocessing'
import { formatStatistic } from '@/lib/format/number'

/**
 * The do-operator as a first-class question on the graph: set one node, read another. The answer
 * comes from the discrete Bayesian network fitted to the DAG (graph surgery cuts the arrows into
 * the set node; adjusting for its parents computes the same expectation when every parent is
 * measured). A level-2 query: what the read node would be if the set node were set, not what it
 * is when the set node is observed. Recorded against the graph revision it was asked of.
 */

export type InterventionQueryId = Brand<string, 'InterventionQueryId'>
export const newInterventionQueryId = (): InterventionQueryId => brand<string, 'InterventionQueryId'>(crypto.randomUUID())

export interface InterventionTarget {
  readonly node: DagNodeId
  readonly name: string
}

const distributionSchema = z.array(z.tuple([z.string(), z.number().finite()]))

const identifiedDiscreteResultSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('identified'),
    algorithm: z.enum(['ID', 'IDC']),
    expression: z.string().min(1),
    latex: z.string().min(1),
    expectations: z.tuple([z.number().finite(), z.number().finite()]),
    effect: z.number().finite(),
    distributionLow: distributionSchema,
    distributionHigh: distributionSchema,
    normalizationLow: z.number().finite(),
    normalizationHigh: z.number().finite(),
  }).strict(),
  z.object({
    kind: z.literal('unidentifiable'),
    hedgeGraph: z.array(z.number().int().nonnegative()),
    hedgeSubgraph: z.array(z.number().int().nonnegative()),
  }).strict(),
])

const identifiedDiscreteQueryKindSchema = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('unconditional') }).strict(),
  z.object({
    kind: z.literal('conditional'),
    variable: z.number().int().nonnegative(),
    state: z.string(),
    representativeValue: z.number().finite(),
  }).strict(),
])

export const identifiedDiscreteQueryEvidenceSchema = z.object({
  kind: z.literal('identifiedDiscreteQuery'),
  observations: z.number().int().positive(),
  bins: z.number().int().min(2).max(10),
  stateCounts: z.array(z.number().int().positive()).min(2),
  statePreparations: z.array(discreteStatePreparationSchema).min(2),
  treatmentStates: z.tuple([z.string(), z.string()]),
  query: identifiedDiscreteQueryKindSchema,
  result: identifiedDiscreteResultSchema,
}).strict()

export type IdentifiedDiscreteQueryEvidence = z.infer<typeof identifiedDiscreteQueryEvidenceSchema>

export type InterventionQueryRoute =
  | {
      readonly kind: 'bayesian-network'
      readonly bins: number
      readonly equivalentSampleSize: number
      readonly result: DiscreteBnEvidence
    }
  | {
      readonly kind: 'identified-expression'
      readonly result: IdentifiedDiscreteQueryEvidence
    }

export type InterventionQueryArtifact = ContrastQueryArtifact | NetworkQueryArtifact | ConditionalGaussianArtifact
export interface ContrastQueryArtifact {
  readonly kind: 'intervention-query'
  readonly id: InterventionQueryId
  readonly dagDocument: DagDocumentId
  readonly dagRevision: DagRevisionId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly set: InterventionTarget
  readonly read: InterventionTarget
  readonly route: InterventionQueryRoute
}

export type InterventionReadinessProblem =
  | { readonly kind: 'set-required' }
  | { readonly kind: 'read-required' }
  | { readonly kind: 'distinct-required' }
  | { readonly kind: 'latent-node'; readonly name: string }
  | { readonly kind: 'graph-invalid' }

export interface InterventionSpecification {
  readonly set: DagNode & { readonly kind: 'observed' }
  readonly read: DagNode & { readonly kind: 'observed' }
}

/** Both nodes chosen, distinct and observed, and the revision structurally valid. */
export function readyInterventionQuery(document: DagDocument, set: DagNodeId | null, read: DagNodeId | null): Result<InterventionSpecification, InterventionReadinessProblem> {
  if (document.current.validation.structure.kind === 'invalid') return err({ kind: 'graph-invalid' })
  const nodes = document.current.graph.nodes
  if (set === null) return err({ kind: 'set-required' })
  if (read === null) return err({ kind: 'read-required' })
  if (set === read) return err({ kind: 'distinct-required' })
  const setNode = nodes.find((node) => node.id === set)
  const readNode = nodes.find((node) => node.id === read)
  if (setNode === undefined) return err({ kind: 'set-required' })
  if (readNode === undefined) return err({ kind: 'read-required' })
  if (setNode.kind === 'latent') return err({ kind: 'latent-node', name: setNode.name })
  if (readNode.kind === 'latent') return err({ kind: 'latent-node', name: readNode.name })
  return ok({ set: setNode, read: readNode })
}

export function describeInterventionReadiness(problem: InterventionReadinessProblem): string {
  switch (problem.kind) {
    case 'set-required': return 'Choose the variable to set.'
    case 'read-required': return 'Choose the variable to read.'
    case 'distinct-required': return 'Set one variable and read a different one.'
    case 'latent-node': return `${problem.name} is unmeasured; only measured variables can be set or read.`
    case 'graph-invalid': return 'The graph has structural issues; resolve them before asking an intervention question.'
    default: { const exhaustive: never = problem; return exhaustive }
  }
}

/** Figures through the formatter the tiles use, so the sentence and the tiles show the same digits. */
const figure = (value: number): string => formatStatistic('raw', value).text

/** The answer in one sentence: both expectations, their difference, and what the surgery adjusted for. */
export function describeInterventionVerdict(artifact: ContrastQueryArtifact): string {
  switch (artifact.route.kind) {
    case 'bayesian-network': {
      const { result } = artifact.route
      const adjusted = result.parentsAdjusted.length === 0
        ? `${artifact.set.name} has no parents in the graph, so no adjustment was needed`
        : `adjusted for ${result.parentsAdjusted.join(', ')}, the parents of ${artifact.set.name}`
      return `Setting ${artifact.set.name} to its lowest bin gives an expected ${artifact.read.name} of ${figure(result.expectations[0])}; setting it to its highest gives ${figure(result.expectations[1])}, a difference of ${figure(result.effect)} (${adjusted}).`
    }
    case 'identified-expression': {
      const { result } = artifact.route.result
      if (result.kind === 'unidentifiable') return `The recorded graph does not identify this intervention query from the observed distribution.`
      return `Setting ${artifact.set.name} to its lowest bin gives an expected ${artifact.read.name} of ${figure(result.expectations[0])}; setting it to its highest gives ${figure(result.expectations[1])}, a difference of ${figure(result.effect)} under the ${result.algorithm} expression.`
    }
    default: return assertNever(artifact.route)
  }
}

/** What the canvas overlays while a question is being framed: the set node, and the read node once chosen. */
export interface InterventionOverlay {
  readonly set: DagNodeId
  readonly read: DagNodeId | null
}
