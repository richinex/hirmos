import { err, ok, type Result } from '@/domain/dop'
import {
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisWorkerEvent,
  type AnalysisWorkerProblem,
} from '@/workers/analysisProtocol'

/**
 * A pool of analysis workers for work that splits into independent pieces.
 *
 * The ordinary run path keeps its single worker. Independent search tasks fan out while sharing
 * the same cancellation owner as that ordinary path.
 */

const activePools = new Set<AbortController>()

/** Register before launching tasks; release in finally, including failed worker creation. */
export function createPoolScope() {
  const controller = new AbortController()
  activePools.add(controller)
  return {
    signal: controller.signal,
    abort: () => controller.abort(),
    release: () => {
      activePools.delete(controller)
    },
  }
}

export function cancelPooledAnalyses(): void {
  for (const controller of activePools) controller.abort()
}

/** FIFO counting semaphore, so a pool at capacity queues rather than oversubscribing. */
export class AsyncSemaphore {
  private permits: number
  private readonly waiters: Array<() => void> = []

  constructor(permits: number) {
    if (permits <= 0) throw new Error('A semaphore needs at least one permit')
    this.permits = permits
  }

  async acquire(): Promise<void> {
    if (this.permits > 0) {
      this.permits -= 1
      return
    }
    return new Promise<void>((resolve) => {
      this.waiters.push(resolve)
    })
  }

  release(): void {
    const next = this.waiters.shift()
    if (next) next()
    else this.permits += 1
  }

  get available(): number {
    return this.permits
  }
  get waiting(): number {
    return this.waiters.length
  }
}

/** Each worker instantiates its own copy of the analysis module. */
const WORKER_MEGABYTES = 64
const BROWSER_OVERHEAD_MEGABYTES = 512
const FEWEST = 1
const MOST = 8

/**
 * How many analysis workers to run beside the main thread. Cores set the ceiling and memory
 * lowers it; `deviceMemory` is Chromium-only, so other browsers take the conservative branch.
 */
export function computeMaxWorkers(): number {
  const cores = navigator.hardwareConcurrency ?? 4
  const byCores = Math.max(1, cores - 1)
  const deviceMemoryGb = (navigator as { deviceMemory?: number }).deviceMemory
  const availableMegabytes = (deviceMemoryGb ?? 4) * 1024 - BROWSER_OVERHEAD_MEGABYTES
  const byMemory = Math.floor(availableMegabytes / WORKER_MEGABYTES)
  return Math.max(FEWEST, Math.min(Math.min(byCores, byMemory), MOST))
}

/**
 * Send one command to a pooled worker and settle with the result event of `kind`, a failure, or
 * cancellation. Progress events are ignored, and the worker is terminated if the scope aborts.
 */
export function runOnWorker<Kind extends AnalysisWorkerEvent['kind']>(
  worker: Worker,
  command: AnalysisWorkerCommand,
  kind: Kind,
  transfer: readonly Transferable[],
  signal: AbortSignal,
): Promise<Result<Extract<AnalysisWorkerEvent, { readonly kind: Kind }>, AnalysisWorkerProblem>> {
  return new Promise((resolve) => {
    const settle = (
      outcome: Result<Extract<AnalysisWorkerEvent, { readonly kind: Kind }>, AnalysisWorkerProblem>,
    ) => {
      signal.removeEventListener('abort', abort)
      worker.onmessage = null
      worker.onerror = null
      resolve(outcome)
    }
    const abort = () => {
      worker.terminate()
      settle(err({ kind: 'analysis-cancelled', detail: 'The analysis run was cancelled.' }))
    }
    signal.addEventListener('abort', abort, { once: true })
    if (signal.aborted) {
      abort()
      return
    }
    worker.onmessage = (message: MessageEvent<unknown>) => {
      const parsed = parseAnalysisWorkerEvent(message.data)
      if (!parsed.ok) {
        settle(err({ kind: 'worker-protocol-failed', detail: parsed.error.detail }))
        return
      }
      const event = parsed.value
      if (event.kind === 'protocol-failed') {
        settle(err({ kind: 'worker-protocol-failed', detail: event.detail }))
        return
      }
      if (event.kind === 'analysis-progress' || event.request !== command.request) return
      if (event.kind === 'analysis-failed') {
        settle(err(event.problem))
        return
      }
      if (event.kind === kind) {
        settle(ok(event as Extract<AnalysisWorkerEvent, { readonly kind: Kind }>))
        return
      }
      settle(err({ kind: 'worker-protocol-failed', detail: `The worker returned ${event.kind}.` }))
    }
    worker.onerror = (event) => {
      event.preventDefault()
      settle(
        err({
          kind: 'worker-unavailable',
          detail: event.message || 'A pooled worker stopped unexpectedly.',
        }),
      )
    }
    try {
      worker.postMessage(command, [...transfer])
    } catch (cause: unknown) {
      settle(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })
}
