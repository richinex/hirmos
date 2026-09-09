import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { chapter, choose, createProject, exportBundle, prepare } from './support'

/** flexsurv's bc data: 686 node-positive breast cancer patients, time to death in years, right-censored. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the breast cancer survival example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('b8d4e1a7-2c69-4f3b-8e15-4a7c9d2f6b31' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: ['censrec', 'recyrs'] })

  await chapter(page, /Survival analysis/)
  await choose(page, 'Duration', 'recyrs')
  await choose(page, 'Event · 1 observed, 0 censored', 'censrec')
  await page.getByRole('spinbutton', { name: 'Prediction horizon' }).fill('7')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Weibull AFT' }).first()).toBeVisible({ timeout: 300_000 })
  await expect(page.getByText('686 observations · 299 events').first()).toBeVisible()

  await exportBundle(page, example)
})
