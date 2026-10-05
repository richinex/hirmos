import { createStore } from 'zustand/vanilla'
import type { Redundancy, TemporalStructure } from './diagnostics'
import { selectedMissingness, stepPreprocessing, type PreprocessingEvent } from './preprocessing'
import { retainPreprocessing, type PreprocessingSession } from './preprocessingSession'
import type { DatasetProfileId } from './dataset'
import {
  estimableIdentification,
  type BackdoorIdentificationEvidence,
  type StudySpecification,
} from './study'
import type { PreparedDatasetVersionId } from './preprocessing'
import {
  initialDiagnosticDraft,
  stepDiagnosticDraft,
  type DiagnosticDraft,
  type DiagnosticEvent,
} from './diagnosticDraft'
import { INITIAL_WORKFLOW, stepWorkflow, type Workflow, type WorkflowEvent } from './workflow'
import {
  initialDiscoverySessionFor,
  stepDiscoverySession,
  type DiscoverySession,
  type DiscoverySessionEvent,
} from './discovery'
import {
  retainCausalModelDrafts,
  sameCausalSelection,
  stepCausalModelDraft,
  type CausalModelDraft,
  type CausalModelEvent,
} from './causalModelDraft'
import type { RootCauseSelection } from './rootCause'
import {
  retainSensitivityDraft,
  stepSensitivityDraft,
  type SensitivitySession,
  type SensitivityEvent,
} from './sensitivityDraft'
import {
  retainCounterfactualDraft,
  stepCounterfactualDraft,
  type CounterfactualSession,
  type CounterfactualEvent,
} from './counterfactualDraft'
import {
  retainSurvivalDraft,
  type SurvivalSession,
  type Draft as SurvivalDraft,
} from './survivalDraft'
import {
  retainEstimationDraft,
  stepEstimationDraft,
  type EstimationSession,
  type EstimationEvent,
} from './estimationDraft'
import {
  retainTimeSeriesDraft,
  stepTimeSeriesDraft,
  type TimeSeriesDraft,
  type TimeSeriesEvent,
} from './timeSeriesDraft'

interface AdjustmentDecision {
  readonly study: StudySpecification
  readonly evidence: BackdoorIdentificationEvidence
}

function decisionApplies(decision: AdjustmentDecision, workflow: Workflow): boolean {
  if (workflow.kind !== 'profiled' || workflow.prepared?.id !== decision.study.preparedDataset)
    return false
  return workflow.dagDocuments.some(
    (document) =>
      document.id === decision.study.dagDocument &&
      document.current.id === decision.study.dagRevision,
  )
}

interface State {
  readonly sourceEditor:
    | { readonly kind: 'closed' }
    | { readonly kind: 'choosing' | 'editing'; readonly profile: DatasetProfileId }
  readonly openSourceEditor: () => void
  readonly showSourceEditor: (kind: 'choosing' | 'editing') => void
  readonly closeSourceEditor: () => void
  readonly timeSeriesDraft: TimeSeriesDraft | null
  readonly changeTimeSeries: (prepared: PreparedDatasetVersionId, event: TimeSeriesEvent) => void
  readonly estimationDraft: EstimationSession | null
  readonly changeEstimation: (prepared: PreparedDatasetVersionId, event: EstimationEvent) => void
  readonly survivalDraft: SurvivalSession | null
  readonly changeSurvival: (prepared: PreparedDatasetVersionId, draft: SurvivalDraft) => void
  readonly counterfactualDraft: CounterfactualSession | null
  readonly changeCounterfactual: (
    prepared: PreparedDatasetVersionId,
    event: CounterfactualEvent,
  ) => void
  readonly sensitivityDraft: SensitivitySession | null
  readonly changeSensitivity: (prepared: PreparedDatasetVersionId, event: SensitivityEvent) => void
  readonly causalModelDrafts: readonly CausalModelDraft[]
  readonly changeCausalModel: (graph: RootCauseSelection, event: CausalModelEvent) => void
  readonly diagnosticDraft: DiagnosticDraft | null
  readonly changeDiagnostic: (prepared: PreparedDatasetVersionId, event: DiagnosticEvent) => void
  readonly adjustmentDecision: AdjustmentDecision | null
  readonly offerAdjustment: (decision: AdjustmentDecision) => void
  readonly clearAdjustment: (study: StudySpecification['id']) => void
  readonly preprocessing: PreprocessingSession | null
  readonly changePreprocessing: (profile: DatasetProfileId, event: PreprocessingEvent) => void
  readonly saveRecipe: (profile: DatasetProfileId, recipe: string) => void
  readonly redundancy: Redundancy | null
  readonly recordRedundancy: (result: Redundancy) => void
  readonly temporalStructure: TemporalStructure | null
  readonly recordTemporalStructure: (result: TemporalStructure) => void
  readonly workflow: Workflow
  readonly dispatch: (event: WorkflowEvent) => void
  readonly discovery: DiscoverySession
  readonly dispatchDiscovery: (event: DiscoverySessionEvent) => void
}

export function createWorkflowStore(initial: Workflow = INITIAL_WORKFLOW) {
  return createStore<State>((set) => ({
    sourceEditor: { kind: 'closed' },
    openSourceEditor: () =>
      set((state) =>
        state.workflow.kind === 'profiled'
          ? { sourceEditor: { kind: 'choosing', profile: state.workflow.profile.id } }
          : state,
      ),
    showSourceEditor: (kind) =>
      set((state) =>
        state.sourceEditor.kind === 'closed'
          ? state
          : { sourceEditor: { ...state.sourceEditor, kind } },
      ),
    closeSourceEditor: () => set({ sourceEditor: { kind: 'closed' } }),
    timeSeriesDraft: retainTimeSeriesDraft(null, initial),
    changeTimeSeries: (prepared, event) =>
      set((state) =>
        state.timeSeriesDraft?.prepared === prepared
          ? { timeSeriesDraft: stepTimeSeriesDraft(state.timeSeriesDraft, event) }
          : state,
      ),
    estimationDraft: retainEstimationDraft(null, initial),
    changeEstimation: (prepared, event) =>
      set((state) => {
        const current = state.estimationDraft
        if (current === null || current.prepared !== prepared || state.workflow.kind !== 'profiled')
          return state
        if (
          event.type === 'identification-chosen' &&
          event.selection.identification !== null &&
          !state.workflow.identifications.some(
            (item) =>
              item.id === event.selection.identification && estimableIdentification(item.result),
          )
        )
          return state
        const draft = stepEstimationDraft(current.draft, event)
        return draft === current.draft ? state : { estimationDraft: { prepared, draft } }
      }),
    survivalDraft: retainSurvivalDraft(null, initial),
    changeSurvival: (prepared, draft) =>
      set((state) =>
        state.survivalDraft?.prepared === prepared ? { survivalDraft: { prepared, draft } } : state,
      ),
    counterfactualDraft: retainCounterfactualDraft(null, initial),
    changeCounterfactual: (prepared, event) =>
      set((state) => {
        const current = state.counterfactualDraft
        if (current === null || current.prepared !== prepared || state.workflow.kind !== 'profiled')
          return state
        if (
          event.type === 'identification-chosen' &&
          event.identification !== null &&
          !state.workflow.identifications.some(
            (item) => item.id === event.identification && item.result.kind === 'identified',
          )
        )
          return state
        return {
          counterfactualDraft: { prepared, draft: stepCounterfactualDraft(current.draft, event) },
        }
      }),
    sensitivityDraft: retainSensitivityDraft(null, initial),
    changeSensitivity: (prepared, event) =>
      set((state) => {
        const current = state.sensitivityDraft
        if (current === null || current.prepared !== prepared || state.workflow.kind !== 'profiled')
          return state
        if (
          event.type === 'run-chosen' &&
          event.run !== null &&
          !state.workflow.estimationRuns.some((run) => run.id === event.run)
        )
          return state
        return { sensitivityDraft: { prepared, draft: stepSensitivityDraft(current.draft, event) } }
      }),
    causalModelDrafts: retainCausalModelDrafts([], initial),
    changeCausalModel: (graph, event) =>
      set((state) => {
        const current = state.causalModelDrafts.find((draft) =>
          sameCausalSelection(draft.graph, graph),
        )
        if (current === undefined) return state
        const next = stepCausalModelDraft(current, event)
        return {
          causalModelDrafts: state.causalModelDrafts.map((draft) =>
            draft === current ? next : draft,
          ),
        }
      }),
    diagnosticDraft:
      initial.kind === 'profiled' && initial.prepared !== null
        ? initialDiagnosticDraft(initial.prepared)
        : null,
    changeDiagnostic: (prepared, event) =>
      set((state) => {
        if (
          state.workflow.kind !== 'profiled' ||
          state.workflow.prepared?.id !== prepared ||
          state.diagnosticDraft === null
        )
          return state
        const diagnosticDraft = stepDiagnosticDraft(
          state.diagnosticDraft,
          event,
          state.workflow.prepared,
        )
        return diagnosticDraft === state.diagnosticDraft ? state : { diagnosticDraft }
      }),
    adjustmentDecision: null,
    offerAdjustment: (decision) =>
      set((state) =>
        decisionApplies(decision, state.workflow) ? { adjustmentDecision: decision } : state,
      ),
    clearAdjustment: (study) =>
      set((state) =>
        state.adjustmentDecision?.study.id === study ? { adjustmentDecision: null } : state,
      ),
    preprocessing: retainPreprocessing(null, initial),
    changePreprocessing: (profile, event) =>
      set((state) => {
        const current = state.preprocessing
        if (current === null || current.profile !== profile || state.workflow.kind !== 'profiled')
          return state
        const next = stepPreprocessing(current.draft, event)
        const draft =
          next.variables === current.draft.variables
            ? next
            : { ...next, missingness: selectedMissingness(next, state.workflow.profile) }
        return draft === current.draft ? state : { preprocessing: { ...current, draft } }
      }),
    saveRecipe: (profile, savedRecipe) =>
      set((state) => {
        const current = state.preprocessing
        if (current === null || current.profile !== profile) return state
        return { preprocessing: { ...current, savedRecipe } }
      }),
    redundancy: null,
    recordRedundancy: (result) =>
      set((state) => {
        if (state.workflow.kind !== 'profiled' || state.workflow.prepared?.id !== result.prepared)
          return state
        return { redundancy: result }
      }),
    temporalStructure: null,
    recordTemporalStructure: (result) =>
      set((state) => {
        if (state.workflow.kind !== 'profiled' || state.workflow.prepared?.id !== result.prepared)
          return state
        return { temporalStructure: result }
      }),
    workflow: initial,
    discovery: initialDiscoverySessionFor(
      initial.kind === 'profiled' ? initial.prepared : null,
      initial.kind === 'profiled' ? (initial.discoveryRuns.at(-1) ?? null) : null,
    ),
    dispatchDiscovery: (event) => {
      set((state) => {
        const discovery = stepDiscoverySession(state.discovery, event)
        return discovery === state.discovery ? state : { discovery }
      })
    },
    dispatch: (event: WorkflowEvent) => {
      set((state) => {
        const workflow = stepWorkflow(state.workflow, event)
        if (workflow === state.workflow) return state
        const discovery = stepDiscoverySession(state.discovery, {
          type: 'prepared-dataset-changed',
          prepared: workflow.kind === 'profiled' ? workflow.prepared : null,
          recorded: workflow.kind === 'profiled' ? (workflow.discoveryRuns.at(-1) ?? null) : null,
        })
        const prepared = workflow.kind === 'profiled' ? workflow.prepared?.id : undefined
        const temporalStructure =
          state.temporalStructure?.prepared === prepared ? state.temporalStructure : null
        const redundancy = state.redundancy?.prepared === prepared ? state.redundancy : null
        const preprocessing = retainPreprocessing(
          state.workflow.kind === 'profiled' &&
            workflow.kind === 'profiled' &&
            state.workflow.project.id === workflow.project.id
            ? state.preprocessing
            : null,
          workflow,
        )
        const adjustmentDecision =
          event.type !== 'study-draft-changed' &&
          state.adjustmentDecision !== null &&
          decisionApplies(state.adjustmentDecision, workflow)
            ? state.adjustmentDecision
            : null
        const diagnosticDraft =
          workflow.kind !== 'profiled' || workflow.prepared === null
            ? null
            : state.workflow.kind === 'profiled' &&
                state.workflow.project.id === workflow.project.id &&
                state.diagnosticDraft?.prepared === workflow.prepared.id
              ? state.diagnosticDraft
              : initialDiagnosticDraft(workflow.prepared)
        const sameProject =
          state.workflow.kind === 'profiled' &&
          workflow.kind === 'profiled' &&
          state.workflow.project.id === workflow.project.id
        const sensitivityDraft = retainSensitivityDraft(
          sameProject ? state.sensitivityDraft : null,
          workflow,
        )
        const counterfactualDraft = retainCounterfactualDraft(
          sameProject ? state.counterfactualDraft : null,
          workflow,
        )
        const survivalDraft = retainSurvivalDraft(
          sameProject ? state.survivalDraft : null,
          workflow,
        )
        const estimationDraft = retainEstimationDraft(
          sameProject ? state.estimationDraft : null,
          workflow,
        )
        const timeSeriesDraft = retainTimeSeriesDraft(
          sameProject ? state.timeSeriesDraft : null,
          workflow,
        )
        const causalModelDrafts = retainCausalModelDrafts(
          sameProject ? state.causalModelDrafts : [],
          workflow,
        )
        const sourceEditor =
          workflow.kind === 'profiled' &&
          state.sourceEditor.kind !== 'closed' &&
          state.sourceEditor.profile === workflow.profile.id
            ? state.sourceEditor
            : { kind: 'closed' as const }
        return {
          workflow,
          sourceEditor,
          discovery,
          temporalStructure,
          redundancy,
          preprocessing,
          adjustmentDecision,
          diagnosticDraft,
          causalModelDrafts,
          sensitivityDraft,
          counterfactualDraft,
          survivalDraft,
          estimationDraft,
          timeSeriesDraft,
        }
      })
    },
  }))
}
