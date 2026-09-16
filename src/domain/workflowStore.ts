import { createStore } from 'zustand/vanilla'
import type { Redundancy, TemporalStructure } from './diagnostics'
import { initialPreprocessingDraft, stepPreprocessing, type PreprocessingDraft, type PreprocessingEvent } from './preprocessing'
import type { DatasetProfileId } from './dataset'
import type { BackdoorIdentificationEvidence, StudySpecification } from './study'
import type { PreparedDatasetVersionId } from './preprocessing'
import { initialDiagnosticDraft, stepDiagnosticDraft, type DiagnosticDraft, type DiagnosticEvent } from './diagnosticDraft'
import { INITIAL_WORKFLOW, stepWorkflow, type Workflow, type WorkflowEvent } from './workflow'
import { initialDiscoverySessionFor, stepDiscoverySession, type DiscoverySession, type DiscoverySessionEvent } from './discovery'
import { retainCausalModelDrafts, sameCausalSelection, stepCausalModelDraft, type CausalModelDraft, type CausalModelEvent } from './causalModelDraft'
import type { RootCauseSelection } from './rootCause'

interface AdjustmentDecision {
  readonly study: StudySpecification
  readonly evidence: BackdoorIdentificationEvidence
}

function decisionApplies(decision: AdjustmentDecision, workflow: Workflow): boolean {
  if (workflow.kind !== 'profiled' || workflow.prepared?.id !== decision.study.preparedDataset) return false
  return workflow.dagDocuments.some(document => document.id === decision.study.dagDocument && document.current.id === decision.study.dagRevision)
}

interface State {
  readonly causalModelDrafts: readonly CausalModelDraft[]
  readonly changeCausalModel: (graph: RootCauseSelection, event: CausalModelEvent) => void
  readonly diagnosticDraft: DiagnosticDraft | null
  readonly changeDiagnostic: (prepared: PreparedDatasetVersionId, event: DiagnosticEvent) => void
  readonly adjustmentDecision: AdjustmentDecision | null
  readonly offerAdjustment: (decision: AdjustmentDecision) => void
  readonly clearAdjustment: (study: StudySpecification['id']) => void
  readonly preprocessing: { readonly profile: DatasetProfileId; readonly draft: PreprocessingDraft; readonly savedRecipe: string | null } | null
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
  return createStore<State>(set => ({
    causalModelDrafts: retainCausalModelDrafts([], initial),
    changeCausalModel: (graph, event) => set(state => {
      const current = state.causalModelDrafts.find(draft => sameCausalSelection(draft.graph, graph))
      if (current === undefined) return state
      const next = stepCausalModelDraft(current, event)
      return { causalModelDrafts: state.causalModelDrafts.map(draft => draft === current ? next : draft) }
    }),
    diagnosticDraft: initial.kind === 'profiled' && initial.prepared !== null ? initialDiagnosticDraft(initial.prepared) : null,
    changeDiagnostic: (prepared, event) => set(state => {
      if (state.workflow.kind !== 'profiled' || state.workflow.prepared?.id !== prepared || state.diagnosticDraft === null) return state
      const diagnosticDraft = stepDiagnosticDraft(state.diagnosticDraft, event, state.workflow.prepared)
      return diagnosticDraft === state.diagnosticDraft ? state : { diagnosticDraft }
    }),
    adjustmentDecision: null,
    offerAdjustment: decision => set(state => decisionApplies(decision, state.workflow) ? { adjustmentDecision: decision } : state),
    clearAdjustment: study => set(state => state.adjustmentDecision?.study.id === study ? { adjustmentDecision: null } : state),
    preprocessing: initial.kind === 'profiled' ? { profile: initial.profile.id, draft: initialPreprocessingDraft(initial.profile), savedRecipe: null } : null,
    changePreprocessing: (profile, event) => set(state => {
      const current = state.preprocessing
      if (current === null || current.profile !== profile) return state
      const draft = stepPreprocessing(current.draft, event)
      return draft === current.draft ? state : { preprocessing: { ...current, draft } }
    }),
    saveRecipe: (profile, savedRecipe) => set(state => {
      const current = state.preprocessing
      if (current === null || current.profile !== profile) return state
      return { preprocessing: { ...current, savedRecipe } }
    }),
    redundancy: null,
    recordRedundancy: result => set(state => {
      if (state.workflow.kind !== 'profiled' || state.workflow.prepared?.id !== result.prepared) return state
      return { redundancy: result }
    }),
    temporalStructure: null,
    recordTemporalStructure: result => set(state => {
      if (state.workflow.kind !== 'profiled' || state.workflow.prepared?.id !== result.prepared) return state
      return { temporalStructure: result }
    }),
    workflow: initial,
    discovery: initialDiscoverySessionFor(initial.kind === 'profiled' ? initial.prepared : null, initial.kind === 'profiled' ? initial.discoveryRuns.at(-1) ?? null : null),
    dispatchDiscovery: event => {
      set(state => {
        const discovery = stepDiscoverySession(state.discovery, event)
        return discovery === state.discovery ? state : { discovery }
      })
    },
    dispatch: (event: WorkflowEvent) => {
      set(state => {
        const workflow = stepWorkflow(state.workflow, event)
        if (workflow === state.workflow) return state
        const discovery = stepDiscoverySession(state.discovery, {
          type: 'prepared-dataset-changed',
          prepared: workflow.kind === 'profiled' ? workflow.prepared : null,
          recorded: workflow.kind === 'profiled' ? workflow.discoveryRuns.at(-1) ?? null : null,
        })
        const prepared = workflow.kind === 'profiled' ? workflow.prepared?.id : undefined
        const temporalStructure = state.temporalStructure?.prepared === prepared ? state.temporalStructure : null
        const redundancy = state.redundancy?.prepared === prepared ? state.redundancy : null
        const preprocessing = workflow.kind !== 'profiled' ? null
          : state.workflow.kind === 'profiled' && state.workflow.project.id === workflow.project.id && state.preprocessing?.profile === workflow.profile.id ? state.preprocessing
          : { profile: workflow.profile.id, draft: initialPreprocessingDraft(workflow.profile), savedRecipe: null }
        const adjustmentDecision = event.type !== 'study-draft-changed' && state.adjustmentDecision !== null && decisionApplies(state.adjustmentDecision, workflow) ? state.adjustmentDecision : null
        const diagnosticDraft = workflow.kind !== 'profiled' || workflow.prepared === null ? null
          : state.workflow.kind === 'profiled' && state.workflow.project.id === workflow.project.id && state.diagnosticDraft?.prepared === workflow.prepared.id ? state.diagnosticDraft
          : initialDiagnosticDraft(workflow.prepared)
        const sameProject = state.workflow.kind === 'profiled' && workflow.kind === 'profiled' && state.workflow.project.id === workflow.project.id
        const causalModelDrafts = retainCausalModelDrafts(sameProject ? state.causalModelDrafts : [], workflow)
        return { workflow, discovery, temporalStructure, redundancy, preprocessing, adjustmentDecision, diagnosticDraft, causalModelDrafts }
      })
    },
  }))
}
