import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, chapter, createDag, createProject, exportBundle, identify } from './support'

/** The March adoption cohort against the never-adopters: a balanced panel and the DID family. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the March cohort AI adoption example bundle', async ({ page }) => {
  test.setTimeout(600_000)
  const example = shippedExampleById('a9e4c2d7-8b31-4f5e-b6c0-3d7a9e2f5b82' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  // Panel DID is offered only for a panel prepared with explicit unit and time keys.
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await page.getByLabel('Unit column').click()
  await page.getByRole('option', { name: 'team', exact: true }).click()
  await page.getByLabel('Time column').click()
  await page.getByRole('option', { name: 'month', exact: true }).click()
  await expect(page.getByRole('checkbox', { name: 'adopted', exact: true })).toBeVisible()
  for (const column of ['adopted', 'bugs_per_kloc']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByText(/Prepared panel/)).toBeVisible({ timeout: 120_000 })

  await createDag(page, 'March cohort adoption')
  await addArrow(page, 'adopted', 'bugs_per_kloc', 'The three March teams wrote their code with the assistant from month 13 onward.')

  await identify(page, {
    graph: 'March cohort adoption', treatment: 'adopted', outcome: 'bugs_per_kloc', mechanism: 'Policy change',
    sentence: 'Teams A, B and C switched the AI assistant on in month 13; teams F, G and H never did.',
  })

  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).click()
  // The panel opens on this estimator for a prepared panel; the click is only needed when it does not.
  const panelDid = page.getByRole('radio', { name: /Panel difference-in-differences/ })
  if (!(await panelDid.isChecked())) await panelDid.click()
  await page.getByRole('button', { name: /^Run panel difference-in-differences/i }).first().click()
  await expect(page.getByText('Current estimate').first()).toBeVisible({ timeout: 300_000 })

  await exportBundle(page, example)
})
