import { z } from 'zod'
import type { Workflow } from '@/domain/workflow'
import { configurationSettings, describeRun, headlineValue } from '@/domain/estimation'
import { resolveStudyDagRevision, studyRevisionNotice } from '@/domain/dagProvenance'
import { readWorkspaceRun } from './workspaceRunRead'
import { readCausalModelRun } from './causalModelRunRead'
import { readSensitivityRun } from './sensitivityRunRead'
import { readSurrogateRun } from './surrogateRunRead'
import { readDiscoveryRun } from './discoveryRunRead'
import { readCounterfactualRun } from './counterfactualRunRead'
import { readGraphRun } from './graphRunRead'
import { readPlanning } from './planningRead'

const runReadArgs = z
  .object({
    id: z.string().min(1).max(200),
    workspace: z
      .enum([
        'estimation',
        'time-series',
        'survival',
        'causal-model',
        'sensitivity',
        'surrogate',
        'discovery',
        'counterfactual',
        'dag-query',
        'swig',
        'planning',
      ])
      .default('estimation'),
    view: z.enum(['summary', 'specification', 'evidence']).default('summary'),
  })
  .strict()

export function readRun(workflow: Workflow, raw: unknown) {
  const parsed = runReadArgs.safeParse(raw)
  if (!parsed.success)
    return {
      kind: 'refused' as const,
      reason:
        'Supply a saved run ID and its workspace, or use workspace planning with id current to read the saved planning calculation. Views are summary, specification and evidence.',
    }
  if (workflow.kind !== 'profiled')
    return { kind: 'refused' as const, reason: 'No project with saved results is open.' }
  if (parsed.data.workspace === 'planning')
    return readPlanning(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'causal-model')
    return readCausalModelRun(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'sensitivity')
    return readSensitivityRun(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'surrogate')
    return readSurrogateRun(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'discovery')
    return readDiscoveryRun(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'counterfactual')
    return readCounterfactualRun(workflow, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace === 'dag-query' || parsed.data.workspace === 'swig')
    return readGraphRun(workflow, parsed.data.workspace, parsed.data.id, parsed.data.view)
  if (parsed.data.workspace !== 'estimation')
    return readWorkspaceRun(workflow, parsed.data.workspace, parsed.data.id, parsed.data.view)
  const run = workflow.estimationRuns.find((item) => item.id === parsed.data.id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That Estimation run is not saved in this project. Read savedEstimationRuns for available IDs.',
    }
  const study = workflow.studies.find((item) => item.id === run.study)
  const identification = workflow.identifications.find(
    (item) => item.id === run.identification && item.study === run.study,
  )
  const graph = study ? resolveStudyDagRevision(study, workflow.dagDocuments) : null
  const prepared = workflow.prepared?.id === run.preparedDataset ? workflow.prepared : null
  const common = {
    kind: 'saved-run' as const,
    workspace: 'estimation' as const,
    run: run.id,
    study: run.study,
    identification: run.identification,
    preparedDataset: run.preparedDataset,
    createdAt: run.createdAt,
    method: describeRun(run.configuration),
    subject: study ? { treatment: study.treatment.name, outcome: study.outcome.name } : null,
    provenance: {
      study: study ? 'available' : 'missing',
      identification: identification ? 'available' : 'missing',
      preparedDataset: prepared ? 'available' : 'unavailable',
      dagRevision: study?.dagRevision ?? null,
      graph: graph?.kind ?? 'missing-study',
      notice: graph
        ? studyRevisionNotice(graph)
        : 'The study for this run is unavailable. Its endpoints and graph cannot be recovered from the current controls.',
    },
  }
  switch (parsed.data.view) {
    case 'summary':
      return {
        ...common,
        view: 'summary' as const,
        estimand: run.estimate.estimand,
        effectKind: run.estimate.effect.kind,
        headline: headlineValue(run.estimate.effect),
        interval: run.estimate.interval,
        standardError: run.estimate.standardError,
        sample: run.estimate.sample,
        adjustment: run.estimate.adjustment,
        eligibility: run.eligibility,
        details:
          'This is a saved result, not a new computation. Read specification for the saved settings and original graph, or evidence for numerical diagnostics and full effects. Diagnostics omitted from this summary have not been checked. Save the response in Python and print selected fields; output is limited to 3,000 characters.',
      }
    case 'specification':
      return {
        ...common,
        view: 'specification' as const,
        configuration: run.configuration,
        settings: configurationSettings(run.configuration),
        studyRecord: study ?? null,
        identificationRecord: identification ?? null,
        graphRevision:
          graph && (graph.kind === 'current' || graph.kind === 'earlier') ? graph.revision : null,
        preparedDatasetRecord: prepared,
        encodings: run.encodings,
      }
    case 'evidence':
      return {
        ...common,
        view: 'evidence' as const,
        estimate: run.estimate,
        evidence: run.evidence,
        eligibility: run.eligibility,
      }
  }
}

export type RunRead = ReturnType<typeof readRun>
