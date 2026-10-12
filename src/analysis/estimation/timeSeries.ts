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

export async function runArdl(
  configuration: Extract<EstimatorConfiguration, { kind: 'ardl-pss' }>,
  {
    study,
    materialise,
    analysis,
    dispatch,
    identification,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    'study' | 'materialise' | 'analysis' | 'dispatch' | 'identification' | 'finish' | 'identity'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome]
  const matrix = await materialise(columns)
  const evidence = await analysis.runArdlPss(matrix.values, matrix.rowCount, columns.length, {
    treatment: 0,
    outcome: 1,
    maxLag: configuration.maxLag,
    trend: configuration.trend,
    case: configuration.case,
  })
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'ardl-run', configuration, evidence: evidence.value } as const
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

export async function runVecm(
  configuration: Extract<EstimatorConfiguration, { kind: 'vecm' }>,
  {
    study,
    identification,
    materialise,
    analysis,
    dispatch,
    finish,
    identity,
  }: Pick<
    BackdoorExecution,
    'study' | 'identification' | 'materialise' | 'analysis' | 'dispatch' | 'finish' | 'identity'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [
    study.outcome,
    study.treatment,
    ...identification.result.adjustment.variables,
  ]
  const matrix = await materialise(columns)
  const evidence = await analysis.runVecm(matrix.values, matrix.rowCount, columns.length, {
    endogenous: columns.map((_, index) => index),
    maxLags: configuration.maxLags,
    deterministic: configuration.deterministic,
    significance: configuration.significance === 90 ? 0 : configuration.significance === 95 ? 1 : 2,
    breakIndex: configuration.breakIndex,
  })
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 'vecm-run', configuration, evidence: evidence.value } as const
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
    evidence.value.rank === 0
      ? 'The Johansen trace test did not identify a cointegrating relation under this specification, so no long-run coefficient is reported.'
      : `The trace test found ${evidence.value.rank} cointegrating relations; a single long-run coefficient is read only at rank one.`,
  )
  return
}
