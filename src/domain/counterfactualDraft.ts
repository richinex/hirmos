import { DEFAULT_DYNAMIC_LINEAR_SCM, DEFAULT_LINEAR_SCM, type CounterfactualConfiguration } from './counterfactual'
import { assertNever } from './dop'
import type { PreparedDatasetVersionId } from './preprocessing'
import type { IdentificationId } from './study'
import type { Workflow } from './workflow'

export interface CounterfactualDraft {
  readonly identification: IdentificationId | null
  readonly configuration: CounterfactualConfiguration
}

export type CounterfactualEvent =
  | { readonly type: 'identification-chosen'; readonly identification: IdentificationId | null }
  | { readonly type: 'configured'; readonly configuration: CounterfactualConfiguration }

export interface CounterfactualSession {
  readonly prepared: PreparedDatasetVersionId
  readonly draft: CounterfactualDraft
}

export const emptyCounterfactualDraft: CounterfactualDraft = { identification: null, configuration: DEFAULT_LINEAR_SCM }
export const emptyDynamicCounterfactualDraft: CounterfactualDraft = { identification: null, configuration: DEFAULT_DYNAMIC_LINEAR_SCM }

export function retainCounterfactualDraft(current: CounterfactualSession | null, workflow: Workflow): CounterfactualSession | null {
  if (workflow.kind !== 'profiled' || workflow.prepared === null) return null
  const identified = workflow.identifications.filter(item => item.result.kind === 'identified')
  if (identified.length === 0) return null
  if (current !== null && current.prepared === workflow.prepared.id) {
    return current.draft.identification === null || identified.some(item => item.id === current.draft.identification)
      ? current : { ...current, draft: { ...current.draft, identification: null } }
  }
  const latest = workflow.counterfactualRuns.at(-1) ?? null
  const recorded = latest === null ? null : identified.find(item => item.id === latest.identification) ?? null
  return {
    prepared: workflow.prepared.id,
    draft: {
      identification: recorded?.id ?? identified.at(-1)?.id ?? null,
      configuration: latest !== null && recorded !== null ? latest.configuration
        : workflow.prepared.kind === 'prepared-time-series' ? DEFAULT_DYNAMIC_LINEAR_SCM : DEFAULT_LINEAR_SCM,
    },
  }
}

export function stepCounterfactualDraft(state: CounterfactualDraft, event: CounterfactualEvent): CounterfactualDraft {
  switch (event.type) {
    case 'identification-chosen': return { ...state, identification: event.identification }
    case 'configured': return { ...state, configuration: event.configuration }
    default: return assertNever(event)
  }
}
