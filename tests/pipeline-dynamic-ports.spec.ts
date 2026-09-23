import { expect, test } from '@playwright/test'

test('new Script input handles accept connections after the port count changes', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Desktop pointer connection regression')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Dynamic Script inputs')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.getByRole('radio', { name: 'Build a pipeline', exact: true }).click()
  await page.getByRole('button', { name: 'Open the editor', exact: true }).click()
  const toolbar = page.getByRole('toolbar', { name: 'Add a block' })
  const inputs = ['input-1']
  for (let i = 0; i < 2; i++) {
    await toolbar.getByRole('button', { name: 'Input file', exact: true }).click()
    const id = await page.locator('[data-testid^="block-input-"]').last().getAttribute('data-testid')
    expect(id).not.toBeNull()
    inputs.push(id!.slice(6))
  }
  await toolbar.getByRole('button', { name: 'Script', exact: true }).click()
  const script = page.locator('[data-testid^="block-script-"]').last()
  const id = (await script.getAttribute('data-testid'))!.slice(6)
  for (const [port, input] of inputs.entries()) {
    await page.getByRole('button', { name: 'Fit the pipeline', exact: true }).click()
    await page.waitForTimeout(400)
    const source = page.getByTestId(`block-${input}`).locator('[data-handleid="out"]')
    const target = script.locator(`[data-handleid="in-${port}"]`)
    const from = (await source.boundingBox())!
    const to = (await target.boundingBox())!
    await page.mouse.move(from.x + from.width / 2, from.y + from.height / 2)
    await page.mouse.down()
    await page.mouse.move(to.x + to.width / 2, to.y + to.height / 2, { steps: 20 })
    await page.mouse.up()
    await expect(page.locator(`.react-flow__edge[data-id="${input}->${id}:${port}"]`)).toHaveCount(1)
    await expect(script.locator('[data-handleid^="in-"]')).toHaveCount(port + 2)
  }
})
