import { err, type Result } from '@/domain/dop'
import type {
  DynotearsEvidence,
  GrangerSsrEvidence,
  LpcmciEvidence,
  OcseEvidence,
  PcmciPlusEvidence,
} from '@/domain/discovery'
import type { StationarityBattery } from '@/domain/stationarity'
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
type OcseOutcome = Result<OcseEvidence, AnalysisWorkerProblem>
type PendingRun =
  | { readonly kind: 'stationarity'; readonly resolve: (outcome: StationarityOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'pcmci-plus'; readonly resolve: (outcome: PcmciPlusOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'lpcmci'; readonly resolve: (outcome: LpcmciOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'dynotears'; readonly resolve: (outcome: DynotearsOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'ocse'; readonly resolve: (outcome: OcseOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }
  | { readonly kind: 'granger'; readonly resolve: (outcome: GrangerOutcome) => void; readonly onProgress?: (progress: AnalysisProgress) => void }

let worker: Worker | null = null
const pending = new Map<WorkerRequestId, PendingRun>()

const failAll = (problem: AnalysisWorkerProblem) => {
  for (const run of pending.values()) run.resolve(err(problem))
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
      run.resolve({ ok: false, error: parsed.value.problem })
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
    if (run.kind === 'ocse' && parsed.value.kind !== 'ocse-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for an oCSE request.' })
      return
    }
    if (run.kind === 'granger' && parsed.value.kind !== 'granger-succeeded') {
      failAll({ kind: 'worker-protocol-failed', detail: 'The analysis worker returned another method result for a Granger request.' })
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
    if (run.kind === 'ocse' && parsed.value.kind === 'ocse-succeeded') {
      run.resolve({ ok: true, value: parsed.value.result })
      return
    }
    if (run.kind === 'granger' && parsed.value.kind === 'granger-succeeded') {
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
