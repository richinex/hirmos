import type { Workflow } from '@/domain/workflow'
import type { SwigAnalysis } from '@/domain/swig'

function didControls(run: SwigAnalysis) {
  const construction = run.specification.construction
  const conclusion = run.result.conclusion
  if (construction.kind !== 'did' || conclusion.kind !== 'did') return null
  const names = (indices: readonly number[]) => indices.map((i) => run.names[i] ?? null)
  const selectedControls = names(construction.design.selected)
  const decision = conclusion.assessment.decision
  switch (decision.kind) {
    case 'supportedSubjectToOverlap':
      return {
        selectedControls,
        requiredControls: names(decision.required),
        availableControls: names(decision.available),
        selectedSupported: decision.selectedSupported,
      }
    case 'notEstablished':
      return {
        selectedControls,
        requiredPotentialControls: names(decision.potentialSet),
      }
    case 'baselineIdentity':
      return { selectedControls }
  }
}

export function readGraphRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  workspace: 'dag-query' | 'swig',
  id: string,
  view: 'summary' | 'specification' | 'evidence' | 'review',
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
  const controls = run.kind === 'swig-analysis' ? didControls(run) : null
  const common = {
    kind: 'saved-run' as const,
    workspace,
    run: run.id,
    model: run.kind,
    createdAt: run.createdAt,
    dagDocument: run.dagDocument,
    dagName: document?.name ?? null,
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
  if (view === 'review') {
    if (run.kind !== 'swig-analysis' || run.result.conclusion.kind !== 'did')
      return {
        ...common,
        view,
        review: {
          kind: 'not-supported',
          reason:
            'This saved result is not a DiD adjustment assessment. Read its specification and evidence for the graph construction or separation question.',
        },
      }
    const assessment = run.result.conclusion.assessment
    const { decision, ...restrictions } = assessment
    return {
      ...common,
      view,
      review: {
        kind: 'available',
        decision: decision.kind,
        ...controls,
        ...(decision.kind === 'supportedSubjectToOverlap'
          ? {
              reading: decision.selectedSupported
                ? 'The selected controls satisfy this sufficient rule, subject to overlap and the recorded assumptions.'
                : 'The graph supports an adjustment family, but the selected controls do not satisfy its requirements.',
            }
          : decision.kind === 'baselineIdentity'
            ? { reading: 'The outcome period is the baseline. Its difference from itself is zero.' }
            : {
                blockers: [
                  ...decision.treatedBlockers.map((b) => ({ group: 'treated', ...b })),
                  ...decision.comparisonBlockers.map((b) => ({ group: 'comparison', ...b })),
                ].map((b) => ({
                  ...b,
                  variable: run.names[b.variable] ?? null,
                  ...('treatment' in b ? { treatment: run.names[b.treatment] ?? null } : {}),
                })),
                reading:
                  'This sufficient adjustment rule is not established because required potential covariates are unavailable in one or both groups. This does not establish general nonidentification.',
              }),
        restrictions,
        interpretation:
          'Restriction flags determine which controls are required. A false flag alone does not invalidate the assessment. A covariate measured after assignment is not automatically excluded by this DiD rule. Missing arrows express assumptions in the declared graph, not findings from the data.',
        next: 'Keep the same source graph and outcome periods when checking another control set. Changing the outcome periods changes the question; it is not a repair for a refused rule.',
      },
    }
  }
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
        ...(controls ? { controls } : {}),
        names: run.names,
        construction: run.specification.construction,
        conclusion: run.result.conclusion,
        ...(run.result.conclusion.kind === 'did'
          ? {
              interpretation:
                'Read the assessment decision first. The structural restriction flags determine the required controls; a false flag alone does not mean the adjustment rule failed. Support remains conditional on the recorded assumptions and overlap.',
            }
          : {}),
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
