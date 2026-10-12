import type { ColumnId } from '@/domain/dataset'
import { assertNever, type NonEmptyArray } from '@/domain/dop'
import {
  causalEstimateFrom,
  methodIdOf,
  newEstimationRunId,
  type EstimationRunArtifact,
} from '@/domain/estimation'
import { identifiedInstruments, type StudyVariable } from '@/domain/study'
import { describeAnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { runAdjustedRegression } from './adjustedRegression'
import { runBayesianGaussian, runNegbinNuts } from './bayesianOutcome'
import { runCausalForest } from './causalForest'
import { runCausalImpact } from './causalImpact'
import type { EstimationEffects, BackdoorExecution, EstimationInput } from './context'
import { runCountSeries } from './countSeries'
import { runDiscreteModel } from './discreteModel'
import { runDoubleMl } from './doubleMl'
import { runGeneralizedLinear } from './generalizedLinear'
import { runPanelIntervention } from './panelIntervention'
import { runContinuousGps, runPropensity } from './propensity'
import { runSyntheticControl } from './syntheticControl'
import { runTLearner } from './tLearner'
import { runTemporalEffect } from './temporalEffect'
import { runArdl, runVecm } from './timeSeries'

export async function executeEstimation(
  input: EstimationInput,
  effects: EstimationEffects,
): Promise<void> {
  const {
    configuration,
    source,
    profile,
    prepared,
    study,
    identification,
    eligibility,
    encodings,
  } = input
  const { isCurrent, offerPointwiseRetry } = effects
  const dispatch = effects.emit
  const columnName = (id: string): string =>
    profile.columns.find((column) => column.id === id)?.name ?? id
  try {
    const [{ materialisePrepared, describePreparedMaterialisationProblem }, analysis] =
      await Promise.all([import('@/data/prepared'), import('@/analysis/client')])
    if (!isCurrent()) return
    const materialise = async (columns: NonEmptyArray<Pick<StudyVariable, 'column' | 'name'>>) => {
      const matrix = await materialisePrepared(
        source,
        profile,
        prepared,
        columns.map((column) => column.column) as unknown as NonEmptyArray<ColumnId>,
      )
      if (!isCurrent()) throw new DOMException('The estimation run was cancelled.', 'AbortError')
      if (!matrix.ok) throw new Error(describePreparedMaterialisationProblem(matrix.error))
      return matrix.value
    }
    const columnAt = (values: Float64Array, rows: number, index: number): number[] =>
      Array.from(values.subarray(index * rows, (index + 1) * rows))
    const identity = {
      id: newEstimationRunId(),
      study: study.id,
      identification: identification.id,
      preparedDataset: prepared.id,
      createdAt: new Date().toISOString(),
      eligibility,
      encodings: encodings,
    } as const
    const finish = (run: EstimationRunArtifact | null, detail: string) => {
      if (!isCurrent()) return
      if (run === null) {
        dispatch({ type: 'run-failed', detail })
        return
      }
      effects.onRun(run)
      dispatch({ type: 'run-finished' })
    }
    if (configuration.kind === 'sharp-rd') {
      if (
        study.estimand.kind !== 'local-cutoff-effect' ||
        identification.result.kind !== 'cutoff-design'
      ) {
        dispatch({
          type: 'run-failed',
          detail: 'Record the cutoff-local study before fitting sharp RD.',
        })
        return
      }
      const columns: NonEmptyArray<StudyVariable> = [
        study.estimand.running,
        study.outcome,
        study.treatment,
      ]
      const matrix = await materialise(columns)
      const evidence = await analysis.runSharpRd(
        matrix.values,
        matrix.rowCount,
        study.estimand.cutoff,
      )
      if (!evidence.ok) {
        dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
        return
      }
      const run = { kind: 'sharp-rd-run', configuration, evidence: evidence.value } as const
      const estimate = causalEstimateFrom(study, identification, run)
      finish(
        estimate === null
          ? null
          : { ...identity, ...run, method: methodIdOf(configuration.kind), columns, estimate },
        'The RD result no longer matches the recorded cutoff-local study.',
      )
      return
    }
    if (configuration.kind === 'frontdoor-two-stage') {
      if (
        identification.result.kind !== 'graphically-identified' ||
        identification.result.frontdoor.kind !== 'identified' ||
        identification.result.frontdoor.mediators.length !== 1
      ) {
        dispatch({
          type: 'run-failed',
          detail: 'This identification record does not contain one observed front-door mediator.',
        })
        return
      }
      const mediator = identification.result.frontdoor.mediators[0]
      const columns: NonEmptyArray<StudyVariable> = [study.treatment, mediator, study.outcome]
      const matrix = await materialise(columns)
      const evidence = await analysis.runFrontdoorTwoStage(
        matrix.values,
        matrix.rowCount,
        columns.length,
        {
          treatment: 0,
          mediator: 1,
          outcome: 2,
          firstStageAdjustment: [],
          secondStageAdjustment: [0],
          controlValue: configuration.interventions[0],
          treatmentValue: configuration.interventions[1],
          uncertainty: {
            kind: 'bootstrap',
            simulations: configuration.simulations,
            sampleSizeFraction: configuration.sampleSizeFraction,
            confidenceLevel: configuration.level,
            seed: configuration.seed,
          },
        },
        (progress) => dispatch({ type: 'run-progressed', progress }),
      )
      if (!evidence.ok) {
        dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
        return
      }
      const run = {
        kind: 'frontdoor-two-stage-run',
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
        'The front-door identification record is no longer available.',
      )
      return
    }
    if (configuration.kind === 'instrumental-variable') {
      const instruments = identifiedInstruments(identification.result)
      if (instruments === null) {
        dispatch({
          type: 'run-failed',
          detail: 'This identification record names no observed instrument.',
        })
        return
      }
      const columns: NonEmptyArray<StudyVariable> = [study.treatment, study.outcome, ...instruments]
      const matrix = await materialise(columns)
      const evidence = await analysis.runInstrumentalVariable(
        matrix.values,
        matrix.rowCount,
        columns.length,
        {
          treatment: 0,
          outcome: 1,
          instruments: instruments.map((_, index) => 2 + index),
          uncertainty: {
            kind: 'bootstrap',
            simulations: configuration.simulations,
            sampleSizeFraction: configuration.sampleSizeFraction,
            confidenceLevel: configuration.level,
            seed: configuration.seed,
          },
        },
        (progress) => dispatch({ type: 'run-progressed', progress }),
      )
      if (!evidence.ok) {
        dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
        return
      }
      const run = {
        kind: 'instrumental-variable-run',
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
        'The instrument identification record is no longer available.',
      )
      return
    }
    if (configuration.kind === 'binary-ett-idc-star') {
      if (identification.result.kind !== 'counterfactually-identified') {
        dispatch({
          type: 'run-failed',
          detail: 'This identification record does not contain binary ETT expressions from IDC*.',
        })
        return
      }
      const observed = study.graph.nodes.flatMap((node, nodeIndex) =>
        node.column === null
          ? []
          : [{ node: node.node, column: node.column, name: node.name, nodeIndex }],
      )
      if (observed.length < 2) {
        dispatch({
          type: 'run-failed',
          detail: 'Binary ETT needs measured treatment and outcome nodes.',
        })
        return
      }
      const columns = observed.map(({ node, column, name }) => ({
        node,
        column,
        name,
      })) as unknown as NonEmptyArray<StudyVariable>
      const matrix = await materialise(columns)
      const treatment = study.graph.nodes.findIndex((node) => node.node === study.treatment.node)
      const outcome = study.graph.nodes.findIndex((node) => node.node === study.outcome.node)
      const evidence = await analysis.runBinaryEtt(matrix.values, matrix.rowCount, columns.length, {
        observedNodes: observed.map((node) => node.nodeIndex),
        names: study.graph.nodes.map((node) => node.name),
        edges: study.graph.edges,
        treatment,
        outcome,
        unobserved: study.graph.nodes.flatMap((node, index) =>
          node.column === null ? [index] : [],
        ),
      })
      if (!evidence.ok) {
        dispatch({ type: 'run-failed', detail: describeAnalysisWorkerProblem(evidence.error) })
        return
      }
      const run = { kind: 'binary-ett-run', configuration, evidence: evidence.value } as const
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
        'The IDC* identification record is no longer available.',
      )
      return
    }
    if (identification.result.kind !== 'identified') {
      dispatch({
        type: 'run-failed',
        detail: 'This estimator requires a back-door identification record.',
      })
      return
    }
    const context: BackdoorExecution = {
      ...input,
      identification: { ...identification, result: identification.result },
      analysis,
      materialise,
      columnAt,
      identity,
      finish,
      dispatch,
      isCurrent,
      offerPointwiseRetry,
      columnName,
    }
    switch (configuration.kind) {
      case 'backdoor-linear-regression':
        return await runAdjustedRegression(configuration, context)

      case 'dml-plr':
      case 'dml-irm':
        return await runDoubleMl(configuration, context)

      case 'causal-forest':
        return await runCausalForest(configuration, context)

      case 't-learner':
        return await runTLearner(configuration, context)

      case 'ardl-pss':
        return await runArdl(configuration, context)

      case 'vecm':
        return await runVecm(configuration, context)

      case 'synthetic-control':
        return await runSyntheticControl(configuration, context)

      case 'panel-intervention':
        return await runPanelIntervention(configuration, context)

      case 'negbin-nuts':
        return await runNegbinNuts(configuration, context)

      case 'bayesian-gaussian':
        return await runBayesianGaussian(configuration, context)

      case 'discrete-bn-query':
        return await runDiscreteModel(configuration, context)

      case 'poisson-glm':
      case 'negative-binomial-p':
        return await runGeneralizedLinear(configuration, context)

      case 'negative-binomial-ingarch':
        return await runCountSeries(configuration, context)

      case 'causal-effects-total':
        return await runTemporalEffect(configuration, context)

      case 'causal-impact':
        return await runCausalImpact(configuration, context)

      case 'propensity-weighting':
      case 'propensity-matching':
      case 'doubly-robust':
        return await runPropensity(configuration, context)

      case 'continuous-gps':
        return await runContinuousGps(configuration, context)

      default:
        return assertNever(configuration)
    }
  } catch (cause: unknown) {
    dispatch({
      type: 'run-failed',
      detail: cause instanceof Error ? cause.message : String(cause),
    })
  }
}
