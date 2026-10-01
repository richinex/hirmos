import { expect, test, type Page } from '@playwright/test'

const id = '7c2f1b3e-5a64-4d1e-9b0a-2e6f8c1d4a71'
const name = 'AI adoption, company-wide'
const saved = (page: Page) => page.evaluate(async id => {
  const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
  const result = await store.loadProject(id)
  if (!result.ok) throw Error(result.error.kind)
  return result.value
}, id)

for (const origin of ['current', 'older', 'unstamped'] as const) {
  test(`opening preserves ${origin} example edits; only confirmed reset adopts a new release`, async ({ page }, info) => {
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible()
    await page.goto('/app/projects')
    await page.evaluate(async ({ id, origin }) => {
      const store = await import(new URL('/src/data/projectStore.ts', location.href).href)
      const loaded = await store.loadProject(id)
      if (!loaded.ok) throw Error(loaded.error.kind)
      const snapshot = loaded.value
      const changed = {
        ...snapshot, project: { ...snapshot.project, name: 'My edited example' },
        origin: origin === 'unstamped' ? { kind: 'user' }
          : origin === 'older' ? { kind: 'shipped-example', exportedAt: '2025-01-01T00:00:00.000Z' } : snapshot.origin,
      }
      const result = await store.saveProject(changed)
      if (!result.ok) throw Error(result.error.kind)
    }, { id, origin })
    const before = await saved(page)
    // A new app release is served, while the saved browser copy still belongs to the user.
    await page.route('**/examples/ai-adoption-company-wide.hirmos.json', async route => {
      const response = await route.fetch()
      const bundle = await response.json()
      bundle.exportedAt = '2026-10-01T12:00:00.000Z'
      bundle.project.project.name = 'Updated shipped example'
      await route.fulfill({ response, json: bundle })
    })
    await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible()
    await expect.poll(async () => (await saved(page)).project.name).toBe('My edited example')
    const reopened = await saved(page)
    expect(reopened.estimationRuns).toEqual(before.estimationRuns)
    expect(reopened.studies).toEqual(before.studies)
    expect(reopened.origin).toEqual(before.origin)
    await page.goto('/app/projects')
    const beforeCancel = await saved(page)
    await page.getByRole('button', { name: `Reset ${name}`, exact: true }).click()
    await page.getByRole('alertdialog').getByRole('button', { name: 'Cancel', exact: true }).click()
    expect(await saved(page)).toEqual(beforeCancel)
    expect((await saved(page)).project.name).toBe('My edited example')
    await page.getByRole('button', { name: `Reset ${name}`, exact: true }).click()
    await page.screenshot({ path: info.outputPath('explicit-reset.png') })
    await page.getByRole('alertdialog').getByRole('button', { name: 'Reset example', exact: true }).click()
    await expect.poll(async () => (await saved(page)).project.name).toBe('Updated shipped example')
    expect((await saved(page)).origin).toEqual({ kind: 'shipped-example', exportedAt: '2026-10-01T12:00:00.000Z' })
    await expect(page).toHaveURL(/\/app\/projects$/)
    await page.getByRole('button', { name: `Open ${name}`, exact: true }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible()
  })
}
