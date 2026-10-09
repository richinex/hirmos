import { expect, test } from '@playwright/test'
import { fileURLToPath } from 'node:url'
import { prepare } from './examples/support'

test('survival shares select-all and clear actions without selecting reserved columns', async ({ page }, info) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(`Selection actions ${info.project.name}`)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL('../docs/2026-09-11-rviews-survival-veteran/data/veteran.csv', import.meta.url)))
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 60_000 })
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: 'Clear selected columns', exact: true }).click()
  await expect(page.getByRole('checkbox', { checked: true })).toHaveCount(0)
  await prepare(page, { structure: 'cross-section', columns: ['time', 'status', 'age', 'karno', 'prior', 'diagtime', 'trt'] })
  const toggle = page.getByRole('button', { name: 'Expand section list' })
  if (await toggle.isVisible()) await toggle.click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Survival analysis/ }).click()
  const setup = page.getByRole('region', { name: 'Survival setup', exact: true })
  const covariates = setup.getByRole('group', { name: 'Covariates', exact: true })
  for (const analysis of ['Cox regression', 'Aalen regression', 'Survival forest']) {
    await setup.getByRole('radio', { name: analysis, exact: true }).check()
    const selectText = covariates.getByRole('button', { name: 'Select all covariates' }).getByText('Select all', { exact: true })
    const clearText = covariates.getByRole('button', { name: 'Clear selected covariates' }).getByText('Clear', { exact: true })
    await expect(selectText).toBeVisible({ visible: (page.viewportSize()?.width ?? 0) >= 768 })
    await expect(clearText).toBeVisible({ visible: (page.viewportSize()?.width ?? 0) >= 768 })
    await covariates.getByRole('button', { name: 'Select all covariates' }).click()
    await expect(covariates.locator('input:checked')).toHaveCount(await covariates.locator('input:not(:disabled)').count())
    await expect(covariates.locator('input:disabled:checked')).toHaveCount(0)
    await covariates.getByRole('button', { name: 'Clear selected covariates' }).click()
    await expect(covariates.locator('input:checked')).toHaveCount(0)
    await covariates.getByRole('checkbox', { name: 'age', exact: true }).check()
    await expect(covariates.locator('input:checked')).toHaveCount(1)
  }
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await covariates.screenshot({ path: info.outputPath('survival-selection.png') })
})

test('scatter labels use shared rounding without changing plotted coordinates', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ scatterMatrixOption }, { readChartTheme }, { formatStatistic }] = await Promise.all([
      load('/src/charts/data/scatterMatrix.ts'), load('/src/charts/theme.ts'), load('/src/lib/format/number.ts'),
    ])
    const values = [7.949247704908109, 1.649777010539557, -0.1006235009476958, 0.00002040603802633085]
    const option = scatterMatrixOption([{ name: 'A', values }, { name: 'B', values }], readChartTheme())
    return {
      labels: values.map(value => option.xAxis[0].axisLabel.formatter(value)),
      expected: values.map(value => formatStatistic('raw', value).text),
      points: option.series[1].data,
      tooltip: option.tooltip.formatter({ seriesIndex: 1, value: [values[0], values[1]] }),
      values,
    }
  })
  expect(result.labels).toEqual(result.expected)
  expect(result.labels[0]).toBe('7.95')
  expect(result.labels[1]).toBe('1.65')
  expect(result.tooltip).not.toContain('7.949247704908109')
  expect(result.points).toEqual(result.values.map(value => [value, value]))
})
