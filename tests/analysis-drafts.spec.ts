import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

test('time-series settings survive model and chapter navigation', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open Campylobacter cases and an outbreak step', exact: true }).filter({ visible: true }).click()
  await chapter(page, /Time-series analysis/)
  await page.getByRole('spinbutton', { name: 'Candidate start row' }).fill('98')
  await page.getByRole('spinbutton', { name: 'Candidate end row' }).fill('102')
  await page.getByRole('radio', { name: 'ARDL', exact: true }).check()
  await page.getByLabel('Multiplier horizon', { exact: true }).fill('24')
  await page.getByRole('radio', { name: 'VECM', exact: true }).check()
  await page.getByLabel('Maximum lag', { exact: true }).fill('4')
  await page.getByRole('spinbutton', { name: /^Forecast periods/ }).fill('8')
  await chapter(page, /DAG workspace/)
  await chapter(page, /Time-series analysis/)
  await expect(page.getByRole('radio', { name: 'VECM', exact: true })).toBeChecked()
  await expect(page.getByLabel('Maximum lag', { exact: true })).toHaveValue('4')
  await expect(page.getByRole('spinbutton', { name: /^Forecast periods/ })).toHaveValue('8')
  await page.getByRole('radio', { name: 'ARDL', exact: true }).check()
  await expect(page.getByLabel('Multiplier horizon', { exact: true })).toHaveValue('24')
  await page.getByRole('radio', { name: 'Count models', exact: true }).check()
  await expect(page.getByRole('spinbutton', { name: 'Candidate start row' })).toHaveValue('98')
  await expect(page.getByRole('spinbutton', { name: 'Candidate end row' })).toHaveValue('102')
})

test('time-series drafts reset for replacement data and estimation rejects stale preflight results', async ({ page }) => {
  await page.goto('/app')
  const checks = await page.evaluate(async () => {
    const series = await import(new URL('/src/domain/timeSeriesDraft.ts', location.href).href)
    const estimation = await import(new URL('/src/domain/estimationDraft.ts', location.href).href)
    const workflow = { kind: 'profiled', prepared: { kind: 'prepared-time-series', id: 'a', observations: 100 }, timeSeriesRuns: [] }
    const initial = series.retainTimeSeriesDraft(null, workflow)
    const edited = series.stepTimeSeriesDraft(initial, { type: 'ardl', field: 'horizon', value: '24' })
    const binding = { prepared: 'a', dagRevision: 'g1', treatment: 't', outcome: 'y' }
    const state = { studyDataPreflight: { kind: 'loading', binding }, panelPreflight: { kind: 'not-required' } }
    const event = { type: 'study-data-preflight-succeeded', binding, treatmentIsBinary: true, outcomeIsCount: false, observedGraphIsBinary: false }
    const panelBinding = { prepared: 'a', unit: 'u', time: 'time', treatment: 't', outcome: 'y' }
    const panelState = { ...state, panelPreflight: { kind: 'loading', binding: panelBinding } }
    return {
      retained: series.retainTimeSeriesDraft(edited, workflow) === edited,
      reset: series.retainTimeSeriesDraft(edited, { ...workflow, prepared: { ...workflow.prepared, id: 'b' } }).ardl.horizon,
      closed: series.retainTimeSeriesDraft(edited, { kind: 'closed' }),
      accepted: estimation.stepEstimationDraft(state, event).studyDataPreflight.kind,
      stale: estimation.stepEstimationDraft(state, { ...event, binding: { ...binding, dagRevision: 'g2' } }) === state,
      inactive: estimation.stepEstimationDraft({ ...state, studyDataPreflight: { kind: 'not-required' } }, event).studyDataPreflight.kind,
      panelStale: estimation.stepEstimationDraft(panelState, { type: 'panel-preflight-refused', binding: { ...panelBinding, prepared: 'b' }, problem: 'unused' }) === panelState,
    }
  })
  expect(checks).toEqual({ retained: true, reset: '12', closed: null, accepted: 'ready', stale: true, inactive: 'not-required', panelStale: true })
})

test('survival keeps the selected model and covariates after navigation', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open A simulated process with a collider', exact: true }).filter({ visible: true }).click()
  await chapter(page, /Survival analysis/)
  await page.getByRole('radio', { name: 'Cox regression', exact: true }).check()
  const covariates = page.getByRole('group', { name: 'Covariates', exact: true })
  const variable = covariates.getByRole('checkbox').first()
  const name = await variable.getAttribute('value')
  await variable.check()
  await chapter(page, /DAG workspace/)
  await chapter(page, /Survival analysis/)
  await expect(page.getByRole('radio', { name: 'Cox regression', exact: true })).toBeChecked()
  await expect(covariates.getByRole('checkbox').first()).toBeChecked()
  expect(await covariates.getByRole('checkbox').first().getAttribute('value')).toBe(name)
})

test('counterfactual interventions survive chapter navigation', async ({ page }) => {
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open A simulated process with a collider', exact: true }).filter({ visible: true }).click()
  await chapter(page, /Counterfactuals/)
  await page.getByRole('spinbutton', { name: 'First intervention value', exact: true }).fill('2.5')
  await chapter(page, /DAG workspace/)
  await chapter(page, /Counterfactuals/)
  await expect(page.getByRole('spinbutton', { name: 'First intervention value', exact: true })).toHaveValue('2.5')
})

test('analysis drafts retain edits and reject deleted dependencies', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const sensitivity = await import(new URL('/src/domain/sensitivityDraft.ts', location.href).href)
    const counterfactual = await import(new URL('/src/domain/counterfactualDraft.ts', location.href).href)
    const survival = await import(new URL('/src/domain/survivalDraft.ts', location.href).href)
    const estimates = [{ id: 'a', kind: 'backdoor-linear-run' }, { id: 'b', kind: 'backdoor-linear-run' }]
    const identified = [{ id: 'a', result: { kind: 'identified' } }, { id: 'b', result: { kind: 'identified' } }]
    const workflow = { kind: 'profiled', prepared: { id: 'prepared', kind: 'prepared-cross-section', columns: ['time', 'event'] }, profile: { columns: [{ id: 'time', name: 'time', duckdbType: 'DOUBLE' }, { id: 'event', name: 'event', duckdbType: 'INTEGER' }] }, estimationRuns: estimates, sensitivityRuns: [], identifications: identified, counterfactualRuns: [], survivalRuns: [] }
    const initial = sensitivity.retainSensitivityDraft(null, workflow)
    const changed = { ...initial, draft: sensitivity.stepSensitivityDraft(initial.draft, { type: 'configured', configuration: { ...initial.draft.configurations['linear-refutation'], seed: 91 } }) }
    const cf = counterfactual.retainCounterfactualDraft(null, workflow)
    const edited = { ...cf, draft: counterfactual.stepCounterfactualDraft(cf.draft, { type: 'configured', configuration: { ...cf.draft.configuration, interventions: [2.5, 0] } }) }
    const survivalInitial = survival.retainSurvivalDraft(null, workflow)
    const survivalEdited = { ...survivalInitial, draft: { ...survivalInitial.draft, horizon: 33 } }
    return {
      sensitivityRetained: sensitivity.retainSensitivityDraft(changed, workflow) === changed,
      counterfactualRetained: counterfactual.retainCounterfactualDraft(edited, workflow) === edited,
      deletedEstimate: sensitivity.retainSensitivityDraft(changed, { ...workflow, estimationRuns: estimates.slice(0, 1) }).draft.estimationRun,
      deletedIdentification: counterfactual.retainCounterfactualDraft(edited, { ...workflow, identifications: identified.slice(0, 1) }).draft.identification,
      resetSensitivity: sensitivity.retainSensitivityDraft(changed, { ...workflow, prepared: { ...workflow.prepared, id: 'new' } }).draft.configurations['linear-refutation'].seed === initial.draft.configurations['linear-refutation'].seed,
      resetCounterfactual: counterfactual.retainCounterfactualDraft(edited, { ...workflow, prepared: { ...workflow.prepared, id: 'new' } }).draft.configuration.interventions,
      originalInterventions: cf.draft.configuration.interventions,
      closed: [sensitivity.retainSensitivityDraft(changed, { kind: 'closed' }), counterfactual.retainCounterfactualDraft(edited, { kind: 'closed' })],
      survivalRetained: survival.retainSurvivalDraft(survivalEdited, workflow) === survivalEdited,
      survivalReset: survival.retainSurvivalDraft(survivalEdited, { ...workflow, prepared: { ...workflow.prepared, id: 'new' } }).draft.horizon,
    }
  })
  expect(result.sensitivityRetained).toBe(true)
  expect(result.counterfactualRetained).toBe(true)
  expect(result.deletedEstimate).toBeNull()
  expect(result.deletedIdentification).toBeNull()
  expect(result.resetSensitivity).toBe(true)
  expect(result.resetCounterfactual).toEqual(result.originalInterventions)
  expect(result.closed).toEqual([null, null])
  expect(result.survivalRetained).toBe(true)
  expect(result.survivalReset).toBe(10)
})
