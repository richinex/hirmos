import { expect, test } from '@playwright/test'

test('pipeline execution survives editor unmount and project disposal releases its session', async ({ page }) => {
  test.setTimeout(90000)
  await page.goto('/app')
  await page.evaluate(async () => {
    const { mount } = await import(new URL(`/tests/fixtures/pipeline-session-harness.tsx?run=${Date.now()}`, location.href).href)
    await mount()
  })
  const harness = page.locator('#pipeline-session-test')
  await expect(harness.getByRole('button', { name: 'Hide test editor' })).toBeVisible({ timeout: 10000 })
  await expect(harness.getByTestId('session-state')).toContainText('"running":true', { timeout: 60000 })
  await harness.getByRole('button', { name: 'Hide test editor' }).click()
  await expect(harness.getByTestId('pipeline-canvas')).toHaveCount(0)
  await expect(harness.getByTestId('session-state')).toContainText('"rows":3', { timeout: 30000 })
  await harness.getByRole('button', { name: 'Show test editor' }).click()
  await expect(harness.getByTestId('block-script')).toContainText('3 rows, 2 columns')
  await expect(harness.getByTestId('session-state')).toContainText('"openings":1')
  await harness.getByRole('button', { name: 'Close test project' }).click()
  await harness.getByRole('button', { name: 'Inspect closed session' }).click()
  await expect(harness.getByTestId('closed-session')).toHaveText('{"closed":"closed","active":null}')
})

test('closing the project during a script prevents a late pipeline result', async ({ page }) => {
  test.setTimeout(90000)
  await page.goto('/app')
  await page.evaluate(async () => {
    const { mount } = await import(new URL(`/tests/fixtures/pipeline-session-harness.tsx?run=${Date.now()}`, location.href).href)
    await mount()
  })
  const harness = page.locator('#pipeline-session-test')
  await expect(harness.getByTestId('session-state')).toContainText('"running":true', { timeout: 60000 })
  await harness.getByRole('button', { name: 'Close test project' }).click()
  await harness.getByRole('button', { name: 'Inspect closed session' }).click()
  await expect(harness.getByTestId('closed-session')).toHaveText('{"closed":"closed","active":null}')
  // Observe beyond the script's three-second delay, not only synchronous cleanup.
  await page.waitForTimeout(4000)
  await harness.getByRole('button', { name: 'Inspect closed session' }).click()
  await expect(harness.getByTestId('closed-run')).toHaveText('idle')
  await expect(harness.getByTestId('closed-session')).toHaveText('{"closed":"closed","active":null}')
})
