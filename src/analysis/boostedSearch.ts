import { err, ok, type Result } from '@/domain/dop'
import {
  newWorkerRequestId,
  parseAnalysisWorkerEvent,
  type AnalysisWorkerCommand,
  type AnalysisWorkerProblem,
} from '@/workers/analysisProtocol'
import { AsyncSemaphore, computeMaxWorkers, createPoolScope } from './workerPool'

const cancelled = (): AnalysisWorkerProblem => ({
  kind: 'analysis-cancelled',
  detail: 'The analysis run was cancelled.',
})

export interface BoostedCandidate {
  readonly learningRate: number
  readonly maxDepth: number
  readonly nEstimators: number
  readonly meanScore: number
}

export interface BoostedGrid {
  readonly learningRate: readonly number[]
  readonly maxDepth: readonly number[]
  readonly nEstimators: readonly number[]
  readonly splits: number
  readonly minSamplesLeaf: number
  readonly minSamplesSplit: number
  readonly seed: number
}

interface Slice {
  readonly learningRate: number
  readonly maxDepth: number
}

/** The kernel's own candidate order: learning rate outermost, then depth. */
const slicesOf = (grid: BoostedGrid): readonly Slice[] =>
  grid.learningRate.flatMap((learningRate) =>
    grid.maxDepth.map((maxDepth) => ({ learningRate, maxDepth })),
  )

const runSlice = (
  worker: Worker,
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
  },
  grid: BoostedGrid,
  slice: Slice,
  signal: AbortSignal,
): Promise<Result<readonly BoostedCandidate[], AnalysisWorkerProblem>> =>
  new Promise((resolve) => {
    const request = newWorkerRequestId()
    const settle = (outcome: Result<readonly BoostedCandidate[], AnalysisWorkerProblem>) => {
      signal.removeEventListener('abort', abort)
      worker.onmessage = null
      worker.onerror = null
      resolve(outcome)
    }
    const abort = () => {
      worker.terminate()
      settle(err(cancelled()))
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
      if (event.kind === 'analysis-progress') return
      if (event.request !== request) return
      if (event.kind === 'analysis-failed') {
        settle(err(event.problem))
        return
      }
      if (event.kind === 'propensity-grid-slice-succeeded') {
        settle(ok(event.result.scores))
        return
      }
      settle(
        err({ kind: 'worker-protocol-failed', detail: `The grid slice returned ${event.kind}.` }),
      )
    }
    worker.onerror = (event) => {
      event.preventDefault()
      settle(
        err({
          kind: 'worker-unavailable',
          detail: event.message || 'A search worker stopped unexpectedly.',
        }),
      )
    }
    // Each worker needs its own copy: posting transfers the buffer and neuters the original.
    const copy = values.slice()
    const command: AnalysisWorkerCommand = {
      kind: 'propensity-grid-slice',
      request,
      values: copy,
      rows,
      columns,
      treatment: design.treatment,
      outcome: design.outcome,
      adjustment: design.adjustment,
      learningRate: slice.learningRate,
      maxDepth: slice.maxDepth,
      nEstimators: grid.nEstimators,
      splits: grid.splits,
      minSamplesLeaf: grid.minSamplesLeaf,
      minSamplesSplit: grid.minSamplesSplit,
      seed: grid.seed,
    }
    try {
      worker.postMessage(command, [copy.buffer])
    } catch (cause: unknown) {
      settle(
        err({
          kind: 'worker-unavailable',
          detail: cause instanceof Error ? cause.message : String(cause),
        }),
      )
    }
  })

/**
 * Scores every candidate across a pool of workers and returns what the single-threaded search
 * would have chosen: merged back into the kernel's candidate order, first strict maximum winning.
 */
export async function runBoostedGridSearch(
  values: Float64Array,
  rows: number,
  columns: number,
  design: {
    readonly treatment: number
    readonly outcome: number
    readonly adjustment: readonly number[]
  },
  grid: BoostedGrid,
  onProgress?: (completed: number, total: number) => void,
): Promise<
  Result<{ readonly best: BoostedCandidate; readonly workers: number }, AnalysisWorkerProblem>
> {
  const slices = slicesOf(grid)
  if (slices.length === 0)
    return err({ kind: 'worker-protocol-failed', detail: 'The grid search has no candidates.' })
  const size = Math.min(computeMaxWorkers(), slices.length)
  const scope = createPoolScope()
  const workers: Worker[] = []
  const semaphore = new AsyncSemaphore(size)
  const scored = new Array<readonly BoostedCandidate[] | null>(slices.length).fill(null)
  let failure: AnalysisWorkerProblem | null = null
  let completed = 0

  try {
    for (let index = 0; index < size; index += 1) {
      workers.push(
        new Worker(new URL('../workers/analysis.worker.ts', import.meta.url), {
          type: 'module',
          name: `hirmos-search-${index}`,
        }),
      )
    }
    let next = 0
    await Promise.all(
      workers.map(async (worker) => {
        for (;;) {
          const index = next
          next += 1
          if (index >= slices.length || failure !== null || scope.signal.aborted) return
          await semaphore.acquire()
          try {
            const outcome = await runSlice(
              worker,
              values,
              rows,
              columns,
              design,
              grid,
              slices[index]!,
              scope.signal,
            )
            if (!outcome.ok) {
              failure ??= outcome.error
              return
            }
            scored[index] = outcome.value
            completed += 1
            onProgress?.(completed, slices.length)
          } finally {
            semaphore.release()
          }
        }
      }),
    )
  } catch (cause: unknown) {
    failure ??= {
      kind: 'worker-unavailable',
      detail: cause instanceof Error ? cause.message : String(cause),
    }
  } finally {
    for (const worker of workers) worker.terminate()
    scope.release()
  }

  if (scope.signal.aborted) return err(cancelled())
  if (failure !== null) return err(failure)
  const merged = scored.flatMap((entry) => entry ?? [])
  if (merged.length === 0) {
    return err({ kind: 'worker-protocol-failed', detail: 'The grid search scored no candidates.' })
  }
  let best = merged[0]!
  for (const candidate of merged) if (candidate.meanScore > best.meanScore) best = candidate
  return ok({ best, workers: size })
}
