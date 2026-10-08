import { expect, test, type Page } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { chapter, choose } from './examples/support'

async function dags(page: Page) {
  await expect(page.getByRole('dialog', { includeHidden: true })).toHaveCount(0)
  const history = page.getByRole('button', { name: /^(Arrows|DAGs) \(/ })
  if ((page.viewportSize()?.width ?? 1280) < 768) {
    await expect(history).toBeVisible()
    await history.click()
  }
  await page.getByRole('radio', { name: 'DAGs', exact: true }).check()
}
async function pick(page: Page, name: string) {
  await dags(page)
  await page.getByRole('button', { name: new RegExp('^' + name + ',') }).click()
}
async function create(page: Page, name: string, graph: string) {
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(name)
  await page.getByRole('button', { name: 'Create DAG draft', exact: true }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page.getByRole('textbox', { name: 'Graph text' }).fill(graph)
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
}
async function restoreSource(page: Page) {
  const needsFile = page.getByRole('heading', { name: 'Choose the data file again', exact: true })
  await expect(needsFile.or(page.locator('#data-profile-title'))).toBeVisible()
  if (await needsFile.isVisible())
    await page.locator('input[type=file]').setInputFiles('docs/controls/data/columns.csv')
  await expect(page.locator('#data-profile-title')).toBeVisible()
}
test('each DAG retains its pair across switching, reload and export/import', async ({
  page,
}, info) => {
  test.setTimeout(120000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Persistent control graphs')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles('docs/controls/data/columns.csv')
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await chapter(page, /DAG workspace/)
  await create(page, 'Mediator descendant', 'dag { X [exposure] Y [outcome] X -> M -> Y M -> Z }')
  await expect(page.getByRole('combobox', { name: 'Treatment', exact: true })).toContainText('X')
  // Open the inspector on mobile.
  const inspect = async () => {
    const button = page.getByRole('button', { name: 'Inspector', exact: true })
    if (await button.isVisible()) await button.click()
  }
  await inspect()
  await expect(page.getByRole('region', { name: 'Post-treatment variables' })).toContainText(
    'partly control for the mediator',
  )
  await page.getByRole('region', { name: 'Post-treatment variables' }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('model12.png') })
  if (await page.getByRole('dialog').isVisible()) await page.keyboard.press('Escape')
  await dags(page)
  await page.getByRole('button', { name: 'New DAG', exact: true }).click()
  await create(page, 'Treatment branch', 'dag { X [exposure] Y [outcome] X -> Y X -> Z }')
  await inspect()
  await expect(page.getByRole('region', { name: 'Post-treatment variables' })).toContainText(
    'alone is valid',
  )
  await page.getByRole('region', { name: 'Post-treatment variables' }).scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('model14.png') })
  if (await page.getByRole('dialog').isVisible()) await page.keyboard.press('Escape')
  await choose(page, 'Outcome', 'Z')
  await pick(page, 'Mediator descendant')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Y')
  await pick(page, 'Treatment branch')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Z')
  await chapter(page, /^Projects/)
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Persistent control graphs', exact: true }).click()
  await restoreSource(page)
  await chapter(page, /DAG workspace/)
  await pick(page, 'Mediator descendant')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Y')
  await pick(page, 'Treatment branch')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Z')
  const expand = page.getByRole('button', { name: 'Expand section list' })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const exported = await readFile(await (await download).path(), 'utf8')
  const bundle = JSON.parse(exported)
  expect(
    bundle.project.dagDocuments.every(
      (d: { exploration?: unknown }) => d.exploration !== undefined,
    ),
  ).toBe(true)
  await chapter(page, /^Projects/)
  await page.getByLabel('Exported project file', { exact: true }).setInputFiles({
    name: 'controls.hirmos.json',
    mimeType: 'application/json',
    buffer: Buffer.from(exported),
  })
  await restoreSource(page)
  await chapter(page, /DAG workspace/)
  await pick(page, 'Mediator descendant')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Y')
  await pick(page, 'Treatment branch')
  await expect(page.getByRole('combobox', { name: 'Outcome', exact: true })).toContainText('Z')
})
