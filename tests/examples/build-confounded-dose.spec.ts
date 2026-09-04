import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, choose, createDag, createProject, exportBundle, prepare } from './support'

/** Severity sets the dose and moves recovery on its own; the do-query reads recovery with dose set. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the confounded dose example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('1b9e6c3d-8a52-4d17-b3e4-6c2f9a5d1e78' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: ['severity', 'dose', 'recovery'] })

  await createDag(page, 'Severity confounds dose')
  await addArrow(page, 'severity', 'dose', 'Sicker patients are prescribed a larger dose, so severity sets the dose.')
  await addArrow(page, 'severity', 'recovery', 'Severity lowers recovery whatever dose is given.')
  await addArrow(page, 'dose', 'recovery', 'The dose is given in order to raise recovery.')

  // The same query at two state budgets, to show the contrast is not a per-unit effect.
  await page.getByRole('button', { name: /Intervene/ }).click()
  await choose(page, 'Variable to set', 'dose')
  await choose(page, 'Variable to read', 'recovery')
  await choose(page, 'State budget', '2')
  await page.getByRole('button', { name: 'Evaluate intervention' }).click()
  const queries = page.getByRole('list', { name: 'Intervention queries' })
  await expect(queries).toBeVisible({ timeout: 300_000 })
  await expect(queries.getByRole('listitem')).toHaveCount(1, { timeout: 300_000 })
  await choose(page, 'State budget', '4')
  await page.getByRole('button', { name: 'Evaluate intervention' }).click()
  await expect(queries.getByRole('listitem')).toHaveCount(2, { timeout: 300_000 })

  await exportBundle(page, example)
})
