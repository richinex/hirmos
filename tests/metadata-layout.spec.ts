import { expect, test } from '@playwright/test'

test('column identity and requirement tallies fit without repeated labels', async ({ page }, info) => {
  await page.goto('/app')
  await page.evaluate(async () => {
    const load = (path: string) => import(/* @vite-ignore */ path)
    const { default: React } = await load('/node_modules/.vite/deps/react.js')
    const { default: { createRoot } } = await load('/node_modules/.vite/deps/react-dom_client.js')
    const { ColumnProfilePane } = await load('/src/components/data/ColumnProfilePane.tsx')
    const { MethodCaveats } = await load('/src/components/MethodCaveats.tsx')
    const h = React.createElement
    const host = document.createElement('main')
    host.id = 'layout-test'
    host.style.cssText = 'position:fixed;inset:0;overflow:auto;padding:20px;background:var(--color-stage);z-index:9999'
    document.body.append(host)
    const caveats = Array.from({ length: 6 }, (_, i) => ({ id: `c${i}`, category: 'data', requirement: `Requirement ${i + 1}`, consequenceIfUnmet: 'Review the study.', sources: [] }))
    createRoot(host).render(h('div', { style: { maxWidth: 460 } },
      h(ColumnProfilePane, { column: { id: 'month', name: 'month', duckdbType: 'BIGINT', nullable: false }, profile: { rowCount: 10 }, description: { kind: 'idle' } }),
      h(MethodCaveats, { methods: [{ id: 'example', name: 'DirectLiNGAM', summary: '', caveats }], eligibility: { kind: 'caution', satisfied: caveats.slice(0, 2).map(caveat => ({ kind: 'satisfied', caveat, evidence: '' })), unresolved: caveats.slice(2).map(caveat => ({ kind: 'unresolved', caveat, missingEvidence: '' })) } }),
    ))
  })
  const host = page.locator('#layout-test')
  await expect(host.getByText('month', { exact: true })).toHaveCount(1)
  await expect(host.getByText('BIGINT', { exact: true })).toHaveCount(1)
  await expect(host.getByText('Not null', { exact: true })).toHaveCount(1)
  await expect(host.getByText('Missing values', { exact: true })).toHaveCount(0)
  const tally = host.getByTestId('requirement-tally')
  await expect(tally.getByText('2 checked', { exact: true })).toBeVisible()
  await expect(tally.getByText('4 to review', { exact: true })).toBeVisible()
  await expect(tally.locator('[aria-hidden]')).toHaveCount(2)
  for (const width of [320, 390, 1440]) {
    await page.setViewportSize({ width, height: 850 })
    expect(await host.evaluate(el => el.scrollWidth <= el.clientWidth)).toBe(true)
    await page.screenshot({ path: info.outputPath(`metadata-${width}.png`) })
  }
  const disclosure = host.locator('summary')
  await disclosure.click()
  await expect(host.getByText('Requirement 1', { exact: true })).toBeHidden()
  await disclosure.focus()
  await page.keyboard.press('Enter')
  await expect(host.getByText('Requirement 1', { exact: true })).toBeVisible()
})

test('the company-wide example shows each variable role once', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand chapter list' }).click()
  await page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name: /Study design/ }).click()
  const roles = page.getByRole('list', { name: 'Variable roles', exact: true })
  if (info.project.name === 'mobile-chromium') await page.getByRole('group', { name: 'Panes' }).getByRole('button').filter({ hasText: /requirements|graph/i }).first().click()
  await expect(roles).toBeVisible()
  await expect(roles.getByText('Treatment', { exact: true })).toHaveCount(1)
  await expect(roles.getByText('treatment', { exact: true })).toHaveCount(0)
  await expect(roles.getByText('Outcome predictor', { exact: true })).toHaveCount(1)
  await expect(roles).not.toContainText('·')
  expect(await roles.evaluate(el => el.scrollWidth <= el.clientWidth)).toBe(true)
  await roles.screenshot({ path: info.outputPath('variable-roles.png') })
})
