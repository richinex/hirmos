import { DisclosureSummary } from '@/components/ui/DisclosureSummary'
import { Metadata } from '@/components/ui/Metadata'
import { ChapterHeading } from '@/components/ui/ChapterHeading'
import { ParameterHelp } from '@/components/ui/ParameterLabel'
import { Orb } from '@/components/ui/Orb'
import { Alert } from '@/components/ui/Alert'
import { Select } from '@/components/ui/Select'
import { useEffect, useMemo, useReducer, useState } from 'react'
import { WorkbenchLayout } from '@/components/shell/WorkbenchLayout'
import { InterventionPanel } from './InterventionPanel'
import type { InterventionOverlay, InterventionQueryArtifact } from '@/domain/intervention'
import type { SelectedSource } from '@/domain/workflow'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { useJob } from '@/analysis/JobsProvider'
import { JobNotice } from '@/components/ui/JobNotice'
import { cn } from '@/lib/utils'
import { Icon } from '@/components/Icon'
import { button, chapterIntro, field, fieldLabel, iconControl, literal, panel, pill, sectionTitle, well } from '@/components/ui/recipes'
import { SegmentedControl } from '@/components/ui/SegmentedControl'
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
  reviseDagWithImportedGraph,
  reviseDagWithLatentNode,
  reviseDagWithEdge,
  reviseDagWithoutLatentNode,
  reviseDagWithoutEdge,
  undoDagRevision,
  type DagCreateProblem,
  type DagImportProblem,
  type DagDocument,
  type DagDocumentId,
  type DagEditProblem,
  type DagEdgeId,
  type DagNodeId,
  type DagOriginDraft,
  type DagVariableEditProblem,
  type DirectedDagEdge,
  type EdgeSupport,
  type EdgeTiming, } from '@/domain/dag'
import { describeDagImportProblem, importOf, planDagImport, type DagImportPlanProblem } from '@/domain/dagImport'
import {
  discoveryEvidenceReference,
  discoveryEvidenceView,
  type DiscoveryCandidate,
} from '@/domain/dagEvidence'
import {
  describeDagStructuralIssue,
  planDagImplications,
  recordDagCheck,
  type DagCheckArtifact,
} from '@/domain/dagValidation'
import type { ColumnId, DatasetProfile } from '@/domain/dataset'
import type { DiscoveryRunArtifact, DiscoveryRunId } from '@/domain/discovery'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import { methodDefinition } from '@/domain/methods'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'
import { formatTime } from '@/lib/format/date'
import { DagCanvas } from './DagCanvas'
import { EdgeLedgerTable } from './EdgeLedgerTable'
import { analyseDagCausalFlow, type DagCausalFlow } from '@/domain/dagFlow'
import { EMPTY_STUDY_DRAFT, type StudyDesignDraft } from '@/domain/study'
import { EvidenceInspector } from './EvidenceInspector'

import { prepareRootCauseGraph, type RootCauseSelection } from '@/domain/rootCause'

interface DagWorkspaceProps {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly interventionQueries: readonly InterventionQueryArtifact[]
  readonly onInterventionQuery: (query: InterventionQueryArtifact) => void
  readonly discoveryRuns: readonly DiscoveryRunArtifact[]
  readonly documents: readonly DagDocument[]
  readonly checks: readonly DagCheckArtifact[]
  readonly onDocumentCreated: (document: DagDocument) => void
  readonly onDocumentRevised: (document: DagDocument) => void
  readonly onCheck: (check: DagCheckArtifact) => void
  /** Opens Study Design; offered only on a structurally valid revision. */
  readonly onUseForStudy: () => void
  readonly onUseForRootCause: (selection: RootCauseSelection) => void
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

type PasteProblem = DagImportPlanProblem | DagImportProblem

type PasteDraft =
  | { readonly kind: 'closed'; readonly problem: PasteProblem | null }
  | { readonly kind: 'open'; readonly text: string; readonly problem: PasteProblem | null }

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
      readonly origin: DagOriginDraft
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
      readonly paste: PasteDraft
    }

type DagWorkspaceEvent =
  | { readonly type: 'new-document-requested' }
  | { readonly type: 'name-changed'; readonly value: string }
  | { readonly type: 'origin-selected'; readonly origin: DagOriginDraft['kind'] }
  | { readonly type: 'discovery-origin-run-toggled'; readonly run: DiscoveryRunId }
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
  | { readonly type: 'paste-requested' }
  | { readonly type: 'paste-text-changed'; readonly value: string }
  | { readonly type: 'paste-cancelled' }
  | { readonly type: 'paste-refused'; readonly problem: PasteProblem }
  | { readonly type: 'paste-applied' }

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
  paste: { kind: 'closed', problem: null },
})

const latestRunId = (runs: readonly DiscoveryRunArtifact[]): DiscoveryRunId | null => runs.at(-1)?.id ?? null

const initialState = (documents: readonly DagDocument[]): DagWorkspaceState => {
  const [first] = documents
  return first === undefined
    ? { kind: 'creating', nameDraft: '', origin: { kind: 'domain-knowledge' }, problem: null }
    : editingState(first.id, null)
}

const emptyOrigin = (kind: DagOriginDraft['kind']): DagOriginDraft => {
  switch (kind) {
    case 'domain-knowledge': return { kind: 'domain-knowledge' }
    case 'experimental-design': return { kind: 'experimental-design' }
    case 'discovery-informed': return { kind: 'discovery-informed', reports: [] }
    default: return assertNever(kind)
  }
}

const withSelectedEdge = (
  state: DagWorkspaceState,
  update: (draft: SelectedEdgeDraft) => SelectedEdgeDraft,
): DagWorkspaceState => state.kind === 'editing' && state.selectedEdge !== null
  ? { ...state, selectedEdge: update(state.selectedEdge) }
  : state

function stepDagWorkspace(state: DagWorkspaceState, event: DagWorkspaceEvent): DagWorkspaceState {
  switch (event.type) {
    case 'new-document-requested': return { kind: 'creating', nameDraft: '', origin: { kind: 'domain-knowledge' }, problem: null }
    case 'name-changed': return state.kind === 'creating' ? { ...state, nameDraft: event.value, problem: null } : state
    case 'origin-selected': return state.kind === 'creating' ? { ...state, origin: emptyOrigin(event.origin), problem: null } : state
    case 'discovery-origin-run-toggled': {
      if (state.kind !== 'creating' || state.origin.kind !== 'discovery-informed') return state
      const reports = state.origin.reports.includes(event.run)
        ? state.origin.reports.filter((run) => run !== event.run)
        : [...state.origin.reports, event.run]
      return { ...state, origin: { kind: 'discovery-informed', reports }, problem: null }
    }
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
    case 'paste-requested': return state.kind === 'editing'
      ? { ...state, paste: { kind: 'open', text: '', problem: null } }
      : state
    case 'paste-text-changed': return state.kind === 'editing' && state.paste.kind === 'open'
      ? { ...state, paste: { ...state.paste, text: event.value, problem: null } }
      : state
    case 'paste-cancelled': return state.kind === 'editing'
      ? { ...state, paste: { kind: 'closed', problem: null } }
      : state
    case 'paste-refused': return state.kind === 'editing'
      ? { ...state, paste: state.paste.kind === 'open'
          ? { ...state.paste, problem: event.problem }
          : { kind: 'closed', problem: event.problem } }
      : state
    case 'paste-applied': return state.kind === 'editing'
      ? { ...state, paste: { kind: 'closed', problem: null } }
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
      <span className={well('mb-3 grid h-9 w-9 place-items-center text-muted')}><Icon name={icon} size={18} /></span>
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
    <ul className="m-0 mt-2 list-none divide-y divide-line border-t border-line p-0 text-body" aria-label="Paths from treatment to outcome">
      {flow.paths.map((path) => {
        const verdict = status(path)
        return (
          <li key={path.nodes.join('>')} className="flex flex-col py-1.5">
            <span className="text-ink">{path.nodes.map(name).join(' – ')}</span>
            <span className={`text-label ${verdict.tone}`}><Metadata><span>{path.type === 'causal' ? 'causal path' : 'back-door path'}</span><span>{verdict.text}</span></Metadata></span>
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
    case 'unnecessary': return <p data-adjustment="unnecessary" className="m-0 text-body text-faint">No back-door path is open in the same-period graph, so no adjustment variable is needed for that graph.</p>
    case 'sufficient': {
      const predictors = flow.adjustment.variables.filter((node) => flow.roles.get(node)?.kind === 'outcome-predictor')
      return (
        <p data-adjustment="sufficient" className="m-0 text-body text-muted">
          Adjusting for <span data-adjustment-variables className="text-ink">{flow.adjustment.variables.map(name).join(', ')}</span> blocks all represented back-door paths.
          {predictors.length > 0 && <> {predictors.map(name).join(', ')} {predictors.length === 1 ? 'is' : 'are'} not required for identification but may improve precision as {predictors.length === 1 ? 'an outcome predictor' : 'outcome predictors'}.</>}
        </p>
      )
    }
    case 'none': return <p data-adjustment="none" className="m-0 text-body text-muted">No measured adjustment set blocks every back-door path in the same-period graph. This check does not assess front-door, instrumental variable, or other identification strategies. Sensitivity analysis alone does not establish identification.</p>
    default: return assertNever(flow.adjustment)
  }
}

function ValidationPanel({ document, flow, onUseForStudy, onUseForRootCause, onSelectEdge }: {
  readonly document: DagDocument
  readonly flow: DagCausalFlow | null
  readonly onUseForStudy: () => void
  readonly onUseForRootCause: (() => void) | null
  readonly onSelectEdge: (edge: DagEdgeId) => void
}) {
  const plan = useMemo(() => planDagImplications(document), [document])
  const { structure, rationales } = document.current.validation
  const sound = structure.kind === 'sound'
  const unstated = rationales.kind === 'outstanding' ? rationales.edges : []
  const heading = sound
    ? rationales.kind === 'complete' ? 'Acyclic structure' : 'Rationale outstanding'
    : 'Draft needs attention'
  return (
    <aside aria-labelledby="dag-validation-title">
      <h3 id="dag-validation-title" className="mb-1 mt-0 text-body font-medium text-ink">Live validation</h3>
      <p className={`m-0 flex items-center gap-1.5 text-body ${sound ? 'text-ok' : 'text-warn'}`}>
        <Icon name={sound ? 'check_circle' : 'error'} size={16} />{heading}
      </p>
      {structure.kind === 'empty' && <p className="mb-0 mt-3 text-body text-muted">Add at least one arrow.</p>}
      {structure.kind === 'invalid' && (
        <ul className="mb-0 mt-3 space-y-1.5 pl-4 text-body text-muted">
          {structure.issues.map((issue, index) => <li key={`${issue.kind}:${index}`}>{describeDagStructuralIssue(issue)}</li>)}
        </ul>
      )}
      {unstated.length > 0 && (
        <div className="mt-3 text-body text-muted">
          <p className="m-0">{unstated.length === 1 ? 'This arrow has' : 'These arrows have'} no recorded substantive rationale. Select an arrow to document the supporting mechanism, design evidence, or external sources.</p>
          <ul className="mb-0 mt-1.5 flex list-none flex-wrap gap-1 p-0" aria-label="Arrows without a rationale">
            {unstated.map((edgeId) => {
              const edge = document.current.graph.edges.find((candidate) => candidate.id === edgeId)
              return edge === undefined ? null : (
                <li key={edgeId}>
                  <button type="button" className={pill(false, 'text-warn')} onClick={() => onSelectEdge(edgeId)}>
                    {nameOfDagNode(document, edge.cause)} → {nameOfDagNode(document, edge.effect)}
                  </button>
                </li>
              )
            })}
          </ul>
        </div>
      )}
      {flow !== null && (
        <div className="mt-6" aria-label="Adjustment">
          <span className="block text-label text-faint">Back-door paths</span>
          <div className="mt-1"><AdjustmentSentence document={document} flow={flow} /></div>
          {flow.laggedArrows > 0 && <p className="mb-0 mt-1 text-label text-faint">This back-door check considers only same-period arrows. It excludes {flow.laggedArrows} lagged arrow{flow.laggedArrows === 1 ? '' : 's'}.</p>}
          <PathList document={document} flow={flow} />
        </div>
      )}
      <div role="group" aria-label="Use this graph" className="mt-3 grid grid-cols-1 auto-rows-fr gap-3 empty:hidden">
        {sound && (
          <button type="button" className={button('outline')} onClick={onUseForStudy}>Use for study</button>
        )}
        {onUseForRootCause !== null && <button type="button" className={button('outline')} onClick={onUseForRootCause}>Use for causal model analysis</button>}
      </div>
      <div className="mt-6 text-body text-muted">
        {plan.kind === 'test' && (
          <details>
            <DisclosureSummary className="text-ink">{plan.implications.length} testable graph implication{plan.implications.length === 1 ? '' : 's'}</DisclosureSummary>
            <ul className="mb-0 mt-2 space-y-1 pl-4 text-label text-faint">
              {plan.implications.map((implication) => (
                <li key={`${implication.x}:${implication.y}:${implication.given.join(',')}`}>
                  {nameOfDagNode(document, implication.x)} ⊥ {nameOfDagNode(document, implication.y)}
                  {implication.given.length > 0 ? ` | ${implication.given.map((node) => nameOfDagNode(document, node)).join(', ')}` : ''}
                </li>
              ))}
            </ul>
            <p className="mb-0 mt-2 text-label text-faint">Kernel conditional-independence tests use α = 0.05 with Holm correction. Rejection provides evidence against the corresponding local-Markov implication. Failure to reject does not validate the entire graph.</p>
          </details>
        )}
        {plan.kind === 'not-testable' && (
          <p className="m-0 text-faint">This draft has no testable observed local-Markov implication. The absence of a test does not provide evidence for the graph.</p>
        )}
        {plan.kind === 'requires-lag-aware-validation' && (
          <p className="m-0 text-faint">This checker does not test lagged implications. They require time-series tests that account for the recorded lags.</p>
        )}
      </div>
    </aside>
  )
}

function GraphCheckPanel({ source, profile, prepared, document, checks, onCheck }: {
  readonly source: SelectedSource
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly document: DagDocument
  readonly checks: readonly DagCheckArtifact[]
  readonly onCheck: (check: DagCheckArtifact) => void
}) {
  const session = useJob(`graph-check:${document.current.id}`)
  const { job } = session
  const plan = useMemo(() => planDagImplications(document), [document])
  const current = [...checks].reverse().find((check) => check.dagDocument === document.id && check.dagRevision === document.current.id)
  const nodeName = (position: number): string => document.current.graph.nodes.filter((node) => node.kind === 'observed')[position]?.name ?? `Variable ${position + 1}`

  const run = async () => {
    if (plan.kind !== 'test') return
    const execution = session.start('checks', 'Testing graph implications')
    if (execution === null) return
    const fail = (detail: string) => session.fail(execution, detail)
    try {
    const observed = document.current.graph.nodes.filter((node) => node.kind === 'observed')
    if (observed.length < 2) {
      fail('At least two observed variables are required.')
      return
    }
    const [{ materialisePrepared, describePreparedMaterialisationProblem }, { runDagCheck }] = await Promise.all([
      import('@/data/prepared'),
      import('@/analysis/client'),
    ])
    if (!session.current(execution)) return
    const columns = observed.map((node) => node.column) as unknown as NonEmptyArray<ColumnId>
    const matrix = await materialisePrepared(source, profile, prepared, columns)
    if (!session.current(execution)) return
    if (!matrix.ok) {
      fail(describePreparedMaterialisationProblem(matrix.error))
      return
    }
    const positions = new Map(observed.map((node, index) => [node.id, index] as const))
    const edgePositions = document.current.graph.edges.flatMap((edge) => {
      if (edge.timing.kind !== 'contemporaneous') return []
      const cause = positions.get(edge.cause)
      const effect = positions.get(edge.effect)
      return cause === undefined || effect === undefined ? [] : [[cause, effect] as const]
    })
    const implications = plan.implications.flatMap((implication) => {
      const x = positions.get(implication.x)
      const y = positions.get(implication.y)
      const given = implication.given.flatMap((node) => {
        const position = positions.get(node)
        return position === undefined ? [] : [position]
      })
      return x === undefined || y === undefined || given.length !== implication.given.length
        ? []
        : [{ x, y, given }]
    })
    if (implications.length !== plan.implications.length) {
      fail('A conditional-independence test requires a variable that is not included in the observed data selected for this check.')
      return
    }
    const result = await runDagCheck(
      Float64Array.from(matrix.value.values),
      matrix.value.rowCount,
      observed.length,
      {
        nodeColumns: observed.map((_, index) => index),
        edges: edgePositions,
        implications,
        maximumObservations: plan.maxObservationsPerTest,
        permutations: 200,
        significanceLevel: plan.significanceLevel,
        runFalsification: document.current.graph.nodes.every((node) => node.kind === 'observed'),
      },
      (progress) => session.progress(execution, progress.stage === 'dag-permutations' ? 'Comparing relabeled graphs' : 'Testing graph implications', progress),
    )
    if (!session.current(execution)) return
    if (!result.ok) {
      fail(describeAnalysisWorkerProblem(result.error))
      return
    }
    onCheck(recordDagCheck(document, result.value))
    session.finish(execution)
    } catch (cause: unknown) {
      fail(cause instanceof Error ? cause.message : String(cause))
    }
  }

  const evidence = current?.evidence
  const contradictions = evidence?.implications.filter((implication) => implication.decision === 'contradicted').length ?? 0
  const systematicTension = evidence !== undefined && evidence.uniformity.pValue < evidence.significanceLevel
  const progressText = job.kind === 'running' && job.progress !== null
    ? `${job.stage}, ${job.progress.completed} of ${job.progress.total}`
    : 'Testing graph implications…'

  return (
    <section className="pt-4" aria-labelledby="graph-check-title">
      <div className="flex flex-wrap items-start justify-between gap-2">
        <div>
          <h3 id="graph-check-title" className="m-0 text-body font-medium text-ink">Graph checks</h3>
          <p className="mb-0 mt-1 text-label text-faint">Test the conditional independences implied by this revision against the prepared data.</p>
        </div>
      </div>
      {plan.kind === 'test' && (
        <div className="mt-3 flex flex-wrap items-center gap-3">
          <button type="button" className={button('outline', 'w-full')} disabled={job.kind === 'running' || session.blocked} aria-busy={job.kind === 'running'} onClick={() => void run()}>
            {current === undefined ? 'Run checks' : 'Run again'}
          </button>
          {job.kind === 'running' && <>
            <Orb state="weaving" aria-label="Graph checks running" />
            <button type="button" className={button('quiet')} onClick={session.cancel}>Cancel run</button>
            <p role="status" className="m-0 basis-full text-label text-muted">{progressText}</p>
          </>}
        </div>
      )}
      <JobNotice job={job} />
      {plan.kind === 'not-testable' && <p className="mb-0 mt-2 text-body text-faint">No observed local-Markov implication is available to test for this revision.</p>}
      {plan.kind === 'requires-lag-aware-validation' && <p className="mb-0 mt-2 text-body text-faint">This checker cannot test the graph’s lagged implications. Time-series tests must account for the recorded lags.</p>}
      {evidence !== undefined && (
        <div className="mt-3">
          <p className={`m-0 text-body font-medium ${contradictions > 0 || systematicTension || (evidence.falsification.kind === 'completed' && evidence.falsification.falsified) ? 'text-danger' : 'text-ink'}`}>
            {contradictions > 0
              ? `${contradictions} of ${evidence.implications.length} graph implications contradicted`
              : systematicTension
                ? 'The distribution of implication p-values is inconsistent with this graph'
              : evidence.falsification.kind === 'completed' && evidence.falsification.falsified
                ? 'Permutation falsification rejects this graph'
                : evidence.falsification.kind === 'completed' && !evidence.falsification.falsifiable
                  ? 'The permutation comparison is not informative for this graph'
                : 'These checks did not reject this graph'}
          </p>
          <p className="mb-0 mt-1 text-label text-faint">Non-rejection is not proof that the graph is correct. These tests assess implications that are observable in this dataset; they cannot rule out every omitted variable or alternative graph.</p>
          <details className="mt-3">
            <DisclosureSummary className="cursor-pointer text-body text-ink">Conditional-independence results</DisclosureSummary>
            <div className="figure-strip mt-2 overflow-x-auto">
              <table className="w-full border-collapse text-left text-label">
                <thead><tr className="border-b border-line text-faint"><th className="py-1 pr-2 font-medium">Implication</th><th className="px-2 py-1 font-medium">Raw p</th><th className="px-2 py-1 font-medium">Holm p</th><th className="py-1 pl-2 font-medium">Decision</th></tr></thead>
                <tbody>{evidence.implications.map((implication) => (
                  <tr key={`${implication.x}:${implication.y}:${implication.given.join(',')}`} className="border-b border-hair">
                    <td className="py-1.5 pr-2 text-ink">{nodeName(implication.x)} ⊥ {nodeName(implication.y)}{implication.given.length > 0 ? ` | ${implication.given.map(nodeName).join(', ')}` : ''}</td>
                    <td className="px-2 py-1.5 tabular-nums">{implication.pValue.toPrecision(3)}</td>
                    <td className="px-2 py-1.5 tabular-nums">{implication.adjustedPValue.toPrecision(3)}</td>
                    <td className={`py-1.5 pl-2 ${implication.decision === 'contradicted' ? 'text-danger' : 'text-muted'}`}>{implication.decision === 'contradicted' ? 'Contradicted' : 'Not rejected'}</td>
                  </tr>
                ))}</tbody>
              </table>
            </div>
          </details>
          <div className="mt-3 grid gap-3 @sm/inspector:grid-cols-2">
            <div className={well('p-(--panel-space)')}>
              <span className="block text-label text-faint">Raw p-value distribution</span>
              <strong className="mt-1 block text-title font-medium tabular-nums text-ink">KS p = {evidence.uniformity.pValue.toPrecision(3)}</strong>
              <p className="mb-0 mt-1 text-label text-muted">{evidence.uniformity.pValue < evidence.significanceLevel ? 'The test found evidence that the unadjusted p-values differ from a uniform distribution.' : 'The test did not find evidence that the unadjusted p-values differ from a uniform distribution.'} Treat this as a supplementary check because the implication tests may depend on each other.</p>
            </div>
            <div className={well('p-(--panel-space)')}>
              {evidence.falsification.kind === 'completed' ? (
                <>
                  <span className="block text-label text-faint">Relabeled-graph comparison</span>
                  <strong className="mt-1 block text-title font-medium tabular-nums text-ink"><Metadata><span>p<sub>LMC</sub> = {evidence.falsification.pValueLmc.toPrecision(3)}</span><span>p<sub>TPA</sub> = {evidence.falsification.pValueTpa.toPrecision(3)}</span></Metadata></strong>
                  <p className="mb-0 mt-1 text-label text-muted">p<sub>LMC</sub> is the share of relabeled graphs whose proportion of violated local-Markov implications is no greater than this graph’s. p<sub>TPA</sub> is the share of relabeled graphs in the same Markov-equivalence class as this graph. {evidence.falsification.falsified ? 'The relabeling comparison rejects this graph at the recorded threshold.' : !evidence.falsification.falsifiable ? 'This comparison cannot evaluate the graph at the recorded threshold. It requires testable implications and enough relabelings outside the graph’s Markov-equivalence class.' : 'The relabeling comparison can evaluate this graph and does not reject it at the recorded threshold. This does not establish that the graph is correct.'}</p>
                </>
              ) : (
                <><span className="block text-label text-faint">Relabeled-graph comparison</span><p className="mb-0 mt-1 text-label text-muted">Not run. {evidence.falsification.reason}</p></>
              )}
            </div>
          </div>
        </div>
      )}
    </section>
  )
}

export function DagWorkspace({
  profile,
  prepared,
  discoveryRuns,
  documents,
  checks,
  onDocumentCreated,
  onDocumentRevised,
  onCheck,
  onUseForStudy,
  onUseForRootCause,
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

  /** Plan against the document, then join every arrow in one revision; a declared exposure and outcome bind the study when the text names exactly one of each. */
  const pasteGraph = (document: DagDocument) => {
    if (state.kind !== 'editing' || state.paste.kind !== 'open') return
    const plan = planDagImport(document, state.paste.text)
    if (!plan.ok) {
      dispatch({ type: 'paste-refused', problem: plan.error })
      return
    }
    const revised = reviseDagWithImportedGraph(document, importOf(plan.value))
    if (!revised.ok) {
      dispatch({ type: 'paste-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    const observedNamed = (name: string | null): DagNodeId | null => name === null
      ? null
      : revised.value.current.graph.nodes.find((node) => node.kind === 'observed' && node.name === name)?.id ?? null
    const treatment = observedNamed(plan.value.exposure)
    const outcome = observedNamed(plan.value.outcome)
    if (treatment !== null || outcome !== null) {
      const base = boundHere ? studyDraft : { ...EMPTY_STUDY_DRAFT, dagDocument: revised.value.id }
      onStudyDraftChanged({ ...base, dagDocument: revised.value.id, treatment: treatment ?? base.treatment, outcome: outcome ?? base.outcome })
    }
    dispatch({ type: 'paste-applied' })
  }
  const [inspectorTab, setInspectorTab] = useState<'selection' | 'evidence' | 'intervene'>(discoveryRuns.length > 0 ? 'evidence' : 'selection')
  const [interventionOverlay, setInterventionOverlay] = useState<InterventionOverlay | null>(null)
  useEffect(() => {
    if (selectedEdge !== null) setInspectorTab('selection')
  }, [selectedEdge])

  const header = (
    <div className="mb-3">
      <div>
        <ChapterHeading id="dag-workspace-title" className="mb-2">DAG workspace</ChapterHeading>
        <p className={chapterIntro}>A directed acyclic graph (DAG) represents assumptions about how data is generated. Nodes represent variables, and each arrow states an assumed direct causal relationship. In this section, build the graph for your causal question and record the reason for each arrow. Discovery results can contribute evidence, but they do not determine the graph.</p>
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
              <section className={panel('p-(--panel-space)')} aria-labelledby="dag-origin-title">
                <h3 id="dag-origin-title" className={cn(sectionTitle, 'mb-2 mt-0')}>Graph basis</h3>
                <div className="grid gap-3 @md/panel:grid-cols-3">
                  <OriginChoice active={state.origin.kind === 'domain-knowledge'} icon="psychology" title="Substantive knowledge" detail="Theory, prior studies, expert knowledge, institutions, and the treatment-assignment process." onClick={() => dispatch({ type: 'origin-selected', origin: 'domain-knowledge' })} />
                  <OriginChoice active={state.origin.kind === 'experimental-design'} icon="experiment" title="Experimental design" detail="The randomisation protocol, intervention timing, and measurement design." onClick={() => dispatch({ type: 'origin-selected', origin: 'experimental-design' })} />
                  <OriginChoice active={state.origin.kind === 'discovery-informed'} disabled={discoveryRuns.length === 0} icon="schema" title="Discovery-informed" detail={discoveryRuns.length === 0 ? 'No discovery results are available for review.' : `Evaluate selected discovery results against substantive knowledge.`} onClick={() => dispatch({ type: 'origin-selected', origin: 'discovery-informed' })} />
                </div>
                {state.origin.kind === 'discovery-informed' && discoveryRuns.length > 0 && (
                  <fieldset className={well('mt-4 p-3')}>
                    <legend className="px-1 text-body font-medium text-ink">Discovery runs reviewed for this DAG</legend>
                    <div className="mt-1 grid gap-2 @md/panel:grid-cols-2">
                      {[...discoveryRuns].reverse().map((run) => {
                        const method = methodDefinition(run.method)
                        return (
                          <label key={run.id} className="flex items-start gap-2 text-body text-ink">
                            <input
                              type="checkbox"
                              className="mt-0.5"
                              checked={state.origin.kind === 'discovery-informed' && state.origin.reports.includes(run.id)}
                              onChange={() => dispatch({ type: 'discovery-origin-run-toggled', run: run.id })}
                            />
                            <span>{method.ok ? method.value.name : run.method}<span className="block text-label text-faint">{formatTime(run.createdAt)}</span></span>
                          </label>
                        )
                      })}
                    </div>
                  </fieldset>
                )}
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
        <SegmentedControl wrap className="mb-3 self-start" ariaLabel="DAG documents" value={document.id}
          onChange={(id) => dispatch({ type: 'document-selected', document: id, latestRun: latestRunId(discoveryRuns) })}
          options={documents.map((candidate) => ({ value: candidate.id, label: candidate.name }))} />
      )}
      <div className="mb-6 flex flex-wrap items-end justify-between gap-4">
        <div>
          <h3 className="mb-1 mt-0 text-title font-medium text-ink">{document.name}</h3>
          <p className="m-0 text-label text-faint">{describeDagOrigin(document.origin)}</p>
        </div>
        <div className="flex flex-wrap items-center gap-x-3 gap-y-4" role="toolbar" aria-label="DAG actions">
          <div className="flex basis-full items-center gap-2 sm:basis-auto" role="group" aria-label="DAG revision controls">
            <button type="button" disabled={document.history.length === 0} className={button('quiet')} onClick={() => moveRevision(document, 'undo')} aria-label="Undo DAG revision" title="Undo DAG revision"><Icon name="undo" size={15} /></button>
            <span className="whitespace-nowrap text-label tabular-nums text-muted">Revision {document.history.length + 1} of {document.history.length + document.future.length + 1}</span>
            <button type="button" disabled={document.future.length === 0} className={button('quiet')} onClick={() => moveRevision(document, 'redo')} aria-label="Redo DAG revision" title="Redo DAG revision"><Icon name="redo" size={15} /></button>
            <ParameterHelp label="revision history" help={`${document.audit.length} revisions retained. Undo and redo change the active revision without deleting retained revisions.`} />
          </div>
          <button type="button" className={button('quiet', 'inline-flex items-center gap-1.5')} onClick={() => dispatch({ type: 'latent-variable-add-requested' })}><Icon name="add" size={14} /> Unmeasured variable</button>
          <button type="button" className={button('quiet', 'inline-flex items-center gap-1.5')} onClick={() => dispatch({ type: 'paste-requested' })}><Icon name="content_paste" size={14} /> From text</button>
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'new-document-requested' })}>New DAG</button>
        </div>
      </div>
      {/* A fixed label width keeps the two selects aligned whether they sit side by side or wrap onto their own lines. */}
      <div className="mb-6 flex flex-wrap items-center gap-3" role="group" aria-label="Study binding">
        <label className="flex items-center gap-3 text-body text-ink"><span className="w-20 shrink-0">Treatment</span>
          <Select aria-label="Treatment" className={field('text', 'w-40')} value={boundTreatment ?? ''} onChange={(event) => bind('treatment', event.target.value === '' ? null : (event.target.value as DagNodeId))}>
            <option value="">Choose</option>
            {document.current.graph.nodes.filter((node) => node.kind === 'observed').map((node) => <option key={node.id} value={node.id} disabled={node.id === boundOutcome}>{node.name}</option>)}
          </Select>
        </label>
        <label className="flex items-center gap-3 text-body text-ink"><span className="w-20 shrink-0">Outcome</span>
          <Select aria-label="Outcome" className={field('text', 'w-40')} value={boundOutcome ?? ''} onChange={(event) => bind('outcome', event.target.value === '' ? null : (event.target.value as DagNodeId))}>
            <option value="">Choose</option>
            {document.current.graph.nodes.filter((node) => node.kind === 'observed').map((node) => <option key={node.id} value={node.id} disabled={node.id === boundTreatment}>{node.name}</option>)}
          </Select>
        </label>
      </div>
      {state.latentVariable.kind === 'adding' && (
        <form className={well('mb-3 flex flex-wrap items-end gap-2 p-3')} onSubmit={(event) => { event.preventDefault(); addLatentVariable(document) }}>
          <label className="min-w-[14rem] flex-1 text-body text-ink">Unmeasured variable name<input autoFocus className={field('text', 'mt-1')} value={state.latentVariable.name} onChange={(event) => dispatch({ type: 'latent-variable-name-changed', value: event.target.value })} onKeyDown={(event) => { if (event.key === 'Escape') dispatch({ type: 'latent-variable-edit-cancelled' }) }} /></label>
          <button type="submit" className={button('outline')}>Add to DAG</button>
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'latent-variable-edit-cancelled' })}>Cancel</button>
          {state.latentVariable.problem !== null && <p role="alert" className="m-0 w-full text-body text-danger">{describeDagVariableEditProblem(state.latentVariable.problem)}</p>}
        </form>
      )}
      {state.latentVariable.kind === 'closed' && state.latentVariable.problem !== null && <p role="alert" className="mb-3 mt-0 text-body text-danger">{describeDagVariableEditProblem(state.latentVariable.problem)}</p>}
      {state.paste.kind === 'open' && (
        <form className={well('mb-3 flex flex-wrap items-end gap-2 p-3')} onSubmit={(event) => { event.preventDefault(); pasteGraph(document) }}>
          <label className="min-w-0 basis-full text-body text-ink">Graph text
            <textarea autoFocus aria-label="Graph text" className={field('text', 'mt-1 min-h-32 w-full resize-y rounded-lg')} value={state.paste.text} onChange={(event) => dispatch({ type: 'paste-text-changed', value: event.target.value })} onKeyDown={(event) => { if (event.key === 'Escape') dispatch({ type: 'paste-cancelled' }) }} placeholder="dag { X [exposure] Y [outcome] X <- A -> M <- B -> Y X -> Y A <-> B }" />
          </label>
          <dl className="m-0 grid basis-full grid-cols-[auto_1fr] gap-x-3 gap-y-1 text-label text-faint">
            <dt className={literal('text-muted')}>{'A -> B'}</dt><dd className="m-0">{'A is a cause of B. B <- A means the same.'}</dd>
            <dt className={literal('text-muted')}>{'A <-> B'}</dt><dd className="m-0">An unmeasured common cause of A and B.</dd>
            <dt className={literal('text-muted')}>{'X [exposure]'}</dt><dd className="m-0">{'[exposure] marks the treatment, [outcome] marks the outcome, and [latent] marks an unmeasured variable.'}</dd>
            <dt className={literal('text-muted')}>{'A -> B [lag=1]'}</dt><dd className="m-0">For a time series, this states that A one time step earlier is a cause of B now. Without a lag, the arrow relates variables in the same period.</dd>
          </dl>
          <p className="m-0 basis-full text-label text-faint">Use the graph-text syntax shown above. Each unmarked variable must be a column in your prepared data. Imported arrows have no recorded rationale; add one for each arrow before analysis. Press Escape to cancel.</p>
          <button type="submit" className={button('outline')}>Convert to DAG</button>
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'paste-cancelled' })}>Cancel</button>
          {state.paste.problem !== null && <Alert tone="info" className="w-full"><p className="m-0">{describeDagImportProblem(state.paste.problem)}</p></Alert>}
        </form>
      )}
      {state.paste.kind === 'closed' && state.paste.problem !== null && <Alert tone="info" className="mb-3"><p className="m-0">{describeDagImportProblem(state.paste.problem)}</p></Alert>}
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
    <section className="pt-4" aria-labelledby="add-edge-title">
      <div className="flex flex-wrap items-baseline justify-between gap-2">
        <h3 id="add-edge-title" className="m-0 text-body font-medium text-ink">Add an arrow or use the editor</h3>
        {(state.cause !== null || state.effect !== null || state.rationale.length > 0) && <button type="button" className="text-label text-muted hover:text-ink" onClick={() => dispatch({ type: 'edge-draft-cancelled' })}>Clear the draft</button>}
      </div>
      <div className="mt-3 grid gap-4">
        <label className="block min-w-0"><span className={fieldLabel}>Cause</span><Select aria-label="Proposed cause" className={field('text', 'mt-1')} value={state.cause ?? ''} onChange={(event) => dispatch({ type: 'cause-selected', node: nodeFromValue(document, event.target.value) })}><option value="">Choose a variable</option>{document.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}</Select></label>
        <label className="block min-w-0"><span className={fieldLabel}>Effect</span><Select aria-label="Proposed effect" className={field('text', 'mt-1')} value={state.effect ?? ''} onChange={(event) => dispatch({ type: 'effect-selected', node: nodeFromValue(document, event.target.value) })}><option value="">Choose a variable</option>{document.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}</Select></label>
        {document.dataset.kind === 'time-series' ? (
          <label className="block min-w-0"><span className={fieldLabel}>Timing</span><Select aria-label="Timing" className={field('text', 'mt-1')} value={state.timing.kind} onChange={(event) => dispatch({ type: 'timing-selected', timing: event.target.value === 'lagged' ? { kind: 'lagged', lag: '1' } : { kind: 'contemporaneous' } })}><option value="contemporaneous">Contemporaneous (t)</option><option value="lagged">Past cause (t−lag)</option></Select></label>
        ) : null}
        {document.dataset.kind === 'time-series' && state.timing.kind === 'lagged'
          ? <label className="block min-w-0"><span className={fieldLabel}>Lag</span><input aria-label="Lag" className={field('text', 'mt-1')} type="number" min={1} max={Math.max(1, document.dataset.observations - 1)} value={state.timing.lag} onChange={(event) => dispatch({ type: 'lag-changed', value: event.target.value })} /></label>
          : null}
      </div>
      <div className="mt-4 grid gap-4">
        <label className="block min-w-0"><span className={fieldLabel}>Rationale</span><input aria-label="Rationale for proposed arrow" className={field('text', 'mt-1')} value={state.rationale} onChange={(event) => dispatch({ type: 'rationale-changed', value: event.target.value })} placeholder="Record the mechanism, assignment rule, protocol, prior study, or expert evidence." /></label>
        <button type="button" className={button('signal', 'inline-flex items-center justify-center gap-1.5')} onClick={addEdge}><Icon name="add" size={16} /> Add the arrow</button>
      </div>
      {canAttach && <label className="mt-2 flex items-start gap-2 text-body text-muted"><input type="checkbox" checked={state.attachSelectedEvidence} onChange={(event) => dispatch({ type: 'evidence-attachment-changed', attached: event.target.checked })} /><span>Attach the selected discovery result to this arrow.</span></label>}
      {state.problem !== null && <p role="alert" className="mb-0 mt-2 text-body text-danger">{describeDagEditProblem(state.problem)}</p>}
    </section>
  )

  const rootCause = prepareRootCauseGraph(document, prepared)
  const inspector = (
    <div className="flex flex-col gap-4">
      <SegmentedControl size="sm" fill ariaLabel="DAG inspector" value={inspectorTab} onChange={setInspectorTab} options={[
        { value: 'selection', label: <span className="flex items-center gap-1.5"><Icon name="ads_click" size={14} />Selection</span> },
        { value: 'evidence', label: <span className="flex items-center gap-1.5"><Icon name="schema" size={14} />Evidence</span> },
        { value: 'intervene', label: <span className="flex items-center gap-1.5"><Icon name="bolt" size={14} />Intervene</span> },
      ]} />
      <ValidationPanel document={document} flow={flow} onUseForStudy={() => { if (!boundHere) onStudyDraftChanged({ ...EMPTY_STUDY_DRAFT, dagDocument: document.id }); onUseForStudy() }} onUseForRootCause={rootCause.ok ? () => onUseForRootCause({ dagDocument: rootCause.value.dagDocument, dagRevision: rootCause.value.dagRevision, preparedDataset: rootCause.value.preparedDataset }) : null} onSelectEdge={(edge) => { setInspectorTab('selection'); selectEdge(document, edge) }} />
      <GraphCheckPanel source={source} profile={profile} prepared={prepared} document={document} checks={checks} onCheck={onCheck} />
      {inspectorTab === 'selection' && (
        selectedEdge !== null && selectedEdgeDraft !== null ? (
          <aside className="pt-4" aria-labelledby="selected-edge-title">
            <h3 id="selected-edge-title" className="mb-1 mt-0 text-body font-medium text-ink">{nameOfDagNode(document, selectedEdge.cause)} → {nameOfDagNode(document, selectedEdge.effect)}</h3>
            <p className={`m-0 text-body ${selectedEdge.support.kind === 'unstated' ? 'text-warn' : 'text-faint'}`}><Metadata><span>{describeTiming(selectedEdge.timing)}</span><span>{describeSupport(selectedEdge.support)}</span></Metadata></p>
            <form className="mt-3" onSubmit={(event) => { event.preventDefault(); saveSelectedEdgeDetails(document) }}>
              {document.dataset.kind === 'time-series' && (
                <div className="grid gap-3 @sm/inspector:grid-cols-2">
                  <label className="block"><span className={fieldLabel}>Arrow timing</span><Select className={field('text', 'mt-1')} value={selectedEdgeDraft.timing.kind} onChange={(event) => dispatch({ type: 'selected-edge-timing-selected', timing: event.target.value === 'lagged' ? { kind: 'lagged', lag: '1' } : { kind: 'contemporaneous' } })}><option value="contemporaneous">Contemporaneous (t)</option><option value="lagged">Past cause (t−lag)</option></Select></label>
                  {selectedEdgeDraft.timing.kind === 'lagged' && <label className="block"><span className={fieldLabel}>Arrow lag</span><input className={field('text', 'mt-1')} type="number" min={1} max={Math.max(1, document.dataset.observations - 1)} value={selectedEdgeDraft.timing.lag} onChange={(event) => dispatch({ type: 'selected-edge-lag-changed', value: event.target.value })} /></label>}
                </div>
              )}
              <label className="mt-3 block"><span className={fieldLabel}>Rationale</span><textarea className={field('text', 'mt-1 min-h-20 resize-y rounded-lg')} value={selectedEdgeDraft.rationale} onChange={(event) => dispatch({ type: 'selected-edge-rationale-changed', value: event.target.value })} placeholder="Record the mechanism, assignment rule, protocol, prior study, or expert evidence supporting this arrow." /></label>
              <p className={literal('mb-0 mt-2 text-micro text-faint')}>{selectedEdge.evidence.length} attached discovery item{selectedEdge.evidence.length === 1 ? '' : 's'}</p>
              {selectedEdgeDraft.problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeDagEditProblem(selectedEdgeDraft.problem)}</p>}
              <div className="mt-3 flex flex-wrap gap-2">
                <button type="submit" className={button('signal')}>Save the arrow</button>
                <button type="button" className={button('danger')} onClick={() => removeEdge(document, selectedEdge.id)}>Remove the arrow</button>
              </div>
            </form>
          </aside>
        ) : (
          addEdgeForm
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
        body: inspector,
      }}
      bottom={{ title: `Arrows (${document.current.graph.edges.length})`, body: ledger, defaultSize: 150 }}
    />
  )
}
