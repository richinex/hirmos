import { type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  interventionStartFromTreatment,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { describePanelDataProblem } from '@/domain/panel'
import { ridgeInput } from '@/domain/ridgeAugmented'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runSyntheticControl(
  configuration: Extract<EstimatorConfiguration, { kind: 'synthetic-control' }>,
  {
    panelBinding,
    dispatch,
    source,
    profile,
    analysis,
    study,
    identification,
    finish,
    identity,
    prepared,
    materialise,
    columnAt,
    isCurrent,
  }: Pick<
    BackdoorExecution,
    | 'panelBinding'
    | 'dispatch'
    | 'source'
    | 'profile'
    | 'analysis'
    | 'study'
    | 'identification'
    | 'finish'
    | 'identity'
    | 'prepared'
    | 'materialise'
    | 'columnAt'
    | 'isCurrent'
  >,
): Promise<void> {
  if (configuration.specification === 'ridge-augmented') {
    if (panelBinding === null) {
      dispatch({ type: 'run-failed', detail: 'Prepare a long panel with an ATT study.' })
      return
    }
    const { materializePanelInWorker } = await import('@/data/client')
    const materialized = await materializePanelInWorker(source.file, profile, {
      unit: panelBinding.unit,
      time: panelBinding.time,
      outcome: panelBinding.outcome,
      treatment: panelBinding.treatment,
    })
    if (!isCurrent()) return
    if (!materialized.ok) {
      dispatch({ type: 'run-failed', detail: describePanelDataProblem(materialized.error) })
      return
    }
    const input = ridgeInput(materialized.value, configuration)
    if (!input.ok) {
      dispatch({ type: 'run-failed', detail: input.error })
      return
    }
    const result = await analysis.runRidgeAugmentedSynthetic(input.value.values, input.value.model)
    if (!isCurrent()) return
    if (!result.ok) {
      dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(result.error) })
      return
    }
    const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment]
    const run = {
      kind: 'ridge-augmented-synthetic-run',
      configuration,
      evidence: result.value,
      sourcePeriods: materialized.value.periods,
    } as const
    const estimate = causalEstimateFrom(study, identification, run)
    finish(
      estimate === null
        ? null
        : {
            ...identity,
            ...run,
            method: methodIdOf(configuration.kind),
            columns,
            estimate,
          },
      'The ridge-augmented comparison does not match the recorded ATT study.',
    )
    return
  }
  if (configuration.specification === 'predictors') {
    const { preparePredictorSynthetic } = await import('@/data/predictorSyntheticInput')
    const input = await preparePredictorSynthetic(
      source,
      profile,
      prepared,
      configuration,
      study.outcome.column,
      study.treatment.column,
      () => isCurrent(),
    )
    if (!isCurrent()) return
    if (!input.ok) {
      dispatch({ type: 'run-failed', detail: input.error })
      return
    }
    const result = await analysis.runPredictorSyntheticControl(
      input.value.values,
      input.value.model,
    )
    if (!isCurrent()) return
    if (!result.ok) {
      dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(result.error) })
      return
    }
    const run = {
      kind: 'predictor-synthetic-control-run',
      configuration,
      evidence: result.value,
      catalog: input.value.catalog,
      columns: input.value.columns,
    } as const
    const estimate = causalEstimateFrom(study, identification, run)
    finish(
      estimate === null
        ? null
        : ({
            ...identity,
            ...run,
            method: methodIdOf(configuration.kind),
            estimate,
          } as EstimationRunArtifact),
      'The synthetic-control result does not match the recorded treated-unit comparison or has missing post-intervention predictions.',
    )
    return
  }
  const donors = prepared.columns.filter(
    (column) =>
      configuration.donors.includes(column) &&
      column !== study.outcome.column &&
      column !== study.treatment.column,
  )
  const donorVariables: StudyVariable[] = donors.map((column) => ({
    node: study.outcome.node,
    column,
    name: profile.columns.find((candidate) => candidate.id === column)?.name ?? String(column),
  }))
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...donorVariables]
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
  const evidence = await analysis.runSyntheticControl(
    matrix.values,
    matrix.rowCount,
    columns.length,
    {
      treated: 1,
      donors: donorVariables.map((_, index) => index + 2),
      nPre,
      crossFitFolds: configuration.crossFitFolds,
      alpha: configuration.alpha,
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'synthetic-control-run',
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
    'The synthetic control run produced no post-intervention rows.',
  )
  return
}
