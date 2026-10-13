import { openRouterStream } from './openRouterStream'
import { expect, test } from '@playwright/test'

test('an approved CodeAct run saves a real estimate and returns its summary', async ({ page }) => {
  test.setTimeout(90000)
  let requests = 0
  const code =
    'p = await tool("project")\nresult = await tool("run_analysis", {"key":"estimation", "subject":p["runs"]["estimation"]["study"]["subject"], "reason":"Run the current reviewed specification."})\nprint(result)\nawait tool("finish", {"answer":"Saved run: " + result.get("runId", result["kind"]), "evidence":["approved-run"]})'
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) => {
    requests++
    return route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Estimation/ })
    .click()
  await expect(page.getByRole('button', { name: /^Run adjusted linear regression/ })).toBeEnabled({
    timeout: 30000,
  })
  const before = await page.getByRole('heading', { name: /^Runs \(/ }).innerText()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Run the current specification after I approve it.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const approval = page.getByRole('region', { name: 'Analysis approval' })
  await expect(approval).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
  await approval.getByRole('button', { name: 'Run and share summary' }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText('Saved run:', {
    timeout: 30000,
  })
  await expect(approval).toContainText('Saved run')
  await expect(page.getByRole('heading', { name: /^Runs \(/ })).not.toHaveText(before)
  expect(requests).toBe(1)
  await page.screenshot({ path: '/tmp/hirmos-approved-run.png' })
})

test('an estimator proposal changes the real controls only after approval', async ({ page }) => {
  test.setTimeout(90000)
  const code = [
    'p = await tool("project")',
    'choice = next(c for c in p["estimation"]["choices"] if c["id"] != p["estimation"]["selected"] and c["eligibility"]["kind"] != "refused")',
    'print(await tool("propose_estimator", {"estimator":choice["id"], "reason":"Review this alternative and its assumptions before running it."}))',
    'await tool("finish", {"answer":"I have proposed a selection, not run an estimate.","evidence":["project-summary"]})',
  ].join('\n')
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Estimation/ })
    .click()
  await expect(page.getByRole('button', { name: /^Run adjusted linear regression/ })).toBeEnabled({
    timeout: 30000,
  })
  const before = await page.getByRole('heading', { name: /^Runs \(/ }).innerText()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByLabel('Ask a question').fill('Suggest another estimator.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const proposal = page.getByRole('region', { name: 'Estimator proposal' })
  await expect(proposal).toBeVisible({ timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
  await expect(page.getByRole('radio', { name: /^Adjusted linear regression/ })).toBeChecked()
  const name = (await proposal.getByRole('heading').innerText()).replace(/^Select /, '')
  await proposal.getByRole('button', { name: 'Apply selection' }).click()
  await expect(proposal).toContainText('Selection applied.')
  await expect(
    page.getByRole('button', {
      name: new RegExp('^Run ' + name.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i'),
    }),
  ).toBeVisible()
  await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
  await page.screenshot({ path: '/tmp/hirmos-approved-selection.png' })
})

test('proposal validation refuses unknown, prohibited and stale choices', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createAssistantBridge, proposeSelection } = await import(
      new URL('/src/assistant/applicationTools.tsx', location.href).href
    )
    let selections = 0
    const bridge = createAssistantBridge()
    const capability = {
      revision: 'first',
      inspection: {
        study: { treatment: 'T', outcome: 'Y', target: 'ate' },
        selected: 'backdoor-linear-regression',
        choices: [
          {
            id: 'backdoor-linear-regression',
            name: 'Regression',
            eligibility: { kind: 'eligible', satisfied: [] },
          },
          {
            id: 'poisson-glm',
            name: 'Poisson',
            eligibility: { kind: 'refused', satisfied: [], unresolved: [], violations: [] },
          },
        ],
      },
      select: () => {
        selections++
      },
    }
    const close = bridge.publish(capability)
    const proposal = proposeSelection(capability, {
      estimator: 'backdoor-linear-regression',
      reason: 'Example',
    })
    const unknown = proposeSelection(capability, { estimator: 'invented', reason: 'Example' })
    const refused = proposeSelection(capability, { estimator: 'poisson-glm', reason: 'Example' })
    bridge.publish({ ...capability, revision: 'second' })
    close()
    const stale = bridge.apply(proposal.value)
    return {
      unknown: unknown.error,
      refused: refused.error,
      stale: stale.error,
      selections,
      current: bridge.store.getState().current.revision,
    }
  })
  expect(result).toEqual({
    unknown: 'invalid',
    refused: { kind: 'requirements-not-met', violations: [] },
    stale: 'stale',
    selections: 0,
    current: 'second',
  })
})

test('CodeAct reads approved metadata and delegates without changing the project', async ({
  page,
}) => {
  test.setTimeout(180000)
  const requests: unknown[] = []
  await page.route('https://openrouter.ai/api/v1/chat/completions', async (route) => {
    const body = route.request().postDataJSON()
    requests.push(body)
    const child = body.messages[0].content.includes('You cannot delegate.')
    const code = child
      ? 'print(await tool("project"))\nawait tool("finish", {"answer":"No data rows were supplied.","evidence":["project-summary"]})'
      : 'summary = await tool("project")\nchecks = await tool("delegate", {"tasks":["Check available metadata"]})\nawait tool("finish", {"answer":"The project needs data before analysis. " + checks[0]["answer"],"evidence":["project-summary"]})'
    await route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    )
  })
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key-not-real')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByLabel('Ask a question').fill('What is available?')
  await expect(page.getByRole('button', { name: 'Send', exact: true })).toBeDisabled()
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'No data rows were supplied.',
    { timeout: 150000 },
  )
  expect(requests).toHaveLength(2)
  await page.getByRole('button', { name: /1 block, 1 subagent/ }).click()
  await expect(page.getByText('Check available metadata', { exact: true })).toBeVisible()
  await expect(
    page.getByRole('button', { name: /Check available metadata.*1 block/ }),
  ).toBeVisible()
  expect(JSON.stringify(requests)).not.toContain('test-key-not-real')
  expect(await page.locator('iframe[sandbox="allow-scripts"]').count()).toBe(0)
  await page.screenshot({ path: '/tmp/hirmos-assistant-result.png' })
  await page
    .getByRole('complementary', { name: 'Assistant' })
    .getByRole('button', { name: 'Close the assistant' })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await expect(page.getByLabel('OpenRouter API key')).toHaveValue('')
})

test('Python cannot read app storage or reach an external endpoint', async ({ page }) => {
  await page.goto('/app')
  let externalRequests = 0
  await page.route('https://example.com/**', (route) => {
    externalRequests++
    return route.abort()
  })
  const output = await page.evaluate(async () => {
    // @ts-expect-error Browser-side Vite module URL.
    const { pythonSandbox } = await import('/src/assistant/sandbox.ts')
    const runtime = await pythonSandbox(AbortSignal.timeout(20000), async () => ({}))
    try {
      return await runtime.execute(`
from js import indexedDB, fetch
try:
    indexedDB.open('hirmos-private')
    print('storage exposed')
except Exception:
    print('storage blocked')
try:
    await fetch('https://example.com/private')
    print('network exposed')
except Exception:
    print('network blocked')
`)
    } finally {
      runtime.close()
    }
  })
  expect(output).toContain('storage blocked')
  expect(output).toContain('network blocked')
  expect(externalRequests).toBe(0)
})

test('cancellation stops an unbounded Python loop without freezing the interface', async ({
  page,
}) => {
  await page.goto('/app')
  const outcome = await page.evaluate(async () => {
    // @ts-expect-error Browser-side Vite module URL.
    const { pythonSandbox } = await import('/src/assistant/sandbox.ts')
    const controller = new AbortController()
    const runtime = await pythonSandbox(controller.signal, async () => ({}))
    const timer = setTimeout(() => controller.abort(), 100)
    try {
      await runtime.execute('while True: pass')
      return 'unexpected completion'
    } catch {
      return 'cancelled'
    } finally {
      clearTimeout(timer)
      runtime.close()
    }
  })
  expect(outcome).toBe('cancelled')
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await expect(page.getByLabel('Ask a question')).toBeVisible()
  await expect(page.locator('iframe[sandbox="allow-scripts"]')).toHaveCount(0)
})
