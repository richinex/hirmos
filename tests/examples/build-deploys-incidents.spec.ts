import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, chapter, createDag, createProject, exportBundle, identify, prepare, runDiscovery, runEstimator } from './support'

/** A weekly series with a planted lag structure: discovery, a lagged DAG, a total effect, a counterfactual. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the deploys and incidents example bundle', async ({ page }) => {
  test.setTimeout(1_500_000)
  const example = shippedExampleById('2d7f9b3c-4e81-4a5d-b6c3-9f0e1a2d7c48' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'time series', time: 'week', columns: 'all' })

  await runDiscovery(page, {
    // The first combobox in the lab is the maximum lag; four covers the planted lag of three.
    settle: async () => {
      await page.getByRole('combobox').first().click()
      await page.getByRole('option', { name: '4', exact: true }).click()
    },
    run: /^Run PCMCI/i,
  })

  await createDag(page, 'Deploys to incidents at lag three')
  await addArrow(page, 'deploys', 'incidents', 'Discovery reports a directed link from deploys to incidents at lag three.', 3)
  await addArrow(page, 'incidents', 'incidents', 'Incident counts carry momentum from the previous week.', 1)
  await addArrow(page, 'deploys', 'deploys', 'Deployment activity is autocorrelated from one week to the next.', 1)

  await identify(page, {
    graph: 'Deploys to incidents at lag three', treatment: 'deploys', outcome: 'incidents', mechanism: 'Observed choice',
    sentence: 'Deployment volume follows team activity; it is not assigned by the analyst.',
    consistency: 'One unit of deployment volume means the same activity in every week.',
    interference: 'Deployments in one week affect other weeks only through the recorded lagged arrows.',
  })

  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Temporal graph/ }).click()
  await page.getByRole('radio', { name: /CausalEffects total effect/ }).click()
  // Treatment lag, control value, treated value: the estimator's numeric fields in display order.
  const fields = page.locator('main input[type="number"]')
  await fields.nth(0).fill('3')
  await fields.nth(1).fill('0')
  await fields.nth(2).fill('1')
  await runEstimator(page, { run: /^Run causalEffects total effect/i })

  await chapter(page, /Counterfactuals/)
  await page.getByRole('radio', { name: /Dynamic SCM/ }).click()
  await page.getByRole('radio', { name: /Block-bootstrap interval/ }).click()
  await page.getByRole('radio', { name: /^Persistent/ }).click()
  // Control value, treated value, start week, horizon.
  const settings = page.locator('main input[type="number"]')
  await settings.nth(0).fill('0')
  await settings.nth(1).fill('1')
  await settings.nth(2).fill('200')
  await settings.nth(3).fill('20')
  await page.getByRole('button', { name: 'Run counterfactual' }).click()
  await expect(page.getByText('Current counterfactual').first()).toBeVisible({ timeout: 600_000 })

  await exportBundle(page, example)
})
