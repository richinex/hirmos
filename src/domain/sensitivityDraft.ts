import { assertNever } from './dop'
import type { EstimationRunArtifact, EstimationRunId } from './estimation'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { Workflow } from './workflow'
import { DEFAULT_DML_REFUTATION, DEFAULT_REFUTATION, DEFAULT_UNOBSERVED, type SensitivityConfiguration, type SensitivityProbe, type SensitivityRunArtifact } from './sensitivity'

export interface SensitivityDraft {
  readonly estimationRun: EstimationRunId | null
  readonly probe: SensitivityProbe
  readonly configurations: { readonly [K in SensitivityProbe]: Extract<SensitivityConfiguration, { readonly kind: K }> }
}

export type SensitivityEvent =
  | { readonly type: 'run-chosen'; readonly run: EstimationRunId | null }
  | { readonly type: 'probe-chosen'; readonly probe: SensitivityProbe }
  | { readonly type: 'configured'; readonly configuration: SensitivityConfiguration }

export function initialSensitivityDraft(estimates: readonly EstimationRunArtifact[], runs: readonly SensitivityRunArtifact[]): SensitivityDraft {
  const latest = runs.filter(run=>run.kind!=='honest-did-run'&&run.kind!=='did-sensitivity-run').at(-1) ?? null
  const probed = latest === null ? null : estimates.find(run => run.id === latest.estimationRun) ?? null
  const defaults = { 'linear-refutation': DEFAULT_REFUTATION, 'unobserved-confounding': DEFAULT_UNOBSERVED, 'dml-refutation': DEFAULT_DML_REFUTATION }
  return {
    estimationRun: probed?.id ?? [...estimates].reverse().find(run => run.kind === 'backdoor-linear-run')?.id ?? estimates.at(-1)?.id ?? null,
    probe: latest !== null && probed !== null ? latest.configuration.kind : 'linear-refutation',
    configurations: latest !== null && probed !== null ? { ...defaults, [latest.configuration.kind]: latest.configuration } : defaults,
  }
}

export const emptySensitivityDraft = initialSensitivityDraft([], [])

export interface SensitivitySession {
  readonly prepared: PreparedDatasetVersionId
  readonly draft: SensitivityDraft
}

export function retainSensitivityDraft(current: SensitivitySession | null, workflow: Workflow): SensitivitySession | null {
  if (workflow.kind !== 'profiled' || workflow.prepared === null || workflow.estimationRuns.length === 0) return null
  if (current === null || current.prepared !== workflow.prepared.id) {
    return { prepared: workflow.prepared.id, draft: initialSensitivityDraft(workflow.estimationRuns, workflow.sensitivityRuns) }
  }
  if (current.draft.estimationRun === null || workflow.estimationRuns.some(run => run.id === current.draft.estimationRun)) return current
  return { ...current, draft: { ...current.draft, estimationRun: null } }
}

export function stepSensitivityDraft(state: SensitivityDraft, event: SensitivityEvent): SensitivityDraft {
  switch (event.type) {
    case 'run-chosen': return { ...state, estimationRun: event.run }
    case 'probe-chosen': return { ...state, probe: event.probe }
    case 'configured': return { ...state, configurations: { ...state.configurations, [event.configuration.kind]: event.configuration } }
    default: return assertNever(event)
  }
}
