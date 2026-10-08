import { expect, test, type Page } from '@playwright/test'
import { chapter } from './examples/support'

test('study deletion removes identification records, retains its DAG, and persists', async ({
  page,
}, info) => {
  await page.route('**/examples/ai-adoption-company-wide.hirmos.json', async (route) => {
    const response = await route.fetch()
    const bundle = await response.json()
    bundle.project.estimationRuns = []
    bundle.project.sensitivityRuns = []
    bundle.project.counterfactualRuns = []
    await route.fulfill({ response, json: bundle })
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /Study design/)
  await expect(page.getByRole('heading', { name: 'Study design', exact: true })).toBeVisible()
  const history = page.getByRole('button', { name: /^Studies \(/ })
  if (await history.isVisible()) await history.click()
  await page.getByRole('button', { name: /^Delete study: / }).click()
  await page.screenshot({ path: info.outputPath('study-delete-confirm.png') })
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Delete study', exact: true })
    .click()
  await chapter(page, /^Projects/)
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  const saved = await page.evaluate(async () => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const result = await store.loadProject('7c2f1b3e-5a64-4d1e-9b0a-2e6f8c1d4a71')
    if (!result.ok) throw Error(result.error.kind)
    return {
      studies: result.value.studies.length,
      identifications: result.value.identifications.length,
      dags: result.value.dagDocuments.length,
    }
  })
  expect(saved).toEqual({ studies: 0, identifications: 0, dags: 1 })
  await chapter(page, /DAG workspace/)
  await openDags(page)
  await page.getByRole('button', { name: /^Delete DAG / }).click()
  await expect(
    page.getByRole('alertdialog').getByRole('button', { name: 'Delete DAG', exact: true }),
  ).toBeEnabled()
})

test('deleting the selected DAG selects a remaining DAG', async ({ page }, info) => {
  await page.route('**/examples/confounded-dose.hirmos.json', async (route) => {
    const response = await route.fetch()
    const bundle = await response.json()
    bundle.project.interventionQueries = []
    const original = bundle.project.dagDocuments[0]
    bundle.project.dagDocuments.push({
      ...original,
      id: '10000000-0000-4000-8000-000000000001',
      name: 'Alternative graph',
    })
    await route.fulfill({ response, json: bundle })
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open ' + example, exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /DAG workspace/)
  await openDags(page)
  await page
    .getByRole('button', { name: 'Delete DAG Severity confounds dose', exact: true })
    .click()
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Delete DAG', exact: true })
    .click()
  await openDags(page)
  await expect(page.getByRole('button', { name: /^Alternative graph,/ })).toHaveAttribute(
    'aria-pressed',
    'true',
  )
  await expect(
    page.getByRole('button', { name: 'Delete DAG Severity confounds dose', exact: true }),
  ).toHaveCount(0)
  await page.screenshot({ path: info.outputPath('remaining-dag-selected.png') })
})

test('a saved graph check can be deleted explicitly from the blocker list', async ({
  page,
}, info) => {
  await page.route('**/examples/lalonde.hirmos.json', async (route) => {
    const response = await route.fetch()
    const bundle = await response.json()
    bundle.project.studies = []
    bundle.project.identifications = []
    bundle.project.estimationRuns = []
    bundle.project.sensitivityRuns = []
    bundle.project.counterfactualRuns = []
    await route.fulfill({ response, json: bundle })
  })
  await page.goto('/app/projects')
  await page
    .getByRole('button', { name: 'Open NSW job training and 1978 earnings', exact: true })
    .click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /DAG workspace/)
  await openDags(page)
  await page.getByRole('button', { name: /^Delete DAG / }).click()
  const dialog = page.getByRole('alertdialog')
  await expect(dialog.getByText('Graph check', { exact: false }).first()).toBeVisible()
  await dialog.getByRole('button', { name: 'Review deletion of this graph check' }).click()
  await expect(dialog.getByRole('button', { name: 'Delete check' })).toBeEnabled()
  await page.screenshot({ path: info.outputPath('check-delete-confirm.png') })
  await dialog.getByRole('button', { name: 'Delete check' }).click()
  await openDags(page)
  await page.getByRole('button', { name: /^Delete DAG / }).click()
  await expect(dialog.getByRole('button', { name: 'Delete DAG', exact: true })).toBeEnabled()
})

const example = 'Severity, dose and recovery'
test.beforeEach(({ page }) => {
  page.setDefaultTimeout(15000)
})
const openDags = async (page: Page) => {
  await expect(page.getByRole('toolbar', { name: 'DAG actions' })).toBeVisible()
  const history = page.getByRole('button', { name: /^(Arrows|DAGs) \(/ })
  if (await history.isVisible()) await history.click()
  await page.getByRole('radio', { name: 'DAGs', exact: true }).check()
}
test('DAG deletion cancels safely, deletes the final DAG, and survives reopening', async ({
  page,
}, info) => {
  test.setTimeout(90000)
  // Start from the shipped data and graph, without its saved distribution queries.
  await page.route('**/examples/confounded-dose.hirmos.json', async (route) => {
    const response = await route.fetch()
    const bundle = await response.json()
    bundle.project.interventionQueries = []
    await route.fulfill({ response, json: bundle })
  })
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open ' + example, exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /DAG workspace/)
  await openDags(page)
  const remove = page.getByRole('button', { name: /^Delete DAG / })
  await remove.click()
  const dialog = page.getByRole('alertdialog')
  await expect(dialog.getByRole('button', { name: 'Delete DAG', exact: true })).toBeEnabled()
  await page.screenshot({ path: info.outputPath('dag-delete-confirm.png') })
  await dialog.getByRole('button', { name: 'Cancel', exact: true }).click()
  await openDags(page)
  await expect(remove).toBeVisible()
  await remove.click()
  await dialog.getByRole('button', { name: 'Delete DAG', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Create DAG draft' })).toBeVisible()
  await page.screenshot({ path: info.outputPath('dag-deleted.png') })
  await chapter(page, /^Projects/)
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open ' + example, exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /DAG workspace/)
  await expect(page.getByRole('button', { name: 'Create DAG draft' })).toBeVisible()
})
test('a saved study blocks DAG deletion and an effect blocks study deletion', async ({
  page,
}, info) => {
  test.setTimeout(90000)
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /DAG workspace/)
  await openDags(page)
  await page.getByRole('button', { name: /^Delete DAG / }).click()
  const dialog = page.getByRole('alertdialog')
  await expect(dialog.getByText('Study design, Studies')).toBeVisible()
  await expect(dialog.getByRole('button', { name: 'Delete DAG', exact: true })).toHaveCount(0)
  await page.screenshot({ path: info.outputPath('dag-delete-blocked.png') })
  await dialog.getByRole('button', { name: 'Close', exact: true }).click()
  await chapter(page, /Study design/)
  await expect(page.getByRole('heading', { name: 'Study design', exact: true })).toBeVisible()
  const history = page.getByRole('button', { name: /^Studies \(/ })
  if (await history.isVisible()) await history.click()
  await page.getByRole('button', { name: /^Delete study: / }).click()
  await expect(dialog.getByText('Estimation, Runs')).toBeVisible()
  await expect(dialog.getByRole('button', { name: 'Delete study', exact: true })).toHaveCount(0)
  await page.screenshot({ path: info.outputPath('study-delete-blocked.png') })
})
