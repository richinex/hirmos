import { err, type Result } from '@/domain/dop'
import type { GrangerSsrEvidence, PcmciPlusEvidence } from '@/domain/discovery'
import type { StationarityBattery } from '@/domain/stationarity'
import {
  newWorkerRequestId,
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisWorkerProblem,
  type WorkerRequestId,
} from '@/workers/analysisProtocol'

type StationarityOutcome = Result<StationarityBattery, AnalysisWorkerProblem>
type PcmciPlusOutcome = Result<PcmciPlusEvidence, AnalysisWorkerProblem>
type GrangerOutcome = Result<GrangerSsrEvidence, AnalysisWorkerProblem>
type PendingRun =
  | { readonly kind: 'stationarity'; readonly resolve: (outcome: StationarityOutcome) => void }
  | { readonly kind: 'pcmci-plus'; readonly resolve: (outcome: PcmciPlusOutcome) => void }
  | { readonly kind: 'granger'; readonly resolve: (outcome: GrangerOutcome) => void }

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
