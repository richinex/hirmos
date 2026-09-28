import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

const full = process.env.HIRMOS_FULL_ORACLE === '1'

test('influence requests and display preserve distinct measures and signs', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const schemas = await import(new URL('/src/domain/gcmInfluence.ts', location.href).href)
    const charts = await import(new URL('/src/charts/gcmInfluence.ts', location.href).href)
    const request = { names: ['A', 'B'], edges: [[0, 1]], rows: 20, target: 1, random: { kind: 'seed', seed: 0 }, query: { kind: 'arrows', conditional: 20, maxRuns: 10, tolerance: 0.01 } }
    return {
      valid: schemas.gcmInfluenceRequestSchema.safeParse(request).success,
      root: schemas.gcmInfluenceRequestSchema.safeParse({ ...request, target: 0 }).success,
      unknown: schemas.gcmInfluenceRequestSchema.safeParse({ ...request, target: 2 }).success,
      tolerance: schemas.gcmInfluenceRequestSchema.safeParse({ ...request, query: { ...request.query, tolerance: -1 } }).success,
      missingSamples: schemas.gcmInfluenceRequestSchema.safeParse({ ...request, query: { kind: 'intrinsic' } }).success,
      shares: charts.absoluteShares([-2, 3]), zero: charts.absoluteShares([0, 0]),
    }
  })
  expect(result).toEqual({ valid: true, root: false, unknown: false, tolerance: false, missingSamples: false, shares: [40, 60], zero: null })
})

for (const dataset of ['mpg', 'river']) test(`${dataset}: actual influence buttons match the Docker source`, async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'The same populated result is checked at mobile size below.')
  test.setTimeout(full ? 900_000 : 180_000)
  page.setDefaultTimeout(20_000)
  const reference = JSON.parse(readFileSync('docs/2026-09-16-gcm-intrinsic-contribution/ui-reference.json', 'utf8'))
  const cases = reference.cases.filter((entry: { dataset: string }) => entry.dataset === dataset)
  const example = cases[0] as { names: string[]; edges: [number, number][]; target: number; rows: number }
  const text = readFileSync(`docs/2026-09-16-gcm-intrinsic-contribution/${dataset === 'mpg' ? 'auto_mpg' : 'river'}.csv`, 'utf8')
  const lines = text.trim().split(/\r?\n/)
  const header = lines[0]!.split(',')
  const selected = example.names.map(name => header.indexOf(name))
  expect(selected.every(index => index >= 0)).toBe(true)
  const csv = [example.names.join(','), ...lines.slice(1, full ? undefined : 201).map(line => { const row = line.split(','); return selected.map(index => row[index]).join(',') })].join('\n')
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.routeWebSocket(/.*/, socket => socket.close())
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(`${dataset} influence verification`)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles({ name: `${dataset}.csv`, mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data' }).click()
  const chooser = page.getByLabel(/^Choose visible columns,/)
  await expect(chooser).toBeVisible()
  await expect(chooser).toHaveText('view_column')
  await chooser.click()
  await expect(page.getByRole('list', { name: 'Shown columns' })).toBeVisible()
  await chooser.click()
  await page.getByRole('radio', { name: 'Independent observations' }).check()
  for (const name of example.names) await page.getByRole('checkbox', { name, exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  const navigation = page.getByRole('navigation', { name: 'Workspace sections' })
  await navigation.getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(`${dataset} reference graph`)
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  for (const [cause, effect] of example.edges) {
    for (const [label, name] of [['Proposed cause', example.names[cause]!], ['Proposed effect', example.names[effect]!]]) {
      await page.getByRole('combobox', { name: label }).click()
      await page.getByRole('option', { name, exact: true }).click()
    }
    await page.getByRole('textbox', { name: /Rationale/ }).first().fill('Relationship specified in the preserved DoWhy 0.14 intrinsic-contribution notebook.')
    await page.getByRole('button', { name: 'Add the arrow' }).click()
  }
  await page.getByRole('button', { name: 'Use for causal model analysis', exact: true }).click()
  for (const oracle of cases) {
    await page.getByRole('radio', { name: oracle.kind === 'intrinsic' ? 'Variance contributions' : 'Arrow strengths', exact: true }).check()
    await page.getByRole('combobox', { name: 'Influence target' }).click()
    await page.getByRole('option', { name: example.names[example.target]!, exact: true }).click()
    await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
    await expect(page.getByRole('region', { name: 'Causal influence result' })).toBeVisible({ timeout: full ? 600_000 : 120_000 })
    const saved = () => page.evaluate(async () => {
      const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
      const project = await store.loadProject((await store.listProjects())[0].id)
      return project.ok ? project.value.rootCause.influences.at(-1) : null
    })
    await expect.poll(async () => (await saved())?.model.query.kind).toBe(oracle.kind)
    const record = await saved()
    expect(record.model.names).toEqual(example.names)
    if (full) {
      for (let i = 0; i < record.evidence.outcome.nodes.length; i++) {
        const name = record.model.names[record.evidence.outcome.nodes[i]]
        const expected = oracle.estimates[name]
        expect(Math.abs(record.evidence.outcome.values[i] - expected), name).toBeLessThanOrEqual(1e-8 * (1 + Math.abs(expected)))
      }
      expect(record.evidence.random).toEqual(oracle.random)
    } else {
      expect(record.evidence.outcome.values.every(Number.isFinite)).toBe(true)
    }
    await info.attach(`${dataset}-${oracle.kind}.json`, { body: JSON.stringify(record, null, 2), contentType: 'application/json' })
    const chart = page.getByTestId('gcm-influence-chart')
    await chart.scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath(`${dataset}-${oracle.kind}-desktop.png`), fullPage: true })
    const label = oracle.kind === 'intrinsic' ? 'Intrinsic variance contributions' : 'Incoming-arrow strengths'
    await page.getByRole('button', { name: `Open ${label} in a floating window` }).click()
    await expect(page.getByRole('dialog', { name: label, exact: true })).toBeVisible()
    await page.getByRole('button', { name: 'Close the floating window', exact: true }).click()
    if (oracle.kind === 'arrows') {
      const before = await chart.boundingBox()
      await page.getByRole('radio', { name: 'Bars', exact: true }).check()
      const after = await chart.boundingBox()
      expect(after?.height).toBe(before?.height)
      const result = page.getByRole('region', { name: 'Causal influence result' })
      const heading = await result.getByRole('heading').first().boundingBox()
      const selector = await page.getByRole('radiogroup', { name: 'Strength plot' }).boundingBox()
      expect(selector!.y - (heading!.y + heading!.height)).toBeLessThan(150)
    }
    await page.setViewportSize({ width: 390, height: 844 })
    if (oracle.kind === 'arrows') {
      const group = page.getByRole('radiogroup', { name: 'Strength plot' })
      await group.getByRole('radio', { name: 'Bars', exact: true }).check()
      await expect(group.getByRole('radio', { name: 'Bars', exact: true })).toBeChecked()
      await group.getByRole('radio', { name: 'Graph', exact: true }).check()
      await expect(group.getByRole('radio', { name: 'Graph', exact: true })).toBeChecked()
      await expect(group.locator('.msym')).toHaveText(['account_tree', 'bar_chart'])
      expect(await group.getByText('Graph', { exact: true }).evaluate(el => getComputedStyle(el).position)).toBe('absolute')
    }
    await chart.scrollIntoViewIfNeeded()
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
    await page.screenshot({ path: info.outputPath(`${dataset}-${oracle.kind}-mobile.png`), fullPage: true })
    await page.setViewportSize({ width: 1440, height: 1000 })
  }
  await navigation.getByRole('button', { name: /Results/ }).click()
  await expect(page.getByRole('region', { name: 'Causal influence result' })).toHaveCount(2)
  expect(errors).toEqual([])
})
