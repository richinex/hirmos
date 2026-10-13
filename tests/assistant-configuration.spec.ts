import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('configuration proposals reject invalid settings and stale approvals', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createAssistantBridge, proposeSelection } = await import(
      new URL('/src/assistant/applicationTools.tsx', location.href).href
    )
    const bridge = createAssistantBridge()
    const configuration = {
      kind: 'backdoor-linear-regression',
      errors: { kind: 'hc1' },
      fixedEffects: { kind: 'none' },
      level: 0.95,
    }
    let applied = 0
    let refused = false
    const capability = {
      revision: 'one',
      inspection: {
        study: { treatment: 'T', outcome: 'Y', target: 'ate' },
        selected: configuration.kind,
        choices: [
          {
            id: configuration.kind,
            name: 'Regression',
            eligibility: { kind: 'eligible', satisfied: [] },
          },
        ],
        configuration: { current: configuration, columns: [{ id: 'group', name: 'Group' }] },
      },
      select: () => {
        throw new Error('Configuration must not select a different method')
      },
      configure: () => {
        applied++
      },
      evaluate: () =>
        refused
          ? { kind: 'refused', violations: [], unresolved: [], satisfied: [] }
          : { kind: 'eligible', satisfied: [] },
    }
    bridge.publish(capability)
    const propose = (settings: unknown) =>
      proposeSelection(capability, {
        estimator: configuration.kind,
        reason: 'Review settings.',
        configuration: settings,
      })
    const invalid = [
      { ...configuration, extra: true },
      { ...configuration, level: 2 },
      { kind: 'doubly-robust', model: 'newton', maxIter: 1000, uncertainty: { kind: 'none' } },
      { ...configuration, errors: { kind: 'cluster', column: 'missing', name: 'Group' } },
      { ...configuration, errors: { kind: 'cluster', column: 'group', name: 'Wrong' } },
      {
        ...configuration,
        errors: { kind: 'hac' },
        fixedEffects: { kind: 'unit', column: 'group', name: 'Group' },
      },
    ].map((settings) => propose(settings).ok)
    const proposed = propose(configuration)
    const before = applied
    const first = bridge.apply(proposed.value)
    refused = true
    const rejected = bridge.apply(proposed.value)
    bridge.publish({ ...capability, revision: 'two' })
    const stale = bridge.apply(proposed.value)
    return {
      invalid,
      before,
      applied,
      first: first.ok,
      rejected: rejected.error,
      stale: stale.error,
    }
  })
  expect(result).toEqual({
    invalid: [false, false, false, false, false, false],
    before: 0,
    applied: 1,
    first: true,
    rejected: { kind: 'requirements-not-met', violations: [] },
    stale: 'stale',
  })
})

test('reviewed regression settings update the controls without running an estimate', async ({
  page,
}) => {
  const code = [
    'p = await tool("project")',
    'c = p["estimation"]["configuration"]["current"]',
    'c["errors"] = {"kind":"hc1"}',
    'r = await tool("propose_estimator", {"estimator":c["kind"],"configuration":c,"reason":"Review robust uncertainty."})',
    'assert r["kind"] == "awaiting-user-approval", r',
    'await FINAL_VAR("Settings proposed; no analysis was run.", ["project-summary"])',
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
  await page.getByLabel('Ask a question').fill('Propose HC1 standard errors.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const proposal = page.getByRole('region', { name: 'Estimator proposal' })
  await expect(proposal).toContainText('HC1', { timeout: 30000 })
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
  await proposal.screenshot({ path: '/tmp/hirmos-regression-settings.png' })
  await proposal.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
  await expect(page.getByRole('radio', { name: /HC1/ })).toBeChecked()
})
