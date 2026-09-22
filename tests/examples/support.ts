import { expect, type Page } from '@playwright/test'
import { readFile, writeFile } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import type { ShippedExample } from '../../src/domain/example'

/**
 * What every example builder shares: driving the product's own controls to build a project, then
 * exporting it with its source file inside and writing the bundle under the id the catalog expects.
 * Run on demand: BUILD_EXAMPLE=1 npx playwright test tests/examples --project=chromium
 */

export const fixture = (name: string): string => fileURLToPath(new URL(`../fixtures/${name}`, import.meta.url))
export const bundleTarget = (example: ShippedExample): string => fileURLToPath(new URL(`../../public${example.bundleUrl}`, import.meta.url))

export const choose = async (page: Page, label: string, option: string): Promise<void> => {
  await page.getByRole('combobox', { name: label }).click()
  await page.getByRole('option', { name: option, exact: true }).click()
}

export const chapter = async (page: Page, name: RegExp) => {
  const toggle = page.getByRole('button', { name: 'Expand chapter list' })
  if (await toggle.isVisible()) await toggle.click()
  const destination = page.getByRole('navigation', { name: 'Workspace chapters' }).getByRole('button', { name })
  await expect(destination).not.toHaveAttribute('aria-disabled', 'true')
  await destination.click()
}

/** Wait for text anywhere on the page; large files make the prepared banner slow to arrive. */
export const bodyText = (page: Page, text: string, timeout = 180_000) =>
  page.waitForFunction((needle) => document.body.innerText.includes(needle), text, { timeout })

export const createProject = async (page: Page, example: ShippedExample): Promise<void> => {
  await page.goto('/app')
  await page.getByRole('textbox', { name: 'Project name' }).fill(example.name)
  await page.getByRole('button', { name: 'Create project' }).click()
  await page.locator('input[type="file"]').setInputFiles(fixture(example.sourceName))
  await page.getByRole('button', { name: /Inspect data/ }).click()
  await expect(page.getByText('Choose the observation structure')).toBeVisible({ timeout: 90_000 })
}

const STRUCTURE = {
  'time series': { radio: /Regular time series/, prepared: /^Time series, / },
  'cross-section': { radio: /Independent observations/, prepared: /^Cross-section, / },
} as const

/** Choose the observation structure, its keys, and the columns, then create the prepared version. */
export const prepare = async (page: Page, options: {
  readonly structure: keyof typeof STRUCTURE
  readonly time?: string
  readonly frequency?: string
  readonly columns: readonly string[] | 'all'
}): Promise<void> => {
  const structure = STRUCTURE[options.structure]
  await page.getByRole('radio', { name: structure.radio }).click()
  if (options.time !== undefined) await choose(page, 'Time column', options.time)
  if (options.frequency !== undefined) await choose(page, 'Frequency', options.frequency)
  if (options.columns === 'all') {
    await page.getByRole('button', { name: 'Select all columns' }).click()
  } else {
    for (const column of options.columns) await page.getByRole('checkbox', { name: column, exact: true }).first().check()
  }
  await page.getByRole('button', { name: /Create prepared/ }).click()
  await expect(page.getByRole('heading', { name: 'Build a DAG or run discovery', exact: true })).toBeVisible()
}

export const addArrow = async (page: Page, cause: string, effect: string, rationale: string, lag?: number): Promise<void> => {
  await choose(page, 'Proposed cause', cause)
  await choose(page, 'Proposed effect', effect)
  if (lag !== undefined) {
    await choose(page, 'Timing', 'Past cause · t−lag')
    await page.getByRole('spinbutton', { name: 'Lag' }).fill(String(lag))
  }
  await page.getByRole('textbox', { name: /Rationale/i }).first().fill(rationale)
  await page.getByRole('button', { name: 'Add the arrow' }).click()
  await expect(page.getByText(new RegExp(`${cause}.*→ ${effect}`)).first()).toBeVisible()
}

export const addUnmeasured = async (page: Page, name: string): Promise<void> => {
  await page.getByRole('button', { name: /Unmeasured variable/ }).click()
  await page.getByLabel('Unmeasured variable name').fill(name)
  await page.getByRole('button', { name: 'Add to DAG' }).click()
}

export const createDag = async (page: Page, name: string): Promise<void> => {
  await chapter(page, /DAG workspace/)
  await page.getByRole('button', { name: /^Substantive knowledge/ }).click()
  await page.getByLabel('DAG name').fill(name)
  await page.getByRole('button', { name: 'Create DAG draft' }).click()
  // With discovery evidence present the inspector opens on Evidence; the arrow controls are under Selection.
  const cause = page.getByRole('combobox', { name: 'Proposed cause' })
  await expect(cause).toBeVisible({ timeout: 5_000 }).catch(() => page.getByRole('radio', { name: 'Selection', exact: true }).first().check())
  await expect(cause).toBeVisible()
}

export const identify = async (page: Page, options: {
  readonly graph: string
  readonly treatment: string
  readonly outcome: string
  readonly mechanism: 'Policy change' | 'Observed choice'
  readonly sentence: string
  readonly consistency?: string
  readonly interference?: string
  /** What the identification step must conclude; back-door adjustment unless the graph needs more. */
  readonly result?: RegExp
  /** The adjustment set to take when the graph offers a choice; the first minimal set by default. */
  readonly adjustment?: RegExp
}): Promise<void> => {
  await chapter(page, /Study design/)
  await choose(page, 'Causal graph', options.graph)
  await choose(page, 'Treatment', options.treatment)
  await choose(page, 'Outcome', options.outcome)
  await page.getByRole('radio', { name: new RegExp(options.mechanism) }).click()
  await page.getByRole('textbox', { name: 'Assignment sentence' }).fill(options.sentence)
  if (options.consistency !== undefined) await page.getByRole('textbox', { name: 'Consistency rationale' }).fill(options.consistency)
  if (options.interference !== undefined) await page.getByRole('textbox', { name: 'No-interference rationale' }).fill(options.interference)
  await identifyEffect(page, options.adjustment)
  await expect(page.getByText(options.result ?? /Identified by back-door adjustment/).first()).toBeVisible({ timeout: 60_000 })
}

/**
 * Identify the effect and, when the graph leaves a choice of adjustment set (several minimal sets, or
 * a canonical set that adds outcome predictors), take the named one; the first minimal set unless told.
 */
export const identifyEffect = async (page: Page, adjustment: RegExp = /^Minimal set 1/): Promise<void> => {
  await page.getByRole('button', { name: 'Identify the effect' }).click()
  const offered = page.getByRole('heading', { name: 'Choose a valid adjustment set' })
  const recorded = page.getByText(/Identified by|not identified/i).first()
  await expect(offered.or(recorded).first()).toBeVisible({ timeout: 60_000 })
  if (await offered.isVisible()) await page.getByRole('button', { name: adjustment }).first().click()
}

/** Pick an estimator in the Estimation chapter, set its options, run it, and wait for the result. */
export const runEstimator = async (page: Page, options: {
  readonly family?: RegExp
  readonly estimator?: RegExp | { readonly value: string }
  readonly choices?: readonly string[]
  readonly run: RegExp
  readonly done?: RegExp | string
}): Promise<void> => {
  await chapter(page, /Estimation/)
  if (options.family !== undefined) await page.getByRole('radio', { name: options.family }).click()
  if (options.estimator instanceof RegExp) await page.getByRole('radio', { name: options.estimator }).click()
  else if (options.estimator !== undefined) await page.locator(`input[type="radio"][value="${options.estimator.value}"]`).check()
  for (const choice of options.choices ?? []) await page.getByRole('radio', { name: choice, exact: true }).click()
  await page.getByRole('button', { name: options.run }).first().click()
  await expect(page.getByText(options.done ?? 'Current estimate').first()).toBeVisible({ timeout: 600_000 })
}

/** Pick a discovery method, apply any settings, run it, and wait until the lab is no longer busy. */
export const runDiscovery = async (page: Page, options: {
  readonly family?: RegExp
  readonly method?: RegExp
  readonly settle?: () => Promise<void>
  readonly run: RegExp
  readonly timeout?: number
}): Promise<void> => {
  await chapter(page, /Discovery lab/)
  if (options.family !== undefined) {
    await page.getByRole('radiogroup', { name: 'Discovery method family' }).getByRole('radio', { name: options.family }).click()
  }
  if (options.method !== undefined) {
    await page.getByRole('radiogroup', { name: 'Discovery method', exact: true }).getByRole('radio', { name: options.method }).click()
  }
  await options.settle?.()
  await page.getByRole('button', { name: options.run }).first().click()
  await page.waitForTimeout(3_000)
  await page.waitForFunction(() => document.querySelector('[aria-busy="true"]') === null, null, { timeout: options.timeout ?? 1_200_000 })
  await expect(page.getByRole('list', { name: 'Discovery runs' })).toBeVisible({ timeout: 60_000 })
}

/** Export with the source file inside, then write the bundle under the catalog's fixed id. */
export const exportBundle = async (page: Page, example: ShippedExample): Promise<void> => {
  await chapter(page, /Data studio/i)
  // The storage and export controls sit in a collapsed panel until the reader opens it.
  const panel = page.locator('details', { has: page.getByText(/Source file\s+storage and export/) })
  if (!(await panel.evaluate((element) => (element as HTMLDetailsElement).open))) await panel.locator('summary').click()
  await page.getByRole('checkbox', { name: /Include the source file/ }).check()
  const download = page.waitForEvent('download')
  // The rail's pill offers the same export; the builder uses the one in the stage.
  await page.locator('main').getByRole('button', { name: /Export project/ }).click()
  const target = bundleTarget(example)
  await (await download).saveAs(target)
  // The app recognises its saved copy of an example by this id, whichever build produced the bundle.
  const bundle = JSON.parse(await readFile(target, 'utf8')) as { project: { project: { id: string } } }
  bundle.project.project.id = example.id
  await writeFile(target, JSON.stringify(bundle, null, 2))
}
