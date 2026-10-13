import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('Panel DiD proposals validate method shapes and column membership', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readEstimatorConfiguration } = await import(
      new URL('/src/assistant/estimatorConfiguration.ts', location.href).href
    )
    const { panelConfigurationSchema } = await import(
      new URL('/src/assistant/panelConfiguration.ts', location.href).href
    )
    const { z } = await import(new URL('/node_modules/.vite/deps/zod.js', location.href).href)
    const conventional = {
      kind: 'panel-intervention',
      primary: 'did',
      placeboReplications: 20,
      seed: 7,
      inference: { kind: 'bootstrap', replications: 20, seed: 7 },
    }
    const adjusted = {
      kind: 'panel-intervention',
      primary: 'adjusted',
      covariates: ['x'],
      specification: { kind: 'regression' },
    }
    const configs = [
      conventional,
      adjusted,
      { kind: 'panel-intervention', primary: 'syntheticDid', placeboReplications: 20, seed: 7 },
      {
        ...adjusted,
        specification: {
          kind: 'doublyRobust',
          folds: 2,
          seed: 7,
          trimming: 0.01,
          normalization: 'in-sample',
        },
      },
      {
        kind: 'panel-intervention',
        primary: 'sunAbraham',
        referencePeriods: [-1],
        referenceCohorts: [],
        confidence: 0.95,
      },
    ]
    return {
      valid: configs.map((c) => readEstimatorConfiguration(c, [{ id: 'x', name: 'X' }]).ok),
      invalid: [
        { ...conventional, placeboReplications: 2001 },
        { ...conventional, seed: -1 },
        { ...adjusted, covariates: ['missing'] },
        { ...adjusted, covariates: ['x', 'x'] },
      ].map((c) => readEstimatorConfiguration(c, [{ id: 'x', name: 'X' }]).ok),
      schema: Boolean(z.toJSONSchema(panelConfigurationSchema)),
    }
  })
  expect(result.valid).toEqual([true, true, true, true, true])
  expect(result.invalid).toEqual([false, false, false, false])
  expect(result.schema).toBe(true)
})

test('reviewed Panel DiD settings use the existing controls without starting a run', async ({
  page,
}) => {
  const code =
    'p = await tool("project")\ns = await tool("panel_did_schema")\nc = {"kind":"panel-intervention","primary":"did","placeboReplications":20,"seed":17,"inference":{"kind":"bootstrap","replications":20,"seed":17}}\nr = await tool("propose_estimator", {"estimator":"panel-intervention","configuration":c,"reason":"Review 20 bootstrap replications with seed 17."})\nassert r["kind"] == "awaiting-user-approval", r\nawait FINAL_VAR("Settings proposed, not run.", ["project-summary"])'
  await page.route('https://openrouter.ai/api/v1/key', (route) =>
    route.fulfill({ json: { data: { limit: null, limit_remaining: null } } }),
  )
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await page
    .getByRole('navigation', { name: 'Workspace sections' })
    .getByRole('button', { name: /Estimation/ })
    .click()
  await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
  await page.getByLabel('OpenRouter API key').fill('test-key')
  await page.getByLabel('OpenRouter model ID').fill('test/model')
  await page.getByRole('button', { name: 'Privacy', exact: true }).click()
  await page.getByRole('button', { name: 'I agree', exact: true }).click()
  await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
  await page
    .getByLabel('Ask a question')
    .fill('Propose conventional DiD with 20 bootstrap replications and seed 17.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  const card = page.getByRole('region', { name: 'Estimator proposal' })
  await expect(card).toContainText('17', { timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied.')
  await expect(page.getByText('Runs (2)', { exact: true })).toBeVisible()
})
