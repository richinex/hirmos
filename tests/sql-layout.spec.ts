import { expect, test } from '@playwright/test'
import { showSqlSource, hideSqlSource } from './sql-pane'

test('SQL workbench keeps the console stable and puts source controls in the responsive inspector', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('SQL layout')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
  await page.getByRole('button', { name: 'Open empty SQL editor' }).click()
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Open SQL file' })).toBeEnabled({ timeout: 30_000 })
  await hideSqlSource(page)
  const terminal = page.getByLabel('SQL console').locator('.xterm')
  await expect(terminal).toBeVisible()
  await terminal.evaluate(node => node.setAttribute('data-retained', 'yes'))
  const consoleHost = page.getByLabel('SQL console')
  await consoleHost.evaluate(node => {
    const heights: number[] = []
    const observer = new ResizeObserver(() => {
      heights.push(node.getBoundingClientRect().height)
      node.setAttribute('data-heights', JSON.stringify(heights))
    })
    observer.observe(node)
  })
  await expect(page.getByText('Input tables', { exact: true })).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Cancel query', exact: true })).toHaveCount(0)
  const mobile = info.project.name === 'mobile-chromium'
  await page.screenshot({ path: info.outputPath('sql-empty.png') })
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled()
  await expect(page.getByRole('button', { name: 'Use selected view' })).toBeDisabled()
  await hideSqlSource(page)
  const input = terminal.locator('.xterm-helper-textarea')
  await input.focus()
  await input.pressSequentially('SHOW TABLES;')
  await input.press('Enter')
  await expect(page.getByRole('button', { name: 'Cancel query', exact: true })).toHaveCount(0)
  await input.pressSequentially('CREATE VIEW example AS SELECT 42 AS answer;')
  await input.press('Enter')
  await showSqlSource(page)
  await page.getByRole('button', { name: 'Refresh views' }).click()
  await expect(page.getByRole('combobox', { name: 'Output view' })).toContainText('example')
  await expect(terminal).toHaveAttribute('data-retained', 'yes')
  if (!mobile) {
    const heights: number[] = JSON.parse(await consoleHost.getAttribute('data-heights') ?? '[]')
    expect(heights.length).toBeGreaterThan(0)
    expect(Math.max(...heights) - Math.min(...heights)).toBeLessThan(0.5)
  }
  await page.screenshot({ path: info.outputPath('sql-source.png') })
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
})
