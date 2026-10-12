import {
  causalForestInputs,
  causalForestTarget,
  parseCausalForestEvidence,
  type ForestFeature,
} from '@/domain/causalForest'
import { forestAnalysisColumns } from '@/domain/causalForestAnalysis'
import type { ColumnId } from '@/domain/dataset'
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

export async function runCausalForest(
  configuration: Extract<EstimatorConfiguration, { kind: 'causal-forest' }>,
  {
    study,
    dispatch,
    identification,
    profile,
    prepared,
    materialise,
    analysis,
    finish,
    identity,
    encodings,
  }: Pick<
    BackdoorExecution,
    | 'study'
    | 'dispatch'
    | 'identification'
    | 'profile'
    | 'prepared'
    | 'materialise'
    | 'analysis'
    | 'finish'
    | 'identity'
    | 'encodings'
  >,
): Promise<void> {
  const target = causalForestTarget(study.estimand)
  if (target === null) {
    dispatch({
      type: 'run-failed',
      detail: 'The recorded target is not supported by causal forest.',
    })
    return
  }
  const inputs = causalForestInputs(identification.result.adjustment.variables, study.estimand)
  const auxiliary =
    configuration.analysis === undefined ? [] : forestAnalysisColumns(configuration.analysis)
  const required = [study.treatment.column, study.outcome.column, ...inputs.map((c) => c.column)]
  const extra: Pick<StudyVariable, 'column' | 'name'>[] = []
  for (const id of auxiliary) {
    if (id === study.treatment.column || id === study.outcome.column)
      throw new Error(
        'Use separate columns for forest sampling, projection and priority scores, not the treatment or outcome.',
      )
    if (required.includes(id as ColumnId)) continue
    const column = profile.columns.find((c) => c.id === id && prepared.columns.includes(c.id))
    if (column === undefined)
      throw new Error('An auxiliary forest column is not in the prepared dataset.')
    extra.push({ column: column.id, name: column.name })
  }
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...inputs]
  const materialized: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>> = [...columns, ...extra]
  const matrix = await materialise(materialized)
  const layouts = [
    ...designLayouts(columns, 2, configuration.kind, encodings),
    ...extra.map(() => ({ kind: 'numeric' }) as const),
  ]
  const design = expandDesign(matrix.values, matrix.rowCount, materialized.length, layouts)
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
  const evidence = await analysis.runCausalForest(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      treatment: 0,
      outcome: 1,
      adjustment: design.value.expanded.slice(2, 2 + inputs.length).flat(),
      target,
      configuration,
      columnNames: materialized.flatMap((column, index) =>
        design.value.expanded[index]!.map(() => column.column),
      ),
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const features = inputs.flatMap<ForestFeature>((column, index) => {
    const levels = design.value.levels[index + 2]!
    return levels.length === 0
      ? [{ kind: 'numeric', column: column.column, name: column.name } as const]
      : levels.map(
          (level) =>
            ({
              kind: 'indicator',
              column: column.column,
              name: column.name,
              level,
            }) as const,
        )
  })
  const labelled = parseCausalForestEvidence({ ...evidence.value, features })
  if (!labelled.ok) {
    dispatch({
      type: 'run-failed',
      detail: 'The encoded covariate labels do not match the forest result.',
    })
    return
  }
  const run = {
    kind: 'causal-forest-run',
    configuration,
    evidence: labelled.value,
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
    'The forest could not report the recorded target for every required observation. Review the sample and forest settings.',
  )
  return
}
