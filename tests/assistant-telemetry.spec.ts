import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'
import type { Trace } from '../src/assistant/agent'

test('CodeAct is warned before its final step and reports unavailable early-stop cost', async ({
  page,
}) => {
  const warnings: string[] = []
  await page.route('https://openrouter.ai/api/v1/chat/completions', async (route) => {
    const messages = route.request().postDataJSON().messages
    const last = messages.at(-1).content
    const warning = typeof last === 'string' && last.startsWith('Execution budget:') ? last : ''
    warnings.push(warning)
    const code = warning
      ? 'await FINAL_VAR("Partial answer: more investigation is needed.", ["user-question"])'
      : 'print("First step complete")'
    await route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { runAssistant } = await import(new URL('/src/assistant/agent.ts', location.href).href)
    const events: Trace[] = []
    const answer = await runAssistant({
      key: 'test-key',
      model: 'test/model',
      question: 'Investigate',
      context: '{}',
      limits: { maxIters: 3, maxRequests: 8, maxTokens: 16384 },
      signal: new AbortController().signal,
      trace: (event: Trace) => events.push(event),
      propose: null,
      proposeSql: null,
      requestRun: async () => {
        throw new Error('No run authorized')
      },
    })
    return { answer, events }
  })
  expect(warnings).toHaveLength(3)
  expect(warnings[0]).toBe('')
  expect(warnings[1]).toBe('')
  expect(warnings[2]).toContain('this is step 3 of 3; 0 later steps')
  expect(result.answer.answer).toContain('Partial answer')
  const budgets = result.events.filter((event) => event.kind === 'budget')
  expect(budgets.map((event) => event.budget.step)).toEqual([1, 2, 3])
  const usage = result.events.filter((event) => event.kind === 'usage')
  expect(usage.map((event) => event.requestId)).toEqual([1, 2, 3])
  for (const event of usage)
    expect(event.usage).toMatchObject({ kind: 'unavailable', reason: 'early-stop', costUsd: null })
})

test('request and deadline warnings do not invent spend or reset times', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { budgetWarning, providerFailure } = await import(
      new URL('/src/assistant/telemetry.ts', location.href).href
    )
    const base = {
      step: 1,
      maxSteps: 8,
      requests: 1,
      maxRequests: 24,
      elapsedMs: 0,
      deadlineMs: 240000,
      outputTokenAllowance: 16384,
    }
    return {
      normal: budgetWarning(base),
      penultimate: budgetWarning({ ...base, step: 7 }),
      last: budgetWarning({ ...base, step: 8 }),
      request: budgetWarning({ ...base, requests: 23 }),
      deadline: budgetWarning({ ...base, elapsedMs: 200000 }),
      rate: providerFailure(429, {}, undefined),
      date: providerFailure(429, {}, 'Thu, 01 Jan 1970 00:01:00 GMT', 10000),
    }
  })
  expect(result.normal).toBeNull()
  expect(result.penultimate).toBeNull()
  expect(result.last).toContain('this is step 8 of 8; 0 later steps')
  expect(result.request).toContain('1 shared requests remain')
  expect(result.deadline).toContain('40 seconds')
  expect(result.rate.retryAfterSeconds).toBeNull()
  expect(result.rate.resetAt).toBeNull()
  expect(result.date.retryAfterSeconds).toBe(50)
})

test('OpenRouter usage retains cost, cached and reasoning tokens from the final event', async ({
  page,
}) => {
  const stream = openRouterStream({
    choices: [{ message: { content: '```python\nprint(1)\n```' } }],
  })
  const usage = {
    prompt_tokens: 100,
    completion_tokens: 30,
    total_tokens: 130,
    cost: 0.004,
    prompt_tokens_details: { cached_tokens: 60 },
    completion_tokens_details: { reasoning_tokens: 20 },
  }
  const final = `data: ${JSON.stringify({ id: 'test', choices: [], usage })}\n\n`
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill({ ...stream, body: stream.body.replace('data: [DONE]', final + 'data: [DONE]') }),
  )
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { openRouterModel } = await import(new URL('/src/assistant/model.ts', location.href).href)
    const reports: unknown[] = []
    await openRouterModel('test-key', 'test/gemini', () => {}, undefined, {
      usage: (value: unknown) => reports.push(value),
      failure: () => {},
    })({
      system: '',
      messages: [{ role: 'user', content: 'Print' }],
      maxTokens: 16384,
      onText: () => {},
    })
    return reports
  })
  expect(result).toEqual([
    {
      kind: 'reported',
      reason: null,
      inputTokens: 100,
      outputTokens: 30,
      reasoningTokens: 20,
      cachedTokens: 60,
      costUsd: 0.004,
      generationId: 'test',
    },
  ])
})

for (const [status, source] of [
  [402, 'openrouter_credits'],
  [402, 'openrouter_in_flight_budget'],
  [403, 'openrouter_key_limit'],
  [403, 'openrouter_workspace_budget'],
  [429, null],
] as const) {
  test(`provider error ${status} ${source} preserves retry metadata without retrying`, async ({
    page,
  }) => {
    let requests = 0
    await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
      requests++
      return route.fulfill({
        status,
        headers: { 'Retry-After': '12', 'Access-Control-Expose-Headers': 'Retry-After' },
        json: {
          error: {
            code: status,
            message: 'Provider detail',
            metadata: {
              ...(source ? { limit_source: source } : {}),
              resets_at: '2026-10-13T00:00:00Z',
            },
          },
        },
      })
    })
    await page.goto('/app')
    const result = await page.evaluate(async () => {
      const { openRouterModel } = await import(
        new URL('/src/assistant/model.ts', location.href).href
      )
      const failures: unknown[] = []
      try {
        await openRouterModel('test-key', 'test/model', () => {}, undefined, {
          usage: () => {},
          failure: (value: unknown) => failures.push(value),
        })({
          system: '',
          messages: [{ role: 'user', content: 'Test request' }],
          maxTokens: 16384,
          onText: () => {},
        })
        throw new Error('Expected provider refusal')
      } catch (error) {
        return { failures, message: (error as Error).message }
      }
    })
    expect(requests).toBe(1)
    expect(result.failures).toEqual([
      expect.objectContaining({
        status,
        limitSource: source,
        retryAfterSeconds: 12,
        resetAt: '2026-10-13T00:00:00Z',
      }),
    ])
    expect(result.message).toContain('Retry after 12 seconds')
  })
}
