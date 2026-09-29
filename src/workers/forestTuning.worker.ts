/// <reference lib="webworker" />
import initWasm, { planForestTuning, scoreForestTuning } from '@/generated/analysis-wasm/hirmos_analysis'
import { forestTuningCommandSchema, forestTuningReplySchema, type ForestTuningEvent } from '@/domain/forestTuning'
import { assertNever } from '@/domain/dop'

let ready: Promise<unknown> | undefined
let batch: string | undefined
const emit = (event: ForestTuningEvent) => self.postMessage(event)
self.onmessage = async (event: MessageEvent<unknown>) => {
  const parsed = forestTuningCommandSchema.safeParse(event.data)
  if (!parsed.success) { emit({ kind: 'failed', id: 0, detail: 'Invalid forest tuning command.' }); return }
  const command = parsed.data
  try {
    ready ??= initWasm()
    await ready
    switch (command.kind) {
      case 'prepare': batch = JSON.stringify(command.batch); emit({ kind: 'prepared', id: command.id }); return
      case 'score': {
        if (batch === undefined) throw new Error('No tuning batch has been prepared.')
        const score: unknown = JSON.parse(scoreForestTuning(batch, command.index))
        if (score !== null && (typeof score !== 'number' || !Number.isFinite(score))) throw new Error('Invalid tuning score.')
        emit({ kind: 'scored', id: command.id, score }); return
      }
      case 'plan': {
        const result = forestTuningReplySchema.parse(JSON.parse(planForestTuning(command.values, JSON.stringify(command.request))))
        emit({ kind: 'planned', id: command.id, result }); return
      }
      default: return assertNever(command)
    }
  } catch (cause) { emit({ kind: 'failed', id: command.id, detail: cause instanceof Error ? cause.message : String(cause) }) }
}
