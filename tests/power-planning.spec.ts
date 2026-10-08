import { test, expect } from '@playwright/test'
import { readFileSync } from 'node:fs'

test('analytical power matches the pinned oracle through the worker', async ({ page }) => {
  const data = JSON.parse(readFileSync('tests/fixtures/power/fixtures.json', 'utf8'))
  const cases = data.power.filter(
    (r: { n: number; d: number; value: number | null }) =>
      r.value !== null && r.n >= 10 && r.n < 10000 && Math.abs(r.d) <= 2,
  )
  await page.goto('/app')
  const outputs = await page.evaluate(async (cases) => {
    const { calculatePower } = await import(new URL('/src/analysis/client.ts', location.href).href)
    return Promise.all(
      cases.map(async (r: Record<string, number | string>) => {
        const value = await calculatePower({
          kind: 'analytical',
          design: { kind: 'independentT', ratio: r.ratio },
          alternative: r.alternative === 'two-sided' ? 'twoSided' : r.alternative,
          alpha: r.alpha,
          target: { kind: 'power', n: r.n, effect: r.d },
        })
        if (!value.ok) throw Error(JSON.stringify({ r, error: value.error }))
        return value.value.power
      }),
    )
  }, cases)
  expect(cases.length).toBeGreaterThan(50)
  for (let i = 0; i < cases.length; i++)
    expect(Math.abs(outputs[i] - cases[i].value)).toBeLessThanOrEqual(1e-9)
})

test('planning UI calculates, refuses invalid values, and records simulations', async ({
  page,
}, testInfo) => {
  await page.goto('/app')
  await page
    .getByRole('button', { name: 'Open A simulated process with a collider', exact: true })
    .click()
  if (testInfo.project.name === 'mobile-chromium')
    await page.getByRole('button', { name: 'Expand section list' }).click()
  await page.getByRole('button', { name: /^Data studio/ }).click()
  const panel = page.getByTestId('power-planning')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await expect(panel.getByTestId('power-result')).toContainText(
    'Minimum detectable standardized effect',
  )
  await expect(panel.getByTestId('power-result')).toContainText('0.8')
  await panel.screenshot({ path: testInfo.outputPath('planning-analytical.png') })
  await panel.getByLabel('Group 1 size', { exact: true }).fill('1')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await expect(panel.getByRole('alert')).toContainText('at least two')
  await panel.getByRole('radiogroup', { name: 'Planning calculation' }).getByRole('radio', { name: 'Simulation', exact: true }).click()
  await panel.getByLabel('Replications', { exact: true }).fill('100')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await expect(panel.getByTestId('power-result')).toContainText('Estimated power')
  await panel.getByRole('radio', { name: /^Two-period difference-in-differences/ }).check()
  await panel.getByLabel('Treatment effect', { exact: true }).fill('0')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await expect(panel.getByTestId('power-result')).toContainText('Type-I rejection rate')
  await panel.screenshot({ path: testInfo.outputPath('planning-simulation.png') })
})

test('sample sizes are rounded and power recalculated; paired and simulation designs run', async ({
  page,
}) => {
  await page.goto('/app')
  const results = await page.evaluate(async () => {
    const { calculatePower } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const requests = [
      {
        kind: 'analytical',
        design: { kind: 'independentT', ratio: 1.3 },
        alternative: 'twoSided',
        alpha: 0.05,
        target: { kind: 'sampleSize', effect: 0.5, target: 0.8 },
      },
      {
        kind: 'analytical',
        design: { kind: 'pairedT' },
        alternative: 'twoSided',
        alpha: 0.05,
        target: { kind: 'power', effect: 0.5, n: 50 },
      },
      ...['independent', 'equalClusters', 'twoPeriodDid'].map((kind) => ({
        kind: 'simulation',
        scenario: {
          kind,
          treated: 20,
          controls: 20,
          ...(kind === 'equalClusters'
            ? { members: 5, icc: 0.2 }
            : kind === 'twoPeriodDid'
              ? { correlation: 0.5, violation: 0 }
              : {}),
        },
        effect: 0.5,
        sd: 1,
        replications: 100,
        seed: 1,
        alpha: 0.05,
      })),
    ]
    return Promise.all(requests.map(async (request) => await calculatePower(request)))
  })
  for (const r of results) expect(r.ok, JSON.stringify(r)).toBe(true)
  const a = results[0].value
  expect(a.n).toBe(Math.ceil(a.continuousN))
  expect(a.secondGroup).toBe(Math.ceil(a.n * 1.3))
  expect(a.power).toBeGreaterThanOrEqual(0.8)
})

test('planning persists across reload and cancellation does not replace the saved result', async ({
  page,
}, info) => {
  await page.goto('/app')
  await page
    .getByRole('button', { name: 'Open A simulated process with a collider', exact: true })
    .click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  const panel = page.getByTestId('power-planning')
  const literature = panel.getByText(/^Literature: statsmodels, power and sample-size calculations/)
  await expect(literature).not.toBeVisible()
  await panel.getByText('Planning assumptions', { exact: true }).click()
  await expect(literature).toBeVisible()
  await panel.getByText('Planning assumptions', { exact: true }).click()
  await panel.getByLabel('Group 1 size', { exact: true }).fill('80')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await expect(panel.getByTestId('power-result')).toContainText('Observations in group 1')
  await expect(panel.getByTestId('power-result')).toContainText('80 in group 2')
  await expect
    .poll(() =>
      page.evaluate(async () => {
        const { loadProject } = await import(
          new URL('/src/data/projectStore.ts', location.href).href
        )
        const p = await loadProject('9c5b2e7a-1d38-4f64-a2b9-7e4c1f8d3a25')
        return p.ok ? p.value.powerPlanning?.result.n : null
      }),
    )
    .toBe(80)
  // Sticky routing reopens the project on reload.
  await page.reload()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await expect(panel.getByLabel('Group 1 size', { exact: true })).toHaveValue('80')
  await expect(panel.getByTestId('power-result')).toContainText('Observations in group 1')
  await expect(panel.getByTestId('power-result')).toContainText('80 in group 2')
  await page.setViewportSize({ width: 1440, height: 1500 })
  await panel.scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('planning-light.png') })
  await page.getByRole('button', { name: 'Change theme' }).click()
  await page.screenshot({ path: info.outputPath('planning-dark.png') })
  // Resizing a phone viewport switches to the desktop layout, which mounts the fold closed.
  if ((await panel.getAttribute('open')) === null)
    await panel.getByRole('radiogroup', { name: 'Planning calculation' }).getByRole('radio', { name: 'Simulation', exact: true }).click()
  await panel.getByLabel('Replications', { exact: true }).fill('10000')
  await panel.getByLabel('Treated units', { exact: true }).fill('1000')
  await panel.getByLabel('Control units', { exact: true }).fill('1000')
  await panel.getByRole('button', { name: 'Calculate planning result' }).click()
  await panel.getByRole('button', { name: 'Cancel run' }).click()
  await expect(panel.getByText('The analysis was cancelled.')).toBeVisible()
  await expect(panel.getByTestId('power-result')).toContainText('Observations in group 1')
  await expect(panel.getByTestId('power-result')).toContainText('80 in group 2')
})
