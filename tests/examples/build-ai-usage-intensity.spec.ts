import { expect, test } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { addArrow, chapter, createDag, createProject, exportBundle, identify } from './support'

/** Continuous usage with three confounders and one mediator the adjustment set must leave alone. */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

test('build the AI usage intensity example bundle', async ({ page }) => {
  test.setTimeout(600_000)
  const example = shippedExampleById('c1d8e7f2-3a49-4b6d-8e5f-4f1b2c3d6a93' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  for (const column of ['team_experience', 'project_complexity', 'release_pressure', 'ai_usage', 'code_volume_kloc', 'bugs_per_kloc']) {
    await page.getByRole('checkbox', { name: column, exact: true }).check()
  }
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByText(/Prepared cross-section/)).toBeVisible({ timeout: 120_000 })

  await createDag(page, 'AI usage, volume and defects')
  // Three common causes of usage and defects, then the path that runs through code volume.
  await addArrow(page, 'team_experience', 'ai_usage', 'Experienced teams take up the assistant faster.')
  await addArrow(page, 'team_experience', 'bugs_per_kloc', 'Experienced teams write fewer defects whatever tooling they use.')
  await addArrow(page, 'project_complexity', 'ai_usage', 'Harder projects give the assistant less purchase, so teams lean on it less.')
  await addArrow(page, 'project_complexity', 'bugs_per_kloc', 'Harder projects carry more defects per unit of code.')
  await addArrow(page, 'project_complexity', 'code_volume_kloc', 'Harder projects need more code.')
  await addArrow(page, 'release_pressure', 'ai_usage', 'Teams under deadline pressure reach for the assistant more.')
  await addArrow(page, 'release_pressure', 'bugs_per_kloc', 'Deadline pressure raises the defect rate directly.')
  await addArrow(page, 'ai_usage', 'code_volume_kloc', 'The assistant produces more code per developer.')
  await addArrow(page, 'code_volume_kloc', 'bugs_per_kloc', 'More code carries more defects, so volume passes part of the effect through.')
  await addArrow(page, 'ai_usage', 'bugs_per_kloc', 'The assistant changes the quality of each line written.')

  await identify(page, {
    graph: 'AI usage, volume and defects', treatment: 'ai_usage', outcome: 'bugs_per_kloc', mechanism: 'Observed choice',
    sentence: 'Each team chose how heavily to use the assistant, given its experience, the project and the deadline.',
  })

  await chapter(page, /Estimation/)
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByText('Current estimate').first()).toBeVisible({ timeout: 300_000 })

  await exportBundle(page, example)
})
