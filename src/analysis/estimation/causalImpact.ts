import { type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  evaluatedEnd,
  interventionStartFromTreatment,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runCausalImpact(
  configuration: Extract<EstimatorConfiguration, { kind: 'causal-impact' }>,
  {
    prepared,
    study,
    profile,
    materialise,
    columnAt,
    dispatch,
    analysis,
    identification,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    | 'prepared'
    | 'study'
    | 'profile'
    | 'materialise'
    | 'columnAt'
    | 'dispatch'
    | 'analysis'
    | 'identification'
    | 'finish'
    | 'identity'
  >,
): Promise<void> {
  const controls = prepared.columns.filter(
    (column) =>
      configuration.controls.includes(column) &&
      column !== study.outcome.column &&
      column !== study.treatment.column,
  )
  const controlVariables: StudyVariable[] = controls.map((column) => ({
    node: study.outcome.node,
    column,
    name: profile.columns.find((candidate) => candidate.id === column)?.name ?? String(column),
  }))
  const columns: NonEmptyArray<StudyVariable> = [
    study.treatment,
    study.outcome,
    ...controlVariables,
  ]
  const matrix = await materialise(columns)
  let nPre: number
  if (configuration.start.kind === 'row') nPre = configuration.start.row - 1
  else {
    const start = interventionStartFromTreatment(columnAt(matrix.values, matrix.rowCount, 0))
    if (!start.ok) {
      dispatch({
        type: 'run-failed',
        detail: `${study.treatment.name} does not switch on once: ${start.error.detail} Give the intervention row instead.`,
      })
      return
    }
    nPre = start.value
  }
  const postEnd = evaluatedEnd(configuration.window, matrix.rowCount)
  if (postEnd <= nPre) {
    dispatch({
      type: 'run-failed',
      detail: `The evaluated window ends at row ${postEnd}, which is not after the intervention row. Choose a later row.`,
    })
    return
  }
  const design = {
    outcome: 1,
    controls: controlVariables.map((_, index) => index + 2),
    nPre,
    postEnd,
  }
  const evidence =
    configuration.inference === undefined
      ? await analysis.runCausalImpact(matrix.values, matrix.rowCount, columns.length, {
          ...design,
          maxIter: configuration.maxIter,
        })
      : configuration.inference.kind === 'structural'
        ? await analysis.runStructuralCausalImpact(matrix.values, matrix.rowCount, columns.length, {
            ...design,
            draws: configuration.inference.draws,
            warmup: configuration.inference.warmup,
            seed: configuration.inference.seed,
            model: configuration.inference.model,
          })
        : await analysis.runBayesianCausalImpact(matrix.values, matrix.rowCount, columns.length, {
            ...design,
            draws: configuration.inference.draws,
            warmup: configuration.inference.warmup,
            seed: configuration.inference.seed,
            priorLevelSd: configuration.inference.priorLevelSd,
          })
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'causal-impact-run',
    configuration,
    evidence: evidence.value,
  } as const
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
    'The impact run produced no post-intervention rows.',
  )
  return
}
