import { expect, test } from '@playwright/test'

test('preparation keeps its horizontal anchor when cancellation appears', async ({ page }, info) => {
  await page.addInitScript(() => {
    const send = Worker.prototype.postMessage
    Worker.prototype.postMessage = new Proxy(send, {
      apply(target, receiver, args) {
        if (args[0]?.kind === 'materialize-numeric') {
          Object.assign(window, { releasePreparation: () => Reflect.apply(target, receiver, args) })
          return
        }
        return Reflect.apply(target, receiver, args)
      },
    })
  })
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Preparation position')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'position.csv', mimeType: 'text/csv', buffer: Buffer.from('value\n1\n2\n3\n') })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Independent observations/ }).check()
  await page.getByRole('checkbox', { name: 'value', exact: true }).check()
  const prepare = page.getByRole('button', { name: 'Create prepared dataset version', exact: true })
  await prepare.scrollIntoViewIfNeeded()
  const before = await prepare.boundingBox()
  expect(before).not.toBeNull()
  await prepare.click()
  await expect(page.getByRole('button', { name: 'Cancel preparation', exact: true })).toBeVisible()
  await expect(prepare).toHaveAttribute('aria-busy', 'true')
  const surface = await prepare.evaluate((element) => {
    const style = getComputedStyle(element)
    return { background: style.backgroundColor, opacity: style.opacity }
  })
  expect(surface.background).not.toBe('rgba(0, 0, 0, 0)')
  expect(surface.background).not.toBe('transparent')
  expect(surface.opacity).toBe('1')
  const originalTheme = await page.evaluate(() => document.documentElement.getAttribute('data-theme'))
  for (const theme of ['dark', 'light']) {
    await page.evaluate((name) => document.documentElement.setAttribute('data-theme', name), theme)
    await expect.poll(() => prepare.evaluate((element) => getComputedStyle(element).boxShadow)).not.toBe('none')
    // The busy button keeps an opaque surface on both faces; the signal fill, not the well.
    await expect.poll(() => prepare.evaluate((element) => {
      const colour = getComputedStyle(element).backgroundColor
      const alpha = colour.startsWith('rgba') ? Number(colour.slice(colour.lastIndexOf(',') + 1, -1)) : 1
      return alpha === 1 && colour !== 'transparent'
    })).toBe(true)
    await page.screenshot({ path: info.outputPath(`prepared-button-${theme}.png`) })
  }
  await page.evaluate((theme) => {
    if (theme === null) document.documentElement.removeAttribute('data-theme')
    else document.documentElement.setAttribute('data-theme', theme)
  }, originalTheme)
  await expect.poll(async () => Math.abs((await prepare.boundingBox())!.x - before!.x)).toBeLessThan(1)
  const running = await prepare.boundingBox()
  expect(running).not.toBeNull()
  expect(Math.abs(running!.x - before!.x)).toBeLessThan(1)
  expect(Math.abs(running!.width - before!.width)).toBeLessThan(1)
  await expect.poll(() => page.evaluate(() => typeof Reflect.get(window, 'releasePreparation'))).toBe('function')
  await page.getByRole('button', { name: 'Cancel preparation', exact: true }).click()
  await expect(prepare).toBeEnabled()
  const cancelled = await prepare.boundingBox()
  expect(Math.abs(cancelled!.x - before!.x)).toBeLessThan(1)
  await page.evaluate(() => Reflect.get(window, 'releasePreparation')())
})
