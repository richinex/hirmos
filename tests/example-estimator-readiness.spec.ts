import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { SHIPPED_EXAMPLES } from '../src/domain/example'
import { chapter } from './examples/support'

// Opening a pristine example must restore a runnable estimation route, not merely its old result.
// This checks readiness without rerunning or replacing historical estimates.
for (const example of SHIPPED_EXAMPLES.filter(example => example.estimationRuns > 0)) {
  test(`shipped estimator readiness: ${example.name}`, async ({ page }, info) => {
    const candidate = example.sourceName === 'cohort-march.csv' ? process.env.COHORT_EXAMPLE_CANDIDATE : undefined
    const body = await readFile(candidate ?? `public${example.bundleUrl}`, 'utf8')
    const latest = JSON.parse(body).project.estimationRuns.at(-1)
    expect(latest).toBeDefined()
    // Verify a repaired export before publishing it as the bundled release.
    if (candidate !== undefined) {
      await page.route(`**${example.bundleUrl}`, route => route.fulfill({ contentType: 'application/json', body }))
    }
    await page.goto('/app')
    await page.getByRole('button', { name: `Open ${example.name}`, exact: true }).click()
    await chapter(page, /Estimation/)
    await expect(page.locator(`input[type="radio"][value="${latest.configuration.kind}"]`)).toBeChecked()
    const run = page.getByRole('button', { name: /^Run / }).first()
    await expect(run).toBeVisible()
    await expect.soft(run, `${example.name}: the restored estimator must be runnable`).toBeEnabled({ timeout: 15_000 })
    await run.scrollIntoViewIfNeeded()
    await page.screenshot({ path: info.outputPath('restored-estimator.png') })
    await info.attach('visible-estimation-copy', { body: await page.locator('body').innerText(), contentType: 'text/plain' })
  })
}
