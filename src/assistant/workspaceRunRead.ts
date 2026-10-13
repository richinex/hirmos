import type { Workflow } from '@/domain/workflow'
import { survivalSummary, timeSeriesSummary } from './analysisSummaries'

export function readWorkspaceRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  workspace: 'time-series' | 'survival',
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const selected =
    workspace === 'time-series'
      ? workflow.timeSeriesRuns.find((run) => run.id === id)
      : workflow.survivalRuns.find((run) => run.id === id)
  if (!selected)
    return {
      kind: 'refused' as const,
      reason: `That run is not saved in the ${workspace} history. Use its saved-run index for an exact ID.`,
    }
  const prepared = workflow.prepared?.id === selected.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace,
    run: selected.id,
    createdAt: selected.createdAt,
    model: selected.kind,
    preparedDataset: selected.preparedDataset,
    provenance: {
      preparedDataset: prepared ? 'available' : 'unavailable',
      study: 'not-recorded' as const,
      graph: 'not-recorded' as const,
      notice:
        'This workspace run has no recorded study or DAG binding. A saved fit alone does not establish a causal effect.',
    },
  }
  if (view === 'evidence') return { ...common, view, evidence: selected.evidence }
  const { evidence: _evidence, ...specification } = selected
  if (view === 'specification')
    return { ...common, view, specification, preparedDatasetRecord: prepared }
  return {
    ...common,
    view,
    summary: JSON.parse(
      'specification' in selected ? timeSeriesSummary(selected) : survivalSummary(selected),
    ) as unknown,
    details:
      'This is a saved result, not a new computation. Read specification for saved columns, settings and time context, or evidence for full numerical results and diagnostics. Some model summaries contain only a receipt; that does not mean their numerical evidence is absent. Omitted diagnostics have not been checked. Retain the response in Python and print selected fields; output is limited to 3,000 characters.',
  }
}
