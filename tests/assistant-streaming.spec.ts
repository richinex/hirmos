import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('streaming aborts at the first split Python fence and retains reasoning without displaying it', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { openRouterModel } = await import(new URL('/src/assistant/model.ts', location.href).href)
    const original = globalThis.fetch
    let wireSignal: AbortSignal | null = null
    let payload: { stream?: boolean } = {}
    let claims = 0
    const visible: string[] = []
    const reports: unknown[] = []
    globalThis.fetch = async (_input, init) => {
      wireSignal = init?.signal ?? null
      payload = JSON.parse(String(init?.body))
      const chunks = [
        { reasoning: 'private deliberation' },
        { content: '```py\nprint(' },
        { content: '1)\n`' },
        { content: '``\n' },
      ]
      let index = 0
      return new Response(
        new ReadableStream({
          pull(controller) {
            if (index === chunks.length) return
            const chunk = {
              id: 'test',
              object: 'chat.completion.chunk',
              created: 1,
              model: 'test/model',
              choices: [{ index: 0, delta: chunks[index++], finish_reason: null }],
            }
            controller.enqueue(new TextEncoder().encode(`data: ${JSON.stringify(chunk)}\n\n`))
          },
        }),
        { headers: { 'Content-Type': 'text/event-stream' } },
      )
    }
    try {
      const reply = await openRouterModel('test-key', 'test/model', () => claims++, undefined, {
        usage: (report: unknown) => reports.push(report),
        failure: () => {},
      })({
        system: 'Return Python.',
        messages: [{ role: 'user', content: 'Print one.' }],
        maxTokens: 100,
        signal: new AbortController().signal,
        onText: (text: string) => visible.push(text),
      })
      return {
        reply,
        visible,
        claims,
        streaming: payload.stream,
        reports,
        aborted: (wireSignal as AbortSignal | null)?.aborted,
      }
    } finally {
      globalThis.fetch = original
    }
  })
  expect(result.streaming).toBe(true)
  expect(result.claims).toBe(1)
  expect(result.aborted).toBe(true)
  expect(result.reports).toEqual([
    expect.objectContaining({ generationId: 'test', kind: 'unavailable', costUsd: null }),
  ])
  expect(result.reply.stop_reason).toBe('end_turn')
  expect(result.reply.text).toBe('```py\nprint(1)\n```\n')
  expect(result.reply.content).toContainEqual({ type: 'reasoning', text: 'private deliberation' })
  expect(result.visible.length).toBeGreaterThan(1)
  expect(result.visible.join('')).not.toContain('private deliberation')
  expect(result.reply.usage).toBeUndefined()
})

test('truncated replies remain truncated and provider errors do not burn retries', async ({
  page,
}) => {
  let calls = 0
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    calls++
    return calls === 1
      ? route.fulfill(
          openRouterStream({
            choices: [{ message: { content: '```python\nprint(' }, finish_reason: 'length' }],
          }),
        )
      : route.fulfill({ status: 401, json: { error: { message: 'Invalid test key', code: 401 } } })
  })
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { openRouterModel } = await import(new URL('/src/assistant/model.ts', location.href).href)
    const call = openRouterModel('test-key', 'test/model', () => {})
    const request = {
      system: 'Python',
      messages: [{ role: 'user', content: 'Print' }],
      maxTokens: 100,
      onText: () => {},
    }
    const truncated = await call(request)
    let failed = false
    try {
      await call(request)
    } catch {
      failed = true
    }
    return { truncated, failed }
  })
  expect(result.truncated.stop_reason).toBe('max_tokens')
  expect(result.failed).toBe(true)
  expect(calls).toBe(2)
})

test('cancelling a request never returns an executable reply', async ({ page }) => {
  await page.goto('/app')
  const cancelled = await page.evaluate(async () => {
    const { openRouterModel } = await import(new URL('/src/assistant/model.ts', location.href).href)
    const original = globalThis.fetch
    const controller = new AbortController()
    globalThis.fetch = async (_input, init) => {
      const signal = init?.signal
      return new Promise<Response>((_resolve, reject) => {
        signal?.addEventListener('abort', () => reject(signal.reason), { once: true })
        controller.abort(new DOMException('Test cancellation', 'AbortError'))
      })
    }
    try {
      await openRouterModel('test-key', 'test/model', () => {})({
        system: 'Python',
        messages: [{ role: 'user', content: 'Wait for cancellation.' }],
        maxTokens: 100,
        signal: controller.signal,
        onText: () => {},
      })
      return false
    } catch {
      return controller.signal.aborted
    } finally {
      globalThis.fetch = original
    }
  })
  expect(cancelled).toBe(true)
})
