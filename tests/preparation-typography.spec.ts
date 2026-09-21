import { expect, test } from '@playwright/test'

test('preparation typography and missing-value controls remain aligned', async ({ page }, info) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Preparation typography')
  await page.getByRole('button', { name: 'Create project', exact: true }).click()
  await page.locator('input[type=file]').setInputFiles({ name: 'missing.csv', mimeType: 'text/csv', buffer: Buffer.from('date,amount\n2026-01-01,1\n2026-01-02,\n2026-01-03,3\n') })
  await page.getByRole('button', { name: 'Inspect data', exact: true }).click()
  await page.getByRole('radio', { name: /^Regular time series/ }).check()
  await page.getByRole('checkbox', { name: 'amount', exact: true }).check()
  await expect(page.getByRole('heading', { name: 'Choose the observation structure' })).toHaveCSS('font-size', '16px')
  await expect(page.getByText('Time column', { exact: true })).toHaveCSS('font-size', /^11\.11/)
  await expect(page.getByText('Source frequency', { exact: true })).toHaveCSS('font-size', /^11\.11/)
  await expect(page.getByRole('combobox', { name: 'Source frequency' })).toHaveCSS('font-size', /^13\.33/)
  const group = page.getByRole('radiogroup', { name: 'Missing-value policy' })
  await expect(group.getByRole('radio')).toHaveCount(4)
  await group.scrollIntoViewIfNeeded()
  const offsets = await group.locator('label').evaluateAll(labels => labels.map(label => {
    const radio = label.querySelector('input')!.getBoundingClientRect()
    const text = label.querySelector('span > span')!.getBoundingClientRect()
    return Math.abs((radio.y + radio.height / 2) - (text.y + text.height / 2))
  }))
  for (const offset of offsets) expect(offset).toBeLessThanOrEqual(info.project.name === 'mobile-chromium' ? 4 : 1)
  await group.getByRole('radio', { name: 'Explicit imputation', exact: true }).check()
  await expect(page.getByRole('radiogroup', { name: 'Imputation method' })).toBeVisible()
  const methods = page.getByRole('radiogroup', { name: 'Imputation method' })
  for (const [name, text] of [
    ['Carry forward', 'Repeat the preceding observed value through a gap; leading gaps and gaps exceeding the limit remain missing.'],
    ['Linear inside the series', 'Interpolate between the observed values on both sides of a gap; edge gaps and gaps exceeding the limit remain missing.'],
    ['Structural zero', 'Replace every missing cell with zero only when you confirm it represents a true zero, not an unknown value.'],
    ['Longest gap to fill', 'Maximum consecutive missing cells per column; a longer gap is left entirely unfilled, not partially filled.'],
  ]) {
    const help = page.getByRole('button', { name: `About ${name}`, exact: true })
    if (info.project.name === 'mobile-chromium') await help.tap()
    else await help.focus()
    const note = page.getByRole(info.project.name === 'mobile-chromium' ? 'dialog' : 'tooltip').filter({ hasText: text })
    await expect(note).toBeVisible()
    await expect(methods.getByRole('radio', { name: 'Linear inside the series', exact: true })).toBeChecked()
    await page.keyboard.press('Escape')
    await expect(note).toBeHidden()
  }
  if (info.project.name === 'chromium') {
    const method = await page.getByRole('radio', { name: /^Linear inside the series/ }).locator('..').locator('span > span').first().boundingBox()
    const input = await page.getByRole('spinbutton', { name: 'Longest gap to fill' }).boundingBox()
    expect(Math.abs(method!.y + method!.height / 2 - input!.y - input!.height / 2)).toBeLessThanOrEqual(2)
  }
  const columns = page.getByTestId('analysis-columns-list')
  await expect(columns).toHaveCSS('overflow-y', 'auto')
  expect(await columns.evaluate(element => element.scrollHeight <= element.clientHeight)).toBe(true)
  await page.locator('section[aria-labelledby="missingness-title"]').screenshot({ path: info.outputPath('imputation.png') })
  await page.locator('section[aria-labelledby="sampling-title"]').screenshot({ path: info.outputPath('observation-typography.png') })
  if (info.project.name === 'mobile-chromium') await page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: 'Column profile' }).click()
  const inspector = page.locator('.inspector-content').filter({ has: page.getByRole('heading', { name: 'date', exact: true }) })
  await expect(inspector.getByRole('heading', { name: 'date', exact: true })).toHaveCSS('font-size', '16px')
  await expect(inspector.getByRole('heading', { name: 'date', exact: true })).toHaveCSS('font-weight', '600')
  await expect(inspector.locator('dd').first()).toHaveCSS('font-size', /^13\.33/)
  await expect(inspector.locator('dt').first()).toHaveCSS('font-size', /^11\.11/)
  await expect(inspector.getByRole('heading', { name: 'Most frequent values' })).toHaveCSS('font-size', '16px')
  await inspector.screenshot({ path: info.outputPath('right-panel-typography.png') })
})
