import { err, ok, type Result } from '@/domain/dop'
import { causalForestSettingsMatch, sameCausalForestTarget, type CausalForestEvidence } from '@/domain/causalForest'
import { forestTuningEventSchema, type ForestTuningCommand, type ForestTuningEvent, forestTuningRequestSchema } from '@/domain/forestTuning'
import type { z } from 'zod'
import type { AnalysisWorkerProblem } from '@/workers/analysisProtocol'
import { computeMaxWorkers, createPoolScope } from './workerPool'

type Design = Omit<z.infer<typeof forestTuningRequestSchema>, 'scores'>
type Command = ForestTuningCommand extends infer C ? C extends { id: number } ? Omit<C, 'id'> : never : never
let sequence = 0

function request(worker: Worker, command: Command, signal: AbortSignal): Promise<ForestTuningEvent> {
  return new Promise((resolve, reject) => {
    const id = ++sequence
    const clean = () => { signal.removeEventListener('abort', abort); worker.onmessage = null; worker.onerror = null }
    const fail = (cause: unknown) => { clean(); reject(cause) }
    const abort = () => { worker.terminate(); fail(new Error('The analysis run was cancelled.')) }
    signal.addEventListener('abort', abort, { once: true })
    if (signal.aborted) { abort(); return }
    worker.onerror = event => { event.preventDefault(); fail(new Error(event.message || 'A tuning worker stopped unexpectedly.')) }
    worker.onmessage = event => {
      const parsed = forestTuningEventSchema.safeParse(event.data)
      if (!parsed.success) { fail(new Error('Invalid forest tuning response.')); return }
      if (parsed.data.id !== id) { fail(new Error('The tuning response belongs to another request.')); return }
      if (parsed.data.kind === 'failed') { fail(new Error(parsed.data.detail)); return }
      clean(); resolve(parsed.data)
    }
    try { worker.postMessage({ ...command, id }, command.kind === 'plan' ? [command.values.buffer] : []) }
    catch (cause) { fail(cause) }
  })
}

/** All fitting decisions stay in Rust. Pool results are stored in candidate order. */
export async function runForestTuning(values: Float64Array, design: Design): Promise<Result<CausalForestEvidence, AnalysisWorkerProblem>> {
  const scope = createPoolScope()
  const workers: Worker[] = []
  const spawn = (name: string) => {
    const worker = new Worker(new URL('../workers/forestTuning.worker.ts', import.meta.url), { type: 'module', name })
    workers.push(worker); return worker
  }
  try {
    const planner = spawn('hirmos-forest-plan')
    const pool: Worker[] = []
    const scores: Array<Array<number | null>> = []
    for (;;) {
      const reply = await request(planner, { kind: 'plan', values: values.slice(), request: { ...design, scores } }, scope.signal)
      if (reply.kind !== 'planned') throw new Error('The forest planner returned another result type.')
      if (reply.result.kind === 'complete') {
        const evidence = reply.result.evidence
        if (!sameCausalForestTarget(evidence.target, design.target) || !causalForestSettingsMatch(design.configuration, evidence)
            || evidence.observations !== design.rows || evidence.variableImportance.length !== design.adjustment.length)
          throw new Error('The tuned forest evidence does not match its specification.')
        return ok(evidence)
      }
      const batch = reply.result.batch
      if (batch.stage > scores.length || batch.offset !== (scores[batch.stage]?.length ?? 0)) throw new Error('The tuning batch is out of sequence.')
      while (pool.length < Math.min(computeMaxWorkers(), batch.options.length)) pool.push(spawn(`hirmos-forest-score-${pool.length}`))
      const completed = new Array<number | null | undefined>(batch.options.length)
      let next = 0
      await Promise.all(pool.slice(0, batch.options.length).map(async worker => {
        const prepared = await request(worker, { kind: 'prepare', batch }, scope.signal)
        if (prepared.kind !== 'prepared') throw new Error('The tuning worker did not accept its batch.')
        for (;;) {
          const index = next++
          if (index >= batch.options.length) return
          const reply = await request(worker, { kind: 'score', index }, scope.signal)
          if (reply.kind !== 'scored') throw new Error('The tuning worker did not return a score.')
          completed[index] = reply.score
        }
      }))
      const ordered = completed.map(score => { if (score === undefined) throw new Error('A tuning candidate has no response.'); return score })
      scores[batch.stage] = [...(scores[batch.stage] ?? []), ...ordered]
    }
  } catch (cause) {
    return err({ kind: scope.signal.aborted ? 'analysis-cancelled' : 'worker-unavailable', detail: cause instanceof Error ? cause.message : String(cause) })
  } finally { scope.abort(); for (const worker of workers) worker.terminate(); scope.release() }
}
