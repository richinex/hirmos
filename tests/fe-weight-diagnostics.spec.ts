import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import reference from './fixtures/fe-weights.json' with { type: 'json' }
import { chapter, choose, identifyEffect } from './examples/support'

test('fixed-effects weights match R through the worker, including unsupported designs', async ({
  page,
}) => {
  await page.goto('/app')
  const results = await page.evaluate(async (rows) => {
    const path = '/src/analysis/client.ts'
    const { runBackdoorLinear } = await import(path)
    const run = (adjustment: number[], fixedEffects: unknown) => {
      const columns = [
        rows.map((r) => r.x),
        rows.map((r) => r.y),
        rows.map((r) => r.g),
        rows.map((_, i) => Math.sin(i)),
        rows.map((_, i) => i % 8),
      ]
      return runBackdoorLinear(new Float64Array(columns.flat()), rows.length, columns.length, {
        treatment: 0,
        outcome: 1,
        adjustment,
        hacMaxLags: null,
        level: 0.95,
        errorModel: { kind: 'neweyWest' },
        fixedEffects,
      })
    }
    return {
      eligible: await run([], { kind: 'unit', column: 2 }),
      adjusted: await run([3], { kind: 'unit', column: 2 }),
      twoWay: await run([], { kind: 'unitAndTime', unit: 2, time: 4 }),
      plain: await run([], null),
    }
  }, reference.data)
  for (const result of Object.values(results)) expect(result.ok, JSON.stringify(result)).toBe(true)
  if (!results.eligible.ok || !results.adjusted.ok || !results.twoWay.ok || !results.plain.ok)
    return
  const d = results.eligible.value.weightDiagnostic
  expect(d.kind).toBe('available')
  if (d.kind !== 'available') return
  expect(d.giniVariance).toBeCloseTo(reference.diagnostics.concentration.gini_variance, 10)
  expect(d.effectiveGroups).toBeCloseTo(reference.diagnostics.concentration.effective_groups, 10)
  for (const [i, group] of d.groups.entries()) {
    expect(group.label).toBe(reference.diagnostics.groups[i]!.group)
    expect(group.weight).toBeCloseTo(reference.diagnostics.groups[i]!.weight, 10)
  }
  for (const [i, point] of d.dropout.entries()) {
    expect(point.fit.kind).toBe('estimated')
    if (point.fit.kind === 'estimated')
      expect(point.fit.coefficient).toBeCloseTo(reference.dropout[i]!.coefficient!, 10)
  }
  expect(d.density).toHaveLength(512)
  expect(results.adjusted.value.weightDiagnostic.kind).toBe('unavailable')
  expect(results.twoWay.value.weightDiagnostic.kind).toBe('unavailable')
  expect(results.plain.value.weightDiagnostic.kind).toBe('notApplicable')
})

test('a fitted diagnostic is visible, exported and restored with its regression', async ({
  page,
}, info) => {
  test.setTimeout(180_000)
  await page.setViewportSize({ width: 1600, height: 1200 })
  const errors: string[] = []
  page.on('pageerror', (error) => errors.push(error.message))
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('FE identifying weights')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  const csv = ['x,y,g', ...reference.data.map((r) => [r.x, r.y, r.g].join(','))].join('\n')
  await page
    .locator('input[type=file]')
    .setInputFiles({ name: 'fe-weights.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByText('Choose the observation structure').waitFor({ timeout: 60_000 })
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await page.getByRole('heading', { name: 'Next steps', exact: true }).waitFor()
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('One grouping')
  await page.getByRole('button', { name: 'Create DAG draft', exact: true }).click()
  await page.getByRole('button', { name: 'From text', exact: true }).click()
  await page
    .getByRole('textbox', { name: 'Graph text' })
    .fill('dag { x [exposure] y [outcome] g x -> y }')
  await page.getByRole('button', { name: 'Convert to DAG', exact: true }).click()
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', 'One grouping')
  await choose(page, 'Treatment', 'x')
  await choose(page, 'Outcome', 'y')
  await page.getByRole('radio', { name: /Observed choice/ }).click()
  await page
    .getByRole('textbox', { name: 'Assignment sentence' })
    .fill('A numerical example for checking the one-grouping regression diagnostic.')
  await identifyEffect(page)
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()
  await page
    .getByRole('radiogroup', { name: 'Fixed effects' })
    .getByRole('radio', { name: 'By unit', exact: true })
    .click()
  await choose(page, 'Unit column', 'g')
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  const result = page.getByRole('region', { name: 'Fixed-effects weight diagnostics' })
  await expect(result).toBeVisible({ timeout: 60_000 })
  await result
    .getByRole('heading', { name: 'Fixed-effects weight diagnostics', exact: true })
    .evaluate((node) => node.scrollIntoView({ block: 'start' }))
  await page.screenshot({ path: info.outputPath('fe-weights-light.png') })
  await expect(result.getByTestId('fe-weights-dropout')).toBeVisible()
  await page.getByRole('button', { name: 'Change theme', exact: true }).click()
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark')
  await expect(page.locator('html')).not.toHaveAttribute('data-theme-transition', '')
  await page.screenshot({ path: info.outputPath('fe-weights-dark.png') })
  const expand = page.getByRole('button', { name: 'Expand section list', exact: true })
  if (await expand.isVisible()) await expand.click()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export project', exact: true }).click()
  const bundle = JSON.parse(await readFile((await (await download).path())!, 'utf8'))
  const saved = bundle.project.estimationRuns.at(-1).evidence.weightDiagnostic
  expect(saved.kind).toBe('available')
  expect(saved.dropout[0].fit.coefficient).toBeCloseTo(reference.dropout[0]!.coefficient!, 10)
  await page.reload()
  await page.getByRole('heading', { name: 'Choose the data file again', exact: true }).waitFor()
  const chooser = page.waitForEvent('filechooser')
  await page.getByRole('button', { name: 'Choose data file', exact: true }).click()
  await (await chooser).setFiles({ name: 'fe-weights.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await chapter(page, /Estimation/)
  await expect(result).toBeVisible()
  expect(errors).toEqual([])
})
