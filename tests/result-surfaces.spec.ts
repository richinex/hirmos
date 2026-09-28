import { expect, test } from '@playwright/test'
import { prepare } from './examples/support'

test('shared surfaces wrap long values and retain virtual table behaviour', async ({ page }, info) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ React, createRoot }, figures, evidence, formats] = await Promise.all([
      load('/tests/support/reactRuntime.ts'),
      load('/src/components/ui/figures.tsx'), load('/src/components/table/EvidenceTable.tsx'), load('/src/lib/format/number.ts'),
    ])
    const h = React.createElement
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;overflow:auto;padding:16px;background:var(--color-panel)'
    document.body.append(host)
    const rows = Array.from({ length: 500 }, (_, id) => ({ id, name: `Variable ${id} with a long explanatory name`, value: id - 250 }))
    const columns = [{ id: 'name', header: 'Variable', value: (r: any) => r.name }, { id: 'value', header: 'Estimate', align: 'right', value: (r: any) => r.value }]
    createRoot(host).render(h('div', null,
      h(figures.MetricGrid, { label: 'Stress metrics' },
        ...['Short label', 'A much longer metric label describing a model comparison', 'Uncertainty interval'].map((label, i) => h(figures.MetricTile, { key: label, label, value: formats.formatWords(i === 2 ? '−123,456.78 to 987,654.32' : '123,456,789.12'), context: 'Supporting information with enough words to wrap at small widths.' }))),
      h(evidence.EvidenceTable, { title: 'Stress estimates', rows, columns, rowKey: (r: any) => String(r.id), noun: 'variable', empty: 'No values', frame: 'none', exportName: 'stress' }),
      h(evidence.EvidenceTable, { title: 'Data preview control', appearance: 'data', rows: rows.slice(0, 2), columns, rowKey: (r: any) => String(r.id), noun: 'row', empty: 'No rows' })))
  })
  const table = page.getByRole('region', { name: 'Stress estimates', exact: true })
  await expect(table).toBeVisible()
  await expect(table.getByRole('table')).toHaveAttribute('aria-rowcount', '501')
  const renderedRows = await table.locator('tbody tr[aria-rowindex]').count()
  expect(renderedRows).toBeGreaterThan(0)
  expect(renderedRows).toBeLessThan(500)
  for (const width of [320, 390, 768, 1440]) {
    await page.setViewportSize({ width, height: 1000 })
    for (const tile of await page.locator('.metric-tile').all()) expect(await tile.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
    await page.screenshot({ path: info.outputPath(`stress-${width}.png`) })
  }
  const scroller = table.locator('.figure-strip')
  await scroller.evaluate(el => { el.scrollTop = el.scrollHeight })
  await expect(table.getByText('Variable 499 with a long explanatory name', { exact: true })).toBeVisible()
  await table.getByRole('searchbox').fill('Variable 499 ')
  await expect(table.locator('tbody tr')).toHaveCount(1)
  await expect(page.getByRole('region', { name: 'Data preview control', exact: true })).not.toHaveClass(/evidence-minimal/)
})

test('shared metrics and minimal evidence table in a fitted survival result', async ({ page }, info) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Result surface verification')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles('docs/2026-09-11-rviews-survival-veteran/data/veteran.csv')
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await prepare(page, { structure: 'cross-section', columns: ['time', 'status', 'age', 'karno'] })
  const toggle = page.getByRole('button', { name: 'Expand section list' })
  if (await toggle.isVisible()) await toggle.click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Survival analysis/ }).click()
  const setup = page.locator('section[aria-labelledby="survival-setup-title"]')
  await setup.getByRole('radio', { name: 'Aalen regression', exact: true }).click({ force: true })
  await setup.getByRole('group', { name: 'Covariates', exact: true }).getByRole('checkbox', { name: 'age', exact: true }).check()
  await setup.getByRole('button', { name: 'Run survival analysis' }).click()
  const table = page.getByRole('region', { name: 'Additive coefficient summary', exact: true }).first()
  await expect(table).toBeVisible({ timeout: 60_000 })
  await expect(table).toHaveClass(/evidence-minimal/)
  const metrics = page.locator('.metric-cards').filter({ hasText: 'Events used' }).first()
  await metrics.scrollIntoViewIfNeeded()
  await expect(metrics.locator('.metric-tile')).toHaveCount(3)
  for (const tile of await metrics.locator('.metric-tile').all()) {
    expect(await tile.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
  }
  await page.screenshot({ path: info.outputPath('survival-metrics.png'), fullPage: true })
  await table.scrollIntoViewIfNeeded()
  await table.getByRole('button', { name: 'Term', exact: true }).click()
  await expect(table.getByRole('columnheader', { name: 'Term', exact: true })).toHaveAttribute('aria-sort', 'ascending')
  await table.getByRole('searchbox').fill('age')
  await expect(table.locator('tbody tr')).toHaveCount(1)
  const download = page.waitForEvent('download')
  await table.getByRole('button', { name: 'Export CSV', exact: true }).click()
  expect((await download).suggestedFilename()).toBe('aalen-coefficients.csv')
  await table.getByRole('radio', { name: 'Compact rows' }).check()
  await expect(table.getByRole('radio', { name: 'Compact rows' })).toBeChecked()
  expect(await table.locator('tbody tr').first().evaluate(el => getComputedStyle(el).borderBottomColor)).toBe('rgba(0, 0, 0, 0)')
  await page.screenshot({ path: info.outputPath('survival-table.png'), fullPage: true })
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
})
