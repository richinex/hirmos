import { runBoostedGridSearch } from '@/analysis/boostedSearch'
import { describeDesignExpansionProblem, designLayouts, expandDesign } from '@/domain/designMatrix'
import { assertNever, err, ok, type NonEmptyArray, type Result } from '@/domain/dop'
import {
  boostedGridCommand,
  causalEstimateFrom,
  methodIdOf,
  tLearnerInputs,
  type ArmChoice,
  type EstimationRunArtifact,
  type EstimatorConfiguration,
} from '@/domain/estimation'
import { type StudyVariable } from '@/domain/study'
import {
  describeAnalysisWorkerProblem,
  type AnalysisWorkerProblem,
} from '@/workers/analysisProtocol'
import type { BackdoorExecution } from './context'

export async function runTLearner(
  configuration: Extract<EstimatorConfiguration, { kind: 't-learner' }>,
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
    isCurrent,
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
    | 'isCurrent'
  >,
): Promise<void> {
  const inputs = tLearnerInputs(identification.result.adjustment.variables, study.estimand)
  const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...inputs]
  const matrix = await materialise(columns)
  if (columnAt(matrix.values, matrix.rowCount, 0).some((value) => value !== 0 && value !== 1)) {
    dispatch({
      type: 'run-failed',
      detail: `The T-learner fits one outcome model per arm, so it cannot be fitted because ${study.treatment.name} is not a 0/1 treatment.`,
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
  const shared = {
    treatment: 0,
    outcome: 1,
    adjustment: design.value.expanded.slice(2).flat(),
  }
  const model = configuration.model
  if (
    model.kind === 'boosted-cross-fitted' &&
    columnAt(matrix.values, matrix.rowCount, 1).some((value) => value !== 0 && value !== 1)
  ) {
    dispatch({
      type: 'run-failed',
      detail: `${study.outcome.name} is not 0 or 1; the boosted learner fits a classifier per arm.`,
    })
    return
  }
  const evidence = await (async () => {
    switch (model.kind) {
      case 'forest':
        return analysis.runTLearner(
          design.value.values,
          matrix.rowCount,
          design.value.columnCount,
          { ...shared, seed: model.seed, uncertainty: model.uncertainty },
        )
      case 'boosted-cross-fitted': {
        // Each arm's grid is searched across a worker pool on that arm's rows, which is
        // the search the kernel would run itself; the kernel then fits the two choices.
        const command = boostedGridCommand(model.grid)
        const chooseArm = async (arm: 0 | 1): Promise<Result<ArmChoice, AnalysisWorkerProblem>> => {
          const rows = columnAt(design.value.values, matrix.rowCount, 0).flatMap((value, row) =>
            value === arm ? [row] : [],
          )
          const armValues = new Float64Array(rows.length * design.value.columnCount)
          for (let column = 0; column < design.value.columnCount; column += 1) {
            rows.forEach((row, at) => {
              armValues[column * rows.length + at] =
                design.value.values[column * matrix.rowCount + row]!
            })
          }
          // The pooled search predicts its `treatment` column, so the outcome takes that place.
          const searched = await runBoostedGridSearch(
            armValues,
            rows.length,
            design.value.columnCount,
            { treatment: 1, outcome: 0, adjustment: shared.adjustment },
            command,
          )
          if (!searched.ok) return searched
          const { learningRate, maxDepth, nEstimators, meanScore } = searched.value.best
          return ok({ learningRate, maxDepth, nEstimators, validationAuc: meanScore })
        }
        const treated = await chooseArm(1)
        if (!treated.ok) return treated
        if (!isCurrent())
          return err({
            kind: 'analysis-cancelled',
            detail: 'The analysis run was cancelled.',
          } as const)
        const control = await chooseArm(0)
        if (!control.ok) return control
        if (!isCurrent())
          return err({
            kind: 'analysis-cancelled',
            detail: 'The analysis run was cancelled.',
          } as const)
        return analysis.runCrossFittedTLearner(
          design.value.values,
          matrix.rowCount,
          design.value.columnCount,
          {
            ...shared,
            ...command,
            selection: { kind: 'chosen', treated: treated.value, control: control.value },
          },
        )
      }
      default:
        return assertNever(model)
    }
  })()
  if (!evidence.ok) {
    dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
    return
  }
  const run = { kind: 't-learner-run', configuration, evidence: evidence.value } as const
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
