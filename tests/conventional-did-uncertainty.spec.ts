import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { addArrow, chapter, choose, createDag, identify } from './examples/support'

const cases = JSON.parse(readFileSync('tests/fixtures/did-uncertainty.json', 'utf8'))

test('conventional DiD worker preserves the estimate and respects inference availability', async ({
  page,
}) => {
  await page.goto('/app')
  const results = await page.evaluate(async (cases) => {
    const { runPanelIntervention } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const results = []
    for (const c of cases) {
      const outcomes = c.y.flat()
      const units = c.y.flatMap((row: number[], i: number) => row.map(() => String(i)))
      const times = c.y.flatMap((row: number[]) => row.map((_, j) => j))
      const treatment = c.y.flatMap((row: number[], i: number) =>
        row.map((_, j) => Number(i >= c.n0 && j >= c.t0)),
      )
      const run = (inference: unknown) =>
        runPanelIntervention(
          new Float64Array([...outcomes, ...treatment]),
          outcomes.length,
          units,
          times,
          { primary: 'did', placeboReplications: 40, seed: 731, inference },
        )
      const bootstrap = await run({ kind: 'bootstrap', replications: 40, seed: 731 })
      results.push({
        bootstrap,
        repeat: await run({ kind: 'bootstrap', replications: 40, seed: 731 }),
        jackknife: await run({ kind: 'jackknife' }),
        placebo: await run({ kind: 'placebo', replications: 40, seed: 731 }),
      })
    }
    return results
  }, cases)
  for (const [i, result] of results.entries()) {
    expect(result.repeat).toEqual(result.bootstrap)
    for (const method of ['bootstrap', 'jackknife', 'placebo'] as const) {
      const evidence = result[method]
      expect(evidence.ok, JSON.stringify(evidence)).toBe(true)
      expect(evidence.value.did.estimate).toBe(result.bootstrap.value.did.estimate)
      if (cases[i][method] === null) expect(evidence.value.uncertainty.kind).toBe('unavailable')
      else {
        expect(evidence.value.uncertainty.kind).toBe('available')
        expect(evidence.value.uncertainty.method).toBe(method)
        if (method === 'jackknife')
          expect(evidence.value.uncertainty.standardError).toBeCloseTo(cases[i].jackknife, 12)
      }
    }
  }
})

test('conventional DiD disables ineligible placebo and restores bootstrap and jackknife runs', async ({
  page,
}, info) => {
  const id = 'a9e4c2d7-8b31-4f5e-b6c0-3d7a9e2f5b82'
  const name = 'Open AI adoption, March cohort'
  await page.goto('/app')
  await page.getByRole('button', { name, exact: true }).click()
  await chapter(page, /Estimation/)
  const snapshot = () =>
    page.evaluate(async (id) => {
      const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
      const result = await store.loadProject(id)
      if (!result.ok) throw Error(result.error.kind)
      return result.value
    }, id)
  let count = (await snapshot()).estimationRuns.length
  await expect(page.getByRole('radio', { name: /^Placebo/ })).toBeDisabled()
  await expect(
    page.locator('p').filter({
      hasText:
        'Placebo uncertainty requires more control units than treated units. This panel has 3 treated and 3 control units.',
    }),
  ).toBeVisible()
  for (const [label, kind] of [
    ['Unit bootstrap', 'bootstrap'],
    ['Jackknife', 'jackknife'],
  ] as const) {
    await page.getByRole('radio', { name: label, exact: true }).check()
    if (kind === 'bootstrap') {
      await page
        .getByRole('radiogroup', { name: 'Conventional DiD uncertainty' })
        .scrollIntoViewIfNeeded()
      await page.screenshot({ path: info.outputPath('controls.png') })
    }
    await page.getByRole('button', { name: 'Run panel DiD', exact: true }).click()
    await expect.poll(async () => (await snapshot()).estimationRuns.length).toBe(++count)
    const run = (await snapshot()).estimationRuns.at(-1)
    expect(run.configuration.inference.kind).toBe(kind)
    expect(run.evidence.uncertainty.kind).toBe('available')
    expect(run.estimate.interval.kind).toBe('confidence')
    expect(run.estimate.standardError).toBeGreaterThan(0)
    expect(run.estimate.interval.lower).toBeCloseTo(
      run.estimate.effect.value - 1.96 * run.estimate.standardError,
      12,
    )
    await expect(page.getByText(/Approximate 95% confidence interval using/).first()).toBeVisible()
    await page.getByTestId('effect-estimate').first().scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath(`${kind}.png`) })
  }
  const saved = (await snapshot()).estimationRuns
  await page.setViewportSize({
    width: info.project.name === 'mobile-chromium' ? 412 : 1280,
    height: 1100,
  })
  const chart = page.getByTestId('run-comparison')
  if (info.project.name === 'mobile-chromium')
    await page.getByRole('button', { name: /^Runs \(/ }).click()
  await chart.scrollIntoViewIfNeeded()
  for (const theme of ['light', 'dark']) {
    await page.evaluate((theme) => {
      document.documentElement.dataset.theme = theme
    }, theme)
    await chart.screenshot({ path: info.outputPath(`comparison-${theme}.png`) })
  }
  await page.goto('/app/projects')
  await page.getByRole('button', { name, exact: true }).click()
  await chapter(page, /Estimation/)
  expect((await snapshot()).estimationRuns).toEqual(saved)
})

test('an eligible panel fits and restores placebo uncertainty through the controls', async ({
  page,
}, info) => {
  test.setTimeout(120_000)
  const fixture = cases.find((entry: { placebo: number | null }) => entry.placebo !== null)
  const rows = fixture.y.flatMap((values: number[], unit: number) =>
    values.map(
      (outcome, period) =>
        `${unit},${period},${Number(unit >= fixture.n0 && period >= fixture.t0)},${outcome}`,
    ),
  )
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Eligible placebo panel')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'placebo.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(['unit,period,treated,outcome', ...rows].join('\n')),
  })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /^Panel/ }).check()
  await choose(page, 'Unit column', 'unit')
  await choose(page, 'Time column', 'period')
  for (const column of ['treated', 'outcome'])
    await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('heading', { name: 'Next steps', exact: true })).toBeVisible()
  await createDag(page, 'Treatment and outcome')
  await addArrow(page, 'treated', 'outcome', 'Treatment may affect the outcome.')
  await identify(page, {
    graph: 'Treatment and outcome',
    treatment: 'treated',
    outcome: 'outcome',
    mechanism: 'Policy change',
    target: /Treated rows/,
    sentence: 'The treated units adopt together; comparison units remain untreated.',
  })
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: /^Interventions/ }).check()
  await page.getByRole('radio', { name: /Panel difference-in-differences/ }).check()
  await page.getByRole('radio', { name: 'Conventional', exact: true }).check()
  await expect(page.getByRole('radio', { name: 'Placebo', exact: true })).toBeEnabled()
  await page.getByRole('radio', { name: 'Placebo', exact: true }).check()
  await page.getByRole('spinbutton', { name: 'DiD replications' }).fill('40')
  await page.getByRole('button', { name: 'Run panel DiD', exact: true }).click()
  await expect(page.getByText(/Approximate 95% confidence interval using/).first()).toBeVisible()
  const estimate = await page.getByTestId('effect-estimate').first().innerText()
  const projectId = new URL(page.url()).pathname.split('/')[3]
  await expect
    .poll(() =>
      page.evaluate(async (id) => {
        const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
        const result = await store.loadProject(id)
        return result.ok ? result.value.estimationRuns.length : 0
      }, projectId),
    )
    .toBe(1)
  await page.getByTestId('effect-estimate').first().scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('eligible-placebo.png') })
  await page.reload()
  await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
  await page.locator('input[type="file"]').setInputFiles({
    name: 'placebo.csv',
    mimeType: 'text/csv',
    buffer: Buffer.from(['unit,period,treated,outcome', ...rows].join('\n')),
  })
  await chapter(page, /Estimation/)
  await expect(page.getByTestId('effect-estimate').first()).toHaveText(estimate, {
    useInnerText: true,
  })
})
