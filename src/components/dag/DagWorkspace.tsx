import { useReducer } from 'react'
import { Icon } from '@/components/Icon'
import { button, field, label, literal, segment } from '@/components/ui/recipes'
import {
  createDagDocument,
  describeDagCreateProblem,
  describeDagEditProblem,
  describeDagOrigin,
  nameOfDagNode,
  reviseDagWithEdge,
  reviseDagWithoutEdge,
  type DagCreateProblem,
  type DagDocument,
  type DagDocumentId,
  type DagEditProblem,
  type DagNodeId,
  type DagOriginChoice,
} from '@/domain/dag'
import type { DatasetProfile } from '@/domain/dataset'
import type { DiscoveryRunArtifact } from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import type { PreparedDatasetArtifact } from '@/domain/preprocessing'

interface DagWorkspaceProps {
  readonly profile: DatasetProfile
  readonly prepared: PreparedDatasetArtifact
  readonly discoveryRuns: readonly DiscoveryRunArtifact[]
  readonly documents: readonly DagDocument[]
  readonly onDocumentCreated: (document: DagDocument) => void
  readonly onDocumentRevised: (document: DagDocument) => void
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
      readonly rationale: string
      readonly problem: DagEditProblem | null
    }

type DagWorkspaceEvent =
  | { readonly type: 'new-document-requested' }
  | { readonly type: 'name-changed'; readonly value: string }
  | { readonly type: 'origin-selected'; readonly origin: DagOriginChoice }
  | { readonly type: 'creation-refused'; readonly problem: DagCreateProblem }
  | { readonly type: 'document-created'; readonly document: DagDocumentId }
  | { readonly type: 'document-selected'; readonly document: DagDocumentId }
  | { readonly type: 'cause-selected'; readonly node: DagNodeId | null }
  | { readonly type: 'effect-selected'; readonly node: DagNodeId | null }
  | { readonly type: 'rationale-changed'; readonly value: string }
  | { readonly type: 'edge-refused'; readonly problem: DagEditProblem }
  | { readonly type: 'edge-saved' }

const editingState = (document: DagDocumentId): DagWorkspaceState => ({
  kind: 'editing',
  document,
  cause: null,
  effect: null,
  rationale: '',
  problem: null,
})

const initialState = (documents: readonly DagDocument[]): DagWorkspaceState => {
  const [first] = documents
  return first === undefined
    ? { kind: 'creating', nameDraft: '', origin: 'domain-knowledge', problem: null }
    : editingState(first.id)
}

function stepDagWorkspace(state: DagWorkspaceState, event: DagWorkspaceEvent): DagWorkspaceState {
  switch (event.type) {
    case 'new-document-requested': return { kind: 'creating', nameDraft: '', origin: 'domain-knowledge', problem: null }
    case 'name-changed': return state.kind === 'creating' ? { ...state, nameDraft: event.value, problem: null } : state
    case 'origin-selected': return state.kind === 'creating' ? { ...state, origin: event.origin, problem: null } : state
    case 'creation-refused': return state.kind === 'creating' ? { ...state, problem: event.problem } : state
    case 'document-created': return editingState(event.document)
    case 'document-selected': return editingState(event.document)
    case 'cause-selected': return state.kind === 'editing' ? { ...state, cause: event.node, problem: null } : state
    case 'effect-selected': return state.kind === 'editing' ? { ...state, effect: event.node, problem: null } : state
    case 'rationale-changed': return state.kind === 'editing' ? { ...state, rationale: event.value, problem: null } : state
    case 'edge-refused': return state.kind === 'editing' ? { ...state, problem: event.problem } : state
    case 'edge-saved': return state.kind === 'editing'
      ? { ...state, cause: null, effect: null, rationale: '', problem: null }
      : state
    default: return assertNever(event)
  }
}

const nodeFromValue = (document: DagDocument, raw: string): DagNodeId | null =>
  document.current.graph.nodes.find((node) => node.id === raw)?.id ?? null

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
      className={`rounded-xl border p-4 text-left transition-colors ${
        active ? 'border-signal bg-raised' : 'border-line bg-panel hover:border-edge disabled:cursor-not-allowed disabled:opacity-45'
      }`}
    >
      <span className="mb-3 grid h-9 w-9 place-items-center rounded-lg border border-hair bg-well text-muted">
        <Icon name={icon} size={18} />
      </span>
      <span className="block text-title font-medium text-ink">{title}</span>
      <span className="mt-1 block text-body text-faint">{detail}</span>
    </button>
  )
}

function AdjustmentAudit() {
  return (
    <aside className="rounded-xl border border-line bg-panel p-4" aria-labelledby="adjustment-audit-title">
      <span className={label('text-faint')}>Adjustment audit</span>
      <h3 id="adjustment-audit-title" className="mb-2 mt-1 text-title font-medium text-ink">More controls can increase bias</h3>
      <p className="mb-3 mt-0 text-body text-muted">Roles are derived from paths and the later estimand; they are not permanent labels attached to columns.</p>
      <dl className="m-0 space-y-3 text-body">
        <div>
          <dt className="font-medium text-ink">Confounder · X ← C → Y</dt>
          <dd className="m-0 text-faint">May close a noncausal backdoor path.</dd>
        </div>
        <div>
          <dt className="font-medium text-ink">Mediator · X → M → Y</dt>
          <dd className="m-0 text-faint">Controlling for it removes part of a total effect.</dd>
        </div>
        <div>
          <dt className="font-medium text-ink">Collider · X → C ← U</dt>
          <dd className="m-0 text-faint">Conditioning can open a path that was naturally closed.</dd>
        </div>
        <div>
          <dt className="font-medium text-ink">Outcome predictor · C → Y</dt>
          <dd className="m-0 text-faint">May improve precision without being required for identification.</dd>
        </div>
      </dl>
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
}: DagWorkspaceProps) {
  const [state, dispatch] = useReducer(stepDagWorkspace, documents, initialState)

  const createDocument = () => {
    if (state.kind !== 'creating') return
    const created = createDagDocument(state.nameDraft, state.origin, prepared, profile, discoveryRuns)
    if (!created.ok) {
      dispatch({ type: 'creation-refused', problem: created.error })
      return
    }
    onDocumentCreated(created.value)
    dispatch({ type: 'document-created', document: created.value.id })
  }

  const selectedDocument = state.kind === 'editing'
    ? documents.find((document) => document.id === state.document)
    : undefined

  const addEdge = () => {
    if (state.kind !== 'editing' || selectedDocument === undefined) return
    const revised = reviseDagWithEdge(selectedDocument, state.cause, state.effect, state.rationale)
    if (!revised.ok) {
      dispatch({ type: 'edge-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
    dispatch({ type: 'edge-saved' })
  }

  const removeEdge = (document: DagDocument, edgeId: Parameters<typeof reviseDagWithoutEdge>[1]) => {
    const revised = reviseDagWithoutEdge(document, edgeId)
    if (!revised.ok) {
      dispatch({ type: 'edge-refused', problem: revised.error })
      return
    }
    onDocumentRevised(revised.value)
  }

  return (
    <section aria-labelledby="dag-workspace-title">
      <div className="mb-5 flex flex-wrap items-end justify-between gap-3">
        <div>
          <span className={label('text-signal')}>04 · DAG workspace</span>
          <h2 id="dag-workspace-title" className="mb-2 mt-2 text-heading text-ink">Build the causal model</h2>
          <p className="m-0 max-w-3xl text-body text-muted">Discovery is optional. A DAG can begin directly from experimental design, institutional knowledge, theory, and prior evidence.</p>
        </div>
        {state.kind === 'editing' && (
          <button type="button" className={button('quiet')} onClick={() => dispatch({ type: 'new-document-requested' })}>
            New DAG
          </button>
        )}
      </div>

      {state.kind === 'creating' && (
        <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(18rem,0.6fr)]">
          <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="dag-origin-title">
            <span className={label('text-faint')}>Graph provenance</span>
            <h3 id="dag-origin-title" className="mb-2 mt-1 text-title font-medium text-ink">What informs this graph?</h3>
            <p className="mb-4 mt-0 text-body text-faint">An arrow asserts a direct causal relationship. Correlation alone is not enough, and a missing arrow is also an assumption.</p>
            <div className="grid gap-3 sm:grid-cols-3">
              <OriginChoice
                active={state.origin === 'domain-knowledge'}
                icon="psychology"
                title="Domain knowledge"
                detail="Theory, institutions, prior studies, expert knowledge, and the data-generating story."
                onClick={() => dispatch({ type: 'origin-selected', origin: 'domain-knowledge' })}
              />
              <OriginChoice
                active={state.origin === 'experimental-design'}
                icon="experiment"
                title="Experimental design"
                detail="Known randomization, assignment mechanism, timing, and protocol. No discovery run required."
                onClick={() => dispatch({ type: 'origin-selected', origin: 'experimental-design' })}
              />
              <OriginChoice
                active={state.origin === 'discovery-informed'}
                disabled={discoveryRuns.length === 0}
                icon="schema"
                title="Discovery-informed"
                detail={discoveryRuns.length === 0 ? 'Optional; no discovery runs exist yet.' : `Review ${discoveryRuns.length} run${discoveryRuns.length === 1 ? '' : 's'} while authoring.`}
                onClick={() => dispatch({ type: 'origin-selected', origin: 'discovery-informed' })}
              />
            </div>
            <label className="mt-4 block text-body font-medium text-ink">
              DAG name
              <input
                className={field('text', 'mt-1')}
                value={state.nameDraft}
                onChange={(event) => dispatch({ type: 'name-changed', value: event.target.value })}
                placeholder="e.g. Assignment mechanism and road fatalities"
              />
            </label>
            {state.problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeDagCreateProblem(state.problem)}</p>}
            <button type="button" className={button('signal', 'mt-4')} onClick={createDocument}>Create DAG draft</button>
          </section>
          <AdjustmentAudit />
        </div>
      )}

      {state.kind === 'editing' && selectedDocument === undefined && (
        <p role="alert" className="rounded-xl border border-danger/30 bg-panel p-4 text-body text-danger">The selected DAG document is no longer available.</p>
      )}

      {state.kind === 'editing' && selectedDocument !== undefined && (
        <>
          {documents.length > 1 && (
            <div className="mb-4 flex flex-wrap gap-1 rounded-lg border border-hair bg-well p-1" aria-label="DAG documents">
              {documents.map((document) => (
                <button
                  key={document.id}
                  type="button"
                  className={segment(document.id === selectedDocument.id)}
                  aria-pressed={document.id === selectedDocument.id}
                  onClick={() => dispatch({ type: 'document-selected', document: document.id })}
                >
                  {document.name}
                </button>
              ))}
            </div>
          )}

          <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(18rem,0.55fr)]">
            <div className="space-y-4">
              <section className="rounded-xl border border-edge bg-panel p-4" aria-labelledby="current-dag-title">
                <div className="flex flex-wrap items-start justify-between gap-3">
                  <div>
                    <span className={label('text-signal')}>Draft revision</span>
                    <h3 id="current-dag-title" className="mb-1 mt-1 text-title font-medium text-ink">{selectedDocument.name}</h3>
                    <p className="m-0 text-body text-faint">{describeDagOrigin(selectedDocument.origin)}</p>
                  </div>
                  <span className={selectedDocument.current.validation.kind === 'structurally-valid' ? 'text-body text-ok' : 'text-body text-warn'}>
                    {selectedDocument.current.validation.kind === 'structurally-valid' ? 'Acyclic structure' : 'No causal edges yet'}
                  </span>
                </div>

                <div className="mt-4 flex flex-wrap gap-2" aria-label="DAG variables">
                  {selectedDocument.current.graph.nodes.map((node) => (
                    <span key={node.id} className="rounded-lg border border-hair bg-well px-3 py-2 text-body text-ink">{node.name}</span>
                  ))}
                </div>

                <div className="mt-4" aria-label="Causal edges">
                  {selectedDocument.current.graph.edges.length === 0 ? (
                    <p className="m-0 rounded-lg border border-dashed border-line p-3 text-body text-faint">The draft starts without inferred arrows. Add only relationships you are prepared to justify.</p>
                  ) : (
                    <ul className="m-0 space-y-2 p-0">
                      {selectedDocument.current.graph.edges.map((edge) => (
                        <li key={edge.id} className="flex flex-wrap items-start justify-between gap-3 rounded-lg border border-hair bg-well px-3 py-2">
                          <div>
                            <p className="m-0 text-body font-medium text-ink">{nameOfDagNode(selectedDocument, edge.cause)} → {nameOfDagNode(selectedDocument, edge.effect)}</p>
                            <p className="mb-0 mt-1 text-body text-faint">{edge.support.rationale}</p>
                            <p className={literal('mb-0 mt-1 text-micro text-faint')}>{edge.support.kind}</p>
                          </div>
                          <button type="button" className={button('quiet')} onClick={() => removeEdge(selectedDocument, edge.id)}>Remove</button>
                        </li>
                      ))}
                    </ul>
                  )}
                </div>
              </section>

              <section className="rounded-xl border border-line bg-panel p-4" aria-labelledby="add-edge-title">
                <span className={label('text-faint')}>Causal assumption</span>
                <h3 id="add-edge-title" className="mb-3 mt-1 text-title font-medium text-ink">Add a direct edge</h3>
                <div className="grid gap-3 sm:grid-cols-2">
                  <label className="text-body text-ink">
                    Proposed cause
                    <select
                      className={field('text', 'mt-1')}
                      value={state.cause ?? ''}
                      onChange={(event) => dispatch({ type: 'cause-selected', node: nodeFromValue(selectedDocument, event.target.value) })}
                    >
                      <option value="">Choose variable</option>
                      {selectedDocument.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}
                    </select>
                  </label>
                  <label className="text-body text-ink">
                    Proposed effect
                    <select
                      className={field('text', 'mt-1')}
                      value={state.effect ?? ''}
                      onChange={(event) => dispatch({ type: 'effect-selected', node: nodeFromValue(selectedDocument, event.target.value) })}
                    >
                      <option value="">Choose variable</option>
                      {selectedDocument.current.graph.nodes.map((node) => <option key={node.id} value={node.id}>{node.name}</option>)}
                    </select>
                  </label>
                </div>
                <label className="mt-3 block text-body text-ink">
                  Why is this direct causal relationship credible?
                  <textarea
                    className={field('text', 'mt-1 min-h-24 resize-y')}
                    value={state.rationale}
                    onChange={(event) => dispatch({ type: 'rationale-changed', value: event.target.value })}
                    placeholder="Record the mechanism, assignment rule, protocol, or external evidence."
                  />
                </label>
                {state.problem !== null && <p role="alert" className="mb-0 mt-3 text-body text-danger">{describeDagEditProblem(state.problem)}</p>}
                <button type="button" className={button('signal', 'mt-4 inline-flex items-center gap-2')} onClick={addEdge}>
                  <Icon name="add" size={16} /> Add edge as new revision
                </button>
              </section>
            </div>
            <AdjustmentAudit />
          </div>
        </>
      )}
    </section>
  )
}
