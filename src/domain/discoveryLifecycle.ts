import type {
  DagDocument,
  DagDocumentId,
  DagEdgeId,
  DagName,
  DagRevisionId,
  DiscoveryCandidateId,
  EdgeTiming,
} from './dag'
import type { DiscoveryRunId } from './discovery'
import { assertNever, isNonEmpty, type NonEmptyArray } from './dop'

const deletionPermission: unique symbol = Symbol('DeletableDiscoveryRun')

/** Proof that the named discovery run exists and no DAG audit record refers to it. */
export interface DeletableDiscoveryRun {
  readonly run: DiscoveryRunId
  readonly [deletionPermission]: true
}

export type DiscoveryRunReference =
  | {
      readonly kind: 'dag-origin-reference'
      readonly document: DagDocumentId
      readonly documentName: DagName
    }
  | {
      readonly kind: 'edge-evidence-reference'
      readonly document: DagDocumentId
      readonly documentName: DagName
      readonly revision: DagRevisionId
      readonly revisionCreatedAt: string
      readonly edge: DagEdgeId
      readonly candidate: DiscoveryCandidateId
      readonly cause: string
      readonly effect: string
      readonly timing: EdgeTiming
    }

export type DiscoveryRunDeletionDecision =
  | { readonly kind: 'deletable'; readonly deletion: DeletableDiscoveryRun }
  | {
      readonly kind: 'referenced'
      readonly run: DiscoveryRunId
      readonly references: NonEmptyArray<DiscoveryRunReference>
    }
  | { readonly kind: 'not-found'; readonly run: DiscoveryRunId }

const nodeName = (revision: DagDocument['current'], node: string): string =>
  revision.graph.nodes.find((candidate) => candidate.id === node)?.name ?? node

/**
 * Decide whether a discovery run may be removed. The audit is append-only, so evidence in an older
 * DAG revision is as much a dependency as evidence on the current graph.
 */
export function assessDiscoveryRunDeletion(
  runs: readonly DiscoveryRunId[],
  documents: readonly DagDocument[],
  run: DiscoveryRunId,
): DiscoveryRunDeletionDecision {
  if (!runs.includes(run)) return { kind: 'not-found', run }

  const references: DiscoveryRunReference[] = []
  for (const document of documents) {
    switch (document.origin.kind) {
      case 'user-authored':
        break
      case 'discovery-informed':
        if (document.origin.reports.includes(run)) {
          references.push({
            kind: 'dag-origin-reference',
            document: document.id,
            documentName: document.name,
          })
        }
        break
      default:
        assertNever(document.origin)
    }

    for (const revision of document.audit) {
      for (const edge of revision.graph.edges) {
        for (const evidence of edge.evidence) {
          if (evidence.run !== run) continue
          references.push({
            kind: 'edge-evidence-reference',
            document: document.id,
            documentName: document.name,
            revision: revision.id,
            revisionCreatedAt: revision.createdAt,
            edge: edge.id,
            candidate: evidence.candidate,
            cause: nodeName(revision, edge.cause),
            effect: nodeName(revision, edge.effect),
            timing: edge.timing,
          })
        }
      }
    }
  }

  return isNonEmpty(references)
    ? { kind: 'referenced', run, references }
    : { kind: 'deletable', deletion: { run, [deletionPermission]: true } }
}

export const deleteDiscoveryRun = <Run extends { readonly id: DiscoveryRunId }>(
  runs: readonly Run[],
  deletion: DeletableDiscoveryRun,
): readonly Run[] => runs.filter((run) => run.id !== deletion.run)
