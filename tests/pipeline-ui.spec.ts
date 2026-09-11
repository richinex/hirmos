import { expect, test, type Locator, type Page } from '@playwright/test'

const CITIES = 'id,city,population\n1,Lyon,513000\n2,Nantes,318000\n3,Lille,232000\n4,Nice,342000\n'
const REGIONS = 'id,region\n1,Auvergne-Rhône-Alpes\n2,Pays de la Loire\n3,Hauts-de-France\n4,Provence-Alpes-Côte d\'Azur\n'

const files = () => [
  { name: 'cities.csv', mimeType: 'text/csv', buffer: Buffer.from(CITIES) },
  { name: 'regions.csv', mimeType: 'text/csv', buffer: Buffer.from(REGIONS) },
]

const startPipeline = async (page: Page) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Pipeline')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.locator('input[type="file"][multiple]').setInputFiles(files())
  await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
  await expect(block(page, 'input-cities')).toContainText('4 rows · 3 columns', { timeout: 30_000 })
  await expect(block(page, 'input-regions')).toContainText('4 rows · 2 columns')
}

const block = (page: Page, id: string): Locator => page.getByTestId(`block-${id}`)

const addBlock = async (page: Page, name: string, prefix: string): Promise<string> => {
  await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name, exact: true }).click()
  const added = page.locator(`[data-testid^="block-${prefix}-"]`).last()
  await expect(added).toBeVisible()
  return (await added.getAttribute('data-testid'))!.replace('block-', '')
}

/** Drags from a block's output port to another block's input port, slowly enough for React Flow to track the pointer. */
const wire = async (page: Page, from: string, to: string, port = 0) => {
  const source = block(page, from).locator('[data-handleid="out"]')
  const target = block(page, to).locator(`[data-handleid="in-${port}"]`)
  const a = (await source.boundingBox())!
  const b = (await target.boundingBox())!
  const ax = a.x + a.width / 2, ay = a.y + a.height / 2, bx = b.x + b.width / 2, by = b.y + b.height / 2
  await page.mouse.move(ax, ay)
  await page.mouse.down()
  for (let i = 1; i <= 16; i++) await page.mouse.move(ax + (bx - ax) * i / 16, ay + (by - ay) * i / 16)
  await page.mouse.up()
}

const pick = async (page: Page, label: string, option: string) => {
  await page.getByRole('combobox', { name: label }).click()
  await page.getByRole('listbox').getByRole('option', { name: option, exact: true }).click()
}

const useAsSource = (page: Page): Locator => page.getByRole('button', { name: 'Use as source' })

test.describe('pipeline canvas', () => {
  test.skip(({ browserName }) => browserName !== 'chromium' || test.info().project.name !== 'chromium', 'The canvas runs DuckDB once')

  test('runs each block as it is wired, previews it, and reports what stops the output', async ({ page }) => {
    await startPipeline(page)
    await expect(page.getByTestId('pipeline-incomplete')).toHaveText('Use as source needs 1 input wired in; it has 0.')
    await expect(useAsSource(page)).toBeDisabled()

    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await expect(block(page, filter)).toContainText('waiting')
    await expect(page.getByTestId('block-status')).toHaveText('Not run yet: needs 1 input wired in; it has 0.')

    await wire(page, 'input-cities', filter)
    await expect(block(page, filter)).toContainText('4 rows · 3 columns')
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table')).toBeVisible()
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr')).toHaveCount(4)
    // A raw preview shows whole numbers as written, with no thousands grouping.
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr').first()).toContainText('513000')

    await page.getByRole('button', { name: 'Add a condition' }).click()
    await pick(page, 'Condition 1 column', 'population')
    await pick(page, 'Condition 1 test', 'is at least')
    await page.getByLabel('Condition 1 value').fill('300000')
    await expect(block(page, filter)).toContainText('population ≥ 300000')
    await expect(block(page, filter)).toContainText('3 rows · 3 columns')
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr')).toHaveCount(3)
    await page.getByText('As SQL').click()
    await expect(page.getByTestId('block-sql')).toHaveText('SELECT * FROM "cities" WHERE "population" >= 300000')

    await wire(page, filter, 'output')
    await expect(block(page, 'output')).toContainText('3 rows · 3 columns')
    await page.locator('.react-flow__pane').click({ position: { x: 20, y: 20 } })
    await expect(page.getByTestId('pipeline-incomplete')).toHaveCount(0)
    await expect(useAsSource(page)).toBeEnabled()
  })

  test('refuses arrows that would not make a pipeline and says why', async ({ page }) => {
    await startPipeline(page)
    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await wire(page, 'input-cities', filter)
    await expect(block(page, filter)).toContainText('4 rows')

    await wire(page, 'input-regions', filter)
    await expect(page.getByRole('alert')).toContainText('That input already has an arrow. Remove it first.')

    await wire(page, filter, filter)
    await expect(page.getByRole('alert')).toContainText('A block cannot feed itself.')

    const sort = await addBlock(page, 'Sort and limit', 'sort-limit')
    await wire(page, filter, sort)
    await wire(page, sort, filter)
    await expect(page.getByRole('alert')).toContainText('That input already has an arrow. Remove it first.')
  })

  test('shows a failed block, skips what follows it, and recovers when the block is fixed', async ({ page }) => {
    await startPipeline(page)
    const derive = await addBlock(page, 'Derive columns', 'derive-columns')
    const sort = await addBlock(page, 'Sort and limit', 'sort-limit')
    await wire(page, 'input-cities', derive)
    await wire(page, derive, sort)
    await wire(page, sort, 'output')

    await block(page, derive).click()
    await page.getByRole('button', { name: 'Add a column' }).click()
    await page.getByLabel('Derived column 1 name').fill('thousands')
    await page.getByLabel('Derived column 1 expression').fill('population / nope')
    await expect(block(page, derive)).toContainText('failed')
    await expect(page.getByTestId('block-status')).toContainText('Referenced column "nope" not found')
    await expect(page.getByTestId('block-status')).not.toContainText('CREATE OR REPLACE VIEW')
    await expect(block(page, sort)).toContainText('not run')
    await expect(block(page, sort)).toHaveCSS('opacity', '0.5')
    await expect(useAsSource(page)).toBeDisabled()

    await page.getByLabel('Derived column 1 expression').fill('population / 1000')
    await expect(block(page, derive)).toContainText('4 rows · 4 columns')
    await expect(block(page, sort)).toContainText('4 rows · 4 columns')
    await expect(useAsSource(page)).toBeEnabled()
  })

  test('joins two files, uses the result as the source, and rebuilds it on reopening', async ({ page }) => {
    await startPipeline(page)
    const join = await addBlock(page, 'Join', 'join')
    await wire(page, 'input-cities', join, 0)
    await wire(page, 'input-regions', join, 1)
    await block(page, join).click()
    await page.getByRole('button', { name: 'Add a key' }).click()
    await pick(page, 'Key 1 in Input file cities', 'id')
    await pick(page, 'Key 1 in Input file regions', 'id')
    await expect(block(page, join)).toContainText('4 rows · 4 columns')
    await expect(page.getByRole('region', { name: 'Join' }).getByRole('table')).toContainText('Pays de la Loire')

    await wire(page, join, 'output')
    await expect(useAsSource(page)).toBeEnabled()
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
    await expect(page.getByText(/built with a pipeline · 4 blocks · 2 inputs/)).toBeVisible()
    await page.getByRole('button', { name: 'Inspect data' }).click()
    await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 30_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('region')
    await page.waitForTimeout(1500)

    await page.goto('/app')
    await page.getByRole('list', { name: 'Projects' }).getByRole('button', { name: 'Open' }).first().click()
    await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
    await expect(page.getByText(/which the pipeline created from cities.csv/)).toBeVisible()
    await page.locator('input[type="file"][multiple]').setInputFiles(files())
    await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('region')
  })

  test('drops a block from the palette onto the canvas and removes it again', async ({ page }) => {
    await startPipeline(page)
    await expect(page.locator('.react-flow__node')).toHaveCount(3)
    await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name: 'Union', exact: true })
      .dragTo(page.locator('.react-flow__pane'), { targetPosition: { x: 600, y: 260 } })
    await expect(page.locator('.react-flow__node')).toHaveCount(4)
    const union = page.locator('[data-testid^="block-union-"]')
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(2)

    const unionId = (await union.getAttribute('data-testid'))!.replace('block-', '')
    await wire(page, 'input-cities', unionId, 0)
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(2)
    await wire(page, 'input-regions', unionId, 1)
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(3)
    await expect(union).toContainText('8 rows · 4 columns')

    await union.click()
    await page.getByRole('button', { name: 'Remove block' }).click()
    await expect(page.locator('.react-flow__node')).toHaveCount(3)
    await expect(page.locator('.react-flow__edge')).toHaveCount(0)
  })

  test('runs a script block in Pyodide, shows its errors by line, and uses its table as the source', async ({ page }) => {
    test.setTimeout(240_000)
    await startPipeline(page)
    const script = await addBlock(page, 'Script', 'script')
    await expect(page.getByTestId('python-runtime')).toContainText('pandas · numpy', { timeout: 180_000 })
    await wire(page, 'input-cities', script)
    await expect(block(page, script)).toContainText('4 rows · 3 columns', { timeout: 60_000 })

    const editor = page.getByTestId('python-editor')
    const set = async (code: string) => { await editor.locator('.cm-content').click(); await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.press('Backspace'); await page.keyboard.type(code) }
    await set('df = inputs[0]\ndf["thousands"] = df["population"] / 1000\nprepared = df[df["population"] >= 300000]\n')
    await expect(block(page, script)).toContainText('3 rows · 4 columns', { timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Script' }).getByRole('table')).toContainText('thousands')
    await expect(page.getByRole('region', { name: 'Script' }).getByRole('table')).toContainText('Lyon')

    await set('df = inputs[0]\nprepared = df[df["populaton"] > 1]\n')
    await expect(page.getByTestId('block-status')).toContainText("line 2: KeyError: 'populaton'", { timeout: 60_000 })
    await set('df = inputs[0]\nprepared = 42\n')
    await expect(page.getByTestId('block-status')).toContainText('prepared must be a DataFrame, not int', { timeout: 60_000 })
    await set('df = inputs[0]\nx = (\n')
    await expect(page.getByTestId('block-status')).toContainText("line 2: SyntaxError: '(' was never closed", { timeout: 60_000 })

    await set('print("grouping")\nprepared = inputs[0].groupby("id")["population"].sum()\n')
    await expect(block(page, script)).toContainText('4 rows · 2 columns', { timeout: 60_000 })
    await expect(page.getByTestId('block-stdout')).toHaveText('grouping')
    await wire(page, script, 'output')
    await expect(useAsSource(page)).toBeEnabled({ timeout: 60_000 })
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 60_000 })
    await page.getByRole('button', { name: 'Inspect data' }).click()
    await expect(page.getByRole('heading', { name: 'Data profile' })).toBeVisible({ timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('population')
  })

  test('shows a running script with elapsed time and cancels it, then runs again on a fresh runtime', async ({ page }) => {
    test.setTimeout(240_000)
    await startPipeline(page)
    const script = await addBlock(page, 'Script', 'script')
    await expect(page.getByTestId('python-runtime')).toContainText('pandas · numpy', { timeout: 180_000 })
    const editor = page.getByTestId('python-editor')
    const set = async (code: string) => { await editor.locator('.cm-content').click(); await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.press('Backspace'); await page.keyboard.type(code) }
    await set('import time\ntime.sleep(60)\nprepared = inputs[0]\n')
    await wire(page, 'input-cities', script)
    await block(page, script).click()
    await expect(page.getByRole('button', { name: 'Cancel run' })).toBeVisible({ timeout: 30_000 })
    await expect(block(page, script)).toContainText('running ·')
    await expect(page.getByTestId('block-status')).toHaveText('Running.')
    await page.getByRole('button', { name: 'Cancel run' }).click()
    await expect(block(page, script)).toContainText('failed', { timeout: 30_000 })
    await expect(page.getByTestId('block-status')).toContainText('The script was cancelled.')
    await expect(page.getByTestId('python-runtime')).toContainText('pandas · numpy', { timeout: 180_000 })
    await set('prepared = inputs[0].head(3)\n')
    await expect(block(page, script)).toContainText('3 rows · 3 columns', { timeout: 60_000 })
  })

  test('keeps each condition row with its own element when one is removed', async ({ page }) => {
    await startPipeline(page)
    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await wire(page, 'input-cities', filter)
    await expect(block(page, filter)).toContainText('4 rows')
    for (let i = 0; i < 3; i++) await page.getByRole('button', { name: 'Add a condition' }).click()
    await page.getByLabel('Condition 1 value').fill('one')
    await page.getByLabel('Condition 2 value').fill('two')
    await page.getByLabel('Condition 3 value').fill('three')
    const third = page.getByLabel('Condition 3 value')
    await page.getByRole('button', { name: 'Remove condition 2' }).click()
    await expect(page.getByLabel('Condition 2 value')).toHaveValue('three')
    await expect(third).toHaveCount(0)
    await expect(page.getByRole('button', { name: 'Remove condition 2' })).toBeFocused()
    await page.getByRole('button', { name: 'Remove condition 2' }).click()
    await expect(page.getByRole('button', { name: 'Remove condition 1' })).toBeFocused()
    await page.getByRole('button', { name: 'Remove condition 1' }).click()
    await expect(page.getByRole('button', { name: 'Add a condition' })).toBeFocused()
  })

  test('edits a script block in a Python editor with line numbers', async ({ page }) => {
    await startPipeline(page)
    await addBlock(page, 'Script', 'script')
    const editor = page.getByTestId('python-editor')
    await expect(editor.locator('.cm-lineNumbers')).toContainText('1')
    await expect(editor.locator('.cm-line').first()).toHaveText('import pandas as pd')
    await editor.locator('.cm-content').click()
    await page.keyboard.press('ControlOrMeta+End')
    await page.keyboard.type('prepared = df.head(2)')
    await expect(page.locator('[data-testid^="block-script-"]')).toContainText('5 lines')
  })
})

test('keeps the pipeline canvas inside a phone viewport', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'This is the phone layout check')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Pipeline')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.locator('input[type="file"][multiple]').setInputFiles(files())
  await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
  const width = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }))
  expect(width.content).toBeLessThanOrEqual(width.viewport)
})
