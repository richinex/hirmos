import { z } from 'zod'
import {
  createDagDocument,
  describeDagCreateProblem,
  describeDagEditProblem,
  describeDagVariableEditProblem,
  reviseDagWithEdge,
  reviseDagWithoutEdge,
  reviseDagEdgeDetails,
  reviseDagWithLatentNode,
  reviseDagRenamingLatentNode,
  reviseDagWithoutLatentNode,
  selectDagExploration,
  type DagDocument,
} from '@/domain/dag'
import { assertNever, err, ok, type Result } from '@/domain/dop'
import type { Workflow, WorkflowEvent } from '@/domain/workflow'
import { proposalReviewSchema, type ProposalReview } from './proposalReview'

const name = z.string().trim().min(1).max(200)
const identifier = z.string().min(1).max(4096)
const timing = z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('contemporaneous') }).strict(),
  z.object({ kind: z.literal('lagged'), lag: z.number().int().positive().max(10000) }).strict(),
])
const operationSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('add-arrow'),
      cause: name,
      effect: name,
      rationale: z.string().trim().min(1).max(2000).nullable(),
      timing,
    })
    .strict(),
  z.object({ kind: z.literal('remove-arrow'), edge: identifier }).strict(),
  z
    .object({
      kind: z.literal('edit-arrow'),
      edge: identifier,
      rationale: z.string().trim().max(2000),
      timing,
    })
    .strict(),
  z.object({ kind: z.literal('add-latent'), name }).strict(),
  z.object({ kind: z.literal('rename-latent'), variable: name, name }).strict(),
  z.object({ kind: z.literal('remove-latent'), variable: name }).strict(),
  z
    .object({
      kind: z.literal('select-variables'),
      treatment: name.nullable(),
      outcome: name.nullable(),
    })
    .strict(),
])
export const graphProposalSchema = proposalReviewSchema
  .extend({
    target: z.discriminatedUnion('kind', [
      z
        .object({
          kind: z.literal('create'),
          name,
          basis: z.enum(['domain-knowledge', 'experimental-design']),
        })
        .strict(),
      z.object({ kind: z.literal('edit'), document: name }).strict(),
    ]),
    operations: z.array(operationSchema).min(1).max(50).readonly(),
  })
  .strict()
type Operation = z.infer<typeof operationSchema>
export interface GraphProposal extends ProposalReview {
  readonly kind: 'graph-proposal'
  readonly binding: string
  readonly target: z.infer<typeof graphProposalSchema>['target']
  readonly operations: readonly Operation[]
  readonly document: DagDocument
  readonly changes: readonly { readonly title: string; readonly rationale: string | null }[]
}

export function describeGraphOperation(
  operation: Operation,
  document: DagDocument,
): { title: string; rationale: string | null } {
  const arrow = (id: string) => {
    const edge = document.current.graph.edges.find((edge) => edge.id === id)
    const name = (id: string) =>
      document.current.graph.nodes.find((node) => node.id === id)?.name ?? 'Unknown variable'
    return edge ? `${name(edge.cause)} → ${name(edge.effect)}` : 'Selected arrow'
  }
  const timingLabel = (value: z.infer<typeof timing>) =>
    value.kind === 'lagged' ? ` (lag ${value.lag})` : ''
  switch (operation.kind) {
    case 'add-arrow':
      return {
        title: `Add ${operation.cause} → ${operation.effect}${timingLabel(operation.timing)}`,
        rationale: operation.rationale,
      }
    case 'edit-arrow': {
      const previous = document.current.graph.edges.find(
        (edge) => edge.id === operation.edge,
      )?.timing
      const change =
        previous?.kind === 'lagged' && operation.timing.kind === 'contemporaneous'
          ? ` (lag ${previous.lag} removed)`
          : timingLabel(operation.timing)
      return { title: `Edit ${arrow(operation.edge)}${change}`, rationale: operation.rationale }
    }
    case 'remove-arrow':
      return { title: `Remove ${arrow(operation.edge)}`, rationale: null }
    case 'add-latent':
      return { title: `Add unmeasured variable: ${operation.name}`, rationale: null }
    case 'rename-latent':
      return { title: `Rename ${operation.variable} to ${operation.name}`, rationale: null }
    case 'remove-latent':
      return { title: `Remove unmeasured variable: ${operation.variable}`, rationale: null }
    case 'select-variables':
      return {
        title: `Treatment: ${operation.treatment ?? 'None'}; outcome: ${operation.outcome ?? 'None'}`,
        rationale: null,
      }
    default:
      return assertNever(operation)
  }
}

export function graphBinding(workflow: Workflow): string | null {
  if (workflow.kind !== 'profiled' || !workflow.prepared) return null
  return JSON.stringify({
    project: workflow.project.id,
    prepared: workflow.prepared.id,
    graphs: workflow.dagDocuments.map((item) => ({
      id: item.id,
      revision: item.current.id,
      exploration: item.exploration,
    })),
  })
}

function revise(document: DagDocument, operation: Operation): Result<DagDocument, string> {
  const node = (value: string) => document.current.graph.nodes.find((item) => item.name === value)
  switch (operation.kind) {
    case 'add-arrow': {
      const cause = node(operation.cause)
      const effect = node(operation.effect)
      if (!cause || !effect) return err('Use exact variable names from the graph.')
      const result = reviseDagWithEdge(
        document,
        cause.id,
        effect.id,
        operation.rationale,
        operation.timing,
      )
      return result.ok ? result : err(describeDagEditProblem(result.error))
    }
    case 'remove-arrow':
    case 'edit-arrow': {
      const edge = document.current.graph.edges.find((item) => item.id === operation.edge)
      if (!edge) return err('That arrow is not in the graph.')
      const result =
        operation.kind === 'remove-arrow'
          ? reviseDagWithoutEdge(document, edge.id)
          : reviseDagEdgeDetails(document, edge.id, operation.rationale, operation.timing)
      return result.ok ? result : err(describeDagEditProblem(result.error))
    }
    case 'add-latent': {
      const result = reviseDagWithLatentNode(document, operation.name)
      return result.ok ? result : err(describeDagVariableEditProblem(result.error))
    }
    case 'rename-latent':
    case 'remove-latent': {
      const variable = node(operation.variable)
      if (!variable) return err('That variable is not in the graph.')
      const result =
        operation.kind === 'rename-latent'
          ? reviseDagRenamingLatentNode(document, variable.id, operation.name)
          : reviseDagWithoutLatentNode(document, variable.id)
      return result.ok ? result : err(describeDagVariableEditProblem(result.error))
    }
    case 'select-variables': {
      const treatment = operation.treatment === null ? null : node(operation.treatment)
      const outcome = operation.outcome === null ? null : node(operation.outcome)
      if (treatment === undefined || outcome === undefined)
        return err('Choose observed variables from the graph.')
      const selected = selectDagExploration(document, {
        treatment: treatment?.id ?? null,
        outcome: outcome?.id ?? null,
      })
      return selected.ok
        ? selected
        : err('Choose different observed variables for treatment and outcome.')
    }
    default:
      return assertNever(operation)
  }
}

export function proposeGraph(workflow: Workflow, raw: unknown): Result<GraphProposal, string> {
  const binding = graphBinding(workflow)
  if (workflow.kind !== 'profiled' || !workflow.prepared || binding === null)
    return err('Prepare a dataset before proposing a graph.')
  const parsed = graphProposalSchema.safeParse(raw)
  if (!parsed.success) return err('Use graph_schema for the proposal fields and operation types.')
  const { target, operations, summary, checks } = parsed.data
  let document: DagDocument
  if (target.kind === 'create') {
    if (workflow.dagDocuments.some((item) => item.name === target.name))
      return err('A graph already has that name. Edit it or choose another name.')
    const created = createDagDocument(
      target.name,
      { kind: target.basis },
      workflow.prepared,
      workflow.profile,
      workflow.discoveryRuns,
    )
    if (!created.ok) return err(describeDagCreateProblem(created.error))
    document = created.value
  } else {
    const existing = workflow.dagDocuments.find((item) => item.id === target.document)
    if (!existing || existing.preparedDataset !== workflow.prepared.id)
      return err('Choose a graph for the current prepared data.')
    document = existing
  }
  const changes: { title: string; rationale: string | null }[] = []
  for (const operation of operations) {
    changes.push(describeGraphOperation(operation, document))
    const result = revise(document, operation)
    if (!result.ok) return result
    document = result.value
  }
  return ok({
    kind: 'graph-proposal',
    binding,
    target,
    operations,
    summary,
    checks,
    document,
    changes,
  })
}

export function graphProposalEvent(
  workflow: Workflow,
  proposal: GraphProposal,
): Result<WorkflowEvent, string> {
  if (graphBinding(workflow) !== proposal.binding)
    return err('The data or graphs changed. Ask for a new proposal.')
  return ok({
    type: proposal.target.kind === 'create' ? 'dag-document-created' : 'dag-document-revised',
    document: proposal.document,
  })
}
