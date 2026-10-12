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

export async function runCountSeries(
  configuration: Extract<EstimatorConfiguration, { kind: 'negative-binomial-ingarch' }>,
  {
    study,
    identification,
    materialise,
    columnAt,
    dispatch,
    analysis,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    | 'study'
    | 'identification'
    | 'materialise'
    | 'columnAt'
    | 'dispatch'
    | 'analysis'
    | 'finish'
    | 'identity'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [
    study.treatment,
    study.outcome,
    ...identification.result.adjustment.variables,
  ]
  const matrix = await materialise(columns)
  const outcome = columnAt(matrix.values, matrix.rowCount, 1)
  if (outcome.some((value) => value < 0 || !Number.isInteger(value))) {
    dispatch({
      type: 'run-failed',
      detail: `${study.outcome.name} is not a count: it holds negative or fractional values.`,
    })
    return
  }
  const regressors = [0, ...identification.result.adjustment.variables.map((_, index) => index + 2)]
  const baselineRegressors = regressors.map(
    (column) => matrix.values[column * matrix.rowCount + matrix.rowCount - 1] ?? 0,
  )
  const evidence = await analysis.runNegativeBinomialIngarch(
    matrix.values,
    matrix.rowCount,
    columns.length,
    {
      outcome: 1,
      link: configuration.link,
      regressors,
      pastObservationLags: configuration.pastObservationLags,
      pastMeanLags: configuration.pastMeanLags,
      externalRegressors: regressors.map(() => false),
      horizon: configuration.horizon,
      baselineRegressors,
      interventionRegressor: 0,
      controlValue: configuration.controlValue,
      treatmentValue: configuration.treatmentValue,
      schedule: configuration.schedule,
    },
    (progress) => dispatch({ type: 'run-progressed', progress }),
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'negative-binomial-ingarch-run',
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
    'The INGARCH run produced no forecast path.',
  )
  return
}
