import { err, ok, type Result } from '@/domain/dop'
import type { CountSeriesInterventionScanEvidence } from '@/domain/countSeries'
import { GrangerSsrEvidence } from '@/domain/granger'
import type {
  DynotearsEvidence,
  DirectLingamEvidence,
  LpcmciEvidence,
  RpcmciEvidence,
  OcseEvidence,
  CmlpEvidence,
  ClstmEvidence,
  PcmciPlusEvidence,
  VarLingamEvidence,
} from '@/domain/discovery'
import type { BackdoorLinearEvidence, CausalEffectsEvidence, CausalEffectsUncertainty, CausalImpactEvidence, CountGlmEvidence, FrontdoorTwoStageEvidence, IngarchInterventionSchedule, NegativeBinomialIngarchEvidence, TotalEffectEstimator } from '@/domain/estimation'
import type { MissingnessResolutionCommand, MissingnessResolvedEvidence } from '@/domain/missingness'
import type { SeasonalAdjustedEvidence } from '@/domain/seasonal'
import type { ArdlEvidence, BayesianGaussianEvidence, BinaryEttEvidence, DiscreteBnEvidence, DoubleMlEvidence, NegbinNutsEvidence, PanelInterventionEvidence, SyntheticControlEvidence, VecmEvidence } from '@/domain/estimation'
import type { DmlRefutationEvidence } from '@/domain/sensitivity'
import type { DynamicCounterfactualUncertainty, DynamicInterventionTiming, DynamicLinearScmEvidence, LinearScmEvidence } from '@/domain/counterfactual'
import type { LinearRefutationEvidence, SeriesStructureEvidence, UnobservedConfoundingEvidence } from '@/domain/sensitivity'
import type { StationarityBattery } from '@/domain/stationarity'
import type { PandasResamplingEvidence, ResamplingAggregation } from '@/domain/resampling'
import type { BackdoorIdentificationEvidence } from '@/domain/study'
import type { DagCheckEvidence } from '@/domain/dagValidation'
import type { IdentifiedDiscreteQueryEvidence } from '@/domain/intervention'
import {
  newWorkerRequestId,
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisProgress,
  type AnalysisWorkerEvent,
  type AnalysisWorkerProblem,
  type WorkerRequestId,
} from '@/workers/analysisProtocol'

type StationarityOutcome = Result<StationarityBattery, AnalysisWorkerProblem>
type PandasResamplingOutcome = Result<PandasResamplingEvidence, AnalysisWorkerProblem>
type PcmciPlusOutcome = Result<PcmciPlusEvidence, AnalysisWorkerProblem>
type GrangerOutcome = Result<GrangerSsrEvidence, AnalysisWorkerProblem>
type LpcmciOutcome = Result<LpcmciEvidence, AnalysisWorkerProblem>
type RpcmciOutcome = Result<RpcmciEvidence, AnalysisWorkerProblem>
type DynotearsOutcome = Result<DynotearsEvidence, AnalysisWorkerProblem>
type DirectLingamOutcome = Result<DirectLingamEvidence, AnalysisWorkerProblem>
type VarLingamOutcome = Result<VarLingamEvidence, AnalysisWorkerProblem>
type OcseOutcome = Result<OcseEvidence, AnalysisWorkerProblem>
type CmlpOutcome = Result<CmlpEvidence, AnalysisWorkerProblem>
type ClstmOutcome = Result<ClstmEvidence, AnalysisWorkerProblem>
type BackdoorIdentificationOutcome = Result<BackdoorIdentificationEvidence, AnalysisWorkerProblem>
type DagCheckOutcome = Result<DagCheckEvidence, AnalysisWorkerProblem>
type BackdoorLinearOutcome = Result<BackdoorLinearEvidence, AnalysisWorkerProblem>
type FrontdoorTwoStageOutcome = Result<FrontdoorTwoStageEvidence, AnalysisWorkerProblem>
type CountGlmOutcome = Result<CountGlmEvidence, AnalysisWorkerProblem>
type NegativeBinomialIngarchOutcome = Result<NegativeBinomialIngarchEvidence, AnalysisWorkerProblem>
type CountSeriesInterventionScanOutcome = Result<CountSeriesInterventionScanEvidence, AnalysisWorkerProblem>
type CausalEffectsOutcome = Result<CausalEffectsEvidence, AnalysisWorkerProblem>
type CausalImpactOutcome = Result<CausalImpactEvidence, AnalysisWorkerProblem>
type LinearRefutationOutcome = Result<LinearRefutationEvidence, AnalysisWorkerProblem>
type UnobservedConfoundingOutcome = Result<UnobservedConfoundingEvidence, AnalysisWorkerProblem>
type SeriesStructureOutcome = Result<SeriesStructureEvidence, AnalysisWorkerProblem>
type MissingnessOutcome = Result<MissingnessResolvedEvidence, AnalysisWorkerProblem>
type SeasonalOutcome = Result<SeasonalAdjustedEvidence, AnalysisWorkerProblem>
type DoubleMlOutcome = Result<DoubleMlEvidence, AnalysisWorkerProblem>
type DmlRefutationOutcome = Result<DmlRefutationEvidence, AnalysisWorkerProblem>
type ArdlOutcome = Result<ArdlEvidence, AnalysisWorkerProblem>
type VecmOutcome = Result<VecmEvidence, AnalysisWorkerProblem>
type SyntheticOutcome = Result<SyntheticControlEvidence, AnalysisWorkerProblem>
type PanelInterventionOutcome = Result<PanelInterventionEvidence, AnalysisWorkerProblem>
type NegbinNutsOutcome = Result<NegbinNutsEvidence, AnalysisWorkerProblem>
type BayesianGaussianOutcome = Result<BayesianGaussianEvidence, AnalysisWorkerProblem>
type DiscreteBnOutcome = Result<DiscreteBnEvidence, AnalysisWorkerProblem>
type IdentifiedDiscreteQueryOutcome = Result<IdentifiedDiscreteQueryEvidence, AnalysisWorkerProblem>
type BinaryEttOutcome = Result<BinaryEttEvidence, AnalysisWorkerProblem>
type LinearScmOutcome = Result<LinearScmEvidence, AnalysisWorkerProblem>
type DynamicLinearScmOutcome = Result<DynamicLinearScmEvidence, AnalysisWorkerProblem>
type SuccessfulAnalysisEvent = Extract<AnalysisWorkerEvent, { readonly result: unknown }>
type SuccessfulAnalysisEventKind = SuccessfulAnalysisEvent['kind']
type SuccessfulAnalysisResults = {
  readonly [Event in SuccessfulAnalysisEvent as Event['kind']]: Event['result']
}
type SuccessfulAnalysisResult<Kind extends SuccessfulAnalysisEventKind> = SuccessfulAnalysisResults[Kind]

type PendingCompletion =
  | { readonly kind: 'completed' }
  | {
      readonly kind: 'unexpected-event'
      readonly expected: SuccessfulAnalysisEventKind
      readonly received: SuccessfulAnalysisEventKind
    }

interface PendingRun {
  readonly expected: SuccessfulAnalysisEventKind
  readonly complete: (event: SuccessfulAnalysisEvent) => PendingCompletion
  readonly reject: (problem: AnalysisWorkerProblem) => void
  readonly onProgress?: (progress: AnalysisProgress) => void
}

function hasEventKind<Kind extends SuccessfulAnalysisEventKind>(
  event: SuccessfulAnalysisEvent,
  kind: Kind,
): event is Extract<SuccessfulAnalysisEvent, { readonly kind: Kind }> {
  return event.kind === kind
}

function pendingRun<Kind extends SuccessfulAnalysisEventKind>(
  expected: Kind,
  resolve: (outcome: Result<SuccessfulAnalysisResult<Kind>, AnalysisWorkerProblem>) => void,
  onProgress?: (progress: AnalysisProgress) => void,
): PendingRun {
  return {
    expected,
    onProgress,
    reject: (problem) => resolve(err(problem)),
    complete: (event) => {
      if (!hasEventKind(event, expected)) {
        return { kind: 'unexpected-event', expected, received: event.kind }
      }

      // The discriminant check above establishes the generic event/result correlation that
      // TypeScript cannot retain after indexing a union by a generic literal.
      resolve(ok(event.result as SuccessfulAnalysisResult<Kind>))
      return { kind: 'completed' }
    },
  }
}

let worker: Worker | null = null
const pending = new Map<WorkerRequestId, PendingRun>()

const failAll = (problem: AnalysisWorkerProblem) => {
  for (const run of pending.values()) run.reject(problem)
  pending.clear()
  worker?.terminate()
  worker = null
}

export const cancelAnalysisRuns = (): void => {
  failAll({ kind: 'analysis-cancelled', detail: 'The analysis run was cancelled.' })
}

const analysisWorker = (): Worker => {
  if (worker) return worker
  const created = new Worker(new URL('../workers/analysis.worker.ts', import.meta.url), {
    type: 'module',
    name: 'hirmos-analysis',
  })
  created.onmessage = (message: MessageEvent<unknown>) => {
    const parsed = parseAnalysisWorkerEvent(message.data)
    if (!parsed.ok) {
      failAll({ kind: 'worker-protocol-failed', detail: parsed.error.detail })
      return
    }
    if (parsed.value.kind === 'protocol-failed') {
      failAll({ kind: 'worker-protocol-failed', detail: parsed.value.detail })
      return
    }
    const run = pending.get(parsed.value.request)
    if (!run) return
    if (parsed.value.kind === 'analysis-progress') {
      run.onProgress?.(parsed.value.progress)
      return
    }
    if (parsed.value.kind === 'analysis-failed') {
      pending.delete(parsed.value.request)
      run.reject(parsed.value.problem)
      return
    }

    pending.delete(parsed.value.request)
    const completion = run.complete(parsed.value)
    if (completion.kind === 'unexpected-event') {
      failAll({
        kind: 'worker-protocol-failed',
        detail: `The analysis worker returned ${completion.received} for a request expecting ${completion.expected}.`,
      })
    }
  }
  created.onerror = (event) => {
    event.preventDefault()
    failAll({
      kind: 'worker-unavailable',
      detail: event.message || 'The analysis worker stopped unexpectedly.',
    })
  }
  worker = created
  return created
}

export function runLpcmci(
  values: Float64Array,
  rows: number,
  columns: number,
  tauMax: number,
  pcAlpha: number,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<LpcmciOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('lpcmci-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'lpcmci', request, values, rows, columns, tauMax, pcAlpha,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runRpcmci(
  values: Float64Array,
  rows: number,
  columns: number,
  configuration: {
    readonly numRegimes: number
    readonly maxTransitions: number
    readonly switchThres: number
    readonly numIterations: number
    readonly maxAnneal: number
    readonly tauMin: number
    readonly tauMax: number
    readonly pcAlpha: number
    readonly alphaLevel: number
    readonly seed: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<RpcmciOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('rpcmci-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'rpcmci', request, values, rows, columns, ...configuration,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runDynotears(
  values: Float64Array,
  rows: number,
  columns: number,
  maxLag: number,
  lambdaW: number,
  lambdaA: number,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<DynotearsOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('dynotears-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'dynotears', request, values, rows, columns, maxLag, lambdaW, lambdaA,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runOcse(
  values: Float64Array,
  rows: number,
  columns: number,
  maxLag: number,
  alpha: number,
  nShuffles: number,
  method: 'gaussian' | 'knn',
  k: number,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<OcseOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('ocse-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'ocse', request, values, rows, columns, maxLag, alpha, nShuffles, method, k,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runCmlp(
  values: Float64Array,
  rows: number,
  columns: number,
  configuration: {
    readonly lag: number
    readonly hidden: readonly number[]
    readonly activation: 'sigmoid' | 'tanh' | 'relu' | 'leakyRelu' | 'identity'
    readonly penalty: 'groupLasso' | 'groupSparseGroupLasso' | 'hierarchical'
    readonly lambda: number
    readonly ridgeLambda: number
    readonly learningRate: number
    readonly maxIter: number
    readonly checkEvery: number
    readonly lookback: number
    readonly seed: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CmlpOutcome> {
  const request = newWorkerRequestId()
  return post('cmlp-succeeded', { kind: 'cmlp', request, values, rows, columns, ...configuration }, values, onProgress)
}

export function runClstm(
  values: Float64Array,
  rows: number,
  columns: number,
  configuration: {
    readonly context: number
    readonly hidden: number
    readonly lambda: number
    readonly ridgeLambda: number
    readonly learningRate: number
    readonly maxIter: number
    readonly checkEvery: number
    readonly lookback: number
    readonly seed: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<ClstmOutcome> {
  const request = newWorkerRequestId()
  return post('clstm-succeeded', { kind: 'clstm', request, values, rows, columns, ...configuration }, values, onProgress)
}

export function runStationarityBattery(values: Float64Array): Promise<StationarityOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('stationarity-succeeded', resolve))
    const command: AnalysisWorkerCommand = { kind: 'stationarity-battery', request, values }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: cause instanceof Error ? cause.message : String(cause),
      }))
    }
  })
}

export function runPandasResampling(
  timestamps: Float64Array,
  values: Float64Array,
  rows: number,
  columns: number,
  target: 'weekly' | 'monthly',
  incompleteBins: 'keep' | 'drop',
  aggregations: readonly ResamplingAggregation[],
  imputedCells: readonly (readonly [number, number])[],
): Promise<PandasResamplingOutcome> {
  const request = newWorkerRequestId()
  const payload = new Float64Array(timestamps.length + values.length)
  payload.set(timestamps)
  payload.set(values, timestamps.length)
  return post(
    'pandas-resampling-succeeded',
    { kind: 'pandas-resample-daily', request, values: payload, rows, columns, target, incompleteBins, aggregations, imputedCells },
    payload,
  )
}

export function runPcmciPlus(
  values: Float64Array,
  rows: number,
  columns: number,
  tauMax: number,
  pcAlpha: number,
): Promise<PcmciPlusOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('pcmci-plus-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'pcmci-plus',
      request,
      values,
      rows,
      columns,
      tauMax,
      pcAlpha,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: cause instanceof Error ? cause.message : String(cause),
      }))
    }
  })
}

export function runGrangerSsrF(
  values: Float64Array,
  rows: number,
  maxLag: number,
): Promise<GrangerOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('granger-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'granger-ssr-f',
      request,
      values,
      rows,
      maxLag,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: cause instanceof Error ? cause.message : String(cause),
      }))
    }
  })
}

export function runDirectLingam(
  values: Float64Array,
  rows: number,
  columns: number,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<DirectLingamOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('direct-lingam-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'direct-lingam', request, values, rows, columns,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runVarLingam(
  values: Float64Array,
  rows: number,
  columns: number,
  lags: number,
  prune: boolean,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<VarLingamOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('var-lingam-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'var-lingam', request, values, rows, columns, lags, prune,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function identifyBackdoor(graph: {
  readonly nodes: number
  readonly names: readonly string[]
  readonly edges: readonly (readonly [number, number])[]
  readonly treatment: number
  readonly outcome: number
  readonly unobserved: readonly number[]
  readonly estimand: 'ate' | 'att'
}): Promise<BackdoorIdentificationOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('backdoor-identification-succeeded', resolve))
    const command: AnalysisWorkerCommand = { kind: 'backdoor-identify', request, values: new Float64Array(0), ...graph }
    try {
      analysisWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runBackdoorLinear(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly hacMaxLags: number | null
    readonly level: number
  },
): Promise<BackdoorLinearOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('backdoor-linear-succeeded', resolve))
    const command: AnalysisWorkerCommand = { kind: 'backdoor-linear', request, values, rows, columns, ...design }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runFrontdoorTwoStage(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly mediator: number
    readonly outcome: number
    readonly firstStageAdjustment: readonly number[]
    readonly secondStageAdjustment: readonly number[]
    readonly controlValue: number
    readonly treatmentValue: number
    readonly uncertainty: {
      readonly kind: 'bootstrap'
      readonly simulations: number
      readonly sampleSizeFraction: number
      readonly confidenceLevel: number
      readonly seed: number
    }
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<FrontdoorTwoStageOutcome> {
  const request = newWorkerRequestId()
  return post(
    'frontdoor-two-stage-succeeded',
    { kind: 'frontdoor-two-stage', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

const post = <Kind extends SuccessfulAnalysisEventKind>(
  expected: Kind,
  command: AnalysisWorkerCommand,
  values: Float64Array,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<SuccessfulAnalysisResult<Kind>, AnalysisWorkerProblem>> =>
  new Promise((resolve) => {
    pending.set(command.request, pendingRun(expected, resolve, onProgress))
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(command.request)
      resolve(err({
        kind: 'worker-unavailable',
        detail: `${command.kind}: ${cause instanceof Error ? cause.message : String(cause)}`,
      }))
    }
  })

export function runCountGlm(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly family: 'poisson' | 'negativeBinomial' }): Promise<CountGlmOutcome> {
  const request = newWorkerRequestId()
  return post('count-glm-succeeded', { kind: 'count-glm', request, values, rows, columns, ...design }, values)
}

export function runNegativeBinomialIngarch(values: Float64Array, rows: number, columns: number, design: {
  readonly outcome: number
  readonly link: 'identity' | 'log'
  readonly regressors: readonly number[]
  readonly pastObservationLags: readonly number[]
  readonly pastMeanLags: readonly number[]
  readonly externalRegressors: readonly boolean[]
  readonly horizon: number
  readonly baselineRegressors: readonly number[]
  readonly interventionRegressor: number
  readonly controlValue: number
  readonly treatmentValue: number
  readonly schedule: IngarchInterventionSchedule
}, onProgress?: (progress: AnalysisProgress) => void): Promise<NegativeBinomialIngarchOutcome> {
  const request = newWorkerRequestId()
  return post('negative-binomial-ingarch-succeeded', { kind: 'negative-binomial-ingarch', request, values, rows, columns, ...design }, values, onProgress)
}

export function runCountSeriesInterventionScan(values: Float64Array, rows: number, columns: number, design: {
  readonly outcome: number
  readonly link: 'identity' | 'log'
  readonly pastObservationLags: readonly number[]
  readonly pastMeanLags: readonly number[]
  readonly candidateReferencePoints: readonly number[]
  readonly delta: number
}, onProgress?: (progress: AnalysisProgress) => void): Promise<CountSeriesInterventionScanOutcome> {
  const request = newWorkerRequestId()
  return post('count-series-intervention-scan-succeeded', { kind: 'count-series-intervention-scan', request, values, rows, columns, ...design }, values, onProgress)
}

export function runCausalEffectsTotal(values: Float64Array, rows: number, columns: number, design: {
  readonly statLag: number
  readonly graph: readonly (readonly (readonly string[])[])[]
  readonly x: readonly (readonly [number, number])[]
  readonly y: readonly (readonly [number, number])[]
  readonly hidden: readonly (readonly [number, number])[]
  readonly estimator: TotalEffectEstimator
  readonly interventions: readonly [number, number]
  readonly uncertainty: CausalEffectsUncertainty
}, onProgress?: (progress: AnalysisProgress) => void): Promise<CausalEffectsOutcome> {
  const request = newWorkerRequestId()
  return post('causal-effects-succeeded', { kind: 'causal-effects-total', request, values, rows, columns, ...design }, values, onProgress)
}

export function runCausalImpact(values: Float64Array, rows: number, columns: number, design: { readonly outcome: number; readonly controls: readonly number[]; readonly nPre: number; readonly maxIter: number }): Promise<CausalImpactOutcome> {
  const request = newWorkerRequestId()
  return post('causal-impact-succeeded', { kind: 'causal-impact', request, values, rows, columns, ...design }, values)
}

export function runLinearRefutation(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly simulations: number; readonly subsetFraction: number; readonly seed: number; readonly ljungBoxLags: number }): Promise<LinearRefutationOutcome> {
  const request = newWorkerRequestId()
  return post('linear-refutation-succeeded', { kind: 'linear-refutation', request, values, rows, columns, ...design }, values)
}

export function runUnobservedConfounding(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly seed: number; readonly kappaT: readonly number[] | null; readonly kappaY: readonly number[] | null }): Promise<UnobservedConfoundingOutcome> {
  const request = newWorkerRequestId()
  return post('unobserved-confounding-succeeded', { kind: 'unobserved-confounding', request, values, rows, columns, ...design }, values)
}

export function runSeriesStructure(values: Float64Array, rows: number, columns: number, design: { readonly period: number | null; readonly robust: boolean; readonly correlationMaxLag: number; readonly peltMinSize: number; readonly peltJump: number; readonly peltPenalty: number }): Promise<SeriesStructureOutcome> {
  const request = newWorkerRequestId()
  return post('series-structure-succeeded', { kind: 'series-structure', request, values, rows, columns, ...design }, values)
}

export interface DmlDesign {
  readonly treatment: number
  readonly outcome: number
  readonly adjustment: readonly number[]
  readonly model: 'plr' | 'irm'
  readonly att: boolean
  readonly seed: number
}

export function runDoubleMl(values: Float64Array, rows: number, columns: number, design: DmlDesign): Promise<DoubleMlOutcome> {
  const request = newWorkerRequestId()
  return post('double-ml-succeeded', { kind: 'double-ml', request, values, rows, columns, ...design }, values)
}

export function runDmlRefutationBatch(values: Float64Array, rows: number, columns: number, design: DmlDesign): Promise<DmlRefutationOutcome> {
  const request = newWorkerRequestId()
  return post('dml-refutation-succeeded', { kind: 'dml-refutation-batch', request, values, rows, columns, ...design }, values)
}

export function runArdlPss(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly maxLag: number; readonly trend: 'c' | 'ct'; readonly case: number }): Promise<ArdlOutcome> {
  const request = newWorkerRequestId()
  return post('ardl-succeeded', { kind: 'ardl-pss', request, values, rows, columns, ...design }, values)
}

export function runVecm(values: Float64Array, rows: number, columns: number, design: { readonly endogenous: readonly number[]; readonly maxLags: number; readonly deterministic: 'n' | 'co' | 'ci' | 'coli'; readonly significance: number; readonly breakIndex: number | null }): Promise<VecmOutcome> {
  const request = newWorkerRequestId()
  return post('vecm-succeeded', { kind: 'vecm', request, values, rows, columns, ...design }, values)
}

export function runSyntheticControl(values: Float64Array, rows: number, columns: number, design: { readonly treated: number; readonly donors: readonly number[]; readonly nPre: number; readonly crossFitFolds: number; readonly alpha: number }): Promise<SyntheticOutcome> {
  const request = newWorkerRequestId()
  return post('synthetic-control-succeeded', { kind: 'synthetic-control', request, values, rows, columns, ...design }, values)
}

export function runPanelIntervention(
  values: Float64Array,
  rows: number,
  units: readonly string[],
  times: readonly number[],
  inference: { readonly placeboReplications: number; readonly seed: number },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<PanelInterventionOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('panel-intervention-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = { kind: 'panel-intervention', request, values, rows, units, times, ...inference }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runNegbinNuts(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly confounder: number; readonly warmup: number; readonly samples: number; readonly seed: number }): Promise<NegbinNutsOutcome> {
  const request = newWorkerRequestId()
  return post('negbin-nuts-succeeded', { kind: 'negbin-nuts', request, values, rows, columns, ...design }, values)
}

export function runBayesianGaussian(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly warmup: number; readonly samples: number; readonly seed: number }): Promise<BayesianGaussianOutcome> {
  const request = newWorkerRequestId()
  return post('bayesian-gaussian-succeeded', { kind: 'bayesian-gaussian', request, values, rows, columns, ...design }, values)
}

export function runDiscreteBnQuery(values: Float64Array, rows: number, columns: number, design: { readonly nodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly bins: number; readonly equivalentSampleSize: number }): Promise<DiscreteBnOutcome> {
  const request = newWorkerRequestId()
  return post('discrete-bn-succeeded', { kind: 'discrete-bn-query', request, values, rows, columns, ...design }, values)
}

export function runIdentifiedDiscreteQuery(values: Float64Array, rows: number, columns: number, design: { readonly observedNodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly unobserved: readonly number[]; readonly bins: number; readonly condition: { readonly variable: number; readonly state: number } | null }): Promise<IdentifiedDiscreteQueryOutcome> {
  const request = newWorkerRequestId()
  return post('identified-discrete-query-succeeded', { kind: 'identified-discrete-query', request, values, rows, columns, ...design }, values)
}

export function runBinaryEtt(values: Float64Array, rows: number, columns: number, design: { readonly observedNodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly unobserved: readonly number[] }): Promise<BinaryEttOutcome> {
  const request = newWorkerRequestId()
  return post('binary-ett-succeeded', { kind: 'binary-ett', request, values, rows, columns, ...design }, values)
}

export function runDagCheck(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly nodeColumns: readonly number[]
    readonly edges: readonly (readonly [number, number])[]
    readonly implications: readonly { readonly x: number; readonly y: number; readonly given: readonly number[] }[]
    readonly maximumObservations: number
    readonly permutations: number
    readonly significanceLevel: number
    readonly runFalsification: boolean
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<DagCheckOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('dag-check-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = { kind: 'dag-check', request, values, rows, columns, ...design }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(err({ kind: 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) }))
    }
  })
}

export function runLinearScmCounterfactual(values: Float64Array, rows: number, columns: number, design: { readonly nodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly interventions: readonly [number, number]; readonly observationNoise: number | null }): Promise<LinearScmOutcome> {
  const request = newWorkerRequestId()
  return post('linear-scm-succeeded', { kind: 'linear-scm-counterfactual', request, values, rows, columns, ...design }, values)
}

export function runDynamicLinearScmCounterfactual(values: Float64Array, rows: number, columns: number, design: { readonly nodes: readonly number[]; readonly statLag: number; readonly graph: readonly (readonly (readonly string[])[])[]; readonly treatment: number; readonly outcome: number; readonly timing: DynamicInterventionTiming; readonly steps: number; readonly interventions: readonly [number, number]; readonly uncertainty: DynamicCounterfactualUncertainty }, onProgress?: (progress: AnalysisProgress) => void): Promise<DynamicLinearScmOutcome> {
  const request = newWorkerRequestId()
  return post('dynamic-linear-scm-succeeded', { kind: 'dynamic-linear-scm-counterfactual', request, values, rows, columns, ...design }, values, onProgress)
}

export function seasonalAdjustInWorker(values: Float64Array, rows: number, columns: number, design: { readonly period: number; readonly robust: boolean; readonly adjust: readonly number[] }): Promise<SeasonalOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its matrix.
  const copy = Float64Array.from(values)
  return post('seasonal-adjusted', { kind: 'seasonal-adjust', request, values: copy, rows, columns, ...design }, copy)
}

export function resolveMissingnessInWorker(values: Float64Array, rows: number, columns: number, validity: Uint8Array, resolution: MissingnessResolutionCommand): Promise<MissingnessOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its nullable matrix.
  const copy = Float64Array.from(values)
  return post('missingness-resolved', { kind: 'resolve-missingness', request, values: copy, rows, columns, validity, resolution }, copy)
}
