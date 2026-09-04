import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, createDag, createProject, exportBundle, identify, prepare, runEstimator } from './support'

/** The tscount campylobacter counts with a step indicator, through the negative-binomial INGARCH estimator. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the campylobacter example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('8e1c4f7b-9a25-4d63-a7f0-2b5c9d8e1a64' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  // Year and season stay out: the walkthrough models the step and the count recursion alone.
  await prepare(page, { structure: 'time series', time: 'period', columns: ['outbreak', 'cases'] })

  await createDag(page, 'Outbreak step and campylobacter cases')
  await addArrow(page, 'outbreak', 'cases', 'A sustained level shift is scanned for in the window the tscount example searches; the indicator marks the period from which it holds.')

  await identify(page, {
    graph: 'Outbreak step and campylobacter cases', treatment: 'outbreak', outcome: 'cases', mechanism: 'Policy change',
    sentence: 'The indicator marks period 84 onward, the strongest level shift in the window the tscount example searches.',
    consistency: 'The indicator means the same raised-incidence regime in every period it marks.',
    interference: 'Periods affect one another only through the recorded count recursion.',
  })

  await runEstimator(page, {
    family: /^Dynamics/, estimator: /Negative-binomial INGARCH/, choices: ['Additive', 'Persistent'],
    run: /^Run negative-binomial INGARCH/i,
  })
  await exportBundle(page, example)
})
