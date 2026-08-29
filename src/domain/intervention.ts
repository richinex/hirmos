import { brand, err, ok, type Brand, type Result } from './dop'
import type { DagDocument, DagDocumentId, DagNode, DagNodeId, DagRevisionId } from './dag'
import type { DiscreteBnEvidence } from './estimation'
import type { PreparedDatasetVersionId } from './preprocessing'

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

export interface InterventionQueryArtifact {
  readonly kind: 'intervention-query'
  readonly id: InterventionQueryId
  readonly dagDocument: DagDocumentId
  readonly dagRevision: DagRevisionId
  readonly preparedDataset: PreparedDatasetVersionId
  readonly createdAt: string
  readonly set: InterventionTarget
  readonly read: InterventionTarget
  readonly bins: number
  readonly equivalentSampleSize: number
  readonly result: DiscreteBnEvidence
}

export type InterventionReadinessProblem =
  | { readonly kind: 'set-required' }
  | { readonly kind: 'read-required' }
  | { readonly kind: 'distinct-required' }
  | { readonly kind: 'latent-node'; readonly name: string }
  | { readonly kind: 'unmeasured-node'; readonly name: string }
  | { readonly kind: 'graph-invalid' }

export interface InterventionSpecification {
  readonly set: DagNode & { readonly kind: 'observed' }
  readonly read: DagNode & { readonly kind: 'observed' }
}

/** Both nodes chosen, distinct and observed; the whole graph measured, since the network is fitted to every node; the revision structurally valid. */
export function readyInterventionQuery(document: DagDocument, set: DagNodeId | null, read: DagNodeId | null): Result<InterventionSpecification, InterventionReadinessProblem> {
  if (document.current.validation.kind === 'invalid') return err({ kind: 'graph-invalid' })
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
  const unmeasured = nodes.find((node) => node.kind === 'latent')
  if (unmeasured !== undefined) return err({ kind: 'unmeasured-node', name: unmeasured.name })
  return ok({ set: setNode, read: readNode })
}

export function describeInterventionReadiness(problem: InterventionReadinessProblem): string {
  switch (problem.kind) {
    case 'set-required': return 'Choose the variable to set.'
    case 'read-required': return 'Choose the variable to read.'
    case 'distinct-required': return 'Set one variable and read a different one.'
    case 'latent-node': return `${problem.name} is unmeasured; only measured variables can be set or read.`
    case 'unmeasured-node': return `${problem.name} is unmeasured. The network is fitted to every node, so every node needs a column.`
    case 'graph-invalid': return 'The graph has structural issues; resolve them before asking an intervention question.'
    default: { const exhaustive: never = problem; return exhaustive }
  }
}

/** Figures at the precision the tiles use, with a true minus sign. */
const figure = (value: number): string => (Math.abs(value) >= 100 ? value.toFixed(0) : Math.abs(value) >= 10 ? value.toFixed(1) : value.toFixed(2)).replace('-', '\u2212')

/** The answer in one sentence: both expectations, their difference, and what the surgery adjusted for. */
export function describeInterventionVerdict(artifact: InterventionQueryArtifact): string {
  const { result } = artifact
  const adjusted = result.parentsAdjusted.length === 0
    ? `${artifact.set.name} has no parents in the graph, so no adjustment was needed`
    : `adjusted for ${result.parentsAdjusted.join(', ')}, the parents of ${artifact.set.name}`
  return `Setting ${artifact.set.name} to its lowest bin gives an expected ${artifact.read.name} of ${figure(result.expectations[0])}; setting it to its highest gives ${figure(result.expectations[1])}, a difference of ${figure(result.effect)} (${adjusted}).`
}

/** What the canvas overlays while a question is being framed: the set node, and the read node once chosen. */
export interface InterventionOverlay {
  readonly set: DagNodeId
  readonly read: DagNodeId | null
}
