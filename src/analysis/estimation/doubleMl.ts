import { describeDesignExpansionProblem, designLayouts, expandDesign } from '@/domain/designMatrix'
import { type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  dmlNuisanceInputs,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem, type DmlGroupsRequest } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runDoubleMl(
  configuration: Extract<EstimatorConfiguration, { kind: 'dml-plr' | 'dml-irm' }>,
  {
    identification,
    study,
    materialise,
    columnAt,
    dispatch,
    analysis,
    finish,
    identity,
    encodings,
  }: Pick<
    BackdoorExecution,
    | 'identification'
    | 'study'
    | 'materialise'
    | 'columnAt'
    | 'dispatch'
    | 'analysis'
    | 'finish'
    | 'identity'
    | 'encodings'
  >,
): Promise<void> {
  const nuisance = dmlNuisanceInputs(identification.result.adjustment.variables, study.estimand)
  const modifier =
    study.estimand.kind === 'conditional-average-treatment-effect' ? study.estimand.modifier : null
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...nuisance]
  const matrix = await materialise(columns)
  if (
    configuration.kind === 'dml-irm' &&
    columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)
  ) {
    dispatch({
      type: 'run-failed',
      detail: `The interactive model cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.`,
    })
    return
  }
  const groups: DmlGroupsRequest =
    study.estimand.kind !== 'conditional-average-treatment-effect'
      ? { kind: 'none' }
      : study.estimand.grouping.kind === 'levels'
        ? {
            kind: 'levels',
            column: columns.findIndex((variable) => variable.column === modifier?.column),
          }
        : {
            kind: 'quantiles',
            column: columns.findIndex((variable) => variable.column === modifier?.column),
            bins: study.estimand.grouping.bins,
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
  const evidence = await analysis.runDoubleMl(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      treatment: 0,
      outcome: 1,
      adjustment: design.value.expanded.slice(2).flat(),
      model: configuration.kind === 'dml-plr' ? 'plr' : 'irm',
      att: configuration.kind === 'dml-irm' && configuration.att,
      seed: configuration.seed,
      groups,
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'double-ml-run', configuration, evidence: evidence.value } as const
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
