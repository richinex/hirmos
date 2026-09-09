import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { chapter, choose, createProject, exportBundle, prepare } from './support'

/** ComparisonSurv's crossdata: two groups whose event-free curves cross, compared through time 2. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the crossing survival curves example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('e2c7a5f1-9d38-4b6e-b7a4-1f6e8c3d5a92' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: ['time', 'status', 'group'] })

  await chapter(page, /Survival analysis/)
  await page.getByRole('radio', { name: 'Compare groups' }).click()
  await choose(page, 'Duration', 'time')
  await choose(page, 'Event · 1 observed, 0 censored', 'status')
  await choose(page, 'Group · 0 or 1', 'group')
  await page.getByRole('spinbutton', { name: 'Compare through time' }).fill('2')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()
  await expect(page.getByRole('heading', { name: 'Group 1 compared with group 0' }).first()).toBeVisible({ timeout: 300_000 })

  await exportBundle(page, example)
})
