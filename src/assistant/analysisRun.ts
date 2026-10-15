import { headlineValue, type CausalEstimate } from '@/domain/estimation'
import { z } from 'zod'
import type { GraphTarget } from '@/domain/dagWorkspaceSettings'

export const runSubjectSchema = z.discriminatedUnion('kind', [
  z
    .object({ kind: z.literal('dag'), document: z.string().min(1), revision: z.string().min(1) })
    .strict(),
  z
    .object({
      kind: z.literal('saved-study'),
      study: z.string().min(1),
      treatment: z.string().min(1),
      outcome: z.string().min(1),
    })
    .strict(),
  z
    .object({
      kind: z.literal('study-draft'),
      document: z.string().min(1),
      treatment: z.string().min(1),
      outcome: z.string().min(1),
    })
    .strict(),
])
export type RunSubject = z.infer<typeof runSubjectSchema>

export function runSubjectProblem(
  inspection: RunInspection,
  requested?: RunSubject,
): string | null {
  if (inspection.graph) {
    if (requested?.kind !== 'dag')
      return 'Name the intended source DAG and revision in run_analysis.subject. Read the SWIG workspace settings for its graph binding.'
    return requested.document === inspection.graph.document &&
      requested.revision === inspection.graph.revision
      ? null
      : `The selected source DAG is ${inspection.graph.name}, or its revision has changed. Select the intended graph and review its settings before running.`
  }
  if (requested?.kind === 'dag') return 'This analysis does not expose a graph binding.'
  const selected = inspection.study?.subject
  if (!selected) return requested ? 'This analysis does not expose a study binding.' : null
  if (!requested)
    return 'Name the intended study and treatment/outcome in run_analysis.subject. Read the current draft and saved studies before requesting a run.'
  const same =
    selected.kind === requested.kind &&
    selected.treatment === requested.treatment &&
    selected.outcome === requested.outcome &&
    (selected.kind === 'saved-study' && requested.kind === 'saved-study'
      ? selected.study === requested.study
      : selected.kind === 'study-draft' &&
        requested.kind === 'study-draft' &&
        selected.document === requested.document)
  return same
    ? null
    : `The selected specification is ${selected.treatment} → ${selected.outcome}, not the requested study. Select the intended study in ${inspection.section}, then request a new run.`
}

export function estimateSummary(estimate: CausalEstimate, subject: RunSubject): string {
  return JSON.stringify({
    subject,
    estimand: estimate.estimand,
    effectKind: estimate.effect.kind,
    headline: headlineValue(estimate.effect),
    interval: estimate.interval,
    standardError: estimate.standardError,
    sample: estimate.sample,
    adjustment: estimate.adjustment,
    interpretation:
      'The headline follows the saved result. Per-row effects and paths are not shared. Causal interpretation depends on the study assumptions.',
  })
}

export type AssistantRunOutcome =
  | { readonly kind: 'calculated'; readonly method: string; readonly summary: string }
  | {
      readonly kind: 'review-required'
      readonly method: string
      readonly reason: string
      readonly summary: string
    }
  | {
      readonly kind: 'completed'
      readonly runId: string
      readonly subject?: RunSubject
      readonly method: string
      readonly summary: string
    }
  | { readonly kind: 'refused' | 'failed'; readonly reason: string }
  | { readonly kind: 'cancelled' }

export interface RunInspection {
  readonly graph?: GraphTarget
  readonly section: string
  readonly method: string
  readonly specification: string
  readonly sharedResult: string
  readonly study?: {
    readonly subject: Exclude<RunSubject, { kind: 'dag' }>
    readonly question: string
    readonly settings: readonly { readonly label: string; readonly value: string }[]
  }
}

export interface RunCapability {
  readonly revision: string
  readonly inspection: RunInspection
  readonly execute: (signal: AbortSignal) => Promise<AssistantRunOutcome>
}
