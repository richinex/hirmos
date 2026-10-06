import { expect, test, type Locator, type Page } from '@playwright/test'
import { identifyEffect } from './examples/support'

const choose = async (trigger: Locator, label: string) => {
  await trigger.click()
  await trigger.page().getByRole('listbox').getByRole('option', { name: label, exact: true }).click()
}

const binaryFixture = (): string => {
  const records = ['Z,X,Y']
  const cells = [
    [0, 0, 0, 30], [0, 0, 1, 10], [0, 1, 0, 5], [0, 1, 1, 5],
    [1, 0, 0, 5], [1, 0, 1, 5], [1, 1, 0, 10], [1, 1, 1, 30],
  ] as const
  for (const [z, x, y, count] of cells) for (let index = 0; index < count; index += 1) records.push(`${z},${x},${y}`)
  return `${records.join('\n')}\n`
}

test('a fresh IDC*-only project reaches and runs binary ETT estimation', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The complete numerical workflow runs once')
  test.setTimeout(120_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Binary ETT workflow')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles({ name: 'binary-ett.csv', mimeType: 'text/csv', buffer: Buffer.from(binaryFixture()) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).click()
  for (const column of ['Z', 'X', 'Y']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared dataset version/ }).click()
  await expect(page.getByRole('status').filter({ hasText: 'Cross-section, 100 rows' })).toBeVisible({ timeout: 30_000 })

  await page.getByRole('button', { name: /Build a DAG/ }).click()
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Front-door graph')
  await page.getByRole('button', { name: /Create DAG/ }).click()
  await page.getByRole('button', { name: /Unmeasured variable/ }).click()
  await page.getByLabel(/Unmeasured variable name/).fill('U')
  await page.getByRole('button', { name: /Add to DAG/ }).click()
  for (const [cause, effect] of [['U', 'X'], ['U', 'Y'], ['X', 'Z'], ['Z', 'Y']] as const) {
    await choose(page.getByLabel('Proposed cause'), cause)
    await choose(page.getByLabel('Proposed effect'), effect)
    await page.getByLabel(/Rationale|Substantive basis/).fill(`${cause} precedes ${effect} in the study model.`)
    await page.getByRole('button', { name: /Add the arrow|Add edge/ }).click()
  }
  await choose(page.getByLabel('Treatment'), 'X')
  await choose(page.getByLabel('Outcome'), 'Y')
  await page.getByRole('button', { name: 'Use for study' }).click()

  await page.getByRole('radio', { name: /Treated rows \(ATT\)/ }).click()
  await page.getByRole('radio', { name: 'Observed choice' }).click()
  await page.getByLabel('Assignment sentence').fill('Treatment was selected by observed units rather than assigned by the analyst.')
  await identifyEffect(page)
  const identifiedStudy = page.getByRole('article', { name: /among treated rows identification/ })
  await expect(identifiedStudy.getByText('Identified by IDC*', { exact: false })).toBeVisible({ timeout: 30_000 })
  const treated = identifiedStudy.getByText(/P\(Y @ \+X: \+Y \| X: \+X\) =/)
  const untreated = identifiedStudy.getByText(/P\(Y @ -X: \+Y \| X: \+X\) =/)
  await expect(treated).toBeVisible()
  await expect(untreated).toBeVisible()
  await expect(treated).not.toHaveText(await untreated.innerText())

  await page.getByRole('button', { name: 'Continue to estimation' }).click()
  await expect(page).toHaveURL(/\/app\/projects\/[^/]+\/estimation$/)
  await expect(page.getByRole('heading', { name: 'Estimation', exact: true })).toBeVisible()
  await expect(page.getByRole('radio', { name: /Binary ETT by IDC\*/ })).toBeChecked()
  const run = page.getByRole('button', { name: /Run binary ETT by IDC\*/i })
  await expect(run).toBeEnabled({ timeout: 30_000 })
  await run.click()
  await expect(page.getByText('0.150', { exact: true }).first()).toBeVisible({ timeout: 30_000 })
})
