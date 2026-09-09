import { expect, test } from '@playwright/test'
import { parseSourceRecipe } from '../src/domain/sqlPreparation'
import { choose, prepare } from './examples/support'

const createSqlProject = async (page: import('@playwright/test').Page) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('SQL preparation')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.getByRole('radio', { name: 'Prepare with SQL' }).click()
}

const selectTwoInputs = async (page: import('@playwright/test').Page) => {
  await page.locator('input[type="file"][multiple]').setInputFiles([
    { name: 'measurements.csv', mimeType: 'text/csv', buffer: Buffer.from('id,value\n1,10\n2,20\n') },
    { name: 'groups.csv', mimeType: 'text/csv', buffer: Buffer.from('id,group_name\n1,A\n2,B\n') },
  ])
  await expect(page.getByLabel('SQL console').locator('.xterm')).toBeVisible({ timeout: 30_000 })
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled({ timeout: 30_000 })
  await expect(page.getByRole('button', { name: 'Use selected view' })).toBeDisabled()
}

const runSql = async (page: import('@playwright/test').Page, sql: string) => {
  const terminalInput = page.getByLabel('SQL console').locator('.xterm-helper-textarea')
  await terminalInput.focus()
  await terminalInput.pressSequentially(sql)
  await terminalInput.press('Enter')
  // The upstream canvas terminal does not expose its prompt text to the accessibility tree.
  await page.waitForTimeout(100)
}

const refreshViews = async (page: import('@playwright/test').Page, selected: string) => {
  await page.getByRole('button', { name: 'Refresh views' }).click()
  await expect(page.getByRole('combobox', { name: 'Output view' })).toBeVisible()
  // The house Select is a listbox behind a combobox trigger, not a native select.
  await page.getByRole('combobox', { name: 'Output view' }).click()
  await page.getByRole('option', { name: selected, exact: true }).click()
}

test('materializes a declared multi-file SQL view through the ordinary profile path', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The canonical SQL engine test runs once')
  const warnings: string[] = []
  page.on('console', (message) => {
    if (message.type() === 'warning') warnings.push(message.text())
  })
  await createSqlProject(page)
  await selectTwoInputs(page)

  await page.getByRole('button', { name: 'Cancel query' }).click()
  await expect(page.getByRole('status')).toHaveText('No query was running.')

  await runSql(page, 'CREATE OR REPLACE VIEW joined_measurements AS SELECT m.id, m.value, g.group_name FROM measurements m JOIN groups g USING (id);')
  await runSql(page, 'CREATE OR REPLACE VIEW analysis_rows AS SELECT * FROM joined_measurements;')
  await refreshViews(page, 'analysis_rows')
  await page.getByRole('button', { name: 'Use selected view' }).click()

  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await expect(page.getByText(/prepared with SQL · analysis_rows · 2 inputs/)).toBeVisible()
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
  await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('group_name')
  expect(warnings).not.toContain('using deprecated parameters for the initialization function; pass a single object instead')
})

test('reopens a project built by the SQL step from its input files', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The replay check runs once')
  await createSqlProject(page)
  await selectTwoInputs(page)
  await runSql(page, 'CREATE OR REPLACE VIEW hirmos_prepared AS SELECT m.id, m.value, g.group_name FROM measurements m JOIN groups g USING (id);')
  await refreshViews(page, 'hirmos_prepared')
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
  await page.waitForTimeout(1500)

  await page.goto('/app')
  await page.getByRole('list', { name: 'Projects' }).getByRole('button', { name: 'Open' }).first().click()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await expect(page.getByText(/the SQL step created from measurements.csv/)).toBeVisible()
  await page.locator('input[type="file"][multiple]').setInputFiles([
    { name: 'measurements.csv', mimeType: 'text/csv', buffer: Buffer.from('id,value\n1,10\n2,20\n') },
    { name: 'groups.csv', mimeType: 'text/csv', buffer: Buffer.from('id,group_name\n1,A\n2,B\n') },
  ])
  await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 60_000 })
  await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('group_name')
})

test('opens the official SQL shell again after clearing a prepared source', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The shell lifecycle test runs once')
  await createSqlProject(page)
  await selectTwoInputs(page)

  await runSql(page, 'CREATE OR REPLACE VIEW hirmos_prepared AS SELECT * FROM measurements;')
  await refreshViews(page, 'hirmos_prepared')
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Choose another file' }).click()

  await selectTwoInputs(page)
  await expect(page.getByLabel('SQL console').locator('.xterm')).toBeVisible()
  await expect(page.getByRole('button', { name: 'Refresh views' })).toBeEnabled()
})

test('keeps the SQL preparation workspace inside a phone viewport', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'This is the phone layout check')
  await createSqlProject(page)
  await selectTwoInputs(page)
  const width = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }))
  expect(width.content).toBeLessThanOrEqual(width.viewport)
})

test('parses only complete source recipes with distinct input aliases', () => {
  const fingerprint = 'a'.repeat(64)
  const valid = parseSourceRecipe({
    kind: 'sql-derived',
    outputView: 'hirmos_prepared',
    statement: 'CREATE VIEW hirmos_prepared AS SELECT * FROM measurements',
    inputs: [{ alias: 'measurements', fileName: 'measurements.csv', bytes: 12, format: 'csv', fingerprint }],
  })
  expect(valid.ok).toBe(true)

  const duplicate = parseSourceRecipe({
    kind: 'sql-derived',
    outputView: 'hirmos_prepared',
    statement: 'CREATE VIEW hirmos_prepared AS SELECT * FROM measurements',
    inputs: [
      { alias: 'measurements', fileName: 'one.csv', bytes: 12, format: 'csv', fingerprint },
      { alias: 'measurements', fileName: 'two.csv', bytes: 12, format: 'csv', fingerprint },
    ],
  })
  expect(duplicate).toEqual({ ok: false, error: { kind: 'invalid-source-recipe', detail: 'Invalid SQL input alias: measurements' } })
})

test('round-trips a SQL source recipe through project persistence', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Saved-project persistence runs once')
  await page.goto('/app')
  const recipes = await page.evaluate(async () => {
    const persistence = await import(new URL('/src/domain/persistence.ts', window.location.href).href)
    const project = {
      kind: 'hirmos-project',
      version: 1,
      savedAt: '2026-09-09T00:00:00.000Z',
      project: { id: 'sql-project', name: 'SQL project', createdAt: '2026-09-09T00:00:00.000Z' },
      profile: null,
      prepared: null,
      stationarity: null,
      grangerEvidence: [],
      countSeriesModels: [],
      discoveryRuns: [],
      dagDocuments: [],
      dagChecks: [],
      interventionQueries: [],
      studyDraft: {},
      studies: [],
      identifications: [],
      estimationRuns: [],
      sensitivityRuns: [],
      counterfactualRuns: [],
      survivalRuns: [],
    }
    const source = {
      name: 'hirmos-prepared.parquet',
      bytes: 128,
      mediaType: 'application/vnd.apache.parquet',
      lastModified: 1,
      format: 'parquet',
    }
    const sql = persistence.parseSnapshotValue({
      ...project,
      source: {
        ...source,
        recipe: {
          kind: 'sql-derived',
          outputView: 'hirmos_prepared',
          statement: 'CREATE VIEW hirmos_prepared AS SELECT * FROM measurements',
          inputs: [{
            alias: 'measurements',
            fileName: 'measurements.csv',
            bytes: 24,
            format: 'csv',
            fingerprint: 'a'.repeat(64),
          }],
        },
      },
    })
    if (!sql.ok) return sql
    const roundTrip = persistence.parseSnapshot(persistence.serialiseSnapshot(sql.value))
    return roundTrip.ok ? roundTrip.value.source?.recipe : roundTrip
  })

  expect(recipes).toEqual({
    kind: 'sql-derived',
    outputView: 'hirmos_prepared',
    statement: 'CREATE VIEW hirmos_prepared AS SELECT * FROM measurements',
    inputs: [{
      alias: 'measurements',
      fileName: 'measurements.csv',
      bytes: 24,
      format: 'csv',
      fingerprint: 'a'.repeat(64),
    }],
  })
})

test('refuses a prepared view that depends on an unrecorded shell table', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The canonical SQL isolation test runs once')
  await createSqlProject(page)
  await selectTwoInputs(page)

  await runSql(page, 'CREATE TABLE hidden AS SELECT * FROM measurements;')
  await runSql(page, 'CREATE OR REPLACE VIEW hirmos_prepared AS SELECT * FROM hidden;')
  await refreshViews(page, 'hirmos_prepared')
  await page.getByRole('button', { name: 'Use selected view' }).click()

  await expect(page.getByRole('alert')).toContainText('The prepared view could not be materialized', { timeout: 30_000 })
  await expect(page.getByRole('heading', { name: 'Source selected' })).toHaveCount(0)
})

test('runs grouped survival analysis from two SQL inputs through the ordinary preparation path', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'The complete SQL-to-WASM path runs once')
  await createSqlProject(page)
  await page.locator('input[type="file"][multiple]').setInputFiles([
    { name: 'cohorts.csv', mimeType: 'text/csv', buffer: Buffer.from('cohort,stratum,entry_time,observation_end,entrants\njan,A,0,5,10\nfeb,A,1,5,5\n') },
    { name: 'events.csv', mimeType: 'text/csv', buffer: Buffer.from('cohort,event_time,event_count\njan,2,2\njan,4,1\nfeb,3,2\n') },
  ])
  await expect(page.getByLabel('SQL console').locator('.xterm')).toBeVisible({ timeout: 30_000 })

  await runSql(page, 'CREATE OR REPLACE VIEW grouped_survival AS WITH observed AS (SELECT e.event_time - c.entry_time AS duration, 1 AS status, e.event_count AS frequency FROM events e JOIN cohorts c USING (cohort)), totals AS (SELECT cohort, sum(event_count) AS observed FROM events GROUP BY cohort), censored AS (SELECT c.observation_end - c.entry_time AS duration, 0 AS status, c.entrants - coalesce(t.observed, 0) AS frequency FROM cohorts c LEFT JOIN totals t USING (cohort)) SELECT * FROM observed UNION ALL SELECT * FROM censored WHERE frequency > 0;')
  await refreshViews(page, 'grouped_survival')
  await page.getByRole('button', { name: 'Use selected view' }).click()
  await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 30_000 })
  await prepare(page, { structure: 'cross-section', columns: ['duration', 'status', 'frequency'] })

  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Survival analysis/ }).click()
  await page.getByRole('radio', { name: 'Kaplan–Meier' }).click()
  await choose(page, 'Duration', 'duration')
  await choose(page, 'Event · 1 observed, 0 censored', 'status')
  await page.getByRole('radio', { name: 'Grouped count' }).click()
  await choose(page, 'Frequency', 'frequency')
  await page.getByRole('button', { name: 'Run survival analysis' }).click()

  await expect(page.getByRole('heading', { name: 'Kaplan–Meier and Nelson–Aalen' }).first()).toBeVisible({ timeout: 120_000 })
  await expect(page.getByText('15 observations · 5 events').first()).toBeVisible()
  await expect(page.getByTestId('kaplan-meier-curve').first()).toBeVisible()
  await expect(page.getByTestId('nelson-aalen-curve').first()).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Curve values and period increments' }).first()).toBeVisible()
  await expect(page.getByText('Survival runs · 1')).toBeVisible()
})
