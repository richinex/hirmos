import { expect, test } from '@playwright/test'

test('DAG actions share dimensions on desktop and in the mobile inspector', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Severity, dose and recovery', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  const mobile = info.project.name === 'mobile-chromium'
  if (mobile) await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /DAG workspace/ }).click()
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
  const create = await toolbar.getByRole('button', { name: 'Create a DAG' }).boundingBox()
  expect(add!.height).toBe(create!.height)
  if (mobile) await page.getByRole('button', { name: 'Inspector', exact: true }).click()
  const selector = page.getByRole('radiogroup', { name: 'DAG inspector' })
  const choices = await selector.getByRole('radio').evaluateAll(elements => elements.map(element => element.getBoundingClientRect().top))
  expect(choices).toHaveLength(3)
  expect(Math.max(...choices) - Math.min(...choices)).toBeLessThan(1)
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
