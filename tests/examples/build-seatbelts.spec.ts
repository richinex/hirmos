import { expect, test, type Locator, type Page } from '@playwright/test'
import { fileURLToPath } from 'node:url'

/**
 * Builds the shipped example: the Seatbelts walkthrough run end to end through the product's own
 * controls, then exported with its source file to public/examples/seatbelts.hirmos.json.
 * Run on demand: BUILD_EXAMPLE=1 npx playwright test tests/examples --project=chromium
 */

const seatbelts = fileURLToPath(new URL('../fixtures/Seatbelts.csv', import.meta.url))
const target = fileURLToPath(new URL('../../public/examples/seatbelts.hirmos.json', import.meta.url))

const choose = async (trigger: Locator, label: string) => {
  await trigger.click()
  await trigger.page().getByRole('listbox').getByRole('option', { name: label, exact: true }).click()
}
const chapter = (page: Page, name: RegExp) => page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name }).click()

test.skip(!process.env.BUILD_EXAMPLE, 'set BUILD_EXAMPLE=1 to rebuild the shipped example')

test('build the Seatbelts example bundle', async ({ page }) => {
  test.setTimeout(600_000)
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill('Seat-belt law and road deaths')
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(seatbelts)
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await page.getByRole('radio', { name: /Regular time series/ }).click()
  await choose(page.getByLabel('Time column'), 'rownames')
  for (const column of ['DriversKilled', 'kms', 'PetrolPrice', 'law']) await page.getByRole('checkbox', { name: column, exact: true }).check()
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByText(/Prepared time series/)).toBeVisible({ timeout: 30_000 })

  // diagnostics
  await page.getByRole('button', { name: /Run stationarity/ }).click()
  await expect(page.getByText(/Stationarity tests · 192 rows/)).toBeVisible({ timeout: 120_000 })
  await page.getByRole('radio', { name: /Breaks and seasonality/ }).click()
  await page.getByRole('button', { name: /Find breaks/ }).click()
  await expect(page.getByText(/4 series checked/)).toBeVisible({ timeout: 120_000 })
  await page.getByRole('radio', { name: /Granger/ }).click()
  await choose(page.getByLabel('Candidate cause'), 'kms')
  await choose(page.getByLabel('Target'), 'DriversKilled')
  await page.getByRole('button', { name: /Run Granger/ }).click()
  await expect(page.getByText(/lags 1 to 4/)).toBeVisible({ timeout: 60_000 })

  // discovery: two runs
  await chapter(page, /Discovery/)
  await page.getByRole('button', { name: /Run PCMCI\+/ }).click()
  await expect(page.getByRole('heading', { name: /Stationary lag-graph evidence/ })).toBeVisible({ timeout: 120_000 })
  await page.getByRole('radio', { name: 'LPCMCI', exact: true }).click()
  await page.getByRole('button', { name: /Run LPCMCI/ }).click()
  await expect(page.getByRole('heading', { name: /Latent-aware/ })).toBeVisible({ timeout: 120_000 })

  // the graph
  await chapter(page, /DAG/)
  await page.getByRole('button', { name: /Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill('Traffic and fatalities')
  await page.getByRole('button', { name: /Create DAG/ }).click()
  const arrows = [
    ['PetrolPrice', 'kms', 'Fuel cost changes how much people drive.'],
    ['PetrolPrice', 'DriversKilled', 'Fuel cost changes exposure and driving style.'],
    ['kms', 'DriversKilled', 'More distance driven means more exposure to fatal crashes.'],
    ['law', 'DriversKilled', 'The seat-belt law changes survival in crashes.'],
  ] as const
  await page.getByRole('button', { name: 'Selection', exact: true }).click()
  const ledger = page.getByRole('table', { name: 'Arrows' })
  for (const [cause, effect, basis] of arrows) {
    await choose(page.getByLabel('Proposed cause'), cause)
    await choose(page.getByLabel('Proposed effect'), effect)
    await page.getByLabel(/Rationale|Substantive basis/).fill(basis)
    await page.getByRole('button', { name: /Add the arrow|Add edge/ }).click()
    await expect(ledger.getByText(`${cause} → ${effect}`, { exact: true })).toBeVisible()
  }
  await choose(page.getByLabel('Treatment'), 'kms')
  await choose(page.getByLabel('Outcome'), 'DriversKilled')
  await page.getByRole('button', { name: /Tidy graph/ }).click()
  await page.getByRole('button', { name: 'Intervene' }).click()
  await choose(page.getByLabel('Variable to set'), 'kms')
  await choose(page.getByLabel('Variable to read'), 'DriversKilled')
  await page.getByRole('button', { name: /Ask the network/ }).click()
  await expect(page.getByText(/Graph revision/)).toBeVisible({ timeout: 60_000 })
  await page.getByRole('button', { name: 'Selection' }).click()
  await page.getByRole('button', { name: /Use for study/ }).click()

  // the study
  await page.getByRole('radio', { name: /Observed choice/ }).click()
  await page.getByPlaceholder(/who or what set the treatment/).fill('Distance driven each month followed fuel prices and the season; nobody assigned it.')
  await page.getByPlaceholder(/Why this holds here/).nth(1).fill("One month of driving does not change another month's deaths.")
  await page.getByRole('button', { name: /Identify the effect/ }).click()
  await expect(page.getByText(/Identified by back-door adjustment/).first()).toBeVisible({ timeout: 60_000 })
  await page.getByRole('button', { name: /Continue to estimation/ }).click()

  // three estimates
  await page.getByRole('radio', { name: /Bayesian negative binomial/ }).click()
  await page.getByRole('spinbutton', { name: 'Warmup' }).fill('200')
  await page.getByRole('spinbutton', { name: 'Draws' }).fill('400')
  await page.getByRole('button', { name: /Run Bayesian/ }).click()
  await expect(page.getByText('Runs · 1')).toBeVisible({ timeout: 180_000 })
  await page.getByRole('radio', { name: /Discrete Bayesian network/ }).click()
  await page.getByRole('button', { name: /Run discrete BN/i }).click()
  await expect(page.getByText('Runs · 2')).toBeVisible({ timeout: 120_000 })
  await page.getByRole('radio', { name: /Adjusted linear regression/ }).click()
  await page.getByRole('button', { name: /Run adjusted linear/ }).click()
  await expect(page.getByText('Runs · 3')).toBeVisible({ timeout: 120_000 })

  // one probe, one counterfactual
  await chapter(page, /Sensitivity/)
  await page.getByRole('button', { name: /^Run (refuters|perturbation)/ }).click()
  await expect(page.getByText('Probes · 1')).toBeVisible({ timeout: 180_000 })
  await chapter(page, /Counterfactual/)
  await page.getByLabel(/First intervention/).fill('10000')
  await page.getByLabel(/Second intervention/).fill('20000')
  await page.getByRole('button', { name: /Run counterfactual/ }).click()
  await expect(page.getByText(/Counterfactuals · 1|Runs · 1/)).toBeVisible({ timeout: 120_000 })

  // export with the source file inside
  await chapter(page, /Data studio/i)
  await page.getByRole('checkbox', { name: /Include the source file/ }).check()
  const download = page.waitForEvent('download')
  await page.getByRole('button', { name: /Export project/ }).click()
  await (await download).saveAs(target)
})
