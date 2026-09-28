import { expect, test } from '@playwright/test'
import { DEFAULT_BOOSTED_GRID, parseBoostedGridAxis } from '../src/domain/estimation'

/**
 * The boosted propensity grid is the reader's to set. Each axis refuses what scikit-learn's
 * GradientBoostingClassifier would refuse.
 */

test('a new boosted model starts from the default grid', () => {
  expect(DEFAULT_BOOSTED_GRID.learningRate).toEqual([0.005, 0.01, 0.15, 0.2])
  expect(DEFAULT_BOOSTED_GRID.maxDepth).toEqual([1, 2, 3, 4, 5])
  expect(DEFAULT_BOOSTED_GRID.nEstimators).toEqual([100, 150, 200, 300])
  expect(DEFAULT_BOOSTED_GRID.splits).toBe(5)
})

test('each axis reads commas or spaces and refuses what the classifier refuses', () => {
  expect(parseBoostedGridAxis('learningRate', '0.005, 0.01 0.15,0.2')).toEqual({ ok: true, value: [0.005, 0.01, 0.15, 0.2] })
  expect(parseBoostedGridAxis('maxDepth', '3, 1')).toEqual({ ok: true, value: [3, 1] })
  expect(parseBoostedGridAxis('learningRate', ' , ')).toEqual({ ok: false, error: { kind: 'empty' } })
  expect(parseBoostedGridAxis('learningRate', '0.1, fast')).toEqual({ ok: false, error: { kind: 'not-a-number', token: 'fast' } })
  expect(parseBoostedGridAxis('learningRate', '0, 0.1')).toEqual({ ok: true, value: [0, 0.1] })
  expect(parseBoostedGridAxis('learningRate', '0.1, -0.2')).toEqual({ ok: false, error: { kind: 'negative', value: -0.2 } })
  expect(parseBoostedGridAxis('nEstimators', '0')).toEqual({ ok: false, error: { kind: 'below-one', value: 0 } })
  expect(parseBoostedGridAxis('maxDepth', '2.5')).toEqual({ ok: false, error: { kind: 'not-whole', value: 2.5 } })
  expect(parseBoostedGridAxis('nEstimators', '100, 100')).toEqual({ ok: false, error: { kind: 'repeated', value: 100 } })
})

test('the grid fields show the grid the run searches, and refuse bad text with its reason', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for three text fields.')
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open NSW job training and 1978 earnings', exact: true }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Estimation/ }).click()
  await page.getByRole('radio', { name: /^Propensity score/ }).click()
  await page.getByRole('radio', { name: /^Inverse propensity weighting/ }).click()
  await page.getByRole('radio', { name: 'Boosted', exact: true }).click()
  await page.getByText('Search grid', { exact: true }).click()

  const rates = page.getByRole('textbox', { name: 'Learning rates', exact: true })
  await expect(rates).toHaveValue('0.005, 0.01, 0.15, 0.2')
  await expect(page.getByRole('textbox', { name: 'Tree depths', exact: true })).toHaveValue('1, 2, 3, 4, 5')
  await expect(page.getByRole('textbox', { name: 'Tree counts', exact: true })).toHaveValue('100, 150, 200, 300')
  await expect(page.getByText(/The search scores 80 candidates/)).toBeVisible()

  await rates.fill('0.05, 0.1')
  await expect(page.getByText(/The search scores 40 candidates/)).toBeVisible()

  await rates.fill('0.05, -1')
  await expect(page.getByRole('alert').filter({ hasText: 'Each learning rate must be 0 or more; -1 is not.' })).toBeVisible()
  await expect(page.getByText(/The search scores 40 candidates/)).toBeVisible()
  await page.getByRole('textbox', { name: 'Tree depths', exact: true }).click()
  await expect(rates).toHaveValue('0.05, 0.1')
  await expect(page.getByRole('alert').filter({ hasText: 'learning rate' })).toHaveCount(0)
})
