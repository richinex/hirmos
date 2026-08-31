import { err, type Result } from '@/domain/dop'
import { GrangerSsrEvidence } from '@/domain/granger'
import type {
  DynotearsEvidence,
  DirectLingamEvidence,
  LpcmciEvidence,
  OcseEvidence,
  PcmciPlusEvidence,
  VarLingamEvidence,
} from '@/domain/discovery'
import type { BackdoorLinearEvidence, CausalEffectsEvidence, CausalImpactEvidence, CountGlmEvidence, FrontdoorTwoStageEvidence } from '@/domain/estimation'
import type { MissingnessResolutionCommand, MissingnessResolvedEvidence } from '@/domain/missingness'
import type { SeasonalAdjustedEvidence } from '@/domain/seasonal'
import type { ArdlEvidence, BayesianGaussianEvidence, BinaryEttEvidence, DiscreteBnEvidence, DoubleMlEvidence, NegbinNutsEvidence, PanelInterventionEvidence, SyntheticControlEvidence, VecmEvidence } from '@/domain/estimation'
import type { DmlRefutationEvidence } from '@/domain/sensitivity'
import type { LinearScmEvidence } from '@/domain/counterfactual'
import type { LinearRefutationEvidence, SeriesStructureEvidence, UnobservedConfoundingEvidence } from '@/domain/sensitivity'
import type { StationarityBattery } from '@/domain/stationarity'
import type { BackdoorIdentificationEvidence } from '@/domain/study'
import type { DagCheckEvidence } from '@/domain/dagValidation'
import {
  newWorkerRequestId,
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisProgress,
  type AnalysisWorkerProblem,
  type WorkerRequestId,
} from '@/workers/analysisProtocol'

type StationarityOutcome = Result<StationarityBattery, AnalysisWorkerProblem>
type PcmciPlusOutcome = Result<PcmciPlusEvidence, AnalysisWorkerProblem>
type GrangerOutcome = Result<GrangerSsrEvidence, AnalysisWorkerProblem>
type LpcmciOutcome = Result<LpcmciEvidence, AnalysisWorkerProblem>
type DynotearsOutcome = Result<DynotearsEvidence, AnalysisWorkerProblem>
type DirectLingamOutcome = Result<DirectLingamEvidence, AnalysisWorkerProblem>
type VarLingamOutcome = Result<VarLingamEvidence, AnalysisWorkerProblem>
type OcseOutcome = Result<OcseEvidence, AnalysisWorkerProblem>
type BackdoorIdentificationOutcome = Result<BackdoorIdentificationEvidence, AnalysisWorkerProblem>
type DagCheckOutcome = Result<DagCheckEvidence, AnalysisWorkerProblem>
type BackdoorLinearOutcome = Result<BackdoorLinearEvidence, AnalysisWorkerProblem>
type FrontdoorTwoStageOutcome = Result<FrontdoorTwoStageEvidence, AnalysisWorkerProblem>
type CountGlmOutcome = Result<CountGlmEvidence, AnalysisWorkerProblem>
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
type BinaryEttOutcome = Result<BinaryEttEvidence, AnalysisWorkerProblem>
type LinearScmOutcome = Result<LinearScmEvidence, AnalysisWorkerProblem>
type PendingRun =
  | { readonly kind: 'stationarity'; readonly resolve: (outcome: StationarityOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'pcmci-plus'; readonly resolve: (outcome: PcmciPlusOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'lpcmci'; readonly resolve: (outcome: LpcmciOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'dynotears'; readonly resolve: (outcome: DynotearsOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'direct-lingam'; readonly resolve: (outcome: DirectLingamOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'var-lingam'; readonly resolve: (outcome: VarLingamOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'ocse'; readonly resolve: (outcome: OcseOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'granger'; readonly resolve: (outcome: GrangerOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'backdoor-identify'; readonly resolve: (outcome: BackdoorIdentificationOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'dag-check'; readonly resolve: (outcome: DagCheckOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'backdoor-linear'; readonly resolve: (outcome: BackdoorLinearOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'frontdoor-two-stage'; readonly resolve: (outcome: FrontdoorTwoStageOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'count-glm'; readonly resolve: (outcome: CountGlmOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'causal-effects-total'; readonly resolve: (outcome: CausalEffectsOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'causal-impact'; readonly resolve: (outcome: CausalImpactOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'linear-refutation'; readonly resolve: (outcome: LinearRefutationOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'unobserved-confounding'; readonly resolve: (outcome: UnobservedConfoundingOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'series-structure'; readonly resolve: (outcome: SeriesStructureOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'resolve-missingness'; readonly resolve: (outcome: MissingnessOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'seasonal-adjust'; readonly resolve: (outcome: SeasonalOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'double-ml'; readonly resolve: (outcome: DoubleMlOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'dml-refutation-batch'; readonly resolve: (outcome: DmlRefutationOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'ardl-pss'; readonly resolve: (outcome: ArdlOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'vecm'; readonly resolve: (outcome: VecmOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'synthetic-control'; readonly resolve: (outcome: SyntheticOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'panel-intervention'; readonly resolve: (outcome: PanelInterventionOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'negbin-nuts'; readonly resolve: (outcome: NegbinNutsOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'bayesian-gaussian'; readonly resolve: (outcome: BayesianGaussianOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'discrete-bn-query'; readonly resolve: (outcome: DiscreteBnOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'binary-ett'; readonly resolve: (outcome: BinaryEttOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'linear-scm-counterfactual'; readonly resolve: (outcome: LinearScmOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }

let worker: Worker | null = null
const pending = new Map<WorkerRequestId, PendingRun>()

/** Reject one run; the union of resolvers is too wide for the checker to fold, and a failure fits every outcome. */
const reject = (run: PendingRun, problem: AnalysisWorkerProblem): void => (run.resolve as (outcome: Result<never, AnalysisWorkerProblem>) => void)(err(problem))

const failAll = (problem: AnalysisWorkerProblem) => {
  for (const run of pending.values()) reject(run, problem)
  pending.clear()
  worker?.terminate()
  worker = null
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
      reject(run, parsed.value.problem)
      return
    }
    if (run.kind === 'stationarity' && parsed.value.kind !== 'stationarity-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned PCMCI+ evidence for a stationarity request.' })
      return
    }
    if (run.kind === 'pcmci-plus' && parsed.value.kind !== 'pcmci-plus-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a PCMCI+ request.' })
      return
    }
    if (run.kind === 'lpcmci' && parsed.value.kind !== 'lpcmci-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an LPCMCI request.' })
      return
    }
    if (run.kind === 'dynotears' && parsed.value.kind !== 'dynotears-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a DYNOTEARS request.' })
      return
    }
    if (run.kind === 'direct-lingam' && parsed.value.kind !== 'direct-lingam-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a DirectLiNGAM request.' })
      return
    }
    if (run.kind === 'var-lingam' && parsed.value.kind !== 'var-lingam-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a VAR-LiNGAM request.' })
      return
    }
    if (run.kind === 'ocse' && parsed.value.kind !== 'ocse-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an oCSE request.' })
      return
    }
    if (run.kind === 'granger' && parsed.value.kind !== 'granger-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a Granger request.' })
      return
    }
    if (run.kind === 'backdoor-identify' && parsed.value.kind !== 'backdoor-identification-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an identification request.' })
      return
    }
    if (run.kind === 'dag-check' && parsed.value.kind !== 'dag-check-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a DAG check request.' })
      return
    }
    if (run.kind === 'backdoor-linear' && parsed.value.kind !== 'backdoor-linear-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an adjusted regression request.' })
      return
    }
    if (run.kind === 'frontdoor-two-stage' && parsed.value.kind !== 'frontdoor-two-stage-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a front-door estimate.' })
      return
    }
    if (run.kind === 'count-glm' && parsed.value.kind !== 'count-glm-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a count model request.' })
      return
    }
    if (run.kind === 'causal-effects-total' && parsed.value.kind !== 'causal-effects-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a CausalEffects request.' })
      return
    }
    if (run.kind === 'causal-impact' && parsed.value.kind !== 'causal-impact-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a causal impact request.' })
      return
    }
    if (run.kind === 'linear-refutation' && parsed.value.kind !== 'linear-refutation-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a refutation request.' })
      return
    }
    if (run.kind === 'unobserved-confounding' && parsed.value.kind !== 'unobserved-confounding-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an unobserved confounding request.' })
      return
    }
    if (run.kind === 'series-structure' && parsed.value.kind !== 'series-structure-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a series structure request.' })
      return
    }
    if (run.kind === 'resolve-missingness' && parsed.value.kind !== 'missingness-resolved') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a missingness request.' })
      return
    }
    if (run.kind === 'seasonal-adjust' && parsed.value.kind !== 'seasonal-adjusted') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a seasonal adjustment request.' })
      return
    }
    if (run.kind === 'double-ml' && parsed.value.kind !== 'double-ml-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a double machine learning request.' })
      return
    }
    if (run.kind === 'dml-refutation-batch' && parsed.value.kind !== 'dml-refutation-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a DML refutation batch.' })
      return
    }
    if (run.kind === 'ardl-pss' && parsed.value.kind !== 'ardl-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for an ARDL request.' })
      return
    }
    if (run.kind === 'vecm' && parsed.value.kind !== 'vecm-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a VECM request.' })
      return
    }
    if (run.kind === 'synthetic-control' && parsed.value.kind !== 'synthetic-control-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a synthetic control request.' })
      return
    }
    if (run.kind === 'panel-intervention' && parsed.value.kind !== 'panel-intervention-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a panel intervention request.' })
      return
    }
    if (run.kind === 'negbin-nuts' && parsed.value.kind !== 'negbin-nuts-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a NUTS request.' })
      return
    }
    if (run.kind === 'bayesian-gaussian' && parsed.value.kind !== 'bayesian-gaussian-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a Bayesian Gaussian request.' })
      return
    }
    if (run.kind === 'discrete-bn-query' && parsed.value.kind !== 'discrete-bn-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a discrete BN query.' })
      return
    }
    if (run.kind === 'binary-ett' && parsed.value.kind !== 'binary-ett-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a binary ETT request.' })
      return
    }
    if (run.kind === 'linear-scm-counterfactual' && parsed.value.kind !== 'linear-scm-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another result for a counterfactual request.' })
      return
    }
    pending.delete(parsed.value.request)
    if (run.kind === 'stationarity' && parsed.value.kind === 'stationarity-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'pcmci-plus' && parsed.value.kind === 'pcmci-plus-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'lpcmci' && parsed.value.kind === 'lpcmci-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'dynotears' && parsed.value.kind === 'dynotears-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'direct-lingam' && parsed.value.kind === 'direct-lingam-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'var-lingam' && parsed.value.kind === 'var-lingam-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'ocse' && parsed.value.kind === 'ocse-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'granger' && parsed.value.kind === 'granger-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'backdoor-identify' && parsed.value.kind === 'backdoor-identification-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'dag-check' && parsed.value.kind === 'dag-check-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'backdoor-linear' && parsed.value.kind === 'backdoor-linear-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'frontdoor-two-stage' && parsed.value.kind === 'frontdoor-two-stage-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'count-glm' && parsed.value.kind === 'count-glm-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'causal-effects-total' && parsed.value.kind === 'causal-effects-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'causal-impact' && parsed.value.kind === 'causal-impact-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'linear-refutation' && parsed.value.kind === 'linear-refutation-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'unobserved-confounding' && parsed.value.kind === 'unobserved-confounding-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'series-structure' && parsed.value.kind === 'series-structure-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'resolve-missingness' && parsed.value.kind === 'missingness-resolved') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'seasonal-adjust' && parsed.value.kind === 'seasonal-adjusted') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'double-ml' && parsed.value.kind === 'double-ml-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'dml-refutation-batch' && parsed.value.kind === 'dml-refutation-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'ardl-pss' && parsed.value.kind === 'ardl-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'vecm' && parsed.value.kind === 'vecm-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'synthetic-control' && parsed.value.kind === 'synthetic-control-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'panel-intervention' && parsed.value.kind === 'panel-intervention-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'negbin-nuts' && parsed.value.kind === 'negbin-nuts-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'bayesian-gaussian' && parsed.value.kind === 'bayesian-gaussian-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'discrete-bn-query' && parsed.value.kind === 'discrete-bn-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'binary-ett' && parsed.value.kind === 'binary-ett-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'linear-scm-counterfactual' && parsed.value.kind === 'linear-scm-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
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
    pending.set(request, { kind: 'lpcmci', resolve, onProgress })
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
    pending.set(request, { kind: 'dynotears', resolve, onProgress })
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
    pending.set(request, { kind: 'ocse', resolve, onProgress })
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

export function runStationarityBattery(values: Float64Array): Promise<StationarityOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'stationarity', resolve })
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

export function runPcmciPlus(
  values: Float64Array,
  rows: number,
  columns: number,
  tauMax: number,
  pcAlpha: number,
): Promise<PcmciPlusOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'pcmci-plus', resolve })
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
    pending.set(request, { kind: 'granger', resolve })
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
    pending.set(request, { kind: 'direct-lingam', resolve, onProgress })
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
    pending.set(request, { kind: 'var-lingam', resolve, onProgress })
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
    pending.set(request, { kind: 'backdoor-identify', resolve })
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
    pending.set(request, { kind: 'backdoor-linear', resolve })
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
  return post<FrontdoorTwoStageOutcome>(
    'frontdoor-two-stage',
    { kind: 'frontdoor-two-stage', request, values, rows, columns, ...design },
    values,
    (resolve) => ({ kind: 'frontdoor-two-stage', resolve, onProgress }),
  )
}

const post = <Outcome>(kind: PendingRun['kind'], command: AnalysisWorkerCommand, values: Float64Array, register: (resolve: (outcome: Outcome) => void) => PendingRun): Promise<Outcome> =>
  new Promise((resolve) => {
    pending.set(command.request, register(resolve))
    try {
      analysisWorker().postMessage(command, [values.buffer])
    } catch (cause: unknown) {
      pending.delete(command.request)
      resolve({ ok: false, error: { kind: 'worker-unavailable', detail: `${kind}: ${cause instanceof Error ? cause.message : String(cause)}` } } as Outcome)
    }
  })

export function runCountGlm(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly family: 'poisson' | 'negativeBinomial' }): Promise<CountGlmOutcome> {
  const request = newWorkerRequestId()
  return post<CountGlmOutcome>('count-glm', { kind: 'count-glm', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'count-glm', resolve }))
}

export function runCausalEffectsTotal(values: Float64Array, rows: number, columns: number, design: {
  readonly statLag: number
  readonly graph: readonly (readonly (readonly string[])[])[]
  readonly x: readonly (readonly [number, number])[]
  readonly y: readonly (readonly [number, number])[]
  readonly hidden: readonly (readonly [number, number])[]
  readonly estimator: { readonly kind: 'linear' } | { readonly kind: 'knn'; readonly k: number }
  readonly interventions: readonly [number, number]
}): Promise<CausalEffectsOutcome> {
  const request = newWorkerRequestId()
  return post<CausalEffectsOutcome>('causal-effects-total', { kind: 'causal-effects-total', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'causal-effects-total', resolve }))
}

export function runCausalImpact(values: Float64Array, rows: number, columns: number, design: { readonly outcome: number; readonly controls: readonly number[]; readonly nPre: number; readonly maxIter: number }): Promise<CausalImpactOutcome> {
  const request = newWorkerRequestId()
  return post<CausalImpactOutcome>('causal-impact', { kind: 'causal-impact', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'causal-impact', resolve }))
}

export function runLinearRefutation(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly simulations: number; readonly subsetFraction: number; readonly seed: number; readonly ljungBoxLags: number }): Promise<LinearRefutationOutcome> {
  const request = newWorkerRequestId()
  return post<LinearRefutationOutcome>('linear-refutation', { kind: 'linear-refutation', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'linear-refutation', resolve }))
}

export function runUnobservedConfounding(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly seed: number; readonly kappaT: readonly number[] | null; readonly kappaY: readonly number[] | null }): Promise<UnobservedConfoundingOutcome> {
  const request = newWorkerRequestId()
  return post<UnobservedConfoundingOutcome>('unobserved-confounding', { kind: 'unobserved-confounding', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'unobserved-confounding', resolve }))
}

export function runSeriesStructure(values: Float64Array, rows: number, columns: number, design: { readonly period: number | null; readonly robust: boolean; readonly peltMinSize: number; readonly peltJump: number; readonly peltPenalty: number }): Promise<SeriesStructureOutcome> {
  const request = newWorkerRequestId()
  return post<SeriesStructureOutcome>('series-structure', { kind: 'series-structure', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'series-structure', resolve }))
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
  return post<DoubleMlOutcome>('double-ml', { kind: 'double-ml', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'double-ml', resolve }))
}

export function runDmlRefutationBatch(values: Float64Array, rows: number, columns: number, design: DmlDesign): Promise<DmlRefutationOutcome> {
  const request = newWorkerRequestId()
  return post<DmlRefutationOutcome>('dml-refutation-batch', { kind: 'dml-refutation-batch', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'dml-refutation-batch', resolve }))
}

export function runArdlPss(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly maxLag: number; readonly trend: 'c' | 'ct'; readonly case: number }): Promise<ArdlOutcome> {
  const request = newWorkerRequestId()
  return post<ArdlOutcome>('ardl-pss', { kind: 'ardl-pss', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'ardl-pss', resolve }))
}

export function runVecm(values: Float64Array, rows: number, columns: number, design: { readonly endogenous: readonly number[]; readonly maxLags: number; readonly deterministic: 'n' | 'co' | 'ci' | 'coli'; readonly significance: number; readonly breakIndex: number | null }): Promise<VecmOutcome> {
  const request = newWorkerRequestId()
  return post<VecmOutcome>('vecm', { kind: 'vecm', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'vecm', resolve }))
}

export function runSyntheticControl(values: Float64Array, rows: number, columns: number, design: { readonly treated: number; readonly donors: readonly number[]; readonly nPre: number }): Promise<SyntheticOutcome> {
  const request = newWorkerRequestId()
  return post<SyntheticOutcome>('synthetic-control', { kind: 'synthetic-control', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'synthetic-control', resolve }))
}

export function runPanelIntervention(
  values: Float64Array,
  rows: number,
  units: readonly string[],
  times: readonly number[],
  onProgress?: (progress: AnalysisProgress) => void,
): Promise<PanelInterventionOutcome> {
  const request = newWorkerRequestId()
  return new Promise((resolve) => {
    pending.set(request, { kind: 'panel-intervention', resolve, onProgress })
    const command: AnalysisWorkerCommand = { kind: 'panel-intervention', request, values, rows, units, times }
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
  return post<NegbinNutsOutcome>('negbin-nuts', { kind: 'negbin-nuts', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'negbin-nuts', resolve }))
}

export function runBayesianGaussian(values: Float64Array, rows: number, columns: number, design: { readonly treatment: number; readonly outcome: number; readonly adjustment: readonly number[]; readonly warmup: number; readonly samples: number; readonly seed: number }): Promise<BayesianGaussianOutcome> {
  const request = newWorkerRequestId()
  return post<BayesianGaussianOutcome>('bayesian-gaussian', { kind: 'bayesian-gaussian', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'bayesian-gaussian', resolve }))
}

export function runDiscreteBnQuery(values: Float64Array, rows: number, columns: number, design: { readonly nodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly bins: number; readonly equivalentSampleSize: number }): Promise<DiscreteBnOutcome> {
  const request = newWorkerRequestId()
  return post<DiscreteBnOutcome>('discrete-bn-query', { kind: 'discrete-bn-query', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'discrete-bn-query', resolve }))
}

export function runBinaryEtt(values: Float64Array, rows: number, columns: number, design: { readonly observedNodes: readonly number[]; readonly names: readonly string[]; readonly edges: readonly (readonly [number, number])[]; readonly treatment: number; readonly outcome: number; readonly unobserved: readonly number[] }): Promise<BinaryEttOutcome> {
  const request = newWorkerRequestId()
  return post<BinaryEttOutcome>('binary-ett', { kind: 'binary-ett', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'binary-ett', resolve }))
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
    pending.set(request, { kind: 'dag-check', resolve, onProgress })
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
  return post<LinearScmOutcome>('linear-scm-counterfactual', { kind: 'linear-scm-counterfactual', request, values, rows, columns, ...design }, values, (resolve) => ({ kind: 'linear-scm-counterfactual', resolve }))
}

export function seasonalAdjustInWorker(values: Float64Array, rows: number, columns: number, design: { readonly period: number; readonly robust: boolean; readonly adjust: readonly number[] }): Promise<SeasonalOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its matrix.
  const copy = Float64Array.from(values)
  return post<SeasonalOutcome>('seasonal-adjust', { kind: 'seasonal-adjust', request, values: copy, rows, columns, ...design }, copy, (resolve) => ({ kind: 'seasonal-adjust', resolve }))
}

export function resolveMissingnessInWorker(values: Float64Array, rows: number, columns: number, validity: Uint8Array, resolution: MissingnessResolutionCommand): Promise<MissingnessOutcome> {
  const request = newWorkerRequestId()
  // The values buffer is copied rather than transferred: the caller keeps its nullable matrix.
  const copy = Float64Array.from(values)
  return post<MissingnessOutcome>('resolve-missingness', { kind: 'resolve-missingness', request, values: copy, rows, columns, validity, resolution }, copy, (resolve) => ({ kind: 'resolve-missingness', resolve }))
}
