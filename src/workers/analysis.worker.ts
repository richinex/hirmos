import { z } from 'zod'
import { parseGrangerSsrEvidence } from '@/domain/granger'
/// <reference lib="webworker" />

import initWasm, { runAnalysis } from '@/generated/analysis-wasm/hirmos_analysis'
import {
  parseDynotearsEvidence,
  parseLpcmciEvidence,
  parseOcseEvidence,
  parsePcmciPlusEvidence,
  parseVarLingamEvidence,
} from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import { parseBackdoorLinearEvidence, parseCausalEffectsEvidence, parseCausalImpactEvidence, parseCountGlmEvidence } from '@/domain/estimation'
import { parseMissingnessResolvedEvidence } from '@/domain/missingness'
import { parseSeasonalAdjustedEvidence } from '@/domain/seasonal'
import { ardlEvidenceSchema, bayesianGaussianEvidenceSchema, discreteBnEvidenceSchema, doubleMlEvidenceSchema, negbinNutsEvidenceSchema, panelInterventionEvidenceSchema, syntheticControlEvidenceSchema, vecmEvidenceSchema } from '@/domain/estimation'
import { parseDmlRefutationEvidence } from '@/domain/sensitivity'
import { linearScmEvidenceSchema } from '@/domain/counterfactual'
import { parseLinearRefutationEvidence, parseSeriesStructureEvidence, parseUnobservedConfoundingEvidence } from '@/domain/sensitivity'
import { parseStationarityBattery } from '@/domain/stationarity'
import { parseBackdoorIdentificationEvidence } from '@/domain/study'
import { dagCheckEvidenceSchema } from '@/domain/dagValidation'
import {
  analysisProgressSchema,
  parseAnalysisWorkerCommand,
  type AnalysisWorkerCommand,
  type AnalysisWorkerEvent,
  type AnalysisWorkerProblem,
  type WorkerRequestId,
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

const rustCommand = (command: AnalysisWorkerCommand): object => {
  switch (command.kind) {
    case 'stationarity-battery':
      return { kind: 'stationarityBattery' }
    case 'pcmci-plus':
      return {
        kind: 'pcmciPlus',
        rows: command.rows,
        columns: command.columns,
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
    case 'granger-ssr-f':
      return { kind: 'grangerSsrF', rows: command.rows, maxLag: command.maxLag }
    case 'backdoor-identify':
      return {
        kind: 'backdoorIdentify',
        nodes: command.nodes,
        edges: command.edges,
        treatment: command.treatment,
        outcome: command.outcome,
        unobserved: command.unobserved,
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
      }
    case 'count-glm':
      return { kind: 'countGlm', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, family: command.family }
    case 'causal-effects-total':
      return { kind: 'causalEffectsTotal', rows: command.rows, columns: command.columns, statLag: command.statLag, graph: command.graph, x: command.x, y: command.y, hidden: command.hidden, estimator: command.estimator, interventions: command.interventions }
    case 'causal-impact':
      return { kind: 'causalImpact', rows: command.rows, columns: command.columns, outcome: command.outcome, controls: command.controls, nPre: command.nPre, maxIter: command.maxIter }
    case 'linear-refutation':
      return { kind: 'linearRefutation', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, simulations: command.simulations, subsetFraction: command.subsetFraction, seed: command.seed, ljungBoxLags: command.ljungBoxLags }
    case 'unobserved-confounding':
      return { kind: 'unobservedConfounding', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, seed: command.seed, kappaT: command.kappaT, kappaY: command.kappaY }
    case 'series-structure':
      return { kind: 'seriesStructure', rows: command.rows, columns: command.columns, period: command.period, robust: command.robust, peltMinSize: command.peltMinSize, peltJump: command.peltJump, peltPenalty: command.peltPenalty }
    case 'seasonal-adjust':
      return { kind: 'seasonalAdjust', rows: command.rows, columns: command.columns, period: command.period, robust: command.robust, adjust: command.adjust }
    case 'double-ml':
      return { kind: 'doubleMl', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, model: command.model, att: command.att, seed: command.seed }
    case 'ardl-pss':
      return { kind: 'ardlPss', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, maxLag: command.maxLag, trend: command.trend, case: command.case }
    case 'vecm':
      return { kind: 'vecm', rows: command.rows, columns: command.columns, endogenous: command.endogenous, maxLags: command.maxLags, deterministic: command.deterministic, significance: command.significance, breakIndex: command.breakIndex }
    case 'synthetic-control':
      return { kind: 'syntheticControl', rows: command.rows, columns: command.columns, treated: command.treated, donors: command.donors, nPre: command.nPre }
    case 'panel-intervention':
      return { kind: 'panelIntervention', rows: command.rows, units: command.units, times: command.times }
    case 'negbin-nuts':
      return { kind: 'negbinNuts', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, confounder: command.confounder, warmup: command.warmup, samples: command.samples, seed: command.seed }
    case 'bayesian-gaussian':
      return { kind: 'bayesianGaussian', rows: command.rows, columns: command.columns, treatment: command.treatment, outcome: command.outcome, adjustment: command.adjustment, warmup: command.warmup, samples: command.samples, seed: command.seed }
    case 'discrete-bn-query':
      return { kind: 'discreteBnQuery', rows: command.rows, columns: command.columns, nodes: command.nodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, bins: command.bins, equivalentSampleSize: command.equivalentSampleSize }
    case 'linear-scm-counterfactual':
      return { kind: 'linearScmCounterfactual', rows: command.rows, columns: command.columns, nodes: command.nodes, names: command.names, edges: command.edges, treatment: command.treatment, outcome: command.outcome, interventions: command.interventions, observationNoise: command.observationNoise }
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
      raw = runAnalysis(JSON.stringify(rustCommand(command)), command.values, (stage: unknown, completed: unknown, total: unknown) => {
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
    switch (command.kind) {
      case 'stationarity-battery': {
        const result = parseStationarityBattery(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'stationarity-succeeded', request: command.request, result: result.value })
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
      case 'lpcmci': {
        const result = parseLpcmciEvidence(decoded)
        if (!result.ok) {
          fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail })
          return
        }
        emit({ kind: 'lpcmci-succeeded', request: command.request, result: result.value })
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
      case 'count-glm': {
        const result = parseCountGlmEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'count-glm-succeeded', request: command.request, result: result.value })
        return
      }
      case 'causal-effects-total': {
        const result = parseCausalEffectsEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
        emit({ kind: 'causal-effects-succeeded', request: command.request, result: result.value })
        return
      }
      case 'causal-impact': {
        const result = parseCausalImpactEvidence(decoded)
        if (!result.ok) { fail(command.request, { kind: 'worker-protocol-failed', detail: result.error.detail }); return }
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
      case 'ardl-pss': {
        const result = ardlEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'ardl-succeeded', request: command.request, result: result.data })
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
      case 'panel-intervention': {
        const result = panelInterventionEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
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
      case 'linear-scm-counterfactual': {
        const result = linearScmEvidenceSchema.safeParse(decoded)
        if (!result.success) { fail(command.request, { kind: 'worker-protocol-failed', detail: z.prettifyError(result.error) }); return }
        emit({ kind: 'linear-scm-succeeded', request: command.request, result: result.data })
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
