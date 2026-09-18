import { expect, test } from '@playwright/test'
import { showSqlSource } from './sql-pane'
import { exportDirectory } from '../src/domain/sourceInputs'
import { existsSync } from 'node:fs'
import { resolve } from 'node:path'

test('book export can be selected, prepared and reopened through the UI', async ({ page }) => {
  test.setTimeout(120_000)
  const folder = resolve('../octopus/rust-causal-transpile/duckdb-examples-main/ch03/ch03_db')
  test.skip(!existsSync(folder), 'Local book export fixture required')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Book database')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click({ force: true })
  await page.getByLabel('DuckDB export folder').setInputFiles(folder)
  await page.getByRole('button', { name: 'Import database', exact: true }).click()
  await showSqlSource(page)
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Refresh views' }).click()
  await page.getByRole('combobox', { name: 'Output view' }).click()
  await page.getByRole('option', { name: 'v_power_per_day', exact: true }).click()
  await page.screenshot({ path: 'test-results/database-import.png', fullPage: true })
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Edit SQL', exact: true }).click()
  await showSqlSource(page)
  await expect(page.getByRole('combobox', { name: 'Output view' })).toContainText('v_power_per_day', { timeout: 30_000 })
  await page.getByRole('button', { name: 'Use selected view', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible()
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.waitForTimeout(1000)
  await page.reload()
  await page.getByRole('button', { name: 'Open Book database', exact: true }).click()
  await page.getByLabel('DuckDB export folder').setInputFiles(folder)
  await page.getByRole('button', { name: 'Import database', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
})

test('export paths identify one complete folder without traversal', () => {
  expect(exportDirectory(['db/schema.sql', 'db/load.sql']).ok).toBe(true)
  for (const paths of [[], ['db/schema.sql'], ['db/schema.sql', 'other/load.sql'], ['db/schema.sql', 'db/load.sql', 'db/../escape'], ['db/schema.sql', 'db/load.sql', 'db/load.sql']]) {
    expect(exportDirectory(paths).ok).toBe(false)
  }
})

test('DuckDB imports, replays and rejects changed dependencies and unsaved mutations', async ({ page }) => {
  test.setTimeout(90_000)
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const sql = await import(new URL('/src/data/sqlPreparation.ts', location.href).href)
    const file = (name: string, content: string) => {
      const value = new File([content], name)
      Object.defineProperty(value, 'webkitRelativePath', { value: `sample/${name}` })
      return value
    }
    const files = [file('schema.sql', 'CREATE TABLE observations(id INTEGER PRIMARY KEY, value INTEGER); CREATE VIEW prepared AS SELECT * FROM observations ORDER BY id;'), file('load.sql', "COPY observations FROM 'sample/data.csv' (HEADER);"), file('data.csv', 'id,value\n1,20\n2,30\n')]
    const inputs = await sql.prepareDatabaseExport(files)
    if (!inputs.ok) throw new Error(JSON.stringify(inputs.error))
    const opened = await sql.openSqlPreparation(inputs.value)
    if (!opened.ok) throw new Error(JSON.stringify(opened.error))
    try {
      const output = await sql.materializePreparedView(opened.value, 'prepared')
      if (!output.ok) throw new Error(JSON.stringify(output.error))
      const replay = await sql.replaySqlRecipe(output.value.recipe, files)
      if (!replay.ok) throw new Error(JSON.stringify(replay.error))
      const changed = await sql.replaySqlRecipe(output.value.recipe, [...files.slice(0, 2), file('data.csv', 'id,value\n1,99\n2,30\n')])
      const missing = await sql.replaySqlRecipe(output.value.recipe, files.slice(0, 2))
      const connection = await opened.value.database.connect()
      await connection.query('UPDATE observations SET value = 99 WHERE id = 1')
      await connection.close()
      const mutation = await sql.materializePreparedView(opened.value, 'prepared')
      return { rows: output.value.rowCount, same: String(new Uint8Array(await output.value.file.arrayBuffer())) === String(new Uint8Array(await replay.value.arrayBuffer())), changed: changed.ok ? null : changed.error.kind, missing: missing.ok ? null : missing.error.kind, mutation: mutation.ok ? null : mutation.error.kind }
    } finally { await sql.closeSqlPreparation(opened.value) }
  })
  expect(result).toEqual({ rows: 2, same: true, changed: 'replay-inputs-missing', missing: 'replay-inputs-missing', mutation: 'prepared-view-invalid' })
})

test('invalid export SQL returns an import error', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const sql = await import(new URL('/src/data/sqlPreparation.ts', location.href).href)
    const files = ['schema.sql', 'load.sql'].map(name => {
      const file = new File([name === 'schema.sql' ? 'THIS IS NOT SQL;' : ''], name)
      Object.defineProperty(file, 'webkitRelativePath', { value: `broken/${name}` })
      return file
    })
    const inputs = await sql.prepareDatabaseExport(files)
    if (!inputs.ok) throw new Error(JSON.stringify(inputs.error))
    const opened = await sql.openSqlPreparation(inputs.value)
    if (opened.ok) { await sql.closeSqlPreparation(opened.value); return null }
    return opened.error.kind
  })
  expect(result).toBe('input-registration-failed')
})
