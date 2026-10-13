import type { Workflow } from '@/domain/workflow'
import { resolveStudyDagRevision, studyRevisionNotice } from '@/domain/dagProvenance'

export function readCounterfactualRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const run = workflow.counterfactualRuns.find((item) => item.id === id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That counterfactual run is not saved in this project. Read savedCounterfactualRuns for available runs.',
    }
  const study = workflow.studies.find((item) => item.id === run.study)
  const identification = workflow.identifications.find(
    (item) => item.id === run.identification && item.study === run.study,
  )
  const graph = study ? resolveStudyDagRevision(study, workflow.dagDocuments) : null
  const prepared = workflow.prepared?.id === run.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace: 'counterfactual' as const,
    run: run.id,
    model: run.kind,
    createdAt: run.createdAt,
    study: run.study,
    identification: run.identification,
    preparedDataset: run.preparedDataset,
    subject: study ? { treatment: study.treatment.name, outcome: study.outcome.name } : null,
    provenance: {
      study: study ? 'available' : 'missing',
      identification: identification ? 'available' : 'missing',
      preparedDataset: prepared ? 'available' : 'unavailable',
      graph: graph?.kind ?? 'missing-study',
      notice: graph
        ? studyRevisionNotice(graph)
        : 'The original study is unavailable. Do not substitute the currently selected study.',
    },
    details:
      'These counterfactual predictions depend on the recorded structural model. They are not observed individual treatment effects. Read evidence for fitted equations and uncertainty before interpreting the predictions.',
  }
  if (view === 'evidence')
    return {
      ...common,
      view,
      evidence: run.evidence,
      nodes: run.nodes,
      eligibility: run.eligibility,
    }
  if (view === 'specification')
    return {
      ...common,
      view,
      configuration: run.configuration,
      nodes: run.nodes,
      studyRecord: study ?? null,
      identificationRecord: identification ?? null,
      graphRevision:
        graph && (graph.kind === 'current' || graph.kind === 'earlier') ? graph.revision : null,
      preparedDatasetRecord: prepared,
    }
  return {
    ...common,
    view,
    configuration: run.configuration,
    averageEffect: run.evidence.averageEffect,
    eligibility: run.eligibility,
  }
}
