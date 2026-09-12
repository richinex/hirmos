import { expect, test } from '@playwright/test'

test('coefficient grid handles many terms without creating every chart', async ({ page }) => {
  await page.goto('/')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ default: { createElement } }, { default: { createRoot } }, { AalenCoefficients }] = await Promise.all([
      load('/node_modules/.vite/deps/react.js'), load('/node_modules/.vite/deps/react-dom_client.js'), load('/src/components/survival/AalenCoefficients.tsx'),
    ])
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;overflow:auto;background:white;padding:16px'
    document.body.append(host)
    const names = Array.from({ length: 37 }, (_, i) => `Term ${i}`)
    const curves = names.map(() => [[0, 0, 0, 0], [1, -1, -2, 0.5], [2, 1, -0.5, 2]])
    createRoot(host).render(createElement(AalenCoefficients, { names, curves }))
  })
  await expect(page.getByTestId('aalen-coefficient-card')).toHaveCount(12)
  await expect(page.getByText('1–12 of 37', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Next coefficients' }).click()
  await expect(page.getByText('13–24 of 37', { exact: true })).toBeVisible()
  await page.getByRole('button', { name: 'Next coefficients' }).click()
  await page.getByRole('button', { name: 'Next coefficients' }).click()
  await expect(page.getByTestId('aalen-coefficient-card')).toHaveCount(1)
  await expect(page.getByRole('heading', { name: 'Term 36', exact: true })).toBeVisible()
  await expect(page.getByRole('button', { name: 'Next coefficients' })).toBeDisabled()
  await page.getByRole('searchbox', { name: 'Filter coefficients' }).fill('Term 2')
  await expect(page.getByTestId('aalen-coefficient-card')).toHaveCount(11)
  await page.getByRole('searchbox', { name: 'Filter coefficients' }).fill('absent')
  await expect(page.getByText('No coefficients match this filter.')).toBeVisible()
  await page.getByRole('searchbox', { name: 'Filter coefficients' }).fill('Term 36')
  await expect(page.getByTestId('aalen-coefficient-card')).toHaveCount(1)
  for (const theme of ['light', 'dark']) {
    await page.evaluate((name) => { document.documentElement.dataset.theme = name }, theme)
    const colour = theme === 'light' ? '#3D9DD1' : '#56B4E9'
    await expect(page.getByTestId('aalen-coefficients').locator(`path[stroke="${colour}"]`).first()).toBeVisible()
  }
  expect(await page.getByTestId('aalen-coefficient-grid').evaluate((el) => el.getBoundingClientRect().right <= window.innerWidth)).toBe(true)
})
