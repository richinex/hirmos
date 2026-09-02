import { expect, test, type Locator } from '@playwright/test'

const choose = async (trigger: Locator, label: string) => {
  await trigger.click()
  await trigger.page().getByRole('listbox').getByRole('option', { name: label, exact: true }).click()
}

test('aggregates daily values by calendar week with per-column rules', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Pure resampling contract runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const timestamps = Float64Array.from({ length: 14 }, (_, i) => Date.UTC(2026, 0, i + 1))
    const values = Float64Array.from([
      ...Array.from({ length: 14 }, (_, i) => i + 1),
      ...Array.from({ length: 14 }, (_, i) => 2 * (i + 1)),
    ])
    const kept = await analysis.runPandasResampling(timestamps, values, 14, 2, 'weekly', 'keep', ['sum', 'mean'], [[5, 0]])
    const dropped = await analysis.runPandasResampling(timestamps, values, 14, 2, 'weekly', 'drop', ['sum', 'mean'], [[5, 0]])
    return {
      kept: kept.ok ? {
        values: Array.from(kept.value.values),
        times: Array.from(kept.value.timestampsMs),
        imputed: kept.value.imputedCells,
        record: kept.value,
      } : kept,
      dropped: dropped.ok ? {
        values: Array.from(dropped.value.values),
        times: Array.from(dropped.value.timestampsMs),
        record: dropped.value,
      } : dropped,
    }
  })

  expect(result.kept).toEqual({
    values: [10, 56, 39, 5, 16, 26],
    times: [Date.UTC(2025, 11, 29), Date.UTC(2026, 0, 5), Date.UTC(2026, 0, 12)],
    imputed: [[1, 0]],
    record: expect.objectContaining({ kind: 'pandasResampled', sourceRows: 14, outputRows: 3, incompleteBins: 2, binsDropped: 0, sourceRowsDropped: 0 }),
  })
  expect(result.dropped).toEqual({
    values: [56, 16],
    times: [Date.UTC(2026, 0, 5)],
    record: expect.objectContaining({ kind: 'pandasResampled', sourceRows: 14, outputRows: 1, incompleteBins: 2, binsDropped: 2, sourceRowsDropped: 7 }),
  })
})

test('implements every recorded aggregation and leap-year completeness', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Pure aggregation contract runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const analysis = await import(new URL('/src/analysis/client.ts', window.location.href).href)
    const rules = ['mean', 'sum', 'median', 'minimum', 'maximum', 'first', 'last']
    const week = [7, 1, 5, 3, 9, 2, 4]
    const weekly = await analysis.runPandasResampling(
      Float64Array.from({ length: 7 }, (_, i) => Date.UTC(2026, 0, i + 5)),
      Float64Array.from(rules.flatMap(() => week)),
      7, 7, 'weekly', 'drop', rules, [],
    )

    const leapDays = Array.from({ length: 30 }, (_, i) => Date.UTC(2024, i < 29 ? 1 : 2, i < 29 ? i + 1 : 1))
    const monthly = await analysis.runPandasResampling(
      Float64Array.from(leapDays), Float64Array.from({ length: 30 }, (_, i) => i + 1),
      30, 1, 'monthly', 'drop', ['sum'], [],
    )
    return {
      weekly: weekly.ok ? Array.from(weekly.value.values) : weekly,
      monthly: monthly.ok ? { values: Array.from(monthly.value.values), record: monthly.value } : monthly,
    }
  })

  expect(result.weekly).toEqual([31 / 7, 31, 4, 1, 9, 7, 4])
  expect(result.monthly).toMatchObject({
    values: [435],
    record: { outputRows: 1, incompleteBins: 1, binsDropped: 1, sourceRowsDropped: 1 },
  })
})

test('sorts calendar rows in DuckDB and materializes the saved weekly version', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Data-worker resampling boundary runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const [data, workflow, preparedModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/data/prepared.ts', window.location.href).href),
    ])
    const rows = Array.from({ length: 14 }, (_, i) => ({ date: new Date(Date.UTC(2026, 0, i + 1)).toISOString().slice(0, 10), x: i + 1 })).reverse()
    const file = new File([`date,x\n${rows.map((row) => `${row.date},${row.x}`).join('\n')}\n`], 'daily.csv', { type: 'text/csv' })
    const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!profile.ok) throw new Error(`Profile failed: ${profile.error.kind}${'detail' in profile.error ? `: ${profile.error.detail}` : ''}`)
    const date = profile.value.columns.find((column: { readonly name: string }) => column.name === 'date')
    const x = profile.value.columns.find((column: { readonly name: string }) => column.name === 'x')
    if (date === undefined || x === undefined) throw new Error('Fixture columns were not profiled.')

    const ordered = await data.materializeTimeSeriesColumnsInWorker(file, profile.value, date.id, [x.id])
    if (!ordered.ok) throw new Error(`Time materialization failed: ${ordered.error.kind}`)
    const source = { file, name: file.name, bytes: file.size, mediaType: file.type, lastModified: file.lastModified, format: 'csv' }
    const prepared = {
      kind: 'prepared-time-series', id: 'prepared-weekly', recipe: 'recipe-weekly', sourceProfile: profile.value.id,
      observations: 3, columns: [x.id],
      sampling: { kind: 'regular-series', timeColumn: date.id, frequency: 'weekly' },
      missingness: { kind: 'not-present' }, resolution: { kind: 'none' },
      resampling: {
        kind: 'daily-downsample', sourceFrequency: 'daily', targetFrequency: 'weekly', weekStartsOn: 'monday', calendar: 'utc', incompleteBins: 'keep',
        aggregations: [{ column: x.id, aggregation: 'sum' }], sourceRows: 14, outputRows: 3, incompleteBinsFound: 2, binsDropped: 0, sourceRowsDropped: 0,
      },
      seasonalAdjustment: { kind: 'none' },
      seriesTransforms: [{ column: x.id, transform: { kind: 'levels' } }],
    }
    const materialized = await preparedModule.materialisePrepared(source, profile.value, prepared, [x.id])
    if (!materialized.ok) throw new Error(`Prepared materialization failed: ${materialized.error.kind}`)
    return {
      timeKind: ordered.value.timeAxis.kind,
      sortedValues: Array.from(ordered.value.values),
      preparedValues: Array.from(materialized.value.values),
      preparedRows: materialized.value.rowCount,
      preparedTimes: materialized.value.timeAxis?.kind === 'calendar' ? Array.from(materialized.value.timeAxis.timestamps) : null,
    }
  })

  expect(result).toEqual({
    timeKind: 'calendar',
    sortedValues: Array.from({ length: 14 }, (_, i) => i + 1),
    preparedValues: [10, 56, 39],
    preparedRows: 3,
    preparedTimes: [Date.UTC(2025, 11, 29), Date.UTC(2026, 0, 5), Date.UTC(2026, 0, 12)],
  })
})

test('keeps ordinal time keys usable but does not treat them as calendar dates', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Ordinal time boundary runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const [data, workflow] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
    ])
    const file = new File(['time,x\n3,30\n1,10\n2,20\n'], 'ordinal.csv', { type: 'text/csv' })
    const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!profile.ok) throw new Error(`Profile failed: ${profile.error.kind}`)
    const time = profile.value.columns.find((column: { readonly name: string }) => column.name === 'time')
    const x = profile.value.columns.find((column: { readonly name: string }) => column.name === 'x')
    if (time === undefined || x === undefined) throw new Error('Fixture columns were not profiled.')
    const ordered = await data.materializeTimeSeriesColumnsInWorker(file, profile.value, time.id, [x.id])
    if (!ordered.ok) throw new Error(`Time materialization failed: ${ordered.error.kind}`)
    return {
      axis: ordered.value.timeAxis.kind,
      times: ordered.value.timeAxis.kind === 'ordinal' ? Array.from(ordered.value.timeAxis.values) : [],
      values: Array.from(ordered.value.values),
    }
  })

  expect(result).toEqual({ axis: 'ordinal', times: [1, 2, 3], values: [10, 20, 30] })
})

test('resolves missing values before aggregation and carries imputation evidence', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Preparation-order contract runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const [data, workflow, preparedModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/data/prepared.ts', window.location.href).href),
    ])
    const file = new File(['date,x\n2026-01-05,1\n2026-01-06,2\n2026-01-07,\n2026-01-08,4\n2026-01-09,5\n2026-01-10,6\n2026-01-11,7\n'], 'missing-daily.csv', { type: 'text/csv' })
    const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!profile.ok) throw new Error(`Profile failed: ${profile.error.kind}`)
    const date = profile.value.columns.find((column: { readonly name: string }) => column.name === 'date')
    const x = profile.value.columns.find((column: { readonly name: string }) => column.name === 'x')
    if (date === undefined || x === undefined) throw new Error('Fixture columns were not profiled.')
    const source = { file, name: file.name, bytes: file.size, mediaType: file.type, lastModified: file.lastModified, format: 'csv' }
    const prepared = {
      kind: 'prepared-time-series', id: 'prepared-imputed-weekly', recipe: 'recipe-imputed-weekly', sourceProfile: profile.value.id,
      observations: 1, columns: [x.id], sampling: { kind: 'regular-series', timeColumn: date.id, frequency: 'weekly' },
      missingness: { kind: 'imputation', cells: 1, method: 'forwardFill', maxGap: 1, confirmedStructuralZero: false },
      resolution: { kind: 'imputed', method: 'forwardFill', maxGap: 1, cells: 1 },
      resampling: {
        kind: 'daily-downsample', sourceFrequency: 'daily', targetFrequency: 'weekly', weekStartsOn: 'monday', calendar: 'utc', incompleteBins: 'drop',
        aggregations: [{ column: x.id, aggregation: 'sum' }], sourceRows: 7, outputRows: 1, incompleteBinsFound: 0, binsDropped: 0, sourceRowsDropped: 0,
      },
      seasonalAdjustment: { kind: 'none' }, seriesTransforms: [{ column: x.id, transform: { kind: 'levels' } }],
    }
    const materialized = await preparedModule.materialisePrepared(source, profile.value, prepared, [x.id])
    if (!materialized.ok) throw new Error(`Prepared materialization failed: ${materialized.error.kind}`)
    return { values: Array.from(materialized.value.values), imputed: materialized.value.imputedCells }
  })

  expect(result).toEqual({ values: [27], imputed: [[0, 0]] })
})

test('creates a weekly prepared version through the Data studio controls', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Resampling UI workflow runs once')
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Resampling UI test')
  await page.getByRole('button', { name: 'Create project' }).click()
  const rows = Array.from({ length: 14 }, (_, i) => `${new Date(Date.UTC(2026, 0, i + 1)).toISOString().slice(0, 10)},${i + 1}`).join('\n')
  await page.locator('input[type="file"]').setInputFiles({ name: 'daily.csv', mimeType: 'text/csv', buffer: Buffer.from(`date,x\n${rows}\n`) })
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'date')
  await choose(page.getByLabel('Source frequency'), 'Daily')
  await page.getByRole('checkbox', { name: 'x', exact: true }).check()
  await page.getByRole('radio', { name: 'Weekly', exact: true }).click()
  await expect(page.getByRole('button', { name: /Create prepared/ })).toHaveCount(0)
  await expect(page.getByText(/Choose how to aggregate the selected column/)).toBeVisible()
  await choose(page.getByLabel('Aggregation for x'), 'Sum')
  await page.getByRole('button', { name: /Create prepared/ }).click()

  await expect(page.getByText(/Prepared time series · 3 rows/)).toBeVisible({ timeout: 30_000 })
  await expect(page.getByText(/Daily to weekly, x: sum/)).toBeVisible()
})

test('keeps the saved complete interval when a caller requests one column', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'chromium', 'Canonical prepared-grid contract runs once')
  await page.goto('/app')

  const result = await page.evaluate(async () => {
    const [data, workflow, preparedModule] = await Promise.all([
      import(new URL('/src/data/client.ts', window.location.href).href),
      import(new URL('/src/domain/workflow.ts', window.location.href).href),
      import(new URL('/src/data/prepared.ts', window.location.href).href),
    ])
    const file = new File(['time,x,y\n1,,10\n2,2,20\n3,3,30\n4,4,40\n'], 'window.csv', { type: 'text/csv' })
    const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!profile.ok) throw new Error(`Profile failed: ${profile.error.kind}`)
    const column = (name: string) => {
      const found = profile.value.columns.find((candidate: { readonly name: string }) => candidate.name === name)
      if (found === undefined) throw new Error(`${name} was not profiled.`)
      return found.id
    }
    const time = column('time')
    const x = column('x')
    const y = column('y')
    const source = { file, name: file.name, bytes: file.size, mediaType: file.type, lastModified: file.lastModified, format: 'csv' }
    const prepared = {
      kind: 'prepared-time-series', id: 'prepared-window', recipe: 'recipe-window', sourceProfile: profile.value.id,
      observations: 3, columns: [x, y], sampling: { kind: 'regular-series', timeColumn: time, frequency: 'daily' },
      missingness: { kind: 'complete-interval', cells: 1 }, resolution: { kind: 'window', start: 1, endExclusive: 4, sourceRows: 4 },
      resampling: { kind: 'none' }, seasonalAdjustment: { kind: 'none' },
      seriesTransforms: [{ column: x, transform: { kind: 'levels' } }, { column: y, transform: { kind: 'levels' } }],
    }
    const yOnly = await preparedModule.materialisePrepared(source, profile.value, prepared, [y])
    if (!yOnly.ok) throw new Error(`Prepared materialization failed: ${yOnly.error.kind}`)
    return { rows: yOnly.value.rowCount, values: Array.from(yOnly.value.values) }
  })

  expect(result).toEqual({ rows: 3, values: [20, 30, 40] })
})
