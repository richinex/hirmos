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

export async function runGeneralizedLinear(
  configuration: Extract<EstimatorConfiguration, { kind: 'poisson-glm' | 'negative-binomial-p' }>,
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
  const outcome = columnAt(matrix.values, matrix.rowCount, 1)
  if (outcome.some((value) => value < 0 || !Number.isInteger(value))) {
    dispatch({
      type: 'run-failed',
      detail: `${study.outcome.name} is not a count: it holds negative or fractional values.`,
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
  const evidence = await analysis.runCountGlm(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      treatment: 0,
      outcome: 1,
      adjustment: design.value.expanded.slice(2).flat(),
      family: configuration.kind === 'poisson-glm' ? 'poisson' : 'negativeBinomial',
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'count-glm-run', configuration, evidence: evidence.value } as const
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
