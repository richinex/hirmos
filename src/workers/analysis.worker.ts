/// <reference lib="webworker" />

import initWasm, { runAnalysis } from '@/generated/analysis-wasm/hirmos_analysis'
import {
  parseDynotearsEvidence,
  parseGrangerSsrEvidence,
  parseLpcmciEvidence,
  parseOcseEvidence,
  parsePcmciPlusEvidence,
} from '@/domain/discovery'
import { assertNever } from '@/domain/dop'
import { parseStationarityBattery } from '@/domain/stationarity'
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
      default:
        return assertNever(command)
    }
  })()
}
