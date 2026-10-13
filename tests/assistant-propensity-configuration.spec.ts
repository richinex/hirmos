import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

test('propensity configuration preserves method boundaries and shared parameter bounds', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { readEstimatorConfiguration } = await import(
      new URL('/src/assistant/estimatorConfiguration.ts', location.href).href
    )
    const boosted = {
      learningRate: [0.1],
      maxDepth: [2],
      nEstimators: [100],
      splits: 3,
      seed: 7,
      scoring: 'cross-fitted',
    }
    const uncertainty = { kind: 'bootstrap', rounds: 20, seed: 11, level: 0.95 }
    const weighting = {
      kind: 'propensity-weighting',
      model: 'newton',
      maxIter: 1000,
      boosted,
      scale: 'stabilized',
      uncertainty,
    }
    const matching = { kind: 'propensity-matching', model: 'boosted', maxIter: 1000, boosted }
    const dr = { kind: 'doubly-robust', model: 'newton', maxIter: 1000, uncertainty }
    const gps = { kind: 'continuous-gps', scale: 'inverseDensity', uncertainty }
    return {
      valid: [
        weighting,
        matching,
        dr,
        gps,
        { ...weighting, model: 'boosted', uncertainty: { kind: 'none' } },
      ].map((c) => readEstimatorConfiguration(c, []).ok),
      invalid: [
        { ...dr, model: 'boosted' },
        { ...matching, uncertainty },
        { ...weighting, model: 'boosted' },
        { ...dr, maxIter: 0 },
        { ...gps, scale: 'inverseProbability' },
        { ...dr, uncertainty: { ...uncertainty, rounds: 2001 } },
        { ...dr, uncertainty: { ...uncertainty, seed: -1 } },
        { ...dr, uncertainty: { ...uncertainty, level: 1 } },
        { ...matching, boosted: { ...boosted, maxDepth: [17] } },
        { ...matching, boosted: { ...boosted, learningRate: [] } },
      ].map((c) => readEstimatorConfiguration(c, []).ok),
    }
  })
  expect(result.valid).toEqual(Array(5).fill(true))
  expect(result.invalid).toEqual(Array(10).fill(false))
})

for (const method of [
  'Inverse propensity weighting',
  'Propensity-score matching',
  'Doubly robust estimation',
]) {
  test(`reviewed settings reach ${method} without running it`, async ({ page }) => {
    const code = [
      'p = await tool("project")',
      'c = p["estimation"]["configuration"]["current"]',
      'c["model"] = "lbfgsb"',
      'c["maxIter"] = 1200',
      'r = await tool("propose_estimator", {"estimator": c["kind"], "configuration": c, "reason": "Review the logistic solver settings."})',
      'assert r["kind"] == "awaiting-user-approval", r',
      'await FINAL_VAR("Settings proposed, not applied.", ["project-summary"])',
    ].join('\n')
    await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
      route.fulfill(
        openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
      ),
    )
    await page.goto('/app/projects')
    await page
      .getByRole('button', { name: 'Open NSW job training and 1978 earnings', exact: true })
      .click()
    await page
      .getByRole('navigation', { name: 'Workspace sections' })
      .getByRole('button', { name: /Estimation/ })
      .click()
    await page.getByRole('radio', { name: /Propensity-score estimators/ }).click()
    await page.getByRole('radio', { name: new RegExp('^' + method) }).click()
    const before = await page.getByRole('heading', { name: /^Runs \(/ }).innerText()
    await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
    await page.getByLabel('OpenRouter API key').fill('test-key')
    await page.getByLabel('OpenRouter model ID').fill('test/model')
    await page.getByRole('button', { name: 'Privacy', exact: true }).click()
    await page.getByRole('button', { name: 'I agree', exact: true }).click()
    await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
    await page.getByLabel('Ask a question').fill('Propose L-BFGS-B with 1200 iterations.')
    await page.getByRole('button', { name: 'Send', exact: true }).click()
    const proposal = page.getByRole('region', { name: 'Estimator proposal' })
    await expect(proposal).toContainText('L-BFGS-B', { timeout: 30000 })
    await expect(page.getByRole('region', { name: 'Assistant answer' })).toBeVisible()
    await expect(page.getByRole('radio', { name: 'Newton', exact: true })).toBeChecked()
    await proposal.screenshot({ path: `/tmp/hirmos-${method.replaceAll(' ', '-')}-settings.png` })
    await proposal.getByRole('button', { name: 'Apply settings' }).click()
    await expect(page.getByRole('radio', { name: 'L-BFGS-B', exact: true })).toBeChecked()
    await expect(
      page.getByRole('spinbutton', { name: 'Iteration limit', exact: true }),
    ).toHaveValue('1200')
    await expect(page.getByRole('heading', { name: /^Runs \(/ })).toHaveText(before)
  })
}
