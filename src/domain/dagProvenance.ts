import { dagExploration, type DagDocument, type DagDraftRevision } from './dag'
import type { StudySpecification } from './study'
import { assertNever } from './dop'

export type StudyDagRevision =
  | {
      readonly kind: 'current' | 'earlier'
      readonly document: DagDocument
      readonly revision: DagDraftRevision
    }
  | { readonly kind: 'missing-document' }
  | { readonly kind: 'missing-revision'; readonly document: DagDocument }

export function resolveStudyDagRevision(
  study: StudySpecification,
  documents: readonly DagDocument[],
): StudyDagRevision {
  const document = documents.find((d) => d.id === study.dagDocument)
  if (document === undefined) return { kind: 'missing-document' }
  const revision = document.audit.find((r) => r.id === study.dagRevision)
  if (revision === undefined) return { kind: 'missing-revision', document }
  return { kind: document.current.id === revision.id ? 'current' : 'earlier', document, revision }
}
export function studyRevisionNotice(state: StudyDagRevision): string | null {
  switch (state.kind) {
    case 'current':
      return null
    case 'earlier':
      return 'This study and its results use an earlier DAG revision. The graph has since changed; the recorded estimates have not been updated. Review the revised graph and identify a new study before estimating its effects.'
    case 'missing-document':
      return 'The DAG recorded for this study is unavailable. Its graph cannot be included in the result manifest.'
    case 'missing-revision':
      return 'The DAG revision recorded for this study is unavailable. The current graph is not a substitute and will not be exported as the study graph.'
    default:
      return assertNever(state)
  }
}

export function restoreMissingDagExploration(
  document: DagDocument,
  studies: readonly StudySpecification[],
): DagDocument {
  // An explicit empty selection is a saved user choice, not missing data.
  if (document.exploration !== undefined) return document
  const latest = studies
    .filter((s) => s.dagDocument === document.id)
    .sort((a, b) => b.createdAt.localeCompare(a.createdAt))[0]
  if (latest === undefined) return document
  const selected = { treatment: latest.treatment.node, outcome: latest.outcome.node }
  return { ...document, exploration: dagExploration({ ...document, exploration: selected }) }
}
