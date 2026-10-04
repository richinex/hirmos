import { expect, test } from '@playwright/test'

test('stationarity tests only selected variables and can cancel and restart', async ({ page }, info) => {
  test.setTimeout(180_000)
  page.setDefaultTimeout(20_000)
  // Hold the first stationarity request so that run stays in flight until it is cancelled.
  await page.addInitScript(() => {
    const post = Worker.prototype.postMessage
    let held = false
    Worker.prototype.postMessage = function (message: unknown, options?: Transferable[] | StructuredSerializeOptions) {
      if (!held && typeof message === 'object' && message !== null && 'kind' in message && message.kind === 'stationarity-battery') { held = true; return }
      post.call(this, message, Array.isArray(options) ? { transfer: options } : options)
    }
  })
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Stationarity selection')
  await page.getByRole('button', { name: 'Create project' }).click()
  let seed = 42
  const rows = Array.from({ length: 160 }, (_, time) => {
    seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0
    return `${time},${seed / 4294967296},${Math.sin(time / 5) + seed / 4294967296}`
  })
  await page.locator('input[type=file]').setInputFiles({ name: 'series.csv', mimeType: 'text/csv', buffer: Buffer.from(`time,x,y\n${rows.join('\n')}`) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Regular time series' }).check()
  await page.getByRole('combobox', { name: 'Time column', exact: true }).click()
  await page.getByRole('option', { name: 'time', exact: true }).click()
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('checkbox', { name: 'y', exact: true }).check()
  const transforms = page.getByRole('group', { name: 'Transformations by column', exact: true })
  await transforms.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('transforms.png') })
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await page.getByRole('radio', { name: /Stationarity/ }).check()
  const selection = page.getByRole('group', { name: 'Stationarity variables', exact: true })
  const run = page.getByRole('button', { name: 'Run stationarity tests', exact: true })
  await expect(run).toBeDisabled()
  await selection.getByRole('button', { name: 'Select all stationarity variables' }).click()
  await expect(selection.getByRole('checkbox', { checked: true })).toHaveCount(2)
  await selection.getByRole('button', { name: 'Clear stationarity variables' }).click()
  await expect(run).toBeDisabled()
  await selection.getByRole('checkbox', { name: 'x', exact: true }).check()
  await run.click()
  await page.getByRole('button', { name: 'Cancel run', exact: true }).click()
  await expect(page.getByRole('alert')).toContainText('Stationarity tests cancelled')
  await expect(run).toBeEnabled()
  await run.click()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toHaveCount(0, { timeout: 120_000 })
  const result = page.getByRole('article', { name: 'Stationarity result for x', exact: true })
  await expect(result).toBeVisible()
  await expect(page.getByRole('article', { name: 'Stationarity result for y', exact: true })).toHaveCount(0)
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('result-card.png') })
  await result.locator('summary').click()
  await expect(result.getByRole('columnheader', { name: 'Statistic', exact: true })).toBeVisible()
  await page.screenshot({ path: info.outputPath('result-details.png') })
  await expect(selection.getByRole('checkbox', { name: 'y', exact: true })).not.toBeChecked()
  await selection.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('stationarity-selection.png') })
  await selection.getByRole('checkbox', { name: 'x', exact: true }).uncheck()
  await selection.getByRole('checkbox', { name: 'y', exact: true }).check()
  await run.click()
  await expect(page.getByRole('article', { name: 'Stationarity result for y', exact: true })).toBeVisible({ timeout: 120_000 })
  await expect(result).toBeVisible()
  await run.click()
  await expect(page.getByRole('button', { name: 'Cancel run', exact: true })).toHaveCount(0, { timeout: 120_000 })
  await expect(page.getByRole('article', { name: /^Stationarity result for/ })).toHaveCount(2)
  await page.getByRole('button', { name: 'Delete stationarity result for x', exact: true }).click()
  await expect(result).toHaveCount(0)
  await expect(page.getByRole('article', { name: 'Stationarity result for y', exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Clear results', exact: true }).click()
  await expect(page.getByRole('article', { name: /^Stationarity result for/ })).toHaveCount(0)
})
