import { expect, test } from '@playwright/test'
import { readFile } from 'node:fs/promises'
import { SHIPPED_EXAMPLES } from '../src/domain/example'
import { chapter } from './examples/support'
import { parseBundle } from '../src/domain/bundle'
import { headlineValue } from '../src/domain/estimation'

for (const example of SHIPPED_EXAMPLES.filter(example => example.estimationRuns > 0)) {
  test(`rerun and restore shipped estimator: ${example.name}`, async ({ page }, info) => {
    test.skip(info.project.name !== 'chromium', 'Numerical reruns once; readiness also covers mobile')
    test.setTimeout(240_000)
    const bundle = parseBundle(await readFile(`public${example.bundleUrl}`, 'utf8'))
    if (!bundle.ok) throw Error(bundle.error.kind)
    const project = bundle.value.project
    const previous = project.estimationRuns.at(-1)
    if (previous === undefined) throw Error('Missing example estimate')
    await page.goto('/app')
    await page.getByRole('button', { name: `Open ${example.name}`, exact: true }).click()
    await chapter(page, /Estimation/)
    await expect(page.locator(`input[type="radio"][value="${previous.configuration.kind}"]`)).toBeChecked()
    await page.getByRole('button', { name: /^Run / }).first().click()
    const snapshot = () => page.evaluate(async id => {
      const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
      const result = await store.loadProject(id)
      if (!result.ok) throw Error(result.error.kind)
      return result.value
    }, example.id)
    await expect.poll(async () => (await snapshot()).estimationRuns.length, { timeout: 180_000 }).toBe(project.estimationRuns.length + 1)
    const fresh = (await snapshot()).estimationRuns.at(-1)
    expect(fresh.configuration).toEqual(previous.configuration)
    expect(fresh.estimate.estimand).toEqual(previous.estimate.estimand)
    expect(fresh.estimate.effect.kind).toBe(previous.estimate.effect.kind)
    expect(Math.abs(headlineValue(fresh.estimate.effect) - headlineValue(previous.estimate.effect))).toBeLessThan(1e-6 * Math.max(1, Math.abs(headlineValue(previous.estimate.effect))))
    await page.screenshot({ path: info.outputPath('rerun-result.png') })
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${example.name}`, exact: true }).click()
    await chapter(page, /Estimation/)
    expect((await snapshot()).estimationRuns.at(-1)).toEqual(fresh)
    await expect(page.locator(`input[type="radio"][value="${fresh.configuration.kind}"]`)).toBeChecked()
  })
}
