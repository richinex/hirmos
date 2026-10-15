import type { Workflow } from '@/domain/workflow'
import { declarationsOf } from '@/domain/fileReading'
import type { EstimationInspection, SqlInspection } from './applicationTools'
import type { RunInspection } from './analysisRun'
import type { PreparationInspection } from './preparation'
import { dagExploration } from '@/domain/dag'
import type { StudySelectionInspection } from './studySelection'
import { adjustmentSummary } from './adjustmentSummary'
import { describeRun } from '@/domain/estimation'

export function assistantContext(
  workflow: Workflow,
  estimation: EstimationInspection | null = null,
  sql: SqlInspection | null = null,
  runs: Readonly<Record<string, RunInspection>> = {},
  preparation: PreparationInspection | null = null,
  studySelection: StudySelectionInspection | null = null,
): string {
  return JSON.stringify(
    {
      evidence: 'project-summary',
      stage: workflow.kind,
      source:
        'source' in workflow
          ? {
              name: workflow.source.name,
              fingerprint:
                workflow.kind === 'profiled' ? workflow.profile.source.fingerprint : null,
              declared: declarationsOf(workflow.source),
            }
          : null,
      estimation,
      sql,
      runs,
      preparation,
      studySelection,
      ...(workflow.kind === 'profiled'
        ? {
            rows: workflow.profile.rowCount,
            columnCount: workflow.profile.columns.length,
            columns: workflow.profile.columns.map((column) => ({
              name: column.name,
              type: column.duckdbType,
              missing: column.nullCount,
            })),
            prepared: workflow.prepared !== null,
            preparedVersion: workflow.prepared?.id ?? null,
            graphCount: workflow.dagDocuments.length,
            graphs: workflow.dagDocuments.map((document) => ({
              id: document.id,
              name: document.name,
              preparedDataset: document.preparedDataset,
              revision: document.current.id,
              graph: document.current.graph,
              nodeCount: document.current.graph.nodes.length,
              arrowCount: document.current.graph.edges.length,
              validation: document.current.validation,
              origin: document.origin,
              selected: dagExploration(document),
              interpretation:
                'Arrows and their rationales record assumptions or declared evidence, not established causal facts.',
            })),
            studyDraft: workflow.studyDraft,
            studyDraftMeaning:
              'The current editable question, distinct from saved studies and the study selected in Estimation. Identification acts on this draft. Saving it does not select it in Estimation.',
            adjustmentSelectionMeaning:
              'A recorded O-set is the recommended set chosen by the user. A supplied set does not record its origin and may be a legacy O-set; do not infer origin from the guarantee alone. Graphical optimality depends on the theorem assumptions. It does not promise the smallest standard error in this sample.',
            studies: workflow.studies.map((study) => ({
              id: study.id,
              preparedDataset: study.preparedDataset,
              dagDocument: study.dagDocument,
              dagRevision: study.dagRevision,
              treatment: study.treatment,
              outcome: study.outcome,
              estimand: study.estimand,
              adjustment: adjustmentSummary(
                workflow.identifications.find((item) => item.study === study.id)?.result ?? null,
              ),
              currentGraphRevision:
                workflow.dagDocuments.find((item) => item.id === study.dagDocument)?.current.id ??
                null,
              identificationRecords: workflow.identifications
                .filter((item) => item.study === study.id)
                .map((item) => ({ id: item.id, kind: item.result.kind, method: item.method })),
            })),
            savedStudies: workflow.studies.length,
            studyRecordAccess:
              'Use study({id}) to read one saved specification and its identification assumptions. The overview is for locating studies, not quoting their assumptions.',
            savedEstimates: workflow.estimationRuns.length,
            savedEstimationRuns: workflow.estimationRuns.map((run) => ({
              id: run.id,
              study: run.study,
              createdAt: run.createdAt,
              method: describeRun(run.configuration),
            })),
            savedTimeSeriesRuns: workflow.timeSeriesRuns.map((run) => ({
              id: run.id,
              workspace: 'time-series',
              model: run.kind,
              createdAt: run.createdAt,
              preparedDataset: run.preparedDataset,
            })),
            savedSurvivalRuns: workflow.survivalRuns.map((run) => ({
              id: run.id,
              workspace: 'survival',
              model: run.kind,
              createdAt: run.createdAt,
              preparedDataset: run.preparedDataset,
            })),
            savedResultDisclosure:
              'The indices list saved analyses, not fitted values. Use run({id, workspace}) to read a listed result. Planning stores one replaceable calculation, not a run history.',
            savedPlanningCalculation:
              workflow.powerPlanning === null
                ? null
                : { workspace: 'planning', id: 'current', model: workflow.powerPlanning.kind },
            savedDagQueryRuns: workflow.interventionQueries.map((run) => ({
              id: run.id,
              workspace: 'dag-query',
              model: run.kind,
              createdAt: run.createdAt,
              dagDocument: run.dagDocument,
              dagRevision: run.dagRevision,
            })),
            savedSwigRuns: workflow.swigAnalyses.map((run) => ({
              id: run.id,
              workspace: 'swig',
              model: run.specification.construction.kind,
              query: run.specification.query.kind,
              dagName:
                workflow.dagDocuments.find((document) => document.id === run.dagDocument)?.name ??
                null,
              createdAt: run.createdAt,
              dagDocument: run.dagDocument,
              dagRevision: run.dagRevision,
            })),
            savedDiscoveryRuns: workflow.discoveryRuns.map((run) => ({
              id: run.id,
              workspace: 'discovery',
              model: run.kind,
              createdAt: run.createdAt,
              preparedDataset: run.preparedDataset,
            })),
            savedCounterfactualRuns: workflow.counterfactualRuns.map((run) => ({
              id: run.id,
              workspace: 'counterfactual',
              model: run.kind,
              createdAt: run.createdAt,
              study: run.study,
              preparedDataset: run.preparedDataset,
            })),
            savedSensitivityRuns: workflow.sensitivityRuns.map((run) => ({
              id: run.id,
              workspace: 'sensitivity',
              model: run.kind,
              createdAt: run.createdAt,
              preparedDataset: run.preparedDataset,
            })),
            savedSurrogateRuns: workflow.surrogateRuns.map((run) => ({
              id: run.id,
              workspace: 'surrogate',
              model: run.selection.estimator,
              createdAt: run.createdAt,
              sourceFingerprint: run.sourceFingerprint,
            })),
            savedCausalModelRuns: causalModelRecords(workflow.rootCause).map(
              ({ kind, record }) => ({
                id: record.id,
                workspace: 'causal-model',
                model: kind,
                createdAt: record.createdAt,
                graph: record.graph,
              }),
            ),
          }
        : {}),
      limitations:
        'No row values or fitted results are supplied; this does not mean no analyses exist. Estimator eligibility is available only when the Estimation panel is open. Requirements belong to their specific estimator, not every result. Selecting an estimator does not run it or confirm its causal assumptions. No verified navigation map is supplied.',
    },
    null,
    2,
  )
}
import { causalModelRecords } from './causalModelRunRead'
