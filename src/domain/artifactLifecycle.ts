import type { Workflow } from './workflow'
import type { DagDocumentId } from './dag'
import { estimandSentence, type StudySpecification } from './study'
import { groupSwigAnalyses, swigGraphTitle, type SwigAnalysis } from './swig'
import type { DagCheckArtifact } from './dagValidation'
import type { RootCauseCheckRecord } from './rootCauseAnalysis'
import { assertNever, isNonEmpty, type NonEmptyArray } from './dop'

export type DeletionTarget =
  | { readonly kind: 'dag'; readonly id: DagDocumentId }
  | { readonly kind: 'study'; readonly id: StudySpecification['id'] }
  | { readonly kind: 'dag-check'; readonly id: DagCheckArtifact['id'] }
  | { readonly kind: 'model-check'; readonly id: RootCauseCheckRecord['id'] }
  /** A saved intervention graph together with the separation checks run on it. */
  | { readonly kind: 'swig-graph'; readonly id: SwigAnalysis['id'] }

export interface DeletionReference {
  readonly id: string
  readonly createdAt: string
  readonly label: string
  readonly location: string
  readonly removable?: DeletionTarget
}
export type DeletionDecision =
  | { readonly kind: 'missing' }
  | {
      readonly kind: 'ready'
      readonly name: string
      readonly createdAt: string
      readonly consequence: string
    }
  | {
      readonly kind: 'blocked'
      readonly name: string
      readonly createdAt: string
      readonly references: NonEmptyArray<DeletionReference>
    }

/** Shared by the confirmation dialog and reducer. Never persist deletion permission. */
export function assessArtifactDeletion(state: Workflow, target: DeletionTarget): DeletionDecision {
  if (state.kind !== 'profiled') return { kind: 'missing' }
  const references: DeletionReference[] = []
  switch (target.kind) {
    case 'study': {
      const study = state.studies.find((item) => item.id === target.id)
      if (study === undefined) return { kind: 'missing' }
      const identifications = new Set(
        state.identifications.filter((item) => item.study === target.id).map((item) => item.id),
      )
      for (const run of state.estimationRuns) {
        if (run.study === target.id || identifications.has(run.identification))
          references.push({
            id: run.id,
            createdAt: run.createdAt,
            label: 'Effect estimate',
            location: 'Estimation, Runs',
          })
      }
      for (const run of state.counterfactualRuns) {
        if (run.study === target.id || identifications.has(run.identification))
          references.push({
            id: run.id,
            createdAt: run.createdAt,
            label: 'Counterfactual result',
            location: 'Counterfactuals, Runs',
          })
      }
      const name = estimandSentence(study)
      const createdAt = study.createdAt
      return isNonEmpty(references)
        ? { kind: 'blocked', name, createdAt, references }
        : {
            kind: 'ready',
            name,
            createdAt,
            consequence:
              'This permanently removes the study and its identification records. The DAG and prepared data remain unchanged.',
          }
    }
    case 'dag': {
      const document = state.dagDocuments.find((item) => item.id === target.id)
      if (document === undefined) return { kind: 'missing' }
      for (const study of state.studies) {
        if (study.dagDocument === target.id)
          references.push({
            id: study.id,
            createdAt: study.createdAt,
            label: estimandSentence(study),
            location: 'Study design, Studies',
          })
      }
      for (const check of state.dagChecks) {
        if (check.dagDocument === target.id)
          references.push({
            id: check.id,
            createdAt: check.createdAt,
            label: 'Graph check',
            location: 'DAG workspace, Graph checks',
            removable: { kind: 'dag-check', id: check.id },
          })
      }
      for (const query of state.interventionQueries) {
        if (query.dagDocument === target.id)
          references.push({
            id: query.id,
            createdAt: query.createdAt,
            label: 'Intervention query',
            location: 'DAG workspace, Intervene',
          })
      }
      for (const entry of groupSwigAnalyses(
        state.swigAnalyses.filter((analysis) => analysis.dagDocument === target.id),
      ))
        references.push({
          id: entry.graph.id,
          createdAt: entry.graph.createdAt,
          label: swigGraphTitle(entry),
          location: 'DAG workspace, SWIG, Saved graphs',
          removable: { kind: 'swig-graph', id: entry.graph.id },
        })
      const groups = [
        { records: state.rootCause.runs, label: 'Causal model analysis' },
        { records: state.rootCause.effects, label: 'Intervention effect' },
        { records: state.rootCause.influences, label: 'Influence analysis' },
      ]
      for (const group of groups) {
        for (const record of group.records) {
          if (record.graph.dagDocument === target.id)
            references.push({
              id: record.id,
              createdAt: record.createdAt,
              label: group.label,
              location: 'Causal model analysis, Run history',
            })
        }
      }
      for (const check of state.rootCause.checks) {
        if (check.graph.dagDocument === target.id)
          references.push({
            id: check.id,
            createdAt: check.createdAt,
            label: 'Causal model check',
            location: 'Causal model analysis',
            removable: { kind: 'model-check', id: check.id },
          })
      }
      const createdAt = document.audit[0].createdAt
      return isNonEmpty(references)
        ? { kind: 'blocked', name: document.name, createdAt, references }
        : {
            kind: 'ready',
            name: document.name,
            createdAt,
            consequence:
              'This permanently removes the DAG and all its revisions. The prepared data and discovery results remain unchanged.',
          }
    }
    case 'dag-check': {
      const check = state.dagChecks.find((item) => item.id === target.id)
      return check !== undefined
        ? {
            kind: 'ready',
            name: 'Graph check',
            createdAt: check.createdAt,
            consequence:
              'This permanently removes this saved graph check. The DAG remains unchanged.',
          }
        : { kind: 'missing' }
    }
    case 'model-check': {
      const check = state.rootCause.checks.find((item) => item.id === target.id)
      return check !== undefined
        ? {
            kind: 'ready',
            name: 'Causal model check',
            createdAt: check.createdAt,
            consequence:
              'This permanently removes this saved model check. The DAG and other results remain unchanged.',
          }
        : { kind: 'missing' }
    }
    case 'swig-graph': {
      const entry = savedSwigGraph(state, target.id)
      return entry === undefined
        ? { kind: 'missing' }
        : {
            kind: 'ready',
            name: swigGraphTitle(entry),
            createdAt: entry.graph.createdAt,
            consequence:
              entry.checks.length === 0
                ? 'This permanently removes the saved graph. The DAG remains unchanged.'
                : 'This permanently removes the saved graph and the separation checks run on it. The DAG remains unchanged.',
          }
    }
    default:
      return assertNever(target)
  }
}

const savedSwigGraph = (state: Extract<Workflow, { kind: 'profiled' }>, id: SwigAnalysis['id']) =>
  groupSwigAnalyses(state.swigAnalyses).find((entry) => entry.graph.id === id)

export function deleteArtifact(
  state: Extract<Workflow, { kind: 'profiled' }>,
  target: DeletionTarget,
): Workflow {
  if (assessArtifactDeletion(state, target).kind !== 'ready') return state
  switch (target.kind) {
    case 'dag':
      return {
        ...state,
        dagDocuments: state.dagDocuments.filter((item) => item.id !== target.id),
        studyDraft:
          state.studyDraft.dagDocument === target.id
            ? { ...state.studyDraft, dagDocument: null }
            : state.studyDraft,
        rootCause:
          state.rootCause.selection?.dagDocument === target.id
            ? { ...state.rootCause, selection: null }
            : state.rootCause,
      }
    case 'study':
      return {
        ...state,
        studies: state.studies.filter((item) => item.id !== target.id),
        identifications: state.identifications.filter((item) => item.study !== target.id),
      }
    case 'dag-check':
      return { ...state, dagChecks: state.dagChecks.filter((item) => item.id !== target.id) }
    case 'model-check':
      return {
        ...state,
        rootCause: {
          ...state.rootCause,
          checks: state.rootCause.checks.filter((item) => item.id !== target.id),
        },
      }
    case 'swig-graph': {
      const entry = savedSwigGraph(state, target.id)
      if (entry === undefined) return state
      const removed = new Set([entry.graph, ...entry.checks].map((record) => record.id))
      return {
        ...state,
        swigAnalyses: state.swigAnalyses.filter((record) => !removed.has(record.id)),
      }
    }
    default:
      return assertNever(target)
  }
}
