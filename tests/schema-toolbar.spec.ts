import { expect, test } from '@playwright/test'

for (const mixed of [false, true]) {
  test(`schema type filters only appear when they offer a choice: mixed=${mixed}`, async ({ page }, info) => {
    await page.goto('/app')
    await page.getByRole('textbox', { name: 'Project name' }).fill('Schema toolbar')
    await page.getByRole('button', { name: 'Create project' }).click()
    const csv = mixed ? 'month,team,value,active\n1,A,2,1\n2,B,3,0' : 'month,tool,legacy,bugs\n1,0,2,3\n2,1,3,4'
    await page.locator('input[type=file]').setInputFiles({ name: 'schema.csv', mimeType: 'text/csv', buffer: Buffer.from(csv) })
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
  })
}
