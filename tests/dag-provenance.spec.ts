import { test, expect } from '@playwright/test'
import { chapter } from './examples/support'

test('manifest uses the recorded revision and never substitutes the current graph', async ({
  page,
}) => {
  await page.goto('/app')
  const r = await page.evaluate(async () => {
    const { parseSnapshotValue } = await import(
      new URL('/src/domain/persistence.ts', location.href).href
    )
    const { buildResultManifest } = await import(
      new URL('/src/domain/results.ts', location.href).href
    )
    const { resolveStudyDagRevision, restoreMissingDagExploration } = await import(
      new URL('/src/domain/dagProvenance.ts', location.href).href
    )
    const bundle = await (await fetch('/examples/molak-ch7.hirmos.json')).json()
    const parsed = parseSnapshotValue(bundle.project)
    if (!parsed.ok) throw Error(JSON.stringify(parsed.error))
    const p = parsed.value,
      run = p.estimationRuns[0],
      study = p.studies.find((s: { id: string }) => s.id === run.study),
      document = p.dagDocuments.find((d: { id: string }) => d.id === study.dagDocument)
    const original = document.audit.find((r: { id: string }) => r.id === study.dagRevision)
    const next = {
      ...original,
      id: 'later-revision',
      parent: original.id,
      graph: { ...original.graph, edges: [] },
    }
    const changed = { ...document, current: next, audit: [...document.audit, next] }
    const inputs = { ...p, source: p.source, documents: [changed] }
    const m = buildResultManifest(inputs, run, '2026-10-07T00:00:00Z')
    const missing = buildResultManifest(
      { ...inputs, documents: [{ ...changed, audit: [next] }] },
      run,
      '2026-10-07T00:00:00Z',
    )
    const absent = buildResultManifest({ ...inputs, documents: [] }, run, '2026-10-07T00:00:00Z')
    const { exploration: _, ...unsaved } = document
    const recovered = restoreMissingDagExploration(unsaved, p.studies)
    const cleared = restoreMissingDagExploration(
      { ...document, exploration: { treatment: null, outcome: null } },
      p.studies,
    )
    return {
      revision: m.dag.revision,
      expected: study.dagRevision,
      graph: m.dag.graph,
      expectedGraph: original.graph,
      warnings: m.warnings,
      status: resolveStudyDagRevision(study, [changed]).kind,
      missing: missing.dag,
      missingWarnings: missing.warnings,
      absent: absent.dag,
      recovered: recovered.exploration,
      expectedPair: { treatment: study.treatment.node, outcome: study.outcome.node },
      cleared: cleared.exploration,
    }
  })
  expect(r.revision).toBe(r.expected)
  expect(r.graph).toEqual(r.expectedGraph)
  expect(r.status).toBe('earlier')
  expect(r.warnings.join(' ')).toContain('earlier DAG revision')
  expect(r.missing).toBeNull()
  expect(r.absent).toBeNull()
  expect(r.missingWarnings.join(' ')).toContain('not a substitute')
  expect(r.recovered).toEqual(r.expectedPair)
  expect(r.cleared).toEqual({ treatment: null, outcome: null })
})

test('earlier revision is visible in Study design, Estimation and exported Results', async ({
  page,
}, info) => {
  await page.goto('/app')
  await page
    .getByRole('button', { name: 'Open A simulated process with a collider', exact: true })
    .click()
  await expect(page.locator('#data-profile-title')).toBeVisible()
  await page.goto('/app/projects')
  await page.evaluate(async () => {
    const { loadProject, saveProject } = await import(
      new URL('/src/data/projectStore.ts', location.href).href
    )
    const r = await loadProject('9c5b2e7a-1d38-4f64-a2b9-7e4c1f8d3a25')
    if (!r.ok) throw Error(JSON.stringify(r.error))
    const p = r.value,
      d = p.dagDocuments[0],
      next = {
        ...d.current,
        id: crypto.randomUUID(),
        parent: d.current.id,
        createdAt: new Date().toISOString(),
        graph: { ...d.current.graph, edges: d.current.graph.edges.slice(1) },
      }
    const saved = await saveProject({
      ...p,
      dagDocuments: [
        { ...d, current: next, audit: [...d.audit, next], history: [...d.history, d.current] },
      ],
    })
    if (!saved.ok) throw Error(JSON.stringify(saved.error))
  })
  await page
    .getByRole('button', { name: 'Open A simulated process with a collider', exact: true })
    .click()
  await chapter(page, /^Study design/)
  await expect(
    page.getByText(/This study and its results use an earlier DAG revision/).first(),
  ).toBeVisible()
  await chapter(page, /^Estimation/)
  await expect(
    page.getByText(/This study and its results use an earlier DAG revision/).first(),
  ).toBeVisible()
  await page.screenshot({ path: info.outputPath('earlier-revision.png') })
  await chapter(page, /^Results/)
  await expect(
    page.getByText(/This study and its results use an earlier DAG revision/).first(),
  ).toBeVisible()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: 'Export the manifest' }).click()
  await download
})
