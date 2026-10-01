import { expect, test } from '@playwright/test'

/**
 * The adjusted regression's robust (HC1) and clustered intervals: the control offers them, a run
 * carries the chosen interval into the estimate, and the diagnostics name the treatment used.
 * Parity with the reference covariances is checked in the kernel's own tests.
 */

test('robust and clustered intervals replace the classical one in the estimate and its diagnostics', async ({ page }, info) => {
  test.skip(info.project.name !== 'chromium', 'One layout is enough for the run path.')
  await page.goto('/app/projects')
  // This example keeps a prepared column outside its adjustment set, which a cluster column has to be.
  await page.getByRole('button', { name: 'Open AI usage intensity', exact: true }).click()
  await page.getByRole('navigation', { name: 'Workspace sections' }).getByRole('button', { name: /Estimation/ }).click()
  await page.getByRole('radio', { name: /^Adjustment/ }).click()
  await page.getByRole('radio', { name: /^Adjusted linear regression/ }).click()

  const treatment = page.getByRole('radiogroup', { name: 'Error treatment' })
  await expect(treatment.getByRole('radio', { name: 'Classical', exact: true })).toBeChecked()
  await treatment.getByRole('radio', { name: 'Robust (HC1)', exact: true }).click()
  await expect(page.getByText(/The robust \(HC1\) interval allows the error variance to differ between rows/)).toBeVisible()
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByRole('heading', { name: 'Runs (2)', exact: true })).toBeVisible({ timeout: 120_000 })
  const robust = page.getByRole('article').first()
  await expect(robust.getByText('Robust (HC1)', { exact: true })).toBeVisible()
  await expect(robust.getByText('Robust SE (HC1)', { exact: true })).toBeVisible()
  await expect(robust.getByText(/For comparison, the classical interval is/)).toBeVisible()

  // The cluster column must sit outside the design: code_volume_kloc is prepared but excluded from the adjustment set.
  await treatment.getByRole('radio', { name: 'Clustered', exact: true }).click()
  const column = page.getByRole('combobox', { name: 'Cluster column' })
  await column.click()
  await expect(page.getByRole('option', { name: 'team_experience', exact: true })).toHaveCount(0)
  await page.getByRole('option', { name: 'code_volume_kloc', exact: true }).click()
  await expect(page.getByText(/The clustered interval allows errors to correlate within each value of code_volume_kloc/)).toBeVisible()
  await expect(page.getByRole('region', { name: 'Method requirements' })).toContainText('Errors may correlate within each value of code_volume_kloc')
  await page.getByRole('button', { name: /^Run adjusted linear regression/ }).click()
  await expect(page.getByRole('heading', { name: 'Runs (3)', exact: true })).toBeVisible({ timeout: 120_000 })
  const clustered = page.getByRole('article').first()
  await expect(clustered.getByText('Clustered by code_volume_kloc', { exact: true })).toBeVisible()
  await expect(clustered.getByText('Clustered SE', { exact: true })).toBeVisible()
  await expect(clustered.getByText(/[\d,]+ clusters/)).toBeVisible()
})
