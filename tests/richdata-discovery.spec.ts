import { expect, test } from '@playwright/test'
import { fileURLToPath } from 'node:url'
import { chapter, choose } from './examples/support'

test('real weekly data distinguishes empty columns from partial missingness in discovery', async ({ page }, info) => {
  test.setTimeout(180_000)
  page.setDefaultTimeout(15_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Weekly discovery checks')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles(fileURLToPath(new URL('../docs/2026-09-17-richdata01/data/105w_tcsp_weekly.parquet', import.meta.url)))
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  await choose(page, 'Time column', 'year_week')
  await choose(page, 'Time interpretation', 'ISO week (2024-W02)')
  await choose(page, 'Source frequency', 'Weekly')
  const empty = ['new_security_issues', 'new_security_remediation_effort']
  for (const column of ['commits', 'mean_ttr_min', ...empty]) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Refresh preview' })).toBeVisible()
  await chapter(page, /Discovery lab/)
  for (const method of ['PCMCI+', 'LPCMCI', 'GRACE']) {
    await page.getByRole('radiogroup', { name: 'Discovery method family' }).getByRole('radio', { name: method === 'GRACE' ? /^Neural/ : /^PCMCI/ }).check()
    await page.getByRole('radio', { name: new RegExp(`^${method.replace('+', '\\+')}`) }).check()
    await expect(page.getByRole('button', { name: new RegExp(`^Run ${method.replace('+', '\\+')}(?: with|$)`) })).toBeDisabled()
    const notice = page.getByRole('alert').filter({ hasText: 'No observed values in:' })
    for (const column of empty) await expect(notice).toContainText(column)
  }
  await page.getByRole('radiogroup', { name: 'Discovery method family' }).getByRole('radio', { name: /^PCMCI/ }).check()
  await expect(page.getByRole('radio', { name: /^RPCMCI/ })).toBeDisabled()
  await page.getByRole('radiogroup', { name: 'Discovery method family' }).getByRole('radio', { name: /^Nonstationary/ }).check()
  for (const method of ['CD-NOTS', 'CD-NOTS+']) {
    await page.getByRole('radio', { name: new RegExp(`^${method.replace('+', '\\+')} Review`) }).check()
    await expect(page.getByRole('alert').filter({ hasText: 'p = 1 and statistic = 0' })).toBeVisible()
    await expect(page.getByRole('button', { name: new RegExp(`^Run ${method.replace('+', '\\+')}(?: with|$)`) })).toBeEnabled()
    await choose(page, 'Missing observations', 'VAR-EM imputation')
    await expect(page.getByRole('button', { name: new RegExp(`^Run ${method.replace('+', '\\+')}(?: with|$)`) })).toBeDisabled()
    await expect(page.getByRole('alert').filter({ hasText: 'VAR-EM cannot initialise' })).toBeVisible()
  }
  await chapter(page, /Data studio/)
  for (const column of empty) await page.getByRole('checkbox', { name: column, exact: true }).uncheck()
  await page.getByRole('radio', { name: 'Lag-aware sample exclusion', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Refresh preview' })).toBeVisible()
  await chapter(page, /Discovery lab/)
  const outcomes: Record<string, string[]> = {}
  for (const [method, family] of [['PCMCI+', 'PCMCI'], ['LPCMCI', 'PCMCI'], ['CD-NOTS', 'Nonstationary'], ['CD-NOTS+', 'Nonstationary'], ['GRACE', 'Neural']]) {
    await page.getByRole('radiogroup', { name: 'Discovery method family' }).getByRole('radio', { name: new RegExp(`^${family}`) }).check()
    await page.getByRole('radio', { name: new RegExp(`^${method.replace('+', '\\+')} Review`) }).check()
    await expect(page.getByRole('alert').filter({ hasText: 'No observed values in:' })).toHaveCount(0)
    const run = page.getByRole('button', { name: new RegExp(`^Run ${method.replace('+', '\\+')}(?: with|$)`) })
    await expect(run).toBeEnabled()
    await run.click()
    await expect(run).toBeEnabled({ timeout: 90_000 })
    outcomes[method] = await page.getByRole('alert').allTextContents()
  }
  console.log(JSON.stringify(outcomes))
  expect(outcomes).toEqual({ 'PCMCI+': [], LPCMCI: [], 'CD-NOTS': [], 'CD-NOTS+': [], GRACE: [] })
  await expect(page.getByRole('button', { name: /Runs \(5\)/ })).toBeVisible()
  await info.attach('discovery-outcomes', { body: JSON.stringify(outcomes, null, 2), contentType: 'application/json' })
  await page.screenshot({ path: info.outputPath('discovery-outcomes.png') })
})
