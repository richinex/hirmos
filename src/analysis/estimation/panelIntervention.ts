import {
  describeDidCovariateRestrictions,
  didCovariateRestrictions,
  staggeredCovariateRestrictions,
} from '@/domain/adjustedDid'
import { isNonEmpty, mapNonEmpty, type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { samePanelBinding } from '@/domain/estimationDraft'
import { describePanelDataProblem } from '@/domain/panel'
import { staggeredInput } from '@/domain/staggeredDid'
import { type StudyVariable } from '@/domain/study'
import { sunAbrahamInput } from '@/domain/sunAbraham'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runPanelIntervention(
  configuration: Extract<EstimatorConfiguration, { kind: 'panel-intervention' }>,
  {
    prepared,
    dispatch,
    sunPanel,
    analysis,
    study,
    identification,
    finish,
    identity,
    panelBinding,
    source,
    profile,
    columnName,
    panelPreflight,
    isCurrent,
    offerPointwiseRetry,
  }: Pick<
    BackdoorExecution,
    | 'prepared'
    | 'dispatch'
    | 'sunPanel'
    | 'analysis'
    | 'study'
    | 'identification'
    | 'finish'
    | 'identity'
    | 'panelBinding'
    | 'source'
    | 'profile'
    | 'columnName'
    | 'panelPreflight'
    | 'isCurrent'
    | 'offerPointwiseRetry'
  >,
): Promise<void> {
  if (prepared.kind !== 'prepared-panel') {
    dispatch({
      type: 'run-failed',
      detail: 'Prepare a balanced long panel before running the panel intervention estimator.',
    })
    return
  }
  if (configuration.primary === 'sunAbraham') {
    if (sunPanel.kind !== 'ready') {
      dispatch({
        type: 'run-failed',
        detail:
          sunPanel.kind === 'failed'
            ? sunPanel.detail
            : 'Wait for the adoption cohorts to be read.',
      })
      return
    }
    const materialized = { value: sunPanel.matrix }
    const input = sunAbrahamInput(materialized.value, configuration)
    if (!input.ok) {
      dispatch({ type: 'run-failed', detail: input.error })
      return
    }
    const evidence = await analysis.runSunAbraham(input.value.values, input.value.model)
    if (!isCurrent()) return
    if (!evidence.ok) {
      dispatch({
        type: 'run-failed',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
      return
    }
    const labels = evidence.value.times.map(
      (t) => materialized.value.periods.find((p) => p.code === t)?.label,
    )
    if (!isNonEmpty(labels) || labels.some((l) => l === undefined)) {
      dispatch({
        type: 'run-failed',
        detail: 'The returned periods do not match the source labels.',
      })
      return
    }
    const timeLabels = labels.filter((l): l is string => l !== undefined)
    if (!isNonEmpty(timeLabels)) return
    const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment]
    const run = {
      kind: 'panel-intervention-run',
      configuration,
      evidence: evidence.value,
      timeLabels,
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
      'The Sun–Abraham result does not match the recorded ATT study.',
    )
    return
  }
  if (configuration.primary === 'staggered') {
    if (panelBinding === null) {
      dispatch({
        type: 'run-failed',
        detail: 'Select panel unit, time, outcome and treatment columns.',
      })
      return
    }
    const { materializePanelInWorker } = await import('@/data/client')
    const materialized = await materializePanelInWorker(source.file, profile, {
      unit: panelBinding.unit,
      time: panelBinding.time,
      outcome: panelBinding.outcome,
      treatment: panelBinding.treatment,
      covariates: configuration.covariates,
      clusterColumn:
        configuration.clustering?.kind === 'column' ? configuration.clustering.column : undefined,
    })
    if (!isCurrent()) return
    if (!materialized.ok) {
      dispatch({ type: 'run-failed', detail: describePanelDataProblem(materialized.error) })
      return
    }
    const refused = staggeredCovariateRestrictions(materialized.value)
    if (refused.length > 0) {
      dispatch({
        type: 'run-failed',
        detail: describeDidCovariateRestrictions(refused, columnName),
      })
      return
    }
    const input = staggeredInput(
      materialized.value,
      configuration.specification,
      configuration.clustering,
    )
    if (!input.ok) {
      dispatch({ type: 'run-failed', detail: input.error })
      return
    }
    const evidence = await analysis.runStaggeredDid(input.value.values, input.value.model)
    if (!evidence.ok) {
      if (
        isCurrent() &&
        configuration.specification.inference.kind === 'bootstrapSimultaneous' &&
        evidence.error.kind === 'kernel-refused' &&
        evidence.error.detail ===
          'The estimated simultaneous critical value is below the pointwise critical value. No interval was substituted.'
      ) {
        offerPointwiseRetry({ configuration, study: study.id, prepared: prepared.id })
      }
      dispatch({
        type: 'run-failed',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
      return
    }
    const columns: NonEmptyArray<StudyVariable> = [
      study.outcome,
      study.treatment,
      ...configuration.covariates.map((column) => ({
        column,
        node: study.outcome.node,
        name: profile.columns.find((c) => c.id === column)?.name ?? String(column),
      })),
    ]
    const labels = evidence.value.times.map(
      (time) => materialized.value.periods.find((p) => p.code === time)?.label ?? String(time),
    )
    if (!isNonEmpty(labels)) {
      dispatch({ type: 'run-failed', detail: 'No retained panel periods were returned.' })
      return
    }
    const run = {
      kind: 'panel-intervention-run',
      configuration,
      evidence: evidence.value,
      timeLabels: labels,
      sourcePeriods: materialized.value.periods.map(({ code, label }) => ({ code, label })),
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
      'The staggered DiD result does not match the treated-group study target.',
    )
    return
  }
  if (
    panelBinding === null ||
    panelPreflight.kind !== 'ready' ||
    !samePanelBinding(panelPreflight.binding, panelBinding)
  ) {
    dispatch({
      type: 'run-failed',
      detail: 'The panel treatment-layout check has not completed for this study.',
    })
    return
  }
  if (configuration.primary === 'adjusted') {
    const { materializePanelInWorker } = await import('@/data/client')
    const materialized = await materializePanelInWorker(source.file, profile, {
      unit: panelBinding.unit,
      time: panelBinding.time,
      outcome: panelBinding.outcome,
      treatment: panelBinding.treatment,
      covariates: configuration.covariates,
    })
    if (!isCurrent()) return
    if (!materialized.ok) {
      dispatch({ type: 'run-failed', detail: describePanelDataProblem(materialized.error) })
      return
    }
    const matrix = materialized.value
    const restrictions = didCovariateRestrictions(
      matrix,
      panelPreflight.layout,
      configuration.specification,
    )
    if (restrictions.length > 0) {
      dispatch({
        type: 'run-failed',
        detail: describeDidCovariateRestrictions(restrictions, columnName),
      })
      return
    }
    const evidence = await analysis.runAdjustedDid(
      matrix.values,
      matrix.rowCount,
      2 + configuration.covariates.length,
      matrix.units,
      matrix.periodCodes,
      configuration.specification,
    )
    if (!evidence.ok) {
      dispatch({
        type: 'run-failed',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
      return
    }
    const columns: NonEmptyArray<StudyVariable> = [
      study.outcome,
      study.treatment,
      ...configuration.covariates.map((column) => ({
        column,
        node: study.outcome.node,
        name: profile.columns.find((c) => c.id === column)?.name ?? String(column),
      })),
    ]
    const run = {
      kind: 'panel-intervention-run',
      configuration,
      evidence: evidence.value,
      timeLabels: mapNonEmpty(panelPreflight.layout.periods, (period) => period.label),
      sourcePeriods: panelPreflight.layout.periods.map(({ code, label }) => ({
        code,
        label,
      })),
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
      'The adjusted panel result does not match its study target.',
    )
    return
  }
  const matrix = panelPreflight.matrix
  const evidence = await analysis.runPanelIntervention(
    matrix.values.slice(),
    matrix.rowCount,
    matrix.units,
    matrix.periodCodes,
    {
      placeboReplications: configuration.placeboReplications,
      seed: configuration.seed,
      primary: configuration.primary,
      inference: configuration.primary === 'did' ? configuration.inference : undefined,
    },
    (progress) => dispatch({ type: 'run-progressed', progress }),
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const columns: NonEmptyArray<StudyVariable> = [study.outcome, study.treatment]
  const run = {
    kind: 'panel-intervention-run',
    configuration,
    evidence: evidence.value,
    timeLabels: mapNonEmpty(panelPreflight.layout.periods, (period) => period.label),
    sourcePeriods: panelPreflight.layout.periods.map(({ code, label }) => ({
      code,
      label,
    })),
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
    'The panel intervention run produced no estimate.',
  )
  return
}
