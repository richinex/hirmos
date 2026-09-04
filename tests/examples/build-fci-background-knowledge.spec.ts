import { expect, test, type Page } from '@playwright/test'
import { shippedExampleById } from '../../src/domain/example'
import { choose, createProject, exportBundle, prepare, runDiscovery } from './support'

/**
 * causal-learn's twenty-variable reference data through FCI four times: as found, with a forbidden
 * pair, with a required direction, and with tiers. The runs stay in the project for comparison.
 */

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped examples')

/** The tier selects carry no accessible name of their own; the row label is the way to them. */
const setTier = async (page: Page, variable: string, tier: string): Promise<void> => {
  const select = page.locator('label').filter({ hasText: new RegExp(`^${variable}No tier`) }).getByRole('combobox').first()
  await select.scrollIntoViewIfNeeded()
  await select.click()
  await page.getByRole('option', { name: tier, exact: true }).click()
}

test('build the FCI background knowledge example bundle', async ({ page }) => {
  test.setTimeout(3_000_000)
  const example = shippedExampleById('4e8d2a6c-3f71-4b95-9c2d-5a1e7f3b8d92' as never)
  expect(example).not.toBeNull()
  if (example === null) return

  await createProject(page, example)
  await prepare(page, { structure: 'cross-section', columns: Array.from({ length: 20 }, (_, index) => `X${index + 1}`) })

  const fci = { family: /Constraint/, method: /^FCI/, run: /^Run FCI/ } as const
  await runDiscovery(page, fci)

  await page.getByText('Background knowledge').first().scrollIntoViewIfNeeded()
  await page.getByRole('button', { name: 'Add forbidden' }).click()
  await page.getByRole('button', { name: 'Add forbidden' }).click()
  await choose(page, 'forbidden direction source 1', 'X1')
  await choose(page, 'forbidden direction target 1', 'X4')
  await choose(page, 'forbidden direction source 2', 'X4')
  await choose(page, 'forbidden direction target 2', 'X1')
  await runDiscovery(page, { run: fci.run })

  await page.getByRole('button', { name: 'Add required' }).click()
  await choose(page, 'required direction source 1', 'X8')
  await choose(page, 'required direction target 1', 'X18')
  await runDiscovery(page, { run: fci.run })

  await setTier(page, 'X20', 'Tier 0')
  await setTier(page, 'X1', 'Tier 1')
  await runDiscovery(page, { run: fci.run })

  await exportBundle(page, example)
})
