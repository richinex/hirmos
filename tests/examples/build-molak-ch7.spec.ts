import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, chapter, createDag, createProject, exportBundle, identify, prepare, runEstimator } from './support'

/** Molak's four steps on his simulated process: model, identify, estimate twice, refute. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

const EDGES: ReadonlyArray<readonly [string, string, string]> = [
  ['S', 'Q', 'Simulated: S enters the equation for Q.'],
  ['S', 'Y', 'Simulated: S enters the equation for Y.'],
  ['Q', 'X', 'Simulated: Q enters the equation for X.'],
  ['Q', 'Y', 'Simulated: Q enters the equation for Y.'],
  ['X', 'P', 'Simulated: X enters the equation for P.'],
  ['Y', 'P', 'Simulated: Y enters the equation for P.'],
  ['X', 'Y', 'Simulated: X enters the equation for Y with coefficient 0.7.'],
]

test('build the Molak chapter 7 example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('9c5b2e7a-1d38-4f64-a2b9-7e4c1f8d3a25' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: 'all' })

  await createDag(page, 'Simulated five-variable process')
  for (const [cause, effect, why] of EDGES) await addArrow(page, cause, effect, why)

  await identify(page, {
    graph: 'Simulated five-variable process', treatment: 'X', outcome: 'Y', mechanism: 'Observed choice',
    sentence: 'X is generated from Q and its own noise in the simulated process; nothing assigns it.',
  })

  await runEstimator(page, { run: /^Run adjusted linear regression/ })
  await runEstimator(page, { estimator: { value: 'dml-plr' }, run: /^Run double machine learning/i, done: /Double machine learning, partially linear/ })

  await chapter(page, /Sensitivity/)
  await page.getByRole('button', { name: 'Run perturbation and residual probes' }).click()
  await expect(page.getByText('Placebo treatment').first()).toBeVisible({ timeout: 300_000 })

  await exportBundle(page, example)
})
