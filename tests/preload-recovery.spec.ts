import { expect, test } from '@playwright/test'
import { decidePreloadRecovery } from '../src/lib/preloadRecovery'

/**
 * A tab that loaded before a deploy asks for chunks the host no longer has. The first failure reloads
 * the page once; a failure straight after that reload is surfaced, so a broken deploy cannot loop.
 */

test('the recovery decision reloads once and then waits a minute', () => {
  const now = 1_000_000
  expect(decidePreloadRecovery(null, now)).toEqual({ kind: 'reload' })
  expect(decidePreloadRecovery(now - 10_000, now)).toEqual({ kind: 'surface', reloadedAgoMs: 10_000 })
  expect(decidePreloadRecovery(now - 60_000, now)).toEqual({ kind: 'surface', reloadedAgoMs: 60_000 })
  expect(decidePreloadRecovery(now - 60_001, now)).toEqual({ kind: 'reload' })
})

test('a chunk that fails to load reloads the page once, not twice', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'One layout is enough for a window listener.')
  await page.goto('/app')
  await page.evaluate(() => { sessionStorage.removeItem('hirmos:preload-reloaded-at') })

  const reloaded = page.waitForEvent('load')
  const first = await page.evaluate(() => {
    const event = new Event('vite:preloadError', { cancelable: true })
    window.dispatchEvent(event)
    return event.defaultPrevented
  })
  expect(first).toBe(true)
  await reloaded
  await expect(page.getByRole('textbox', { name: 'Project name' })).toBeVisible()
  const stamp = await page.evaluate(() => sessionStorage.getItem('hirmos:preload-reloaded-at'))
  expect(Number(stamp)).toBeGreaterThan(0)

  const second = await page.evaluate(() => {
    const event = new Event('vite:preloadError', { cancelable: true })
    window.dispatchEvent(event)
    return event.defaultPrevented
  })
  expect(second).toBe(false)
  await page.waitForTimeout(500)
  expect(await page.evaluate(() => sessionStorage.getItem('hirmos:preload-reloaded-at'))).toBe(stamp)
})
