import type { DagDocument, DagDocumentId, DagNodeId, DagRevisionId, ObservedDagNode } from './dag'
import { inspectDagStructure } from './dag'
import { z } from 'zod'
import { brand } from './dop'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'
import { assertNever, err, isNonEmpty, ok, type NonEmptyArray, type Result } from './dop'

/** A recorded graph revision, independent of the treatment-effect study selection. */
export interface RootCauseSelection {
  readonly dagDocument: DagDocumentId
  readonly dagRevision: DagRevisionId
  readonly preparedDataset: PreparedDatasetVersionId
}

export const rootCauseSelectionSchema = z.object({
  dagDocument: z.string().min(1).transform((value) => brand<string, 'DagDocumentId'>(value)),
  dagRevision: z.string().min(1).transform((value) => brand<string, 'DagRevisionId'>(value)),
  preparedDataset: z.string().min(1).transform((value) => brand<string, 'PreparedDatasetVersionId'>(value)),
}).strict()

/** Resolve the recorded revision, even when the editor has since changed the graph. */
export function selectedRootCauseGraph(selection: RootCauseSelection | null, documents: readonly DagDocument[], prepared: PreparedDatasetArtifact): Result<RootCauseGraph, RootCauseGraphProblem> {
  if (selection === null) return err({ kind: 'no-selection' })
  if (selection.preparedDataset !== prepared.id) return err({ kind: 'different-preparation' })
  const document = documents.find((candidate) => candidate.id === selection.dagDocument)
  const revision = document?.audit.find((candidate) => candidate.id === selection.dagRevision)
  if (document === undefined || revision === undefined) return err({ kind: 'missing-revision' })
  return prepareRootCauseGraph({ ...document, current: revision }, prepared)
}

const checkedGraph: unique symbol = Symbol('RootCauseGraph')

export interface RootCauseGraph extends RootCauseSelection {
  readonly [checkedGraph]: true
  readonly nodes: NonEmptyArray<ObservedDagNode>
  readonly edges: readonly (readonly [number, number])[]
}

/** The numeric model must describe the recorded graph, including its column order. */
export function matchesRootCauseGraph(graph: RootCauseGraph, model: { readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[] }): boolean {
  return model.names.length === graph.nodes.length
    && model.names.every((name, index) => name === graph.nodes[index]!.name)
    && model.edges.length === graph.edges.length
    && model.edges.every(([cause, effect], index) => cause === graph.edges[index]![0] && effect === graph.edges[index]![1])
}

export type RootCauseGraphProblem =
  | { readonly kind: 'no-selection' }
  | { readonly kind: 'missing-revision' }
  | { readonly kind: 'different-preparation' }
  | { readonly kind: 'invalid-graph' }
  | { readonly kind: 'unmeasured-variable'; readonly name: string }
  | { readonly kind: 'lagged-relationship' }
  | { readonly kind: 'missing-column'; readonly name: string }
  | { readonly kind: 'duplicate-column' }
  | { readonly kind: 'duplicate-name' }

/** Validate the selected graph before mapping its node order to numeric columns. */
export function prepareRootCauseGraph(document: DagDocument, prepared: PreparedDatasetArtifact): Result<RootCauseGraph, RootCauseGraphProblem> {
  if (document.preparedDataset !== prepared.id) return err({ kind: 'different-preparation' })
  const { graph } = document.current
  const validation = inspectDagStructure(graph, document.dataset)
  if (validation.kind === 'invalid') return err({ kind: 'invalid-graph' })
  if (validation.kind === 'incomplete' && validation.issues.some((issue) => issue.kind !== 'no-edges')) return err({ kind: 'invalid-graph' })

  const nodes: ObservedDagNode[] = []
  for (const node of graph.nodes) {
    if (node.kind === 'latent') return err({ kind: 'unmeasured-variable', name: node.name })
    if (!prepared.columns.includes(node.column)) return err({ kind: 'missing-column', name: node.name })
    nodes.push(node)
  }
  if (!isNonEmpty(nodes)) return err({ kind: 'invalid-graph' })
  if (new Set(nodes.map((node) => node.id)).size !== nodes.length) return err({ kind: 'invalid-graph' })
  if (new Set(nodes.map((node) => node.column)).size !== nodes.length) return err({ kind: 'duplicate-column' })
  if (new Set(nodes.map((node) => node.name)).size !== nodes.length) return err({ kind: 'duplicate-name' })
  const positions = new Map<DagNodeId, number>(nodes.map((node, index) => [node.id, index]))
  const edges: (readonly [number, number])[] = []
  for (const edge of graph.edges) {
    if (edge.timing.kind === 'lagged') return err({ kind: 'lagged-relationship' })
    const cause = positions.get(edge.cause)
    const effect = positions.get(edge.effect)
    if (cause === undefined || effect === undefined) return err({ kind: 'invalid-graph' })
    edges.push([cause, effect])
  }
  return ok({ [checkedGraph]: true, dagDocument: document.id, dagRevision: document.current.id, preparedDataset: prepared.id, nodes, edges })
}

export function describeRootCauseGraphProblem(problem: RootCauseGraphProblem): string {
  switch (problem.kind) {
    case 'no-selection': return 'Choose a graph revision from the DAG workspace.'
    case 'missing-revision': return 'The selected graph revision is no longer available. Choose a graph from the DAG workspace.'
    case 'different-preparation': return 'Choose a graph for the current prepared dataset.'
    case 'invalid-graph': return 'Resolve the graph issues and record the reasons for its arrows before using it for analysis.'
    case 'unmeasured-variable': return `${problem.name} is unmeasured. This model requires an observed column for every variable.`
    case 'lagged-relationship': return 'This model uses relationships within each observation. Lagged relationships need a time-indexed model.'
    case 'missing-column': return `${problem.name} is not included in the prepared dataset.`
    case 'duplicate-column': return 'Each graph variable must use a different data column.'
    case 'duplicate-name': return 'Each graph variable must have a different name.'
    default: return assertNever(problem)
  }
}
