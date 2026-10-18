import { describePanelDataProblem } from '@/domain/panel'
import { adjustedRegressionPlan } from '@/analysis/adjustedRegressionPlan'
import type { ColumnId } from '@/domain/dataset'
import { type NonEmptyArray } from '@/domain/dop'
import { describeDesignExpansionProblem, designLayouts, expandDesign } from '@/domain/designMatrix'
import {
  causalEstimateFrom,
  methodIdOf,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runAdjustedRegression(
  configuration: Extract<EstimatorConfiguration, { kind: 'backdoor-linear-regression' }>,
  {
    study,
    identification,
    prepared,
    materialise,
    dispatch,
    source,
    profile,
    analysis,
    finish,
    identity,
    encodings,
    isCurrent,
  }: Pick<
    BackdoorExecution,
    | 'study'
    | 'identification'
    | 'prepared'
    | 'materialise'
    | 'dispatch'
    | 'source'
    | 'profile'
    | 'analysis'
    | 'finish'
    | 'identity'
    | 'encodings'
    | 'isCurrent'
  >,
): Promise<void> {
  const columns: NonEmptyArray<StudyVariable> = [
    study.treatment,
    study.outcome,
    ...identification.result.adjustment.variables,
  ]
  type Grouping = Pick<StudyVariable, 'column' | 'name'>
  const effects = configuration.fixedEffects
  const unit: Grouping | null =
    effects.kind === 'none' ? null : { column: effects.column, name: effects.name }
  const time: Grouping | null =
    effects.kind === 'unit-and-time' ? { column: effects.timeColumn, name: effects.timeName } : null
  const cluster: Grouping | null =
    configuration.errors.kind === 'cluster'
      ? { column: configuration.errors.column, name: configuration.errors.name }
      : null
  for (const [role, grouping] of [
    ['unit', unit],
    ['time', time],
    ['cluster', cluster],
  ] as const) {
    if (grouping !== null && columns.some((column) => column.column === grouping.column))
      throw new Error(
        `The ${role} column must be outside the design: not the treatment, the outcome or an adjustment variable.`,
      )
  }
  if (unit !== null && time !== null && time.column === unit.column)
    throw new Error('The unit and time columns must differ.')
  const groupings = [
    ...new Map(
      [unit, time, cluster]
        .filter((grouping): grouping is Grouping => grouping !== null)
        .map((grouping) => [grouping.column, grouping]),
    ).values(),
  ]
  const panel = prepared.kind === 'prepared-panel' ? prepared : null
  const isPanelKey = (grouping: Grouping) =>
    panel !== null &&
    (grouping.column === panel.sampling.unitColumn || grouping.column === panel.sampling.timeColumn)
  const extras = groupings.filter((grouping) => !isPanelKey(grouping))
  const materialized: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>> = [
    ...columns,
    ...extras,
  ]
  const matrix = await materialise(materialized)
  const layouts = [
    ...designLayouts(columns, 2, configuration.kind, encodings),
    ...extras.map(() => ({ kind: 'numeric' }) as const),
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
  const adjustment = design.value.expanded
    .slice(2, 2 + identification.result.adjustment.variables.length)
    .flat()
  let values = design.value.values
  let columnCount = design.value.columnCount
  const columnOf = new Map<ColumnId, number>()
  let weightLabels: ReadonlyMap<number, string> | null = null
  for (const [at, grouping] of extras.entries()) {
    const index = design.value.expanded[columns.length + at]?.[0]
    if (index === undefined) {
      dispatch({
        type: 'run-failed',
        detail: 'A selected grouping column is missing from the prepared regression matrix.',
      })
      return
    }
    columnOf.set(grouping.column, index)
  }
  if (panel !== null && groupings.some(isPanelKey)) {
    const { materializePanelKeysInWorker } = await import('@/data/client')
    const keys = await materializePanelKeysInWorker(
      source.file,
      profile,
      panel.sampling.unitColumn,
      panel.sampling.timeColumn,
    )
    if (!isCurrent()) return
    if (!keys.ok) {
      dispatch({ type: 'run-failed', detail: describePanelDataProblem(keys.error) })
      return
    }
    if (
      keys.value.sourceFingerprint !== profile.source.fingerprint ||
      keys.value.rowCount !== matrix.rowCount
    ) {
      dispatch({
        type: 'run-failed',
        detail:
          'The panel keys no longer match the prepared rows. Recreate the prepared panel version.',
      })
      return
    }
    const append = (codes: readonly number[]): number => {
      const coded = new Float64Array(values.length + matrix.rowCount)
      coded.set(values)
      codes.forEach((code, row) => {
        coded[columnCount * matrix.rowCount + row] = code
      })
      values = coded
      columnCount += 1
      return columnCount - 1
    }
    if (groupings.some((grouping) => grouping.column === panel.sampling.unitColumn)) {
      const labels = [...new Set(keys.value.units)].sort()
      if (unit?.column === panel.sampling.unitColumn)
        weightLabels = new Map(labels.map((label, index) => [index, label]))
      columnOf.set(
        panel.sampling.unitColumn,
        append(keys.value.units.map((label: string) => labels.indexOf(label))),
      )
    }
    if (groupings.some((grouping) => grouping.column === panel.sampling.timeColumn)) {
      columnOf.set(panel.sampling.timeColumn, append(keys.value.periodCodes))
      if (unit?.column === panel.sampling.timeColumn)
        weightLabels = new Map(keys.value.periods.map((period) => [period.code, period.label]))
    }
  }
  const plan = adjustedRegressionPlan(configuration, columnOf)
  if (!plan.ok) {
    dispatch({
      type: 'run-failed',
      detail:
        'A selected fixed-effect or cluster column could not be materialised. Review the grouping columns.',
    })
    return
  }
  const evidence = await analysis.runBackdoorLinear(values, matrix.rowCount, columnCount, {
    treatment: 0,
    outcome: 1,
    adjustment,
    hacMaxLags: null,
    level: configuration.level,
    ...plan.value,
  })
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = {
    kind: 'backdoor-linear-run',
    configuration,
    evidence: {
      ...evidence.value,
      weightDiagnostic:
        evidence.value.weightDiagnostic.kind === 'available' && weightLabels !== null
          ? {
              ...evidence.value.weightDiagnostic,
              groups: evidence.value.weightDiagnostic.groups.map((group) => {
                const label =
                  typeof group.label === 'number' ? weightLabels.get(group.label) : undefined
                if (label === undefined)
                  throw new Error('A fixed-effects group label is missing from the fitted panel.')
                return { ...group, label }
              }),
            }
          : evidence.value.weightDiagnostic,
    },
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
