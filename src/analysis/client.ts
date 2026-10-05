import { err, ok, type Result } from '@/domain/dop'
import { cancelPooledAnalyses } from './workerPool'
import type {
  CausalForestConfiguration,
  CausalForestEvidence,
  CausalForestTarget,
} from '@/domain/causalForest'

export function runCausalForest(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly target: CausalForestTarget
    readonly configuration: CausalForestConfiguration
    readonly columnNames?: readonly string[]
  },
): Promise<Result<CausalForestEvidence, AnalysisWorkerProblem>> {
  if (design.configuration.tuning.kind === 'all') {
    return import('./forestTuning').then(({ runForestTuning }) =>
      runForestTuning(values, {
        ...design,
        rows,
        columns,
        adjustment: [...design.adjustment],
        columnNames: [...(design.columnNames ?? [])],
      }),
    )
  }
  const request = newWorkerRequestId()
  return post(
    'causal-forest-succeeded',
    { kind: 'causal-forest', request, values, rows, columns, ...design },
    values,
  )
}
import type { AalenEvidence, ForestEvidence } from '@/domain/survivalRegression'
import type {
  AalenWorkerDesign,
  AdjustedRegressionErrorModel,
  AdjustedRegressionFixedEffects,
  DiscreteConditionState,
  ForestWorkerDesign,
} from '@/workers/analysisProtocol'

export function runAalen(
  values: Float64Array,
  rows: number,
  columns: number,
  design: AalenWorkerDesign,
): Promise<Result<AalenEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('aalen-succeeded', { kind: 'aalen', request, values, rows, columns, design }, values)
}

export function runSurvivalForest(
  values: Float64Array,
  rows: number,
  columns: number,
  design: ForestWorkerDesign,
): Promise<Result<ForestEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'survival-forest-succeeded',
    { kind: 'survival-forest', request, values, rows, columns, design },
    values,
  )
}
import type { CountSeriesInterventionScanEvidence } from '@/domain/countSeries'
import type {
  InterruptedImpact,
  InterruptedModel,
  InterruptedSeasonal,
  InterruptedSeriesEvidence,
  LinearErrorModel,
} from '@/domain/interruptedSeries'
import type { MulticollinearityEvidence } from '@/domain/multicollinearity'
import { GrangerSsrEvidence } from '@/domain/granger'
import type {
  DynotearsEvidence,
  DirectLingamEvidence,
  FciEvidence,
  PcStableEvidence,
  ConstraintBackgroundKnowledge,
  ConstraintCiTest,
  LpcmciEvidence,
  RpcmciEvidence,
  OcseEvidence,
  CmlpEvidence,
  ClstmEvidence,
  CdnotsEvidence,
  CdnotsPlusEvidence,
  GraceEvidence,
  JpcmciPlusEvidence,
  PcmciPlusEvidence,
  VarLingamEvidence,
} from '@/domain/discovery'
import type {
  BackdoorLinearEvidence,
  ContinuousGpsEvidence,
  DoublyRobustEvidence,
  PropensityMatchingEvidence,
  PropensityWeightingEvidence,
  CausalEffectsEvidence,
  CausalEffectsUncertainty,
  CausalImpactEvidence,
  CountGlmEvidence,
  FrontdoorTwoStageEvidence,
  IngarchInterventionSchedule,
  InstrumentalVariableEvidence,
  NegativeBinomialIngarchEvidence,
  TotalEffectEstimator,
} from '@/domain/estimation'
import type {
  MissingnessResolutionCommand,
  MissingnessResolvedEvidence,
} from '@/domain/missingness'
import type { SeasonalAdjustedEvidence } from '@/domain/seasonal'
import type {
  ArdlEvidence,
  ArmSelection,
  BayesianGaussianEvidence,
  BinaryEttEvidence,
  DiscreteBnEvidence,
  DoubleMlEvidence,
  NegbinNutsEvidence,
  PanelInterventionEvidence,
  SyntheticControlEvidence,
  TLearnerEvidence,
  CrossFittedTLearnerEvidence,
  VecmEvidence,
} from '@/domain/estimation'
import type { DmlRefutationEvidence } from '@/domain/sensitivity'
import type {
  DynamicCounterfactualUncertainty,
  DynamicInterventionTiming,
  DynamicLinearScmEvidence,
  LinearScmEvidence,
} from '@/domain/counterfactual'
import type {
  LinearRefutationEvidence,
  SeriesStructureEvidence,
  UnobservedConfoundingEvidence,
} from '@/domain/sensitivity'
import type { StationarityBattery } from '@/domain/stationarity'
import type { PandasResamplingEvidence, ResamplingAggregation } from '@/domain/resampling'
import type { BackdoorIdentificationEvidence } from '@/domain/study'
import type { DagCheckEvidence } from '@/domain/dagValidation'
import type { IdentifiedDiscreteQueryEvidence } from '@/domain/intervention'
import type { NetworkQuery, NetworkQueryEvidence } from '@/domain/networkQuery'
import type {
  ConditionalGaussianQuery,
  ConditionalGaussianEvidence,
} from '@/domain/conditionalGaussianQuery'

export function runConditionalGaussianQuery(
  values: Float64Array,
  query: ConditionalGaussianQuery,
): Promise<Result<ConditionalGaussianEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'conditional-gaussian-query-succeeded',
    { kind: 'conditional-gaussian-query', request, values, query },
    values,
  )
}

export function runNetworkQuery(
  values: Float64Array,
  query: NetworkQuery,
): Promise<Result<NetworkQueryEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('network-query-succeeded', { kind: 'network-query', request, values, query }, values)
}
import type {
  ComparisonSurvivalEvidence,
  CoxRegressionEvidence,
  FlexSurvEvidence,
  MultiStateSurvivalEvidence,
  NonparametricSurvivalEvidence,
  ParametricSurvivalFamily,
  PenalizedAftEvidence,
  ProportionalHazardsFamily,
} from '@/domain/survival'
import {
  newWorkerRequestId,
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisProgress,
  type AnalysisWorkerEvent,
  type AnalysisWorkerProblem,
  type DmlGroupsRequest,
  type PropensityBootstrapRequest,
  type PropensityTreatmentModel,
  type PropensityWeightingFit,
  type LogisticTreatmentModel,
  type CoxRegressionWorkerDesign,
  type MultiStateWorkerInput,
  type TemporalSamples,
  type WorkerRequestId,
} from '@/workers/analysisProtocol'

type StationarityOutcome = Result<StationarityBattery, AnalysisWorkerProblem>
type MulticollinearityOutcome = Result<MulticollinearityEvidence, AnalysisWorkerProblem>
type PandasResamplingOutcome = Result<PandasResamplingEvidence, AnalysisWorkerProblem>
type PcmciPlusOutcome = Result<PcmciPlusEvidence, AnalysisWorkerProblem>
type JpcmciPlusOutcome = Result<JpcmciPlusEvidence, AnalysisWorkerProblem>
type GrangerOutcome = Result<GrangerSsrEvidence, AnalysisWorkerProblem>
type LpcmciOutcome = Result<LpcmciEvidence, AnalysisWorkerProblem>
type RpcmciOutcome = Result<RpcmciEvidence, AnalysisWorkerProblem>
type DynotearsOutcome = Result<DynotearsEvidence, AnalysisWorkerProblem>
type DirectLingamOutcome = Result<DirectLingamEvidence, AnalysisWorkerProblem>
type PcStableOutcome = Result<PcStableEvidence, AnalysisWorkerProblem>
type FciOutcome = Result<FciEvidence, AnalysisWorkerProblem>
type VarLingamOutcome = Result<VarLingamEvidence, AnalysisWorkerProblem>
type OcseOutcome = Result<OcseEvidence, AnalysisWorkerProblem>
type CmlpOutcome = Result<CmlpEvidence, AnalysisWorkerProblem>
type ClstmOutcome = Result<ClstmEvidence, AnalysisWorkerProblem>
type CdnotsOutcome = Result<CdnotsEvidence, AnalysisWorkerProblem>
type CdnotsPlusOutcome = Result<CdnotsPlusEvidence, AnalysisWorkerProblem>
type GraceOutcome = Result<GraceEvidence, AnalysisWorkerProblem>
type BackdoorIdentificationOutcome = Result<BackdoorIdentificationEvidence, AnalysisWorkerProblem>
type DagCheckOutcome = Result<DagCheckEvidence, AnalysisWorkerProblem>
type BackdoorLinearOutcome = Result<BackdoorLinearEvidence, AnalysisWorkerProblem>
type FrontdoorTwoStageOutcome = Result<FrontdoorTwoStageEvidence, AnalysisWorkerProblem>
type InstrumentalVariableOutcome = Result<InstrumentalVariableEvidence, AnalysisWorkerProblem>
type CountGlmOutcome = Result<CountGlmEvidence, AnalysisWorkerProblem>
type NegativeBinomialIngarchOutcome = Result<NegativeBinomialIngarchEvidence, AnalysisWorkerProblem>
type CountSeriesInterventionScanOutcome = Result<
  CountSeriesInterventionScanEvidence,
  AnalysisWorkerProblem
>
type InterruptedSeriesOutcome = Result<InterruptedSeriesEvidence, AnalysisWorkerProblem>
type CausalEffectsOutcome = Result<CausalEffectsEvidence, AnalysisWorkerProblem>
type CausalImpactOutcome = Result<CausalImpactEvidence, AnalysisWorkerProblem>
type LinearRefutationOutcome = Result<LinearRefutationEvidence, AnalysisWorkerProblem>
type UnobservedConfoundingOutcome = Result<UnobservedConfoundingEvidence, AnalysisWorkerProblem>
type SeriesStructureOutcome = Result<SeriesStructureEvidence, AnalysisWorkerProblem>
type MissingnessOutcome = Result<MissingnessResolvedEvidence, AnalysisWorkerProblem>
type SeasonalOutcome = Result<SeasonalAdjustedEvidence, AnalysisWorkerProblem>
type DoubleMlOutcome = Result<DoubleMlEvidence, AnalysisWorkerProblem>
type TLearnerOutcome = Result<TLearnerEvidence, AnalysisWorkerProblem>
type CrossFittedTLearnerOutcome = Result<CrossFittedTLearnerEvidence, AnalysisWorkerProblem>
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
type FlexSurvOutcome = Result<FlexSurvEvidence, AnalysisWorkerProblem>
type CoxRegressionOutcome = Result<CoxRegressionEvidence, AnalysisWorkerProblem>
type PenalizedAftOutcome = Result<PenalizedAftEvidence, AnalysisWorkerProblem>
type NonparametricSurvivalOutcome = Result<NonparametricSurvivalEvidence, AnalysisWorkerProblem>
type ComparisonSurvivalOutcome = Result<ComparisonSurvivalEvidence, AnalysisWorkerProblem>
type MultiStateSurvivalOutcome = Result<MultiStateSurvivalEvidence, AnalysisWorkerProblem>
type SuccessfulAnalysisEvent = Extract<AnalysisWorkerEvent, { readonly result: unknown }>
type SuccessfulAnalysisEventKind = SuccessfulAnalysisEvent['kind']
type SuccessfulAnalysisResults = {
  readonly [Event in SuccessfulAnalysisEvent as Event['kind']]: Event['result']
}
type SuccessfulAnalysisResult<Kind extends SuccessfulAnalysisEventKind> =
  SuccessfulAnalysisResults[Kind]

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
  cancelPooledAnalyses()
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
  samples: TemporalSamples,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<LpcmciOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('lpcmci-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'lpcmci',
      request,
      values,
      rows,
      columns,
      tauMax,
      pcAlpha,
      samples,
    }
    try {
      const transfer =
        samples.kind === 'role-aware'
          ? [values.buffer, samples.validity.buffer, samples.analysisMask.buffer]
          : [values.buffer]
      analysisWorker().postMessage(command, transfer)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
      kind: 'rpcmci',
      request,
      values,
      rows,
      columns,
      ...configuration,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

type CdnotsRunConfiguration = {
  readonly maxLag: number
  readonly alpha: number
  readonly missing: 'pairwiseComplete' | 'varEm'
  readonly context:
    | 'none'
    | 'linear'
    | 'linearSine'
    | 'linearExponential'
    | 'linearQuadratic'
    | 'step'
    | 'stepLinear'
}

export function runCdnots(
  values: Float64Array,
  validity: Uint8Array,
  rows: number,
  columns: number,
  configuration: CdnotsRunConfiguration,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CdnotsOutcome> {
  const request = newWorkerRequestId()
  return postNullable(
    'cdnots-succeeded',
    { kind: 'cdnots', request, values, validity, rows, columns, ...configuration },
    onProgress,
  )
}

export function runCdnotsPlus(
  values: Float64Array,
  validity: Uint8Array,
  rows: number,
  columns: number,
  configuration: CdnotsRunConfiguration,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CdnotsPlusOutcome> {
  const request = newWorkerRequestId()
  return postNullable(
    'cdnots-plus-succeeded',
    { kind: 'cdnots-plus', request, values, validity, rows, columns, ...configuration },
    onProgress,
  )
}

export function runGrace(
  values: Float64Array,
  validity: Uint8Array,
  rows: number,
  columns: number,
  configuration: {
    readonly maxLag: number
    readonly alpha: number
    readonly context: CdnotsRunConfiguration['context']
    readonly gateThreshold: number
    readonly epochs: number
    readonly patience: number
    readonly seed: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<GraceOutcome> {
  const request = newWorkerRequestId()
  return postNullable(
    'grace-succeeded',
    { kind: 'grace', request, values, validity, rows, columns, ...configuration },
    onProgress,
  )
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
      kind: 'dynotears',
      request,
      values,
      rows,
      columns,
      maxLag,
      lambdaW,
      lambdaA,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
      kind: 'ocse',
      request,
      values,
      rows,
      columns,
      maxLag,
      alpha,
      nShuffles,
      method,
      k,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
  return post(
    'cmlp-succeeded',
    { kind: 'cmlp', request, values, rows, columns, ...configuration },
    values,
    onProgress,
  )
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
  return post(
    'clstm-succeeded',
    { kind: 'clstm', request, values, rows, columns, ...configuration },
    values,
    onProgress,
  )
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
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runMulticollinearity(
  values: Float64Array,
  rows: number,
  columns: number,
  thresholds: { readonly correlation: number; readonly vif: number },
): Promise<MulticollinearityOutcome> {
  const request = newWorkerRequestId()
  return post(
    'multicollinearity-succeeded',
    {
      kind: 'multicollinearity',
      request,
      values,
      rows,
      columns,
      correlationThreshold: thresholds.correlation,
      vifThreshold: thresholds.vif,
    },
    values,
  )
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
    {
      kind: 'pandas-resample-daily',
      request,
      values: payload,
      rows,
      columns,
      target,
      incompleteBins,
      aggregations,
      imputedCells,
    },
    payload,
  )
}

export function runPcmciPlus(
  values: Float64Array,
  rows: number,
  columns: number,
  tauMax: number,
  pcAlpha: number,
  samples: TemporalSamples,
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
      samples,
    }
    try {
      const transfer =
        samples.kind === 'role-aware'
          ? [values.buffer, samples.validity.buffer, samples.analysisMask.buffer]
          : [values.buffer]
      analysisWorker().postMessage(command, transfer)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runJpcmciPlus(
  values: Float64Array,
  rows: number,
  datasets: number,
  periods: number,
  classes: readonly ('system' | 'timeContext' | 'spaceContext')[],
  configuration: {
    readonly timeDummy: boolean
    readonly spaceDummy: boolean
    readonly tauMax: number
    readonly pcAlpha: number
  },
): Promise<JpcmciPlusOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('jpcmci-plus-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'jpcmci-plus',
      request,
      values,
      rows,
      datasets,
      periods,
      observedColumns: classes.length,
      classes,
      // Named, not spread: the ready specification carries the role assignments as well, and the command schema is strict.
      timeDummy: configuration.timeDummy,
      spaceDummy: configuration.spaceDummy,
      tauMax: configuration.tauMax,
      pcAlpha: configuration.pcAlpha,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
      kind: 'direct-lingam',
      request,
      values,
      rows,
      columns,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

interface ConstraintDiscoveryRequest {
  readonly names: readonly string[]
  readonly alpha: number
  readonly maxDepth: number | null
  readonly ciTest: ConstraintCiTest
  readonly background: ConstraintBackgroundKnowledge
}

export function runPcStable(
  values: Float64Array,
  rows: number,
  columns: number,
  configuration: ConstraintDiscoveryRequest,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<PcStableOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('pc-stable-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'pc-stable',
      request,
      values,
      rows,
      columns,
      ...configuration,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runFci(
  values: Float64Array,
  rows: number,
  columns: number,
  configuration: ConstraintDiscoveryRequest & { readonly maxPathLength: number | null },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<FciOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('fci-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'fci',
      request,
      values,
      rows,
      columns,
      ...configuration,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
      kind: 'var-lingam',
      request,
      values,
      rows,
      columns,
      lags,
      prune,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
    const command: AnalysisWorkerCommand = {
      kind: 'backdoor-identify',
      request,
      values: new Float64Array(0),
      ...graph,
    }
    try {
      analysisWorker().postMessage(command)
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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
    readonly errorModel: AdjustedRegressionErrorModel
    readonly fixedEffects: AdjustedRegressionFixedEffects | null
  },
): Promise<BackdoorLinearOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('backdoor-linear-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'backdoor-linear',
      request,
      values,
      rows,
      columns,
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runPropensityWeighting(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly target?: 'ate' | 'att'
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly scale: 'inverseProbability' | 'stabilized'
    readonly fit: PropensityWeightingFit
  },
): Promise<Result<PropensityWeightingEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('propensity-weighting-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'propensity-weighting',
      request,
      values,
      rows,
      columns,
      target: design.target ?? 'ate',
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runPropensityMatching(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly target?: 'ate' | 'att'
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly model: PropensityTreatmentModel
  },
): Promise<Result<PropensityMatchingEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('propensity-matching-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'propensity-matching',
      request,
      values,
      rows,
      columns,
      target: design.target ?? 'ate',
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runDoublyRobust(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly target?: 'ate' | 'att'
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly model: LogisticTreatmentModel
    readonly bootstrap: PropensityBootstrapRequest | null
  },
): Promise<Result<DoublyRobustEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('doubly-robust-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'doubly-robust',
      request,
      values,
      rows,
      columns,
      target: design.target ?? 'ate',
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runContinuousGps(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly scale: 'inverseDensity' | 'stabilized'
    readonly bootstrap: PropensityBootstrapRequest | null
  },
): Promise<Result<ContinuousGpsEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('continuous-gps-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'continuous-gps',
      request,
      values,
      rows,
      columns,
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
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

export function runInstrumentalVariable(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly instruments: readonly number[]
    readonly uncertainty: {
      readonly kind: 'bootstrap'
      readonly simulations: number
      readonly sampleSizeFraction: number
      readonly confidenceLevel: number
      readonly seed: number
    }
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<InstrumentalVariableOutcome> {
  const request = newWorkerRequestId()
  return post(
    'instrumental-variable-succeeded',
    { kind: 'instrumental-variable', request, values, rows, columns, ...design },
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
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: `${command.kind}: ${cause instanceof Error ? cause.message : String(cause)}`,
        }),
      )
    }
  })

const postNullable = <Kind extends SuccessfulAnalysisEventKind>(
  expected: Kind,
  command: Extract<AnalysisWorkerCommand, { readonly validity: Uint8Array }>,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<SuccessfulAnalysisResult<Kind>, AnalysisWorkerProblem>> =>
  new Promise((resolve) => {
    pending.set(command.request, pendingRun(expected, resolve, onProgress))
    try {
      analysisWorker().postMessage(command, [command.values.buffer, command.validity.buffer])
    } catch (cause: unknown) {
      pending.delete(command.request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: `${command.kind}: ${cause instanceof Error ? cause.message : String(cause)}`,
        }),
      )
    }
  })

export function runFlexSurv(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly observation:
      | { readonly kind: 'rightCensored'; readonly duration: number; readonly event: number }
      | {
          readonly kind: 'startStop'
          readonly start: number
          readonly stop: number
          readonly event: number
        }
    readonly rowFrequency:
      | { readonly kind: 'oneObservationPerRow' }
      | { readonly kind: 'frequencyColumn'; readonly column: number }
    readonly covariates: readonly number[]
    readonly family: ParametricSurvivalFamily
    readonly predictionTimes: readonly number[]
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<FlexSurvOutcome> {
  const request = newWorkerRequestId()
  return post(
    'flexsurv-succeeded',
    { kind: 'flexsurv', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runCoxRegression(
  values: Float64Array,
  rows: number,
  columns: number,
  design: CoxRegressionWorkerDesign,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CoxRegressionOutcome> {
  const request = newWorkerRequestId()
  return post(
    'cox-regression-succeeded',
    { kind: 'cox-regression', request, values, rows, columns, design },
    values,
    onProgress,
  )
}

export function runPenalizedAft(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly duration: number
    readonly event: number
    readonly covariates: readonly number[]
    readonly family: 'weibull' | 'logLogistic'
    readonly penalizer: number
    readonly confidenceLevel: number
    readonly predictionTimes: readonly number[]
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<PenalizedAftOutcome> {
  const request = newWorkerRequestId()
  return post(
    'penalized-aft-succeeded',
    { kind: 'penalized-aft', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runComparisonSurvival(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly duration: number
    readonly event: number
    readonly group: number
    readonly truncationTime: number
    readonly permutations: number
    readonly seed: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<ComparisonSurvivalOutcome> {
  const request = newWorkerRequestId()
  return post(
    'comparison-survival-succeeded',
    { kind: 'comparison-survival', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runNonparametricSurvival(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly duration: number
    readonly event: number
    readonly rowFrequency:
      | { readonly kind: 'oneObservationPerRow' }
      | { readonly kind: 'frequencyColumn'; readonly column: number }
    readonly predictionTimes: readonly number[]
    readonly ties: 'discrete' | 'smoothed'
  },
): Promise<NonparametricSurvivalOutcome> {
  const request = newWorkerRequestId()
  return post(
    'nonparametric-survival-succeeded',
    { kind: 'nonparametric-survival', request, values, rows, columns, ...design },
    values,
  )
}

export function runMultiStateSurvival(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly input: MultiStateWorkerInput
    readonly family: ProportionalHazardsFamily
    readonly predictionTimes: readonly number[]
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<MultiStateSurvivalOutcome> {
  const request = newWorkerRequestId()
  return post(
    'multi-state-survival-succeeded',
    { kind: 'multi-state-survival', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runCountGlm(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly family: 'poisson' | 'negativeBinomial'
  },
): Promise<CountGlmOutcome> {
  const request = newWorkerRequestId()
  return post(
    'count-glm-succeeded',
    { kind: 'count-glm', request, values, rows, columns, ...design },
    values,
  )
}

export function runNegativeBinomialIngarch(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
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
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<NegativeBinomialIngarchOutcome> {
  const request = newWorkerRequestId()
  return post(
    'negative-binomial-ingarch-succeeded',
    { kind: 'negative-binomial-ingarch', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runCountSeriesInterventionScan(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly outcome: number
    readonly link: 'identity' | 'log'
    readonly pastObservationLags: readonly number[]
    readonly pastMeanLags: readonly number[]
    readonly candidateReferencePoints: readonly number[]
    readonly delta: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CountSeriesInterventionScanOutcome> {
  const request = newWorkerRequestId()
  return post(
    'count-series-intervention-scan-succeeded',
    { kind: 'count-series-intervention-scan', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runInterruptedSeries(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly outcome: number
    readonly model: InterruptedModel
    readonly interventionRow: number
    readonly lag: number
    readonly impact: InterruptedImpact
    readonly seasonal: InterruptedSeasonal
    readonly ljungBoxLags: number
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<InterruptedSeriesOutcome> {
  const request = newWorkerRequestId()
  return post(
    'interrupted-series-succeeded',
    { kind: 'interrupted-series', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runCausalEffectsTotal(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly statLag: number
    readonly graph: readonly (readonly (readonly string[])[])[]
    readonly x: readonly (readonly [number, number])[]
    readonly y: readonly (readonly [number, number])[]
    readonly hidden: readonly (readonly [number, number])[]
    readonly estimator: TotalEffectEstimator
    readonly interventions: readonly [number, number]
    readonly uncertainty: CausalEffectsUncertainty
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<CausalEffectsOutcome> {
  const request = newWorkerRequestId()
  return post(
    'causal-effects-succeeded',
    { kind: 'causal-effects-total', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function runSharpRd(
  values: Float64Array,
  rows: number,
  cutoff: number,
): Promise<Result<SharpRdEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('sharp-rd-succeeded', { kind: 'sharp-rd', request, values, rows, cutoff }, values)
}

export function runCausalImpact(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly outcome: number
    readonly controls: readonly number[]
    readonly nPre: number
    readonly postEnd: number
    readonly maxIter: number
  },
): Promise<CausalImpactOutcome> {
  const request = newWorkerRequestId()
  return post(
    'causal-impact-succeeded',
    { kind: 'causal-impact', request, values, rows, columns, ...design },
    values,
  )
}

export function runStructuralCausalImpact(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly outcome: number
    readonly controls: readonly number[]
    readonly nPre: number
    readonly postEnd: number
    readonly draws: number
    readonly warmup: number
    readonly seed: number
    readonly model: import('@/domain/structuralImpact').StructuralModel
  },
): Promise<CausalImpactOutcome> {
  const request = newWorkerRequestId()
  return post(
    'causal-impact-succeeded',
    { kind: 'structural-causal-impact', request, values, rows, columns, ...design },
    values,
  )
}

export function runBayesianCausalImpact(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly outcome: number
    readonly controls: readonly number[]
    readonly nPre: number
    readonly postEnd: number
    readonly draws: number
    readonly warmup: number
    readonly seed: number
    readonly priorLevelSd: number
  },
): Promise<CausalImpactOutcome> {
  const request = newWorkerRequestId()
  return post(
    'causal-impact-succeeded',
    { kind: 'bayesian-causal-impact', request, values, rows, columns, ...design },
    values,
  )
}

export function runLinearRefutation(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly simulations: number
    readonly subsetFraction: number
    readonly seed: number
    readonly ljungBoxLags: number
  },
): Promise<LinearRefutationOutcome> {
  const request = newWorkerRequestId()
  return post(
    'linear-refutation-succeeded',
    { kind: 'linear-refutation', request, values, rows, columns, ...design },
    values,
  )
}

export function runUnobservedConfounding(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly seed: number
    readonly kappaT: readonly number[] | null
    readonly kappaY: readonly number[] | null
  },
): Promise<UnobservedConfoundingOutcome> {
  const request = newWorkerRequestId()
  return post(
    'unobserved-confounding-succeeded',
    { kind: 'unobserved-confounding', request, values, rows, columns, ...design },
    values,
  )
}

export function runSeriesStructure(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly period: number | null
    readonly robust: boolean
    readonly correlationMaxLag: number
    readonly peltMinSize: number
    readonly peltJump: number
    readonly peltPenalty: number
  },
): Promise<SeriesStructureOutcome> {
  const request = newWorkerRequestId()
  return post(
    'series-structure-succeeded',
    { kind: 'series-structure', request, values, rows, columns, ...design },
    values,
  )
}

export interface DmlDesign {
  readonly treatment: number
  readonly outcome: number
  readonly adjustment: readonly number[]
  readonly model: 'plr' | 'irm'
  readonly att: boolean
  readonly seed: number
}

export function runDoubleMl(
  values: Float64Array,
  rows: number,
  columns: number,
  design: DmlDesign & { readonly groups: DmlGroupsRequest },
): Promise<DoubleMlOutcome> {
  const request = newWorkerRequestId()
  return post(
    'double-ml-succeeded',
    { kind: 'double-ml', request, values, rows, columns, ...design },
    values,
  )
}

export interface TLearnerDesign {
  readonly treatment: number
  readonly outcome: number
  readonly adjustment: readonly number[]
  readonly seed: number
  readonly uncertainty: import('@/domain/tLearner').TLearnerUncertainty
}

export function runTLearner(
  values: Float64Array,
  rows: number,
  columns: number,
  design: TLearnerDesign,
): Promise<TLearnerOutcome> {
  const request = newWorkerRequestId()
  return post(
    't-learner-succeeded',
    { kind: 't-learner', request, values, rows, columns, ...design },
    values,
  )
}

export interface CrossFittedTLearnerDesign {
  readonly treatment: number
  readonly outcome: number
  readonly adjustment: readonly number[]
  readonly learningRate: readonly number[]
  readonly maxDepth: readonly number[]
  readonly nEstimators: readonly number[]
  readonly splits: number
  readonly minSamplesLeaf: number
  readonly minSamplesSplit: number
  readonly seed: number
  readonly selection: ArmSelection
}

export function runCrossFittedTLearner(
  values: Float64Array,
  rows: number,
  columns: number,
  design: CrossFittedTLearnerDesign,
): Promise<CrossFittedTLearnerOutcome> {
  const request = newWorkerRequestId()
  return post(
    'cross-fitted-t-learner-succeeded',
    { kind: 'cross-fitted-t-learner', request, values, rows, columns, ...design },
    values,
  )
}

export function runDmlRefutationBatch(
  values: Float64Array,
  rows: number,
  columns: number,
  design: DmlDesign,
): Promise<DmlRefutationOutcome> {
  const request = newWorkerRequestId()
  return post(
    'dml-refutation-succeeded',
    { kind: 'dml-refutation-batch', request, values, rows, columns, ...design },
    values,
  )
}

export function runRootCause(
  values: Float64Array,
  model: import('@/domain/rootCauseAnalysis').RootCauseRequest,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<import('@/domain/rootCauseAnalysis').RootCauseEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'root-cause-succeeded',
    { kind: 'root-cause', request, values, model },
    values,
    onProgress,
  )
}

export function runGcmEffects(
  values: Float64Array,
  model: import('@/domain/gcmEffects').GcmEffectsRequest,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<import('@/domain/gcmEffects').GcmEffectsEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'gcm-effects-succeeded',
    { kind: 'gcm-effects', request, values, model },
    values,
    onProgress,
  )
}

export function runGcmInfluence(
  values: Float64Array,
  model: import('@/domain/gcmInfluence').GcmInfluenceRequest,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<import('@/domain/gcmInfluence').GcmInfluenceEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'gcm-influence-succeeded',
    { kind: 'gcm-influence', request, values, model },
    values,
    onProgress,
  )
}

export function checkRootCause(
  values: Float64Array,
  model: import('@/domain/rootCauseAnalysis').RootCauseCheckRequest,
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<Result<import('@/domain/rootCauseAnalysis').RootCauseChecks, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'root-cause-checks-succeeded',
    { kind: 'root-cause-checks', request, values, model },
    values,
    onProgress,
  )
}

export function runArdlModel(
  values: Float64Array,
  rows: number,
  columns: number,
  model: import('@/domain/ardlModel').ArdlModelRequest,
): Promise<Result<import('@/domain/ardlModel').ArdlModelEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post(
    'ardl-model-succeeded',
    { kind: 'ardl-model', request, values, rows, columns, model },
    values,
  )
}

export function runHonestDid(
  model: import('@/domain/honestDid').HonestRequest,
): Promise<Result<import('@/domain/honestDid').HonestEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId(),
    values = new Float64Array()
  return post('honest-did-succeeded', { kind: 'honest-did', request, values, model }, values)
}
export function runDidSensitivity(
  values: Float64Array,
  rows: number,
  columns: number,
  units: readonly string[],
  times: readonly number[],
  model: import('@/domain/didSensitivity').DidSensitivityRequest,
): Promise<
  Result<import('@/domain/didSensitivity').DidSensitivityEvidence, AnalysisWorkerProblem>
> {
  const request = newWorkerRequestId()
  return post(
    'did-sensitivity-succeeded',
    { kind: 'did-sensitivity', request, values, rows, columns, units, times, model },
    values,
  )
}
export function runArdlPss(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly maxLag: number
    readonly trend: 'c' | 'ct'
    readonly case: number
  },
): Promise<ArdlOutcome> {
  const request = newWorkerRequestId()
  return post(
    'ardl-succeeded',
    { kind: 'ardl-pss', request, values, rows, columns, ...design },
    values,
  )
}

export function runVecm(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly endogenous: readonly number[]
    readonly maxLags: number
    readonly deterministic: 'n' | 'co' | 'ci' | 'coli'
    readonly significance: number
    readonly breakIndex: number | null
    readonly forecastSteps?: number | null
  },
): Promise<VecmOutcome> {
  const request = newWorkerRequestId()
  return post('vecm-succeeded', { kind: 'vecm', request, values, rows, columns, ...design }, values)
}

export function runPredictorSyntheticControl(
  values: Float64Array,
  model: import('@/domain/predictorSyntheticControl').PredictorSyntheticRequest,
): Promise<
  Result<
    import('@/domain/predictorSyntheticControl').PredictorSyntheticEvidence,
    AnalysisWorkerProblem
  >
> {
  const request = newWorkerRequestId()
  return post(
    'predictor-synthetic-control-succeeded',
    { kind: 'predictor-synthetic-control', request, values, model },
    values,
  )
}

export function runSyntheticControl(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treated: number
    readonly donors: readonly number[]
    readonly nPre: number
    readonly crossFitFolds: number
    readonly alpha: number
  },
): Promise<SyntheticOutcome> {
  const request = newWorkerRequestId()
  return post(
    'synthetic-control-succeeded',
    { kind: 'synthetic-control', request, values, rows, columns, ...design },
    values,
  )
}

export function runCountRegression(
  values: Float64Array,
  model: import('@/domain/countRegression').CountRegressionRequest,
): Promise<
  Result<import('@/domain/countRegression').CountRegressionEvidence, AnalysisWorkerProblem>
> {
  const request = newWorkerRequestId()
  return post(
    'count-regression-succeeded',
    { kind: 'count-regression', request, values, model },
    values,
  )
}
export function runSunAbraham(
  values: Float64Array,
  model: import('@/domain/remixExtensions').SunAbrahamRequest,
): Promise<Result<import('@/domain/remixExtensions').SunAbrahamEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('sun-abraham-succeeded', { kind: 'sun-abraham', request, values, model }, values)
}
export function runRidgeAugmentedSynthetic(
  values: Float64Array,
  model: import('@/domain/remixExtensions').RidgeAugmentedRequest,
): Promise<
  Result<import('@/domain/remixExtensions').RidgeAugmentedEvidence, AnalysisWorkerProblem>
> {
  const request = newWorkerRequestId()
  return post(
    'ridge-augmented-synthetic-succeeded',
    { kind: 'ridge-augmented-synthetic', request, values, model },
    values,
  )
}
export function runPanelRegression(
  values: Float64Array,
  model: import('@/domain/panelRegression').PanelRegressionRequest,
): Promise<
  Result<import('@/domain/panelRegression').PanelRegressionEvidence, AnalysisWorkerProblem>
> {
  const request = newWorkerRequestId()
  return post(
    'panel-regression-succeeded',
    { kind: 'panel-regression', request, values, model },
    values,
  )
}
export function runBacon(
  values: Float64Array,
  model: import('@/domain/panelRegression').BaconRequest,
): Promise<Result<import('@/domain/panelRegression').BaconEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('bacon-succeeded', { kind: 'bacon', request, values, model }, values)
}
export function runStaggeredDid(
  values: Float64Array,
  model: import('@/domain/staggeredDid').StaggeredRequest,
): Promise<Result<import('@/domain/staggeredDid').StaggeredEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  return post('staggered-did-succeeded', { kind: 'staggered-did', request, values, model }, values)
}

export function runAdjustedDid(
  values: Float64Array,
  rows: number,
  columns: number,
  units: readonly string[],
  times: readonly number[],
  specification: AdjustedDidSpecification,
): Promise<PanelInterventionOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('panel-intervention-succeeded', resolve))
    const command: AnalysisWorkerCommand = {
      kind: 'panel-adjusted',
      request,
      values,
      rows,
      columns,
      units,
      times,
      specification,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runPanelIntervention(
  values: Float64Array,
  rows: number,
  units: readonly string[],
  times: readonly number[],
  inference: {
    readonly placeboReplications: number
    readonly seed: number
    readonly primary?: 'did' | 'syntheticDid'
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<PanelInterventionOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, pendingRun('panel-intervention-succeeded', resolve, onProgress))
    const command: AnalysisWorkerCommand = {
      kind: 'panel-intervention',
      request,
      values,
      rows,
      units,
      times,
      ...inference,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runNegbinNuts(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly confounder: number
    readonly warmup: number
    readonly samples: number
    readonly seed: number
  },
): Promise<NegbinNutsOutcome> {
  const request = newWorkerRequestId()
  return post(
    'negbin-nuts-succeeded',
    { kind: 'negbin-nuts', request, values, rows, columns, ...design },
    values,
  )
}

export function runBayesianGaussian(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
    readonly warmup: number
    readonly samples: number
    readonly seed: number
  },
): Promise<BayesianGaussianOutcome> {
  const request = newWorkerRequestId()
  return post(
    'bayesian-gaussian-succeeded',
    { kind: 'bayesian-gaussian', request, values, rows, columns, ...design },
    values,
  )
}

export function runDiscreteBnQuery(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly nodes: readonly number[]
    readonly names: readonly string[]
    readonly edges: readonly (readonly [number, number])[]
    readonly treatment: number
    readonly outcome: number
    readonly bins: number
    readonly equivalentSampleSize: number
  },
): Promise<DiscreteBnOutcome> {
  const request = newWorkerRequestId()
  return post(
    'discrete-bn-succeeded',
    { kind: 'discrete-bn-query', request, values, rows, columns, ...design },
    values,
  )
}

export function runIdentifiedDiscreteQuery(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly observedNodes: readonly number[]
    readonly names: readonly string[]
    readonly edges: readonly (readonly [number, number])[]
    readonly treatment: number
    readonly outcome: number
    readonly unobserved: readonly number[]
    readonly bins: number
    readonly condition: { readonly variable: number; readonly state: DiscreteConditionState } | null
  },
): Promise<IdentifiedDiscreteQueryOutcome> {
  const request = newWorkerRequestId()
  return post(
    'identified-discrete-query-succeeded',
    { kind: 'identified-discrete-query', request, values, rows, columns, ...design },
    values,
  )
}

export function runBinaryEtt(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly observedNodes: readonly number[]
    readonly names: readonly string[]
    readonly edges: readonly (readonly [number, number])[]
    readonly treatment: number
    readonly outcome: number
    readonly unobserved: readonly number[]
  },
): Promise<BinaryEttOutcome> {
  const request = newWorkerRequestId()
  return post(
    'binary-ett-succeeded',
    { kind: 'binary-ett', request, values, rows, columns, ...design },
    values,
  )
}

export function runDagCheck(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly nodeColumns: readonly number[]
    readonly edges: readonly (readonly [number, number])[]
    readonly implications: readonly {
      readonly x: number
      readonly y: number
      readonly given: readonly number[]
    }[]
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
    const command: AnalysisWorkerCommand = {
      kind: 'dag-check',
      request,
      values,
      rows,
      columns,
      ...design,
    }
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(request)
      resolve(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}

export function runLinearScmCounterfactual(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly nodes: readonly number[]
    readonly names: readonly string[]
    readonly edges: readonly (readonly [number, number])[]
    readonly treatment: number
    readonly outcome: number
    readonly interventions: readonly [number, number]
    readonly observationNoise: number | null
  },
): Promise<LinearScmOutcome> {
  const request = newWorkerRequestId()
  return post(
    'linear-scm-succeeded',
    { kind: 'linear-scm-counterfactual', request, values, rows, columns, ...design },
    values,
  )
}

export function runDynamicLinearScmCounterfactual(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly nodes: readonly number[]
    readonly statLag: number
    readonly graph: readonly (readonly (readonly string[])[])[]
    readonly treatment: number
    readonly outcome: number
    readonly timing: DynamicInterventionTiming
    readonly steps: number
    readonly interventions: readonly [number, number]
    readonly uncertainty: DynamicCounterfactualUncertainty
  },
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<DynamicLinearScmOutcome> {
  const request = newWorkerRequestId()
  return post(
    'dynamic-linear-scm-succeeded',
    { kind: 'dynamic-linear-scm-counterfactual', request, values, rows, columns, ...design },
    values,
    onProgress,
  )
}

export function seasonalAdjustInWorker(
  values: Float64Array,
  rows: number,
  columns: number,
  design: { readonly period: number; readonly robust: boolean; readonly adjust: readonly number[] },
): Promise<SeasonalOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its matrix.
  const copy = Float64Array.from(values)
  return post(
    'seasonal-adjusted',
    { kind: 'seasonal-adjust', request, values: copy, rows, columns, ...design },
    copy,
  )
}

export function resolveMissingnessInWorker(
  values: Float64Array,
  rows: number,
  columns: number,
  validity: Uint8Array,
  resolution: MissingnessResolutionCommand,
): Promise<MissingnessOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its nullable matrix.
  const copy = Float64Array.from(values)
  return post(
    'missingness-resolved',
    { kind: 'resolve-missingness', request, values: copy, rows, columns, validity, resolution },
    copy,
  )
}
import type { SharpRdEvidence } from '@/domain/sharpRd'
import type { AdjustedDidSpecification } from '@/domain/adjustedDid'

export function runSurrogate(
  model: import('@/domain/surrogate').SurrogateRequest,
): Promise<Result<import('@/domain/surrogate').SurrogateEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId()
  const values = new Float64Array(0)
  return post('surrogate-succeeded', { kind: 'surrogate', request, values, model }, values)
}

export function runSurrogateDiagnostics(
  model: import('@/domain/surrogateDiagnostics').SurrogateDiagnosticRequest,
): Promise<
  Result<import('@/domain/surrogateDiagnostics').SurrogateDiagnosticEvidence, AnalysisWorkerProblem>
> {
  const request = newWorkerRequestId(),
    values = new Float64Array(0)
  return post(
    'surrogate-diagnostics-succeeded',
    { kind: 'surrogate-diagnostics', request, values, model },
    values,
  )
}

export function runSurrogatePath(
  model: import('@/domain/surrogatePath').SurrogatePathRequest,
): Promise<Result<import('@/domain/surrogatePath').SurrogatePathEvidence, AnalysisWorkerProblem>> {
  const request = newWorkerRequestId(),
    values = new Float64Array(0)
  return post(
    'surrogate-path-succeeded',
    { kind: 'surrogate-path', request, values, model },
    values,
  )
}
