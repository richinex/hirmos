import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { chapter, choose } from './examples/support'
import { ask, connectAssistant, snapshot } from './assistantTestFlow'

test('coverage worker reproduces the paper calibration without changing source estimates', async ({
  page,
}) => {
  test.setTimeout(120000)
  await page.goto('/app')
  const fixtures = JSON.parse(
    readFileSync(new URL('./fixtures/pretest-calibration.json', import.meta.url), 'utf8'),
  )
  const differences = await page.evaluate(async (rows) => {
    const { runPretestCoverage } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const differences = []
    for (const row of rows) {
      const result = await runPretestCoverage({
        targetBias: row.target_bias,
        pretestBias: row.pretest_bias,
        correlation: row.correlation,
        intervalAlpha: row.interval_alpha,
        pretestAlpha: row.pretest_alpha,
      })
      if (!result.ok) throw new Error(JSON.stringify(result.error))
      differences.push(
        Math.abs(result.value.conditional - row.conditional),
        Math.abs(result.value.unconditional - row.unconditional),
      )
    }
    const refused = await runPretestCoverage({
      targetBias: 0,
      pretestBias: 50,
      correlation: 0,
      intervalAlpha: 0.05,
      pretestAlpha: 0.05,
    })
    return { maximum: Math.max(...differences), refused: !refused.ok }
  }, fixtures.rows)
  expect(differences.maximum).toBeLessThan(1e-10)
  expect(differences.refused).toBe(true)
})

test('coverage records bind settings, periods, covariance and source for assistant review', async ({
  page,
}) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const d = await import(new URL('/src/domain/pretestCoverage.ts', location.href).href)
    const { honestCandidates } = await import(
      new URL('/src/domain/honestDid.ts', location.href).href
    )
    const { runPretestCoverage } = await import(
      new URL('/src/analysis/client.ts', location.href).href
    )
    const { readRun } = await import(new URL('/src/assistant/runRead.ts', location.href).href)
    const { reviewAnalysis } = await import(
      new URL('/src/domain/analysisReview.ts', location.href).href
    )
    const { project: p } = await (await fetch('/examples/ai-adoption-cohort.hirmos.json')).json()
    const base = p.estimationRuns.find(
      (r: { evidence: { kind: string } }) => r.evidence.kind === 'panelDid',
    )
    const specification = {
      controls: 'never-treated',
      baseline: 'universal',
      anticipation: 0,
      firstEvent: null,
      lastEvent: null,
      balance: null,
      confidence: 0.95,
      inference: { kind: 'analytical' },
    }
    const source = {
      ...base,
      configuration: { ...base.configuration, primary: 'staggered', covariates: [], specification },
      evidence: {
        kind: 'staggeredDid',
        specification,
        observations: 40,
        retainedObservations: 40,
        changes: [],
        smallCohorts: [],
        fits: [],
        events: {
          keys: [-2, -1, 0],
          coverage: { kind: 'pointwise' },
          intervals: [
            { kind: 'estimated', estimate: -1, standardError: 1, lower: -2.96, upper: 0.96 },
            { kind: 'reference' },
            { kind: 'estimated', estimate: 2, standardError: 1, lower: 0.04, upper: 3.96 },
          ],
          analyticalCovariance: [
            [1, 0, 0.5],
            [0, 0, 0],
            [0.5, 0, 1],
          ],
        },
      },
    }
    const eventStudy = honestCandidates([source], [])[0].input.value
    const settings = {
      sourceRun: source.id,
      prePeriod: -2,
      postPeriod: 0,
      scenario: { kind: 'calibrated', offset: 0 },
      confidence: 0.95,
      pretestAlpha: 0.05,
    }
    const request = d.pretestRequest(eventStudy, settings)
    const result = await runPretestCoverage(request.value)
    if (!result.ok) throw new Error(JSON.stringify(result.error))
    const run = d.pretestRunSchema.parse({
      kind: 'pretest-coverage-run',
      id: crypto.randomUUID(),
      preparedDataset: source.preparedDataset,
      createdAt: new Date().toISOString(),
      source: { kind: 'estimation', run: source.id },
      eventStudy,
      settings,
      evidence: result.value,
    })
    const workflow = { ...p, kind: 'profiled', estimationRuns: [source], sensitivityRuns: [run] }
    const review = readRun(workflow, { workspace: 'sensitivity', id: run.id, view: 'review' })
    const sourceReview = reviewAnalysis(source, {
      studies: p.studies,
      identifications: p.identifications,
      documents: p.dagDocuments,
      preparedDataset: p.prepared.id,
      sensitivityRuns: [run],
    })
    return {
      matched: d.pretestRunMatches(run, [source], []),
      review,
      sourceFinding: sourceReview.items.find((i: { id: string }) => i.id === 'pretest-coverage'),
      missingSource: d.pretestRunMatches(run, [], []),
      differentScenario: d.pretestRunSchema.safeParse({
        ...run,
        settings: { ...settings, scenario: { kind: 'specified', bias: 4 } },
      }).success,
      differentCovariance: d.pretestRunMatches(
        {
          ...run,
          eventStudy: {
            ...eventStudy,
            covariance: [
              [1, 0.2],
              [0.2, 1],
            ],
          },
        },
        [source],
        [],
      ),
      missingPeriod: d.pretestRequest(eventStudy, { ...settings, prePeriod: -3 }).ok,
      injectedEstimates: d.pretestSettingsSchema.safeParse({ ...settings, coefficients: [0, 0] })
        .success,
      singular: d.pretestRequest(
        {
          ...eventStudy,
          covariance: [
            [1, 2],
            [2, 1],
          ],
        },
        settings,
      ).ok,
    }
  })
  expect(result.matched).toBe(true)
  expect(result.review.review.sourceMatches).toBe(true)
  expect(result.review.review.interpretation).toContain('does not establish parallel trends')
  expect(result.sourceFinding.evidence.kind).toBe('available')
  for (const key of [
    'missingSource',
    'differentScenario',
    'differentCovariance',
    'missingPeriod',
    'injectedEstimates',
    'singular',
  ] as const)
    expect(result[key]).toBe(false)
})

test('pre-test coverage is available in Sensitivity and does not fabricate an event study', async ({
  page,
}) => {
  test.setTimeout(90000)
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await chapter(page, /Sensitivity/)
  await page.getByRole('radio', { name: 'Pre-test coverage', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Calculate coverage', exact: true })).toBeDisabled()
  await expect(
    page.getByText('Choose a saved event study from this prepared dataset.', { exact: true }),
  ).toBeVisible()
  await chapter(page, /Estimation/)
  await page.getByRole('radio', { name: 'Regression designs', exact: true }).click()
  await page.getByRole('radio', { name: 'Event study', exact: true }).click()
  await choose(page, 'Outcome', 'bugs_per_kloc')
  await choose(page, 'Treatment indicator', 'adopted')
  await page.getByRole('button', { name: 'Fit event study', exact: true }).click()
  await expect
    .poll(
      async () =>
        (await snapshot(page, 'AI adoption, March cohort'))?.timeSeriesRuns.some(
          (r: { kind: string }) => r.kind === 'panel-regression',
        ),
      { timeout: 60000 },
    )
    .toBe(true)
  await chapter(page, /Sensitivity/)
  await page.getByRole('radio', { name: 'Pre-test coverage', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Calculate coverage', exact: true })).toBeEnabled()
  await page.getByRole('button', { name: 'Calculate coverage', exact: true }).click()
  await expect(
    page.getByText('Coverage without the pre-test', { exact: true }).first(),
  ).toBeVisible()
  await expect
    .poll(async () => (await snapshot(page, 'AI adoption, March cohort'))?.sensitivityRuns.length)
    .toBe(1)
  const saved = await snapshot(page, 'AI adoption, March cohort')
  expect(saved.sensitivityRuns[0].kind).toBe('pretest-coverage-run')
  await page.reload()
  await chapter(page, /Sensitivity/)
  await expect(
    page.getByText('Coverage without the pre-test', { exact: true }).first(),
  ).toBeVisible()
  const requests = await connectAssistant(page, [
    `s = await tool("workspace_settings", {"key":"pretest-coverage"})
c = s["current"]
c["scenario"] = {"kind":"specified", "bias":0}
p = await tool("propose_workspace", {"key":"pretest-coverage", "configuration":c, "reason":"Calculate coverage under a zero-bias scenario."})
assert p["kind"] == "awaiting-user-review", p
await FINAL_VAR("Review the zero-bias scenario.", ["user-question"])`,
    `r = await tool("run_analysis", {"key":"pretest-coverage", "reason":"Calculate the reviewed scenario."})
assert r["kind"] == "completed", r
v = await tool("run", {"workspace":"sensitivity", "id":r["runId"], "view":"review"})
assert v["review"]["sourceMatches"], v
await FINAL_VAR("Read the saved coverage review.", ["saved-run"])`,
  ])
  await ask(page, 'Propose a zero-bias coverage scenario.')
  const card = page.getByRole('region', { name: 'Workspace configuration proposal' }).last()
  await expect(card).toBeVisible({ timeout: 30000 })
  await card.getByRole('button', { name: 'Apply settings', exact: true }).click()
  await expect(card).toContainText('Settings applied. No analysis was run.')
  expect((await snapshot(page, 'AI adoption, March cohort')).sensitivityRuns).toHaveLength(1)
  await ask(page, 'Calculate the reviewed scenario and read its review.')
  const approval = page.getByRole('region', { name: 'Analysis approval' }).last()
  await expect(approval).toBeVisible({ timeout: 30000 })
  await expect(approval).toContainText('Pre-test coverage')
  expect((await snapshot(page, 'AI adoption, March cohort')).sensitivityRuns).toHaveLength(1)
  await approval.getByRole('button', { name: 'Run and share summary', exact: true }).click()
  await expect(page.getByRole('region', { name: 'Assistant answer' }).last()).toContainText(
    'Read the saved coverage review.',
    { timeout: 45000 },
  )
  await expect
    .poll(async () => (await snapshot(page, 'AI adoption, March cohort')).sensitivityRuns.length)
    .toBe(2)
  const after = await snapshot(page, 'AI adoption, March cohort')
  expect(after.sensitivityRuns).toHaveLength(2)
  expect(after.sensitivityRuns.at(-1).settings.scenario).toEqual({ kind: 'specified', bias: 0 })
  expect(requests()).toBe(2)
})
