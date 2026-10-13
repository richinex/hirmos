import type { Workflow } from '@/domain/workflow'
import type { SensitivityRunArtifact } from '@/domain/sensitivity'
import { assertNever } from '@/domain/dop'

export function sensitivitySource(run: SensitivityRunArtifact) {
  return run.kind === 'honest-did-run'
    ? {
        workspace: run.source.kind === 'estimation' ? 'estimation' : 'time-series',
        id: run.source.run,
      }
    : { workspace: 'estimation', id: run.estimationRun }
}

function summary(run: SensitivityRunArtifact) {
  switch (run.kind) {
    case 'honest-did-run':
      return {
        estimate: run.evidence.estimate,
        standardError: run.evidence.standardError,
        conventional: run.evidence.conventional,
        results: run.evidence.results,
        configuration: run.evidence.request.configuration,
      }
    case 'did-sensitivity-run': {
      const e = run.evidence
      return {
        estimate: e.estimate,
        standardError: e.standardError,
        baseline: e.baseline,
        scenarios: e.request.scenarios.map((shares, i) => ({ shares, bounds: e.scenarios[i] })),
        robustnessValue: e.robustnessValue,
        robustnessValueCi: e.robustnessValueCi,
        level: e.request.level,
        null: e.request.null,
        rho: e.request.rho,
        optimizerStatus: e.optimizerStatus,
      }
    }
    case 'linear-refutation-run':
      return { refuters: run.refuters, diagnostics: run.diagnostics }
    case 'dml-refutation-run':
      return { refuters: run.refuters }
    case 'unobserved-confounding-run':
      return {
        configuration: run.configuration,
        notice: 'Read the evidence view for the recorded sensitivity grid.',
      }
    default:
      return assertNever(run)
  }
}

export function readSensitivityRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const run = workflow.sensitivityRuns.find((item) => item.id === id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That sensitivity run is not saved in this project. Read savedSensitivityRuns for available runs.',
    }
  const source = sensitivitySource(run)
  const available =
    source.workspace === 'estimation'
      ? workflow.estimationRuns.some((r) => r.id === source.id)
      : workflow.timeSeriesRuns.some((r) => r.id === source.id)
  const common = {
    kind: 'saved-run' as const,
    workspace: 'sensitivity' as const,
    run: run.id,
    model: run.kind,
    createdAt: run.createdAt,
    source,
    preparedDataset: run.preparedDataset,
    provenance: {
      sourceRun: available ? 'available' : 'missing',
      preparedDataset: workflow.prepared?.id === run.preparedDataset ? 'available' : 'unavailable',
    },
    details:
      'This sensitivity result belongs to its recorded source run. Read that run for the original effect and assumptions. A sensitivity bound is conditional on the specified restrictions; it is not an estimate of confounding or a trend violation.',
  }
  if (view === 'evidence') return { ...common, view, evidence: run.evidence }
  if (view === 'summary') return { ...common, view, summary: summary(run) }
  const { evidence, ...record } = run
  return {
    ...common,
    view,
    specification: record,
    request: 'request' in evidence ? evidence.request : null,
  }
}
