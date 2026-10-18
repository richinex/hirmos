import { err, ok, type Result } from '@/domain/dop'
import type { ImitationRuleRequest } from '@/domain/decisionEvidence'
import { newWorkerRequestId, type AnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { AsyncSemaphore, computeMaxWorkers, createPoolScope, runOnWorker } from './workerPool'

/**
 * Copy the demonstrator once per seed, the seeds spread across a pool of workers, and return each
 * seed's action probabilities for every input pattern, in seed order.
 */
export async function copyRuleAcrossSeeds(
  inputs: Float64Array,
  model: ImitationRuleRequest,
  seeds: readonly number[],
  onProgress?: (completed: number, total: number) => void,
): Promise<Result<readonly (readonly (readonly number[])[])[], AnalysisWorkerProblem>> {
  const size = Math.min(computeMaxWorkers(), seeds.length)
  const scope = createPoolScope()
  const semaphore = new AsyncSemaphore(size)
  const workers: Worker[] = []
  const copied = new Array<readonly (readonly number[])[] | null>(seeds.length).fill(null)
  let failure: AnalysisWorkerProblem | null = null
  let completed = 0
  try {
    for (let index = 0; index < size; index += 1)
      workers.push(
        new Worker(new URL('../workers/analysis.worker.ts', import.meta.url), {
          type: 'module',
          name: `hirmos-imitation-${index}`,
        }),
      )
    let next = 0
    await Promise.all(
      workers.map(async (worker) => {
        for (;;) {
          const index = next
          next += 1
          if (index >= seeds.length || failure !== null || scope.signal.aborted) return
          await semaphore.acquire()
          try {
            const seed = seeds[index]!
            // Each worker needs its own copy: posting transfers the buffer and neuters the original.
            const values = inputs.slice()
            const outcome = await runOnWorker(
              worker,
              {
                kind: 'imitation-rule',
                request: newWorkerRequestId(),
                values,
                model: {
                  ...model,
                  settings: { ...model.settings, seed },
                  sample: model.sample === null ? null : { ...model.sample, seed },
                },
              },
              'imitation-rule-succeeded',
              [values.buffer],
              scope.signal,
            )
            if (!outcome.ok) {
              failure ??= outcome.error
              return
            }
            copied[index] = outcome.value.result.probabilities
            completed += 1
            onProgress?.(completed, seeds.length)
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
  if (scope.signal.aborted)
    return err({ kind: 'analysis-cancelled', detail: 'The analysis run was cancelled.' })
  if (failure !== null) return err(failure)
  if (copied.some((rule) => rule === null))
    return err({ kind: 'worker-protocol-failed', detail: 'A seed returned no copied rule.' })
  return ok(copied as readonly (readonly (readonly number[])[])[])
}
