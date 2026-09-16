import { createStore } from 'zustand/vanilla'
import type { Redundancy, TemporalStructure } from './diagnostics'
import { initialPreprocessingDraft, stepPreprocessing, type PreprocessingDraft, type PreprocessingEvent } from './preprocessing'
import type { DatasetProfileId } from './dataset'
import type { BackdoorIdentificationEvidence, StudySpecification } from './study'

interface AdjustmentDecision {
  readonly study: StudySpecification
  readonly evidence: BackdoorIdentificationEvidence
}

function decisionApplies(decision: AdjustmentDecision, workflow: Workflow): boolean {
  if (workflow.kind !== 'profiled' || workflow.prepared?.id !== decision.study.preparedDataset) return false
  return workflow.dagDocuments.some(document => document.id === decision.study.dagDocument && document.current.id === decision.study.dagRevision)
}
import { INITIAL_WORKFLOW, stepWorkflow, type Workflow, type WorkflowEvent } from './workflow'
import { initialDiscoverySessionFor, stepDiscoverySession, type DiscoverySession, type DiscoverySessionEvent } from './discovery'

interface State {
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
        return { workflow, discovery, temporalStructure, redundancy, preprocessing, adjustmentDecision }
      })
    },
  }))
}
