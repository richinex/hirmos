import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, createDag, createProject, exportBundle, identify, prepare, runEstimator } from './support'

/** The NSW job-training data: covariate adjustment against a non-experimental comparison group. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

const WHY: Record<string, string> = {
  age: 'Age determines labour-market experience and eligibility screening for the NSW programme.',
  educ: 'Years of schooling drive both selection into training and subsequent earnings capacity.',
  black: 'Race is associated with programme recruitment and with labour-market discrimination affecting earnings.',
  hispan: 'Ethnicity is associated with programme recruitment and with labour-market discrimination affecting earnings.',
  married: 'Marital status affects both willingness to enrol and household labour supply.',
  nodegree: 'Absence of a high-school degree was an NSW eligibility criterion and independently depresses earnings.',
  re75: 'Pre-treatment earnings drive programme eligibility and are strongly autocorrelated with later earnings.',
}

test('build the NSW job training example bundle', async ({ page }) => {
  test.setTimeout(900_000)
  const example = shippedExampleById('3f6a9c1e-2b7d-4e58-9a01-6c4d8e2f7b13' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: 'all' })

  await createDag(page, 'NSW training and 1978 earnings')
  await addArrow(page, 'treat', 're78', 'The NSW programme provided subsidised work experience intended to raise later earnings.')
  for (const [confounder, why] of Object.entries(WHY)) {
    await addArrow(page, confounder, 'treat', why)
    await addArrow(page, confounder, 're78', why)
  }
  // The graph's testable implications are left on screen as the walkthrough found them: some fail.
  await page.getByRole('button', { name: 'Run checks' }).click()
  await expect(page.getByText(/graph implications contradicted/).first()).toBeVisible({ timeout: 120_000 })

  await identify(page, {
    graph: 'NSW training and 1978 earnings', treatment: 'treat', outcome: 're78', mechanism: 'Observed choice',
    sentence: 'Treated rows are NSW programme enrollees; control rows are a non-experimental PSID comparison group, so treatment status reflects programme participation and sample construction rather than randomisation.',
  })

  await runEstimator(page, { run: /^Run adjusted linear regression/ })
  await exportBundle(page, example)
})
