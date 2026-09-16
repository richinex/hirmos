import { expect, test } from '@playwright/test'

test('mobile sheet openers have depth without changing size or dialog behaviour', async ({ page }, info) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string) => import(/* @vite-ignore */ path)
    const { default: React } = await load('/node_modules/.vite/deps/react.js')
    const { default: { createRoot } } = await load('/node_modules/.vite/deps/react-dom_client.js')
    const { WorkbenchLayout } = await load('/src/components/shell/WorkbenchLayout.tsx')
    const host = document.createElement('main')
    host.id = 'sheet-test'
    host.style.cssText = 'position:fixed;inset:0;display:flex;background:var(--color-stage);z-index:20'
    document.body.append(host)
    const h = React.createElement
    createRoot(host).render(h(WorkbenchLayout, {
      id: 'sheet-test', stage: h('h1', null, 'Stationarity tests'),
      inspector: { title: 'Prepared dataset and method requirements', trigger: { label: 'Requirements', icon: 'fact_check' }, body: h('p', null, 'Method requirements') },
      bottom: { title: 'Saved runs', trigger: { label: 'History', icon: 'history' }, body: h('p', null, 'Saved analyses') },
    }))
  })
  const actions = page.getByRole('group', { name: 'Panes' })
  await expect(actions.getByRole('button')).toHaveCount(2)
  for (const theme of ['dark', 'light', 'original-light', 'soft-dark']) {
    await page.evaluate(theme => document.documentElement.dataset.theme = theme, theme)
    for (const width of [320, 390]) {
      await page.setViewportSize({ width, height: 844 })
      const dimensions = await actions.getByRole('button').evaluateAll(buttons => buttons.map(button => {
        const rect = button.getBoundingClientRect()
        return { width: rect.width, height: rect.height, shadow: getComputedStyle(button).boxShadow }
      }))
      expect(dimensions[0].width).toBe(dimensions[1].width)
      expect(dimensions[0].height).toBe(dimensions[1].height)
      expect(dimensions[0].shadow).not.toBe('none')
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
      await actions.locator('..').screenshot({ path: info.outputPath(`${theme}-${width}.png`) })
    }
  }
  const opener = actions.getByRole('button', { name: 'Prepared dataset and method requirements' })
  await opener.click()
  const dialog = page.getByRole('dialog', { name: 'Prepared dataset and method requirements' })
  await expect(dialog).toBeVisible()
  await page.keyboard.press('Escape')
  await expect(dialog).toBeHidden()
  await expect(opener).toBeFocused()
  await page.emulateMedia({ reducedMotion: 'reduce' })
  await expect.poll(() => opener.evaluate(el => getComputedStyle(el).transitionProperty)).not.toContain('transform')
})
