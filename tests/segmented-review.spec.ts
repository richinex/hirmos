import { expect, test } from '@playwright/test'

test.beforeEach(async ({ page }) => {
  await page.goto('/app')
  await page.evaluate(async () => { const { mount } = await import(new URL('/tests/fixtures/segmented-review.tsx', location.href).href); mount() })
})

test('selection previews locally and commits once on release', async ({ page }) => {
  const group = page.getByRole('radiogroup', { name: 'Transform', exact: true })
  const a = (await group.getByRole('radio', { name: 'Keep levels' }).boundingBox())!
  const b = (await group.getByRole('radio', { name: 'Linear detrend' }).boundingBox())!
  await page.mouse.move(a.x + a.width / 2, a.y + a.height / 2)
  await page.mouse.down()
  await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2, { steps: 8 })
  await expect(page.getByTestId('changes')).toHaveText('0')
  await expect(group.locator('[data-segment-preview="true"]')).toContainText('Linear detrend')
  await page.mouse.up()
  await expect(page.getByTestId('selection')).toHaveText('detrend')
  await expect(page.getByTestId('changes')).toHaveText('1')
  await group.getByRole('radio', { name: 'First difference' }).click()
  await expect(page.getByTestId('changes')).toHaveText('2')
})

test('interrupted drag leaves the committed setting intact', async ({ page }) => {
  const group = page.getByRole('radiogroup', { name: 'Transform', exact: true })
  const b = (await group.getByRole('radio', { name: 'Linear detrend' }).boundingBox())!
  await page.mouse.move(b.x + b.width / 2, b.y + b.height / 2)
  await page.mouse.down()
  await page.evaluate(() => window.dispatchEvent(new Event('blur')))
  await page.mouse.up()
  await expect(page.getByTestId('changes')).toHaveText('0')
  await expect(group.getByRole('radio', { name: 'Keep levels' })).toBeChecked()
})

test('keyboard skips disabled choices and long labels stay readable', async ({ page }, info) => {
  await page.getByRole('button', { name: 'Toggle unavailable option' }).click()
  const group = page.getByRole('radiogroup', { name: 'Transform', exact: true })
  await group.getByRole('radio', { name: 'Keep levels' }).focus()
  await page.keyboard.press('ArrowRight')
  await expect(group.getByRole('radio', { name: 'Linear detrend' })).toBeChecked()
  await page.keyboard.press('Home')
  await expect(group.getByRole('radio', { name: 'Keep levels' })).toBeChecked()
  await page.keyboard.press('End')
  await expect(group.getByRole('radio', { name: 'Linear detrend' })).toBeChecked()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('segmented-controls.png') })
})
