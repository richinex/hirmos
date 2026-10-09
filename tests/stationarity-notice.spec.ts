import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'

/**
 * The pre-run notice groups series by their stationarity issue: one line per reason on the stage, the
 * series listed in the requirements panel, and an action that opens the test that is missing.
 */

test('the stage names the series once per reason and its action opens the stationarity pane', async ({ page }, info) => {
  await page.goto('/app/projects')
  await page.getByRole('button', { name: 'Open Seat-belt law and road deaths', exact: true }).click()
  await page.waitForURL(/\/app\/projects\/[^/]+\/[a-z-]+$/)
  await chapter(page, /Discovery lab/)
  const notice = page.getByTestId('eligibility-notice').first()
  await expect(notice).toBeVisible({ timeout: 30_000 })
  await expect(notice).toContainText(/for 4 series \(/)
  await expect(notice).not.toContainText(/'s stationarity evidence/)
  const action = notice.getByRole('button', { name: /^(Run stationarity tests|Test the first difference)$/ })
  await expect(action).toBeVisible()
  // The series are listed once, in the requirements panel: beside the stage, or in the pane the Panes bar opens on a phone.
  if (info.project.name === 'mobile-chromium') {
    await page.getByRole('group', { name: 'Panes' }).getByRole('button', { name: /requirements/i }).click()
    await expect(page.getByRole('dialog').getByRole('listitem').filter({ hasText: /^DriversKilled/ })).toBeVisible()
    await page.keyboard.press('Escape')
  } else {
    await expect(page.getByRole('region', { name: 'Method requirements' }).getByRole('listitem').filter({ hasText: /^DriversKilled/ })).toBeVisible()
  }
  await action.click()
  await page.waitForURL(/\/app\/projects\/[^/]+\/data$/)
  await expect(page.getByRole('radio', { name: /^Stationarity/ })).toBeChecked()
})
