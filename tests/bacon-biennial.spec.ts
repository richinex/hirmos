import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose } from './examples/support'
const oracle = JSON.parse(readFileSync('crates/causal-core/oracle/fixtures/bacon.json', 'utf8'))
  .cases.castle

test('Bacon accepts biennial years, preserves labels, and matches consecutive-period decomposition', async ({
  page,
}, info) => {
  test.setTimeout(120000)
  const times: number[] = [
    ...new Set<number>(oracle.data.map((r: { time: number }) => r.time)),
  ].sort((a, b) => a - b)
  const year = (time: number) => 1992 + 2 * times.indexOf(time)
  const csv = [
    'unit,year,outcome,adopted',
    ...oracle.data.map((r: { id: number; time: number; y: number; treated: number }) =>
      [r.id, year(r.time), r.y, r.treated].join(','),
    ),
  ].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Biennial Bacon')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page
    .locator('input[type=file]')
    .setInputFiles({ name: 'biennial.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Panel/ }).click()
  await choose(page, 'Unit column', 'unit')
  await choose(page, 'Time column', 'year')
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(
    page.getByRole('heading', { name: 'Next steps', exact: true }),
  ).toBeVisible()
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Regression designs', exact: true }).click()
  await page.getByRole('radio', { name: 'Bacon decomposition', exact: true }).click()
  await choose(page, 'Outcome', 'outcome')
  await choose(page, 'Treatment indicator', 'adopted')
  await page.getByRole('button', { name: 'Decompose coefficient', exact: true }).click()
  const result = page.getByRole('region', { name: 'Bacon decomposition result', exact: true })
  await expect(result).toBeVisible({ timeout: 60000 })
  const firstAdoption = Math.min(
    ...oracle.data
      .filter((r: { treated: number }) => r.treated === 1)
      .map((r: { time: number }) => r.time),
  )
  await expect(result).toContainText('Adoption ' + year(firstAdoption))
  await result.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('biennial-bacon.png'), animations: 'disabled' })
  await expect
    .poll(async () =>
      page.evaluate(async () => {
        const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
        const projects = await store.listProjects()
        const project = projects.find((p: { name: string }) => p.name === 'Biennial Bacon')
        if (!project) return null
        const saved = await store.loadProject(project.id)
        return saved.ok ? saved.value.timeSeriesRuns.at(-1)?.kind : null
      }),
    )
    .toBe('bacon')
  const compared = await page.evaluate(async (data) => {
    const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const client = await import(new URL('/src/analysis/client.ts', location.href).href)
    const projects = await store.listProjects()
    const project = projects.find((p: { name: string }) => p.name === 'Biennial Bacon')
    const saved = await store.loadProject(project.id)
    if (!saved.ok) throw Error('Missing saved run')
    const run = saved.value.timeSeriesRuns.at(-1)
    const times = [...new Set<number>(data.map((r: { time: number }) => r.time))].sort(
      (a, b) => a - b,
    )
    const baseline = await client.runBacon(
      Float64Array.from(data.flatMap((r: { y: number; treated: number }) => [r.y, r.treated])),
      {
        rows: data.length,
        columns: 2,
        units: data.map((r: { id: number }) => String(r.id)),
        times: data.map((r: { time: number }) => times.indexOf(r.time)),
        outcome: 0,
        treatment: 1,
        specification: { kind: 'unadjusted' },
      },
    )
    if (!baseline.ok) throw Error(JSON.stringify(baseline.error))
    return { evidence: run.evidence, baseline: baseline.value, periods: run.periods }
  }, oracle.data)
  // CSV parsing and unit ordering can change the last floating-point bit.
  expect(compared.evidence.twfe).toBeCloseTo(compared.baseline.twfe, 12)
  expect(compared.evidence.reconstructed).toBeCloseTo(compared.baseline.reconstructed, 12)
  const components = compared.evidence.decomposition.components
  expect(components).toHaveLength(compared.baseline.decomposition.components.length)
  components.forEach((component: { comparison: unknown; estimate: number; weight: number }, index: number) => {
    const expected = compared.baseline.decomposition.components[index]
    expect(component.comparison).toEqual(expected.comparison)
    expect(component.estimate).toBeCloseTo(expected.estimate, 12)
    expect(component.weight).toBeCloseTo(expected.weight, 12)
  })
  expect(Math.abs(compared.evidence.twfe - oracle.twfe)).toBeLessThan(1e-8)
  expect(compared.periods).toEqual(times.map((t) => String(year(t))))
})
