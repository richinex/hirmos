import { readFileSync } from 'node:fs'
import { expect, test } from '@playwright/test'

test('full microservices model checks match the preserved v0.14 notebook through the Hirmos worker', async ({ page }, info) => {
  test.skip(process.env.HIRMOS_FULL_ORACLE !== '1' && process.env.HIRMOS_GCM_NOTEBOOK !== '1', 'Opt-in full-size numerical replay.')
  test.setTimeout(1_800_000)
  const reference = '../octopus/rust-causal-transpile/oracle/gcm/fixtures/'
  const model = JSON.parse(readFileSync(reference + 'model.json', 'utf8'))
  const oracle = JSON.parse(readFileSync(reference + 'notebook-evaluation-hash0.json', 'utf8'))
  const [header, ...lines] = readFileSync('docs/2026-09-15-dowhy-microservices-rca/data/baseline.csv', 'utf8').trim().split(/\r?\n/)
  const columns = header.split(',')
  const rows = lines.map((line) => line.split(',').map(Number))
  const values = model.names.flatMap((name: string) => rows.map((row) => row[columns.indexOf(name)]))
  await page.routeWebSocket(/.*/, (socket) => socket.close())
  await page.exposeFunction('reportPhase', (phase: unknown) => console.log('GCM', phase))
  await page.goto('/app')
  const result = await page.evaluate(async ({ names, edges, rows, values }) => {
    const { checkRootCause } = await import(new URL('/src/analysis/client.ts', location.href).href)
    return checkRootCause(new Float64Array(values), { names, edges, rows, seed: 0 }, (progress: unknown) => Reflect.get(window, 'reportPhase')(progress))
  }, { names: model.names, edges: model.edges, rows: rows.length, values })
  await info.attach('model-checks.json', { body: JSON.stringify(result, null, 2), contentType: 'application/json' })
  expect(result.ok, JSON.stringify(result)).toBe(true)
  if (!result.ok) return
  const actual = result.value
  const expected = oracle.result.fields
  for (const entry of actual.mechanisms) {
    const wanted = expected.mechanism_performances[model.names[entry.node]].fields
    for (const [key, value] of Object.entries(entry)) {
      if (key === 'kind' || key === 'node') continue
      const field = key === 'klDivergence' ? 'kl_divergence' : key
      expect(Math.abs(Number(value) - wanted[field]), `${model.names[entry.node]} ${key}`).toBeLessThanOrEqual(1e-10)
    }
  }
  for (const entry of actual.invertibility) {
    const wanted = expected.pnl_assumptions[model.names[entry.node]]
    expect(Math.abs(entry.pValue - wanted[0])).toBeLessThanOrEqual(1e-8)
    expect(entry.rejected).toBe(wanted[1])
  }
  expect(Math.abs(actual.overallDivergence - expected.overall_kl_divergence)).toBeLessThanOrEqual(1e-9)
  for (const [method, count, p] of [['LMC', 'lmcViolations', 'pValueLmc'], ['TPA', 'tpaViolations', 'pValueTpa']]) {
    const wanted = expected.graph_falsification.fields.summary[`FalsifyConst.VALIDATE_${method}`]
    expect(actual.given[count]).toBe(wanted['FalsifyConst.GIVEN_VIOLATIONS'])
    expect(actual.permutations.map((entry: Record<string, number>) => entry[count])).toEqual(wanted['FalsifyConst.PERM_VIOLATIONS'])
    expect(actual[p]).toBe(wanted['FalsifyConst.P_VALUE'])
  }
  expect(actual.random).toEqual({ keys: oracle.exit_rng[1], position: oracle.exit_rng[2], normal: oracle.exit_rng[3] === 0 ? null : oracle.exit_rng[4] })
  const notebook = JSON.parse(readFileSync(reference + 'notebook-hash0.json', 'utf8'))
  const wasmCloud = JSON.parse(readFileSync(reference + 'wasm-cell-24-summary.json', 'utf8'))
  const summaries = JSON.parse(readFileSync(reference + 'geometric-median-wasm-linux.json', 'utf8'))
  const sameInput = summaries.cases.find((entry: { samples: unknown }) => JSON.stringify(entry.samples) === JSON.stringify(wasmCloud.samples))
  expect(sameInput).toBeDefined()
  let random = actual.random
  for (const cell of notebook.cells) {
    const file = cell.cell === 24 ? 'unusual-request.csv' : 'degraded.csv'
    const [header, ...lines] = readFileSync('docs/2026-09-15-dowhy-microservices-rca/data/' + file, 'utf8').trim().split(/\r?\n/)
    const columns = header.split(',')
    const observed = lines.map((line) => line.split(',').map(Number))
    const queryValues = model.names.flatMap((name: string) => observed.map((row) => row[columns.indexOf(name)]))
    const fitting = cell.cell === 38 ? queryValues : values
    const query = cell.cell === 24 ? { kind: 'anomaly', samples: 3000 } : cell.cell === 35
      ? { kind: 'change', rows: observed.length, samples: 2000, execution: { kind: 'recordedBatches', repetitions: cell.batches.map((batches: number[][][]) => batches.map((batch) => batch.map((coalition) => coalition.map(Boolean)))) } }
      : { kind: 'intervention', rows: observed.length, order: columns.map((name) => model.names.indexOf(name)), shifts: [{ node: model.names.indexOf('Caching Service'), amount: -1 }, { node: model.names.indexOf('Shipping Cost Service'), amount: 2 }] }
    const request = { names: model.names, edges: model.edges, rows: cell.cell === 38 ? observed.length : rows.length, target: model.names.indexOf('Website'), repetitions: 10, upperQuantile: 0.95, fraction: cell.cell === 35 ? 0.6 : 0.75, random: { kind: 'resume', state: random }, query }
    const result = await page.evaluate(async ({ request, values }) => {
      const { runRootCause } = await import(new URL('/src/analysis/client.ts', location.href).href)
      return runRootCause(new Float64Array(values), request, (progress: unknown) => Reflect.get(window, 'reportPhase')(progress))
    }, { request, values: [...fitting, ...queryValues] })
    await info.attach(`cell-${cell.cell}.json`, { body: JSON.stringify({ request, result }, null, 2), contentType: 'application/json' })
    expect(result.ok, JSON.stringify(result)).toBe(true)
    if (!result.ok) return
    random = result.value.random
    expect(random.keys).toEqual(cell.exit_rng[1])
    expect(random.position).toBe(cell.exit_rng[2])
    expect(random.normal).toBe(cell.exit_rng[3] === 0 ? null : cell.exit_rng[4])
    const outcome = result.value.outcome
    expect(outcome.nodes.map((node: number) => model.names[node])).toEqual(cell.names)
    const summary = outcome.summary
    for (let row = 0; row < 10; row++) for (let column = 0; column < cell.names.length; column++) expect(Math.abs(summary.replicates[row][column] - cell.replicates[row][column])).toBeLessThanOrEqual(1e-10)
    if (cell.cell === 24) expect(summary.replicates).toEqual(wasmCloud.samples)
    const centers = cell.cell === 24 ? sameInput.center : cell.summary
    for (let column = 0; column < cell.names.length; column++) {
      expect(Math.abs(summary.estimates[column] - centers[column])).toBeLessThanOrEqual(cell.cell === 24 ? 1e-7 : 3e-7)
      for (let bound = 0; bound < 2; bound++) expect(Math.abs(summary.bounds[column][bound] - cell.bounds[column][bound])).toBeLessThanOrEqual(1e-10)
    }
  }
})
