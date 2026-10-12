import { type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runDiscreteModel(
  configuration: Extract<EstimatorConfiguration, { kind: 'discrete-bn-query' }>,
  {
    study,
    dispatch,
    materialise,
    analysis,
    identification,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    'study' | 'dispatch' | 'materialise' | 'analysis' | 'identification' | 'finish' | 'identity'
  >,
): Promise<void> {
  const measured = study.graph.nodes.flatMap((node) =>
    node.column === null ? [] : [{ node: node.node, column: node.column, name: node.name }],
  )
  if (measured.length !== study.graph.nodes.length) {
    dispatch({
      type: 'run-failed',
      detail: 'Every DAG node must be measured for the network.',
    })
    return
  }
  const columns = measured as unknown as NonEmptyArray<StudyVariable>
  const matrix = await materialise(columns)
  const position = (node: StudyVariable['node']) =>
    measured.findIndex((candidate) => candidate.node === node)
  const evidence = await analysis.runDiscreteBnQuery(
    matrix.values,
    matrix.rowCount,
    columns.length,
    {
      nodes: measured.map((_, index) => index),
      names: measured.map((variable) => variable.name),
      edges: study.graph.edges.map(
        ([cause, effect]) =>
          [
            position(study.graph.nodes[cause]?.node ?? study.treatment.node),
            position(study.graph.nodes[effect]?.node ?? study.outcome.node),
          ] as const,
      ),
      treatment: position(study.treatment.node),
      outcome: position(study.outcome.node),
      bins: configuration.bins,
      equivalentSampleSize: configuration.equivalentSampleSize,
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'discrete-bn-run', configuration, evidence: evidence.value } as const
  const estimate = causalEstimateFrom(study, identification, run)
  finish(
    estimate === null
      ? null
      : ({
          ...identity,
          ...run,
          method: methodIdOf(configuration.kind),
          columns,
          estimate,
        } as EstimationRunArtifact),
    'The identification is no longer identified.',
  )
  return
}
