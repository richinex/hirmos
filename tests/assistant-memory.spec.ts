import { openRouterStream } from './openRouterStream'
import { test, expect } from '@playwright/test'
import { matchPriorAnswers } from '../src/assistant/memory/priorAnswers'
import { priorTaskRecords } from '../src/assistant/octopus/rlm/priorTasks'

test('shortened recaps point to the original turn', () => {
  const recap = priorTaskRecords([
    {
      question: 'What changed?',
      answer: 'a'.repeat(1100) + 'Balance improved to 0.05.',
      record: { turnId: 'earlier-turn', kind: 'task_record' },
    },
  ])
  expect(recap).toContain('This recap is shortened')
  expect(recap).not.toContain('Balance improved')
  expect(recap).toContain('await DAG_HISTORY(kind="task_record", turn_id="earlier-turn")')
})

test('semantic recall ranks similar questions and excludes unrelated ones', () => {
  const record = (question: string) => ({ question, answer: question, citations: [] })
  const matched = matchPriorAnswers(new Float32Array([1, 0]), [
    { record: record('unrelated'), vec: new Float32Array([0, 1]) },
    { record: record('similar'), vec: new Float32Array([0.8, 0.6]) },
    { record: record('same'), vec: new Float32Array([1, 0]) },
  ])
  expect(matched.map((r) => r.question)).toEqual(['same', 'similar'])
})

test('Octopus memory keeps originals, flags changed specifications and isolates projects', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    // @ts-expect-error Vite module.
    const { beginMemoryTask, loadMemoryTurns } = await import('/src/assistant/memory/access.ts')
    // @ts-expect-error Vite module.
    const { getActiveSessionId } = await import('/src/assistant/memory/session.ts')
    // @ts-expect-error Vite module.
    const { clearProjectMemory, nodesBySession } = await import('/src/assistant/memory/dagStore.ts')
    const a = getActiveSessionId('project-a'),
      b = getActiveSessionId('project-b')
    const errors: string[] = []
    const task = await beginMemoryTask(a, 'Why were 192 rows retained?', 'old', 't1', (s: string) =>
      errors.push(s),
    )
    task.trace({ agent: 'Assistant', kind: 'python', text: 'print(192)' })
    await task.finish({ answer: 'Earlier discussion of 192 rows.', evidence: ['user-question'] })
    const other = await beginMemoryTask(b, 'Other project', 'other', 't2', (s: string) =>
      errors.push(s),
    )
    await other.finish({ answer: 'Other answer', evidence: ['user-question'] })
    const next = await beginMemoryTask(a, 'Recall it', 'changed', 't3', (s: string) =>
      errors.push(s),
    )
    const records = await next.access.read(
      'DAG_HISTORY',
      { kind: 'task_record', limit: 10 },
      AbortSignal.timeout(10000),
    )
    const found = await next.access.read('DAG_GREP', { pattern: '192' }, AbortSignal.timeout(10000))
    const exact = await next.access.read(
      'DAG_HISTORY',
      { kind: 'task_record', turn_id: 't1', limit: 1 },
      AbortSignal.timeout(10000),
    )
    const foreign = await next.access.read(
      'DAG_HISTORY',
      { turn_id: 't2' },
      AbortSignal.timeout(10000),
    )
    const unknown = await next.access.read(
      'DAG_HISTORY',
      { turn_id: 'absent' },
      AbortSignal.timeout(10000),
    )
    const expanded = await next.access.read(
      'DAG_EXPAND',
      { node_id: records[0].node_id },
      AbortSignal.timeout(10000),
    )
    const before = await loadMemoryTurns(a)
    await clearProjectMemory('project-a')
    return {
      records,
      exact,
      foreign,
      unknown,
      found,
      expanded,
      before,
      after: await nodesBySession(a),
      other: await loadMemoryTurns(b),
      errors,
    }
  })
  expect(result.records).toHaveLength(1)
  expect(result.exact).toEqual(result.records)
  expect(result.foreign).toEqual([])
  expect(result.unknown).toEqual([])
  expect(result.records[0].currentSpecification).toBe(false)
  expect(result.found.some((r: { kind: string }) => r.kind === 'code_execution')).toBe(true)
  expect(result.expanded[0].answer).toContain('192')
  expect(result.before).toHaveLength(1)
  expect(result.after).toEqual([])
  expect(result.other).toHaveLength(1)
  expect(result.errors).toEqual([])
})

test('conversation survives reload, is available to CodeAct and can be deleted', async ({
  page,
}) => {
  let calls = 0
  const fullAnswer =
    'Context for this analysis. '.repeat(50) + 'Balance improved from 1.67 to 0.05.'
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    const code =
      ++calls === 1
        ? `await FINAL_VAR(${JSON.stringify(fullAnswer)}, ["user-question"])`
        : 'records = await DAG_HISTORY(kind="task_record")\noriginal = await DAG_HISTORY(kind="task_record", turn_id=records[0]["metadata"]["turnId"])\nawait FINAL_VAR("Remembered: " + original[0]["answer"], ["conversation-memory"])'
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  const connect = async () => {
    await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
    await page.getByLabel('OpenRouter API key').fill('test-not-stored')
    await page.getByLabel('OpenRouter model ID').fill('test/model')
    await page.getByRole('button', { name: 'Privacy', exact: true }).click()
    await page.getByRole('button', { name: 'I agree', exact: true }).click()
    await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  }
  await page.goto('/app')
  await connect()
  await page.getByLabel('Ask a question').fill('We have 192 rows.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(fullAnswer, {
    timeout: 30000,
  })
  await page.reload()
  await connect()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(fullAnswer)
  await page.getByLabel('Ask a question').fill('What did we discuss?')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Remembered: ' + fullAnswer,
    { timeout: 30000 },
  )
  await page.screenshot({ path: '/tmp/hirmos-persistent-memory.png' })
  await page.getByRole('button', { name: 'Clear conversation', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toHaveCount(0)
  await page.reload()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await expect(page.getByLabel('OpenRouter API key')).toHaveValue('')
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toHaveCount(0)
})
