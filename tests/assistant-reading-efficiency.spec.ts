import { expect, test } from '@playwright/test'

test('compact views preserve assumptions and stored Python records can be reused without another read', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readStudy } = await import(new URL('/src/assistant/studyRead.ts', location.href).href)
    const { pythonSandbox } = await import(new URL('/src/assistant/sandbox.ts', location.href).href)
    const bundle = await (await fetch('/examples/ai-usage-intensity.hirmos.json')).json()
    const workflow = { ...bundle.project, kind: 'profiled' }
    const id = workflow.studies[0].id
    const summary = readStudy(workflow, { id })
    const full = readStudy(workflow, { id, view: 'full' })
    const assumptions = readStudy(workflow, { id, view: 'assumptions' })
    let reads = 0
    const runtime = await pythonSandbox(
      new AbortController().signal,
      async (_name: string, args: unknown) => {
        reads++
        return readStudy(workflow, args)
      },
    )
    try {
      const first = await runtime.executeReport(`s = await tool('study', {'id': '${id}'})`)
      const second = await runtime.executeReport(
        "print(next(a['statement'] for a in s['assumptions'] if a['id'] == 'no-unmeasured-confounding'))",
      )
      const third = await runtime.executeReport('del s')
      return {
        first,
        second,
        third,
        reads,
        summary,
        full,
        assumptions,
        summarySize: JSON.stringify(summary).length,
        fullSize: JSON.stringify(full).length,
      }
    } finally {
      runtime.close()
    }
  })
  expect(result.summary.view).toBe('summary')
  expect(result.summary.specification).not.toHaveProperty('graph')
  expect(result.summary.identifications[0]).not.toHaveProperty('record')
  expect(result.summary.assumptions).toEqual(result.full.assumptions)
  expect(result.assumptions.assumptions).toEqual(result.full.assumptions)
  expect(result.summarySize).toBeLessThan(3000)
  expect(result.fullSize).toBeGreaterThan(result.summarySize * 3)
  expect(result.first.locals.s).toBe('dict')
  expect(result.first.locals).not.toHaveProperty('tool')
  expect(result.second.locals.s).toBe('dict')
  expect(result.second.output).toContain('No unmeasured common cause')
  expect(result.third.locals).not.toHaveProperty('s')
  expect(result.reads).toBe(1)
  console.log(`Study read: ${result.summarySize} characters compact; ${result.fullSize} full.`)
})

test('a streamed draft is visible before it becomes executable code', async ({ page }) => {
  test.setTimeout(60000)
  await page.addInitScript(() => {
    const original = globalThis.fetch
    globalThis.fetch = async (input, init) => {
      const url = typeof input === 'string' ? input : input instanceof URL ? input.href : input.url
      if (!url.includes('openrouter.ai/api/v1/chat/completions')) return original(input, init)
      const encode = (content: string) =>
        new TextEncoder().encode(
          `data: ${JSON.stringify({
            id: 'draft-test',
            object: 'chat.completion.chunk',
            created: 1,
            model: 'test/model',
            choices: [{ index: 0, delta: { content }, finish_reason: null }],
          })}\n\n`,
        )
      return new Response(
        new ReadableStream({
          start(controller) {
            controller.enqueue(
              encode('```python\nawait FINAL_VAR("Draft completed.", ["user-question"])'),
            )
            Object.assign(globalThis, { finishDraft: () => controller.enqueue(encode('\n```\n')) })
          },
        }),
        { headers: { 'Content-Type': 'text/event-stream' } },
      )
    }
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).first().click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Say when the draft is complete.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Draft [1]:' })).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toHaveCount(0)
  await page
    .getByRole('button', { name: 'Draft [1]:' })
    .locator('..')
    .screenshot({ path: '/tmp/hirmos-streamed-draft.png' })
  await page.evaluate(() => (globalThis as unknown as { finishDraft: () => void }).finishDraft())
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Draft completed.',
  )
})
