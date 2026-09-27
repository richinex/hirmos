import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('causal-model settings survive analysis tabs and chapter navigation', async ({ page }, info) => {
  const errors: string[] = []
  page.on('pageerror', error => errors.push(error.message))
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open Microservices: why did the website slow down?', exact: true }).filter({ visible: true }).click()
  const navigate = (name: RegExp) => chapter(page, name)
  await navigate(/Causal model analysis/)
  await page.getByRole('radio', { name: 'Unusual observation', exact: true }).check()
  await page.getByRole('textbox', { name: 'Observed Website', exact: true }).fill('12.5')
  await page.getByRole('checkbox', { name: 'These values use the same units and transformations as the prepared data.' }).check()
  await page.getByRole('radio', { name: 'Variance contributions', exact: true }).check()
  await page.getByRole('combobox', { name: 'Influence target' }).click()
  await page.getByRole('option', { name: 'Website', exact: true }).click()
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Random seed', exact: true }).fill('47')
  await page.getByRole('spinbutton', { name: 'Baseline samples', exact: true }).fill('222')

  await page.getByRole('radio', { name: 'Arrow strengths', exact: true }).check()
  await page.getByRole('combobox', { name: 'Influence target' }).click()
  await page.getByRole('option', { name: 'API', exact: true }).click()
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Conditional samples', exact: true }).fill('123')

  await page.getByRole('radio', { name: 'Intervention effects', exact: true }).check()
  await page.getByRole('combobox', { name: 'GCM outcome' }).click()
  await page.getByRole('option', { name: 'Website', exact: true }).click()
  await page.getByText('Sampling settings', { exact: true }).click()
  await page.getByRole('spinbutton', { name: 'Trees per model', exact: true }).fill('37')
  await navigate(/DAG workspace/)
  await navigate(/Causal model analysis/)
  await expect(page.getByRole('radio', { name: 'Intervention effects', exact: true })).toBeChecked()
  await expect(page.getByRole('combobox', { name: 'GCM outcome' })).toContainText('Website')
  await page.getByText('Sampling settings', { exact: true }).click()
  await expect(page.getByRole('spinbutton', { name: 'Trees per model', exact: true })).toHaveValue('37')

  await page.getByRole('radio', { name: 'Arrow strengths', exact: true }).check()
  await expect(page.getByRole('combobox', { name: 'Influence target' })).toContainText('API')
  await page.getByText('Sampling settings', { exact: true }).click()
  await expect(page.getByRole('spinbutton', { name: 'Conditional samples', exact: true })).toHaveValue('123')
  await page.getByRole('radio', { name: 'Variance contributions', exact: true }).check()
  await expect(page.getByRole('combobox', { name: 'Influence target' })).toContainText('Website')
  await page.getByText('Sampling settings', { exact: true }).click()
  await expect(page.getByRole('spinbutton', { name: 'Random seed', exact: true })).toHaveValue('47')
  await expect(page.getByRole('spinbutton', { name: 'Baseline samples', exact: true })).toHaveValue('222')
  await page.getByRole('radio', { name: 'Unusual observation', exact: true }).check()
  await expect(page.getByRole('textbox', { name: 'Observed Website', exact: true })).toHaveValue('12.5')
  const confirmation = page.getByRole('checkbox', { name: 'These values use the same units and transformations as the prepared data.' })
  await expect(confirmation).toBeChecked()
  await page.getByRole('textbox', { name: 'Observed Website', exact: true }).fill('13.5')
  await expect(confirmation).not.toBeChecked()
  await page.screenshot({ path: info.outputPath('retained-causal-settings.png') })
  expect(errors).toEqual([])
})

test('causal-model drafts isolate recorded graph revisions and discard invalid bindings', async ({ page }) => {
  await page.goto('/app')
  const actual = await page.evaluate(async () => {
    const { retainCausalModelDrafts, stepCausalModelDraft } = await import(new URL('/src/domain/causalModelDraft.ts', location.href).href)
    const graph = {
      kind: 'editable-dag',
      nodes: [{ kind: 'observed', id: 'a', column: 'A', name: 'A' }, { kind: 'observed', id: 'b', column: 'B', name: 'B' }],
      edges: [{ kind: 'directed', id: 'a-b', cause: 'a', effect: 'b', timing: { kind: 'contemporaneous' }, support: { kind: 'user-assumption', rationale: 'A affects B.' }, evidence: [] }],
    }
    const first = { id: 'first', graph, validation: { structure: { kind: 'sound' }, rationales: { kind: 'complete' } } }
    const second = { ...first, id: 'second' }
    const document = { id: 'document', preparedDataset: 'prepared', dataset: { kind: 'cross-section', observations: 100 }, current: second, audit: [first, second] }
    const selection = { dagDocument: 'document', dagRevision: 'first', preparedDataset: 'prepared' }
    const workspace = { selection, runs: [], checks: [], effects: [], influences: [] }
    const workflow = { kind: 'profiled', prepared: { id: 'prepared', columns: ['A', 'B'], observations: 100 }, dagDocuments: [document], rootCause: workspace }
    const initial = retainCausalModelDrafts([], workflow)
    const edited = stepCausalModelDraft(initial[0], { type: 'intrinsic', draft: { ...initial[0].intrinsic, target: '1', seed: 47 } })
    const confirmed = stepCausalModelDraft(edited, { type: 'confirm', confirmed: true })
    const observed = stepCausalModelDraft(confirmed, { type: 'observation', node: 'a', value: '12' })
    const both = stepCausalModelDraft(observed, { type: 'observation', node: 'b', value: '34' })
    const seeded = stepCausalModelDraft(both, { type: 'attribution', field: 'seed', value: 91 })
    const sampled = stepCausalModelDraft(seeded, { type: 'attribution', field: 'samples', value: 123 })
    const next = retainCausalModelDrafts([edited], { ...workflow, rootCause: { ...workspace, selection: { ...selection, dagRevision: 'second' } } })
    const returned = retainCausalModelDrafts(next, workflow)
    return {
      seeds: returned.map((draft: { intrinsic: { seed: number } }) => draft.intrinsic.seed),
      retained: returned === next && returned[0] === edited,
      replaced: retainCausalModelDrafts(next, { ...workflow, prepared: { ...workflow.prepared, id: 'replacement' } }).length,
      deleted: retainCausalModelDrafts(next, { ...workflow, dagDocuments: [] }).length,
      closed: retainCausalModelDrafts(next, { kind: 'closed' }).length,
      original: initial[0].intrinsic.seed,
      inputs: sampled.inputs.observationDraft,
      confirmed: sampled.inputs.confirmed,
      attribution: [sampled.attribution.seed, sampled.attribution.samples],
      originalInputs: initial[0].inputs.observationDraft,
    }
  })
  expect(actual).toEqual({ seeds: [47, 0], retained: true, replaced: 0, deleted: 0, closed: 0, original: 0, inputs: { a: '12', b: '34' }, confirmed: false, attribution: [91, 123], originalInputs: {} })
})
