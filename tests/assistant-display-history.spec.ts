import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('display records retain telemetry separately, upsert by turn and clear only their project', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { saveDisplayTurn, loadDisplayTurns, clearDisplayHistory, mergeTurnHistory } =
      await import(new URL('/src/assistant/displayHistory.ts', location.href).href)
    const { nodesBySession } = await import(
      new URL('/src/assistant/memory/dagStore.ts', location.href).href
    )
    const trace = [
      { agent: 'Assistant', text: 'request', at: 1, kind: 'request' },
      {
        agent: 'Assistant',
        text: 'usage',
        at: 2,
        kind: 'usage',
        requestId: 1,
        usage: {
          kind: 'unavailable',
          generationId: null,
          reason: 'early-stop',
          inputTokens: null,
          outputTokens: null,
          reasoningTokens: null,
          cachedTokens: null,
          costUsd: null,
        },
      },
      { agent: 'Subagent 1', text: 'Check assumptions', at: 3, kind: 'task' },
    ]
    const turn = {
      id: 'one',
      question: 'q',
      context: '{}',
      trace,
      limits: { maxSteps: 8, maxRequests: 24, deadlineMs: 240000 },
      outcome: { kind: 'cancelled' },
    }
    await saveDisplayTurn('a', 'a:session', 1, turn)
    await saveDisplayTurn('a', 'a:session', 1, turn)
    await saveDisplayTurn('b', 'b:session', 1, turn)
    const restored = await loadDisplayTurns('a:session')
    const merged = mergeTurnHistory([{ ...turn, trace: [] }], restored)
    const memory = await nodesBySession('a:session')
    await clearDisplayHistory('a')
    return {
      restored,
      merged,
      memory,
      cleared: await loadDisplayTurns('a:session'),
      other: await loadDisplayTurns('b:session'),
      trace,
    }
  })
  expect(result.restored).toHaveLength(1)
  expect(result.restored[0].trace).toEqual(result.trace)
  expect(result.restored[0].context).toMatch(/^sha256:/)
  expect(result.merged).toEqual(result.restored)
  expect(result.memory).toEqual([])
  expect(result.cleared).toEqual([])
  expect(result.other).toHaveLength(1)
})

for (const outcome of ['answered', 'failed', 'cancelled'] as const) {
  test(`${outcome} turn and full trace survive reopening after reload`, async ({ page }) => {
    await page.route('https://openrouter.ai/api/v1/key', (route) =>
      route.fulfill({ json: { data: { limit: null, limit_remaining: null } } }),
    )
    await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
      if (outcome === 'failed')
        return route.fulfill({
          status: 402,
          json: {
            error: {
              code: 402,
              message: 'No credit',
              metadata: { limit_source: 'openrouter_credits' },
            },
          },
        })
      const code =
        outcome === 'answered'
          ? 'print("trace survives")\nawait FINAL_VAR("Saved answer", ["user-question"])'
          : 'import asyncio\nprint("waiting")\nawait asyncio.sleep(30)'
      return route.fulfill(
        openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
      )
    })
    await page.goto('/app/projects')
    const open = async () => {
      await page
        .getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true })
        .click()
      await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
    }
    await open()
    await page.getByLabel('OpenRouter API key').fill('test-key')
    await page.getByLabel('OpenRouter model ID').fill('test/model')
    await page.getByRole('button', { name: 'Privacy', exact: true }).click()
    await page.getByRole('button', { name: 'I agree', exact: true }).click()
    await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
    await page.getByLabel('Ask a question').fill(`Persist ${outcome}`)
    await page.getByRole('button', { name: 'Send', exact: true }).click()
    if (outcome === 'cancelled') {
      await expect(page.getByText('import asyncio', { exact: false }).first()).toBeVisible({
        timeout: 15000,
      })
      await page.getByRole('button', { name: 'Stop', exact: true }).click()
    }
    const read = () =>
      page.evaluate(async () => {
        const { loadDisplayTurns } = await import(
          new URL('/src/assistant/displayHistory.ts', location.href).href
        )
        const key = Object.keys(localStorage).find((key) => key.startsWith('hirmos_assistant_session:'))
        return key ? loadDisplayTurns(localStorage.getItem(key)!) : []
      })
    await expect.poll(async () => (await read()).length, { timeout: 20000 }).toBe(1)
    const before = await read()
    expect(before[0].outcome.kind).toBe(outcome)
    expect(before[0].trace.some((event: { kind: string }) => event.kind === 'budget')).toBe(true)
    await page.goto('/app/projects')
    await open()
    await expect(
      page.getByRole('heading', { name: `Persist ${outcome}`, exact: true }),
    ).toBeVisible()
    expect(await read()).toEqual(before)
    await expect(
      page
        .locator('article')
        .filter({ hasText: `Persist ${outcome}` })
        .getByRole('button', { expanded: false }),
    ).toBeVisible()
    await page.getByRole('button', { name: 'Clear conversation', exact: true }).click()
    await expect.poll(async () => (await read()).length).toBe(0)
    await page.goto('/app/projects')
    await open()
    await expect(
      page.getByRole('heading', { name: `Persist ${outcome}`, exact: true }),
    ).toHaveCount(0)
  })
}
