import { assertNever } from './dop'
import type { GcmEffectsRequest } from './gcmEffects'
import type { GcmInfluenceRequest } from './gcmInfluence'
import { selectedRootCauseGraph, type RootCauseGraph, type RootCauseSelection } from './rootCause'
import type { RootCauseRequest, RootCauseWorkspace } from './rootCauseAnalysis'
import type { Workflow } from './workflow'

export type CausalAnalysis =
  'anomaly' | 'change' | 'intervention' | 'effects' | 'intrinsic' | 'arrows'
type InfluenceKind = GcmInfluenceRequest['query']['kind']
export interface InfluenceDraft<K extends InfluenceKind> {
  readonly target: string
  readonly query: Extract<GcmInfluenceRequest['query'], { readonly kind: K }>
  readonly seed: number
}

export interface AttributionDraft {
  readonly target: string
  readonly repetitions: number
  readonly samples: number
  readonly changeFitting: 'halfNormalLinear' | 'automaticFull'
  readonly seed: number
  readonly randomSource: string
}

type AttributionEvent = {
  [K in keyof AttributionDraft]: {
    readonly type: 'attribution'
    readonly field: K
    readonly value: AttributionDraft[K]
  }
}[keyof AttributionDraft]

export interface CausalInputs {
  readonly file: File | null
  readonly observationMode: 'values' | 'file'
  readonly observationDraft: Readonly<Record<string, string>>
  readonly shifts: Readonly<Record<string, string>>
  readonly confirmed: boolean
  readonly replay: RootCauseRequest | null
}

export const emptyCausalInputs: CausalInputs = {
  file: null,
  observationMode: 'values',
  observationDraft: {},
  shifts: {},
  confirmed: false,
  replay: null,
}

export interface CausalModelDraft {
  readonly graph: RootCauseSelection
  readonly analysis: CausalAnalysis
  readonly intrinsic: InfluenceDraft<'intrinsic'>
  readonly arrows: InfluenceDraft<'arrows'>
  readonly effects: GcmEffectsRequest
  readonly attribution: AttributionDraft
  readonly inputs: CausalInputs
}

export type CausalModelEvent =
  | AttributionEvent
  | { readonly type: 'observation'; readonly node: string; readonly value: string }
  | { readonly type: 'shift'; readonly node: string; readonly value: string }
  | { readonly type: 'file'; readonly file: File }
  | { readonly type: 'observation-mode'; readonly mode: CausalInputs['observationMode'] }
  | { readonly type: 'confirm'; readonly confirmed: boolean }
  | { readonly type: 'replay'; readonly model: RootCauseRequest | null }
  | { readonly type: 'analysis'; readonly analysis: CausalAnalysis }
  | { readonly type: 'intrinsic'; readonly draft: InfluenceDraft<'intrinsic'> }
  | { readonly type: 'arrows'; readonly draft: InfluenceDraft<'arrows'> }
  | { readonly type: 'effects'; readonly model: GcmEffectsRequest }

export function sameCausalSelection(left: RootCauseSelection, right: RootCauseSelection): boolean {
  return (
    left.preparedDataset === right.preparedDataset &&
    left.dagDocument === right.dagDocument &&
    left.dagRevision === right.dagRevision
  )
}

function initialDraft(
  graph: RootCauseGraph,
  rows: number,
  workspace: RootCauseWorkspace,
): CausalModelDraft {
  const influences = workspace.influences.filter((run) => sameCausalSelection(run.graph, graph))
  const intrinsic = influences.filter((run) => run.model.query.kind === 'intrinsic').at(-1)?.model
  const arrows = influences.filter((run) => run.model.query.kind === 'arrows').at(-1)?.model
  const effects = workspace.effects
    .filter((run) => sameCausalSelection(run.graph, graph))
    .at(-1)?.model
  const previous = workspace.runs.filter((run) => sameCausalSelection(run.graph, graph)).at(-1)
  const seed = (model: GcmInfluenceRequest | undefined) =>
    model?.random.kind === 'seed' ? model.random.seed : 0
  return {
    graph: {
      dagDocument: graph.dagDocument,
      dagRevision: graph.dagRevision,
      preparedDataset: graph.preparedDataset,
    },
    analysis: previous?.model.query.kind ?? 'anomaly',
    inputs: {
      ...emptyCausalInputs,
      observationDraft:
        previous?.observation === undefined
          ? {}
          : Object.fromEntries(
              graph.nodes.map((node, index) => [node.id, String(previous.observation![index])]),
            ),
      shifts:
        previous?.model.query.kind === 'intervention'
          ? Object.fromEntries(
              previous.model.query.shifts.map((shift) => [
                graph.nodes[shift.node]!.id,
                String(shift.amount),
              ]),
            )
          : {},
    },
    attribution: {
      target: previous === undefined ? '' : String(previous.model.target),
      repetitions: previous?.model.repetitions ?? 10,
      samples:
        previous === undefined || previous.model.query.kind === 'intervention'
          ? 3000
          : previous.model.query.samples,
      changeFitting:
        previous?.model.query.kind === 'change'
          ? previous.model.query.fitting.kind
          : 'halfNormalLinear',
      seed: previous?.model.random.kind === 'seed' ? previous.model.random.seed : 0,
      randomSource: 'seed',
    },
    intrinsic: {
      target: intrinsic === undefined ? '' : String(intrinsic.target),
      seed: seed(intrinsic),
      query:
        intrinsic?.query.kind === 'intrinsic'
          ? intrinsic.query
          : { kind: 'intrinsic', training: 100_000, randomization: 250, baseline: 1000 },
    },
    arrows: {
      target: arrows === undefined ? '' : String(arrows.target),
      seed: seed(arrows),
      query:
        arrows?.query.kind === 'arrows'
          ? arrows.query
          : { kind: 'arrows', conditional: 2000, maxRuns: 5000, tolerance: 0.01 },
    },
    effects: effects ?? {
      names: graph.nodes.map((node) => node.name),
      edges: graph.edges.map(([a, b]) => [a, b]),
      rows,
      treatment: 0,
      outcome: 1,
      trees: 100,
      minLeaf: 1,
      fitSeed: 0,
      simulationSeed: 47,
      repetitions: 100,
      upperQuantile: 0.95,
      grouping: { kind: 'none' },
      mechanisms: graph.nodes.map((_, i) =>
        !graph.edges.some(([, child]) => child === i)
          ? { kind: 'empirical' }
          : i === 0
            ? { kind: 'classifier', classes: 2 }
            : { kind: 'regression' },
      ),
    },
  }
}

export function stepCausalModelDraft(
  state: CausalModelDraft,
  event: CausalModelEvent,
): CausalModelDraft {
  switch (event.type) {
    case 'observation':
      return {
        ...state,
        inputs: {
          ...state.inputs,
          observationDraft: { ...state.inputs.observationDraft, [event.node]: event.value },
          confirmed: false,
        },
      }
    case 'shift':
      return {
        ...state,
        inputs: { ...state.inputs, shifts: { ...state.inputs.shifts, [event.node]: event.value } },
      }
    case 'file':
      return { ...state, inputs: { ...state.inputs, file: event.file, confirmed: false } }
    case 'observation-mode':
      return {
        ...state,
        inputs: { ...state.inputs, observationMode: event.mode, confirmed: false },
      }
    case 'confirm':
      return { ...state, inputs: { ...state.inputs, confirmed: event.confirmed } }
    case 'replay':
      return {
        ...state,
        analysis: event.model?.query.kind ?? state.analysis,
        attribution:
          event.model === null
            ? state.attribution
            : { ...state.attribution, target: String(event.model.target) },
        inputs: {
          ...state.inputs,
          replay: event.model,
          confirmed: event.model === null ? state.inputs.confirmed : false,
        },
      }
    case 'attribution':
      return { ...state, attribution: { ...state.attribution, [event.field]: event.value } }
    case 'analysis':
      return { ...state, analysis: event.analysis }
    case 'intrinsic':
      return { ...state, intrinsic: event.draft }
    case 'arrows':
      return { ...state, arrows: event.draft }
    case 'effects':
      return { ...state, effects: event.model }
    default:
      return assertNever(event)
  }
}

/** Keep drafts only while their recorded graph and prepared-data bindings exist. */
export function retainCausalModelDrafts(
  drafts: readonly CausalModelDraft[],
  workflow: Workflow,
): readonly CausalModelDraft[] {
  if (workflow.kind !== 'profiled' || workflow.prepared === null) return []
  const { prepared, dagDocuments, rootCause } = workflow
  const retained = drafts.filter(
    (draft) => selectedRootCauseGraph(draft.graph, dagDocuments, prepared).ok,
  )
  const selected = selectedRootCauseGraph(rootCause.selection, dagDocuments, prepared)
  if (selected.ok && !retained.some((draft) => sameCausalSelection(draft.graph, selected.value))) {
    return [...retained, initialDraft(selected.value, prepared.observations, rootCause)]
  }
  return retained.length === drafts.length ? drafts : retained
}
