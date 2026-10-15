import { expect, test } from '@playwright/test'
import { chapter } from './examples/support'
import { connectAssistant } from './assistantTestFlow'

test('credentials survive project switches without transferring consent or persisting the key', async ({
  page,
}) => {
  page.setDefaultTimeout(15000)
  await page.goto('/app')
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await connectAssistant(page, [])
  await expect(
    page.getByRole('button', { name: 'CausalBuddy settings', exact: true }),
  ).toHaveAttribute('title', 'Settings: connected to OpenRouter')
  await page
    .getByRole('complementary', { name: 'Assistant', exact: true })
    .getByRole('button', { name: 'Close the assistant', exact: true })
    .click()
  await chapter(page, /^Projects/)
  await page
    .getByRole('button', { name: 'Open NSW job training and 1978 earnings', exact: true })
    .click()
  await page
    .getByRole('button', { name: 'Open the assistant', exact: true })
    .click({ position: { x: 20, y: 20 } })
  await expect(page.getByLabel('OpenRouter API key')).toHaveValue('test-key')
  await expect(page.getByLabel('OpenRouter model ID')).toHaveValue('test/model')
  await expect(page.getByRole('button', { name: 'Use these settings', exact: true })).toBeDisabled()
  await page
    .getByRole('complementary', { name: 'Assistant', exact: true })
    .getByRole('button', { name: 'Close the assistant', exact: true })
    .click()
  await chapter(page, /^Projects/)
  await page.getByRole('button', { name: 'Open AI adoption, March cohort', exact: true }).click()
  await page
    .getByRole('button', { name: 'Open the assistant', exact: true })
    .click({ position: { x: 20, y: 20 } })
  await expect(
    page.getByRole('button', { name: 'CausalBuddy settings', exact: true }),
  ).toHaveAttribute('title', 'Settings: connected to OpenRouter')
  const persisted = await page.evaluate(() =>
    JSON.stringify({ local: { ...localStorage }, session: { ...sessionStorage } }),
  )
  expect(persisted).not.toContain('test-key')
  await page.reload()
  const open = page.getByRole('button', { name: 'Open the assistant', exact: true })
  await open.click({ position: { x: 20, y: 20 } })
  await expect(page.getByLabel('OpenRouter API key')).toHaveValue('')
})
