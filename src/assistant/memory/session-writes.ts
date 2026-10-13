// Octopus task records retain the originals and supersede their active conversational window.
import { addNode, closeTask, touchSession } from './dagStore'

export { addNode, closeTask, touchSession }

export async function writeTaskRecord(
  sessionId: string,
  task: string,
  answer: string,
  citations: readonly string[],
  replVariables?: Readonly<Record<string, string>>,
  metadata: Readonly<Record<string, unknown>> = {},
): Promise<void> {
  const content = [
    `Task: ${task.slice(0, 500)}`,
    `Answer: ${answer.slice(0, 1000)}`,
    citations.length ? `Cited: ${citations.slice(0, 12).join(', ')}` : '',
    replVariables && Object.keys(replVariables).length
      ? `REPL variables:\n${Object.entries(replVariables)
          .map(([k, v]) => `  ${k} = ${v.slice(0, 200)}`)
          .join('\n')}`
      : '',
  ]
    .filter(Boolean)
    .join('\n')
  const recordId = await addNode({
    session_id: sessionId,
    kind: 'task_record',
    content,
    metadata: {
      ...metadata,
      task,
      answer,
      citations: [...citations],
      repl_variables: replVariables ?? {},
    },
  })
  await closeTask(sessionId, recordId)
}
