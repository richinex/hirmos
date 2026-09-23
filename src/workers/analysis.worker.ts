import { z } from 'zod'
import { sameStructuralModel } from '@/domain/structuralImpact'
import { ardlModelResponseSchema } from '@/domain/ardlModel'
import { rootCauseResponseSchema } from '@/domain/rootCauseAnalysis'
import { rootCauseChecksResponseSchema } from '@/domain/rootCauseAnalysis'
import { parseMulticollinearityEvidence } from '@/domain/multicollinearity'
import { parseGrangerSsrEvidence } from '@/domain/granger'
/// <reference lib="webworker" />

import initWasm, { runAnalysis } from '@/generated/analysis-wasm/hirmos_analysis'
import {
  parseDynotearsEvidence,
  parseDirectLingamEvidence,
  parseFciEvidence,
  parsePcStableEvidence,
  parseLpcmciEvidence,
  parseRpcmciEvidence,
  parseOcseEvidence,
  parseCmlpEvidence,
  parseClstmEvidence,
  parseCdnotsResult,
  parseCdnotsPlusResult,
  parseGraceEvidence,
  parseJpcmciPlusEvidence,
  parsePcmciPlusEvidence,
  parseVarLingamEvidence,
} from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import { parseBackdoorLinearEvidence, parseCausalEffectsEvidence, parseCausalImpactEvidence, parseCountGlmEvidence, parseFrontdoorTwoStageEvidence, parseInstrumentalVariableEvidence, parseNegativeBinomialIngarchEvidence } from '@/domain/estimation'
import { parseCountSeriesInterventionScanEvidence } from '@/domain/countSeries'
import { parseInterruptedSeriesEvidence } from '@/domain/interruptedSeries'
import { parseMissingnessResolvedEvidence } from '@/domain/missingness'
import { parseSeasonalAdjustedEvidence } from '@/domain/seasonal'
import { parsePandasResamplingEvidence } from '@/domain/resampling'
import { ardlEvidenceSchema, bayesianGaussianEvidenceSchema, binaryEttEvidenceSchema, discreteBnEvidenceSchema, doubleMlEvidenceSchema, negbinNutsEvidenceSchema, panelInterventionEvidenceSchema, syntheticControlEvidenceSchema, tLearnerEvidenceSchema, vecmEvidenceSchema } from '@/domain/estimation'
import { parseDmlRefutationEvidence } from '@/domain/sensitivity'
import { dynamicCounterfactualUncertaintyMatches, dynamicLinearScmEvidenceSchema, linearScmEvidenceSchema } from '@/domain/counterfactual'
import { parseLinearRefutationEvidence, parseSeriesStructureEvidence, parseUnobservedConfoundingEvidence } from '@/domain/sensitivity'
import { parseStationarityBattery } from '@/domain/stationarity'
import { parseBackdoorIdentificationEvidence } from '@/domain/study'
import { dagCheckEvidenceSchema } from '@/domain/dagValidation'
import { identifiedDiscreteQueryEvidenceSchema } from '@/domain/intervention'
import { parseComparisonSurvivalEvidence, parseCoxRegressionEvidence, parseFlexSurvEvidence, parseMultiStateSurvivalEvidence, parseNonparametricSurvivalEvidence, parsePenalizedAftEvidence } from '@/domain/survival'
import { parseAalenEvidence, parseForestEvidence } from '@/domain/survivalRegression'
import {
  analysisProgressSchema,
  parseAnalysisRefusal,
  parseAnalysisWorkerCommand,
  type AnalysisWorkerCommand,
  type AnalysisWorkerEvent,
  type AnalysisWorkerProblem,
  type WorkerRequestId,
  type TemporalSamples,
} from './analysisProtocol'

const emit = (event: AnalysisWorkerEvent) => self.postMessage(event)
let wasmReady: Promise<unknown> | null = null

const detailOf = (cause: unknown): string => cause instanceof Error ? cause.message : String(cause)

const fail = (request: WorkerRequestId, problem: AnalysisWorkerProblem) =>
  emit({ kind: 'analysis-failed', request, problem })

const loadWasm = (): Promise<unknown> => {
  wasmReady ??= initWasm().catch((cause: unknown) => {
    wasmReady = null
    throw cause
  })
  return wasmReady
}

const rustTemporalSamples = (samples: TemporalSamples): object => {
  switch (samples.kind) {
    case 'dense': return { kind: 'dense' }
    case 'role-aware': return {
      kind: 'roleAware',
      cutOff: samples.cutOff,
      propagateThroughMaxLag: samples.propagateThroughMaxLag,
      maskType: samples.maskType,
    }
    default: return assertNever(samples)
  }
}

const sampleArrays = (command: AnalysisWorkerCommand): readonly [Uint8Array, Uint8Array] => {
  if ((command.kind === 'pcmci-plus' || command.kind === 'lpcmci') && command.samples.kind === 'role-aware') {
    return [command.samples.validity, command.samples.analysisMask]
  }
  if (command.kind === 'cdnots' || command.kind === 'cdnots-plus' || command.kind === 'grace') {
    return [command.validity, new Uint8Array()]
  }
  return [new Uint8Array(), new Uint8Array()]
}

const rustCommand = (command: AnalysisWorkerCommand): object => {
  switch (command.kind) {
    case 'aalen': return { kind: 'aalen', rows: command.rows, columns: command.columns, ...command.design }
    case 'survival-forest': return { kind: 'survivalForest', rows: command.rows, columns: command.columns, ...command.design }
    case 'flexsurv':
      return { kind: 'flexSurv', rows: command.rows, columns: command.columns, observation: command.observation, rowFrequency: command.rowFrequency, covariates: command.covariates, family: command.family, predictionTimes: command.predictionTimes }
    case 'cox-regression':
      return { kind: 'coxRegression', rows: command.rows, columns: command.columns, ...command.design }
    case 'penalized-aft':
      return { kind: 'penalizedAft', rows: command.rows, columns: command.columns, duration: command.duration, event: command.event, covariates: command.covariates, family: command.family, penalizer: command.penalizer, confidenceLevel: command.confidenceLevel, predictionTimes: command.predictionTimes }
    case 'nonparametric-survival':
      return { kind: 'nonparametricSurvival', rows: command.rows, columns: command.columns, duration: command.duration, event: command.event, rowFrequency: command.rowFrequency, predictionTimes: command.predictionTimes, ties: command.ties }
    case 'comparison-survival':
      return { kind: 'comparisonSurvival', rows: command.rows, columns: command.columns, duration: command.duration, event: command.event, group: command.group, truncationTime: command.truncationTime, permutations: command.permutations, seed: command.seed }
    case 'multi-state-survival':
      return { kind: 'multiStateSurvival', rows: command.rows, columns: command.columns, input: command.input, family: command.family, predictionTimes: command.predictionTimes }
    case 'stationarity-battery':
      return { kind: 'stationarityBattery' }
    case 'multicollinearity':
      return {
        kind: 'multicollinearity',
        rows: command.rows,
        columns: command.columns,
        correlationThreshold: command.correlationThreshold,
        vifThreshold: command.vifThreshold,
      }
    case 'pandas-resample-daily':
      return {
        kind: 'pandasResampleDaily',
        rows: command.rows,
        columns: command.columns,
        target: command.target,
        incompleteBins: command.incompleteBins,
        aggregations: command.aggregations,
        imputedCells: command.imputedCells,
      }
    case 'pcmci-plus':
      return {
        kind: 'pcmciPlus',
        rows: command.rows,
        columns: command.columns,
        tauMax: command.tauMax,
        pcAlpha: command.pcAlpha,
        samples: rustTemporalSamples(command.samples),
      }
    case 'jpcmci-plus':
      return {
        kind: 'jpcmciplus',
        rows: command.rows,
        datasets: command.datasets,
        periods: command.periods,
        observedColumns: command.observedColumns,
        classes: command.classes,
        timeDummy: command.timeDummy,
        spaceDummy: command.spaceDummy,
        tauMax: command.tauMax,
        pcAlpha: command.pcAlpha,
      }
    case 'lpcmci':
      return {
        kind: 'lpcmci',
        rows: command.rows,
        columns: command.columns,
        tauMax: command.tauMax,
        pcAlpha: command.pcAlpha,
        samples: rustTemporalSamples(command.samples),
      }
    case 'rpcmci':
      return {
        kind: 'rpcmci',
        rows: command.rows,
        columns: command.columns,
        numRegimes: command.numRegimes,
        maxTransitions: command.maxTransitions,
        switchThres: command.switchThres,
        numIterations: command.numIterations,
        maxAnneal: command.maxAnneal,
        tauMin: command.tauMin,
        tauMax: command.tauMax,
        pcAlpha: command.pcAlpha,
        alphaLevel: command.alphaLevel,
        seed: command.seed,
      }
    case 'cdnots':
    case 'cdnots-plus':
      return {
        kind: command.kind === 'cdnots' ? 'cdnots' : 'cdnotsPlus',
        rows: command.rows,
        columns: command.columns,
        maxLag: command.maxLag,
        alpha: command.alpha,
        missing: command.missing,
        context: command.context,
      }
    case 'grace':
      return {
        kind: 'grace',
        rows: command.rows,
        columns: command.columns,
        maxLag: command.maxLag,
        alpha: command.alpha,
        context: command.context,
        gateThreshold: command.gateThreshold,
        epochs: command.epochs,
        patience: command.patience,
        seed: command.seed,
      }
    case 'dynotears':
      return {
        kind: 'dynotears',
        rows: command.rows,
        columns: command.columns,
        maxLag: command.maxLag,
        lambdaW: command.lambdaW,
        lambdaA: command.lambdaA,
      }
    case 'direct-lingam':
      return {
        kind: 'directLingam',
        rows: command.rows,
        columns: command.columns,
      }
    case 'pc-stable':
      return {
        kind: 'pcStable',
        rows: command.rows,
        columns: command.columns,
        names: command.names,
        alpha: command.alpha,
        maxDepth: command.maxDepth,
        ciTest: command.ciTest,
        background: command.background,
      }
    case 'fci':
      return {
        kind: 'fci',
        rows: command.rows,
        columns: command.columns,
        names: command.names,
        alpha: command.alpha,
        maxDepth: command.maxDepth,
        maxPathLength: command.maxPathLength,
        ciTest: command.ciTest,
        background: command.background,
      }
    case 'var-lingam':
      return {
        kind: 'varLingam',
        rows: command.rows,
        columns: command.columns,
        lags: command.lags,
        prune: command.prune,
      }
    case 'ocse':
      return {
        kind: 'ocse',
        rows: command.rows,
        columns: command.columns,
        maxLag: command.maxLag,
        alpha: command.alpha,
        nShuffles: command.nShuffles,
        method: command.method,
        k: command.k,
      }
    case 'cmlp':
      return {
        kind: 'cmlp',
        rows: command.rows,
        columns: command.columns,
        lag: command.lag,
        hidden: command.hidden,
        activation: command.activation,
        penalty: command.penalty,
        lambda: command.lambda,
        ridgeLambda: command.ridgeLambda,
        learningRate: command.learningRate,
        maxIter: command.maxIter,
        checkEvery: command.checkEvery,
        lookback: command.lookback,
        seed: command.seed,
      }
    case 'clstm':
      return {
        kind: 'clstm',
        rows: command.rows,
        columns: command.columns,
        context: command.context,
        hidden: command.hidden,
        lambda: command.lambda,
        ridgeLambda: command.ridgeLambda,
        learningRate: command.learningRate,
        maxIter: command.maxIter,
        checkEvery: command.checkEvery,
        lookback: command.lookback,
        seed: command.seed,
      }
    case 'granger-ssr-f':
      return { kind: 'grangerSsrF', rows: command.rows, maxLag: command.maxLag }
    case 'backdoor-identify':
      return {
        kind: 'backdoorIdentify',
        nodes: command.nodes,
        names: command.names,
        edges: command.edges,
        treatment: command.treatment,
        outcome: command.outcome,
        unobserved: command.unobserved,
        estimand: command.estimand,
      }
    case 'dag-check':
      return {
        kind: 'dagCheck',
        rows: command.rows,
        columns: command.columns,
        nodeColumns: command.nodeColumns,
        edges: command.edges,
        implications: command.implications,
        maximumObservations: command.maximumObservations,
        permutations: command.permutations,
        significanceLevel: command.significanceLevel,
        runFalsification: command.runFalsification,
      }
    case 'backdoor-linear':
      return {
        kind: 'backdoorLinear',
        rows: command.rows,
        columns: command.columns,
        treatment: command.treatment,
        outcome: command.outcome,
        adjustment: command.adjustment,
        hacMaxLags: command.hacMaxLags,
        level: command.level,
        errorModel: command.errorModel,
      }
    case 'frontdoor-two-stage':
      return {
        kind: 'frontdoorTwoStage',
        rows: command.rows,
        columns: command.columns,
        treatment: command.treatment,
        mediator: command.mediator,
        outcome: command.outcome,
        firstStageAdjustment: command.firstStageAdjustment,
        secondStageAdjustment: command.secondStageAdjustment,
        controlValue: command.controlValue,
        treatmentValue: command.treatmentValue,
        uncertainty: command.uncertainty,
      }
    case 'instrumental-variable':
      return {
        kind: 'instrumentalVariable',
        rows: command.rows,
        columns: command.columns,
        treatment: command.treatment,
        outcome: command.outcome,
        instruments: command.instruments,
        uncertainty: command.uncertainty,
      }
    case 'count-glm':
      return { kind: 'countGlm', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, family: command.family }
    case 'negative-binomial-ingarch':
      return { kind: 'negativeBinomialIngarch', rows: command.rows, columns: command.columns, outcome: command.outcome, link: command.link, regressors: command.regressors, pastObservationLags: command.pastObservationLags, pastMeanLags: command.pastMeanLags, externalRegressors: command.externalRegressors, horizon: command.horizon, baselineRegressors: command.baselineRegressors, interventionRegressor: command.interventionRegressor, controlValue: command.controlValue, treatmentValue: command.treatmentValue, schedule: command.schedule }
    case 'count-series-intervention-scan':
      return { kind: 'countSeriesInterventionScan', rows: command.rows, columns: command.columns, outcome: command.outcome, link: command.link, pastObservationLags: command.pastObservationLags, pastMeanLags: command.pastMeanLags, candidateReferencePoints: command.candidateReferencePoints, delta: command.delta }
    case 'interrupted-series':
      return { kind: 'interruptedSeries', rows: command.rows, columns: command.columns, outcome: command.outcome, model: command.model, interventionRow: command.interventionRow, lag: command.lag, impact: command.impact, seasonal: command.seasonal, ljungBoxLags: command.ljungBoxLags }
    case 'causal-effects-total':
      return { kind: 'causalEffectsTotal', rows: command.rows, columns: command.columns, statLag: command.statLag, graph: command.graph, x: command.x, y: command.y, hidden: command.hidden, estimator: command.estimator, interventions: command.interventions, uncertainty: command.uncertainty }
    case 'sharp-rd': return { kind: 'sharpRd', rows: command.rows, cutoff: command.cutoff }
    case 'causal-impact':
      return { kind: 'causalImpact', rows: command.rows, columns: command.columns, outcome: command.outcome, controls: command.controls, nPre: command.nPre, postEnd: command.postEnd, maxIter: command.maxIter }
    case 'structural-causal-impact':
      return { kind:'structuralCausalImpact', rows:command.rows, columns:command.columns, outcome:command.outcome, controls:command.controls, nPre:command.nPre, postEnd:command.postEnd, draws:command.draws, warmup:command.warmup, seed:command.seed, model:command.model }
    case 'bayesian-causal-impact':
      return { kind: 'bayesianCausalImpact', rows: command.rows, columns: command.columns, outcome: command.outcome, controls: command.controls, nPre: command.nPre, postEnd: command.postEnd, draws: command.draws, warmup: command.warmup, seed: command.seed, priorLevelSd: command.priorLevelSd }
    case 'linear-refutation':
      return { kind: 'linearRefutation', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, simulations: command.simulations, subsetFraction: command.subsetFraction, seed: command.seed, ljungBoxLags: command.ljungBoxLags }
    case 'unobserved-confounding':
      return { kind: 'unobservedConfounding', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, seed: command.seed, kappaT: command.kappaT, kappaY: command.kappaY }
    case 'series-structure':
      return { kind: 'seriesStructure', rows: command.rows, columns: command.columns, period: command.period, robust: command.robust, correlationMaxLag: command.correlationMaxLag, peltMinSize: command.peltMinSize, peltJump: command.peltJump, peltPenalty: command.peltPenalty }
    case 'seasonal-adjust':
      return { kind: 'seasonalAdjust', rows: command.rows, columns: command.columns, period: command.period, robust: command.robust, adjust: command.adjust }
    case 'double-ml':
      return { kind: 'doubleMl', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, model: command.model, att: command.att, seed: command.seed, groups: command.groups }
    case 't-learner':
      return { kind: 'tLearner', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, seed: command.seed, uncertainty: command.uncertainty }
    case 'ardl-pss':
      return { kind: 'ardlPss', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, maxLag: command.maxLag, trend: command.trend, case: command.case }
    case 'ardl-model':
      return { kind: 'ardlModel', rows: command.rows, columns: command.columns, model: command.model }
    case 'root-cause':
      return { kind: 'rootCause', request: command.model }
    case 'gcm-effects':
      return { kind: 'gcmEffects', request: command.model }
    case 'gcm-influence':
      return { kind: 'gcmInfluence', request: command.model }
    case 'root-cause-checks':
      return { kind: 'rootCauseChecks', request: command.model }
    case 'vecm':
      return { kind: 'vecm', rows: command.rows, columns: command.columns, endogenous: command.endogenous, maxLags: command.maxLags, deterministic: command.deterministic, significance: command.significance, breakIndex: command.breakIndex, forecastSteps: command.forecastSteps ?? null }
    case 'synthetic-control':
      return { kind: 'syntheticControl', rows: command.rows, columns: command.columns, treated: command.treated, donors: command.donors, nPre: command.nPre, crossFitFolds: command.crossFitFolds, alpha: command.alpha }
    case 'panel-intervention':
      return { kind: 'panelIntervention', rows: command.rows, units: command.units, times: command.times, placeboReplications: command.placeboReplications, seed: command.seed, primary: command.primary ?? 'syntheticDid' }
    case 'panel-adjusted':
      return { kind: 'panelAdjusted', rows: command.rows, columns: command.columns, units: command.units, times: command.times, specification: command.specification }
    case 'negbin-nuts':
      return { kind: 'negbinNuts', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, confounder: command.confounder, warmup: command.warmup, samples: command.samples, seed: command.seed }
    case 'bayesian-gaussian':
      return { kind: 'bayesianGaussian', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, warmup: command.warmup, samples: command.samples, seed: command.seed }
    case 'discrete-bn-query':
      return { kind: 'discreteBnQuery', rows: command.rows, columns: command.columns, nodes: command.nodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, bins: command.bins, equivalentSampleSize: command.equivalentSampleSize }
    case 'identified-discrete-query':
      return { kind: 'identifiedDiscreteQuery', rows: command.rows, columns: command.columns, observedNodes: command.observedNodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, unobserved: command.unobserved, bins: command.bins, condition: command.condition }
    case 'binary-ett':
      return { kind: 'binaryEtt', rows: command.rows, columns: command.columns, observedNodes: command.observedNodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, unobserved: command.unobserved }
    case 'linear-scm-counterfactual':
      return { kind: 'linearScmCounterfactual', rows: command.rows, columns: command.columns, nodes: command.nodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, interventions: command.interventions, observationNoise: command.observationNoise }
    case 'dynamic-linear-scm-counterfactual':
      return { kind: 'dynamicLinearScmCounterfactual', rows: command.rows, columns: command.columns, nodes: command.nodes, statLag: command.statLag, graph: command.graph, treatment: command.treatment, outcome: command.outcome, timing: command.timing, steps: command.steps, interventions: command.interventions, uncertainty: command.uncertainty }
    case 'dml-refutation-batch':
      return { kind: 'dmlRefutationBatch', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, model: command.model, att: command.att, seed: command.seed }
    case 'resolve-missingness':
      return { kind: 'resolveMissingness', rows: command.rows, columns: command.columns, validity: Array.from(command.validity), resolution: command.resolution }
    default:
      return assertNever(command)
  }
}

self.onmessage = (message: MessageEvent<unknown>) => {
  void (async () => {
    const parsed = parseAnalysisWorkerCommand(message.data)
    if (!parsed.ok) {
      emit({ kind: 'protocol-failed', detail: parsed.error.detail })
      return
    }

    try {
      await loadWasm()
    } catch (cause: unknown) {
      fail(parsed.value.request, { kind: 'wasm-unavailable', detail: detailOf(cause) })
      return
    }

    const command = parsed.value
    let raw: string
    try {
      const [validity, analysisMask] = sampleArrays(command)
      raw = runAnalysis(JSON.stringify(rustCommand(command)), command.values, validity, analysisMask, (stage: unknown, completed: unknown, total: unknown) => {
        const progress = analysisProgressSchema.safeParse({ stage, completed, total })
        if (progress.success) emit({ kind: 'analysis-progress', request: command.request, progress: progress.data })
      })
    } catch (cause: unknown) {
      fail(command.request, { kind: 'kernel-refused', detail: detailOf(cause) })
      return
    }

    let decoded: unknown
    try {
      decoded = JSON.parse(raw)
    } catch (cause: unknown) {
      fail(command.request, {
        kind: 'worker-protocol-failed',
        detail: `Rust returned invalid JSON: ${detailOf(cause)}`,
      })
      return
    }
    const refusal = parseAnalysisRefusal(decoded)
    if (!refusal.ok) {
      fail(command.request, { kind: 'worker-protocol-failed', detail: refusal.error.detail })
      return
    }
    if (refusal.value !== null) {
      fail(command.request, refusal.value)
      return
    }
    switch (command.kind) {
      case 'flexsurv': {
        const result = parseFlexSurvEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'flexsurv-succeeded', request: command.request, result: result.value })
        return
      }
      case 'cox-regression': {
        const result = parseCoxRegressionEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'cox-regression-succeeded', request: command.request, result: result.value })
        return
      }
      case 'penalized-aft': {
        const result = parsePenalizedAftEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'penalized-aft-succeeded', request: command.request, result: result.value })
        return
      }
      case 'aalen': {
        const result = parseAalenEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'aalen-succeeded', request: command.request, result: result.value })
        return
      }
      case 'survival-forest': {
        const result = parseForestEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'survival-forest-succeeded', request: command.request, result: result.value })
        return
      }
      case 'nonparametric-survival': {
        const result = parseNonparametricSurvivalEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'nonparametric-survival-succeeded', request: command.request, result: result.value })
        return
      }
      case 'comparison-survival': {
        const result = parseComparisonSurvivalEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'comparison-survival-succeeded', request: command.request, result: result.value })
        return
      }
      case 'multi-state-survival': {
        const result = parseMultiStateSurvivalEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'multi-state-survival-succeeded', request: command.request, result: result.value })
        return
      }
      case 'stationarity-battery': {
        const result = parseStationarityBattery(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'stationarity-succeeded', request: command.request, result: result.value })
        return
      }
      case 'multicollinearity': {
        const result = parseMulticollinearityEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.kind === 'invalid-evidence' ? result.error.detail : 'The multicollinearity result does not match its columns.' })
          return
        }
        emit({ kind: 'multicollinearity-succeeded', request: command.request, result: result.value })
        return
      }
      case 'pandas-resample-daily': {
        const result = parsePandasResamplingEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'pandas-resampling-succeeded', request: command.request, result: result.value })
        return
      }
      case 'pcmci-plus': {
        const result = parsePcmciPlusEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'pcmci-plus-succeeded', request: command.request, result: result.value })
        return
      }
      case 'jpcmci-plus': {
        const result = parseJpcmciPlusEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'jpcmci-plus-succeeded', request: command.request, result: result.value })
        return
      }
      case 'lpcmci': {
        const result = parseLpcmciEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'lpcmci-succeeded', request: command.request, result: result.value })
        return
      }
      case 'rpcmci': {
        const result = parseRpcmciEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'rpcmci-succeeded', request: command.request, result: result.value })
        return
      }
      case 'cdnots': {
        const result = parseCdnotsResult(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'cdnots-succeeded', request: command.request, result: result.value })
        return
      }
      case 'cdnots-plus': {
        const result = parseCdnotsPlusResult(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'cdnots-plus-succeeded', request: command.request, result: result.value })
        return
      }
      case 'grace': {
        const result = parseGraceEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'grace-succeeded', request: command.request, result: result.value })
        return
      }
      case 'dynotears': {
        const result = parseDynotearsEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'dynotears-succeeded', request: command.request, result: result.value })
        return
      }
      case 'direct-lingam': {
        const result = parseDirectLingamEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'direct-lingam-succeeded', request: command.request, result: result.value })
        return
      }
      case 'pc-stable': {
        const result = parsePcStableEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'pc-stable-succeeded', request: command.request, result: result.value })
        return
      }
      case 'fci': {
        const result = parseFciEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'fci-succeeded', request: command.request, result: result.value })
        return
      }
      case 'var-lingam': {
        const result = parseVarLingamEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'var-lingam-succeeded', request: command.request, result: result.value })
        return
      }
      case 'ocse': {
        const result = parseOcseEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'ocse-succeeded', request: command.request, result: result.value })
        return
      }
      case 'cmlp': {
        const result = parseCmlpEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'cmlp-succeeded', request: command.request, result: result.value })
        return
      }
      case 'clstm': {
        const result = parseClstmEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'clstm-succeeded', request: command.request, result: result.value })
        return
      }
      case 'granger-ssr-f': {
        const result = parseGrangerSsrEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'granger-succeeded', request: command.request, result: result.value })
        return
      }
      case 'backdoor-identify': {
        const result = parseBackdoorIdentificationEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'backdoor-identification-succeeded', request: command.request, result: result.value })
        return
      }
      case 'dag-check': {
        const result = dagCheckEvidenceSchema.safeParse(decoded)
        if (!result.success) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) })
          return
        }
        emit({ kind: 'dag-check-succeeded', request: command.request, result: result.data })
        return
      }
      case 'backdoor-linear': {
        const result = parseBackdoorLinearEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'backdoor-linear-succeeded', request: command.request, result: result.value })
        return
      }
      case 'frontdoor-two-stage': {
        const result = parseFrontdoorTwoStageEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'frontdoor-two-stage-succeeded', request: command.request, result: result.value })
        return
      }
      case 'instrumental-variable': {
        const result = parseInstrumentalVariableEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'instrumental-variable-succeeded', request: command.request, result: result.value })
        return
      }
      case 'count-glm': {
        const result = parseCountGlmEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'count-glm-succeeded', request: command.request, result: result.value })
        return
      }
      case 'negative-binomial-ingarch': {
        const result = parseNegativeBinomialIngarchEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'negative-binomial-ingarch-succeeded', request: command.request, result: result.value })
        return
      }
      case 'count-series-intervention-scan': {
        const result = parseCountSeriesInterventionScanEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'count-series-intervention-scan-succeeded', request: command.request, result: result.value })
        return
      }
      case 'interrupted-series': {
        const result = parseInterruptedSeriesEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'interrupted-series-succeeded', request: command.request, result: result.value })
        return
      }
      case 'causal-effects-total': {
        const result = parseCausalEffectsEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'causal-effects-succeeded', request: command.request, result: result.value })
        return
      }
      case 'sharp-rd': {
        const result = parseSharpRdEvidence(decoded)
        if (!result.ok) throw new Error(result.error.detail)
        if (result.value.cutoff !== command.cutoff || result.value.points.length !== command.rows) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: 'The RD result does not match the requested cutoff or sample.' }); return
        }
        emit({ kind: 'sharp-rd-succeeded', request: command.request, result: result.value })
        return
      }
      case 'structural-causal-impact':
      case 'bayesian-causal-impact':
      case 'causal-impact': {
        const result = parseCausalImpactEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        const e = result.value
        const settingsMatch = command.kind === 'structural-causal-impact'
          ? e.kind === 'structuralCausalImpact' && e.draws === command.draws && e.warmup === command.warmup && e.seed === command.seed && sameStructuralModel(e.model,command.model)
          : command.kind === 'bayesian-causal-impact'
          ? e.kind === 'bayesianCausalImpact' && e.draws === command.draws && e.warmup === command.warmup && e.seed === command.seed && e.priorLevelSd === command.priorLevelSd
          : e.kind === 'causalImpact'
        if (!settingsMatch || e.observations !== command.rows || e.nPre !== command.nPre || e.postEnd !== command.postEnd || e.outcome !== command.outcome || e.controls.length !== command.controls.length || e.controls.some((value,index) => value !== command.controls[index])) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: 'The impact result does not match its requested model, columns or intervention window.' }); return
        }
        emit({ kind: 'causal-impact-succeeded', request: command.request, result: result.value })
        return
      }
      case 'linear-refutation': {
        const result = parseLinearRefutationEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'linear-refutation-succeeded', request: command.request, result: result.value })
        return
      }
      case 'unobserved-confounding': {
        const result = parseUnobservedConfoundingEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'unobserved-confounding-succeeded', request: command.request, result: result.value })
        return
      }
      case 'series-structure': {
        const result = parseSeriesStructureEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'series-structure-succeeded', request: command.request, result: result.value })
        return
      }
      case 'double-ml': {
        const result = doubleMlEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'double-ml-succeeded', request: command.request, result: result.data })
        return
      }
      case 't-learner': {
        const result = tLearnerEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 't-learner-succeeded', request: command.request, result: result.data })
        return
      }
      case 'ardl-pss': {
        const result = ardlEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'ardl-succeeded', request: command.request, result: result.data })
        return
      }
      case 'ardl-model': {
        const result = ardlModelResponseSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'ardl-model-succeeded', request: command.request, result: result.data.evidence })
        return
      }
      case 'root-cause': {
        const result = rootCauseResponseSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'root-cause-succeeded', request: command.request, result: result.data.evidence })
        return
      }
      case 'gcm-effects': {
        const result = gcmEffectsResponseSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'gcm-effects-succeeded', request: command.request, result: result.data.evidence })
        return
      }
      case 'gcm-influence': {
        const result = gcmInfluenceResponseSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'gcm-influence-succeeded', request: command.request, result: result.data.evidence })
        return
      }
      case 'root-cause-checks': {
        const result = rootCauseChecksResponseSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'root-cause-checks-succeeded', request: command.request, result: result.data.evidence })
        return
      }
      case 'vecm': {
        const result = vecmEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'vecm-succeeded', request: command.request, result: result.data })
        return
      }
      case 'synthetic-control': {
        const result = syntheticControlEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'synthetic-control-succeeded', request: command.request, result: result.data })
        return
      }
      case 'panel-adjusted':
      case 'panel-intervention': {
        const result = panelInterventionEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        const evidence = result.data
        const matches = command.kind === 'panel-adjusted'
          ? evidence.kind === 'panelAdjusted' && sameDidSpecification(command.specification,evidence.specification) && evidence.covariates === command.columns-2
          : command.primary === 'did' ? evidence.kind === 'panelDid' : evidence.kind === 'panelIntervention'
        if (!matches || evidence.observations !== command.rows) { fail(command.request, { kind: 'worker-protocol-failed', detail: 'The panel result does not match the requested estimator and data dimensions.' }); return }
        emit({ kind: 'panel-intervention-succeeded', request: command.request, result: result.data })
        return
      }
      case 'negbin-nuts': {
        const result = negbinNutsEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'negbin-nuts-succeeded', request: command.request, result: result.data })
        return
      }
      case 'bayesian-gaussian': {
        const result = bayesianGaussianEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'bayesian-gaussian-succeeded', request: command.request, result: result.data })
        return
      }
      case 'discrete-bn-query': {
        const result = discreteBnEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'discrete-bn-succeeded', request: command.request, result: result.data })
        return
      }
      case 'identified-discrete-query': {
        const result = identifiedDiscreteQueryEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'identified-discrete-query-succeeded', request: command.request, result: result.data })
        return
      }
      case 'binary-ett': {
        const result = binaryEttEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'binary-ett-succeeded', request: command.request, result: result.data })
        return
      }
      case 'linear-scm-counterfactual': {
        const result = linearScmEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'linear-scm-succeeded', request: command.request, result: result.data })
        return
      }
      case 'dynamic-linear-scm-counterfactual': {
        const result = dynamicLinearScmEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        if (!dynamicCounterfactualUncertaintyMatches(command.uncertainty, result.data.uncertainty)) { fail(command.request, { kind: 'worker-protocol-failed', detail: 'Dynamic counterfactual uncertainty evidence does not match the requested configuration.' }); return }
        emit({ kind: 'dynamic-linear-scm-succeeded', request: command.request, result: result.data })
        return
      }
      case 'dml-refutation-batch': {
        const result = parseDmlRefutationEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'dml-refutation-succeeded', request: command.request, result: result.value })
        return
      }
      case 'seasonal-adjust': {
        const result = parseSeasonalAdjustedEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'seasonal-adjusted', request: command.request, result: result.value })
        return
      }
      case 'resolve-missingness': {
        const result = parseMissingnessResolvedEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'missingness-resolved', request: command.request, result: result.value })
        return
      }
      default:
        return assertNever(command)
    }
  })()
}
import { gcmEffectsResponseSchema } from '@/domain/gcmEffects'
import { gcmInfluenceResponseSchema } from '@/domain/gcmInfluence'
import { parseSharpRdEvidence } from '@/domain/sharpRd'
import { sameDidSpecification } from '@/domain/adjustedDid'
