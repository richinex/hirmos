import { expect, test } from '@playwright/test'

// The cycle from the light default: dark, then system, which resolves to light under a light OS.
const themes = [
  ['dark', 'Dark'], ['light', 'System'],
] as const

test('an unknown or retired preference resolves to light', async ({ page }) => {
  for (const saved of ['soft-dark', 'original-light', 'nonsense']) {
    await page.addInitScript(value => localStorage.setItem('hirmos-theme', value), saved)
    await page.goto('/app/projects')
    await expect(page.locator('.example-card').first()).toBeVisible()
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light')
    await expect.poll(() => page.evaluate(() => localStorage.getItem('hirmos-theme'))).toBe('light')
  }
})

test('themes preserve preference, readable tokens and chart palette mode', async ({ page }, info) => {
  await page.emulateMedia({ colorScheme: 'light' })
  await page.goto('/app')
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light')
  for (const [id, label] of themes) {
    await page.getByRole('button', { name: 'Change theme', exact: true }).click()
    await expect(page.getByRole('dialog', { name: 'Choose theme' })).toHaveCount(0)
    await expect(page.locator('html')).toHaveAttribute('data-theme', id)
    await expect.poll(() => page.evaluate(() => {
      const css = getComputedStyle(document.documentElement)
      const href = document.querySelector<HTMLLinkElement>('link[rel=icon]')!.href
      const svg = new DOMParser().parseFromString(decodeURIComponent(href.split(',')[1] ?? ''), 'image/svg+xml')
      return svg.querySelector('g')?.getAttribute('fill') === css.getPropertyValue('--color-rail-ink').trim()
    })).toBe(true)
    const mark = page.locator('svg g[fill="var(--color-rail-ink)"]').first()
    const fill = await mark.evaluate(el => getComputedStyle(el).fill)
    const expectedFill = await page.evaluate(() => { const probe = document.createElement('span'); probe.style.color='var(--color-rail-ink)'; document.body.append(probe); const value=getComputedStyle(probe).color; probe.remove(); return value })
    expect(fill).toBe(expectedFill)
    await expect(page.getByRole('button', { name: 'Change theme', exact: true })).toHaveAttribute('title', new RegExp(`Theme: ${label}`))
    expect(await page.locator('.chapter-heading').evaluate(el => getComputedStyle(el).fontFamily)).toContain('Roboto Slab')
    const audit = await page.evaluate(async () => {
      const css = getComputedStyle(document.documentElement)
      const colour = (name: string) => css.getPropertyValue(`--color-${name}`).trim()
      const luminance = (hex: string) => {
        const rgb = hex.replace('#', '').match(/../g)!.map(v => parseInt(v,16)/255).map(v => v <= .04045 ? v/12.92 : ((v+.055)/1.055)**2.4)
        return rgb[0]*.2126 + rgb[1]*.7152 + rgb[2]*.0722
      }
      const ratio = (a: string, b: string) => { const x=luminance(colour(a)), y=luminance(colour(b)); return (Math.max(x,y)+.05)/(Math.min(x,y)+.05) }
      const contrasts = ['stage','panel','well','raised'].flatMap(bg => ['ink','bone','muted','faint'].map(fg => ({ pair:`${fg}/${bg}`, ratio:ratio(fg,bg) })))
      contrasts.push({pair:'signal-text/panel',ratio:ratio('signal-text','panel')})
      contrasts.push({pair:'signal-text/stage',ratio:ratio('signal-text','stage')})
      contrasts.push({pair:'signal-ink/signal',ratio:ratio('signal-ink','signal')})
      contrasts.push({pair:'signal-ink/signal-hover',ratio:ratio('signal-ink','signal-hover')})
      contrasts.push({pair:'link/panel',ratio:ratio('link','panel')})
      contrasts.push({pair:'rail-ink/rail',ratio:ratio('rail-ink','rail')})
      contrasts.push({pair:'rail-faint/rail',ratio:ratio('rail-faint','rail')})
      contrasts.push({pair:'rail-ink/rail-active',ratio:ratio('rail-ink','rail-active')})
      const charts = await import(new URL('/src/charts/theme.ts', location.href).href)
      const chartTheme = charts.readChartTheme()
      return { contrasts, mode: css.colorScheme, palette: chartTheme.categorical, font: chartTheme.font, labelSize: chartTheme.labelSize, light: charts.LIGHT_CATEGORICAL_RAMP, dark: charts.CATEGORICAL_RAMP }
    })
    for (const item of audit.contrasts) expect(item.ratio, `${id} ${item.pair}`).toBeGreaterThanOrEqual(4.5)
    expect(audit.palette).toEqual(audit.mode === 'light' ? audit.light : audit.dark)
    expect(audit.font).toContain('Lato')
    expect(audit.labelSize).toBeCloseTo(11.11, 1)
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
    await page.screenshot({ path: info.outputPath(`${id}.png`) })
    await page.reload()
    await expect(page.locator('html')).toHaveAttribute('data-theme', id)
  }
  await expect(page.getByRole('button', { name: 'Change theme', exact: true })).toHaveAttribute('title', /Theme: System/)
  await page.emulateMedia({ colorScheme: 'dark' })
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark')
  await page.emulateMedia({ colorScheme: 'light' })
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light')
})

test('saved palettes apply before the React entry point loads', async ({ page }) => {
  await page.route('**/src/main.tsx', route => route.abort())
  for (const [id] of themes) {
    await page.addInitScript(value => localStorage.setItem('hirmos-theme', value), id)
    await page.goto('/app')
    await expect(page.locator('html')).toHaveAttribute('data-theme', id)
  }
})
