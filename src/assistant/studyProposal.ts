import { z } from 'zod'
import { assertNever, err, ok, type Result } from '@/domain/dop'
import {
  EMPTY_STUDY_DRAFT,
  QUANTILE_GROUP_CHOICES,
  describeStudyDesignProblem,
  describeEstimand,
  readyStudySpecification,
  type StudyDesignDraft,
} from '@/domain/study'
import type { DagDocument, DagNodeId } from '@/domain/dag'
import type { Workflow, WorkflowEvent } from '@/domain/workflow'
import { graphBinding } from './graphProposal'
import { proposalReviewSchema, type ProposalReview } from './proposalReview'

const variable = z
  .string()
  .min(1)
  .max(4096)
  .describe('Exact observed variable name from graph.nodes[].name, not the node ID.')
const targetSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.enum([
        'average-treatment-effect',
        'average-treatment-effect-on-treated',
        'average-treatment-effect-on-controls',
        'overlap-weighted-average-treatment-effect',
        'average-partial-effect',
        'variance-weighted-average-partial-effect',
      ]),
    })
    .strict(),
  z
    .object({
      kind: z.literal('conditional-average-treatment-effect'),
      modifier: variable,
      grouping: z.discriminatedUnion('kind', [
        z.object({ kind: z.literal('levels') }).strict(),
        z
          .object({
            kind: z.literal('quantiles'),
            bins: z
              .number()
              .int()
              .refine(
                (value) => QUANTILE_GROUP_CHOICES.includes(value),
                'Choose 2, 3, 4 or 5 quantile groups.',
              ),
          })
          .strict(),
      ]),
    })
    .strict(),
  z
    .object({
      kind: z.enum([
        'conditional-average-treatment-effect-per-row',
        'conditional-partial-effect-per-row',
      ]),
      modifiers: z
        .array(variable)
        .max(50)
        .refine((names) => new Set(names).size === names.length, 'Do not repeat effect modifiers.'),
    })
    .strict(),
  z
    .object({
      kind: z.literal('local-cutoff-effect'),
      running: variable,
      cutoff: z.number().finite(),
    })
    .strict(),
])
export const studyProposalSchema = proposalReviewSchema
  .extend({
    document: z.string().min(1).max(4096),
    treatment: variable,
    outcome: variable,
    target: targetSchema,
    assignment: z
      .object({
        kind: z.enum(['randomised', 'policy-change', 'observed-choice']),
        description: z.string().trim().min(1).max(2000),
      })
      .strict(),
    consistencyRationale: z.string().trim().min(1).max(2000).nullable(),
    noInterferenceRationale: z.string().trim().min(1).max(2000).nullable(),
  })
  .strict()

export interface StudyProposal extends ProposalReview {
  readonly kind: 'study-proposal'
  readonly binding: string
  readonly draft: StudyDesignDraft
  readonly question: string
  readonly specification: z.infer<typeof studyProposalSchema>
  readonly graphName: string
  readonly graphRevision: string
}

export function studyBinding(workflow: Workflow): string | null {
  const graph = graphBinding(workflow)
  return graph === null || workflow.kind !== 'profiled'
    ? null
    : JSON.stringify({ graph, draft: workflow.studyDraft })
}

function observed(document: DagDocument, name: string): Result<DagNodeId, string> {
  const matches = document.current.graph.nodes.filter((node) => node.name === name)
  const namedById = document.current.graph.nodes.find((node) => node.id === name)
  if (matches.length === 0 && namedById)
    return err(
      `You supplied a node ID. Use the variable name ${JSON.stringify(namedById.name)} instead.`,
    )
  if (matches.length !== 1 || matches[0]?.kind !== 'observed')
    return err(
      `Choose an observed variable with the exact name ${JSON.stringify(name)} in this graph.`,
    )
  return ok(matches[0].id)
}

function targetDraft(
  document: DagDocument,
  target: z.infer<typeof targetSchema>,
): Result<Pick<StudyDesignDraft, 'modifier' | 'modifiers' | 'grouping' | 'cutoff'>, string> {
  const base = { modifier: null, modifiers: [], grouping: EMPTY_STUDY_DRAFT.grouping }
  switch (target.kind) {
    case 'average-treatment-effect':
    case 'average-treatment-effect-on-treated':
    case 'average-treatment-effect-on-controls':
    case 'overlap-weighted-average-treatment-effect':
    case 'average-partial-effect':
    case 'variance-weighted-average-partial-effect':
      return ok(base)
    case 'conditional-average-treatment-effect': {
      const modifier = observed(document, target.modifier)
      return modifier.ok
        ? ok({ ...base, modifier: modifier.value, grouping: target.grouping })
        : modifier
    }
    case 'conditional-average-treatment-effect-per-row':
    case 'conditional-partial-effect-per-row': {
      const modifiers: DagNodeId[] = []
      for (const name of target.modifiers) {
        const modifier = observed(document, name)
        if (!modifier.ok) return modifier
        modifiers.push(modifier.value)
      }
      return ok({ ...base, modifiers })
    }
    case 'local-cutoff-effect': {
      const running = observed(document, target.running)
      return running.ok
        ? ok({ ...base, cutoff: { variable: running.value, value: String(target.cutoff) } })
        : running
    }
    default:
      return assertNever(target)
  }
}

export function proposeStudy(workflow: Workflow, raw: unknown): Result<StudyProposal, string> {
  const binding = studyBinding(workflow)
  if (workflow.kind !== 'profiled' || !workflow.prepared || binding === null)
    return err('Prepare data and select a graph before proposing a study.')
  const parsed = studyProposalSchema.safeParse(raw)
  if (!parsed.success)
    return err(
      'Use study_schema for the required fields. Supply the fields for the chosen target only; use null for an unstated rationale.',
    )
  const spec = parsed.data
  const document = workflow.dagDocuments.find((item) => item.id === spec.document)
  if (!document) return err('Choose a graph from this project.')
  const treatment = observed(document, spec.treatment)
  if (!treatment.ok) return treatment
  const outcome = observed(document, spec.outcome)
  if (!outcome.ok) return outcome
  const target = targetDraft(document, spec.target)
  if (!target.ok) return target
  const draft: StudyDesignDraft = {
    ...target.value,
    dagDocument: document.id,
    treatment: treatment.value,
    outcome: outcome.value,
    estimand: spec.target.kind,
    assignment: spec.assignment,
    consistencyRationale: spec.consistencyRationale ?? '',
    noInterferenceRationale: spec.noInterferenceRationale ?? '',
  }
  const ready = readyStudySpecification(
    draft,
    workflow.dagDocuments,
    workflow.prepared,
    workflow.studies,
  )
  if (!ready.ok) return err(describeStudyDesignProblem(ready.error))
  return ok({
    kind: 'study-proposal',
    binding,
    draft,
    question: describeEstimand(ready.value),
    specification: spec,
    graphName: document.name,
    graphRevision: document.current.id,
    summary: spec.summary,
    checks: spec.checks,
  })
}

export function studyProposalEvent(
  workflow: Workflow,
  proposal: StudyProposal,
): Result<WorkflowEvent, string> {
  if (
    studyBinding(workflow) !== proposal.binding ||
    workflow.kind !== 'profiled' ||
    !workflow.prepared
  )
    return err('The data, graph or study draft changed. Ask for a new proposal.')
  const ready = readyStudySpecification(
    proposal.draft,
    workflow.dagDocuments,
    workflow.prepared,
    workflow.studies,
  )
  if (!ready.ok) return err(describeStudyDesignProblem(ready.error))
  return ok({ type: 'study-draft-changed', draft: proposal.draft })
}
