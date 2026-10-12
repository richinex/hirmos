import { describeDesignExpansionProblem, designLayouts, expandDesign } from '@/domain/designMatrix'
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

export async function runNegbinNuts(
  configuration: Extract<EstimatorConfiguration, { kind: 'negbin-nuts' }>,
  {
    identification,
    dispatch,
    study,
    materialise,
    columnAt,
    analysis,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    | 'identification'
    | 'dispatch'
    | 'study'
    | 'materialise'
    | 'columnAt'
    | 'analysis'
    | 'finish'
    | 'identity'
  >,
): Promise<void> {
  const confounder = identification.result.adjustment.variables[0]
  if (confounder === undefined || identification.result.adjustment.variables.length !== 1) {
    dispatch({ type: 'run-failed', detail: 'This model takes exactly one confounder.' })
    return
  }
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, confounder]
  const matrix = await materialise(columns)
  const outcome = columnAt(matrix.values, matrix.rowCount, 1)
  if (outcome.some((value) => value < 0 || !Number.isInteger(value))) {
    dispatch({
      type: 'run-failed',
      detail: `${study.outcome.name} is not a count: it holds negative or fractional values.`,
    })
    return
  }
  const evidence = await analysis.runNegbinNuts(matrix.values, matrix.rowCount, columns.length, {
    treatment: 0,
    outcome: 1,
    confounder: 2,
    warmup: configuration.warmup,
    samples: configuration.samples,
    seed: configuration.seed,
  })
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'negbin-nuts-run', configuration, evidence: evidence.value } as const
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

export async function runBayesianGaussian(
  configuration: Extract<EstimatorConfiguration, { kind: 'bayesian-gaussian' }>,
  {
    study,
    identification,
    materialise,
    columnAt,
    dispatch,
    analysis,
    finish,
    identity,
    encodings,
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
    | 'encodings'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [
    study.treatment,
    study.outcome,
    ...identification.result.adjustment.variables,
  ]
  const matrix = await materialise(columns)
  if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) {
    dispatch({
      type: 'run-failed',
      detail: `The Gaussian model cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.`,
    })
    return
  }
  const design = expandDesign(
    matrix.values,
    matrix.rowCount,
    columns.length,
    designLayouts(columns, 2, configuration.kind, encodings),
  )
  if (!design.ok) {
    dispatch({
      type: 'run-failed',
      detail: describeDesignExpansionProblem(
        design.error,
        columns.map((column) => column.name),
      ),
    })
    return
  }
  const evidence = await analysis.runBayesianGaussian(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      treatment: 0,
      outcome: 1,
      adjustment: design.value.expanded.slice(2).flat(),
      warmup: configuration.warmup,
      samples: configuration.samples,
      seed: configuration.seed,
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'bayesian-gaussian-run',
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
    'The identification is no longer identified.',
  )
  return
}
