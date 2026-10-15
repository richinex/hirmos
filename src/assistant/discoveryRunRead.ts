import type { Workflow } from '@/domain/workflow'
import { reviewDiscovery } from '@/domain/discoveryReview'

export function readDiscoveryRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence' | 'review',
) {
  const run = workflow.discoveryRuns.find((item) => item.id === id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That discovery run is not saved in this project. Read savedDiscoveryRuns for available runs.',
    }
  const prepared = workflow.prepared?.id === run.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace: 'discovery' as const,
    run: run.id,
    model: run.kind,
    createdAt: run.createdAt,
    projectOrigin: workflow.origin,
    dateMeaning:
      'createdAt is the saved run timestamp, not the time it was read. Project origin does not establish whether an individual run shipped with the example or was added later.',
    preparedDataset: run.preparedDataset,
    provenance: {
      preparedDataset: prepared ? 'available' : 'unavailable',
      graph: 'discovery-output',
    },
    details:
      'Discovery reports graph structure under the method assumptions. It does not estimate an intervention effect. Read evidence for edges, endpoint marks and diagnostics; do not treat every edge as a directed causal arrow.',
  }
  if (view === 'review') return { ...common, view, review: reviewDiscovery(run) }
  if (view === 'evidence')
    return {
      ...common,
      view,
      evidence: run.result,
      variables: run.variables,
      nodes: 'nodes' in run ? run.nodes : null,
    }
  if (view === 'summary')
    return {
      ...common,
      view,
      configuration: run.configuration,
      variables: run.variables,
      eligibility: run.eligibility,
      review:
        'Read view="review" for named background constraints, method-specific findings and recorded requirements before interpreting this graph.',
    }
  const { result: _result, ...specification } = run
  return { ...common, view, specification, preparedDatasetRecord: prepared }
}
