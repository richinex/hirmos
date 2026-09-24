import { expect, test, type Locator, type Page } from '@playwright/test'
import { choose, chapter } from './examples/support'

const CITIES = 'id,city,population\n1,Lyon,513000\n2,Nantes,318000\n3,Lille,232000\n4,Nice,342000\n'
const REGIONS = 'id,region\n1,Auvergne-Rhône-Alpes\n2,Pays de la Loire\n3,Hauts-de-France\n4,Provence-Alpes-Côte d\'Azur\n'

const files = () => [
  { name: 'cities.csv', mimeType: 'text/csv', buffer: Buffer.from(CITIES) },
  { name: 'regions.csv', mimeType: 'text/csv', buffer: Buffer.from(REGIONS) },
]

const block = (page: Page, id: string): Locator => page.getByTestId(`block-${id}`)

/** Opens the editor and gives the first input card the cities file; a second card takes the regions file when asked. */
const openEditor = async (page: Page) => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Pipeline')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.getByRole('radio', { name: 'Build a pipeline' }).click({ force: true })
  await page.getByRole('button', { name: 'Open the editor' }).click()
  await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
  await expect(block(page, 'input-1')).toContainText('choose a file')
}

const giveFile = async (page: Page, id: string, file: { name: string; mimeType: string; buffer: Buffer }) => {
  // On a phone the inspector is a sheet, and a tap on a block brings it up. The bar is read before the tap:
  // behind an open sheet it is aria-hidden and no longer found by role.
  const phone = await page.getByRole('group', { name: 'Panes' }).count() > 0
  await block(page, id).click({ position: { x: 20, y: 8 } })
  if (phone) await expect(page.getByRole('dialog', { name: 'Input file' })).toBeVisible()
  await page.getByLabel('File for this card').setInputFiles(file)
  if (phone) {
    await expect(block(page, id)).toContainText(/rows,|failed/, { timeout: 30_000 })
    await page.keyboard.press('Escape')
    await expect(page.locator('[data-vaul-overlay]')).toHaveCount(0)
  }
}

const startPipeline = async (page: Page, both = true) => {
  await openEditor(page)
  const [cities, regions] = files()
  await giveFile(page, 'input-1', cities!)
  await expect(block(page, 'input-1')).toContainText('4 rows, 3 columns', { timeout: 30_000 })
  if (!both) return
  await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name: 'Input file', exact: true }).click()
  const added = page.locator('[data-testid^="block-input-"]').last()
  const id = (await added.getAttribute('data-testid'))!.replace('block-', '')
  await giveFile(page, id, regions!)
  await expect(block(page, id)).toContainText('4 rows, 2 columns', { timeout: 30_000 })
  return id
}

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
    await page.locator('.react-flow__pane').click({ position: { x: 20, y: 20 } })
    await expect(page.getByTestId('pipeline-incomplete')).toContainText('Use as source needs 1 input wired in; it has 0.')
    await expect(useAsSource(page)).toBeDisabled()

    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await expect(block(page, filter)).toContainText('waiting')
    await expect(page.getByTestId('block-status')).toHaveText('Not run yet: needs 1 input wired in; it has 0.')

    await wire(page, 'input-1', filter)
    await expect(block(page, filter)).toContainText('4 rows, 3 columns')
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table')).toBeVisible()
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr')).toHaveCount(4)
    // A raw preview shows whole numbers as written, with no thousands grouping.
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr').first()).toContainText('513000')

    await page.getByRole('button', { name: 'Add a condition' }).click()
    await pick(page, 'Condition 1 column', 'population')
    await pick(page, 'Condition 1 test', 'is at least')
    await page.getByLabel('Condition 1 value').fill('300000')
    await expect(block(page, filter)).toContainText('population ≥ 300000')
    await expect(block(page, filter)).toContainText('3 rows, 3 columns')
    await expect(page.getByRole('region', { name: 'Filter rows' }).getByRole('table').locator('tbody tr')).toHaveCount(3)
    await page.getByText('As SQL').click()
    await expect(page.getByTestId('block-sql')).toHaveText('SELECT * FROM "cities" WHERE "population" >= 300000')

    await wire(page, filter, 'output')
    await expect(block(page, 'output')).toContainText('3 rows, 3 columns')
    await page.locator('.react-flow__pane').click({ position: { x: 20, y: 20 } })
    await expect(page.getByTestId('pipeline-incomplete')).toHaveCount(0)
    await expect(useAsSource(page)).toBeEnabled()
  })

  test('refuses arrows that would not make a pipeline and says why', async ({ page }) => {
    const regions = (await startPipeline(page))!
    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await wire(page, 'input-1', filter)
    await expect(block(page, filter)).toContainText('4 rows')

    await wire(page, regions, filter)
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
    await wire(page, 'input-1', derive)
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
    await expect(block(page, derive)).toContainText('4 rows, 4 columns')
    await expect(block(page, sort)).toContainText('4 rows, 4 columns')
    await expect(useAsSource(page)).toBeEnabled()
  })

  test('joins two files, uses the result as the source, and rebuilds it on reopening', async ({ page }) => {
    const regions = (await startPipeline(page))!
    const join = await addBlock(page, 'Join', 'join')
    await wire(page, 'input-1', join, 0)
    await wire(page, regions, join, 1)
    await block(page, join).click()
    await page.getByRole('button', { name: 'Add a key' }).click()
    await pick(page, 'Key 1 in Input file cities', 'id')
    await pick(page, 'Key 1 in Input file regions', 'id')
    await expect(block(page, join)).toContainText('4 rows, 4 columns')
    await expect(page.getByRole('region', { name: 'Join' }).getByRole('table')).toContainText('Pays de la Loire')

    await wire(page, join, 'output')
    await expect(useAsSource(page)).toBeEnabled()
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
    await expect(page.getByText(/built with a pipeline, 4 blocks, 2 inputs/)).toBeVisible()
    await page.getByRole('button', { name: 'Inspect data' }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('region')
    await page.waitForTimeout(1500)

    await page.goto('/app')
    await page.getByRole('button', { name: 'Open Pipeline', exact: true }).filter({ visible: true }).click()
    await expect(page.getByRole('heading', { name: 'Choose the data file again' })).toBeVisible()
    await expect(page.getByText(/which the pipeline created from cities.csv/)).toBeVisible()
    await page.locator('input[type="file"][multiple]').setInputFiles(files())
    await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('region')
  })

  test('drops a block from the palette onto the canvas and removes it again', async ({ page }) => {
    const regions = (await startPipeline(page))!
    await expect(page.locator('.react-flow__node')).toHaveCount(3)
    await page.getByRole('toolbar', { name: 'Add a block' }).getByRole('button', { name: 'Union', exact: true })
      .dragTo(page.locator('.react-flow__pane'), { targetPosition: { x: 600, y: 260 } })
    await expect(page.locator('.react-flow__node')).toHaveCount(4)
    const union = page.locator('[data-testid^="block-union-"]')
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(2)

    const unionId = (await union.getAttribute('data-testid'))!.replace('block-', '')
    await wire(page, 'input-1', unionId, 0)
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(2)
    await wire(page, regions, unionId, 1)
    await expect(union.locator('[data-handleid^="in-"]')).toHaveCount(3)
    await expect(union).toContainText('8 rows, 4 columns')

    await union.click()
    await page.getByRole('button', { name: 'Remove block' }).click()
    await expect(page.locator('.react-flow__node')).toHaveCount(3)
    await expect(page.locator('.react-flow__edge')).toHaveCount(0)
  })

  test('retains calendar window evidence through pipeline preparation', async ({page}, info) => {
    test.setTimeout(120000)
    await openEditor(page)
    const csv='date,y\n'+Array.from({length:60},(_,i)=>`${new Date(Date.UTC(2022,10,21+7*i)).toISOString().slice(0,10)},${i<8||i>=58?'':10+i*.2+Math.sin(i*.7)}`).join('\n')
    await giveFile(page,'input-1',{name:'calendar-window.csv',mimeType:'text/csv',buffer:Buffer.from(csv)})
    await expect(block(page,'input-1')).toContainText('60 rows, 2 columns')
    const calendar=await addBlock(page,'Calendar events','calendar-events')
    await wire(page,'input-1',calendar)
    await block(page,calendar).click()
    await pick(page,'Date column','date')
    await expect(block(page,calendar)).toContainText('60 rows, 3 columns')
    await wire(page,calendar,'output')
    await expect(block(page,'output')).toContainText('60 rows, 3 columns')
    await useAsSource(page).click()
    await page.getByRole('button',{name:'Inspect data',exact:true}).click()
    await page.getByRole('radio',{name:/Regular time series/}).click()
    await choose(page,'Time column','date')
    await choose(page,'Source frequency','Weekly')
    await page.getByRole('checkbox',{name:'y',exact:true}).check()
    await page.getByRole('radio',{name:'Complete contiguous interval',exact:true}).click()
    await page.getByRole('button',{name:/Create prepared dataset/}).click()
    await expect(page.getByRole('heading',{name:'Build a DAG or run discovery',exact:true})).toBeVisible()
    await chapter(page,/Time-series analysis/)
    await page.getByRole('radio',{name:'Interrupted series',exact:true}).click()
    const evidence=page.getByRole('region',{name:'Analysis window'})
    await expect(evidence).toContainText('kept 50 of 60 source rows')
    await expect(evidence).toContainText('Coverage ends on 31 December 2023')
    await expect(evidence).toContainText('This is before the end of')
    await evidence.scrollIntoViewIfNeeded()
    await page.screenshot({path:info.outputPath('calendar-window.png')})
  })

  test('marks the year-end shutdown from a calendar-events block and picks a custom window on the calendar', async ({ page }) => {
    await openEditor(page)
    const weeks = { name: 'weeks.csv', mimeType: 'text/csv', buffer: Buffer.from('week_start,merged\n2023-12-18,40\n2023-12-25,3\n2024-01-01,12\n2024-01-08,38\n') }
    await giveFile(page, 'input-1', weeks)
    await expect(block(page, 'input-1')).toContainText('4 rows, 2 columns', { timeout: 30_000 })
    const calendar = await addBlock(page, 'Calendar events', 'calendar-events')
    await wire(page, 'input-1', calendar)
    await expect(block(page, calendar)).toContainText('choose the date column')

    await block(page, calendar).click()
    await pick(page, 'Date column', 'week_start')
    await expect(block(page, calendar)).toContainText('holiday_share: year-end shutdown, 24 Dec to 2 Jan over week_start')
    await expect(block(page, calendar)).toContainText('4 rows, 3 columns')
    const table = page.getByRole('region', { name: 'Calendar events' }).getByRole('table')
    await expect(table).toContainText('0.1429')
    await expect(table).toContainText('0.2857')

    await page.getByRole('radio', { name: 'Custom window' }).click({ force: true })
    await expect(page.getByLabel('Calendar column name')).toHaveValue('calendar_share')
    const window = page.getByRole('button', { name: 'Window of the year' })
    await expect(window).toHaveText(/24 Dec to 2 Jan/)
    await window.click()
    // The calendar opens on the window's December and the January after; the picked days are stored without a year.
    await expect(page.getByRole('dialog')).toContainText('December')
    await page.getByRole('dialog').getByRole('button', { name: /^Wednesday, January 1st, 2025/ }).click()
    await page.getByRole('dialog').getByRole('button', { name: /^Friday, January 3rd, 2025/ }).click()
    await expect(window).toHaveText(/1 Jan to 3 Jan/)
    await expect(block(page, calendar)).toContainText('calendar_share: 1 Jan to 3 Jan over week_start')
    await expect(page.getByRole('region', { name: 'Calendar events' }).getByRole('table')).toContainText('0.4286')
  })

  test('runs a script block in Pyodide, shows its errors by line, and uses its table as the source', async ({ page }) => {
    test.setTimeout(240_000)
    await startPipeline(page)
    const script = await addBlock(page, 'Script', 'script')
    await expect(page.getByTestId('python-runtime')).toContainText(/pandas.*numpy/, { timeout: 180_000 })
    await wire(page, 'input-1', script)
    await expect(block(page, script)).toContainText('4 rows, 3 columns', { timeout: 60_000 })

    const editor = page.getByTestId('python-editor')
    const set = async (code: string) => { await editor.locator('.cm-content').click(); await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.press('Backspace'); await page.keyboard.type(code) }
    await set('df = inputs[0]\ndf["thousands"] = df["population"] / 1000\nprepared = df[df["population"] >= 300000]\n')
    await expect(block(page, script)).toContainText('3 rows, 4 columns', { timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Script' }).getByRole('table')).toContainText('thousands')
    await expect(page.getByRole('region', { name: 'Script' }).getByRole('table')).toContainText('Lyon')

    await set('df = inputs[0]\nprepared = df[df["populaton"] > 1]\n')
    await expect(page.getByTestId('block-status')).toContainText("line 2: KeyError: 'populaton'", { timeout: 60_000 })
    await set('df = inputs[0]\nprepared = [1, 2, 3]\n')
    await expect(page.getByTestId('block-status')).toContainText('prepared must be a DataFrame, a Series or a single value, not list', { timeout: 60_000 })
    await set('df = inputs[0]\nx = (\n')
    await expect(page.getByTestId('block-status')).toContainText("line 2: SyntaxError: '(' was never closed", { timeout: 60_000 })

    await set('print("grouping")\nprepared = inputs[0].groupby("id")["population"].sum()\n')
    await expect(block(page, script)).toContainText('4 rows, 2 columns', { timeout: 60_000 })
    await expect(page.getByTestId('block-stdout')).toHaveText('grouping')
    await wire(page, script, 'output')
    await expect(useAsSource(page)).toBeEnabled({ timeout: 60_000 })
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 60_000 })
    await page.getByRole('button', { name: 'Inspect data' }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 60_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('population')
  })

  test('a script that evaluates to one value reads as a number and still composes downstream', async ({ page }) => {
    test.setTimeout(240_000)
    await startPipeline(page)
    const script = await addBlock(page, 'Script', 'script')
    await expect(page.getByTestId('python-runtime')).toContainText(/pandas.*numpy/, { timeout: 180_000 })
    await wire(page, 'input-1', script)
    await expect(block(page, script)).toContainText('4 rows, 3 columns', { timeout: 60_000 })

    const editor = page.getByTestId('python-editor')
    const set = async (code: string) => { await editor.locator('.cm-content').click(); await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.press('Backspace'); await page.keyboard.type(code) }

    // The naive minus adjusted comparison from Facure, chapter 1: a bare number.
    await set('df = inputs[0]\nprepared = df["population"].mean()\n')
    await expect(block(page, script)).toContainText('1 row, 1 column', { timeout: 60_000 })
    await expect(page.getByTestId('pipeline-value')).toContainText('351250', { timeout: 60_000 })

    // A one-cell table is a table, not a value: the shape follows what the script assigned.
    await set('df = inputs[0]\nprepared = pd.DataFrame({"value": [df["population"].mean()]})\n')
    await expect(block(page, script)).toContainText('1 row, 1 column', { timeout: 60_000 })
    await expect(page.getByTestId('pipeline-value')).toBeHidden()
    await expect(page.getByRole('region', { name: 'Script' }).getByRole('table')).toContainText('351250')

    // A value keeps its output port, so it still reaches the source.
    await set('df = inputs[0]\nprepared = df["population"].mean()\n')
    await expect(page.getByTestId('pipeline-value')).toBeVisible({ timeout: 60_000 })
    await wire(page, script, 'output')
    await expect(useAsSource(page)).toBeEnabled({ timeout: 60_000 })
  })

  test('shows a running script with elapsed time and cancels it, then runs again on a fresh runtime', async ({ page }) => {
    test.setTimeout(240_000)
    await startPipeline(page)
    const script = await addBlock(page, 'Script', 'script')
    await expect(page.getByTestId('python-runtime')).toContainText(/pandas.*numpy/, { timeout: 180_000 })
    const editor = page.getByTestId('python-editor')
    const set = async (code: string) => { await editor.locator('.cm-content').click(); await page.keyboard.press('ControlOrMeta+A'); await page.keyboard.press('Backspace'); await page.keyboard.type(code) }
    await set('import time\ntime.sleep(60)\nprepared = inputs[0]\n')
    await wire(page, 'input-1', script)
    await block(page, script).click()
    await expect(page.getByRole('button', { name: 'Cancel run' })).toBeVisible({ timeout: 30_000 })
    await expect(block(page, script)).toContainText('running')
    await expect(page.getByTestId('block-status')).toHaveText('Running.')
    await page.getByRole('button', { name: 'Cancel run' }).click()
    await expect(block(page, script)).toContainText('failed', { timeout: 30_000 })
    await expect(page.getByTestId('block-status')).toContainText('The script was cancelled.')
    await expect(page.getByTestId('python-runtime')).toContainText(/pandas.*numpy/, { timeout: 180_000 })
    await set('prepared = inputs[0].head(3)\n')
    await expect(block(page, script)).toContainText('3 rows, 3 columns', { timeout: 60_000 })
  })

  test('keeps each condition row with its own element when one is removed', async ({ page }) => {
    await startPipeline(page)
    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await wire(page, 'input-1', filter)
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

  test('drags one card at a time, whatever is selected, without React Flow complaints', async ({ page }) => {
    const complaints: string[] = []
    page.on('console', (message) => { if (message.type() === 'error' || message.text().includes('not initialized')) complaints.push(message.text()) })
    const regions = (await startPipeline(page))!
    await block(page, 'input-1').click({ position: { x: 20, y: 8 } })
    const positions = () => page.locator('.react-flow__node').evaluateAll((nodes) => Object.fromEntries(nodes.map((node) => [node.getAttribute('data-id'), (node as HTMLElement).style.transform])))
    const before = await positions()
    const card = (await block(page, regions).boundingBox())!
    const x = card.x + card.width / 2, y = card.y + 12
    await page.mouse.move(x, y)
    await page.mouse.down()
    for (let i = 1; i <= 12; i++) await page.mouse.move(x + 5 * i, y + 8 * i)
    await page.mouse.up()
    await expect.poll(positions).not.toEqual(before)
    const after = await positions()
    expect(after['input-1']).toBe(before['input-1'])
    expect(after[regions]).not.toBe(before[regions])
    expect(after['output']).toBe(before['output'])
    const added = await addBlock(page, 'Union', 'union')
    const union = (await block(page, added).boundingBox())!
    await page.mouse.move(union.x + union.width / 2, union.y + 12)
    await page.mouse.down()
    for (let i = 1; i <= 8; i++) await page.mouse.move(union.x + union.width / 2 + 10 * i, union.y + 12)
    await page.mouse.up()
    await expect.poll(async () => Object.keys(await positions()).length).toBe(4)
    expect(complaints).toEqual([])
  })

  test('reopens the canvas on the source it made: from Source selected, from the profile with a confirm, and after the files are forgotten', async ({ page }) => {
    test.setTimeout(120_000)
    const regions = (await startPipeline(page))!
    const join = await addBlock(page, 'Join', 'join')
    await wire(page, 'input-1', join, 0)
    await wire(page, regions, join, 1)
    await block(page, join).click()
    await page.getByRole('button', { name: 'Add a key' }).click()
    await pick(page, 'Key 1 in Input file cities', 'id')
    await pick(page, 'Key 1 in Input file regions', 'id')
    await wire(page, join, 'output')
    await expect(useAsSource(page)).toBeEnabled()
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })

    // Back from Source selected: the same cards, arrows and join keys, files still loaded.
    await page.getByRole('button', { name: 'Edit pipeline' }).click()
    await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
    await expect(page.locator('.react-flow__node')).toHaveCount(4)
    await expect(page.locator('.react-flow__edge')).toHaveCount(3)
    await expect(block(page, join)).toContainText('4 rows, 4 columns', { timeout: 30_000 })
    await block(page, join).click()
    await expect(page.getByRole('combobox', { name: 'Key 1 in Input file cities' })).toContainText('id')

    // A change: the arrow into the output goes, a filter takes its place.
    // A smoothstep arrow's box centre is off its path, so the arrow is selected by the event itself; its button removes it.
    await page.locator(`.react-flow__edge[data-id="${join}->output:0"]`).dispatchEvent('click')
    await page.getByRole('button', { name: 'Remove arrow' }).click()
    await expect(page.locator('.react-flow__edge')).toHaveCount(2)
    const filter = await addBlock(page, 'Filter rows', 'filter-rows')
    await wire(page, join, filter)
    await wire(page, filter, 'output')
    await block(page, filter).click()
    await page.getByRole('button', { name: 'Add a condition' }).click()
    await pick(page, 'Condition 1 column', 'population')
    await pick(page, 'Condition 1 test', 'is at least')
    await page.getByLabel('Condition 1 value').fill('300000')
    await expect(block(page, filter)).toContainText('3 rows, 4 columns')
    await useAsSource(page).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })
    await page.getByRole('button', { name: 'Inspect data' }).click()
    await expect(page.locator('#data-profile-title')).toBeVisible({ timeout: 30_000 })
    await expect(page.getByRole('region', { name: 'Physical schema' })).toContainText('region')

    // Opening preserves the accepted source; confirmation belongs to accepting the replacement.
    await page.getByRole('button', { name: 'Edit data', exact: true }).click()
    await page.getByRole('button', { name: 'Edit pipeline' }).click()
    await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
    await expect(page.locator('.react-flow__node')).toHaveCount(5)
    await expect(block(page, filter)).toContainText('3 rows, 4 columns', { timeout: 30_000 })
    await useAsSource(page).click()
    await page.getByRole('alertdialog', { name: 'Replace dataset?' }).getByRole('button', { name: 'Replace dataset', exact: true }).click()
    await expect(page.getByRole('heading', { name: 'Source selected' })).toBeVisible({ timeout: 30_000 })

    // Files forgotten, as after a reload: the editor asks for them first, refuses the wrong one, then opens where it left off.
    await page.evaluate(async () => { const files = await import(new URL('/src/data/inputFiles.ts', window.location.href).href); files.forgetInputFiles() })
    await page.getByRole('button', { name: 'Edit pipeline' }).click()
    await expect(page.getByRole('heading', { name: 'Choose the input files again' })).toBeVisible()
    await page.locator('input[type="file"][multiple]').setInputFiles([files()[0]!])
    await expect(page.getByRole('alert')).toContainText('do not include regions.csv')
    await page.locator('input[type="file"][multiple]').setInputFiles(files())
    await expect(page.getByTestId('pipeline-canvas')).toBeVisible({ timeout: 30_000 })
    await expect(page.locator('.react-flow__node')).toHaveCount(5)
    await expect(block(page, filter)).toContainText('3 rows, 4 columns', { timeout: 30_000 })
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

test('keeps the pipeline canvas inside a phone viewport, fills the stage, and pans under a finger', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile-chromium', 'This is the phone layout check')
  await openEditor(page)
  await giveFile(page, 'input-1', files()[0]!)
  await expect(block(page, 'input-1')).toContainText('4 rows', { timeout: 30_000 })

  // A tap on a block brings its settings up as a sheet, and the bar shows which pane is open; a second tap on
  // the same block, its selection unchanged, brings the sheet up again.
  // Behind the open sheet the bar is aria-hidden, so its opener is read by attribute rather than by role.
  const opener = page.locator('[aria-label="Panes"] button', { hasText: 'Input file' })
  for (let tap = 0; tap < 2; tap++) {
    await block(page, 'input-1').click({ position: { x: 20, y: 8 } })
    await expect(page.getByRole('dialog', { name: 'Input file' })).toBeVisible()
    await expect(opener).toHaveAttribute('aria-expanded', 'true')
    await page.keyboard.press('Escape')
    await expect(page.locator('[data-vaul-overlay]')).toHaveCount(0)
    await expect(opener).toHaveAttribute('aria-expanded', 'false')
  }
  const paneBox = (await page.locator('.react-flow__pane').boundingBox())!
  await page.mouse.click(paneBox.x + 10, paneBox.y + 10)
  const width = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }))
  expect(width.content).toBeLessThanOrEqual(width.viewport)

  // The palette stays on one row; every block remains reachable by scrolling.
  const palette = page.getByRole('toolbar', { name: 'Add a block' })
  expect(await palette.evaluate((el) => el.clientHeight)).toBeLessThan(60)
  const chips = await palette.locator('button[draggable]').evaluateAll((buttons) => buttons.map((button) => { const box = button.getBoundingClientRect(); return { name: button.getAttribute('aria-label'), inView: box.width > 0 && box.right <= document.documentElement.clientWidth } }))
  expect(chips.map((chip) => chip.name)).toEqual(['Input file', 'Filter rows', 'Sort and limit', 'Select columns', 'Derive columns', 'Calendar events', 'Join', 'Union', 'Group and aggregate', 'Script'])
  for (const chip of await palette.locator('button[draggable]').all()) {
    await chip.scrollIntoViewIfNeeded()
    await expect(chip).toBeInViewport()
  }
  await palette.locator('[data-block-scroll]').evaluate(element => element.scrollTo({ left: 0, behavior: 'instant' }))

  // The canvas takes the stage's height, so its controls sit along the foot of the screen, not mid-way.
  const canvas = (await page.getByTestId('pipeline-canvas').boundingBox())!
  const controls = (await page.getByRole('toolbar', { name: 'Canvas' }).boundingBox())!
  expect(canvas.height).toBeGreaterThan(400)
  expect(controls.y + controls.height).toBeGreaterThan(canvas.y + canvas.height - 40)
  for (const name of ['Zoom in', 'Zoom out', 'Fit the pipeline', 'Tidy pipeline', 'Lock the view']) await expect(page.getByRole('button', { name, exact: true })).toBeInViewport()

  // A finger on empty canvas pans it.
  const before = await page.locator('.react-flow__viewport').getAttribute('style')
  const pane = (await page.locator('.react-flow__pane').boundingBox())!
  const x = pane.x + 30, y = pane.y + pane.height / 2
  const client = await page.context().newCDPSession(page)
  await client.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y }] })
  for (let i = 1; i <= 8; i++) await client.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x, y: y - 15 * i }] })
  await client.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] })
  await expect.poll(async () => page.locator('.react-flow__viewport').getAttribute('style')).not.toBe(before)
})
