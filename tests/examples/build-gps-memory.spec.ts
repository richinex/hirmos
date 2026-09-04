import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, addUnmeasured, createDag, createProject, exportBundle, identify, prepare, runEstimator } from './support'

/** An unmeasured confounder, so no back-door set exists; the effect is identified through the mediator. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the GPS memory front-door example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('6a3e1d9f-5b47-4c82-8d1e-3f7a2b9c5e06' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: 'all' })

  await createDag(page, 'GPS use, hippocampal volume and spatial memory')
  await addUnmeasured(page, 'U')
  await addArrow(page, 'X', 'Z', 'Habitual satellite-navigation use reduces reliance on internal spatial encoding, which is reflected in hippocampal volume.')
  await addArrow(page, 'Z', 'Y', 'Hippocampal volume supports allocentric spatial representation, and so determines spatial memory performance.')
  await addArrow(page, 'U', 'X', 'An unobserved disposition towards technology use and exploration drives how much a person relies on GPS.')
  await addArrow(page, 'U', 'Y', 'The same unobserved disposition affects spatial memory directly, independently of hippocampal volume.')

  await identify(page, {
    graph: 'GPS use, hippocampal volume and spatial memory', treatment: 'X', outcome: 'Y', mechanism: 'Observed choice',
    sentence: 'Participants chose how much to rely on satellite navigation; no protocol assigned it.',
    result: /Identified by the general ID algorithm/,
  })

  await runEstimator(page, { estimator: { value: 'frontdoor-two-stage' }, run: /^Run linear front-door regression/ })
  await exportBundle(page, example)
})
