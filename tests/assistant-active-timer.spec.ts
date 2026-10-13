import { expect, test } from '@playwright/test'

test('approval waits preserve the remaining execution budget', async ({ page }) => {
  await page.goto('/app')
  await page.clock.install()
  await page.clock.pauseAt(new Date())
  await page.evaluate(async () => {
    const { activeTimer } = await import(new URL('/src/assistant/activeTimer.ts', location.href).href)
    const state = { expired: false, timer: activeTimer(1000, () => { state.expired = true }) }
    Reflect.set(window, 'timerCheck', state)
  })
  await page.clock.runFor(400)
  await page.evaluate(() => Reflect.get(window, 'timerCheck').timer.pause())
  await page.clock.runFor(300000)
  expect(await page.evaluate(() => Reflect.get(window, 'timerCheck').expired)).toBe(false)
  await page.evaluate(() => Reflect.get(window, 'timerCheck').timer.resume())
  await page.clock.runFor(599)
  expect(await page.evaluate(() => Reflect.get(window, 'timerCheck').expired)).toBe(false)
  await page.clock.runFor(1)
  expect(await page.evaluate(() => Reflect.get(window, 'timerCheck').expired)).toBe(true)
})

test('stopping during approval cannot restart the deadline', async ({ page }) => {
  await page.goto('/app')
  await page.clock.install()
  await page.clock.pauseAt(new Date())
  await page.evaluate(async () => {
    const { activeTimer } = await import(new URL('/src/assistant/activeTimer.ts', location.href).href)
    Reflect.set(window, 'expired', false)
    const timer = activeTimer(1000, () => Reflect.set(window, 'expired', true))
    timer.pause()
    timer.stop()
    timer.resume()
  })
  await page.clock.runFor(300000)
  expect(await page.evaluate(() => Reflect.get(window, 'expired'))).toBe(false)
})
