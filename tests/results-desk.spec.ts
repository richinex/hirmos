import { expect, test, type Page } from '@playwright/test'
import { chapter } from './examples/support'

interface Box {
  readonly key: string
  readonly left: number
  readonly top: number
  readonly right: number
  readonly bottom: number
}

/** Windows as drawn once they have stopped moving: their keys and screen rectangles, front ones only. */
const drawn = (page: Page) =>
  page.evaluate(() =>
    [...document.querySelectorAll<HTMLElement>('.desk-window')]
      .filter((window) => getComputedStyle(window).visibility === 'visible')
      .map((window) => {
        const box = window.getBoundingClientRect()
        return { key: window.dataset.key!, left: box.left, top: box.top, right: box.right, bottom: box.bottom }
      }),
  )

const overlapping = (boxes: readonly Box[]): string[] => {
  const pairs: string[] = []
  boxes.forEach((a, i) =>
    boxes.slice(i + 1).forEach((b) => {
      if (a.left < b.right - 1 && b.left < a.right - 1 && a.top < b.bottom - 1 && b.top < a.bottom - 1)
        pairs.push(`${a.key} × ${b.key}`)
    }),
  )
  return pairs
}

/** Whether every window sits inside the desk and above its dock. */
const outside = async (page: Page, boxes: readonly Box[]): Promise<string[]> => {
  const desk = await page.locator('.results-desk').boundingBox()
  if (desk === null) return ['no desk']
  return boxes
    .filter(
      (box) =>
        box.left < desk.x - 1 ||
        box.top < desk.y - 1 ||
        box.right > desk.x + desk.width + 1 ||
        box.bottom > desk.y + desk.height - 76 + 1,
    )
    .map((box) => box.key)
}

const settled = async (page: Page) => {
  let previous = ''
  await expect
    .poll(async () => {
      const now = JSON.stringify(await drawn(page))
      const still = now === previous
      previous = now
      return still
    }, { intervals: [250] })
    .toBe(true)
  return drawn(page)
}

const openResults = async (page: Page, example: string) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: `Open ${example}`, exact: true }).click()
  await chapter(page, /^Results/)
  await expect(page.getByRole('region', { name: 'Saved results' })).toBeAttached()
}

const view = (page: Page, name: string) => page.getByRole('radio', { name }).click({ force: true })
const dock = (page: Page) => page.getByRole('navigation', { name: 'Windows on the desk' })

/** Saved results in front, as a person brings it back: from its dock tile when it is not the front window. */
const raiseSavedResults = async (page: Page) => {
  const front = await page.evaluate(() => {
    const window = document.querySelector<HTMLElement>('.desk-window[data-key="runs"]')!
    return getComputedStyle(window).visibility === 'visible' && window.style.getPropertyValue('--focused') === '1'
  })
  if (!front) await dock(page).getByRole('button', { name: 'Saved results' }).click()
  await expect(page.locator('.desk-window[data-key="runs"]')).toHaveCSS('visibility', 'visible')
}

/** Two more estimates on the seat-belt study, so opening every result needs more than ten windows. */
const fillDesk = async (page: Page) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true }).click()
  await chapter(page, /^Estimation/)
  for (const count of [3, 4]) {
    await page.getByRole('button', { name: 'Run adjusted linear regression' }).click()
    await expect(page.getByText(`Runs (${count})`)).toBeVisible({ timeout: 60_000 })
  }
  await chapter(page, /^Results/)
  await expect(page.getByRole('region', { name: 'Saved results' })).toBeAttached()
  await view(page, 'Desk')
  return openEverySavedResult(page)
}

const openEverySavedResult = async (page: Page) => {
  const rows = page.getByRole('region', { name: 'Saved results' }).getByRole('listitem')
  const count = await rows.count()
  for (let i = 0; i < count; i++) {
    await raiseSavedResults(page)
    await rows.nth(i).getByRole('button', { name: 'Open' }).click()
  }
  return count
}

/** Carries a window by its handle until its centre lies on `to`, then releases unless told to hold. */
const carry = async (page: Page, key: string, to: Box, hold = false) => {
  const window = page.locator(`.desk-window[data-key="${key}"]`)
  const handle = (await window.locator('.desk-handle').boundingBox())!
  const box = (await window.boundingBox())!
  const dx = (to.left + to.right) / 2 - (box.x + box.width / 2)
  const dy = (to.top + to.bottom) / 2 - (box.y + box.height / 2)
  await page.mouse.move(handle.x + handle.width / 2, handle.y + handle.height / 2)
  await page.mouse.down()
  await page.mouse.move(handle.x + handle.width / 2 + dx, handle.y + handle.height / 2 + dy, { steps: 20 })
  if (!hold) await page.mouse.up()
}

/** Keys in reading order: top to bottom, then left to right. */
const reading = (boxes: readonly Box[]) =>
  [...boxes]
    .sort((a, b) => (Math.abs(a.top - b.top) > 2 ? a.top - b.top : a.left - b.left))
    .map((box) => box.key)

test.beforeEach(({}, info) => {
  test.skip(info.project.name !== 'chromium', 'Phones keep the single-column Results page.')
})

for (const size of [
  { width: 1440, height: 900 },
  { width: 1024, height: 700 },
]) {
  test(`Arrange places ten windows side by side inside a ${size.width}×${size.height} desk`, async ({ page }) => {
    await page.setViewportSize(size)
    expect(await fillDesk(page)).toBe(6)
    await expect(page.getByRole('status').filter({ hasText: 'which holds ten windows at a time' })).toBeVisible()
    await expect(page.locator('.desk-window')).toHaveCount(10)

    await page.getByRole('button', { name: 'Arrange' }).click()
    const boxes = await settled(page)
    expect(boxes).toHaveLength(10)
    expect(overlapping(boxes)).toEqual([])
    expect(await outside(page, boxes)).toEqual([])
  })
}

test('a minimised window goes to the dock and comes back where it was', async ({ page }) => {
  await openResults(page, 'Seat-belt law and road deaths')
  await view(page, 'Desk')
  await raiseSavedResults(page)
  const before = (await settled(page)).find((box) => box.key === 'runs')!

  await page.getByRole('button', { name: 'Minimize Saved results' }).click()
  await expect(page.locator('.desk-window[data-key="runs"]')).toHaveCSS('visibility', 'hidden')

  await dock(page).getByRole('button', { name: 'Saved results' }).click()
  const after = (await settled(page)).find((box) => box.key === 'runs')!
  expect(after).toEqual(before)
})

test('Evidence and Focus tile without overlap, and a dragged title bar returns to the free desk', async ({ page }) => {
  await openResults(page, 'A simulated process with a collider')
  const estimates = page.getByRole('list', { name: 'Estimates' }).getByRole('listitem')
  await view(page, 'Desk')
  await raiseSavedResults(page)
  const count = await estimates.count()
  expect(count).toBe(2)
  let tiled: Box[] = []
  for (let i = 0; i < count && tiled.length < 3; i++) {
    await view(page, 'Desk')
    await raiseSavedResults(page)
    await estimates.nth(i).getByRole('button', { name: 'Open' }).click()
    await view(page, 'Evidence')
    tiled = await settled(page)
  }
  expect(tiled.map((box) => box.key.split(':')[0]).sort()).toEqual(['estimate', 'record', 'sensitivity'])
  expect(overlapping(tiled)).toEqual([])
  expect(await outside(page, tiled)).toEqual([])

  const recordKey = tiled.find((box) => box.key.startsWith('record'))!.key
  const order = reading(tiled)
  const last = tiled.find((box) => box.key === order.at(-1))!
  await carry(page, recordKey, last)
  await expect(page.getByRole('radio', { name: 'Evidence' })).toBeChecked()
  const reordered = await settled(page)
  expect(reading(reordered)).toEqual([...order.filter((key) => key !== recordKey), recordKey])
  expect(overlapping(reordered)).toEqual([])
  expect(await outside(page, reordered)).toEqual([])

  await page.locator(`.desk-window[data-key="${recordKey}"] .desk-name`).click()
  await page.keyboard.press('4')
  const focused = await settled(page)
  expect(focused).toHaveLength(1)
  expect(focused[0]!.key).toBe(recordKey)
  expect(await outside(page, focused)).toEqual([])
})

test('a carried window is inserted where it lands, Escape cancels, and its handle moves it with the arrow keys', async ({ page }) => {
  await openResults(page, 'Seat-belt law and road deaths')
  await view(page, 'Desk')
  await openEverySavedResult(page)
  await page.getByRole('button', { name: 'Arrange' }).click()
  const arranged = await settled(page)
  const order = reading(arranged)
  const [first, , third] = order.map((key) => arranged.find((box) => box.key === key)!)

  await carry(page, first!.key, third!, true)
  const during = await settled(page)
  expect(reading(during)).toEqual([order[1], order[2], order[0], ...order.slice(3)])
  await page.keyboard.press('Escape')
  await page.mouse.up()
  expect(reading(await settled(page))).toEqual(order)

  await carry(page, first!.key, third!)
  const after = await settled(page)
  expect(reading(after)).toEqual([order[1], order[2], order[0], ...order.slice(3)])
  expect(overlapping(after)).toEqual([])

  const handle = page.locator(`.desk-window[data-key="${first!.key}"] .desk-handle`)
  await expect(handle).toHaveAccessibleName(/^Move /)
  await handle.focus()
  await page.keyboard.press('ArrowLeft')
  expect(reading(await settled(page))).toEqual([order[1], order[0], order[2], ...order.slice(3)])
  await page.keyboard.press('ArrowRight')
  expect(reading(await settled(page))).toEqual([order[1], order[2], order[0], ...order.slice(3)])
})

test('Compare on a full desk keeps both estimates and their differences', async ({ page }) => {
  await fillDesk(page)
  const estimates = page.getByRole('region', { name: 'Saved results' }).getByRole('list', { name: 'Estimates' }).getByRole('listitem')

  await raiseSavedResults(page)
  await estimates.nth(1).getByRole('button', { name: 'Open' }).click()
  await raiseSavedResults(page)
  await estimates.nth(3).getByRole('button', { name: 'Compare' }).click()

  await expect(page.getByRole('radio', { name: 'Compare' })).toBeChecked()
  const boxes = await settled(page)
  expect(boxes.map((box) => box.key.split(':')[0]).sort()).toEqual(['differences', 'estimate', 'estimate'])
  expect(overlapping(boxes)).toEqual([])

  const order = reading(boxes)
  const differences = boxes.find((box) => box.key === 'differences')!
  await carry(page, order[0]!, differences)
  const reordered = await settled(page)
  expect(reading(reordered)).toEqual([...order.slice(1), order[0]])
  expect(overlapping(reordered)).toEqual([])
  await page.reload()
  await expect(page.locator('.desk-window[data-key="differences"]')).toHaveCSS('visibility', 'visible')
  expect(reading(await settled(page))).toEqual(reading(reordered))
})

test('a run put on the desk from its chapter opens there, and the desk survives a reload', async ({ page }) => {
  await openResults(page, 'Seat-belt law and road deaths')
  const series = page.locator('.desk-window[data-key^="time-series"]')
  if ((await series.count()) > 0) await series.getByRole('button', { name: /^Close / }).click({ force: true })
  await expect(series).toHaveCount(0)

  await chapter(page, /^Time-series analysis/)
  await page.locator('input[type=radio][value="interrupted"]').click()
  const history = page.getByRole('list', { name: 'Time-series runs' })
  await history.getByRole('button', { name: 'Put on the Results desk' }).first().click()
  await expect(history.getByRole('button', { name: 'On the Results desk' })).toBeVisible()

  await chapter(page, /^Results/)
  await expect(series).toHaveCSS('visibility', 'visible')
  const before = await settled(page)

  await page.reload()
  await expect(series).toHaveCSS('visibility', 'visible')
  expect(await settled(page)).toEqual(before)
})
