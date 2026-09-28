import { expect, test } from '@playwright/test'

test('sidebar project name and folder follow the rail foreground in both themes', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand section list' }).click()
  const link = page.locator('.dashboard-project-link')
  await expect(link).toBeVisible()
  for (const theme of ['light', 'dark']) {
    await page.evaluate(value => { document.documentElement.dataset.theme = value }, theme)
    const colors = await link.evaluate(el => {
      const probe = document.createElement('span')
      probe.style.color = 'var(--color-rail-ink)'
      el.append(probe)
      const expected = getComputedStyle(probe).color
      probe.remove()
      return {
        expected,
        name: getComputedStyle(el.querySelector('.font-medium')!).color,
        folder: getComputedStyle(el.querySelector('.msym')!).color,
      }
    })
    expect(colors.name).toBe(colors.expected)
    expect(colors.folder).toBe(colors.expected)
    await page.locator('.dashboard-sidebar').screenshot({ path: info.outputPath(`sidebar-${theme}.png`) })
  }
})

test('project actions fill the input width with equal button sizes', async ({ page }) => {
  await page.goto('/app/projects')
  const input = await page.getByRole('textbox', { name: 'Project name', exact: true }).boundingBox()
  const create = await page.getByRole('button', { name: 'Create project', exact: true }).boundingBox()
  const open = await page.getByRole('button', { name: 'Open exported file', exact: true }).boundingBox()
  expect(input).not.toBeNull()
  expect(create).not.toBeNull()
  expect(open).not.toBeNull()
  expect(create!.x).toBeCloseTo(input!.x, 0)
  expect(open!.x + open!.width).toBeCloseTo(input!.x + input!.width, 0)
  expect(create!.width).toBeCloseTo(open!.width, 0)
  expect(create!.height).toBeCloseTo(open!.height, 0)
  expect(create!.y).toBeCloseTo(open!.y, 0)
})

test('reset restores the saved example without opening it; cancel keeps edits', async ({ page }) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
  await page.waitForTimeout(600)
  await page.evaluate(async () => {
    await new Promise<void>((resolve, reject) => {
      const request = indexedDB.open('hirmos', 1)
      request.onsuccess = () => {
        const db = request.result
        const tx = db.transaction('projects', 'readwrite')
        const store = tx.objectStore('projects')
        const cursor = store.openCursor()
        cursor.onsuccess = () => {
          const row = cursor.result
          if (!row) return
          const record = row.value
          const body = JSON.parse(record.body)
          body.project.name = 'Edited example'
          record.body = JSON.stringify(body)
          record.header.name = 'Edited example'
          row.update(record)
        }
        tx.oncomplete = () => { db.close(); resolve() }
        tx.onerror = () => reject(tx.error)
      }
      request.onerror = () => reject(request.error)
    })
  })
  await page.goto('/app/projects')
  const savedName = () => page.evaluate(async () => new Promise<string>((resolve) => {
    const request = indexedDB.open('hirmos', 1)
    request.onsuccess = () => {
      const db = request.result
      const tx = db.transaction('projects', 'readonly')
      const rows = tx.objectStore('projects').getAll()
      rows.onsuccess = () => resolve(JSON.parse(rows.result[0].body).project.name)
      tx.oncomplete = () => db.close()
    }
  }))
  const reset = page.getByRole('button', { name: 'Reset AI adoption, company-wide', exact: true })
  await reset.click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Cancel', exact: true }).click()
  expect(await savedName()).toBe('Edited example')
  await reset.click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Reset example', exact: true }).click()
  await expect.poll(savedName).toBe('AI adoption, company-wide')
  await expect(page.getByRole('heading', { name: 'Projects', exact: true })).toBeVisible()
  await expect(page).toHaveURL(/\/app\/projects$/)
  await page.getByRole('button', { name: 'Open AI adoption, company-wide', exact: true }).click()
  await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
})
