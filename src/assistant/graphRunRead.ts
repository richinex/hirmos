import type { Workflow } from '@/domain/workflow'

export function readGraphRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  workspace: 'dag-query' | 'swig',
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const run =
    workspace === 'swig'
      ? workflow.swigAnalyses.find((r) => r.id === id)
      : workflow.interventionQueries.find((r) => r.id === id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That graph analysis is not saved in this workspace. Read the saved-run index for an exact ID.',
    }
  const document = workflow.dagDocuments.find((d) => d.id === run.dagDocument)
  const revision = document?.audit.find((r) => r.id === run.dagRevision) ?? null
  const prepared = workflow.prepared?.id === run.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace,
    run: run.id,
    model: run.kind,
    createdAt: run.createdAt,
    dagDocument: run.dagDocument,
    dagRevision: run.dagRevision,
    preparedDataset: run.preparedDataset,
    provenance: {
      graph: !document
        ? 'missing-document'
        : !revision
          ? 'missing-revision'
          : document.current.id === run.dagRevision
            ? 'current'
            : 'earlier',
      preparedDataset: prepared ? 'available' : 'unavailable',
    },
    details:
      workspace === 'swig'
        ? 'This is a graphical assessment under recorded assumptions, not a numerical treatment-effect estimate.'
        : 'This query belongs to its recorded graph and fitted model. Its causal interpretation depends on those assumptions, not on the current canvas.',
  }
  const evidence = run.kind === 'intervention-query' ? run.route.result : run.result
  if (view === 'evidence') return { ...common, view, evidence }
  if (view === 'specification') {
    if (run.kind === 'intervention-query') {
      const { result: _result, ...route } = run.route
      return {
        ...common,
        view,
        specification: { set: run.set, read: run.read, route },
        graphRevision: revision,
        preparedDatasetRecord: prepared,
      }
    }
    const { result: _result, ...specification } = run
    return {
      ...common,
      view,
      specification,
      graphRevision: revision,
      preparedDatasetRecord: prepared,
    }
  }
  return run.kind === 'swig-analysis'
    ? {
        ...common,
        view,
        names: run.names,
        construction: run.specification.construction,
        conclusion: run.result.conclusion,
        details: `${common.details} Read evidence for the transformed graph and specification for node roles and rationale.`,
      }
    : {
        ...common,
        view,
        query:
          run.kind === 'intervention-query'
            ? { set: run.set, read: run.read, route: run.route.kind }
            : run.specification,
        details: `${common.details} Read evidence for the saved distribution and diagnostics.`,
      }
}
