import { expect, test } from '@playwright/test'

for (const mixed of [false, true]) {
  test(`schema type filters only appear when they offer a choice: mixed=${mixed}`, async ({ page }, info) => {
    test.setTimeout(90000)
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill('Schema toolbar')
    await page.getByRole('button', { name: 'Create project' }).click()
    const csv = mixed ? 'month,team,value,active\n1,A,2,1\n2,B,3,0' : 'month,tool,legacy,bugs\n1,0,2,3\n2,1,3,4'
    await page.locator('input[type=file]').setInputFiles({ name: 'company-wide-adoption.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
    await page.getByRole('button', { name: 'Inspect data' }).click()
    const schema = page.getByRole('region', { name: 'Physical schema' })
    await expect(schema).toBeVisible()
    const facets = schema.getByRole('group', { name: 'Column types' })
    const search = schema.getByRole('searchbox', { name: 'Search columns' })
    if (mixed) {
      await expect(facets).toBeVisible()
      await facets.getByRole('button', { name: 'Text 1' }).click()
      await expect(schema.getByRole('table')).toContainText('team')
      await expect(schema.getByRole('table')).not.toContainText('month')
      await search.fill('team')
      await expect(facets).toBeVisible()
      await facets.getByRole('button', { name: 'Text 1' }).click()
      await search.fill('')
      await expect(schema.getByRole('table')).toContainText('month')
    } else {
      await expect(facets).toHaveCount(0)
      for (const width of [320, 390, 768, 1440]) {
        await page.setViewportSize({ width, height: 844 })
        await search.scrollIntoViewIfNeeded()
        const field = (await search.locator('..').boundingBox())!
        const density = (await schema.getByRole('group', { name: 'Row density' }).boundingBox())!
        expect(Math.abs(field.y - density.y)).toBeLessThanOrEqual(1)
        expect(Math.abs(field.height - density.height)).toBeLessThanOrEqual(1)
        await schema.screenshot({ path: info.outputPath(`schema-${width}.png`) })
      }
    }
    if (info.project.name === 'mobile-chromium') {
      for (const viewport of [{ width: 320, height: 740 }, { width: 360, height: 800 }, { width: 390, height: 844 }, { width: 430, height: 932 }, { width: 844, height: 390 }]) {
        await page.setViewportSize(viewport)
        await schema.scrollIntoViewIfNeeded()
        await page.screenshot({ path: info.outputPath(`page-${viewport.width}.png`), fullPage: true })
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
        const preview = page.getByRole('region', { name: 'Preview', exact: true })
        for (const [region, field, control] of [
          [schema, search.locator('..'), schema.getByRole('group', { name: 'Row density' })],
          [preview, preview.getByRole('searchbox', { name: 'Search rows' }).locator('..'), preview.locator('summary')],
        ] as const) {
          const row = field.locator('..')
          const rowBox = (await row.boundingBox())!
          const fieldBox = (await field.boundingBox())!
          const controlBox = (await control.boundingBox())!
          expect(Math.abs(fieldBox.y - controlBox.y)).toBeLessThanOrEqual(1)
          expect(Math.abs(fieldBox.height - controlBox.height)).toBeLessThanOrEqual(1)
          expect(Math.abs(controlBox.x + controlBox.width - rowBox.x - rowBox.width)).toBeLessThanOrEqual(1)
          if (viewport.width < 640) {
            const availableWidth = await region.locator(':scope > div').first().evaluate(element => {
              const css = getComputedStyle(element)
              return element.clientWidth - parseFloat(css.paddingLeft) - parseFloat(css.paddingRight)
            })
            expect(Math.abs(rowBox.width - availableWidth)).toBeLessThanOrEqual(1)
          }
        }
        const schemaBox = (await schema.boundingBox())!
        const previewBox = (await preview.boundingBox())!
        expect(previewBox.y).toBeGreaterThanOrEqual(schemaBox.y + schemaBox.height)
      }
      await page.setViewportSize({ width: 390, height: 844 })
      await page.evaluate(() => { document.documentElement.style.fontSize = '125%' })
      await schema.scrollIntoViewIfNeeded()
      expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
      await page.screenshot({ path: info.outputPath('page-larger-text.png'), fullPage: true })
    }
  })
}
