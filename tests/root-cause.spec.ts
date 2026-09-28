import { expect, test } from '@playwright/test'

test('the DAG opens standalone root-cause analysis and saves its result', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'Desktop workflow also checks the mobile result layout.')
  test.setTimeout(120_000)
  await page.routeWebSocket(/.*/, socket => socket.close())
  const csv = { name: 'root-cause-baseline.csv', mimeType: 'text/csv', buffer: Buffer.from('X,Y\n' + Array.from({ length: 20 }, (_, index) => `${index + 1},${2 * (index + 1)}`).join('\n')) }
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Root-cause workflow')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type=file]').setInputFiles(csv)
  await page.getByRole('button', { name: 'Inspect data' }).click()
  await page.getByRole('radio', { name: 'Independent observations' }).click()
  await page.getByRole('checkbox', { name: 'X', exact: true }).check()
  await page.getByRole('checkbox', { name: 'Y', exact: true }).check()
  await page.getByRole('button', { name: 'Create prepared dataset version' }).click()
  await expect(page.getByRole('status').filter({ hasText: /^Cross-section, / })).toContainText('20 rows')
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Causal model analysis/ }).click()
  await expect(page.getByRole('region', { name: 'Root-cause graph selection' })).toContainText('Use for causal model analysis')
  await page.screenshot({ path: info.outputPath('root-cause-selection.png'), fullPage: true })
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /DAG workspace/ }).click()
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('X affects Y')
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  for (const [label, option] of [['Proposed cause', 'X'], ['Proposed effect', 'Y']]) {
    await page.getByRole('combobox', { name: label }).click()
    await page.getByRole('option', { name: option, exact: true }).click()
  }
  await page.getByRole('textbox', { name: /Rationale/ }).first().fill('Y is twice X in this verification dataset.')
  await page.getByRole('button', { name: 'Add the arrow' }).click()
  await page.getByRole('button', { name: 'Use for causal model analysis', exact: true }).click()
  await expect(page).toHaveURL(/\/app\/root-cause$/)
  await expect(page.getByRole('region', { name: 'Root-cause setup' })).toContainText('X affects Y')
  await expect(page.getByLabel('Refitted estimates', { exact: true })).toBeHidden()
  await expect(page.getByLabel('Observed X', { exact: true })).toBeVisible()
  expect(await page.getByTestId('root-cause-inputs').evaluate((element) => getComputedStyle(element).gridTemplateColumns.split(' ').length)).toBe(2)
  await page.setViewportSize({ width: 390, height: 844 })
  await expect(page.getByRole('region', { name: 'Root-cause setup' })).toBeVisible()
  expect(await page.getByTestId('root-cause-inputs').evaluate((element) => getComputedStyle(element).gridTemplateColumns.split(' ').length)).toBe(1)
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('root-cause-setup-mobile.png'), fullPage: true })
  await page.setViewportSize({ width: 1440, height: 1000 })
  await page.screenshot({ path: info.outputPath('root-cause-setup.png'), fullPage: true })
  const setup = page.getByRole('region', { name: 'Root-cause setup' })
  const assessment = page.getByRole('region', { name: 'Model assessment' })
  const idleHeight = await page.getByTestId('checks-actions').evaluate((element) => element.getBoundingClientRect().height)
  await assessment.getByRole('button', { name: 'Check fitted model', exact: true }).click()
  await expect(assessment.getByLabel('Model checks running')).toBeVisible()
  expect(await page.getByTestId('checks-actions').evaluate((element) => element.getBoundingClientRect().height)).toBe(idleHeight)
  const checkButton = await assessment.getByRole('button', { name: 'Check fitted model', exact: true }).boundingBox()
  const checkOrb = await assessment.getByLabel('Model checks running').boundingBox()
  expect(checkButton).not.toBeNull()
  expect(checkOrb).not.toBeNull()
  expect(Math.abs(checkButton!.y + checkButton!.height / 2 - checkOrb!.y - checkOrb!.height / 2)).toBeLessThan(2)
  expect(checkOrb!.x).toBeGreaterThan(checkButton!.x + checkButton!.width)
  await expect(setup.getByRole('status')).toHaveCount(0)
  await assessment.getByRole('button', { name: 'Cancel check', exact: true }).click()
  await expect(assessment.getByRole('alert')).toContainText('The model check was cancelled.')
  await expect(setup.getByRole('alert')).toHaveCount(0)
  await expect(assessment.getByRole('button', { name: 'Open DAG workspace', exact: true })).toHaveCount(0)
  await expect(setup.getByText('Selected graph', { exact: true })).toHaveCount(0)
  const graphButton = setup.getByRole('button', { name: /^Graph / })
  await graphButton.click()
  const graphDetails = page.getByRole('dialog', { name: /^Graph details:/ })
  await expect(graphDetails).toBeVisible()
  await expect(graphDetails.getByRole('region', { name: 'Graph relationships' })).toContainText('X')
  await page.screenshot({ path: info.outputPath('graph-details-desktop.png') })
  await page.keyboard.press('Escape')
  await expect(graphDetails).not.toBeVisible()
  await expect(graphButton).toBeFocused()
  await page.setViewportSize({ width: 390, height: 844 })
  await graphButton.click()
  const graphBounds = await graphDetails.boundingBox()
  expect(graphBounds!.x).toBeGreaterThanOrEqual(0)
  expect(graphBounds!.x + graphBounds!.width).toBeLessThanOrEqual(390)
  await page.screenshot({ path: info.outputPath('graph-details-mobile.png') })
  await page.keyboard.press('Escape')
  await page.setViewportSize({ width: 1440, height: 1000 })
  await graphButton.click()
  await graphDetails.getByRole('button', { name: 'Open DAG workspace', exact: true }).click()
  await expect(page.getByRole('heading', { name: 'Graph checks', exact: true })).toBeVisible()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Causal model analysis/ }).click()
  await page.getByText('Baseline data relationships', { exact: true }).click()
  await page.getByRole('button', { name: 'Explore baseline data', exact: true }).click()
  await expect(page.getByTestId('root-cause-scatter-matrix')).toBeVisible()
  const plotVariables = page.getByRole('group', { name: 'Variables to plot' })
  await plotVariables.getByRole('button', { name: 'Clear selected variables' }).click()
  await expect(plotVariables.getByRole('checkbox', { checked: true })).toHaveCount(0)
  await expect(page.getByTestId('root-cause-scatter-matrix')).toHaveCount(0)
  await plotVariables.getByRole('checkbox', { name: 'X', exact: true }).check()
  await expect(plotVariables.getByRole('checkbox', { checked: true })).toHaveCount(1)
  await expect(page.getByTestId('root-cause-scatter-matrix')).toBeVisible()
  await plotVariables.getByRole('button', { name: 'Select all variables' }).click()
  await expect(plotVariables.getByRole('checkbox', { checked: true })).toHaveCount(2)
  await plotVariables.screenshot({ path: info.outputPath('plot-selection-desktop.png') })
  const openScatter = page.getByRole('button', { name: 'Open Baseline scatter matrix in a floating window' })
  await openScatter.click()
  const floatingScatter = page.getByRole('dialog', { name: 'Baseline scatter matrix', exact: true })
  await expect(floatingScatter.getByRole('img', { name: 'Baseline scatter matrix', exact: true })).toBeVisible()
  const beforeResize = await floatingScatter.boundingBox()
  await page.mouse.move(beforeResize!.x + beforeResize!.width - 2, beforeResize!.y + beforeResize!.height - 2)
  await page.mouse.down()
  await page.mouse.move(beforeResize!.x + beforeResize!.width - 122, beforeResize!.y + beforeResize!.height - 82, { steps: 8 })
  await page.mouse.up()
  await expect.poll(async () => (await floatingScatter.boundingBox())!.width).toBeLessThan(beforeResize!.width - 50)
  const scatterDownload = page.waitForEvent('download')
  await floatingScatter.getByRole('button', { name: 'Export Baseline scatter matrix as SVG', exact: true }).click()
  expect((await scatterDownload).suggestedFilename()).toMatch(/\.svg$/)
  await floatingScatter.screenshot({ path: info.outputPath('scatter-expanded-desktop.png') })
  await page.keyboard.press('Escape')
  await expect(floatingScatter).toHaveCount(0)
  await openScatter.click()
  await floatingScatter.getByRole('button', { name: 'Close the floating window' }).click()
  await expect(floatingScatter).toHaveCount(0)
  await page.setViewportSize({ width: 390, height: 844 })
  await page.getByText('Baseline data relationships', { exact: true }).click()
  await page.getByRole('button', { name: 'Explore baseline data', exact: true }).click()
  await expect(page.getByTestId('root-cause-scatter-matrix')).toBeVisible()
  await plotVariables.getByRole('button', { name: 'Clear selected variables' }).click()
  await expect(plotVariables.getByRole('checkbox', { checked: true })).toHaveCount(0)
  await plotVariables.getByRole('button', { name: 'Select all variables' }).click()
  await expect(plotVariables.getByRole('checkbox', { checked: true })).toHaveCount(2)
  expect(await plotVariables.evaluate((element) => element.scrollWidth <= element.clientWidth)).toBe(true)
  await plotVariables.screenshot({ path: info.outputPath('plot-selection-mobile.png') })
  await openScatter.click()
  await expect(floatingScatter.getByRole('img', { name: 'Baseline scatter matrix', exact: true })).toBeVisible()
  const mobileScatter = await floatingScatter.boundingBox()
  expect(mobileScatter!.x).toBeGreaterThanOrEqual(0)
  expect(mobileScatter!.x + mobileScatter!.width).toBeLessThanOrEqual(390)
  await floatingScatter.screenshot({ path: info.outputPath('scatter-expanded-mobile.png') })
  await floatingScatter.getByRole('button', { name: 'Close the floating window' }).click()
  await expect(floatingScatter).toHaveCount(0)
  await page.setViewportSize({ width: 1440, height: 1000 })
  await page.getByRole('combobox', { name: 'Target variable' }).click()
  await page.getByRole('option', { name: 'Y', exact: true }).click()
  await page.getByLabel('Observed X', { exact: true }).fill('7')
  await page.getByLabel('Observed Y', { exact: true }).fill('23')
  await page.getByRole('radio', { name: 'Upload file', exact: true }).check()
  await page.getByLabel('Unusual observation file', { exact: true }).setInputFiles({ name: 'one-row.csv', mimeType: 'text/csv', buffer: Buffer.from('X,Y\n7,23\n') })
  await page.getByRole('radio', { name: 'Enter values', exact: true }).check()
  await expect(page.getByLabel('Observed X', { exact: true })).toHaveValue('7')
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByLabel('Refitted estimates', { exact: true }).fill('2')
  await page.getByLabel('Distribution samples', { exact: true }).fill('20')
  await page.getByRole('checkbox', { name: /These values use/ }).check()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Expand Run history (1)', exact: true })).toBeVisible({ timeout: 60_000 })
  await expect(page.getByRole('list', { name: 'Root-cause runs', exact: true })).toHaveCount(0)
  await page.getByRole('button', { name: 'Expand Run history (1)', exact: true }).click()
  await expect(page.getByRole('button', { name: /Unusual observation.*entered-observation.csv/ })).toBeVisible({ timeout: 60_000 })
  await page.getByRole('radio', { name: 'Upload file', exact: true }).check()
  await page.getByRole('checkbox', { name: /same variable definitions/ }).check()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(page.getByRole('button', { name: /Unusual observation.*one-row.csv/ })).toBeVisible({ timeout: 60_000 })
  await expect.poll(() => page.evaluate(async () => {
    const { listProjects, loadProject } = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await listProjects()).find((entry: { name: string }) => entry.name === 'Root-cause workflow')
    if (!header) return false
    const saved = await loadProject(header.id)
    if (!saved.ok || saved.value.rootCause.runs.length !== 2) return false
    const [manual, uploaded] = saved.value.rootCause.runs
    return JSON.stringify(manual.evidence) === JSON.stringify(uploaded.evidence) && JSON.stringify(manual.observation) === '[7,23]'
  })).toBe(true)
  await page.getByRole('button', { name: /^Delete .* run$/ }).first().click()
  await page.getByRole('button', { name: /^Delete .* run$/ }).first().click()
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByRole('radio', { name: 'Shift intervention', exact: true }).check()
  await page.getByRole('combobox', { name: 'Target variable' }).click()
  await page.getByRole('option', { name: 'Y', exact: true }).click()
  await page.getByLabel('Intervention model data', { exact: true }).setInputFiles(csv)
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByRole('button', { name: 'About Refitted estimates', exact: true }).focus()
  await expect(page.getByRole('tooltip')).not.toBeEmpty()
  await page.keyboard.press('Escape')
  await expect(page.getByRole('tooltip')).toBeHidden()
  await page.getByLabel('Refitted estimates', { exact: true }).fill('3')
  await page.getByLabel('Shift in X', { exact: true }).fill('-1')
  await page.getByRole('checkbox', { name: /same variable definitions/ }).check()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toContainText('estimated mean after the specified shifts is 19', { timeout: 60_000 })
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export analysis record' }).click()
  const exported = await download
  const exportedPath = info.outputPath('analysis-record.json')
  await exported.saveAs(exportedPath)
  await page.getByText('Reuse saved settings', { exact: true }).click()
  await page.getByLabel('Saved analysis settings', { exact: true }).setInputFiles(exportedPath)
  await expect(page.getByText(/Loaded shift-intervention settings for Y/)).toBeVisible()
  await expect(page.getByLabel('Shift in X', { exact: true })).toBeHidden()
  await page.getByRole('button', { name: 'Use editable settings', exact: true }).click()
  await page.getByText('Reuse saved settings', { exact: true }).click()
  await page.getByTestId('root-cause-bars').scrollIntoViewIfNeeded()
  await page.screenshot({ path: info.outputPath('root-cause-desktop.png'), fullPage: true })
  await page.setViewportSize({ width: 390, height: 844 })
  await page.getByTestId('root-cause-bars').scrollIntoViewIfNeeded()
  await expect(page.getByTestId('root-cause-bars')).toBeVisible()
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true)
  await page.screenshot({ path: info.outputPath('root-cause-mobile.png'), fullPage: true })
  await page.setViewportSize({ width: 1280, height: 900 })
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Results/ }).click()
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toContainText('estimated mean after the specified shifts is 19')
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Causal model analysis/ }).click()
  await page.getByRole('button', { name: 'Expand Run history (1)', exact: true }).click()
  await expect(page.getByRole('button', { name: /Shift intervention.*root-cause-baseline.csv/ })).toBeVisible()
  await expect.poll(() => page.evaluate(async () => {
    const { listProjects, loadProject } = await import(new URL('/src/data/projectStore.ts', location.href).href)
    const header = (await listProjects()).find((entry: { name: string }) => entry.name === 'Root-cause workflow')
    if (header === undefined) return false
    const snapshot = await loadProject(header.id)
    return snapshot.ok && snapshot.value.rootCause.runs.length === 1 && snapshot.value.studyDraft.dagDocument === null
  })).toBe(true)
  await page.reload()
  await page.getByRole('button', { name: 'Open Root-cause workflow', exact: true }).click()
  await page.getByRole('heading', { name: 'Choose the data file again' }).waitFor()
  await page.locator('input[type=file]').setInputFiles(csv)
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Causal model analysis/ }).click()
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toContainText('estimated mean after the specified shifts is 19', { timeout: 30_000 })
  await page.getByRole('button', { name: 'Expand Run history (1)', exact: true }).click()
  await page.getByRole('button', { name: /^Delete .* run$/ }).click()
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toHaveCount(0)
  await page.getByLabel('Intervention model data', { exact: true }).setInputFiles(csv)
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByLabel('Refitted estimates', { exact: true }).fill('100000')
  await page.getByRole('checkbox', { name: /same variable definitions/ }).check()
  await page.getByRole('button', { name: 'Run analysis', exact: true }).click()
  await expect(setup.getByLabel('Causal model analysis running')).toBeVisible()
  const runButton = await setup.getByRole('button', { name: 'Run analysis', exact: true }).boundingBox()
  const runOrb = await setup.getByLabel('Causal model analysis running').boundingBox()
  expect(runButton).not.toBeNull()
  expect(runOrb).not.toBeNull()
  expect(Math.abs(runButton!.y + runButton!.height / 2 - runOrb!.y - runOrb!.height / 2)).toBeLessThan(2)
  expect(runOrb!.x).toBeGreaterThan(runButton!.x + runButton!.width)
  await page.getByTestId('analysis-actions').screenshot({ path: info.outputPath('analysis-actions-running.png') })
  await expect(assessment.getByRole('status')).toHaveCount(0)
  await page.getByRole('button', { name: 'Cancel run', exact: true }).click()
  await expect(setup.getByRole('alert')).toContainText('cancelled')
  await expect(assessment.getByRole('alert')).toHaveCount(0)
  await expect(page.getByRole('region', { name: 'Root-cause result' })).toHaveCount(0)
})

test('model checks return separate root and conditional diagnostics through WASM', async ({ page }) => {
  test.setTimeout(120_000)
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { checkRootCause } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const x = Array.from({ length: 60 }, (_, index) => 1 + Math.abs(Math.sin(index * 1.31)))
    const y = x.map((value, index) => 2 * value + 0.3 * Math.cos(index * 2.73))
    return checkRootCause(new Float64Array([...x, ...y]), { names: ['X', 'Y'], edges: [[0, 1]], rows: 60, seed: 0 })
  })
  expect(result.ok, JSON.stringify(result)).toBe(true)
  if (!result.ok) return
  expect(result.value.mechanisms.map((entry: { kind: string }) => entry.kind)).toEqual(['root', 'conditional'])
  expect(result.value.invertibility).toHaveLength(1)
  expect(result.value.verdict).toBe('noImplications')
  expect(result.value.random.keys).toHaveLength(624)
  expect(result.value.permutations).toHaveLength(2)
  const fitted = await page.evaluate(async () => {
    const { checkRootCause } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const x = Array.from({ length: 60 }, (_, index) => 1 + Math.abs(Math.sin(index * 1.31)))
    const y = x.map((value, index) => 2 * value + 0.3 * Math.cos(index * 2.73))
    const stages: string[] = []
    const result = await checkRootCause(new Float64Array([...x, ...y]), { names: ['X', 'Y'], edges: [[0, 1]], rows: 60, seed: 0, scope: 'fitted' }, (progress: { stage: string }) => stages.push(progress.stage))
    return { result, stages }
  })
  expect(fitted.result.ok, JSON.stringify(fitted.result)).toBe(true)
  if (!fitted.result.ok) return
  expect(fitted.result.value.scope).toBe('fitted')
  expect(fitted.result.value).not.toHaveProperty('verdict')
  expect(fitted.result.value).not.toHaveProperty('permutations')
  expect(fitted.stages).not.toContain('Graph checks')
  expect(fitted.stages).toContain('Fitted-model checks complete')
  expect(fitted.result.value.mechanisms).toEqual(result.value.mechanisms)
  expect(fitted.result.value.invertibility).toEqual(result.value.invertibility)
  expect(fitted.result.value.overallDivergence).toBe(result.value.overallDivergence)
  await page.evaluate(async (evidence) => {
    const { React, createRoot } = await import(new URL('/tests/support/reactRuntime.ts', location.href).href)
    const { RootCauseChecks } = await import(new URL('/src/components/root-cause/RootCauseChecks.tsx', location.href).href)
    const host = document.createElement('div')
    document.body.replaceChildren(host)
    createRoot(host).render(React.createElement(RootCauseChecks, { record: { model: { names: ['X', 'Y'] }, evidence } }))
  }, fitted.result.value)
  await expect(page.getByRole('region', { name: 'Fitted-model diagnostics' })).toBeVisible()
  await expect(page.getByRole('heading', { name: 'Prediction performance' })).toBeVisible()
  const help = page.getByRole('button', { name: 'About Prediction performance', exact: true })
  const coarse = await page.evaluate(() => matchMedia('(pointer: coarse)').matches)
  if (coarse) await help.tap()
  else await help.focus()
  const note = page.getByRole(coarse ? 'dialog' : 'tooltip')
  await expect(note).not.toBeEmpty()
  await page.keyboard.press('Escape')
  await expect(note).toBeHidden()
  const performance = page.getByRole('table', { name: 'Prediction performance' })
  await expect(performance).toBeVisible()
  await expect(performance.getByRole('columnheader')).toHaveCount(6)
  await expect(performance.getByRole('columnheader', { name: /Normalised RMSE/ })).toBeVisible()
  await expect(performance.getByRole('row').filter({ hasText: 'X' })).toContainText('—')
  await expect(performance.getByRole('row').filter({ hasText: 'Y' })).toContainText('—')
  await expect(page.getByRole('heading', { name: 'Noise independence' })).toBeVisible()
  const noise = page.getByRole('table', { name: 'Noise independence' })
  await expect(noise.getByRole('columnheader')).toHaveCount(3)
  await expect(noise.getByRole('columnheader', { name: 'p-value' })).toBeVisible()
  await expect(noise.getByRole('cell', { name: /^(Not rejected|Rejected)$/ }).first()).toBeVisible()
  await expect(page.getByTestId('root-cause-graph-checks')).toHaveCount(0)
  await expect(page.getByText('This graph has no testable conditional-independence implications.')).toHaveCount(0)
})

test('RCA bars retain signed values and source-compatible uncertainty visibility', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { rootCauseOption } = await import(new URL('/src/charts/rootCause.ts', location.href).href)
    const { readChartTheme } = await import(new URL('/src/charts/theme.ts', location.href).href)
    const run = { model: { names: ['A', 'B'], target: 1 }, evidence: { outcome: { kind: 'anomaly', nodes: [0, 1], summary: { estimates: [-2, 4], bounds: [[-3, -1], [1, 2]] } } } }
    const option = rootCauseOption(run, readChartTheme())
    return { bars: option.series[0].data.map((entry: { value: number }) => entry.value), intervals: option.series.slice(1).map((entry: { data: number[][] }) => entry.data), axis: option.xAxis.type }
  })
  expect(result.bars).toEqual([-2, 4])
  expect(result.intervals).toEqual([[[-3, 0], [-1, 0]]])
  expect(result.axis).toBe('value')
})

test('the scatter matrix retains all observations and diagonal histogram counts', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { scatterMatrixOption } = await import(new URL('/src/charts/data/scatterMatrix.ts', location.href).href)
    const { readChartTheme } = await import(new URL('/src/charts/theme.ts', location.href).href)
    const columns = ['A', 'B'].map((name, column) => ({ name, values: Array.from({ length: 3001 }, (_, row) => row + column) }))
    const option = scatterMatrixOption(columns, readChartTheme())
    return { points: option.series[1].data.length, histograms: [option.series[0], option.series[3]].map((series: { data: number[][] }) => series.data.reduce((sum, point) => sum + point[1], 0)) }
  })
  expect(result).toEqual({ points: 3001, histograms: [3001, 3001] })
})

test('root-cause navigation preserves the treatment-effect chapter sequence', async ({ page }) => {
  await page.goto('/app')
  const chapters = await page.evaluate(async () => {
    const { CHAPTER_IDS, parseRoute } = await import(new URL('/src/domain/navigation.ts', location.href).href)
    return { ids: CHAPTER_IDS, route: parseRoute('/app/root-cause', '') }
  })
  const start = chapters.ids.indexOf('dag')
  expect(chapters.ids.slice(start, start + 5)).toEqual(['dag', 'study', 'estimation', 'sensitivity', 'counterfactual'])
  expect(chapters.ids.indexOf('root-cause')).toBe(chapters.ids.indexOf('survival') - 1)
  expect(chapters.route).toEqual({ ok: true, value: { kind: 'chapter', chapter: 'root-cause' } })
})

test('the root-cause worker preserves signed shifts, percentiles and resumable random state', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { runRootCause } = await import(new URL('/src/analysis/client.ts', location.href).href)
    const { rootCauseRequestSchema } = await import(new URL('/src/domain/rootCauseAnalysis.ts', location.href).href)
    const rows = 20
    const x = Array.from({ length: rows }, (_, index) => index + 1)
    const y = x.map((value) => 2 * value)
    const model = rootCauseRequestSchema.parse({
      names: ['X', 'Y'], edges: [[0, 1]], rows, target: 1, repetitions: 3,
      upperQuantile: 0.95, fraction: 0.75, random: { kind: 'seed', seed: 11 },
      query: { kind: 'intervention', rows, order: [0, 1], shifts: [{ node: 0, amount: -1 }] },
    })
    const values = () => new Float64Array([...x, ...y, ...x, ...y])
    const first = await runRootCause(values(), model)
    const second = await runRootCause(values(), model)
    return { first, second }
  })
  expect(result.first.ok).toBe(true)
  expect(result.second).toEqual(result.first)
  if (!result.first.ok) throw new Error(JSON.stringify(result.first))
  const { outcome, random } = result.first.value
  expect(outcome.kind).toBe('intervention')
  expect(outcome.observedMean).toBe(21)
  expect(outcome.nodes).toEqual([0, 1])
  expect(outcome.summary.estimates[1]).toBeCloseTo(19, 7)
  expect(outcome.summary.quantiles[0]).toBeCloseTo(0.05, 12)
  expect(outcome.summary.quantiles[1]).toBe(0.95)
  expect(outcome.summary.replicates).toHaveLength(3)
  expect(random.keys).toHaveLength(624)
})

test('root-cause graph preparation validates structure and preserves column order', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const { prepareRootCauseGraph } = await import(new URL('/src/domain/rootCause.ts', location.href).href)
    const nodes = [
      { kind: 'observed', id: 'b', column: '1:B', name: 'B' },
      { kind: 'observed', id: 'a', column: '0:A', name: 'A' },
    ]
    const edge = {
      kind: 'directed', id: 'a-b', cause: 'a', effect: 'b',
      timing: { kind: 'contemporaneous' },
      support: { kind: 'user-assumption', rationale: 'A affects B.' }, evidence: [],
    }
    const graph = { kind: 'editable-dag', nodes, edges: [edge] }
    const document = {
      id: 'document', preparedDataset: 'prepared', dataset: { kind: 'cross-section', observations: 100 },
      current: { id: 'revision', graph, validation: { structure: { kind: 'sound' }, rationales: { kind: 'complete' } } },
    }
    const prepared = { id: 'prepared', columns: ['0:A', '1:B'] }
    const check = (changes: Record<string, unknown>) => prepareRootCauseGraph({
      ...document, current: { ...document.current, graph: { ...graph, ...changes } },
    }, prepared)
    return {
      valid: check({}),
      roots: check({ edges: [] }),
      stale: prepareRootCauseGraph(document, { ...prepared, id: 'new-preparation' }),
      missing: prepareRootCauseGraph(document, { ...prepared, columns: ['0:A'] }),
      latent: check({ nodes: [nodes[0], { kind: 'latent', id: 'a', name: 'A' }] }),
      repeatedColumn: check({ nodes: [nodes[0], { ...nodes[1], column: '1:B' }] }),
      repeatedName: check({ nodes: [nodes[0], { ...nodes[1], name: 'B' }] }),
      repeatedRoot: check({ nodes: [nodes[0], nodes[0]], edges: [] }),
      empty: check({ nodes: [], edges: [] }),
      unstated: check({ edges: [{ ...edge, support: { kind: 'unstated' } }] }),
      cycle: check({ edges: [edge, { ...edge, id: 'b-a', cause: 'b', effect: 'a' }] }),
      unknown: check({ edges: [{ ...edge, cause: 'absent' }] }),
      lagged: prepareRootCauseGraph({ ...document, dataset: { kind: 'time-series', observations: 100 },
        current: { ...document.current, graph: { ...graph, edges: [{ ...edge, timing: { kind: 'lagged', lag: 1 } }] } },
      }, prepared),
    }
  })
  expect(result.valid).toMatchObject({ ok: true, value: {
    dagDocument: 'document', dagRevision: 'revision', preparedDataset: 'prepared', edges: [[1, 0]],
    nodes: [{ name: 'B' }, { name: 'A' }],
  } })
  expect(result.roots).toMatchObject({ ok: true, value: { edges: [] } })
  expect(result.unstated).toMatchObject({ ok: true, value: { edges: [[1, 0]] } })
  for (const [name, kind] of Object.entries({
    stale: 'different-preparation', missing: 'missing-column', latent: 'unmeasured-variable',
    repeatedColumn: 'duplicate-column', repeatedName: 'duplicate-name', repeatedRoot: 'invalid-graph',
    empty: 'invalid-graph', cycle: 'invalid-graph', unknown: 'invalid-graph',
    lagged: 'lagged-relationship',
  })) {
    expect(result[name as keyof typeof result], name).toMatchObject({ ok: false, error: { kind } })
  }
})
