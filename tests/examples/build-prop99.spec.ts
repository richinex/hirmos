import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, createDag, createProject, exportBundle, identify, prepare, runEstimator } from './support'

/** California's 1988 tobacco programme against a synthetic California built from donor states. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the Proposition 99 example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('5b2e8d4a-7c19-4f36-b8e2-1d9a3c6e4f57' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'time series', time: 'Year', frequency: 'Yearly', columns: 'all' })

  await createDag(page, 'Proposition 99 and cigarette sales')
  await addArrow(page, 'prop99', 'California', 'Proposition 99 raised the cigarette excise tax in California from 1989 and funded a tobacco control programme.')

  await identify(page, {
    graph: 'Proposition 99 and cigarette sales', treatment: 'prop99', outcome: 'California', mechanism: 'Policy change',
    sentence: 'Proposition 99 took effect in California from 1989; the other states did not adopt it.',
    consistency: 'The programme means the same tax and campaign in every year it is in force.',
    interference: 'The donor states are assumed not to be affected by California policy.',
  })

  await runEstimator(page, { family: /^Interventions/, estimator: /^Synthetic control/, run: /^Run synthetic control/i })
  await exportBundle(page, example)
})
