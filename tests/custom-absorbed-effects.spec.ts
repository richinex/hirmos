import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose } from './examples/support'
import { snapshot } from './assistantTestFlow'

const fixtures = JSON.parse(
  readFileSync('crates/causal-core/oracle/fixtures/fixest_cluster.json', 'utf8'),
).cases

test('absorption requires a distinct recorded column and refuses invalid fits', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async (f) => {
    const { runPanelRegression } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const { panelRegressionRequestSchema } = await import(
      new URL('/src/domain/panelRegression.ts', location.href).href
    )
    const { checkPanelRegressionDraft, panelRegressionColumns } = await import(
      new URL('/src/domain/regressionWorkspaceSettings.ts', location.href).href
    )
    const names = ['y', 'lead', 'post', 'unit', 'weight']
    const specification = {
      kind: 'interactions',
      covariance: 'fixest',
      absorbed: 3,
      predictors: [1, 2].map((column) => ({
        column,
        name: names[column],
        coding: { kind: 'numeric' },
      })),
      terms: [[0], [1]],
    }
    const request = {
      rows: f.y.length,
      columns: 5,
      names,
      outcome: 0,
      cluster: 3,
      weights: 4,
      confidence: 0.95,
      specification,
    }
    const columns = [
      f.y,
      f.x.map((r: number[]) => r[0]),
      f.x.map((r: number[]) => r[1]),
      f.unit,
      f.y.map(() => 1),
    ]
    const invalid = [
      { ...specification, absorbed: undefined },
      { ...specification, covariance: 'cr2' },
      { ...specification, absorbed: 0 },
      { ...specification, absorbed: 1 },
      { ...specification, absorbed: 4 },
      { ...specification, absorbed: 99 },
    ].map(
      (specification) =>
        panelRegressionRequestSchema.safeParse({ ...request, specification }).success,
    )
    const negative = await runPanelRegression(
      new Float64Array(columns.map((c, j) => (j === 4 ? c.map(() => -1) : c)).flat()),
      request,
    )
    const absorbed = await runPanelRegression(
      new Float64Array(columns.map((c, j) => (j === 1 || j === 2 ? f.unit : c)).flat()),
      request,
    )
    const draft = {
      outcome: 'y',
      cluster: 'unit',
      weight: null,
      confidence: '.95',
      model: {
        kind: 'interactions',
        covariance: 'fixest',
        absorbed: 'unit',
        predictors: [{ column: 'lead', coding: { kind: 'numeric' } }],
        terms: [['lead']],
      },
    }
    return {
      invalid,
      negative: negative.ok,
      absorbed: absorbed.ok,
      validDraft: checkPanelRegressionDraft(draft, false).ok,
      missingDraft: checkPanelRegressionDraft(
        { ...draft, model: { ...draft.model, absorbed: null } },
        false,
      ).ok,
      columns: panelRegressionColumns(draft),
    }
  }, fixtures.unit_unit_FALSE)
  expect(result.invalid).toEqual([false, false, false, false, false, false])
  expect(result.negative).toBe(false)
  expect(result.absorbed).toBe(false)
  expect(result.validDraft).toBe(true)
  expect(result.missingDraft).toBe(false)
  expect(result.columns).toContain('unit')
})

test('custom absorbed regressions reuse the saved weighted and unweighted fixest references', async ({
  page,
}) => {
  await page.goto('/app')
  for (const [name, f] of Object.entries(fixtures) as [string, any][]) {
    if (f.fe !== 'unit' && f.fe !== 'time') continue
    const result = await page.evaluate(async (f) => {
      const { runPanelRegression } = await import(
        new URL('/src/analysis/client.ts', location.href).href
      )
      const columns = [
        f.y,
        f.x.map((r: number[]) => r[0]),
        f.x.map((r: number[]) => r[1]),
        f.cluster,
        f.fe === 'unit' ? f.unit : f.time,
        f.weighted ? f.weights : f.y.map(() => 1),
      ]
      const names = ['y', 'lead', 'post', 'cluster', 'effect', 'weight']
      const request = {
        rows: f.y.length + 1,
        columns: 6,
        names,
        outcome: 0,
        cluster: 3,
        weights: 5,
        confidence: 0.95,
        specification: {
          kind: 'interactions',
          covariance: 'fixest',
          absorbed: 4,
          predictors: [1, 2].map((column) => ({
            column,
            name: names[column],
            coding: { kind: 'numeric' },
          })),
          terms: [[0], [1]],
        },
      }
      const values = new Float64Array(
        columns.flatMap((c: number[], j: number) => [...c, j === 5 ? 0 : 999]),
      )
      const fixest = await runPanelRegression(values.slice(), request)
      const xtreg = await runPanelRegression(values, {
        ...request,
        specification: { ...request.specification, covariance: 'xtreg' },
      })
      return { fixest, xtreg }
    }, f)
    expect(result.fixest.ok, name).toBe(true)
    expect(result.xtreg.ok, name).toBe(true)
    const e = result.fixest.value
    const other = result.xtreg.value
    const ratio = (f.y.length - f.k) / (f.y.length - 3)
    expect(e.observations).toBe(f.y.length)
    expect(e.terms.map((t: any) => t.name)).toEqual(['lead', 'post'])
    expect(e.omitted).toEqual(['Intercept'])
    for (let i = 0; i < 2; i++) {
      expect(e.terms[i].estimate, name).toBeCloseTo(f.coefficients[i], 9)
      expect(e.terms[i].standardError, name).toBeCloseTo(f.se[i], 9)
      expect(e.terms[i].degreesOfFreedom, name).toBe(f.df)
      expect(other.terms[i].estimate, name).toBeCloseTo(e.terms[i].estimate, 10)
      expect(other.terms[i].degreesOfFreedom, name).toBe(f.df)
      for (let j = 0; j < 2; j++) {
        expect(e.covariance[i][j], name).toBeCloseTo(f.covariance[i][j], 9)
        expect(other.covariance[i][j], name).toBeCloseTo(f.covariance[i][j] * ratio, 9)
      }
    }
  }
})

for (const covariance of ['fixest', 'xtreg'] as const)
  test(`absorbed ${covariance} setting survives fitting, coverage and reload without fixed-effect dummy columns`, async ({
    page,
  }) => {
    const f = fixtures.unit_unit_FALSE
    const csv = [
      'y,lead,post,unit',
      ...f.y.map((y: number, i: number) => [y, ...f.x[i], f.unit[i]].join(',')),
    ].join('\n')
    const file = { name: 'within.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) }
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill('Absorbed coverage')
    await page.getByRole('button', { name: 'Create project', exact: true }).click()
    await page.locator('input[type=file]').setInputFiles(file)
    await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
    await page.getByRole('radio', { name: /Independent observations/ }).click()
    await page.getByRole('button', { name: 'Select all columns', exact: true }).click()
    await page.getByRole('button', { name: /Create prepared/ }).click()
    await expect(page.getByRole('heading', { name: 'Next steps', exact: true })).toBeVisible()
    await chapter(page, /Estimation/)
    await page.getByRole('radio', { name: 'Regression designs', exact: true }).click()
    await page.getByRole('radio', { name: 'Interactions / DDD', exact: true }).click()
    await choose(page, 'Outcome', 'y')
    await choose(page, 'Cluster column', 'unit')
    for (const name of ['lead', 'post'])
      await page
        .getByRole('group', { name: 'Predictors', exact: true })
        .getByRole('checkbox', { name, exact: true })
        .check()
    await page
      .getByRole('radio', {
        name:
          covariance === 'fixest' ? 'CR1 with absorbed effects' : 'CR1 excluding absorbed effects',
        exact: true,
      })
      .click()
    await expect(
      page.getByRole('button', { name: 'Fit interaction regression', exact: true }),
    ).toBeDisabled()
    await choose(page, 'Absorbed fixed effect', 'unit')
    await page.getByRole('button', { name: 'Fit interaction regression', exact: true }).click()
    await expect
      .poll(async () => (await snapshot(page, 'Absorbed coverage'))?.timeSeriesRuns.length)
      .toBe(1)
    const run = (await snapshot(page, 'Absorbed coverage')).timeSeriesRuns[0]
    expect(run.evidence.terms).toHaveLength(2)
    expect(run.specification.specification.covariance).toBe(covariance)
    expect(run.specification.names[run.specification.specification.absorbed]).toBe('unit')
    await chapter(page, /Sensitivity/)
    await page.getByRole('radio', { name: 'Pre-test coverage', exact: true }).click()
    await choose(page, 'Pre-treatment coefficient', 'lead')
    await choose(page, 'Post-treatment coefficient', 'post')
    await page.getByRole('button', { name: 'Calculate coverage', exact: true }).click()
    await expect
      .poll(async () => (await snapshot(page, 'Absorbed coverage'))?.sensitivityRuns.length)
      .toBe(1)
    const saved = await snapshot(page, 'Absorbed coverage')
    expect(saved.sensitivityRuns[0].eventStudy.covariance).toEqual(run.evidence.covariance)
    await page.reload()
    await expect(
      page.getByRole('heading', { name: 'Choose the data file again', exact: true }),
    ).toBeVisible()
    await page.locator('input[type=file]').setInputFiles(file)
    await chapter(page, /Sensitivity/)
    await expect(page.getByRole('article', { name: 'Pre-test coverage result' })).toBeVisible()
  })
