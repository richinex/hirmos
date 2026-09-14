import { expect, test } from '@playwright/test'
import { readFileSync } from 'node:fs'

const source = (file: string) => readFileSync(new URL(`../src/${file}`, import.meta.url), 'utf8')

test('descriptions name model behaviour while retaining source references', () => {
  const ardl = source('components/time-series/ArdlModelPanel.tsx')
  expect(ardl).toContain('Specify lags with diagnostics')
  expect(ardl).toContain('“Select by AIC” may exclude predictors.')
  expect(ardl).toContain('keep every selected predictor and choose its lag order.')
  expect(ardl).not.toContain('R ARDL')
  expect(source('components/time-series/ArdlModelResult.tsx')).toContain('Critical bounds for these statistics are not reported.')
  expect(source('domain/timeSeriesEquations.ts')).toContain('Pesaran, Shin and Smith (2001)')
  expect(source('domain/survivalEquations.ts')).toContain('Prentice parameterisation')
  expect(source('components/survival/SurvivalPanel.tsx')).not.toMatch(/the way lifelines does|lifelines fits them in alphabetical order|lifelines NelsonAalenFitter default/)
  expect(source('components/survival/SurvivalRegressionResult.tsx')).not.toContain('Source: ranger survival trees.')
  expect(source('components/data/SeriesStructureCard.tsx')).not.toContain('follow the notebook defaults')
})
