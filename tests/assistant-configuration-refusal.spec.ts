import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('the agent receives the time-series requirement when classical errors are refused', async ({
  page,
}) => {
  const code = [
    'p = await tool("project")',
    'c = p["estimation"]["configuration"]["current"]',
    'c["errors"] = {"kind":"classical"}',
    'r = await tool("propose_estimator", {"estimator":c["kind"],"configuration":c,"reason":"Check classical errors."})',
    'assert r["kind"] == "refused", r',
    'assert any(v["caveat"]["id"] == "linear-serial-dependence" for v in r["requirements"]), r',
    'await FINAL_VAR(r["reason"], ["project-summary"])',
  ].join('\n')
  await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
    route.fulfill(
      openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
    ),
  )
  await page.goto('/app/projects')
  await page
    .getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true })
    .click()
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
  await page.getByLabel('Ask a question').fill('Check classical errors.')
  await page.getByRole('button', { name: 'Send', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
    'Choose the HAC interval or an ARMA error process.',
    { timeout: 30000 },
  )
  await expect(page.getByRole('region', { name: 'Estimator proposal' })).toHaveCount(0)
})
