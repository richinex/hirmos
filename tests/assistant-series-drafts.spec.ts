import { expect, test } from '@playwright/test'

test('reviewed series settings reject invalid inputs and preserve other drafts', async ({ page }) => {
  await page.goto('/app')
  const result = await page.evaluate(async () => {
    const series = await import(new URL('/src/domain/timeSeriesWorkspaceSettings.ts', location.href).href)
    const regression = await import(new URL('/src/domain/regressionWorkspaceSettings.ts', location.href).href)
    const { stepTimeSeriesDraft } = await import(new URL('/src/domain/timeSeriesDraft.ts', location.href).href)
    const { z } = await import(new URL('/node_modules/.vite/deps/zod.js', location.href).href)
    const count = { outcome: 'y', link: 'log', pastObservationLags: [1], pastMeanLags: [1], candidateStart: 20, candidateEnd: 30, delta: 0 }
    const cohort = { outcome: 'y', family: { kind: 'negativeBinomial', exposure: null }, model: { kind: 'events', onset: 'treated', cohort: '2023', window: { kind: 'all' } }, covariates: [], confidence: '0.95', iterations: '100', tolerance: '0.000001' }
    const original = { count, regression: { marker: 'distributed lags' }, cohortRegression: cohort, ardl: { marker: 'ARDL' } }
    const updated = stepTimeSeriesDraft(original, { type: 'count-settings', value: { ...count, link: 'identity' } })
    const updatedCohort = stepTimeSeriesDraft(updated, { type: 'cohort-regression', value: { ...cohort, confidence: '0.9' } })
    return {
      valid: series.countDraftSchema.safeParse(count).success,
      invalid: series.countDraftSchema.safeParse({ ...count, pastObservationLags: [2, 1] }).success,
      missingColumn: series.checkSettingsColumns(count, ['missing'], [{ id: 'y', name: 'Outcome' }]).ok,
      cohortValid: regression.countRegressionDraftSchema.safeParse(cohort).success,
      cohortWithoutPanel: regression.checkCountRegressionDraft(cohort, false).ok,
      schemaExportable: !!z.toJSONSchema(regression.countRegressionDraftSchema, { io: 'input' }).properties,
      unchanged: updatedCohort.regression === original.regression && updatedCohort.ardl === original.ardl,
      count: updatedCohort.count.link,
      confidence: updatedCohort.cohortRegression.confidence,
    }
  })
  expect(result).toEqual({ valid: true, invalid: false, missingColumn: false, cohortValid: true, cohortWithoutPanel: false, schemaExportable: true, unchanged: true, count: 'identity', confidence: '0.9' })
})
