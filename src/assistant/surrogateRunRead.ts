import type { Workflow } from '@/domain/workflow'

export function readSurrogateRun(
  workflow: Extract<Workflow, { kind: 'profiled' }>,
  id: string,
  view: 'summary' | 'specification' | 'evidence',
) {
  const run = workflow.surrogateRuns.find((item) => item.id === id)
  if (!run)
    return {
      kind: 'refused' as const,
      reason:
        'That surrogate run is not saved in this project. Read savedSurrogateRuns for available runs.',
    }
  const common = {
    kind: 'saved-run' as const,
    workspace: 'surrogate' as const,
    run: run.id,
    createdAt: run.createdAt,
    sourceFingerprint: run.sourceFingerprint,
    provenance: {
      source:
        workflow.profile.source.fingerprint === run.sourceFingerprint ? 'current' : 'different',
      study: 'not-recorded',
      graph: 'not-recorded',
    },
    details:
      'This result uses the recorded experimental and observational samples. Read the specification for their definitions and the identifying rationale. Read evidence for diagnostics and horizon results before interpreting them.',
  }
  if (view === 'evidence')
    return {
      ...common,
      view,
      evidence: run.evidence,
      diagnostics: run.diagnostics,
      paths: run.paths,
    }
  if (view === 'specification')
    return {
      ...common,
      view,
      selection: run.selection,
      rationale: run.rationale,
      sourceRows: run.sourceRows,
      rows: run.rows,
    }
  return {
    ...common,
    view,
    summary: {
      estimator: run.evidence.estimator,
      estimate: run.evidence.estimate,
      uncertainty: run.evidence.uncertainty,
      experimentalRows: run.evidence.experimentalRows,
      observationalRows: run.evidence.observationalRows,
      diagnosticCount: run.diagnostics.length,
      horizonResultCount: run.paths.results.length,
    },
  }
}
