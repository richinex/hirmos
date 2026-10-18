import { z } from 'zod'
import { brand, err, ok, type Result } from './dop'
import type { DagDocument } from './dag'
import { projectSwigGraph } from './swigProjection'

const node = z.number().int().nonnegative().max(255)

/** One decision in time order and the observed nodes its rule may read. */
export const decisionStageSchema = z.object({ node, reads: z.array(node).max(256) }).strict()

export const decisionChecksRequestSchema = z
  .object({
    names: z.array(z.string().min(1)).min(2).max(256),
    edges: z.array(z.tuple([node, node])).max(65_536),
    unobserved: z.array(node).max(256),
    decisions: z.array(decisionStageSchema).min(1).max(32),
    objective: node,
  })
  .strict()

export type DecisionChecksRequest = z.infer<typeof decisionChecksRequestSchema>

export const decisionChecksEvidenceSchema = z
  .object({
    stages: z
      .array(
        z.object({ decision: node, conditioning: z.array(node), blocked: z.boolean() }).strict(),
      )
      .min(1)
      .max(32),
    admissible: z.boolean(),
    possiblyOptimal: z.array(z.array(node)).max(4_096),
    /** For each decision, what a copier must read to keep the demonstrator's value; null when no such sets exist. Frames checked before this field existed omit it. */
    imitation: z.array(z.array(node)).nullable().optional(),
  })
  .strict()

export type DecisionChecksEvidence = z.infer<typeof decisionChecksEvidenceSchema>

/** Names and evidence belong to the recorded revision, never the currently open DAG. */
export const decisionFrameSchema = z
  .object({
    kind: z.literal('decision-frame'),
    id: z
      .string()
      .uuid()
      .transform((value) => brand<string, 'DecisionFrameId'>(value)),
    dagDocument: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'DagDocumentId'>(value)),
    dagRevision: z
      .string()
      .min(1)
      .transform((value) => brand<string, 'DagRevisionId'>(value)),
    createdAt: z.string().datetime(),
    rationale: z.string().trim(),
    request: decisionChecksRequestSchema,
    result: decisionChecksEvidenceSchema,
  })
  .strict()
  .superRefine((record, context) => {
    const decisions = record.request.decisions.map((stage) => stage.node)
    if (
      record.result.stages.length !== decisions.length ||
      record.result.stages.some((stage, i) => stage.decision !== decisions[i]) ||
      record.result.admissible !== record.result.stages.every((stage) => stage.blocked)
    )
      context.addIssue({
        code: 'custom',
        message: 'The checks do not match the recorded decisions.',
      })
  })

export type DecisionFrame = z.infer<typeof decisionFrameSchema>

/** Decisions in time order, then the objective, by the recorded revision's names. */
export function decisionFrameTitle(frame: DecisionFrame): string {
  const names = frame.request.names
  const decisions = frame.request.decisions.map((stage) => names[stage.node]).join(', then ')
  return `${decisions} for ${names[frame.request.objective]}`
}

export type DecisionGraph = {
  readonly names: readonly string[]
  readonly edges: readonly (readonly [number, number])[]
  readonly unobserved: readonly number[]
}

export type DecisionGraphProblem =
  | { readonly kind: 'invalid-graph' }
  | { readonly kind: 'time-expansion-required' }
  | { readonly kind: 'too-many-variables' }

/** The current revision's nodes and arrows, with every unmeasured node marked unobserved. */
export function decisionGraph(document: DagDocument): Result<DecisionGraph, DecisionGraphProblem> {
  const projected = projectSwigGraph(document, { kind: 'explicit' })
  if (!projected.ok) {
    const kind = projected.error.kind
    return err(
      kind === 'time-expansion-required' || kind === 'too-many-variables'
        ? { kind }
        : { kind: 'invalid-graph' },
    )
  }
  const measured = new Set(projected.value.measured)
  return ok({
    names: projected.value.names,
    edges: projected.value.edges,
    unobserved: projected.value.names.flatMap((_, i) => (measured.has(i) ? [] : [i])),
  })
}

export function describeDecisionGraphProblem(problem: DecisionGraphProblem): string {
  switch (problem.kind) {
    case 'invalid-graph':
      return 'Resolve the structural issues in this DAG before framing a decision on it.'
    case 'time-expansion-required':
      return 'This graph uses lagged arrows. Give each decision and state its own node per period before framing a decision on it.'
    case 'too-many-variables':
      return 'Decision checks support up to 256 nodes.'
  }
}

/** Observed variables settled before `decision`: not caused by it, and not the objective. */
export function readableInputs(
  graph: DecisionGraph,
  decision: number,
  objective: number | null,
): readonly number[] {
  const later = new Set([decision])
  const queue = [decision]
  while (queue.length > 0) {
    const cause = queue.pop()!
    for (const [from, to] of graph.edges)
      if (from === cause && !later.has(to)) {
        later.add(to)
        queue.push(to)
      }
  }
  const hidden = new Set(graph.unobserved)
  return graph.names.flatMap((_, i) =>
    later.has(i) || hidden.has(i) || i === objective ? [] : [i],
  )
}

export type DecisionDraft = {
  readonly decisions: readonly { readonly node: number; readonly reads: readonly number[] }[]
  readonly objective: number | null
}

export type DecisionDraftProblem =
  | { readonly kind: 'no-decision' }
  | { readonly kind: 'no-objective' }
  | { readonly kind: 'unobserved'; readonly node: number }
  | { readonly kind: 'objective-is-decision' }
  | { readonly kind: 'repeated-decision'; readonly node: number }

/** A draft is ready when it names at least one observed decision, an observed objective, and observed inputs. */
export function readyDecisionChecks(
  graph: DecisionGraph,
  draft: DecisionDraft,
): Result<DecisionChecksRequest, DecisionDraftProblem> {
  if (draft.decisions.length === 0) return err({ kind: 'no-decision' })
  if (draft.objective === null) return err({ kind: 'no-objective' })
  const hidden = new Set(graph.unobserved)
  const seen = new Set<number>()
  for (const stage of draft.decisions) {
    if (seen.has(stage.node)) return err({ kind: 'repeated-decision', node: stage.node })
    seen.add(stage.node)
    for (const index of [stage.node, ...stage.reads])
      if (hidden.has(index)) return err({ kind: 'unobserved', node: index })
  }
  if (seen.has(draft.objective)) return err({ kind: 'objective-is-decision' })
  if (hidden.has(draft.objective)) return err({ kind: 'unobserved', node: draft.objective })
  return ok({
    names: [...graph.names],
    edges: graph.edges.map(([a, b]) => [a, b] as [number, number]),
    unobserved: [...graph.unobserved],
    decisions: draft.decisions.map((stage) => ({
      node: stage.node,
      reads: [...stage.reads].sort((a, b) => a - b),
    })),
    objective: draft.objective,
  })
}

export function describeDecisionDraftProblem(
  problem: DecisionDraftProblem,
  names: readonly string[],
): string {
  switch (problem.kind) {
    case 'no-decision':
      return 'Add at least one decision.'
    case 'no-objective':
      return 'Choose the objective.'
    case 'unobserved':
      return `${names[problem.node] ?? 'This node'} is unobserved, so it cannot be a decision, the objective or an input to a rule.`
    case 'objective-is-decision':
      return 'The objective must differ from every decision.'
    case 'repeated-decision':
      return `${names[problem.node] ?? 'This node'} is already a decision.`
  }
}
