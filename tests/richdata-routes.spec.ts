import { expect, test } from '@playwright/test'
import { fileURLToPath } from 'node:url'
import { choose } from './examples/support'

async function upload(page: import('@playwright/test').Page, name: string) {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(name)
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL(`../docs/2026-09-17-richdata01/data/${name}`, import.meta.url)))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
}

test('single-resource consecutive series prepares without an unrelated missing-value policy', async ({ page }, info) => {
  await upload(page, '105w_tcsp_weekly.parquet')
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  await choose(page, 'Time column', 'year_week')
  await choose(page, 'Time interpretation', 'ISO week (2024-W02)')
  await choose(page, 'Source frequency', 'Weekly')
  await page.getByRole('checkbox', { name: 'commits', exact: true }).check()
  await expect(page.locator('section[aria-labelledby="missingness-title"]')).toContainText('No missing values detected.')
  const prepare = page.getByRole('button', { name: 'Create prepared dataset version', exact: true })
  await page.getByRole('checkbox', { name: 'mean_ttr_min', exact: true }).check()
  await expect(prepare).toBeDisabled()
  await expect(page.locator('#preparation-requirement')).toContainText('51 missing values still need a policy.')
  await page.getByRole('checkbox', { name: 'mean_ttr_min', exact: true }).uncheck()
  await expect(prepare).toBeEnabled()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Column profile', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Prepared data', exact: true })).toContainText('69 rows')
  await page.screenshot({ path: info.outputPath('consecutive-series.png') })
})

test('lag-aware preparation previews saved gaps without asking for another policy', async ({ page }, info) => {
  await upload(page, '105w_tcsp_weekly.parquet')
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  await choose(page, 'Time column', 'year_week')
  await choose(page, 'Time interpretation', 'ISO week (2024-W02)')
  await choose(page, 'Source frequency', 'Weekly')
  await page.getByRole('checkbox', { name: 'mean_ttr_min', exact: true }).check()
  await page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  const preview = page.getByRole('region', { name: 'Prepared values', exact: true })
  await expect(preview.getByRole('button', { name: 'Refresh preview' })).toBeVisible()
  await expect(preview.getByTestId('prepared-series')).toBeVisible()
  await expect(preview.getByRole('alert')).toHaveCount(0)
  const values = await page.evaluate(async () => {
    const charts = await import(new URL('/node_modules/.vite/deps/echarts_core.js', location.href).href)
    const element = document.querySelector('[data-testid=prepared-series] [_echarts_instance_]') ?? document.querySelector('[data-testid=prepared-series]')
    return charts.getInstanceByDom(element).getOption().series[0].data.map((point: [number, number | null]) => point[1])
  })
  expect(values).toHaveLength(69)
  expect(values.filter((value: number | null) => value === null)).toHaveLength(51)
  await preview.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('prepared-gaps.png') })
  await preview.getByRole('button', { name: 'Refresh preview' }).click()
  await expect(preview.getByRole('button', { name: 'Refresh preview' })).toBeVisible()
  await expect(page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true })).toBeChecked()
})

for (const file of ['105w_merged_weekly.parquet', 'onboarded_with_incidents_2026-06-11.parquet']) {
  test(`panel structure audit: ${file}`, async ({ page }, info) => {
    await upload(page, file)
    await page.getByRole('radio', { name: /^Panel/ }).check()
    await choose(page, 'Unit column', 'resource_id')
    await choose(page, 'Time column', 'year_week')
    await choose(page, 'Source frequency', 'Weekly')
    const complete = await page.evaluate(async (name) => {
      const data = await import(new URL('/src/data/client.ts', location.href).href)
      const workflow = await import(new URL('/src/domain/workflow.ts', location.href).href)
      const types = await import(new URL('/src/domain/dataset.ts', location.href).href)
      const response = await fetch(`/docs/2026-09-17-richdata01/data/${name}`)
      const source = new File([await response.blob()], name)
      const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), source)
      if (!profile.ok) throw new Error(JSON.stringify(profile.error))
      return profile.value.columns.filter((column: { nullCount: number; duckdbType: string }) => column.nullCount === 0 && types.isNumericDuckDbType(column.duckdbType)).map((column: { name: string }) => column.name)
    }, file)
    console.log(file, 'Complete numeric columns:', complete)
    if (file === 'onboarded_with_incidents_2026-06-11.parquet') {
      expect(complete).toEqual([])
      await page.getByRole('checkbox', { name: 'active_components', exact: true }).check()
      await expect(page.getByRole('button', { name: 'Create prepared dataset version', exact: true })).toBeDisabled()
      await expect(page.locator('#preparation-requirement')).toContainText('missing values')
      await page.screenshot({ path: info.outputPath('panel-missing-values.png') })
      return
    }
    expect(complete.length).toBeGreaterThan(0)
    await page.getByRole('checkbox', { name: complete[0], exact: true }).check()
    await expect(page.locator('section[aria-labelledby="missingness-title"]')).toContainText('No missing values detected.')
    await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
    await expect(page.getByRole('alert')).toBeVisible()
    await expect(page.getByRole('alert')).toContainText('288 rows for 3 units × 120 periods')
    console.log(file, await page.getByRole('alert').allTextContents())
    await page.screenshot({ path: info.outputPath('panel-refusal.png') })
  })
}
