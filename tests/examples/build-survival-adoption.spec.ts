import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { chapter, createProject, exportBundle, prepare } from './support'

/** The generated feature-adoption panel: start-stop rows, a covariate that switches on mid-spell, Weibull PH. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the feature adoption survival example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('6a3f2c9e-7b14-4d58-a2e6-9c5d1f8b3e07' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: ['start', 'stop', 'event', 'feature_active', 'experience', 'complexity'] })

  await chapter(page, /Survival analysis/)
  await page.getByRole('radio', { name: 'Start–stop' }).click()
  for (const covariate of ['feature_active', 'experience', 'complexity']) await page.getByRole('checkbox', { name: covariate, exact: true }).check()
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('36')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Weibull PH' }).first()).toBeVisible({ timeout: 300_000 })
  await expect(page.getByText('455 rows · 132 events').first()).toBeVisible()

  await exportBundle(page, example)
})
