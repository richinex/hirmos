import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose } from './examples/support'
import { snapshot, connectAssistant, ask } from './assistantTestFlow'

test('CR1 counts retained rank and positive-weight clusters; omitted covariance remains CR2', async ({
  page,
}) => {
  const fixture = JSON.parse(
    readFileSync('crates/causal-core/oracle/fixtures/estimation.json', 'utf8'),
  ).cluster
  await page.goto('/app')
  const result = await page.evaluate(async (f) => {
    const { runPanelRegression } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const columns = [
      f.y.concat(100),
      f.x.map((r: number[]) => r[1]).concat(3),
      f.x.map((r: number[]) => r[2]).concat(4),
      f.groups.concat(999),
      f.x.map((r: number[]) => r[1]).concat(3),
      f.y.map(() => 7).concat(0),
    ]
    const names = ['y', 'lead', 'post', 'cluster', 'copy', 'weight']
    const specification = {
      kind: 'interactions',
      predictors: [1, 2, 4].map((column) => ({
        column,
        name: names[column],
        coding: { kind: 'numeric' },
      })),
      terms: [[0], [1], [2]],
      covariance: 'cr1',
    }
    const request = {
      rows: columns[0].length,
      columns: 6,
      names,
      outcome: 0,
      cluster: 3,
      weights: 5,
      confidence: 0.95,
      specification,
    }
    const run = (r: unknown, cols = columns) => runPanelRegression(new Float64Array(cols.flat()), r)
    const selected = await run(request)
    const oneCluster = await run(
      request,
      columns.map((c, i) => (i === 3 ? c.map(() => 0) : c)),
    )
    const negative = await run(
      request,
      columns.map((c, i) => (i === 5 ? c.map(() => -1) : c)),
    )
    const cr2 = {
      ...specification,
      covariance: 'cr2',
      predictors: specification.predictors.slice(0, 2),
      terms: [[0], [1]],
    }
    const explicit = await run({ ...request, specification: cr2 })
    const { covariance: _, ...legacy } = cr2
    const implicit = await run({ ...request, specification: legacy })
    return { selected, oneCluster, negative, explicit, implicit }
  }, fixture)
  expect(result.selected.ok).toBe(true)
  const e = result.selected.value
  expect(e.observations).toBe(fixture.y.length)
  expect(e.clusters).toBe(new Set(fixture.groups).size)
  expect(e.terms).toHaveLength(3)
  expect(e.omitted).toHaveLength(1)
  const original = (name: string) => (name === 'Intercept' ? 0 : name === 'post' ? 2 : 1)
  for (const [i, t] of e.terms.entries()) {
    expect(t.estimate).toBeCloseTo(fixture.params[original(t.name)], 9)
    for (const [j, u] of e.terms.entries())
      expect(e.covariance[i][j]).toBeCloseTo(
        fixture.covariance[original(t.name)][original(u.name)],
        10,
      )
  }
  expect(result.oneCluster.ok).toBe(false)
  expect(result.negative.ok).toBe(false)
  expect(result.explicit.ok).toBe(true)
  expect(result.implicit.ok).toBe(true)
  expect(result.implicit.value.terms).toEqual(result.explicit.value.terms)
  expect(result.implicit.value.covariance).toEqual(result.explicit.value.covariance)
})

test('custom CR1 regression retains its covariance through coverage, review and reload', async ({
  page,
}) => {
  test.setTimeout(120000)
  const fixture = JSON.parse(
    readFileSync('crates/causal-core/oracle/fixtures/estimation.json', 'utf8'),
  ).cluster
  const csv = [
    'outcome,cluster,lead,post',
    ...fixture.y.map((y: number, i: number) =>
      [y, fixture.groups[i], fixture.x[i][1], fixture.x[i][2]].join(','),
    ),
  ].join('\n')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Custom coverage')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page
    .locator('input[type=file]')
    .setInputFiles({ name: 'custom.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /Independent observations/ }).click()
  await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('heading', { name: 'Next steps', exact: true })).toBeVisible()
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Regression designs', exact: true }).click()
  await page.getByRole('radio', { name: 'Interactions / DDD', exact: true }).click()
  await choose(page, 'Outcome', 'outcome')
  await choose(page, 'Cluster column', 'cluster')
  await expect(page.getByRole('radio', { name: 'CR2', exact: true })).toBeChecked()
  await page.getByRole('radio', { name: 'CR1', exact: true }).click()
  for (const name of ['lead', 'post'])
    await page
      .getByRole('group', { name: 'Predictors', exact: true })
      .getByRole('checkbox', { name, exact: true })
      .check()
  await page.getByRole('button', { name: 'Fit interaction regression', exact: true }).click()
  await expect
    .poll(async () => (await snapshot(page, 'Custom coverage'))?.timeSeriesRuns.length)
    .toBe(1)
  const source = (await snapshot(page, 'Custom coverage')).timeSeriesRuns[0]
  expect(source.specification.specification.covariance).toBe('cr1')
  for (let i = 0; i < 3; i++) {
    expect(source.evidence.terms[i].estimate).toBeCloseTo(fixture.params[i], 9)
    expect(source.evidence.terms[i].degreesOfFreedom).toBe(new Set(fixture.groups).size - 1)
    for (let j = 0; j < 3; j++)
      expect(source.evidence.covariance[i][j]).toBeCloseTo(fixture.covariance[i][j], 10)
  }
  await chapter(page, /Sensitivity/)
  await page.getByRole('radio', { name: 'Pre-test coverage', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Calculate coverage', exact: true })).toBeDisabled()
  await choose(page, 'Pre-treatment coefficient', 'lead')
  await choose(page, 'Post-treatment coefficient', 'post')
  await page.getByLabel('Omitted reference period', { exact: true }).fill('-1')
  await page.getByRole('button', { name: 'Calculate coverage', exact: true }).click()
  await expect
    .poll(async () => (await snapshot(page, 'Custom coverage'))?.sensitivityRuns.length)
    .toBe(1)
  const saved = await snapshot(page, 'Custom coverage')
  const coverage = saved.sensitivityRuns[0]
  expect(coverage.eventStudy.names).toEqual(['lead', 'post'])
  expect(coverage.eventStudy.covariance).toEqual([
    [source.evidence.covariance[1][1], source.evidence.covariance[1][2]],
    [source.evidence.covariance[2][1], source.evidence.covariance[2][2]],
  ])
  const boundaries = await page.evaluate(
    async ({ source, coverage }) => {
      const d = await import(new URL('/src/domain/pretestCoverage.ts', location.href).href)
      const candidate = d.pretestCandidates([], [source])[0]
      const settings = coverage.settings
      const altered = structuredClone(source)
      altered.evidence.covariance[1][1] *= 2
      return {
        matched: d.pretestRunMatches(coverage, [], [source]),
        changed: d.pretestRunMatches(coverage, [], [altered]),
        missing: d.pretestInput(candidate, { ...settings, coefficients: undefined }).ok,
        unknown: d.pretestInput(candidate, {
          ...settings,
          coefficients: { ...settings.coefficients, pre: 999 },
        }).ok,
        duplicate: d.pretestInput(candidate, {
          ...settings,
          coefficients: { ...settings.coefficients, pre: settings.coefficients.post },
        }).ok,
        reference: d.pretestInput(candidate, {
          ...settings,
          coefficients: { ...settings.coefficients, reference: 0 },
        }).ok,
      }
    },
    { source, coverage },
  )
  expect(boundaries).toEqual({
    matched: true,
    changed: false,
    missing: false,
    unknown: false,
    duplicate: false,
    reference: false,
  })
  await page.reload()
  await expect(
    page.getByRole('heading', { name: 'Choose the data file again', exact: true }),
  ).toBeVisible()
  await page
    .locator('input[type=file]')
    .setInputFiles({ name: 'custom.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
  await chapter(page, /Sensitivity/)
  await expect(page.getByRole('article', { name: 'Pre-test coverage result' })).toBeVisible()
  const requests = await connectAssistant(page, [
    `s = await tool("workspace_settings", {"key":"pretest-coverage"})
c = s["current"]
c["coefficients"]["reference"] = 0
p = await tool("propose_workspace", {"key":"pretest-coverage", "configuration":c, "reason":"Check the reference boundary."})
assert p["kind"] == "refused", p
v = await tool("run", {"workspace":"sensitivity", "id":"${coverage.id}", "view":"review"})
assert v["review"]["coefficients"] == {"pre":"lead", "post":"post"}, v
assert v["review"]["periods"]["reference"] == -1, v
await FINAL_VAR("Read the custom coverage review; invalid reference refused.", ["saved-run"])`,
  ])
  await ask(page, 'Check the reference boundary and read the saved coverage review.')
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'invalid reference refused',
    { timeout: 45000 },
  )
  expect(requests()).toBe(1)
})
