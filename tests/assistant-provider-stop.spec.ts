import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('provider-side fence stop keeps generation ID, usage and executable code without the closing fence', async ({
  page,
}) => {
  let stop: unknown
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    stop = route.request().postDataJSON().stop
    const text = 'I will check this now.\n```python\nprint(42)\n```\nDo not execute this prose.'
    const positions = (stop as string[])
      .map((sequence) => text.indexOf(sequence))
      .filter((index) => index >= 0)
    const response = openRouterStream({
      choices: [
        {
          message: { content: positions.length ? text.slice(0, Math.min(...positions)) : text },
          finish_reason: 'stop',
        },
      ],
    })
    const usage = `data: ${JSON.stringify({ id: 'gen-saved', choices: [], usage: { prompt_tokens: 10, completion_tokens: 8, total_tokens: 18, cost: 0.001 } })}\n\n`
    return route.fulfill({
      ...response,
      body: response.body.replace('data: [DONE]', usage + 'data: [DONE]'),
    })
  })
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { openRouterModel } = await import(new URL('/src/assistant/model.ts', location.href).href)
    const { extractFirstCodeBlock } = await import(
      new URL('/src/assistant/octopus/rlm/parseCode.ts', location.href).href
    )
    const reports: unknown[] = []
    const reply = await openRouterModel('test-key', 'test/model', () => {}, undefined, {
      usage: (r: unknown) => reports.push(r),
      failure: () => {},
    })({
      system: '',
      messages: [{ role: 'user', content: 'Print' }],
      maxTokens: 16384,
      onText: () => {},
    })
    return { reports, code: extractFirstCodeBlock(reply.text).code, stop: reply.stop_reason }
  })
  expect(stop).toEqual(['\n```\n'])
  expect(result.code).toBe('print(42)')
  expect(result.stop).toBe('end_turn')
  expect(result.reports).toEqual([
    expect.objectContaining({ generationId: 'gen-saved', kind: 'reported', costUsd: 0.001 }),
  ])
})
