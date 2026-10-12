import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  methodIdOf,
  stationaryMarksOf,
  type CausalEffectsAdjustmentProblem,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

const timeIndexedNodeLabel = (
  node: readonly [number, number],
  names: readonly string[],
): string => {
  const variable = names[node[0]] ?? `Variable ${node[0] + 1}`
  return node[1] === 0 ? `${variable} (t)` : `${variable} (t−${Math.abs(node[1])})`
}

const describeAdjustmentProblems = (
  problems: readonly CausalEffectsAdjustmentProblem[],
  names: readonly string[],
): string =>
  problems
    .map((problem) => {
      switch (problem.kind) {
        case 'queryTreatment':
          return `${timeIndexedNodeLabel(problem.node, names)} is the treatment node in this query.`
        case 'queryOutcome':
          return `${timeIndexedNodeLabel(problem.node, names)} is the outcome node in this query.`
        case 'laterTreatmentOccurrence':
          return `${timeIndexedNodeLabel(problem.node, names)} is a later occurrence of the treatment variable than the intervention node.`
        case 'forbiddenNode':
          return `${timeIndexedNodeLabel(problem.node, names)} is excluded by the time-indexed adjustment criterion.`
        case 'openNonCausalPath':
          return 'The complete set leaves open at least one path between treatment and outcome that is not causal.'
        default:
          return assertNever(problem)
      }
    })
    .join(' ')
export async function runTemporalEffect(
  configuration: Extract<EstimatorConfiguration, { kind: 'causal-effects-total' }>,
  {
    document,
    dispatch,
    materialise,
    study,
    analysis,
    identification,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    | 'document'
    | 'dispatch'
    | 'materialise'
    | 'study'
    | 'analysis'
    | 'identification'
    | 'finish'
    | 'identity'
  >,
): Promise<void> {
  if (document === null) {
    dispatch({ type: 'run-failed', detail: 'The study’s DAG document is missing.' })
    return
  }
  const nodes = document.current.graph.nodes
  const graphVariables = nodes.map((node): StudyVariable | null =>
    node.kind === 'observed' ? { node: node.id, column: node.column, name: node.name } : null,
  )
  const observed = nodes.flatMap((node) =>
    node.kind === 'observed' ? [{ node: node.id, column: node.column, name: node.name }] : [],
  )
  if (observed.length === 0) {
    dispatch({ type: 'run-failed', detail: 'The DAG has no observed variables.' })
    return
  }
  const matrix = await materialise(observed as unknown as NonEmptyArray<StudyVariable>)
  const rows = matrix.rowCount
  const values = new Float64Array(rows * nodes.length)
  nodes.forEach((node, position) => {
    if (node.kind !== 'observed') return
    const source = observed.findIndex((candidate) => candidate.node === node.id)
    values.set(matrix.values.subarray(source * rows, (source + 1) * rows), position * rows)
  })
  const marks = stationaryMarksOf(document)
  const treatmentIndex = nodes.findIndex((node) => node.id === study.treatment.node)
  const outcomeIndex = nodes.findIndex((node) => node.id === study.outcome.node)
  const lags = marks.statLag + configuration.treatmentLag
  const evidence = await analysis.runCausalEffectsTotal(
    values,
    rows,
    nodes.length,
    {
      statLag: marks.statLag,
      graph: marks.marks,
      x: [[treatmentIndex, -configuration.treatmentLag]],
      y: [[outcomeIndex, 0]],
      hidden: marks.hidden.flatMap((variable) =>
        Array.from({ length: lags + 1 }, (_, lag) => [variable, -lag] as const),
      ),
      estimator: configuration.estimator,
      interventions: configuration.interventions,
      uncertainty: configuration.uncertainty,
    },
    (progress) => dispatch({ type: 'run-progressed', progress }),
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  if (evidence.value.fit.kind === 'invalidAdjustment') {
    dispatch({
      type: 'run-failed',
      detail: describeAdjustmentProblems(
        evidence.value.fit.problems,
        nodes.map((node) => node.name),
      ),
    })
    return
  }
  const run = {
    kind: 'causal-effects-run',
    configuration,
    evidence: evidence.value,
  } as const
  const estimate = causalEstimateFrom(study, identification, { ...run, graphVariables })
  const columns = nodes.map((node) =>
    node.kind === 'observed'
      ? { node: node.id, column: node.column, name: node.name }
      : {
          node: node.id,
          column: study.treatment.column,
          name: `${node.name} (unmeasured)`,
        },
  ) as unknown as NonEmptyArray<StudyVariable>
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
    evidence.value.noCausalPath
      ? 'The time-series graph has no causal path from the treatment to the outcome.'
      : 'The effect is not identifiable by adjustment in the projected time-series graph.',
  )
  return
}
