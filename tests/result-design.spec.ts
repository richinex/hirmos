import {expect,test} from '@playwright/test'

test('a counterfactual run uses the shared cards without changing its figures', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open A simulated process with a collider', exact: true }).click()
  if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Counterfactuals/ }).click()
  await page.getByRole('button', { name: 'Run counterfactual', exact: true }).click()
  const cards = page.locator('.metric-cards[aria-label="Counterfactual summary"]').first()
  await expect(cards).toBeVisible({ timeout: 60_000 })
  await expect(cards.locator('.metric-tile')).toHaveCount(4)
  await expect(cards.getByText('Effect for every row', { exact: true })).toBeVisible()
  for (const tile of await cards.locator('.metric-tile').all()) {
    expect(await tile.evaluate(el => el.scrollWidth <= el.clientWidth + 1)).toBe(true)
  }
  await cards.screenshot({ path: info.outputPath('counterfactual-cards.png') })
})

for (const example of [
  { name: 'AI usage intensity', chapter: /Estimation/, summary: 'Diagnostics' },
  { name: 'AI adoption, March cohort', chapter: /Estimation/, summary: 'Diagnostics' },
  { name: 'A simulated process with a collider', chapter: /Sensitivity/, summary: 'Refuters' },
]) {
  test(`${example.name} retains readable summary cards`, async ({ page }, info) => {
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${example.name}`, exact: true }).click()
    if (info.project.name === 'mobile-chromium') await page.getByRole('button', { name: 'Expand section list' }).click()
    await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: example.chapter }).click()
    const cards = page.locator(`.metric-cards[aria-label="${example.summary}"]`).first()
    await expect(cards).toBeVisible()
    expect(await cards.locator('.metric-tile').count()).toBeGreaterThan(0)
    for (const theme of ['light', 'dark']) {
      await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
      const widths = await cards.locator('.metric-tile').evaluateAll(tiles => tiles.map(tile => ({ width: tile.clientWidth, content: tile.scrollWidth })))
      for (const width of widths) expect(width.content).toBeLessThanOrEqual(width.width + 1)
      await cards.screenshot({ path: info.outputPath(`summary-${theme}.png`) })
    }
  })
}

test('example cards are lit blocks and table headers sit on their surface in both themes',async({page},info)=>{
  await page.goto('/app/projects')
  const resolve=(colour:string)=>page.evaluate(colour=>{const probe=document.createElement('div');probe.style.backgroundColor=colour;document.body.append(probe);const result=getComputedStyle(probe).backgroundColor;probe.remove();return result},colour)
  for(const theme of ['light','dark']) {
    await page.evaluate(theme=>{document.documentElement.dataset.theme=theme},theme)
    const cards=page.getByRole('list',{name:'Examples'})
    await expect(cards).toBeVisible()
    const card=await cards.locator('.example-card').first().evaluate(el=>{const css=getComputedStyle(el);return {background:css.backgroundColor,border:css.borderWidth,shadow:css.boxShadow,panel:getComputedStyle(document.documentElement).getPropertyValue('--color-panel').trim()}})
    expect(card.background).toBe(await resolve(card.panel))
    expect(card.border).toBe('0px')
    expect(card.shadow).not.toBe('none')
    expect(await page.locator('body').evaluate(el=>el.scrollWidth<=innerWidth+1)).toBe(true)
    if(info.project.name==='chromium') await cards.screenshot({path:info.outputPath(`examples-${theme}.png`)})
  }
  if(info.project.name!=='chromium') return
  await page.getByRole('button',{name:'Open Seat-belt law and road deaths',exact:true}).click()
  const schema=page.getByRole('table').first()
  await expect(schema).toBeVisible({timeout:30_000})
  for(const theme of ['light','dark']) {
    await page.evaluate(theme=>{document.documentElement.dataset.theme=theme},theme)
    const header=schema.getByRole('columnheader').first()
    const colours=await header.evaluate(el=>({background:getComputedStyle(el).backgroundColor,position:getComputedStyle(el).position,surface:getComputedStyle(el).getPropertyValue('--table-surface').trim()||getComputedStyle(document.documentElement).getPropertyValue('--color-stage').trim()}))
    expect(colours.position).toBe('sticky')
    expect(colours.background).toBe(await resolve(colours.surface))
  }
})

test('dashboard summary cards and borderless equations retain content on desktop and mobile',async({page},info)=>{
  await page.goto('/app/projects')
  await page.getByRole('button',{name:/Open Breast cancer/i}).click()
  if(info.project.name==='mobile-chromium')await page.getByRole('button',{name:'Expand section list'}).click()
  await page.getByRole('navigation',{name:'Workspace sections'}).getByRole('button',{name:/Survival analysis/}).click()
  const cards=page.getByTestId('survival-summary-cards').first()
  await expect(cards.locator('.metric-tile')).toHaveCount(3)
  await expect(cards.getByText(/log likelihood/)).toBeVisible()
  const equation=page.getByTestId('survival-equation').first()
  await equation.locator('summary').first().click()
  await expect(equation.locator('.katex')).not.toHaveCount(0)
  const fitted=equation.getByTestId('survival-equation-fitted')
  expect(await fitted.evaluate(el=>getComputedStyle(el).borderLeftWidth)).toBe('0px')
  for(const theme of ['light','dark']) {
    await page.evaluate(theme=>{document.documentElement.dataset.theme=theme},theme)
    await cards.screenshot({path:info.outputPath(`cards-${theme}.png`)})
    await equation.screenshot({path:info.outputPath(`equations-${theme}.png`)})
    expect(await equation.evaluate(el=>el.scrollWidth<=el.clientWidth+1)).toBe(true)
  }
})
