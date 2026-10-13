import { expect, test } from '@playwright/test'
import { openRouterStream } from './openRouterStream'

for (const [file, workspace] of [
  ['seatbelts', 'time-series'],
  ['breast-cancer-survival', 'survival'],
  ['crossing-survival-curves', 'survival'],
  ['feature-adoption-survival', 'survival'],
] as const) {
  test(`${file}: reads saved settings and evidence without substituting current data`, async ({
    page,
  }) => {
    await page.goto('/app')
    const result = await page.evaluate(
      async ({ file, workspace }) => {
        const { parseSnapshotValue } = await import(
          new URL('/src/domain/persistence.ts', location.href).href
        )
        const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
        const { assistantContext } = await import(
          new URL('/src/assistant/context.ts', location.href).href
        )
        const bundle = await (await fetch(`/examples/${file}.hirmos.json`)).json()
        const parsed = parseSnapshotValue(bundle.project)
        if (!parsed.ok) throw new Error(JSON.stringify(parsed.error))
        const workflow = { ...parsed.value, kind: 'profiled' }
        const run = (
          workspace === 'time-series' ? workflow.timeSeriesRuns : workflow.survivalRuns
        )[0]
        const before = JSON.stringify(workflow)
        const summary = readRun(workflow, { workspace, id: run.id })
        const specification = readRun(workflow, { workspace, id: run.id, view: 'specification' })
        const evidence = readRun(workflow, { workspace, id: run.id, view: 'evidence' })
        const { evidence: originalEvidence, ...originalSpecification } = run
        const overview = JSON.parse(assistantContext(workflow))
        const unchanged = before === JSON.stringify(workflow)
        workflow.prepared = { ...workflow.prepared, id: 'different-prepared-data' }
        return {
          summary,
          specification,
          evidence,
          originalEvidence,
          originalSpecification,
          unchanged,
          index:
            workspace === 'time-series' ? overview.savedTimeSeriesRuns : overview.savedSurvivalRuns,
          missing: readRun(workflow, { workspace, id: run.id, view: 'specification' }),
          wrongWorkspace: readRun(workflow, {
            workspace: workspace === 'time-series' ? 'survival' : 'time-series',
            id: run.id,
          }),
          unknown: readRun(workflow, { workspace, id: 'absent' }),
        }
      },
      { file, workspace },
    )
    expect(result.summary.kind).toBe('saved-run')
    expect(result.summary.workspace).toBe(workspace)
    expect(result.summary.provenance.graph).toBe('not-recorded')
    expect(result.summary.provenance.study).toBe('not-recorded')
    expect(result.index.some((r: { id: string }) => r.id === result.summary.run)).toBe(true)
    expect(result.specification.specification).toEqual(result.originalSpecification)
    expect(result.evidence.evidence).toEqual(result.originalEvidence)
    expect(result.missing.preparedDatasetRecord).toBeNull()
    expect(result.missing.provenance.preparedDataset).toBe('unavailable')
    expect(result.unknown.kind).toBe('refused')
    expect(result.wrongWorkspace.kind).toBe('refused')
    expect(result.unchanged).toBe(true)
  })
}

for (const [project, workspace, index] of [
  ['Seat-belt law and road deaths', 'time-series', 'savedTimeSeriesRuns'],
  ['Breast cancer survival, German study', 'survival', 'savedSurvivalRuns'],
] as const) {
  test(`${workspace}: assistant retrieves numerical evidence through the UI without a new run`, async ({
    page,
  }) => {
    const code = [
      'p = await tool("project")',
      `item = p["${index}"][0]`,
      `s = await tool("run", {"workspace":"${workspace}","id":item["id"],"view":"specification"})`,
      `e = await tool("run", {"workspace":"${workspace}","id":item["id"],"view":"evidence"})`,
      'assert s["kind"] == e["kind"] == "saved-run"',
      'assert s["run"] == e["run"] == item["id"]',
      'assert e["evidence"]',
      'await FINAL_VAR("Read saved " + e["workspace"] + " evidence without a new fit.", ["saved-run"])',
    ].join('\n')
    await page.route('https://openrouter.ai/api/v1/chat/completions', (route) =>
      route.fulfill(
        openRouterStream({ choices: [{ message: { content: '```python\n' + code + '\n```' } }] }),
      ),
    )
    await page.goto('/app/projects')
    await page.getByRole('button', { name: `Open ${project}`, exact: true }).click()
    await page.getByRole('button', { name: 'Open the assistant', exact: true }).click()
    await page.getByLabel('OpenRouter API key').fill('test-key')
    await page.getByLabel('OpenRouter model ID').fill('test/model')
    await page.getByRole('button', { name: 'Privacy', exact: true }).click()
    await page.getByRole('button', { name: 'I agree', exact: true }).click()
    await page.getByRole('button', { name: 'Use these settings', exact: true }).click()
    const snapshot = () =>
      page.evaluate(async () => {
        const { listProjects, loadProject } = await import(
          new URL('/src/data/projectStore.ts', location.href).href
        )
        return JSON.stringify(
          await Promise.all(
            (await listProjects()).map(async (header: { id: string }) => {
              const loaded = await loadProject(header.id)
              if (!loaded.ok) throw new Error('Cannot read project')
              return {
                time: loaded.value.timeSeriesRuns,
                survival: loaded.value.survivalRuns,
                estimation: loaded.value.estimationRuns,
              }
            }),
          ),
        )
      })
    const before = await snapshot()
    await page
      .getByLabel('Ask a question')
      .fill(`Read my saved ${workspace} result, without fitting again.`)
    await page.getByRole('button', { name: 'Send', exact: true }).click()
    await expect(page.getByRole('region', { name: 'Assistant answer' })).toContainText(
      `Read saved ${workspace} evidence without a new fit.`,
      { timeout: 45000 },
    )
    expect(await snapshot()).toBe(before)
  })
}
