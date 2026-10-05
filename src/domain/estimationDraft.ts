import { assertNever } from './dop'
import type { ColumnId } from './dataset'
import {
  ESTIMATOR_GROUPS,
  ESTIMATOR_IDS,
  defaultConfiguration,
  defaultEstimatorFor,
  type EstimatorId,
  type EstimatorConfiguration,
  type EstimatorGroupId,
  type EstimationRunArtifact,
} from './estimation'
import type { PanelLongMatrix, PanelInterventionLayout, PanelInterventionPreflight } from './panel'
import type { PreparedDatasetArtifact, PreparedDatasetVersionId } from './preprocessing'
import {
  estimableIdentification,
  type CovariateEncoding,
  type IdentificationId,
  type IdentificationArtifact,
  type StudySpecification,
} from './study'
import type { Workflow } from './workflow'

export const estimatorGroupFor = (estimator: EstimatorId) =>
  ESTIMATOR_GROUPS.find((group) => group.estimators.includes(estimator)) ?? ESTIMATOR_GROUPS[0]

export interface PanelBinding {
  readonly prepared: PreparedDatasetArtifact['id']
  readonly unit: ColumnId
  readonly time: ColumnId
  readonly outcome: ColumnId
  readonly treatment: ColumnId
}

export interface StudyDataBinding {
  readonly prepared: PreparedDatasetArtifact['id']
  readonly dagRevision: StudySpecification['dagRevision']
  readonly treatment: ColumnId
  readonly outcome: ColumnId
}

export type StudyDataPreflightJob =
  | { readonly kind: 'not-required' }
  | { readonly kind: 'loading'; readonly binding: StudyDataBinding }
  | {
      readonly kind: 'ready'
      readonly binding: StudyDataBinding
      readonly treatmentIsBinary: boolean
      readonly outcomeIsCount: boolean
      readonly observedGraphIsBinary: boolean
    }
  | { readonly kind: 'failed'; readonly binding: StudyDataBinding; readonly detail: string }

export type PanelPreflightJob =
  | { readonly kind: 'not-required' }
  | { readonly kind: 'loading'; readonly binding: PanelBinding }
  | {
      readonly kind: 'ready'
      readonly binding: PanelBinding
      readonly matrix: PanelLongMatrix
      readonly layout: PanelInterventionLayout
    }
  | {
      readonly kind: 'layout-refused'
      readonly binding: PanelBinding
      readonly matrix: PanelLongMatrix
      readonly problem: Extract<
        Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem'],
        { readonly kind: 'panel-layout' }
      >
    }
  | {
      readonly kind: 'refused'
      readonly binding: PanelBinding
      readonly problem: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem']
    }

/** The identification the panel works from, with the estimator that fits it and fresh defaults for every estimator. */
export interface EstimationSelection {
  readonly identification: IdentificationId | null
  readonly estimator: EstimatorId
  readonly configurations: Readonly<Record<EstimatorId, EstimatorConfiguration>>
  /** Columns left out of this record are numeric. */
  readonly encodings: Readonly<Record<ColumnId, CovariateEncoding>>
}

export const estimationSelection = (
  identification: IdentificationArtifact | null,
  studies: readonly StudySpecification[],
  prepared: PreparedDatasetArtifact,
): EstimationSelection => {
  const study = studies.find((candidate) => candidate.id === identification?.study) ?? null
  return {
    identification: identification?.id ?? null,
    estimator: defaultEstimatorFor(identification?.result ?? null, prepared, study),
    configurations: Object.fromEntries(
      ESTIMATOR_IDS.map((estimator) => [
        estimator,
        defaultConfiguration(estimator, prepared, study),
      ]),
    ) as Record<EstimatorId, EstimatorConfiguration>,
    encodings: {},
  }
}

export interface EstimationDraft extends EstimationSelection {
  readonly group: EstimatorGroupId
  readonly panelPreflight: PanelPreflightJob
  readonly studyDataPreflight: StudyDataPreflightJob
}

export type EstimationEvent =
  | { readonly type: 'identification-chosen'; readonly selection: EstimationSelection }
  | { readonly type: 'group-chosen'; readonly group: EstimatorGroupId }
  | { readonly type: 'estimator-chosen'; readonly estimator: EstimatorId }
  | { readonly type: 'configured'; readonly configuration: EstimatorConfiguration }
  | {
      readonly type: 'encoding-declared'
      readonly column: ColumnId
      readonly encoding: CovariateEncoding
    }
  | { readonly type: 'panel-preflight-not-required' }
  | { readonly type: 'panel-preflight-started'; readonly binding: PanelBinding }
  | {
      readonly type: 'panel-preflight-succeeded'
      readonly binding: PanelBinding
      readonly matrix: PanelLongMatrix
      readonly layout: PanelInterventionLayout
    }
  | {
      readonly type: 'panel-layout-refused'
      readonly binding: PanelBinding
      readonly matrix: PanelLongMatrix
      readonly problem: Extract<
        Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem'],
        { readonly kind: 'panel-layout' }
      >
    }
  | {
      readonly type: 'panel-preflight-refused'
      readonly binding: PanelBinding
      readonly problem: Extract<PanelInterventionPreflight, { readonly kind: 'refused' }>['problem']
    }
  | { readonly type: 'study-data-preflight-not-required' }
  | { readonly type: 'study-data-preflight-started'; readonly binding: StudyDataBinding }
  | {
      readonly type: 'study-data-preflight-succeeded'
      readonly binding: StudyDataBinding
      readonly treatmentIsBinary: boolean
      readonly outcomeIsCount: boolean
      readonly observedGraphIsBinary: boolean
    }
  | {
      readonly type: 'study-data-preflight-failed'
      readonly binding: StudyDataBinding
      readonly detail: string
    }

export const stepEstimationDraft = (
  state: EstimationDraft,
  event: EstimationEvent,
): EstimationDraft => {
  switch (event.type) {
    case 'identification-chosen':
      return {
        ...state,
        ...event.selection,
        group: estimatorGroupFor(event.selection.estimator).id,
        panelPreflight: { kind: 'not-required' },
        studyDataPreflight: { kind: 'not-required' },
      }
    case 'group-chosen':
      return { ...state, group: event.group }
    case 'estimator-chosen':
      return { ...state, estimator: event.estimator, group: estimatorGroupFor(event.estimator).id }
    case 'configured':
      return {
        ...state,
        configurations: {
          ...state.configurations,
          [event.configuration.kind]: event.configuration,
        },
      }
    case 'encoding-declared':
      return { ...state, encodings: { ...state.encodings, [event.column]: event.encoding } }
    case 'panel-preflight-not-required':
      return { ...state, panelPreflight: { kind: 'not-required' } }
    case 'panel-preflight-started':
      return { ...state, panelPreflight: { kind: 'loading', binding: event.binding } }
    case 'panel-preflight-succeeded':
      return state.panelPreflight.kind === 'loading' &&
        samePanelBinding(state.panelPreflight.binding, event.binding)
        ? {
            ...state,
            panelPreflight: {
              kind: 'ready',
              binding: event.binding,
              matrix: event.matrix,
              layout: event.layout,
            },
          }
        : state
    case 'panel-layout-refused':
      return state.panelPreflight.kind === 'loading' &&
        samePanelBinding(state.panelPreflight.binding, event.binding)
        ? {
            ...state,
            panelPreflight: {
              kind: 'layout-refused',
              binding: event.binding,
              matrix: event.matrix,
              problem: event.problem,
            },
          }
        : state
    case 'panel-preflight-refused':
      return state.panelPreflight.kind === 'loading' &&
        samePanelBinding(state.panelPreflight.binding, event.binding)
        ? {
            ...state,
            panelPreflight: { kind: 'refused', binding: event.binding, problem: event.problem },
          }
        : state
    case 'study-data-preflight-not-required':
      return { ...state, studyDataPreflight: { kind: 'not-required' } }
    case 'study-data-preflight-started':
      return { ...state, studyDataPreflight: { kind: 'loading', binding: event.binding } }
    case 'study-data-preflight-succeeded':
      return state.studyDataPreflight.kind === 'loading' &&
        sameStudyDataBinding(state.studyDataPreflight.binding, event.binding)
        ? {
            ...state,
            studyDataPreflight: {
              kind: 'ready',
              binding: event.binding,
              treatmentIsBinary: event.treatmentIsBinary,
              outcomeIsCount: event.outcomeIsCount,
              observedGraphIsBinary: event.observedGraphIsBinary,
            },
          }
        : state
    case 'study-data-preflight-failed':
      return state.studyDataPreflight.kind === 'loading' &&
        sameStudyDataBinding(state.studyDataPreflight.binding, event.binding)
        ? {
            ...state,
            studyDataPreflight: { kind: 'failed', binding: event.binding, detail: event.detail },
          }
        : state
    default:
      return assertNever(event)
  }
}

export const samePanelBinding = (left: PanelBinding, right: PanelBinding): boolean =>
  left.prepared === right.prepared &&
  left.unit === right.unit &&
  left.time === right.time &&
  left.outcome === right.outcome &&
  left.treatment === right.treatment

export const sameStudyDataBinding = (left: StudyDataBinding, right: StudyDataBinding): boolean =>
  left.prepared === right.prepared &&
  left.dagRevision === right.dagRevision &&
  left.treatment === right.treatment &&
  left.outcome === right.outcome

export function initialEstimationDraft(
  prepared: PreparedDatasetArtifact,
  studies: readonly StudySpecification[],
  identifications: readonly IdentificationArtifact[],
  runs: readonly EstimationRunArtifact[],
): EstimationDraft {
  const identified = identifications.filter((item) => estimableIdentification(item.result))
  const latest =
    runs
      .filter((run) =>
        ESTIMATOR_GROUPS.some((group) => group.estimators.includes(run.configuration.kind)),
      )
      .at(-1) ?? null
  const recorded =
    latest === null ? null : (identified.find((item) => item.id === latest.identification) ?? null)
  const selection = estimationSelection(recorded ?? identified.at(-1) ?? null, studies, prepared)
  const estimator =
    latest !== null && recorded !== null ? latest.configuration.kind : selection.estimator
  return {
    ...selection,
    estimator,
    group: estimatorGroupFor(estimator).id,
    configurations:
      latest !== null && recorded !== null
        ? { ...selection.configurations, [latest.configuration.kind]: latest.configuration }
        : selection.configurations,
    encodings: latest !== null && recorded !== null ? latest.encodings : selection.encodings,
    panelPreflight: { kind: 'not-required' },
    studyDataPreflight: { kind: 'not-required' },
  }
}

export interface EstimationSession {
  readonly prepared: PreparedDatasetVersionId
  readonly draft: EstimationDraft
}

export function retainEstimationDraft(
  current: EstimationSession | null,
  workflow: Workflow,
): EstimationSession | null {
  if (workflow.kind !== 'profiled' || workflow.prepared === null) return null
  const identified = workflow.identifications.filter((item) => estimableIdentification(item.result))
  if (identified.length === 0) return null
  if (current?.prepared === workflow.prepared.id) {
    return current.draft.identification === null ||
      identified.some((item) => item.id === current.draft.identification)
      ? current
      : {
          ...current,
          draft: {
            ...current.draft,
            identification: null,
            panelPreflight: { kind: 'not-required' },
            studyDataPreflight: { kind: 'not-required' },
          },
        }
  }
  return {
    prepared: workflow.prepared.id,
    draft: initialEstimationDraft(
      workflow.prepared,
      workflow.studies,
      identified,
      workflow.estimationRuns,
    ),
  }
}
