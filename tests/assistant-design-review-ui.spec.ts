import { expect, test, type Page, type TestInfo } from '@playwright/test'
import { resolve } from 'node:path'
import { chapter, createDag, identify, prepare } from './examples/support'
import { snapshot } from './assistantTestFlow'

async function savedReview(page: Page, info: TestInfo, name: string, kind: string) {
  await chapter(page, /^Projects/)
  await expect
    .poll(async () =>
      (await snapshot(page, name))?.estimationRuns.some(
        (run: { kind: string }) => run.kind === kind,
      ),
    )
    .toBe(true)
  const saved = await snapshot(page, name)
  const reviewed = await page.evaluate(
    async ({ saved, kind }) => {
      const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
      const run = saved.estimationRuns.findLast(
        (candidate: { kind: string }) => candidate.kind === kind,
      )
      return readRun({ ...saved, kind: 'profiled' }, { id: run.id, view: 'review' })
    },
    { saved, kind },
  )
  expect(reviewed.review.kind).toBe('available')
  await info.attach('saved-design-review.json', {
    body: JSON.stringify({ project: saved, review: reviewed }, null, 2),
    contentType: 'application/json',
  })
  return reviewed.review
}

test('Sun–Abraham fitted in Estimation has a saved design review', async ({ page }, info) => {
  test.setTimeout(120000)
  page.setDefaultTimeout(15000)
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Sun–Abraham', exact: true }).click()
  await page.getByRole('button', { name: 'Run panel DiD', exact: true }).click()
  await expect(page.getByText('Retained clusters', { exact: true }).first()).toBeVisible({
    timeout: 60000,
  })
  const review = await savedReview(
    page,
    info,
    'AI adoption, March cohort',
    'panel-intervention-run',
  )
  expect(review.items.find((item: { id: string }) => item.id === 'comparison').reading).toContain(
    'Reference periods',
  )
})

test('IV fitted in Estimation has a saved design review', async ({ page }, info) => {
  test.setTimeout(120000)
  page.setDefaultTimeout(15000)
  const name = 'IV saved review'
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page
    .locator('input[type=file]')
    .setInputFiles(resolve('docs/medium/ab-test/data/loyalty.csv'))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await prepare(page, {
    structure: 'cross-section',
    columns: ['received_nudge_email', 'loyalty_program', 'spend'],
  })
  await createDag(page, 'Instrument assignment')
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page
    .getByRole('textbox', { name: 'Graph text' })
    .fill(
      'dag { U [latent] received_nudge_email -> loyalty_program U -> loyalty_program U -> spend loyalty_program -> spend }',
    )
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await identify(page, {
    graph: 'Instrument assignment',
    treatment: 'loyalty_program',
    outcome: 'spend',
    mechanism: 'Observed choice',
    sentence:
      'Assume the email influences spending only through programme membership and is independent of latent loyalty.',
    result: /instrument/i,
  })
  await chapter(page, /Estimation/)
  await page.locator('input[type=radio][value="instrumental-variable"]').check()
  await page
    .getByRole('spinbutton', { name: 'Instrumental-variable bootstrap resamples', exact: true })
    .fill('20')
  await page.getByRole('button', { name: /^Run instrumental variable/i }).click()
  await expect(page.getByText('Current estimate', { exact: true }).first()).toBeVisible({
    timeout: 60000,
  })
  const review = await savedReview(page, info, name, 'instrumental-variable-run')
  expect(review.items.find((item: { id: string }) => item.id === 'strength').evidence.kind).toBe(
    'not-recorded',
  )
  expect(review.items.find((item: { id: string }) => item.id === 'instruments').reading).toContain(
    'The recorded instrument is received_nudge_email.',
  )
})
