import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('clearing an obsolete notice preserves active runs and other analyses', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { createJobs } = await import(new URL('/src/analysis/jobs.ts', location.href).href)
    let cancellations = 0
    const jobs = createJobs(() => {
      cancellations++
    })
    const first = jobs.start('estimation', 'analysis', 'Fitting')
    jobs.clearNotice('estimation')
    const running = jobs.current('estimation', first)
    jobs.fail('estimation', first, 'Insufficient comparisons')
    const other = jobs.start('sensitivity', 'analysis', 'Checking')
    jobs.fail('sensitivity', other, 'Other failure')
    jobs.clearNotice('estimation')
    const cleared = jobs.store.getState().jobs.estimation.kind
    const preserved = jobs.store.getState().jobs.sensitivity.kind
    const next = jobs.start('estimation', 'analysis', 'Fitting')
    const restarted = next !== null && jobs.current('estimation', next)
    jobs.finish('estimation', next)
    return { running, cleared, preserved, restarted, cancellations }
  })
  expect(result).toEqual({
    running: true,
    cleared: 'idle',
    preserved: 'failed',
    restarted: true,
    cancellations: 0,
  })
})

test('a small never-treated refusal does not block a not-yet-treated fit', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await chapter(page, /Estimation/)
  const help = page.getByRole('button', { name: 'About Estimation', exact: true })
  await help.focus()
  await expect(
    page
      .getByText('Choose an estimator for the identified effect and run the analysis.', {
        exact: true,
      })
      .first(),
  ).toBeVisible()
  await page.keyboard.press('Escape')
  await page.getByRole('radio', { name: 'Staggered adoption', exact: true }).check()
  await page.getByRole('radio', { name: 'Analytical', exact: true }).check()
  const run = page.getByRole('button', { name: 'Run panel DiD', exact: true })
  await run.click()
  await expect(page.getByRole('alert')).toContainText(
    'The never-treated comparison group has 3 units, but this specification requires at least 5.',
  )
  await page.getByRole('radio', { name: 'Not yet treated', exact: true }).check()
  await expect(page.getByRole('alert')).toHaveCount(0)
  await expect(run).toBeEnabled()
  await run.click()
  await expect
    .poll(() =>
      page.evaluate(async () => {
        const { loadProject } = await import(
          new URL('/src/data/projectStore.ts', location.href).href
        )
        const saved = await loadProject('a9e4c2d7-8b31-4f5e-b6c0-3d7a9e2f5b82')
        const last = saved.ok ? saved.value.estimationRuns.at(-1) : null
        return last?.evidence.kind === 'staggeredDid'
          ? last.configuration.specification.controls
          : null
      }),
    )
    .toBe('not-yet-treated')
  await expect(page.getByRole('alert')).toHaveCount(0)
})
