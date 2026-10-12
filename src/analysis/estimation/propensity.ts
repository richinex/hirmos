import { runBoostedGridSearch } from '@/analysis/boostedSearch'
import { describeDesignExpansionProblem, designLayouts, expandDesign } from '@/domain/designMatrix'
import { ok, type NonEmptyArray, type Result } from '@/domain/dop'
import {
  boostedCommand,
  causalEstimateFrom,
  methodIdOf,
  type BoostedTreatmentModelChoice,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
  type PropensityUncertainty,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import type { PropensityTreatmentModel } from '@/workers/analysisProtocol'
import {
  describeAnalysisWorkerProblem,
  type AnalysisWorkerProblem,
} from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runPropensity(
  configuration: Extract<
    EstimatorConfiguration,
    { kind: 'propensity-weighting' | 'propensity-matching' | 'doubly-robust' }
  >,
  {
    study,
    identification,
    materialise,
    dispatch,
    finish,
    identity,
    analysis,
    encodings,
    isCurrent,
  }: Pick<
    BackdoorExecution,
    | 'study'
    | 'identification'
    | 'materialise'
    | 'dispatch'
    | 'finish'
    | 'identity'
    | 'analysis'
    | 'encodings'
    | 'isCurrent'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [
    study.treatment,
    study.outcome,
    ...identification.result.adjustment.variables,
  ]
  const matrix = await materialise(columns)
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
  const shared = {
    treatment: 0,
    outcome: 1,
    adjustment: design.value.expanded.slice(2).flat(),
  }
  const logistic =
    configuration.model === 'lbfgsb'
      ? ({ kind: 'lbfgsb', maxIter: configuration.maxIter } as const)
      : ({ kind: 'newton' } as const)
  const drawn = (uncertainty: PropensityUncertainty) =>
    uncertainty.kind === 'none'
      ? null
      : { rounds: uncertainty.rounds, seed: uncertainty.seed, level: uncertainty.level }
  const treatmentModel = (
    picked: {
      readonly grid: BoostedTreatmentModelChoice
      readonly searched: number
    } | null,
  ): PropensityTreatmentModel =>
    picked === null
      ? { kind: 'logistic', model: logistic }
      : { kind: 'boosted', model: boostedCommand(picked.grid, picked.searched) }
  const record = (run: Parameters<typeof causalEstimateFrom>[2]) => {
    if (
      run.kind === 'propensity-weighting-run' ||
      run.kind === 'propensity-matching-run' ||
      run.kind === 'doubly-robust-run'
    ) {
      const balance = run.evidence.balance
      if (balance.kind === 'available') {
        const labels = columns.slice(2).flatMap((column, i) => {
          const levels = design.value.levels[i + 2]!
          return levels.length === 0
            ? [column.name]
            : levels.map((level) => column.name + ' = ' + String(level))
        })
        run = {
          ...run,
          evidence: { ...run.evidence, balance: { ...balance, labels } },
        } as typeof run
      }
    }

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
  }
  type ChosenGrid = {
    readonly grid: BoostedTreatmentModelChoice
    readonly searched: number
  }
  const search = async (): Promise<Result<ChosenGrid | null, AnalysisWorkerProblem>> => {
    if (configuration.kind === 'doubly-robust' || configuration.model !== 'boosted') return ok(null)
    const declared = configuration.boosted
    const outcome = await runBoostedGridSearch(
      design.value.values.slice(),
      matrix.rowCount,
      design.value.columnCount,
      shared,
      boostedCommand(declared),
    )
    if (!outcome.ok) return outcome
    return ok({
      searched:
        declared.learningRate.length * declared.maxDepth.length * declared.nEstimators.length,
      grid: {
        ...declared,
        learningRate: [outcome.value.best.learningRate],
        maxDepth: [outcome.value.best.maxDepth],
        nEstimators: [outcome.value.best.nEstimators],
      },
    })
  }
  const searched = await search()
  if (!isCurrent()) return
  if (!searched.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(searched.error) })
    return
  }
  const chosen = searched.value
  if (configuration.kind === 'propensity-weighting') {
    const fit =
      chosen === null
        ? ({
            kind: 'logistic',
            model: logistic,
            bootstrap: drawn(configuration.uncertainty),
          } as const)
        : ({
            kind: 'boosted',
            model: boostedCommand(chosen.grid, chosen.searched),
          } as const)
    const evidence = await analysis.runPropensityWeighting(
      design.value.values,
      matrix.rowCount,
      design.value.columnCount,
      {
        ...shared,
        target: study.estimand.kind === 'average-treatment-effect-on-treated' ? 'att' : 'ate',
        scale: configuration.scale,
        fit,
      },
    )
    if (!evidence.ok) {
      dispatch({
        type: 'run-failed',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
      return
    }
    record({ kind: 'propensity-weighting-run', configuration, evidence: evidence.value })
    return
  }
  if (configuration.kind === 'propensity-matching') {
    const evidence = await analysis.runPropensityMatching(
      design.value.values,
      matrix.rowCount,
      design.value.columnCount,
      {
        ...shared,
        target: study.estimand.kind === 'average-treatment-effect-on-treated' ? 'att' : 'ate',
        model: treatmentModel(chosen),
      },
    )
    if (!evidence.ok) {
      dispatch({
        type: 'run-failed',
        detail: describeAnalysisWorkerProblem(evidence.error),
      })
      return
    }
    record({ kind: 'propensity-matching-run', configuration, evidence: evidence.value })
    return
  }
  const evidence = await analysis.runDoublyRobust(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      ...shared,
      target: study.estimand.kind === 'average-treatment-effect-on-treated' ? 'att' : 'ate',
      model: logistic,
      bootstrap: drawn(configuration.uncertainty),
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  record({ kind: 'doubly-robust-run', configuration, evidence: evidence.value })
  return
}

export async function runContinuousGps(
  configuration: Extract<EstimatorConfiguration, { kind: 'continuous-gps' }>,
  {
    study,
    identification,
    materialise,
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
  const evidence = await analysis.runContinuousGps(
    design.value.values,
    matrix.rowCount,
    design.value.columnCount,
    {
      treatment: 0,
      outcome: 1,
      adjustment: design.value.expanded.slice(2).flat(),
      scale: configuration.scale,
      bootstrap:
        configuration.uncertainty.kind === 'none'
          ? null
          : {
              rounds: configuration.uncertainty.rounds,
              seed: configuration.uncertainty.seed,
              level: configuration.uncertainty.level,
            },
    },
  )
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'continuous-gps-run',
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
