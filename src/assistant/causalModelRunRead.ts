import type { RootCauseWorkspace } from '@/domain/rootCauseAnalysis'
import type { Workflow } from '@/domain/workflow'
import { assertNever } from '@/domain/dop'

export function causalModelRecords(workspace: RootCauseWorkspace) {
  return [
    ...workspace.runs.map((record) => ({ kind: 'attribution' as const, record })),
    ...workspace.effects.map((record) => ({ kind: 'effects' as const, record })),
    ...workspace.influences.map((record) => ({ kind: 'influence' as const, record })),
    ...workspace.checks.map((record) => ({ kind: 'checks' as const, record })),
  ]
}

function summary(entry: ReturnType<typeof causalModelRecords>[number]) {
  switch (entry.kind) {
    case 'attribution': {
      const { summary: values, ...outcome } = entry.record.evidence.outcome
      const { replicates: _replicates, ...aggregate } = values
      return { ...outcome, summary: aggregate }
    }
    case 'effects': {
      const { replicates: _replicates, ...aggregate } = entry.record.evidence
      return aggregate
    }
    case 'influence':
      return {
        outcome: entry.record.evidence.outcome,
        mechanisms: entry.record.evidence.mechanisms,
      }
    case 'checks': {
      const evidence = entry.record.evidence
      return {
        mechanisms: evidence.mechanisms,
        invertibility: evidence.invertibility,
        overallDivergence: evidence.overallDivergence,
        ...('verdict' in evidence
          ? {
              verdict: evidence.verdict,
              pValueLmc: evidence.pValueLmc,
              pValueTpa: evidence.pValueTpa,
            }
          : { scope: evidence.scope }),
      }
    }
    default:
      return assertNever(entry)
  }
}

export function readCausalModelRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const entry = causalModelRecords(workflow.rootCause).find(({ record }) => record.id === id)
  if (!entry)
    return {
      kind: 'refused' as const,
      reason:
        'That causal-model run is not saved in this project. Read savedCausalModelRuns for exact IDs.',
    }
  const record = entry.record
  const document = workflow.dagDocuments.find((item) => item.id === record.graph.dagDocument)
  const revision = document?.audit.find((item) => item.id === record.graph.dagRevision) ?? null
  const graph = !document
    ? 'missing-document'
    : !revision
      ? 'missing-revision'
      : document.current.id === revision.id
        ? 'current'
        : 'earlier'
  const prepared = workflow.prepared?.id === record.graph.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace: 'causal-model' as const,
    run: id,
    model: entry.kind,
    createdAt: record.createdAt,
    binding: record.graph,
    provenance: {
      graph,
      preparedDataset: prepared ? 'available' : 'unavailable',
      notice:
        graph === 'current'
          ? null
          : 'This result uses its recorded graph revision, not the current graph. An unavailable revision is not replaced.',
    },
  }
  if (view === 'evidence') return { ...common, view, evidence: record.evidence }
  if (view === 'specification')
    return {
      ...common,
      view,
      specification: record.model,
      graphRevision: revision,
      preparedDatasetRecord: prepared,
      comparison: 'comparison' in record ? record.comparison : null,
    }
  return {
    ...common,
    view,
    names: record.model.names,
    summary: summary(entry),
    details:
      'Saved model result, not a new computation. Read specification for the original graph and settings, or evidence for full numerical output. Simulation ranges are not automatically confidence intervals for a causal effect.',
  }
}
