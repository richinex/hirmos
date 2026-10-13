import type { PriorTask } from './octopus/rlm/priorTasks'
import type { Turn } from './TurnView'

export function conversationRecords(turns: readonly Turn[]): PriorTask[] {
  return turns
    .filter((turn) => turn.outcome.kind !== 'running')
    .slice(-6)
    .map((turn) => ({
      question: turn.question,
      record: {
        turnId: turn.id,
        kind: turn.outcome.kind === 'answered' ? 'task_record' : 'user_message',
      },
      answer:
        turn.outcome.kind === 'answered'
          ? turn.outcome.answer.answer
          : turn.outcome.kind === 'failed'
            ? `The turn failed: ${turn.outcome.message}. Check current records before repeating any action.`
            : 'The turn was stopped. Check current records before repeating any action.',
    }))
}
