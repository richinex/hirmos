import { openRouterStream } from './openRouterStream'
import { test, expect } from '@playwright/test'
import { runCodeActLoop } from '../src/assistant/octopus/rlm/codeactLoop'
import type { CodeActLoopOptions, CodeExecResult } from '../src/assistant/octopus/rlm/codeactPorts'
import { asNodeId } from '../src/assistant/octopus/rlm/types'
import { conversationRecords } from '../src/assistant/conversation'
import type { Turn } from '../src/assistant/TurnView'
import { assistantLimits } from '../src/assistant/limits'

const finished = (): CodeExecResult => ({
  stdout: '',
  stderr: '',
  exit_code: 0,
  answer: { ready: true, content: 'Supported answer', citations: ['project-summary'] },
  locals: {},
})
const fixture = (texts: readonly string[]) => {
  const executed: string[] = []
  const budgets: number[] = []
  const feedback: string[] = []
  let calls = 0
  let resets = 0
  const options: CodeActLoopOptions = {
    system: 'test',
    maxIters: 4,
    maxTokens: 100,
    guidance: {
      finish: 'Finish with evidence.',
      uncited: 'Evidence required.',
      rejected: 'Evidence rejected.',
    },
    callModel: async (request) => {
      budgets.push(request.maxTokens)
      feedback.push(JSON.stringify(request.messages))
      const text = texts[calls++] ?? '```python\nfinish()\n```'
      return { text, content: [{ type: 'text', text }], stop_reason: 'end_turn' }
    },
    acquireExecutor: async () => ({
      execute: async (code) => {
        executed.push(code)
        return finished()
      },
      resetAnswer: () => {
        resets++
      },
    }),
    validateCitations: async () => ({
      kind: 'grounded',
      citations: [{ nodeId: asNodeId('project-summary'), label: 'Project summary' }],
    }),
  }
  return { options, executed, budgets, feedback, resets: () => resets }
}

test('Octopus recovers prose and executes only the first Python block', async () => {
  const f = fixture(['I will check.', '```python\nfirst()\n```\n```python\nnever()\n```'])
  const session = await runCodeActLoop('question', f.options)
  expect(f.executed).toEqual(['first()'])
  expect(f.feedback[1]).toContain('No python code block found')
  expect(session.kind === 'terminated' && session.outcome.kind).toBe('answered')
})

test('Python errors discard an answer and feed the error back for repair', async () => {
  const f = fixture(['```python\nbroken()\n```'])
  f.options = {
    ...f.options,
    acquireExecutor: async () => ({
      execute: async (code) => {
        f.executed.push(code)
        return f.executed.length === 1
          ? { ...finished(), exit_code: 1, stderr: 'NameError: missing variable' }
          : finished()
      },
      resetAnswer: () => {
        f.executed.push('reset')
      },
    }),
  }
  const session = await runCodeActLoop('question', f.options)
  expect(f.executed).toEqual(['broken()', 'reset', 'finish()'])
  expect(f.feedback[1]).toContain('NameError')
  expect(session.kind === 'terminated' && session.outcome.kind).toBe('answered')
})

test('truncated code is retried with more tokens and never executed as a fragment', async () => {
  const f = fixture([])
  f.options = {
    ...f.options,
    callModel: async (request) => {
      f.budgets.push(request.maxTokens)
      const truncated = f.budgets.length === 1
      return {
        text: truncated ? '```python\nbroken(' : '```python\nfinish()\n```',
        content: [],
        stop_reason: truncated ? 'max_tokens' : 'end_turn',
      }
    },
  }
  await runCodeActLoop('question', f.options)
  expect(f.budgets).toEqual([100, 400])
  expect(f.executed).toEqual(['finish()'])
})

test('unverified evidence cannot settle an answer', async () => {
  const f = fixture([])
  let checks = 0
  f.options = {
    ...f.options,
    validateCitations: async () =>
      ++checks === 1
        ? { kind: 'uncited' }
        : {
            kind: 'grounded',
            citations: [{ nodeId: asNodeId('project-summary'), label: 'Project summary' }],
          },
  }
  await runCodeActLoop('question', f.options)
  expect(f.resets()).toBe(1)
  expect(f.feedback[1]).toContain('Evidence required')
})

test('conversation recall survives context changes, retains failed questions and excludes running turns and traces', () => {
  const turns: Turn[] = Array.from({ length: 9 }, (_, i) => ({
    id: String(i),
    question: 'Q'.repeat(400),
    context: 'current',
    trace: [{ agent: 'Assistant', kind: 'python', text: 'private code', at: 1 }],
    outcome: {
      kind: 'answered',
      answer: { answer: String(i) + 'A'.repeat(900), evidence: ['approved-run'] },
    },
  }))
  turns.push(
    { ...turns[0], context: 'old' },
    { ...turns[0], outcome: { kind: 'failed', message: 'failed' } },
    { ...turns[0], question: 'Use period, not date', outcome: { kind: 'cancelled' } },
    { ...turns[0], question: 'Still running', outcome: { kind: 'running' } },
  )
  const memory = conversationRecords(turns)
  expect(memory).toHaveLength(6)
  expect(memory[0].answer.startsWith('6')).toBe(true)
  expect(memory.at(-1)?.question).toBe('Use period, not date')
  expect(memory.at(-2)?.answer).toContain('turn failed')
  expect(memory.at(-3)?.answer.startsWith('0')).toBe(true)
  expect(JSON.stringify(memory)).not.toContain('private code')
  expect(JSON.stringify(memory)).not.toContain('approved-run')
  expect(JSON.stringify(memory)).not.toContain('Still running')
  expect(conversationRecords([])).toEqual([])
})

test('host cancellation prevents another model request', async () => {
  const f = fixture([])
  const controller = new AbortController()
  controller.abort()
  await expect(
    runCodeActLoop('question', { ...f.options, signal: controller.signal }),
  ).rejects.toThrow()
  expect(f.budgets).toEqual([])
})

test('step limits reject fractional, non-finite and contradictory budgets', () => {
  for (const maxIters of [0, 1.5, NaN, Infinity, 25]) {
    expect(assistantLimits.safeParse({ maxIters, maxRequests: 30 }).success).toBe(false)
  }
  expect(assistantLimits.safeParse({ maxIters: 12, maxRequests: 10 }).success).toBe(false)
  expect(assistantLimits.safeParse({ maxIters: 12, maxRequests: 30 }).success).toBe(true)
})

test('the configured step limit controls execution through the UI', async ({ page }) => {
  let calls = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', async (route) => {
    expect(route.request().postDataJSON().max_tokens).toBe(20000)
    const code =
      ++calls % 3 === 0
        ? 'await FINAL_VAR("Completed on step three", ["user-question"])'
        : 'print("Need another step")'
    await route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await expect(
    page.getByRole('spinbutton', { name: 'Initial output tokens per request' }),
  ).toHaveValue('16384')
  await page.getByRole('spinbutton', { name: 'Initial output tokens per request' }).fill('20000')
  await page.getByRole('spinbutton', { name: 'Maximum steps per agent', exact: true }).fill('2')
  await page.getByRole('spinbutton', { name: 'Shared request limit', exact: true }).fill('8')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.screenshot({ path: '/tmp/hirmos-codeact-settings.png' })
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Use two steps.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('alert').filter({ hasText: 'max iterations' })).toBeVisible({
    timeout: 30000,
  })
  expect(calls).toBe(2)
  await page
    .getByRole('region', { name: 'OpenRouter connection' })
    .getByRole('button')
    .first()
    .click()
  await page.getByRole('spinbutton', { name: 'Maximum steps per agent', exact: true }).fill('3')
  calls = 0
  await page.getByLabel('Ask a question').fill('Use three steps.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Completed on step three',
    { timeout: 30000 },
  )
  expect(calls).toBe(3)
})

test('browser CodeAct recovers a Python error without losing its namespace', async ({ page }) => {
  let calls = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', async (route) => {
    const code =
      calls++ === 0
        ? 'remembered = 42\nraise ValueError("repair me")'
        : 'await FINAL_VAR("Recovered " + str(remembered), ["user-question"])'
    await route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  const answer = await page.evaluate(async () => {
    // @ts-expect-error Vite browser module.
    const { runAssistant } = await import('/src/assistant/agent.ts')
    return runAssistant({
      key: 'test',
      model: 'test/model',
      question: 'Remember 42',
      context: '{}',
      signal: AbortSignal.timeout(30000),
      trace: () => {},
      propose: null,
      proposeSql: null,
      requestRun: async () => ({ kind: 'refused', reason: 'Not enabled' }),
    })
  })
  expect(answer.answer).toBe('Recovered 42')
  expect(calls).toBe(2)
  expect(await page.locator('iframe[sandbox="allow-scripts"]').count()).toBe(0)
})
