import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, chapter, choose, createDag, createProject, exportBundle, identify } from './support'

/** One rollout date, a legacy portfolio as the untouched control, and causal impact against it. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the company-wide AI adoption example bundle', async ({ page }) => {
  test.setTimeout(600_000)
  const example = shippedExampleById('7c2f1b3e-5a64-4d1e-9b0a-2e6f8c1d4a71' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page, 'Time column', 'month')
  for (const column of ['ai_active', 'legacy_bugs_per_kloc', 'bugs_per_kloc']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByText(/Prepared time series/)).toBeVisible({ timeout: 120_000 })

  await createDag(page, 'AI rollout and code quality')
  await addArrow(page, 'ai_active', 'bugs_per_kloc', 'The rollout changed how product code is written from the switch-on month.')
  await addArrow(page, 'legacy_bugs_per_kloc', 'bugs_per_kloc', 'The legacy portfolio shares scanner rules and review policy, so it tracks the same background conditions.')

  await identify(page, {
    graph: 'AI rollout and code quality', treatment: 'ai_active', outcome: 'bugs_per_kloc', mechanism: 'Policy change',
    sentence: 'Engineering leadership enabled the AI coding assistant for every product team in month 25.',
  })

  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).click()
  await page.getByRole('radio', { name: /^Causal impact/ }).click()
  await page.getByRole('button', { name: /^Run causal impact/i }).first().click()
  await expect(page.getByText('Current estimate').first()).toBeVisible({ timeout: 300_000 })

  await exportBundle(page, example)
})
