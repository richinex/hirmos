import { expect, test } from '@playwright/test'

test('table controls align on touch and desktop, including after search and wrapping', async ({ page }, info) => {
  await page.goto('/')
  await page.evaluate(async () => {
    const load = (path: string): Promise<any> => import(/* @vite-ignore */ path)
    const [{ React, createRoot }, ui] = await Promise.all([
      load('/tests/support/reactRuntime.ts'), load('/src/components/table/primitives.tsx'),
    ])
    const host = document.createElement('div')
    host.style.cssText = 'position:fixed;inset:0;z-index:9999;overflow:auto;background:var(--color-panel);padding:16px'
    document.body.append(host)
    function Controls() {
      const [search, setSearch] = React.useState('')
      const [active, setActive] = React.useState(false)
      const [density, setDensity] = React.useState('comfortable')
      return React.createElement(ui.TableShell, {
        title: 'Physical schema', titleId: 'schema', count: '11 columns',
        toolbar: React.createElement(React.Fragment, null,
          React.createElement(ui.FilterField, { value: search, onChange: setSearch, placeholder: 'Find a column', label: 'Search columns', className: 'w-40' }),
          React.createElement(ui.FacetPills, { facets: [{ id: 'numeric', text: 'Numeric', count: 11, active }], label: 'Column types', onToggle: () => setActive(!active) }),
          React.createElement(ui.DensityToggle, { density, onChange: setDensity })),
      })
    }
    createRoot(host).render(React.createElement(Controls))
  })
  const search = page.getByRole('searchbox', { name: 'Search columns' })
  const facet = page.getByRole('button', { name: 'Numeric 11' })
  const density = page.getByRole('radiogroup', { name: 'Row density' })
  await expect(search).toBeVisible()
  const touch = await page.evaluate(() => matchMedia('(pointer: coarse)').matches)
  for (const width of [390, 320, 768, 1440]) {
    await page.setViewportSize({ width, height: 844 })
    for (const value of ['', 'age']) {
      await search.fill(value)
      const heights = await Promise.all([search.locator('..'), facet, density].map(async (element) => (await element.boundingBox())!.height))
      expect(Math.max(...heights) - Math.min(...heights)).toBeLessThanOrEqual(1)
      if (touch) expect(Math.min(...heights)).toBeGreaterThanOrEqual(44)
      expect(await density.evaluate((element) => element.getBoundingClientRect().right <= innerWidth)).toBe(true)
      await page.screenshot({ path: info.outputPath(`controls-${width}-${value || 'empty'}.png`) })
    }
  }
  await page.getByRole('button', { name: 'Clear search columns' }).click()
  await expect(search).toHaveValue('')
  await expect(search).toBeFocused()
  await facet.click()
  await expect(facet).toHaveAttribute('aria-pressed', 'true')
  await page.getByRole('radio', { name: 'Compact rows' }).check()
  await expect(page.getByRole('radio', { name: 'Compact rows' })).toBeChecked()
})
