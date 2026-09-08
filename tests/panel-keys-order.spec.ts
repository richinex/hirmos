import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { expect, test } from '@playwright/test'

// The panel keys and the value matrix are read in two queries. The value matrix comes back in source row
// order, so the keys must too: a key query that sorts by time would pair every row's unit and period with
// another row's values, and a balanced panel would still pass the structure check.
const FIXTURE = 'jpcmci-shuffled-panel.csv'

test('panel keys stay on the source rows of a shuffled balanced panel', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Data boundary runs once')
  const lines = readFileSync(fileURLToPath(new URL(`./fixtures/${FIXTURE}`, import.meta.url)), 'utf8').trim().split(/\r?\n/)
  const header = lines[0].split(',').map((cell) => cell.trim())
  const at = (name: string) => header.indexOf(name)
  const original = lines.slice(1).map((line) => line.split(','))
  const byCell = new Map(original.map((cells) => [`${cells[at('domain')]}|${cells[at('step')]}`, cells]))

  await page.goto('/app')
  const observed: { readonly units: string[]; readonly periods: string[]; readonly c4: number[]; readonly c5: number[] } = await page.evaluate(async (fixture) => {
    const [dataModule, workflowModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
    ])
    const response = await fetch(`/tests/fixtures/${fixture}`)
    if (!response.ok) throw new Error(`Fixture request failed with ${response.status}.`)
    const file = new File([await response.blob()], fixture, { type: 'text/csv' })
    const profiled = await dataModule.profileSourceInWorker(workflowModule.newImportRequestId(), file)
    if (!profiled.ok) throw new Error(`Profile failed: ${profiled.error.kind}`)
    const column = (name: string) => {
      const found = profiled.value.columns.find((candidate: { readonly name: string }) => candidate.name === name)
      if (!found) throw new Error(`${name} was not profiled.`)
      return found.id
    }
    const keys = await dataModule.materializePanelKeysInWorker(file, profiled.value, column('domain'), column('step'))
    if (!keys.ok) throw new Error(`Panel keys failed: ${keys.error.kind}`)
    const values = await dataModule.materializeNumericColumnsInWorker(file, profiled.value, [column('C4'), column('C5')])
    if (!values.ok) throw new Error(`Materialization failed: ${values.error.kind}`)
    const rows = values.value.rowCount
    const label = new Map(keys.value.periods.map((period: { readonly code: number; readonly label: string }) => [period.code, period.label]))
    return {
      units: keys.value.units,
      periods: keys.value.periodCodes.map((code: number) => label.get(code) ?? ''),
      c4: Array.from(values.value.values.subarray(0, rows)),
      c5: Array.from(values.value.values.subarray(rows, 2 * rows)),
    }
  }, FIXTURE)

  expect(observed.units).toHaveLength(original.length)
  expect(observed.units).toEqual(original.map((cells) => cells[at('domain')]))
  expect(observed.periods).toEqual(original.map((cells) => cells[at('step')]))
  for (let row = 0; row < original.length; row += 1) {
    const cells = byCell.get(`${observed.units[row]}|${observed.periods[row]}`)
    expect(cells, `row ${row} names a cell the file has`).toBeDefined()
    expect(observed.c4[row]).toBeCloseTo(Number(cells?.[at('C4')]), 6)
    expect(observed.c5[row]).toBeCloseTo(Number(cells?.[at('C5')]), 6)
  }
})
