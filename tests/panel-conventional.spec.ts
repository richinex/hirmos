import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'
import { panelInterventionEvidenceSchema } from '../src/domain/estimation'

test('chapter two-period DiD runs through data and analysis workers without synthetic fitting', async ({ page }) => {
  const csv = readFileSync('tests/fixtures/chapter-did.csv', 'utf8')
  await page.goto('/app')
  const result = await page.evaluate(async (source) => {
    const data = await import(new URL('/src/data/client.ts', location.href).href)
    const workflow = await import(new URL('/src/domain/workflow.ts', location.href).href)
    const analysis = await import(new URL('/src/analysis/client.ts', location.href).href)
    const panel = await import(new URL('/src/domain/panel.ts', location.href).href)
    const file = new File([source], 'did-python.csv', { type: 'text/csv' })
    const profile = await data.profileSourceInWorker(workflow.newImportRequestId(), file)
    if (!profile.ok) throw new Error(JSON.stringify(profile.error))
    const column = (name: string) => profile.value.columns.find((item: { name: string }) => item.name === name)?.id
    const matrix = await data.materializePanelInWorker(file, profile.value, {
      unit: column('id'), time: column('time_points'), outcome: column('Y'), treatment: column('treatment'),
    })
    if (!matrix.ok) throw new Error(JSON.stringify(matrix.error))
    const layout = panel.assessPanelInterventionLayout(matrix.value)
    const progress: string[] = []
    const estimate = await analysis.runPanelIntervention(matrix.value.values, matrix.value.rowCount,
      matrix.value.units, matrix.value.periodCodes, { primary: 'did', placeboReplications: 100, seed: 0 },
      (event: { stage: string }) => progress.push(event.stage))
    if (!estimate.ok) throw new Error(JSON.stringify(estimate.error))
    const persistence = await import(new URL('/src/domain/persistence.ts', location.href).href)
    const estimation = await import(new URL('/src/domain/estimation.ts', location.href).href)
    const run = { id: 'did-run', kind: 'panel-intervention-run', configuration: { kind: 'panel-intervention', primary: 'did', placeboReplications: 100, seed: 0 }, evidence: estimate.value }
    const snapshot = persistence.parseSnapshotValue({
      kind: 'hirmos-project', version: 1, savedAt: new Date().toISOString(), origin: { kind: 'user' },
      project: { id: 'did-project', name: 'Chapter DiD', createdAt: new Date().toISOString() },
      source: null, profile: null, prepared: null, stationarity: null, discoveryRuns: [], dagDocuments: [],
      studyDraft: {}, studies: [], identifications: [], estimationRuns: [run], sensitivityRuns: [], counterfactualRuns: [],
    })
    if (!snapshot.ok) throw new Error(JSON.stringify(snapshot.error))
    const study = { estimand: { kind: 'average-treatment-effect-on-treated', scale: 'additive', treatedValue: 1 } }
    const identification = { result: { kind: 'identified', adjustment: { kind: 'canonical', variables: [] } } }
    const matched = estimation.causalEstimateFrom(study, identification, run)
    const mismatched = estimation.causalEstimateFrom(study, identification, { ...run, configuration: { ...run.configuration, primary: 'syntheticDid' } })
    return { layout, estimate, progress, restored: snapshot.value.estimationRuns[0], matched, mismatched }
  }, csv)
  expect(result.layout.ok).toBe(true)
  expect(result.layout.value.controlPreDifferenceSd).toBeNull()
  expect(result.estimate.ok, JSON.stringify(result.estimate)).toBe(true)
  const evidence = panelInterventionEvidenceSchema.parse(result.estimate.value)
  expect(evidence.kind).toBe('panelDid')
  if (evidence.kind !== 'panelDid') throw new Error('Expected conventional DiD evidence')
  expect(evidence.did.estimate).toBeCloseTo(290.53750263095304, 8)
  expect(result.progress).toEqual(['validated-panel', 'difference-in-differences'])
  expect(evidence).not.toHaveProperty('syntheticDid')
  expect(result.restored.evidence).toEqual(evidence)
  expect(result.restored.configuration.primary).toBe('did')
  expect(result.matched.effect.value).toBeCloseTo(evidence.did.estimate, 10)
  expect(result.matched.interval.kind).toBe('none')
  expect(result.mismatched).toBeNull()
})
