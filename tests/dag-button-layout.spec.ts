import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('narrow DAG controls fill the pane without orphaned actions or overflow', async ({ page }, info) => {
  test.skip(info.project.name !== 'mobile-chromium', 'mobile layout sweep')
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Proposition 99 and cigarette sales', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await chapter(page, /^DAG workspace/)
  await page.keyboard.press('Escape')
  const controls = page.getByRole('group', { name: 'DAG editing controls', exact: true })
  for (const width of [320, 393, 600]) {
    await page.setViewportSize({ width, height: 851 })
    await controls.scrollIntoViewIfNeeded()
    const treatment = await page.getByRole('combobox', { name: 'Treatment', exact: true }).boundingBox()
    const outcome = await page.getByRole('combobox', { name: 'Outcome', exact: true }).boundingBox()
    expect(treatment!.y).toBeCloseTo(outcome!.y, 1)
    expect(treatment!.width).toBeCloseTo(outcome!.width, 1)
    const undo = await page.getByRole('button', { name: 'Undo DAG revision', exact: true }).boundingBox()
    expect(undo!.x).toBeCloseTo(treatment!.x, 1)
    const add = await page.getByRole('button', { name: 'Unmeasured variable', exact: true }).boundingBox()
    const paste = await page.getByRole('button', { name: 'From text', exact: true }).boundingBox()
    expect(add!.y).toBeCloseTo(paste!.y, 1)
    expect(add!.width).toBeCloseTo(paste!.width, 1)
    expect(add!.x).toBeCloseTo(treatment!.x, 1)
    expect(paste!.x).toBeCloseTo(outcome!.x, 1)
    expect(paste!.width).toBeCloseTo(outcome!.width, 1)
    for (const box of [treatment, outcome, undo, add, paste]) {
      expect(box!.x).toBeGreaterThanOrEqual(0)
      expect(box!.x + box!.width).toBeLessThanOrEqual(width)
      expect(box!.height).toBeGreaterThanOrEqual(44)
    }
    for (const theme of ['light', 'dark']) {
      await page.evaluate(t => document.documentElement.setAttribute('data-theme', t), theme)
      await controls.screenshot({ path: info.outputPath(`dag-controls-${width}-${theme}.png`) })
    }
  }
})

test('DAG actions share dimensions on desktop and in the mobile inspector', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  const mobile = info.project.name === 'mobile-chromium'
  if (mobile) await page.getByRole('button', { name: 'Expand section list' }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /DAG workspace/ }).click()
  const toolbar = page.getByRole('toolbar', { name: 'DAG actions' })
  const trigger = await page.getByRole('combobox', { name: 'Outcome', exact: true }).elementHandle()
  const before = await trigger!.boundingBox()
  await page.mouse.move(before!.x + 20, before!.y + 15)
  await page.mouse.down()
  const pressed = await trigger!.boundingBox()
  expect(pressed!.y).toBeCloseTo(before!.y, 2)
  await page.mouse.up()
  await page.keyboard.press('Escape')
  const closed = await trigger!.boundingBox()
  expect(closed!.y).toBeCloseTo(before!.y, 2)
  const add = await toolbar.getByRole('button', { name: 'Unmeasured variable' }).boundingBox()
  const paste = await toolbar.getByRole('button', { name: 'From text' }).boundingBox()
  expect(add!.height).toBe(paste!.height)
  if (mobile) await page.getByRole('button', { name: 'Inspector', exact: true }).click()
  const selector = page.getByRole('radiogroup', { name: 'DAG inspector' })
  // Equal rows (four in one, or two by two) and no tab drawn over another.
  const choices = await selector.getByRole('radio').evaluateAll(elements => elements.map(element => {
    const box = element.getBoundingClientRect()
    return { top: Math.round(box.top), left: box.left, right: box.right }
  }))
  expect(choices).toHaveLength(4)
  const rows = [...new Set(choices.map(choice => choice.top))].map(top => choices.filter(choice => choice.top === top).sort((a, b) => a.left - b.left))
  expect(new Set(rows.map(row => row.length)).size).toBe(1)
  for (const row of rows) for (let i = 1; i < row.length; i += 1) expect(row[i].left).toBeGreaterThanOrEqual(row[i - 1].right - 0.5)
  await selector.getByRole('radio', { name: 'Intervene', exact: true }).check()
  await expect(selector.getByRole('radio', { name: 'Intervene', exact: true })).toBeChecked()
  await selector.getByRole('radio', { name: 'Selection', exact: true }).check()
  const group = page.getByRole('group', { name: 'Use this graph' })
  await expect(group).toBeVisible()
  const sizes = await group.evaluate(el => ({
    width: el.getBoundingClientRect().width,
    buttons: [...el.querySelectorAll('button')].map(button => {
      const box = button.getBoundingClientRect()
      return { width: box.width, height: box.height }
    }),
  }))
  expect(sizes.buttons).toHaveLength(2)
  expect(sizes.buttons[0]!.width).toBeCloseTo(sizes.buttons[1]!.width, 2)
  expect(sizes.buttons[0]!.height).toBeCloseTo(sizes.buttons[1]!.height, 2)
  expect(sizes.buttons[0]!.width).toBeCloseTo(sizes.width, 2)
  expect(sizes.buttons[0]!.height).toBeCloseTo(mobile ? 44 : 36, 2)
  await page.screenshot({ path: info.outputPath('dag-buttons.png') })
  await group.getByRole('button', { name: 'Use for study', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Study design', exact: true })).toBeVisible()
})
