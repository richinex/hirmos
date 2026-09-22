import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { choose, prepare } from './examples/support'

// The interrupted series on the tutorial's own data (Lopez Bernal, Cummins and Gasparrini, IJE 2017):
// the Sicily smoking ban, 59 months, the ban from month 37. Model 3 of the paper's code is a
// quasi-Poisson step change with two harmonic pairs and the standardised population as the
// offset: rate ratio 0.885 (0.839 to 0.933), dispersion 2.262, trend 1.067 per year.
const sicily = readFileSync(new URL('../crates/causal-core/oracle/sicily.csv', import.meta.url))

test('fits the paper\'s smoking-ban model through the UI and reads it as the paper does', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One engine run')
  test.setTimeout(180_000)
  const errors: string[] = []
  page.on('pageerror', (e) => errors.push(e.message))
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Sicily smoking ban')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'sicily.csv', mimeType: 'text/csv', buffer: sicily })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 60_000 })
  await prepare(page, { structure: 'time series', time: 'time', frequency: 'Monthly', columns: ['aces', 'stdpop', 'smokban'] })
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Time-series analysis/ }).click()
  await page.getByRole('radio', { name: 'Interrupted series', exact: true }).click()
  const fit = page.getByRole('button', { name: 'Fit interrupted series', exact: true })
  await expect(fit).toBeDisabled()
  await choose(page, 'Series', 'aces')
  await page.getByRole('radio', { name: 'Count', exact: true }).click({ force: true })
  await choose(page, 'Exposure', 'stdpop')
  await page.getByRole('spinbutton', { name: 'Intervention row' }).fill('37')
  await expect(page.getByRole('spinbutton', { name: 'Seasonal terms' })).toHaveValue('2')
  await fit.click()
  const result = page.getByRole('region', { name: 'Time-series result' }).filter({ visible: true })
  await expect(result).toHaveCount(1, { timeout: 60_000 })
  await expect(result).toContainText('Interrupted series for aces, level change')
  await expect(result).toContainText('rate ratio of 0.885 [0.839, 0.933] 95% CI, p < 0.001')
  await expect(result).toContainText('The interval excludes one.')
  await expect(result).toContainText('dispersion 2.262')
  const tiles = result.locator('[aria-label="Interrupted series summary"]')
  await expect(tiles).toContainText('Trend per 12 rows, rate ratio')
  await expect(tiles).toContainText('1.067')
  await expect(tiles).toContainText('2.262')
  // The paper's figures: the fit with its counterfactual, the deseasonalised trend, the residuals, the ACF and PACF.
  for (const chart of ['Interrupted series fit', 'Deseasonalised trend', 'Residuals over time', 'Residual autocorrelation', 'Residual partial autocorrelation']) {
    await expect(result.getByRole('figure', { name: chart }).or(result.locator(`[aria-label="${chart}"]`)).first()).toBeVisible()
  }
  const terms = result.getByRole('table', { name: 'Fitted terms' })
  await expect(terms).toContainText('Level change')
  await expect(terms.locator('tbody tr').nth(2)).toContainText('−0.122')
  await expect(terms.locator('tbody tr').nth(2)).toContainText('0.885 (0.839 to 0.933)')
  await page.screenshot({ path: info.outputPath('sicily-model3.png'), fullPage: true })

  // Model 4: the change in slope, parameterised as in the corrigendum, is not distinguishable from none.
  await page.getByRole('radio', { name: 'Level and slope change', exact: true }).click({ force: true })
  await fit.click()
  await expect(result.filter({ hasText: 'level and slope change' })).toHaveCount(1, { timeout: 60_000 })
  await expect(result).toContainText('Slope change per row, rate ratio')
  await expect(result.getByRole('table', { name: 'Fitted terms' }).locator('tbody tr').nth(3)).toContainText('0.00148')
  // The same design on the standardised count as a continuous series with AR(1) errors: the
  // statsmodels SARIMAX fixture (crates/causal-core/oracle/fixtures/arma_regression.json) fits
  // the rate; the count at the mean population is the rate times a constant, so the step and
  // the error process read the same and the level change scales with it.
  await page.getByRole('radio', { name: 'Level change', exact: true }).click({ force: true })
  await page.getByRole('radio', { name: 'Continuous', exact: true }).click({ force: true })
  await page.getByRole('radio', { name: 'ARMA errors', exact: true }).click({ force: true })
  await expect(page.getByRole('spinbutton', { name: 'Autoregressive order' })).toHaveValue('1')
  await expect(page.getByRole('spinbutton', { name: 'Moving-average order' })).toHaveValue('0')
  await expect(page.getByRole('spinbutton', { name: 'Optimiser iterations' })).toHaveValue('50')
  await page.getByRole('spinbutton', { name: 'Moving-average order' }).fill('1')
  await page.getByRole('spinbutton', { name: 'Autoregressive order' }).fill('0')
  await fit.click()
  await expect(result.filter({ hasText: 'ARMA(0, 1) errors' })).toHaveCount(1, { timeout: 90_000 })
  await expect(result).toContainText('The terms were fitted jointly with ARMA(0, 1) errors by maximum likelihood')
  const errorTable = result.getByRole('table', { name: 'Error process' })
  await expect(errorTable).toContainText('ma.L1')
  await expect(errorTable).toContainText('converged in')
  await expect(result.getByRole('figure', { name: 'Residuals over time' }).or(result.locator('[aria-label="Residuals over time"]')).first()).toBeVisible()
  await expect(result).toContainText('Innovation variance')
  await page.screenshot({ path: info.outputPath('sicily-arma.png'), fullPage: true })

  // Every run lands in the Results ledger with the same reading.
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Results/ }).click()
  const ledger = page.getByRole('region', { name: 'Time-series result' })
  await expect(ledger).toHaveCount(3)
  await expect(ledger.first()).toContainText('ARMA(0, 1) errors')
  await expect(ledger.nth(1)).toContainText('level and slope change')
  await expect(ledger.last()).toContainText('rate ratio of 0.885 [0.839, 0.933] 95% CI')
  await page.screenshot({ path: info.outputPath('sicily-ledger.png'), fullPage: true })
  expect(errors).toEqual([])
})
