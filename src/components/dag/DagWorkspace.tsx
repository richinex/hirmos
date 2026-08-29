import { Alert } from '@/components/ui/Alert'
import { Select } from '@/components/ui/Select'
import { useEffect, useMemo, useReducer, useState } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { InterventionPanel } from './InterventionPanel'
import type { InterventionOverlay, InterventionQueryArtifact } from '@/domain/intervention'
import type { SelectedSource } from '@/domain/workflow'
import { cn } from '@/lib/utils'
import { Icon } from '@/components/Icon'
import { button, field, iconControl, label, literal, pill, segment } from '@/components/ui/recipes'
import {
  createDagDocument,
  describeDagCreateProblem,
  describeDagEditProblem,
  describeDagVariableEditProblem,
  redoDagRevision,
  describeDagOrigin,
  nameOfDagNode,
  reviseDagByReplacingEdge,
  reviseDagEdgeDetails,
  reviseDagWithLatentConfounder,
  reviseDagWithLatentNode,
  reviseDagWithEdge,
  reviseDagWithoutLatentNode,
  reviseDagWithoutEdge,
  undoDagRevision,
  type DagCreateProblem,
  type DagDocument,
  type DagDocumentId,
  type DagEditProblem,
  type DagEdgeId,
  type DagNodeId,
  type DagOriginChoice,
  type DagVariableEditProblem,
  type DirectedDagEdge,
  type EdgeSupport,
  type EdgeTiming,
} from '@/domain/dag'
import {
  discoveryEvidenceReference,
  discoveryEvidenceView,
  type DiscoveryCandidate,
} from '@/domain/dagEvidence'
import {
  describeDagStructuralIssue,
  planDagImplications,
} from '@/domain/dagValidation'
import type { DatasetProfile } from '@/domain/dataset'
import type { DiscoveryRunArtifact, DiscoveryRunId } from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { DagCanvas } from './DagCanvas'
import { EdgeLedgerTable } from './EdgeLedgerTable'
import { analyseDagCausalFlow, type DagCausalFlow } from '@/domain/dagFlow'
import { EMPTY_STUDY_DRAFT, type StudyDesignDraft } from '@/domain/study'
import { EvidenceInspector } from './EvidenceInspector'

interface DagWorkspaceProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly interventionQueries: readonly InterventionQueryArtifact[]
  readonly onInterventionQuery: (query: InterventionQueryArtifact) => void
  readonly discoveryRuns: readonly DiscoveryRunArtifact[]
  readonly documents: readonly DagDocument[]
  readonly onDocumentCreated: (document: DagDocument) => void
  readonly onDocumentRevised: (document: DagDocument) => void
  /** Opens Study Design; offered only on a structurally valid revision. */
  readonly onUseForStudy: () => void
  /** The treatment and outcome being bound, shared with Study Design. */
  readonly studyDraft: StudyDesignDraft
  readonly onStudyDraftChanged: (draft: StudyDesignDraft) => void
}

type EdgeTimingDraft =
  | { readonly kind: 'contemporaneous' }
  | { readonly kind: 'lagged'; readonly lag: string }

type LatentVariableDraft =
  | { readonly kind: 'closed'; readonly problem: DagVariableEditProblem | null }
  | { readonly kind: 'adding'; readonly name: string; readonly problem: DagVariableEditProblem | null }

/** The selected edge's editable details; the saved edge is unchanged until the draft is committed as a revision. */
interface SelectedEdgeDraft {
  readonly edge: DagEdgeId
  readonly rationale: string
  readonly timing: EdgeTimingDraft
  readonly problem: DagEditProblem | null
}

type DagWorkspaceState =
  | {
      readonly kind: 'creating'
      readonly nameDraft: string
      readonly origin: DagOriginChoice
      readonly problem: DagCreateProblem | null
    }
  | {
      readonly kind: 'editing'
      readonly document: DagDocumentId
      readonly cause: DagNodeId | null
      readonly effect: DagNodeId | null
      readonly timing: EdgeTimingDraft
      readonly rationale: string
      readonly problem: DagEditProblem | null
      readonly selectedRun: DiscoveryRunId | null
      readonly selectedCandidate: DiscoveryCandidate | null
      readonly selectedEdge: SelectedEdgeDraft | null
      readonly attachSelectedEvidence: boolean
      readonly latentVariable: LatentVariableDraft
    }

type DagWorkspaceEvent =
  | { readonly type: 'new-document-requested' }
  | { readonly type: 'name-changed'; readonly value: string }
  | { readonly type: 'origin-selected'; readonly origin: DagOriginChoice }
  | { readonly type: 'creation-refused'; readonly problem: DagCreateProblem }
  | { readonly type: 'document-created'; readonly document: DagDocumentId; readonly latestRun: DiscoveryRunId | null }
  | { readonly type: 'document-selected'; readonly document: DagDocumentId; readonly latestRun: DiscoveryRunId | null }
  | { readonly type: 'cause-selected'; readonly node: DagNodeId | null }
  | { readonly type: 'effect-selected'; readonly node: DagNodeId | null }
  | { readonly type: 'timing-selected'; readonly timing: EdgeTimingDraft }
  | { readonly type: 'lag-changed'; readonly value: string }
  | { readonly type: 'rationale-changed'; readonly value: string }
  | { readonly type: 'edge-refused'; readonly problem: DagEditProblem }
  | { readonly type: 'edge-saved' }
  | { readonly type: 'run-selected'; readonly run: DiscoveryRunId }
  | { readonly type: 'candidate-selected'; readonly candidate: DiscoveryCandidate }
  | { readonly type: 'edge-selected'; readonly edge: SelectedEdgeDraft | null }
  | { readonly type: 'selected-edge-rationale-changed'; readonly value: string }
  | { readonly type: 'selected-edge-timing-selected'; readonly timing: EdgeTimingDraft }
  | { readonly type: 'selected-edge-lag-changed'; readonly value: string }
  | { readonly type: 'selected-edge-refused'; readonly problem: DagEditProblem }
  | { readonly type: 'evidence-attachment-changed'; readonly attached: boolean }
  | { readonly type: 'edge-draft-cancelled' }
  | { readonly type: 'latent-variable-add-requested' }
  | { readonly type: 'latent-variable-name-changed'; readonly value: string }
  | { readonly type: 'latent-variable-edit-cancelled' }
  | { readonly type: 'latent-variable-refused'; readonly problem: DagVariableEditProblem }
  | { readonly type: 'latent-variable-saved' }

const editingState = (document: DagDocumentId, latestRun: DiscoveryRunId | null): DagWorkspaceState => ({
  kind: 'editing',
  document,
  cause: null,
  effect: null,
  timing: { kind: 'contemporaneous' },
  rationale: '',
  problem: null,
  selectedRun: latestRun,
  selectedCandidate: null,
  selectedEdge: null,
  attachSelectedEvidence: false,
  latentVariable: { kind: 'closed', problem: null },
})

const latestRunId = (runs: readonly DiscoveryRunArtifact[]): DiscoveryRunId | null => runs.at(-1)?.id ?? null

const initialState = (documents: readonly DagDocument[]): DagWorkspaceState => {
  const [first] = documents
  return first === undefined
    ? { kind: 'creating', nameDraft: '', origin: 'domain-knowledge', problem: null }
    : editingState(first.id, null)
}

const withSelectedEdge = (
  state: DagWorkspaceState,
  update: (draft: SelectedEdgeDraft) => SelectedEdgeDraft,
): DagWorkspaceState => state.kind === 'editing' && state.selectedEdge !== null
  ? { ...state, selectedEdge: update(state.selectedEdge) }
  : state

function stepDagWorkspace(state: DagWorkspaceState, event: DagWorkspaceEvent): DagWorkspaceState {
  switch (event.type) {
    case 'new-document-requested': return { kind: 'creating', nameDraft: '', origin: 'domain-knowledge', problem: null }
    case 'name-changed': return state.kind === 'creating' ? { ...state, nameDraft: event.value, problem: null } : state
    case 'origin-selected': return state.kind === 'creating' ? { ...state, origin: event.origin, problem: null } : state
    case 'creation-refused': return state.kind === 'creating' ? { ...state, problem: event.problem } : state
    case 'document-created': return editingState(event.document, event.latestRun)
    case 'document-selected': return editingState(event.document, event.latestRun)
    case 'cause-selected': return state.kind === 'editing'
      ? { ...state, cause: event.node, problem: null, attachSelectedEvidence: false }
      : state
    case 'effect-selected': return state.kind === 'editing'
      ? { ...state, effect: event.node, problem: null, attachSelectedEvidence: false }
      : state
    case 'timing-selected': return state.kind === 'editing'
      ? { ...state, timing: event.timing, problem: null, attachSelectedEvidence: false }
      : state
    case 'lag-changed': return state.kind === 'editing' && state.timing.kind === 'lagged'
      ? { ...state, timing: { kind: 'lagged', lag: event.value }, problem: null, attachSelectedEvidence: false }
      : state
    case 'rationale-changed': return state.kind === 'editing' ? { ...state, rationale: event.value, problem: null } : state
    case 'edge-refused': return state.kind === 'editing' ? { ...state, problem: event.problem } : state
    case 'edge-saved': return state.kind === 'editing'
      ? { ...state, cause: null, effect: null, timing: { kind: 'contemporaneous' }, rationale: '', problem: null, selectedEdge: null, attachSelectedEvidence: false }
      : state
    case 'run-selected': return state.kind === 'editing'
      ? { ...state, selectedRun: event.run, selectedCandidate: null, attachSelectedEvidence: false }
      : state
    case 'candidate-selected': return state.kind === 'editing'
      ? { ...state, selectedRun: event.candidate.run, selectedCandidate: event.candidate, attachSelectedEvidence: false }
      : state
    case 'edge-selected': return state.kind === 'editing' ? { ...state, selectedEdge: event.edge } : state
    case 'selected-edge-rationale-changed': return withSelectedEdge(state, (draft) => ({ ...draft, rationale: event.value, problem: null }))
    case 'selected-edge-timing-selected': return withSelectedEdge(state, (draft) => ({ ...draft, timing: event.timing, problem: null }))
    case 'selected-edge-lag-changed': return withSelectedEdge(state, (draft) => draft.timing.kind === 'lagged'
      ? { ...draft, timing: { kind: 'lagged', lag: event.value }, problem: null }
      : draft)
    case 'selected-edge-refused': return withSelectedEdge(state, (draft) => ({ ...draft, problem: event.problem }))
    case 'evidence-attachment-changed': return state.kind === 'editing'
      ? { ...state, attachSelectedEvidence: event.attached }
      : state
    case 'edge-draft-cancelled': return state.kind === 'editing'
      ? { ...state, cause: null, effect: null, timing: { kind: 'contemporaneous' }, rationale: '', problem: null }
      : state
    case 'latent-variable-add-requested': return state.kind === 'editing'
      ? { ...state, latentVariable: { kind: 'adding', name: '', problem: null } }
      : state
    case 'latent-variable-name-changed': return state.kind === 'editing' && state.latentVariable.kind === 'adding'
      ? { ...state, latentVariable: { ...state.latentVariable, name: event.value, problem: null } }
      : state
    case 'latent-variable-edit-cancelled': return state.kind === 'editing'
      ? { ...state, latentVariable: { kind: 'closed', problem: null } }
      : state
    case 'latent-variable-refused': return state.kind === 'editing'
      ? { ...state, latentVariable: state.latentVariable.kind === 'adding'
          ? { ...state.latentVariable, problem: event.problem }
          : { kind: 'closed', problem: event.problem } }
      : state
    case 'latent-variable-saved': return state.kind === 'editing'
      ? { ...state, latentVariable: { kind: 'closed', problem: null } }
      : state
    default: return assertNever(event)
  }
}

const nodeFromValue = (document: DagDocument, raw: string): DagNodeId | null =>
  document.current.graph.nodes.find((node) => node.id === raw)?.id ?? null

const nodeForColumn = (document: DagDocument, column: string): DagNodeId | null =>
  document.current.graph.nodes.find((node) => node.kind === 'observed' && node.column === column)?.id ?? null

const timingValue = (draft: EdgeTimingDraft): EdgeTiming => draft.kind === 'contemporaneous'
  ? draft
  : { kind: 'lagged', lag: Number(draft.lag) }

const timingDraft = (timing: EdgeTiming): EdgeTimingDraft => timing.kind === 'contemporaneous'
  ? timing
  : { kind: 'lagged', lag: String(timing.lag) }

const rationaleText = (support: EdgeSupport): string | null => support.kind === 'unstated' ? null : support.rationale

const describeSupport = (support: EdgeSupport): string => {
  switch (support.kind) {
    case 'user-assumption': return 'User assumption'
    case 'experimental-design': return 'Experimental design'
    case 'unstated': return 'Rationale not yet recorded'
    default: return assertNever(support)
  }
}

const describeTiming = (timing: EdgeTiming): string => timing.kind === 'contemporaneous' ? 'Contemporaneous' : `Lag ${timing.lag}`

const edgeDraft = (edge: DirectedDagEdge): SelectedEdgeDraft => ({
  edge: edge.id,
  rationale: rationaleText(edge.support) ?? '',
  timing: timingDraft(edge.timing),
  problem: null,
})

const selectedEvidenceMatches = (
  document: DagDocument,
  candidate: DiscoveryCandidate | null,
  cause: DagNodeId | null,
  effect: DagNodeId | null,
  timing: EdgeTimingDraft,
): boolean => {
  if (candidate === null || candidate.relationMatch.kind !== 'directed-candidate' || cause === null || effect === null) return false
  const evidenceCause = nodeForColumn(document, candidate.relationMatch.cause.column)
  const evidenceEffect = nodeForColumn(document, candidate.relationMatch.effect.column)
  if (cause !== evidenceCause || effect !== evidenceEffect || timing.kind !== candidate.relationMatch.timing.kind) return false
  return timing.kind === 'contemporaneous'
    || (candidate.relationMatch.timing.kind === 'lagged' && Number(timing.lag) === candidate.relationMatch.timing.lag)
}

function OriginChoice({
  active,
  disabled = false,
  icon,
  title,
  detail,
  onClick,
}: {
  readonly active: boolean
  readonly disabled?: boolean
  readonly icon: string
  readonly title: string
  readonly detail: string
  readonly onClick: () => void
}) {
  return (
    <button
      type="button"
      disabled={disabled}
      aria-pressed={active}
      onClick={onClick}
      className={`rounded-xl border p-4 text-left transition-colors ${active ? 'border-signal bg-raised' : 'border-line bg-panel hover:border-edge disabled:cursor-not-allowed disabled:opacity-45'}`}
    >
      <span className="mb-3 grid h-9 w-9 place-items-center rounded-lg border border-hair bg-well text-muted"><Icon name={icon} size={18} /></span>
      <span className="block text-title font-medium text-ink">{title}</span>
      <span className="mt-1 block text-body text-faint">{detail}</span>
    </button>
  )
}

function PathList({ document, flow }: { readonly document: DagDocument; readonly flow: DagCausalFlow }) {
  const name = (node: DagNodeId) => nameOfDagNode(document, node)
  const status = (path: DagCausalFlow['paths'][number]): { readonly text: string; readonly tone: string } => {
    switch (path.status.kind) {
      case 'causal': return { text: 'directed causal path', tone: 'text-ok' }
      case 'open': return { text: 'open', tone: 'text-danger' }
      case 'open-through-unmeasured': return { text: `open through ${path.status.unmeasured.map(name).join(', ')} (unmeasured)`, tone: 'text-danger' }
      case 'closed-by-adjustment': return { text: `closed by ${path.status.by.map(name).join(', ')}`, tone: 'text-muted' }
      case 'closed-at-collider': return { text: `closed at collider ${path.status.colliders.map(name).join(', ')}; opens if adjusted`, tone: 'text-muted' }
      default: return assertNever(path.status)
    }
  }
  return (
    <ul className="m-0 mt-2 list-none divide-y divide-line border-y border-line p-0 text-body" aria-label="Paths from treatment to outcome">
      {flow.paths.map((path) => {
        const verdict = status(path)
        return (
          <li key={path.nodes.join('>')} className="flex flex-col py-1.5">
            <span className="text-ink">{path.nodes.map(name).join(' – ')}</span>
            <span className={`text-label ${verdict.tone}`}>{path.type === 'causal' ? 'causal path' : 'back-door path'} · {verdict.text}</span>
          </li>
        )
      })}
      {flow.pathCapReached && <li className="py-1.5 text-label text-faint">Only the first 64 paths are listed.</li>}
    </ul>
  )
}

function AdjustmentSentence({ document, flow }: { readonly document: DagDocument; readonly flow: DagCausalFlow }) {
  const name = (node: DagNodeId) => nameOfDagNode(document, node)
  switch (flow.adjustment.kind) {
    case 'unnecessary': return <p className="m-0 text-body text-faint">No back-door path is open. No adjustment variable is needed.</p>
    case 'sufficient': {
      const predictors = flow.adjustment.variables.filter((node) => flow.roles.get(node)?.kind === 'outcome-predictor')
      return (
        <p className="m-0 text-body text-muted">
          Adjusting for <span className="text-ink">{flow.adjustment.variables.map(name).join(', ')}</span> blocks all represented back-door paths.
          {predictors.length > 0 && <> {predictors.map(name).join(', ')} {predictors.length === 1 ? 'is' : 'are'} not required for identification but may improve precision as {predictors.length === 1 ? 'an outcome predictor' : 'outcome predictors'}.</>}
        </p>
      )
    }
    case 'none': return <p className="m-0 text-body text-muted">No measured adjustment set blocks every back-door path under this graph. Front-door, instrumental-variable and other identification strategies are not assessed here; sensitivity analysis does not establish identification.</p>
    default: return assertNever(flow.adjustment)
  }
}

function ValidationPanel({ document, flow, onUseForStudy, onSelectEdge }: {
  readonly document: DagDocument
  readonly flow: DagCausalFlow | null
  readonly onUseForStudy: () => void
  readonly onSelectEdge: (edge: DagEdgeId) => void
}) {
  const plan = useMemo(() => planDagImplications(document), [document])
  const validation = document.current.validation
  const issues = validation.kind === 'structurally-valid' ? [] : validation.issues
  const unstated = issues.flatMap((issue) => (issue.kind === 'missing-rationale' ? [issue.edge] : []))
  const otherIssues = issues.filter((issue) => issue.kind !== 'missing-rationale')
  const heading = validation.kind === 'structurally-valid'
    ? 'Acyclic structure'
    : validation.kind === 'incomplete' && document.current.graph.edges.length > 0
      ? 'Rationale outstanding'
      : 'Draft needs attention'
  return (
    <aside aria-labelledby="dag-validation-title">
      <h3 id="dag-validation-title" className="mb-1 mt-0 text-body font-medium text-ink">Live validation</h3>
      <p className={`m-0 flex items-center gap-1.5 text-body ${validation.kind === 'structurally-valid' ? 'text-ok' : 'text-warn'}`}>
        <Icon name={validation.kind === 'structurally-valid' ? 'check_circle' : 'error'} size={16} />{heading}
      </p>
      {otherIssues.length > 0 && (
        <ul className="mb-0 mt-3 space-y-1.5 pl-4 text-body text-muted">
          {otherIssues.map((issue, index) => <li key={`${issue.kind}:${index}`}>{describeDagStructuralIssue(issue)}</li>)}
        </ul>
      )}
      {unstated.length > 0 && (
        <div className="mt-3 text-body text-muted">
          <p className="m-0">{unstated.length === 1 ? 'This arrow has' : 'These arrows have'} no substantive rationale. Select an arrow to record the mechanism, design evidence, or external evidence supporting it.</p>
          <ul className="mb-0 mt-1.5 flex list-none flex-wrap gap-1 p-0" aria-label="Arrows without a rationale">
            {unstated.map((edgeId) => {
              const edge = document.current.graph.edges.find((candidate) => candidate.id === edgeId)
              return edge === undefined ? null : (
                <li key={edgeId}>
                  <button type="button" className={pill(false, 'normal-case tracking-normal text-warn')} onClick={() => onSelectEdge(edgeId)}>
                    {nameOfDagNode(document, edge.cause)} → {nameOfDagNode(document, edge.effect)}
                  </button>
                </li>
              )
            })}
          </ul>
        </div>
      )}
      {flow !== null && (
        <div className="mt-3 border-t border-hair pt-3" aria-label="Adjustment">
          <span className="block text-label text-faint">Back-door paths</span>
          <div className="mt-1"><AdjustmentSentence document={document} flow={flow} /></div>
          {flow.laggedArrows > 0 && <p className="mb-0 mt-1 text-label text-faint">{flow.laggedArrows} lagged arrow{flow.laggedArrows === 1 ? '' : 's'} left to CausalEffects; this reads the same-period graph.</p>}
          <PathList document={document} flow={flow} />
        </div>
      )}
      {validation.kind === 'structurally-valid' && (
        <button type="button" className={button('signal', 'mt-3')} onClick={onUseForStudy}>Use for study</button>
      )}
      <div className="mt-3 border-t border-hair pt-3 text-body text-muted">
        {plan.kind === 'test' && (
          <details>
            <summary className="text-ink">{plan.implications.length} testable graph implication{plan.implications.length === 1 ? '' : 's'}</summary>
            <ul className="mb-0 mt-2 space-y-1 pl-4 text-label text-faint">
              {plan.implications.map((implication) => (
                <li key={`${implication.x}:${implication.y}:${implication.given.join(',')}`}>
                  {nameOfDagNode(document, implication.x)} ⊥ {nameOfDagNode(document, implication.y)}
                  {implication.given.length > 0 ? ` | ${implication.given.map((node) => nameOfDagNode(document, node)).join(', ')}` : ''}
                </li>
              ))}
            </ul>
            <p className="mb-0 mt-2 text-label text-faint">Kernel conditional-independence tests use α = 0.05 with Holm correction. Rejection provides evidence against the corresponding local-Markov implication; non-rejection does not validate the complete graph.</p>
          </details>
        )}
        {plan.kind === 'not-testable' && (
          <p className="m-0 text-faint">This draft has no testable observed local-Markov implication. The absence of a test does not provide evidence for the graph.</p>
        )}
        {plan.kind === 'requires-lag-aware-validation' && (
          <p className="m-0 text-faint">Lagged implications require the time-series CausalEffects validation route; the cross-sectional Markov checker does not evaluate them.</p>
        )}
      </div>
    </aside>
  )
}

export function DagWorkspace({
  profile,
  prepared,
  discoveryRuns,
  documents,
  onDocumentCreated,
  onDocumentRevised,
  onUseForStudy,
  studyDraft,
  onStudyDraftChanged, source, interventionQueries, onInterventionQuery }: DagWorkspaceProps) {
  const [state, dispatch] = useReducer(stepDagWorkspace, documents, initialState)

  const createDocument = () => {
    if (state.kind !== 'creating') return
    const created = createDagDocument(state.nameDraft, state.origin, prepared, profile, discoveryRuns)
    if (!created.ok) {
      dispatch({ type: 'creation-refused', problem: created.error })
      return
    }
    onDocumentCreated(created.value)
    dispatch({ type: 'document-created', document: created.value.id, latestRun: latestRunId(discoveryRuns) })
  }

  const selectedDocument = state.kind === 'editing'
    ? documents.find((document) => document.id === state.document)
    : undefined

  const selectEdge = (document: DagDocument, edgeId: DagEdgeId | null) => {
    const edge = edgeId === null ? undefined : document.current.graph.edges.find((candidate) => candidate.id === edgeId)
    dispatch({ type: 'edge-selected', edge: edge === undefined ? null : edgeDraft(edge) })
  }

  /** Commit a revision and select its newest edge, so the rationale panel opens on the arrow just drawn. */
  const commitWithNewestEdge = (revised: DagDocument) => {
    onDocumentRevised(revised)
    selectEdge(revised, revised.current.graph.edges.at(-1)?.id ?? null)
  }

  const drawEdge = (document: DagDocument, cause: DagNodeId, effect: DagNodeId): DagEditProblem | null => {
    const revised = reviseDagWithEdge(document, cause, effect, null)
    if (!revised.ok) return revised.error
    commitWithNewestEdge(revised.value)
    return null
  }

  const reconnectEdge = (document: DagDocument, edgeId: DagEdgeId, cause: DagNodeId, effect: DagNodeId): DagEditProblem | null => {
    const original = document.current.graph.edges.find((edge) => edge.id === edgeId)
    if (original === undefined) return { kind: 'unknown-edge', edge: edgeId }
    const revised = reviseDagByReplacingEdge(document, edgeId, cause, effect, null, original.timing)
    if (!revised.ok) return revised.error
    commitWithNewestEdge(revised.value)
    return null
  }

  const addEdge = () => {
    if (state.kind !== 'editing' || selectedDocument === undefined) return
    const matched = selectedEvidenceMatches(selectedDocument, state.selectedCandidate, state.cause, state.effect, state.timing)
    const reference = state.attachSelectedEvidence && matched && state.selectedCandidate !== null
      ? discoveryEvidenceReference(state.selectedCandidate)
      : null
    const revised = reviseDagWithEdge(
      selectedDocument,
      state.cause,
      state.effect,
      state.rationale,
      timingValue(state.timing),
      reference === null ? [] : [reference],
    )
    if (!revised.ok) {
      dispatch({ type: 'edge-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    dispatch({ type: 'edge-saved' })
  }

  const saveSelectedEdgeDetails = (document: DagDocument) => {
    if (state.kind !== 'editing' || state.selectedEdge === null) return
    const revised = reviseDagEdgeDetails(document, state.selectedEdge.edge, state.selectedEdge.rationale, timingValue(state.selectedEdge.timing))
    if (!revised.ok) {
      dispatch({ type: 'selected-edge-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    const original = document.current.graph.edges.findIndex((edge) => edge.id === state.selectedEdge?.edge)
    selectEdge(revised.value, revised.value.current.graph.edges[original]?.id ?? null)
  }

  const confoundEdge = (document: DagDocument, edgeId: DagEdgeId) => {
    const revised = reviseDagWithLatentConfounder(document, edgeId)
    if (!revised.ok) return
    onDocumentRevised(revised.value)
    dispatch({ type: 'edge-selected', edge: null })
  }

  const removeEdge = (document: DagDocument, edgeId: DagEdgeId) => {
    const revised = reviseDagWithoutEdge(document, edgeId)
    if (!revised.ok) {
      dispatch({ type: 'edge-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    dispatch({ type: 'edge-selected', edge: null })
  }

  const moveRevision = (document: DagDocument, direction: 'undo' | 'redo') => {
    const moved = direction === 'undo' ? undoDagRevision(document) : redoDagRevision(document)
    if (!moved.ok) return
    onDocumentRevised(moved.value)
    dispatch({ type: 'edge-selected', edge: null })
  }

  const addLatentVariable = (document: DagDocument) => {
    if (state.kind !== 'editing' || state.latentVariable.kind !== 'adding') return
    const revised = reviseDagWithLatentNode(document, state.latentVariable.name)
    if (!revised.ok) {
      dispatch({ type: 'latent-variable-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    dispatch({ type: 'latent-variable-saved' })
  }

  const removeLatentVariable = (document: DagDocument, node: DagNodeId) => {
    const revised = reviseDagWithoutLatentNode(document, node)
    if (!revised.ok) {
      dispatch({ type: 'latent-variable-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    dispatch({ type: 'latent-variable-saved' })
  }

  const selectedEdge = state.kind === 'editing' && selectedDocument !== undefined && state.selectedEdge !== null
    ? selectedDocument.current.graph.edges.find((edge) => edge.id === state.selectedEdge?.edge) ?? null
    : null
  const selectedEdgeDraft = state.kind === 'editing' && selectedEdge !== null ? state.selectedEdge : null
  const canAttach = state.kind === 'editing' && selectedDocument !== undefined
    && selectedEvidenceMatches(selectedDocument, state.selectedCandidate, state.cause, state.effect, state.timing)
  const boundHere = selectedDocument !== undefined && studyDraft.dagDocument === selectedDocument.id
  const boundTreatment = boundHere && selectedDocument.current.graph.nodes.some((node) => node.id === studyDraft.treatment) ? studyDraft.treatment : null
  const boundOutcome = boundHere && selectedDocument.current.graph.nodes.some((node) => node.id === studyDraft.outcome) ? studyDraft.outcome : null
  const flow = useMemo(
    () => (selectedDocument !== undefined && boundTreatment !== null && boundOutcome !== null && boundTreatment !== boundOutcome
      ? analyseDagCausalFlow(selectedDocument.current.graph, boundTreatment, boundOutcome)
      : null),
    [boundOutcome, boundTreatment, selectedDocument],
  )
  const bind = (part: 'treatment' | 'outcome', node: DagNodeId | null) => {
    if (selectedDocument === undefined) return
    const base = boundHere ? studyDraft : { ...EMPTY_STUDY_DRAFT, dagDocument: selectedDocument.id }
    onStudyDraftChanged({ ...base, dagDocument: selectedDocument.id, [part]: node })
  }
  const [inspectorTab, setInspectorTab] = useState<'selection' | 'evidence' | 'intervene'>(discoveryRuns.length > 0 ? 'evidence' : 'selection')
  const [interventionOverlay, setInterventionOverlay] = useState<InterventionOverlay | null>(null)
  useEffect(() => {
    if (selectedEdge !== null) setInspectorTab('selection')
  }, [selectedEdge])

  const header = (
    <div className="mb-3">
      <div>
        <span className={label('text-signal')}>04 · DAG workspace</span>
        <h2 id="dag-workspace-title" className="mb-2 mt-2 text-heading text-ink">Build the causal model</h2>
        <p className="m-0 max-w-[65ch] text-body text-muted">Draw the arrows supported by theory, prior evidence, institutional knowledge, and the treatment-assignment mechanism. Review discovery results alongside this evidence.</p>
      </div>
    </div>
  )

  if (state.kind === 'creating' || selectedDocument === undefined) {
    return (
      <WorkbenchLayout
        id="dag"
        stage={(
          <section aria-labelledby="dag-workspace-title">
            {header}
            {state.kind === 'creating' ? (
              <section className="rounded-xl border border-hair bg-panel p-4" aria-labelledby="dag-origin-title">
                <h3 id="dag-origin-title" className="mb-2 mt-0 text-title font-medium text-ink">Graph basis</h3>
                <div className="grid gap-3 @md/panel:grid-cols-3">
                  <OriginChoice active={state.origin === 'domain-knowledge'} icon="psychology" title="Substantive knowledge" detail="Theory, prior studies, expert knowledge, institutions, and the treatment-assignment process." onClick={() => dispatch({ type: 'origin-selected', origin: 'domain-knowledge' })} />
                  <OriginChoice active={state.origin === 'experimental-design'} icon="experiment" title="Experimental design" detail="The randomisation protocol, intervention timing, and measurement design." onClick={() => dispatch({ type: 'origin-selected', origin: 'experimental-design' })} />
                  <OriginChoice active={state.origin === 'discovery-informed'} disabled={discoveryRuns.length === 0} icon="schema" title="Discovery-informed" detail={discoveryRuns.length === 0 ? 'No discovery results are available for review.' : `Evaluate candidate relations from ${discoveryRuns.length} discovery run${discoveryRuns.length === 1 ? '' : 's'} against substantive knowledge.`} onClick={() => dispatch({ type: 'origin-selected', origin: 'discovery-informed' })} />
                </div>
                <label className="mt-4 block max-w-xl text-body font-medium text-ink">
                  DAG name
                  <input className={field('text', 'mt-1')} value={state.nameDraft} onChange={(event) => dispatch({ type: 'name-changed', value: event.target.value })} placeholder="For example: assignment mechanism and road fatalities" />
                </label>
                {state.problem !== null && <Alert tone="danger" className="mt-3"><p className="m-0">{describeDagCreateProblem(state.problem)}</p></Alert>}
                <button type="button" className={button('signal', 'mt-4')} onClick={createDocument}>Create DAG draft</button>
              </section>
            ) : (
              <Alert tone="danger"><p className="m-0">The selected DAG document is no longer available.</p></Alert>
            )}
          </section>
        )}
      />
    )
  }

  const document = selectedDocument
  const stage = (
    <section aria-labelledby="dag-workspace-title" className="@container/panel flex h-full min-h-0 flex-col">
      {header}
      {documents.length > 1 && (
        <div className="mb-3 flex flex-wrap gap-1 self-start rounded-lg border border-hair bg-well p-1" aria-label="DAG documents">
          {documents.map((candidate) => <button key={candidate.id} type="button" className={segment(candidate.id === document.id)} aria-pressed={candidate.id === document.id} onClick={() => dispatch({ type: 'document-selected', document: candidate.id, latestRun: latestRunId(discoveryRuns) })}>{candidate.name}</button>)}
        </div>
      )}
      <div className="mb-3 flex flex-wrap items-end justify-between gap-3">
        <div>
          <h3 className="mb-1 mt-0 text-title font-medium text-ink">{document.name}</h3>
          <p className="m-0 text-label text-faint">Active revision {document.history.length + 1} of {document.history.length + document.future.length + 1} · {document.audit.length} retained · {describeDagOrigin(document.origin)}</p>
        </div>
        <div className="flex flex-wrap items-center gap-2" role="toolbar" aria-label="DAG actions">
          <div className="flex overflow-hidden rounded-lg border border-hair bg-panel" aria-label="DAG revision controls">
            <button type="button" disabled={document.history.length === 0} className={button('quiet', 'rounded-none border-0')} onClick={() => moveRevision(document, 'undo')} aria-label="Undo DAG revision" title="Undo DAG revision"><Icon name="undo" size={15} /></button>
            <button type="button" disabled={document.future.length === 0} className={button('quiet', 'rounded-none border-0 border-l border-hair')} onClick={() => moveRevision(document, 'redo')} aria-label="Redo DAG revision" title="Redo DAG revision"><Icon name="redo" size={15} /></button>
          </div>
          <button type="button" className={button('quiet', 'inline-flex items-center gap-1.5')} onClick={() => dispatch({ type: 'latent-variable-add-requested' })}><Icon name="add" size={14} /> Unmeasured variable</button>
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'new-document-requested' })}>Create a DAG</button>
        </div>
      </div>
      <div className="mb-3 flex flex-wrap items-center gap-x-4 gap-y-2" role="group" aria-label="Study binding">
        <label className="flex items-center gap-2 text-body text-ink">Treatment
            <Select aria-label="Treatment" className={field('text', 'w-40')} value={boundTreatment ?? ''} onChange={(event) => bind('treatment', event.target.value === '' ? null : (event.target.value as DagNodeId))}>
              <option value="">Choose</option>
              {document.current.graph.nodes.filter((node) => node.kind === 'observed').map((node) => <option key={node.id} value={node.id} disabled={node.id === boundOutcome}>{node.name}</option>)}
            </Select>
        </label>
        <label className="flex items-center gap-2 text-body text-ink">Outcome
            <Select aria-label="Outcome" className={field('text', 'w-40')} value={boundOutcome ?? ''} onChange={(event) => bind('outcome', event.target.value === '' ? null : (event.target.value as DagNodeId))}>
              <option value="">Choose</option>
              {document.current.graph.nodes.filter((node) => node.kind === 'observed').map((node) => <option key={node.id} value={node.id} disabled={node.id === boundTreatment}>{node.name}</option>)}
            </Select>
        </label>
      </div>
      {state.latentVariable.kind === 'adding' && (
        <form className="mb-3 flex flex-wrap items-end gap-2 rounded-lg border border-hair bg-well p-3" onSubmit={(event) => { event.preventDefault(); addLatentVariable(document) }}>
          <label className="min-w-[14rem] flex-1 text-body text-ink">Unmeasured variable name<input autoFocus className={field('text', 'mt-1')} value={state.latentVariable.name} onChange={(event) => dispatch({ type: 'latent-variable-name-changed', value: event.target.value })} onKeyDown={(event) => { if (event.key === 'Escape') dispatch({ type: 'latent-variable-edit-cancelled' }) }} /></label>
          <button type="submit" className={button('outline')}>Add to DAG</button>
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'latent-variable-edit-cancelled' })}>Cancel</button>
          {state.latentVariable.problem !== null && <p role="alert" className="m-0 w-full text-body text-danger">{describeDagVariableEditProblem(state.latentVariable.problem)}</p>}
        </form>
      )}
      {state.latentVariable.kind === 'closed' && state.latentVariable.problem !== null && <p role="alert" className="mb-3 mt-0 text-body text-danger">{describeDagVariableEditProblem(state.latentVariable.problem)}</p>}
      {document.current.graph.nodes.some((node) => node.kind === 'latent') && (
        <div className="mb-3 flex flex-wrap gap-2" aria-label="Unmeasured DAG variables">
          {document.current.graph.nodes.filter((node) => node.kind === 'latent').map((node) => (
            <span key={node.id} className="inline-flex items-center overflow-hidden rounded-lg border border-dashed border-edge bg-well text-label text-muted">
              <span className="px-2.5 py-1.5">{node.name}</span>
              <button type="button" className={iconControl('danger', 'rounded-none border-0 border-l border-hair')} aria-label={`Remove unmeasured variable ${node.name}`} title="Remove after its incident edges are removed" onClick={() => removeLatentVariable(document, node.id)}><Icon name="close" size={13} /></button>
            </span>
          ))}
        </div>
      )}
      <DagCanvas
        document={document}
        selectedEvidence={state.selectedCandidate}
        selectedEdge={selectedEdge?.id ?? null}
        flow={flow}
        intervention={inspectorTab === 'intervene' ? interventionOverlay : null}
        onConnectionDrawn={(cause, effect) => drawEdge(document, cause, effect)}
        onEdgeReconnected={(edge, cause, effect) => reconnectEdge(document, edge, cause, effect)}
        onEdgeConfounded={(edge) => confoundEdge(document, edge)}
        onEdgeRemoved={(edge) => removeEdge(document, edge)}
        onEdgeSelected={(edge) => selectEdge(document, edge)}
      />

    </section>
  )

  const addEdgeForm = (
    <section className="border-t border-hair pt-4" aria-labelledby="add-edge-title">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <h3 id="add-edge-title" className="m-0 text-body font-medium text-ink">Add an arrow</h3>
        {(state.cause !== null || state.effect !== null || state.rationale.length > 0) && <button type="button" className="text-label text-muted hover:text-ink" onClick={() => dispatch({ type: 'edge-draft-cancelled' })}>Clear the draft</button>}
      </div>
      <div className="mt-2 grid gap-2">
        <label className="min-w-0 text-body text-ink"><span className="sr-only">Proposed cause</span><Select aria-label="Proposed cause" className={field('text')} value={state.cause ?? ''} onChange={(event) => dispatch({ type: 'cause-selected', node: nodeFromValue(document, event.target.value) })}><option value="">Cause</option>{document.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}</Select></label>
        <label className="min-w-0 text-body text-ink"><span className="sr-only">Proposed effect</span><Select aria-label="Proposed effect" className={field('text')} value={state.effect ?? ''} onChange={(event) => dispatch({ type: 'effect-selected', node: nodeFromValue(document, event.target.value) })}><option value="">Effect</option>{document.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}</Select></label>
        {document.dataset.kind === 'time-series' ? (
          <Select aria-label="Timing" className={field('text')} value={state.timing.kind} onChange={(event) => dispatch({ type: 'timing-selected', timing: event.target.value === 'lagged' ? { kind: 'lagged', lag: '1' } : { kind: 'contemporaneous' } })}><option value="contemporaneous">Contemporaneous · t</option><option value="lagged">Past cause · t−lag</option></Select>
        ) : null}
        {document.dataset.kind === 'time-series' && state.timing.kind === 'lagged'
          ? <input aria-label="Lag" className={field('text')} type="number" min={1} max={Math.max(1, document.dataset.observations - 1)} value={state.timing.lag} onChange={(event) => dispatch({ type: 'lag-changed', value: event.target.value })} />
          : null}
      </div>
      <div className="mt-2 grid gap-2">
        <input aria-label="Rationale for proposed arrow" className={field('text')} value={state.rationale} onChange={(event) => dispatch({ type: 'rationale-changed', value: event.target.value })} placeholder="Record the mechanism, assignment rule, protocol, prior study, or expert evidence." />
        <button type="button" className={button('signal', 'inline-flex items-center justify-center gap-1.5')} onClick={addEdge}><Icon name="add" size={16} /> Add the arrow</button>
      </div>
      {canAttach && <label className="mt-2 flex items-start gap-2 text-body text-muted"><input type="checkbox" checked={state.attachSelectedEvidence} onChange={(event) => dispatch({ type: 'evidence-attachment-changed', attached: event.target.checked })} /><span>Attach the selected discovery result to this user-authored edge.</span></label>}
      {state.problem !== null && <p role="alert" className="mb-0 mt-2 text-body text-danger">{describeDagEditProblem(state.problem)}</p>}
    </section>
  )

  const tab = (id: 'selection' | 'evidence' | 'intervene', icon: string, text: string) => (
    <button
      type="button"
      onClick={() => setInspectorTab(id)}
      aria-pressed={inspectorTab === id}
      className={cn('flex items-center gap-1.5 rounded-md px-2 py-1 text-label transition-colors', inspectorTab === id ? 'bg-raised text-ink' : 'text-muted hover:text-ink')}
    >
      <Icon name={icon} size={14} />
      {text}
    </button>
  )

  const inspector = (
    <div className="flex flex-col gap-4">
      <ValidationPanel document={document} flow={flow} onUseForStudy={() => { if (!boundHere) onStudyDraftChanged({ ...EMPTY_STUDY_DRAFT, dagDocument: document.id }); onUseForStudy() }} onSelectEdge={(edge) => { setInspectorTab('selection'); selectEdge(document, edge) }} />
      {inspectorTab === 'selection' && (
        selectedEdge !== null && selectedEdgeDraft !== null ? (
          <aside className="border-t border-hair pt-4" aria-labelledby="selected-edge-title">
            <h3 id="selected-edge-title" className="mb-1 mt-0 text-body font-medium text-ink">{nameOfDagNode(document, selectedEdge.cause)} → {nameOfDagNode(document, selectedEdge.effect)}</h3>
            <p className={`m-0 text-body ${selectedEdge.support.kind === 'unstated' ? 'text-warn' : 'text-faint'}`}>{describeTiming(selectedEdge.timing)} · {describeSupport(selectedEdge.support)}</p>
            <form className="mt-3" onSubmit={(event) => { event.preventDefault(); saveSelectedEdgeDetails(document) }}>
              {document.dataset.kind === 'time-series' && (
                <div className="grid gap-3 @sm/inspector:grid-cols-2">
                  <label className="text-body text-ink">Arrow timing<Select className={field('text', 'mt-1')} value={selectedEdgeDraft.timing.kind} onChange={(event) => dispatch({ type: 'selected-edge-timing-selected', timing: event.target.value === 'lagged' ? { kind: 'lagged', lag: '1' } : { kind: 'contemporaneous' } })}><option value="contemporaneous">Contemporaneous · t</option><option value="lagged">Past cause · t−lag</option></Select></label>
                  {selectedEdgeDraft.timing.kind === 'lagged' && <label className="text-body text-ink">Arrow lag<input className={field('text', 'mt-1')} type="number" min={1} max={Math.max(1, document.dataset.observations - 1)} value={selectedEdgeDraft.timing.lag} onChange={(event) => dispatch({ type: 'selected-edge-lag-changed', value: event.target.value })} /></label>}
                </div>
              )}
              <label className="mt-3 block text-body text-ink">Rationale<textarea className={field('text', 'mt-1 min-h-20 resize-y')} value={selectedEdgeDraft.rationale} onChange={(event) => dispatch({ type: 'selected-edge-rationale-changed', value: event.target.value })} placeholder="Record the mechanism, assignment rule, protocol, prior study, or expert evidence supporting this arrow." /></label>
              <p className={literal('mb-0 mt-2 text-micro text-faint')}>{selectedEdge.evidence.length} attached discovery item{selectedEdge.evidence.length === 1 ? '' : 's'}</p>
              {selectedEdgeDraft.problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeDagEditProblem(selectedEdgeDraft.problem)}</p>}
              <div className="mt-3 flex flex-wrap gap-2">
                <button type="submit" className={button('signal')}>Save the arrow</button>
                <button type="button" className={button('danger')} onClick={() => removeEdge(document, selectedEdge.id)}>Remove the arrow</button>
              </div>
            </form>
          </aside>
        ) : (
          <>
            {addEdgeForm}
            <p className="m-0 text-body text-faint">Or drag from a variable’s handle onto another variable. Select an arrow to record its rationale, change its timing, reverse it, replace it with an unmeasured cause, or remove it.</p>
          </>
        )
      )}
      {inspectorTab === 'intervene' && (
        <InterventionPanel
          document={document}
          source={source}
          profile={profile}
          prepared={prepared}
          queries={interventionQueries}
          onQuery={onInterventionQuery}
          onOverlay={setInterventionOverlay}
        />
      )}
      {inspectorTab === 'evidence' && (
        <EvidenceInspector
          runs={discoveryRuns}
          selectedRun={state.selectedRun}
          selectedCandidate={state.selectedCandidate}
          onRunSelected={(run) => dispatch({ type: 'run-selected', run })}
          onCandidateSelected={(candidate) => dispatch({ type: 'candidate-selected', candidate })}
        />
      )}
    </div>
  )

  const ledger = (
    <EdgeLedgerTable
      document={document}
      selectedEdge={selectedEdge?.id ?? null}
      onSelectEdge={(edge) => { setInspectorTab('selection'); selectEdge(document, edge) }}
    />
  )

  return (
    <WorkbenchLayout
      id="dag"
      stage={stage}
      stageScroll={false}
      inspector={{
        title: 'Inspector',
        controls: <div className="flex gap-1">{tab('selection', 'ads_click', 'Selection')}{tab('evidence', 'schema', 'Evidence')}{tab('intervene', 'bolt', 'Intervene')}</div>,
        body: inspector,
      }}
      bottom={{ title: `Arrows · ${document.current.graph.edges.length}`, body: ledger, defaultSize: 150 }}
    />
  )
}
