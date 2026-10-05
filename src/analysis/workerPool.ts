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
